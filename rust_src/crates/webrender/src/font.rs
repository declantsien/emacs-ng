use std::collections::HashMap;
use std::sync::OnceLock;

use std::ffi::CString;
use std::ptr;

use fontdb::{Stretch, Style, Weight};
use std::str;

use webrender::api::*;

use emacs::{
    bindings::{
        font, font_driver, font_make_entity, font_make_object, font_metrics, font_property_index,
        font_style_to_value, frame, glyph_string, intern, register_font_driver, Fassoc, Fcdr,
        Fcons, Fmake_symbol, Fnreverse, Fprovide, FONT_INVALID_CODE,
    },
    frame::LispFrameRef,
    globals::{
        Qbold, Qextra_bold, Qextra_light, Qiso10646_1, Qitalic, Qlight, Qmedium, Qnil, Qnormal,
        Qoblique, Qsemi_bold, Qthin, Qttfp, Qultra_bold,
    },
    lisp::{ExternalPtr, LispObject},
    symbol::LispSymbolRef,
};

use crate::{font_db::FontDB, font_db::FontDescriptor, frame::LispFrameExt};

static mut FONT_DB: OnceLock<FontDB> = OnceLock::new();
impl FontDB<'static> {
    pub fn ensure() -> &'static FontDB<'static> {
        unsafe {
            FONT_DB.get_or_init(|| {
                log::trace!("font_db is being created...");

                Self::new()
            })
        }
    }
    pub fn global() -> &'static mut FontDB<'static> {
        let _ = Self::ensure();
        unsafe { FONT_DB.get_mut().unwrap() }
    }
}
pub type FontRef = ExternalPtr<font>;

pub struct FontDriver(pub font_driver);
unsafe impl Sync for FontDriver {}

static FONT_DRIVER: OnceLock<FontDriver> = OnceLock::new();
impl FontDriver {
    fn global() -> &'static FontDriver {
        FONT_DRIVER.get_or_init(|| {
            log::trace!("FONT_DRIVER is being created...");
            let mut font_driver = font_driver::default();

            font_driver.type_ = Qttfp;
            font_driver.case_sensitive = true;
            font_driver.get_cache = Some(get_cache);
            font_driver.list = Some(list);
            font_driver.match_ = Some(match_);
            font_driver.list_family = Some(list_family);
            font_driver.open_font = Some(open_font);
            font_driver.close_font = Some(close_font);
            font_driver.encode_char = Some(encode_char);
            font_driver.has_char = Some(has_char);
            font_driver.text_extents = Some(text_extents);
            font_driver.draw = Some(draw);

            FontDriver(font_driver)
        })
    }
}

/// A newtype for objects we know are font_spec.
#[derive(Clone, Copy)]
pub struct LispFontLike(LispObject);

impl LispFontLike {
    fn aref(&self, index: font_property_index::Type) -> LispObject {
        let vl = self.0.as_vectorlike().unwrap();
        let v = unsafe { vl.as_vector_unchecked() };
        unsafe { v.get_unchecked(index as usize) }
    }

    fn get_family(&self) -> Option<String> {
        let tem = self.aref(font_property_index::FONT_FAMILY_INDEX);

        if tem.is_nil() {
            None
        } else {
            let string: String = tem.into();
            let family_name = string.to_string().replace("-", "\\-");

            #[cfg(free_unix)]
            let family_name = FontDB::fc_family_name(&family_name);

            return Some(family_name);
        }
    }

    fn get_postscript_name(&self) -> Option<String> {
        // Get postscript name form font_entity.
        let font_extra = self.aref(font_property_index::FONT_EXTRA_INDEX);

        let val = unsafe { Fassoc(":postscript-name".into(), font_extra, Qnil) };

        if val.is_nil() {
            return None;
        }

        let postscript_name = unsafe { Fcdr(val) }.as_string().unwrap().to_string();
        Some(postscript_name)
    }

    fn get_slant(&self) -> Option<Style> {
        let slant = self.aref(font_property_index::FONT_SLANT_INDEX);

        if slant.is_nil() {
            None
        } else {
            let string: String = slant.into();
            match string.as_ref() {
                "Qnormal" => Some(Style::Normal),
                "Qitalic" => Some(Style::Italic),
                "Qoblique" => Some(Style::Oblique),
                _ => Some(Style::Normal),
            }
        }
    }

