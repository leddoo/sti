use core::ptr::NonNull;
use core::borrow::Borrow;
use core::marker::PhantomData;

use crate::num::OrdUtils;
use crate::hint::{likely, unlikely};
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

pub struct Slot<K, V> {
    pub key:   K,
    pub value: V,
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
    pub fn alloc(&self) -> &A { &self.alloc }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.num_groups as usize * Group::WIDTH
    }

    #[inline(always)]
    pub fn cap(&self) -> usize {
        self.num_groups as usize * load::EMPTY_PER_GROUP as usize
    }

    #[inline(always)]
    pub fn resident(&self) -> usize {
        self.cap() - self.empty as usize
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.used as usize
    }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.empty == 0 {
            self.grow();
            assert!(self.empty > 0);
        }

        let hash = self.seed.hash(&key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = group_first(hash, self.num_groups);
        loop {
            let group = unsafe { group_ref(self.groups, group_idx) };

            for i in group.match_hash(hash) {
                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                if slot.key == key {
                    return Some(core::mem::replace(&mut slot.value, value));
                }
            }

            if let Some(i) = group.match_free().next() {
                unsafe {
                    let slot = slot_ptr(slots, group_idx, i);
                    slot.write(Slot { key, value });
                    group.use_entry(i, hash);

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
        if self.used == 0 {
            return None;
        }

        let hash = self.seed.hash(key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = group_first(hash, self.num_groups);
        loop {
            let group = unsafe { group_ref(self.groups, group_idx) };

            for i in group.match_hash(hash) {
                unsafe {
                    let slot_ptr = slot_ptr(slots, group_idx, i);
                    if (*slot_ptr).key.borrow() == key {
                        let Slot { key, value } =  slot_ptr.read();

                        self.empty += group.free_entry(i);
                        self.used  -= 1;

                        return Some((key, value));
                    }
                }
            }

            if group.match_empty().any() {
                return None;
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }

    pub fn search<Q: ?Sized + Eq>(&self, key: &Q) -> Option<NonNull<V>>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if self.used == 0 {
            return None;
        }

        let hash = self.seed.hash(key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = group_first(hash, self.num_groups);
        loop {
            let group = unsafe { *group_ref(self.groups, group_idx) };

            for i in group.match_hash(hash) {
                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                if slot.key.borrow() == key {
                    return Some((&mut slot.value).into());
                }
            }

            if group.match_empty().any() {
                return None;
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }


    #[inline]
    pub fn get_or_insert<'q, Q: ?Sized + Eq, F>(&mut self, key: &'q Q, f: F) -> &mut V
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32>, F: FnOnce(&'q Q) -> (K, V) {
        let hash = self.seed.hash(key);

        let mut entry;
        if likely(self.used > 0) {
            entry = unsafe { self.entry(key, hash) };
            if entry.used {
                return unsafe { &mut (*entry.slot.as_ptr()).value };
            }

            if unlikely(self.empty == 0) {
                self.grow();
                entry = unsafe { self.entry(key, hash) };
            }
        }
        else {
            if self.empty == 0 {
                self.grow();
            }
            entry = unsafe { self.entry(key, hash) };
        }
        debug_assert!(entry.used == false);

        unsafe {
            let group = &mut *entry.group.as_ptr();
            let slot  = entry.slot.as_ptr();

            let (key, value) = f(key);
            slot.write(Slot { key, value });
            group.use_entry(entry.i, hash);

            self.empty -= 1;
            self.used  += 1;

            &mut (*slot).value
        }
    }


    pub fn clone_in<A2>(&self, alloc: A2) -> RawHashMap<K, V, S, A2>
    where K: Clone, V: Clone, S: Clone, A2: Alloc
    {
        // allocate uninitialized hash map with same capacity.
        let mut result = {
            let layout = Self::layout(self.num_groups).unwrap();
            let data = alloc.alloc(layout).expect("allocation failed");

            RawHashMap {
                seed: self.seed.clone(),
                alloc,

                groups: data.cast(),
                num_groups: self.num_groups,
                empty: self.empty,
                // `used` is set once data was cloned successfully.
                // this prevents `drop` from accessing uninit data,
                // if `K/V::clone` panic.
                used: 0,

                phantom: PhantomData,
            }
        };

        // initialize groups.
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.groups.as_ptr(),
                result.groups.as_ptr(),
                self.num_groups as usize);
        }

        // clone slots.
        let src_slots = Self::slots_ptr(self.groups,   self.num_groups);
        let dst_slots = Self::slots_ptr(result.groups, result.num_groups);
        for group_idx in 0..self.num_groups as usize {
            let group = unsafe { *group_ref(result.groups, group_idx) };

            for i in group.match_used() {
                unsafe {
                    let src = slot_ptr(src_slots, group_idx, i);
                    let dst = slot_ptr(dst_slots, group_idx, i);
                    dst.write(Slot {
                        key:   (*src).key.clone(),
                        value: (*src).value.clone(),
                    });
                }
            }
        }

        // finalize.
        result.used = self.used;

        result
    }


    pub fn probe_length<Q: ?Sized + Eq>(&self, key: &Q) -> (usize, usize)
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if self.used == 0 {
            return (0, 0);
        }

        let hash = self.seed.hash(key);

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut groups_visited = 0;
        let mut keys_compared  = 0;

        let mut group_idx = group_first(hash, self.num_groups);
        loop {
            let group = unsafe { *group_ref(self.groups, group_idx) };

            groups_visited += 1;

            for i in group.match_hash(hash) {
                keys_compared += 1;

                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                if slot.key.borrow() == key {
                    return (groups_visited, keys_compared);
                }
            }

            if group.match_empty().any() {
                return (groups_visited, keys_compared);
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }


    #[inline(always)]
    pub fn iter(&self) -> RawIter<K, V> {
        RawIter {
            groups: self.groups,
            slots:  Self::slots_ptr(self.groups, self.num_groups),
            num_groups: self.num_groups,

            group_idx:
                if self.used > 0 { 0 }
                else { self.num_groups as usize },

            bitmask:
                if self.num_groups > 0 {
                    let group = unsafe { *group_ref(self.groups, 0) };
                    group.match_used()
                }
                else { Bitmask::none() },

            phantom: PhantomData,
        }
    }


    fn resize(&mut self, new_num_groups: u32) {
        let layout = Self::layout(new_num_groups).expect("capacity overflow");
        let data = self.alloc.alloc(layout).expect("allocation failed");

        let new_groups: NonNull<Group> = data.cast();

        // initialize groups:
        for i in 0..new_num_groups as usize {
            unsafe { new_groups.as_ptr().add(i).write(Group::empty()) }
        }

        let old_groups = self.groups;
        let old_slots = Self::slots_ptr(old_groups, self.num_groups);
        let old_num_groups = self.num_groups;
        let old_used = self.used;

        self.groups = new_groups;
        self.num_groups = new_num_groups;
        self.empty = load::EMPTY_PER_GROUP * new_num_groups;
        self.used  = 0;

        if old_num_groups != 0 {
            if old_used != 0 {
                for group_idx in 0..old_num_groups as usize {
                    let group = unsafe { *group_ref(old_groups, group_idx) };

                    for i in group.match_used() {
                        let Slot { key, value } = unsafe {
                            slot_ptr(old_slots, group_idx, i).read()
                        };

                        debug_assert!(self.empty > 0);

                        let none = self.insert(key, value);
                        debug_assert!(none.is_none());
                    }
                }
            }

            let layout = Self::layout(old_num_groups).unwrap();
            unsafe { self.alloc.free(old_groups.cast(), layout) }
        }
        else { debug_assert_eq!(old_used, 0) }
    }

    #[inline]
    fn grow(&mut self) {
        self.resize(
            self.num_groups.checked_mul(2).expect("capacity overflow")
            .at_least(1));
    }


    unsafe fn entry<Q: ?Sized + Eq>(&self, key: &Q, hash: u32) -> RawEntry<K, V>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        let slots = Self::slots_ptr(self.groups, self.num_groups);

        let mut group_idx = group_first(hash, self.num_groups);
        loop {
            let group = unsafe { group_ref(self.groups, group_idx) };

            for i in group.match_hash(hash) {
                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                if slot.key.borrow() == key {
                    return RawEntry {
                        group: group.into(),
                        slot:  slot.into(),
                        used:  true,
                        i,
                    };
                }
            }

            if let Some(i) = group.match_free().next() {
                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                return RawEntry {
                    group: group.into(),
                    slot:  slot.into(),
                    used:  false,
                    i,
                };
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }


    #[inline(always)]
    fn layout(num_groups: u32) -> Option<Layout> {
        let num_groups: usize = num_groups.try_into().unwrap();
        let num_slots = num_groups.checked_mul(Group::WIDTH)?;
        cat_join(
            Layout::array::<Group>(num_groups).ok()?,
            Layout::array::<Slot<K, V>>(num_slots).ok()?)
    }

    #[inline(always)]
    fn slots_ptr(groups: NonNull<Group>, num_groups: u32) -> NonNull<Slot<K, V>> {
        unsafe { NonNull::new_unchecked(cat_next_mut(groups.as_ptr(), num_groups as usize)) }
    }
}

#[inline(always)]
unsafe fn group_ref<'a>(groups: NonNull<Group>, idx: usize) -> &'a mut Group {
    unsafe { &mut *groups.as_ptr().add(idx) }
}

#[inline(always)]
fn slot_ptr<K, V>(slots: NonNull<Slot<K, V>>, group_idx: usize, sub_idx: usize) -> *mut Slot<K, V> {
    unsafe { slots.as_ptr().add(Group::WIDTH*group_idx + sub_idx) }
}


unsafe impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc>
    Sync for RawHashMap<K, V, S, A> where K: Sync, V: Sync, S: Sync, A: Sync {}

unsafe impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc>
    Send for RawHashMap<K, V, S, A> where K: Send, V: Send, S: Send, A: Send {}


impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> Drop for RawHashMap<K, V, S, A> {
    fn drop(&mut self) {
        if self.num_groups != 0 {
            if self.used != 0 {
                let slots = Self::slots_ptr(self.groups, self.num_groups);

                // drop slots.
                for group_idx in 0..self.num_groups as usize {
                    let group = unsafe { group_ref(self.groups, group_idx) };

                    for i in group.match_used() { unsafe {
                        core::ptr::drop_in_place(slot_ptr(slots, group_idx, i))
                    }}
                }
            }

            let layout = Self::layout(self.num_groups).unwrap();
            unsafe { self.alloc.free(self.groups.cast(), layout) }
        }
        else { debug_assert_eq!(self.used, 0) }
    }
}

impl<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> Clone for RawHashMap<K, V, S, A>
where K: Clone, V: Clone, S: Clone, A: Clone {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.clone_in(self.alloc.clone())
    }
}


pub struct RawIter<'a, K, V> {
    groups: NonNull<Group>,
    slots:  NonNull<Slot<K, V>>,
    num_groups: u32,

    group_idx: usize,
    bitmask: Bitmask,

    phantom: PhantomData<&'a Slot<K, V>>,
}

impl<'a, K, V> Iterator for RawIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(i) = self.bitmask.next() {
                let slot = unsafe { &*slot_ptr(self.slots, self.group_idx, i) };
                return Some((&slot.key, &slot.value));
            }

            self.group_idx += 1;

            if self.group_idx < self.num_groups as usize {
                let group = unsafe { *group_ref(self.groups, self.group_idx) };
                self.bitmask = group.match_used();
            }
            else {
                return None;
            }
        }
    }
}


