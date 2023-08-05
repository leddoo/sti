use super::*;


/// B32: a 32 bit mask.
/// - either `0` or `u32::MAX`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct B32(pub(crate) u32);

impl B32 {
    pub const FALSE: B32 = B32(0);
    pub const TRUE:  B32 = B32(u32::MAX);

    #[inline(always)]
    pub fn new(v: bool) -> B32 {
        B32((-(v as i32)) as u32)
    }
}

impl SimdElement for B32 {}

impl Into<B32> for bool {
    #[inline(always)]
    fn into(self) -> B32 {
        B32::new(self)
    }
}



pub type B32x<const N: usize> = Simd<B32, N>;
pub type B32x2 = Simd<B32, 2>;
pub type B32x4 = Simd<B32, 4>;

impl<const N: usize> B32x<N> where (): SimdLanes<N> {
    pub const NONE: B32x<N> = Simd::csplat(B32::FALSE);
    pub const ALL:  B32x<N> = Simd::csplat(B32::TRUE);

    #[inline(always)]
    pub fn new_b(v: [bool; N]) -> Self {
        let v = v.map(B32::new);
        Self { align: Self::ALIGN, v }
    }

    #[inline(always)]
    pub fn splat(v: B32) -> Self {
        let v = <() as SimdLanes<N>>::b32_splat(v);
        Self { align: Self::ALIGN, v }
    }

    #[inline(always)]
    pub fn splat_b(v: bool) -> Self {
        let v = <() as SimdLanes<N>>::b32_splat(v.into());
        Self { align: Self::ALIGN, v }
    }


    #[inline(always)]
    pub fn select_u32(self, on_true: U32x<N>, on_false: U32x<N>) -> U32x<N> {
        let v = <() as SimdLanes<N>>::b32_select_u32(self.v, on_true.v, on_false.v);
        U32x { align: U32x::ALIGN, v }
    }
}

/*

    fn b32_as_u32(v: [B32; N]) -> [u32; N];
    fn b32_as_i32(v: [B32; N]) -> [u32; N];

    fn b32_select_b32(mask: [B32; N], on_true: [B32; N], on_false: [B32; N]) -> [B32; N];
    fn b32_select_i32(mask: [B32; N], on_true: [i32; N], on_false: [i32; N]) -> [i32; N];
    fn b32_select_u32(mask: [B32; N], on_true: [u32; N], on_false: [u32; N]) -> [u32; N];
    fn b32_select_f32(mask: [B32; N], on_true: [f32; N], on_false: [f32; N]) -> [f32; N];

    fn b32_none(v: [B32; N]) -> bool;
    fn b32_any (v: [B32; N]) -> bool;
    fn b32_all (v: [B32; N]) -> bool;

    fn b32_zip  (lhs: [B32; N], rhs: [B32; N]) -> ([B32; N], [B32; N]);
    fn b32_unzip(lhs: [B32; N], rhs: [B32; N]) -> ([B32; N], [B32; N]);

    fn b32_and(lhs: [B32; N], rhs: [B32; N]) -> [B32; N];
    fn b32_or (lhs: [B32; N], rhs: [B32; N]) -> [B32; N];
    fn b32_not(v:   [B32; N]) -> [B32; N];
*/


