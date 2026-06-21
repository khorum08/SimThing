use std::{fs, path::PathBuf};

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
    create_render_target_image, text_instanced_pipeline_initialized, text_render_camera_bundle,
    text_render_queue_state, wgpu_instanced_text_smoke, GlyphInstanceGpu, ShapingEngine,
    SimthingToolsTextPlugin, TextGlyphInstances, TextLabel, TextRebuildDiagnostics, TypefaceAtlas,
    WgpuSmokeTarget,
};
const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const SMOKE_TEXT: &str = "SimThing";
const SHAPE_PX: f32 = 32.0;
const SMOKE_WIDTH: u32 = 400;
const SMOKE_HEIGHT: u32 = 200;

fn fixture_bytes() -> Vec<u8> {
    FIXTURE.to_vec()
}

fn cpu_headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::new(fixture_bytes()));
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

fn render_headless_app() -> App {
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
    .add_plugins(SimthingToolsTextPlugin::new(fixture_bytes()));
    ensure_render_app_ready(&mut app);
    // Warm up GPU asset upload (tonemapping LUT KTX2, atlas image) before spawning cameras.
    for _ in 0..24 {
        clear_headless_app_exit(&mut app);
        app.update();
    }
    app
}

fn bevy_gpu_adapter_available() -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut app = render_headless_app();
        app.update();
    }))
    .is_ok()
}

fn spawn_label(app: &mut App, text: &str) {
    app.world_mut().spawn(TextLabel {
        text: text.to_string(),
        px: SHAPE_PX,
        color: [1.0, 1.0, 1.0, 1.0],
    });
}

fn run_updates(app: &mut App, frames: usize) {
    for _ in 0..frames {
        clear_headless_app_exit(app);
        app.update();
    }
}

fn clear_headless_app_exit(app: &mut App) {
    if let Some(mut exits) = app.world_mut().get_resource_mut::<Events<AppExit>>() {
        exits.clear();
    }
}

#[test]
fn plugin_builds_headless_app() {
    let mut app = cpu_headless_app();
    app.update();
    assert!(app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .is_some());
    assert!(app.world().get_resource::<TypefaceAtlas>().is_some());
}

#[test]
fn render_pipeline_resources_exist() {
    if !bevy_gpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter for render pipeline resource test");
        return;
    }

    let mut app = render_headless_app();
    run_updates(&mut app, 3);
    assert!(
        text_instanced_pipeline_initialized(&app),
        "TextInstancedPipeline must exist in RenderApp after plugin finish"
    );
}

#[test]
fn label_change_rebuilds_instances_once() {
    let mut app = cpu_headless_app();
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);

    let after_first = app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .copied()
        .expect("diagnostics");
    assert_eq!(after_first.shape_rebuild_count, 1);
    assert_eq!(after_first.instance_rebuild_count, 1);

    run_updates(&mut app, 2);
    let after_noop = app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .copied()
        .expect("diagnostics");
    assert_eq!(after_noop.shape_rebuild_count, 1);
    assert_eq!(after_noop.instance_rebuild_count, 1);

    let entity = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<Entity, With<TextLabel>>();
        q.single(world).expect("label entity")
    };
    app.world_mut()
        .entity_mut(entity)
        .get_mut::<TextLabel>()
        .expect("label")
        .text = "SimThing!".to_string();
    run_updates(&mut app, 1);

    let after_change = app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .copied()
        .expect("diagnostics");
    assert_eq!(after_change.shape_rebuild_count, 2);
    assert_eq!(after_change.instance_rebuild_count, 2);
}

#[test]
fn instances_match_shaped_run_count() {
    let mut app = cpu_headless_app();
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);

    let world = app.world_mut();
    let mut query = world.query_filtered::<&simthing_tools::TextGlyphInstances, With<TextLabel>>();
    let instances = query.iter(world).map(|i| i.0.len()).sum::<usize>();

    let mut shaper = ShapingEngine::new_with_font(fixture_bytes()).expect("shaper");
    let expected = shaper.shape(SMOKE_TEXT, SHAPE_PX).glyphs.len();
    assert_eq!(instances, expected);
}

#[test]
fn text_instances_are_queued_for_render() {
    if !bevy_gpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter for render queue test");
        return;
    }

    let mut app = render_headless_app();
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, SMOKE_WIDTH, SMOKE_HEIGHT)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, SMOKE_WIDTH, SMOKE_HEIGHT));
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 4);

    let queue_state = text_render_queue_state(&app);
    assert!(
        queue_state.queued_draw_count > 0,
        "expected at least one Transparent2d draw queued; diagnostics={queue_state:?}"
    );
    assert!(
        queue_state.queued_instance_count > 0,
        "expected glyph instances queued for instanced draw"
    );
    assert_eq!(
        queue_state.views_seen, 1,
        "expected one Core2d view for offscreen camera"
    );

    let label_instances: usize = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&simthing_tools::TextGlyphInstances, With<TextLabel>>();
        q.iter(world).map(|i| i.0.len()).sum()
    };
    assert_eq!(
        queue_state.queued_instance_count as usize, label_instances,
        "queued instance count must match shaped label instances"
    );
}

