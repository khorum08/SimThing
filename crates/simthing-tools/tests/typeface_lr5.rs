use simthing_tools::{
    icon_tile_in_atlas, run_typeface_bench, TypefaceBenchConfig, TypefaceBenchHarness,
    CI_BENCH_CONFIG, HEAVY_BENCH_CONFIG, ICON_PUA_START,
};

const SMALL_CONFIG: TypefaceBenchConfig = TypefaceBenchConfig {
    static_labels: 250,
    damage_labels: 50,
    frames: 8,
    icon_every_n_labels: 5,
    atlas_size: 2048,
};

#[test]
fn high_volume_static_labels_noop_frame_does_not_reshape() {
    let mut harness = TypefaceBenchHarness::new(SMALL_CONFIG).expect("harness");
    harness.register_fixture_icons().expect("icons");
    harness
        .build_static_labels(SMALL_CONFIG.static_labels, SMALL_CONFIG.icon_every_n_labels)
        .expect("static labels");

    let before = harness.diagnostics().shape_rebuild_count;
    harness.run_noop_frames(4).expect("noop frames");
    let after = harness.diagnostics().shape_rebuild_count;

    assert_eq!(
        before, after,
        "no-op frames must not reshape unchanged labels"
    );
}

#[test]
fn high_volume_static_labels_noop_frame_does_not_rerasterize() {
    let mut harness = TypefaceBenchHarness::new(SMALL_CONFIG).expect("harness");
    harness.register_fixture_icons().expect("icons");
    harness
        .build_static_labels(SMALL_CONFIG.static_labels, SMALL_CONFIG.icon_every_n_labels)
        .expect("static labels");

    let before = harness.atlas_stats().rasterize_count;
    let icon_entries_before = harness.icon_cache_entries();
    harness.run_noop_frames(4).expect("noop frames");
    let after = harness.atlas_stats().rasterize_count;

    assert_eq!(
        before, after,
        "no-op frames must not rerasterize cached glyphs/icons"
    );
    assert_eq!(
        icon_entries_before,
        harness.icon_cache_entries(),
        "no-op frames must not register new icons"
    );
}

#[test]
fn damage_text_churn_rebuilds_only_changed_labels() {
    let mut harness = TypefaceBenchHarness::new(SMALL_CONFIG).expect("harness");
    harness.register_fixture_icons().expect("icons");
    harness
        .build_static_labels(SMALL_CONFIG.static_labels, SMALL_CONFIG.icon_every_n_labels)
        .expect("static labels");
    harness
        .build_damage_labels(SMALL_CONFIG.damage_labels)
        .expect("damage labels");

    let static_rebuilds_before = harness.diagnostics().shape_rebuild_count;
    let damage_frames = 3usize;
    harness
        .run_damage_frames(damage_frames)
        .expect("damage frames");
    let static_rebuilds_after = harness.diagnostics().shape_rebuild_count;

    let damage_rebuilds = static_rebuilds_after - static_rebuilds_before;
    let expected_damage_rebuilds = (SMALL_CONFIG.damage_labels * damage_frames) as u64;
    assert_eq!(
        damage_rebuilds, expected_damage_rebuilds,
        "only damage labels should rebuild on churn frames"
    );
}

#[test]
fn mixed_text_icon_workload_reuses_one_atlas() {
    let mut harness = TypefaceBenchHarness::new(SMALL_CONFIG).expect("harness");
    harness.register_fixture_icons().expect("icons");
    harness
        .build_static_labels(SMALL_CONFIG.static_labels, SMALL_CONFIG.icon_every_n_labels)
        .expect("static labels");

    let icon_tile = icon_tile_in_atlas(harness.atlas_core(), harness.icons(), ICON_PUA_START + 1)
        .expect("icon tile in shared atlas");
    let icon_pixels = harness.atlas_core().tile_pixels(icon_tile);
    assert!(
        icon_pixels.chunks(4).any(|px| px[3] > 0),
        "icon tile must live in shared atlas pixels"
    );

    assert_eq!(
        harness.icon_cache_entries(),
        2,
        "fixture registers two icons"
    );
    assert!(
        harness.atlas_stats().rasterize_count >= 2,
        "atlas must contain both icon raster inserts"
    );
}

#[test]
fn repeated_svg_icons_are_cached_under_load() {
    let mut harness = TypefaceBenchHarness::new(SMALL_CONFIG).expect("harness");
    harness.register_fixture_icons().expect("icons");
    let icon_entries = harness.icon_cache_entries();

    harness
        .build_static_labels(SMALL_CONFIG.static_labels, SMALL_CONFIG.icon_every_n_labels)
        .expect("static labels");

    assert_eq!(
        harness.icon_cache_entries(),
        icon_entries,
        "repeated icon codepoints must not grow icon cache during label build"
    );
}

#[test]
fn bench_result_report_is_deterministic_enough() {
    let first = run_typeface_bench(CI_BENCH_CONFIG).expect("first bench");
    let second = run_typeface_bench(CI_BENCH_CONFIG).expect("second bench");

    assert_eq!(first.static_labels, second.static_labels);
    assert_eq!(first.damage_labels, second.damage_labels);
    assert_eq!(first.frames, second.frames);
    assert_eq!(first.initial_shape_rebuilds, second.initial_shape_rebuilds);
    assert_eq!(first.noop_shape_rebuilds, second.noop_shape_rebuilds);
    assert_eq!(first.damage_shape_rebuilds, second.damage_shape_rebuilds);
    assert_eq!(
        first.initial_rasterize_count,
        second.initial_rasterize_count
    );
    assert_eq!(
        first.noop_rasterize_count_delta,
        second.noop_rasterize_count_delta
    );
    assert_eq!(
        first.damage_rasterize_count_delta,
        second.damage_rasterize_count_delta
    );
    assert_eq!(first.instance_count, second.instance_count);
    assert_eq!(first.icon_cache_entries, second.icon_cache_entries);
}

#[test]
fn ci_bench_budget_gates_pass() {
    let result = run_typeface_bench(CI_BENCH_CONFIG).expect("ci bench");

    assert!(result.initial_shape_rebuilds > 0);
    assert_eq!(result.noop_shape_rebuilds, 0);
    assert_eq!(result.noop_rasterize_count_delta, 0);
    assert!(result.instance_count > 0);
    assert!(result.icon_cache_entries >= 2);
}

#[test]
#[ignore = "manual heavy bench: 5k static + 500 damage labels"]
fn heavy_bench_manual() {
    let result = run_typeface_bench(HEAVY_BENCH_CONFIG).expect("heavy bench");
    assert_eq!(result.noop_shape_rebuilds, 0);
    assert_eq!(result.noop_rasterize_count_delta, 0);
    eprintln!(
        "heavy bench: initial={:.1}ms noop={:.1}ms damage={:.1}ms instances={}",
        result.elapsed_initial_ms,
        result.elapsed_noop_ms,
        result.elapsed_damage_ms,
        result.instance_count
    );
}
