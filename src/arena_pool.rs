use core::cell::{Cell, UnsafeCell};
use core::ptr::NonNull;
use core::mem::{ManuallyDrop, MaybeUninit};

use crate::hint::cold;
use crate::alloc::GlobalAlloc;
use crate::arena::{Arena, ArenaSavePoint};
use crate::boks::Box;


pub const MAX_ARENAS_LIMIT: usize = 16;

// @todo: make configurable.
pub const DEFAULT_MAX_ARENAS: usize = 4;
pub const DEFAULT_ARENA_SIZE: usize = 512*1024;


thread_local! {
    pub static ARENA_POOL: ArenaPool = unsafe { ArenaPool::new() };
}


pub struct ArenaPool {
    slots: [MaybeUninit<UnsafeCell<ArenaSlot>>; MAX_ARENAS_LIMIT],
    len: Cell<usize>,
    max_arenas: usize, // <= MAX_ARENAS_LIMIT
    arena_size: usize,
}

struct ArenaSlot {
    arena: Box<UnsafeCell<Arena>>,
    refs: Cell<usize>,
    mode: Cell<SlotMode>, // only valid if `refs > 0`.
}

#[derive(Clone, Copy, PartialEq)]
enum SlotMode {
    Temp,
    Rec,
    Scoped,
}

impl ArenaPool {
    pub const unsafe fn new() -> Self {
        Self {
            slots: unsafe {
                // cf MaybeUninit::uninit_array()
                MaybeUninit::uninit().assume_init()
            },
            len: Cell::new(0),
            max_arenas: DEFAULT_MAX_ARENAS,
            arena_size: DEFAULT_ARENA_SIZE,
        }
    }


    #[inline(always)]
    pub fn get_temp(&self) -> PoolArena {
        let (arena, slot) = self.get(SlotMode::Temp, &[]);
        PoolArena { arena, slot }
    }

    #[inline(always)]
    pub fn get_rec(&self) -> PoolArena {
        let (arena, slot) = self.get(SlotMode::Rec, &[]);
        PoolArena { arena, slot }
    }

    /// see `tls_get_scoped`.
    #[inline(always)]
    pub unsafe fn get_scoped(&self, conflicts: &[*const Arena]) -> ScopedPoolArena {
        let (arena, slot) = self.get(SlotMode::Scoped, conflicts);
        let save = unsafe { (*(*arena.as_ptr()).get()).save() };
        ScopedPoolArena { arena, slot, save }
    }

    fn get(&self, requested_mode: SlotMode, conflicts: &[*const Arena])
    -> (NonNull<UnsafeCell<Arena>>, Option<NonNull<ArenaSlot>>) {
        let mut best = 0;
        let mut min_refs = usize::MAX;
        for i in 0..self.len.get() {
            let slot = unsafe { &*self.slot_ptr(i) };

            let arena = slot.arena.get() as *const Arena;
            let mode = slot.mode.get();
            let refs = slot.refs.get();

            // free arena.
            if refs == 0 {
                // only temp takes free arenas greedily.
                if requested_mode == SlotMode::Temp {
                    slot.refs.set(1);
                    slot.mode.set(requested_mode);
                    return (slot.arena.inner(), Some(slot.into()));
                }
                // found nothing so far, remember this free slot.
                else if min_refs == usize::MAX {
                    best = i;
                    min_refs = usize::MAX - 1;
                }
            }
            else {
                // best in-use arena with requested mode.
                if mode == requested_mode
                && refs < min_refs
                && !conflicts.contains(&arena) {
                    best = i;
                    min_refs = refs;
                }
            }
        }

        // found a usable arena.
        if min_refs < usize::MAX {
            let slot = unsafe { &*self.slot_ptr(best) };
            slot.refs.set(slot.refs.get() + 1);
            slot.mode.set(requested_mode);
            return (slot.arena.inner(), Some(slot.into()));
        }
        // else, allocate a new arena.
        else {
            let arena = Arena::new();
            arena.min_block_size.set(self.arena_size);
            let arena = Box::new(UnsafeCell::new(arena));

            let arena_ptr = arena.inner();
            let slot = ArenaSlot {
                arena,
                refs: Cell::new(1),
                mode: Cell::new(requested_mode),
            };

            // add to slots, if we have space left.
            if self.len.get() < self.max_arenas {
                // @temp.
                println!("allocated arena number {}", self.len.get() + 1);
                let slot = unsafe {
                    let ptr = self.slot_ptr(self.len.get());
                    ptr.write(slot);
                    &*ptr
                };
                self.len.set(self.len.get() + 1);
                (arena_ptr, Some(slot.into()))
            }
            // give as owned to user.
            else {
                // @temp.
                println!("allocated arena, but oh no, we'll need to drop it :L");
                let _ = ManuallyDrop::new(slot.arena);
                (arena_ptr, None)
            }
        }
    }

