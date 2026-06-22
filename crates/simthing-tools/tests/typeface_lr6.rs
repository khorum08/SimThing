use std::{fs, path::PathBuf};

use bevy::prelude::*;
use msdf_font::ttf_parser::Face;
use simthing_tools::{
    build_distance_field_instance, load_font, numeric_damage_lane_diagnostics,
    spawn_static_and_numeric_damage_labels, wgpu_instanced_text_smoke,
    wgpu_sdf_instanced_text_smoke, DistanceFieldAtlasCore, DistanceFieldError, DistanceFieldKind,
    GlyphInstanceGpu, SimthingToolsTextPlugin, TextGlyphInstances, TextLabel, WgpuSmokeTarget,
    DISTANCE_FIELD_RENDER_MSDF, DISTANCE_FIELD_RENDER_RASTER,
};

const FIXTURE: &[u8] = include_bytes!("../../simthing-workshop/assets/typeface/test_font.ttf");
const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;

fn fixture_bytes() -> Vec<u8> {
    FIXTURE.to_vec()
}

fn probe_font() -> simthing_tools::ProbeFont {
    load_font(&fixture_bytes()).expect("fixture font")
}

fn glyph_id_for_char(ch: char) -> u32 {
    let face = Face::parse(FIXTURE, 0).expect("parse fixture");
    face.glyph_index(ch).expect("glyph index").0.into()
}

fn atlas_region_pixels(
    atlas: &DistanceFieldAtlasCore,
    tile: &simthing_tools::DistanceFieldTile,
) -> Vec<u8> {
    let staging = atlas.staging_pixels();
    let size = atlas.atlas_size();
    let mut out = vec![0u8; (tile.atlas_tile.w * tile.atlas_tile.h * 4) as usize];
    for row in 0..tile.atlas_tile.h {
        let src_row = ((tile.atlas_tile.y + row) * size + tile.atlas_tile.x) * 4;
        let dst_row = row * tile.atlas_tile.w * 4;
        let len = (tile.atlas_tile.w * 4) as usize;
        out[dst_row as usize..dst_row as usize + len]
            .copy_from_slice(&staging[src_row as usize..src_row as usize + len]);
    }
    out
}

fn cpu_bevy_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .add_plugins(SimthingToolsTextPlugin::with_atlas_size(
            fixture_bytes(),
            4096,
        ));
    app
}

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
fn msdf_glyph_tile_is_deterministic() {
    let font = probe_font();
    let glyph_id = glyph_id_for_char('A');
    let mut a = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let mut b = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile_a = a
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("msdf a");
    let tile_b = b
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("msdf b");
    assert_eq!(tile_a, tile_b);
    assert_eq!(
        atlas_region_pixels(&a, &tile_a),
        atlas_region_pixels(&b, &tile_b)
    );
}

#[test]
fn same_glyph_same_px_msdf_is_cached() {
    let font = probe_font();
    let glyph_id = glyph_id_for_char('B');
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let first = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("first");
    let before = atlas.diagnostics();
    let second = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("second");
    let after = atlas.diagnostics();
    assert_eq!(first, second);
    assert_eq!(
        after.glyph_msdf_generate_count,
        before.glyph_msdf_generate_count
    );
    assert!(after.msdf_cache_hit_count > before.msdf_cache_hit_count);
}

#[test]
fn different_px_bucket_gets_distinct_msdf_tile() {
    let font = probe_font();
    let glyph_id = glyph_id_for_char('C');
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let small = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, 16.0)
        .expect("16px");
    let large = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, 48.0)
        .expect("48px");
    assert_ne!(small.atlas_tile, large.atlas_tile);
    assert_eq!(atlas.diagnostics().glyph_msdf_generate_count, 2);
}

#[test]
fn msdf_tile_has_distance_field_metadata() {
    let font = probe_font();
    let glyph_id = glyph_id_for_char('D');
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("tile");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);
    assert!(tile.px_range > 0.0);
    assert!(tile.atlas_tile.w > 0);
    assert!(tile.atlas_tile.h > 0);
}

#[test]
fn icon_msdf_deferred_raster_icon_path_preserved() {
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let icon = simthing_tools::IconVector {
        layers: Vec::new(),
        view_box: [0.0, 0.0, 24.0, 24.0],
    };
    let err = atlas
        .get_or_generate_icon_msdf(&icon, 0xe000, TEST_PX)
        .expect_err("icon msdf deferred");
    assert!(matches!(err, DistanceFieldError::IconDeferred(_)));
    assert_eq!(atlas.diagnostics().icon_msdf_generate_count, 0);
}

