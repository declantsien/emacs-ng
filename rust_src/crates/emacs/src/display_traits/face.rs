use webrender_api::ColorF;
use webrender_api::LineStyle;

use crate::bindings::face as Face;
use crate::color::pixel_to_color;
use crate::font::FontRef;
use crate::lisp::ExternalPtr;

use super::FaceUnderlineType;

pub type FaceRef = ExternalPtr<Face>;

impl FaceRef {
    pub fn font(&self) -> Option<FontRef> {
        FontRef::new(self.font)
    }

    pub fn underline_type(&self) -> FaceUnderlineType {
        self.underline().into()
    }

    pub fn underline_style(&self) -> Option<LineStyle> {
        self.underline_type().into()
    }

    pub fn bg_color(&self) -> ColorF {
        pixel_to_color(self.background)
    }

    pub fn fg_color(&self) -> ColorF {
        pixel_to_color(self.foreground)
    }

    pub fn underline_color(&self) -> ColorF {
        pixel_to_color(self.underline_color)
    }

    pub fn overline_color(&self) -> ColorF {
        pixel_to_color(self.overline_color)
    }

    pub fn strike_through_color(&self) -> ColorF {
        pixel_to_color(self.strike_through_color)
    }
}
