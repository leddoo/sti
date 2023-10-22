use core::marker::PhantomData;

use super::{Key, KRange, KSlice};

use crate::alloc::{Alloc, GlobalAlloc};
use crate::vec::Vec;


#[derive(Clone)]
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
    pub fn with_cap_in(alloc: A, cap: usize) -> Self {
        KVec { inner: Vec::with_cap_in(alloc, cap), phantom: PhantomData }
    }


    #[inline(always)]
    pub fn inner(&self) -> &Vec<V, A> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut_unck(&mut self) -> &mut Vec<V, A> {
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
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    

    #[inline(always)]
    pub fn range(&self) -> KRange<K> {
        KRange::new_unck(
            K::from_usize_unck(0),
            K::from_usize_unck(self.inner.len()))
    }

    #[inline(always)]
    pub fn next_key(&self) -> K {
        K::from_usize_unck(self.len())
    }


    #[inline(always)]
    pub fn try_push(&mut self, value: V) -> Result<K, ()> {
        if self.len() + 1 < K::LIMIT {
            let result = K::from_usize_unck(self.len());
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
    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn resize(&mut self, new_len: usize, value: V)  where V: Clone {
        assert!(new_len < K::LIMIT);
        self.inner.resize(new_len, value);
    }


    #[inline(always)]
    pub fn as_slice(&self) -> &KSlice<K, V> {
        KSlice::new_unck(&self.inner)
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut KSlice<K, V> {
        KSlice::new_mut_unck(&mut self.inner)
    }


    #[inline(always)]
    pub fn clone_in<B: Alloc>(&self, alloc: B) -> KVec<K, V, B> where V: Clone {
        KVec { inner: self.inner.clone_in(alloc), phantom: PhantomData }
    }

    #[inline(always)]
    pub fn leak<'a>(self) -> &'a mut KSlice<K, V>  where A: 'a {
        KSlice::new_mut_unck(self.inner.leak())
    }
}


impl<K: Key, V, A: Alloc> core::ops::Deref for KVec<K, V, A> {
    type Target = KSlice<K, V>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<K: Key, V, A: Alloc> core::ops::DerefMut for KVec<K, V, A> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<K: Key, V, A: Alloc> core::fmt::Debug for KVec<K, V, A>
where K: core::fmt::Debug, V: core::fmt::Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}
