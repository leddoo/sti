mod range;
mod slice;
mod vec;
mod free_vec;

pub use range::*;
pub use slice::*;
pub use vec::*;
pub use free_vec::*;


pub trait Key: Copy + PartialEq {
    const LIMIT_SELF: Self;
    const LIMIT:      usize;

    const MAX: Self;


    unsafe fn from_usize_unck(value: usize) -> Self;

    fn usize(self) -> usize;


    #[inline(always)]
    fn from_usize(value: usize) -> Option<Self> {
        (value < Self::LIMIT).then(|| unsafe { Self::from_usize_unck(value) })
    }

    #[inline(always)]
    unsafe fn sub_unck(self, other: Self) -> usize {
        self.usize() - other.usize()
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Option<usize> {
        self.usize().checked_sub(other.usize())
    }

    #[inline(always)]
    unsafe fn add_unck(self, offset: usize) -> Self {
        unsafe { Self::from_usize_unck(self.usize() + offset) }
    }

    #[inline(always)]
    fn add(self, offset: usize) -> Option<Self> {
        Self::from_usize(self.usize().checked_add(offset)?)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        if self.usize() >= other.usize() { self  }
        else                             { other }
    }


    #[inline(always)]
    fn next(&mut self) -> Option<Self> {
        let result = *self;
        *self = self.add(1)?;
        return Some(result);
    }
}

impl<K: Key> crate::packed_option::Reserved for K {
    const RESERVED: Self = K::LIMIT_SELF;
}



#[macro_export]
macro_rules! define_key_basic {
    ($ty:ty, $name_vis:vis, $name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $name_vis struct $name($ty);

        impl $name {
            #[inline(always)]
            pub const fn new_unck(value: $ty) -> Self { Self(value) }

            #[inline(always)]
            pub const fn new(value: $ty) -> sti::packed_option::PackedOption<Self> {
                sti::packed_option::PackedOption::from_raw(Self(value))
            }

            #[inline(always)]
            pub const fn inner(self) -> $ty { self.0 }

            #[inline(always)]
            pub fn some(self) -> sti::packed_option::PackedOption<Self> {
                sti::packed_option::Reserved::some(self)
            }
        }

        impl sti::keyed::Key for $name {
            const LIMIT_SELF: Self = Self(<$ty>::MAX);
            const LIMIT: usize = <$ty>::MAX as usize;

            const MAX: Self = Self(<$ty>::MAX - 1);

            #[inline(always)]
            unsafe fn from_usize_unck(value: usize) -> Self {
                Self(value as $ty)
            }

            #[inline(always)]
            fn usize(self) -> usize {
                self.0 as usize
            }
        }
    };
}

#[macro_export]
macro_rules! define_key_opt {
    ($name:ident, $opt_vis:vis, $opt_name:ident) => {
        $opt_vis type $opt_name = sti::packed_option::PackedOption<$name>;
    };
}

#[macro_export]
macro_rules! define_key_rng {
    ($name: ident, $rng_vis:vis, $rng_name: ident) => {
        $rng_vis type $rng_name = sti::keyed::KRange<$name>;
    };
}


#[macro_export]
macro_rules! define_key_dsp {
    ($name: ident, $fmt: expr) => {
        impl core::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $fmt, self.0)
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
        sti::define_key_basic!($ty, $name_vis, $name);
        $( sti::define_key_opt!($name, $opt_vis, $opt_name); )?
        $( sti::define_key_rng!($name, $rng_vis, $rng_name); )?
        $( sti::define_key_dsp!($name, $dsp_name); )?
    };

    ($all_vis:vis, $ty:ty,
        $name:ident
        $(, opt : $opt_name:ident)?
        $(, rng : $rng_name:ident)?
        $(, dsp : $dsp_name:expr)?
    ) => {
        sti::define_key_basic!($ty, $all_vis, $name);
        $( sti::define_key_opt!($name, $all_vis, $opt_name); )?
        $( sti::define_key_rng!($name, $all_vis, $rng_name); )?
        $( sti::define_key_dsp!($name, $dsp_name); )?
    };
}

