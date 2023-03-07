/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::mem;
use std::cmp::max;
use parking_lot::Mutex;
use api::{ColorU, GlyphDimensions, FontKey, FontRenderMode};
use api::{FontInstanceFlags, FontTemplate, NativeFontHandle};
use crate::rasterizer::{FontInstance, GlyphKey};
use crate::rasterizer::{GlyphFormat, GlyphRasterError, GlyphRasterResult, RasterizedGlyph};
use crate::types::FastHashMap;
use std::sync::{Arc};
use std::sync::OnceLock;

type FontHash = FontKey;
type RawTemplate = (Arc<Vec<u8>>, u32);
#[derive(Debug)]
struct CachedFont {
    hash: FontHash,
    data: RawTemplate,
    font: fontdue::Font,
}

// Maps a template to a cached font that may be used across all threads.
struct FontCache {
    fonts: FastHashMap<FontHash, Arc<CachedFont>>,
}

// Fontdue resources are safe to move between threads as long as they
// are not concurrently accessed. In our case, everything is behind a
// Mutex so it is safe to move them between threads.
// unsafe impl Send for CachedFont {}
unsafe impl Send for FontCache {}

static FONT_CACHE: OnceLock<Arc<Mutex<FontCache>>> = OnceLock::new();

impl FontCache {
    fn new() -> Self {
        FontCache {
            fonts: FastHashMap::default(),
        }
    }

    pub fn global() -> &'static Mutex<FontCache> {
        FONT_CACHE.get_or_init(|| {
            log::trace!("font cache is being created...");
            Arc::new(Mutex::new(Self::new()))
        })
    }

    fn cache_mut<P, T>(p: P) -> Option<T>
    where
        P: FnOnce(&mut FontCache) -> T,
    {
        match Self::global().try_lock() {
            Some(mut cache) => Some(p(&mut cache)),
            None => {
                error!("font cache not available...");
                None
            }
        }
    }

    pub fn with_font<P, T>(font_key: FontKey, font_template: FontTemplate, p: P) -> Option<T>
    where
        P: FnOnce(Arc<CachedFont>) -> T,
    {
        let hash = font_key;

        FontCache::cache_mut(|cache| {
            if let Some(cached) = cache.fonts.get(&hash) {
                return p(cached.clone());
            }

            let (bytes, index) = match font_template {
                FontTemplate::Raw(ref bytes, index) => (bytes.clone(), index),
                FontTemplate::Native(NativeFontHandle { ref path, index }) => {
                    match std::fs::read(&path) {
                        Ok(bytes) => (Arc::new(bytes), index),
                        Err(result) => {
                            panic!(
                                "adding native font failed: file={} err={:?}",
                                path.to_string_lossy().into_owned(),
                                result
                            );
                        }
                    }
                }
            };

            let settings = fontdue::FontSettings {
                collection_index: index,
                ..fontdue::FontSettings::default()
            };

            let cached = match fontdue::Font::from_bytes(bytes.as_slice(), settings) {
                Ok(font) => Arc::new(CachedFont {
                    hash: hash.clone(),
                    data: (bytes, index),
                    font,
                }),
                Err(e) => {
                    panic!(
                        "Faile to create fontdue instance: scale={} collection_index={} err={:?}",
                        settings.scale, settings.collection_index, e
                    );
                }
            };
            cache.fonts.insert(hash, cached.clone());
            p(cached)
        })
    }

    fn delete_font(cached: Arc<CachedFont>) {
        FontCache::cache_mut(|cache| {
            cache.fonts.remove(&cached.hash);
        });
    }
}

impl Drop for FontCache {
    fn drop(&mut self) {
        self.fonts.clear();
    }
}

pub struct FontContext {
    fonts: FastHashMap<FontHash, Arc<CachedFont>>,
}

impl FontContext {
    pub fn distribute_across_threads() -> bool {
        true
    }

    pub fn new() -> FontContext {
        FontContext {
            fonts: FastHashMap::default(),
        }
    }

