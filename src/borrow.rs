use crate::mem::{Cell, NonNull, PhantomData};


pub struct BorrowFlag {
    value: Cell<i32>,
}

impl BorrowFlag {
    #[inline(always)]
    pub fn new() -> Self {
        Self { value: Cell::new(0) }
    }

    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        self.value.get() == 0
    }

    #[inline(always)]
    pub fn is_borrowed(&self) -> bool {
        self.value.get() != 0
    }

    #[inline(always)]
    pub fn is_reading(&self) -> bool {
        self.value.get() > 0
    }

    #[inline(always)]
    pub fn is_writing(&self) -> bool {
        self.value.get() < 0
    }


    #[inline]
    pub fn try_borrow(&self) -> Option<BorrowRef> {
        if !self.is_writing() {
            self.value.set(self.value.get().checked_add(1).expect("too many borrows"));
            Some(BorrowRef { flag: self })
        }
        else { None }
    }

    #[track_caller]
    #[inline]
    pub fn borrow(&self) -> BorrowRef {
        self.try_borrow().expect("already mutably borrowed")
    }


    #[inline]
    pub fn try_borrow_mut(&self) -> Option<BorrowRefMut> {
        if self.is_idle() {
            self.value.set(-1);
            Some(BorrowRefMut { flag: self })
        }
        else { None }
    }

    #[track_caller]
    #[inline]
    pub fn borrow_mut(&self) -> BorrowRefMut {
        self.try_borrow_mut().expect("already borrowed")
    }
}


pub struct BorrowRef<'a> {
    flag: &'a BorrowFlag,
}

impl<'a> BorrowRef<'a> {
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
    #[inline]
    pub unsafe fn new(borrow: BorrowRef<'a>, value: NonNull<T>) -> Self {
        Self { value, borrow, phantom: PhantomData }
    }
}

impl<'a, T: ?Sized> Clone for Ref<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new(BorrowRef::clone(&self.borrow), self.value) }
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
    _borrow: BorrowRefMut<'a>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized> RefMut<'a, T> {
    #[inline]
    pub unsafe fn new(borrow: BorrowRefMut<'a>, value: NonNull<T>) -> Self {
        Self { value, _borrow: borrow, phantom: PhantomData }
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


