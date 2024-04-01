pub use impel::{RefCellUnck, Ref, RefMut};


#[cfg(debug_assertions)]
mod impel {
    use crate::cell::RefCell;
    pub use crate::cell::{Ref, RefMut};


    pub struct RefCellUnck<T: ?Sized> {
        inner: RefCell<T>,
    }

    impl<T> RefCellUnck<T> {
        #[inline(always)]
        pub fn new(value: T) -> Self {
            Self { inner: RefCell::new(value) }
        }
    }

    impl<T: ?Sized> RefCellUnck<T> {
        #[track_caller]
        #[inline(always)]
        pub unsafe fn borrow(&self) -> Ref<T> {
            self.inner.borrow()
        }

        #[track_caller]
        #[inline(always)]
        pub unsafe fn borrow_mut(&self) -> RefMut<T> {
            self.inner.borrow_mut()
        }
    }

}


#[cfg(not(debug_assertions))]
mod impel {
    use core::ptr::NonNull;
    use core::cell::UnsafeCell;
    pub use crate::unck::borrow::{Ref, RefMut};


    pub struct RefCellUnck<T: ?Sized> {
        inner: UnsafeCell<T>,
    }

    crate::static_assert_eq!(core::mem::size_of::<RefCellUnck<i32>>(), core::mem::size_of::<i32>());

    impl<T> RefCellUnck<T> {
        #[inline(always)]
        pub fn new(value: T) -> Self {
            Self { inner: UnsafeCell::new(value) }
        }
    }

    impl<T: ?Sized> RefCellUnck<T> {
        #[track_caller]
        #[inline(always)]
        pub unsafe fn borrow(&self) -> Ref<T> {
            unsafe { Ref::new(NonNull::new_unchecked(self.inner.get())) }
        }

        #[track_caller]
        #[inline(always)]
        pub unsafe fn borrow_mut(&self) -> RefMut<T> {
            unsafe { RefMut::new(NonNull::new_unchecked(self.inner.get())) }
        }
    }

}

#[cfg(all(test, debug_assertions))]
mod debug_tests {
    use super::*;

    #[should_panic(expected = "already mutably borrowed")]
    #[test]
    fn already_mutably_borrowed() { unsafe {
        let r = RefCellUnck::new(42);
        let _m = r.borrow_mut();
        let _s1 = r.borrow();
    }}

    #[should_panic(expected = "already borrowed")]
    #[test]
    fn already_borrowed() { unsafe {
        let r = RefCellUnck::new(42);
        let _s1 = r.borrow();
        let _m = r.borrow_mut();
    }}
}

#[cfg(all(test, not(debug_assertions)))]
mod unck_tests {
    use super::*;

    #[test]
    fn size() {
        use core::mem::size_of;
        assert_eq!(size_of::<RefCellUnck<i32>>(), size_of::<i32>());
    }

    #[test]
    fn already_mutably_borrowed() { unsafe {
        let r = RefCellUnck::new(42);
        let mut m = r.borrow_mut();
        assert_eq!(*m, 42);
        *m = 69;
        // technically valid due to the use of `NonNull`,
        // but UB via the contract of `RefCellUnck`
        let s1 = r.borrow();
        assert_eq!(*m, 69);
        assert_eq!(*s1, 69);
    }}

    #[test]
    fn already_borrowed() { unsafe {
        let r = RefCellUnck::new(42);
        let s1 = r.borrow();
        assert_eq!(*s1, 42);
        // technically valid due to the use of `NonNull`,
        // but UB via the contract of `RefCellUnck`
        let mut m = r.borrow_mut();
        *m = 69;
        assert_eq!(*s1, 69);
        assert_eq!(*m, 69);
    }}
}

