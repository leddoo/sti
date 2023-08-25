use core::borrow::Borrow;

use crate::alloc::Alloc;
use crate::hash::HashFnSeed;

// @temp
use core::mem::MaybeUninit;
use crate::vec::Vec;


pub(super) struct RawHashMap<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> {
    seed: S,
    alloc: A,

    groups: Vec<Group>,
    entries: Vec<MaybeUninit<(K, V)>>,
    cap: u32,
    len: u32,
}

#[repr(align(16))]
struct Group {
    entries: [u8; 16],
}


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> RawHashMap<K, V, S, A> {
    #[inline(always)]
    pub fn new(seed: S, alloc: A) -> Self {
        Self {
            seed,
            alloc,
            groups:  Vec::new(),
            entries: Vec::new(),
            cap: 0,
            len: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len as usize }


    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        unimplemented!()
    }


    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        unimplemented!()
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        unimplemented!()
    }
}

