
/// a slice reader to aid with parsing.
///
/// - `deref`s into a slice with the remaining values.
///   this enables use of `starts_with` and other slice methods.
/// - not `Copy`, to prevent accidental clones. as, when parsing,
///   it is important to maintain a unique current position in
///   the input.
///   but `Clone`, to support reverting. see also: `set_offset`.
#[derive(Clone)]
pub struct Reader<'a, T> {
    start: *const T,
    data: &'a [T],
}

impl<'a, T> Reader<'a, T> {
    #[inline(always)]
    pub fn new(data: &'a [T]) -> Self {
        Reader { start: data.as_ptr(), data }
    }


    /// number of values in original slice.
    #[inline(always)]
    pub fn original_len(&self) -> usize {
        self.consumed() + self.remaining()
    }

    /// returns the original slice, that was passed to `new`.
    #[inline(always)]
    pub fn original_slice(&self) -> &'a [T] {
        unsafe {
            core::slice::from_raw_parts(
                self.start,
                self.original_len())
        }
    }

    /// returns the consumed slice.
    ///
    /// - same as `&original_slice[..offset]`
    #[inline(always)]
    pub fn consumed_slice(&self) -> &'a [T] {
        unsafe {
            core::slice::from_raw_parts(
                self.start,
                self.consumed())
        }
    }

    /// returns the remaining slice.
    ///
    /// - same as `&original_slice[offset..]`
    /// - same as `deref` (modulo the lifetime).
    #[inline(always)]
    pub fn as_slice(&self) -> &'a [T] {
        self.data
    }

    /// returns the current offset from the start of the `original_slice`.
    #[inline(always)]
    pub fn offset(&self) -> usize {
        self.data.as_ptr() as usize - self.start as usize
    }

    /// set the `offset` (in the `original_slice`) to a different value.
    ///
    /// # panics
    /// - if `new_offset > self.original_len()`.
    #[inline(always)]
    pub fn set_offset(&mut self, new_offset: usize) {
        self.data = &self.original_slice()[new_offset..];
    }

    /// number of consumed values.
    ///
    /// - same as `offset`.
    #[inline(always)]
    pub fn consumed(&self) -> usize {
        self.offset()
    }

    /// number of remaining values.
    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.data.len()
    }


    /// reference to next element.
    #[inline(always)]
    pub fn peek_ref(&self) -> Option<&'a T> {
        self.data.get(0)
    }

    /// reference to element at index.
    #[inline(always)]
    pub fn peek_ref_at(&self, index: usize) -> Option<&'a T> {
        self.data.get(index)
    }

    /// value of next element.
    #[inline(always)]
    pub fn peek(&self) -> Option<T>  where T: Copy {
        self.data.get(0).copied()
    }

    /// value of element at index.
    #[inline(always)]
    pub fn peek_at(&self, index: usize) -> Option<T>  where T: Copy {
        self.data.get(index).copied()
    }

    /// next `n` elements.
    #[inline(always)]
    pub fn peek_n(&self, n: usize) -> Option<&'a [T]> {
        if n <= self.data.len() {
            return Some(&self.data[..n]);
        }
        return None;
    }



    /// reference to next element and advance offset.
    #[inline(always)]
    #[must_use]
    pub fn next_ref(&mut self) -> Option<&'a T> {
        let result = self.data.get(0);
        if result.is_some() {
            self.consume(1);
        }
        return result;
    }

    /// value of next element and advance offset.
    #[inline(always)]
    #[must_use]
    pub fn next(&mut self) -> Option<T>  where T: Copy {
        self.next_ref().copied()
    }

    /// next `n` elements and advance offset.
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

    /// reference to next element and advance offset, if predicate true.
    #[inline(always)]
    pub fn next_ref_if<F: FnOnce(&T) -> bool>(&mut self, f: F) -> Option<&'a T> {
        if let Some(at) = self.data.get(0) {
            if f(at) {
                self.consume(1);
                return Some(at);
            }
        }
        return None;
    }

    /// value of next element and advance offset, if predicate true.
    #[inline(always)]
    pub fn next_if<F: FnOnce(&T) -> bool>(&mut self, f: F) -> Option<T>  where T: Copy {
        self.next_ref_if(f).copied()
    }



    /// consume `n` elements of input.
    ///
    /// # panics
    /// - if there are fewer than `n` elements remaining.
    #[inline(always)]
    pub fn consume(&mut self, n: usize) {
        self.data = &self.data[n..];
    }

    /// consume input, if a predicate is true.
    ///
    /// - returns `true`, if the predicate returned true.
    /// - returns `false` otherwise, or if no input was left.
    #[inline(always)]
    pub fn consume_if<F: FnOnce(&T) -> bool>(&mut self, f: F) -> bool {
        self.next_ref_if(f).is_some()
    }

    /// consume input, if equal.
    ///
    /// - returns `true`, if the next element was equal.
    /// - returns `false` otherwise, or if no input was left.
    #[inline(always)]
    pub fn consume_if_eq(&mut self, v: &T) -> bool  where T: PartialEq {
        self.next_ref_if(|at| at == v).is_some()
    }

    /// consume input, while a predicate is true.
    ///
    /// - returns a bool `no_eoi`, that's `false`, if the end of input was reached
    ///   before the predicate returned `false`.
    #[inline(always)]
    pub fn consume_while<F: FnMut(&T) -> bool>(&mut self, mut f: F) -> bool {
        while let Some(at) = self.data.get(0) {
            if f(at) { self.consume(1); }
            else     { return true      }
        }
        return false;
    }

    /// consume input, while a predicate is true.
    ///
    /// - returns a slice, from the current offset, up to (and including) the last
    ///   element, for which the predicate returned true.
    /// - returns a bool `no_eoi`, that's `false`, if the end of input was reached
    ///   before the predicate returned `false`. (same as `consume_while`)
    /// - useful for parsing strings.
    #[inline(always)]
    pub fn consume_while_slice<F: FnMut(&T) -> bool>(&mut self, f: F) -> (&'a [T], bool) {
        let offset = self.offset();
        self.consume_while_slice_from(offset, f)
    }

    /// consume input, while a predicate is true.
    ///
    /// - returns a slice, from the specified `from_offset`, up to (and including) the
    ///   last element, for which the predicate returned true.
    /// - returns a bool `no_eoi`, that's `false`, if the end of input was reached
    ///   before the predicate returned `false`. (same as `consume_while`)
    /// - elements from the specified `from_offset` to the (initial) current offset are
    ///   included in the slice, without being passed to the predicate.
    /// - useful for parsing strings & identifiers.
    #[inline(always)]
    pub fn consume_while_slice_from<F: FnMut(&T) -> bool>(&mut self, from_offset: usize, f: F) -> (&'a [T], bool) {
        assert!(from_offset <= self.offset());

        let no_eoi = self.consume_while(f);
        let slice = &self.original_slice()[from_offset..self.offset()];
        return (slice, no_eoi);
    }
}


