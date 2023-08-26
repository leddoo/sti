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

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        Self { inner: RawHashMap::with_cap(cap, S::default(), GlobalAlloc) }
    }
}

impl<K: Eq, V, A: Alloc, S: HashFnSeed<K, Hash=u32> + Default> HashMapEx<K, V, S, A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Self { inner: RawHashMap::new(S::default(), alloc) }
    }

    #[inline(always)]
    pub fn with_cap_in(cap: usize, alloc: A) -> Self {
        Self { inner: RawHashMap::with_cap(cap, S::default(), alloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>> HashMapEx<K, V, S, GlobalAlloc> {
    #[inline(always)]
    pub fn new_ex(seed: S) -> Self {
        Self { inner: RawHashMap::new(seed, GlobalAlloc) }
    }

    #[inline(always)]
    pub fn with_cap_ex(cap: usize, seed: S) -> Self {
        Self { inner: RawHashMap::with_cap(cap, seed, GlobalAlloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> HashMapEx<K, V, S, A> {
    #[inline(always)]
    pub fn new_ex_in(seed: S, alloc: A) -> Self {
        Self { inner: RawHashMap::new(seed, alloc) }
    }

    #[inline(always)]
    pub fn with_cap_in_ex(cap: usize, seed: S, alloc: A) -> Self {
        Self { inner: RawHashMap::with_cap(cap, seed, alloc) }
    }


    #[inline(always)]
    pub fn size(&self) -> usize { self.inner.size() }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.inner.cap() }

    #[inline(always)]
    pub fn resident(&self) -> usize { self.inner.resident() }

    #[inline(always)]
    pub fn len(&self) -> usize { self.inner.len() }


    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> { self.inner.insert(k, v) }

    #[inline(always)]
    pub fn remove<Q: ?Sized + Eq>(&mut self, k: &Q) -> Option<(K, V)>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.remove(k)
    }


    #[inline(always)]
    pub fn get<Q: ?Sized + Eq>(&self, k: &Q) -> Option<&V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if let Some(v) = self.inner.search(k) {
            unsafe { Some(v.as_ref()) }
        }
        else { None }
    }

    #[inline(always)]
    pub fn get_mut<Q: ?Sized + Eq>(&mut self, k: &Q) -> Option<&mut V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if let Some(mut v) = self.inner.search(k) {
            unsafe { Some(v.as_mut()) }
        }
        else { None }
    }
}

impl<Q, K, V, S, A> core::ops::Index<&Q> for HashMapEx<K, V, S, A>
where 
    Q: ?Sized + Eq, K: Eq + Borrow<Q>,
    S: HashFnSeed<K, Hash=u32> + HashFnSeed<Q, Hash=u32>,
    A: Alloc
{
    type Output = V;

    #[track_caller]
    #[inline(always)]
    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).expect("invalid key")
    }
}

impl<Q, K, V, S, A> core::ops::IndexMut<&Q> for HashMapEx<K, V, S, A>
where 
    Q: ?Sized + Eq, K: Eq + Borrow<Q>,
    S: HashFnSeed<K, Hash=u32> + HashFnSeed<Q, Hash=u32>,
    A: Alloc
{
    #[track_caller]
    #[inline(always)]
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(index).expect("invalid key")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hash_map_impl::GROUP_SIZE;

    #[test]
    fn hm_basic() {
        let mut hm: HashMap<String, u32> = HashMap::with_cap(69);
        let size = (69*8/7 + GROUP_SIZE) / GROUP_SIZE * GROUP_SIZE;
        let cap = size*7/8;
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 0, 0));

        hm.insert("hi".into(), 42);
        assert_eq!(hm["hi"], 42);
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 1, 1));

        hm.insert("ho".into(), 69);
        assert_eq!(hm["hi"], 42);
        assert_eq!(hm["ho"], 69);
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 2, 2));

        hm["hi"] = 17;
        assert_eq!(hm["hi"], 17);
        assert_eq!(hm["ho"], 69);
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 2, 2));

        let old = hm.insert("ho".into(), 19).unwrap();
        assert_eq!(old, 69);
        assert_eq!(hm["hi"], 17);
        assert_eq!(hm["ho"], 19);
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 2, 2));

        let (hi_k, hi_v) = hm.remove("hi").unwrap();
        assert_eq!(hi_k, "hi");
        assert_eq!(hi_v, 17);
        assert!(hm.get("hi").is_none());
        assert_eq!(hm["ho"], 19);
        assert_eq!((hm.size(), hm.cap(), hm.resident(), hm.len()), (size, cap, 1, 1));
    }

    #[test]
    fn hm_growing() {
        let mut hm: HashMap<String, u32> = HashMap::new();

        hm.insert("a".into(), 0);

        let size_0 = hm.size();
        let mut i = 1;
        while hm.len() < hm.cap() {
            hm.insert(format!("{i}"), i);
            i += 1;
        }
        assert_eq!(hm.size(), size_0);
        assert_eq!(hm.len(), hm.resident());

        let len_1 = hm.len();

        hm.insert("b".into(), 1);
        assert_eq!(hm.size(), size_0*2);
        assert_eq!(hm.len(), hm.resident());
        assert_eq!(hm.len(), len_1 + 1);

        assert_eq!(hm["a"], 0);
        assert_eq!(hm["b"], 1);
        for i in 1..i {
            assert_eq!(hm[&format!("{i}")], i);
        }
    }
}

