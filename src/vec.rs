use core::ptr::NonNull;
use core::mem::{size_of, ManuallyDrop};
use core::alloc::Layout;

use crate::num::OrdUtils;
use crate::alloc::{Alloc, AllocError, GlobalAlloc};
use crate::traits::FromIn;


pub struct Vec<T, A: Alloc = GlobalAlloc> {
    alloc: A,

    cap: usize, // valid for use in `Layout::array::<T>(cap)`.
    len: usize, // <= cap

    // is a live allocation iff `cap > 0`.
    // objects in `0..self.len` are initialized.
    data: NonNull<T>, 
}

impl<T> Vec<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        Self::with_cap_in(GlobalAlloc, cap)
    }


    #[inline(always)]
    pub fn from_value(v: T, len: usize) -> Self  where T: Clone {
        Self::from_value_in(GlobalAlloc, v, len)
    }

    #[inline(always)]
    pub fn from_array<const N: usize>(vs: [T; N]) -> Self {
        Self::from_array_in(GlobalAlloc, vs)
    }

    #[inline(always)]
    pub fn from_slice(vs: &[T]) -> Self  where T: Clone {
        Self::from_slice_in(GlobalAlloc, vs)
    }

    #[inline(always)]
    pub fn from_fn(f: impl FnMut() -> T, len: usize) -> Self {
        Self::from_fn_in(GlobalAlloc, f, len)
    }
}

