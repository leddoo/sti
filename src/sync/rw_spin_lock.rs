use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::cell::UnsafeCell;

use crate::sync::spin_lock::SpinLock;


pub struct RwSpinLock<T> {
    locked: AtomicBool,
    // the number of writers trying to acquire the lock.
    writers: AtomicU32,
    readers: SpinLock<u32>,
    value: UnsafeCell<T>,
}

pub struct ReadGuard<'a, T> {
    lock: &'a RwSpinLock<T>,
}

pub struct WriteGuard<'a, T> {
    lock: &'a RwSpinLock<T>,
}


impl<T> RwSpinLock<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            writers: AtomicU32::new(0),
            readers: SpinLock::new(0),
            value: UnsafeCell::new(value),
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn read(&self) -> ReadGuard<T> {
        while self.writers.load(Ordering::Relaxed) != 0 {
            core::hint::spin_loop();
        }

        let mut readers = self.readers.lock();
        if *readers == 0 {
            while self.locked.swap(true, Ordering::Acquire) {
                core::hint::spin_loop();
            }
            *readers = 1;
        }
        else {
            *readers = readers.checked_add(1).expect("too many readers");
        }

        return ReadGuard { lock: self };
    }

    #[track_caller]
    #[inline(always)]
    pub fn try_read(&self) -> Option<ReadGuard<T>> {
        if self.writers.load(Ordering::Relaxed) != 0 {
            return None;
        }

        let mut readers = self.readers.try_lock()?;
        if *readers == 0 {
            if self.locked.swap(true, Ordering::Acquire) {
                return None;
            }
            *readers = 1;
        }
        else {
            *readers = readers.checked_add(1).expect("too many readers");
        }

        return Some(ReadGuard { lock: self });
    }


    #[track_caller]
    #[inline(always)]
    pub fn write(&self) -> WriteGuard<T> {
        // wrapping is fine (not that it should ever happen).
        // it would just not favor that one writer.
        // the value must however always be decremented again,
        // otherwise the readers never get access again.
        self.writers.fetch_add(1, Ordering::Acquire);

        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }

        return WriteGuard { lock: self };
    }

    #[track_caller]
    #[inline(always)]
    pub fn try_write(&self) -> Option<WriteGuard<T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            return None;
        }

        // the write guard always decrements!
        self.writers.fetch_add(1, Ordering::Acquire);

        return Some(WriteGuard { lock: self });
    }


    #[inline(always)]
    pub fn get(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

unsafe impl<T: Send> Sync for RwSpinLock<T> {}
unsafe impl<T: Send> Send for RwSpinLock<T> {}


impl<'a, T> core::ops::Deref for ReadGuard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        let mut readers = self.lock.readers.lock();
        *readers -= 1;
        if *readers == 0 {
            self.lock.locked.store(false, Ordering::Release);
        }
    }
}


impl<'a, T> core::ops::Deref for WriteGuard<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for WriteGuard<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
        self.lock.writers.fetch_sub(1, Ordering::Relaxed);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rw_spin_lock_st() {
        let lock = RwSpinLock::new(42);

        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_some());

        let r1 = lock.read();
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_none());
        let r2 = lock.read();
        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_none());
        drop(r1);
        drop(r2);

        let w = lock.write();
        assert!(lock.try_read().is_none());
        assert!(lock.try_write().is_none());
        drop(w);

        assert!(lock.try_read().is_some());
        assert!(lock.try_write().is_some());
    }

    /*
        this needs some investigation.
        it's a lot slower when using `core::hint::spin_loop()`
        instead of `std::thread::yield_now()`.
        and both just seem way too slow in general.
        hmm, maybe that's cause the writers aren't getting scheduled
        cause all threads look busy. we could try a mutex + condvar instead.
        would also be good to check that a real reader is blocking
        when the writers are trying to write. we'd need to detect that somehow though.
    */
    #[test]
    fn rw_spin_lock() {
        // we would like to validate:
        // - multiple concurrent readers can access the value.
        // - if a writer attempts to access the value, no
        //   new readers can acquire the lock.
        // - after the writer is done, new readers have access again.
        // - multiple concurrent writers don't break things.

        // the test:
        // - n_r threads read.
        // - then n_w threads write.
        // - the readers release the lock once trying to acquire another
        //   read lock fails.
        // - the readers immediately return to step 1.
        // - the writers increment the value.
        // - the writers return to step 2 (wait until n_r threads are reading).
        // - this is repeated for n_iters.

        const READERS: u32 = 10; // more!
        const WRITERS: u32 = 5;
        const ITERS: u32 = 50;

        fn reader(num_readers: &AtomicU32, num_writers: &AtomicU32, lock: &RwSpinLock<u32>) {
            let mut prev = 0;
            loop {
                let v = lock.read();
                num_readers.fetch_add(1, Ordering::SeqCst);

                assert_eq!(*v, prev + WRITERS);
                prev = *v;

                if *v >= ITERS {
                    break;
                }

                while lock.writers.load(Ordering::Relaxed) == 0 {
                    //std::thread::yield_now();
                    core::hint::spin_loop();
                }

                while num_writers.load(Ordering::Relaxed) != WRITERS {
                    //std::thread::yield_now();
                    core::hint::spin_loop();
                }

                num_readers.fetch_sub(1, Ordering::SeqCst);
                drop(v);
            }
        }

        fn writer(num_readers: &AtomicU32, num_writers: &AtomicU32, lock: &RwSpinLock<u32>) {
            loop {
                while num_readers.load(Ordering::Relaxed) != READERS {
                    //std::thread::yield_now();
                    core::hint::spin_loop();
                }

                num_writers.fetch_add(1, Ordering::Relaxed);

                let mut v = lock.write();
                *v += 1;

                //println!("{}", *v);

                if *v >= ITERS {
                    break;
                }

                num_writers.fetch_sub(1, Ordering::Relaxed);
            }
        }

        let state = std::sync::Arc::new((
            AtomicU32::new(0),
            AtomicU32::new(0),
            RwSpinLock::new(WRITERS)));

        let mut handles = Vec::new();
        for _ in 0..READERS {
            handles.push(std::thread::spawn(
                crate::enclose!(state; move ||
                    reader(&state.0, &state.1, &state.2))));
        }
        for _ in 0..WRITERS {
            handles.push(std::thread::spawn(
                crate::enclose!(state; move ||
                    writer(&state.0, &state.1, &state.2))));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

