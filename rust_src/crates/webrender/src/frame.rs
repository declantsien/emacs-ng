use emacs::frame::LispFrameRef;

use crate::canvas::Canvas;
use crate::canvas::CanvasRef;
use crate::output::Output;
use crate::output::OutputRef;
use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use webrender::api::ColorF;
use webrender::{self, api::units::*};

use super::display_info::DisplayInfoRef;

pub trait LispFrameExt {
    fn output(&self) -> OutputRef;
    fn canvas(&self) -> CanvasRef;
    fn set_cursor_color(&self, color: ColorF);
    fn cursor_color(&self) -> ColorF;
    fn cursor_foreground_color(&self) -> ColorF;
    fn set_background_color(&self, color: ColorF);
    fn display_info(&self) -> DisplayInfoRef;
    fn window_handle(&self) -> Option<RawWindowHandle>;
    fn display_handle(&self) -> Option<RawDisplayHandle>;
    fn size(&self) -> DeviceIntSize;
    #[cfg(window_system = "winit")]
    fn uuid(&self) -> emacs::windowing::window::WindowId;
    #[cfg(not(window_system = "winit"))]
    fn uuid(&self) -> u64;
}

impl LispFrameExt for LispFrameRef {
    fn output(&self) -> OutputRef {
        #[cfg(window_system = "winit")]
        return OutputRef::new(unsafe { self.output_data.winit } as *mut Output);
        #[cfg(window_system = "pgtk")]
        return OutputRef::new(unsafe { self.output_data.pgtk } as *mut Output);
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
        #[cfg(window_system = "winit")]
        if let Some(window) = &self.output().get_inner().window {
            use raw_window_handle::HasRawWindowHandle;
            return Some(window.raw_window_handle());
        } else {
            return None;
        }

        #[cfg(window_system = "pgtk")]
        {
            use raw_window_handle::WaylandWindowHandle;
            let mut output = self.output();
            let widget = output.as_raw().edit_widget;
            if !widget.is_null() {
                let gwin = unsafe { gtk_sys::gtk_widget_get_window(widget) };
                let surface = unsafe {
                    gdk_wayland_sys::gdk_wayland_window_get_wl_surface(
                        gwin as *mut _ as *mut gdk_wayland_sys::GdkWaylandWindow,
                    )
                };
                log::debug!("surface: {:?}", surface);
                let mut window_handle = WaylandWindowHandle::empty();
                window_handle.surface = surface;
                return Some(RawWindowHandle::Wayland(window_handle));
            }
            return None;
        }

        #[cfg(not(any(window_system = "winit", window_system = "pgtk")))]
        unimplemented!()
    }

    fn display_handle(&self) -> Option<RawDisplayHandle> {
        #[cfg(window_system = "winit")]
        return self.output().display_info().get_inner().raw_display_handle;

        #[cfg(window_system = "pgtk")]
        {
            use raw_window_handle::WaylandDisplayHandle;

            let display = unsafe {
                self.output()
                    .display_info()
                    .get_raw()
                    .__bindgen_anon_1
                    .display
            };
            let wl_display = unsafe {
                gdk_wayland_sys::gdk_wayland_display_get_wl_display(
                    display as *mut _ as *mut gdk_wayland_sys::GdkWaylandDisplay,
                )
            };
            let mut display_handle = WaylandDisplayHandle::empty();
            display_handle.display = wl_display;
            return Some(RawDisplayHandle::Wayland(display_handle));
        }

        #[cfg(not(any(window_system = "winit", window_system = "pgtk")))]
        unimplemented!()
    }

    fn size(&self) -> DeviceIntSize {
        DeviceIntSize::new(self.pixel_width, self.pixel_height)
    }

    #[cfg(window_system = "winit")]
    fn uuid(&self) -> emacs::windowing::window::WindowId {
        self.output()
            .get_inner()
            .window
            .as_ref()
            .expect("frame doesnt have associated winit window yet")
            .id()
            .clone()
    }

    #[cfg(not(window_system = "winit"))]
    fn uuid(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.window_handle().hash(&mut hasher);
        hasher.finish()
    }
}
