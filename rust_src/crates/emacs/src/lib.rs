#![allow(clippy::cognitive_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
// we need this to be able to inclde FieldOffsets in C structs
#![allow(improper_ctypes)]
// we have a bunch of unused code during testing at the moment, somehow
#![cfg_attr(test, allow(unused))]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(concat_idents)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]
#![feature(async_closure)]

#[cfg(all(glutin, surfman, winit))]
compile_error!("You cannot specify both `glutin` and `surfman` features for winit window system");
#[cfg(all(not(glutin), not(surfman), winit))]
compile_error!("One of `glutin` and `surfman` features is required for winit window system");

#[rustfmt::skip]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
#[rustfmt::skip]
pub mod definitions {
    include!(concat!(env!("OUT_DIR"), "/definitions.rs"));
}
#[rustfmt::skip]
pub mod globals {
    include!(concat!(env!("OUT_DIR"), "/globals.rs"));
}

#[macro_use]
pub mod sys;
#[macro_use]
pub mod eval_macros;
#[macro_use]
pub mod vector_macros;
pub mod lisp;

pub mod buffer;
#[cfg(feature = "window-system")]
pub mod color;
#[cfg(window_system)]
pub mod dispextern;
#[cfg(feature = "window-system")]
pub mod display_info;
pub mod eval;
pub mod font;
pub mod frame;
#[cfg(feature = "window-system")]
pub mod glyph;
pub mod keyboard;
#[cfg(feature = "window-system")]
pub mod lglyph;
pub mod list;
pub mod multibyte;
pub mod number;
pub mod obarray;
#[cfg(feature = "window-system")]
pub mod output;
pub mod process;
pub mod string;
pub mod symbol;
pub mod terminal;
pub mod thread;
pub mod vector;
pub mod window;
#[cfg(any(winit, pgtk))]
mod window_system;
pub mod xdisp;
#[cfg(any(winit, pgtk))]
pub use window_system::*;
#[cfg(any(glutin, surfman, gtk3))]
pub mod gfx {
    pub mod context;

    pub mod context_impl {
        #[cfg(glutin)]
        pub use crate::gfx::context_impl::glutin::*;
        #[cfg(gtk3)]
        pub use crate::gfx::context_impl::gtk3::*;
        #[cfg(surfman)]
        pub use crate::gfx::context_impl::surfman::*;

        #[cfg(glutin)]
        pub mod glutin;
        #[cfg(gtk3)]
        pub mod gtk3;
        #[cfg(surfman)]
        pub mod surfman;
    }
}
