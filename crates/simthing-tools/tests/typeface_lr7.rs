use std::{
    fs,
    path::{Path, PathBuf},
};

use simthing_tools::{
    bake_icon_manifest, fixture_manifest_path, load_font, load_icon_manifest, GlyphAtlasCore,
    IconLayerRole, IconSet, ShapingEngine,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const PX: f32 = 32.0;
const GOLDEN_CODEPOINT_TABLE: &str = "F0001 test.background-accent\nF0002 test.outline-accent";

fn fresh_atlas() -> GlyphAtlasCore {
    GlyphAtlasCore::new(256)
}

fn fixture_font() -> simthing_tools::ProbeFont {
    load_font(FIXTURE).expect("fixture font")
}

fn read_doc(path: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(root.join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
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
fn lr6d_closeout_records_da_approval() {
    let ladder = read_doc("docs/design_typeface_ladder.md");
    assert!(ladder.contains("LR6D — text-on-path + warp field / control lattice"));
    assert!(ladder.contains("DONE / DA APPROVED") || ladder.contains("**DONE / DA APPROVED**"));

    let index = read_doc("docs/tests/current_evidence_index.md");
    assert!(index.contains("TYPEFACE-LR6D-TEXT-ON-PATH-WARP-FIELD-0"));
    assert!(index.contains("DA APPROVED"));
    assert!(index.contains("#891"));
    assert!(index.contains("ffc4bb6891"));
    assert!(index.contains("TYPEFACE-LR6D-COMBINED-MSDF-DEFORM-PROOF-0R"));
    assert!(index.contains("ACCEPTED / closed"));

    let lr6d = read_doc("docs/tests/typeface_lr6d_results.md");
    assert!(lr6d.contains("DA APPROVED"));
    assert!(lr6d.contains("#891"));
    assert!(lr6d.contains("6a32763bdd"));

    let combined = read_doc("docs/tests/typeface_lr6d_combined_msdf_deform_results.md");
    assert!(combined.contains("ACCEPTED / closed") || combined.contains("DA APPROVED"));
}

#[test]
fn manifest_loads_fixture_icons() {
    let manifest = load_icon_manifest(fixture_manifest_path()).expect("load fixture");
    assert_eq!(manifest.reserved_range_start, 0xF0000);
    assert_eq!(manifest.reserved_range_end, 0xF00FF);
    assert_eq!(manifest.icons.len(), 2);
    assert!(manifest
        .icons
        .iter()
        .any(|entry| entry.name == "test.background-accent"));
}

#[test]
fn manifest_bakes_all_icons() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let bake = bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX)
        .expect("bake fixture");
    assert_eq!(bake.baked_codepoints.len(), 2);
    assert!(icons.tile_for(0xF0001).is_some());
    assert!(icons.tile_for(0xF0002).is_some());
}

#[test]
fn codepoint_table_is_stable_golden() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let bake =
        bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX).expect("bake");
    assert_eq!(format_codepoint_table(&bake), GOLDEN_CODEPOINT_TABLE);
}

#[test]
fn duplicate_codepoint_rejected() {
    let dir = temp_manifest_dir("dup_cp");
    let body = r##"(
        reserved_range_start: 0xF0000,
        reserved_range_end: 0xF00FF,
        icons: [
            (name: "a", codepoint: 0xF0001, svg_path: "a.svg"),
            (name: "b", codepoint: 0xF0001, svg_path: "a.svg"),
        ],
    )"##;
    let path = write_manifest(&dir, body, &[("a.svg", VALID_SVG)]);
    let err = load_icon_manifest(&path).expect_err("duplicate codepoint");
    assert!(err.to_string().contains("duplicate manifest codepoint"));
}

#[test]
fn duplicate_name_rejected() {
    let dir = temp_manifest_dir("dup_name");
    let body = r##"(
        reserved_range_start: 0xF0000,
        reserved_range_end: 0xF00FF,
        icons: [
            (name: "same", codepoint: 0xF0001, svg_path: "a.svg"),
            (name: "same", codepoint: 0xF0002, svg_path: "a.svg"),
        ],
    )"##;
    let path = write_manifest(&dir, body, &[("a.svg", VALID_SVG)]);
    let err = load_icon_manifest(&path).expect_err("duplicate name");
    assert!(err.to_string().contains("duplicate manifest name"));
}

#[test]
fn codepoint_outside_reserved_range_rejected() {
    let dir = temp_manifest_dir("out_of_range");
    let body = r##"(
        reserved_range_start: 0xF0000,
        reserved_range_end: 0xF00FF,
        icons: [
            (name: "a", codepoint: 0xF0100, svg_path: "a.svg"),
        ],
    )"##;
    let path = write_manifest(&dir, body, &[("a.svg", VALID_SVG)]);
    let err = load_icon_manifest(&path).expect_err("out of range");
    assert!(err.to_string().contains("outside reserved range"));
}

