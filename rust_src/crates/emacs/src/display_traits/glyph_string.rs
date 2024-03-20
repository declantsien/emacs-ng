use std::cmp::min;
use std::slice;

use webrender_api::ColorF;

use super::DrawGlyphsFace;
use super::EmacsGCRef;
use super::FaceRef;
use super::GlyphType;
use crate::bindings::composition_gstring_from_id;
use crate::bindings::composition_hash_table;
use crate::bindings::composition_method;
use crate::bindings::font_for_underline_metrics;
use crate::bindings::get_glyph_string_clip_rect;
use crate::bindings::glyph_row as GlyphRow;
use crate::bindings::glyph_string;
use crate::bindings::hash_hash_t;
use crate::bindings::hash_lookup_get_hash;
use crate::bindings::prepare_face_for_display;
use crate::bindings::Emacs_Rectangle as NativeRectangle;
use crate::bindings::XHASH_TABLE;
use crate::color::pixel_to_color;
use crate::definitions::EmacsInt;
use crate::display_traits::GlyphRef;
use crate::font::FontRef;
use crate::frame::FrameRef;
use crate::globals::Qnil;
use crate::lisp::ExternalPtr;
use crate::lisp::LispObject;

pub type XChar2b = u32;

pub type GlyphRowRef = ExternalPtr<GlyphRow>;

pub type GlyphStringRef = ExternalPtr<glyph_string>;
impl GlyphStringRef {
    pub fn glyph_type(&self) -> Option<GlyphType> {
        self.first_glyph().map(|g| g.type_().into())
    }

    pub fn hl(&self) -> DrawGlyphsFace {
        self.hl.into()
    }

    pub fn font(&self) -> Option<FontRef> {
        FontRef::new(self.font)
    }

    pub fn face(&self) -> Option<FaceRef> {
        FaceRef::new(self.face)
    }

    pub fn frame(&self) -> Option<FrameRef> {
        FrameRef::new(self.f)
    }

    pub fn font_for_underline_metrics(&self) -> Option<FontRef> {
        let font = unsafe { font_for_underline_metrics(self.clone().as_mut()) };
        FontRef::new(font)
    }

    pub fn clip_head(&self) -> Option<Self> {
        Self::new(self.clip_head)
    }
    pub fn clip_tail(&self) -> Option<Self> {
        Self::new(self.clip_tail)
    }
    pub fn prev(&self) -> Option<Self> {
        Self::new(self.prev)
    }
    pub fn next(&self) -> Option<Self> {
        Self::new(self.next)
    }

    pub fn is_for_overlaps(&self) -> bool {
        self.for_overlaps() != 0
    }

    pub fn row(&self) -> Option<GlyphRowRef> {
        GlyphRowRef::new(self.row)
    }

    pub fn gc(&self) -> Option<EmacsGCRef> {
        EmacsGCRef::new(self.gc)
    }

    pub fn prepare_face_for_display(&self) {
        unsafe { prepare_face_for_display(self.f, self.face) }
    }

    pub fn bg_color(&self) -> Option<ColorF> {
        self.gc().map(|gc| pixel_to_color(gc.background as u64))
    }

    pub fn fg_color(&self) -> Option<ColorF> {
        self.gc().map(|gc| pixel_to_color(gc.foreground as u64))
    }

    pub fn clip_rect(&mut self) -> NativeRectangle {
        let mut clip_rect = NativeRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };

        unsafe { get_glyph_string_clip_rect(self.as_mut(), &mut clip_rect) };
        clip_rect
    }

    pub fn underline_color(&self) -> Option<ColorF> {
        if self.face().map_or(false, |f| f.underline_defaulted_p()) {
            self.fg_color()
        } else {
            self.face().map(|f| f.underline_color())
        }
    }

    pub fn overline_color(&self) -> Option<ColorF> {
        if self
            .face()
            .map_or(false, |f| f.overline_color_defaulted_p())
        {
            self.fg_color()
        } else {
            self.face().map(|f| f.overline_color())
        }
    }

    pub fn strike_through_color(&self) -> Option<ColorF> {
        if self
            .face()
            .map_or(false, |f| f.strike_through_color_defaulted_p())
        {
            self.fg_color()
        } else {
            self.face().map(|f| f.strike_through_color())
        }
    }

    pub fn get_chars(&self) -> &[XChar2b] {
        let len = self.nchars as usize;

        unsafe { slice::from_raw_parts(self.char2b, len) }
    }

    pub fn first_glyph(&self) -> Option<GlyphRef> {
        GlyphRef::new(self.first_glyph)
    }

    pub fn composite_offsets(&self) -> &[i16] {
        let len = (self.nchars * 2) as usize;

        let offsets = unsafe { slice::from_raw_parts((*self.cmp).offsets, len) };

        let from = (self.cmp_from * 2) as usize;
        let to = min((self.cmp_to * 2) as usize, len);

        &offsets[from..to]
    }

    pub fn composite_chars(&self) -> &[XChar2b] {
        let from = self.cmp_from as usize;
        let to = min(self.cmp_to, self.nchars) as usize;

        &self.get_chars()[from..to]
    }

    pub fn composite_glyph(&self, n: usize) -> Option<EmacsInt> {
        let n = self.cmp_from as usize + n;

        let hash_table = unsafe { XHASH_TABLE(composition_hash_table) };

        let key_and_value = unsafe { *(*hash_table).key_and_value }.as_vector().unwrap();

        let key = unsafe { (*self.cmp).key };
        let hash_code: Box<hash_hash_t> = Box::new(0);
        let hash_index = unsafe { hash_lookup_get_hash(hash_table, key, Box::into_raw(hash_code)) };

        let composition_index = (hash_index * 2) as usize;
        let composition =
            unsafe { key_and_value.contents.as_slice(composition_index + 1) }[composition_index];
        if let Some(composition) = composition.as_vector() {
            let glyph_index = if unsafe { (*self.cmp).method }
                == composition_method::COMPOSITION_WITH_RULE_ALTCHARS
            {
                n * 2
            } else {
                n
            };

            let glyph = unsafe { composition.contents.as_slice(glyph_index + 1) }[glyph_index];

            Some(glyph.as_fixnum_or_error())
        } else {
            None
        }
    }

    pub fn is_automatic_composition(&self) -> bool {
        unsafe { (*self.first_glyph).u.cmp.automatic() }
    }
    pub fn get_lgstring(&self) -> LispObject {
        if !self.is_automatic_composition() {
            return Qnil;
        }
        unsafe { composition_gstring_from_id(self.cmp_id) }
    }
}

impl IntoIterator for GlyphStringRef {
    type Item = GlyphStringRef;
    type IntoIter = GlyphStringIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        GlyphStringIntoIterator { next: Some(self) }
    }
}

pub struct GlyphStringIntoIterator {
    next: Option<GlyphStringRef>,
}

impl Iterator for GlyphStringIntoIterator {
    type Item = GlyphStringRef;

    fn next(&mut self) -> Option<GlyphStringRef> {
        let new_next = self.next.and_then(|n| GlyphStringRef::new(n.next));

        let result = self.next;
        self.next = new_next;

        result
    }
}
