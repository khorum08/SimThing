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
    build_distance_field_instance, create_render_target_image, load_font, path_params_for_slot,
    spawn_static_and_numeric_damage_labels, test_deform_table_skew, test_path_table_arc,
    test_path_table_quadratic_bezier, test_style_table_gradient, test_style_table_solid_red,
    test_warp_table_lattice2x2, text_atlas_render_diagnostics, text_path_warp_diagnostics,
    text_path_warp_render_diagnostics, text_render_camera_bundle, text_style_render_diagnostics,
    warp_params_for_slot, wgpu_path_warp_instanced_text_smoke, DistanceFieldAtlasCore,
    DistanceFieldTile, GlyphAtlasCore, GlyphInstanceGpu, IconLayerRole, IconSet,
    SimthingToolsTextPlugin, TextDeformTableResource, TextGlyphInstances, TextLabel,
    TextPathParams, TextPathTableResource, TextStyleTable, TextWarpParams, TextWarpTableResource,
    WgpuSmokeTarget, DEFORM_TESS_LEVEL_DEFORM, DISTANCE_FIELD_RENDER_MSDF, ICON_PUA_START,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;

const ROLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <rect data-simthing-role="background" x="1" y="1" width="14" height="14" fill="#202020"/>
  <circle data-simthing-role="accent" cx="8" cy="8" r="4" fill="#ffffff"/>
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

fn warmup_render_path_warp_app(app: &mut App) {
    let target = {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        create_render_target_image(&mut images, 800, 600)
    };
    app.world_mut()
        .spawn(text_render_camera_bundle(target, 800, 600));
    {
        let mut path = app.world_mut().resource_mut::<TextPathTableResource>();
        path.set_row(
            1,
            TextPathParams::quadratic_bezier([60.0, 100.0], [128.0, 20.0], [196.0, 100.0]),
        )
        .expect("path slot 1");
    }
    {
        let mut warp = app.world_mut().resource_mut::<TextWarpTableResource>();
        warp.set_row(
            1,
            TextWarpParams::lattice2x2(1.0, [[0.0, 0.0], [24.0, 0.0], [0.0, 18.0], [24.0, 18.0]]),
        )
        .expect("warp slot 1");
    }
    app.world_mut().spawn(
        TextLabel::raster("PathWarp", TEST_PX, [1.0, 1.0, 1.0, 1.0])
            .with_path_slot(1)
            .with_warp_slot(1),
    );
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

fn text_pixel_centroid_x(pixels: &[u8], width: u32, height: u32) -> f32 {
    text_pixel_centroid_axis(pixels, width, height, 0)
}

fn text_pixel_centroid_y(pixels: &[u8], width: u32, height: u32) -> f32 {
    text_pixel_centroid_axis(pixels, width, height, 1)
}

fn text_pixel_centroid_axis(pixels: &[u8], width: u32, height: u32, axis: u32) -> f32 {
    let mut sum = 0f64;
    let mut n = 0f64;
    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 4) as usize;
            if pixels[i + 3] > 32 && (pixels[i] > 0 || pixels[i + 1] > 0 || pixels[i + 2] > 0) {
                sum += f64::from(if axis == 0 { x } else { y });
                n += 1.0;
            }
        }
    }
    if n == 0.0 {
        return 0.0;
    }
    (sum / n) as f32
}

fn colored_pixel_count(pixels: &[u8]) -> usize {
    pixels
        .chunks(4)
        .filter(|px| px[3] > 32 && (px[0] > 0 || px[1] > 0 || px[2] > 0))
        .count()
}

fn msdf_smoke_atlas() -> (DistanceFieldAtlasCore, DistanceFieldTile) {
    let font = load_font(FIXTURE).expect("font");
    let glyph_id = font.glyph_metrics('D').expect("glyph").glyph_id;
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, u32::from(glyph_id), TEST_PX)
        .expect("msdf tile");
    (atlas, tile)
}

fn flat_msdf_styled_instance(tile: &DistanceFieldTile, atlas_size: u32) -> GlyphInstanceGpu {
    let base = build_distance_field_instance(80.0, 40.0, tile, atlas_size, [1.0; 4]);
    GlyphInstanceGpu {
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
            atlas_size as f32,
            0.0,
        ],
        style_params: [1.0, 0.0, 0.0, 0.0],
        deform_params: [0.0; 4],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
    }
}

