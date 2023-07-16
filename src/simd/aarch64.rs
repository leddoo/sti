use core::arch::aarch64::*;


//
// B32x2
//

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct B32x2 {
    v: uint32x2_t,
}

impl B32x2 {
    #[inline(always)]
    pub fn new(v0: bool, v1: bool) -> Self {
        let v0 = (-(v0 as i32)) as u32;
        let v1 = (-(v1 as i32)) as u32;
        unsafe { core::mem::transmute([v0, v1]) }
    }

    #[inline(always)]
    pub fn from_array(vs: [bool; 2]) -> Self {
        Self::new(vs[0], vs[1])
    }

    #[inline(always)]
    pub fn to_array(self) -> [bool; 2] {
        unsafe {
            let u32s: [u32; 2] = core::mem::transmute(
                vneg_s32(vreinterpret_s32_u32(self.v)));
            core::mem::transmute(
                [u32s[0] as u8, u32s[1] as u8])
        }
    }

    #[inline(always)]
    pub fn to_array_u32_01(self) -> [u32; 2] {
        unsafe {
            core::mem::transmute(
                vneg_s32(vreinterpret_s32_u32(self.v)))
        }
    }
}


impl core::fmt::Debug for B32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array_u32_01().fmt(f)
    }
}



impl core::ops::Not for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        unsafe { Self { v: vmvn_u32(self.v) } }
    }
}

impl core::ops::BitAnd for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vand_u32(self.v, rhs.v) } }
    }
}

impl core::ops::BitOr for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vorr_u32(self.v, rhs.v) } }
    }
}



//
// F32x2
//

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct F32x2 {
    v: float32x2_t,
}

impl F32x2 {
    #[inline(always)]
    pub fn new(v0: f32, v1: f32) -> Self {
        Self::from_array([v0, v1])
    }

    #[inline(always)]
    pub fn from_array(vs: [f32; 2]) -> Self {
        unsafe { core::mem::transmute(vs) }
    }

    #[inline(always)]
    pub fn to_array(self) -> [f32; 2] {
        unsafe { core::mem::transmute(self.v) }
    }
}


impl core::fmt::Debug for F32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl core::ops::Add for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vadd_f32(self.v, rhs.v) } }
    }
}

impl core::ops::Sub for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vsub_f32(self.v, rhs.v) } }
    }
}

impl core::ops::Mul for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vmul_f32(self.v, rhs.v) } }
    }
}

impl core::ops::Div for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Self { v: vdiv_f32(self.v, rhs.v) } }
    }
}


impl F32x2 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x2 {
        unsafe { B32x2 { v: vceq_f32(self.v, other.v) } }
    }

    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x2 {
        !self.eq(other)
    }

    #[inline(always)]
    pub fn le(self, other: Self) -> B32x2 {
        unsafe { B32x2 { v: vcle_f32(self.v, other.v) } }
    }

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x2 {
        unsafe { B32x2 { v: vclt_f32(self.v, other.v) } }
    }

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x2 {
        unsafe { B32x2 { v: vcge_f32(self.v, other.v) } }
    }

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x2 {
        unsafe { B32x2 { v: vcgt_f32(self.v, other.v) } }
    }
}


