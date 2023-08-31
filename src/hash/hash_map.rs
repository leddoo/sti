use core::hash::Hash;
use core::borrow::Borrow;

use crate::alloc::{Alloc, GlobalAlloc};
use crate::hash::{HashFnSeed, DefaultHashFnSeed};
use crate::hash::fxhash::FxHasher32;

use super::raw_hash_map::{RawHashMap, RawIter};


pub struct HashMap<K: Eq, V,
    S: HashFnSeed<K, Hash=u32> = DefaultSeed,
    A: Alloc = GlobalAlloc>
{
    inner: RawHashMap<K, V, S, A>,
}


pub type DefaultSeed = DefaultHashFnSeed<FxHasher32>;

pub type HashMapF<K, V, F, A = GlobalAlloc> = HashMap<K, V, DefaultHashFnSeed<F>, A>;


impl<K: Hash + Eq, V> HashMap<K, V, DefaultSeed, GlobalAlloc> {
    /// construct with default seed in `GlobalAlloc`.
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: RawHashMap::new(DefaultSeed::new(), GlobalAlloc) }
    }

    /// construct with capacity, with default seed in `GlobalAlloc`.
    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        Self { inner: RawHashMap::with_cap(cap, DefaultSeed::new(), GlobalAlloc) }
    }
}

impl<K: Hash + Eq, V, A: Alloc> HashMap<K, V, DefaultSeed, A> {
    /// construct with default seed in `alloc`.
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Self { inner: RawHashMap::new(DefaultSeed::new(), alloc) }
    }

    /// construct with capacity, with default seed in `alloc`.
    #[inline(always)]
    pub fn with_cap_in(cap: usize, alloc: A) -> Self {
        Self { inner: RawHashMap::with_cap(cap, DefaultSeed::new(), alloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>> HashMap<K, V, S, GlobalAlloc> {
    /// construct with default seed in `GlobalAlloc`.
    #[inline(always)]
    pub fn fnew() -> Self where S: Default {
        Self { inner: RawHashMap::new(S::default(), GlobalAlloc) }
    }

    /// construct with capacity, with default seed in `GlobalAlloc`.
    #[inline(always)]
    pub fn fwith_cap(cap: usize) -> Self where S: Default {
        Self { inner: RawHashMap::with_cap(cap, S::default(), GlobalAlloc) }
    }

    /// construct with `seed` in `GlobalAlloc`.
    #[inline(always)]
    pub fn with_seed(seed: S) -> Self {
        Self { inner: RawHashMap::new(seed, GlobalAlloc) }
    }

    /// construct with capacity, with `seed` in `GlobalAlloc`.
    #[inline(always)]
    pub fn with_cap_with_seed(cap: usize, seed: S) -> Self {
        Self { inner: RawHashMap::with_cap(cap, seed, GlobalAlloc) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> HashMap<K, V, S, A> {
    /// construct with default seed in `alloc`.
    #[inline(always)]
    pub fn fnew_in(alloc: A) -> Self where S: Default {
        Self { inner: RawHashMap::new(S::default(), alloc) }
    }

    /// construct with capacity, with default seed in `alloc`.
    #[inline(always)]
    pub fn fwith_cap_in(cap: usize, alloc: A) -> Self where S: Default {
        Self { inner: RawHashMap::with_cap(cap, S::default(), alloc) }
    }

    /// construct with `seed` in `alloc`.
    #[inline(always)]
    pub fn with_seed_in(seed: S, alloc: A) -> Self {
        Self { inner: RawHashMap::new(seed, alloc) }
    }

    /// construct with capacity, with `seed` in `alloc`.
    #[inline(always)]
    pub fn with_cap_with_seed_in(cap: usize, seed: S, alloc: A) -> Self {
        Self { inner: RawHashMap::with_cap(cap, seed, alloc) }
    }


    /// backing allocator.
    #[inline(always)]
    pub fn alloc(&self) -> &A { self.inner.alloc() }

    /// size.
    /// - total number of allocated key/value pairs.
    #[inline(always)]
    pub fn size(&self) -> usize { self.inner.size() }

    /// capacity.
    /// - how many residents the hash map can have before resizing.
    #[inline(always)]
    pub fn cap(&self) -> usize { self.inner.cap() }

    /// number of residents.
    /// - at least `len`, includes tombstones.
    #[inline(always)]
    pub fn resident(&self) -> usize { self.inner.resident() }

    /// number of entries.
    #[inline(always)]
    pub fn len(&self) -> usize { self.inner.len() }


    /// insert a key/value pair.
    /// - returns old value, if present.
    #[inline(always)]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// remove a key/value pair.
    #[inline(always)]
    pub fn remove<Q: ?Sized + Eq>(&mut self, key: &Q) -> Option<(K, V)>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.remove(key)
    }


    pub fn get_or_insert<'q, Q: ?Sized + Eq>(&mut self, key: &'q Q, default: V) -> &mut V
    where K: From<&'q Q> + Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.get_or_insert(key, |_| (key.into(), default))
    }

    pub fn get_or_insert_with<'q, Q: ?Sized + Eq, F>(&mut self, key: &'q Q, f: F) -> &mut V
    where F: FnOnce() -> V, K: From<&'q Q> + Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.get_or_insert(key, |_| (key.into(), f()))
    }

    pub fn get_or_insert_with_key<'q, Q: ?Sized + Eq, F>(&mut self, key: &'q Q, f: F) -> &mut V
    where F: FnOnce(&'q Q) -> (K, V), K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.get_or_insert(key, f)
    }

    pub fn kget_or_insert(&mut self, key: K, default: V) -> &mut V {
        self.inner.kget_or_insert(key, || default)
    }

    pub fn kget_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        self.inner.kget_or_insert(key, f)
    }


    /// get a key's value.
    #[inline(always)]
    pub fn get<Q: ?Sized + Eq>(&self, key: &Q) -> Option<&V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if let Some(v) = self.inner.search(key) {
            unsafe { Some(v.as_ref()) }
        }
        else { None }
    }

    /// get a key's value mutably.
    #[inline(always)]
    pub fn get_mut<Q: ?Sized + Eq>(&mut self, key: &Q) -> Option<&mut V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if let Some(mut v) = self.inner.search(key) {
            unsafe { Some(v.as_mut()) }
        }
        else { None }
    }


    /// probe length of a key.
    /// - returns `(groups_visited, keys_compared)`.
    /// - useful for debugging hash collision issues.
    /// - specific to the swiss table implementation.
    ///   may be removed/changed in the future.
    #[inline(always)]
    pub fn probe_length<Q: ?Sized + Eq>(&self, key: &Q) -> (usize, usize)
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        self.inner.probe_length(key)
    }


    /// iterate key/value pairs.
    /// - iteration order is not specified.
    #[inline(always)]
    pub fn iter(&self) -> Iter<K, V> {
        Iter { inner: self.inner.iter() }
    }


    // clears the hashmap dropping the items within
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

}

