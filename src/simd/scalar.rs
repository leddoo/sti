use core::mem::transmute;

use super::*;


macro_rules! meth2 {
    ($v:ident $($meth:tt)*) => {
        [$v[0] $($meth)*,
         $v[1] $($meth)*]
    }
}

macro_rules! unop2 {
    ($op:tt $v:ident) => {
        [$op $v[0],
         $op $v[1]]
    }
}

macro_rules! binop2 {
    ($lhs:ident $op:tt $rhs:ident) => {
        [$lhs[0] $op $rhs[0],
         $lhs[1] $op $rhs[1]]
    }
}

macro_rules! binop2_c {
    ($lhs:ident $op:tt $rhs:expr) => {
        [$lhs[0] $op $rhs,
         $lhs[1] $op $rhs]
    }
}

macro_rules! binop2_b {
    ($lhs:ident $op:tt $rhs:ident) => {
        [($lhs[0] $op $rhs[0]).into(),
         ($lhs[1] $op $rhs[1]).into()]
    }
}

macro_rules! binop2_m {
    ($lhs:ident $op:ident $rhs:ident) => {
        [$lhs[0].$op($rhs[0]),
         $lhs[1].$op($rhs[1])]
    }
}

impl SimdLanes<2> for () {
    type Align = Align8;

    const ALIGN: Self::Align = Align8;

    fn b32_splat(v: B32) -> [B32; 2] {
        [v, v]
    }

    fn b32_as_u32(v: [B32; 2]) -> [u32; 2] {
        unsafe { transmute(v) }
    }

    fn b32_from_u32_unck(v: [u32; 2]) -> [B32; 2] {
        unsafe { transmute(v) }
    }

    fn b32_as_i32(v: [B32; 2]) -> [i32; 2] {
        meth2!(v.as_u32() as i32)
    }

    fn b32_select_u32(mask: [B32; 2], on_true: [u32; 2], on_false: [u32; 2]) -> [u32; 2] {
        [mask[0].as_u32()&on_true[0] | !mask[0].as_u32()&on_false[0],
         mask[1].as_u32()&on_true[1] | !mask[1].as_u32()&on_false[1]]
    }

    fn b32_none(v: [B32; 2]) -> bool {
        !v[0].to_bool() && !v[1].to_bool()
    }

    fn b32_any(v: [B32; 2]) -> bool {
        v[0].to_bool() || v[1].to_bool()
    }

    fn b32_all(v: [B32; 2]) -> bool {
        v[0].to_bool() && v[1].to_bool()
    }

    fn b32_zip(lhs: [B32; 2], rhs: [B32; 2]) -> ([B32; 2], [B32; 2]) {
        ([lhs[0], rhs[0]],
         [lhs[1], rhs[1]])
    }

    fn b32_unzip(lhs: [B32; 2], rhs: [B32; 2]) -> ([B32; 2], [B32; 2]) {
        ([lhs[0], rhs[0]],
         [lhs[1], rhs[1]])
    }

    fn b32_and(lhs: [B32; 2], rhs: [B32; 2]) -> [B32; 2] {
        binop2!(lhs & rhs)
    }

    fn b32_or(lhs: [B32; 2], rhs: [B32; 2]) -> [B32; 2] {
        binop2!(lhs | rhs)
    }

    fn b32_not(v: [B32; 2]) -> [B32; 2] {
        unop2!(!v)
    }


    fn i32_splat(v: i32) -> [i32; 2] {
        [v, v]
    }

    fn i32_as_u32(v: [i32; 2]) -> [u32; 2] {
        unsafe { transmute(v) }
    }

    fn i32_to_f32(v: [i32; 2]) -> [f32; 2] {
        meth2!(v as f32)
    }

    fn i32_min(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2_m!(lhs min rhs)
    }

    fn i32_max(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2_m!(lhs max rhs)
    }

