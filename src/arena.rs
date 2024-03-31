use core::ptr::NonNull;
use core::alloc::Layout;
use core::cell::Cell;
use core::mem::{size_of, align_of};

use crate::static_assert;
use crate::num::{is_pow2, ceil_to_multiple_pow2, OrdUtils};
use crate::alloc::{Alloc, GlobalAlloc, alloc_ptr, alloc_new, alloc_array};


/// maximum allocation size.
/// - larger allocations fail.
/// - it's kinda big tho.
pub const MAX_ALLOC_SIZE: usize = MAX_CAP - HEADER_SIZE;

/// maximum alignment.
/// - allocations with greater alignment fail.
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

    // - block != NonNull::dangling() iff cap != 0
    // - block % MAX_ALIGN == 0       iff cap != 0
    block: Cell<NonNull<BlockHeader>>,

    // - cap <= MAX_CAP
    // - cap >= size_of::<BlockHeader>() || cap == 0
    cap: Cell<usize>,

    // - used <= self.cap
    // - self.used >= size_of::<BlockHeader>() || cap == 0
    used: Cell<usize>,

    pub min_block_size: Cell<usize>,
    pub max_block_size: Cell<usize>,
}

unsafe impl<A: Alloc + Send> Send for Arena<A> {}

// @todo: verify this is also true for panics raised in the impl.
impl<A: Alloc> core::panic::RefUnwindSafe for Arena<A> {}
impl<A: Alloc> core::panic::UnwindSafe for Arena<A> {}


