#![cfg(windows)]
#![allow(non_snake_case)]

use minidl::*;
use std::io::Result;
use std::os::raw::*;

#[allow(dead_code)]
struct XInput {
    XInputGetState:     unsafe extern "system" fn (_: *const c_char),
    XInputGetStateEx:   unsafe extern "system" fn (_: *const c_char),
}

impl XInput {
    pub fn new() -> Result<Self> { Self::from(Library::load("xinput1_3.dll")?) }
    pub fn from(lib: Library) -> Result<Self> {
        unsafe{Ok(Self{
            XInputGetState:     lib.sym("XInputGetState\0")?,
            XInputGetStateEx:   lib.sym_by_ordinal(100)?,
        })}
    }
}

#[test] fn ok_sym() {
    let xinput = XInput::new();
    if !std::env::var_os("CI").is_some() {
        xinput.expect("XInput");
    }
}
