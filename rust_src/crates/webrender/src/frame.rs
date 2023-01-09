use emacs::frame::LispFrameRef;

use crate::draw_canvas::DrawCanvas;
use crate::output::CanvasDataRef;
use crate::output::Output;
use crate::output::OutputRef;
use raw_window_handle::RawWindowHandle;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::display_info::DisplayInfoRef;

pub trait LispFrameExt {
    fn output(&self) -> OutputRef;
    fn canvas_data(&self) -> CanvasDataRef;
    fn display_info(&self) -> DisplayInfoRef;
    fn canvas(&self) -> DrawCanvas;
    fn raw_window_handle(&self) -> RawWindowHandle;
    fn uuid(&self) -> u64;
}

impl LispFrameExt for LispFrameRef {
    fn output(&self) -> OutputRef {
        OutputRef::new(unsafe { self.output_data.winit } as *mut Output)
    }

    fn canvas_data(&self) -> CanvasDataRef {
        self.output().canvas_data()
    }

    fn display_info(&self) -> DisplayInfoRef {
        self.output().display_info()
    }

    fn canvas(&self) -> DrawCanvas {
        DrawCanvas::new(self.canvas_data())
    }

    fn raw_window_handle(&self) -> RawWindowHandle {
        self.canvas_data().raw_window_handle
    }

    fn uuid(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.raw_window_handle().hash(&mut hasher);
        hasher.finish()
    }
}
