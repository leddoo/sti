use core::cell::Cell;
use core::ptr::NonNull;
use core::marker::PhantomData;


pub struct BorrowFlag {
    value: Cell<i32>,
}

impl BorrowFlag {
    #[inline]
    pub fn new() -> Self {
        Self { value: Cell::new(0) }
    }

    #[inline]
    pub fn is_idle(&self) -> bool {
        self.value.get() == 0
    }

    #[inline]
    pub fn is_reading(&self) -> bool {
        self.value.get() > 0
    }

    #[inline]
    pub fn is_writing(&self) -> bool {
        self.value.get() < 0
    }
}


pub struct BorrowRef<'a> {
    flag: &'a BorrowFlag,
}

impl<'a> BorrowRef<'a> {
    #[track_caller]
    #[inline]
    pub fn new(flag: &'a BorrowFlag) -> Option<Self> {
        if !flag.is_writing() {
            flag.value.set(flag.value.get().checked_add(1).expect("too many borrows"));
            Some(Self { flag })
        }
        else { None }
    }

    #[track_caller]
    #[inline]
    pub fn clone(this: &Self) -> Self {
        debug_assert!(this.flag.is_reading());
        let flag = this.flag;
        flag.value.set(flag.value.get().checked_add(1).expect("too many borrows"));
        Self { flag }
    }
}

impl<'a> Drop for BorrowRef<'a> {
    #[inline]
    fn drop(&mut self) {
        debug_assert!(self.flag.is_reading());
        self.flag.value.set(self.flag.value.get() - 1);
    }
}


pub struct BorrowRefMut<'a> {
    flag: &'a BorrowFlag,
}

impl<'a> BorrowRefMut<'a> {
    #[inline]
    pub fn new(flag: &'a BorrowFlag) -> Option<Self> {
        if flag.is_idle() {
            flag.value.set(-1);
            Some(Self { flag })
        }
        else { None }
    }

    #[track_caller]
    #[inline]
    pub fn clone(this: &Self) -> Self {
        debug_assert!(this.flag.is_writing());
        let flag = this.flag;
        flag.value.set(flag.value.get().checked_sub(1).expect("too many borrows"));
        Self { flag }
    }
}

impl<'a> Drop for BorrowRefMut<'a> {
    #[inline]
    fn drop(&mut self) {
        debug_assert!(self.flag.is_writing());
        self.flag.value.set(self.flag.value.get() + 1);
    }
}


pub struct Ref<'a, T: ?Sized> {
    // from rust's impl:
    // > NB: we use a pointer instead of `&'b mut T` to avoid `noalias` violations, because a
    //   `RefMut` argument doesn't hold exclusivity for its whole scope, only until it drops.
    value: NonNull<T>,
    borrow: BorrowRef<'a>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: ?Sized> Ref<'a, T> {
    #[inline(always)]
    pub unsafe fn new(value: NonNull<T>, borrow: BorrowRef<'a>) -> Self {
        Self { value, borrow, phantom: PhantomData }
    }
}

impl<'a, T: ?Sized> Clone for Ref<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new(self.value, BorrowRef::clone(&self.borrow)) }
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
    borrow: BorrowRefMut<'a>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized> RefMut<'a, T> {
    #[inline(always)]
    pub unsafe fn new(value: NonNull<T>, borrow: BorrowRefMut<'a>) -> Self {
        Self { value, borrow, phantom: PhantomData }
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


