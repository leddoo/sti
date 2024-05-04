
pub struct RwLock<T: ?Sized> {
    inner: std::sync::RwLock<T>,
}

impl<T> RwLock<T> {
    #[inline]
    pub const fn new(value: T) -> RwLock<T> {
        RwLock { inner: std::sync::RwLock::new(value) }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner.into_inner().expect("poison")
    }
}

impl<T: ?Sized> RwLock<T> {
    #[inline]
    pub fn try_read(&self) -> Option<ReadGuard<T>> {
        match self.inner.try_read() {
            Ok(inner) => Some(ReadGuard { inner }),
            Err(e) => match e {
                std::sync::TryLockError::Poisoned(_) => panic!("poison"),
                std::sync::TryLockError::WouldBlock => None,
            }
        }
    }

    #[inline]
    pub fn read(&self) -> ReadGuard<T> {
        ReadGuard { inner: self.inner.read().expect("poison") }
    }

    #[inline]
    pub fn try_write(&self) -> Option<WriteGuard<T>> {
        match self.inner.try_write() {
            Ok(inner) => Some(WriteGuard { inner }),
            Err(e) => match e {
                std::sync::TryLockError::Poisoned(_) => panic!("poison"),
                std::sync::TryLockError::WouldBlock => None,
            }
        }
    }

    #[inline]
    pub fn write(&self) -> WriteGuard<T> {
        WriteGuard { inner: self.inner.write().expect("poison") }
    }


    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut().expect("poison")
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}



pub struct ReadGuard<'a, T: ?Sized> {
    inner: std::sync::RwLockReadGuard<'a, T>,
}

impl<'a, T: ?Sized> core::ops::Deref for ReadGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}



pub struct WriteGuard<'a, T: ?Sized> {
    inner: std::sync::RwLockWriteGuard<'a, T>,
}

impl<'a, T: ?Sized> core::ops::Deref for WriteGuard<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T: ?Sized> core::ops::DerefMut for WriteGuard<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

