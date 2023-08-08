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

    #[inline(always)]
    pub fn as_u32(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn to_bool(self) -> bool {
        self.0 != 0
    }
}

impl core::fmt::Debug for B32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_bool().fmt(f)
    }
}

impl Into<bool> for B32 {
    #[inline(always)]
    fn into(self) -> bool {
        self.to_bool()
    }
}

impl Into<B32> for bool {
    #[inline(always)]
    fn into(self) -> B32 {
        B32::new(self)
    }
}

impl core::ops::BitAnd for B32 {
    type Output = B32;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        B32(self.0 & rhs.0)
    }
}

impl core::ops::BitOr for B32 {
    type Output = B32;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        B32(self.0 | rhs.0)
    }
}

impl core::ops::Not for B32 {
    type Output = B32;

    #[inline(always)]
    fn not(self) -> Self::Output {
        B32(!self.0)
    }
}


unsafe impl SimdElement for B32 {
    #[inline(always)]
    fn se_to_u32x2(v: [Self; 2]) -> [u32; 2] { unsafe { core::mem::transmute(v) } }

    #[inline(always)]
    fn se_to_u32x4(v: [Self; 4]) -> [u32; 4] { unsafe { core::mem::transmute(v) } }
}


pub type B32x<const N: usize> = Simd<B32, N>;
pub type B32x2 = Simd<B32, 2>;
pub type B32x4 = Simd<B32, 4>;

impl<const N: usize> B32x<N> where (): SimdLanes<N> {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn NONE() -> B32x<N> { B32x::splat(B32::FALSE) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ALL() -> B32x<N> { B32x::splat(B32::TRUE) }


    #[inline(always)]
    pub fn new_b(v: [bool; N]) -> Self {
        Self::new(v.map(B32::new))
    }

    #[inline(always)]
    pub fn splat(v: B32) -> Self {
        Self { p: PhantomData, v: <()>::b32_splat(v) }
    }

    #[inline(always)]
    pub fn splat_b(v: bool) -> Self {
        Self { p: PhantomData, v: <()>::b32_splat(v.into()) }
    }

    #[inline(always)]
    pub fn to_array_b(self) -> [bool; N] {
        self.map(B32::to_bool)
    }


    #[inline(always)]
    pub fn as_u32(self) -> U32x<N> {
        U32x { p: PhantomData, v: self.v }
    }

    #[inline(always)]
    pub fn as_i32(self) -> I32x<N> {
        I32x { p: PhantomData, v: self.v }
    }


    #[inline(always)]
    pub fn select<T: SimdElement>(self, on_true: Simd<T, N>, on_false: Simd<T, N>) -> Simd<T, N> {
        Simd { p: PhantomData, v: <()>::b32_select(self.v, on_true.v, on_false.v) }
    }


    #[inline(always)]
    pub fn none(self) -> bool {
        <()>::b32_none(self.v)
    }

    #[inline(always)]
    pub fn any(self) -> bool {
        <()>::b32_any(self.v)
    }

    #[inline(always)]
    pub fn all(self) -> bool {
        <()>::b32_all(self.v)
    }


    #[inline(always)]
    pub fn zip(self, rhs: B32x<N>) -> (B32x<N>, B32x<N>) {
        let (v1, v2) = <()>::repr_zip(self.v, rhs.v);
        (B32x { p: PhantomData, v: v1 },
         B32x { p: PhantomData, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: B32x<N>) -> (B32x<N>, B32x<N>) {
        let (v1, v2) = <()>::repr_unzip(self.v, rhs.v);
        (B32x { p: PhantomData, v: v1 },
         B32x { p: PhantomData, v: v2 })
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
        B32x { p: PhantomData, v: <()>::b32_and(self.v, rhs.v) }
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
        B32x { p: PhantomData, v: <()>::b32_or(self.v, rhs.v) }
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
        B32x { p: PhantomData, v: <()>::b32_not(self.v) }
    }
}