struct RawEntry<K, V> {
    group: NonNull<Group>,
    slot:  NonNull<Slot<K, V>>,
    used:  bool,
    i:     usize,
}


mod load {
    use super::Group;

    // max load: 14/16 = 7/8.

    #[inline(always)]
    pub const fn num_groups_for_cap(cap: u32) -> Option<u32> {
        const W: u32 = Group::WIDTH as u32;

        // max cap we can support:
        //  cap*8/7 + w-1 <= u32::MAX
        //  cap <= (u32::MAX - w-1)*7/8
        const MAX_CAP: u32 = (((u32::MAX - (W-1)) as u64)*7/8) as u32;

        const _MAX_GROUPS: u32 = (MAX_CAP + MAX_CAP/7 + (W-1)) / W;
        const _: () = assert!(MAX_CAP <= _MAX_GROUPS*W);

        if cap <= MAX_CAP {
            let groups = (cap + cap/7 + (W-1)) / W;
            Some(groups)
        }
        else { None }
    }

    const _: () = assert!(Group::WIDTH % 8 == 0);
    pub const EMPTY_PER_GROUP: u32 = Group::WIDTH as u32 * 7/8;
}


#[inline(always)]
fn group_first(hash: u32, num_groups: u32) -> usize {
    let result = ((hash as u64 * num_groups as u64) >> 32) as usize;
    debug_assert!(result < num_groups as usize);
    return result;
}

