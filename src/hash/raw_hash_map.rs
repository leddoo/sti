use crate::alloc::{Alloc, Layout, cat_join, cat_next_mut};
use crate::mem::{NonNull, PhantomData};
use crate::hash::HashFnSeed;
use crate::hint::unlikely;
use core::borrow::Borrow;


pub(super) struct RawHashMap<K: Eq, V, S: HashFnSeed<K, Hash=u32>, A: Alloc> {
    seed: S,
    alloc: A,

    groups: NonNull<Group>,
    num_groups: u32, // valid for use in `Self::layout(num_groups)`.
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
        let cap = cap.try_into().expect("capacity overflow");
        let num_groups = load::num_groups_for_cap(cap).expect("capacity overflow");
        unsafe { this.resize(num_groups) };
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


    pub fn reserve(&mut self, min_cap: usize) {
        let cap = min_cap.try_into().expect("capacity overflow");
        let num_groups = load::num_groups_for_cap(cap).expect("capacity overflow");
        if num_groups > self.num_groups {
            unsafe { self.resize(num_groups) };
        }
    }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if unlikely(self.empty == 0) {
            self.grow();
        }

        let hash = self.seed.hash(&key);

        let entry = unsafe { self.entry(&key, hash) };
        if !entry.used {
            unsafe {
                entry.slot.write(Slot { key, value });
                (*entry.group).use_entry(entry.i, hash);
            }

            self.empty -= 1;
            self.used  += 1;

            return None;
        }
        else {
            let slot = unsafe { &mut *entry.slot };
            return Some(core::mem::replace(&mut slot.value, value));
        }
    }

    pub fn remove<Q: ?Sized + Eq>(&mut self, key: &Q) -> Option<(K, V)>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if self.used == 0 {
            return None;
        }

        let hash = self.seed.hash(key);

        let entry = unsafe { self.entry(key, hash) };
        if entry.used {
            unsafe {
                self.empty += (*entry.group).free_entry(entry.i);
                self.used  -= 1;

                let Slot { key, value } = entry.slot.read();
                return Some((key, value));
            }
        }
        else { None }
    }

    pub fn search<Q: ?Sized + Eq>(&self, key: &Q) -> Option<NonNull<V>>
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32> {
        if self.used == 0 {
            return None;
        }

        let hash = self.seed.hash(key);

        let entry = unsafe { self.entry(key, hash) };
        if entry.used {
            let v = unsafe { &mut (*entry.slot).value };
            return Some(v.into());
        }
        else { None }
    }


    #[inline]
    pub fn get_or_insert<'q, Q: ?Sized + Eq, F>(&mut self, key: &'q Q, f: F) -> &mut V
    where K: Borrow<Q>, S: HashFnSeed<Q, Hash=u32>, F: FnOnce(&'q Q) -> (K, V) {
        if unlikely(self.empty == 0) {
            self.grow();
        }

        let hash = self.seed.hash(key);

        let entry = unsafe { self.entry(key, hash) };
        if entry.used {
            return unsafe { &mut (*entry.slot).value };
        }

        unsafe {
            let (key, value) = f(key);
            entry.slot.write(Slot { key, value });
            (*entry.group).use_entry(entry.i, hash);

            self.empty -= 1;
            self.used  += 1;

            &mut (*entry.slot).value
        }
    }

    #[inline]
    pub fn kget_or_insert<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        if unlikely(self.empty == 0) {
            self.grow();
        }

        let hash = self.seed.hash(&key);

        let entry = unsafe { self.entry(&key, hash) };
        if entry.used {
            return unsafe { &mut (*entry.slot).value };
        }

        unsafe {
            let value = f();
            entry.slot.write(Slot { key, value });
            (*entry.group).use_entry(entry.i, hash);

            self.empty -= 1;
            self.used  += 1;

            &mut (*entry.slot).value
        }
    }


    pub fn retain(&mut self, mut f: impl FnMut(&K, &V) -> bool) {
        if self.used == 0 {
            return;
        }

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        for group_idx in 0..self.num_groups as usize {
            let group = unsafe { group_ref(self.groups, group_idx) };

            for i in group.match_used() { unsafe {
                let slot = slot_ptr(slots, group_idx, i);

                if !f(&(*slot).key, &(*slot).value) {
                    self.empty += group.free_entry(i);
                    self.used  -= 1;
                    core::ptr::drop_in_place(slot);
                }
            }}
        }
    }


    pub fn clone_in<A2>(&self, alloc: A2) -> RawHashMap<K, V, S, A2>
    where K: Clone, V: Clone, S: Clone, A2: Alloc
    {
        // allocate uninitialized hash map with same capacity.
        let mut result = {
            // `self.num_groups` is always valid for `Self::layout`.
            let layout = unsafe { Self::layout(self.num_groups).unwrap_unchecked() };

            let data = alloc.alloc(layout).expect("oom");

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

    #[inline(always)]
    pub fn copy(&self) -> RawHashMap<K, V, S, A>
    where K: Copy, V: Copy, S: Clone, A: Clone {
        self.copy_in(self.alloc.clone())
    }

    pub fn copy_in<A2>(&self, alloc: A2) -> RawHashMap<K, V, S, A2>
    where K: Copy, V: Copy, S: Clone, A2: Alloc
    {
        // `self.num_groups` is always valid for `Self::layout`.
        let layout = unsafe { Self::layout(self.num_groups).unwrap_unchecked() };

        let data = alloc.alloc(layout).expect("oom");
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.groups.as_ptr() as *const u8,
                data.as_ptr(),
                layout.size());
        }

        RawHashMap {
            seed: self.seed.clone(),
            alloc,

            groups: data.cast(),
            num_groups: self.num_groups,
            empty: self.empty,
            used: self.used,

            phantom: PhantomData,
        }
    }

    pub fn move_into<A2: Alloc>(self, alloc: A2) -> RawHashMap<K, V, S, A2> {
        // `self.num_groups` is always valid for `Self::layout`.
        let layout = unsafe { Self::layout(self.num_groups).unwrap_unchecked() };
        let data = alloc.alloc(layout).expect("oom");

        let this = core::mem::ManuallyDrop::new(self);
        unsafe {
            // copy groups/slots to new map.
            core::ptr::copy_nonoverlapping(
                this.groups.as_ptr() as *const u8,
                data.as_ptr(),
                layout.size());

            // free old groups/slots.
            this.alloc.free(this.groups.cast(), layout);

            // drop allocator.
            drop(core::ptr::read(&this.alloc));
        }

        // take seed.
        let seed = unsafe { core::ptr::read(&this.seed) };

        RawHashMap {
            seed,
            alloc,

            groups: data.cast(),
            num_groups: this.num_groups,
            empty: this.empty,
            used: this.used,

            phantom: PhantomData,
        }
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
                else { Bitmask::NONE },

            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> RawIterMut<K, V> {
        RawIterMut {
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
                else { Bitmask::NONE },

            phantom: PhantomData,
        }
    }


    pub fn clear(&mut self) {
        if self.used == 0 {
            return;
        }

        let slots = Self::slots_ptr(self.groups, self.num_groups);

        self.used = 0;

        for group_idx in 0..self.num_groups as usize {
            let group = unsafe { group_ref(self.groups, group_idx) };

            for i in group.match_used() { unsafe {
                core::ptr::drop_in_place(slot_ptr(slots, group_idx, i));
            }}

            *group = Group::empty();
        }

        self.empty = load::EMPTY_PER_GROUP * self.num_groups;
    }


    unsafe fn resize(&mut self, new_num_groups: u32) {
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

                        let hash = self.seed.hash(&key);

                        // may be used, if the user did dumb stuff
                        // with interior mutability.
                        let entry = unsafe { self.entry(&key, hash) };
                        assert!(entry.used == false);

                        unsafe {
                            entry.slot.write(Slot { key, value });
                            (*entry.group).use_entry(entry.i, hash);
                        }

                        self.empty -= 1;
                        self.used  += 1;
                    }
                }
            }

            unsafe {
                // `self.num_groups` is always valid for `Self::layout`.
                let layout = Self::layout(old_num_groups).unwrap_unchecked();
                self.alloc.free(old_groups.cast(), layout);
            }
        }
        else { debug_assert_eq!(old_used, 0) }
    }

    #[inline]
    fn grow(&mut self) {
        let new_num_groups =
            self.num_groups
            .checked_mul(2).expect("capacity overflow")
            .max(1);
        unsafe { self.resize(new_num_groups) };
        assert!(self.empty > 0);
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
                    return RawEntry { used: true, group, slot, i };
                }
            }

            if let Some(i) = group.match_free().next() {
                let slot = unsafe { &mut *slot_ptr(slots, group_idx, i) };
                return RawEntry { used: false, group, slot, i };
            }

            group_next(&mut group_idx, self.num_groups);
        }
    }


    #[inline(always)]
    fn layout(num_groups: u32) -> Option<Layout> {
        let num_groups: usize = num_groups.try_into().expect("unreachable");
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



impl<K: Eq, V, S: HashFnSeed<K, Hash=u32> + Default, A: Alloc + Default> Default for RawHashMap<K, V, S, A> {
    #[inline]
    fn default() -> Self {
        Self {
            seed: S::default(),
            alloc: A::default(),
            groups: NonNull::dangling(),
            num_groups: 0,
            empty: 0,
            used: 0,
            phantom: PhantomData,
        }
    }
}


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

            unsafe {
                // `self.num_groups` is always valid for `Self::layout`.
                let layout = Self::layout(self.num_groups).unwrap_unchecked();
                self.alloc.free(self.groups.cast(), layout);
            }
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

    phantom: PhantomData<(&'a K, &'a V)>,
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


pub struct RawIterMut<'a, K, V> {
    groups: NonNull<Group>,
    slots:  NonNull<Slot<K, V>>,
    num_groups: u32,

    group_idx: usize,
    bitmask: Bitmask,

    phantom: PhantomData<(&'a K, &'a mut V)>,
}