    fn i32_eq(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs == rhs)
    }

    fn i32_ne(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs != rhs)
    }

    fn i32_le(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs <= rhs)
    }

    fn i32_lt(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs < rhs)
    }

    fn i32_ge(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs >= rhs)
    }

    fn i32_gt(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        binop2_b!(lhs > rhs)
    }

    fn i32_zip(lhs: [i32; 2], rhs: [i32; 2]) -> ([i32; 2], [i32; 2]) {
        todo!()
    }

    fn i32_unzip(lhs: [i32; 2], rhs: [i32; 2]) -> ([i32; 2], [i32; 2]) {
        todo!()
    }

    fn i32_add(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2!(lhs + rhs)
    }

    fn i32_sub(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2!(lhs - rhs)
    }

    fn i32_neg(v: [i32; 2]) -> [i32; 2] {
        unop2!(-v)
    }

    fn i32_shl(v: [i32; 2], shift: i32) -> [i32; 2] {
        binop2_c!(v << shift)
    }

    fn i32_shr(v: [i32; 2], shift: i32) -> [i32; 2] {
        binop2_c!(v >> shift)
    }

    fn i32_and(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2!(lhs & rhs)
    }

    fn i32_or(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2!(lhs | rhs)
    }

    fn i32_not(v: [i32; 2]) -> [i32; 2] {
        unop2!(!v)
    }


    fn u32_splat(v: u32) -> [u32; 2] {
        todo!()
    }

    fn u32_as_i32(v: [u32; 2]) -> [i32; 2] {
        todo!()
    }

    fn u32_min(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_max(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_eq(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_ne(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_le(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_lt(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_ge(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_gt(lhs: [u32; 2], rhs: [u32; 2]) -> [B32; 2] {
        todo!()
    }

    fn u32_zip(lhs: [u32; 2], rhs: [u32; 2]) -> ([u32; 2], [u32; 2]) {
        todo!()
    }

    fn u32_unzip(lhs: [u32; 2], rhs: [u32; 2]) -> ([u32; 2], [u32; 2]) {
        todo!()
    }

    fn u32_add(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_sub(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_shl(v: [u32; 2], shift: u32) -> [u32; 2] {
        todo!()
    }

    fn u32_shr(v: [u32; 2], shift: u32) -> [u32; 2] {
        todo!()
    }

    fn u32_and(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_or(lhs: [u32; 2], rhs: [u32; 2]) -> [u32; 2] {
        todo!()
    }

    fn u32_not(v: [u32; 2]) -> [u32; 2] {
        todo!()
    }


    fn f32_splat(v: f32) -> [f32; 2] {
        todo!()
    }

    fn f32_as_bits(v: [f32; 2]) -> [u32; 2] {
        todo!()
    }

    fn f32_from_bits(v: [u32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_to_i32_unck(v: [f32; 2]) -> [i32; 2] {
        todo!()
    }

    fn f32_to_i32(v: [f32; 2]) -> [i32; 2] {
        todo!()
    }

    fn f32_floor(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_ceil(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_round(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_trunc(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_abs(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_sqrt(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_with_sign_of(v: [f32; 2], sign: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_hadd(v: [f32; 2]) -> f32 {
        todo!()
    }

    fn f32_min(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_max(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_eq(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_ne(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_le(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_lt(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_ge(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_gt(lhs: [f32; 2], rhs: [f32; 2]) -> [B32; 2] {
        todo!()
    }

    fn f32_zip(lhs: [f32; 2], rhs: [f32; 2]) -> ([f32; 2], [f32; 2]) {
        todo!()
    }

    fn f32_unzip(lhs: [f32; 2], rhs: [f32; 2]) -> ([f32; 2], [f32; 2]) {
        todo!()
    }

    fn f32_neg(v: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_add(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_sub(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_mul(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn f32_div(lhs: [f32; 2], rhs: [f32; 2]) -> [f32; 2] {
        todo!()
    }
}

impl SimdLanes<4> for () {
    type Align = Align16;

    const ALIGN: Self::Align = Align16;

    fn b32_splat(v: B32) -> [B32; 4] {
        todo!()
    }

    fn b32_as_u32(v: [B32; 4]) -> [u32; 4] {
        todo!()
    }

    fn b32_from_u32_unck(v: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn b32_as_i32(v: [B32; 4]) -> [i32; 4] {
        todo!()
    }

    fn b32_select_u32(mask: [B32; 4], on_true: [u32; 4], on_false: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn b32_none(v: [B32; 4]) -> bool {
        todo!()
    }

    fn b32_any(v: [B32; 4]) -> bool {
        todo!()
    }

    fn b32_all(v: [B32; 4]) -> bool {
        todo!()
    }

    fn b32_zip(lhs: [B32; 4], rhs: [B32; 4]) -> ([B32; 4], [B32; 4]) {
        ([lhs[0], rhs[0], lhs[1], rhs[1]],
         [lhs[2], rhs[2], lhs[3], rhs[3]])
    }

    fn b32_unzip(lhs: [B32; 4], rhs: [B32; 4]) -> ([B32; 4], [B32; 4]) {
        ([lhs[0], lhs[2], rhs[0], rhs[2]],
         [lhs[1], lhs[3], rhs[1], rhs[3]])
    }

    fn b32_and(lhs: [B32; 4], rhs: [B32; 4]) -> [B32; 4] {
        todo!()
    }

    fn b32_or(lhs: [B32; 4], rhs: [B32; 4]) -> [B32; 4] {
        todo!()
    }

    fn b32_not(v: [B32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_splat(v: i32) -> [i32; 4] {
        todo!()
    }

    fn i32_as_u32(v: [i32; 4]) -> [u32; 4] {
        todo!()
    }

    fn i32_to_f32(v: [i32; 4]) -> [f32; 4] {
        todo!()
    }

    fn i32_min(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_max(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_eq(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_ne(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_le(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_lt(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_ge(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_gt(lhs: [i32; 4], rhs: [i32; 4]) -> [B32; 4] {
        todo!()
    }

    fn i32_zip(lhs: [i32; 4], rhs: [i32; 4]) -> ([i32; 4], [i32; 4]) {
        todo!()
    }

    fn i32_unzip(lhs: [i32; 4], rhs: [i32; 4]) -> ([i32; 4], [i32; 4]) {
        todo!()
    }

    fn i32_add(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_sub(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_neg(v: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_shl(v: [i32; 4], shift: i32) -> [i32; 4] {
        todo!()
    }

    fn i32_shr(v: [i32; 4], shift: i32) -> [i32; 4] {
        todo!()
    }

    fn i32_and(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_or(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn i32_not(v: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn u32_splat(v: u32) -> [u32; 4] {
        todo!()
    }

    fn u32_as_i32(v: [u32; 4]) -> [i32; 4] {
        todo!()
    }

    fn u32_min(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_max(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_eq(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_ne(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_le(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_lt(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_ge(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_gt(lhs: [u32; 4], rhs: [u32; 4]) -> [B32; 4] {
        todo!()
    }

    fn u32_zip(lhs: [u32; 4], rhs: [u32; 4]) -> ([u32; 4], [u32; 4]) {
        todo!()
    }

    fn u32_unzip(lhs: [u32; 4], rhs: [u32; 4]) -> ([u32; 4], [u32; 4]) {
        todo!()
    }

    fn u32_add(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_sub(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_shl(v: [u32; 4], shift: u32) -> [u32; 4] {
        todo!()
    }

    fn u32_shr(v: [u32; 4], shift: u32) -> [u32; 4] {
        todo!()
    }

    fn u32_and(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_or(lhs: [u32; 4], rhs: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn u32_not(v: [u32; 4]) -> [u32; 4] {
        todo!()
    }

    fn f32_splat(v: f32) -> [f32; 4] {
        todo!()
    }

    fn f32_as_bits(v: [f32; 4]) -> [u32; 4] {
        todo!()
    }

    fn f32_from_bits(v: [u32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_to_i32_unck(v: [f32; 4]) -> [i32; 4] {
        todo!()
    }

    fn f32_to_i32(v: [f32; 4]) -> [i32; 4] {
        todo!()
    }

    fn f32_floor(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_ceil(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_round(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_trunc(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_abs(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_sqrt(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_with_sign_of(v: [f32; 4], sign: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_hadd(v: [f32; 4]) -> f32 {
        todo!()
    }

    fn f32_min(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_max(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_eq(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_ne(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_le(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_lt(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_ge(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_gt(lhs: [f32; 4], rhs: [f32; 4]) -> [B32; 4] {
        todo!()
    }

    fn f32_zip(lhs: [f32; 4], rhs: [f32; 4]) -> ([f32; 4], [f32; 4]) {
        todo!()
    }

    fn f32_unzip(lhs: [f32; 4], rhs: [f32; 4]) -> ([f32; 4], [f32; 4]) {
        todo!()
    }

    fn f32_neg(v: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_add(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_sub(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_mul(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn f32_div(lhs: [f32; 4], rhs: [f32; 4]) -> [f32; 4] {
        todo!()
    }
}

