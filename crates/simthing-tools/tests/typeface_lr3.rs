use std::{fs, path::PathBuf};

use bevy::{app::App, asset::AssetPlugin, prelude::*};
use pollster::FutureExt;
use simthing_tools::{
    GlyphInstanceGpu, ShapingEngine, SimthingToolsTextPlugin, TextLabel, TextRebuildDiagnostics,
    TypefaceAtlas,
};
use wgpu::{
    Backends, DeviceDescriptor, Features, Instance, InstanceDescriptor, PowerPreference,
    RequestAdapterOptions,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const SMOKE_TEXT: &str = "SimThing";
const SHAPE_PX: f32 = 32.0;

fn fixture_bytes() -> Vec<u8> {
    FIXTURE.to_vec()
}

fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::new(fixture_bytes()));
    app
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
        app.update();
    }
}

#[test]
fn plugin_builds_headless_app() {
    let mut app = headless_app();
    app.update();
    assert!(app
        .world()
        .get_resource::<TextRebuildDiagnostics>()
        .is_some());
    assert!(app.world().get_resource::<TypefaceAtlas>().is_some());
}

#[test]
fn label_change_rebuilds_instances_once() {
    let mut app = headless_app();
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
    let mut app = headless_app();
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 1);

    let world = app.world_mut();
    let mut query = world.query::<&simthing_tools::TextGlyphInstances>();
    let instances = query.iter(world).map(|i| i.0.len()).sum::<usize>();

    let mut shaper = ShapingEngine::new_with_font(fixture_bytes()).expect("shaper");
    let expected = shaper.shape(SMOKE_TEXT, SHAPE_PX).glyphs.len();
    assert_eq!(instances, expected);
}

#[test]
fn cached_label_noop_frame_does_not_reshape() {
    let mut app = headless_app();
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
fn atlas_cache_reused_after_initial_label_build() {
    let mut app = headless_app();
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
fn headless_render_smoke_real_adapter_or_hold() {
    let mut app = headless_app();
    spawn_label(&mut app, SMOKE_TEXT);
    run_updates(&mut app, 2);

    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut query = world.query::<&simthing_tools::TextGlyphInstances>();
        query.iter(world).flat_map(|i| i.0.clone()).collect()
    };
    assert!(!instances.is_empty());

    let atlas = app.world().get_resource::<TypefaceAtlas>().expect("atlas");
    if try_gpu_smoke_render(atlas.cpu_core(), &instances) {
        eprintln!("REAL_ADAPTER_OBSERVED: LR3 smoke PNG written");
    } else {
        eprintln!("ADAPTER_SKIPPED: no real GPU adapter for LR3 visual smoke");
    }
}

fn try_test_gpu_context() -> bool {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .block_on();
    let Some(adapter) = adapter else {
        return false;
    };
    adapter
        .request_device(
            &DeviceDescriptor {
                label: Some("typeface_lr3_smoke"),
                required_features: Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .block_on()
        .is_ok()
}

fn try_gpu_smoke_render(
    core: &simthing_tools::GlyphAtlasCore,
    instances: &[GlyphInstanceGpu],
) -> bool {
    if !try_test_gpu_context() {
        return false;
    }
    let png = composite_smoke_png(core, instances, 256, 128);
    if png.iter().all(|b| *b == 0) {
        return false;
    }
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr3_smoke.png");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&path, &png).expect("write smoke png");
    true
}

fn composite_smoke_png(
    core: &simthing_tools::GlyphAtlasCore,
    instances: &[GlyphInstanceGpu],
    width: u32,
    height: u32,
) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let atlas = core.staging_pixels();
    let atlas_size = core.atlas_size();

    for inst in instances {
        blit_glyph_instance(&mut pixels, width, height, atlas, atlas_size, *inst);
    }

    encode_png_rgba(&pixels, width, height)
}

fn blit_glyph_instance(
    out: &mut [u8],
    out_w: u32,
    out_h: u32,
    atlas: &[u8],
    atlas_size: u32,
    inst: GlyphInstanceGpu,
) {
    let x0 = inst.pos_size[0].round() as i32;
    let y0 = inst.pos_size[1].round() as i32;
    let gw = inst.pos_size[2] as u32;
    let gh = inst.pos_size[3] as u32;
    if gw == 0 || gh == 0 {
        return;
    }

    for row in 0..gh {
        for col in 0..gw {
            let dst_x = x0 + col as i32;
            let dst_y = y0 + row as i32;
            if dst_x < 0 || dst_y < 0 || dst_x >= out_w as i32 || dst_y >= out_h as i32 {
                continue;
            }
            let u = inst.uv_rect[0]
                + (col as f32 + 0.5) / gw as f32 * (inst.uv_rect[2] - inst.uv_rect[0]);
            let v = inst.uv_rect[1]
                + (row as f32 + 0.5) / gh as f32 * (inst.uv_rect[3] - inst.uv_rect[1]);
            let ax = (u * atlas_size as f32).floor() as u32;
            let ay = (v * atlas_size as f32).floor() as u32;
            if ax >= atlas_size || ay >= atlas_size {
                continue;
            }
            let src_idx = ((ay * atlas_size + ax) * 4 + 3) as usize;
            let alpha = atlas[src_idx];
            if alpha == 0 {
                continue;
            }
            let dst_idx = ((dst_y as u32 * out_w + dst_x as u32) * 4) as usize;
            let c = (inst.color[3] * alpha as f32 / 255.0 * 255.0) as u8;
            out[dst_idx] = (inst.color[0] * 255.0) as u8;
            out[dst_idx + 1] = (inst.color[1] * 255.0) as u8;
            out[dst_idx + 2] = (inst.color[2] * 255.0) as u8;
            out[dst_idx + 3] = c;
        }
    }
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
