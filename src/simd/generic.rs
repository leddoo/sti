use super::*;


impl<T: SimdElement, const N: usize> Simd<T, N> where (): SimdLanes<N> {
    pub(crate) const ALIGN: <() as SimdLanes<N>>::Align = <() as SimdLanes<N>>::ALIGN;

    #[inline(always)]
    pub const fn new(v: [T; N]) -> Self {
        Self { align: Self::ALIGN, v }
    }

    #[inline(always)]
    pub const fn csplat(v: T) -> Self {
        Self { align: Self::ALIGN, v: [v; N] }
    }

    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        Self { align: Self::ALIGN, v: array }
    }

    #[inline(always)]
    pub fn to_array(self) -> [T; N] {
        self.v
    }

    #[inline(always)]
    pub const fn as_array(&self) -> &[T; N] {
        &self.v
    }

    #[inline(always)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.v
    }
}

impl<T: SimdElement, const N: usize> Into<Simd<T, N>> for [T; N] where (): SimdLanes<N> {
    #[inline(always)]
    fn into(self) -> Simd<T, N> {
        Simd::from_array(self)
    }
}

impl<T: SimdElement, const N: usize> core::ops::Deref for Simd<T, N> where (): SimdLanes<N> {
    type Target = [T; N];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T: SimdElement, const N: usize> core::ops::DerefMut for Simd<T, N> where (): SimdLanes<N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

