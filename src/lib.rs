#![cfg_attr(feature = "sti_bench", feature(test))]

pub mod num;
pub mod hint;

pub mod simd;
pub mod float;
pub mod bit;
pub mod reader;

pub mod traits;

pub mod alloc;
pub mod arena;
pub mod arena_pool;
pub mod boks;
pub mod rc;
pub mod vec;
pub mod hash;
pub mod static_vec;

pub mod sync;

pub mod utf8;
pub mod string;

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

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        { let _ = ::core::write!($dst, $($arg)*); }
    };
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        $crate::string::format(::core::format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! format_in {
    ($alloc:expr, $($arg:tt)*) => {
        $crate::string::format_in($alloc, ::core::format_args!($($arg)*))
    }
}

