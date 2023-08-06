use super::*;


impl SimdLanes<2> for () {
    type Align = Align8;

    const ALIGN: Self::Align = Align8;


    fn b32_splat(v: super::b32x::B32) -> [super::b32x::B32; 2] {
        todo!()
    }

    fn b32_as_u32(v: [super::b32x::B32; 2]) -> [u32; 2] {
        todo!()
    }

    fn b32_as_i32(v: [super::b32x::B32; 2]) -> [i32; 2] {
        todo!()
    }

    fn b32_select_b32(mask: [super::b32x::B32; 2], on_true: [super::b32x::B32; 2], on_false: [super::b32x::B32; 2]) -> [super::b32x::B32; 2] {
        todo!()
    }

    fn b32_select_i32(mask: [super::b32x::B32; 2], on_true: [i32; 2], on_false: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn b32_select_u32(mask: [super::b32x::B32; 2], on_true: [u32; 2], on_false: [u32; 2]) -> [u32; 2] {
        [mask[0].0 & on_true[0] | !mask[0].0 & on_false[0],
         mask[1].0 & on_true[1] | !mask[1].0 & on_false[1]]
        /*
        */
        /*
        unsafe {
            core::mem::transmute(
                core::arch::aarch64::vbsl_u32(
                    core::mem::transmute(mask),
                    core::mem::transmute(on_true),
                    core::mem::transmute(on_false)))
        }
        */
    }

    fn b32_select_f32(mask: [super::b32x::B32; 2], on_true: [f32; 2], on_false: [f32; 2]) -> [f32; 2] {
        todo!()
    }

    fn b32_none(v: [super::b32x::B32; 2]) -> bool {
        todo!()
    }

    fn b32_any (v: [super::b32x::B32; 2]) -> bool {
        todo!()
    }

    fn b32_all (v: [super::b32x::B32; 2]) -> bool {
        todo!()
    }

    fn b32_zip(lhs: [super::b32x::B32; 2], rhs: [super::b32x::B32; 2]) -> ([super::b32x::B32; 2], [super::b32x::B32; 2]) {
        todo!()
    }

    fn b32_unzip(lhs: [super::b32x::B32; 2], rhs: [super::b32x::B32; 2]) -> ([super::b32x::B32; 2], [super::b32x::B32; 2]) {
        todo!()
    }

    fn b32_and(lhs: [super::b32x::B32; 2], rhs: [super::b32x::B32; 2]) -> [super::b32x::B32; 2] {
        todo!()
    }

    fn b32_or(lhs: [super::b32x::B32; 2], rhs: [super::b32x::B32; 2]) -> [super::b32x::B32; 2] {
        todo!()
    }

    fn b32_not(v: [super::b32x::B32; 2]) -> [super::b32x::B32; 2] {
        todo!()
    }

    fn i32_splat(v: i32) -> [i32; 2] {
        todo!()
    }

    fn i32_as_u32(v: [i32; 2]) -> [u32; 2] {
        todo!()
    }

    fn i32_to_f32(v: [i32; 2]) -> [f32; 2] {
        todo!()
    }

    fn i32_min(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_max(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_zip(lhs: [i32; 2], rhs: [i32; 2]) -> ([i32; 2], [i32; 2]) {
        todo!()
    }

    fn i32_unzip(lhs: [i32; 2], rhs: [i32; 2]) -> ([i32; 2], [i32; 2]) {
        todo!()
    }

    fn i32_eq(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_ne(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_le(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_lt(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_ge(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_gt(lhs: [i32; 2], rhs: [i32; 2]) -> [B32; 2] {
        todo!()
    }

    fn i32_add(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_sub(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_neg(v: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_shl(v: [i32; 2], shift: i32) -> [i32; 2] {
        todo!()
    }

    fn i32_shr(v: [i32; 2], shift: i32) -> [i32; 2] {
        todo!()
    }

    fn i32_and(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_or(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        todo!()
    }

    fn i32_not(v: [i32; 2]) -> [i32; 2] {
        todo!()
    }
}

impl SimdLanes<4> for () {
    type Align = Align16;

    const ALIGN: Self::Align = Align16;

    fn b32_splat(v: super::b32x::B32) -> [super::b32x::B32; 4] {
        unsafe {
            core::mem::transmute(core::arch::aarch64::vdupq_n_u32(v.0))
        }
    }

    fn b32_as_u32(v: [super::b32x::B32; 4]) -> [u32; 4] {
        todo!()
    }

    fn b32_as_i32(v: [super::b32x::B32; 4]) -> [i32; 4] {
        todo!()
    }

    fn b32_select_b32(mask: [super::b32x::B32; 4], on_true: [super::b32x::B32; 4], on_false: [super::b32x::B32; 4]) -> [super::b32x::B32; 4] {
        todo!()
    }

    fn b32_select_i32(mask: [super::b32x::B32; 4], on_true: [i32; 4], on_false: [i32; 4]) -> [i32; 4] {
        todo!()
    }

    fn b32_select_u32(mask: [super::b32x::B32; 4], on_true: [u32; 4], on_false: [u32; 4]) -> [u32; 4] {
        [mask[0].0 & on_true[0] | !mask[0].0 & on_false[0],
         mask[1].0 & on_true[1] | !mask[1].0 & on_false[1],
         mask[2].0 & on_true[2] | !mask[2].0 & on_false[2],
         mask[3].0 & on_true[3] | !mask[3].0 & on_false[3]]
    }

    fn b32_select_f32(mask: [super::b32x::B32; 4], on_true: [f32; 4], on_false: [f32; 4]) -> [f32; 4] {
        todo!()
    }

    fn b32_none(v: [super::b32x::B32; 4]) -> bool {
        todo!()
    }

    fn b32_any(v: [super::b32x::B32; 4]) -> bool {
        todo!()
    }

    fn b32_all(v: [super::b32x::B32; 4]) -> bool {
        todo!()
    }

    fn b32_zip(lhs: [super::b32x::B32; 4], rhs: [super::b32x::B32; 4]) -> ([super::b32x::B32; 4], [super::b32x::B32; 4]) {
        todo!()
    }

    fn b32_unzip(lhs: [super::b32x::B32; 4], rhs: [super::b32x::B32; 4]) -> ([super::b32x::B32; 4], [super::b32x::B32; 4]) {
        todo!()
    }

    fn b32_and(lhs: [super::b32x::B32; 4], rhs: [super::b32x::B32; 4]) -> [super::b32x::B32; 4] {
        todo!()
    }

    fn b32_or(lhs: [super::b32x::B32; 4], rhs: [super::b32x::B32; 4]) -> [super::b32x::B32; 4] {
        todo!()
    }

    fn b32_not(v: [super::b32x::B32; 4]) -> [super::b32x::B32; 4] {
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

    fn i32_zip(lhs: [i32; 4], rhs: [i32; 4]) -> ([i32; 4], [i32; 4]) {
        todo!()
    }

    fn i32_unzip(lhs: [i32; 4], rhs: [i32; 4]) -> ([i32; 4], [i32; 4]) {
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
}

/*
macro_rules! binop2 {
    ($lhs:ident $op:tt $rhs:ident) => {
        [$lhs[0] $op $rhs[0],
         $lhs[1] $op $rhs[1]]
    }
}

impl SimdLanes<2> for () {
    type Align = Align8;
    const ALIGN: Self::Align = Align8;


    #[inline(always)]
    fn b32_select_b32(mask: [B32; 2], on_true: [B32; 2], on_false: [B32; 2]) -> [B32; 2] {
        todo!()
    }

    #[inline(always)]
    fn b32_select_i32(mask: [B32; 2], on_true: [B32; 2], on_false: [B32; 2]) -> [B32; 2] {
        todo!()
    }

    #[inline(always)]
    fn b32_select_u32(mask: [B32; 2], on_true: [B32; 2], on_false: [B32; 2]) -> [B32; 2] {
        todo!()
    }

    #[inline(always)]
    fn b32_select_f32(mask: [B32; 2], on_true: [B32; 2], on_false: [B32; 2]) -> [B32; 2] {
        todo!()
    }


    #[inline(always)]
    fn i32_add(lhs: [i32; 2], rhs: [i32; 2]) -> [i32; 2] {
        binop2!(lhs + rhs)
    }
}
*/

