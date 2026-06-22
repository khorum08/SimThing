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
    build_distance_field_instance, create_render_target_image, load_font,
    spawn_static_and_numeric_damage_labels, test_deform_table_skew, test_deform_table_stretch,
    test_style_table_gradient, test_style_table_solid_red, text_atlas_render_diagnostics,
    text_deform_diagnostics, text_deform_render_diagnostics, text_render_camera_bundle,
    text_style_render_diagnostics, wgpu_deformed_instanced_text_smoke, DistanceFieldAtlasCore,
    GlyphAtlasCore, GlyphInstanceGpu, IconLayerRole, IconSet, SimthingToolsTextPlugin,
    TextDeformParams, TextDeformTableResource, TextGlyphInstances, TextLabel, TextStyleTable,
    WgpuSmokeTarget, DEFORM_TESS_LEVEL_DEFORM, DEFORM_TESS_LEVEL_FLAT, DISTANCE_FIELD_RENDER_MSDF,
    ICON_PUA_START,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;
const DIAG_ATLAS_SIZE: u32 = 64;
const NEIGHBOR_RGBA: [u8; 4] = [255, 0, 255, 255];

const ROLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <rect data-simthing-role="background" x="1" y="1" width="14" height="14" fill="#202020"/>
  <circle data-simthing-role="accent" cx="8" cy="8" r="4" fill="#ffffff"/>
  <path data-simthing-role="outline" d="M 1 1 L 15 1 L 15 15 L 1 15 Z" fill="none" stroke="#ffffff"/>
</svg>
"##;

fn wgpu_adapter_available() -> bool {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .is_some()
}

fn cpu_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
            FIXTURE.to_vec(),
            4096,
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
        FIXTURE.to_vec(),
        4096,
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

fn warmup_render_flat_app(app: &mut App) {
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, 800, 600)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, 800, 600));
    app.world_mut()
        .spawn(TextLabel::raster("Flat", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    run_bevy_updates(app, 8);
}

fn warmup_render_deform_app(app: &mut App) {
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, 800, 600)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, 800, 600));
    {
        let mut deform = app.world_mut().resource_mut::<TextDeformTableResource>();
        deform
            .set_row(1, TextDeformParams::stretch(0.35, 0.0))
            .expect("deform slot 1");
    }
    app.world_mut()
        .spawn(TextLabel::raster("Deform", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_deform_slot(1));
    run_bevy_updates(app, 8);
}

fn raster_glyph_instance(atlas: &mut GlyphAtlasCore, atlas_size: u32) -> GlyphInstanceGpu {
    let font = load_font(FIXTURE).expect("font");
    let tile = atlas
        .get_or_rasterize(
            &font,
            font.glyph_metrics('A').expect("glyph").glyph_id,
            TEST_PX,
        )
        .expect("tile");
    let inv = 1.0 / atlas_size as f32;
    GlyphInstanceGpu {
        pos_size: [40.0, 40.0, tile.w as f32, tile.h as f32],
        uv_rect: [
            tile.x as f32 * inv,
            tile.y as f32 * inv,
            (tile.x + tile.w) as f32 * inv,
            (tile.y + tile.h) as f32 * inv,
        ],
        color: [1.0, 1.0, 1.0, 1.0],
        sdf_params: [0.0; 4],
        style_params: [0.0; 4],
        deform_params: [0.0; 4],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    }
}