impl<T, A: Alloc> Vec<T, A> {
    #[inline(always)]
    pub const fn new_in(alloc: A) -> Self {
        Vec {
            alloc,
            data: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }


    pub fn try_with_cap_in(alloc: A, cap: usize) -> Result<Vec<T, A>, AllocError> {
        let mut result = Vec::new_in(alloc);
        // `self.len == 0`.
        unsafe { result.try_set_cap(cap)? }
        Ok(result)
    }

    pub fn with_cap_in(alloc: A, cap: usize) -> Vec<T, A> {
        Self::try_with_cap_in(alloc, cap).unwrap()
    }


    #[inline(always)]
    pub fn alloc(&self) -> &A { &self.alloc }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.cap }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len }
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
        let new_layout = Layout::array::<T>(new_cap).map_err(|_| AllocError::SizeOverflow)?;

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



    pub fn try_trim_exact(&mut self) -> Result<(), AllocError> {
        // `new_cap >= self.len`.
        unsafe { self.try_set_cap(self.len) }
    }

    /// # panics:
    /// - if the allocation fails.
    #[track_caller]
    pub fn trim_exact(&mut self) {
        self.try_trim_exact().unwrap();
    }

    /*
    // doesn't fail, if the allocation fails.
    pub fn trim(&mut self) {
        // round `self.len` up to pow2.
        // if less than `self.cap`, `set_cap`.
        unimplemented!()
    }
    */


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
            .ok_or(AllocError::SizeOverflow)?)
    }

    /// reserve space for `extra` more elements.
    #[track_caller]
    pub fn grow_by(&mut self, extra: usize) {
        self.try_grow_by(extra).unwrap();
    }


    #[cold]
    #[track_caller]
    fn grow_for_push(&mut self) {
        self.grow_by(1);
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow_for_push();
        }
        unsafe { crate::assume!(self.len < self.cap) }

        unsafe {
            // can't overflow cause `len < cap`.
            // is a valid write, cause `cap > 0` -> `data` is a live allocation.
            self.data.as_ptr().add(self.len).write(value);
            self.len += 1;
        }
    }

    pub fn extend_from_slice(&mut self, values: &[T])  where T: Clone {
        self.grow_by(values.len());
        unsafe { crate::assume!(self.len + values.len() <= self.cap) }

        unsafe {
            let ptr = self.data.as_ptr().add(self.len);
            for i in 0..values.len() {
                core::ptr::write(ptr.add(i), values[i].clone());
            }
            self.len += values.len();
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len, "insert index {index} out of bounds (len: {})", self.len);

        if self.len == self.cap {
            self.grow_for_push();
        }
        unsafe { crate::assume!(self.len < self.cap) }

        unsafe {
            let ptr = self.data.as_ptr().add(index);
            if index < self.len {
                core::ptr::copy(ptr, ptr.add(1), self.len - index);
            }

            core::ptr::write(ptr, value);
            self.len += 1;
        }
    }

    pub fn insert_from_slice(&mut self, index: usize, values: &[T])  where T: Clone {
        assert!(index <= self.len, "insert index {index} out of bounds (len: {})", self.len);

        if self.len + values.len() > self.cap {
            self.grow_by(values.len())
        }
        unsafe { crate::assume!(self.len + values.len() <= self.cap) }

        unsafe {
            let ptr = self.data.as_ptr().add(index);
            if index < self.len {
                core::ptr::copy(ptr, ptr.add(values.len()), self.len - index);
            }

            for i in 0..values.len() {
                core::ptr::write(ptr.add(i), values[i].clone());
            }
            self.len += values.len();
        }
    }


    #[inline]
    pub fn from_value_in(alloc: A, v: T, len: usize) -> Self  where T: Clone {
        let mut result = Vec::with_cap_in(alloc, len);
        for _ in 1..len {
            result.push(v.clone());
        }
        if len > 0 {
            result.push(v);
        }
        return result;
    }

    #[inline]
    pub fn from_array_in<const N: usize>(alloc: A, vs: [T; N]) -> Self {
        let len = vs.len();

        let mut result = Vec::with_cap_in(alloc, len);
        unsafe {
            let vs = ManuallyDrop::new(vs);
            core::ptr::copy_nonoverlapping(
                vs.as_ptr(),
                result.as_mut_ptr(),
                len);

            result.len = len;
        }

        return result;
    }

    #[inline]
    pub fn from_slice_in(alloc: A, vs: &[T]) -> Self  where T: Clone {
        let mut result = Vec::with_cap_in(alloc, vs.len());
        for v in vs {
            result.push(v.clone());
        }
        return result;
    }

    #[inline]
    pub fn from_fn_in(alloc: A, mut f: impl FnMut() -> T, len: usize) -> Self {
        let mut result = Vec::with_cap_in(alloc, len);
        for _ in 0..len {
            result.push(f());
        }
        return result;
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


    #[track_caller]
    #[inline(always)]
    pub fn remove_swap(&mut self, index: usize) -> T {
        assert!(index < self.len, "index {index} out of bounds (len: {})", self.len);

        let last = unsafe { self.data.as_ptr().add(self.len - 1).read() };
        self.len -= 1;

        if index == self.len {
            return last;
        }
        else {
            return unsafe { self.data.as_ptr().add(index).replace(last) };
        }
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
    pub fn resize(&mut self, new_len: usize, value: T)  where T: Clone {
        if new_len <= self.len {
            self.truncate(new_len);
        }
        else {
            self.reserve(new_len);
            debug_assert!(new_len <= self.cap);

            // we don't wanna stress llvm too much.
            // if you're using this function to clone expensive things,
            // you may wanna reconsider your life choices.
            for i in self.len..new_len {
                unsafe { self.data.as_ptr().add(i).write(value.clone()) };
            }

            self.len = new_len;
        }
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


    pub fn try_clone_in<B: Alloc>(&self, alloc: B) -> Result<Vec<T, B>, AllocError> where T: Clone {
        let mut result = Vec::try_with_cap_in(alloc, self.len)?;

        for value in self.iter() {
            result.push(value.clone());
        }

        return Ok(result);
    }

    #[track_caller]
    #[inline(always)]
    pub fn clone_in<B: Alloc>(&self, alloc: B) -> Vec<T, B> where T: Clone {
        self.try_clone_in(alloc).unwrap()
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

    pub fn move_into<B: Alloc>(mut self, new_alloc: B) -> Vec<T, B> {
        let len = self.len;

        let mut new_vec: Vec<T, B> = Vec::with_cap_in(new_alloc, len);

        unsafe {
            self.len = 0;

            core::ptr::copy_nonoverlapping(
                self.data.as_ptr(),
                new_vec.data.as_ptr(),
                len);

            new_vec.len = len;
        }

        new_vec
    }
}



unsafe impl<T: Sync, A: Alloc + Sync> Sync for Vec<T, A> {}
unsafe impl<T: Send, A: Alloc + Send> Send for Vec<T, A> {}


impl<T, A: Alloc + Default> Default for Vec<T, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::new_in(A::default())
    }
}


impl<T: Clone, A: Alloc + Clone> Clone for Vec<T, A> {
    fn clone(&self) -> Self {
        self.clone_in(self.alloc.clone())
    }
}


impl<T, A: Alloc> Drop for Vec<T, A> {
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


impl<T: PartialEq, A: Alloc> PartialEq for Vec<T, A> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, A: Alloc> Eq for Vec<T, A> {}


impl<T: core::fmt::Debug, A: Alloc> core::fmt::Debug for Vec<T, A> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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


impl<T> FromIterator<T> for Vec<T, GlobalAlloc> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_in(GlobalAlloc, iter.into_iter())
    }
}

