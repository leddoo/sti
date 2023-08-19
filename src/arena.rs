use core::ptr::NonNull;
use core::alloc::Layout;
use core::cell::Cell;
use core::mem::{size_of, align_of};

use crate::static_assert;
use crate::num::{is_pow2, ceil_to_multiple_pow2, OrdUtils};
use crate::alloc::{Alloc, GlobalAlloc};


pub const MAX_ALIGN:      usize =   16;
pub const MIN_BLOCK_SIZE: usize = 4096;

static_assert!(is_pow2(MAX_ALIGN));


const HEADER_SIZE: usize = (size_of::<BlockHeader>() + MAX_ALIGN-1) / MAX_ALIGN * MAX_ALIGN;

static_assert!(HEADER_SIZE >= size_of::<BlockHeader>());


const CAP_MAX: usize = usize::MAX/4 + 1;

static_assert!(CAP_MAX % MAX_ALIGN == 0);



pub struct Arena<A: Alloc = GlobalAlloc> {
    alloc: A,
    block: Cell<NonNull<u8>>,
    cap:   Cell<usize>, // <= CAP_MAX
    used:  Cell<usize>, // <= self.cap
                        // && (self.used >= size_of::<BlockHeader>() || cap == 0)
                        // && self.used % MAX_ALIGN == 0

    pub min_block_size: Cell<usize>,
}

struct BlockHeader {
    prev:      NonNull<u8>,
    prev_cap:  usize,
}


impl Arena<GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }
}

impl<A: Alloc> Arena<A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Arena {
            alloc,
            block: NonNull::dangling().into(),
            cap:  0.into(),
            used: 0.into(),
            min_block_size: MIN_BLOCK_SIZE.into(),
        }
    }


    #[inline]
    pub fn alloc_ptr<T>(&self) -> NonNull<T> {
        self.alloc(Layout::new::<T>()).unwrap().cast()
    }

    #[inline]
    pub fn alloc_new<T>(&self, value: T) -> &mut T {
        let mut ptr = self.alloc_ptr::<T>();
        unsafe {
            ptr.as_ptr().write(value);
            ptr.as_mut()
        }
    }


    pub fn reset(&mut self) {
        self.debug_integrity_check();

        // @todo: unwind.
        if self.cap.get() > 0 {
            self.used.set(HEADER_SIZE);
        }

        self.debug_integrity_check();
    }



    /// # safety:
    /// - `layout.align() <= MAX_ALIGN`.
    #[cold]
    unsafe fn alloc_slow_path(&self, layout: Layout) -> Option<NonNull<u8>> {
        debug_assert!(layout.align() <= MAX_ALIGN);

        let size = ceil_to_multiple_pow2(layout.size(), MAX_ALIGN);

        let new_cap = {
            // can't overflow cause `cap <= CAP_MAX`.
            let new_cap = 2*self.cap.get();

            // can't overflow cause `layout.size() <= usize::MAX/2` (and block headers are small).
            let new_cap = new_cap.at_least(HEADER_SIZE + size);

            let new_cap = new_cap.at_least(self.min_block_size.get());

            if new_cap > CAP_MAX {
                return None;
            }

            static_assert!(CAP_MAX % MAX_ALIGN == 0);

            // MAX_ALIGN divides CAP_MAX.
            let new_cap = ceil_to_multiple_pow2(new_cap, MAX_ALIGN);
            debug_assert!(new_cap <= CAP_MAX);

            new_cap
        };

        // `MAX_ALIGN` is a small non-zero power of two.
        // `new_cap <= isize::MAX/2`.
        let block_layout = unsafe { Layout::from_size_align_unchecked(new_cap, MAX_ALIGN) };
        let block = self.alloc.alloc(block_layout)?;

        // save current state.
        unsafe {
            static_assert!(align_of::<BlockHeader>() <= MAX_ALIGN);

            let header = BlockHeader {
                prev:     self.block.get(),
                prev_cap: self.cap.get(),
            };

            // new_cap has enough space for a block header.
            (block.as_ptr() as *mut BlockHeader).write(header);
        }

        // make allocation.
        let result = unsafe {
            // new_cap has enough space for the padded header and the allocation.
            // `block + HEADER_SIZE` is MAX_ALIGN aligned (which is `>= layout.align()`).
            let ptr = block.as_ptr().add(HEADER_SIZE);

            // block is `NonNull` and `ptr` is a valid derived pointer.
            NonNull::new_unchecked(ptr)
        };

        // attach new block.
        self.block.set(block);
        self.cap.set(new_cap);
        self.used.set(HEADER_SIZE + size);

        self.debug_integrity_check();

        return Some(result);
    }


    fn _integrity_check(&self) {
        let cap = self.cap.get();
        debug_assert!(cap <= CAP_MAX);

        let used = self.used.get();
        debug_assert!(used <= cap);
        debug_assert!(used >= size_of::<BlockHeader>() || cap == 0);
        debug_assert!(used % MAX_ALIGN == 0);
    }

    #[inline(always)]
    fn debug_integrity_check(&self) {
        #[cfg(debug_assertions)]
        self._integrity_check();
    }
}


