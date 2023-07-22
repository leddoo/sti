pub trait F32Ext {
    fn ffloor(self) -> Self;
    fn fceil(self)  -> Self;
    fn fround(self) -> Self;
    fn ftrunc(self) -> Self;

    fn fsqrt(self) -> Self;

    fn safe_div(self, other: Self, default: Self) -> Self;
}

mod simd;