impl<A: Alloc> Arena<A> {
    fn _integrity_check(&self) {
        let cap = self.cap.get();
        assert!(cap <= MAX_CAP);
        assert!(cap >= size_of::<BlockHeader>() || cap == 0);

        let block = self.block.get();
        assert!((block != NonNull::dangling()) == (cap != 0));
        assert!((block.as_ptr() as usize % MAX_ALIGN == 0) == (cap != 0));

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
    prev:      NonNull<BlockHeader>,
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
            block: Cell::new(NonNull::dangling()),
            cap:  0.into(),
            used: 0.into(),
            min_block_size: DEFAULT_CAP.into(),
            max_block_size: usize::MAX.into(),
        }
    }


    #[inline]
    pub fn alloc_ptr<T>(&self) -> NonNull<T> {
        alloc_ptr::<T, Self>(self).unwrap()
    }

    #[inline]
    pub fn alloc_new<T>(&self, value: T) -> &mut T {
        unsafe { alloc_new(self, value).unwrap().as_mut() }
    }

    #[inline]
    pub fn alloc_str<'a>(&'a self, value: &str) -> &'a str {
        unsafe {
            let bytes = alloc_array(self, value.len()).unwrap();
            core::ptr::copy_nonoverlapping(value.as_ptr(), bytes.as_ptr(), value.len());
            core::str::from_utf8_unchecked(
                core::slice::from_raw_parts(bytes.as_ptr(), value.len()))
        }
    }


    // pred: (block, cap).
    #[inline]
    fn reset_until<F: Fn(NonNull<BlockHeader>, usize) -> bool>(&self, f: F) {
        self.debug_integrity_check();

        if self.cap.get() == 0 {
            return;
        }

        // `free` can panic.
        self.used.set(HEADER_SIZE);

        loop {
            let block = self.block.get();
            let cap   = self.cap.get();

            if cap == 0 {
                self.used.set(0);
                break;
            }

            if f(block, cap) {
                break;
            }

            unsafe {
                let header = block.as_ptr().read();

                self.alloc.free(block.cast(),
                    Layout::from_size_align_unchecked(cap, MAX_ALIGN));

                // `free` can panic.
                self.block.set(header.prev);
                self.cap.set(header.prev_cap);
            }
        }

        self.debug_integrity_check();
    }

    /// reset the arena.
    /// - frees all allocations made on this arena.
    /// - internally frees all but the first block.
    pub fn reset(&mut self) {
        self.reset_until(|block, _| {
            unsafe { block.as_ptr().read().prev_cap == 0 }
        });
    }

    /// reset the arena.
    /// - frees all allocations made on this arena.
    /// - internally frees all blocks.
    pub fn reset_all(&mut self) {
        self.reset_until(|_, _| false);
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

            // clamp to user prefs.
            let new_cap = new_cap.at_least(self.min_block_size.get());
            let new_cap = new_cap.at_most(self.max_block_size.get());

            // can't overflow cause `layout.size() <= MAX_ALLOC_SIZE`.
            let new_cap = new_cap.at_least(HEADER_SIZE + layout.size());

            let new_cap = new_cap.at_most(MAX_CAP);
            new_cap
        };

        assert!(new_cap <= MAX_CAP);
        assert!(HEADER_SIZE + layout.size() <= new_cap);

        // `MAX_ALIGN` is a power of two.
        // `new_cap <= MAX_CAP <= isize::MAX/2`.
        static_assert!(align_of::<BlockHeader>() <= MAX_ALIGN);
        let block_layout = unsafe { Layout::from_size_align_unchecked(new_cap, MAX_ALIGN) };
        let block: NonNull<BlockHeader> = self.alloc.alloc(block_layout)?.cast();

        // save current state.
        unsafe {
            // new_cap has enough space for a block header.
            block.as_ptr().write(BlockHeader {
                prev:     self.block.get(),
                prev_cap: self.cap.get(),
            });
        }

        // make allocation.
        let result = unsafe {
            // new_cap has enough space for the padded header and the allocation.
            // `block + HEADER_SIZE` is MAX_ALIGN aligned (which is `>= layout.align()`).
            let ptr = block.as_ptr().cast::<u8>().add(HEADER_SIZE);

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


#[derive(Clone, Copy, Debug)]
pub struct ArenaStats {
    pub total_allocated: usize,
    pub num_blocks: usize,
}

impl<A: Alloc> Arena<A> {
    pub fn stats(&self) -> ArenaStats {
        let mut stats = ArenaStats {
            total_allocated: 0,
            num_blocks: 0,
        };

        let mut block = self.block.get();
        let mut cap   = self.cap.get();
        while cap != 0 {
            stats.total_allocated += cap;
            stats.num_blocks += 1;

            let header = unsafe { block.as_ptr().read() };

            block = header.prev;
            cap   = header.prev_cap;
        }

        stats
    }

    #[inline(always)]
    pub fn current_block_size(&self) -> usize { self.cap.get() }

    #[inline(always)]
    pub fn current_block_used(&self) -> usize { self.used.get() }
}


#[derive(Clone)]
pub struct ArenaSavePoint {
    used_end: usize,  // pointer as usize.
}

impl<A: Alloc> Arena<A> {
    /// save the current arena state.
    ///
    /// - creates a save point that can later be restored.
    /// - includes all allocations made before this call.
    ///
    #[inline(always)]
    pub fn save(&self) -> ArenaSavePoint {
        let block = self.block.get().as_ptr() as usize;
        let used_end = block + self.used.get();
        ArenaSavePoint { used_end }
    }

    /// restore an arena state.
    ///
    /// - frees all allocations made since the save point.
    /// - all allocations made before the save point remain live.
    ///
    /// # safety:
    /// - the save point must be from this arena.
    /// - all allocations made before the save point must still be live.
    ///   this means, the arena must not have been reset since the save point.
    ///   and no save point made before this save point may have been restored.
    ///
    pub unsafe fn restore(&self, save: ArenaSavePoint) {
        self.reset_until(|block, cap| {
            let block = block.as_ptr() as usize;

            // `used_end > block` to handle contiguous blocks.
            // valid, because `used >= HEADER_SIZE` for non-empty blocks.
            // and this predicate isn't called for empty blocks.
               save.used_end >  block
            && save.used_end <= block + cap
        });

        let block = self.block.get().as_ptr() as usize;

        // detect incorrect usage.
        if self.cap.get() > 0 {
            debug_assert!(
                   save.used_end >= block + HEADER_SIZE
                && save.used_end <= block + self.cap.get());
        }
        else { assert_eq!(save.used_end, block) }

        self.used.set(save.used_end - block);

        self.debug_integrity_check();
    }
}


impl<A: Alloc> Drop for Arena<A> {
    fn drop(&mut self) {
        self.reset_all();
    }
}


// safe: `Arena` is not `Clone`.
unsafe impl<A: Alloc> Alloc for Arena<A> {
    /// # safety:
    /// - `layout.size() > 0`.
    #[inline]
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
                let ptr = block.as_ptr().cast::<u8>().add(aligned_used);

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
        // no-op.
        let _ = (ptr, layout);
    }

    #[inline(always)]
    unsafe fn try_realloc_nonzero(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        debug_assert!(old_layout.size() > 0);
        debug_assert!(new_layout.size() > 0);
        debug_assert!(old_layout.align() == new_layout.align());
        self.debug_integrity_check();

        let old_size = old_layout.size();
        let new_size = new_layout.size();

        let ptr = ptr.as_ptr() as usize;
        let block = self.block.get().as_ptr() as usize;

        let alloc_end = ptr + old_size;
        let used_end  = block + self.used.get();
        if alloc_end != used_end {
            return Err(())
        }

        let block_end = block + self.cap.get();
        let block_rem = block_end - ptr;
        if new_size > block_rem {
            return Err(())
        }

        self.used.set(self.used.get() - old_size + new_size);

        debug_assert!(self.used.get() <= self.cap.get());
        self.debug_integrity_check();

        return Ok(());
    }
}


