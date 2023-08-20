use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;


pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self { locked: false.into(), value: value.into() }
    }

    #[inline(always)]
    pub fn lock(&self) -> SpinGuard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }

        SpinGuard { lock: self }
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<SpinGuard<T>> {
        if !self.locked.swap(true, Ordering::Acquire) {
            return Some(SpinGuard { lock: self });
        }
        None
    }

    #[inline(always)]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}



pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinGuard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinGuard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for SpinGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_lock() {
        #[cfg(miri)]
        const TARGET: i32 = 25;
        #[cfg(not(miri))]
        const TARGET: i32 = 1_000;

        fn thread(a: &SpinLock<i32>, b: &SpinLock<(i32, i32)>) {
            /*
                - try lock `a`.
                    - lock `b` and store the delta.
                    - wait for `b` to have the same value as `a`.
                    - increment `a`.
                - else:
                    - lock `b` and add delta.
            */
            loop {
                if let Some(mut a) = a.try_lock() {
                    if *a == TARGET {
                        return;
                    }

                    {
                        let mut b = b.lock();
                        b.1 = *a - b.0;
                    }

                    while b.lock().0 != *a {}

                    *a += 1;
                    std::thread::yield_now();
                }
                else {
                    let mut b = b.lock();
                    let delta = b.1;
                    std::thread::yield_now();
                    b.0 += delta;
                    std::thread::yield_now();
                    b.1  = 0;
                }
            }
        }

        use std::sync::Arc;

        let a = Arc::new(SpinLock::new(1));
        let b = Arc::new(SpinLock::new((0, 0)));

        let t1 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });
        let t2 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(*a.lock(), TARGET);
    }
}

