pub use crate::bindings::output as Output;
use crate::display_info::DisplayInfoRef;
use crate::font::FontRef;
use crate::lisp::ExternalPtr;

pub type OutputRef = ExternalPtr<Output>;

pub trait OutputExtWindowSystem {
    fn display_info(&self) -> DisplayInfoRef;
    fn set_font(&mut self, font: FontRef);
    fn set_fontset(&mut self, fontset: i32);
}

impl OutputExtWindowSystem for OutputRef {
    fn set_font(&mut self, mut font: FontRef) {
        self.font = font.as_mut();
    }

    fn set_fontset(&mut self, fontset: i32) {
        self.fontset = fontset;
    }
    fn display_info(&self) -> DisplayInfoRef {
        DisplayInfoRef::new(self.display_info as *mut _)
    }
}
