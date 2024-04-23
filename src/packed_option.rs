#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PackedOption<T: Reserved> {
    value: T
}

pub trait Reserved: Copy + PartialEq {
    const RESERVED: Self;

    #[track_caller]
    #[inline(always)]
    fn some(self) -> PackedOption<Self> {
        debug_assert!(self != Self::RESERVED);
        PackedOption { value: self }
    }
}

impl<T: Reserved> PackedOption<T> {
    pub const NONE: PackedOption<T> = PackedOption { value: Reserved::RESERVED };

    #[inline(always)]
    pub const fn from_raw(value: T) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.value == T::RESERVED
    }

    #[inline(always)]
    pub fn is_some(&self) -> bool {
        self.value != T::RESERVED
    }

    #[inline]
    pub fn to_option(self) -> Option<T> {
        self.is_some().then_some(self.value)
    }

    #[track_caller]
    #[inline]
    pub fn unwrap(self) -> T {
        self.to_option().unwrap()
    }

    #[inline]
    pub fn unwrap_unck(self) -> T {
        self.value
    }

    #[inline]
    pub fn take(&mut self) -> Self {
        core::mem::replace(self, Self::NONE)
    }
}


impl<T: Reserved> From<Option<T>> for PackedOption<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            PackedOption { value }
        }
        else {
            PackedOption { value: T::RESERVED }
        }
    }
}

impl<T: Reserved> From<PackedOption<T>> for Option<T> {
    #[inline(always)]
    fn from(value: PackedOption<T>) -> Self {
        value.to_option()
    }
}


impl<T: Reserved + core::fmt::Debug> core::fmt::Debug for PackedOption<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.to_option().fmt(f)
    }
}

