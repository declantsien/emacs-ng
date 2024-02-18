pub use crate::bindings::Display_Info as DisplayInfo;
use crate::{lisp::ExternalPtr, terminal::TerminalRef};

pub type DisplayInfoRef = ExternalPtr<DisplayInfo>;

impl DisplayInfoRef {
    pub fn terminal(&self) -> TerminalRef {
        return TerminalRef::new(self.terminal);
    }
}
