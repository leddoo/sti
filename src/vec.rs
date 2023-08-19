use core::ptr::NonNull;
use core::mem::size_of;
use core::alloc::Layout;

use crate::num::OrdUtils;
use crate::alloc::*;


pub struct Vec<T, A: Alloc = GlobalAlloc> {
    alloc: A,
    data: NonNull<T>, // is a live allocation iff `cap > 0`.
                      // objects in `0..self.len` are live.
    cap: usize, // valid for use in `Layout::array::<T>(cap)`.
    len: usize, // <= cap
}

impl<T> Vec<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        Self::with_cap_in(cap, GlobalAlloc)
    }
}

impl<T, A: Alloc> Vec<T, A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Vec {
            alloc,
            data: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }


    #[inline(always)]
    pub fn alloc(&self) -> &A {
        &self.alloc
    }


    pub fn try_with_cap_in(cap: usize, alloc: A) -> Result<Vec<T, A>, AllocError> {
        let mut result = Vec::new_in(alloc);
        // `self.len == 0`.
        unsafe { result.try_set_cap(cap)? }
        Ok(result)
    }

    pub fn with_cap_in(cap: usize, alloc: A) -> Vec<T, A> {
        Self::try_with_cap_in(cap, alloc).unwrap()
    }


    #[inline(always)]
    pub fn cap(&self) -> usize { self.cap }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len }
}


#[derive(Clone, Copy, Debug)]
pub enum AllocError {
    CapacityOverflow,
    OutOfMemory,
}

impl<T, A: Alloc> Vec<T, A> {
    /// try to set the vector's capacity.
    ///
    /// - if the call succeeds, `self.cap == new_cap`.
    /// - if the call fails, `self.cap` remains unchanged.
    ///
    /// # safety:
    /// - `self.len <= new_cap`.
    ///
    unsafe fn try_set_cap(&mut self, new_cap: usize) -> Result<(), AllocError> {
        debug_assert!(self.len <= new_cap);

        if new_cap == self.cap {
            return Ok(());
        }

        let old_layout = Layout::array::<T>(self.cap).unwrap();
        let new_layout = Layout::array::<T>(new_cap).map_err(|_| AllocError::CapacityOverflow)?;

        let new_data = unsafe {
            // `self.data` is an allocation iff `self.cap > 0`.
            // align is equal.
            self.alloc.realloc(self.data.cast(), old_layout, new_layout)
            .ok_or(AllocError::OutOfMemory)?
            .cast()
        };
        self.data = new_data;
        self.cap  = new_cap;

        return Ok(());
    }



    pub fn try_trim_exact(&mut self) -> Result<(), AllocError>{
        // `new_cap >= self.len`.
        unsafe { self.try_set_cap(self.len) }
    }

    pub fn trim_exact(&mut self) {
        self.try_trim_exact().unwrap();
    }

    // doesn't fail, if the allocation fails.
    pub fn trim(&mut self) {
        // round `self.len` up to pow2.
        // if less than `self.cap`, `set_cap`.
        unimplemented!()
    }


    pub fn try_reserve_exact(&mut self, min_cap: usize) -> Result<(), AllocError> {
        if min_cap > self.cap {
            // `min_cap > self.cap >= self.len`.
            return unsafe { self.try_set_cap(min_cap) };
        }
        return Ok(());
    }

    pub fn reserve_exact(&mut self, min_cap: usize) {
        self.try_reserve_exact(min_cap).unwrap();
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


    pub fn try_reserve(&mut self, min_cap: usize) -> Result<(), AllocError> {
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
            return unsafe { self.try_set_cap(new_cap) };
        }
        return Ok(());
    }

    pub fn reserve(&mut self, min_cap: usize) {
        self.try_reserve(min_cap).unwrap();
    }


    pub fn try_grow_by(&mut self, extra: usize) -> Result<(), AllocError> {
        self.try_reserve(
            self.len.checked_add(extra)
            .ok_or(AllocError::CapacityOverflow)?)
    }

    pub fn grow_by(&mut self, extra: usize) {
        self.try_grow_by(extra).unwrap();
    }


    #[cold]
    pub fn grow_for_push(&mut self) {
        self.grow_by(1);
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        debug_assert!(self.len <= self.cap);
        if self.len == self.cap {
            self.grow_for_push();
        }
        debug_assert!(self.len < self.cap);

        unsafe {
            // can't overflow cause `len < cap`.
            // is a valid write, cause `cap > 0` -> `data` is a live allocation.
            self.data.as_ptr().add(self.len).write(value);
            self.len += 1;
        }
    }

    #[inline(always)]
    pub fn push_unique(&mut self, value: T) -> bool  where T: PartialEq {
        if !self.contains(&value) {
            self.push(value);
            return true;
        }
        false
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


    #[inline(always)]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.cap);
        self.len = new_len;
    }

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