    pub fn add_raw_font(&mut self, font_key: &FontKey, bytes: Arc<Vec<u8>>, index: u32) {
        let cached =
            FontCache::with_font(*font_key, FontTemplate::Raw(bytes, index), |cached| cached);
        if let Some(cached) = cached {
            self.fonts.entry(*font_key).or_insert_with(|| cached);
        }
    }

    pub fn add_native_font(&mut self, font_key: &FontKey, native_font_handle: NativeFontHandle) {
        let cached = FontCache::with_font(
            *font_key,
            FontTemplate::Native(native_font_handle),
            |cached| cached,
        );
        if let Some(cached) = cached {
            self.fonts.entry(*font_key).or_insert_with(|| cached);
        }
    }

    pub fn delete_font(&mut self, font_key: &FontKey) {
        if let Some(cached) = self.fonts.remove(font_key) {
            // If the only references to this font are the FontCache and this FontContext,
            // then delete the font as there are no other existing users.
            if Arc::strong_count(&cached) <= 2 {
                FontCache::delete_font(cached);
            }
        }
    }

    pub fn delete_font_instance(&mut self, _: &FontInstance) {}

    pub fn get_glyph_index(&self, font_key: FontKey, ch: char) -> Option<u32> {
        let rasterizer = self.fonts.get(&font_key);
        if rasterizer.is_none() {
            return None;
        }

        let rasterizer = rasterizer.unwrap();
        let index = rasterizer.font.lookup_glyph_index(ch);
        if index == 0 {
            None
        } else {
            Some(index as u32)
        }
    }

    pub fn get_glyph_dimensions(
        &mut self,
        font: &FontInstance,
        key: &GlyphKey,
    ) -> Option<GlyphDimensions> {
        let rasterizer = self.fonts.get(&font.font_key);
        if rasterizer.is_none() {
            return None;
        }

        let rasterizer = rasterizer.unwrap();
        let glyph = key.index() as u16;
        let size = font.size.to_f32_px();
        let metrics = rasterizer.font.metrics_indexed(glyph, size);

        if metrics.width == 0 || metrics.height == 0 {
            None
        } else {
            Some(GlyphDimensions {
                left: metrics.xmin as i32,
                top: metrics.ymin as i32,
                width: metrics.width as i32,
                height: metrics.height as i32,
                advance: metrics.advance_width,
            })
        }
    }

    pub fn prepare_font(font: &mut FontInstance) {
        match font.render_mode {
            FontRenderMode::Mono => {
                // In mono mode the color of the font is irrelevant.
                font.color = ColorU::new(0xFF, 0xFF, 0xFF, 0xFF);
                // Subpixel positioning is disabled in mono mode.
                font.disable_subpixel_position();
            }
            FontRenderMode::Alpha | FontRenderMode::Subpixel => {
                // We don't do any preblending with FreeType currently, so the color is not used.
                font.color = ColorU::new(0xFF, 0xFF, 0xFF, 0xFF);
            }
        }
    }

    pub fn begin_rasterize(_: &FontInstance) {}

    pub fn end_rasterize(_: &FontInstance) {}

