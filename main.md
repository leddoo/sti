


### backlog:

- sync:
    - improve rwspinlock test.

- cell & unck docs.
    - why we have them, what they're for.
    - how to use them correctly.

- arena:
    - large allocations -> globalalloc.
    - stats isize, sub, include total used.
    - test temp arena overflow case.
    - consider returning growing arena for temp overflow case.

- leb128:
    - single byte fast path inline fns.
    - writing.
    - tests.

- kslice index proper panic & track caller.

- str::Repeat.

- mimalloc.

- utf-8 module:
    - ceil/floor.
    - iteration.
    - consider removing `_inline` versions.
      perhaps maybe have short string opt inline versions.

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


