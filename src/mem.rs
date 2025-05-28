pub use core::{
    cell::{Cell, UnsafeCell},
    marker::{PhantomData, Unpin},
    mem::{
        MaybeUninit, ManuallyDrop,
        transmute, transmute_copy,
        take, replace, swap, forget,
        size_of, size_of_val, align_of, offset_of,
    },
    ptr::{
        NonNull,
        write_bytes, copy, copy_nonoverlapping,
        drop_in_place,
    },
    pin::{Pin, pin},
};


pub unsafe fn as_bytes<T>(value: &T) -> &[u8] { unsafe {
    crate::slice::from_raw_parts(
        value as *const T as *const u8,
        size_of_val(value))
}}

