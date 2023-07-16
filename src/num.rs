
#[inline(always)]
pub const fn is_pow2(n: usize) -> bool {
    n != 0 && (n & (n-1) == 0)
}

#[inline(always)]
pub fn ceil_to_multiple_pow2(x: usize, n: usize) -> usize {
    debug_assert!(is_pow2(n));
    (x + (n-1)) & !(n-1)
}


pub trait OrdUtils: Ord + Sized {
    #[inline(always)]
    fn at_least(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn at_most(self, other: Self) -> Self {
        self.min(other)
    }
}

impl<T: Ord + Sized> OrdUtils for T {}