impl<T, A: Alloc, I: Iterator<Item = T>> FromIn<I, A> for Vec<T, A> {
    #[inline]
    fn from_in(alloc: A, iter: I) -> Self {
        let (min_len, max_len) = iter.size_hint();
        let cap = max_len.unwrap_or(min_len);

        let mut result = Vec::with_cap_in(alloc, cap);
        for v in iter {
            result.push(v);
        }
        return result;
    }
}


impl<T, A: Alloc> Extend<T> for Vec<T, A> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();

        let (min_len, max_len) = iter.size_hint();
        let cap = max_len.unwrap_or(min_len);

        self.grow_by(cap);
        for v in iter {
            self.push(v);
        }
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


impl<T: Clone> From<&[T]> for Vec<T, GlobalAlloc> {
    #[inline(always)]
    fn from(value: &[T]) -> Self {
        Self::from_slice(value)
    }
}


#[macro_export]
macro_rules! vec_in {
    ($alloc:expr) => (
        $crate::vec::Vec::new_in($alloc)
    );
    ($alloc:expr, $elem:expr; $n:expr) => (
        $crate::vec::Vec::from_value_in($alloc, $elem, $n)
    );
    ($alloc:expr; $($x:expr),+ $(,)?) => (
        $crate::vec::Vec::from_array_in($alloc, [$($x),+])
    );
}