impl<A: Alloc> Drop for Arena<A> {
    fn drop(&mut self) {
        let mut block = self.block.get();
        let mut cap   = self.cap.get();
        while cap != 0 {
            unsafe {
                let header = (block.as_ptr() as *mut BlockHeader).read();
                self.alloc.free(block, Layout::from_size_align_unchecked(cap, MAX_ALIGN));
                block = header.prev;
                cap   = header.prev_cap;
            }
        }

        self.block.set(NonNull::dangling());
        self.cap.set(0);
        self.used.set(0);
    }
}


impl<A: Alloc> Alloc for Arena<A> {
    /// # safety:
    /// - `layout.size() > 0`.
    unsafe fn alloc_nonzero(&self, layout: Layout) -> Option<NonNull<u8>> {
        debug_assert!(layout.size() > 0);
        self.debug_integrity_check();

        if layout.align() > MAX_ALIGN {
            return None;
        }

        // at most `isize::MAX + a bit`.
        let size = ceil_to_multiple_pow2(layout.size(), MAX_ALIGN);

        let cap  = self.cap.get();
        let used = self.used.get();

        // can't overflow cause `used <= cap <= CAP_MAX`
        // and `size <= usize::MAX/2 + a bit`.
        if used + size <= cap {
            let result = unsafe {
                let block = self.block.get();

                // `block.add(cap)` is the end of the allocation,
                // and `used <= cap`.
                let ptr = block.as_ptr().add(used);

                // the resulting pointer is aligned, because
                // `block + used` is always `MAX_ALIGN` aligned,
                // and `layout.align() divides MAX_ALIGN` (<= for power of two).

                // `block` is always `NonNull`, so any valid derived pointer is also `NonNull`.
                NonNull::new_unchecked(ptr)
            };

            self.used.set(used + size);

            Some(result)
        }
        else {
            // layout.align() <= MAX_ALIGN.
            unsafe { self.alloc_slow_path(layout) }
        }
    }

    #[inline(always)]
    unsafe fn free_nonzero(&self, ptr: NonNull<u8>, layout: Layout) {
        let _ = (ptr, layout);
        // no-op.
    }

    #[inline(always)]
    unsafe fn try_realloc_nonzero(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        self.debug_integrity_check();

        let block = self.block.get().as_ptr() as usize;
        let ptr = ptr.as_ptr() as usize;
        debug_assert!(ptr % MAX_ALIGN == 0);

        // at most `isize::MAX + a bit`.
        let old_size = ceil_to_multiple_pow2(old_layout.size(), MAX_ALIGN);

        let used_end = block + self.used.get();
        let alloc_end = ptr + old_size;

        if alloc_end == used_end {
            // at most `isize::MAX + a bit`.
            let new_size = ceil_to_multiple_pow2(new_layout.size(), MAX_ALIGN);

            let block_end = block + self.cap.get();
            let block_rem = block_end - ptr;
            if new_size <= block_rem {
                self.used.set(self.used.get() - old_size + new_size);
                debug_assert!(self.used.get() <= self.cap.get());
                return Ok(());
            }
        }

        // @temp
        Err(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_base<A: Alloc>(arena: &Arena<A>) -> Option<usize> {
        (arena.cap.get() != 0).then_some(arena.block.get().as_ptr() as usize + HEADER_SIZE)
    }

    #[test]
    fn arena_general() {
        struct Foo {
            name: &'static str,
            age:  u8,
        }

        // init.
        let mut arena = Arena::new();
        assert!(get_base(&arena).is_none());

        // zst does nothing.
        arena.alloc_new(());
        assert!(get_base(&arena).is_none());

        // first allocation allocates block.
        let first = arena.alloc_ptr::<u8>().as_ptr() as usize;
        assert!(get_base(&arena).is_some());
        assert_eq!(first, get_base(&arena).unwrap());

        arena.reset();

        // resetting results in equivalent behavior.
        for _ in 0..2 {
            // first alloc is base.
            let first = arena.alloc_ptr::<u8>().as_ptr() as usize;
            assert_eq!(first, get_base(&arena).unwrap());

            // zst does nothing.
            arena.alloc_new([(); 1_000_000]);

            // next alloc is after first alloc (rounded up to MAX_ALIGN).
            let second = arena.alloc_ptr::<u8>().as_ptr() as usize;
            assert_eq!(second, first + MAX_ALIGN);

            // bigger than max align.
            let third = arena.alloc_ptr::<[u64; 2]>().as_ptr() as usize;
            assert_eq!(third, second + MAX_ALIGN);

            // alloc_new sets value.
            let foo = arena.alloc_new(Foo { name: "bar", age: 42 });
            assert!(foo.name == "bar");
            assert!(foo.age  == 42);

            let foo = foo as *mut Foo as usize;
            assert_eq!(foo, third + size_of::<[u64; 2]>());

            // reset
            arena.reset();
        }

        // arena is reset.
        let first = arena.alloc_ptr::<u8>().as_ptr() as usize;
        assert_eq!(first, get_base(&arena).unwrap());
    }

    #[test]
    fn arena_max_align() {
        let arena = Arena::new();

        let failed = arena.alloc(Layout::from_size_align(1, 2*MAX_ALIGN).unwrap());
        assert!(failed.is_none());

        let succeeded = arena.alloc(Layout::from_size_align(1, MAX_ALIGN).unwrap());
        assert!(succeeded.is_some());
    }
}

