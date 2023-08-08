use super::*;


unsafe impl SimdElement for i32 {
    #[inline(always)]
    fn se_to_u32x2(v: [Self; 2]) -> [u32; 2] { unsafe { core::mem::transmute(v) } }

    #[inline(always)]
    fn se_to_u32x4(v: [Self; 4]) -> [u32; 4] { unsafe { core::mem::transmute(v) } }
}


pub type I32x<const N: usize> = Simd<i32, N>;
pub type I32x2 = Simd<i32, 2>;
pub type I32x4 = Simd<i32, 4>;

impl<const N: usize> I32x<N> where (): SimdLanes<N> {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ZERO() -> I32x<N> { I32x::splat(0) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ONE() -> I32x<N> { I32x::splat(1) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MIN() -> I32x<N> { I32x::splat(i32::MIN) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MAX() -> I32x<N> { I32x::splat(i32::MAX) }


    #[inline(always)]
    pub fn splat(v: i32) -> I32x<N> {
        Self { p: PhantomData, v: <()>::i32_splat(v) }
    }


    #[inline(always)]
    pub fn as_u32(self) -> U32x<N> {
        U32x { p: PhantomData, v: self.v }
    }

    #[inline(always)]
    pub fn to_f32(self) -> F32x<N> {
        F32x { p: PhantomData, v: <()>::i32_to_f32(self.v) }
    }


    #[inline(always)]
    pub fn min(self, rhs: I32x<N>) -> I32x<N> {
        I32x { p: PhantomData, v: <()>::i32_min(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn max(self, rhs: I32x<N>) -> I32x<N> {
        I32x { p: PhantomData, v: <()>::i32_max(self.v, rhs.v) }
    }


    #[inline(always)]
    pub fn eq(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_eq(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn ne(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_ne(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn le(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_le(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn lt(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_lt(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn ge(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_ge(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn gt(self, rhs: I32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::i32_gt(self.v, rhs.v) }
    }


    #[inline(always)]
    pub fn zip(self, rhs: I32x<N>) -> (I32x<N>, I32x<N>) {
        let (v1, v2) = <()>::repr_zip(self.v, rhs.v);
        (I32x { p: PhantomData, v: v1 },
         I32x { p: PhantomData, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: I32x<N>) -> (I32x<N>, I32x<N>) {
        let (v1, v2) = <()>::repr_unzip(self.v, rhs.v);
        (I32x { p: PhantomData, v: v1 },
         I32x { p: PhantomData, v: v2 })
    }
}


impl<const N: usize> core::fmt::Debug for I32x<N> where (): SimdLanes<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        (**self).fmt(f)
    }
}


impl<const N: usize> PartialEq for I32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        I32x::eq(*self, *other).all()
    }
}


impl<const N: usize> core::ops::Add for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        I32x { p: PhantomData, v: <()>::i32_add(self.v, rhs.v) }
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
        I32x { p: PhantomData, v: <()>::i32_sub(self.v, rhs.v) }
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
        I32x { p: PhantomData, v: <()>::i32_neg(self.v) }
    }
}


impl<const N: usize> core::ops::Shl<i32> for I32x<N> where (): SimdLanes<N> {
    type Output = I32x<N>;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Self::Output {
        I32x { p: PhantomData, v: <()>::i32_shl(self.v, rhs) }
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
        I32x { p: PhantomData, v: <()>::i32_shr(self.v, rhs) }
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

