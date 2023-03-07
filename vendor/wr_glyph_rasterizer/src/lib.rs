/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A glyph rasterizer for webrender
//!
//! ## Overview
//!
//! ## Usage
//!

#![feature(once_cell)]

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod gamma_lut;
mod rasterizer;
mod telemetry;
mod types;

pub mod font;
pub mod profiler;

pub use rasterizer::*;
pub use types::*;

#[macro_use]
extern crate malloc_size_of_derive;
#[macro_use]
extern crate tracy_rs;
#[macro_use]
extern crate log;
#[macro_use]
extern crate smallvec;

#[cfg(any(feature = "serde"))]
#[macro_use]
extern crate serde;

extern crate malloc_size_of;

