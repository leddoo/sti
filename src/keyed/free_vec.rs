use crate::alloc::{Alloc, GlobalAlloc};
use crate::packed_option::PackedOption;

use super::{Key, KVec, KRange};


pub struct KFreeVec<K: Key, V, A: Alloc = GlobalAlloc> {
    entries: KVec<K, Entry<K, V>, A>,
    first_free: PackedOption<K>,
}

enum Entry<K: Key, V> {
    Free { next_free: K },
    Used(V),
}


impl<K: Key, V> KFreeVec<K, V, GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        KFreeVec { entries: KVec::new(), first_free: None.into() }
    }

}

impl<K: Key, V, A: Alloc> KFreeVec<K, V, A> {
    #[inline(always)]
    pub fn len(&self) -> usize { self.entries.len() }

    #[inline(always)]
    pub fn range(&self) -> KRange<K> { self.entries.range() }


    #[inline]
    pub fn alloc(&mut self, v: V) -> K {
        if let Some(k) = self.first_free.take().to_option() {
            let e = &mut self.entries[k];

            let Entry::Free { next_free } = *e else { unreachable!() };
            if next_free != k {
                self.first_free = Some(next_free).into();
            }

            *e = Entry::Used(v);

            return k;
        }
        else {
            return self.entries.push(Entry::Used(v));
        }
    }

    #[track_caller]
    #[inline]
    pub fn free(&mut self, k: K) -> V {
        let e = &mut self.entries[k];

        let next_free = self.first_free.to_option().unwrap_or(k);
        self.first_free = Some(k).into();

        let v = core::mem::replace(e, Entry::Free { next_free });
        let Entry::Used(v) = v else { unreachable!() };
        return v;
    }


    #[inline]
    pub fn next_key(&self) -> K {
        self.first_free.to_option()
        .unwrap_or_else(|| self.entries.next_key())
    }

    #[inline]
    pub fn alloc_with(&mut self, f: impl FnOnce(&mut Self, K) -> V) -> K {
        let k = self.next_key();
        let v = f(self, k);
        let k2 = self.alloc(v);
        assert!(k == k2);
        return k;
    }


    #[inline]
    pub fn retain(&mut self, mut f: impl FnMut(K, &V) -> bool) {
        for (id, entry) in self.entries.iter_mut() {
            if let Entry::Used(v) = entry {
                if !f(id, v) {
                    let next_free = self.first_free.to_option().unwrap_or(id);
                    *entry = Entry::Free { next_free };
                    self.first_free = Some(id).into();
                }
            }
        }
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> Option<&V> {
        self.entries.get(k)
        .and_then(|e| match e {
            Entry::Free{..} => None,
            Entry::Used(v)  => Some(v)
        })
    }

    #[inline(always)]
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        self.entries.get_mut(k)
        .and_then(|e| match e {
            Entry::Free{..} => None,
            Entry::Used(v)  => Some(v)
        })
    }
}



impl<K: Key, V, A: Alloc> core::ops::Index<K> for KFreeVec<K, V, A> {
    type Output = V;

    #[track_caller]
    #[inline(always)]
    fn index(&self, index: K) -> &Self::Output {
        let e = &self.entries[index];
        let Entry::Used(v) = e else { unreachable!() };
        return v;
    }
}

impl<K: Key, V, A: Alloc> core::ops::IndexMut<K> for KFreeVec<K, V, A> {
    #[track_caller]
    #[inline(always)]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let e = &mut self.entries[index];
        let Entry::Used(v) = e else { unreachable!() };
        return v;
    }
}


