use core::mem::ManuallyDrop;

use crate::vec::Vec;
use crate::arena::Arena;
use crate::sync::spin_lock::SpinLock;


pub const DEFAULT_MAX_ARENAS: usize = 16;

pub const DEFAULT_ARENA_SIZE: usize = 1*1024*1024;


pub static ARENA_POOL: ArenaPool = ArenaPool::new();


pub struct ArenaPool {
    inner: SpinLock<Inner>,
}

struct Inner {
    max_arenas: usize,
    arena_size: usize,

    // - `arenas.len() <= max_arenas`
    // - `∀ a in arenas, a.total_allocated == arena_size && a.num_blocks == 1`
    arenas: Vec<Arena>,
}

impl ArenaPool {
    pub const fn new() -> Self {
        ArenaPool { inner: SpinLock::new(Inner {
            max_arenas: DEFAULT_MAX_ARENAS,
            arena_size: DEFAULT_ARENA_SIZE,

            arenas: Vec::new(),
        })}
    }

    pub fn set_max_arenas(&self, max_arenas: usize) {
        let mut this = self.inner.lock();
        this.max_arenas = max_arenas;

        if this.arenas.len() > max_arenas {
            this.arenas.truncate(max_arenas);
        }
    }

    pub fn set_arena_size(&self, arena_size: usize) {
        let mut this = self.inner.lock();

        if arena_size != this.arena_size {
            this.arena_size = arena_size;
            this.arenas.clear();
        }
    }

    pub fn get(&self) -> PoolArenaA {
        let mut this = self.inner.lock();

        if let Some(arena) = this.arenas.pop() {
            return PoolArenaA::new(self, arena);
        }

        let arena_size = this.arena_size;
        drop(this);

        let arena = Arena::new();
        arena.min_block_size.set(arena_size);
        PoolArenaA::new(self, arena)
    }

    pub fn put(&self, mut arena: Arena) {
        arena.reset();

        let arena_size = arena.current_block_size();
        debug_assert_eq!(arena_size, arena.stats().total_allocated);

        arena.min_block_size.set(arena_size);
        arena.max_block_size.set(usize::MAX);

        let mut this = self.inner.lock();

        let max_arenas = this.max_arenas;

        if this.arenas.len() < max_arenas && arena_size == this.arena_size {
            this.arenas.reserve(max_arenas);
            this.arenas.push(arena);
        }
    }
}


pub type PoolArena = PoolArenaA<'static>;

pub struct PoolArenaA<'a> {
    pool:  &'a ArenaPool,
    arena: ManuallyDrop<Arena>,
}

impl<'a> PoolArenaA<'a> {
    #[inline(always)]
    pub const fn new(pool: &'a ArenaPool, arena: Arena) -> Self {
        Self { pool, arena: ManuallyDrop::new(arena) }
    }
}

impl<'a> Drop for PoolArenaA<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        let arena = unsafe { ManuallyDrop::take(&mut self.arena) };
        self.pool.put(arena);
    }
}

impl<'a> core::ops::Deref for PoolArenaA<'a> {
    type Target = Arena;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.arena }
}

impl<'a> core::ops::DerefMut for PoolArenaA<'a> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.arena }
}