fn diagnostic_border_atlas(deform_slot: f32) -> (Vec<u8>, u32, GlyphInstanceGpu) {
    let atlas_size = DIAG_ATLAS_SIZE;
    let tile_x = 8u32;
    let tile_y = 8u32;
    let tile_w = 24u32;
    let tile_h = 24u32;
    let mut pixels = vec![0u8; (atlas_size * atlas_size * 4) as usize];
    for px in pixels.chunks_mut(4) {
        px.copy_from_slice(&NEIGHBOR_RGBA);
    }
    for y in (tile_y + 4)..(tile_y + tile_h - 4) {
        for x in (tile_x + 4)..(tile_x + tile_w - 4) {
            let i = ((y * atlas_size + x) * 4) as usize;
            let mid = tile_x + tile_w / 2;
            if x < mid {
                pixels[i..i + 4].copy_from_slice(&[160, 160, 160, 255]);
            } else {
                pixels[i..i + 4].copy_from_slice(&[255, 255, 255, 255]);
            }
        }
    }
    let inv = 1.0 / atlas_size as f32;
    let instance = GlyphInstanceGpu {
        pos_size: [80.0, 40.0, tile_w as f32, tile_h as f32],
        uv_rect: [
            tile_x as f32 * inv,
            tile_y as f32 * inv,
            (tile_x + tile_w) as f32 * inv,
            (tile_y + tile_h) as f32 * inv,
        ],
        color: [1.0, 1.0, 1.0, 1.0],
        sdf_params: [0.0; 4],
        style_params: [0.0; 4],
        deform_params: [deform_slot, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    };
    (pixels, atlas_size, instance)
}

fn count_neighbor_magenta_pixels(pixels: &[u8]) -> usize {
    pixels
        .chunks(4)
        .filter(|px| px[3] > 32 && px[0] > 200 && px[1] < 64 && px[2] > 200)
        .count()
}

fn count_whiteish_pixels(pixels: &[u8]) -> usize {
    pixels
        .chunks(4)
        .filter(|px| px[3] > 32 && px[0] > 200 && px[1] > 200 && px[2] > 200)
        .count()
}

fn sample_region_avg(pixels: &[u8], width: u32, x0: u32, x1: u32, y0: u32, y1: u32) -> (u32, u32) {
    let mut r = 0u64;
    let mut g = 0u64;
    let mut n = 0u64;
    for y in y0..y1 {
        for x in x0..x1 {
            let i = ((y * width + x) * 4) as usize;
            if pixels[i + 3] > 0 {
                r += u64::from(pixels[i]);
                g += u64::from(pixels[i + 1]);
                n += 1;
            }
        }
    }
    if n == 0 {
        return (0, 0);
    }
    ((r / n) as u32, (g / n) as u32)
}

#[test]
fn deform_shader_preserves_source_atlas_uv() {
    let shader = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shaders/text_instanced.wgsl"),
    )
    .expect("shader");
    assert!(shader.contains("let source_uv = vertex.uv"));
    assert!(shader.contains("let deformed_uv = apply_parametric_deform(source_uv, deform_slot)"));
    assert!(shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)"));
    assert!(
        !shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, local_uv)"),
        "atlas UV must not use deformed local_uv"
    );
    assert!(
        !shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, deformed_uv)"),
        "atlas UV must not use deformed_uv"
    );
}

#[test]
fn stretch_deform_does_not_sample_outside_tile() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: stretch_deform_does_not_sample_outside_tile");
        return;
    }
    let (pixels, atlas_size, mut instance) = diagnostic_border_atlas(1.0);
    let smoke = wgpu_deformed_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        &pixels,
        atlas_size,
        &TextStyleTable::with_defaults(),
        &test_deform_table_stretch(),
        0.0,
    )
    .expect("stretch diagnostic smoke");
    assert_eq!(
        count_neighbor_magenta_pixels(&smoke.pixels),
        0,
        "stretch deform must not sample magenta neighbor atlas padding"
    );
    assert!(
        count_whiteish_pixels(&smoke.pixels) > 16,
        "stretch deform should still draw tile interior"
    );
    let _ = &mut instance;
}

#[test]
fn skew_deform_preserves_glyph_tile_coverage() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: skew_deform_preserves_glyph_tile_coverage");
        return;
    }
    let (pixels, atlas_size, instance) = diagnostic_border_atlas(2.0);
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let skew_smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[instance],
        &pixels,
        atlas_size,
        &TextStyleTable::with_defaults(),
        &test_deform_table_skew(),
        0.0,
    )
    .expect("skew diagnostic smoke");
    assert_eq!(
        count_neighbor_magenta_pixels(&skew_smoke.pixels),
        0,
        "skew deform must not bleed neighbor atlas color"
    );
    let left = sample_region_avg(&skew_smoke.pixels, target.width, 70, 110, 45, 75);
    let right = sample_region_avg(&skew_smoke.pixels, target.width, 120, 160, 45, 75);
    assert!(
        left.0.abs_diff(right.0) > 4,
        "skew should remap split tile interior across screen (left={left:?} right={right:?})"
    );
}

