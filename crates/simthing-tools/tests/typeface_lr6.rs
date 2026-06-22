use std::{fs, path::PathBuf};

use bevy::prelude::*;
use msdf_font::ttf_parser::Face;
use simthing_tools::{
    build_distance_field_instance, distance_field_diagnostics, load_font,
    numeric_damage_lane_diagnostics, spawn_static_and_numeric_damage_labels, text_perf_diagnostics,
    wgpu_instanced_text_smoke, wgpu_sdf_instanced_text_smoke, DistanceFieldAtlasCore,
    DistanceFieldError, DistanceFieldKind, GlyphInstanceGpu, IconVector, SimthingToolsTextPlugin,
    TextGlyphInstances, TextLabel, WgpuSmokeTarget, DISTANCE_FIELD_RENDER_MSDF,
    DISTANCE_FIELD_RENDER_RASTER,
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
fn icon_msdf_from_geometry_preserves_raster_icon_path() {
    let vector = IconVector::from_svg(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>"##,
    )
    .expect("vector");
    let mut df_atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = df_atlas
        .get_or_generate_icon_msdf(&vector, 0xf0001, TEST_PX)
        .expect("icon msdf");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);

    let mut raster_atlas = simthing_tools::GlyphAtlasCore::new(ATLAS_SIZE);
    let mut icons = simthing_tools::IconSet::new();
    let reg = icons
        .register_svg(
            0xf0001,
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>"##,
            TEST_PX,
            &mut raster_atlas,
        )
        .expect("raster icon");
    assert!(raster_atlas
        .tile_pixels(reg.tile)
        .chunks(4)
        .any(|px| px[3] > 0));
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
        style_params: [0.0; 4],
        deform_params: [0.0; 4],
        path_params: [0.0; 4],
        warp_params: [0.0; 4],
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
    app.world_mut()
        .spawn(TextLabel::raster("LR6", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
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
fn gpu_residency_audit_updated_for_lr6a() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/archive/typeface_track_2026_06/typeface_lr6a_results.md");
    let text = fs::read_to_string(&path).expect("typeface_lr6a_results.md must exist");
    for section in [
        "## GPU residency / CPU surfacing audit",
        "- CPU operations introduced:",
        "- CPU operations removed:",
        "- Numeric production authority remains GPU-resident:",
    ] {
        assert!(
            text.contains(section),
            "missing required LR6A GPU residency audit section: {section}"
        );
    }
}

#[test]
fn production_msdf_label_builds_msdf_instances() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::msdf("MSDF", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert!(!instances.is_empty());
    assert!(
        instances
            .iter()
            .all(|i| i.sdf_params[0] == DISTANCE_FIELD_RENDER_MSDF),
        "opt-in MSDF label must encode MSDF render mode"
    );
    let df = distance_field_diagnostics(&app);
    assert!(df.production_msdf_label_count >= 1);
    assert!(df.production_msdf_instance_count >= instances.len() as u64);
}

#[test]
fn raster_label_remains_default_render_mode() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("Raster", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert!(instances
        .iter()
        .all(|i| i.sdf_params[0] == DISTANCE_FIELD_RENDER_RASTER));
}

#[test]
fn production_msdf_label_noop_does_not_regenerate_distance_fields() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::msdf("Hold", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    let before = distance_field_diagnostics(&app);
    let perf_before = text_perf_diagnostics(&app);
    app.update();
    let after = distance_field_diagnostics(&app);
    let perf_after = text_perf_diagnostics(&app);
    assert_eq!(
        after.glyph_msdf_generate_count, before.glyph_msdf_generate_count,
        "no-op frame must not generate MSDF tiles"
    );
    assert_eq!(
        perf_after.shape_rebuild_count, perf_before.shape_rebuild_count,
        "no-op frame must not reshape"
    );
    assert_eq!(
        perf_after.instance_rebuild_count, perf_before.instance_rebuild_count,
        "no-op frame must not rebuild instances"
    );
}

#[test]
fn glyph_source_api_does_not_silently_claim_unsupported_glyph_ids() {
    let font = probe_font();
    let face = Face::parse(FIXTURE, 0).expect("parse fixture");
    let invalid = u32::from(face.number_of_glyphs()) + 100;
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let err = atlas
        .get_or_generate_glyph_msdf(&font, invalid, TEST_PX)
        .expect_err("glyph id without outline must error explicitly");
    assert!(matches!(err, DistanceFieldError::MissingOutline(_)));
}

#[test]
fn shaped_glyph_id_msdf_generation_supports_non_ascii_or_ligature_fixture() {
    let font = probe_font();
    let face = Face::parse(FIXTURE, 0).expect("parse fixture");
    let fi_glyph_id = face.glyph_index('ﬁ').map(|id| id.0 as u32).or_else(|| {
        let mut shaper =
            simthing_tools::ShapingEngine::new_with_font(fixture_bytes()).expect("shaper");
        shaper
            .shape("fi", TEST_PX)
            .glyphs
            .first()
            .map(|g| g.glyph_id as u32)
    });
    let glyph_id = fi_glyph_id.expect("fixture must expose fi or ligature glyph id");
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_glyph_msdf(&font, glyph_id, TEST_PX)
        .expect("ligature/non-ascii glyph id MSDF");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);
}

#[test]
fn mixed_raster_and_msdf_labels_share_instanced_pipeline() {
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::raster("A", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.world_mut()
        .spawn(TextLabel::msdf("B", TEST_PX, [1.0, 0.5, 0.2, 1.0]));
    app.update();
    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert!(instances
        .iter()
        .any(|i| i.sdf_params[0] == DISTANCE_FIELD_RENDER_RASTER));
    assert!(instances
        .iter()
        .any(|i| i.sdf_params[0] == DISTANCE_FIELD_RENDER_MSDF));
    let aggregate = app
        .world()
        .get_resource::<simthing_tools::TextInstanceAggregate>()
        .expect("aggregate");
    assert_eq!(aggregate.0.len(), instances.len());
}

#[test]
fn msdf_opt_in_raw_wgpu_smoke_draws_nonzero_pixels() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: msdf_opt_in_raw_wgpu_smoke_draws_nonzero_pixels");
        return;
    }
    let mut app = cpu_bevy_app();
    app.world_mut()
        .spawn(TextLabel::msdf("S", TEST_PX, [1.0, 1.0, 1.0, 1.0]));
    app.update();
    let instances: Vec<GlyphInstanceGpu> = {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&TextGlyphInstances, With<TextLabel>>();
        q.iter(world).flat_map(|i| i.0.iter().copied()).collect()
    };
    assert!(!instances.is_empty());
    let mut smoke_instances = instances;
    for inst in &mut smoke_instances {
        inst.pos_size[0] = 80.0;
        inst.pos_size[1] = 40.0;
    }
    let atlas = app
        .world()
        .get_resource::<simthing_tools::TypefaceAtlas>()
        .expect("atlas");
    let smoke = match wgpu_sdf_instanced_text_smoke(
        WgpuSmokeTarget {
            width: SMOKE_WIDTH,
            height: SMOKE_HEIGHT,
        },
        &smoke_instances,
        atlas.cpu.staging_pixels(),
        atlas.atlas_size,
    ) {
        Ok(result) => result,
        Err(err) if err.contains("no wgpu adapter") => {
            eprintln!("ADAPTER_SKIPPED: msdf_opt_in_raw_wgpu_smoke ({err})");
            return;
        }
        Err(err) => panic!("production msdf smoke failed: {err}"),
    };
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    assert!(target.has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn icon_msdf_is_implemented_from_icon_vector_geometry() {
    let defer_doc = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/archive/typeface_track_2026_06/typeface_lr6a_icon_msdf_deferred.md"),
    )
    .expect("icon deferral doc");
    assert!(defer_doc.contains("IMPLEMENTED"));
    let vector = IconVector::from_svg(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
  <path d="M 8 1 L 15 15 L 1 15 Z" fill="#ffffff"/>
</svg>"##,
    )
    .expect("vector");
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_icon_msdf(&vector, 0xf0000, TEST_PX)
        .expect("icon msdf implemented");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);
}

#[test]
fn lr5_numeric_damage_lane_still_passes() {
    numeric_damage_lane_still_passes_binding_or_structural_guard();
}