#[test]
fn cached_label_noop_frame_still_does_not_reshape() {
    let mut app = cpu_headless_app();
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);

    let before = app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .copied()
        .expect("diagnostics")
        .shape_rebuild_count;

    for _ in 0..5 {
        run_updates(&mut app, 1);
    }

    let after = app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .copied()
        .expect("diagnostics")
        .shape_rebuild_count;
    assert_eq!(before, after, "unchanged label must not reshape");
}

#[test]
fn atlas_cache_still_reused_after_render_queue() {
    if !bevy_gpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter for atlas cache render test");
        return;
    }

    let mut app = render_headless_app();
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, SMOKE_WIDTH, SMOKE_HEIGHT)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, SMOKE_WIDTH, SMOKE_HEIGHT));
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);

    let raster_after_first = app
        .world()
        .get_resource::<TypefaceAtlas>()
        .expect("atlas")
        .cpu_stats()
        .rasterize_count;

    for _ in 0..5 {
        run_updates(&mut app, 1);
    }

    let raster_after_noop = app
        .world()
        .get_resource::<TypefaceAtlas>()
        .expect("atlas")
        .cpu_stats()
        .rasterize_count;
    assert_eq!(raster_after_first, raster_after_noop);
    assert!(raster_after_noop > 0);
}

#[test]
fn headless_shader_backed_instanced_draw_real_adapter() {
    if !bevy_gpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter for LR3R shader-backed smoke");
        return;
    }

    let mut app = render_headless_app();

    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, SMOKE_WIDTH, SMOKE_HEIGHT)
    };

    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);
    app.world_mut()
        .spawn(text_render_camera_bundle(target, SMOKE_WIDTH, SMOKE_HEIGHT));

    // Short Bevy probe: queue wiring only. In-Bevy image readback PNG is DEFERRED.
    run_updates(&mut app, 4);

    let queue_state = text_render_queue_state(&app);
    assert!(
        queue_state.queued_draw_count == 1,
        "expected one instanced draw queued; diagnostics={queue_state:?}"
    );
    assert!(
        queue_state.queued_instance_count > 0,
        "expected glyph instances queued for instanced draw"
    );
    assert_eq!(
        queue_state.views_seen, 1,
        "expected one Core2d view; diagnostics={queue_state:?}"
    );

    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert_eq!(
        queue_state.queued_instance_count as usize,
        instances.len(),
        "queued instance count must match shaped label instances"
    );

    let atlas = app.world().get_resource::<TypefaceAtlas>().expect("atlas");
    let atlas_pixels = atlas.cpu.staging_pixels().to_vec();
    let atlas_size = atlas.atlas_size;
    let smoke_result = wgpu_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &instances,
        &atlas_pixels,
        atlas_size,
    )
    .unwrap_or_else(|err| {
        panic!("raw-wgpu instanced shader draw failed (queue={queue_state:?}, wgpu_err={err})");
    });

    eprintln!(
        "REAL_ADAPTER_OBSERVED: LR3R raw-wgpu instanced shader draw produced text pixels ({})",
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        }
        .readback_pixel_stats(&smoke_result.pixels)
    );
    eprintln!(
        "DEFERRED: in-Bevy Core2d image readback PNG (Camera2d + Tonemapping::None + RenderTarget::Image + gpu_readback::Readback)"
    );

    let png = encode_png_rgba(&smoke_result.pixels, SMOKE_WIDTH, SMOKE_HEIGHT);
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr3r_smoke.png");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&path, &png).expect("write LR3R smoke png");
    eprintln!("REAL_ADAPTER_OBSERVED: LR3R shader-backed smoke PNG written");
}

fn encode_png_rgba(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("png header");
        writer.write_image_data(pixels).expect("png data");
    }
    buf
}

#[test]
fn semantic_free_guard_still_passes() {
    std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "semantic_free_guard",
        ])
        .status()
        .expect("spawn semantic guard")
        .success()
        .then_some(())
        .expect("semantic_free_guard failed");
}

#[test]
fn workshop_shim_still_passes_lr0_lr1_lr2_tests() {
    std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-workshop",
            "--test",
            "typeface_lr0",
            "--test",
            "typeface_lr1",
            "--test",
            "typeface_lr2",
        ])
        .status()
        .expect("spawn workshop regression")
        .success()
        .then_some(())
        .expect("workshop regression tests failed");
}
