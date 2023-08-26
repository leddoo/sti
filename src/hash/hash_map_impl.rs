use core::ptr::NonNull;
use core::borrow::Borrow;
use core::marker::PhantomData;

use crate::num::OrdUtils;
use crate::alloc::*;
use crate::hash::HashFnSeed;

pub(super) struct RawHashMap<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> {
    seed: S,
    alloc: A,

    groups: NonNull<Group>,
    num_groups: u32,
    empty: u32,
    used:  u32,

    phantom: PhantomData<(K, V)>,
}

#[derive(Clone, Copy)]
#[repr(align(16))]
struct Group {
    entries: [u8; 16],
}

pub struct Slot<K, V> {
    pub key:   K,
    pub value: V,
}


mod load {
    // max load: 14/16 = 7/8.

    #[inline(always)]
    pub const fn num_groups_for_cap(cap: u32) -> Option<u32> {
        // max cap we can support:
        //  cap*8/7 + 15 <= u32::MAX
        //  cap <= (u32::MAX - 15)*7/8
        const MAX_CAP: u32 = ((u32::MAX as u64 - 15)*7/8) as u32;
        const _MAX_GROUPS: u32 = (MAX_CAP + MAX_CAP/7 + 15) / 16;
        const _CHECK: () = assert!(MAX_CAP <= _MAX_GROUPS*16);

        if cap <= MAX_CAP {
            let groups = (cap + cap/7 + 15) / 16;
            Some(groups)
        }
        else { None }
    }

    #[inline(always)]
    pub const fn num_empty_for_groups(num_groups: u32) -> u32 {
        num_groups*14
    }
}

#[inline(always)]
const fn reduce_hash(hash: u32, n: u32) -> u32 {
    ((hash as u64 * n as u64) >> 32) as u32
}

