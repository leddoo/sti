use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;


pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}


impl<T> SpinLock<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value:  UnsafeCell::new(value),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> SpinGuard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }

        return SpinGuard { lock: self };
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<SpinGuard<T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            return None;
        }

        return Some(SpinGuard { lock: self });
    }

    #[inline(always)]
    pub fn get(&mut self) -> &mut T {
        return unsafe { &mut *self.value.get() };
    }

    #[inline(always)]
    pub fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut guard = self.lock();
        return f(&mut guard);
    }
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}



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
        self.lock.locked.store(false, Ordering::Release);
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
        const TARGET: i32 = 5_000;

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

                    while b.lock().0 != *a {
                        std::thread::yield_now();
                    }

                    *a += 1;
                    std::thread::yield_now();
                }
                else {
                    b.with_lock(|b| {
                        // separate reads & writes to validate exclusive access.
                        let delta = b.1;
                        std::thread::yield_now();
                        b.0 += delta;
                        std::thread::yield_now();
                        b.1 = 0;
                    });

                    // don't prevent thread holding `a` from writing delta.
                    std::thread::yield_now();
                }
            }
        }

        use std::sync::Arc;

        let a = Arc::new(SpinLock::new(1));
        let b = Arc::new(SpinLock::new((0, 0)));

        let t1 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });
        let t2 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });
        let t3 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });
        let t4 = std::thread::spawn({ let a = a.clone(); let b = b.clone(); move || thread(&a, &b) });

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        t4.join().unwrap();

        assert_eq!(*a.lock(), TARGET);
    }
}