impl core::fmt::Debug for Arena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.stats().fmt(f)
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

        let block_size = 500;
        arena.max_block_size.set(block_size);

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
            assert_eq!(arena.stats().total_allocated, block_size);
            assert_eq!(arena.stats().num_blocks, 1);

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

            let hi = arena.alloc_str("hi");
            assert_eq!(hi, "hi");
            assert_eq!(hi.as_ptr() as usize, foo + size_of::<Foo>());

            let last = arena.alloc_ptr::<u8>().as_ptr() as usize;
            assert_eq!(last, hi.as_ptr() as usize + hi.len());

            // reset
            arena.reset();
        }

        // arena is reset.
        let first = arena.alloc_ptr::<u8>().as_ptr() as usize;
        assert_eq!(first, get_base(&arena).unwrap());
        assert_eq!(arena.stats().total_allocated, block_size);
        assert_eq!(arena.stats().num_blocks, 1);
    }

    #[test]
    fn arena_block_size() {
        let mut arena = Arena::new();

        assert_eq!(arena.stats().total_allocated, 0);
        assert_eq!(arena.stats().num_blocks, 0);
        assert_eq!(arena.current_block_used(), 0);

        arena.max_block_size.set(1);

        arena.alloc_ptr::<u8>();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 1);

        arena.reset();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE);

        arena.alloc_ptr::<u16>();

        assert_eq!(arena.stats().total_allocated, 2*HEADER_SIZE + 1 + 2);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 2);

        arena.reset();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE);

        arena.max_block_size.set(64);

        arena.alloc_ptr::<u8>();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 1);

        arena.alloc_ptr::<u8>();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1 + 64);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 1);

        arena.alloc_ptr::<u8>();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1 + 64);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 2);

        arena.alloc_ptr::<[u8; 64 - HEADER_SIZE - 2]>();

        assert_eq!(arena.stats().total_allocated, HEADER_SIZE + 1 + 64);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), 64);

        arena.reset_all();

        assert_eq!(arena.stats().total_allocated, 0);
        assert_eq!(arena.stats().num_blocks, 0);
        assert_eq!(arena.current_block_used(), 0);
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
    fn arena_realloc_edge_cases() {
        let backing = Arena::new();
        backing.min_block_size.set(1024);

        let arena = Arena::new();
        arena.min_block_size.set(128);
        arena.max_block_size.set(128);

        let layout_1 = Layout::from_size_align(1, 1).unwrap(); // must be 1,1.
        let layout_8 = Layout::from_size_align(8, 1).unwrap();

        // no block.
        assert_eq!(arena.stats().total_allocated, 0);
        let err = unsafe { arena.try_realloc(NonNull::new(&mut 1u8).unwrap(), layout_1, layout_8) };
        assert!(err.is_err());

        let before = backing.alloc_ptr::<[u8; 128]>().cast::<u8>();

        // with block.
        let ptr = arena.alloc_ptr::<u8>();
        let stats = arena.stats();
        assert_eq!(stats.total_allocated, 128);
        assert_eq!(stats.num_blocks, 1);

        let after = backing.alloc_ptr::<[u8; 128]>().cast::<u8>();
        assert_eq!(after.as_ptr() as usize - before.as_ptr() as usize, 128);

        // out of bounds pointers.
        let err = unsafe { arena.try_realloc(before, layout_1, layout_8) };
        assert!(err.is_err());
        let err = unsafe { arena.try_realloc(after, layout_1, layout_8) };
        assert!(err.is_err());
        let err = unsafe { arena.try_realloc(NonNull::new(after.as_ptr().add(128)).unwrap(), layout_1, layout_8) };
        assert!(err.is_err());

        // can't resize beyond end of block.
        let layout_too_full = Layout::from_size_align(128 - HEADER_SIZE + 1, 1).unwrap();
        let err = unsafe { arena.try_realloc(ptr, layout_1, layout_too_full) };
        assert!(err.is_err());

        // can resize to end of block.
        let layout_full = Layout::from_size_align(128 - HEADER_SIZE, 1).unwrap();
        let ok = unsafe { arena.try_realloc(ptr, layout_1, layout_full) };
        assert!(ok.is_ok());
    }

    #[test]
    fn arena_save_restore() {
        let arena = Arena::new();
        arena.min_block_size.set(64);
        arena.max_block_size.set(64);

        let save_0 = arena.save();
        assert_eq!(save_0.used_end, align_of::<BlockHeader>());

        unsafe { arena.restore(save_0.clone()) };
        assert_eq!(arena.stats().total_allocated, 0);
        assert_eq!(arena.stats().num_blocks, 0);
        assert_eq!(arena.current_block_used(), 0);

        let a = arena.alloc_ptr::<[u8; 12]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 12);

        // same block save.
        let save_1 = arena.save();
        assert_eq!(save_1.used_end, a + 12);

        // restore immediately.
        unsafe { arena.restore(save_1.clone()) }
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 12);

        arena.alloc_ptr::<u32>();
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 12 + 4);

        // restore same block.
        unsafe { arena.restore(save_1.clone()) }
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 12);

        // allocate rest of block.
        arena.alloc_ptr::<[u8; 64 - HEADER_SIZE - 12]>();
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), 64);

        // force another block.
        let b = arena.alloc_ptr::<[u8; 8]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 128);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 8);

        let save_2 = arena.save();
        assert_eq!(save_2.used_end, b + 8);

        // and an oversided block.
        let c = arena.alloc_ptr::<[u8; 128]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 128 + HEADER_SIZE+128);
        assert_eq!(arena.stats().num_blocks, 3);
        assert_eq!(arena.current_block_used(), HEADER_SIZE+128);

        let save_3 = arena.save();
        assert_eq!(save_3.used_end, c + 128);

        // and another normal block.
        let d = arena.alloc_ptr::<[u8; 8]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 128 + HEADER_SIZE+128 + 64);
        assert_eq!(arena.stats().num_blocks, 4);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 8);

        let save_4 = arena.save();
        assert_eq!(save_4.used_end, d + 8);

        // restore skipping 2 blocks.
        unsafe { arena.restore(save_2) }
        assert_eq!(arena.stats().total_allocated, 128);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 8);

        // restore save_1 again.
        unsafe { arena.restore(save_1) }
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 12);

        // restore save_0 again.
        unsafe { arena.restore(save_0) }
        assert_eq!(arena.stats().total_allocated, 0);
        assert_eq!(arena.stats().num_blocks, 0);
        assert_eq!(arena.current_block_used(), 0);
    }

    #[test]
    fn arena_save_restore_contiguous() {
        let backing = Arena::new();

        let arena = Arena::new_in(&backing);
        arena.min_block_size.set(64);
        arena.max_block_size.set(64);

        arena.alloc_ptr::<[u8; 64-HEADER_SIZE]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), 64);

        let save = arena.save();

        arena.alloc_ptr::<[u8; 8]>().as_ptr() as usize;
        assert_eq!(arena.stats().total_allocated, 128);
        assert_eq!(arena.stats().num_blocks, 2);
        assert_eq!(arena.current_block_used(), HEADER_SIZE + 8);

        // check blocks contiguous.
        assert_eq!(save.used_end, arena.block.get().as_ptr() as usize);

        // correctly frees current block.
        unsafe { arena.restore(save) }
        assert_eq!(arena.stats().total_allocated, 64);
        assert_eq!(arena.stats().num_blocks, 1);
        assert_eq!(arena.current_block_used(), 64);
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

