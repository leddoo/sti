use core::ptr::NonNull;
use crate::borrow::BorrowFlag;

pub use core::cell::{Cell, UnsafeCell};
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
    pub fn try_borrow(&self) -> Option<Ref<T>> { unsafe {
        Some(Ref::new(self.flag.try_borrow()?, NonNull::new_unchecked(self.value.get())))
    }}

    #[track_caller]
    #[inline(always)]
    pub fn borrow(&self) -> Ref<T> { unsafe {
        Ref::new(self.flag.borrow(), NonNull::new_unchecked(self.value.get()))
    }}


    #[inline]
    pub fn try_borrow_mut(&self) -> Option<RefMut<T>> { unsafe {
        Some(RefMut::new(self.flag.try_borrow_mut()?, NonNull::new_unchecked(self.value.get())))
    }}

    #[track_caller]
    #[inline]
    pub fn borrow_mut(&self) -> RefMut<T> { unsafe {
        RefMut::new(self.flag.borrow_mut(), NonNull::new_unchecked(self.value.get()))
    }}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let r = RefCell::new((1, 2));
        assert!(r.try_borrow().is_some());
        assert!(r.try_borrow_mut().is_some());
        assert_eq!(*r.borrow(), (1, 2));
        assert_eq!(*r.borrow_mut(), (1, 2));

        let v = (r.borrow().1, r.borrow().0 + r.borrow().1);
        *r.borrow_mut() = v;
        assert_eq!(*r.borrow(), (2, 3));
    }

    #[test]
    fn shared_nand_mut() {
        let r = RefCell::new((1, 2));

        let s1 = r.borrow();
        let s2 = r.borrow();
        let s3 = s2.clone();
        assert!(r.try_borrow().is_some());
        assert!(r.try_borrow_mut().is_none());

        drop((s1, s2, s3));
        assert!(r.try_borrow().is_some());
        assert!(r.try_borrow_mut().is_some());

        let m1 = r.borrow_mut();
        assert!(r.try_borrow().is_none());
        assert!(r.try_borrow_mut().is_none());

        drop(m1);
        assert!(r.try_borrow().is_some());
        assert!(r.try_borrow_mut().is_some());
    }
}


