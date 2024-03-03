use core::ptr::NonNull;
use core::alloc::Layout;
use core::mem::size_of;

use crate::num::OrdUtils;
use crate::alloc::{Alloc, GlobalAlloc};


pub struct ManualVec<T, A: Alloc = GlobalAlloc> {
    alloc: A,

    cap: usize, // valid for use in `Layout::array::<T>(cap)`.
    len: usize, // <= cap

    // is a live allocation iff `cap > 0`.
    // objects in `0..self.len` are initialized.
    data: NonNull<T>,
}


impl<T> ManualVec<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Option<Self> {
        Self::with_cap_in(GlobalAlloc, cap)
    }
}

impl<T, A: Alloc> ManualVec<T, A> {
    #[inline(always)]
    pub const fn new_in(alloc: A) -> Self {
        Self {
            alloc,
            data: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub fn with_cap_in(alloc: A, cap: usize) -> Option<Self> {
        let mut result = Self::new_in(alloc);
        // `self.len == 0`.
        unsafe { result.set_cap(cap).ok()? }
        return Some(result);
    }


    #[inline(always)]
    pub fn alloc(&self) -> &A { &self.alloc }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.cap }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len }


    /// try to set the vector's capacity.
    ///
    /// - if the call succeeds, `self.cap == new_cap`.
    /// - if the call fails, `self.cap` remains unchanged.
    ///
    /// # safety:
    /// - `self.len <= new_cap`.
    ///
    unsafe fn set_cap(&mut self, new_cap: usize) -> Result<(), ()> {
        debug_assert!(self.len <= new_cap);

        if new_cap == self.cap {
            return Ok(());
        }

        let new_data = unsafe {
            // `self.cap` is always valid for `Layout::array`.
            let old_layout = Layout::array::<T>(self.cap).unwrap_unchecked();

            // ensure new layout is valid.
            let new_layout = Layout::array::<T>(new_cap).map_err(|_| ())?;

            // `self.data` is an allocation iff `self.cap > 0`.
            // align is equal.
            self.alloc.realloc(self.data.cast(), old_layout, new_layout)
            .ok_or(())?
            .cast()
        };

        self.data = new_data;
        self.cap  = new_cap;

        return Ok(());
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

    pub fn reserve(&mut self, min_cap: usize) -> Result<(), ()> {
        let new_cap = min_cap;
        if new_cap > self.cap {
            let new_cap =
                if size_of::<T>() > 0 {
                    // can't overflow, cause `self.cap <= isize::MAX/sizeof(T)`.
                    new_cap.at_least(2*self.cap)
                }
                else { new_cap };

            let new_cap = new_cap.at_least(Self::GROW_MIN_CAP);

            // `new_cap > self.cap >= self.len`.
            return unsafe { self.set_cap(new_cap) };
        }
        return Ok(());
    }

    pub fn reserve_exactly(&mut self, min_cap: usize) -> Result<(), ()> {
        if min_cap > self.cap {
            // `min_cap > self.cap >= self.len`.
            return unsafe { self.set_cap(min_cap) };
        }
        return Ok(())
    }

    #[inline]
    pub fn reserve_extra(&mut self, extra: usize) -> Result<(), ()> {
        self.reserve(self.len.checked_add(extra).ok_or(())?)
    }

    #[cold]
    fn reserve_one_extra(&mut self) -> Result<(), ()> {
        self.reserve_extra(1)
    }


    #[inline(always)]
    pub fn push_or_alloc(&mut self, value: T) -> Result<(), ()> {
        if self.len == self.cap {
            self.reserve_one_extra()?;
        }
        unsafe { crate::assume!(self.len < self.cap) }

        unsafe {
            // can't overflow cause `len < cap`.
            // is a valid write, cause `cap > 0` -> `data` is a live allocation.
            self.data.as_ptr().add(self.len).write(value);
            self.len += 1;
        }

        return Ok(());
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if self.len == self.cap {
            return Err(());
        }
        unsafe { crate::assume!(self.len < self.cap) }

        unsafe {
            // can't overflow cause `len < cap`.
            // is a valid write, cause `cap > 0` -> `data` is a live allocation.
            self.data.as_ptr().add(self.len).write(value);
            self.len += 1;
        }

        return Ok(());
    }

    pub fn extend_from_slice_or_alloc(&mut self, values: &[T]) -> Result<(), ()>  where T: Clone {
        self.reserve_extra(values.len())?;
        unsafe { crate::assume!(self.len + values.len() <= self.cap) }

        unsafe {
            let ptr = self.data.as_ptr().add(self.len);
            for i in 0..values.len() {
                core::ptr::write(ptr.add(i), values[i].clone());
            }
            self.len += values.len();
        }

        return Ok(());
    }


    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;

            return Some(unsafe {
                // can't overflow, cause `len <= cap`.
                // is a valid read, cause `cap > 0` -> `data` is a live allocation.
                self.data.as_ptr().add(self.len).read()
            });
        }
        return None;
    }



    /// #safety:
    /// - `new_len < self.cap()`.
    /// - all values in `self[0..new_len]` must be properly initialized.
    #[inline(always)]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.cap);
        self.len = new_len;
    }

    #[track_caller]
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        assert!(new_len <= self.len);

        if new_len == self.len {
            return;
        }

        let old_len = self.len;
        self.len = new_len;

        // drop values
        unsafe {
            core::ptr::drop_in_place(
                core::slice::from_raw_parts_mut(
                    self.data.as_ptr().add(new_len), old_len - new_len));
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }



    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.len()) }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr(), self.len()) }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_ptr()
    }


    #[track_caller]
    #[inline(always)]
    pub fn rev(&self, index: usize) -> &T {
        assert!(index < self.len, "rev index {index} out of bounds (len: {})", self.len);
        unsafe { &*self.data.as_ptr().add(self.len-1 - index) }
    }

    #[track_caller]
    #[inline(always)]
    pub fn rev_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len, "rev index {index} out of bounds (len: {})", self.len);
        unsafe { &mut *self.data.as_ptr().add(self.len-1 - index) }
    }



    #[inline]
    pub fn leak<'a>(self) -> &'a mut [T]  where A: 'a {
        unsafe {
            let mut this = core::mem::ManuallyDrop::new(self);

            // drop alloc.
            core::ptr::drop_in_place(&mut this.alloc);

            core::slice::from_raw_parts_mut(this.data.as_ptr(), this.len())
        }
    }
}