#[test]
fn sdf_shader_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: sdf_shader_smoke_draws_nonzero_pixels");
        return;
    }

    let font = probe_font();
    let glyph_id = glyph_id_for_char('S');
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("msdf tile");
    let instance = GlyphInstanceGpu {
        pos_size: [
            80.0,
            40.0,
            tile.atlas_tile.w as f32,
            tile.atlas_tile.h as f32,
        ],
        uv_rect: build_distance_field_instance(0.0, 0.0, &tile, atlas.atlas_size(), [1.0; 4])
            .uv_rect,
        color: [1.0, 1.0, 1.0, 1.0],
        sdf_params: [
            DISTANCE_FIELD_RENDER_MSDF,
            tile.px_range,
            atlas.atlas_size() as f32,
            0.0,
        ],
    };
    assert_eq!(instance.sdf_params[0], DISTANCE_FIELD_RENDER_MSDF);

    let smoke = match wgpu_sdf_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &[instance],
        atlas.staging_pixels(),
        atlas.atlas_size(),
    ) {
        Ok(result) => result,
        Err(err) if err.contains("no wgpu adapter") => {
            eprintln!("ADAPTER_SKIPPED: sdf_shader_smoke_draws_nonzero_pixels ({err})");
            return;
        }
        Err(err) => panic!("sdf smoke draw failed: {err}"),
    };

    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    assert!(
        target.has_alpha_text_pixels(&smoke.pixels),
        "stats={}",
        target.readback_pixel_stats(&smoke.pixels)
    );

    let png = encode_png_rgba(&smoke.pixels, SMOKE_WIDTH, SMOKE_HEIGHT);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/tests/typeface_lr6_sdf_smoke.png");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&path, &png).expect("write LR6 sdf smoke png");
    eprintln!("REAL_ADAPTER_OBSERVED: LR6 SDF shader smoke PNG written");
}

#[test]
fn raster_path_regression_still_draws() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: raster_path_regression_still_draws");
        return;
    }

    let mut app = cpu_bevy_app();
    app.world_mut().spawn(TextLabel {
        text: "LR6".into(),
        px: TEST_PX,
        color: [1.0, 1.0, 1.0, 1.0],
    });
    for _ in 0..2 {
        app.update();
    }

    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert!(!instances.is_empty());
    assert!(
        instances
            .iter()
            .all(|i| i.sdf_params[0] == DISTANCE_FIELD_RENDER_RASTER),
        "LR6 must keep raster as default render mode"
    );

    let atlas = app
        .world()
        .get_resource::<simthing_tools::TypefaceAtlas>()
        .expect("atlas");
    let smoke = match wgpu_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &instances,
        atlas.cpu.staging_pixels(),
        atlas.atlas_size,
    ) {
        Ok(result) => result,
        Err(err) if err.contains("no wgpu adapter") => {
            eprintln!("ADAPTER_SKIPPED: raster_path_regression_still_draws ({err})");
            return;
        }
        Err(err) => panic!("raster smoke failed: {err}"),
    };
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    assert!(target.has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn numeric_damage_lane_still_passes_binding_or_structural_guard() {
    let mut app = cpu_bevy_app();
    let damage = spawn_static_and_numeric_damage_labels(&mut app, 8, 4, TEST_PX);
    for _ in 0..2 {
        app.update();
    }
    let before = simthing_tools::text_perf_diagnostics(&app);
    let num_before = numeric_damage_lane_diagnostics(&app);
    for (index, entity) in damage.iter().enumerate() {
        app.world_mut()
            .entity_mut(*entity)
            .get_mut::<simthing_tools::NumericDamageLabel>()
            .expect("numeric label")
            .value = -((index + 1) as i32);
    }
    app.update();
    let after = simthing_tools::text_perf_diagnostics(&app);
    let num_after = numeric_damage_lane_diagnostics(&app);
    assert_eq!(
        after.shape_rebuild_count, before.shape_rebuild_count,
        "numeric lane must not invoke shaping after init"
    );
    assert_eq!(
        num_after.numeric_shape_bypass_count - num_before.numeric_shape_bypass_count,
        4
    );
    assert!(num_after.numeric_glyph_instance_patch_count > 0);
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
fn gpu_residency_audit_documented_for_lr6() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/tests/typeface_lr6_results.md");
    let text = fs::read_to_string(&path).expect("typeface_lr6_results.md must exist");
    for section in [
        "## GPU residency / CPU surfacing audit",
        "- CPU operations introduced:",
        "- CPU operations removed:",
        "- CPU operations retained and why:",
        "- Numeric production authority remains GPU-resident:",
        "- Deviations:",
        "- Next GPU-residency debt:",
    ] {
        assert!(
            text.contains(section),
            "missing required GPU residency audit section: {section}"
        );
    }
}
