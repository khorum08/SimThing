use std::path::PathBuf;

use bevy::prelude::*;
use simthing_mapeditor::{
    mount_studio_typeface_plugins, typeface_fixture_font_bytes, StudioTypefaceShellMounted,
    StudioTypefaceShellPlugin,
};
use simthing_tools::{
    fixture_manifest_path, icon_name_to_codepoint, load_icon_manifest, NumericDamageLabel,
    SimthingToolsTextPlugin, StudioDamageTextEmitter, StudioLabelKind, StudioTypefaceLabel,
    StudioTypefaceLabelDiagnostics, StudioTypefaceLabelPlugin, TextLabel, TextLabelRenderMode,
    TextPerfDiagnostics, TypefaceIconSet,
};

const PX: f32 = 32.0;

fn read_doc(path: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

/// Headless LR8 seam tests — plugin stack only (no shell probe staging).
fn lr8_headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    app.add_plugins(SimthingToolsTextPlugin::new(typeface_fixture_font_bytes()));
    app.add_plugins(StudioTypefaceLabelPlugin);
    app
}

fn studio_app() -> App {
    lr8_headless_app()
}

/// Same plugin registration path as `app::run_studio` (without DefaultPlugins/window).
fn studio_app_shell() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    mount_studio_typeface_plugins(&mut app);
    app
}

fn run_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
            exits.clear();
        }
        app.update();
    }
}

#[test]
fn lr7_closeout_records_da_approval() {
    let ladder = read_doc("docs/design_typeface_ladder.md");
    assert!(ladder.contains("LR7 — custom character set / icon-font manifest"));
    assert!(ladder.contains("DA APPROVED for manifest machinery"));

    let index = read_doc("docs/tests/current_evidence_index.md");
    assert!(index.contains("TYPEFACE-LR7-ICON-FONT-MANIFEST-0"));
    assert!(index.contains("DA APPROVED for manifest machinery"));
    assert!(index.contains("#892"));
    assert!(index.contains("ac320204eb"));

    let lr7 = read_doc("docs/tests/typeface_lr7_results.md");
    assert!(lr7.contains("#892"));
    assert!(lr7.contains("ac320204eb"));
    assert!(lr7.contains("be8dde2388"));
}

#[test]
fn studio_label_spawn_uses_typeface_components() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    app.world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]));
    run_frames(&mut app, 1);

    let mut q = app.world_mut().query::<&TextLabel>();
    assert_eq!(q.iter(&app.world()).count(), 1);
    let label = q.single(&app.world()).expect("text label");
    assert_eq!(label.text, "Sol");
}

#[test]
fn studio_entity_label_uses_style_slot() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    app.world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]).with_style_slot(2));
    run_frames(&mut app, 1);

    let mut q = app.world_mut().query::<&TextLabel>();
    let label = q.single(&app.world()).expect("text label");
    assert_eq!(label.style_slot, 2);
}

#[test]
fn studio_entity_label_can_select_msdf_render_mode() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    app.world_mut().spawn(
        StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4])
            .with_render_mode(TextLabelRenderMode::Msdf),
    );
    run_frames(&mut app, 1);

    let mut q = app.world_mut().query::<&TextLabel>();
    let label = q.single(&app.world()).expect("text label");
    assert_eq!(label.render_mode, TextLabelRenderMode::Msdf);
}

#[test]
fn studio_label_noop_does_not_rebuild_or_reshape() {
    let mut app = studio_app();
    app.world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]));
    run_frames(&mut app, 2);

    let after_first = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");
    run_frames(&mut app, 3);
    let after_noop = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");

    assert_eq!(
        after_noop.shape_rebuild_count,
        after_first.shape_rebuild_count
    );
    assert_eq!(
        after_noop.instance_rebuild_count,
        after_first.instance_rebuild_count
    );
}

#[test]
fn studio_label_text_change_rebuilds_once() {
    let mut app = studio_app();
    let entity = app
        .world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]))
        .id();
    run_frames(&mut app, 1);

    let before = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");

    app.world_mut()
        .entity_mut(entity)
        .get_mut::<StudioTypefaceLabel>()
        .expect("studio label")
        .text = "Altair".into();
    run_frames(&mut app, 1);

    let after = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");
    assert!(after.shape_rebuild_count > before.shape_rebuild_count);
    assert!(after.instance_rebuild_count > before.instance_rebuild_count);

    let label = app.world().get::<TextLabel>(entity).expect("text label");
    assert_eq!(label.text, "Altair");
}

