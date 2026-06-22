//! Declarative icon-font manifest: stable name ↔ PUA codepoint ↔ static SVG (LR7).

use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Component, Path, PathBuf},
};

use serde::Deserialize;

use crate::{
    atlas::GlyphAtlasCore,
    font::TypefaceError,
    icons::{IconError, IconSet},
};

/// Reserved PUA range and icon entries loaded from RON.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IconManifest {
    pub reserved_range_start: u32,
    pub reserved_range_end: u32,
    pub icons: Vec<IconManifestEntry>,
}

/// One manifest row: stable name, PUA codepoint, relative static SVG path.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct IconManifestEntry {
    pub name: String,
    pub codepoint: u32,
    pub svg_path: String,
}

/// Stable tables produced by manifest bake (import/staging only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconManifestBake {
    pub name_to_codepoint: BTreeMap<String, u32>,
    pub codepoint_to_name: BTreeMap<u32, String>,
    pub baked_codepoints: Vec<u32>,
}

impl IconManifest {
    /// Normalize icon order by codepoint for deterministic bake and golden tables.
    pub fn normalized_icons(&self) -> Vec<IconManifestEntry> {
        let mut icons = self.icons.clone();
        icons.sort_by_key(|entry| entry.codepoint);
        icons
    }

    fn validate(&self, manifest_dir: &Path) -> Result<(), TypefaceError> {
        if self.reserved_range_start > self.reserved_range_end {
            return Err(TypefaceError::Manifest(
                "reserved_range_start must be <= reserved_range_end".into(),
            ));
        }

        let mut names = HashSet::new();
        let mut codepoints = HashSet::new();
        for entry in &self.icons {
            if !names.insert(entry.name.clone()) {
                return Err(TypefaceError::Manifest(format!(
                    "duplicate manifest name `{}`",
                    entry.name
                )));
            }
            if !codepoints.insert(entry.codepoint) {
                return Err(TypefaceError::Manifest(format!(
                    "duplicate manifest codepoint {:#x}",
                    entry.codepoint
                )));
            }
            if entry.codepoint < self.reserved_range_start
                || entry.codepoint > self.reserved_range_end
            {
                return Err(TypefaceError::Manifest(format!(
                    "codepoint {:#x} outside reserved range {:#x}..={:#x}",
                    entry.codepoint, self.reserved_range_start, self.reserved_range_end
                )));
            }
            let svg_path = resolve_svg_path(manifest_dir, &entry.svg_path)?;
            if !svg_path.is_file() {
                return Err(TypefaceError::Manifest(format!(
                    "missing SVG path `{}` for icon `{}`",
                    entry.svg_path, entry.name
                )));
            }
        }
        Ok(())
    }
}

/// Load and validate a manifest from disk. Paths resolve relative to the manifest file directory.
pub fn load_icon_manifest(path: impl AsRef<Path>) -> Result<IconManifest, TypefaceError> {
    let path = path.as_ref();
    let bytes = fs::read_to_string(path)
        .map_err(|err| TypefaceError::Manifest(format!("read {}: {err}", path.display())))?;
    let manifest: IconManifest = ron::from_str(&bytes)
        .map_err(|err| TypefaceError::Manifest(format!("parse {}: {err}", path.display())))?;
    let manifest_dir = path
        .parent()
        .ok_or_else(|| TypefaceError::Manifest("manifest path has no parent directory".into()))?;
    manifest.validate(manifest_dir)?;
    Ok(manifest)
}

/// Bake all manifest icons into `IconSet` + raster atlas. Import/staging only — no draw-loop SVG reads.
pub fn bake_icon_manifest(
    manifest_path: impl AsRef<Path>,
    icons: &mut IconSet,
    raster_atlas: &mut GlyphAtlasCore,
    px: f32,
) -> Result<IconManifestBake, TypefaceError> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_icon_manifest(manifest_path)?;
    let manifest_dir = manifest_path
        .parent()
        .ok_or_else(|| TypefaceError::Manifest("manifest path has no parent directory".into()))?;

    let mut name_to_codepoint = BTreeMap::new();
    let mut codepoint_to_name = BTreeMap::new();
    let mut baked_codepoints = Vec::new();

    for entry in manifest.normalized_icons() {
        let svg_path = resolve_svg_path(manifest_dir, &entry.svg_path)?;
        let svg = fs::read_to_string(&svg_path).map_err(|err| {
            TypefaceError::Manifest(format!("read SVG {}: {err}", svg_path.display()))
        })?;
        icons
            .register_svg(entry.codepoint, &svg, px, raster_atlas)
            .map_err(manifest_icon_error)?;
        name_to_codepoint.insert(entry.name.clone(), entry.codepoint);
        codepoint_to_name.insert(entry.codepoint, entry.name.clone());
        baked_codepoints.push(entry.codepoint);
    }

    Ok(IconManifestBake {
        name_to_codepoint,
        codepoint_to_name,
        baked_codepoints,
    })
}

/// Path to the LR7 fixture manifest shipped with this crate.
pub fn fixture_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/typeface/icons/manifest.ron")
}

fn resolve_svg_path(manifest_dir: &Path, svg_path: &str) -> Result<PathBuf, TypefaceError> {
    if svg_path.is_empty() {
        return Err(TypefaceError::Manifest("empty svg_path".into()));
    }
    let rel = Path::new(svg_path);
    if rel.is_absolute() {
        return Err(TypefaceError::Manifest(format!(
            "svg_path must be relative to manifest directory: `{svg_path}`"
        )));
    }
    for component in rel.components() {
        match component {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(TypefaceError::Manifest(format!(
                    "svg_path escapes manifest directory: `{svg_path}`"
                )));
            }
            Component::Normal(_) | Component::CurDir => {}
        }
    }
    Ok(manifest_dir.join(rel))
}

fn manifest_icon_error(err: IconError) -> TypefaceError {
    TypefaceError::Manifest(err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_manifest_loads() {
        let manifest = load_icon_manifest(fixture_manifest_path()).expect("fixture manifest");
        assert_eq!(manifest.icons.len(), 2);
    }
}
