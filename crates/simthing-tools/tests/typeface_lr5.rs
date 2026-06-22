use bevy::{
    app::{PluginGroup, PluginsState},
    prelude::*,
    render::pipelined_rendering::PipelinedRenderingPlugin,
    sprite::{Mesh2dRenderPlugin, SpritePlugin},
    window::{ExitCondition, WindowPlugin},
    winit::WinitPlugin,
    DefaultPlugins,
};
use simthing_tools::{
    create_render_target_image, icon_tile_in_atlas, profile_bevy_text_bench, run_typeface_bench,
    spawn_static_and_damage_labels, text_label_entity_counts, text_perf_diagnostics,
    text_render_camera_bundle, text_render_queue_state, SimthingToolsTextPlugin, TextLabel,
    TypefaceBenchConfig, TypefaceBenchHarness, CI_BENCH_CONFIG, HEAVY_BENCH_CONFIG, ICON_PUA_START,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const LABEL_PX: f32 = 24.0;
const CI_BEVY_STATIC: usize = 1_000;
const CI_BEVY_DAMAGE: usize = 100;
const BINDING_BEVY_STATIC: usize = 5_000;
const BINDING_BEVY_DAMAGE: usize = 500;
const BEVY_ATLAS_SIZE: u32 = 4096;

fn fixture_bytes() -> Vec<u8> {
    FIXTURE.to_vec()
}

fn cpu_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
            fixture_bytes(),
            BEVY_ATLAS_SIZE,
        ));
    app
}

fn ensure_render_app_ready(app: &mut App) {
    while app.plugins_state() == PluginsState::Adding {
        bevy_tasks::tick_global_task_pools_on_main_thread();
    }
    if app.plugins_state() != PluginsState::Cleaned {
        app.finish();
        app.cleanup();
    }
}

fn render_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<WinitPlugin>()
            .disable::<PipelinedRenderingPlugin>()
            .disable::<SpritePlugin>()
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                close_when_requested: false,
            }),
    )
    .add_plugins(Mesh2dRenderPlugin)
    .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
        fixture_bytes(),
        BEVY_ATLAS_SIZE,
    ));
    ensure_render_app_ready(&mut app);
    for _ in 0..24 {
        if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
            exits.clear();
        }
        app.update();
    }
    app
}

fn bevy_gpu_available() -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut app = render_bevy_app();
        app.update();
    }))
    .is_ok()
}

fn clear_exit(app: &mut App) {
    if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
        exits.clear();
    }
}

fn run_bevy_updates(app: &mut App, frames: usize) {
    for _ in 0..frames {
        clear_exit(app);
        app.update();
    }
}

fn warmup_bevy_labels(app: &mut App, static_count: usize, damage_count: usize) -> Vec<Entity> {
    let damage = spawn_static_and_damage_labels(app, static_count, damage_count, LABEL_PX);
    run_bevy_updates(app, 1);
    damage
}

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
fn direct_lr5_harness_regressions_still_pass() {
    ci_bench_budget_gates_pass();
}

#[test]
fn bevy_noop_frames_do_not_reaggregate_or_resync() {
    let mut app = cpu_bevy_app();
    warmup_bevy_labels(&mut app, CI_BEVY_STATIC, 0);
    let before = text_perf_diagnostics(&app);

    run_bevy_updates(&mut app, 8);
    let after = text_perf_diagnostics(&app);

    assert_eq!(after.shape_rebuild_count, before.shape_rebuild_count);
    assert_eq!(
        after.aggregate_rebuild_count,
        before.aggregate_rebuild_count
    );
    assert_eq!(after.draw_entity_sync_count, before.draw_entity_sync_count);
    assert_eq!(after.atlas_sync_count, before.atlas_sync_count);
    assert_eq!(after.atlas_sync_bytes, before.atlas_sync_bytes);
}

#[test]
fn bevy_damage_churn_rebuilds_changed_labels_only() {
    let mut app = cpu_bevy_app();
    let damage = warmup_bevy_labels(&mut app, CI_BEVY_STATIC, CI_BEVY_DAMAGE);
    let before = text_perf_diagnostics(&app);

    for frame in 0..3_usize {
        for (index, entity) in damage.iter().enumerate() {
            let value = (index.wrapping_mul(17).wrapping_add(frame.wrapping_mul(13))) % 9999;
            app.world_mut()
                .entity_mut(*entity)
                .get_mut::<TextLabel>()
                .expect("label")
                .text = format!("-{value}");
        }
        run_bevy_updates(&mut app, 1);
    }

    let after = text_perf_diagnostics(&app);
    let shape_delta = after.shape_rebuild_count - before.shape_rebuild_count;
    assert_eq!(
        shape_delta,
        (CI_BEVY_DAMAGE * 3) as u64,
        "only changed damage labels should reshape"
    );
}

#[test]
fn bevy_damage_churn_aggregates_once_per_frame_not_per_label() {
    let mut app = cpu_bevy_app();
    let damage = warmup_bevy_labels(&mut app, CI_BEVY_STATIC, CI_BEVY_DAMAGE);
    let before = text_perf_diagnostics(&app);

    for (index, entity) in damage.iter().enumerate() {
        app.world_mut()
            .entity_mut(*entity)
            .get_mut::<TextLabel>()
            .expect("label")
            .text = format!("-{index}");
    }
    run_bevy_updates(&mut app, 1);

    let after = text_perf_diagnostics(&app);
    assert_eq!(
        after.aggregate_rebuild_count - before.aggregate_rebuild_count,
        1,
        "aggregate must rebuild once per frame, not per label"
    );
    assert_eq!(
        after.draw_entity_sync_count - before.draw_entity_sync_count,
        1,
        "draw entity sync must happen once per aggregate version change"
    );
}

