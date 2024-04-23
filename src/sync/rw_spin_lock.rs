use crate::mem::UnsafeCell;
use crate::sync::atomic::{AtomicU32, Ordering};


pub struct RwSpinLock<T> {
    state: AtomicState,
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
            state: AtomicState::new(UNLOCKED),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn read(&self) -> ReadGuard<T> {
        if !try_read(&self.state) {
            lock(&self.state, false);
        }

        return ReadGuard { lock: self };
    }

    #[inline]
    pub fn try_read(&self) -> Option<ReadGuard<T>> {
        if !try_read(&self.state) {
            return None
        }

        return Some(ReadGuard { lock: self });
    }


    #[track_caller]
    #[inline(always)]
    pub fn write(&self) -> WriteGuard<T> {
        if !try_write(&self.state) {
            lock(&self.state, true);
        }

        return WriteGuard { lock: self };
    }

    #[track_caller]
    #[inline(always)]
    pub fn try_write(&self) -> Option<WriteGuard<T>> {
        if !try_write(&self.state) {
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
        let ok = self.lock.state.fetch_update(Ordering::Release, Ordering::Acquire, |s| Some({
            debug_assert!(s & LOCKED != 0);

            let count = s >> READERS_SHIFT;
            debug_assert!(count > 0);

            if count == 1 { s - (READER | LOCKED) }
            else          { s - READER            }
        }));
        assert!(ok.is_ok());
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
        debug_assert!(self.lock.state.load(Ordering::Relaxed) & LOCKED != 0);

        self.lock.state.store(UNLOCKED, Ordering::Release);
    }
}


// implementation inspired by std's sys::sync::queue::RwLock.

type State = u32;
type AtomicState = AtomicU32;

const UNLOCKED: State = 0;

/// writer intention. setting this flag as a writer is optional!
const WRITER: State = 0b001;

/// locked by either readers or a writer.
const LOCKED: State = 0b010;

/// constant added for each reader.
const READER: State = 0b100;

const READERS_SHIFT: u32 = 2;

#[inline]
fn update_read(s: u32) -> Option<u32> {
    // cases:
    //  s = UNLOCKED            // can acquire, must set LOCKED.
    //  s = WRITER              // writer intention.
    //  s = LOCKED              // writer 1.
    //  s = LOCKED | WRITER     // writer 2.
    //  s = LOCKED | READER(s)  // can join.
    if s != LOCKED {
        // not writer 1.
        if s & WRITER == 0 {
            // not writer intention, writer 2.
            if let Some(next) = s.checked_add(READER) {
                return Some(next | LOCKED);
            }
        }
    }
    None
}

#[inline]
fn update_write(s: u32) -> Option<u32> {
    if s & LOCKED == 0 {
        return Some(LOCKED);
    }
    None
}

#[inline]
fn try_read(a: &AtomicState) -> bool {
    // idk why Relaxed on the fetch is fine here.
    a.fetch_update(Ordering::Acquire, Ordering::Relaxed, update_read).is_ok()
}

#[inline]
fn try_write(a: &AtomicState) -> bool {
    a.fetch_or(LOCKED, Ordering::Acquire) & !WRITER == UNLOCKED
}


// exponential backoff -> 127 in total before yielding.
const MAX_SPINS: u32 = 64;

#[cold]
fn lock(a: &AtomicState, write: bool) {
    let update = if write { update_write } else { update_read };

    let mut s = a.load(Ordering::Relaxed);
    let mut n = 1;
    loop {
        if let Some(next) = update(s) {
            match a.compare_exchange_weak(s, next, Ordering::Acquire, Ordering::Relaxed) {
                Ok(_) => return,
                Err(new) => s = new,
            }
        }
        else if cfg!(feature = "std") && n > MAX_SPINS {
            #[cfg(feature = "std")]
            std::thread::yield_now();

            s = a.load(Ordering::Relaxed);
            n = 1;
        }
        else {
            if write && s & WRITER == 0 {
                a.fetch_or(WRITER, Ordering::Relaxed);
            }

            for _ in 0..n {
                core::hint::spin_loop();
            }

            s = a.load(Ordering::Relaxed);
            n = (n << 1).min(MAX_SPINS+1);
        }
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
        const WRITERS: u32 = 1; // currently broken for multiple writers
                                // cause writer intention is "weak".
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

                while lock.try_read().is_some() {
                    //std::thread::yield_now();
                    core::hint::spin_loop();
                }

                while num_writers.load(Ordering::Relaxed) != WRITERS {
                    std::thread::yield_now();
                    //core::hint::spin_loop();
                }

                num_readers.fetch_sub(1, Ordering::SeqCst);
                drop(v);
            }
        }

        fn writer(num_readers: &AtomicU32, num_writers: &AtomicU32, lock: &RwSpinLock<u32>) {
            loop {
                while num_readers.load(Ordering::Relaxed) != READERS {
                    std::thread::yield_now();
                    //core::hint::spin_loop();
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

