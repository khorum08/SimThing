//! Visual preview remediation tests (jitter, base hyperlanes, spiral 1500 preset).

use simthing_mapgenerator::{
    cell_center_pixel, collect_cell_center_pixels, collect_rendered_star_pixels,
    count_bridge_edges, deterministic_unit_hash, generate_visual_spiral_1500,
    jitter_fraction_from_hash, render_galaxy_preview_png_bytes, rendered_star_pixel,
    validate_hyperlane_edges, visual_spiral_1500_params, CouplingEdgeKind,
    HyperlanePreviewFilter, ShapeRegistry, GALAXY_PREVIEW_PNG_SIZE, VISUAL_SPIRAL_1500_LATTICE_EDGE,
    VISUAL_SPIRAL_1500_SEED, VISUAL_SPIRAL_1500_STARS,
};

fn spiral_scene() -> simthing_mapgenerator::GalaxyPreviewScene {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    generation.preview_scene()
}

#[test]
fn star_jitter_is_deterministic() {
    let scene = spiral_scene();
    let first = collect_rendered_star_pixels(&scene);
    let second = collect_rendered_star_pixels(&scene);
    assert_eq!(first, second);
}

fn hyperlane_pairs(edges: &[simthing_mapgenerator::HyperlaneEdge]) -> Vec<(String, String)> {
    edges
        .iter()
        .map(|edge| (edge.from.clone(), edge.to.clone()))
        .collect()
}