impl<'a, T: Copy> Reader<'a, T> {
    #[inline(always)]
    #[must_use]
    pub fn next_array<const N: usize>(&mut self) -> Option<[T; N]> {
        let slice = self.next_n(N)?;
        Some(slice.try_into().unwrap())
    }


    #[inline(always)]
    pub fn expect(&mut self, value: T) -> Result<(), ()>  where T: PartialEq {
        if let Some(at) = self.peek() {
            if at == value {
                self.consume(1);
                return Ok(());
            }
        }
        return Err(());
    }

    #[inline(always)]
    pub fn expect_n(&mut self, values: &[T]) -> Result<(), ()>  where T: PartialEq {
        if self.len() < values.len() {
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

impl<'a, T> core::ops::Deref for Reader<'a, T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.data }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader() {
        let input = b"123 xy  \t\n \t\n\r  dfsadf'abc'def";

        let check_offset = |r: &Reader<u8>, offset: usize| {
            assert_eq!(r.original_len(), input.len());
            assert_eq!(r.original_slice(), input);
            assert_eq!(r.consumed_slice(), &input[..offset]);
            assert_eq!(r.as_slice(), &input[offset..]);
            assert_eq!(&**r, &input[offset..]);
            assert_eq!(r.offset(), offset);
            assert_eq!(r.consumed(), offset);
            assert_eq!(r.remaining(), input.len() - offset);
            assert_eq!(r.len(), input.len() - offset);
        };

        let mut r = Reader::new(input);

        check_offset(&r, 0);

        assert_eq!(r.peek_ref(), Some(&b'1'));
        assert_eq!(r.peek_ref_at(0), Some(&b'1'));
        assert_eq!(r.peek_ref_at(1), Some(&b'2'));
        assert_eq!(r.peek_ref_at(3), Some(&b' '));
        assert_eq!(r.peek(), Some(b'1'));
        assert_eq!(r.peek_at(0), Some(b'1'));
        assert_eq!(r.peek_at(1), Some(b'2'));
        assert_eq!(r.peek_at(3), Some(b' '));
        assert_eq!(r.peek_n(4), Some(b"123 ".as_slice()));


        assert_eq!(r.next_ref(), Some(&b'1'));
        assert_eq!(r.next(), Some(b'2'));
        assert_eq!(r.next_n(2), Some(b"3 ".as_slice()));

        check_offset(&r, 4);

        assert_eq!(r.next_ref_if(|at| at.is_ascii_whitespace()), None);
        assert_eq!(r.offset(), 4);
        assert_eq!(r.next_ref_if(|at| at.is_ascii_alphabetic()), Some(&b'x'));
        assert_eq!(r.offset(), 5);
        assert_eq!(r.next_if(|at| at.is_ascii_whitespace()), None);
        assert_eq!(r.offset(), 5);
        assert_eq!(r.next_if(|at| at.is_ascii_alphabetic()), Some(b'y'));
        assert_eq!(r.offset(), 6);

        r.consume(2);
        assert_eq!(r.peek(), Some(b'\t'));

        check_offset(&r, 8);

        assert_eq!(r.consume_if(|at| at.is_ascii_digit()), false);
        assert_eq!(r.offset(), 8);
        assert_eq!(r.consume_if(|at| at.is_ascii_whitespace()), true);
        assert_eq!(r.offset(), 9);

        assert_eq!(r.consume_if_eq(&b'\n'), true);
        assert_eq!(r.offset(), 10);
        assert_eq!(r.consume_if_eq(&b'\n'), false);
        assert_eq!(r.offset(), 10);

        assert_eq!(r.consume_while(|at| at.is_ascii_whitespace()), true);
        assert_eq!(r.offset(), 16);

        check_offset(&r, 16);

        assert_eq!(r.consume_while_slice(|at| at.is_ascii_alphabetic()),
            (b"dfsadf".as_slice(), true));
        check_offset(&r, 22);

        let from = r.offset();
        assert_eq!(r.consume_if_eq(&b'\''), true);
        assert_eq!(r.consume_while_slice_from(from, |at| *at != b'\''),
            (b"'abc".as_slice(), true));
        r.consume(1);
        check_offset(&r, 27);

        assert_eq!(r.consume_while(|_| true), false);
        check_offset(&r, 30);

        r.set_offset(27);
        check_offset(&r, 27);
        assert_eq!(r.consume_while_slice(|_| true), (b"def".as_slice(), false));

        r.set_offset(5);
        check_offset(&r, 5);
        assert_eq!(r.consume_while_slice_from(1, |_| true), (&input[1..], false));
    }
}

