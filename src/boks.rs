use core::ptr::NonNull;
use core::mem::ManuallyDrop;

use crate::alloc::{Alloc, GlobalAlloc, alloc_new, drop_and_free};


pub struct Box<T: ?Sized, A: Alloc = GlobalAlloc> {
    value: NonNull<T>,
    alloc: A,
}

impl<T> Box<T, GlobalAlloc> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Box::new_in(GlobalAlloc, value)
    }
}

impl<T, A: Alloc> Box<T, A> {
    #[track_caller]
    #[inline(always)]
    pub fn new_in(alloc: A, value: T) -> Self {
        let value = alloc_new(&alloc, value).expect("oom");
        Self { value, alloc }
    }
}

impl<T: ?Sized, A: Alloc> Box<T, A> {
    #[inline(always)]
    pub fn inner(&self) -> NonNull<T> { self.value }

    #[inline(always)]
    pub fn into_raw_parts(self) -> (NonNull<T>, A) {
        let this = ManuallyDrop::new(self);
        let alloc = unsafe { core::ptr::read(&this.alloc) };
        (this.value, alloc)
    }

    /// #safety:
    /// - `value` must be a live allocation of a `T` in `alloc`.
    /// - in particular, `Layout::for_value(value.as_ref())`
    ///   must be the active layout.
    #[inline(always)]
    pub unsafe fn from_raw_parts(value: NonNull<T>, alloc: A) -> Self {
        Self { value, alloc }
    }
}

impl<T: ?Sized, A: Alloc> core::ops::Deref for Box<T, A> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.as_ptr() }
    }
}

impl<T: ?Sized, A: Alloc> core::ops::DerefMut for Box<T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value.as_ptr() }
    }
}

impl<T: ?Sized, A: Alloc> Drop for Box<T, A> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { drop_and_free(&self.alloc, self.value) }
    }
}

