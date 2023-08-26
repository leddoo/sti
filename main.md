
- todo:
    - hash map.
        - growing.
        - test collisions.
        - entry api.
    - rw lock.
    - vec:
        - `Vec::extend`.
        - truncate track caller.
    - `String`.



### backlog:

- string formatting:
    - infallible write trait.
    - format/in/arena macro.

- utf-8 module:
    - ceil/floor.
    - iteration.

- arena:
    - `prev_cap` for consistent geometric growth.
        - don't write when allocating block `> max_block_size`.
        - test that.

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

- stuff:
    - `no_std` support.
        - `thread_local`.
        - global heap allocator (mimalloc).


