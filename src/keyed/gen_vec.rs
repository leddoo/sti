use core::num::NonZeroU32;

use crate::alloc::{Alloc, GlobalAlloc};
use crate::packed_option::PackedOption;
use crate::keyed::{Key, KVec};


pub trait Gen: Copy + PartialEq {
    const INIT: Self;

    fn inc(self) -> Self;
}

impl Gen for NonZeroU32 {
    const INIT: Self = NonZeroU32::MIN;

    #[inline]
    fn inc(self) -> Self {
        self.checked_add(1).unwrap_or(Self::INIT)
    }
}


pub struct KGenVec<K: Key, G: Gen, V, A: Alloc = GlobalAlloc> {
    inner: KVec<K, Entry<K, G, V>, A>,
    first_free: PackedOption<K>,
}

enum Entry<K: Key, G, V> {
    Free((G, PackedOption<K>)),
    Used((G, V)),
}

impl<K: Key, G: Gen, V> KGenVec<K, G, V> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::new_in(GlobalAlloc)
    }
}

impl<K: Key, G: Gen, V, A: Alloc> KGenVec<K, G, V, A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            inner: KVec::new_in(alloc),
            first_free: PackedOption::NONE,
        }
    }


    #[inline(always)]
    pub fn alloc(&self) -> &A { self.inner.alloc() }

    #[inline]
    pub fn cap(&self) -> usize { self.inner.cap() }


    #[track_caller]
    #[inline]
    pub fn insert(&mut self, v: V) -> (K, G) {
        if let Some(k) = self.first_free.to_option() {
            let entry = &mut self.inner[k];

            let Entry::Free((g, next_free)) = *entry else { unreachable!() };
            *entry = Entry::Used((g, v));

            self.first_free = next_free;

            return (k, g);
        }
        else {
            let g = G::INIT;
            let k = self.inner.push(Entry::Used((g, v)));
            return (k, g);
        }
    }

    #[track_caller]
    #[inline]
    pub fn kremove(&mut self, key: K) -> (G, V) {
        let entry = &mut self.inner[key];

        let Entry::Used((g, _)) = entry else { unreachable!() };
        let free = Entry::Free((g.inc(), self.first_free));
        let used = core::mem::replace(entry, free);
        let Entry::Used((g, v)) = used else { unreachable!() };

        self.first_free = Some(key).into();

        return (g, v);
    }

    #[inline(always)]
    pub fn kget(&self, key: K) -> Option<(G, &V)> {
        let Some(entry) = self.inner.get(key) else { return None };
        let Entry::Used((g, v)) = entry else { return None };
        return Some((*g, v));
    }

    #[inline(always)]
    pub fn get(&self, key: K, gen: G) -> Option<&V> {
        let Some((g, v)) = self.kget(key) else { return None };
        if g == gen { Some(v) } else { None }
    }

    #[inline(always)]
    pub fn kget_mut(&mut self, key: K) -> Option<(G, &mut V)> {
        let Some(entry) = self.inner.get_mut(key) else { return None };
        let Entry::Used((g, v)) = entry else { return None };
        return Some((*g, v));
    }

    #[inline(always)]
    pub fn get_mut(&mut self, key: K, gen: G) -> Option<&mut V> {
        let Some((g, v)) = self.kget_mut(key) else { return None };
        if g == gen { Some(v) } else { None }
    }

    #[inline(always)]
    pub fn next_key(&self) -> K {
        if let Some(k) = self.first_free.to_option() { k }
        else { self.inner.next_key() }
    }


    #[track_caller]
    #[inline(always)]
    pub fn check_handle(&self, key: K, gen: G) {
        self.get(key, gen).expect("invalid key/gen");
    }

    #[track_caller]
    #[inline(always)]
    pub fn gen(&self, key: K) -> G {
        match &self.inner[key] {
            Entry::Free((g, _)) |
            Entry::Used((g, _)) => *g,
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn inc_gen(&mut self, key: K) -> G {
        match &mut self.inner[key] {
            Entry::Free((g, _)) |
            Entry::Used((g, _)) => {
                *g = g.inc();
                return *g;
            }
        }
    }
}

impl<K: Key, G: Gen, V, A: Alloc> core::ops::Index<K> for KGenVec<K, G, V, A> {
    type Output = V;

    #[track_caller]
    #[inline]
    fn index(&self, index: K) -> &Self::Output {
        self.kget(index).expect("invalid key").1
    }
}

impl<K: Key, G: Gen, V, A: Alloc> core::ops::IndexMut<K> for KGenVec<K, G, V, A> {
    #[track_caller]
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.kget_mut(index).expect("invalid key").1
    }
}


impl<K: Key, G: Gen, V, A: Alloc> core::ops::Index<(K, G)> for KGenVec<K, G, V, A> {
    type Output = V;

    #[track_caller]
    #[inline]
    fn index(&self, (k, g): (K, G)) -> &Self::Output {
        self.get(k, g).expect("invalid key/gen")
    }
}

impl<K: Key, G: Gen, V, A: Alloc> core::ops::IndexMut<(K, G)> for KGenVec<K, G, V, A> {
    #[track_caller]
    #[inline]
    fn index_mut(&mut self, (k, g): (K, G)) -> &mut Self::Output {
        self.get_mut(k, g).expect("invalid key/gen")
    }
}

