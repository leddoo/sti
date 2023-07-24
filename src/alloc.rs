use core::ptr::NonNull;
use crate::num::ceil_to_multiple_pow2;

pub use core::alloc::Layout;


pub trait Alloc {
    /// allocates a block of memory.
    ///
    /// - if the call succeeds:
    ///     - the returned pointer refers to a live allocation.
    ///     - `layout` is the active layout of the returned memory block.
    ///
    #[inline(always)]
    fn alloc(&self, layout: Layout) -> Option<NonNull<u8>> {
        if layout.size() != 0 {
            // layout.size() > 0.
            let result = unsafe { self.alloc_nonzero(layout) };

            if let Some(result) = result {
                debug_assert!(result.as_ptr() as usize % layout.align() == 0);
            }

            return result;
        }
        else {
            return Some(dangling(layout));
        }
    }

    /// allocates a block of memory.
    ///
    /// - if the call succeeds:
    ///     - the returned pointer refers to a live allocation.
    ///     - `layout` is the active layout of the returned memory block.
    ///
    /// # safety:
    /// - `layout.size() > 0`
    ///
    unsafe fn alloc_nonzero(&self, layout: Layout) -> Option<NonNull<u8>>;


    /// frees an allocation.
    ///
    /// - after the call, `ptr` is no longer a live allocation.
    ///
    /// # safety:
    /// - if `layout.size() > 0`:
    ///     - `ptr` must be a live allocation, allocated from this allocator.
    ///     - `layout` must be the active layout of the memory block.
    ///
    #[inline(always)]
    unsafe fn free(&self, ptr: NonNull<u8>, layout: Layout) {
        if layout.size() != 0 {
            // invariants upheld by the caller, because `layout.size() > 0`.
            unsafe { self.free_nonzero(ptr, layout) }
        }
    }


    /// frees an allocation.
    ///
    /// - after the call, `ptr` is no longer a live allocation.
    ///
    /// # safety:
    /// - `ptr` must be a live allocation, allocated from this allocator.
    /// - `layout` must be the active layout of the memory block.
    /// - `layout.size() > 0`.
    ///
    unsafe fn free_nonzero(&self, ptr: NonNull<u8>, layout: Layout);


    /// attempts to resize an allocation.
    ///
    /// - returns whether resizing succeeded.
    /// - the allocation referenced by `ptr` always remains live.
    /// - if the call succeeds, `new_layout` is the active layout,
    ///   otherwise, `old_layout` remains the active layout.
    ///
    /// # safety
    /// - if `old_layout.size() > 0`:
    ///     - `ptr` must be a live allocation, allocated from this allocator.
    ///     - `old_layout` must be the active layout of the memory block.
    /// - `old_layout.align() == new_layout.align()`.
    ///
    #[inline(always)]
    unsafe fn try_realloc(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        if old_layout.size() == 0 || new_layout.size() == 0 {
            return Err(())
        }
        self.try_realloc_nonzero(ptr, old_layout, new_layout)
    }

    /// attempts to resize an allocation.
    ///
    /// - returns whether resizing succeeded.
    /// - the allocation referenced by `ptr` always remains live.
    /// - if the call succeeds, `new_layout` is the active layout,
    ///   otherwise, `old_layout` remains the active layout.
    ///
    /// # safety
    /// - `ptr` must be a live allocation, allocated from this allocator.
    /// - `old_layout` must be the active layout of the memory block.
    /// - `old_layout.align() == new_layout.align()`.
    /// - `old_layout.size() > 0`.
    /// - `new_layout.size() > 0`.
    ///
    #[inline(always)]
    unsafe fn try_realloc_nonzero(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        let _ = (ptr, old_layout, new_layout);
        Err(())
    }

    /// resize an allocation.
    ///
    /// - if the call succeeds:
    ///     - the returned pointer must be used for subsequent `Alloc` method calls.
    ///     - the active layout is `new_layout`.
    ///     - the new memory block's bytes in `0..min(old_layout.size(), new_layout.size())`
    ///       have the old memory block's values, other bytes are undefined.
    /// - if the call fails:
    ///     - `ptr` remains live.
    ///     - `old_layout` remains the active layout.
    ///     - the memory block referenced by `ptr` remains unchanged.
    ///
    /// # safety
    /// - if old_layout.size() > 0:
    ///     - `ptr` must be a live allocation, allocated from this allocator.
    ///     - `old_layout` must be the active layout of the memory block.
    /// - `old_layout.align() == new_layout.align()`.
    ///
    unsafe fn realloc(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
        debug_assert_eq!(old_layout.align(), new_layout.align());

        // invariants upheld by the caller.
        if unsafe { self.try_realloc(ptr, old_layout, new_layout).is_ok() } {
            return Some(ptr);
        }
        else {
            let new_ptr = self.alloc(new_layout);

            if let Some(new_ptr) = new_ptr {
                unsafe {
                    let min_size = old_layout.size().min(new_layout.size());
                    if min_size > 0 {
                        // both allocations are live and at least `min_size` large.
                        // live allocations can't overlap.
                        core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), min_size);
                    }

                    // invariants upheld by the caller.
                    self.free(ptr, old_layout);
                }
            }
            // else: failed. keep old allocation live.

            return new_ptr;
        }
    }
}