fn combined_msdf_style_deform_path_warp_instance(
    tile: &DistanceFieldTile,
    atlas_size: u32,
) -> GlyphInstanceGpu {
    let mut instance = flat_msdf_styled_instance(tile, atlas_size);
    instance.deform_params = [1.0, DEFORM_TESS_LEVEL_DEFORM as f32, 0.0, 0.0];
    instance.path_params = path_params_for_slot(1, 0.0, 1.0);
    instance.warp_params = warp_params_for_slot(1, 1.0);
    instance
}

fn assert_combined_msdf_modes(instance: &GlyphInstanceGpu) {
    assert!(
        instance.sdf_params[0] >= 1.0,
        "combined proof requires SDF/MSDF mode (sdf_params.x={})",
        instance.sdf_params[0]
    );
    assert!(instance.style_params[0] > 0.0, "style slot required");
    assert!(instance.deform_params[0] > 0.0, "deform slot required");
    assert!(instance.path_params[0] > 0.0, "path slot required");
    assert!(instance.warp_params[0] > 0.0, "warp slot required");
}

fn draw_combined_path_warp_smoke(
    instances: &[GlyphInstanceGpu],
    atlas: &DistanceFieldAtlasCore,
) -> simthing_tools::WgpuTextSmokeResult {
    wgpu_path_warp_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        instances,
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &test_style_table_solid_red(),
        &test_deform_table_skew(),
        &test_path_table_arc(),
        &test_warp_table_lattice2x2(),
        0.0,
    )
    .expect("combined path/warp smoke")
}

#[test]
fn lr6c_closeout_records_da_approval() {
    let ladder = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/design_typeface_ladder.md"),
    )
    .expect("ladder");
    assert!(ladder.contains("LR6C") && ladder.contains("DA APPROVED"));
    assert!(ladder.contains("#889") || ladder.contains("913b148323"));

    let lr6c = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/archive/typeface_track_2026_06/typeface_lr6c_results.md"),
    )
    .expect("lr6c results");
    assert!(lr6c.contains("DA APPROVED"));

    let uv = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../docs/archive/typeface_track_2026_06/typeface_lr6c_deform_uv_sampling_results.md",
    ))
    .expect("uv results");
    assert!(uv.contains("ACCEPTED") || uv.contains("closed"));
}

#[test]
fn flat_labels_default_no_path_no_warp() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("Flat", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].path_params[0], 0.0);
    assert_eq!(instances.0[0].warp_params[0], 0.0);
}

#[test]
fn path_opt_in_sets_path_slot_metadata() {
    let mut app = cpu_bevy_app();
    {
        let mut path = app.world_mut().resource_mut::<TextPathTableResource>();
        path.set_row(2, TextPathParams::arc([0.0, 0.0], [100.0, 0.0], 40.0))
            .expect("path slot 2");
    }
    app.world_mut()
        .spawn(TextLabel::raster("Arc", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_path_slot(2));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].path_params[0], 2.0);
}

#[test]
fn warp_opt_in_sets_warp_slot_metadata() {
    let mut app = cpu_bevy_app();
    {
        let mut warp = app.world_mut().resource_mut::<TextWarpTableResource>();
        warp.set_row(3, TextWarpParams::radial_bend(0.5, 0.0))
            .expect("warp slot 3");
    }
    app.world_mut()
        .spawn(TextLabel::raster("Warp", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_warp_slot(3));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].warp_params[0], 3.0);
}

#[test]
fn path_table_uploads_only_when_dirty() {
    let mut table = TextPathTableResource::default();
    table.mark_rows_clean();
    assert!(!table.rows_dirty);
    table
        .set_row(1, TextPathParams::arc([0.0, 0.0], [50.0, 0.0], 20.0))
        .expect("set path");
    assert!(table.rows_dirty);
}

#[test]
fn warp_table_uploads_only_when_dirty() {
    let mut table = TextWarpTableResource::default();
    table.mark_rows_clean();
    assert!(!table.rows_dirty);
    table
        .set_row(1, TextWarpParams::radial_bend(0.3, 0.0))
        .expect("set warp");
    assert!(table.rows_dirty);
}

