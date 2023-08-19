
- todo:
    - arena:
        - `prev_cap`.
        - realloc test.
        - usage stats.
        - alloc str.
    - vec:
        - leak method.
        - `Vec::extend`.
        - truncate track caller.
    - thread local temp arena (dynamic stack enforcement).
    - `String`.
    - hash module.



### backlog:

- hash module:
    - swiss table.
    - fnv-1 hasher.
    - `hash<Hasher=DefaultHasher>` function.

- string formatting:
    - infallible write trait.
    - format/in/arena macro.

- utf-8 module:
    - ceil/floor.
    - iteration.

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


