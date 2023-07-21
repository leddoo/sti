use core::arch::aarch64::*;
use core::mem::transmute;

#[derive(Clone, Copy)]
#[repr(align(8))]
pub struct B32x2 {
    v: uint32x2_t,
}

impl B32x2 {
    pub const NONE: B32x2 = B32x2::splat(false);
    pub const ALL:  B32x2 = B32x2::splat(true);

    #[inline(always)]
    pub const fn new(v0: bool, v1: bool) -> Self {
        let v0 = -(v0 as i32);
        let v1 = -(v1 as i32);
        unsafe { transmute([v0, v1]) }
    }

    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self::from_array([v; 2])
    }

    #[inline(always)]
    pub const fn from_array(vs: [bool; 2]) -> Self {
        Self::new(vs[0], vs[1])
    }

    #[inline(always)]
    pub fn to_array_u32_01(self) -> [u32; 2] {
        (-self.as_u32()).to_array()
    }

    #[inline(always)]
    pub fn to_array(self) -> [bool; 2] {
        let u32s = self.to_array_u32_01();
        unsafe { transmute([u32s[0] as u8, u32s[1] as u8]) }
    }
}


impl Into<B32x2> for bool {
    #[inline(always)]
    fn into(self) -> B32x2 {
        B32x2::splat(self)
    }
}

impl Into<B32x2> for [bool; 2] {
    #[inline(always)]
    fn into(self) -> B32x2 {
        B32x2::from_array(self)
    }
}

impl Default for B32x2 {
    #[inline(always)]
    fn default() -> Self { Self::NONE }
}


impl core::fmt::Debug for B32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array_u32_01().fmt(f)
    }
}


impl B32x2 {
    #[inline(always)]
    pub fn as_u32(self) -> U32x2 { unsafe { transmute(self) } }

    #[inline(always)]
    pub fn as_i32(self) -> U32x2 { unsafe { transmute(self) } }
}


impl B32x2 {
    #[inline(always)]
    pub fn select_u32(self, on_false: U32x2, on_true: U32x2) -> U32x2 { unsafe {
        let this     = self.v;
        let on_false = on_false.v;
        let on_true  = on_true.v;
        let r = vbsl_u32(this, on_true, on_false);
        U32x2 { v: r }
    }}

    #[inline(always)]
    pub fn select_b32(self, on_false: B32x2, on_true: B32x2) -> B32x2 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn select_i32(self, on_false: I32x2, on_true: I32x2) -> I32x2 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn select_f32(self, on_false: F32x2, on_true: F32x2) -> F32x2 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn any(self) -> bool {
        self.as_u32().hmax() != 0
    }

    #[inline(always)]
    pub fn all(self) -> bool {
        (!self).as_u32().hmax() == 0
    }
}


impl core::ops::Not for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output { unsafe {
        let r = vmvn_u32(self.v);
        Self { v: r }
    }}
}

impl core::ops::BitAnd for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output { unsafe {
        let r = vand_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::BitOr for B32x2 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output { unsafe {
        let r = vorr_u32(self.v, rhs.v);
        Self { v: r }
    }}
}


#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct B32x4 {
    v: uint32x4_t,
}

impl B32x4 {
    pub const NONE: B32x4 = B32x4::splat(false);
    pub const ALL:  B32x4 = B32x4::splat(true);

    #[inline(always)]
    pub const fn new(v0: bool, v1: bool, v2: bool, v3: bool) -> Self {
        let v0 = -(v0 as i32);
        let v1 = -(v1 as i32);
        let v2 = -(v2 as i32);
        let v3 = -(v3 as i32);
        unsafe { transmute([v0, v1, v2, v3]) }
    }

    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self::from_array([v; 4])
    }

    #[inline(always)]
    pub const fn from_array(vs: [bool; 4]) -> Self {
        Self::new(vs[0], vs[1], vs[2], vs[3])
    }

    #[inline(always)]
    pub fn to_array_u32_01(self) -> [u32; 4] {
        (-self.as_u32()).to_array()
    }

    #[inline(always)]
    pub fn to_array(self) -> [bool; 4] {
        let u32s = self.to_array_u32_01();
        unsafe { transmute([u32s[0] as u8, u32s[1] as u8, u32s[2] as u8, u32s[3] as u8]) }
    }
}


impl Into<B32x4> for bool {
    #[inline(always)]
    fn into(self) -> B32x4 {
        B32x4::splat(self)
    }
}