    #[inline]
    pub fn free(&mut self) {
        self.truncate(0);
        unsafe { self.try_set_cap(0).unwrap() };
    }


    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut src = 0;
        let mut dst = 0;
        while src < self.len() {
            let v = unsafe { &*self.data.as_ptr().add(src) };
            if f(v) {
                if src != dst {
                    unsafe {
                        self.data.as_ptr().add(dst).write(
                            self.data.as_ptr().add(src).read());
                    }
                }
                dst += 1;
            }
            else {
                unsafe {
                    core::ptr::drop_in_place(
                        self.data.as_ptr().add(src));
                }
            }
            src += 1;
        }
        self.len = dst;
    }

    #[inline(always)]
    pub fn take(&mut self) -> Self  where A: Clone {
        let result = Self {
            alloc: self.alloc.clone(),
            data:  self.data,
            cap:   self.cap,
            len:   self.len,
        };
        self.data = NonNull::dangling();
        self.cap  = 0;
        self.len  = 0;
        return result;
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


    #[inline(always)]
    pub fn rev(&self, index: usize) -> &T {
        let last = self.len - 1;
        &self.as_slice()[last - index]
    }

    #[inline(always)]
    pub fn rev_mut(&mut self, index: usize) -> &mut T {
        let last = self.len - 1;
        &mut self.as_slice_mut()[last - index]
    }


    pub fn try_clone_in<B: Alloc>(&self, alloc: B) -> Result<Vec<T, B>, AllocError> where T: Clone {
        let mut result = Vec::try_with_cap_in(self.len, alloc)?;

        for value in self {
            result.push(value.clone());
        }

        return Ok(result);
    }

    pub fn clone_in<B: Alloc>(&self, alloc: B) -> Vec<T, B> where T: Clone {
        self.try_clone_in(alloc).unwrap()
    }


    #[inline]
    pub fn leak<'a>(self) -> &'a mut [T]  where A: 'a {
        unsafe {
            let mut this = core::mem::ManuallyDrop::new(self);

            core::ptr::drop_in_place(&mut this.alloc);

            core::slice::from_raw_parts_mut(this.data.as_ptr(), this.len())
        }
    }
}


impl<T: Clone, A: Alloc + Copy> Clone for Vec<T, A> {
    fn clone(&self) -> Self {
        self.clone_in(self.alloc)
    }
}


impl<T, A: Alloc> Drop for Vec<T, A> {
    fn drop(&mut self) {
        let len = self.len;
        #[cfg(debug_assertions)] {
            self.len = 0;
        }

        // drop values.
        unsafe {
            core::ptr::drop_in_place(
                core::slice::from_raw_parts_mut(
                    self.data.as_ptr(), len));
        }

        // @todo-speed: check if this prevents `drop` elision with arenas.
        let layout = Layout::array::<T>(self.cap).unwrap();

        // `self.data` is an allocation iff `self.cap > 0`.
        unsafe { self.alloc.free(self.data.cast(), layout) }

        #[cfg(debug_assertions)] {
            self.data = NonNull::dangling();
            self.cap  = 0;
        }
    }
}


impl<T: core::fmt::Debug, A: Alloc> core::fmt::Debug for Vec<T, A> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}


impl<T, A: Alloc> core::ops::Deref for Vec<T, A> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, A: Alloc> core::ops::DerefMut for Vec<T, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}


impl<T, A: Alloc, I: core::slice::SliceIndex<[T]>> core::ops::Index<I> for Vec<T, A> {
    type Output = I::Output;

    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        core::ops::Index::index(&**self, index)
    }
}

impl<T, A: Alloc, I: core::slice::SliceIndex<[T]>> core::ops::IndexMut<I> for Vec<T, A> {
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        core::ops::IndexMut::index_mut(&mut **self, index)
    }
}



impl<'a, T, A: Alloc> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A: Alloc> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<T, A: Alloc> core::borrow::Borrow<[T]> for Vec<T, A> {
    #[inline(always)]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A: Alloc> core::borrow::BorrowMut<[T]> for Vec<T, A> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_drop() {
        use core::cell::Cell;

        struct Dropper<'a> {
            ticket: u32,
            counter: &'a Cell<u32>
        }

        impl<'a> Drop for Dropper<'a> {
            fn drop(&mut self) {
                assert_eq!(self.counter.get(), self.ticket);
                self.counter.set(self.counter.get() + 1);
            }
        }

        let counter = Cell::new(0);
        let d = |ticket: u32| {
            Dropper { ticket, counter: &counter }
        };

        // basic drop.
        let mut v = Vec::new();
        v.push(d(0));
        v.push(d(1));
        v.push(d(2));
        drop(v);
        assert_eq!(counter.get(), 3);

        // truncate.
        counter.set(0);
        let mut v = Vec::new();
        v.push(d(2));
        v.push(d(0));
        v.push(d(1));
        v.truncate(1);
        drop(v);
        assert_eq!(counter.get(), 3);
    }
}