#[test]
fn studio_damage_text_emitter_uses_existing_typeface_path() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    app.world_mut()
        .spawn(StudioDamageTextEmitter::default())
        .get_mut::<StudioDamageTextEmitter>()
        .expect("emitter")
        .emit(42);
    run_frames(&mut app, 1);

    let mut q = app.world_mut().query::<&NumericDamageLabel>();
    assert_eq!(q.iter(&app.world()).count(), 1);
    let mut q = app.world_mut().query::<&NumericDamageLabel>();
    let numeric = q.single(&app.world()).expect("numeric label");
    assert_eq!(numeric.value, -42);
}

#[test]
fn manifest_icon_name_resolves_to_pua_codepoint() {
    let manifest = load_icon_manifest(fixture_manifest_path()).expect("fixture manifest");
    let bake = simthing_tools::bake_icon_manifest(
        fixture_manifest_path(),
        &mut simthing_tools::IconSet::new(),
        &mut simthing_tools::GlyphAtlasCore::new(256),
        PX,
    )
    .expect("bake");
    let cp = icon_name_to_codepoint(&bake, "test.background-accent").expect("codepoint");
    assert_eq!(cp, 0xF0001);
    assert_eq!(manifest.icons.len(), 2);
}

#[test]
fn studio_mixed_text_icon_label_uses_manifest_icon() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    assert!(app.world().get_resource::<TypefaceIconSet>().is_some());

    app.world_mut().spawn(
        StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4])
            .with_icon_name("test.background-accent"),
    );
    run_frames(&mut app, 1);

    let mut text_q = app.world_mut().query::<&TextLabel>();
    let label = text_q.single(&app.world()).expect("text label");
    assert!(label.text.chars().any(|ch| (ch as u32) == 0xF0001));

    let mut inst_q = app
        .world_mut()
        .query::<(&TextLabel, &simthing_tools::TextGlyphInstances)>();
    let (_, instances) = inst_q.single(&app.world()).expect("instances");
    assert!(instances.0.len() > 1);
}

#[test]
fn manifest_not_reloaded_per_frame() {
    let mut app = studio_app();
    run_frames(&mut app, 1);
    let first = app
        .world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag");
    assert_eq!(first.manifest_reload_count, 1);

    run_frames(&mut app, 5);
    let after = app
        .world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag");
    assert_eq!(after.manifest_reload_count, 1);
}

#[test]
fn runtime_svg_not_parsed_per_frame() {
    let studio_diag = app_studio_diag_after_frames(6);
    assert_eq!(studio_diag.runtime_svg_parse_count, 0);
}

#[test]
fn no_bespoke_text_renderer_fallback() {
    let studio_diag = app_studio_diag_after_frames(3);
    assert_eq!(studio_diag.bespoke_text_fallback_count, 0);
}

fn app_studio_diag_after_frames(frames: usize) -> StudioTypefaceLabelDiagnostics {
    let mut app = studio_app();
    app.world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]));
    run_frames(&mut app, frames);
    app.world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag")
}

#[test]
fn lr7_manifest_regression_still_passes() {
    let status = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr7",
            "--",
            "--exact",
            "manifest_bakes_all_icons",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr7");
    assert!(status.success(), "typeface_lr7 regression failed");
}

#[test]
fn lr6d_path_warp_regression_still_passes() {
    let status = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr6d",
            "--",
            "--exact",
            "path_opt_in_sets_path_slot_metadata",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr6d");
    assert!(status.success(), "typeface_lr6d regression failed");
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
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn semantic guard");
    assert!(status.success(), "semantic_free_guard failed");
}

#[test]
fn gpu_residency_audit_documented_for_lr8() {
    let results = read_doc("docs/tests/typeface_lr8_results.md");
    assert!(results.contains("GPU residency"));
    assert!(results.contains("CPU surfacing"));
    assert!(results.contains("import/staging"));
}

#[test]
fn studio_label_kind_damage_uses_numeric_lane() {
    let mut app = studio_app();
    let entity = app
        .world_mut()
        .spawn(StudioTypefaceLabel::damage_value(128, PX, [1.0; 4]))
        .id();
    run_frames(&mut app, 1);

    assert!(app.world().get::<NumericDamageLabel>(entity).is_some());
    let mut text_q = app.world_mut().query_filtered::<(), With<TextLabel>>();
    assert!(text_q.iter(&app.world()).next().is_none());
}

#[test]
fn studio_label_kind_entity_is_not_damage() {
    let label = StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4]);
    assert_eq!(label.kind, StudioLabelKind::EntityName);
}

#[test]
fn studio_app_shell_mounts_typeface_label_plugin() {
    let app = studio_app_shell();
    assert!(app.is_plugin_added::<StudioTypefaceShellPlugin>());
    assert!(app.is_plugin_added::<SimthingToolsTextPlugin>());
    assert!(app.is_plugin_added::<StudioTypefaceLabelPlugin>());
}

