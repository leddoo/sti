pub mod num;
pub mod hint;
pub mod mem;
pub mod borrow;
pub mod cell;

pub mod simd;
pub mod float;
pub mod bit;
pub mod reader;

pub mod traits;

pub mod alloc;
pub mod arena;
pub mod boks;
pub mod rc;
pub mod manual_vec;
pub mod vec;
pub mod vec_deque;
pub mod hash;
pub mod static_vec;

pub mod sync;

pub mod utf8;
pub mod string;

pub mod packed_option;
pub mod keyed;

pub mod unck;

pub mod leb128;



#[macro_export]
macro_rules! static_assert {
    ($cond: expr) => {
        const _: () = assert!($cond);
    };
}

#[macro_export]
macro_rules! static_assert_eq {
    ($a: expr, $b: expr) => {
        const _: () = assert!($a == $b);
    };
}

#[macro_export]
macro_rules! static_assert_ne {
    ($a: expr, $b: expr) => {
        const _: () = assert!($a != $b);
    };
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        { let _ = ::core::write!($dst, $($arg)*); }
    };
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {
        $crate::string::format(::core::format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! format_in {
    ($alloc:expr, $($arg:tt)*) => {
        $crate::string::format_in($alloc, ::core::format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! assume {
    ($cond: expr) => {
        if !($cond) {
            ::core::hint::unreachable_unchecked();
        }
    };
}

/// erase lifetimes in `x: T`
///
/// this is safer than using transmute directly
/// as you're forced to specify the type
/// and you don't have to repeat it.
#[macro_export]
macro_rules! erase {
    ($T:ty, $x:expr) => {
        ::core::mem::transmute::<$T, $T>($x)
    };
}

/// increment `*x`, return the previous value.
#[macro_export]
macro_rules! inc {
    ($x:expr) => {{
        let x = $x;
        let result = *x;
        *x += 1;
        result
    }};
}

/// clone values for use in `move` closures.
/// example: `enclose!(foo as bar; foo; move || { ... })`
#[macro_export]
macro_rules! enclose {
    ($x:ident ; $($rest:tt)*) => {{
        let $x = $x.clone();
        $crate::enclose!($($rest)*)
    }};

    ($x:ident as $y:ident ; $($rest:tt)*) => {{
        let $y = $x.clone();
        $crate::enclose!($($rest)*)
    }};

    ($r:expr) => {
        $r
    };
}


#[cfg(test)]
mod tests {
    static_assert!(true);
    static_assert_eq!(33 + 36, 69);
    static_assert_ne!(0, 1);

    #[test]
    fn erase() {
        let mut v = crate::vec![4, 2];
        //let v0 = v.idx_mut(0);  // borrowck error.
        let v0 = unsafe { erase!(&mut i32, v.idx_mut(0)) };
        let v1 = v.idx_mut(1);
        assert_eq!(*v0, 4);
        assert_eq!(*v1, 2);
        *v0 += 2;
        *v1 += 7;
        assert_eq!(*v0, 6);
        assert_eq!(*v1, 9);
    }

    #[test]
    fn inc() {
        let mut foo = 1u8;
        assert_eq!(inc!(&mut foo), 1);
        assert_eq!(inc!(&mut foo), 2);
    }

    #[test]
    fn retain() {
        let foo = crate::rc::Rc::new(core::cell::Cell::new(1));
        assert_eq!(foo.ref_count(), 1);
        assert_eq!(foo.get(), 1);

        let f = enclose!(foo as bar; foo; move |n| {
            assert_eq!(foo.ref_count(), bar.ref_count());
            assert_eq!(foo.ref_count(), 3);
            foo.set(n);
            assert_eq!(bar.get(), n);
        });
        assert_eq!(foo.ref_count(), 3);
        assert_eq!(foo.get(), 1);
        f(42);
        assert_eq!(foo.ref_count(), 3);
        assert_eq!(foo.get(), 42);
        drop(f);
        assert_eq!(foo.ref_count(), 1);
    }
}

