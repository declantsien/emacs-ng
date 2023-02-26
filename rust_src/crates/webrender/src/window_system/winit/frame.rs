use super::cursor::build_mouse_cursors;
use super::cursor::emacs_to_winit_cursor;
use crate::event_loop::WrEventLoop;
use crate::output::Canvas;
use crate::output::CanvasRef;
use crate::window_system::api::monitor::MonitorHandle;
use crate::window_system::output::OutputRef;
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
use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use webrender::api::ColorF;
use webrender::{self, api::units::*};

use super::output::Output;
use crate::frame::LispFrameExt;

use crate::window_system::api::{
    dpi::{LogicalPosition, PhysicalPosition},
    window::WindowBuilder,
};

use crate::display_info::DisplayInfoRef;

pub type FrameID = crate::window_system::api::window::WindowId;

pub trait LispFrameWinitExt {
    fn build(
        display: LispObject,
        dpyinfo: DisplayInfoRef,
        tem: LispObject,
        kb: KeyboardRef,
    ) -> Self;
    fn set_window(&self, handle: crate::window_system::api::window::Window);
    #[cfg(use_winit)]
    fn set_cursor_position(&self, pos: PhysicalPosition<f64>);
    fn set_visible2(&mut self, visible: bool);
    fn set_cursor_icon(&self, cursor: Emacs_Cursor);
    fn edges(&self, type_: LispObject) -> LispObject;
    fn fullscreen(&self);
    fn implicitly_set_name(&mut self, arg: LispObject, _old_val: LispObject);
    fn iconify(&mut self);
    fn current_monitor(&self) -> Option<MonitorHandle>;
    fn cursor_position(&self) -> LogicalPosition<i32>;
}

impl LispFrameExt for LispFrameRef {
    fn output(&self) -> OutputRef {
        return OutputRef::new(unsafe { self.output_data.winit } as *mut Output);
    }

    fn canvas(&self) -> CanvasRef {
        if self.output().get_canvas().is_null() {
            log::debug!("canvas_data empty");
            let canvas = Box::new(Canvas::build(self.clone()));
            self.output().get_inner().set_canvas(canvas);
        }

        self.output().get_canvas()
    }

    fn set_cursor_color(&self, color: ColorF) {
        self.output().get_inner().set_cursor_color(color);
    }

    fn cursor_color(&self) -> ColorF {
        self.output().get_inner().cursor_color
    }

    fn cursor_foreground_color(&self) -> ColorF {
        self.output().get_inner().cursor_foreground_color
    }

    fn set_background_color(&self, color: ColorF) {
        self.output().get_inner().set_background_color(color);
    }

    fn display_info(&self) -> DisplayInfoRef {
        self.output().display_info()
    }

    fn window_handle(&self) -> Option<RawWindowHandle> {
        if let Some(window) = &self.output().get_inner().window {
            use raw_window_handle::HasRawWindowHandle;
            return Some(window.raw_window_handle());
        } else {
            return None;
        }
    }

    fn display_handle(&self) -> Option<RawDisplayHandle> {
        return self.output().display_info().get_inner().raw_display_handle;
    }

    fn size(&self) -> DeviceIntSize {
        DeviceIntSize::new(self.pixel_width, self.pixel_height)
    }

    fn uuid(&self) -> FrameID {
        self.output()
            .get_inner()
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet")
            .id()
            .clone()
    }
}

impl LispFrameWinitExt for LispFrameRef {
    fn build(
        display: LispObject,
        mut dpyinfo: DisplayInfoRef,
        tem: LispObject,
        mut kb: KeyboardRef,
    ) -> Self {
        log::trace!("Winit creating new frame");
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

        let invocation_name: emacs::multibyte::LispStringRef =
            unsafe { emacs::bindings::globals.Vinvocation_name.into() };
        let invocation_name = invocation_name.to_utf8();

        #[cfg(all(wayland_platform, use_winit))]
        let window_builder = {
            use crate::window_system::api::platform::wayland::WindowBuilderExtWayland;
            window_builder.with_name(&invocation_name, "")
        };
        #[cfg(use_tao)]
        let window_builder = window_builder.with_title(invocation_name);

        let window = window_builder.build(&event_loop.el()).unwrap();
        #[cfg(use_winit)]
        window.set_theme(None);
        #[cfg(use_winit)]
        window.set_title(&invocation_name);
        let mut output = Box::new(Output::default());
        output.set_display_info(dpyinfo);
        build_mouse_cursors(&mut output.as_mut().as_raw());

        // TODO default frame size?
        log::trace!("frame total_cols {:?}", frame.total_cols);
        log::trace!("frame line_height {:?}", frame.line_height);

        frame.pixel_width = window.inner_size().width as i32;
        frame.pixel_height = window.inner_size().height as i32;

        // Remeber to destory the Output object when frame destoried.
        let output = Box::into_raw(output);
        frame.output_data.winit = output as *mut winit_output;

        frame.set_window(window);
        dpyinfo.get_inner().frames.insert(frame.uuid(), frame);
        log::trace!("create_frame done");
        frame
    }

    fn set_window(&self, window: crate::window_system::api::window::Window) {
        self.output().get_inner().set_window(window);
    }

    #[cfg(use_winit)]
    fn set_cursor_position(&self, pos: PhysicalPosition<f64>) {
        self.output().get_inner().set_cursor_position(pos);
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

    fn cursor_position(&self) -> LogicalPosition<i32> {
        let inner = self.output().get_inner();
        let window = inner
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        #[cfg(use_tao)]
        {
            if let Ok(pos) = window.cursor_position() {
                return LogicalPosition::<i32>::from_physical(pos, 1.0 / window.scale_factor());
            }
            LogicalPosition::new(0, 0)
        }
        #[cfg(use_winit)]
        {
            LogicalPosition::<i32>::from_physical(
                inner.cursor_position,
                1.0 / window.scale_factor(),
            )
        }
    }
}