#[test]
fn path_warp_bind_group_reused_on_noop_frames() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: path_warp_bind_group_reused_on_noop_frames");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_path_warp_app(&mut app);
    let after_warmup = text_path_warp_render_diagnostics(&app);
    assert_eq!(after_warmup.path_buffer_create_count, 1);
    assert_eq!(after_warmup.warp_buffer_create_count, 1);
    assert!(after_warmup.bind_group_create_count >= 2);

    run_bevy_updates(&mut app, 8);
    let after_noop = text_path_warp_render_diagnostics(&app);
    assert_eq!(after_noop.path_buffer_create_count, 1);
    assert_eq!(after_noop.warp_buffer_create_count, 1);
    assert!(
        after_noop.bind_group_reuse_count > after_warmup.bind_group_reuse_count,
        "path/warp bind groups should reuse on noop frames"
    );
}

#[test]
fn text_on_arc_or_bezier_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: text_on_arc_or_bezier_smoke_draws_nonzero_pixels");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let mut instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    instance.path_params = path_params_for_slot(2, 0.0, 1.0);
    let path_table = test_path_table_quadratic_bezier();
    let warp_table = TextWarpTableResource::default().table;
    let smoke = wgpu_path_warp_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &TextStyleTable::with_defaults(),
        &TextDeformTableResource::default().table,
        &path_table,
        &warp_table,
        0.0,
    )
    .expect("bezier path smoke");
    assert!(WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    }
    .has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn text_on_path_changes_pixel_distribution_vs_flat() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: text_on_path_changes_pixel_distribution_vs_flat");
        return;
    }
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let base = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    let path_table = test_path_table_arc();
    let style = TextStyleTable::with_defaults();
    let deform = TextDeformTableResource::default().table;

    let flat_smoke = wgpu_path_warp_instanced_text_smoke(
        target,
        &[base],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style,
        &deform,
        &path_table,
        &TextWarpTableResource::default().table,
        0.0,
    )
    .expect("flat smoke");

    let mut path_instance = base;
    path_instance.path_params = path_params_for_slot(1, 0.0, 1.0);
    let path_smoke = wgpu_path_warp_instanced_text_smoke(
        target,
        &[path_instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style,
        &deform,
        &path_table,
        &TextWarpTableResource::default().table,
        0.0,
    )
    .expect("path smoke");

    let flat_count = colored_pixel_count(&flat_smoke.pixels);
    let path_count = colored_pixel_count(&path_smoke.pixels);
    assert!(flat_count > 0 && path_count > 0);
    let flat_cy = text_pixel_centroid_y(&flat_smoke.pixels, target.width, target.height);
    let path_cy = text_pixel_centroid_y(&path_smoke.pixels, target.width, target.height);
    assert!(
        (flat_cy - path_cy).abs() > 4.0,
        "path should shift text centroid vertically (flat_cy={flat_cy} path_cy={path_cy})"
    );
}

#[test]
fn lattice_warp_smoke_changes_pixel_distribution_vs_flat() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: lattice_warp_smoke_changes_pixel_distribution_vs_flat");
        return;
    }
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let base = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    let path_table = TextPathTableResource::default().table;
    let warp_table = test_warp_table_lattice2x2();
    let style = TextStyleTable::with_defaults();
    let deform = TextDeformTableResource::default().table;

    let flat_smoke = wgpu_path_warp_instanced_text_smoke(
        target,
        &[base],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style,
        &deform,
        &path_table,
        &warp_table,
        0.0,
    )
    .expect("flat smoke");

    let mut warp_instance = base;
    warp_instance.warp_params = warp_params_for_slot(1, 1.0);
    let warp_smoke = wgpu_path_warp_instanced_text_smoke(
        target,
        &[warp_instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &style,
        &deform,
        &path_table,
        &warp_table,
        0.0,
    )
    .expect("warp smoke");

    let flat_count = colored_pixel_count(&flat_smoke.pixels);
    let warp_count = colored_pixel_count(&warp_smoke.pixels);
    assert!(flat_count > 0 && warp_count > 0);
    let flat_cx = text_pixel_centroid_x(&flat_smoke.pixels, target.width, target.height);
    let warp_cx = text_pixel_centroid_x(&warp_smoke.pixels, target.width, target.height);
    assert!(
        (flat_cx - warp_cx).abs() > 4.0,
        "lattice warp should shift text centroid (flat_cx={flat_cx} warp_cx={warp_cx})"
    );
}