#[inline(always)]
fn group_next(i: &mut usize, num_groups: u32) {
    *i += 1;
    if *i >= num_groups as usize {
        *i = 0;
    }
}


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> RawHashMap<K, V, S, A> {
    #[inline(always)]
    pub fn new(seed: S, alloc: A) -> Self {
        Self {
            seed,
            alloc,
            groups: NonNull::dangling(),
            num_groups: 0,
            empty: 0,
            used:  0,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn with_cap(cap: usize, seed: S, alloc: A) -> Self {
        let mut this = Self::new(seed, alloc);
        this.resize(load::num_groups_for_cap(
            cap.try_into().expect("capacity overflow")).expect("capacity overflow"));
        return this;
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.used as usize }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.empty == 0 {
            self.resize(
                self.num_groups.checked_mul(2).expect("capacity overflow")
                .at_least(1));

            assert!(self.empty > 0);
        }


        let hash = self.seed.hash(&key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = reduce_hash(hash, self.num_groups) as usize;
        loop {
            let group_ptr = Self::group_ptr(self.groups, group_idx);
            let group = unsafe { *group_ptr };

            for i in group.hash_matches(hash) {
                let slot = unsafe { &mut *slots.add(16*group_idx + i) };
                if slot.key == key {
                    return Some(core::mem::replace(&mut slot.value, value));
                }
            }

            if let Some(i) = group.empty_matches().next() {
                unsafe {
                    slots.add(16*group_idx + i).write(Slot { key, value });
                    (*group_ptr).use_entry(i, hash);

                    self.empty -= 1;
                    self.used  += 1;

                    return None;
                }
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }

    pub fn remove<Q: ?Sized + Eq>(&mut self, key: &Q) -> Option<(K, V)>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        let hash = self.seed.hash(key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = reduce_hash(hash, self.num_groups) as usize;
        loop {
            let group_ptr = Self::group_ptr(self.groups, group_idx);
            let group = unsafe { *group_ptr };

            for i in group.hash_matches(hash) {
                unsafe {
                    let slot_ptr = slots.add(16*group_idx + i);
                    if (*slot_ptr).key.borrow() == key {
                        let Slot { key, value } =  slot_ptr.read();

                        self.empty += (*group_ptr).free_entry(i);
                        self.used  -= 1;

                        return Some((key, value));
                    }
                }
            }

            if group.any_empty() {
                return None;
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }

    pub fn search<Q: ?Sized + Eq>(&self, key: &Q) -> Option<NonNull<V>>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        let hash = self.seed.hash(key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = reduce_hash(hash, self.num_groups) as usize;
        loop {
            let group = unsafe { *Self::group_ptr(self.groups, group_idx) };

            for i in group.hash_matches(hash) {
                let slot = unsafe { &mut *slots.add(16*group_idx + i) };
                if slot.key.borrow() == key {
                    return Some((&mut slot.value).into());
                }
            }

            if group.any_empty() {
                return None;
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }


    fn resize(&mut self, num_groups: u32) {
        let layout = Self::layout(num_groups).expect("capacity overflow");
        let data = self.alloc.alloc(layout).expect("allocation failed");

        let groups: *mut Group      = data.cast().as_ptr();
        let slots:  *mut Slot<K, V> = unsafe { cat_next_mut(groups, num_groups as usize) };

        // initialize groups:
        for i in 0..num_groups as usize {
            unsafe { groups.add(i).write(Group::empty()) }
        }

        if self.used != 0 {
            let _ = slots;
            unimplemented!()
        }

        if self.num_groups != 0 {
            unimplemented!()
        }

        self.groups = unsafe { NonNull::new_unchecked(groups) };
        self.num_groups = num_groups;
        self.empty = load::num_empty_for_groups(num_groups);
        self.used  = 0;
    }


    #[inline(always)]
    fn layout(num_groups: u32) -> Option<Layout> {
        let num_groups: usize = num_groups.try_into().unwrap();
        let num_slots = num_groups.checked_mul(16)?;
        cat_join(
            Layout::array::<Group>(num_groups).ok()?,
            Layout::array::<Slot<K, V>>(num_slots).ok()?)
    }

    #[inline(always)]
    fn group_ptr(groups: NonNull<Group>, idx: usize) -> *mut Group {
        unsafe { groups.as_ptr().add(idx) }
    }

    #[inline(always)]
    fn slots_ptr(groups: NonNull<Group>, num_groups: u32) -> *mut Slot<K, V> {
        unsafe { cat_next_mut(groups.as_ptr(), num_groups as usize) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> Drop for RawHashMap<K, V, S, A> {
    fn drop(&mut self) {
        if self.num_groups != 0 {
            let slots = Self::slots_ptr(self.groups, self.num_groups);

            // drop slots.
            for group_idx in 0..self.num_groups as usize {
                let group = unsafe { *Self::group_ptr(self.groups, group_idx) };
                group.iter_used(|i| { unsafe {
                    core::ptr::drop_in_place(&mut *slots.add(16*group_idx + i))
                }});
            }

            let layout = Self::layout(self.num_groups).unwrap();
            unsafe { self.alloc.free(self.groups.cast(), layout) }
        }
    }
}

impl Group {
    const EMPTY:     u8 = 0x80;
    const TOMBSTONE: u8 = 0xff;

    #[inline(always)]
    const fn empty() -> Group {
        Group { entries: [Self::EMPTY; 16] }
    }

    #[inline(always)]
    fn use_entry(&mut self, idx: usize, hash: u32) {
        self.entries[idx] = (hash & 0x7f) as u8;
    }

    #[inline(always)]
    fn free_entry(&mut self, idx: usize) -> u32 {
        if self.any_empty() {
            self.entries[idx] = Self::EMPTY;
            return 1;
        }
        else {
            self.entries[idx] = Self::TOMBSTONE;
            return 0;
        }
    }
}

mod group_scalar {
    use super::Group;

    pub struct Matches {
        group:  Group,
        cursor: usize,
        query:  u8,
    }

    impl Group {
        #[inline(always)]
        pub fn any_empty(&self) -> bool {
            for e in self.entries {
                if e == Self::EMPTY {
                    return true;
                }
            }
            false
        }

        #[inline(always)]
        pub fn hash_matches(&self, hash: u32) -> Matches {
            Matches {
                group:  *self,
                cursor: 0,
                query:  (hash & 0x7f) as u8,
            }
        }

        #[inline(always)]
        pub fn empty_matches(&self) -> Matches {
            Matches {
                group:  *self,
                cursor: 0,
                query:  Self::EMPTY,
            }
        }

        #[inline(always)]
        pub fn iter_used<F: FnMut(usize)>(&self, mut f: F) {
            for i in 0..self.entries.len() {
                if self.entries[i] & 0x80 == 0 {
                    f(i);
                }
            }
        }
    }

    impl Iterator for Matches {
        type Item = usize;

        #[inline(always)]
        fn next(&mut self) -> Option<Self::Item> {
            while self.cursor < self.group.entries.len() {
                let at = self.cursor;
                self.cursor += 1;

                if self.group.entries[at] == self.query {
                    return Some(at);
                }
            }
            None
        }
    }
}

