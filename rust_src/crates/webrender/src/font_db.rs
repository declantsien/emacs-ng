#[cfg(all(unix, not(target_os = "macos")))]
use font_loader::system_fonts;
use fontdb::{FaceInfo, Family, Query, Stretch, Style, Weight};

use std::str;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FontDescriptor {
    PostScript(String),
    Properties {
        family: String,
        weight: Weight,
        slant: Style,
        stretch: Stretch,
    },
}

pub struct FontDB {
    pub db: fontdb::Database,
}

impl FontDB {
    pub fn new() -> FontDB {
        let mut db = fontdb::Database::new();

        #[cfg(not(all(unix, not(target_os = "macos"))))]
        db.load_system_fonts();

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let dirs = system_fonts::get_font_dirs();
            for dir in &dirs {
                log::trace!("Load fonts dir: {:?}", dir);
                db.load_fonts_dir(dir);
            }

            if let Some(serif) = Self::query_generic("serif") {
                db.set_serif_family(serif);
            };

            if let Some(sans_serif) = Self::query_generic("sans serif") {
                db.set_serif_family(sans_serif);
            };

            if let Some(monospace) = Self::query_generic("monospace") {
                db.set_monospace_family(monospace);
            };

            if let Some(cursive) = Self::query_generic("cursive") {
                db.set_cursive_family(cursive);
            };

            if let Some(fantasy) = Self::query_generic("fantasy") {
                db.set_cursive_family(fantasy);
            };
        }

        FontDB { db }
    }

    pub fn select_postscript(&self, postscript_name: &str) -> Option<&FaceInfo> {
        self.db
            .faces()
            .iter()
            .filter(|f| f.post_script_name == postscript_name)
            .next()
    }

    pub fn query(&self, query: &Query<'_>) -> Option<&FaceInfo> {
        self.db.query(query).and_then(|id| self.db.face(id))
    }

    pub fn all_fonts(&self) -> Vec<&FaceInfo> {
        self.db.faces().iter().collect::<Vec<&FaceInfo>>()
    }

    pub fn fonts_by_family(&self, family: &Family) -> Vec<&FaceInfo> {
        Some(self.db.family_name(family))
            .map(|name| {
                self.db
                    .faces()
                    .iter()
                    .filter(|face| {
                        face.families
                            .iter()
                            .find(|family| family.0 == name)
                            .is_some()
                    })
                    .collect::<Vec<&FaceInfo>>()
            })
            .unwrap_or_else(|| Vec::new())
    }

    pub fn family_name(family_name: &str) -> Family {
        match family_name.clone().to_lowercase().as_str() {
            "serif" => Family::Serif,
            "sans-serif" => Family::SansSerif,
            "sans serif" => Family::SansSerif,
            "monospace" => Family::Monospace,
            "cursive" => Family::Cursive,
            "fantasy" => Family::Fantasy,
            _ => Family::Name(family_name),
        }
    }

    pub fn font_from_desc(&self, desc: FontDescriptor) -> Option<&FaceInfo> {
        // println!("desc: {:?}", desc);
        match desc {
            FontDescriptor::PostScript(ref name) => self.select_postscript(name),
            FontDescriptor::Properties {
                ref family,
                weight,
                slant,
                stretch,
            } => self.query(&Query {
                families: &[Self::family_name(family)],
                stretch,
                weight,
                style: slant,
            }),
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn query_generic(name: &str) -> Option<String> {
        let property = system_fonts::FontPropertyBuilder::new()
            .family(name)
            .build();
        if let Some((_, _, family_name)) = system_fonts::get(&property) {
            log::trace!("Query: {} Name: {}", name, family_name,);
            return Some(family_name.to_owned());
        }
        log::trace!("Query: {} not found", name);
        None
    }
}