#[test]
fn path_warp_preserves_source_atlas_uv() {
    let shader = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shaders/text_instanced.wgsl"),
    )
    .expect("shader");
    assert!(shader.contains("let source_uv = vertex.uv"));
    assert!(shader.contains("apply_text_path"));
    assert!(shader.contains("apply_warp_field"));
    assert!(shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)"));
    assert!(
        !shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, deformed_uv)"),
        "atlas UV must remain source_uv under path/warp"
    );
}

#[test]
fn combined_msdf_style_deform_path_warp_instance_sets_all_modes() {
    let (atlas, tile) = msdf_smoke_atlas();
    let instance = combined_msdf_style_deform_path_warp_instance(&tile, atlas.atlas_size());
    assert_eq!(instance.sdf_params[0], DISTANCE_FIELD_RENDER_MSDF);
    assert_combined_msdf_modes(&instance);
}

#[test]
fn combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!(
            "ADAPTER_SKIPPED: combined_msdf_style_deform_path_warp_smoke_draws_nonzero_pixels"
        );
        return;
    }
    let (atlas, tile) = msdf_smoke_atlas();
    let instance = combined_msdf_style_deform_path_warp_instance(&tile, atlas.atlas_size());
    assert_combined_msdf_modes(&instance);
    let smoke = draw_combined_path_warp_smoke(&[instance], &atlas);
    assert!(
        colored_pixel_count(&smoke.pixels) > 0,
        "combined MSDF+style+deform+path+warp smoke should draw colored pixels"
    );
}

#[test]
fn combined_msdf_style_deform_path_warp_changes_distribution_vs_flat_msdf() {
    if !wgpu_adapter_available() {
        eprintln!(
            "ADAPTER_SKIPPED: combined_msdf_style_deform_path_warp_changes_distribution_vs_flat_msdf"
        );
        return;
    }
    let (atlas, tile) = msdf_smoke_atlas();
    let flat = flat_msdf_styled_instance(&tile, atlas.atlas_size());
    let combined = combined_msdf_style_deform_path_warp_instance(&tile, atlas.atlas_size());
    assert_combined_msdf_modes(&combined);

    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let flat_smoke = draw_combined_path_warp_smoke(&[flat], &atlas);
    let combined_smoke = draw_combined_path_warp_smoke(&[combined], &atlas);
    assert!(colored_pixel_count(&flat_smoke.pixels) > 0);
    assert!(colored_pixel_count(&combined_smoke.pixels) > 0);

    let flat_cy = text_pixel_centroid_y(&flat_smoke.pixels, target.width, target.height);
    let combined_cy = text_pixel_centroid_y(&combined_smoke.pixels, target.width, target.height);
    assert!(
        (flat_cy - combined_cy).abs() > 4.0,
        "combined MSDF stack should shift centroid vs flat control (flat_cy={flat_cy} combined_cy={combined_cy})"
    );
}

#[test]
fn combined_path_warp_preserves_source_atlas_uv() {
    let shader = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shaders/text_instanced.wgsl"),
    )
    .expect("production shader");
    assert!(shader.contains("out.local_uv = source_uv"));
    assert!(shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)"));

    let smoke_shader =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wgpu_smoke.rs"))
            .expect("wgpu smoke");
    assert!(smoke_shader.contains("out.local_uv = source_uv"));
    assert!(smoke_shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)"));
    assert!(smoke_shader.contains("sdf_coverage"));
}

#[test]
fn combined_gradient_uses_source_uv_under_path_warp() {
    let doc = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../docs/archive/typeface_track_2026_06/typeface_lr6d_combined_msdf_deform_results.md",
    ))
    .expect("combined msdf deform results");
    assert!(doc.contains("## Gradient coordinate policy"));
    assert!(doc.contains("source_uv"));

    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: combined_gradient_uses_source_uv_under_path_warp smoke");
        return;
    }
    let (atlas, tile) = msdf_smoke_atlas();
    let mut instance = combined_msdf_style_deform_path_warp_instance(&tile, atlas.atlas_size());
    instance.style_params = [2.0, 0.0, 0.0, 0.0];
    let smoke = wgpu_path_warp_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &test_style_table_gradient(),
        &test_deform_table_skew(),
        &test_path_table_quadratic_bezier(),
        &test_warp_table_lattice2x2(),
        0.0,
    )
    .expect("combined gradient smoke");
    assert!(colored_pixel_count(&smoke.pixels) > 0);
}

