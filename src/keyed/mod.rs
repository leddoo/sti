mod range;
mod slice;
mod vec;
mod free_vec;
mod gen_vec;

pub use range::*;
pub use slice::*;
pub use vec::*;
pub use free_vec::*;
pub use gen_vec::*;


pub trait Key: Copy + PartialEq + PartialOrd {
    /// the reserved value.
    /// `Key` assumes valid keys are less than `LIMIT`.
    const LIMIT: usize;
    const LIMIT_SELF: Self;

    fn from_usize_unck(value: usize) -> Self;

    fn usize(self) -> usize;


    #[inline]
    fn from_usize(value: usize) -> Option<Self> {
        if value < Self::LIMIT { Some(Self::from_usize_unck(value)) }
        else { None }
    }
}

impl<K: Key> crate::packed_option::Reserved for K {
    const RESERVED: Self = K::LIMIT_SELF;
}



#[macro_export]
macro_rules! define_key_basic {
    ($ty:ty, $name_vis:vis, $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $name_vis struct $name($ty);

        #[allow(dead_code)]
        impl $name {
            #[inline(always)]
            pub const fn new_unck(value: $ty) -> Self { Self(value) }

            #[inline(always)]
            pub const fn new(value: $ty) -> $crate::packed_option::PackedOption<Self> {
                $crate::packed_option::PackedOption::from_raw(Self(value))
            }

            #[inline(always)]
            pub const fn inner(self) -> $ty { self.0 }

            #[inline(always)]
            pub fn inner_mut_unck(&mut self) -> &mut $ty { &mut self.0 }

            #[inline(always)]
            pub fn some(self) -> $crate::packed_option::PackedOption<Self> {
                $crate::packed_option::Reserved::some(self)
            }
        }

        impl $crate::keyed::Key for $name {
            const LIMIT: usize = <$ty>::MAX as usize;
            const LIMIT_SELF: Self = Self(<$ty>::MAX);

            #[inline(always)]
            fn from_usize_unck(value: usize) -> Self {
                Self(value as $ty)
            }

            #[inline(always)]
            fn usize(self) -> usize {
                self.0 as usize
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}({})", core::stringify!($name), self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! define_key {
    ($ty:ty,
        $name_vis:vis $name:ident
        $(, opt : $opt_vis:vis $opt_name:ident)?
        $(, rng : $rng_vis:vis $rng_name:ident)?
        $(, dsp : $dsp_name:expr)?
    ) => {
        $crate::define_key_basic!($ty, $name_vis, $name);

        $( $opt_vis type $opt_name = $crate::packed_option::PackedOption<$name>; )?

        $( $rng_vis type $rng_name = $crate::keyed::KRange<$name>; )?

        $(
            impl core::fmt::Display for $name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, $dsp_name, self.0)
                }
            }
        )?
    };

    ($all_vis:vis, $ty:ty,
        $name:ident
        $(, opt : $opt_name:ident)?
        $(, rng : $rng_name:ident)?
        $(, dsp : $dsp_name:expr)?
    ) => {
        $crate::define_key_basic!($ty, $all_vis, $name);

        $( $all_vis type $opt_name = $crate::packed_option::PackedOption<$name>; )?

        $( $all_vis type $rng_name = $crate::keyed::KRange<$name>; )?

        $(
            impl core::fmt::Display for $name {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, $dsp_name, self.0)
                }
            }
        )?
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        use super::{Key, KVec};

        crate::define_key!(u32, Id);

        let mut vs: KVec<Id, i32> = KVec::new();
        let k1 = vs.push(42);
        let k2 = vs.push(69);

        assert_eq!(k1.usize(), 0);
        assert_eq!(k2.usize(), 1);
        assert_eq!(vs[k1], 42);
        assert_eq!(vs[k2], 69);

        let mut r = vs.range();
        assert_eq!(r.len(), 2);
        assert_eq!(r.idx(0), k1);
        assert_eq!(r.idx(1), k2);
        assert_eq!(r.rev(0), k2);
        assert_eq!(r.rev(1), k1);

        assert_eq!(r.next(), Some(k1));
        assert_eq!(r.len(), 1);
        assert_eq!(r.idx(0), k2);
        assert_eq!(r.rev(0), k2);

        assert_eq!(r.next(), Some(k2));
        assert_eq!(r.len(), 0);

        assert_eq!(r.next(), None);
        assert_eq!(r.len(), 0);
    }
}

