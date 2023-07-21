#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn b32x2() {
        let b = B32x2::new;
        let f = F32x2::new;

        assert_eq!(B32x2::splat(false).to_array(), [false, false]);
        assert_eq!(B32x2::splat(true).to_array(),  [true,  true]);

        assert_eq!(b(false, false).to_array(), [false, false]);
        assert_eq!(b(false, true).to_array(),  [false, true]);
        assert_eq!(b(true,  false).to_array(), [true,  false]);
        assert_eq!(b(true,  true).to_array(),  [true,  true]);

        assert_eq!((!b(false, false)).to_array(), [true,  true]);
        assert_eq!((!b(false, true)).to_array(),  [true,  false]);
        assert_eq!((!b(true,  false)).to_array(), [false, true]);
        assert_eq!((!b(true,  true)).to_array(),  [false, false]);

        assert_eq!((b(false, true) & b(false, true)).to_array(), [false, true]);
        assert_eq!((b(false, true) & b(true, false)).to_array(), [false, false]);
        assert_eq!((b(true, false) & b(false, true)).to_array(), [false, false]);
        assert_eq!((b(true, false) & b(true, false)).to_array(), [true,  false]);

        assert_eq!((b(false, false) | b(false, false)).to_array(), [false, false]);
        assert_eq!((b(false, false) | b(false, true)).to_array(),  [false, true]);
        assert_eq!((b(false, false) | b(true,  false)).to_array(), [true, false]);
        assert_eq!((b(false, true)  | b(false, false)).to_array(), [false, true]);
        assert_eq!((b(true,  false) | b(false, false)).to_array(), [true, false]);

        assert_eq!(b(false, false).select_b32(b(false, true), b(true, false)).to_array(), [false, true]);
        assert_eq!(b(false, true ).select_b32(b(false, true), b(true, false)).to_array(), [false, false]);
        assert_eq!(b(true,  false).select_b32(b(false, true), b(true, false)).to_array(), [true,  true]);
        assert_eq!(b(true,  true ).select_b32(b(false, true), b(true, false)).to_array(), [true,  false]);

        assert_eq!(b(false, false).select_f32(f(1.0, 2.0), f(3.0, 4.0)).to_array(), [1.0, 2.0]);
        assert_eq!(b(false, true ).select_f32(f(1.0, 2.0), f(3.0, 4.0)).to_array(), [1.0, 4.0]);
        assert_eq!(b(true,  false).select_f32(f(1.0, 2.0), f(3.0, 4.0)).to_array(), [3.0, 2.0]);
        assert_eq!(b(true,  true ).select_f32(f(1.0, 2.0), f(3.0, 4.0)).to_array(), [3.0, 4.0]);
    }

    #[test]
    fn f32x2() {
        let f = F32x2::new;

        assert_eq!(F32x2::splat(0.0).to_array(), [0.0, 0.0]);
        assert_eq!(F32x2::splat(1.0).to_array(), [1.0, 1.0]);

        assert_eq!(f(1.0, 2.0).to_array(), [1.0, 2.0]);

        assert_eq!((f(1.0, 2.0) + f(3.0, 4.0)).to_array(), [4.0, 6.0]);
        assert_eq!((f(1.0, 2.0) - f(3.0, 5.0)).to_array(), [-2.0, -3.0]);
        assert_eq!((f(5.0, 2.0) * f(3.0, 4.0)).to_array(), [15.0, 8.0]);
        assert_eq!((f(6.0, 2.0) / f(3.0, 4.0)).to_array(), [2.0, 0.5]);

        assert_eq!((f(1.0, 2.0).eq(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(1.0, 2.0).eq(f(3.0, 2.0))).to_array(), [false, true]);
        assert_eq!((f(1.0, 2.0).eq(f(1.0, 4.0))).to_array(), [true,  false]);
        assert_eq!((f(1.0, 2.0).eq(f(1.0, 2.0))).to_array(), [true,  true]);

        assert_eq!((f(1.0, 2.0).ne(f(1.0, 2.0))).to_array(), [false, false]);
        assert_eq!((f(1.0, 2.0).ne(f(1.0, 4.0))).to_array(), [false, true]);
        assert_eq!((f(1.0, 2.0).ne(f(3.0, 2.0))).to_array(), [true,  false]);
        assert_eq!((f(1.0, 2.0).ne(f(3.0, 4.0))).to_array(), [true,  true]);

        assert_eq!((f(3.0, 4.0).le(f(1.0, 2.0))).to_array(), [false, false]);
        assert_eq!((f(3.0, 4.0).le(f(1.0, 4.0))).to_array(), [false, true]);
        assert_eq!((f(3.0, 4.0).le(f(1.0, 5.0))).to_array(), [false, true]);
        assert_eq!((f(3.0, 4.0).le(f(3.0, 2.0))).to_array(), [true,  false]);
        assert_eq!((f(3.0, 4.0).le(f(4.0, 2.0))).to_array(), [true,  false]);
        assert_eq!((f(3.0, 4.0).le(f(3.0, 4.0))).to_array(), [true,  true]);
        assert_eq!((f(3.0, 4.0).le(f(4.0, 5.0))).to_array(), [true,  true]);

        assert_eq!((f(3.0, 4.0).lt(f(1.0, 2.0))).to_array(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(1.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(1.0, 5.0))).to_array(), [false, true]);
        assert_eq!((f(3.0, 4.0).lt(f(3.0, 2.0))).to_array(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(4.0, 2.0))).to_array(), [true,  false]);
        assert_eq!((f(3.0, 4.0).lt(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(4.0, 5.0))).to_array(), [true,  true]);

        assert_eq!((f(1.0, 2.0).ge(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(1.0, 4.0).ge(f(3.0, 4.0))).to_array(), [false, true]);
        assert_eq!((f(1.0, 5.0).ge(f(3.0, 4.0))).to_array(), [false, true]);
        assert_eq!((f(3.0, 2.0).ge(f(3.0, 4.0))).to_array(), [true,  false]);
        assert_eq!((f(4.0, 2.0).ge(f(3.0, 4.0))).to_array(), [true,  false]);
        assert_eq!((f(3.0, 4.0).ge(f(3.0, 4.0))).to_array(), [true,  true]);
        assert_eq!((f(4.0, 5.0).ge(f(3.0, 4.0))).to_array(), [true,  true]);

        assert_eq!((f(1.0, 2.0).gt(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(1.0, 4.0).gt(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(1.0, 5.0).gt(f(3.0, 4.0))).to_array(), [false, true]);
        assert_eq!((f(3.0, 2.0).gt(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(4.0, 2.0).gt(f(3.0, 4.0))).to_array(), [true,  false]);
        assert_eq!((f(3.0, 4.0).gt(f(3.0, 4.0))).to_array(), [false, false]);
        assert_eq!((f(4.0, 5.0).gt(f(3.0, 4.0))).to_array(), [true,  true]);

        assert_eq!(f(4.5, -2.5).to_i32_unck().to_array(), [4, -3]);

        assert_eq!((f(4.0, 5.0).min(f(3.0, 6.0))).to_array(), [3.0, 5.0]);
        assert_eq!((f(2.0, 5.0).min(f(3.0, 4.0))).to_array(), [2.0, 4.0]);

        assert_eq!(f(1.0, 2.0).x(), 1.0);
        assert_eq!(f(1.0, 2.0).y(), 2.0);
    }

    #[test]
    fn i32x2() {
        let i = I32x2::new;

        assert_eq!(i(4, -2).to_f32().to_array(), [4.0, -2.0]);

        assert_eq!((i(4, 5).min(i(3, 6))).to_array(), [3, 5]);
        assert_eq!((i(2, 5).min(i(3, 4))).to_array(), [2, 4]);
    }
}


