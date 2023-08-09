
- todo:
    - new simd module:
        - scalar add/sub (ext trait, "add", "sub", maybe "sadd").
        - scalar into, ext for "v2", "v4".
        - impl aarch64.
        - platform detection.
        - impl x86.
            - sse4 & error.
        - use `core::simd` on nightly.
        - make simd opt-in on x86 and warn if not enabled sse4.
    - `Vec::extend`.
    - thread local temp arena (dynamic stack enforcement).

- backlog:
    - simd `x8` vectors using `x4`.
        - should get promoted to `x8` if avx is available. ig.
    - `no_std` support.
        - `thread_local`.
        - global heap allocator (mimalloc).


