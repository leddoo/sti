use core::mem::transmute;

use super::*;


#[inline(always)]
fn load_i32x2(repr: f64) -> [i32; 2] { unsafe { transmute(repr) } }

#[inline(always)]
fn load_u32x2(repr: f64) -> [u32; 2] { unsafe { transmute(repr) } }

#[inline(always)]
fn load_f32x2(repr: f64) -> [f32; 2] { unsafe { transmute(repr) } }

#[inline(always)]
fn store2<T: SimdElement>(v: [T; 2]) -> f64 { unsafe { transmute(T::se_to_u32x2(v)) } }


macro_rules! meth2 {
    ($load:ident, $v:ident $($meth:tt)*) => {{
        let v = $load($v);
        store2(
            [v[0] $($meth)*,
             v[1] $($meth)*])
    }}
}

macro_rules! unop2 {
    ($load:ident, $op:tt $v:ident) => {{
        let v = $load($v);
        store2(
            [$op v[0],
             $op v[1]])
    }}
}

macro_rules! binop2 {
    ($load:ident, $lhs:ident $op:tt $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store2(
            [lhs[0] $op rhs[0],
             lhs[1] $op rhs[1]])
    }}
}

macro_rules! binop2_c {
    ($load:ident, $lhs:ident $op:tt $rhs:expr) => {{
        let lhs = $load($lhs);
        store2(
            [lhs[0] $op $rhs,
             lhs[1] $op $rhs])
    }}
}

macro_rules! binop2_b {
    ($load:ident, $lhs:ident $op:tt $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store2(
            [B32::new(lhs[0] $op rhs[0]),
             B32::new(lhs[1] $op rhs[1])])
    }}
}

macro_rules! binop2_m {
    ($load:ident, $lhs:ident $op:ident $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store2(
            [lhs[0].$op(rhs[0]),
             lhs[1].$op(rhs[1])])
    }}
}


impl SimdLanes<2> for () {
    type Repr = f64;


    #[inline(always)]
    fn repr_from_se<T: SimdElement>(v: [T; 2]) -> Self::Repr {
        store2(v)
    }

