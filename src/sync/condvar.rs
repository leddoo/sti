use crate::sync::mutex;


pub struct Condvar {
    inner: std::sync::Condvar,
}

impl Condvar {
    #[inline]
    pub const fn new() -> Condvar {
        Condvar { inner: std::sync::Condvar::new() }
    }

    #[inline]
    pub fn wait<'a, T>(&self, guard: mutex::Guard<'a, T>) -> mutex::Guard<'a, T> {
        mutex::Guard { inner: self.inner.wait(guard.inner).expect("poison") }
    }

    #[inline]
    pub fn notify_one(&self) {
        self.inner.notify_one();
    }

    #[inline]
    pub fn notify_n(&self, n: usize) {
        for _ in 0..n {
            self.inner.notify_one();
        }
    }

    #[inline]
    pub fn notify_all(&self) {
        self.inner.notify_all();
    }
}

