#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![feature(concat_idents)]
#![allow(non_upper_case_globals)]
#![feature(once_cell)]

#[macro_use]
extern crate emacs;
extern crate lisp_macros;
#[macro_use]
extern crate lisp_util;
extern crate colors;

pub mod canvas;
pub mod color;
pub mod display_info;
pub mod font;
pub mod frame;
pub mod image;
pub mod output;
pub mod term;

mod cursor;
mod font_db;
mod fringe;
mod gl;
mod renderer;
mod texture;
pub mod util;
mod wrterm;

pub use crate::font::*;
pub use crate::term::*;
pub use crate::wrterm::*;

pub use webrender::api::units::DeviceIntSize;

pub mod gl {
    #[cfg(feature = "glutin")]
    pub use crate::gl::glutin::*;
    #[cfg(feature = "gtk3")]
    pub use crate::gl::gtk3::*;
    #[cfg(feature = "surfman")]
    pub use crate::gl::surfman::*;

    #[cfg(feature = "glutin")]
    pub mod glutin;
    #[cfg(feature = "gtk3")]
    pub mod gtk3;
    #[cfg(feature = "surfman")]
    pub mod surfman;
}

// pub use crate::wrterm::{wr_can_use_native_image_api, wr_load_image, wr_transform_image};

#[cfg(not(test))]
include!(concat!(env!("OUT_DIR"), "/c_exports.rs"));
