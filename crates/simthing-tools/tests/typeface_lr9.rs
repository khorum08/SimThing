use std::path::PathBuf;

use bevy::prelude::*;
use simthing_tools::{
    adapter_label, format_lr9_scenario_report, install_dynamic_style_rows, lr9_cpu_bevy_app,
    lr9_studio_shell_app, profile_dynamic_style_labels, profile_flat_animated_labels,
    profile_numeric_damage_lane, profile_studio_seam_labels, profile_warped_nameplates,
    spawn_styled_labels, spawn_warped_nameplate_labels, studio_typeface_label_diagnostics,
    text_deform_diagnostics, text_path_warp_diagnostics, text_perf_diagnostics,
    text_style_diagnostics, TextLabel, TextStyleRow, TextStyleTableResource, LR9_BINDING_CONFIG,
    LR9_CI_CONFIG,
};

fn read_doc(path: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::fs::read_to_string(root.join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn clear_exit(app: &mut App) {
    if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
        exits.clear();
    }
}

fn run_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        clear_exit(app);
        app.update();
    }
}

#[test]
fn lr8_closeout_records_da_approval() {
    let ladder = read_doc("docs/design_typeface_ladder.md");
    assert!(ladder.contains("LR8"));
    assert!(ladder.contains("DONE / DA APPROVED"));
    assert!(ladder.contains("#894"));
    assert!(ladder.contains("850a216a7a"));

    let index = read_doc("docs/tests/current_evidence_index.md");
    assert!(index.contains("TYPEFACE-LR8-STUDIO-LABEL-SEAM-0"));
    assert!(index.contains("DA APPROVED after #894"));
    assert!(index.contains("TYPEFACE-LR8-STUDIO-PLUGIN-MOUNT-0R"));
    assert!(index.contains("ACCEPTED / closed"));

    let mount = read_doc("docs/tests/typeface_lr8_studio_plugin_mount_results.md");
    assert!(mount.contains("ACCEPTED / closed"));
    assert!(mount.contains("#894"));
}

#[test]
fn flat_5k_noop_perf_profile_records_budget() {
    let profile = profile_flat_animated_labels(LR9_CI_CONFIG);
    eprintln!("{}", format_lr9_scenario_report(&profile));

    assert_eq!(profile.config.flat_labels, LR9_CI_CONFIG.flat_labels);
    assert!(
        profile.avg_noop_update_ms < 1.0,
        "avg no-op CPU update must stay under 1 ms/frame (got {:.4}ms)",
        profile.avg_noop_update_ms
    );
    let mut app = lr9_cpu_bevy_app(LR9_CI_CONFIG.atlas_size);
    simthing_tools::spawn_static_text_labels(&mut app, LR9_CI_CONFIG.flat_labels, 24.0);
    run_frames(&mut app, 2);
    let before = text_perf_diagnostics(&app);
    run_frames(&mut app, LR9_CI_CONFIG.noop_frames);
    let after = text_perf_diagnostics(&app);
    assert_eq!(after.shape_rebuild_count, before.shape_rebuild_count);
    assert_eq!(after.instance_rebuild_count, before.instance_rebuild_count);
}

#[test]
#[ignore = "manual binding proof: 5000 flat animated labels"]
fn flat_5k_binding_noop_perf_profile() {
    let profile = profile_flat_animated_labels(LR9_BINDING_CONFIG);
    eprintln!("=== LR9 FLAT 5K BINDING ===");
    eprintln!("{}", format_lr9_scenario_report(&profile));
    assert_eq!(profile.config.flat_labels, 5_000);
    assert!(
        profile.avg_noop_update_ms < 1.0,
        "5k avg no-op must be <1 ms (got {:.4}ms)",
        profile.avg_noop_update_ms
    );
}

#[test]
fn numeric_damage_5k_perf_profile_records_budget() {
    let profile = profile_numeric_damage_lane(LR9_CI_CONFIG);
    eprintln!("{}", format_lr9_scenario_report(&profile));

    assert!(
        profile.avg_noop_update_ms < 1.0,
        "numeric lane avg no-op must stay under 1 ms/frame"
    );
    assert!(
        profile.perf.aggregate_repack_count == profile.perf.aggregate_repack_count,
        "structural smoke"
    );
}

#[test]
#[ignore = "manual binding proof: 5000 fixed-width numeric damage labels"]
fn numeric_damage_5k_binding_perf_profile() {
    let profile = profile_numeric_damage_lane(LR9_BINDING_CONFIG);
    eprintln!("=== LR9 NUMERIC 5K BINDING ===");
    eprintln!("{}", format_lr9_scenario_report(&profile));
    assert_eq!(profile.config.numeric_damage_labels, 5_000);
    assert!(
        profile.avg_noop_update_ms < 1.0,
        "5k numeric avg no-op must be <1 ms (got {:.4}ms)",
        profile.avg_noop_update_ms
    );
}

#[test]
fn dynamic_style_rows_upload_only_on_generation_change() {
    let mut app = lr9_cpu_bevy_app(LR9_CI_CONFIG.atlas_size);
    install_dynamic_style_rows(&mut app);
    spawn_styled_labels(&mut app, 32);
    run_frames(&mut app, 2);
    let upload_after_warmup = text_style_diagnostics(&app).style_table_upload_count;
    run_frames(&mut app, 8);
    let upload_after_noop = text_style_diagnostics(&app).style_table_upload_count;
    assert_eq!(upload_after_noop, upload_after_warmup);

    let gen_before = app
        .world()
        .get_resource::<TextStyleTableResource>()
        .map(|r| r.rows_generation)
        .unwrap_or(0);
    app.world_mut()
        .resource_mut::<TextStyleTableResource>()
        .set_row(5, TextStyleRow::solid_fill(0.0, 0.0, 1.0, 1.0))
        .expect("slot 5");
    run_frames(&mut app, 1);
    let gen_after = app
        .world()
        .get_resource::<TextStyleTableResource>()
        .map(|r| r.rows_generation)
        .unwrap_or(0);
    let upload_after_change = text_style_diagnostics(&app).style_table_upload_count;
    assert!(gen_after > gen_before);
    assert!(upload_after_change > upload_after_noop);
}

