use emacs::frame::LispFrameRef;

use crate::canvas::Canvas;
use crate::canvas::CanvasRef;
use crate::draw_commands::DrawCommands;
use crate::output::Output;
use crate::output::OutputRef;
#[cfg(window_system = "pgtk")]
use crate::wr_canvas_init_from_wayland;
use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use webrender::api::ColorF;
use webrender::{self, api::units::*};

use super::display_info::DisplayInfoRef;

pub trait LispFrameExt {
    fn output(&self) -> OutputRef;
    fn canvas(&self) -> CanvasRef;
    fn set_cursor_color(&self, color: ColorF);
    fn cursor_color(&self) -> ColorF;
    fn cursor_foreground_color(&self) -> ColorF;
    #[cfg(window_system = "winit")]
    fn set_window_handle(&self, handle: RawWindowHandle);
    fn set_background_color(&self, color: ColorF);
    fn display_info(&self) -> DisplayInfoRef;
    fn draw_commands(&self) -> DrawCommands;
    fn window_handle(&self) -> Option<RawWindowHandle>;
    fn display_handle(&self) -> Option<RawDisplayHandle>;
    fn size(&self) -> DeviceIntSize;
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
            #[cfg(window_system = "pgtk")]
            {
                let mut output = self.output();
                let widget = output.get_raw().widget;
                if !widget.is_null() {
                    let gwin = unsafe { gtk_sys::gtk_widget_get_window(widget) };
                    let surface = unsafe {
                        gdk_wayland_sys::gdk_wayland_window_get_wl_surface(
                            gwin as *mut _ as *mut gdk_wayland_sys::GdkWaylandWindow,
                        )
                    };
                    log::debug!("surface: {:?}", surface);
                    wr_canvas_init_from_wayland(surface, self.clone());
                }
            }
            #[cfg(window_system = "winit")]
            {
                let canvas = Box::new(Canvas::build(self.clone()));
                self.output().get_inner().set_canvas(canvas);
            }
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

    #[cfg(window_system = "winit")]
    fn set_window_handle(&self, handle: RawWindowHandle) {
        self.output().get_inner().set_window_handle(handle);
    }

    fn set_background_color(&self, color: ColorF) {
        self.output().get_inner().set_background_color(color);
    }

    fn display_info(&self) -> DisplayInfoRef {
        self.output().display_info()
    }

    fn draw_commands(&self) -> DrawCommands {
        DrawCommands::new(self.clone())
    }

    fn window_handle(&self) -> Option<RawWindowHandle> {
        #[cfg(window_system = "winit")]
        return self.output().get_inner().window_handle;

        #[cfg(not(window_system = "winit"))]
        unimplemented!()
    }

    fn display_handle(&self) -> Option<RawDisplayHandle> {
        #[cfg(window_system = "winit")]
        return self.output().display_info().get_inner().raw_display_handle;

        #[cfg(not(window_system = "winit"))]
        unimplemented!()
    }

    fn size(&self) -> DeviceIntSize {
        DeviceIntSize::new(self.pixel_width, self.pixel_height)
    }

    fn uuid(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.window_handle().hash(&mut hasher);
        hasher.finish()
    }
}