#[inline(always)]
fn group_next(i: &mut usize, num_groups: u32) {
    debug_assert!(*i < num_groups as usize);

    *i += 1;
    if *i >= num_groups as usize {
        *i = 0;
    }

    debug_assert!(*i < num_groups as usize);
}


use group_u64::*;
mod group_u64 {
    #[derive(Clone, Copy)]
    pub(super) struct Group(u64);

    #[derive(Clone, Copy)]
    pub(super) struct Bitmask(u64);

    #[inline(always)]
    const fn splat(value: u8) -> u64 {
        u64::from_ne_bytes([value; 8])
    }

    impl Group {
        pub const WIDTH: usize = 8;

        const EMPTY:     u8  = 0xff;
        const TOMBSTONE: u8  = 0x80;
        const HASH_MASK: u32 = 0x7f;


        #[inline(always)]
        pub const fn empty() -> Group {
            Self(splat(Self::EMPTY))
        }

        #[inline(always)]
        const fn mask_hash(hash: u32) -> u8 {
            (hash & Self::HASH_MASK) as u8
        }

        #[inline(always)]
        pub fn match_hash(&self, hash: u32) -> Bitmask {
            // 0x00 for all matching bytes.
            let mask = self.0 ^ splat(Self::mask_hash(hash));
            // https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
            let zero_or_high = mask.wrapping_sub(splat(1));
            let not_high = !mask & splat(0x80);
            let mask = zero_or_high & not_high;
            Bitmask(mask)
        }