impl<'a, K, V> Iterator for RawIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(i) = self.bitmask.next() {
                let slot = unsafe { &mut *slot_ptr(self.slots, self.group_idx, i) };
                return Some((&slot.key, &mut slot.value));
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
    used:  bool,
    group: *mut Group,
    slot:  *mut Slot<K, V>,
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
    // note: we're quadratic.
    // because we're using the high bits of the 64 bit product,
    // the first group is primarily determined by the hash's high bits.
    // this is problematic, because it effectively sorts the entries by hash.
    // which then leads to quadratic runtime when inserting the elements
    // in `iter` order into a fresh hash map with the same hash function + seed.
    // this happens, because the new, smaller hashmap will contain the entries
    // in the same order. but because its capacity is smaller, the entries
    // need to "sit closer together". and because we're inserting in order,
    // we get a ton of hash collisions, as we over-populate the first group,
    // then the second, and so on.
    //
    // note: we currently don't address this issue.
    // potential fixes:
    //  - make the hash function capacity dependeng.
    //      - eg: `let hash = hash.rotate_right(num_groups.trailing_zeros());`
    //      - this makes resizes *much more* expensive, as memory is no longer
    //        walked linearly.
    //      - perf wise, this solution is unacceptable.
    //  - use a different reduction function.
    //      - eg modulo, which may be too slow.
    //      - or masking with 2^n table sizes.
    //          - this mitigates the above issue, as each resize introduces another
    //            high bit, distributing the elements evenly in the upper/lower half.
    //          - but 2^n table sizes come at the cost of up to 100% memory overhead.
    //            though that may not be too much of an issue, as most hashmaps aren't
    //            created with a known capacity, and `grow` still produces 2^n sizes.
    //      - neither of these fix the issue, they merely reduce the slowdown to < 10x,
    //        compared to becoming completely unusable (current slowdown is >100x).
    //  - random seed by default.
    //      - this would be the most robust solution.
    //      - but it also has the highest cost: non-determinism.
    //        no more easy `diff` on debug prints to find bugs.
    //      - on 64 bit architectures, we 4 unused bytes.
    //      - i suppose, if you could globally disable randomization or seed it,
    //        this solution could be fine.
    //        it would also help detect hash map iteration order dependent logic,
    //        which should be avoided.
    //  - randomized iteration as an opt-in method.
    //      - user still needs to be aware of the issue.
    //      - and it still breaks determinism.
    //  - retained insertion order.
    //      - this would be the nicest solution. also because it actually adds
    //        functionality.
    //      - but it's unclear, whether we want to pay the memory overhead.
    //      - could this be an opt-out?
    //      - what about the runtime overhead? reverse insertion order would be
    //        effectively free for insertion.

    let result = ((hash as u64 * num_groups as u64) >> 32) as usize;
    debug_assert!(result < num_groups as usize);

    return result;
}

