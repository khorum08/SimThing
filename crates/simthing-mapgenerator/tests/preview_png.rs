//! Producer-side 1000×1000 galaxy preview PNG tests.

use simthing_mapgenerator::{
    generate_success_galaxy_with_preview, render_galaxy_preview_png_bytes,
    success_galaxy_1000_params, validate_default, ShapeRegistry, GALAXY_PREVIEW_PNG_SIZE,
};

#[test]
fn success_galaxy_preview_png_is_1000_by_1000() {
    let generation = generate_success_galaxy_with_preview(&ShapeRegistry::default())
        .expect("success galaxy generation");
    assert_eq!(generation.placement.systems.len(), 1000);
    let png = generation.render_preview_png().expect("preview png");
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    let decoder = png::Decoder::new(std::io::Cursor::new(&png));
    let reader = decoder.read_info().expect("png header");
    assert_eq!(reader.info().width, GALAXY_PREVIEW_PNG_SIZE);
    assert_eq!(reader.info().height, GALAXY_PREVIEW_PNG_SIZE);
}

#[test]
fn success_galaxy_preview_png_has_visible_system_pixels() {
    let generation = generate_success_galaxy_with_preview(&ShapeRegistry::default())
        .expect("success galaxy generation");
    let png = render_galaxy_preview_png_bytes(&generation.preview_scene()).expect("preview png");
    let decoder = png::Decoder::new(std::io::Cursor::new(&png));
    let mut reader = decoder.read_info().expect("png header");
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("png frame");
    let bright_pixels = buf[..info.buffer_size()]
        .chunks_exact(4)
        .filter(|rgba| rgba[0] > 200 && rgba[1] > 200 && rgba[3] > 0)
        .count();
    assert!(bright_pixels > 500, "expected many bright system pixels");
}

#[test]
fn success_galaxy_preview_png_is_stable_for_same_seed() {
    let first = generate_success_galaxy_with_preview(&ShapeRegistry::default())
        .expect("first")
        .render_preview_png()
        .expect("png");
    let second = generate_success_galaxy_with_preview(&ShapeRegistry::default())
        .expect("second")
        .render_preview_png()
        .expect("png");
    assert_eq!(first, second);
}

#[test]
fn success_galaxy_params_validate() {
    validate_default(&success_galaxy_1000_params()).expect("valid");
}

#[test]
fn producer_still_has_no_forbidden_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
        "simthing-clausething",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "manifest must not depend on {forbidden}"
        );
    }
}