impl Into<B32x4> for [bool; 4] {
    #[inline(always)]
    fn into(self) -> B32x4 {
        B32x4::from_array(self)
    }
}

impl Default for B32x4 {
    #[inline(always)]
    fn default() -> Self { Self::NONE }
}


impl core::fmt::Debug for B32x4 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array_u32_01().fmt(f)
    }
}


impl B32x4 {
    #[inline(always)]
    pub fn as_u32(self) -> U32x4 { unsafe { transmute(self) } }

    #[inline(always)]
    pub fn as_i32(self) -> U32x4 { unsafe { transmute(self) } }
}


impl B32x4 {
    #[inline(always)]
    pub fn select_u32(self, on_false: U32x4, on_true: U32x4) -> U32x4 { unsafe {
        let this     = self.v;
        let on_false = on_false.v;
        let on_true  = on_true.v;
        let r = vbslq_u32(this, on_true, on_false);
        U32x4 { v: r }
    }}

    #[inline(always)]
    pub fn select_b32(self, on_false: B32x4, on_true: B32x4) -> B32x4 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn select_i32(self, on_false: I32x4, on_true: I32x4) -> I32x4 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn select_f32(self, on_false: F32x4, on_true: F32x4) -> F32x4 {
        unsafe { transmute(self.select_u32(transmute(on_false), transmute(on_true))) }
    }

    #[inline(always)]
    pub fn any(self) -> bool {
        self.as_u32().hmax() != 0
    }

    #[inline(always)]
    pub fn all(self) -> bool {
        (!self).as_u32().hmax() == 0
    }
}


impl core::ops::Not for B32x4 {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output { unsafe {
        let r = vmvnq_u32(self.v);
        Self { v: r }
    }}
}

impl core::ops::BitAnd for B32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output { unsafe {
        let r = vandq_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::BitOr for B32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output { unsafe {
        let r = vorrq_u32(self.v, rhs.v);
        Self { v: r }
    }}
}


#[derive(Clone, Copy)]
#[repr(align(8))]
pub struct I32x2 {
    v: int32x2_t,
}

impl I32x2 {
    pub const ZERO: I32x2 = I32x2::splat(0);
    pub const ONE:  I32x2 = I32x2::splat(1);
    pub const MIN:  I32x2 = I32x2::splat(i32::MIN);
    pub const MAX:  I32x2 = I32x2::splat(i32::MAX);

    #[inline(always)]
    pub const fn new(v0: i32, v1: i32) -> Self {
        Self::from_array([v0, v1])
    }

    #[inline(always)]
    pub const fn splat(v: i32) -> Self {
        Self::from_array([v; 2])
    }