#[inline(always)]
pub fn dangling(layout: Layout) -> NonNull<u8> {
    // this is kinda sketchy.
    // taken from the unstable impl of `Layout::dangling`
    unsafe { NonNull::new_unchecked(core::mem::transmute(layout.align())) }
}

#[inline(always)]
pub fn cat_join(a: Layout, b: Layout) -> Option<Layout> {
    let b_begin = ceil_to_multiple_pow2(a.size(), b.align());

    let new_size = b_begin.checked_add(b.size())?;
    let new_align = a.align().max(b.align());
    Layout::from_size_align(new_size, new_align).ok()
}

#[inline(always)]
pub unsafe fn cat_next<T, U>(base: *const T, base_size: usize) -> *const U {
    unsafe { cat_next_ex(base, base_size, core::mem::align_of::<U>()) }
}

#[inline(always)]
pub unsafe fn cat_next_ex<T, U>(base: *const T, base_size: usize, next_align: usize) -> *const U {
    let result = ceil_to_multiple_pow2(base as usize + base_size, next_align);
    #[cfg(miri)] {
        // miri doesn't like int->ptr casts.
        let delta = result - base as usize;
        return (base as *const u8).add(delta) as *const U;
    }
    #[cfg(not(miri))] {
        return result as *const U;
    }
}

#[inline(always)]
pub unsafe fn cat_next_mut<T, U>(base: *mut T, base_size: usize) -> *mut U {
    unsafe { cat_next_mut_ex(base, base_size, core::mem::align_of::<U>()) }
}

#[inline(always)]
pub unsafe fn cat_next_mut_ex<T, U>(base: *mut T, base_size: usize, next_align: usize) -> *mut U {
    let result = ceil_to_multiple_pow2(base as usize + base_size, next_align);
    #[cfg(miri)] {
        // miri doesn't like int->ptr casts.
        let delta = result - base as usize;
        return (base as *mut u8).add(delta) as *mut U;
    }
    #[cfg(not(miri))] {
        return result as *mut U;
    }
}


#[derive(Copy, Clone, Default, Debug)]
pub struct GlobalAlloc;

impl Alloc for GlobalAlloc {
    /// # safety (same as `Alloc::alloc_impl`).
    /// - `layout.size() > 0`.
    unsafe fn alloc_nonzero(&self, layout: Layout) -> Option<NonNull<u8>> {
        debug_assert!(layout.size() > 0);
        unsafe {
            // `layout.size() > 0`.
            let ptr = std::alloc::alloc(layout);
            NonNull::new(ptr)
        }
    }

    /// # safety (same as `Alloc::free_impl`).
    /// - `ptr` must be a live allocation, allocated from this allocator.
    /// - `layout` must be the active layout of the memory block.
    /// - `layout.size() > 0`.
    unsafe fn free_nonzero(&self, ptr: NonNull<u8>, layout: Layout) {
        debug_assert!(layout.size() > 0);
        // allocation invariants upheld by the caller.
        unsafe { std::alloc::dealloc(ptr.as_ptr(), layout); }
    }

    /// # safety (same as `Alloc::realloc`).
    /// - if old_layout.size() > 0:
    ///     - `ptr` must be a live allocation, allocated from this allocator.
    ///     - `old_layout` must be the active layout of the memory block.
    /// - `old_layout.align() == new_layout.align()`.
    unsafe fn realloc(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
        debug_assert_eq!(old_layout.align(), new_layout.align());

        // and this is why you want the `try_realloc` api.
        if old_layout.size() != 0 {
            if new_layout.size() != 0 {
                // allocation invariants upheld by the caller, because `old_layout.size() > 0`
                // new_size is non-zero. `Layout` guarantees rounding requirements.
                let result = unsafe { std::alloc::realloc(ptr.as_ptr(), old_layout, new_layout.size()) };
                NonNull::new(result)
            }
            else {
                // invariants upheld by the caller, because `old_layout.size() > 0`.
                unsafe { std::alloc::dealloc(ptr.as_ptr(), old_layout) };
                return Some(dangling(new_layout));
            }
        }
        else {
            return self.alloc(new_layout);
        }
    }
}


impl<A: Alloc + ?Sized> Alloc for &A {
    #[inline(always)]
    fn alloc(&self, layout: Layout) -> Option<NonNull<u8>> {
        (**self).alloc(layout)
    }

    #[inline(always)]
    unsafe fn alloc_nonzero(&self, layout: Layout) -> Option<NonNull<u8>> {
        (**self).alloc_nonzero(layout)
    }


    #[inline(always)]
    unsafe fn free(&self, ptr: NonNull<u8>, layout: Layout) {
        (**self).free(ptr, layout)
    }

    #[inline(always)]
    unsafe fn free_nonzero(&self, ptr: NonNull<u8>, layout: Layout) {
        (**self).free_nonzero(ptr, layout)
    }


    #[inline(always)]
    unsafe fn try_realloc(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        (**self).try_realloc(ptr, old_layout, new_layout)
    }

    #[inline(always)]
    unsafe fn try_realloc_nonzero(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Result<(), ()> {
        (**self).try_realloc_nonzero(ptr, old_layout, new_layout)
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
        (**self).realloc(ptr, old_layout, new_layout)
    }
}

