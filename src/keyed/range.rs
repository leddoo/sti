use super::Key;


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KRange<K: Key> {
    begin: K,
    end:   K,  // >= begin
}

impl<K: Key> KRange<K> {
    #[inline(always)]
    pub unsafe fn new_unck(begin: K, end: K) -> KRange<K> {
        KRange { begin, end }
    }

    #[inline(always)]
    pub fn new(begin: K, end: K) -> KRange<K> {
        KRange {
            begin,
            end: end.max(begin),
        }
    }

    #[inline(always)]
    pub fn begin(self) -> K {
        self.begin
    }

    #[inline(always)]
    pub fn set_begin(&mut self, new_begin: K) {
        self.begin = new_begin;
        self.end   = self.end.max(new_begin);
    }


    #[inline(always)]
    pub fn end(self) -> K {
        self.end
    }

    #[inline(always)]
    pub fn set_end(&mut self, new_end: K) {
        self.end = new_end.max(self.begin);
    }


    #[inline(always)]
    pub fn len(self) -> usize {
        unsafe { self.end.sub_unck(self.begin) }
    }


    #[inline(always)]
    pub fn try_idx(self, i: usize) -> Option<K> {
        if i < self.len() {
            return Some(unsafe { self.begin.add_unck(i) });
        }
        None
    }

    #[inline(always)]
    pub fn idx(self, i: usize) -> K {
        self.try_idx(i).unwrap()
    }

    #[inline(always)]
    pub fn try_first(self) -> Option<K> {
        self.try_idx(0)
    }

    #[inline(always)]
    pub fn first(self) -> K {
        self.idx(0)
    }


    #[inline(always)]
    pub fn try_rev(self, i: usize) -> Option<K> {
        if i < self.len() {
            let r = (self.len() - 1) - i;
            return Some(unsafe { self.begin.add_unck(r) });
        }
        None
    }

    #[inline(always)]
    pub fn rev(self, i: usize) -> K {
        self.try_rev(i).unwrap()
    }

    #[inline(always)]
    pub fn try_last(self) -> Option<K> {
        self.try_rev(0)
    }

    #[inline(always)]
    pub fn last(self) -> K {
        self.rev(0)
    }
}


impl<K: Key + core::fmt::Debug> core::fmt::Debug for KRange<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}..{:?}", self.begin, self.end)
    }
}


impl<K: Key> Iterator for KRange<K> {
    type Item = K;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len() > 0 {
            let result = self.begin;
            self.begin = unsafe { self.begin.add_unck(1) };
            return Some(result);
        }
        None
    }

    #[inline(always)]
    fn nth(&mut self, i: usize) -> Option<Self::Item> {
        if i < self.len() {
            let result = unsafe { self.begin.add_unck(i) };
            self.begin = unsafe { result.add_unck(1) };
            return Some(result);
        }
        None
    }

    #[inline(always)]
    fn last(self) -> Option<Self::Item> {
        self.try_last()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

