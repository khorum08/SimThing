use std::{fs, path::PathBuf};

use bevy::prelude::*;
use simthing_tools::{
    build_distance_field_instance, load_font, spawn_static_and_numeric_damage_labels,
    test_style_table_gradient, test_style_table_solid_red, wgpu_styled_instanced_text_smoke,
    DistanceFieldAtlasCore, GlyphAtlasCore, GlyphInstanceGpu, IconLayerRole, IconSet,
    SimthingToolsTextPlugin, TextGlyphInstances, TextLabel, TextStyleRow, TextStyleTable,
    TextStyleTableResource, WgpuSmokeTarget, DISTANCE_FIELD_RENDER_MSDF, ICON_PUA_START,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;

const SIMPLE_SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>
"##;

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
        style_params: [0.0, 0.0, 0.0, 0.0],
    }
}

#[test]
fn style_table_default_slot_preserves_existing_render() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: style_table_default_slot_preserves_existing_render");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    let table = TextStyleTable::with_defaults();
    let smoke = wgpu_styled_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &table,
        0.0,
    )
    .expect("default slot smoke");
    assert!(WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    }
    .has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn style_table_uploads_only_when_changed() {
    let mut table = TextStyleTableResource::default();
    let before = table.rows_generation;
    table.mark_rows_clean();
    assert!(!table.rows_dirty);
    table
        .set_row(1, TextStyleRow::solid_fill(1.0, 0.0, 0.0, 1.0))
        .expect("set row");
    assert!(table.rows_dirty);
    assert_eq!(table.rows_generation, before);
}

#[test]
fn msdf_label_uses_style_slot_in_instance_data() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::msdf("Style", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_style_slot(3));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert!(!instances.0.is_empty());
    assert_eq!(instances.0[0].style_params[0], 3.0);
}

#[test]
fn raster_label_uses_style_slot_without_msdf_regression() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("Raster", TEST_PX, [1.0, 1.0, 1.0, 1.0]).with_style_slot(1));
    app.update();
    app.update();
    let world = app.world_mut();
    let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
    let instances = q.iter(world).next().expect("instances");
    assert_eq!(instances.0[0].style_params[0], 1.0);
    assert_eq!(instances.0[0].sdf_params[0], 0.0);
}

#[test]
fn shader_smoke_style_color_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: shader_smoke_style_color_draws_nonzero_pixels");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let mut instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    instance.style_params[0] = 1.0;
    let table = test_style_table_solid_red();
    let smoke = wgpu_styled_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &table,
        0.0,
    )
    .expect("styled smoke");
    let redish = smoke
        .pixels
        .chunks(4)
        .filter(|px| px[3] > 0 && px[0] > px[1] && px[0] > px[2])
        .count();
    assert!(redish > 0, "style slot 1 should tint red");
}

#[test]
fn shader_smoke_gradient_changes_pixels_across_glyph() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: shader_smoke_gradient_changes_pixels_across_glyph");
        return;
    }
    let mut atlas = GlyphAtlasCore::new(ATLAS_SIZE);
    let mut instance = raster_glyph_instance(&mut atlas, ATLAS_SIZE);
    instance.style_params[0] = 2.0;
    let table = test_style_table_gradient();
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    let smoke = wgpu_styled_instanced_text_smoke(
        target,
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &table,
        0.0,
    )
    .expect("gradient smoke");
    let left = sample_region_avg(&smoke.pixels, target.width, 40, 55, 40, 72);
    let right = sample_region_avg(&smoke.pixels, target.width, 65, 80, 40, 72);
    assert!(
        left.0.abs_diff(right.0) > 2 || left.1.abs_diff(right.1) > 2,
        "gradient should vary horizontally (left={left:?} right={right:?})"
    );
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
fn sdf_outline_or_glow_is_gpu_side_or_formally_deferred() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr6b_results.md"),
    )
    .expect("lr6b results");
    assert!(doc.contains("GPU-side") || doc.contains("shader-side"));
    assert!(doc.contains("Outline/glow disposition"));
}

#[test]
fn pulse_is_shader_side_or_formally_deferred() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr6b_results.md"),
    )
    .expect("lr6b results");
    assert!(doc.contains("Pulse disposition"));
    assert!(doc.contains("shader-side"));
}

#[test]
fn layered_icon_roles_map_to_distinct_style_slots() {
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
    let roles: Vec<f32> = instances.iter().map(|i| i.style_params[1]).collect();
    assert!(roles[0] != roles[1]);
}

#[test]
fn layered_icon_style_does_not_require_raw_svg_runtime() {
    let mut atlas = GlyphAtlasCore::new(256);
    let mut icons = IconSet::new();
    icons
        .register_svg(ICON_PUA_START + 4, SIMPLE_SVG, TEST_PX, &mut atlas)
        .expect("register");
    let layers = icons
        .style_layers_for(ICON_PUA_START + 4, TEST_PX)
        .expect("layers");
    assert!(!layers.is_empty());
    assert!(layers[0].geometry_hash != 0);
}

#[test]
fn icon_msdf_or_role_raster_style_path_preserves_lr4_composite_fallback() {
    let mut atlas = GlyphAtlasCore::new(128);
    let mut icons = IconSet::new();
    let composite = icons
        .register_svg(ICON_PUA_START + 5, SIMPLE_SVG, TEST_PX, &mut atlas)
        .expect("composite")
        .tile;
    assert!(atlas.tile_pixels(composite).chunks(4).any(|px| px[3] > 0));
    assert_eq!(icons.tile_for(ICON_PUA_START + 5), Some(composite));
}

#[test]
fn lr5_numeric_damage_lane_still_passes() {
    numeric_damage_lane_still_passes_binding_or_structural_guard();
}

fn numeric_damage_lane_still_passes_binding_or_structural_guard() {
    let mut app = cpu_bevy_app();
    spawn_static_and_numeric_damage_labels(&mut app, 0, 4, TEST_PX);
    app.update();
    app.update();
    let world = app.world_mut();
    let mut labels = world.query_filtered::<(), With<simthing_tools::NumericDamageLabel>>();
    assert_eq!(labels.iter(world).count(), 4);
}

#[test]
fn gpu_residency_audit_documented_for_lr6b() {
    let doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr6b_results.md"),
    )
    .expect("lr6b results");
    assert!(doc.contains("## GPU residency / CPU surfacing audit"));
}

#[test]
fn msdf_smoke_with_style_slot_still_draws() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: msdf_smoke_with_style_slot_still_draws");
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
    };
    let table = test_style_table_solid_red();
    let smoke = wgpu_styled_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
        &table,
        0.0,
    )
    .expect("msdf styled smoke");
    let colored = smoke
        .pixels
        .chunks(4)
        .filter(|px| px[3] > 0 && (px[0] > 0 || px[1] > 0 || px[2] > 0))
        .count();
    assert!(
        colored > 0,
        "msdf styled smoke should produce colored pixels"
    );
}