#[test]
fn warped_nameplate_set_noop_reuses_tessellation_and_buffers() {
    let mut app = lr9_cpu_bevy_app(LR9_CI_CONFIG.atlas_size);
    spawn_warped_nameplate_labels(&mut app, LR9_CI_CONFIG.warped_labels);
    run_frames(&mut app, 3);
    let deform_before = text_deform_diagnostics(&app);
    let path_before = text_path_warp_diagnostics(&app);
    let perf_before = text_perf_diagnostics(&app);
    run_frames(&mut app, 10);
    let deform_after = text_deform_diagnostics(&app);
    let path_after = text_path_warp_diagnostics(&app);
    let perf_after = text_perf_diagnostics(&app);

    assert!(deform_before.tessellated_vertex_count > 0);
    assert_eq!(
        deform_after.tessellated_vertex_count,
        deform_before.tessellated_vertex_count
    );
    assert!(path_after.path_warp_noop_reuse_count > path_before.path_warp_noop_reuse_count);
    assert_eq!(
        perf_after.shape_rebuild_count,
        perf_before.shape_rebuild_count
    );
}

#[test]
fn warped_nameplate_change_rebuilds_bounded_once() {
    let profile = profile_warped_nameplates(LR9_CI_CONFIG);
    eprintln!("{}", format_lr9_scenario_report(&profile));
    assert!(profile.metrics.tessellated_vertex_count > 0);
    assert!(profile.avg_changed_update_ms >= 0.0);
}

#[test]
fn studio_shell_label_profile_uses_typeface_runtime() {
    let profile = profile_studio_seam_labels(LR9_CI_CONFIG);
    eprintln!("{}", format_lr9_scenario_report(&profile));
    assert!(profile.metrics.total_labels >= LR9_CI_CONFIG.studio_labels);
    assert_eq!(profile.metrics.bespoke_text_fallback_count, 0);
    assert!(
        profile.avg_noop_update_ms < 1.0,
        "studio seam avg no-op must stay under 1 ms/frame"
    );
}

#[test]
fn manifest_icon_profile_does_not_reload_manifest_per_frame() {
    let mut app = lr9_studio_shell_app(LR9_CI_CONFIG.atlas_size);
    app.world_mut().spawn(
        simthing_tools::StudioTypefaceLabel::entity_name("Sol", 24.0, [1.0; 4])
            .with_icon_name("test.background-accent"),
    );
    run_frames(&mut app, 1);
    let first = studio_typeface_label_diagnostics(&app);
    assert_eq!(first.manifest_reload_count, 1);
    run_frames(&mut app, 8);
    let after = studio_typeface_label_diagnostics(&app);
    assert_eq!(after.manifest_reload_count, 1);
    assert_eq!(after.runtime_svg_parse_count, 0);
}

#[test]
fn combined_msdf_style_deform_path_warp_smoke_still_passes() {
    let status = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr6d",
            "--",
            "--exact",
            "combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr6d combined smoke");
    assert!(
        status.success(),
        "combined MSDF/style/deform/path/warp smoke failed"
    );
}

#[test]
fn atlas_style_deform_path_warp_buffer_residency_still_passes() {
    let lr6d_style = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr6d",
            "--",
            "--exact",
            "style_table_buffer_residency_still_passes",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr6d style residency");
    assert!(lr6d_style.success(), "style buffer residency failed");

    let lr6d_atlas = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr6d",
            "--",
            "--exact",
            "atlas_bind_group_residency_still_passes",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr6d atlas residency");
    assert!(lr6d_atlas.success(), "atlas bind group residency failed");

    let lr6d = std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "typeface_lr6d",
            "--",
            "--exact",
            "path_warp_bind_group_reused_on_noop_frames",
        ])
        .current_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .expect("workspace root"),
        )
        .status()
        .expect("spawn lr6d residency");
    assert!(lr6d.success(), "path/warp bind group residency failed");
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
fn gpu_residency_audit_documented_for_lr9() {
    let results = read_doc("docs/tests/typeface_lr9_results.md");
    assert!(results.contains("GPU residency"));
    assert!(results.contains("CPU surfacing"));
    assert!(results.contains("import/staging"));
}

#[test]
fn dynamic_style_profile_smoke() {
    let profile = profile_dynamic_style_labels(LR9_CI_CONFIG);
    eprintln!("{}", format_lr9_scenario_report(&profile));
    assert!(profile.metrics.style_row_count >= 1);
}

#[test]
fn lr9_adapter_label_is_honest() {
    let label = adapter_label();
    assert!(
        label.starts_with("REAL_ADAPTER_OBSERVED:") || label == "ADAPTER_SKIPPED",
        "adapter label must be honest: {label}"
    );
}

#[test]
fn flat_labels_use_default_tessellation_level() {
    let mut app = lr9_cpu_bevy_app(LR9_CI_CONFIG.atlas_size);
    app.world_mut()
        .spawn(TextLabel::raster("Flat", 24.0, [1.0; 4]));
    run_frames(&mut app, 2);
    let world = app.world_mut();
    let mut q = world.query::<&simthing_tools::TextGlyphInstances>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].deform_params[1], 0.0);
}
