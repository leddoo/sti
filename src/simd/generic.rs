use super::*;


impl<T: SimdElement> Simd<T, 2> {
    #[inline(always)]
    pub fn new(v0: T, v1: T) -> Self {
        Self::from_array([v0, v1])
    }
}

impl<T: SimdElement> Simd<T, 4> {
    #[inline(always)]
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Self::from_array([v0, v1, v2, v3])
    }
}

impl<T: SimdElement, const N: usize> Simd<T, N> where (): SimdLanes<N> {
    #[inline(always)]
    pub fn from_array(array: [T; N]) -> Self {
        Self { p: PhantomData, v: <()>::repr_from_se(array) }
    }

    #[inline(always)]
    pub fn as_array(self) -> [T; N] {
        *self
    }
}

impl<T: SimdElement> Simd<T, 2> {
    #[inline(always)]
    pub fn x(self) -> T { self[0] }

    #[inline(always)]
    pub fn y(self) -> T { self[1] }
}

impl<T: SimdElement> Simd<T, 4> {
    #[inline(always)]
    pub fn x(self) -> T { self[0] }

    #[inline(always)]
    pub fn y(self) -> T { self[1] }

    #[inline(always)]
    pub fn z(self) -> T { self[2] }

    #[inline(always)]
    pub fn w(self) -> T { self[3] }
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
        unsafe { core::mem::transmute(self) }
    }
}

impl<T: SimdElement, const N: usize> core::ops::DerefMut for Simd<T, N> where (): SimdLanes<N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

