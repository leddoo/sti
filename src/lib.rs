pub mod num;
pub mod simd;

pub mod alloc;
pub mod growing_arena;
pub mod vec;
pub mod rc;

pub mod reader;

pub mod packed_option;
pub mod keyed;


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