    pub fn rasterize_glyph(&mut self, font: &FontInstance, key: &GlyphKey) -> GlyphRasterResult {
        log::trace!("rasterize_glyph");
        let rasterizer = self.fonts.get(&font.font_key);
        if rasterizer.is_none() {
            return Err(GlyphRasterError::LoadFailed);
        }

        let rasterizer = rasterizer.unwrap();

        let render_mode = font.render_mode;
        let size = font.size.to_f32_px();

        let glyph = key.index() as u16;

        let (metrics, mut bitmap) = if render_mode == FontRenderMode::Subpixel {
            rasterizer
                .font
                .rasterize_indexed_subpixel(glyph, size as f32)
        } else {
            rasterizer.font.rasterize_indexed(glyph, size as f32)
        };

        debug!(
            "Rasterizing {:?} as {:?} with dimensions {:?}",
            key, render_mode, metrics
        );

        let mut gbra8_pixels: Vec<u8> = Vec::new();

        if metrics.width == 0 || metrics.height == 0 {
            if let Some((mut pixmap, x, y)) = glyph_using_svg_or_raster(
                &rasterizer.data,
                rustybuzz::ttf_parser::GlyphId(glyph as u16),
                size,
            ) {
                for src in pixmap.data_mut().iter_mut().collect::<Vec<_>>().chunks(4) {
                    let (r, g, b, a) = (*src[0], *src[1], *src[2], *src[3]);
                    gbra8_pixels.push(b); // u8
                    gbra8_pixels.push(g); // u8
                    gbra8_pixels.push(r); // u8
                    gbra8_pixels.push(a); // u8
                }

                let scale = size / max(pixmap.width(), pixmap.height()) as f32;

                let top = pixmap.height() as f32 + y;
                return Ok(RasterizedGlyph {
                    left: x,
                    top,
                    width: pixmap.width() as i32,
                    height: pixmap.height() as i32,
                    scale,
                    format: GlyphFormat::ColorBitmap,
                    bytes: gbra8_pixels,
                });
            } else {
                return Err(GlyphRasterError::LoadFailed);
            }
        } else {
            let format = match render_mode {
                FontRenderMode::Subpixel => {
                    let subpixel_bgr = font.flags.contains(FontInstanceFlags::SUBPIXEL_BGR);
                    for src in bitmap.iter_mut().collect::<Vec<_>>().chunks(3) {
                        let (mut r, g, mut b) = (*src[0], *src[1], *src[2]);
                        if subpixel_bgr {
                            mem::swap(&mut r, &mut b);
                        }
                        gbra8_pixels.push(b); // u8
                        gbra8_pixels.push(g); // u8
                        gbra8_pixels.push(r); // u8
                        gbra8_pixels.push(max(max(b, g), r)); // u8
                    }
                    GlyphFormat::Subpixel
                }
                _ => {
                    for pixel in bitmap.iter_mut() {
                        let alpha = *pixel;
                        gbra8_pixels.push(alpha); // u8
                        gbra8_pixels.push(alpha); // u8
                        gbra8_pixels.push(alpha); // u8
                        gbra8_pixels.push(alpha); // u8
                    }
                    GlyphFormat::Bitmap
                }
            };
            let top = metrics.height as f32 + metrics.ymin as f32;
            return Ok(RasterizedGlyph {
                left: metrics.xmin as f32,
                top,
                width: metrics.width as i32,
                height: metrics.height as i32,
                scale: 1.0,
                format,
                bytes: gbra8_pixels,
            });
        }
    }
}

fn glyph_using_svg_or_raster(
    (bytes, index): &RawTemplate,
    glyph_id: rustybuzz::ttf_parser::GlyphId,
    size: f32,
) -> Option<(tiny_skia::Pixmap, f32, f32)> {
    let face = rustybuzz::Face::from_slice(bytes.as_slice(), *index);

    if face.is_none() {
        return None;
    }

    let face = face.unwrap();

    if let Some(svg_data) = face.glyph_svg_image(glyph_id) {
        let opts = usvg::Options {
            ..usvg::Options::default()
        };
        let result = usvg::Tree::from_data(svg_data, &opts);
        let tree = match result {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to parse svg {e:?}");
                return None;
            }
        };

        let pixmap_size = tree.size.to_screen_size();
        let result = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height());
        if result.is_none() {
            error!("Failed to create tiny_skia pixmap");
            return None;
        }
        let mut pixmap = result.unwrap();
        match resvg::render(
            &tree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        ) {
            None => {
                error!("Failed to render svg using resvg");
                return None;
            }
            _ => {}
        }

        debug!("Glyph using svg: {:?}", glyph_id);
        return Some((pixmap, 0.0, 0.0));
    } else if let Some(raster) = face.glyph_raster_image(glyph_id, size as u16) {
        match tiny_skia::Pixmap::decode_png(raster.data) {
            Ok(pixmap) => {
                debug!("Glyph using raster: {:?}", glyph_id);
                return Some((pixmap, raster.x as f32, raster.y as f32));
            }
            Err(e) => {
                error!("Pixmap decode png error {e:?}");
                return None;
            }
        }
    }
    return None;
}