pub struct Iter<'a, K, V> {
    inner: RawIter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> Clone for HashMap<K, V, S, A>
where K: Clone, V: Clone, S: Clone, A: Clone {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}


impl<Q, K, V, S, A> core::ops::Index<&Q> for HashMap<K, V, S, A>
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

impl<Q, K, V, S, A> core::ops::IndexMut<&Q> for HashMap<K, V, S, A>
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


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> core::fmt::Debug for HashMap<K, V, S, A> 
where K: core::fmt::Debug, V: core::fmt::Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::raw_hash_map::GROUP_SIZE;

    #[test]
    fn hm_basic() {
        let mut hm: HashMap<String, u32> = HashMap::with_cap(69);

        assert!(hm.get("hi").is_none());
        assert!(hm.remove("ho").is_none());

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

    #[test]
    fn hm_probe_length() {
        let mut hm: HashMapF<u32, u32, DumbHash> = HashMap::fnew();

        assert_eq!(hm.probe_length(&0),  (0, 0));
        assert_eq!(hm.probe_length(&69), (0, 0));

        hm.insert(0, 0);
        assert_eq!(hm.probe_length(&0), (1, 1));
        assert_eq!(hm.probe_length(&1), (1, 1));
        assert_eq!(hm.probe_length(&2), (1, 0));
        assert_eq!(hm.probe_length(&3), (1, 0));

        hm.insert(1, 1);
        assert_eq!(hm.probe_length(&32), (1, 2));

        for i in 2..GROUP_SIZE as u32 {
            hm.insert(2*i, 2*i);
        }
        assert_eq!(hm.size(), 2*GROUP_SIZE);
        assert_eq!(hm.probe_length(&32), (2, 2));


        assert_eq!(hm.resident(), hm.len());
        hm.remove(&1).unwrap();
        assert_eq!(hm.resident(), hm.len() + 1);
        assert_eq!(hm.probe_length(&32), (2, 1));

        hm.insert(32, 32);
        assert_eq!(hm.probe_length(&32), (1, 2));
    }

    #[test]
    fn hm_iter() {
        let hm: HashMapF<u32, i8, IdHash> = HashMap::fwith_cap(69);
        assert!(hm.iter().next().is_none());

        let mut hm: HashMapF<u32, i8, IdHash> = HashMap::fnew();
        assert!(hm.iter().next().is_none());

        for i in 0..2*GROUP_SIZE as u32 {
            hm.insert(i, i as i8 + 1);
        }

        let mut iter = hm.iter();
        for i in 0..2*GROUP_SIZE as u32 {
            let (k, v) = iter.next().unwrap();
            assert_eq!(*k, i);
            assert_eq!(*v, i as i8 + 1);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn hm_clone() {
        let mut hm1: HashMapF<String, Vec<i8>, ConstHash> = HashMap::fnew();

        assert!(hm1.iter().next().is_none());

        for i in 0..2*GROUP_SIZE as u32 {
            let mut v = Vec::new();
            for k in 0..i { v.push(k as i8) }

            hm1.insert(format!("{i}"), v);
        }

        let hm2 = hm1.clone();

        let iter1 = hm1.iter();
        let iter2 = hm2.iter();
        let mut iter = iter1.zip(iter2);
        for i in 0..2*GROUP_SIZE as u32 {
            let mut v = Vec::new();
            for k in 0..i { v.push(k as i8) }

            let ((k1, v1), (k2, v2)) = iter.next().unwrap();
            assert_eq!(*k1, format!("{i}"));
            assert_eq!(*v1, v);
            assert_eq!(*k1, *k2);
            assert_eq!(*v1, *v2);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn hm_get_or_insert() {
        use core::hash::{Hash, Hasher};
        use core::cell::Cell;


        struct MagicString {
            str: MagicStr<'static>,
        }

        impl PartialEq for MagicString {
            fn eq(&self, other: &Self) -> bool { self.str.value == other.str.value }
        }
        impl Eq for MagicString {}

        impl Hash for MagicString {
            fn hash<H: Hasher>(&self, state: &mut H) { self.str.value.hash(state) }
        }

        impl<'a> Borrow<MagicStr<'a>> for MagicString {
            fn borrow(&self) -> &MagicStr<'a> { &self.str }
        }


        struct MagicStr<'a> {
            value: &'static str,
            counter: Option<&'a Cell<u32>>,
        }

        impl<'a> PartialEq for MagicStr<'a> {
            fn eq(&self, other: &Self) -> bool { self.value == other.value }
        }
        impl<'a> Eq for MagicStr<'a> {}

        impl<'a> Hash for MagicStr<'a> {
            fn hash<H: Hasher>(&self, state: &mut H) { self.value.hash(state) }
        }

        impl<'a> From<&MagicStr<'a>> for MagicString {
            fn from(value: &MagicStr<'a>) -> Self {
                let counter = value.counter.as_ref().unwrap();
                counter.set(counter.get() + 1);

                MagicString { str: MagicStr {
                    value: value.value,
                    counter: None,
                }}
            }
        }


        let mut hm: HashMap<MagicString, i32> = HashMap::new();

        let counter = Cell::new(0);
        let str = |value: &'static str| -> MagicStr {
            MagicStr { counter: Some(&counter), value }
        };
        let string = |value: &'static str| -> MagicString {
            MagicString { str: MagicStr { counter: None, value } }
        };

        let inc = || { counter.set(counter.get() + 1); };

        assert_eq!(counter.get(), 0);
        assert_eq!(hm.resident(), 0);
        assert_eq!(hm.len(), 0);
        assert_eq!(hm.cap(), 0);

        // get_or_insert.
        let v = hm.get_or_insert(&str("hi"), 69);
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);


        // insert same key does nothing.

        let v = hm.get_or_insert(&str("hi"), 70);
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);

        let v = hm.get_or_insert_with(&str("hi"), || { inc(); 70 });
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);

        let v = hm.get_or_insert_with_key(&str("hi"),
            |k| { inc(); (k.into(), 70) });
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);

        let v = hm.kget_or_insert(string("hi"), 70);
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);

        let v = hm.kget_or_insert_with(string("hi"), || { inc(); 70 });
        assert_eq!(*v, 69);
        assert_eq!(counter.get(), 1);
        assert_eq!(hm.resident(), 1);
        assert_eq!(hm.len(), 1);


        // get_or_insert_with.
        let v = hm.get_or_insert_with(&str("ho"), || { inc(); 12 });
        assert_eq!(*v, 12);
        assert_eq!(counter.get(), 3);
        assert_eq!(hm.resident(), 2);
        assert_eq!(hm.len(), 2);

        // get_or_insert_with_key.
        let v = hm.get_or_insert_with_key(&str("hu"), |k| {
            assert_eq!(counter.get(), 3);
            inc();
            (k.into(), 8)
        });
        assert_eq!(*v, 8);
        assert_eq!(counter.get(), 5);
        assert_eq!(hm.resident(), 3);
        assert_eq!(hm.len(), 3);

        // kget_or_insert
        let v = hm.kget_or_insert(string("he"), 123);
        assert_eq!(*v, 123);
        assert_eq!(counter.get(), 5);
        assert_eq!(hm.resident(), 4);
        assert_eq!(hm.len(), 4);

        // kget_or_insert_with
        let v = hm.kget_or_insert_with(string("ha"), || { inc(); 231 });
        assert_eq!(*v, 231);
        assert_eq!(counter.get(), 6);
        assert_eq!(hm.resident(), 5);
        assert_eq!(hm.len(), 5);
    }

    #[test]
    fn hm_get_or_insert_present_on_grow() {
        let mut hm: HashMap<u32, u32> = HashMap::new();
        hm.kget_or_insert(0, 0);

        while hm.len() < hm.cap() {
            let i = hm.len() as u32;
            let x = *hm.kget_or_insert(i, i);
            assert_eq!(x, i);
        }

        let x = *hm.kget_or_insert(0, 1);
        assert_eq!(x, 0);
    }


    use crate::hash::HashFn;

    struct DumbHash;
    impl HashFn<u32> for DumbHash {
        type Seed = ();
        type Hash = u32;
        const DEFAULT_SEED: () = ();
        fn hash_with_seed(_: (), value: &u32) -> u32 { *value % 32 / 2 }
    }

    struct IdHash;
    impl HashFn<u32> for IdHash {
        type Seed = ();
        type Hash = u32;
        const DEFAULT_SEED: () = ();
        fn hash_with_seed(_: (), value: &u32) -> u32 { *value }
    }

    struct ConstHash;
    impl<T> HashFn<T> for ConstHash {
        type Seed = ();
        type Hash = u32;
        const DEFAULT_SEED: () = ();
        fn hash_with_seed(_: (), _: &T) -> u32 { 0 }
    }


    #[test]
    fn hm_drop_and_clear() {
        use core::cell::Cell;

        #[derive(PartialEq, Eq)]
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

        impl<'a> core::hash::Hash for Dropper<'a> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.ticket.hash(state)
            }
        }

        let counter = Cell::new(0);
        let d = |ticket: u32| {
            Dropper { ticket, counter: &counter }
        };

        // basic drop.
        let mut hm = HashMap::new();
        hm.insert(d(0), d(1));
        hm.insert(d(2), d(3));
        hm.insert(d(4), d(5));
        drop(hm);
        assert_eq!(counter.get(), 6);

        // clear.
        counter.set(0);
        let mut hm = HashMap::new();
        hm.insert(d(0), d(1));
        hm.insert(d(2), d(3));
        hm.insert(d(4), d(5));
        hm.clear();
        assert_eq!(hm.len(), 0);
        assert_eq!(counter.get(), 6);

        counter.set(0);
        hm.insert(d(0), d(1));
        hm.insert(d(2), d(3));
        hm.insert(d(4), d(5));
        drop(hm);
        assert_eq!(counter.get(), 6);

        counter.set(0);
        let mut hm = HashMap::new();
        hm.insert(d(0), d(1));
        hm.insert(d(2), d(3));
        hm.insert(d(4), d(5));
        hm.clear();

        counter.set(0);
        drop(hm);
        assert_eq!(counter.get(), 0);

        // use after clear.
        counter.set(0);
        let mut hm = HashMap::new();
        hm.insert(0, 1);
        hm.insert(2, 3);
        hm.insert(4, 5);
        hm.clear();
        hm.insert(6, 7);
        assert!(hm.get(&0).is_none());
        assert!(hm.get(&2).is_none());
        assert!(hm.get(&4).is_none());
        assert_eq!(hm.get(&6), Some(&7));
    }
}

