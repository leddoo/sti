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
    pub fn as_u32(self) -> U32x<N> {
        let v = <() as SimdLanes<N>>::b32_as_u32(self.v);
        U32x { align: U32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn as_i32(self) -> I32x<N> {
        let v = <() as SimdLanes<N>>::b32_as_i32(self.v);
        I32x { align: I32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn select_b32(self, on_true: B32x<N>, on_false: B32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::b32_select_b32(self.v, on_true.v, on_false.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn select_u32(self, on_true: U32x<N>, on_false: U32x<N>) -> U32x<N> {
        let v = <() as SimdLanes<N>>::b32_select_u32(self.v, on_true.v, on_false.v);
        U32x { align: U32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn select_i32(self, on_true: I32x<N>, on_false: I32x<N>) -> I32x<N> {
        let v = <() as SimdLanes<N>>::b32_select_i32(self.v, on_true.v, on_false.v);
        I32x { align: I32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn select_f32(self, on_true: F32x<N>, on_false: F32x<N>) -> F32x<N> {
        let v = <() as SimdLanes<N>>::b32_select_f32(self.v, on_true.v, on_false.v);
        F32x { align: F32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn none(self) -> bool {
        <() as SimdLanes<N>>::b32_none(self.v)
    }

    #[inline(always)]
    pub fn any(self) -> bool {
        <() as SimdLanes<N>>::b32_any(self.v)
    }

    #[inline(always)]
    pub fn all(self) -> bool {
        <() as SimdLanes<N>>::b32_all(self.v)
    }


    #[inline(always)]
    pub fn zip(self, rhs: B32x<N>) -> (B32x<N>, B32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::b32_zip(self.v, rhs.v);
        (B32x { align: B32x::ALIGN, v: v1 },
         B32x { align: B32x::ALIGN, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: B32x<N>) -> (B32x<N>, B32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::b32_unzip(self.v, rhs.v);
        (B32x { align: B32x::ALIGN, v: v1 },
         B32x { align: B32x::ALIGN, v: v2 })
    }
}

impl<const N: usize> core::fmt::Debug for B32x<N> where (): SimdLanes<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        (-self.as_i32()).fmt(f)
    }
}

impl<const N: usize> core::ops::BitAnd for B32x<N> where (): SimdLanes<N> {
    type Output = B32x<N>;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::b32_and(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::BitAndAssign for B32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const N: usize> core::ops::BitOr for B32x<N> where (): SimdLanes<N> {
    type Output = B32x<N>;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::b32_or(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::BitOrAssign for B32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const N: usize> core::ops::Not for B32x<N> where (): SimdLanes<N> {
    type Output = B32x<N>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let v = <() as SimdLanes<N>>::b32_not(self.v);
        B32x { align: B32x::ALIGN, v }
    }
}
