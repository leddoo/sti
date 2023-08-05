use super::*;


impl SimdElement for u32 {}

pub type U32x<const N: usize> = Simd<u32, N>;
pub type U32x2 = Simd<u32, 2>;
pub type U32x4 = Simd<u32, 4>;

impl<const N: usize> U32x<N> where (): SimdLanes<N> {
}