    #[inline(always)]
    unsafe fn slot_ptr(&self, i: usize) -> *mut ArenaSlot {
        debug_assert!(i < self.slots.len());
        unsafe { (*self.slots.get_unchecked(i).as_ptr()).get() }
    }


    #[inline(always)]
    pub fn tls_get_temp() -> PoolArena {
        ARENA_POOL.with(|this| { this.get_temp() })
    }

    #[inline(always)]
    pub fn tls_get_rec() -> PoolArena {
        ARENA_POOL.with(|this| { this.get_rec() })
    }

    /// get a scoped arena.
    ///
    /// scoped arenas save the arena state on creation
    /// and restore this saved state on drop.
    ///
    /// this is unsafe, because multiple `ScopedPoolArena`s
    /// can use the same underlying `Arena`.
    /// the issue is, if an outer scope makes an allocation between
    /// the save/restore of an inner scope, that allocation
    /// is freed, when the inner scope ends, not when the
    /// outer scope ends, as the user of the outer scoped arena
    /// would expect.
    /// the borrow checker can't detect this.
    ///
    /// scoped arenas are useful for non-leaf and especially
    /// recursive functions. their safe alternative, `tls_get_rec`,
    /// also uses shared arenas internally, but doesn't free,
    /// until all `PoolArena`s using the same arena are dropped.
    /// in recursive functions, this can delay freeing for a long
    /// time, which can be problematic, if many allocations are made.
    /// the scoped arena solves this by resetting immediately upon drop.
    ///
    /// #safety:
    /// - all other scoped arenas in this pool,
    ///   that may be allocated from,
    ///   during the lifetime of the returned scoped arena,
    ///   must be specified in `conflicts`.
    /// - note: be careful, allocations aren't always explicit.
    ///   examples: `Vec::push` or calling a closure.
    ///
    #[inline(always)]
    pub unsafe fn tls_get_scoped(conflicts: &[*const Arena]) -> ScopedPoolArena {
        ARENA_POOL.with(|this| unsafe { this.get_scoped(conflicts) })
    }
}

impl Drop for ArenaPool {
    fn drop(&mut self) {
        debug_assert!(self.len.get() < self.slots.len());

        for i in 0..self.len.get() {
            unsafe {
                let ptr = self.slot_ptr(i);
                if (*ptr).refs.get() > 0 {
                    panic!("ArenaPool dropped while in use");
                }
                core::ptr::drop_in_place(ptr);
            }
        }
    }
}


pub struct PoolArena {
    arena: NonNull<UnsafeCell<Arena>>,

    // none if this is an owned arena.
    slot: Option<NonNull<ArenaSlot>>,
}

impl Drop for PoolArena {
    #[inline]
    fn drop(&mut self) {
        // borrowed arena.
        if let Some(slot) = self.slot {
            let slot = unsafe { &*slot.as_ptr() };

            // dec ref (implicit free).
            let refs = slot.refs.get();
            debug_assert!(refs > 0);
            slot.refs.set(refs - 1);

            // reset.
            if refs == 1 { unsafe {
                let arena = &mut *(*self.arena.as_ptr()).get();
                arena.reset();
            }}
        }
        // owned arena -> drop.
        else {
            cold();
            unsafe { drop(Box::from_raw_parts(self.arena, GlobalAlloc)) }
        }
    }
}

impl core::ops::Deref for PoolArena {
    type Target = Arena;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*self.arena.as_ptr()).get() }
    }
}


pub struct ScopedPoolArena {
    arena: NonNull<UnsafeCell<Arena>>,

    // none if this is an owned arena.
    slot: Option<NonNull<ArenaSlot>>,

    save: ArenaSavePoint,
}

