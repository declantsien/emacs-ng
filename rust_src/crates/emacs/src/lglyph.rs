use crate::{
    bindings::{
        composition_gstring_from_id, lglyph_indices, make_vector, pvec_type, Fcopy_sequence, ASET,
    },
    globals::Qnil,
    glyph::GlyphStringRef,
    lisp::LispObject,
    vector::LVector,
};

impl GlyphStringRef {
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

pub trait LGlyphString {
    fn is_lgstring(self) -> bool;
    fn header(self) -> LispObject;
    fn set_header(self, header: LispObject);
    fn font(self) -> LispObject;
    fn lchar(self, i: u32) -> LispObject;
    fn char(self, i: u32) -> u32;
    fn char_len(self) -> u32;
    fn shaped_p(self) -> bool;
    fn set_id(self, id: u32);
    fn lglyph(self, i: u32) -> LispObject;
    fn lglyph_len(self) -> u32;
    fn set_lglyph(self, i: u32, lglyph: LispObject);
}

impl LGlyphString for LispObject {
    fn is_lgstring(self) -> bool {
        self.as_vectorlike()
            .map_or(false, |v| v.is_pseudovector(pvec_type::PVEC_NORMAL_VECTOR))
    }
    fn header(self) -> LispObject {
        self.aref(0)
    }

    fn set_header(self, header: LispObject) {
        self.aset(0, header);
    }
    fn font(self) -> LispObject {
        self.header().aref(0)
    }
    fn lchar(self, i: u32) -> LispObject {
        self.header().aref(i + 1)
    }
    fn char(self, i: u32) -> u32 {
        self.lchar(i).into()
    }
    fn char_len(self) -> u32 {
        self.header().asize() - 1
    }
    fn shaped_p(self) -> bool {
        self.aref(1).into()
    }
    fn set_id(self, id: u32) {
        self.aset(1, id.into())
    }
    fn lglyph(self, i: u32) -> LispObject {
        self.aref(i + 2)
    }
    fn lglyph_len(self) -> u32 {
        self.asize() - 2
    }
    fn set_lglyph(self, i: u32, lglyph: LispObject) {
        self.aset(i + 2, lglyph)
    }
}

trait LGlyph {
    fn from_(self) -> u32;
    fn to(self) -> u32;
    fn char(self) -> u32;
    fn code(self) -> u32;
    fn width(self) -> u32;
    fn lbearing(self) -> u32;
    fn rbearing(self) -> u32;
    fn ascent(self) -> u32;
    fn descent(self) -> u32;
    fn adjustment(self) -> u32;
    fn set_from_to(self, from: u32, to: u32);
    fn set_char(self, c: u32);
    fn set_code(self, code: u32);
    fn set_width(self, width: u32);
    fn set_adjustment(self, xoff: Option<u32>, yoff: Option<u32>, wadjust: Option<u32>);
    // Return the shallow Copy of GLYPH.
    fn copy(self) -> Self;
}

impl LGlyph for LispObject {
    fn from_(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_FROM).into()
    }
    fn to(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_TO).into()
    }
    fn char(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_CHAR).into()
    }
    fn code(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_CODE).into()
    }
    fn width(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_WIDTH).into()
    }
    fn lbearing(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_LBEARING).into()
    }
    fn rbearing(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_RBEARING).into()
    }
    fn ascent(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_ASCENT).into()
    }
    fn descent(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_DESCENT).into()
    }
    fn adjustment(self) -> u32 {
        self.aref(lglyph_indices::LGLYPH_IX_ADJUSTMENT).into()
    }
    fn set_from_to(self, from: u32, to: u32) {
        self.aset(lglyph_indices::LGLYPH_IX_FROM, from.into());
        self.aset(lglyph_indices::LGLYPH_IX_TO, to.into());
    }
    fn set_char(self, c: u32) {
        self.aset(lglyph_indices::LGLYPH_IX_CHAR, c.into());
    }
    fn set_code(self, code: u32) {
        self.aset(lglyph_indices::LGLYPH_IX_CODE, code.into());
    }
    fn set_width(self, width: u32) {
        self.aset(lglyph_indices::LGLYPH_IX_WIDTH, width.into());
    }

    fn set_adjustment(self, xoff: Option<u32>, yoff: Option<u32>, wadjust: Option<u32>) {
        let result = unsafe { make_vector(3, Qnil) };
        unsafe {
            ASET(result, 0, xoff.unwrap_or(0).into());
            ASET(result, 1, yoff.unwrap_or(0).into());
            ASET(result, 2, wadjust.unwrap_or(0).into())
        };
        self.aset(lglyph_indices::LGLYPH_IX_ADJUSTMENT, result);
    }

    fn copy(self) -> Self {
        unsafe { Fcopy_sequence(self) }
    }
}
