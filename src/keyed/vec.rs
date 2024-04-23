use crate::alloc::{Alloc, GlobalAlloc};
use crate::mem::PhantomData;
use crate::vec::Vec;
use crate::traits::FromIn;

use super::{Key, KRange, KSlice};


#[derive(Clone)]
pub struct KVec<K: Key, V, A: Alloc = GlobalAlloc> {
    inner:   Vec<V, A>, // .len() < K::LIMIT
    phantom: PhantomData<K>,
}

impl<K: Key, V> KVec<K, V, GlobalAlloc> {
    #[inline]
    pub fn new() -> Self {
        KVec { inner: Vec::new(), phantom: PhantomData }
    }

    #[inline]
    pub fn with_cap(cap: usize) -> Self {
        KVec { inner: Vec::with_cap(cap), phantom: PhantomData }
    }
}

impl<K: Key, V, A: Alloc> KVec<K, V, A> {
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        KVec { inner: Vec::new_in(alloc), phantom: PhantomData }
    }

    #[inline]
    pub fn with_cap_in(alloc: A, cap: usize) -> Self {
        KVec { inner: Vec::with_cap_in(alloc, cap), phantom: PhantomData }
    }

    #[inline]
    pub fn from_inner(inner: Vec<V, A>) -> Self {
        assert!(inner.len() < K::LIMIT);
        KVec { inner, phantom: PhantomData }
    }

    #[inline]
    pub fn from_inner_unck(inner: Vec<V, A>) -> Self {
        KVec { inner, phantom: PhantomData }
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
    pub fn alloc(&self) -> &A { self.inner.alloc() }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.inner.cap() }

    #[inline(always)]
    pub fn len(&self) -> usize { self.inner.len() }


    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }


    #[inline]
    pub fn range(&self) -> KRange<K> {
        KRange::new_unck(
            K::from_usize_unck(0),
            K::from_usize_unck(self.inner.len()))
    }

    #[inline]
    pub fn next_key(&self) -> K {
        K::from_usize_unck(self.len())
    }


    #[inline]
    pub fn push(&mut self, value: V) -> K {
        assert!(self.len() + 1 < K::LIMIT, "too many elements");
        let result = K::from_usize_unck(self.len());
        self.inner.push(value);
        return result;
    }


    #[inline]
    pub fn push_with(&mut self, f: impl FnOnce(&mut Self, K) -> V) -> K {
        let k = self.next_key();
        let v = f(self, k);
        let k2 = self.push(v);
        assert!(k == k2);
        return k;
    }


    #[inline(always)]
    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[track_caller]
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: V)  where V: Clone {
        assert!(new_len < K::LIMIT);
        self.inner.resize(new_len, value);
    }


    #[inline]
    pub fn as_slice(&self) -> &KSlice<K, V> {
        KSlice::new_unck(&self.inner)
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut KSlice<K, V> {
        KSlice::new_mut_unck(&mut self.inner)
    }


    #[inline]
    pub fn map_in<V2, A2: Alloc, F: FnMut((K, &V)) -> V2>(&self, alloc: A2, f: F) -> KVec<K, V2, A2> {
        KVec { inner: Vec::from_in(alloc, self.iter().map(f)), phantom: PhantomData }
    }

    #[inline]
    pub fn map<V2, F: FnMut((K, &V)) -> V2>(&self, f: F) -> KVec<K, V2, A>  where A: Clone {
        self.map_in(self.alloc().clone(), f)
    }

    #[inline]
    pub fn clone_in<B: Alloc>(&self, alloc: B) -> KVec<K, V, B> where V: Clone {
        KVec { inner: self.inner.clone_in(alloc), phantom: PhantomData }
    }

    #[inline]
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


impl<K: Key, V: PartialEq, A: Alloc> PartialEq for KVec<K, V, A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<K: Key, V: Eq, A: Alloc> Eq for KVec<K, V, A> {}


impl<K: Key, V, A: Alloc> core::fmt::Debug for KVec<K, V, A>
where K: core::fmt::Debug, V: core::fmt::Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.as_slice().fmt(f)
    }
}
