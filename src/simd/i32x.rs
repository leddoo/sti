use super::*;


impl SimdElement for i32 {}


pub type I32x<const N: usize> = Simd<i32, N>;
pub type I32x2 = Simd<i32, 2>;
pub type I32x4 = Simd<i32, 4>;

impl<const N: usize> I32x<N> where (): SimdLanes<N> {
}