#[test]
fn bevy_noop_frames_do_not_sync_full_atlas() {
    let mut app = cpu_bevy_app();
    warmup_bevy_labels(&mut app, CI_BEVY_STATIC, 0);
    let before = text_perf_diagnostics(&app);
    run_bevy_updates(&mut app, 6);
    let after = text_perf_diagnostics(&app);
    assert_eq!(after.atlas_sync_count, before.atlas_sync_count);
    assert_eq!(after.atlas_sync_bytes, before.atlas_sync_bytes);
}

#[test]
fn bevy_queue_remains_single_draw_entity_single_atlas_bind() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: render queue structural test");
        return;
    }

    let mut app = render_bevy_app();
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, 800, 600)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, 800, 600));
    warmup_bevy_labels(&mut app, 256, 32);
    run_bevy_updates(&mut app, 4);

    let (label_count, draw_entities) = text_label_entity_counts(&mut app);
    assert_eq!(draw_entities, 1, "one aggregate draw entity");
    assert!(label_count > 0);

    let queue = text_render_queue_state(&app);
    assert_eq!(queue.queued_draw_count, 1, "one instanced draw queued");
    assert!(queue.queued_instance_count > 0);
    assert_eq!(
        queue.queued_instance_count as usize,
        app.world()
            .get_resource::<simthing_tools::TextInstanceAggregate>()
            .map(|agg| agg.0.len())
            .unwrap_or(0)
    );
}

#[test]
fn bevy_noop_frames_do_not_recreate_instance_buffer() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: instance buffer reuse test");
        return;
    }

    let mut app = render_bevy_app();
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, 800, 600)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, 800, 600));
    warmup_bevy_labels(&mut app, 512, 64);
    run_bevy_updates(&mut app, 4);

    let before = text_perf_diagnostics(&app);
    run_bevy_updates(&mut app, 6);
    let after = text_perf_diagnostics(&app);

    assert_eq!(
        after.instance_buffer_create_count, before.instance_buffer_create_count,
        "no-op frames must not create new GPU instance buffers"
    );
    assert!(
        after.instance_buffer_reuse_count > before.instance_buffer_reuse_count,
        "no-op render prepares should reuse existing instance buffers"
    );
}

#[test]
fn binding_1k_budget_profile_records_avg_and_max_frame_cost() {
    let mut app = cpu_bevy_app();
    let damage = warmup_bevy_labels(&mut app, CI_BEVY_STATIC, CI_BEVY_DAMAGE);
    let profile = profile_bevy_text_bench(&mut app, &damage, 30, 10);

    eprintln!(
        "CI Bevy profile: labels={} damage={} avg_noop={:.3}ms max_noop={:.3}ms avg_damage={:.3}ms max_damage={:.3}ms",
        profile.labels,
        profile.damage_labels,
        profile.avg_noop_update_ms,
        profile.max_noop_update_ms,
        profile.avg_damage_update_ms,
        profile.max_damage_update_ms,
    );

    assert!(
        profile.avg_noop_update_ms < 1.0,
        "avg no-op CPU update must stay under 1 ms/frame (got {:.3}ms)",
        profile.avg_noop_update_ms
    );
    assert_eq!(
        profile.diagnostics_after_noop.aggregate_rebuild_count,
        profile.diagnostics_after_damage.aggregate_rebuild_count - 10
    );
}

#[test]
#[ignore = "manual binding proof: 5000 static + 500 damage labels"]
fn binding_5k_budget_profile_records_avg_and_max_frame_cost() {
    let mut app = cpu_bevy_app();
    let damage = warmup_bevy_labels(&mut app, BINDING_BEVY_STATIC, BINDING_BEVY_DAMAGE);
    let profile = profile_bevy_text_bench(&mut app, &damage, 60, 60);

    eprintln!("=== BINDING 5K BEVY PROFILE ===");
    eprintln!("labels={}", profile.labels);
    eprintln!("damage_labels={}", profile.damage_labels);
    eprintln!("avg_noop_update_ms={:.4}", profile.avg_noop_update_ms);
    eprintln!("max_noop_update_ms={:.4}", profile.max_noop_update_ms);
    eprintln!("avg_damage_update_ms={:.4}", profile.avg_damage_update_ms);
    eprintln!("max_damage_update_ms={:.4}", profile.max_damage_update_ms);
    eprintln!(
        "diagnostics_after_noop={:?}",
        profile.diagnostics_after_noop
    );
    eprintln!(
        "diagnostics_after_damage={:?}",
        profile.diagnostics_after_damage
    );

    assert_eq!(profile.labels, BINDING_BEVY_STATIC + BINDING_BEVY_DAMAGE);
    assert!(
        profile.avg_noop_update_ms < 1.0,
        "5k avg no-op must be <1 ms (got {:.4}ms)",
        profile.avg_noop_update_ms
    );
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
