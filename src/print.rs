use crate::fmt::Arguments;
use crate::static_vec::StaticVec;


const CAP: usize = 16*1024 - 4*crate::mem::size_of::<usize>();

#[thread_local]
static mut BUFFER: Buffer = Buffer {
    at_exit: AtThreadExit::new(),
    buffer: StaticVec::new(),
    lazy: false,
};

struct Buffer {
    at_exit: AtThreadExit,
    buffer: StaticVec<u8, CAP>,
    lazy: bool,
}

crate::static_assert_eq!(crate::mem::size_of::<Buffer>(), 16*1024);

impl Buffer {
    #[inline]
    fn get(&mut self) -> &mut Self {
        unsafe {
            self.at_exit.init(|| {
                BUFFER.flush();
            });
        }
        return self;
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        let mut i = self.buffer.extend_from_slice(bytes);
        while i < bytes.len() {
            self.flush();
            i += self.buffer.extend_from_slice(&bytes[i..]);
        }
    }

    fn flush(&mut self) {
        if self.buffer.len() > 0 {
            crate::os::write_stdout_all(self.buffer.as_slice()).unwrap();
            self.buffer.clear();
        }
    }
}

impl crate::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> crate::fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}


pub fn set_lazy(lazy: bool) -> bool {
    unsafe { crate::mem::replace(&mut BUFFER.lazy, lazy) }
}

pub fn flush() {
    unsafe { BUFFER.flush() }
}


pub fn print_args_lazy(args: Arguments) {
    let buffer = unsafe { BUFFER.get() };
    _ = crate::fmt::Write::write_fmt(buffer, args);
}

pub fn print_args_ln(args: Arguments, newline: bool) {
    print_args_lazy(args);
    if newline { print_str("\n") }
}


#[inline]
pub fn print_str_lazy(str: &str) {
    print_bytes_lazy(str.as_bytes());
}

#[inline]
pub fn print_str(str: &str) {
    print_bytes(str.as_bytes());
}

pub fn print_str_ln(str: &str, newline: bool) {
    print_str_lazy(str);
    if newline { print_str("\n") }
}


pub fn print_bytes_lazy(bytes: &[u8]) {
    let buffer = unsafe { BUFFER.get() };
    buffer.write_bytes(bytes);
}

pub fn print_bytes(bytes: &[u8]) {
    let buffer = unsafe { BUFFER.get() };
    buffer.write_bytes(bytes);
    if !buffer.lazy {
        buffer.flush();
    }
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::print_args_lazy($crate::fmt!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print::print_str_ln("", true)
    };

    ($($arg:tt)*) => {
        $crate::print::print_args_ln($crate::fmt!($($arg)*), true)
    };
}


// @todo: os/thread module.
use crate::mem::NonNull;


#[thread_local]
static mut AT_EXIT_HEAD: Option<NonNull<AtThreadExit>> = None;

pub struct AtThreadExit {
    func: Option<fn()>,
    next: Option<NonNull<AtThreadExit>>,
}

impl AtThreadExit {
    #[inline]
    pub const fn new() -> Self {
        Self { func: None, next: None }
    }

    #[inline]
    pub unsafe fn init(&mut self, func: fn()) {
        if self.func.is_none() {
            unsafe { self.init_cold(func) }
        }
    }

    #[cold]
    unsafe fn init_cold(&mut self, func: fn()) {
        if self.func.is_some() || self.next.is_some() {
            panic!()
        }

        self.func = Some(func);

        unsafe {
            self.next = AT_EXIT_HEAD;
            AT_EXIT_HEAD = Some(self.into());
        }
    }
}

