
#[inline]
pub const fn ceil_to_multiple_pow2(x: usize, n: usize) -> usize {
    debug_assert!(n.is_power_of_two());
    (x + (n-1)) & !(n-1)
}


