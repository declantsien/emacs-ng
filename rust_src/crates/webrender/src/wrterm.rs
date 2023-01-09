//! wrterm.rs

use crate::frame::LispFrameExt;
use crate::output::CanvasData;
use crate::output::OutputRef;
use core::ffi::c_void;
use emacs::multibyte::LispStringRef;
use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use raw_window_handle::WaylandDisplayHandle;
use surfman::Connection;

use lisp_macros::lisp_fn;

use crate::{color::lookup_color_by_name_or_hex, font::FontRef};

use emacs::{
    bindings::{image as Emacs_Image, list3i, Emacs_Pixmap},
    frame::LispFrameRef,
    globals::{Qnil, Qt},
    lisp::LispObject,
};

pub use crate::display_info::{DisplayInfo, DisplayInfoRef};

#[no_mangle]
pub extern "C" fn wr_get_fontset(output: OutputRef) -> i32 {
    output.fontset()
}

#[no_mangle]
pub extern "C" fn wr_get_font(output: OutputRef) -> FontRef {
    output.font()
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn wr_get_baseline_offset(output: OutputRef) -> i32 {
    0
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn wr_get_pixel(ximg: *mut Emacs_Image, x: i32, y: i32) -> i32 {
    unimplemented!();
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn wr_put_pixel(ximg: *mut Emacs_Image, x: i32, y: i32, pixel: u64) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wr_can_use_native_image_api(image_type: LispObject) -> bool {
    crate::image::can_use_native_image_api(image_type)
}

#[no_mangle]
pub extern "C" fn wr_load_image(
    frame: LispFrameRef,
    img: *mut Emacs_Image,
    spec_file: LispObject,
    spec_data: LispObject,
) -> bool {
    crate::image::load_image(frame, img, spec_file, spec_data)
}

#[no_mangle]
pub extern "C" fn wr_transform_image(
    frame: LispFrameRef,
    img: *mut Emacs_Image,
    width: i32,
    height: i32,
    rotation: f64,
) {
    crate::image::transform_image(frame, img, width, height, rotation);
}

#[no_mangle]
pub extern "C" fn image_sync_to_pixmaps(_frame: LispFrameRef, _img: *mut Emacs_Image) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn image_pixmap_draw_cross(
    _frame: LispFrameRef,
    _pixmap: Emacs_Pixmap,
    _x: i32,
    _y: i32,
    _width: i32,
    _height: u32,
    _color: u64,
) {
    unimplemented!();
}

/// Internal function called by `display-color-p', which see.
#[lisp_fn(min = "0")]
pub fn xw_display_color_p(_terminal: LispObject) -> LispObject {
    // webrender support color display
    Qt
}

/// Return t if the X display supports shades of gray.
/// Note that color displays do support shades of gray.
/// The optional argument TERMINAL specifies which display to ask about.
/// TERMINAL should be a terminal object, a frame or a display name (a string).
/// If omitted or nil, that stands for the selected frame's display.
#[lisp_fn(min = "0")]
pub fn x_display_grayscale_p(_terminal: LispObject) -> LispObject {
    // webrender support shades of gray
    Qt
}

/// Internal function called by `color-values', which see.
#[lisp_fn(min = "1")]
pub fn xw_color_values(color: LispObject, _frame: Option<LispFrameRef>) -> LispObject {
    let color_str = format!("{}", color.force_string());
    match lookup_color_by_name_or_hex(&color_str) {
        Some(c) => unsafe {
            list3i(
                (c.r * u16::MAX as f32) as i64,
                (c.g * u16::MAX as f32) as i64,
                (c.b * u16::MAX as f32) as i64,
            )
        },
        None => Qnil,
    }
}

/// Capture the contents of the current WebRender frame and
/// save them to a folder relative to the current working directory.
///
/// If START-SEQUENCE is not nil, start capturing each WebRender frame to disk.
/// If there is already a sequence capture in progress, stop it and start a new
/// one, with the new path and flags.
#[allow(unused_variables)]
#[lisp_fn(min = "2")]
pub fn wr_api_capture(path: LispStringRef, bits_raw: LispObject, start_sequence: LispObject) {
    #[cfg(not(feature = "capture"))]
    error!("Webrender capture not avaiable");
    #[cfg(feature = "capture")]
    {
        use std::fs::{create_dir_all, File};
        use std::io::Write;

        let path = std::path::PathBuf::from(path.to_utf8());
        match create_dir_all(&path) {
            Ok(_) => {}
            Err(err) => {
                error!("Unable to create path '{:?}' for capture: {:?}", &path, err);
            }
        };
        let bits_raw = unsafe {
            emacs::bindings::check_integer_range(
                bits_raw,
                webrender::CaptureBits::SCENE.bits() as i64,
                webrender::CaptureBits::all().bits() as i64,
            )
        };

        let frame = window_frame_live_or_selected(Qnil);
        let output = frame.winit_output();
        let bits = webrender::CaptureBits::from_bits(bits_raw as _).unwrap();
        let revision_file_path = path.join("wr.txt");
        message!("Trying to save webrender capture under {:?}", &path);

        // api call here can possibly make Emacs panic. For example there isn't
        // enough disk space left. `panic::catch_unwind` isn't support here.
        if start_sequence.is_nil() {
            output.render_api.save_capture(path, bits);
        } else {
            output.render_api.start_capture_sequence(path, bits);
        }

        match File::create(revision_file_path) {
            Ok(mut file) => {
                if let Err(err) = write!(&mut file, "{}", "") {
                    error!("Unable to write webrender revision: {:?}", err)
                }
            }
            Err(err) => error!(
                "Capture triggered, creating webrender revision info skipped: {:?}",
                err
            ),
        }
    }
}

/// Stop a capture begun with `wr--capture'.
#[lisp_fn(min = "0")]
pub fn wr_api_stop_capture_sequence() {
    #[cfg(not(feature = "capture"))]
    error!("Webrender capture not avaiable");
    #[cfg(feature = "capture")]
    {
        message!("Stop capturing WR state");
        let frame = window_frame_live_or_selected(Qnil);
        let output = frame.winit_output();
        output.render_api.stop_capture_sequence();
    }
}

pub fn wr_display_init(
    mut dpyinfo_ref: DisplayInfoRef,
    raw_handle: RawDisplayHandle,
    scale_factor: f32,
) {
    if dpyinfo_ref.get_inner().is_null() {
        dpyinfo_ref.init_inner();
    }

    if let Ok(connection) = Connection::from_raw_display_handle(raw_handle) {
        dpyinfo_ref.get_inner().raw_display_handle = Some(raw_handle);
        dpyinfo_ref.get_inner().connection = Some(connection);
        dpyinfo_ref.get_inner().scale_factor = scale_factor;
    } else {
        panic!("Failed to initialize surfman");
    };
}

pub fn wr_display_init_from_wayland(
    mut dpyinfo_ref: DisplayInfoRef,
    display: *mut c_void,
    scale_factor: f32,
) {
    if dpyinfo_ref.get_inner().is_null() {
        dpyinfo_ref.init_inner();
    }
    let mut display_handle = WaylandDisplayHandle::empty();
    display_handle.display = display;

    wr_display_init(
        dpyinfo_ref,
        RawDisplayHandle::Wayland(display_handle),
        scale_factor,
    );
}

pub fn wr_canvas_init(raw_handle: RawWindowHandle, frame: LispFrameRef) {
    let mut output = frame.output();

    let canvas_data = Box::new(CanvasData::build(raw_handle, frame));

    output.set_inner(canvas_data);
}

include!(concat!(env!("OUT_DIR"), "/wrterm_exports.rs"));
