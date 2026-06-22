use std::{fs, path::PathBuf};

use simthing_tools::{
    build_distance_field_instance, wgpu_sdf_instanced_text_smoke, DistanceFieldAtlasCore,
    DistanceFieldError, DistanceFieldKind, GlyphAtlasCore, IconLayerRole, IconSet, IconVector,
    WgpuSmokeTarget, ICON_PUA_START,
};

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

const TEST_PX: f32 = 32.0;
const ATLAS_SIZE: u32 = 512;
const SMOKE_WIDTH: u32 = 256;
const SMOKE_HEIGHT: u32 = 128;

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

#[test]
fn icon_vector_extracts_normalized_geometry() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("normalize");
    assert_eq!(vector.layers.len(), 1);
    let path = &vector.layers[0].paths[0];
    assert!(path.commands.len() >= 4);
    assert!(path.bounds[2] > 0.0);
    assert!(path.bounds[3] > 0.0);
}

#[test]
fn icon_vector_preserves_role_layer_order() {
    let vector = IconVector::from_svg(ROLE_SVG).expect("role vector");
    let roles = vector.layer_roles().collect::<Vec<_>>();
    assert_eq!(
        roles,
        vec![
            IconLayerRole::Background,
            IconLayerRole::Accent,
            IconLayerRole::Outline,
        ]
    );
}

#[test]
fn icon_vector_rejects_dynamic_svg_still() {
    let dynamic = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16">
  <script>alert(1)</script>
  <rect x="1" y="1" width="14" height="14"/>
</svg>
"##;
    let err = IconVector::from_svg(dynamic).expect_err("dynamic svg");
    assert!(err.to_string().contains("StaticOnly") || err.to_string().contains("script"));
}

#[test]
fn icon_vector_geometry_is_deterministic() {
    let a = IconVector::from_svg(SIMPLE_SVG).expect("a");
    let b = IconVector::from_svg(SIMPLE_SVG).expect("b");
    assert_eq!(a, b);
    assert_eq!(a.geometry_hash(), b.geometry_hash());
}

#[test]
fn icon_vector_has_no_raw_svg_runtime_dependency() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("vector");
    for layer in &vector.layers {
        for path in &layer.paths {
            assert!(!path.commands.is_empty());
            let sig = path.debug_signature();
            assert!(sig.starts_with('M'));
        }
    }
}

#[test]
fn icon_msdf_generates_or_explicitly_defers_after_geometry() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("vector");
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_icon_msdf(&vector, ICON_PUA_START + 1, TEST_PX)
        .expect("icon msdf from geometry");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);
    assert!(tile.px_range > 0.0);
    assert_eq!(atlas.diagnostics().icon_msdf_generate_count, 1);
}

#[test]
fn same_icon_same_px_msdf_or_layered_raster_is_cached() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("vector");
    let mut df_atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let first = df_atlas
        .get_or_generate_icon_msdf(&vector, ICON_PUA_START + 1, TEST_PX)
        .expect("first msdf");
    let second = df_atlas
        .get_or_generate_icon_msdf(&vector, ICON_PUA_START + 1, TEST_PX)
        .expect("cached msdf");
    assert_eq!(first, second);
    assert_eq!(df_atlas.diagnostics().icon_msdf_generate_count, 1);
    assert_eq!(df_atlas.diagnostics().msdf_cache_hit_count, 1);

    let mut raster_atlas = GlyphAtlasCore::new(256);
    let mut icons = IconSet::new();
    icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, TEST_PX, &mut raster_atlas)
        .expect("register");
    let before = raster_atlas.tile_count();
    icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, TEST_PX, &mut raster_atlas)
        .expect("cached register");
    assert_eq!(raster_atlas.tile_count(), before);
}

#[test]
fn role_layered_icon_data_is_ready_for_style_slots() {
    let vector = IconVector::from_svg(ROLE_SVG).expect("vector");
    let accent_paths = vector.paths_for_role(IconLayerRole::Accent);
    assert_eq!(accent_paths.len(), 1);

    let mut atlas = GlyphAtlasCore::new(256);
    let mut icons = IconSet::new();
    icons
        .register_svg(ICON_PUA_START + 2, ROLE_SVG, TEST_PX, &mut atlas)
        .expect("register");
    let layers = icons
        .style_layers_for(ICON_PUA_START + 2, TEST_PX)
        .expect("style layers");
    assert_eq!(layers.len(), 3);
    assert!(layers
        .iter()
        .any(|layer| layer.role == IconLayerRole::Accent));
    for layer in &layers {
        assert!(layer.geometry_hash != 0);
        assert!(layer.raster_tile.w > 0);
    }
}

#[test]
fn lr4_raster_icon_path_still_passes() {
    let mut atlas = GlyphAtlasCore::new(128);
    let mut icons = IconSet::new();
    let tile = icons
        .register_svg(ICON_PUA_START + 1, SIMPLE_SVG, TEST_PX, &mut atlas)
        .expect("register")
        .tile;
    let pixels = atlas.tile_pixels(tile);
    assert!(pixels.chunks(4).any(|px| px[3] > 0));
}

#[test]
fn gpu_residency_audit_documented_for_icon_geometry() {
    let doc =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "../../docs/archive/typeface_track_2026_06/typeface_lr6a_icon_geometry_results.md",
        ))
        .expect("geometry results doc");
    assert!(doc.contains("## GPU residency / CPU surfacing audit"));
    assert!(doc.contains("import-time static SVG parsing"));
}

#[test]
fn icon_msdf_tile_has_distance_field_metadata() {
    let vector = IconVector::from_svg(SIMPLE_SVG).expect("vector");
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_icon_msdf(&vector, ICON_PUA_START + 1, TEST_PX)
        .expect("msdf tile");
    assert_eq!(tile.kind, DistanceFieldKind::Msdf);
    assert!(tile.px_range > 0.0);
    assert!(tile.atlas_tile.w > 0);
    assert!(tile.atlas_tile.h > 0);
}

#[test]
fn icon_msdf_raw_wgpu_smoke_draws_nonzero_pixels_or_adapter_skipped() {
    if !wgpu_adapter_available() {
        eprintln!("ADAPTER_SKIPPED: icon_msdf_raw_wgpu_smoke");
        return;
    }

    let vector = IconVector::from_svg(SIMPLE_SVG).expect("vector");
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let tile = atlas
        .get_or_generate_icon_msdf(&vector, ICON_PUA_START + 1, TEST_PX)
        .expect("icon msdf");
    let instance =
        build_distance_field_instance(40.0, 40.0, &tile, atlas.atlas_size(), [1.0, 1.0, 1.0, 1.0]);
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
            eprintln!("ADAPTER_SKIPPED: icon_msdf_raw_wgpu_smoke ({err})");
            return;
        }
        Err(err) => panic!("icon msdf smoke failed: {err}"),
    };
    let target = WgpuSmokeTarget {
        width: SMOKE_WIDTH,
        height: SMOKE_HEIGHT,
    };
    assert!(target.has_alpha_text_pixels(&smoke.pixels));
}

#[test]
fn icon_msdf_empty_geometry_still_defers() {
    let icon = IconVector {
        layers: Vec::new(),
        view_box: [0.0, 0.0, 16.0, 16.0],
    };
    let mut atlas = DistanceFieldAtlasCore::new(ATLAS_SIZE);
    let err = atlas
        .get_or_generate_icon_msdf(&icon, ICON_PUA_START + 1, TEST_PX)
        .expect_err("empty icon");
    assert!(matches!(err, DistanceFieldError::IconDeferred(_)));
}
