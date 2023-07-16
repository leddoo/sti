
#[derive(Clone)]
pub struct Reader<'a, T: Copy> {
    pub data: &'a [T],
}

impl<'a, T: Copy> Reader<'a, T> {
    #[inline(always)]
    pub fn new(data: &'a [T]) -> Self {
        Reader { data }
    }

    #[inline(always)]
    pub fn has_some(&self) -> bool {
        self.data.len() > 0
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.data.len() == 0
    }

    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.data.len()
    }

    #[inline(always)]
    pub fn rest(&self) -> &'a [T] {
        self.data
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }


    #[inline(always)]
    pub fn peek(&self) -> Option<T> {
        self.data.get(0).copied()
    }

    #[inline(always)]
    pub fn peek_at(&self, offset: usize) -> Option<T> {
        self.data.get(offset).copied()
    }

    #[inline(always)]
    pub fn peek_n(&self, n: usize) -> Option<&'a [T]> {
        if n <= self.data.len() {
            return Some(&self.data[..n]);
        }
        return None;
    }

    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> Option<T> {
        if let Some(at) = self.peek() {
            self.consume(1);
            return Some(at);
        }
        return None;
    }

    #[inline(always)]
    pub fn next_if<F: FnOnce(T) -> bool>(&mut self, f: F) -> Option<T> {
        if let Some(at) = self.peek() {
            if f(at) {
                self.consume(1);
                return Some(at);
            }
        }
        return None;
    }

    #[inline(always)]
    #[must_use]
    pub fn next_n(&mut self, n: usize) -> Option<&'a [T]> {
        if n <= self.data.len() {
            let result = &self.data[..n];
            self.consume(n);
            return Some(result);
        }
        return None;
    }

    #[inline(always)]
    #[must_use]
    pub fn next_array<const N: usize>(&mut self) -> Option<[T; N]> {
        let slice = self.next_n(N)?;
        Some(slice.try_into().unwrap())
    }

    #[inline(always)]
    pub fn consume(&mut self, n: usize) {
        self.data = &self.data[n..];
    }

    #[inline(always)]
    pub fn consume_while<F: FnMut(T) -> bool>(&mut self, mut f: F) -> bool {
        while let Some(at) = self.peek() {
            if f(at) { self.consume(1); }
            else     { return true      }
        }
        return false;
    }

    #[inline(always)]
    pub fn consume_while_slice<F: FnMut(T) -> bool>(&mut self, mut f: F) -> Option<&'a [T]> {
        let mut i = 0;
        while let Some(at) = self.peek_at(i) {
            if f(at) {
                i += 1;
            }
            else {
                let result = &self.data[..i];
                self.consume(i);
                return Some(result);
            }
        }
        return None;
    }


    #[inline(always)]
    pub fn expect(&mut self, value: T) -> Result<(), ()> where T: PartialEq {
        if let Some(at) = self.peek() {
            if at == value {
                self.consume(1);
                return Ok(());
            }
        }
        return Err(());
    }

    #[inline(always)]
    pub fn expect_n(&mut self, values: &[T]) -> Result<(), ()> where T: PartialEq {
        if self.remaining() < values.len() {
            return Err(());
        }

        for i in 0..values.len() {
            if self.data[i] != values[i] {
                return Err(());
            }
        }

        self.consume(values.len());
        return Ok(());
    }
}

