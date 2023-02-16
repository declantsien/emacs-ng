use crate::cursor::build_mouse_cursors;
use crate::cursor::emacs_to_winit_cursor;
use crate::event_loop::WrEventLoop;
use emacs::globals::Qfullscreen;
use emacs::globals::Qmaximized;
use emacs::{
    bindings::{
        fullscreen_type, list4i, make_frame, make_frame_without_minibuffer, make_minibuffer_frame,
        output_method, winit_output, Emacs_Cursor,
    },
    frame::LispFrameRef,
    globals::{Qinner_edges, Qnil, Qnone, Qonly, Qouter_edges},
    keyboard::KeyboardRef,
    lisp::LispObject,
};
use winit::monitor::MonitorHandle;
use winit::window::WindowId;
use wr_renderer::frame::LispFrameExt;
use wr_renderer::output::Output;

use winit::{dpi::PhysicalPosition, window::WindowBuilder};

use wr_renderer::display_info::DisplayInfoRef;

pub fn create_frame(
    display: LispObject,
    mut dpyinfo: DisplayInfoRef,
    tem: LispObject,
    mut kb: KeyboardRef,
) -> (LispFrameRef, WindowId) {
    log::trace!("create_frame");
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

    let event_loop = WrEventLoop::global().try_lock().unwrap();
    let window_builder = WindowBuilder::new().with_visible(true);

    #[cfg(all(wayland_platform))]
    let window_builder = {
        let invocation_name: emacs::multibyte::LispStringRef =
            unsafe { emacs::bindings::globals.Vinvocation_name.into() };
        let invocation_name = invocation_name.to_utf8();
        window_builder.with_title(invocation_name)
    };

    let window = window_builder.build(&event_loop.el()).unwrap();
    let mut output = Box::new(Output::default());
    output.set_display_info(dpyinfo);
    build_mouse_cursors(&mut output.as_mut().as_raw());

    // TODO default frame size?
    frame.pixel_width = window.inner_size().width as i32;
    frame.pixel_height = window.inner_size().height as i32;

    // Remeber to destory the Output object when frame destoried.
    let output = Box::into_raw(output);
    frame.output_data.winit = output as *mut winit_output;

    let window_id = window.id();
    frame.set_window(window);
    dpyinfo.get_inner().frames.insert(frame.uuid(), frame);
    log::trace!("create_frame done");
    (frame, window_id)
}

pub trait LispFrameWinitExt {
    fn set_window(&self, handle: winit::window::Window);
    fn set_visible2(&mut self, visible: bool);
    fn set_cursor_icon(&self, cursor: Emacs_Cursor);
    fn edges(&self, type_: LispObject) -> LispObject;
    fn fullscreen(&self);
    fn implicitly_set_name(&mut self, arg: LispObject, _old_val: LispObject);
    fn iconify(&mut self);
    fn current_monitor(&self) -> Option<MonitorHandle>;
    fn cursor_position(&self) -> PhysicalPosition<i32>;
}

impl LispFrameWinitExt for LispFrameRef {
    fn set_window(&self, window: winit::window::Window) {
        self.output().get_inner().set_window(window);
    }

    fn set_visible2(&mut self, is_visible: bool) {
        let _ = &self.set_visible(is_visible as u32);

        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");

        if is_visible {
            window.set_visible(true);
        } else {
            window.set_visible(false);
        }
    }

    fn set_cursor_icon(&self, cursor: Emacs_Cursor) {
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        let cursor = emacs_to_winit_cursor(cursor);
        window.set_cursor_icon(cursor);
    }

    fn edges(&self, type_: LispObject) -> LispObject {
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");

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
                let internal_border_width = self.internal_border_width();

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

    fn fullscreen(&self) {
        if !self.is_visible() {
            return;
        }

        if self.want_fullscreen() == fullscreen_type::FULLSCREEN_MAXIMIZED {
            let inner = self.output().get_inner();
            let window = inner
                .window
                .as_ref()
                .expect("frame doesnt have associated winit window yet");
            window.set_maximized(true);
            self.store_param(Qfullscreen, Qmaximized);
        }
    }
    fn implicitly_set_name(&mut self, arg: LispObject, _old_val: LispObject) {
        if self.name.eq(arg) {
            return;
        }

        self.name = arg;

        let title = format!("{}", arg.force_string());
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");

        window.set_title(&title);
    }

    fn iconify(&mut self) {
        self.set_iconified(true);
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        window.set_visible(false);
    }

    fn current_monitor(&self) -> Option<MonitorHandle> {
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        window.current_monitor()
    }

    fn cursor_position(&self) -> PhysicalPosition<i32> {
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        if let Ok(pos) = window.cursor_position() {
            return pos.cast();
        }

        PhysicalPosition::new(0, 0)
    }
}
