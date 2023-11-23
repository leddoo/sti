0.1.10:
- `Arena: (Ref)UnwindSafe`.
- add `RwSpinLock`.
- `KFreeVec`:
    - now returns values on free.
    - add `retain`.
    - add `next_key`, `alloc_with`.
- `impl PartialEq, Eq for Vec, KVec, KSlice`.
- add `VecDeque`.
- `HashMap`:
    - add `retain`.
    - rename `remove` -> `remove_with_key`.
    - rename `remove_value` -> `remove`.
- `Vec`:
    - add `remove_swap`.


0.1.9:
- `HashMap`: add `clear` and `Debug`.
- improved `ArenaPool`.
- add `StaticVec`.
- add `Box`.
- add `FromIn`, `CopyIt`, `MapIt` traits.
- `Vec`:
    - add `vec/_in` macros.
    - add `From/In<I: Iterator>`.
    - add `Extend`.
- major breaking change: alloc params go first now.


0.1.8:
- (breaking change): `cat_next` now takes a length, not a size in bytes.
- new `hash` module:
    - `fxhash` implementation.
    - `HashFn` & `HashFnSeed` traits.
    - `HashMap`.
- new `hint` module:
    - `cold`, `likely`, `unlikely`.
- `GlobalAlloc` is `Send + Sync` now.
- `Reader` is now `Debug`.


0.1.7:
- arena:
    - add save/restore api.
- add `SpinLock`.
- add `ArenaPool`.
- add `prelude`.


0.1.6:
- arena:
    - rename `GrowingArena` -> `Arena`.
    - arena no longer rounds up allocation sizes to `MAX_ALIGN`.
    - `MAX_ALIGN` is now 32.
    - add `stats()`, which returns usage statistics.
    - add `reset_all()`, which frees all arena blocks.
    - add `alloc_str()`.
- vec:
    - make `leak` a method.
    - add `move_into()`.


0.1.5:
- slice reader:
    - `as_slice`, cause deref doesn't return `&'a [T]`.
    - `consume_while_slice[_from]` now return `(&'a [T], bool)`.
    - `consumed_slice`, opposite of `as_slice`.
    - fix `offset` for types of size greater than 1.


0.1.4:
- slice reader.
    - rewrite to be more useful.
    - support non-`Copy` types.


0.1.3:
- simd module.
    - rewrite to support abstracting over lanes (`Simd<T, N>`).
    - scalar impl for now.
    - stop failing to build on `x86`.