        #[inline(always)]
        pub fn match_empty(&self) -> Bitmask {
            // check high bit and second highest bit set.
            // only empty & tombstone have the high bit.
            // and tombstone only has the high bit.
            let mask = self.0 & (self.0 << 1);
            let mask = mask & splat(0x80);
            Bitmask(mask)
        }

        #[inline(always)]
        pub fn match_free(&self) -> Bitmask {
            // only empty & tombstone have the high bit.
            let mask = self.0 & splat(0x80);
            Bitmask(mask)
        }

        #[inline(always)]
        pub fn match_used(&self) -> Bitmask {
            // used entries don't have the high bit.
            let mask = (self.0 & splat(0x80)) ^ splat(0x80);
            Bitmask(mask)
        }


        #[inline(always)]
        fn set(&mut self, idx: usize, value: u8) {
            let mut this = self.0.to_ne_bytes();
            this[idx] = value;
            self.0 = u64::from_ne_bytes(this);
        }

        #[inline(always)]
        pub fn use_entry(&mut self, idx: usize, hash: u32) {
            self.set(idx, Self::mask_hash(hash))
        }

        #[inline(always)]
        pub fn free_entry(&mut self, idx: usize) -> u32 {
            if self.match_empty().any() {
                self.set(idx, Self::EMPTY);
                return 1;
            }
            else {
                self.set(idx, Self::TOMBSTONE);
                return 0;
            }
        }
    }

    impl Bitmask {
        #[inline(always)]
        pub fn none() -> Self { Bitmask(0) }

        #[inline(always)]
        pub fn any(&self) -> bool { self.0 != 0 }
    }

    impl Iterator for Bitmask {
        type Item = usize;

        #[inline(always)]
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 != 0 {
                let i = self.0.trailing_zeros() / 8;
                self.0 &= self.0 - 1;
                return Some(i as usize);
            }
            return None;
        }
    }
}


#[cfg(test)]
pub(crate) const GROUP_SIZE: usize = Group::WIDTH;