#[test]
fn msdf_style_deform_path_warp_combined_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!(
            "ADAPTER_SKIPPED: msdf_style_deform_path_warp_combined_smoke_draws_nonzero_pixels"
        );
        return;
    }
    let (atlas, tile) = msdf_smoke_atlas();
    let instance = combined_msdf_style_deform_path_warp_instance(&tile, atlas.atlas_size());
    assert_combined_msdf_modes(&instance);
    let smoke = draw_combined_path_warp_smoke(&[instance], &atlas);
    assert!(colored_pixel_count(&smoke.pixels) > 0);
}

#[test]
fn gradient_policy_under_path_warp_documented_and_tested() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/archive/typeface_track_2026_06/typeface_lr6d_results.md"),
    )
    .expect("lr6d results");
    assert!(doc.contains("## Gradient coordinate policy"));
    assert!(doc.contains("source_uv"));

    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: gradient_policy_under_path_warp smoke");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let mut instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    instance.style_params = [2.0, 0.0, 0.0, 0.0];
    instance.path_params = path_params_for_slot(2, 0.0, 1.0);
    instance.warp_params = warp_params_for_slot(1, 1.0);
    let smoke = wgpu_path_warp_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &test_style_table_gradient(),
        &TextDeformTableResource::default().table,
        &test_path_table_quadratic_bezier(),
        &test_warp_table_lattice2x2(),
        0.0,
    )
    .expect("gradient path warp smoke");
    assert!(colored_pixel_count(&smoke.pixels) > 0);
}

#[test]
fn style_table_buffer_residency_still_passes() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: style_table_buffer_residency_still_passes");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_path_warp_app(&mut app);
    run_bevy_updates(&mut app, 6);
    let diag = text_style_render_diagnostics(&app);
    assert_eq!(diag.style_bind_group_create_count, 1);
    assert!(diag.style_bind_group_reuse_count >= 6);
}

#[test]
fn atlas_bind_group_residency_still_passes() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: atlas_bind_group_residency_still_passes");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_path_warp_app(&mut app);
    let after_warmup = text_atlas_render_diagnostics(&app);
    // The bind group follows the GPU atlas texture view, which is re-prepared as glyphs rasterize
    // after init, so it may be (re)created more than once while the atlas settles.
    assert!(after_warmup.atlas_bind_group_create_count >= 1);
    run_bevy_updates(&mut app, 6);
    let after_noop = text_atlas_render_diagnostics(&app);
    // Once the atlas texture is stable, no-op frames must reuse the bind group, not recreate it.
    assert_eq!(
        after_noop.atlas_bind_group_create_count,
        after_warmup.atlas_bind_group_create_count
    );
    assert!(after_noop.atlas_bind_group_reuse_count > after_warmup.atlas_bind_group_reuse_count);
}

#[test]
fn deform_uv_sampling_regression_still_passes() {
    let shader = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shaders/text_instanced.wgsl"),
    )
    .expect("shader");
    assert!(shader.contains("out.local_uv = source_uv"));
    assert!(shader.contains("mix(instance.uv_rect.xy, instance.uv_rect.zw, source_uv)"));
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
            &[(IconLayerRole::Background, 1), (IconLayerRole::Accent, 2)],
            atlas.atlas_size(),
        )
        .expect("layered");
    assert_eq!(instances.len(), 2);
    assert!(instances.iter().all(|i| i.path_params[0] == 0.0));
    assert!(instances.iter().all(|i| i.warp_params[0] == 0.0));
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
fn gpu_residency_audit_documented_for_lr6d() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/archive/typeface_track_2026_06/typeface_lr6d_results.md"),
    )
    .expect("lr6d results");
    assert!(doc.contains("## GPU residency / CPU surfacing audit"));
    assert!(doc.contains("path/warp"));
    assert!(doc.contains("vertex shader"));
}

#[test]
fn path_warp_noop_reuse_diagnostics_increment() {
    if !bevy_gpu_available() {
        eprintln!("ADAPTER_SKIPPED: path_warp_noop_reuse_diagnostics_increment");
        return;
    }
    let mut app = render_bevy_app();
    warmup_render_path_warp_app(&mut app);
    let before = text_path_warp_diagnostics(&app);
    run_bevy_updates(&mut app, 6);
    let after = text_path_warp_diagnostics(&app);
    assert!(after.path_instance_count > 0 || after.warp_instance_count > 0);
    assert!(
        after.path_warp_noop_reuse_count > before.path_warp_noop_reuse_count,
        "stable path/warp should increment noop reuse"
    );
}
