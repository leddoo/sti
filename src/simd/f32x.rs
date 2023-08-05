use super::*;


impl SimdElement for f32 {}

pub type F32x<const N: usize> = Simd<f32, N>;
pub type F32x2 = Simd<f32, 2>;
pub type F32x4 = Simd<f32, 4>;

impl<const N: usize> F32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    pub fn splat(v: f32) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn floor(self) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn ceil(self) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn round(self) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn trunc(self) -> Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn sqrt(self) -> Self {
        unimplemented!()
    }
}

