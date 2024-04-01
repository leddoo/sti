use core::cell::UnsafeCell;
use core::ptr::NonNull;

use crate::borrow::{BorrowFlag, BorrowRef, BorrowRefMut};

pub use crate::borrow::{Ref, RefMut};



pub struct RefCell<T: ?Sized> {
    flag: BorrowFlag,
    value: UnsafeCell<T>,
}

impl<T> RefCell<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self {
            flag: BorrowFlag::new(),
            value: UnsafeCell::new(value),
        }
    }
}

impl<T: ?Sized> RefCell<T> {
    #[inline]
    pub fn try_borrow(&self) -> Option<Ref<T>> {
        match BorrowRef::new(&self.flag) {
            Some(borrow) => unsafe {
                let value = NonNull::new_unchecked(self.value.get());
                Some(Ref::new(value, borrow))
            }

            None => None,
        }
    }

    #[track_caller]
    #[inline]
    pub fn borrow(&self) -> Ref<T> {
        self.try_borrow().expect("already mutably borrowed")
    }


    #[inline]
    pub fn try_borrow_mut(&self) -> Option<RefMut<T>> {
        match BorrowRefMut::new(&self.flag) {
            Some(borrow) => unsafe {
                let value = NonNull::new_unchecked(self.value.get());
                Some(RefMut::new(value, borrow))
            }

            None => None,
        }
    }

    #[track_caller]
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.try_borrow_mut().expect("already borrowed")
    }
}