#[test]
fn studio_app_shell_bakes_fixture_manifest_once() {
    let mut app = studio_app_shell();
    run_frames(&mut app, 1);
    assert!(app.world().get_resource::<TypefaceIconSet>().is_some());
    let mounted = app
        .world()
        .get_resource::<StudioTypefaceShellMounted>()
        .copied()
        .expect("shell mounted resource");
    assert!(mounted.0);

    let first = app
        .world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag");
    assert_eq!(first.manifest_reload_count, 1);

    run_frames(&mut app, 4);
    let after = app
        .world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag");
    assert_eq!(after.manifest_reload_count, 1);
}

#[test]
fn studio_app_shell_can_spawn_typeface_label() {
    let mut app = studio_app_shell();
    run_frames(&mut app, 1);
    let probe_count = app
        .world_mut()
        .query_filtered::<(), With<StudioTypefaceLabel>>()
        .iter(&app.world())
        .count();
    assert!(probe_count >= 1);

    app.world_mut()
        .spawn(StudioTypefaceLabel::entity_name("Altair", PX, [1.0; 4]));
    run_frames(&mut app, 1);

    let mut labels = app.world_mut().query::<&TextLabel>();
    assert!(labels
        .iter(&app.world())
        .any(|label| label.text == "Altair"));
}

#[test]
fn studio_app_shell_can_resolve_manifest_icon_label() {
    let mut app = studio_app_shell();
    run_frames(&mut app, 1);
    app.world_mut().spawn(
        StudioTypefaceLabel::entity_name("Sol", PX, [1.0; 4])
            .with_icon_name("test.background-accent"),
    );
    run_frames(&mut app, 1);

    let mut labels = app.world_mut().query::<&TextLabel>();
    let label = labels
        .iter(&app.world())
        .find(|entry| entry.text.contains("Sol"))
        .expect("icon label");
    assert!(label.text.chars().any(|ch| (ch as u32) == 0xF0001));
}

#[test]
fn studio_app_shell_can_emit_damage_text() {
    let mut app = studio_app_shell();
    run_frames(&mut app, 1);
    assert!(app
        .world_mut()
        .query_filtered::<(), With<StudioDamageTextEmitter>>()
        .iter(&app.world())
        .next()
        .is_some());

    app.world_mut()
        .spawn(StudioDamageTextEmitter::default())
        .get_mut::<StudioDamageTextEmitter>()
        .expect("emitter")
        .emit(99);
    run_frames(&mut app, 1);

    let mut numeric = app.world_mut().query::<&NumericDamageLabel>();
    assert!(numeric.iter(&app.world()).any(|label| label.value == -99));
}

#[test]
fn studio_app_shell_noop_does_not_rebuild_or_reshape() {
    let mut app = studio_app_shell();
    run_frames(&mut app, 2);
    let after_warmup = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");
    run_frames(&mut app, 4);
    let after_noop = app
        .world()
        .get_resource::<TextPerfDiagnostics>()
        .copied()
        .expect("diag");
    assert_eq!(
        after_noop.shape_rebuild_count,
        after_warmup.shape_rebuild_count
    );
    assert_eq!(
        after_noop.instance_rebuild_count,
        after_warmup.instance_rebuild_count
    );
}

#[test]
fn studio_app_shell_no_runtime_svg_or_manifest_reload() {
    let studio_diag = app_studio_diag_after_shell_frames(6);
    assert_eq!(studio_diag.runtime_svg_parse_count, 0);
    assert_eq!(studio_diag.manifest_reload_count, 1);
}

fn app_studio_diag_after_shell_frames(frames: usize) -> StudioTypefaceLabelDiagnostics {
    let mut app = studio_app_shell();
    run_frames(&mut app, frames);
    app.world()
        .get_resource::<StudioTypefaceLabelDiagnostics>()
        .copied()
        .expect("studio diag")
}

#[test]
fn lr8_headless_tests_still_pass() {
    let status = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-mapeditor",
            "--test",
            "typeface_lr8",
            "--",
            "studio_label_spawn_uses_typeface_components",
            "manifest_not_reloaded_per_frame",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr8 subset");
    assert!(status.success(), "lr8 headless subset failed");
}

#[test]
fn gpu_residency_audit_documented_for_lr8_plugin_mount() {
    let results = read_doc("docs/tests/typeface_lr8_studio_plugin_mount_results.md");
    assert!(results.contains("GPU residency"));
    assert!(results.contains("CPU surfacing"));
    assert!(results.contains("import/staging"));
}
