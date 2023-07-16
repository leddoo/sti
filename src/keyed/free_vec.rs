use crate::alloc::{Alloc, GlobalAlloc};
use crate::packed_option::PackedOption;

use super::{Key, KVec};


pub struct KFreeVec<K: Key, V, A: Alloc = GlobalAlloc> {
    vec: KVec<K, Entry<K, V>, A>,
    first_free: PackedOption<K>,
}

struct Entry<K: Key, V> {
    value: V,

    // points to self for last entry in free list.
    // -> is some for all free entries.
    next_free: PackedOption<K>,
}


impl<K: Key, V> KFreeVec<K, V, GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        KFreeVec { vec: KVec::new(), first_free: None.into() }
    }

}

impl<K: Key, V, A: Alloc> KFreeVec<K, V, A> {
    #[inline]
    pub fn try_alloc(&mut self, v: V) -> Result<K, ()> {
        let _ = v;
        unimplemented!()
    }

    #[inline]
    pub fn alloc(&mut self, v: V) -> K {
        if let Some(k) = self.first_free.take() {
            let e = &mut self.vec[k];

            let next = e.next_free.take().unwrap();
            if next != k {
                self.first_free = Some(next).into();
            }

            e.value = v;

            return k;
        }
        else {
            return self.vec.push(Entry { value: v, next_free: None.into() });
        }
    }

    #[inline]
    pub fn alloc_gen<Gen, Inc: Fn(Option<&V>) -> Gen>(&mut self, v: V) -> K {
        let _ = v;
        unimplemented!()
    }

    #[inline]
    pub fn free(&mut self, k: K) {
        let e = &mut self.vec[k];
        assert!(e.next_free.is_none());

        let prev_head = self.first_free.to_option();
        e.next_free = Some(prev_head.unwrap_or(k)).into();

        self.first_free = Some(k).into();
    }
}



impl<K: Key, V, A: Alloc> core::ops::Index<K> for KFreeVec<K, V, A> {
    type Output = V;

    #[inline(always)]
    fn index(&self, index: K) -> &Self::Output {
        let e = &self.vec[index];
        debug_assert!(e.next_free.is_none());
        return &e.value;
    }
}

impl<K: Key, V, A: Alloc> core::ops::IndexMut<K> for KFreeVec<K, V, A> {
    #[inline(always)]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let e = &mut self.vec[index];
        debug_assert!(e.next_free.is_none());
        return &mut e.value;
    }
}


