//! # `gl_loader` : A simple OpenGL function pointer loader.
//! This (small) package aim to do *only one thing*: Provide a `get_proc_address` function to load OpenGL function pointer.
//! It will not provide any form of window or OpenGL context creation. You will have to handle them by yourself.
//! The code is simply a binding to the pointer loading part of the glad library: [glad](https://glad.dav1d.de/)
//!
//! ## Usage:
//! ```
//! extern crate gl;
//! extern crate gl_loader;
//!
//! // Load OpenGL library.
//! gl_loader::init_gl();
//! // Load all the OpenGL function pointer using the `gl` crate.
//! gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
//! // Unload the OpenGL library.
//! gl_loader::end_gl();
//! ```

extern crate libc;

use std::ffi::CString;

extern "C" {
    fn open_gl() -> libc::c_int;
    fn close_gl();
    fn get_proc(name: *const libc::c_char) -> *const libc::c_void;
}

/// Load the OpenGL system library.
/// It is usually necesseray to be able to load function pointer.
///
/// ```
/// let ret = init_gl();
/// assert_ne!(ret, 0);
/// ```
pub fn init_gl() -> i32 {
    unsafe { open_gl() }
}

/// Close the OpenGL library.
/// This function does nothing if `init_gl()` has not been called.
pub fn end_gl() {
    unsafe {
        close_gl();
    }
}

/// Safe wrapper around Glad's `get_proc` function.
fn get_proc_addr(name: *const libc::c_char) -> *const libc::c_void {
    unsafe { get_proc(name) }
}

/// This function take an OpenGL function name and output its function pointer.
///
/// ```
/// let glCreateShader = get_proc_address("glCreateShader");
/// assert_ne!(glCreateShader, 0);
/// ```
pub fn get_proc_address(func: &str) -> *const () {
    let c_str = CString::new(func.as_bytes()).unwrap();
    let str_ptr = c_str.as_ptr();
    let p = get_proc_addr(str_ptr) as *const ();
    p
}
