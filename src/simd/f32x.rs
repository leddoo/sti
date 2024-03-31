use crate::float::F32Ext;
use super::*;


unsafe impl SimdElement for f32 {
    #[inline(always)]
    fn se_to_u32x2(v: [Self; 2]) -> [u32; 2] { unsafe { core::mem::transmute(v) } }

    #[inline(always)]
    fn se_to_u32x4(v: [Self; 4]) -> [u32; 4] { unsafe { core::mem::transmute(v) } }
}


pub type F32x<const N: usize> = Simd<f32, N>;
pub type F32x2 = Simd<f32, 2>;
pub type F32x4 = Simd<f32, 4>;

impl<const N: usize> Default for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::splat(Default::default())
    }
}

impl<const N: usize> F32x<N> where (): SimdLanes<N> {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ZERO() -> F32x<N> { F32x::splat(0.0) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ONE() -> F32x<N> { F32x::splat(1.0) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MIN() -> F32x<N> { F32x::splat(f32::MIN) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MAX() -> F32x<N> { F32x::splat(f32::MAX) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn INF() -> F32x<N> { F32x::splat(f32::INFINITY) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn NAN() -> F32x<N> { F32x::splat(f32::NAN) }


    #[inline(always)]
    pub fn splat(v: f32) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_splat(v) }
    }


    #[inline(always)]
    pub fn as_bits(self) -> U32x<N> {
        U32x { p: PhantomData, v: self.v }
    }

    #[inline(always)]
    pub fn from_bits(bits: U32x<N>) -> F32x<N> {
        F32x { p: PhantomData, v: bits.v }
    }


    /// - rounds towards zero.
    /// - behavior for values outside the `i32` range is platform dependent
    ///   and considered a bug (there is no guarantee that the program won't crash).
    ///   technically, this function should be unsafe, but that would make it rather
    ///   annoying to use.
    #[inline(always)]
    pub fn to_i32_unck(self) -> I32x<N> {
        I32x { p: PhantomData, v: <()>::f32_to_i32_unck(self.v) }
    }

    /// - rounds towards zero.
    /// - clamps results outside the `i32` range.
    #[inline(always)]
    pub fn to_i32(self) -> I32x<N> {
        I32x { p: PhantomData, v: <()>::f32_to_i32(self.v) }
    }


    #[inline(always)]
    pub fn floor(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_floor(self.v) }
    }

    #[inline(always)]
    pub fn ceil(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_ceil(self.v) }
    }

    #[inline(always)]
    pub fn round(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_round(self.v) }
    }

    #[inline(always)]
    pub fn trunc(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_trunc(self.v) }
    }

    #[inline(always)]
    pub fn abs(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_abs(self.v) }
    }

    #[inline(always)]
    pub fn sqrt(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_sqrt(self.v) }
    }

    #[inline(always)]
    pub fn with_sign_of(self, sign: F32x<N>) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_with_sign_of(self.v, sign.v) }
    }


    #[inline(always)]
    pub fn hadd(self) -> f32 {
        <()>::f32_hadd(self.v)
    }


    #[inline(always)]
    pub fn dot(self, rhs: F32x<N>) -> f32 {
        (self * rhs).hadd()
    }

    #[inline(always)]
    pub fn length_sq(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.length_sq().fsqrt()
    }


    #[inline(always)]
    pub fn lerp(self, rhs: F32x<N>, t: f32) -> F32x<N> {
        self.lerp_v(rhs, F32x::splat(t))
    }

    #[inline(always)]
    pub fn lerp_v(self, rhs: F32x<N>, t: F32x<N>) -> F32x<N> {
        (F32x::splat(1.0) - t)*self + t*rhs
    }


    #[inline(always)]
    pub fn min(self, rhs: F32x<N>) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_min(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn max(self, rhs: F32x<N>) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::f32_max(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn at_least(self, rhs: F32x<N>) -> F32x<N> {
        self.max(rhs)
    }

    #[inline(always)]
    pub fn at_most(self, rhs: F32x<N>) -> F32x<N> {
        self.min(rhs)
    }

    #[inline(always)]
    pub fn clamp(self, low: F32x<N>, high: F32x<N>) -> F32x<N> {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn eq(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_eq(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn ne(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_ne(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn le(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_le(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn lt(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_lt(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn ge(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_ge(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn gt(self, rhs: F32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::f32_gt(self.v, rhs.v) }
    }


    #[inline(always)]
    pub fn zip(self, rhs: F32x<N>) -> (F32x<N>, F32x<N>) {
        let (v1, v2) = <()>::repr_zip(self.v, rhs.v);
        (F32x { p: PhantomData, v: v1 },
         F32x { p: PhantomData, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: F32x<N>) -> (F32x<N>, F32x<N>) {
        let (v1, v2) = <()>::repr_unzip(self.v, rhs.v);
        (F32x { p: PhantomData, v: v1 },
         F32x { p: PhantomData, v: v2 })
    }
}


impl<const N: usize> core::fmt::Debug for F32x<N> where (): SimdLanes<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        (**self).fmt(f)
    }
}


impl<const N: usize> PartialEq for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        F32x::eq(*self, *other).all()
    }
}


impl<const N: usize> core::ops::Neg for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        F32x { p: PhantomData, v: <()>::f32_neg(self.v) }
    }
}

impl<const N: usize> core::ops::Add for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        F32x { p: PhantomData, v: <()>::f32_add(self.v, rhs.v) }
    }
}

impl<const N: usize> core::ops::AddAssign for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> core::ops::Sub for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        F32x { p: PhantomData, v: <()>::f32_sub(self.v, rhs.v) }
    }
}

impl<const N: usize> core::ops::SubAssign for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const N: usize> core::ops::Mul for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        F32x { p: PhantomData, v: <()>::f32_mul(self.v, rhs.v) }
    }
}

impl<const N: usize> core::ops::MulAssign for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: usize> core::ops::Mul<f32> for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        self * F32x::splat(rhs)
    }
}

impl<const N: usize> core::ops::MulAssign<f32> for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl<const N: usize> core::ops::Mul<F32x<N>> for f32 where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn mul(self, rhs: F32x<N>) -> Self::Output {
        F32x::splat(self) * rhs
    }
}

impl<const N: usize> core::ops::Div for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        F32x { p: PhantomData, v: <()>::f32_div(self.v, rhs.v) }
    }
}

impl<const N: usize> core::ops::DivAssign for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const N: usize> core::ops::Div<f32> for F32x<N> where (): SimdLanes<N> {
    type Output = F32x<N>;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self / F32x::splat(rhs)
    }
}

impl<const N: usize> core::ops::DivAssign<f32> for F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

