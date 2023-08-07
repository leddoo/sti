use super::*;


impl SimdElement for i32 {}


pub type I32x<const N: usize> = Simd<i32, N>;
pub type I32x2 = Simd<i32, 2>;
pub type I32x4 = Simd<i32, 4>;

impl<const N: usize> I32x<N> where (): SimdLanes<N> {
    pub const ZERO: I32x<N> = Simd::csplat(0);
    pub const ONE:  I32x<N> = Simd::csplat(1);
    pub const MIN:  I32x<N> = Simd::csplat(i32::MIN);
    pub const MAX:  I32x<N> = Simd::csplat(i32::MAX);

    #[inline(always)]
    pub fn splat(v: i32) -> I32x<N> {
        let v = <() as SimdLanes<N>>::i32_splat(v);
        I32x { align: I32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn as_u32(self) -> U32x<N> {
        let v = <() as SimdLanes<N>>::i32_as_u32(self.v);
        U32x { align: U32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn to_f32(self) -> F32x<N> {
        let v = <() as SimdLanes<N>>::i32_to_f32(self.v);
        F32x { align: F32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn min(self, rhs: I32x<N>) -> I32x<N> {
        let v = <() as SimdLanes<N>>::i32_min(self.v, rhs.v);
        I32x { align: I32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn max(self, rhs: I32x<N>) -> I32x<N> {
        let v = <() as SimdLanes<N>>::i32_max(self.v, rhs.v);
        I32x { align: I32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn eq(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_eq(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn ne(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_ne(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn le(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_le(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn lt(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_lt(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn ge(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_ge(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }

    #[inline(always)]
    pub fn gt(self, rhs: I32x<N>) -> B32x<N> {
        let v = <() as SimdLanes<N>>::i32_gt(self.v, rhs.v);
        B32x { align: B32x::ALIGN, v }
    }


    #[inline(always)]
    pub fn zip(self, rhs: I32x<N>) -> (I32x<N>, I32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::u32_zip(self.sb_to().v, rhs.sb_to().v);
        unsafe {
            (Self::sb_from(U32x { align: U32x::ALIGN, v: v1 }),
             Self::sb_from(U32x { align: U32x::ALIGN, v: v2 }))
        }
    }

    #[inline(always)]
    pub fn unzip(self, rhs: B32x<N>) -> (I32x<N>, I32x<N>) {
        let (v1, v2) = <() as SimdLanes<N>>::u32_unzip(self.sb_to().v, rhs.sb_to().v);
        unsafe {
            (Self::sb_from(U32x { align: U32x::ALIGN, v: v1 }),
             Self::sb_from(U32x { align: U32x::ALIGN, v: v2 }))
        }
    }
}



impl<const N: usize> core::ops::Add for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::i32_add(self.v, rhs.v);
        I32x { align: I32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::AddAssign for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> core::ops::Sub for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let v = <() as SimdLanes<N>>::i32_sub(self.v, rhs.v);
        I32x { align: I32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::SubAssign for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const N: usize> core::ops::Neg for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        let v = <() as SimdLanes<N>>::i32_neg(self.v);
        I32x { align: I32x::ALIGN, v }
    }
}


impl<const N: usize> core::ops::Shl<i32> for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        let v = <() as SimdLanes<N>>::i32_shl(self.v, rhs);
        I32x { align: I32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::ShlAssign<i32> for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        *self = *self << rhs;
    }
}

impl<const N: usize> core::ops::Shr<i32> for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn shr(self, rhs: i32) -> Self::Output {
        let v = <() as SimdLanes<N>>::i32_shr(self.v, rhs);
        I32x { align: I32x::ALIGN, v }
    }
}

impl<const N: usize> core::ops::ShrAssign<i32> for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = *self >> rhs;
    }
}

impl<const N: usize> core::ops::BitAnd for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        (self.as_u32() & rhs.as_u32()).as_i32()
    }
}

impl<const N: usize> core::ops::BitAndAssign for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl<const N: usize> core::ops::BitOr for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        (self.as_u32() | rhs.as_u32()).as_i32()
    }
}

impl<const N: usize> core::ops::BitOrAssign for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl<const N: usize> core::ops::Not for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        (!self.as_u32()).as_i32()
    }
}

