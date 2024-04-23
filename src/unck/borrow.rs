use crate::mem::{NonNull, PhantomData};


pub struct Ref<'a, T: ?Sized> {
    // from rust's impl:
    // > NB: we use a pointer instead of `&'b mut T` to avoid `noalias` violations, because a
    //   `RefMut` argument doesn't hold exclusivity for its whole scope, only until it drops.
    value: NonNull<T>,
    phantom: PhantomData<&'a T>,
}

crate::static_assert_eq!(core::mem::size_of::<Ref<i32>>(), core::mem::size_of::<*const i32>());

impl<'a, T: ?Sized> Ref<'a, T> {
    #[inline(always)]
    pub unsafe fn new(value: NonNull<T>) -> Self {
        Self { value, phantom: PhantomData }
    }
}

impl<'a, T: ?Sized> Clone for Ref<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new(self.value) }
    }
}

impl<'a, T: ?Sized> core::ops::Deref for Ref<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}


pub struct RefMut<'a, T: ?Sized> {
    // from rust's impl:
    // > NB: we use a pointer instead of `&'b mut T` to avoid `noalias` violations, because a
    //   `RefMut` argument doesn't hold exclusivity for its whole scope, only until it drops.
    value: NonNull<T>,
    phantom: PhantomData<&'a mut T>,
}

crate::static_assert_eq!(core::mem::size_of::<RefMut<i32>>(), core::mem::size_of::<*const i32>());

impl<'a, T: ?Sized> RefMut<'a, T> {
    #[inline(always)]
    pub unsafe fn new(value: NonNull<T>) -> Self {
        Self { value, phantom: PhantomData }
    }
}

impl<'a, T: ?Sized> core::ops::Deref for RefMut<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<'a, T: ?Sized> core::ops::DerefMut for RefMut<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() }
    }
}


