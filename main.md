
- todo:
    - new simd module:
        - impl scalar.
            - impl i32 bitops using u32.
            - impl zips using u32.
        - impl aarch64.
    - `Vec::extend`.
    - `Vec` drop tests & fix truncate.
    - thread local temp arena (dynamic stack enforcement).
    - simd:
        - more platforms:
            - `x86_64`.
            - `wasm`.
            - `scalar`.
        - `x8` vectors.
            - `aarch64`: does it always have them?
            - `x86_64`: avx if available.
            - `x4` fallback.

- backlog:
    - `no_std` support.
        - `thread_local`.
        - global heap allocator (mimalloc).


