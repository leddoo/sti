use crate::bit::*;

/*
    utf-8 encoding:
    #bytes  #bits  code-point  encoding
         1      7      U+0000  0|0000000
                       U+007F  0|1111111
         2     11      U+0080  110|00010 10|000000
                       U+07FF  110|11111 10|111111
         3     16      U+0800  1110|0000 10|100000 10|000000
                       U+D7FF  1110|1101 10|011111 10|111111
                       U+E000  1110|1110 10|000000 10|000000
                       U+FFFF  1110|1111 10|111111 10|111111
         4     21    U+010000  11110|000 10|010000 10|000000 10|000000
                     U+10FFFF  11110|100 10|001111 10|111111 10|111111
*/


pub use core::str::from_utf8_unchecked as str_from_slice_unck;

#[inline(always)]
pub unsafe fn str_from_parts_unck<'a>(ptr: *const u8, len: usize) -> &'a str {
    unsafe { str_from_slice_unck(core::slice::from_raw_parts(ptr, len)) }
}


#[inline(always)]
pub fn is_ascii(b: u8) -> bool {
    b < 0x80
}

#[inline(always)]
pub fn non_ascii_mask_4(word: u32) -> Bitmask4 {
    Bitmask4::find_high_bit_bytes(word)
}

#[inline(always)]
pub fn non_ascii_mask_8(word: u64) -> Bitmask8 {
    Bitmask8::find_high_bit_bytes(word)
}


pub struct Utf8Error {
    pub valid_until: *const u8,
}

/// check one utf-8 encoded codepoint.
/// - assumes `buffer.len() > 0`.
/// - on success, returns the remaining buffer after the codepoint.
#[inline(always)]
pub fn check_1(buffer: &[u8]) -> Result<&[u8], Utf8Error> {
    let _ = buffer;
    unimplemented!()
}


/// validate entire buffer as utf-8.
#[inline]
pub fn validate_inline(buffer: &[u8]) -> Result<&str, Utf8Error> {
    let mut rem = buffer;

    while rem.len() > 0 {
        if is_ascii(rem[0]) {
            rem = &rem[1..];

            if rem.len() >= 8 {
                while rem.len() >= 8 {
                    let word = unsafe { core::ptr::read_unaligned(rem.as_ptr() as *const u64) };
                    let word = u64::from_le(word);

                    match non_ascii_mask_8(word).next() {
                        None    => { rem = &rem[8..]        }
                        Some(i) => { rem = &rem[i..]; break }
                    }
                }
            }
            else if rem.len() >= 4 {
                let word = unsafe { core::ptr::read_unaligned(rem.as_ptr() as *const u32) };
                let word = u32::from_le(word);

                match non_ascii_mask_4(word).next() {
                    None    => rem = &rem[4..],
                    Some(i) => rem = &rem[i..],
                }
            }
        }
        else {
            rem = match check_1(rem) {
                Ok(rem) => rem,
                Err(e) => return Err(e),
            };
        }
    }

    return Ok(unsafe { str_from_slice_unck(buffer) });
}

/// validate entire buffer as utf-8.
pub fn validate(buffer: &[u8]) -> Result<&str, Utf8Error> {
    validate_inline(buffer)
}


/// utf-8 validation for string parsing.
/// - returns the valid string up to '"' or '\' or eof,
///   and whether it stopped because of '"' or '\'.
#[inline]
pub fn validate_string_inline(string: &[u8]) -> Result<(&str, bool), Utf8Error> {
    let mut rem = string;

    while rem.len() > 0 {
        let at = rem[0];

        if is_ascii(at) {
            if at == b'"' || at == b'\\' {
                let valid = unsafe {
                    str_from_parts_unck(
                        string.as_ptr(),
                        rem.as_ptr() as usize - string.as_ptr() as usize)
                };
                return Ok((valid, true));
            }
            rem = &rem[1..];

            while rem.len() >= 4 {
                let word = unsafe { core::ptr::read_unaligned(rem.as_ptr() as *const u32) };
                let word = u32::from_le(word);

                let mut non_ascii = non_ascii_mask_4(word);
                let mut stoppers =
                      Bitmask4::find_equal_bytes(word, b'"')
                    | Bitmask4::find_equal_bytes(word, b'\\');

                if (non_ascii | stoppers).none() {
                    rem = &rem[4..];
                }
                else {
                    let non_ascii = non_ascii.next();
                    let stopper = stoppers.next();

                    'non_ascii: {
                    if let Some(stopper) = stopper {
                        if let Some(non_ascii) = non_ascii {
                            if non_ascii < stopper {
                                break 'non_ascii;
                            }
                        }

                        let end = rem.as_ptr() as usize + stopper;
                        let valid = unsafe {
                            str_from_parts_unck(
                                string.as_ptr(),
                                end - string.as_ptr() as usize)
                        };
                        return Ok((valid, true));
                    }}

                    let Some(non_ascii) = non_ascii else { unreachable!() };
                    rem = &rem[non_ascii..];
                    break;
                }
            }
        }
        else {
            rem = match check_1(rem) {
                Ok(rem) => rem,
                Err(e) => return Err(e),
            };
        }
    }

    // didn't encounter '"' or '\' or error.
    return Ok((unsafe { str_from_slice_unck(string) }, false));
}

/// utf-8 validation for string parsing.
/// - returns the valid string up to '"' or '\' or eof,
///   and whether it stopped because of '"' or '\'.
pub fn validate_string(string: &[u8]) -> Result<(&str, bool), Utf8Error> {
    validate_string_inline(string)
}


#[inline(always)]
pub fn is_boundary(b: u8) -> bool {
    //     b <  0b10000000 = 128 ~= -128
    // ||  b >= 0b11000000 = 196 ~=  -64
    //
    //    (b < -128 || b >= -64)
    // == !(b >= -128 && b < -64)
    // == !(true && b < -64)
    // == b >= -64
    (b as i8) >= -64
}

#[inline(always)]
pub fn is_continuation(b: u8) -> bool {
    !is_boundary(b)
}



