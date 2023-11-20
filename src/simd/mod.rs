use core::marker::PhantomData;


/// Simd<T, N>
/// - single instruction, multiple data.
/// - implemented as an aligned array.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Simd<T: SimdElement, const N: usize> where (): SimdLanes<N> {
    v: <() as SimdLanes<N>>::Repr,
    p: PhantomData<fn(T) -> T>
}

mod generic;



/// SimdElement
/// - the trait for supported simd element types.
pub unsafe trait SimdElement: Copy {
    fn se_to_u32x2(v: [Self; 2]) -> [u32; 2];
    fn se_to_u32x4(v: [Self; 4]) -> [u32; 4];
}

mod b32x; pub use b32x::*;
mod i32x; pub use i32x::*;
mod u32x; pub use u32x::*;
mod f32x; pub use f32x::*;



//
// SimdLanes
//  - the trait implemented by each platform.
//

pub trait SimdLanes<const N: usize> {
    type Repr: Copy;

    fn repr_from_se<T: SimdElement>(v: [T; N]) -> Self::Repr;

    fn repr_zip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr);
    fn repr_unzip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr);


    fn b32_splat(v: B32) -> Self::Repr;

    fn b32_select(mask: Self::Repr, on_true: Self::Repr, on_false: Self::Repr) -> Self::Repr;

    fn b32_none(v: Self::Repr) -> bool;
    fn b32_any(v: Self::Repr) -> bool;
    fn b32_all(v: Self::Repr) -> bool;

    fn b32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn b32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn b32_not(v: Self::Repr) -> Self::Repr;


    fn i32_splat(v: i32) -> Self::Repr;

    fn i32_to_f32(v: Self::Repr) -> Self::Repr;

    fn i32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn i32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn i32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn i32_neg(v: Self::Repr) -> Self::Repr;

    fn i32_shl(v: Self::Repr, shift: i32) -> Self::Repr;
    fn i32_shr(v: Self::Repr, shift: i32) -> Self::Repr;


    fn u32_splat(v: u32) -> Self::Repr;

    fn u32_as_i32(v: Self::Repr) -> Self::Repr;

    fn u32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn u32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn u32_shr(v: Self::Repr, shift: u32) -> Self::Repr;

    fn u32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn u32_not(v: Self::Repr) -> Self::Repr;


    fn f32_splat(v: f32) -> Self::Repr;

    fn f32_to_i32_unck(v: Self::Repr) -> Self::Repr;
    fn f32_to_i32(v: Self::Repr) -> Self::Repr;

    fn f32_floor(v: Self::Repr) -> Self::Repr;
    fn f32_ceil(v: Self::Repr) -> Self::Repr;
    fn f32_round(v: Self::Repr) -> Self::Repr;
    fn f32_trunc(v: Self::Repr) -> Self::Repr;
    fn f32_abs(v: Self::Repr) -> Self::Repr;
    fn f32_sqrt(v: Self::Repr) -> Self::Repr;
    fn f32_with_sign_of(v: Self::Repr, sign: Self::Repr) -> Self::Repr;

    fn f32_hadd(v: Self::Repr) -> f32;

    fn f32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn f32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;

    fn f32_neg(v: Self::Repr) -> Self::Repr;
    fn f32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_mul(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
    fn f32_div(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
}

mod scalar;


mod tests;