#[test]
fn msdf_deformed_label_preserves_static_tile_identity() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: msdf_deformed_label_preserves_static_tile_identity");
        return;
    }
    let font = load_font(FIXTURE).expect("font");
    let glyph_id = font.glyph_metrics('D').expect("glyph").glyph_id;
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, u32::from(glyph_id), TEST_PX)
        .expect("msdf");
    let base = build_distance_field_instance(0.0, 0.0, &tile, atlas.atlas_size(), [1.0; 4]);
    let flat = GlyphInstanceGpu {
        pos_size: [
            80.0,
            40.0,
            tile.atlas_tile.w as f32,
            tile.atlas_tile.h as f32,
        ],
        uv_rect: base.uv_rect,
        color: [1.0, 1.0, 1.0, 1.0],
        sdf_params: [
            DISTANCE_FIELD_RENDER_MSDF,
            tile.px_range,
            atlas.atlas_size() as f32,
            0.0,
        ],
        style_params: [0.0; 4],
        deform_params: [0.0; 4],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    };
    let mut deformed = flat;
    deformed.deform_params = [2.0, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0];
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let flat_smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[flat],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &TextStyleTable::with_defaults(),
        &TextDeformTableResource::default().table,
        0.0,
    )
    .expect("flat msdf smoke");
    let deformed_smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[deformed],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &TextStyleTable::with_defaults(),
        &test_deform_table_skew(),
        0.0,
    )
    .expect("deformed msdf smoke");
    let flat_alpha = flat_smoke.pixels.chunks(4).filter(|px| px[3] > 32).count();
    let deformed_alpha = deformed_smoke
        .pixels
        .chunks(4)
        .filter(|px| px[3] > 32)
        .count();
    assert!(flat_alpha > 0 && deformed_alpha > 0);
    let ratio = deformed_alpha as f32 / flat_alpha as f32;
    assert!(
        ratio > 0.25 && ratio < 4.0,
        "MSDF deformed coverage should stay bounded (flat={flat_alpha} deformed={deformed_alpha})"
    );
}

#[test]
fn gradient_coordinate_policy_documented_and_tested() {
    let doc = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../docs/archive/typeface_track_2026_06/typeface_lr6c_deform_uv_sampling_results.md",
    ))
    .expect("uv sampling results");
    assert!(doc.contains("## Gradient coordinate policy"));
    assert!(doc.contains("source_uv") || doc.contains("source_local_uv"));

    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: gradient_coordinate_policy_documented_and_tested smoke");
        return;
    }
    let (pixels, atlas_size, mut instance) = diagnostic_border_atlas(2.0);
    instance.style_params = [2.0, 0.0, 0.0, 0.0];
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[instance],
        &pixels,
        atlas_size,
        &test_style_table_gradient(),
        &test_deform_table_skew(),
        0.0,
    )
    .expect("gradient skew smoke");
    let left = sample_region_avg(&smoke.pixels, target.width, 70, 95, 45, 75);
    let right = sample_region_avg(&smoke.pixels, target.width, 120, 145, 45, 75);
    assert!(
        left.0.abs_diff(right.0) > 2 || left.1.abs_diff(right.1) > 2,
        "gradient should vary across source-local U under deformation (left={left:?} right={right:?})"
    );
}

#[test]
fn atlas_bind_group_created_once_then_reused() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: atlas_bind_group_created_once_then_reused");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_flat_app(&mut app);
    let after_warmup = text_atlas_render_diagnostics(&app);
    assert_eq!(after_warmup.atlas_bind_group_create_count, 1);

    run_bevy_updates(&mut app, 8);
    let after_noop = text_atlas_render_diagnostics(&app);
    assert_eq!(after_noop.atlas_bind_group_create_count, 1);
    assert!(
        after_noop.atlas_bind_group_reuse_count > after_warmup.atlas_bind_group_reuse_count,
        "atlas bind group should be reused across render prepares"
    );
}

#[test]
fn flat_labels_remain_one_quad_no_deform() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("Flat", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].deform_params[0], 0.0);
    assert_eq!(
        instances.0[0].deform_params[1],
        DEFORM_TESS_LEVEL_FLAT as f32
    );

    if bevy_gpu_available() {
        let mut render_app = render_bevy_app();
        warmup_render_flat_app(&mut render_app);
        let diag = text_deform_diagnostics(&render_app);
        assert_eq!(diag.tessellated_vertex_count, 0);
        assert_eq!(diag.deform_instance_count, 0);
    }
}

#[test]
fn deform_opt_in_sets_deform_params_or_slot() {
    let mut app = cpu_bevy_app();
    {
        let mut deform = app.world_mut().resource_mut::<TextDeformTableResource>();
        deform
            .set_row(1, TextDeformParams::stretch(0.25, 0.0))
            .expect("slot 1");
    }
    app.world_mut()
        .spawn(TextLabel::raster("Deform", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_deform_slot(1));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].deform_params[0], 1.0);
    assert!(
        instances.0[0].deform_params[1] > 0.0,
        "deformed label should carry tess level"
    );
}

