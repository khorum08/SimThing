use std::{
    fs,
    path::{Path, PathBuf},
};

use simthing_tools::{
    bake_icon_manifest, fixture_manifest_path, load_icon_manifest, GlyphAtlasCore, IconSet,
};

const PX: f32 = 32.0;
const GOLDEN_CODEPOINT_TABLE: &str = "F0001 test.background-accent\nF0002 test.outline-accent";

fn fresh_atlas() -> GlyphAtlasCore {
    GlyphAtlasCore::new(256)
}

fn format_codepoint_table(bake: &simthing_tools::IconManifestBake) -> String {
    bake.codepoint_to_name
        .iter()
        .map(|(codepoint, name)| format!("{codepoint:04X} {name}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn temp_manifest_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simthing_lr7_{name}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn write_manifest(dir: &Path, body: &str, svg_files: &[(&str, &str)]) -> PathBuf {
    for (file_name, contents) in svg_files {
        fs::write(dir.join(file_name), contents).expect("write svg");
    }
    let manifest_path = dir.join("manifest.ron");
    fs::write(&manifest_path, body).expect("write manifest");
    manifest_path
}

const VALID_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

const BASE_MANIFEST: &str = r##"(
    reserved_range_start: 0xF0000,
    reserved_range_end: 0xF00FF,
    icons: [
        (
            name: "test.a",
            codepoint: 0xF0001,
            svg_path: "a.svg",
        ),
    ],
)"##;

#[test]
fn codepoint_table_is_stable_golden() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let bake =
        bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX).expect("bake");
    assert_eq!(format_codepoint_table(&bake), GOLDEN_CODEPOINT_TABLE);
}