#[macro_export]
macro_rules! vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vec::Vec::from_value($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        $crate::vec::Vec::from_array([$($x),+])
    );
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_basics() {
        let v: Vec<bool> = vec!();
        assert_eq!(*v, []);
        assert_eq!(v.cap(), 0);

        let v = vec!(1, 2, 3);
        assert_eq!(*v, [1, 2, 3]);
        assert_eq!(v.cap(), 3);

        let v = vec!("hi".to_string(); 2);
        assert_eq!(*v, ["hi".to_string(), "hi".to_string()]);
        assert_eq!(v.cap(), 2);

        let v = Vec::from_slice(&["hi".to_string(), "ho".to_string()]);
        assert_eq!(*v, ["hi".to_string(), "ho".to_string()]);
        assert_eq!(v.cap(), 2);


        let mut v = vec![1, 2, 3, 4];
        assert_eq!(v.remove_swap(1), 2);
        assert_eq!(*v, [1, 4, 3]);
        assert_eq!(v.remove_swap(2), 3);
        assert_eq!(*v, [1, 4]);


        let mut arena = crate::arena::Arena::new();
        {
            let v: Vec<bool, _> = vec_in!(&arena);
            assert_eq!(*v, []);

            let v = vec_in!(&arena; 1, 2, 3);
            assert_eq!(*v, [1, 2, 3]);

            let v = vec_in!(&arena, "hi".to_string(); 2);
            assert_eq!(*v, ["hi".to_string(), "hi".to_string()]);

            let v = Vec::from_slice_in(&arena, &["hi".to_string(), "ho".to_string()]);
            assert_eq!(*v, ["hi".to_string(), "ho".to_string()]);
            assert_eq!(v.cap(), 2);
        }
        arena.reset();
    }

    #[test]
    fn vec_from_iter() {
        use crate::traits::CopyIt;

        let a = Vec::from_iter(6..9);
        assert_eq!(a.len(), 3);
        assert_eq!(a.cap(), 3);
        assert_eq!(*a, [6, 7, 8]);

        let b = Vec::from_iter(a.copy_map_it(|x| x - 5));
        assert_eq!(b.len(), 3);
        assert_eq!(b.cap(), 3);
        assert_eq!(*b, [1, 2, 3]);


        let mut arena = crate::arena::Arena::new();
        {
            let aa = Vec::from_in(&arena, a.copy_it());
            assert_eq!(aa.len(), 3);
            assert_eq!(aa.cap(), 3);
            assert_eq!(*aa, [6, 7, 8]);

            let bb = Vec::from_in(&arena, aa.copy_it().rev());
            assert_eq!(bb.len(), 3);
            assert_eq!(bb.cap(), 3);
            assert_eq!(*bb, [8, 7, 6]);
        }
        arena.reset();
    }

    #[test]
    fn vec_extend() {
        let mut v = vec![1, 2, 3];
        assert_eq!(*v, [1, 2, 3]);

        v.extend([4, 5]);
        assert_eq!(*v, [1, 2, 3, 4, 5]);
        assert_eq!(v.cap(), 2*3);

        let mut v = vec!["hi".to_string(); 6];
        v.extend_from_slice(&["ho".to_string(), "ha".to_string()]);
        assert_eq!(v.cap(), 2*6);
        let mut vs = v.iter();
        for _ in 0..6 {
            assert_eq!(vs.next().unwrap(), "hi");
        }
        assert_eq!(vs.next().unwrap(), "ho");
        assert_eq!(vs.next().unwrap(), "ha");
        assert_eq!(vs.next(), None);
    }

    #[test]
    fn vec_resize() {
        let mut v = vec![1, 2, 3];

        v.resize(2, 69);
        assert_eq!(*v, [1, 2]);

        v.resize(7, 69);
        assert_eq!(*v, [1, 2, 69, 69, 69, 69, 69]);
    }

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


    #[test]
    fn vec_insert() {
        let mut v = Vec::new();
        v.push(69);
        v.push(420);
        v.push(31298);
        v.insert(0, 0);
        assert_eq!(&*v, [0, 69, 420, 31298]);
        v.insert(1, 1);
        assert_eq!(&*v, [0, 1, 69, 420, 31298]);
        v.push(4389);
        v.insert(2, 2);
        assert_eq!(&*v, [0, 1, 2, 69, 420, 31298, 4389]);
        v.insert(3, 3);
        assert_eq!(&*v, [0, 1, 2, 3, 69, 420, 31298, 4389]);
        v.push(574);
        v.push(12398);
        v.insert(v.len(), 4);
        assert_eq!(&*v, [0, 1, 2, 3, 69, 420, 31298, 4389, 574, 12398, 4]);
    }


    #[test]
    fn vec_insert_from_slice() {
        let mut v = Vec::new();
        v.push(69);
        v.push(31298);
        v.push(4389);
        v.insert_from_slice(0, &[0, 1, 2, 3]);
        assert_eq!(&*v, [0, 1, 2, 3, 69, 31298, 4389]);
        v.push(574);
        v.push(12398);
        v.insert_from_slice(v.len(), &[6, 7, 8]);
        assert_eq!(&*v, [0, 1, 2, 3, 69, 31298, 4389, 574, 12398, 6, 7, 8]);
        v.push(9);
        v.push(10);
        v.insert_from_slice(4, &[4, 5]);
        assert_eq!(&*v, [0, 1, 2, 3, 4, 5, 69, 31298, 4389, 574, 12398, 6, 7, 8, 9, 10]);
    }
}

