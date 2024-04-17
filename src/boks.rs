use core::ptr::NonNull;
use core::mem::ManuallyDrop;

use crate::alloc::{Alloc, GlobalAlloc, alloc_new, alloc_array, drop_and_free};


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

impl<T> Box<[T], GlobalAlloc> {
    #[inline(always)]
    pub fn from_slice(values: &[T]) -> Self  where T: Clone {
        Box::from_slice_in(GlobalAlloc, values)
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

impl<T, A: Alloc> Box<[T], A> {
    #[track_caller]
    #[inline(always)]
    pub fn from_slice_in(alloc: A, values: &[T]) -> Self  where T: Clone {
        let ptr = alloc_array::<T, _>(&alloc, values.len()).expect("oom").as_ptr();
        for i in 0..values.len() {
            unsafe { ptr.add(i).write(values[i].clone()) };
        }
        let value = unsafe { NonNull::from(core::slice::from_raw_parts_mut(ptr, values.len())) };
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
    /// - `value` must be valid at `T`.
    #[inline(always)]
    pub unsafe fn from_raw_parts(value: NonNull<T>, alloc: A) -> Self {
        Self { value, alloc }
    }

    /// - this does not drop the allocator.
    #[inline(always)]
    pub fn leak<'a>(self) -> &'a mut T  where A: 'a {
        unsafe {
            let mut this = core::mem::ManuallyDrop::new(self);
            this.value.as_mut()
        }
    }
}

unsafe impl<T: ?Sized + Sync, A: Alloc + Sync> Sync for Box<T, A> {}
unsafe impl<T: ?Sized + Send, A: Alloc + Send> Send for Box<T, A> {}

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

