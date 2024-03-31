0.2.1:
- now bumping minor for features & changes, patch for fixes.
- `trait Alloc` is now unsafe (`Clone` requirement).
- breaking change: `Vec::from_{value, fn}` now take the `len` before the value/fn.

0.1.11:
- add `UnwrapDebug`.
- add `ManualVec`.
- remove `AllocError`.
- rename `reserve_exact` -> `reserve_exactly` (to disambiguate from `reserve_extra`).
- `PackedOption::take` now returns `PackedOption` instead of `Option`.
- add `KGenVec`.

0.1.10:
- `Arena: (Ref)UnwindSafe`.
- add `RwSpinLock`.
- add `KVec::push_with`.
- `KFreeVec`:
    - now returns values on free.
    - add `retain`.
    - add `next_key`, `alloc_with`.
    - add `get`, `get_mut`.
- `impl PartialEq, Eq for Vec, KVec, KSlice`.
- add `VecDeque`.
- `HashMap`:
    - add `retain`.
    - rename `remove` -> `remove_with_key`.
    - rename `remove_value` -> `remove`.
    - add `reserve`.
    - `impl Default`.
    - add `iter_mut`.
- `Vec`:
    - add `remove_swap`.
    - add `insert/_from_slice`.
    - rename `grow_by` -> `reserve_extra`.
    - remove oom fallible methods.
    - add `vec_extend` macro.


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


