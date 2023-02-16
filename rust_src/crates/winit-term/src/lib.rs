#![feature(concat_idents)]
#![allow(non_upper_case_globals)]
#![feature(once_cell)]

#[macro_use]
extern crate emacs;
extern crate lisp_macros;
#[macro_use]
extern crate lisp_util;

pub mod frame;
pub mod input;
pub mod term;

mod clipboard;
mod cursor;
mod event;
mod event_loop;
mod winit_term;

mod platform {
    #[cfg(macos_platform)]
    pub mod macos;
}

pub mod windowing {
    // macro for building key_name c string
    macro_rules! kn {
        ($e:expr) => {
            concat!($e, '\0').as_ptr() as *const libc::c_char
        };
    }

    #[cfg(use_tao)]
    pub use crate::windowing::tao::*;
    #[cfg(not(use_tao))]
    pub use crate::windowing::winit::*;

    #[cfg(use_tao)]
    pub mod tao;
    #[cfg(not(use_tao))]
    pub mod winit;
}

#[cfg(macos_platform)]
pub use crate::platform::macos;

pub use crate::winit_term::{tip_frame, winit_display_list};

#[cfg(not(test))]
include!(concat!(env!("OUT_DIR"), "/c_exports.rs"));