#[test]
fn deform_opt_in_tessellates_only_when_needed() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("Flat", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.world_mut()
        .spawn(TextLabel::raster("Deform", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_deform_slot(1));
    {
        let mut deform = app.world_mut().resource_mut::<TextDeformTableResource>();
        deform
            .set_row(1, TextDeformParams::skew(0.3, 0.0))
            .expect("slot 1");
    }
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let mut flat_tess = None;
    let mut deform_tess = None;
    for instances in q.iter(world) {
        if instances.0[0].deform_params[0] == 0.0 {
            flat_tess = Some(instances.0[0].deform_params[1]);
        } else {
            deform_tess = Some(instances.0[0].deform_params[1]);
        }
    }
    assert_eq!(flat_tess, Some(DEFORM_TESS_LEVEL_FLAT as f32));
    assert_eq!(deform_tess, Some(DEFORM_TESS_LEVEL_DEFORM as f32));
}

#[test]
fn deform_noop_frames_do_not_retessellate() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: deform_noop_frames_do_not_retessellate");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_deform_app(&mut app);
    let before = text_deform_diagnostics(&app);
    run_bevy_updates(&mut app, 8);
    let after = text_deform_diagnostics(&app);
    assert_eq!(
        after.deformation_rebuild_count, before.deformation_rebuild_count,
        "stable deformation must not rebuild mesh"
    );
    assert!(
        after.deformation_noop_reuse_count > before.deformation_noop_reuse_count,
        "noop frames should increment deformation reuse"
    );
}

#[test]
fn deform_param_change_rebuilds_bounded_geometry_once() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: deform_param_change_rebuilds_bounded_geometry_once");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_flat_app(&mut app);
    let flat_before = text_deform_diagnostics(&app);

    app.world_mut()
        .spawn(TextLabel::raster("Deform", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_deform_slot(1));
    {
        let mut deform = app.world_mut().resource_mut::<TextDeformTableResource>();
        deform
            .set_row(1, TextDeformParams::fold(1.0, 0.0, 0.4))
            .expect("slot 1");
    }
    run_bevy_updates(&mut app, 6);
    let after_enable = text_deform_diagnostics(&app);
    assert!(
        after_enable.deformation_rebuild_count > flat_before.deformation_rebuild_count,
        "enabling deformation should rebuild draw mesh once"
    );

    run_bevy_updates(&mut app, 6);
    let after_noop = text_deform_diagnostics(&app);
    assert_eq!(
        after_noop.deformation_rebuild_count, after_enable.deformation_rebuild_count,
        "stable deformed label must not rebuild again"
    );
}

#[test]
fn vertex_shader_deform_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: vertex_shader_deform_smoke_draws_nonzero_pixels");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let mut instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    instance.deform_params = [1.0, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0];
    let style_table = TextStyleTable::with_defaults();
    let deform_table = test_deform_table_stretch();
    let smoke = wgpu_deformed_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style_table,
        &deform_table,
        0.0,
    )
    .expect("deformed smoke");
    assert!(WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    }
    .has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn fold_or_skew_deform_smoke_changes_pixel_distribution() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: fold_or_skew_deform_smoke_changes_pixel_distribution");
        return;
    }
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let base = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    let style_table = TextStyleTable::with_defaults();

    let mut flat = base;
    flat.deform_params = [0.0; 4];
    let flat_smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[flat],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style_table,
        &TextDeformTableResource::default().table,
        0.0,
    )
    .expect("flat smoke");

    let mut skewed = base;
    skewed.deform_params = [2.0, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0];
    let skew_smoke = wgpu_deformed_instanced_text_smoke(
        target,
        &[skewed],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style_table,
        &test_deform_table_skew(),
        0.0,
    )
    .expect("skew smoke");

    let diff_pixels = flat_smoke
        .pixels
        .chunks(4)
        .zip(skew_smoke.pixels.chunks(4))
        .filter(|(a, b)| {
            (i32::from(a[3]) - i32::from(b[3])).abs() > 8
                || (i32::from(a[0]) - i32::from(b[0])).abs() > 8
        })
        .count();
    assert!(
        diff_pixels > 16,
        "skew deformation should change pixel distribution (diff_pixels={diff_pixels})"
    );
}

