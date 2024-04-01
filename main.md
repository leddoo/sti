
- todo:
    - leb128.
    - update readme & license.
    - cell & unck docs.
        - why we have them, what they're for.
        - how to use them correctly.
    - remove manual vec?
    - mimalloc.



### backlog:

- utf-8 module:
    - ceil/floor.
    - iteration.
    - consider removing `_inline` versions.
      perhaps maybe have short string opt inline versions.

- arena:
    - `prev_cap` for consistent geometric growth.
        - don't write when allocating block `> max_block_size`.
        - test that.
        - maybe just delegate to global instead, if size exceeds some limit.
          (we're given the size on free)
    - remove `A` parameter.
        - consider `backing: Option<Rc<dyn Alloc>>,`

- simd:
    - scalar add/sub (ext trait, "add", "sub", maybe "sadd").
    - scalar into, ext for "v2", "v4".
    - impl aarch64.
    - platform detection.
    - impl x86.
        - sse4 & error.
    - use `core::simd` on nightly.
    - make simd opt-in on x86 and warn if not enabled sse4.
    - simd `x8` vectors using `x4`.
        - should get use avx, if available.

- hash map.
    - test non-allocation for non-inserting funcs.
        - full test for `get_or_insert`, fn to test both versions (k/not).
    - test key mutation.
    - rename `empty` to something less misleading.
    - slot api for ptr eq keys.

- stuff:
    - `no_std` support.
        - `thread_local`.
        - global heap allocator (mimalloc).