unsafe impl<T: Sync, A: Alloc + Sync> Sync for ManualVec<T, A> {}
unsafe impl<T: Send, A: Alloc + Send> Send for ManualVec<T, A> {}


impl<T, A: Alloc + Default> Default for ManualVec<T, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::new_in(A::default())
    }
}


impl<T, A: Alloc> Drop for ManualVec<T, A> {
    fn drop(&mut self) {
        let len = self.len;
        #[cfg(debug_assertions)] { self.len = 0; }

        // drop values.
        unsafe {
            core::ptr::drop_in_place(
                core::slice::from_raw_parts_mut(
                    self.data.as_ptr(), len));
        }

        let layout = Layout::array::<T>(self.cap).unwrap();

        // `self.data` is an allocation iff `self.cap > 0`.
        unsafe { self.alloc.free(self.data.cast(), layout) }

        #[cfg(debug_assertions)] {
            self.data = NonNull::dangling();
            self.cap  = 0;
        }
    }
}


impl<T, A: Alloc> core::ops::Deref for ManualVec<T, A> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, A: Alloc> core::ops::DerefMut for ManualVec<T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}


impl<T, A: Alloc, I: core::slice::SliceIndex<[T]>> core::ops::Index<I> for ManualVec<T, A> {
    type Output = I::Output;

    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        core::ops::Index::index(&**self, index)
    }
}

impl<T, A: Alloc, I: core::slice::SliceIndex<[T]>> core::ops::IndexMut<I> for ManualVec<T, A> {
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        core::ops::IndexMut::index_mut(&mut **self, index)
    }
}


