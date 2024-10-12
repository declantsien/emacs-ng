use crate::bindings::fullscreen_type;
use crate::display_traits::FrameParam;
use crate::frame::FrameRef;
use crate::lisp::ExternalPtr;
use crate::terminal::TerminalRef;
use raw_window_handle::HasDisplayHandle;
use raw_window_handle::HasWindowHandle;
use raw_window_handle::RawDisplayHandle;
use raw_window_handle::RawWindowHandle;
use webrender_api::ColorF;

use std::ptr;

impl TerminalRef {
    pub fn raw_display_handle(&self) -> Option<RawDisplayHandle> {
        unimplemented!();
    }
}

impl FrameRef {
    // Using frame winit window display handle, fallback to terminal display handle
    pub fn raw_display_handle(&self) -> Option<RawDisplayHandle> {
        unimplemented!();
    }

    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        unimplemented!();        
    }

    pub fn cursor_color(&self) -> ColorF {
        unimplemented!();
    }

    // This value may differ from MonitorHandle::scale_factor.
    pub fn scale_factor(&self) -> f64 {
        unimplemented!();
    }

    pub fn cursor_foreground_color(&self) -> ColorF {
        unimplemented!();
    }

    pub fn maximized(&self) -> bool {
        unimplemented!();
    }

    pub fn parent_frame_handle(&self) -> Option<RawWindowHandle> {
        if self.parent_frame.is_nil() {
            return None;
        }

        let parent_frame = FrameRef::from(self.parent_frame);
        parent_frame.raw_window_handle()
    }
}
