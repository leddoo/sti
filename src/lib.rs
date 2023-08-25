pub mod num;

pub mod simd;
pub mod float;

pub mod alloc;
pub mod arena;
pub mod arena_pool;
pub mod vec;
pub mod rc;
pub mod hash;

pub mod sync;

pub mod reader;

pub mod packed_option;
pub mod keyed;


pub mod prelude;


#[macro_export]
macro_rules! static_assert {
    ($cond: expr) => {
        const _: () = assert!($cond);
    };
}

#[macro_export]
macro_rules! static_assert_eq {
    ($a: expr, $b: expr) => {
        //const _: () = assert_eq!($a, $b);
        const _: () = assert!($a == $b);
    };
}

#[macro_export]
macro_rules! static_assert_ne {
    ($a: expr, $b: expr) => {
        //const _: () = assert_ne!($a, $b);
        const _: () = assert!($a != $b);
    };
}