#[inline(always)]
fn group_next(i: &mut usize, num_groups: u32) {
    debug_assert!(*i < num_groups as usize);

    *i += 1;
    if *i == num_groups as usize {
        *i = 0;
    }

    debug_assert!(*i < num_groups as usize);
}


use group_u64::*;
mod group_u64 {
    #[derive(Clone, Copy)]
    pub(super) struct Group(u64);

    pub(super) use crate::bit::Bitmask8 as Bitmask;

    impl Group {
        pub const WIDTH: usize = 8;

        const EMPTY:     u8  = 0xff;
        const TOMBSTONE: u8  = 0x80;
        const HASH_MASK: u32 = 0x7f;


        #[inline(always)]
        pub const fn empty() -> Group {
            Self(crate::bit::splat_8(Self::EMPTY))
        }

        #[inline(always)]
        const fn mask_hash(hash: u32) -> u8 {
            (hash & Self::HASH_MASK) as u8
        }


        #[inline(always)]
        pub fn match_hash(&self, hash: u32) -> Bitmask {
            Bitmask::find_equal_bytes(self.0, Self::mask_hash(hash))
        }

        #[inline(always)]
        pub fn match_empty(&self) -> Bitmask {
            // check high bit and second highest bit set.
            // only empty & tombstone have the high bit.
            // and tombstone only has the high bit.
            Bitmask::find_high_bit_bytes(self.0 & (self.0 << 1))
        }

        #[inline(always)]
        pub fn match_free(&self) -> Bitmask {
            // only empty & tombstone have the high bit.
            Bitmask::find_high_bit_bytes(self.0)
        }

        #[inline(always)]
        pub fn match_used(&self) -> Bitmask {
            // used entries don't have the high bit.
            Bitmask::find_high_bit_bytes(self.0).not()
        }


        #[inline(always)]
        fn set(&mut self, idx: usize, value: u8) {
            // note: the hashmap ops only ever mutate <= 1 byte
            // of a group. so we can just use a single byte store.
            unsafe {
                // the safe version of this generates shit code.
                let bytes: &mut [u8; Group::WIDTH] = core::mem::transmute(&mut self.0);
                bytes[idx] = value;
            }
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
}


#[cfg(test)]
pub(crate) const GROUP_SIZE: usize = Group::WIDTH;

