#![cfg(unix)]

use minidl::*;
use std::fmt::{self, Debug, Formatter};
use std::io::Result;
use std::os::raw::*;

#[allow(dead_code)]
struct Example {
    puts:               unsafe extern "C" fn (_: *const c_char) -> c_int,
    invalid_optional:   Option<unsafe extern "C" fn (_: *const c_char) -> c_int>,
    invalid_required:   unsafe extern "C" fn (_: *const c_char) -> c_int,
}

impl Debug for Example {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Example {{ ... }}")
    }
}

impl Example {
    pub fn new() -> Result<Self> {
        Self::from(Library::load("/lib/x86_64-linux-gnu/libc.so.6")?)
    }

    pub fn from(lib: Library) -> Result<Self> {
        unsafe{Ok(Self{
            puts:               lib.sym("puts\0")?,
            invalid_optional:   lib.sym_opt("invalid_optional\0"),
            invalid_required:   lib.sym("invalid_required\0")?,
        })}
    }
}

#[test] fn bad_load() {
    let e = Library::load("libdoes_not_exist_invalid.so").expect_err("Invalid SO should've failed to load");
    let e = format!("{}", e);
    assert!(e.contains("does_not_exist_invalid"), "{}", e);
}

#[test] fn bad_sym() {
    let e = Example::new().expect_err("Example should've failed to load invalid_required");
    let e = format!("{}", e);
    assert!(!e.contains("invalid_optional"), "{}", e);
    assert!( e.contains("invalid_required"), "{}", e);
}

#[test] fn ok_sym() {
    unsafe {
        let puts : unsafe extern "C" fn (_: *const c_char) -> c_int
            = Library::load("/lib/x86_64-linux-gnu/libc.so.6").unwrap()
            .sym("puts\0").unwrap();

        puts(b"Hello, world!\0".as_ptr() as _);
    }
}
