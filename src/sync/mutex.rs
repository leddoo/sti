
pub struct Mutex<T: ?Sized> {
    inner: std::sync::Mutex<T>,
}

impl<T> Mutex<T> {
    #[inline]
    pub const fn new(value: T) -> Mutex<T> {
        Mutex { inner: std::sync::Mutex::new(value) }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner.into_inner().expect("poison")
    }
}

impl<T: ?Sized> Mutex<T> {
    #[inline]
    pub fn try_lock(&self) -> Option<Guard<T>> {
        match self.inner.try_lock() {
            Ok(inner) => Some(Guard { inner }),
            Err(e) => match e {
                std::sync::TryLockError::Poisoned(_) => panic!("poison"),
                std::sync::TryLockError::WouldBlock => None,
            }
        }
    }

    #[inline]
    pub fn lock(&self) -> Guard<T> {
        Guard { inner: self.inner.lock().expect("poison") }
    }


    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut().expect("poison")
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}



pub struct Guard<'a, T: ?Sized> {
    pub(super) inner: std::sync::MutexGuard<'a, T>,
}

impl<'a, T: ?Sized> core::ops::Deref for Guard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}