    #[inline(always)]
    fn repr_zip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr) {
        let lhs = load_f32x2(lhs);
        let rhs = load_f32x2(rhs);
        (store2([lhs[0], rhs[0]]),
         store2([lhs[1], rhs[1]]))
    }

    #[inline(always)]
    fn repr_unzip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr) {
        let lhs = load_f32x2(lhs);
        let rhs = load_f32x2(rhs);
        (store2([lhs[0], rhs[0]]),
         store2([lhs[1], rhs[1]]))
    }


    #[inline(always)]
    fn b32_splat(v: B32) -> Self::Repr {
        store2([v, v])
    }

    #[inline(always)]
    fn b32_select(mask: Self::Repr, on_true: Self::Repr, on_false: Self::Repr) -> Self::Repr {
        let m = load_u32x2(mask);
        let t = load_u32x2(on_true);
        let f = load_u32x2(on_false);
        store2(
            [(m[0] & t[0]) | (!m[0] & f[0]),
             (m[1] & t[1]) | (!m[1] & f[1])])
    }

    #[inline(always)]
    fn b32_none(v: Self::Repr) -> bool {
        let v = load_u32x2(v);
        (v[0] | v[1]) == 0
    }

    #[inline(always)]
    fn b32_any(v: Self::Repr) -> bool {
        let v = load_u32x2(v);
        (v[0] | v[1]) != 0
    }

    #[inline(always)]
    fn b32_all(v: Self::Repr) -> bool {
        let v = load_u32x2(v);
        (v[0] & v[1]) != 0
    }

    #[inline(always)]
    fn b32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_u32x2, lhs & rhs)
    }

    #[inline(always)]
    fn b32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_u32x2, lhs | rhs)
    }

    #[inline(always)]
    fn b32_not(v: Self::Repr) -> Self::Repr {
        unop2!(load_u32x2, !v)
    }


    #[inline(always)]
    fn i32_splat(v: i32) -> Self::Repr {
        store2([v, v])
    }

    #[inline(always)]
    fn i32_to_f32(v: Self::Repr) -> Self::Repr {
        meth2!(load_i32x2, v as f32)
    }

    #[inline(always)]
    fn i32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_i32x2, lhs min rhs)
    }

    #[inline(always)]
    fn i32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_i32x2, lhs max rhs)
    }

    #[inline(always)]
    fn i32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs == rhs)
    }

    #[inline(always)]
    fn i32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs != rhs)
    }

    #[inline(always)]
    fn i32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs <= rhs)
    }

    #[inline(always)]
    fn i32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs < rhs)
    }

    #[inline(always)]
    fn i32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs >= rhs)
    }

    #[inline(always)]
    fn i32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_i32x2, lhs > rhs)
    }

    #[inline(always)]
    fn i32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_i32x2, lhs + rhs)
    }

    #[inline(always)]
    fn i32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_i32x2, lhs - rhs)
    }

    #[inline(always)]
    fn i32_neg(v: Self::Repr) -> Self::Repr {
        unop2!(load_i32x2, -v)
    }

    #[inline(always)]
    fn i32_shl(v: Self::Repr, shift: i32) -> Self::Repr {
        binop2_c!(load_i32x2, v << shift)
    }

    #[inline(always)]
    fn i32_shr(v: Self::Repr, shift: i32) -> Self::Repr {
        binop2_c!(load_i32x2, v >> shift)
    }


    #[inline(always)]
    fn u32_splat(v: u32) -> Self::Repr {
        store2([v, v])
    }

    #[inline(always)]
    fn u32_as_i32(v: Self::Repr) -> Self::Repr {
        meth2!(load_u32x2, v as i32)
    }

    #[inline(always)]
    fn u32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_u32x2, lhs min rhs)
    }

    #[inline(always)]
    fn u32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_u32x2, lhs max rhs)
    }

    #[inline(always)]
    fn u32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_u32x2, lhs <= rhs)
    }

    #[inline(always)]
    fn u32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_u32x2, lhs < rhs)
    }

    #[inline(always)]
    fn u32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_u32x2, lhs >= rhs)
    }

    #[inline(always)]
    fn u32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_u32x2, lhs > rhs)
    }

    #[inline(always)]
    fn u32_shr(v: Self::Repr, shift: u32) -> Self::Repr {
        binop2_c!(load_u32x2, v >> shift)
    }

    #[inline(always)]
    fn u32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_u32x2, lhs & rhs)
    }

    #[inline(always)]
    fn u32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_u32x2, lhs | rhs)
    }

    #[inline(always)]
    fn u32_not(v: Self::Repr) -> Self::Repr {
        unop2!(load_u32x2, !v)
    }


    #[inline(always)]
    fn f32_splat(v: f32) -> Self::Repr {
        store2([v, v])
    }

    #[inline(always)]
    fn f32_to_i32_unck(v: Self::Repr) -> Self::Repr {
        unsafe { meth2!(load_f32x2, v.to_int_unchecked::<i32>()) }
    }

    #[inline(always)]
    fn f32_to_i32(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v as i32)
    }

    #[inline(always)]
    fn f32_floor(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.floor())
    }

    #[inline(always)]
    fn f32_ceil(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.ceil())
    }

    #[inline(always)]
    fn f32_round(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.round())
    }

    #[inline(always)]
    fn f32_trunc(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.trunc())
    }

    #[inline(always)]
    fn f32_abs(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.abs())
    }

    #[inline(always)]
    fn f32_sqrt(v: Self::Repr) -> Self::Repr {
        meth2!(load_f32x2, v.sqrt())
    }

    #[inline(always)]
    fn f32_with_sign_of(v: Self::Repr, sign: Self::Repr) -> Self::Repr {
        binop2_m!(load_f32x2, v copysign sign)
    }

    #[inline(always)]
    fn f32_hadd(v: Self::Repr) -> f32 {
        let v = load_f32x2(v);
        v[0] + v[1]
    }

    #[inline(always)]
    fn f32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_f32x2, lhs min rhs)
    }

    #[inline(always)]
    fn f32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_m!(load_f32x2, lhs max rhs)
    }

    #[inline(always)]
    fn f32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs == rhs)
    }

    #[inline(always)]
    fn f32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs != rhs)
    }

    #[inline(always)]
    fn f32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs <= rhs)
    }

    #[inline(always)]
    fn f32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs < rhs)
    }

    #[inline(always)]
    fn f32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs >= rhs)
    }

    #[inline(always)]
    fn f32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2_b!(load_f32x2, lhs > rhs)
    }

    #[inline(always)]
    fn f32_neg(v: Self::Repr) -> Self::Repr {
        unop2!(load_f32x2, -v)
    }

    #[inline(always)]
    fn f32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_f32x2, lhs + rhs)
    }

    #[inline(always)]
    fn f32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_f32x2, lhs - rhs)
    }

    #[inline(always)]
    fn f32_mul(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_f32x2, lhs * rhs)
    }

    #[inline(always)]
    fn f32_div(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop2!(load_f32x2, lhs / rhs)
    }
}