    fn get_descriptor(&self) -> Option<FontDescriptor> {
        // Get postscript name form font_entity.
        let font_extra = self.aref(font_property_index::FONT_EXTRA_INDEX);

        let val = unsafe { Fassoc(":postscript-name".into(), font_extra, Qnil) };

        if val.is_nil() {
            if let Some(family) = self.get_family() {
                let slant = self.get_slant().unwrap_or(Style::Normal);

                return Some(FontDescriptor::Properties {
                    family,
                    weight: Weight::NORMAL,
                    slant,
                    stretch: Stretch::Normal,
                });
            };
            return None;
        } else {
            let postscript_name = unsafe { Fcdr(val) }.as_string().unwrap().to_string();
            Some(FontDescriptor::PostScript(postscript_name))
        }
    }

    fn aset(&self, index: font_property_index::Type, val: LispObject) {
        let vl = self.0.as_vectorlike().unwrap();
        let mut v = unsafe { vl.as_vector_unchecked() };
        unsafe { v.set_unchecked(index as usize, val) };
    }

    fn set_style(&self, index: font_property_index::Type, val: LispObject) {
        let value = unsafe { font_style_to_value(index, val, true) };

        self.aset(index, LispObject::from(value));
    }

    fn as_lisp_object(self) -> LispObject {
        self.0
    }
}

impl From<LispObject> for LispFontLike {
    fn from(v: LispObject) -> LispFontLike {
        LispFontLike(v)
    }
}

extern "C" fn get_cache(f: *mut frame) -> LispObject {
    let frame = LispFrameRef::new(f);
    let mut dpyinfo = frame.display_info();

    dpyinfo.get_raw().name_list_element
}

extern "C" fn draw(
    _s: *mut glyph_string,
    _from: i32,
    _to: i32,
    _x: i32,
    _y: i32,
    _with_background: bool,
) -> i32 {
    0
}

extern "C" fn list(frame: *mut frame, font_spec: LispObject) -> LispObject {
    // FIXME: implment the real list in future
    match_(frame, font_spec)
}

extern "C" fn match_(_f: *mut frame, spec: LispObject) -> LispObject {
    // let now = std::time::Instant::now();

    let font_spec = LispFontLike(spec);

    let family = font_spec.get_family();
    let font_db = FontDB::global();

    let fonts = if let Some(family) = family {
        let family = FontDB::family_name(&family);
        font_db.fonts_by_family(&family)
    } else {
        font_db.all_fonts()
    };

    let mut list = Qnil;

    for f in fonts {
        let entity: LispFontLike = unsafe { font_make_entity() }.into();

        // set type
        entity.aset(font_property_index::FONT_TYPE_INDEX, Qttfp);

        let family_name = f.families.get(0);
        if family_name.is_none() {
            continue;
        }
        let (family_name, _) = family_name.unwrap();
        let family_name = family_name.replace("\u{0}", "");
        let family_name: &str = &family_name;
        // set family
        entity.aset(font_property_index::FONT_FAMILY_INDEX, unsafe {
            Fmake_symbol(LispObject::from(family_name))
        });

        let weight = f.weight;

        let weight = if weight <= Weight::THIN {
            Qthin
        } else if weight <= Weight::EXTRA_LIGHT {
            Qextra_light
        } else if weight <= Weight::LIGHT {
            Qlight
        } else if weight <= Weight::NORMAL {
            Qnormal
        } else if weight <= Weight::MEDIUM {
            Qmedium
        } else if weight <= Weight::SEMIBOLD {
            Qsemi_bold
        } else if weight <= Weight::BOLD {
            Qbold
        } else if weight <= Weight::EXTRA_BOLD {
            Qextra_bold
        } else if weight <= Weight::BLACK {
            Qultra_bold
        } else {
            Qultra_bold
        };

        // set weight
        entity.set_style(font_property_index::FONT_WEIGHT_INDEX, weight);

        let slant = match f.style {
            Style::Normal => Qnormal,
            Style::Italic => Qitalic,
            Style::Oblique => Qoblique,
        };

        // set slant
        entity.set_style(font_property_index::FONT_SLANT_INDEX, slant);

        // set width
        entity.set_style(font_property_index::FONT_WIDTH_INDEX, (0 as usize).into());

        // set size
        entity.aset(font_property_index::FONT_SIZE_INDEX, (0 as usize).into());

        // set registry
        entity.aset(font_property_index::FONT_REGISTRY_INDEX, Qiso10646_1);

        let postscript_name = f.post_script_name.replace("\u{0}", "");
        let postscript_name: &str = &postscript_name;
        // set name
        entity.aset(font_property_index::FONT_EXTRA_INDEX, unsafe {
            Fcons(
                Fcons(":postscript-name".into(), LispObject::from(postscript_name)),
                Qnil,
            )
        });

        list = unsafe { Fcons(entity.as_lisp_object(), list) }
    }

    unsafe { Fnreverse(list) }
}