impl Drop for ScopedPoolArena {
    #[inline]
    fn drop(&mut self) {
        // borrowed arena.
        if let Some(slot) = self.slot {
            let slot = unsafe { &*slot.as_ptr() };

            // dec ref (implicit free).
            let refs = slot.refs.get();
            debug_assert!(refs > 0);
            slot.refs.set(refs - 1);

            unsafe {
                let arena = &*(*self.arena.as_ptr()).get();
                arena.restore(self.save.clone());
            }
        }
        // owned arena -> drop.
        else {
            cold();
            unsafe { drop(Box::from_raw_parts(self.arena, GlobalAlloc)) }
        }
    }
}

impl core::ops::Deref for ScopedPoolArena {
    type Target = Arena;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(*self.arena.as_ptr()).get() }
    }
}


#[cfg(all(feature = "sti_bench", test))]
mod benches {
    extern crate test;
    use test::Bencher;

    use crate::alloc::Alloc;
    use crate::vec::Vec;

    use super::*;

    mod single {
        use super::*;

        fn do_it<A: Alloc>(alloc: A) -> i32 {
            let mut v = Vec::with_cap_in(4, alloc);
            v.push(1);
            v.push(unsafe { core::ptr::read_volatile(&v[0]) });
            let result = v.iter().sum();
            assert_eq!(result, 2);
            return result;
        }

        #[bench]
        fn global_alloc(b: &mut Bencher) {
            b.iter(|| { do_it(GlobalAlloc) });
        }

        #[bench]
        fn tls_get_temp(b: &mut Bencher) {
            b.iter(|| { do_it(&*ArenaPool::tls_get_temp()) });
        }

        #[bench]
        fn own_arena(b: &mut Bencher) {
            let mut arena = Arena::new();
            arena.min_block_size.set(DEFAULT_ARENA_SIZE);
            b.iter(|| { arena.reset(); do_it(&arena) });
        }
    }

    mod multi {
        use super::*;

        fn do_it<A: Alloc>(alloc: A) -> i32 {
            let mut v = Vec::new_in(alloc);
            for _ in 0..10 {
                v.reserve_exact(v.len() + 2);
                v.push(1);
                v.push(unsafe { core::ptr::read_volatile(v.rev(0)) });
            }
            let result = v.iter().sum();
            assert_eq!(result, 20);
            return result;
        }

        #[bench]
        fn global_alloc(b: &mut Bencher) {
            b.iter(|| { do_it(GlobalAlloc) });
        }

        #[bench]
        fn tls_get_temp(b: &mut Bencher) {
            b.iter(|| { do_it(&*ArenaPool::tls_get_temp()) });
        }

        #[bench]
        fn own_arena(b: &mut Bencher) {
            let mut arena = Arena::new();
            arena.min_block_size.set(DEFAULT_ARENA_SIZE);
            b.iter(|| { arena.reset(); do_it(&arena) });
        }
    }

    mod compute {
        use super::*;

        fn do_it<A: Alloc>(reserve: bool, alloc: A) -> i32 {
            let n = 50;
            let mut v = Vec::with_cap_in(if reserve { n } else { 0 }, alloc);
            v.push(1);
            for _ in 0..n-1 {
                let a: i32 = v.iter().sum();
                let b: i32 = v.iter().rev().sum();
                let c: i32 = v.iter().chain(v.iter().rev()).sum();
                v.push(a.wrapping_mul(b).wrapping_add(c));
            }
            if reserve { assert_eq!(v.cap(), n) }
            return v.iter().sum();
        }

        #[bench]
        fn grow_global_alloc(b: &mut Bencher) {
            b.iter(|| { do_it(false, GlobalAlloc) });
        }

        #[bench]
        fn grow_tls_get_temp(b: &mut Bencher) {
            b.iter(|| { do_it(false, &*ArenaPool::tls_get_temp()) });
        }

        #[bench]
        fn grow_own_arena(b: &mut Bencher) {
            let mut arena = Arena::new();
            arena.min_block_size.set(DEFAULT_ARENA_SIZE);
            b.iter(|| { arena.reset(); do_it(false, &arena) });
        }

        #[bench]
        fn rsrv_global_alloc(b: &mut Bencher) {
            b.iter(|| { do_it(true, GlobalAlloc) });
        }

        #[bench]
        fn rsrv_tls_get_temp(b: &mut Bencher) {
            b.iter(|| { do_it(true, &*ArenaPool::tls_get_temp()) });
        }

        #[bench]
        fn rsrv_own_arena(b: &mut Bencher) {
            let mut arena = Arena::new();
            arena.min_block_size.set(DEFAULT_ARENA_SIZE);
            b.iter(|| { arena.reset(); do_it(true, &arena) });
        }
    }
}

