use crate::mem::UnsafeCell;
use crate::sync::atomic::{AtomicBool, Ordering};


pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value:  UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> Guard<T> {
        if !try_lock(&self.locked) {
            lock(&self.locked);
        }

        return Guard { lock: self };
    }

    #[inline]
    pub fn try_lock(&self) -> Option<Guard<T>> {
        if !try_lock(&self.locked) {
            return None;
        }

        return Some(Guard { lock: self });
    }

    #[inline]
    pub fn get(&mut self) -> &mut T {
        return unsafe { &mut *self.value.get() };
    }

    #[inline]
    pub fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut guard = self.lock();
        return f(&mut guard);
    }
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}


#[inline]
fn try_lock(a: &AtomicBool) -> bool {
    a.swap(true, Ordering::Acquire) == false
}

// exponential backoff -> 127 in total before yielding.
const MAX_SPINS: u32 = 64;

#[cold]
fn lock(a: &AtomicBool) {
    let mut n = 1;
    loop {
        if try_lock(a) {
            return;
        }

        if cfg!(feature = "std") && n > MAX_SPINS {
            #[cfg(feature = "std")]
            std::thread::yield_now();

            n = 1;
        }
        else {
            for _ in 0..n {
                core::hint::spin_loop();
            }

            n = (n << 1).min(MAX_SPINS+1);
        }
    }
}



pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for Guard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for Guard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
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

