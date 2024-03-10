use crate::cursor::build_mouse_cursors;
use crate::cursor::emacs_to_winit_cursor;
use emacs::globals::Qfullscreen;
use emacs::globals::Qmaximized;
use emacs::terminal::TerminalRef;
use emacs::{
    bindings::{
        do_pending_window_change, fullscreen_type, gl_renderer_fit_context, gui_figure_window_size,
        list4i, make_frame, make_frame_without_minibuffer, make_minibuffer_frame, output_method,
        winit_output, Emacs_Cursor, Fcons, Vframe_list,
    },
    frame::FrameRef,
    globals::{Qinner_edges, Qnil, Qnone, Qonly, Qouter_edges},
    keyboard::KeyboardRef,
    lisp::LispObject,
};
use webrender_api::units::*;
use webrender_api::ColorF;
use winit::dpi::LogicalPosition;
use winit::dpi::PhysicalSize;
use winit::monitor::MonitorHandle;

use winit::{dpi::PhysicalPosition, window::WindowBuilder};

use emacs::display_info::DisplayInfoRef;
use emacs::output::Output;

pub trait FrameExtWinit {
    fn setup_winit(&mut self, params: LispObject);
    fn set_winit_window(&self, handle: winit::window::Window);
    fn set_inner_size(&self, size: PhysicalSize<u32>);
    fn set_cursor_color(&self, color: ColorF);
    fn set_background_color(&self, color: ColorF);
    fn set_cursor_position(&self, pos: PhysicalPosition<f64>);
    fn set_visible_(&mut self, visible: bool);
    fn set_cursor_icon(&self, cursor: Emacs_Cursor);
    fn edges(&self, type_: LispObject) -> LispObject;
    fn fullscreen(&self);
    fn implicitly_set_name(&mut self, arg: LispObject, _old_val: LispObject);
    fn iconify(&mut self);
    fn current_monitor(&self) -> Option<MonitorHandle>;
    fn cursor_position(&self) -> LogicalPosition<i32>;
    fn winit_scale_factor(&self) -> f64;
    fn handle_size_change(&mut self, size: DeviceIntSize, scale_factor: f64);
    fn handle_scale_factor_change(&mut self, _scale_factor: f64);
}

impl FrameExtWinit for FrameRef {
    fn setup_winit(&mut self, params: LispObject) {
        /* Compute the size of the winit window.  */
        // FIXME what to do with the window_prompting here
        let _window_prompting =
            unsafe { gui_figure_window_size(self.as_mut(), params, true, true) };

        let mut terminal = self.terminal();

        let event_loop = &terminal.winit_term_data().event_loop;
        let window_builder = WindowBuilder::new().with_visible(true);
        let primary_monitor = terminal.primary_monitor();
        let scale_factor = primary_monitor.scale_factor();

        let invocation_name: String = unsafe { emacs::bindings::globals.Vinvocation_name.into() };

        #[cfg(free_unix)]
        let window_builder = {
            use winit::platform::wayland::WindowBuilderExtWayland;
            window_builder.with_name(&invocation_name, "")
        };

        let window = window_builder.build(&event_loop).unwrap();
        window.set_theme(None);
        window.set_title(&invocation_name);

        self.pixel_width = (window.inner_size().width as f64 / scale_factor).round() as i32;
        self.pixel_height = (window.inner_size().height as f64 / scale_factor).round() as i32;

        self.set_winit_window(window);
    }

    fn set_winit_window(&self, window: winit::window::Window) {
        self.output().winit_term_data().set_window(window);
    }

    fn set_inner_size(&self, size: PhysicalSize<u32>) {
        if let Some(ref window) = self.output().winit_term_data().window {
            let _ = window.request_inner_size(size);
        }
    }

    fn set_cursor_position(&self, pos: PhysicalPosition<f64>) {
        self.output().winit_term_data().set_cursor_position(pos);
    }

    fn set_cursor_color(&self, color: ColorF) {
        self.output().winit_term_data().set_cursor_color(color);
    }

    fn set_background_color(&self, color: ColorF) {
        self.output().winit_term_data().set_background_color(color);
    }

    fn set_visible_(&mut self, is_visible: bool) {
        let _ = &self.set_visible(is_visible as u32);

        let data = self.output().winit_term_data();
        let window = data
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
        let data = self.output().winit_term_data();
        let window = data
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        let cursor = emacs_to_winit_cursor(cursor);
        window.set_cursor_icon(cursor);
    }

    fn edges(&self, type_: LispObject) -> LispObject {
        let data = self.output().winit_term_data();
        let window = data
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
            let data = self.output().winit_term_data();
            let window = data
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
        let data = self.output().winit_term_data();
        let window = data
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");

        window.set_title(&title);
    }

    fn iconify(&mut self) {
        self.set_iconified(true);
        let data = self.output().winit_term_data();
        let window = data
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        window.set_visible(false);
    }

    fn current_monitor(&self) -> Option<MonitorHandle> {
        let data = self.output().winit_term_data();
        let window = data
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet");
        window.current_monitor()
    }

    fn cursor_position(&self) -> LogicalPosition<i32> {
        let pos = self.output().winit_term_data().cursor_position;
        LogicalPosition::new(
            (pos.x / self.winit_scale_factor()).round() as i32,
            (pos.y / self.winit_scale_factor()).round() as i32,
        )
    }

    fn winit_scale_factor(&self) -> f64 {
        if let Some(monitor) = self.current_monitor() {
            return monitor.scale_factor();
        }

        if let Some(ref window) = self.output().winit_term_data().window {
            return window.scale_factor();
        }

        1.0
    }

    fn handle_size_change(&mut self, size: DeviceIntSize, _scale_factor: f64) {
        log::trace!("frame handle_size_change: {size:?}");
        self.change_size(
            size.width as i32,
            size.height as i32 - self.menu_bar_height,
            false,
            true,
            false,
        );

        unsafe { do_pending_window_change(false) };
        unsafe { gl_renderer_fit_context(self.as_mut()) };
    }

    fn handle_scale_factor_change(&mut self, scale_factor: f64) {
        log::trace!("frame handle_scale_factor_change... {scale_factor:?}");
        unsafe { gl_renderer_fit_context(self.as_mut()) };
    }
}
