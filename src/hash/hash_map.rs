use core::borrow::Borrow;

use crate::alloc::{Alloc, GlobalAlloc};
use crate::hash::{HashFnSeed, DefaultHashFnSeed};
use crate::hash::fxhash::FxHasher32DefaultSeed;

use super::hash_map_impl::RawHashMap;


pub struct HashMapEx<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc = GlobalAlloc> {
    inner: RawHashMap<K, V, S, A>,
}

pub type HashMap<K, V, A = GlobalAlloc> = HashMapEx<K, V, FxHasher32DefaultSeed, A>;

pub type HashMapF<K, V, F, A = GlobalAlloc> = HashMapEx<K, V, DefaultHashFnSeed<F>, A>;


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32> + Default> HashMapEx<K, V, S, GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: RawHashMap::new(S::default(), GlobalAlloc) }
    }
}

impl<K: Eq, V, A: Alloc, S: HashFnSeed<K, Hash=u32> + Default> HashMapEx<K, V, S, A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Self { inner: RawHashMap::new(S::default(), alloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>> HashMapEx<K, V, S, GlobalAlloc> {
    #[inline(always)]
    pub fn new_ex(seed: S) -> Self {
        Self { inner: RawHashMap::new(seed, GlobalAlloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> HashMapEx<K, V, S, A> {
    #[inline(always)]
    pub fn new_ex_in(seed: S, alloc: A) -> Self {
        Self { inner: RawHashMap::new(seed, alloc) }
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.inner.len() }


    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> { self.inner.insert(k, v) }


    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> { self.inner.get(k) }

    #[inline(always)]
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> { self.inner.get_mut(k) }
}

impl<Q, K, V, S, A> core::ops::Index<&Q> for HashMapEx<K, V, S, A>
where 
    Q: ?Sized, K: Eq + Borrow<Q>,
    S: HashFnSeed<K, Hash=u32> + HashFnSeed<Q, Hash=u32>,
    A: Alloc
{
    type Output = V;

    #[track_caller]
    #[inline(always)]
    fn index(&self, index: &Q) -> &Self::Output {
        self.inner.get(index).expect("invalid key")
    }
}

impl<Q, K, V, S, A> core::ops::IndexMut<&Q> for HashMapEx<K, V, S, A>
where 
    Q: ?Sized, K: Eq + Borrow<Q>,
    S: HashFnSeed<K, Hash=u32> + HashFnSeed<Q, Hash=u32>,
    A: Alloc
{
    #[track_caller]
    #[inline(always)]
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.inner.get_mut(index).expect("invalid key")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hm_basic() {
        let mut hm: HashMap<String, u32> = HashMap::new();

        hm.insert("hi".into(), 42);
        assert_eq!(hm["hi"], 42);
    }
}

