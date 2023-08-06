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
    fn b32_as_i32(v: [B32; N]) -> [i32; N];

    fn b32_select_b32(mask: [B32; N], on_true: [B32; N], on_false: [B32; N]) -> [B32; N];
    fn b32_select_i32(mask: [B32; N], on_true: [i32; N], on_false: [i32; N]) -> [i32; N];
    fn b32_select_u32(mask: [B32; N], on_true: [u32; N], on_false: [u32; N]) -> [u32; N];
    fn b32_select_f32(mask: [B32; N], on_true: [f32; N], on_false: [f32; N]) -> [f32; N];

    fn b32_none(v: [B32; N]) -> bool;
    fn b32_any(v: [B32; N]) -> bool;
    fn b32_all(v: [B32; N]) -> bool;

    fn b32_zip(lhs: [B32; N], rhs: [B32; N]) -> ([B32; N], [B32; N]);
    fn b32_unzip(lhs: [B32; N], rhs: [B32; N]) -> ([B32; N], [B32; N]);

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

    fn i32_zip(lhs: [i32; N], rhs: [i32; N]) -> ([i32; N], [i32; N]);
    fn i32_unzip(lhs: [i32; N], rhs: [i32; N]) -> ([i32; N], [i32; N]);

    fn i32_add(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_sub(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_neg(v: [i32; N]) -> [i32; N];

    fn i32_shl(v: [i32; N], shift: i32) -> [i32; N];
    fn i32_shr(v: [i32; N], shift: i32) -> [i32; N];

    fn i32_and(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_or(lhs: [i32; N], rhs: [i32; N]) -> [i32; N];
    fn i32_not(v: [i32; N]) -> [i32; N];
}

#[repr(align(8))]
#[derive(Clone, Copy)]
pub struct Align8;

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct Align16;


mod scalar;



