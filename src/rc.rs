use core::alloc::Layout;
use core::ptr::NonNull;
use core::cell::Cell;
use core::mem::ManuallyDrop;

use crate::alloc::{Alloc, GlobalAlloc, alloc_ptr};


pub struct Rc<T: ?Sized, A: Alloc = GlobalAlloc> {
    inner: NonNull<RcInner<T, A>>,
}

pub struct RcInner<T: ?Sized, A: Alloc = GlobalAlloc> {
    alloc: A,
    refs:  Cell<usize>,
    data:  T,
}


impl<T> Rc<T, GlobalAlloc> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Rc::new_in(GlobalAlloc, value)
    }
}

impl<T, A: Alloc> Rc<T, A> {
    #[inline]
    pub fn try_new_in(alloc: A, value: T) -> Option<Self> {
        let inner = alloc_ptr::<RcInner<T, A>, _>(&alloc)?;
        unsafe {
            inner.as_ptr().write(RcInner {
                alloc,
                refs: Cell::new(1),
                data: value,
            });
        }
        return Some(Rc { inner })
    }

    #[track_caller]
    #[inline]
    pub fn new_in(alloc: A, value: T) -> Self {
        Self::try_new_in(alloc, value).unwrap()
    }
}

impl<T: ?Sized, A: Alloc> Rc<T, A> {
    #[inline]
    pub fn alloc(&self) -> &A {
        unsafe { &self.inner.as_ref().alloc }
    }

    #[inline]
    pub fn ref_count(&self) -> usize {
        unsafe { self.inner.as_ref().refs.get() }
    }

    #[inline]
    pub fn try_mut(&mut self) -> Option<&mut T> {
        (self.ref_count() == 1).then_some(
            unsafe { &mut self.inner.as_mut().data })
    }

    #[inline]
    pub fn make_mut(&mut self) -> &mut T  where T: Clone, A: Clone {
        self.make_mut_ex(|this|
            Rc::new_in(this.alloc().clone(), this.as_ref().clone()))
    }

    #[inline(always)]
    pub fn make_mut_ex<F: FnOnce(&Self) -> Self>(&mut self, clone: F) -> &mut T {
        if self.ref_count() != 1 {
            *self = clone(self);
        }
        assert_eq!(self.ref_count(), 1);
        unsafe { &mut self.inner.as_mut().data }
    }


    #[inline(always)]
    pub unsafe fn cast<U: ?Sized, F>(self, f: F) -> Rc<U, A>
    where F: FnOnce(*mut RcInner<T, A>) -> *mut RcInner<U, A> {
        let this = ManuallyDrop::new(self);
        Rc { inner: unsafe { NonNull::new_unchecked(f(this.inner.as_ptr())) } }
    }
}



impl<T: ?Sized, A: Alloc> Rc<T, A> {
    #[cold]
    fn drop_impl(&mut self) {
        unsafe {
            // dropped at end of scope.
            let alloc = (&mut (*self.inner.as_ptr()).alloc as *mut A).read();

            // drop value.
            core::ptr::drop_in_place(&mut (*self.inner.as_ptr()).data);

            // free memory.
            // ig `self.inner.as_ref` is technically UB, but std's rc does this too.
            alloc.free(self.inner.cast(), Layout::for_value(self.inner.as_ref()));
        }
    }
}

impl<T: ?Sized, A: Alloc> Drop for Rc<T, A> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let refs = &self.inner.as_ref().refs;
            debug_assert!(refs.get() > 0);

            refs.set(refs.get() - 1);
            if refs.get() == 0 {
                self.drop_impl();
            }
        }
    }
}

impl<T: ?Sized, A: Alloc> Clone for Rc<T, A> {
    #[inline]
    fn clone(&self) -> Self {
        let refs = unsafe { &self.inner.as_ref().refs };
        refs.set(refs.get().checked_add(1).unwrap());
        return Rc { inner: self.inner };
    }
}

impl<T: ?Sized, A: Alloc> core::ops::Deref for Rc<T, A> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().data }
    }
}

impl<T: ?Sized, A: Alloc> core::convert::AsRef<T> for Rc<T, A> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &*self
    }
}

impl<T: ?Sized, A: Alloc> core::borrow::Borrow<T> for Rc<T, A> {
    #[inline(always)]
    fn borrow(&self) -> &T {
        &*self
    }
}

