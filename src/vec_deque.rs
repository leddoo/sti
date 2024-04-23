use core::ptr::NonNull;
use core::mem::size_of;

use crate::alloc::{Alloc, GlobalAlloc, Layout};


pub struct VecDeque<T, A: Alloc = GlobalAlloc> {
    alloc: A,

    cap: usize,  // valid for use in `Layout::array::<T>(cap)`.
    len: usize,  // <= cap
    head: usize, // < cap

    // is a live allocation iff `cap > 0`.
    // objects in `head..head+len` (wrapping) are initialized.
    data: NonNull<T>,
}


impl<T> VecDeque<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        Self::with_cap_in(GlobalAlloc, cap)
    }
}

impl<T, A: Alloc> VecDeque<T, A> {
    #[inline(always)]
    pub const fn new_in(alloc: A) -> Self {
        Self {
            alloc,
            cap: 0,
            len: 0,
            head: 0,
            data: NonNull::dangling(),
        }
    }

    #[inline]
    pub fn with_cap_in(alloc: A, cap: usize) -> VecDeque<T, A> {
        let mut result = Self::new_in(alloc);
        // `self.len == 0`.
        unsafe { result.set_cap(cap) }
        return result;
    }


    #[inline(always)]
    pub fn alloc(&self) -> &A { &self.alloc }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.cap }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len }


    /// try to set the deque's capacity.
    ///
    /// - if the call succeeds, `self.cap == new_cap`.
    /// - if the call fails, `self.cap` remains unchanged.
    ///
    /// # safety:
    /// - `self.len <= new_cap`.
    ///
    unsafe fn set_cap(&mut self, new_cap: usize) {
        debug_assert!(self.len <= new_cap);

        if new_cap == self.cap {
            return;
        }

        let new_data = unsafe {
            // `self.cap` is always valid for `Layout::array`.
            let old_layout = Layout::array::<T>(self.cap).unwrap_unchecked();
            let new_layout = Layout::array::<T>(new_cap).expect("too many elements");

            // `self.data` is an allocation iff `self.cap > 0`.
            // align is equal.
            if let Ok(()) = self.alloc.try_realloc(self.data.cast(), old_layout, new_layout) {
                if self.head + self.len > self.cap {
                    let n = self.cap - self.head;
                    core::ptr::copy(
                        self.data.as_ptr().add(self.head),
                        self.data.as_ptr().add(new_cap - n),
                        n);
                    self.head = new_cap - n;
                }
                self.data
            }
            else {
                let new_data = self.alloc.alloc(new_layout).expect("oom").cast::<T>();

                let wrapped = (self.head + self.len).checked_sub(self.cap).unwrap_or(0);
                core::ptr::copy_nonoverlapping(
                    self.data.as_ptr().add(self.head),
                    new_data.as_ptr(),
                    self.len - wrapped);
                core::ptr::copy_nonoverlapping(
                    self.data.as_ptr(),
                    new_data.as_ptr().add(self.len - wrapped),
                    wrapped);
                self.head = 0;

                self.alloc.free(self.data.cast(), old_layout);

                new_data
            }
        };
        self.data = new_data;
        self.cap  = new_cap;
    }


    pub const GROW_MIN_CAP: usize =
        if size_of::<T>() == 0 {
            usize::MAX
        }
        else if size_of::<T>() <= 256 {
            let cap = 16 / size_of::<T>();
            if cap < 4 { 4 } else { cap }
        }
        else { 1 };


    pub fn reserve(&mut self, min_cap: usize) {
        let new_cap = min_cap;
        if new_cap > self.cap {
            let new_cap =
                if size_of::<T>() > 0 {
                    // can't overflow, cause `self.cap <= isize::MAX/sizeof(T)`.
                    new_cap.max(2*self.cap)
                }
                else { new_cap };

            let new_cap = new_cap.max(Self::GROW_MIN_CAP);

            // `new_cap > self.cap >= self.len`.
            unsafe { self.set_cap(new_cap) };
        }
    }

    /// reserve space for `extra` more elements.
    pub fn grow_by(&mut self, extra: usize) {
        self.reserve(
            self.len.checked_add(extra)
            .unwrap_or_else(|| panic!("size overflow")));
    }


    #[cold]
    fn grow_for_push(&mut self) {
        self.grow_by(1);
    }

    #[inline(always)]
    pub fn push_back(&mut self, value: T) {
        if self.len == self.cap {
            self.grow_for_push();
        }
        debug_assert!(self.len < self.cap);

        unsafe {
            let mut i = self.head + self.len;
            if i >= self.cap { i -= self.cap }

            self.data.as_ptr().add(i).write(value);
            self.len += 1;
        }
    }

    #[inline(always)]
    pub fn push_front(&mut self, value: T) {
        if self.len == self.cap {
            self.grow_for_push();
        }
        debug_assert!(self.len < self.cap);

        unsafe {
            if self.head == 0 { self.head = self.cap }
            self.head -= 1;

            self.data.as_ptr().add(self.head).write(value);
            self.len += 1;
        }
    }


    #[inline(always)]
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;

        let mut i = self.head + self.len;
        if i >= self.cap { i -= self.cap }

        return Some(unsafe { self.data.as_ptr().add(i).read() });
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;

        let i = self.head;
        self.head += 1;
        if self.head == self.cap { self.head = 0 }

        return Some(unsafe { self.data.as_ptr().add(i).read() });
    }


    #[inline(always)]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        let wrapped = (self.head + self.len).checked_sub(self.cap).unwrap_or(0);
        return unsafe {(
            core::slice::from_raw_parts(
                self.data.as_ptr().add(self.head),
                self.len - wrapped),
            core::slice::from_raw_parts(
                self.data.as_ptr(),
                wrapped),
        )};
    }
}


