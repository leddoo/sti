use core::ptr::NonNull;
use core::mem::ManuallyDrop;

use crate::alloc::{Alloc, GlobalAlloc, Layout};


pub struct Box<T: ?Sized, A: Alloc = GlobalAlloc> {
    value: NonNull<T>,
    alloc: A,
}


impl<T> Box<T, GlobalAlloc> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Box::new_in(value, GlobalAlloc)
    }
}

impl<T, A: Alloc> Box<T, A> {
    #[inline(always)]
    pub fn new_in(value: T, alloc: A) -> Self {
        let ptr = alloc.alloc(Layout::new::<T>()).unwrap();
        let ptr = ptr.cast::<T>();
        unsafe { ptr.as_ptr().write(value) }
        Self { value: ptr, alloc }
    }


    #[inline(always)]
    pub fn inner(&self) -> NonNull<T> { self.value }

    #[inline(always)]
    pub fn into_raw_parts(self) -> (NonNull<T>, A) {
        let this = ManuallyDrop::new(self);
        let alloc = unsafe { core::ptr::read(&this.alloc) };
        (this.value, alloc)
    }

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
        unsafe {
            let layout = Layout::for_value::<T>(&**self);
            core::ptr::drop_in_place(self.value.as_ptr());
            self.alloc.free(self.value.cast(), layout);
        }
    }
}

