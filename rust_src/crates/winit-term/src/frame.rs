use crate::cursor::build_mouse_cursors;
use crate::winit_term::insert_winit_window;
use crate::winit_term::WINIT_WINDOWS;
use emacs::{
    bindings::{
        list4i, make_frame, make_frame_without_minibuffer, make_minibuffer_frame, output_method,
        winit_output,
    },
    frame::{window_frame_live_or_selected, LispFrameRef},
    globals::{Qinner_edges, Qnil, Qnone, Qonly, Qouter_edges},
    keyboard::KeyboardRef,
    lisp::LispObject,
};
use raw_window_handle::HasRawWindowHandle;
use webrender_bindings::frame::LispFrameExt;
use webrender_bindings::output::Output;
use webrender_bindings::wr_canvas_init;

use winit::dpi::PhysicalPosition;
#[cfg(wayland_platform)]
use winit::platform::wayland::WindowBuilderExtWayland;

use crate::event_loop::EVENT_LOOP;

use webrender_bindings::display_info::DisplayInfoRef;

pub fn create_frame(
    display: LispObject,
    dpyinfo: DisplayInfoRef,
    tem: LispObject,
    mut kb: KeyboardRef,
) -> LispFrameRef {
    let frame = if tem.eq(Qnone) || tem.is_nil() {
        unsafe { make_frame_without_minibuffer(Qnil, kb.as_mut(), display) }
    } else if tem.eq(Qonly) {
        unsafe { make_minibuffer_frame() }
    } else if tem.is_window() {
        unsafe { make_frame_without_minibuffer(tem, kb.as_mut(), display) }
    } else {
        unsafe { make_frame(true) }
    };

    let mut frame = LispFrameRef::new(frame);

    frame.terminal = dpyinfo.get_inner().terminal.as_mut();
    frame.set_output_method(output_method::output_winit);

    let event_loop = EVENT_LOOP.lock().unwrap();
    let window_builder = winit::window::WindowBuilder::new().with_visible(true);

    #[cfg(wayland_platform)]
    let window_builder = {
        let invocation_name: emacs::multibyte::LispStringRef =
            unsafe { emacs::bindings::globals.Vinvocation_name.into() };
        let invocation_name = invocation_name.to_utf8();
        window_builder.with_name(invocation_name, "")
    };

    let window = window_builder.build(&event_loop.el()).unwrap();
    window.set_theme(None);
    window.set_title("Winit Emacs");
    let window_id = window.id();
    let raw_window_handle = window.raw_window_handle();
    let mut output = Box::new(Output::default());
    output.set_display_info(dpyinfo);
    build_mouse_cursors(&mut output.as_mut().get_raw());

    // TODO default frame size?
    frame.pixel_width = window.inner_size().width as i32;
    frame.pixel_height = window.inner_size().height as i32;

    // Remeber to destory the Output object when frame destoried.
    let output = Box::into_raw(output);
    frame.output_data.winit = output as *mut winit_output;

    wr_canvas_init(raw_window_handle, frame);

    insert_winit_window(frame.uuid(), window);
    dpyinfo
        .get_inner()
        .outputs
        .insert(window_id.into(), frame.output());

    frame
}

pub fn frame_edges(frame: LispObject, type_: LispObject) -> LispObject {
    let frame = window_frame_live_or_selected(frame);

    let uuid = frame.uuid();
    let wins = WINIT_WINDOWS.lock().unwrap();
    let window = wins.get(&uuid).unwrap();

    let (left, top, right, bottom) = match type_ {
        Qouter_edges => {
            let pos = window
                .outer_position()
                .unwrap_or_else(|_| PhysicalPosition::<i32>::new(0, 0));

            let size = window.outer_size();

            let left = pos.x;
            let top = pos.y;
            let right = left + size.width as i32;
            let bottom = top + size.height as i32;

            (left, top, right, bottom)
        }
        Qinner_edges => {
            let pos = window
                .inner_position()
                .unwrap_or_else(|_| PhysicalPosition::<i32>::new(0, 0));
            let size = window.inner_size();
            let internal_border_width = frame.internal_border_width();

            // webrender window has no interanl menu_bar, tab_bar and tool_bar
            let left = pos.x + internal_border_width;
            let top = pos.x + internal_border_width;
            let right = (left + size.width as i32) - internal_border_width;
            let bottom = (top + size.height as i32) - internal_border_width;

            (left, top, right, bottom)
        }
        // native edges
        _ => {
            let pos = window
                .inner_position()
                .unwrap_or_else(|_| PhysicalPosition::<i32>::new(0, 0));
            let size = window.inner_size();

            let left = pos.x;
            let top = pos.y;
            let right = left + size.width as i32;
            let bottom = top + size.height as i32;

            (left, top, right, bottom)
        }
    };
    unsafe { list4i(left as i64, top as i64, right as i64, bottom as i64) }
}