extern "C" fn list_family(_f: *mut frame) -> LispObject {
    let mut list = Qnil;
    let font_db = FontDB::global();

    for font in font_db.all_fonts() {
        let app_locale = fontdb::Language::English_UnitedStates;
        if let Some((family_name, _)) = &font
            .families
            .iter()
            .find(|family| family.1 == app_locale)
            .or(font.families.get(0))
        {
            let f = family_name.replace('\'', "").trim().to_string();
            list = LispObject::cons(unsafe { intern(f.as_ptr() as *const ::libc::c_char) }, list);
        }
    }

    list
}

#[repr(C)]
pub struct WRFont {
    // extend basic font
    pub font: font,

    pub scale: f32,

    pub face_index: u32,

    pub face_id: fontdb::ID,

    pub instance_keys: HashMap<u64, FontInstanceKey>,
}

impl WRFont {
    pub fn glyph_for_char(&self, character: char) -> Option<u32> {
        if let Some(font) = FontDB::global().get_font(self.face_id) {
            return font.face.glyph_index(character).map(|c| c.0 as u32);
        }
        None
    }

    pub fn get_glyph_advance_width(&self, glyph_indices: Vec<GlyphIndex>) -> Vec<Option<i32>> {
        if let Some(font) = FontDB::global().get_font(self.face_id) {
            return glyph_indices
                .into_iter()
                .map(|i| {
                    font.face
                        .glyph_hor_advance(ttf_parser::GlyphId(i as u16))
                        .map(|a| (a as f32 * self.scale).round() as i32)
                })
                .collect();
        }
        Vec::new()
    }
    pub fn too_high_p(&self) -> bool {
        let pixel_size = self.font.pixel_size;
        let ascent = self.font.ascent;
        let descent = self.font.descent;

        pixel_size > 0 && (ascent + descent) > 3 * pixel_size
    }
}

pub type WRFontRef = ExternalPtr<WRFont>;