    #[inline(always)]
    pub const fn from_array(vs: [i32; 2]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [i32; 2] {
        unsafe { transmute(self.v) }
    }
}

impl Into<I32x2> for i32 {
    #[inline(always)]
    fn into(self) -> I32x2 {
        I32x2::splat(self)
    }
}

impl Into<I32x2> for [i32; 2] {
    #[inline(always)]
    fn into(self) -> I32x2 {
        I32x2::from_array(self)
    }
}

impl Default for I32x2 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for I32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl I32x2 {
    #[inline(always)]
    pub fn x(self) -> i32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> i32 { self[1] }
}

impl core::ops::Deref for I32x2 {
    type Target = [i32; 2];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for I32x2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl I32x2 {
    #[inline(always)]
    pub fn as_u32(self) -> U32x2 { unsafe { transmute(self) } }

    #[inline(always)]
    pub fn to_f32(self) -> F32x2 { unsafe {
        let r = vcvt_f32_s32(self.v);
        F32x2 { v: r }
    }}
}


impl core::ops::Add for I32x2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vadd_s32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for I32x2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsub_s32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for I32x2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
            unsafe {
        let r = vneg_s32(self.v);
        Self { v: r }
    }
    }
}


impl I32x2 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vmin_s32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmax_s32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> i32 { unsafe {
        vminv_s32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> i32 { unsafe {
        vmaxv_s32(self.v)
    }}
}


impl I32x2 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x2 { unsafe {
        let r = vceq_s32(self.v, other.v);
        B32x2 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x2 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x2 { unsafe {
        let r = vcle_s32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x2 { unsafe {
        let r = vclt_s32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x2 { unsafe {
        let r = vcge_s32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x2 { unsafe {
        let r = vcgt_s32(self.v, other.v);
        B32x2 { v: r }
    }}
}

impl PartialEq for I32x2 {
    fn eq(&self, other: &Self) -> bool {
        I32x2::eq(*self, *other).all()
    }
}


#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct I32x4 {
    v: int32x4_t,
}

impl I32x4 {
    pub const ZERO: I32x4 = I32x4::splat(0);
    pub const ONE:  I32x4 = I32x4::splat(1);
    pub const MIN:  I32x4 = I32x4::splat(i32::MIN);
    pub const MAX:  I32x4 = I32x4::splat(i32::MAX);

    #[inline(always)]
    pub const fn new(v0: i32, v1: i32, v2: i32, v3: i32) -> Self {
        Self::from_array([v0, v1, v2, v3])
    }

    #[inline(always)]
    pub const fn splat(v: i32) -> Self {
        Self::from_array([v; 4])
    }

    #[inline(always)]
    pub const fn from_array(vs: [i32; 4]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [i32; 4] {
        unsafe { transmute(self.v) }
    }
}

impl Into<I32x4> for i32 {
    #[inline(always)]
    fn into(self) -> I32x4 {
        I32x4::splat(self)
    }
}

impl Into<I32x4> for [i32; 4] {
    #[inline(always)]
    fn into(self) -> I32x4 {
        I32x4::from_array(self)
    }
}

impl Default for I32x4 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for I32x4 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl I32x4 {
    #[inline(always)]
    pub fn x(self) -> i32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> i32 { self[1] }

    #[inline(always)]
    pub fn z(self) -> i32 { self[2] }

    #[inline(always)]
    pub fn w(self) -> i32 { self[3] }
}

impl core::ops::Deref for I32x4 {
    type Target = [i32; 4];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for I32x4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl I32x4 {
    #[inline(always)]
    pub fn as_u32(self) -> U32x4 { unsafe { transmute(self) } }

    #[inline(always)]
    pub fn to_f32(self) -> F32x4 { unsafe {
        let r = vcvtq_f32_s32(self.v);
        F32x4 { v: r }
    }}
}


impl core::ops::Add for I32x4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vaddq_s32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for I32x4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsubq_s32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for I32x4 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
            unsafe {
        let r = vnegq_s32(self.v);
        Self { v: r }
    }
    }
}


impl I32x4 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vminq_s32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmaxq_s32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> i32 { unsafe {
        vminvq_s32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> i32 { unsafe {
        vmaxvq_s32(self.v)
    }}
}


impl I32x4 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x4 { unsafe {
        let r = vceqq_s32(self.v, other.v);
        B32x4 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x4 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x4 { unsafe {
        let r = vcleq_s32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x4 { unsafe {
        let r = vcltq_s32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x4 { unsafe {
        let r = vcgeq_s32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x4 { unsafe {
        let r = vcgtq_s32(self.v, other.v);
        B32x4 { v: r }
    }}
}

impl PartialEq for I32x4 {
    fn eq(&self, other: &Self) -> bool {
        I32x4::eq(*self, *other).all()
    }
}


#[derive(Clone, Copy)]
#[repr(align(8))]
pub struct U32x2 {
    v: uint32x2_t,
}

impl U32x2 {
    pub const ZERO: U32x2 = U32x2::splat(0);
    pub const ONE:  U32x2 = U32x2::splat(1);
    pub const MIN:  U32x2 = U32x2::splat(u32::MIN);
    pub const MAX:  U32x2 = U32x2::splat(u32::MAX);

    #[inline(always)]
    pub const fn new(v0: u32, v1: u32) -> Self {
        Self::from_array([v0, v1])
    }

    #[inline(always)]
    pub const fn splat(v: u32) -> Self {
        Self::from_array([v; 2])
    }

    #[inline(always)]
    pub const fn from_array(vs: [u32; 2]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [u32; 2] {
        unsafe { transmute(self.v) }
    }
}

impl Into<U32x2> for u32 {
    #[inline(always)]
    fn into(self) -> U32x2 {
        U32x2::splat(self)
    }
}

impl Into<U32x2> for [u32; 2] {
    #[inline(always)]
    fn into(self) -> U32x2 {
        U32x2::from_array(self)
    }
}

impl Default for U32x2 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for U32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl U32x2 {
    #[inline(always)]
    pub fn x(self) -> u32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> u32 { self[1] }
}

impl core::ops::Deref for U32x2 {
    type Target = [u32; 2];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for U32x2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl U32x2 {
    #[inline(always)]
    pub fn as_i32(self) -> I32x2 { unsafe { transmute(self) } }
}


impl core::ops::Add for U32x2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vadd_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for U32x2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsub_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for U32x2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        (-self.as_i32()).as_u32()
    }
}


impl U32x2 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vmin_u32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmax_u32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> u32 { unsafe {
        vminv_u32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> u32 { unsafe {
        vmaxv_u32(self.v)
    }}
}


impl U32x2 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x2 { unsafe {
        let r = vceq_u32(self.v, other.v);
        B32x2 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x2 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x2 { unsafe {
        let r = vcle_u32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x2 { unsafe {
        let r = vclt_u32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x2 { unsafe {
        let r = vcge_u32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x2 { unsafe {
        let r = vcgt_u32(self.v, other.v);
        B32x2 { v: r }
    }}
}

impl PartialEq for U32x2 {
    fn eq(&self, other: &Self) -> bool {
        U32x2::eq(*self, *other).all()
    }
}


#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct U32x4 {
    v: uint32x4_t,
}

impl U32x4 {
    pub const ZERO: U32x4 = U32x4::splat(0);
    pub const ONE:  U32x4 = U32x4::splat(1);
    pub const MIN:  U32x4 = U32x4::splat(u32::MIN);
    pub const MAX:  U32x4 = U32x4::splat(u32::MAX);

    #[inline(always)]
    pub const fn new(v0: u32, v1: u32, v2: u32, v3: u32) -> Self {
        Self::from_array([v0, v1, v2, v3])
    }

    #[inline(always)]
    pub const fn splat(v: u32) -> Self {
        Self::from_array([v; 4])
    }

    #[inline(always)]
    pub const fn from_array(vs: [u32; 4]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [u32; 4] {
        unsafe { transmute(self.v) }
    }
}

impl Into<U32x4> for u32 {
    #[inline(always)]
    fn into(self) -> U32x4 {
        U32x4::splat(self)
    }
}

impl Into<U32x4> for [u32; 4] {
    #[inline(always)]
    fn into(self) -> U32x4 {
        U32x4::from_array(self)
    }
}

impl Default for U32x4 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for U32x4 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl U32x4 {
    #[inline(always)]
    pub fn x(self) -> u32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> u32 { self[1] }

    #[inline(always)]
    pub fn z(self) -> u32 { self[2] }

    #[inline(always)]
    pub fn w(self) -> u32 { self[3] }
}

impl core::ops::Deref for U32x4 {
    type Target = [u32; 4];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for U32x4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl U32x4 {
    #[inline(always)]
    pub fn as_i32(self) -> I32x4 { unsafe { transmute(self) } }
}


impl core::ops::Add for U32x4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vaddq_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for U32x4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsubq_u32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for U32x4 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        (-self.as_i32()).as_u32()
    }
}


impl U32x4 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vminq_u32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmaxq_u32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> u32 { unsafe {
        vminvq_u32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> u32 { unsafe {
        vmaxvq_u32(self.v)
    }}
}


impl U32x4 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x4 { unsafe {
        let r = vceqq_u32(self.v, other.v);
        B32x4 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x4 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x4 { unsafe {
        let r = vcleq_u32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x4 { unsafe {
        let r = vcltq_u32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x4 { unsafe {
        let r = vcgeq_u32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x4 { unsafe {
        let r = vcgtq_u32(self.v, other.v);
        B32x4 { v: r }
    }}
}

impl PartialEq for U32x4 {
    fn eq(&self, other: &Self) -> bool {
        U32x4::eq(*self, *other).all()
    }
}


#[derive(Clone, Copy)]
#[repr(align(8))]
pub struct F32x2 {
    v: float32x2_t,
}

impl F32x2 {
    pub const ZERO: F32x2 = F32x2::splat(0.0);
    pub const ONE:  F32x2 = F32x2::splat(1.0);
    pub const MIN:  F32x2 = F32x2::splat(f32::MIN);
    pub const MAX:  F32x2 = F32x2::splat(f32::MAX);

    #[inline(always)]
    pub const fn new(v0: f32, v1: f32) -> Self {
        Self::from_array([v0, v1])
    }

    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self::from_array([v; 2])
    }

    #[inline(always)]
    pub const fn from_array(vs: [f32; 2]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [f32; 2] {
        unsafe { transmute(self.v) }
    }
}

impl Into<F32x2> for f32 {
    #[inline(always)]
    fn into(self) -> F32x2 {
        F32x2::splat(self)
    }
}

impl Into<F32x2> for [f32; 2] {
    #[inline(always)]
    fn into(self) -> F32x2 {
        F32x2::from_array(self)
    }
}

impl Default for F32x2 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for F32x2 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl F32x2 {
    #[inline(always)]
    pub fn x(self) -> f32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> f32 { self[1] }
}

impl core::ops::Deref for F32x2 {
    type Target = [f32; 2];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for F32x2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl F32x2 {
    /// behavior for values outside the `i32` range is platform dependent
    /// and considered a bug (there is no guarantee that the program won't crash).
    /// technically, this function should be unsafe, but that would make it rather
    /// annoying to use.
    #[inline(always)]
    pub fn to_i32_unck(self) -> I32x2 { unsafe {
        let r = vcvtm_s32_f32(self.v);
        I32x2 { v: r }
    }}
}

impl F32x2 {
    #[inline(always)]
    pub const fn to_bits(self) -> U32x2 { unsafe { transmute(self) } }

    #[inline(always)]
    pub const fn from_bits(v: U32x2) -> Self { unsafe { transmute(v) } }

    #[inline(always)]
    pub fn floor(self) -> Self { unsafe {
        let r = vrndm_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn ceil(self) -> Self { unsafe {
        let r = vrndp_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn round(self) -> Self { unsafe {
        let r = vrndn_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn trunc(self) -> Self { unsafe {
        let r = vrnd_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self.lerpv(other, t.into())
    }

    #[inline(always)]
    pub fn lerpv(self, other: Self, ts: Self) -> Self {
        (Self::ONE - ts)*self + ts*other
    }
}


impl core::ops::Add for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vadd_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsub_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
            unsafe {
        let r = vneg_f32(self.v);
        Self { v: r }
    }
    }
}

impl F32x2 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vmin_f32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmax_f32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> f32 { unsafe {
        vminv_f32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> f32 { unsafe {
        vmaxv_f32(self.v)
    }}
}


impl core::ops::Mul for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output { unsafe {
        let r = vmul_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Mul<f32> for F32x2 {
    type Output = F32x2;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        self * F32x2::splat(rhs)
    }
}

impl core::ops::Mul<F32x2> for f32 {
    type Output = F32x2;

    #[inline(always)]
    fn mul(self, rhs: F32x2) -> Self::Output {
        F32x2::splat(self) * rhs
    }
}

impl core::ops::Div for F32x2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output { unsafe {
        let r = vdiv_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Div<f32> for F32x2 {
    type Output = F32x2;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self / F32x2::splat(rhs)
    }
}

impl F32x2 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x2 { unsafe {
        let r = vceq_f32(self.v, other.v);
        B32x2 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x2 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x2 { unsafe {
        let r = vcle_f32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x2 { unsafe {
        let r = vclt_f32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x2 { unsafe {
        let r = vcge_f32(self.v, other.v);
        B32x2 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x2 { unsafe {
        let r = vcgt_f32(self.v, other.v);
        B32x2 { v: r }
    }}
}

impl PartialEq for F32x2 {
    fn eq(&self, other: &Self) -> bool {
        F32x2::eq(*self, *other).all()
    }
}


#[derive(Clone, Copy)]
#[repr(align(16))]
pub struct F32x4 {
    v: float32x4_t,
}

impl F32x4 {
    pub const ZERO: F32x4 = F32x4::splat(0.0);
    pub const ONE:  F32x4 = F32x4::splat(1.0);
    pub const MIN:  F32x4 = F32x4::splat(f32::MIN);
    pub const MAX:  F32x4 = F32x4::splat(f32::MAX);

    #[inline(always)]
    pub const fn new(v0: f32, v1: f32, v2: f32, v3: f32) -> Self {
        Self::from_array([v0, v1, v2, v3])
    }

    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self::from_array([v; 4])
    }

    #[inline(always)]
    pub const fn from_array(vs: [f32; 4]) -> Self {
        unsafe { transmute(vs) }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [f32; 4] {
        unsafe { transmute(self.v) }
    }
}

impl Into<F32x4> for f32 {
    #[inline(always)]
    fn into(self) -> F32x4 {
        F32x4::splat(self)
    }
}

impl Into<F32x4> for [f32; 4] {
    #[inline(always)]
    fn into(self) -> F32x4 {
        F32x4::from_array(self)
    }
}

impl Default for F32x4 {
    #[inline(always)]
    fn default() -> Self { Self::ZERO }
}


impl core::fmt::Debug for F32x4 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_array().fmt(f)
    }
}


impl F32x4 {
    #[inline(always)]
    pub fn x(self) -> f32 { self[0] }

    #[inline(always)]
    pub fn y(self) -> f32 { self[1] }

    #[inline(always)]
    pub fn z(self) -> f32 { self[2] }

    #[inline(always)]
    pub fn w(self) -> f32 { self[3] }
}

impl core::ops::Deref for F32x4 {
    type Target = [f32; 4];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.v) }
    }
}

impl core::ops::DerefMut for F32x4 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.v) }
    }
}


impl F32x4 {
    /// behavior for values outside the `i32` range is platform dependent
    /// and considered a bug (there is no guarantee that the program won't crash).
    /// technically, this function should be unsafe, but that would make it rather
    /// annoying to use.
    #[inline(always)]
    pub fn to_i32_unck(self) -> I32x4 { unsafe {
        let r = vcvtmq_s32_f32(self.v);
        I32x4 { v: r }
    }}
}

impl F32x4 {
    #[inline(always)]
    pub const fn to_bits(self) -> U32x4 { unsafe { transmute(self) } }

    #[inline(always)]
    pub const fn from_bits(v: U32x4) -> Self { unsafe { transmute(v) } }

    #[inline(always)]
    pub fn floor(self) -> Self { unsafe {
        let r = vrndmq_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn ceil(self) -> Self { unsafe {
        let r = vrndpq_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn round(self) -> Self { unsafe {
        let r = vrndnq_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn trunc(self) -> Self { unsafe {
        let r = vrndq_f32(self.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self.lerpv(other, t.into())
    }

    #[inline(always)]
    pub fn lerpv(self, other: Self, ts: Self) -> Self {
        (Self::ONE - ts)*self + ts*other
    }
}


impl core::ops::Add for F32x4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { unsafe {
        let r = vaddq_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Sub for F32x4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { unsafe {
        let r = vsubq_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Neg for F32x4 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
            unsafe {
        let r = vnegq_f32(self.v);
        Self { v: r }
    }
    }
}

impl F32x4 {
    #[inline(always)]
    pub fn min(self, other: Self) -> Self { unsafe {
        let r = vminq_f32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn max(self, other: Self) -> Self { unsafe {
        let r = vmaxq_f32(self.v, other.v);
        Self { v: r }
    }}

    #[inline(always)]
    pub fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    pub fn at_most(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    pub fn clamp(self, low: Self, high: Self) -> Self {
        self.at_least(low).at_most(high)
    }


    #[inline(always)]
    pub fn hmin(self) -> f32 { unsafe {
        vminvq_f32(self.v)
    }}

    #[inline(always)]
    pub fn hmax(self) -> f32 { unsafe {
        vmaxvq_f32(self.v)
    }}
}


impl core::ops::Mul for F32x4 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output { unsafe {
        let r = vmulq_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Mul<f32> for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        self * F32x4::splat(rhs)
    }
}

impl core::ops::Mul<F32x4> for f32 {
    type Output = F32x4;

    #[inline(always)]
    fn mul(self, rhs: F32x4) -> Self::Output {
        F32x4::splat(self) * rhs
    }
}

impl core::ops::Div for F32x4 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output { unsafe {
        let r = vdivq_f32(self.v, rhs.v);
        Self { v: r }
    }}
}

impl core::ops::Div<f32> for F32x4 {
    type Output = F32x4;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self / F32x4::splat(rhs)
    }
}

impl F32x4 {
    #[inline(always)]
    pub fn eq(self, other: Self) -> B32x4 { unsafe {
        let r = vceqq_f32(self.v, other.v);
        B32x4 { v: r }
    }}

    
    #[inline(always)]
    pub fn ne(self, other: Self) -> B32x4 {
        !self.eq(other)
    }


    #[inline(always)]
    pub fn le(self, other: Self) -> B32x4 { unsafe {
        let r = vcleq_f32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn lt(self, other: Self) -> B32x4 { unsafe {
        let r = vcltq_f32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn ge(self, other: Self) -> B32x4 { unsafe {
        let r = vcgeq_f32(self.v, other.v);
        B32x4 { v: r }
    }}

    #[inline(always)]
    pub fn gt(self, other: Self) -> B32x4 { unsafe {
        let r = vcgtq_f32(self.v, other.v);
        B32x4 { v: r }
    }}
}

impl PartialEq for F32x4 {
    fn eq(&self, other: &Self) -> bool {
        F32x4::eq(*self, *other).all()
    }
}



