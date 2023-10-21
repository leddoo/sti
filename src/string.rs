use crate::alloc::{Alloc, GlobalAlloc};
use crate::vec::Vec;
use crate::utf8;


pub struct String<A: Alloc = GlobalAlloc> {
    buffer: Vec<u8, A>,
}

impl String<GlobalAlloc> {
    #[inline(always)]
    pub fn new() -> Self {
        String::new_in(GlobalAlloc)
    }

    #[inline(always)]
    pub fn with_cap(cap: usize) -> Self {
        String::with_cap_in(GlobalAlloc, cap)
    }


    #[inline(always)]
    pub fn from_str(value: &str) -> Self {
        String::from_str_in(GlobalAlloc, value)
    }
}

impl<A: Alloc> String<A> {
    #[inline(always)]
    pub fn new_in(alloc: A) -> Self {
        Self { buffer: Vec::new_in(alloc) }
    }

    #[inline(always)]
    pub fn with_cap_in(alloc: A, cap: usize) -> Self {
        Self { buffer: Vec::with_cap_in(alloc, cap) }
    }


    #[inline(always)]
    pub fn from_str_in(alloc: A, value: &str) -> Self {
        Self { buffer: Vec::from_slice_in(alloc, value.as_bytes()) }
    }


    #[inline(always)]
    pub fn len(&self) -> usize { self.buffer.len() }

    #[inline(always)]
    pub fn cap(&self) -> usize { self.buffer.cap() }

    #[inline(always)]
    pub fn alloc(&self) -> &A { self.buffer.alloc() }


    #[inline(always)]
    pub fn reserve(&mut self, min_cap: usize) {
        self.buffer.reserve(min_cap);
    }

    #[inline(always)]
    pub fn reserve_exact(&mut self, min_cap: usize) {
        self.buffer.reserve_exact(min_cap);
    }

    #[inline(always)]
    pub fn grow_by(&mut self, extra: usize) {
        self.buffer.grow_by(extra);
    }


    #[inline(always)]
    pub fn push(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes())
    }

    #[inline(always)]
    pub fn push_char(&mut self, c: char) {
        self.buffer.extend_from_slice(
            c.encode_utf8(&mut [0; 4]).as_bytes())
    }


    #[inline(always)]
    pub fn clone_in<A2: Alloc>(&self, alloc: A2) -> String<A2> {
        String { buffer: self.buffer.clone_in(alloc) }
    }

    #[inline(always)]
    pub fn leak<'a>(self) -> &'a str  where A: 'a {
        unsafe { utf8::str_unck(self.buffer.leak()) }
    }


    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { utf8::str_unck(self.buffer.as_slice()) }
    }


    #[inline(always)]
    pub fn into_inner(self) -> Vec<u8, A> { self.buffer }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] { &self.buffer }

    #[inline(always)]
    pub unsafe fn inner_mut(&mut self) -> &mut Vec<u8, A> { &mut self.buffer }
}


impl<A: Alloc + Clone> Clone for String<A> {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.clone_in(self.buffer.alloc().clone())
    }
}


impl<A: Alloc> core::fmt::Debug for String<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<A: Alloc> core::fmt::Display for String<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}


impl<A: Alloc> core::borrow::Borrow<str> for String<A> {
    #[inline(always)]
    fn borrow(&self) -> &str { self.as_str() }
}

impl<A: Alloc> core::ops::Deref for String<A> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.as_str() }
}


impl<A: Alloc> core::hash::Hash for String<A> {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<A: Alloc> PartialEq for String<A> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl<A: Alloc> Eq for String<A> {}


impl<A: Alloc> core::fmt::Write for String<A> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.push(s);
        Ok(())
    }
}


impl From<&str> for String<GlobalAlloc> {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

