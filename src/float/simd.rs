use crate::simd::F32x4;

use super::F32Ext;

impl F32Ext for f32 {
    #[inline(always)]
    fn ffloor(self) -> Self {
        F32x4::splat(self).floor()[0]
    }

    #[inline(always)]
    fn fceil(self) -> Self {
        F32x4::splat(self).ceil()[0]
    }

    #[inline(always)]
    fn fround(self) -> Self {
        F32x4::splat(self).round()[0]
    }

    #[inline(always)]
    fn ftrunc(self) -> Self {
        F32x4::splat(self).trunc()[0]
    }

    #[inline(always)]
    fn fsqrt(self) -> Self {
        F32x4::splat(self).sqrt()[0]
    }

    #[inline(always)]
    fn safe_div(self, other: Self, default: Self) -> Self {
        let result = self/other; // so compiler can use select.
        if other != 0.0 { result } else { default }
    }
}