#[test]
fn msdf_deformed_label_still_draws_with_style_slot() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: msdf_deformed_label_still_draws_with_style_slot");
        return;
    }
    let font = load_font(FIXTURE).expect("font");
    let glyph_id = font.glyph_metrics('D').expect("glyph").glyph_id;
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, u32::from(glyph_id), TEST_PX)
        .expect("msdf");
    let base = build_distance_field_instance(0.0, 0.0, &tile, atlas.atlas_size(), [1.0; 4]);
    let instance = GlyphInstanceGpu {
        pos_size: [
            80.0,
            40.0,
            tile.atlas_tile.w as f32,
            tile.atlas_tile.h as f32,
        ],
        uv_rect: base.uv_rect,
        color: [1.0, 1.0, 1.0, 1.0],
        sdf_params: [
            DISTANCE_FIELD_RENDER_MSDF,
            tile.px_range,
            atlas.atlas_size() as f32,
            0.0,
        ],
        style_params: [1.0, 0.0, 0.0, 0.0],
        deform_params: [1.0, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    };
    let style_table = test_style_table_solid_red();
    let deform_table = test_deform_table_stretch();
    let smoke = wgpu_deformed_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style_table,
        &deform_table,
        0.0,
    )
    .expect("msdf deformed styled smoke");
    let colored = smoke
        .pixels
        .chunks(4)
        .filter(|px| px[3] > 0 && (px[0] > 0 || px[1] > 0 || px[2] > 0))
        .count();
    assert!(colored > 0, "msdf deformed styled smoke should draw pixels");
}

#[test]
fn style_table_buffer_residency_still_passes() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: style_table_buffer_residency_still_passes");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_deform_app(&mut app);
    run_bevy_updates(&mut app, 6);
    let diag = text_style_render_diagnostics(&app);
    assert_eq!(diag.style_bind_group_create_count, 1);
    assert!(diag.style_bind_group_reuse_count >= 6);
    assert_eq!(diag.rows_buffer_create_count, 1);
}

#[test]
fn layered_icon_style_slots_still_pass() {
    let mut atlas = GlyphAtlasCore::new(256);
    let mut icons = IconSet::new();
    icons
        .register_svg(ICON_PUA_START + 3, ROLE_SVG, TEST_PX, &mut atlas)
        .expect("register");
    let instances = icons
        .build_layered_icon_style_instances(
            ICON_PUA_START + 3,
            TEST_PX,
            0.0,
            0.0,
            [1.0, 1.0, 1.0, 1.0],
            &[
                (IconLayerRole::Background, 1),
                (IconLayerRole::Accent, 2),
                (IconLayerRole::Outline, 3),
            ],
            atlas.atlas_size(),
        )
        .expect("layered");
    assert_eq!(instances.len(), 3);
    let slots: Vec<f32> = instances.iter().map(|i| i.style_params[0]).collect();
    assert_eq!(slots, vec![1.0, 2.0, 3.0]);
    assert!(instances.iter().all(|i| i.deform_params[0] == 0.0));
}

#[test]
fn lr5_numeric_damage_lane_still_passes() {
    let mut app = cpu_bevy_app();
    spawn_static_and_numeric_damage_labels(&mut app, 0, 4, TEST_PX);
    app.update();
    app.update();
    let world = app.world_mut();
    let mut labels = world.query_filtered::<(), With<simthing_tools::NumericDamageLabel>>();
    assert_eq!(labels.iter(world).count(), 4);
}

#[test]
fn semantic_free_guard_still_passes() {
    assert!(std::process::Command::new(env!("CARGO"))
        .args([
            "test",
            "-p",
            "simthing-tools",
            "--test",
            "semantic_free_guard",
        ])
        .status()
        .expect("spawn semantic guard")
        .success());
}

#[test]
fn gpu_residency_audit_documented_for_lr6c() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/archive/typeface_track_2026_06/typeface_lr6c_results.md"),
    )
    .expect("lr6c results");
    assert!(doc.contains("## GPU residency / CPU surfacing audit"));
    assert!(doc.contains("atlas bind group"));
    assert!(doc.contains("vertex shader"));
}

#[test]
fn deform_table_uploads_only_when_changed() {
    let mut table = TextDeformTableResource::default();
    let before = table.rows_generation;
    table.mark_rows_clean();
    assert!(!table.rows_dirty);
    table
        .set_row(1, TextDeformParams::stretch(0.2, 0.0))
        .expect("set row");
    assert!(table.rows_dirty);
    assert_eq!(table.rows_generation, before);
}

#[test]
fn deform_gpu_buffers_created_once_then_reused() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: deform_gpu_buffers_created_once_then_reused");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_deform_app(&mut app);
    let after_warmup = text_deform_render_diagnostics(&app);
    assert_eq!(after_warmup.rows_buffer_create_count, 1);
    assert_eq!(after_warmup.deform_bind_group_create_count, 1);

    run_bevy_updates(&mut app, 6);
    let after_noop = text_deform_render_diagnostics(&app);
    assert_eq!(after_noop.rows_buffer_create_count, 1);
    assert_eq!(after_noop.deform_bind_group_create_count, 1);
    assert!(after_noop.deform_bind_group_reuse_count > after_warmup.deform_bind_group_reuse_count);
}
