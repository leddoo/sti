use core::ptr::NonNull;
use core::alloc::Layout;
use core::cell::Cell;
use core::mem::{size_of, align_of};

use crate::static_assert;
use crate::num::{is_pow2, ceil_to_multiple_pow2, OrdUtils};
use crate::alloc::{Alloc, GlobalAlloc};


/// maximum allocation size.
///
/// - larger allocations fail.
/// - it's kinda big tho.
///
pub const MAX_ALLOC_SIZE: usize = MAX_CAP - HEADER_SIZE;

/// maximum alignment.
///
/// - allocations with greater alignment fail.
///
pub const MAX_ALIGN: usize = 32;
static_assert!(is_pow2(MAX_ALIGN));


/// default minimum block size.
pub const DEFAULT_CAP: usize = 4096;

/// maximum block size.
pub const MAX_CAP: usize = usize::MAX/4 + 1;



/// Arena
///
/// - a memory arena, aka bump allocator, aka linear allocator.
/// - allocates blocks on demand from a backing allocator.
/// - blocks grow geometrically, but are bounded
///   by the configurable fields `min/max_block_size`.
/// - note: some space in each block may be reserved for metadata.
/// - see `MAX_ALLOC_SIZE` and `MAX_ALIGN`.
///
pub struct Arena<A: Alloc = GlobalAlloc> {
    alloc: A,

    // - block != NonNull::dangling() || cap == 0
    // - block % MAX_ALIGN == 0       || cap == 0
    block: Cell<NonNull<u8>>,

    // - cap <= MAX_CAP
    // - cap >= size_of::<BlockHeader>() || cap == 0
    cap: Cell<usize>,

    // - used <= self.cap
    // - self.used >= size_of::<BlockHeader>() || cap == 0
    used: Cell<usize>,

    pub min_block_size: Cell<usize>,
    pub max_block_size: Cell<usize>,
}

impl<A: Alloc> Arena<A> {
    fn _integrity_check(&self) {
        let cap = self.cap.get();
        assert!(cap <= MAX_CAP);
        assert!(cap >= size_of::<BlockHeader>() || cap == 0);

        let block = self.block.get();
        assert!(block != NonNull::dangling() || cap == 0);
        assert!(block.as_ptr() as usize % MAX_ALIGN == 0 || cap == 0);

        let used = self.used.get();
        assert!(used <= cap);
        assert!(used >= size_of::<BlockHeader>() || cap == 0);
    }

    #[inline(always)]
    fn debug_integrity_check(&self) {
        #[cfg(debug_assertions)]
        self._integrity_check();
    }
}


struct BlockHeader {
    prev:      NonNull<u8>,
    prev_cap:  usize,
}

static_assert!(size_of::<BlockHeader>()  <= HEADER_SIZE);
static_assert!(align_of::<BlockHeader>() <= MAX_ALIGN);