extern "C" fn open_font(frame: *mut frame, font_entity: LispObject, pixel_size: i32) -> LispObject {
    log::trace!("open font: {:?}", pixel_size);
    let font_entity: LispFontLike = font_entity.into();
    let desc = font_entity.get_descriptor();
    if desc.is_none() {
        return Qnil;
    }
    let desc = desc.unwrap();

    let frame: LispFrameRef = frame.into();

    let pixel_size = if pixel_size == 0 {
        // pixel_size here reflects to DPR 1 for webrender display, we have scale_factor from winit.
        // while pgtk/ns/w32 reflects to actual DPR on device by setting resx/resy to display
        if !frame.font().is_null() {
            frame.font().pixel_size as i64
        } else {
            // fallback font size
            16
        }
    } else {
        // prefer elisp specific font size
        pixel_size as i64
    };

    let font_object: LispFontLike = unsafe {
        font_make_object(
            vecsize!(WRFont) as i32,
            font_entity.as_lisp_object(),
            pixel_size as i32,
        )
    }
    .into();

    // set type
    font_object.aset(font_property_index::FONT_TYPE_INDEX, Qttfp);

    // set name
    font_object.aset(
        font_property_index::FONT_NAME_INDEX,
        LispSymbolRef::from(font_entity.aref(font_property_index::FONT_FAMILY_INDEX)).symbol_name(),
    );

    let mut wr_font = WRFontRef::new(
        font_object
            .as_lisp_object()
            .as_font()
            .unwrap()
            .as_font_mut() as *mut WRFont,
    );
    let font_db = FontDB::global();
    let font_result = font_db.get_font_matches(desc.clone());

    if font_result.is_none() {
        return Qnil;
    }

    let font_result = font_result.unwrap();
    wr_font.face_id = font_result.info.id;

    let face = &font_result.face;
    let units_per_em = face.units_per_em();
    let underline_metrics = face.underline_metrics().unwrap();
    let ascent = face.ascender();
    let descent = face.descender();
    let average_width = face.glyph_hor_advance(ttf_parser::GlyphId(0)).unwrap();

    let scale = pixel_size as f32 / units_per_em as f32;
    wr_font.scale = scale;

    wr_font.font.pixel_size = pixel_size as i32;
    wr_font.font.average_width = (average_width as f32 * scale) as i32;
    wr_font.font.ascent = (scale * ascent as f32).round() as i32;
    wr_font.font.descent = (-scale * descent as f32).round() as i32;
    wr_font.font.space_width = wr_font.font.average_width;
    wr_font.font.max_width = wr_font.font.average_width;
    wr_font.font.underline_thickness = (scale * underline_metrics.thickness as f32) as i32;
    wr_font.font.underline_position = (scale * underline_metrics.position as f32) as i32;

    wr_font.font.height = (scale * (ascent - descent) as f32).round() as i32;
    wr_font.font.baseline_offset = 0;

    let driver = FontDriver::global();
    wr_font.font.driver = &driver.0;

    log::trace!("open font done: {:?}", pixel_size);
    font_object.as_lisp_object()
}

extern "C" fn close_font(_font: *mut font) {}

extern "C" fn encode_char(font: *mut font, c: i32) -> u32 {
    let font = WRFontRef::new(font as *mut WRFont);

    std::char::from_u32(c as u32)
        .and_then(|c| font.glyph_for_char(c))
        .unwrap_or(FONT_INVALID_CODE)
}

extern "C" fn has_char(font: LispObject, c: i32) -> i32 {
    if font.is_font_entity() {
        let font_entity: LispFontLike = font.into();
        let postscript_name = font_entity.get_postscript_name();

        if postscript_name.is_none() {
            return -1;
        }

        let postscript_name = postscript_name.unwrap();

        let c = std::char::from_u32(c as u32);

        if c.is_none() {
            return 0;
        }

        let c = c.unwrap();
        let font_db = FontDB::global();
        font_db
            .select_postscript(&postscript_name)
            .cloned()
            .and_then(|face_info| {
                if let Some(font) = font_db.get_font(face_info.id) {
                    return font.face.glyph_index(c);
                }
                None
            })
            .is_some() as i32
    } else {
        let font = font.as_font().unwrap().as_font_mut();
        (encode_char(font, c) != FONT_INVALID_CODE) as i32
    }
}

#[allow(unused_variables)]
extern "C" fn text_extents(
    font: *mut font,
    code: *const u32,
    nglyphs: i32,
    metrics: *mut font_metrics,
) {
    let font = WRFontRef::new(font as *mut WRFont);

    let glyph_indices: Vec<u32> = unsafe { std::slice::from_raw_parts(code, nglyphs as usize) }
        .iter()
        .copied()
        .collect();

    let width: i32 = font
        .get_glyph_advance_width(glyph_indices.clone())
        .into_iter()
        .filter_map(|w| w)
        .sum();

    unsafe {
        (*metrics).lbearing = 0;
        (*metrics).rbearing = width as i16;
        (*metrics).width = width as i16;
        (*metrics).ascent = font.font.ascent as i16;
        (*metrics).descent = font.font.descent as i16;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn register_ttfp_font_driver(f: *mut frame) {
    let driver = FontDriver::global();
    unsafe {
        register_font_driver(&driver.0, f);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn syms_of_ttfp_font() {
    let ttfp_symbol =
        CString::new("ttfp").expect("Failed to create string for intern function call");
    def_lisp_sym!(Qttfp, "ttfp");
    unsafe {
        Fprovide(emacs::bindings::intern_c_string(ttfp_symbol.as_ptr()), Qnil);
    }

    register_ttfp_font_driver(ptr::null_mut());
}
