#![cfg(windows)]
#![allow(non_snake_case)]

use minidl::*;
use std::fmt::{self, Debug, Formatter};
use std::io::Result;
use std::os::raw::*;

#[allow(dead_code)]
struct Example {
    OutputDebugStringA: unsafe extern "system" fn (_: *const c_char),
    Invalid_Optional:   Option<unsafe extern "system" fn (_: *const c_char)>,
    Invalid_Required:   unsafe extern "system" fn (_: *const c_char),
}

impl Example {
    pub fn new() -> Result<Self> {
        Self::from(Library::load("kernel32.dll")?)
    }

    pub fn from(lib: Library) -> Result<Self> {
        unsafe{Ok(Self{
            OutputDebugStringA: lib.sym("OutputDebugStringA\0")?,
            Invalid_Optional:   lib.sym_opt("Invalid_Optional\0"),
            Invalid_Required:   lib.sym("Invalid_Required\0")?,
        })}
    }
}

impl Debug for Example {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Example {{ ... }}")
    }
}

#[test] fn bad_load() {
    let e = Library::load("does_not_exist_invalid.dll").expect_err("Invalid DLL should've failed to load");
    let e = format!("{}", e);
    assert!(e.contains("does_not_exist_invalid"), "{}", e);
}

#[test] fn bad_sym() {
    let e = Example::new().expect_err("Example should've failed to load Invalid_Required");
    let e = format!("{}", e);
    assert!(!e.contains("Invalid_Optional"), "{}", e);
    assert!( e.contains("Invalid_Required"), "{}", e);
}

#[test] fn ok_sym() {
    unsafe {
        let OutputDebugStringA : unsafe extern "system" fn (_: *const c_char)
            = Library::load("kernel32.dll").unwrap()
            .sym("OutputDebugStringA\0").unwrap();

        OutputDebugStringA(b"Hello, world!\0".as_ptr() as _);
    }
}