#[test]
fn missing_svg_path_errors() {
    let dir = temp_manifest_dir("missing_svg");
    let path = write_manifest(&dir, BASE_MANIFEST, &[]);
    let err = load_icon_manifest(&path).expect_err("missing svg");
    assert!(err.to_string().contains("missing SVG path"));
}

#[test]
fn path_escape_rejected() {
    let dir = temp_manifest_dir("escape");
    let body = r##"(
        reserved_range_start: 0xF0000,
        reserved_range_end: 0xF00FF,
        icons: [
            (name: "a", codepoint: 0xF0001, svg_path: "../outside.svg"),
        ],
    )"##;
    let path = write_manifest(&dir, body, &[]);
    let err = load_icon_manifest(&path).expect_err("path escape");
    assert!(err.to_string().contains("escapes manifest directory"));
}

#[test]
fn invalid_or_dynamic_svg_rejected() {
    let dir = temp_manifest_dir("dynamic_svg");
    let dynamic = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
  <script>alert(1)</script>
  <rect x="1" y="1" width="14" height="14"/>
</svg>
"##;
    let path = write_manifest(&dir, BASE_MANIFEST, &[("a.svg", dynamic)]);
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let err = bake_icon_manifest(&path, &mut icons, &mut atlas, PX).expect_err("dynamic svg");
    assert!(err.to_string().contains("StaticOnly") || err.to_string().contains("script"));
}

#[test]
fn manifest_icons_preserve_iconvector_geometry() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX).expect("bake");
    let vector = icons.vector_for(0xF0001).expect("vector");
    assert!(!vector.layers.is_empty());
    assert!(vector.layers.iter().all(|layer| !layer.paths.is_empty()));
}

#[test]
fn manifest_icons_preserve_role_layers() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX).expect("bake");

    let bg_accent = icons.vector_for(0xF0001).expect("bg accent");
    assert_eq!(
        bg_accent.layer_roles().collect::<Vec<_>>(),
        vec![IconLayerRole::Background, IconLayerRole::Accent]
    );

    let outline_accent = icons.vector_for(0xF0002).expect("outline accent");
    assert_eq!(
        outline_accent.layer_roles().collect::<Vec<_>>(),
        vec![IconLayerRole::Outline, IconLayerRole::Accent]
    );
}

#[test]
fn manifest_icons_can_render_mixed_text_icon_run() {
    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    let font = fixture_font();
    let mut shaper = ShapingEngine::new_with_font(FIXTURE.to_vec()).expect("shaper");
    let icon_tile = bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX)
        .expect("bake")
        .baked_codepoints
        .first()
        .copied()
        .and_then(|cp| icons.tile_for(cp))
        .expect("icon tile");

    let instances = icons
        .build_mixed_instances(
            &font,
            &mut shaper,
            &mut atlas,
            "Sol \u{F0001} 42",
            PX,
            [1.0, 1.0, 1.0, 1.0],
        )
        .expect("mixed run");

    let inv = 1.0 / atlas.atlas_size() as f32;
    let icon_uv = [
        icon_tile.x as f32 * inv,
        icon_tile.y as f32 * inv,
        (icon_tile.x + icon_tile.w) as f32 * inv,
        (icon_tile.y + icon_tile.h) as f32 * inv,
    ];
    assert!(instances.len() > 1);
    assert_eq!(instances.iter().filter(|i| i.uv_rect == icon_uv).count(), 1);
}

#[test]
fn manifest_bake_has_no_runtime_svg_dependency() {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_src = fs::read_to_string(crate_root.join("src/manifest.rs")).expect("manifest.rs");
    for forbidden in [
        "per-frame",
        "per_frame",
        "from_svg.*update",
        "parse.*SVG.*frame",
        "runtime.*SVG",
    ] {
        assert!(
            !manifest_src.contains(forbidden),
            "manifest.rs must not reference runtime SVG parsing: `{forbidden}`"
        );
    }

    let mut atlas = fresh_atlas();
    let mut icons = IconSet::new();
    bake_icon_manifest(fixture_manifest_path(), &mut icons, &mut atlas, PX).expect("bake");
    assert!(icons.vector_for(0xF0001).is_some());
    assert!(icons.tile_for(0xF0001).is_some());
}

#[test]
fn semantic_free_guard_still_passes() {
    let status = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "semantic_free_guard",
            "--",
            "--exact",
            "shader_and_src_are_semantic_free",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn semantic_free_guard");
    assert!(status.success(), "semantic_free_guard failed");
}

#[test]
fn gpu_residency_audit_documented_for_lr7() {
    let results = read_doc("docs/tests/typeface_lr7_results.md");
    assert!(results.contains("GPU residency"));
    assert!(results.contains("CPU surfacing"));
    assert!(results.contains("import/staging"));
}
