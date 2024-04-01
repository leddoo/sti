/// # safety:
/// - in debug, alloc must live at least as long as any (dropped) pointer
///   to this shared box.
///   when in doubt, Rc the alloc in debug.


#[cfg(debug_assertions)]
pub use impel::{SharedBoxUnck, SharedPtrUnck, Ref, RefMut};


#[cfg(debug_assertions)]
mod impel {
    use core::ptr::NonNull;
    use core::cell::{Cell, UnsafeCell};
    use core::marker::PhantomData;

    use crate::alloc::{alloc_ptr, alloc_new, drop_and_free, Alloc, GlobalAlloc, Layout};
    use crate::borrow::BorrowFlag;
    pub use crate::cell::{Ref, RefMut};


    pub struct SharedBoxUnck<T, A: Alloc = GlobalAlloc> {
        alloc: PhantomData<A>,
        inner: NonNull<Inner<T>>,
    }

    pub struct SharedPtrUnck<T> {
        inner: NonNull<Inner<T>>,
    }

    struct Inner<T> {
        alloc: NonNull<dyn Alloc>, // in global alloc.
        ptrs: Cell<usize>,
        borrow_flag: BorrowFlag,
        value: Option<UnsafeCell<T>>,
    }


    impl<T> SharedBoxUnck<T> {
        pub fn new(value: T) -> Self {
            // GlobalAlloc: 'static
            unsafe { Self::new_in(GlobalAlloc, value) }
        }
    }

    impl<T, A: Alloc> SharedBoxUnck<T, A> {
        /// cf module level docs.
        pub unsafe fn new_in(alloc: A, value: T) -> Self {
            let inner = alloc_ptr::<Inner<T>, _>(&alloc).expect("oom");
            unsafe {
                let alloc = alloc_new(&GlobalAlloc, alloc).expect("oom");
                // safe: by contract.
                let alloc = crate::erase!(NonNull<dyn Alloc>, alloc);

                inner.as_ptr().write(Inner {
                    alloc,
                    ptrs: Cell::new(0),
                    borrow_flag: BorrowFlag::new(),
                    value: Some(UnsafeCell::new(value)),
                });
            }
            return Self { alloc: PhantomData, inner }
        }

        pub fn new_ptr(&self) -> SharedPtrUnck<T> {
            unsafe {
                let ptrs = &self.inner.as_ref().ptrs;
                ptrs.set(ptrs.get().checked_add(1).expect("too many pointers"));
            }
            SharedPtrUnck { inner: self.inner }
        }

        #[track_caller]
        pub unsafe fn borrow(&self) -> Ref<T> { unsafe {
            let inner = self.inner.as_ref();
            Ref::new(inner.borrow_flag.borrow(), {
                let value = inner.value.as_ref().expect("something went seriously wrong");
                NonNull::new_unchecked(value.get())
            })
        }}

        #[track_caller]
        pub unsafe fn borrow_mut(&self) -> RefMut<T> { unsafe {
            let inner = self.inner.as_ref();
            RefMut::new(inner.borrow_flag.borrow_mut(), {
                let value = inner.value.as_ref().expect("something went seriously wrong");
                NonNull::new_unchecked(value.get())
            })
        }}
    }

    impl<T> SharedPtrUnck<T> {
        pub fn new_ptr(&self) -> SharedPtrUnck<T> {
            unsafe {
                let ptrs = &self.inner.as_ref().ptrs;
                ptrs.set(ptrs.get().checked_add(1).expect("too many pointers"));
            }
            SharedPtrUnck { inner: self.inner }
        }

        #[track_caller]
        pub unsafe fn borrow(&self) -> Ref<T> { unsafe {
            let inner = self.inner.as_ref();
            Ref::new(inner.borrow_flag.borrow(), {
                let Some(value) = inner.value.as_ref() else {
                    panic!("use after free");
                };
                NonNull::new_unchecked(value.get())
            })
        }}

        #[track_caller]
        pub unsafe fn borrow_mut(&self) -> RefMut<T> { unsafe {
            let inner = self.inner.as_ref();
            RefMut::new(inner.borrow_flag.borrow_mut(), {
                let Some(value) = inner.value.as_ref() else {
                    panic!("use after free");
                };
                NonNull::new_unchecked(value.get())
            })
        }}
    }

    impl<T, A: Alloc> Drop for SharedBoxUnck<T, A> {
        fn drop(&mut self) {
            // drop value.
            unsafe {
                let borrows = &self.inner.as_ref().borrow_flag;
                if borrows.is_borrowed() {
                    panic!("value borrowed on drop");
                }

                drop(self.inner.as_mut().value.take());
            }
            Inner::release(self.inner);
        }
    }

    impl<T> Drop for SharedPtrUnck<T> {
        fn drop(&mut self) {
            // dec #ptrs.
            unsafe {
                let ptrs = &self.inner.as_ref().ptrs;
                ptrs.set(ptrs.get().checked_sub(1).expect("pointer was dropped multiple times"));
            }
            Inner::release(self.inner);
        }
    }

