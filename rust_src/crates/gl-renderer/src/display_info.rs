pub use emacs::display_info::DisplayInfoRef;
use libc;
use std::collections::HashMap;
use std::ptr;

use emacs::bindings::Emacs_GC;
use emacs::lisp::ExternalPtr;

use crate::fringe::FringeBitmap;
pub type FringeBitmapCaches = HashMap<i32, FringeBitmap>;

pub struct GlRendererData {
    pub scratch_cursor_gc: Box<Emacs_GC>,
    pub fringe_bitmap_caches: FringeBitmapCaches,
}

impl Default for GlRendererData {
    fn default() -> Self {
        GlRendererData {
            scratch_cursor_gc: Box::new(Emacs_GC {
                foreground: 0,
                background: 0,
            }),

            fringe_bitmap_caches: HashMap::new(),
        }
    }
}

pub type GlRendererDataRef = ExternalPtr<GlRendererData>;

pub trait DisplayInfoExtGlRenderer {
    fn init_gl_renderer_data(&mut self);
    fn gl_renderer_data(&mut self) -> Option<GlRendererDataRef>;
    fn free_gl_renderer_data(&mut self);
}

impl DisplayInfoExtGlRenderer for DisplayInfoRef {
    fn init_gl_renderer_data(&mut self) {
        let data = Box::new(GlRendererData::default());
        self.gl_renderer_data = Box::into_raw(data) as *mut libc::c_void;
    }

    fn gl_renderer_data(&mut self) -> Option<GlRendererDataRef> {
        if self.gl_renderer_data.is_null() {
            self.init_gl_renderer_data();
        }
        GlRendererDataRef::new(self.gl_renderer_data as *mut GlRendererData)
    }

    //FIXME this needs to be called somewhere.
    fn free_gl_renderer_data(&mut self) {
        if self.gl_renderer_data != ptr::null_mut() {
            unsafe {
                let _ = Box::from_raw(self.gl_renderer_data as *mut GlRendererData);
            }
        }
    }
}
