use super::*;


impl SimdElement for u32 {}


pub type U32x<const N: usize> = Simd<u32, N>;
pub type U32x2 = Simd<u32, 2>;
pub type U32x4 = Simd<u32, 4>;

impl<const N: usize> U32x<N> where (): SimdLanes<N> {
    pub const ZERO: U32x<N> = Simd::csplat(0);
    pub const ONE:  U32x<N> = Simd::csplat(1);
    pub const MIN:  U32x<N> = Simd::csplat(u32::MIN);
    pub const MAX:  U32x<N> = Simd::csplat(u32::MAX);

    #[inline(always)]
    pub fn splat(v: u32) -> U32x<N> {
        let v = <() as SimdLanes<N>>::u32_splat(v);
        U32x { align: U32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn as_i32(self) -> I32x<N> {
        let v = <() as SimdLanes<N>>::u32_as_i32(self.v);
        I32x { align: I32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn min(self, rhs: U32x<N>) -> U32x<N> {
        let v = <() as SimdLanes<N>>::u32_min(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn max(self, rhs: U32x<N>) -> U32x<N> {
        let v = <() as SimdLanes<N>>::u32_max(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn eq(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_eq(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn ne(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_ne(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn le(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_le(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn lt(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_lt(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn ge(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_ge(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn gt(self, rhs: U32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::u32_gt(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn zip(self, rhs: U32x<N>) -> (U32x<N>, U32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::u32_zip(self.v, rhs.v);
        (U32x { align: U32x::ALIGN, v: v1 },
         U32x { align: U32x::ALIGN, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: U32x<N>) -> (U32x<N>, U32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::u32_unzip(self.v, rhs.v);
        (U32x { align: U32x::ALIGN, v: v1 },
         U32x { align: U32x::ALIGN, v: v2 })
    }
}



impl<const N: usize> core::ops::Add for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_add(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::AddAssign for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> core::ops::Sub for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_sub(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::SubAssign for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}


impl<const N: usize> core::ops::Shl<u32> for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_shl(self.v, rhs);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::ShlAssign<u32> for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl<const N: usize> core::ops::Shr<u32> for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_shr(self.v, rhs);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::ShrAssign<u32> for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

impl<const N: usize> core::ops::BitAnd for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_and(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::BitAndAssign for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const N: usize> core::ops::BitOr for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_or(self.v, rhs.v);
        U32x { align: U32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::BitOrAssign for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const N: usize> core::ops::Not for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let v = <() as SimdLanes<N>>::u32_not(self.v);
        U32x { align: U32x::ALIGN, v }
    }
}