    impl<T> Inner<T> {
        // free the allocation if no more pointers exist.
        fn release(this: NonNull<Self>) { unsafe {
            let me = this.as_ref();
            if me.ptrs.get() != 0 || me.value.is_some() {
                return;
            }

            let alloc_box = me.alloc;
            let alloc = alloc_box.as_ref();
            // free `Inner`
            alloc.free(this.cast(), Layout::new::<Self>());
            // free `alloc`
            drop_and_free(&GlobalAlloc, alloc_box);
        }}
    }
}


#[cfg(all(test, debug_assertions))]
mod debug_tests {
    use super::*;

    #[test]
    fn basic() { unsafe {
        let b = SharedBoxUnck::new(42);
        assert_eq!(*b.borrow(), 42);
        assert_eq!(*b.borrow_mut(), 42);

        let p1 = b.new_ptr();
        let p2 = p1.new_ptr();
        assert_eq!(*p1.borrow(), 42);
        assert_eq!(*p1.borrow_mut(), 42);
        assert_eq!(*p2.borrow(), 42);
        assert_eq!(*p2.borrow_mut(), 42);

        *b.borrow_mut() += 27;
        assert_eq!(*b.borrow(), 69);
        assert_eq!(*b.borrow_mut(), 69);
        assert_eq!(*p1.borrow(), 69);
        assert_eq!(*p1.borrow_mut(), 69);
        assert_eq!(*p2.borrow(), 69);
        assert_eq!(*p2.borrow_mut(), 69);
    }}

    #[should_panic(expected = "already mutably borrowed")]
    #[test]
    fn already_mutably_borrowed_1() { unsafe {
        let b = SharedBoxUnck::new(42);
        let _m = b.borrow_mut();
        let _s1 = b.borrow();
    }}

    #[should_panic(expected = "already mutably borrowed")]
    #[test]
    fn already_mutably_borrowed_2() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        let _m = ptr.borrow_mut();
        let _s1 = b.borrow();
    }}

    #[should_panic(expected = "already mutably borrowed")]
    #[test]
    fn already_mutably_borrowed_3() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        let _m = b.borrow_mut();
        let _s1 = ptr.borrow();
    }}

    #[should_panic(expected = "already borrowed")]
    #[test]
    fn already_borrowed_1() { unsafe {
        let b = SharedBoxUnck::new(42);
        let _s1 = b.borrow();
        let _m = b.borrow_mut();
    }}

    #[should_panic(expected = "already borrowed")]
    #[test]
    fn already_borrowed_2() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        let _s1 = b.borrow();
        let _m = ptr.borrow_mut();
    }}

    #[should_panic(expected = "already borrowed")]
    #[test]
    fn already_borrowed_3() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        let _s1 = ptr.borrow();
        let _m = b.borrow_mut();
    }}

    #[should_panic(expected = "use after free")]
    #[test]
    fn use_after_free_1() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        drop(b);
        ptr.borrow();
    }}

    #[should_panic(expected = "use after free")]
    #[test]
    fn use_after_free_2() { unsafe {
        let b = SharedBoxUnck::new(42);
        let ptr = b.new_ptr();
        drop(b);
        ptr.borrow_mut();
    }}

    #[test]
    fn delayed_free() {
        use crate::cell::RefCell;
        use crate::vec::Vec;

        struct LogAlloc<'a> {
            log: &'a RefCell<Vec<&'static str>>,
        }

        unsafe impl<'a> crate::alloc::Alloc for LogAlloc<'a> {
            unsafe fn alloc_nonzero(&self, layout: std::alloc::Layout) -> Option<std::ptr::NonNull<u8>> {
                self.log.borrow_mut().push("LogAlloc::alloc");
                unsafe { crate::alloc::GlobalAlloc.alloc_nonzero(layout) }
            }
            unsafe fn free_nonzero(&self, ptr: std::ptr::NonNull<u8>, layout: std::alloc::Layout) {
                self.log.borrow_mut().push("LogAlloc::free");
                unsafe { crate::alloc::GlobalAlloc.free_nonzero(ptr, layout) }
            }
        }

        impl<'a> Drop for LogAlloc<'a> {
            fn drop(&mut self) {
                self.log.borrow_mut().push("LogAlloc::drop");
            }
        }


        let log = RefCell::new(Vec::new());

        let alloc = LogAlloc { log: &log };
        let b = unsafe { SharedBoxUnck::new_in(alloc, 42) };
        let p1 = b.new_ptr();
        let p2 = b.new_ptr();
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::alloc"]);
        drop(p1);
        assert!(log.borrow().is_empty());
        drop(p2);
        assert!(log.borrow().is_empty());
        drop(b);
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::free", "LogAlloc::drop"]);

        let alloc = LogAlloc { log: &log };
        let b = unsafe { SharedBoxUnck::new_in(alloc, 42) };
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::alloc"]);
        drop(b);
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::free", "LogAlloc::drop"]);

        let alloc = LogAlloc { log: &log };
        let b = unsafe { SharedBoxUnck::new_in(alloc, 42) };
        let p1 = b.new_ptr();
        let p2 = p1.new_ptr();
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::alloc"]);
        drop(b);
        assert!(log.borrow().is_empty());
        drop(p1);
        assert!(log.borrow().is_empty());
        drop(p2);
        assert_eq!(&*log.borrow_mut().take(), &["LogAlloc::free", "LogAlloc::drop"]);
    }
}


#[cfg(all(test, not(debug_assertions)))]
mod unck_tests {
    use super::*;

}