unsafe impl<T: Sync, A: Alloc + Sync> Sync for VecDeque<T, A> {}
unsafe impl<T: Send, A: Alloc + Send> Send for VecDeque<T, A> {}


impl<T, A: Alloc> Drop for VecDeque<T, A> {
    fn drop(&mut self) {
        // drop values.
        unsafe {
            let wrapped = (self.head + self.len).checked_sub(self.cap).unwrap_or(0);

            core::ptr::drop_in_place(
                core::slice::from_raw_parts_mut(
                    self.data.as_ptr().add(self.head),
                    self.len - wrapped));

            core::ptr::drop_in_place(
                core::slice::from_raw_parts_mut(
                    self.data.as_ptr(),
                    wrapped));
        }

        unsafe {
            // `self.cap` is always valid for `Layout::array`.
            let layout = Layout::array::<T>(self.cap).unwrap_unchecked();

            // `self.data` is an allocation iff `self.cap > 0`.
            self.alloc.free(self.data.cast(), layout);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deque_basics() {
        let mut q = VecDeque::new();
        assert_eq!(q.len(), 0);
        assert_eq!(q.as_slices(), (&[][..], &[][..]));

        q.push_back(1);
        assert_eq!(q.len(), 1);
        assert_eq!(q.as_slices(), (&[1][..], &[][..]));

        q.push_back(2);
        assert_eq!(q.len(), 2);
        assert_eq!(q.as_slices(), (&[1, 2][..], &[][..]));

        q.push_back(3);
        assert_eq!(q.len(), 3);
        assert_eq!(q.as_slices(), (&[1, 2, 3][..], &[][..]));

        assert_eq!(q.pop_front(), Some(1));
        assert_eq!(q.len(), 2);
        assert_eq!(q.as_slices(), (&[2, 3][..], &[][..]));

        assert_eq!(q.pop_front(), Some(2));
        assert_eq!(q.len(), 1);
        assert_eq!(q.as_slices(), (&[3][..], &[][..]));

        assert_eq!(q.pop_front(), Some(3));
        assert_eq!(q.len(), 0);
        assert_eq!(q.as_slices(), (&[][..], &[][..]));


        let mut q = VecDeque::new();
        q.push_back(1);
        q.push_back(2);
        q.push_back(3);
        q.push_back(4);
        assert_eq!(q.len(), 4);
        assert_eq!(q.cap(), 4);
        assert_eq!(q.as_slices(), (&[1, 2, 3, 4][..], &[][..]));

        q.pop_front();
        q.push_back(5);
        assert_eq!(q.len(), 4);
        assert_eq!(q.as_slices(), (&[2, 3, 4][..], &[5][..]));
    }

    #[test]
    fn deque_growing() {
        let arena = crate::arena::Arena::new();

        let mut q = VecDeque::new_in(&arena);
        assert_eq!(q.len(), 0);
        assert_eq!(q.as_slices(), (&[][..], &[][..]));

        q.push_back(3);
        assert_eq!(q.len(), 1);
        assert_eq!(q.as_slices(), (&[3][..], &[][..]));

        q.push_front(2);
        assert_eq!(q.len(), 2);
        assert_eq!(q.as_slices(), (&[2][..], &[3][..]));

        q.push_front(1);
        assert_eq!(q.len(), 3);
        assert_eq!(q.as_slices(), (&[1, 2][..], &[3][..]));

        q.push_back(4);
        assert_eq!(q.len(), 4);
        assert_eq!(q.as_slices(), (&[1, 2][..], &[3, 4][..]));

        assert_eq!(q.cap(), 4);
        q.push_back(5);
        assert_eq!(q.len(), 5);
        assert_eq!(q.cap(), 8);
        assert_eq!(q.as_slices(), (&[1, 2][..], &[3, 4, 5][..]));

        q.push_back(6);
        q.push_back(7);
        q.push_front(0);
        assert_eq!(q.len(), 8);
        assert_eq!(q.cap(), 8);
        assert_eq!(q.as_slices(), (&[0, 1, 2][..], &[3, 4, 5, 6, 7][..]));

        arena.alloc_new(42);

        q.push_back(8);
        assert_eq!(q.len(),  9);
        assert_eq!(q.cap(), 16);
        assert_eq!(q.as_slices(), (&[0, 1, 2, 3, 4, 5, 6, 7, 8][..], &[][..]));
    }
}