const HEADER_SIZE: usize = (size_of::<BlockHeader>() + MAX_ALIGN - 1) / MAX_ALIGN * MAX_ALIGN;


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
            min_block_size: DEFAULT_CAP.into(),
            max_block_size: MAX_CAP.into(),
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
    /// - `layout.align() <= MAX_ALIGN`
    #[cold]
    unsafe fn alloc_slow_path(&self, layout: Layout) -> Option<NonNull<u8>> {
        assert!(layout.align() <= MAX_ALIGN);

        if layout.size() > MAX_ALLOC_SIZE {
            return None;
        }

        let new_cap = {
            // can't overflow cause `cap <= MAX_CAP`.
            let new_cap = 2*self.cap.get();

            // can't overflow cause `layout.size() <= MAX_ALLOC_SIZE`.
            let new_cap = new_cap.at_least(HEADER_SIZE + layout.size());

            let new_cap = new_cap.at_least(self.min_block_size.get());
            let new_cap = new_cap.at_most(self.max_block_size.get());
            let new_cap = new_cap.at_most(MAX_CAP);
            new_cap
        };

        assert!(new_cap <= MAX_CAP);
        assert!(HEADER_SIZE + layout.size() <= new_cap);

        // `MAX_ALIGN` is a power of two.
        // `new_cap <= MAX_CAP <= isize::MAX/2`.
        let block_layout = unsafe { Layout::from_size_align_unchecked(new_cap, MAX_ALIGN) };
        let block = self.alloc.alloc(block_layout)?;

        // save current state.
        unsafe {
            // new_cap has enough space for a block header.
            (block.as_ptr() as *mut BlockHeader).write(BlockHeader {
                prev:     self.block.get(),
                prev_cap: self.cap.get(),
            });
        }

        // make allocation.
        let result = unsafe {
            // new_cap has enough space for the padded header and the allocation.
            // `block + HEADER_SIZE` is MAX_ALIGN aligned (which is `>= layout.align()`).
            let ptr = block.as_ptr().add(HEADER_SIZE);

            // block is `NonNull` and `ptr` is a valid derived pointer.
            NonNull::new_unchecked(ptr)
        };
        assert!(result.as_ptr() as usize % layout.align() == 0);

        // attach new block.
        self.block.set(block);
        self.cap.set(new_cap);
        self.used.set(HEADER_SIZE + layout.size());

        self.debug_integrity_check();

        return Some(result);
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

        let cap  = self.cap.get();
        let used = self.used.get();

        let aligned_used = ceil_to_multiple_pow2(used, layout.align());

        // can't overflow cause `used <= cap <= CAP_MAX`
        // and `size <= isize::MAX`.
        if aligned_used + layout.size() <= cap {
            let result = unsafe {
                let block = self.block.get();

                // `block.add(cap)` is the end of the allocation,
                // and `result + layout.size() <= cap`.
                let ptr = block.as_ptr().add(aligned_used);

                // the resulting pointer is aligned, because
                // `layout.align() <= MAX_ALIGN` and `block`
                // is `MAX_ALIGN` aligned.

                // `block` is always `NonNull`, so any valid derived pointer is also `NonNull`.
                NonNull::new_unchecked(ptr)
            };

            self.used.set(aligned_used + layout.size());

            debug_assert!(result.as_ptr() as usize % layout.align() == 0);
            self.debug_integrity_check();

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
        debug_assert!(old_layout.size() > 0);
        debug_assert!(new_layout.size() > 0);
        debug_assert!(old_layout.align() == new_layout.align());
        self.debug_integrity_check();

        let old_size = old_layout.size();
        let new_size = new_layout.size();

        let block = self.block.get().as_ptr() as usize;
        let block_end = block + self.cap.get();
        let used_end  = block + self.used.get();

        let ptr = ptr.as_ptr() as usize;
        let alloc_end = ptr + old_size;

        let block_rem = block_end - ptr;

        if alloc_end == used_end && new_size <= block_rem {
            self.used.set(self.used.get() - old_size + new_size);

            debug_assert!(self.used.get() <= self.cap.get());
            self.debug_integrity_check();

            Ok(())
        } else { Err(()) }
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

            // next alloc is after first alloc.
            let second = arena.alloc_ptr::<u8>().as_ptr() as usize;
            assert_eq!(second, first + 1);

            let third = arena.alloc_ptr::<[u64; 2]>().as_ptr() as usize;
            assert_eq!(third, second + 7);

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
    fn arena_realloc() {
        let arena = Arena::new();

        let layout_1 = Layout::from_size_align(1, 1).unwrap();
        let ptr = arena.alloc(layout_1).unwrap();

        let layout_2 = Layout::from_size_align(9, 1).unwrap();
        let ok = unsafe { arena.try_realloc(ptr, layout_1, layout_2) };
        assert!(ok.is_ok());

        // can't ever resize to `0`.
        let layout_cant = Layout::from_size_align(0, 1).unwrap();
        let err = unsafe { arena.try_realloc(ptr, layout_2, layout_cant) };
        assert!(err.is_err());

        let layout_3 = Layout::from_size_align(5, 1).unwrap();
        let ok = unsafe { arena.try_realloc(ptr, layout_2, layout_3) };
        assert!(ok.is_ok());

        let ok = unsafe { arena.try_realloc(ptr, layout_3, layout_3) };
        assert!(ok.is_ok());

        // new alloc.
        let new_ptr = arena.alloc(Layout::from_size_align(2, 2).unwrap()).unwrap();
        assert!(new_ptr.as_ptr() as usize == ptr.as_ptr() as usize + 6);

        let layout_cant = Layout::from_size_align(5, 1).unwrap();
        let err = unsafe { arena.try_realloc(ptr, layout_3, layout_cant) };
        assert!(err.is_err());

        let layout_cant = Layout::from_size_align(0, 1).unwrap();
        let err = unsafe { arena.try_realloc(ptr, layout_3, layout_cant) };
        assert!(err.is_err());

        let layout_cant = Layout::from_size_align(1, 1).unwrap();
        let err = unsafe { arena.try_realloc(ptr, layout_3, layout_cant) };
        assert!(err.is_err());
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