#[inline(always)]
fn load_i32x4(repr: Vec4) -> [i32; 4] { unsafe { transmute(repr) } }

#[inline(always)]
fn load_u32x4(repr: Vec4) -> [u32; 4] { unsafe { transmute(repr) } }

#[inline(always)]
fn load_f32x4(repr: Vec4) -> [f32; 4] { unsafe { transmute(repr) } }

#[inline(always)]
fn store4<T: SimdElement>(v: [T; 4]) -> Vec4 { unsafe { transmute(T::se_to_u32x4(v)) } }


macro_rules! meth4 {
    ($load:ident, $v:ident $($meth:tt)*) => {{
        let v = $load($v);
        store4(
            [v[0] $($meth)*,
             v[1] $($meth)*,
             v[2] $($meth)*,
             v[3] $($meth)*])
    }}
}

macro_rules! unop4 {
    ($load:ident, $op:tt $v:ident) => {{
        let v = $load($v);
        store4(
            [$op v[0],
             $op v[1],
             $op v[2],
             $op v[3]])
    }}
}

macro_rules! binop4 {
    ($load:ident, $lhs:ident $op:tt $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store4(
            [lhs[0] $op rhs[0],
             lhs[1] $op rhs[1],
             lhs[2] $op rhs[2],
             lhs[3] $op rhs[3]])
    }}
}

macro_rules! binop4_c {
    ($load:ident, $lhs:ident $op:tt $rhs:expr) => {{
        let lhs = $load($lhs);
        store4(
            [lhs[0] $op $rhs,
             lhs[1] $op $rhs,
             lhs[2] $op $rhs,
             lhs[3] $op $rhs])
    }}
}

macro_rules! binop4_b {
    ($load:ident, $lhs:ident $op:tt $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store4(
            [B32::new(lhs[0] $op rhs[0]),
             B32::new(lhs[1] $op rhs[1]),
             B32::new(lhs[2] $op rhs[2]),
             B32::new(lhs[3] $op rhs[3])])
    }}
}

macro_rules! binop4_m {
    ($load:ident, $lhs:ident $op:ident $rhs:ident) => {{
        let lhs = $load($lhs);
        let rhs = $load($rhs);
        store4(
            [lhs[0].$op(rhs[0]),
             lhs[1].$op(rhs[1]),
             lhs[2].$op(rhs[2]),
             lhs[3].$op(rhs[3])])
    }}
}



#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec4 { _inner: u128 }

impl SimdLanes<4> for () {
    type Repr = Vec4;


