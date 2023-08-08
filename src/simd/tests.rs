#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn b32x2() {
        let b = B32x2::new_b;
        let f = F32x2::new;

        assert_eq!(B32x2::splat_b(false).to_array_b(), [false, false]);
        assert_eq!(B32x2::splat_b(true).to_array_b(),  [true,  true]);

        assert_eq!(b(false, false).to_array_b(), [false, false]);
        assert_eq!(b(false, true).to_array_b(),  [false, true]);
        assert_eq!(b(true, false).to_array_b(), [true,  false]);
        assert_eq!(b(true, true).to_array_b(),  [true,  true]);

        assert_eq!((!b(false, false)).to_array_b(), [true,  true]);
        assert_eq!((!b(false, true)).to_array_b(),  [true,  false]);
        assert_eq!((!b(true, false)).to_array_b(), [false, true]);
        assert_eq!((!b(true, true)).to_array_b(),  [false, false]);

        assert_eq!((b(false, true) & b(false, true)).to_array_b(), [false, true]);
        assert_eq!((b(false, true) & b(true, false)).to_array_b(), [false, false]);
        assert_eq!((b(true, false) & b(false, true)).to_array_b(), [false, false]);
        assert_eq!((b(true, false) & b(true, false)).to_array_b(), [true,  false]);

        assert_eq!((b(false, false) | b(false, false)).to_array_b(), [false, false]);
        assert_eq!((b(false, false) | b(false, true)).to_array_b(),  [false, true]);
        assert_eq!((b(false, false) | b(true, false)).to_array_b(), [true, false]);
        assert_eq!((b(false, true)  | b(false, false)).to_array_b(), [false, true]);
        assert_eq!((b(true, false) | b(false, false)).to_array_b(), [true, false]);

        assert_eq!(b(false, false).select(b(false, true), b(true, false)).to_array_b(), [true,  false]);
        assert_eq!(b(false, true).select(b(false, true), b(true, false)).to_array_b(), [true,  true]);
        assert_eq!(b(true, false).select(b(false, true), b(true, false)).to_array_b(), [false, false]);
        assert_eq!(b(true, true).select(b(false, true), b(true, false)).to_array_b(), [false, true]);

        assert_eq!(b(false, false).select(f(1.0, 2.0), f(3.0, 4.0)).as_array(), [3.0, 4.0]);
        assert_eq!(b(false, true).select(f(1.0, 2.0), f(3.0, 4.0)).as_array(), [3.0, 2.0]);
        assert_eq!(b(true, false).select(f(1.0, 2.0), f(3.0, 4.0)).as_array(), [1.0, 4.0]);
        assert_eq!(b(true, true).select(f(1.0, 2.0), f(3.0, 4.0)).as_array(), [1.0, 2.0]);

        assert_eq!(b(false, false).any(), false);
        assert_eq!(b(false, true).any(), true);
        assert_eq!(b(true, false).any(), true);
        assert_eq!(b(true, true).any(), true);

        assert_eq!(b(false, false).all(), false);
        assert_eq!(b(false, true).all(), false);
        assert_eq!(b(true, false).all(), false);
        assert_eq!(b(true, true).all(), true);
    }

    #[test]
    fn f32x2() {
        let f = F32x2::new;

        assert_eq!(F32x2::splat(0.0).as_array(), [0.0, 0.0]);
        assert_eq!(F32x2::splat(1.0).as_array(), [1.0, 1.0]);

        assert_eq!(f(1.0, 2.0).as_array(), [1.0, 2.0]);

        assert_eq!((f(1.0, 2.0) + f(3.0, 4.0)).as_array(), [4.0, 6.0]);
        assert_eq!((f(1.0, 2.0) - f(3.0, 5.0)).as_array(), [-2.0, -3.0]);
        assert_eq!((f(5.0, 2.0) * f(3.0, 4.0)).as_array(), [15.0, 8.0]);
        assert_eq!((f(6.0, 2.0) / f(3.0, 4.0)).as_array(), [2.0, 0.5]);

        assert_eq!((0.5 * f(6.0, 2.0)).as_array(), [3.0, 1.0]);
        assert_eq!((f(6.0, 2.0) * 0.5).as_array(), [3.0, 1.0]);
        assert_eq!((f(6.0, 2.0) / 2.0).as_array(), [3.0, 1.0]);

        assert_eq!(f(0.25, 0.75).hadd(), 1.0);
        assert_eq!(f(1.0, 3.0).dot(f(-5.0, 0.5)), -3.5);

        assert_eq!(f(3.0, 4.0).length(), 5.0);

        assert_eq!((f(1.0, 2.0).eq(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(1.0, 2.0).eq(f(3.0, 2.0))).to_array_b(), [false, true]);
        assert_eq!((f(1.0, 2.0).eq(f(1.0, 4.0))).to_array_b(), [true,  false]);
        assert_eq!((f(1.0, 2.0).eq(f(1.0, 2.0))).to_array_b(), [true,  true]);

        assert_eq!((f(1.0, 2.0).ne(f(1.0, 2.0))).to_array_b(), [false, false]);
        assert_eq!((f(1.0, 2.0).ne(f(1.0, 4.0))).to_array_b(), [false, true]);
        assert_eq!((f(1.0, 2.0).ne(f(3.0, 2.0))).to_array_b(), [true,  false]);
        assert_eq!((f(1.0, 2.0).ne(f(3.0, 4.0))).to_array_b(), [true,  true]);

        assert_eq!((f(3.0, 4.0).le(f(1.0, 2.0))).to_array_b(), [false, false]);
        assert_eq!((f(3.0, 4.0).le(f(1.0, 4.0))).to_array_b(), [false, true]);
        assert_eq!((f(3.0, 4.0).le(f(1.0, 5.0))).to_array_b(), [false, true]);
        assert_eq!((f(3.0, 4.0).le(f(3.0, 2.0))).to_array_b(), [true,  false]);
        assert_eq!((f(3.0, 4.0).le(f(4.0, 2.0))).to_array_b(), [true,  false]);
        assert_eq!((f(3.0, 4.0).le(f(3.0, 4.0))).to_array_b(), [true,  true]);
        assert_eq!((f(3.0, 4.0).le(f(4.0, 5.0))).to_array_b(), [true,  true]);

        assert_eq!((f(3.0, 4.0).lt(f(1.0, 2.0))).to_array_b(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(1.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(1.0, 5.0))).to_array_b(), [false, true]);
        assert_eq!((f(3.0, 4.0).lt(f(3.0, 2.0))).to_array_b(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(4.0, 2.0))).to_array_b(), [true,  false]);
        assert_eq!((f(3.0, 4.0).lt(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(3.0, 4.0).lt(f(4.0, 5.0))).to_array_b(), [true,  true]);

        assert_eq!((f(1.0, 2.0).ge(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(1.0, 4.0).ge(f(3.0, 4.0))).to_array_b(), [false, true]);
        assert_eq!((f(1.0, 5.0).ge(f(3.0, 4.0))).to_array_b(), [false, true]);
        assert_eq!((f(3.0, 2.0).ge(f(3.0, 4.0))).to_array_b(), [true,  false]);
        assert_eq!((f(4.0, 2.0).ge(f(3.0, 4.0))).to_array_b(), [true,  false]);
        assert_eq!((f(3.0, 4.0).ge(f(3.0, 4.0))).to_array_b(), [true,  true]);
        assert_eq!((f(4.0, 5.0).ge(f(3.0, 4.0))).to_array_b(), [true,  true]);

        assert_eq!((f(1.0, 2.0).gt(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(1.0, 4.0).gt(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(1.0, 5.0).gt(f(3.0, 4.0))).to_array_b(), [false, true]);
        assert_eq!((f(3.0, 2.0).gt(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(4.0, 2.0).gt(f(3.0, 4.0))).to_array_b(), [true,  false]);
        assert_eq!((f(3.0, 4.0).gt(f(3.0, 4.0))).to_array_b(), [false, false]);
        assert_eq!((f(4.0, 5.0).gt(f(3.0, 4.0))).to_array_b(), [true,  true]);

        assert_eq!(f(4.5, -2.5).to_i32_unck().as_array(), [4, -2]);
        assert_eq!(f(4.5, -2.5).to_i32().as_array(),      [4, -2]);
        assert_eq!(f(f32::INFINITY, 8_000_000_000.0).to_i32().as_array(), [i32::MAX, i32::MAX]);
        assert_eq!(f(-f32::INFINITY, -8_000_000_000.0).to_i32().as_array(), [i32::MIN, i32::MIN]);

        assert_eq!((f(4.0, 5.0).min(f(3.0, 6.0))).as_array(), [3.0, 5.0]);
        assert_eq!((f(2.0, 5.0).min(f(3.0, 4.0))).as_array(), [2.0, 4.0]);

        //assert_eq!(f(2.0, 5.0).hmin(), 2.0);
        //assert_eq!(f(2.0, 5.0).hmax(), 5.0);

        assert_eq!(f(1.0, 2.0).x(), 1.0);
        assert_eq!(f(1.0, 2.0).y(), 2.0);

        assert_eq!(f(3.5, -3.5).floor().as_array(), [3.0, -4.0]);
        assert_eq!(f(3.5, -3.5).ceil().as_array(),  [4.0, -3.0]);
        assert_eq!(f(3.5, -3.5).round().as_array(), [4.0, -4.0]);
        assert_eq!(f(3.7, -3.7).round().as_array(), [4.0, -4.0]);
        assert_eq!(f(3.2, -3.2).round().as_array(), [3.0, -3.0]);
        assert_eq!(f(3.5, -3.5).trunc().as_array(), [3.0, -3.0]);
        assert_eq!(f(3.7, -3.7).trunc().as_array(), [3.0, -3.0]);

        assert_eq!(f(3.7, -3.8).abs().as_array(), [3.7, 3.8]);

        assert_eq!(f(3.7, -3.8).with_sign_of(f(-69.0, -42.0)).as_array(), [-3.7, -3.8]);
        assert_eq!(f(3.7, -3.8).with_sign_of(f(-69.0, 42.0)).as_array(), [-3.7,  3.8]);
        assert_eq!(f(3.7, -3.8).with_sign_of(f(69.0, -42.0)).as_array(), [ 3.7, -3.8]);
        assert_eq!(f(3.7, -3.8).with_sign_of(f(69.0, 42.0)).as_array(), [ 3.7,  3.8]);

        assert_eq!(f(4.0, 9.0).sqrt().as_array(), [2.0, 3.0]);

        assert_eq!(f(1.0, 2.0).lerp(f(2.0, 3.0), 0.25).as_array(), [1.25, 2.25]);
        assert_eq!(f(1.0, 2.0).lerp_v(f(2.0, 3.0), [0.25, 0.75].into()).as_array(), [1.25, 2.75]);
    }

    #[test]
    fn i32x2() {
        let i = I32x2::new;

        assert_eq!(i(4, -2).to_f32().as_array(), [4.0, -2.0]);

        assert_eq!((i(4, 5).min(i(3, 6))).as_array(), [3, 5]);
        assert_eq!((i(2, 5).min(i(3, 4))).as_array(), [2, 4]);

        //assert_eq!(i(2, 5).hmin(), 2);
        //assert_eq!(i(2, 5).hmax(), 5);

        assert_eq!((i(0xff01f0ffu32 as i32, 0x00ff1208) >> 17).as_array(),
            [0xffffff80u32 as i32, 0x0000007f]);
        assert_eq!((i(0xff01f0ffu32 as i32, 0x00ff1208) << 17).as_array(),
            [0xe1fe0000u32 as i32, 0x24100000]);
    }

    #[test]
    fn u32x2() {
        let u = U32x2::new;

        assert_eq!((u(0xff01f0ff, 0x00ff1208) >> 17).as_array(), [0x00007f80, 0x0000007f]);
        assert_eq!((u(0xff01f0ff, 0x00ff1208) << 17).as_array(), [0xe1fe0000, 0x24100000]);
    }
}


