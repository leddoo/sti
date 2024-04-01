use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;

use crate::sync::spin_lock::SpinLock;


pub struct RwSpinLock<T> {
    locked: AtomicBool,
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
            readers: SpinLock::new(0),
            value: UnsafeCell::new(value),
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn read(&self) -> ReadGuard<T> {
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
}