    #[inline(always)]
    fn repr_from_se<T: SimdElement>(v: [T; 4]) -> Self::Repr {
        store4(v)
    }

    #[inline(always)]
    fn repr_zip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr) {
        let lhs = load_f32x4(lhs);
        let rhs = load_f32x4(rhs);
        (store4([lhs[0], rhs[0], lhs[1], rhs[1]]),
         store4([lhs[2], rhs[2], lhs[3], rhs[3]]))
    }

    #[inline(always)]
    fn repr_unzip(lhs: Self::Repr, rhs: Self::Repr) -> (Self::Repr, Self::Repr) {
        let lhs = load_f32x4(lhs);
        let rhs = load_f32x4(rhs);
        (store4([lhs[0], lhs[2], rhs[0], rhs[2]]),
         store4([lhs[1], lhs[3], rhs[1], rhs[3]]))
    }


    #[inline(always)]
    fn b32_splat(v: B32) -> Self::Repr {
        store4([v, v, v, v])
    }

    #[inline(always)]
    fn b32_select(mask: Self::Repr, on_true: Self::Repr, on_false: Self::Repr) -> Self::Repr {
        let m = load_u32x4(mask);
        let t = load_u32x4(on_true);
        let f = load_u32x4(on_false);
        store4(
            [(m[0] & t[0]) | (!m[0] & f[0]),
             (m[1] & t[1]) | (!m[1] & f[1]),
             (m[2] & t[2]) | (!m[2] & f[2]),
             (m[3] & t[3]) | (!m[3] & f[3])])
    }

    #[inline(always)]
    fn b32_none(v: Self::Repr) -> bool {
        let v = load_u32x4(v);
        (v[0] | v[1] | v[2] | v[3]) == 0
    }

    #[inline(always)]
    fn b32_any(v: Self::Repr) -> bool {
        let v = load_u32x4(v);
        (v[0] | v[1] | v[2] | v[3]) != 0
    }

    #[inline(always)]
    fn b32_all(v: Self::Repr) -> bool {
        let v = load_u32x4(v);
        (v[0] & v[1] & v[2] & v[3]) != 0
    }

    #[inline(always)]
    fn b32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_u32x4, lhs & rhs)
    }

    #[inline(always)]
    fn b32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_u32x4, lhs | rhs)
    }

    #[inline(always)]
    fn b32_not(v: Self::Repr) -> Self::Repr {
        unop4!(load_u32x4, !v)
    }


    #[inline(always)]
    fn i32_splat(v: i32) -> Self::Repr {
        store4([v, v, v, v])
    }

    #[inline(always)]
    fn i32_to_f32(v: Self::Repr) -> Self::Repr {
        meth4!(load_i32x4, v as f32)
    }

    #[inline(always)]
    fn i32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_i32x4, lhs min rhs)
    }

    #[inline(always)]
    fn i32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_i32x4, lhs max rhs)
    }

    #[inline(always)]
    fn i32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs == rhs)
    }

    #[inline(always)]
    fn i32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs != rhs)
    }

    #[inline(always)]
    fn i32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs <= rhs)
    }

    #[inline(always)]
    fn i32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs < rhs)
    }

    #[inline(always)]
    fn i32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs >= rhs)
    }

    #[inline(always)]
    fn i32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_i32x4, lhs > rhs)
    }

    #[inline(always)]
    fn i32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_i32x4, lhs + rhs)
    }

    #[inline(always)]
    fn i32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_i32x4, lhs - rhs)
    }

    #[inline(always)]
    fn i32_neg(v: Self::Repr) -> Self::Repr {
        unop4!(load_i32x4, -v)
    }

    #[inline(always)]
    fn i32_shl(v: Self::Repr, shift: i32) -> Self::Repr {
        binop4_c!(load_i32x4, v << shift)
    }

    #[inline(always)]
    fn i32_shr(v: Self::Repr, shift: i32) -> Self::Repr {
        binop4_c!(load_i32x4, v >> shift)
    }


    #[inline(always)]
    fn u32_splat(v: u32) -> Self::Repr {
        store4([v, v, v, v])
    }

    #[inline(always)]
    fn u32_as_i32(v: Self::Repr) -> Self::Repr {
        meth4!(load_u32x4, v as i32)
    }

    #[inline(always)]
    fn u32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_u32x4, lhs min rhs)
    }

    #[inline(always)]
    fn u32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_u32x4, lhs max rhs)
    }

    #[inline(always)]
    fn u32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_u32x4, lhs <= rhs)
    }

    #[inline(always)]
    fn u32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_u32x4, lhs < rhs)
    }

    #[inline(always)]
    fn u32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_u32x4, lhs >= rhs)
    }

    #[inline(always)]
    fn u32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_u32x4, lhs > rhs)
    }

    #[inline(always)]
    fn u32_shr(v: Self::Repr, shift: u32) -> Self::Repr {
        binop4_c!(load_u32x4, v >> shift)
    }

    #[inline(always)]
    fn u32_and(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_u32x4, lhs & rhs)
    }

    #[inline(always)]
    fn u32_or(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_u32x4, lhs | rhs)
    }

    #[inline(always)]
    fn u32_not(v: Self::Repr) -> Self::Repr {
        unop4!(load_u32x4, !v)
    }


    #[inline(always)]
    fn f32_splat(v: f32) -> Self::Repr {
        store4([v, v, v, v])
    }

    #[inline(always)]
    fn f32_to_i32_unck(v: Self::Repr) -> Self::Repr {
        unsafe { meth4!(load_f32x4, v.to_int_unchecked::<i32>()) }
    }

    #[inline(always)]
    fn f32_to_i32(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v as i32)
    }

    #[inline(always)]
    fn f32_floor(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.floor())
    }

    #[inline(always)]
    fn f32_ceil(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.ceil())
    }

    #[inline(always)]
    fn f32_round(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.round())
    }

    #[inline(always)]
    fn f32_trunc(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.trunc())
    }

    #[inline(always)]
    fn f32_abs(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.abs())
    }

    #[inline(always)]
    fn f32_sqrt(v: Self::Repr) -> Self::Repr {
        meth4!(load_f32x4, v.sqrt())
    }

    #[inline(always)]
    fn f32_with_sign_of(v: Self::Repr, sign: Self::Repr) -> Self::Repr {
        binop4_m!(load_f32x4, v copysign sign)
    }

    #[inline(always)]
    fn f32_hadd(v: Self::Repr) -> f32 {
        let v = load_f32x4(v);
        v[0] + v[1] + v[2] + v[3]
    }

    #[inline(always)]
    fn f32_min(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_f32x4, lhs min rhs)
    }

    #[inline(always)]
    fn f32_max(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_m!(load_f32x4, lhs max rhs)
    }

    #[inline(always)]
    fn f32_eq(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs == rhs)
    }

    #[inline(always)]
    fn f32_ne(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs != rhs)
    }

    #[inline(always)]
    fn f32_le(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs <= rhs)
    }

    #[inline(always)]
    fn f32_lt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs < rhs)
    }

    #[inline(always)]
    fn f32_ge(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs >= rhs)
    }

    #[inline(always)]
    fn f32_gt(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4_b!(load_f32x4, lhs > rhs)
    }

    #[inline(always)]
    fn f32_neg(v: Self::Repr) -> Self::Repr {
        unop4!(load_f32x4, -v)
    }

    #[inline(always)]
    fn f32_add(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_f32x4, lhs + rhs)
    }

    #[inline(always)]
    fn f32_sub(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_f32x4, lhs - rhs)
    }

    #[inline(always)]
    fn f32_mul(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_f32x4, lhs * rhs)
    }

    #[inline(always)]
    fn f32_div(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        binop4!(load_f32x4, lhs / rhs)
    }
}


