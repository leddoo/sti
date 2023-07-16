use core::marker::PhantomData;

use super::{Key, KRange, KSlice, KIter};

use crate::alloc::{Alloc, GlobalAlloc};
use crate::vec::Vec;


pub struct KVec<K: Key, V, A: Alloc = GlobalAlloc> {
    inner:   Vec<V, A>, // .len() < K::LIMIT
    phantom: PhantomData<K>,
}

impl<K: Key, V> KVec<K, V, GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        KVec { inner: Vec::new(), phantom: PhantomData }
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        KVec { inner: Vec::with_cap(cap), phantom: PhantomData }
    }
}

impl<K: Key, V, A: Alloc> KVec<K, V, A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        KVec { inner: Vec::new_in(alloc), phantom: PhantomData }
    }

    #[inline(always)]
    pub fn with_cap_in(cap: usize, alloc: A) -> Self {
        KVec { inner: Vec::with_cap_in(cap, alloc), phantom: PhantomData }
    }


    #[inline(always)]
    pub fn inner(&self) -> &Vec<V, A> {
        &self.inner
    }

    #[inline(always)]
    pub unsafe fn inner_mut(&mut self) -> &mut Vec<V, A> {
        &mut self.inner
    }

    #[inline(always)]
    pub fn into_inner(self) -> Vec<V, A> {
        self.inner
    }


    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn range(&self) -> KRange<K> {
        unsafe { KRange::new(
            K::from_usize_unck(0),
            K::from_usize_unck(self.inner.len())) }
    }

    #[inline(always)]
    pub fn next_key(&self) -> K {
        unsafe { K::from_usize_unck(self.len()) }
    }


    #[inline(always)]
    pub fn try_push(&mut self, value: V) -> Result<K, ()> {
        if self.len() + 1 < K::LIMIT {
            let result = unsafe { K::from_usize_unck(self.len()) };
            self.inner.push(value);
            return Ok(result);
        }
        return Err(());
    }

    #[inline(always)]
    pub fn push(&mut self, value: V) -> K {
        self.try_push(value).unwrap()
    }

    #[inline(always)]
    pub fn push_f<F: FnOnce(&mut Self)>(&mut self, f: F) -> KRange<K> {
        let begin = unsafe { K::from_usize_unck(self.len()) };
        f(self);
        let end = unsafe { K::from_usize_unck(self.len()) };

        return KRange::new(begin, end);
    }


    #[inline(always)]
    pub fn as_slice(&self) -> &KSlice<K, V> {
        KSlice::new_unck(&self.inner)
    }

    #[inline(always)]
    pub fn iter(&self) -> KIter<K, V> {
        self.as_slice().iter()
    }


    #[inline(always)]
    pub fn get(&self, index: K) -> Option<&V> {
        self.inner.get(index.usize())
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: K) -> Option<&mut V> {
        self.inner.get_mut(index.usize())
    }
}


impl<K: Key, V, A: Alloc> core::ops::Index<K> for KVec<K, V, A> {
    type Output = V;

    #[inline(always)]
    fn index(&self, index: K) -> &Self::Output {
        &self.inner[index.usize()]
    }
}

impl<K: Key, V, A: Alloc> core::ops::IndexMut<K> for KVec<K, V, A> {
    #[inline(always)]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.inner[index.usize()]
    }
}

