use super::*;


unsafe impl SimdElement for u32 {
    #[inline(always)]
    fn se_to_u32x2(v: [Self; 2]) -> [u32; 2] { unsafe { core::mem::transmute(v) } }

    #[inline(always)]
    fn se_to_u32x4(v: [Self; 4]) -> [u32; 4] { unsafe { core::mem::transmute(v) } }
}


pub type U32x<const N: usize> = Simd<u32, N>;
pub type U32x2 = Simd<u32, 2>;
pub type U32x4 = Simd<u32, 4>;

impl<const N: usize> Default for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn default() -> Self {
        Self::splat(Default::default())
    }
}

impl<const N: usize> U32x<N> where (): SimdLanes<N> {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ZERO() -> U32x<N> { U32x::splat(0) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn ONE() -> U32x<N> { U32x::splat(1) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MIN() -> U32x<N> { U32x::splat(u32::MIN) }

    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn MAX() -> U32x<N> { U32x::splat(u32::MAX) }


    #[inline(always)]
    pub fn splat(v: u32) -> U32x<N> {
        U32x { p: PhantomData, v: <()>::u32_splat(v) }
    }


    #[inline(always)]
    pub fn as_i32(self) -> I32x<N> {
        I32x { p: PhantomData, v: <()>::u32_as_i32(self.v) }
    }


    #[inline(always)]
    pub fn min(self, rhs: U32x<N>) -> U32x<N> {
        U32x { p: PhantomData, v: <()>::u32_min(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn max(self, rhs: U32x<N>) -> U32x<N> {
        U32x { p: PhantomData, v: <()>::u32_max(self.v, rhs.v) }
    }


    #[inline(always)]
    pub fn eq(self, rhs: U32x<N>) -> B32x<N> {
        self.as_i32().eq(rhs.as_i32())
    }

    #[inline(always)]
    pub fn ne(self, rhs: U32x<N>) -> B32x<N> {
        self.as_i32().ne(rhs.as_i32())
    }

    #[inline(always)]
    pub fn le(self, rhs: U32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::u32_le(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn lt(self, rhs: U32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::u32_lt(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn ge(self, rhs: U32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::u32_ge(self.v, rhs.v) }
    }

    #[inline(always)]
    pub fn gt(self, rhs: U32x<N>) -> B32x<N> {
        B32x { p: PhantomData, v: <()>::u32_gt(self.v, rhs.v) }
    }


    #[inline(always)]
    pub fn zip(self, rhs: U32x<N>) -> (U32x<N>, U32x<N>) {
        let (v1, v2) = <()>::repr_zip(self.v, rhs.v);
        (U32x { p: PhantomData, v: v1 },
         U32x { p: PhantomData, v: v2 })
    }

    #[inline(always)]
    pub fn unzip(self, rhs: U32x<N>) -> (U32x<N>, U32x<N>) {
        let (v1, v2) = <()>::repr_unzip(self.v, rhs.v);
        (U32x { p: PhantomData, v: v1 },
         U32x { p: PhantomData, v: v2 })
    }
}


impl<const N: usize> core::fmt::Debug for U32x<N> where (): SimdLanes<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        (**self).fmt(f)
    }
}


impl<const N: usize> PartialEq for U32x<N> where (): SimdLanes<N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        U32x::eq(*self, *other).all()
    }
}


impl<const N: usize> core::ops::Add for U32x<N> where (): SimdLanes<N> {
    type Output = U32x<N>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        (self.as_i32() + rhs.as_i32()).as_u32()
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
        (self.as_i32() - rhs.as_i32()).as_u32()
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
        (self.as_i32() << (rhs as i32)).as_u32()
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
        U32x { p: PhantomData, v: <()>::u32_shr(self.v, rhs) }
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
        U32x { p: PhantomData, v: <()>::u32_and(self.v, rhs.v) }
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
        U32x { p: PhantomData, v: <()>::u32_or(self.v, rhs.v) }
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
        U32x { p: PhantomData, v: <()>::u32_not(self.v) }
    }
}

