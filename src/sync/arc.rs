
pub struct Arc<T: ?Sized> {
    inner: std::sync::Arc<T>,
}

impl<T> Arc<T> {
    #[inline]
    pub fn new(value: T) -> Arc<T> {
        Arc { inner: std::sync::Arc::new(value) }
    }

    #[inline]
    pub fn new_cyclic(value: impl FnOnce(&WeakArc<T>) -> T) -> Arc<T> {
        Arc { inner: std::sync::Arc::new_cyclic(|weak| value(&WeakArc { inner: weak.clone() })) }
    }

    #[inline]
    pub fn into_inner(self) -> Option<T> {
        std::sync::Arc::into_inner(self.inner)
    }
}

impl<T: ?Sized> Arc<T> {
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        std::sync::Arc::as_ptr(&self.inner)
    }

    #[inline]
    pub fn into_raw(self) -> *const T {
        std::sync::Arc::into_raw(self.inner)
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *const T) -> Arc<T> {
        unsafe { Arc { inner: std::sync::Arc::from_raw(ptr) } }
    }


    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        std::sync::Arc::get_mut(&mut self.inner)
    }

    #[inline]
    pub fn make_mut(&mut self) -> &mut T  where T: Clone {
        std::sync::Arc::make_mut(&mut self.inner)
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    #[inline]
    fn clone(&self) -> Self {
        Arc { inner: self.inner.clone() }
    }
}

impl<T: ?Sized> core::ops::Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}



pub struct WeakArc<T: ?Sized> {
    inner: std::sync::Weak<T>,
}

impl<T: ?Sized> WeakArc<T> {
    #[inline]
    pub fn new(arc: &Arc<T>) -> WeakArc<T> {
        WeakArc { inner: std::sync::Arc::downgrade(&arc.inner) }
    }

    #[inline]
    pub fn upgrade(&self) -> Option<Arc<T>> {
        self.inner.upgrade().map(|inner| Arc { inner })
    }
}

impl<T: ?Sized> Clone for WeakArc<T> {
    #[inline]
    fn clone(&self) -> Self {
        WeakArc { inner: self.inner.clone() }
    }
}

