use core::marker::PhantomData;
use core::ptr::NonNull;

use super::Key;


#[repr(transparent)]
pub struct KSlice<K: Key, V> {
    phantom: PhantomData<fn (K) -> K>,
    inner:   [V],
}

impl<K: Key, V> KSlice<K, V> {
    #[inline(always)]
    pub fn new_unck<'a>(slice: &'a [V]) -> &'a Self {
        unsafe { &*(slice as *const [V] as *const Self) }
    }

    #[inline(always)]
    pub fn inner(&self) -> &[V] {
        &self.inner
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn iter(&self) -> KIter<K, V> {
        KIter {
            ptr: unsafe { NonNull::new_unchecked(self.inner.as_ptr() as *mut V) },
            len: self.inner.len(),
            idx: 0,
            phantom: PhantomData,
        }
    }
}


#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct KIter<'a, K: Key, V> {
    ptr: NonNull<V>,
    len: usize,
    idx: usize, // <= len
    phantom: PhantomData<(K, &'a V)>,
}

impl<'a, K: Key, V> Iterator for KIter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let k = unsafe { K::from_usize_unck(self.idx) };
            let v = unsafe { &*self.ptr.as_ptr().add(self.idx) };
            self.idx += 1;
            return Some((k, v));
        }
        None
    }

    #[inline(always)]
    fn nth(&mut self, i: usize) -> Option<Self::Item> {
        if i < self.len - self.idx {
            self.idx += i;

            let k = unsafe { K::from_usize_unck(self.idx) };
            let v = unsafe { &*self.ptr.as_ptr().add(self.idx) };
            self.idx += 1;
            return Some((k, v));
        }
        None
    }

    #[inline(always)]
    fn last(self) -> Option<Self::Item> {
        if self.idx < self.len {
            let idx = self.len - 1;
            let k = unsafe { K::from_usize_unck(idx) };
            let v = unsafe { &*self.ptr.as_ptr().add(idx) };
            return Some((k, v));
        }
        None
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len - self.idx;
        (len, Some(len))
    }
}

