/// Simd<T, N>
/// - single instruction, multiple data.
/// - implemented as an aligned array.
#[derive(Clone, Copy)]
pub struct Simd<T: SimdElement, const N: usize> where (): SimdLanes<N> {
    #[allow(dead_code)]
    pub(crate) align: <() as SimdLanes<N>>::Align,

    pub v: [T; N],
}

mod generic;



/// SimdElement
/// - the trait for supported simd element types.
pub trait SimdElement: Copy {}

mod b32x; pub use b32x::*;
mod i32x; pub use i32x::*;
mod u32x; pub use u32x::*;
mod f32x; pub use f32x::*;



//
// SimdLanes
//  - the trait implemented by each platform.
//

use b32x::B32;

pub trait SimdLanes<const N: usize> {
    type Align: Copy;
    const ALIGN: Self::Align;


    fn b32_splat(v: B32) -> [B32; N];

    fn b32_as_u32(v: [B32; N]) -> [u32; N];
    fn b32_from_u32_unck(v: [u32; N]) -> [B32; N];
    fn b32_as_i32(v: [B32; N]) -> [i32; N];

    fn b32_select_u32(mask: [B32; N], on_true: [u32; N], on_false: [u32; N]) -> [u32; N];

    fn b32_none(v: [B32; N]) -> bool;
    fn b32_any(v: [B32; N]) -> bool;
    fn b32_all(v: [B32; N]) -> bool;

    fn b32_and(lhs: [B32; N], rhs: [B32; N]) -> [B32; N];
    fn b32_or(lhs: [B32; N], rhs: [B32; N]) -> [B32; N];
    fn b32_not(v: [B32; N]) -> [B32; N];


    fn i32_splat(v: i32) -> [i32; N];

    fn i32_as_u32(v: [i32; N]) -> [u32; N];
    fn i32_to_f32(v: [i32; N]) -> [f32; N];

    fn i32_min(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_max(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];

    fn i32_eq(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];
    fn i32_ne(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];
    fn i32_le(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];
    fn i32_lt(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];
    fn i32_ge(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];
    fn i32_gt(lhs: [i32; N], rhs: [i32; N]) -> [B32; N];

    fn i32_add(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_sub(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_neg(v: [i32; N]) -> [i32; N];

    fn i32_shl(v: [i32; N], shift: i32) -> [i32; N];
    fn i32_shr(v: [i32; N], shift: i32) -> [i32; N];


    fn u32_splat(v: u32) -> [u32; N];

    fn u32_as_i32(v: [u32; N]) -> [i32; N];

    fn u32_min(lhs: [u32; N], rhs: [u32; N]) -> [u32; N];
    fn u32_max(lhs: [u32; N], rhs: [u32; N]) -> [u32; N];

    fn u32_le(lhs: [u32; N], rhs: [u32; N]) -> [B32; N];
    fn u32_lt(lhs: [u32; N], rhs: [u32; N]) -> [B32; N];
    fn u32_ge(lhs: [u32; N], rhs: [u32; N]) -> [B32; N];
    fn u32_gt(lhs: [u32; N], rhs: [u32; N]) -> [B32; N];

    fn u32_zip(lhs: [u32; N], rhs: [u32; N]) -> ([u32; N], [u32; N]);
    fn u32_unzip(lhs: [u32; N], rhs: [u32; N]) -> ([u32; N], [u32; N]);

    fn u32_shr(v: [u32; N], shift: u32) -> [u32; N];

    fn u32_and(lhs: [u32; N], rhs: [u32; N]) -> [u32; N];
    fn u32_or(lhs: [u32; N], rhs: [u32; N]) -> [u32; N];
    fn u32_not(v: [u32; N]) -> [u32; N];


    fn f32_splat(v: f32) -> [f32; N];

    fn f32_as_bits(v: [f32; N]) -> [u32; N];
    fn f32_from_bits(v: [u32; N]) -> [f32; N];

    fn f32_to_i32_unck(v: [f32; N]) -> [i32; N];
    fn f32_to_i32(v: [f32; N]) -> [i32; N];

    fn f32_floor(v: [f32; N]) -> [f32; N];
    fn f32_ceil(v: [f32; N]) -> [f32; N];
    fn f32_round(v: [f32; N]) -> [f32; N];
    fn f32_trunc(v: [f32; N]) -> [f32; N];
    fn f32_abs(v: [f32; N]) -> [f32; N];
    fn f32_sqrt(v: [f32; N]) -> [f32; N];
    fn f32_with_sign_of(v: [f32; N], sign: [f32; N]) -> [f32; N];

    fn f32_hadd(v: [f32; N]) -> f32;

    fn f32_min(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];
    fn f32_max(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];

    fn f32_eq(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];
    fn f32_ne(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];
    fn f32_le(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];
    fn f32_lt(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];
    fn f32_ge(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];
    fn f32_gt(lhs: [f32; N], rhs: [f32; N]) -> [B32; N];

    fn f32_neg(v: [f32; N]) -> [f32; N];
    fn f32_add(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];
    fn f32_sub(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];
    fn f32_mul(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];
    fn f32_div(lhs: [f32; N], rhs: [f32; N]) -> [f32; N];
}


pub trait SimdBits<const N: usize> where (): SimdLanes<N> {
    fn sb_to(self) -> U32x<N>;
    unsafe fn sb_from(v: U32x<N>) -> Self;
}

impl<const N: usize> SimdBits<N> for B32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sb_to(self) -> U32x<N> { self.as_u32() }

    #[inline(always)]
    unsafe fn sb_from(v: U32x<N>) -> Self {
        let v = <() as SimdLanes<N>>::b32_from_u32_unck(v.v);
        Self { align: Self::ALIGN, v }
    }
}

impl<const N: usize> SimdBits<N> for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sb_to(self) -> U32x<N> { self.as_u32() }

    #[inline(always)]
    unsafe fn sb_from(v: U32x<N>) -> Self { v.as_i32() }
}

impl<const N: usize> SimdBits<N> for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sb_to(self) -> U32x<N> { self.as_bits() }

    #[inline(always)]
    unsafe fn sb_from(v: U32x<N>) -> Self { Self::from_bits(v) }
}


#[repr(align(8))]
#[derive(Clone, Copy)]
pub struct Align8;

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct Align16;


mod scalar;


mod tests;


