0.1.6:
- arena:
    - renamed `GrowingArena` -> `Arena`.
    - arena no longer rounds up allocation sizes to `MAX_ALIGN`.
    - `MAX_ALIGN` is now 32.
    - added `stats()`, which returns usage statistics.
    - added `reset_all()`, which frees all arena blocks.
    - added `alloc_str()`.

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

