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

#[test]
fn different_seed_changes_jitter_when_possible() {
    let mut scene_a = spiral_scene();
    scene_a.seed = 1;
    let mut scene_b = spiral_scene();
    scene_b.seed = 2;
    let positions_a = collect_rendered_star_pixels(&scene_a);
    let positions_b = collect_rendered_star_pixels(&scene_b);
    assert_ne!(positions_a, positions_b);
}

#[test]
fn star_jitter_stays_within_gridcell() {
    let scene = spiral_scene();
    let edge = scene.lattice.edge() as u32;
    let (cell_w, cell_h) =
        simthing_mapgenerator::preview_png::cell_pixel_size(edge, scene.options.png_size);
    let half_w = cell_w * 0.42;
    let half_h = cell_h * 0.42;
    for system in &scene.placement.systems {
        let center = cell_center_pixel(system.coord, edge, scene.options.png_size);
        let rendered = rendered_star_pixel(
            scene.seed,
            system.id,
            system.coord,
            edge,
            scene.options.png_size,
            true,
        );
        assert!(
            (rendered.0 - center.0).abs() <= half_w + 0.01,
            "x jitter out of bounds for system {}",
            system.id
        );
        assert!(
            (rendered.1 - center.1).abs() <= half_h + 0.01,
            "y jitter out of bounds for system {}",
            system.id
        );
    }
}

#[test]
fn jitter_does_not_change_generated_lattice_coord() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    let before: Vec<_> = generation
        .placement
        .systems
        .iter()
        .map(|system| (system.id, system.coord))
        .collect();
    let _png = generation.render_preview_png().expect("png");
    let after: Vec<_> = generation
        .placement
        .systems
        .iter()
        .map(|system| (system.id, system.coord))
        .collect();
    assert_eq!(before, after);
}

#[test]
fn rendered_star_positions_are_not_all_cell_centers() {
    let scene = spiral_scene();
    let rendered = collect_rendered_star_pixels(&scene);
    let centers = collect_cell_center_pixels(&scene);
    assert_eq!(rendered.len(), centers.len());
    assert!(
        rendered
            .iter()
            .zip(centers.iter())
            .any(|((rx, ry), (cx, cy))| (rx - cx).abs() > 0.05 || (ry - cy).abs() > 0.05),
        "expected at least one jittered star off cell center"
    );
}

#[test]
fn hyperlane_edges_are_independent_segments() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    let scene = generation.preview_scene();
    let preview_edges = scene.hyperlane_edges_for_preview();
    assert!(!preview_edges.is_empty());
    for edge in &preview_edges {
        assert_ne!(edge.from, edge.to, "self-link in preview edges");
    }
}

#[test]
fn renderer_uses_base_hyperlane_edges_not_system_order() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    let scene = generation.preview_scene();
    let preview_count = scene.hyperlane_edges_for_preview().len();
    assert_eq!(preview_count, generation.base_hyperlane_edges.len());
    assert!(
        preview_count < generation.hyperlane_edges.len()
            || generation
                .classified_edges
                .iter()
                .all(|entry| { entry.kind == CouplingEdgeKind::BaseHyperlane }),
        "preview should prefer base topology edges over full coupling merge"
    );
}

#[test]
fn renderer_excludes_special_and_bridge_edges_by_default() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    let scene = generation.preview_scene();
    let bridge_like = count_bridge_edges(&scene);
    if bridge_like > 0 {
        assert!(
            scene.hyperlane_edges_for_preview().len() < generation.hyperlane_edges.len(),
            "base-only preview must omit bridge/special couplings when present"
        );
    }
    assert_eq!(
        scene.options.hyperlane_filter,
        HyperlanePreviewFilter::BaseOnly
    );
}

#[test]
fn renderer_outputs_1000x1000_png() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    let png = generation.render_preview_png().expect("png");
    let decoder = png::Decoder::new(std::io::Cursor::new(&png));
    let reader = decoder.read_info().expect("png header");
    assert_eq!(reader.info().width, GALAXY_PREVIEW_PNG_SIZE);
    assert_eq!(reader.info().height, GALAXY_PREVIEW_PNG_SIZE);
}

#[test]
fn renderer_draws_no_grid_lines() {
    let scene = spiral_scene();
    assert!(!scene.options.draw_core_mask);
    let png = render_galaxy_preview_png_bytes(&scene).expect("png");
    let decoder = png::Decoder::new(std::io::Cursor::new(&png));
    let mut reader = decoder.read_info().expect("png header");
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("png frame");
    let core_mask_pixels = buf[..info.buffer_size()]
        .chunks_exact(4)
        .filter(|rgba| rgba[0] == 20 && rgba[1] == 24 && rgba[2] == 36)
        .count();
    assert_eq!(
        core_mask_pixels, 0,
        "core-mask grid debug pixels must be absent"
    );
}

#[test]
fn custom_spiral_1500_generates_1500_systems() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    assert_eq!(generation.placement.systems.len(), 1500);
}

#[test]
fn custom_spiral_1500_uses_300_lattice_edge() {
    let params = visual_spiral_1500_params();
    assert_eq!(params.scale_core.lattice_size, Some(300));
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    assert_eq!(generation.lattice.edge(), 300);
}

#[test]
fn custom_spiral_1500_generates_base_hyperlanes() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    assert!(!generation.base_hyperlane_edges.is_empty());
}

#[test]
fn custom_spiral_1500_has_no_self_links() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    for edge in &generation.base_hyperlane_edges {
        assert_ne!(edge.from, edge.to);
    }
}

fn hyperlane_pairs(edges: &[simthing_mapgenerator::HyperlaneEdge]) -> Vec<(String, String)> {
    edges
        .iter()
        .map(|edge| (edge.from.clone(), edge.to.clone()))
        .collect()
}

#[test]
fn custom_spiral_1500_has_no_unknown_endpoint_links() {
    let generation = generate_visual_spiral_1500(&ShapeRegistry::default()).expect("generation");
    validate_hyperlane_edges(
        &generation.placement,
        &hyperlane_pairs(&generation.base_hyperlane_edges),
    )
    .expect("known endpoints");
}

#[test]
fn visual_spiral_preset_constants_match_spec() {
    let params = visual_spiral_1500_params();
    assert_eq!(params.shape.shape, "spiral_4");
    assert_eq!(params.scale_core.num_stars, VISUAL_SPIRAL_1500_STARS);
    assert_eq!(params.seed, VISUAL_SPIRAL_1500_SEED);
    assert_eq!(
        params.scale_core.lattice_size,
        Some(VISUAL_SPIRAL_1500_LATTICE_EDGE)
    );
}

#[test]
fn same_seed_same_star_jitter() {
    let scene = spiral_scene();
    let unit_x = deterministic_unit_hash(scene.seed, 7, "x");
    let unit_y = deterministic_unit_hash(scene.seed, 7, "y");
    assert_eq!(unit_x, deterministic_unit_hash(scene.seed, 7, "x"));
    assert!(jitter_fraction_from_hash(unit_x).abs() <= 0.42 + f32::EPSILON);
    assert!(jitter_fraction_from_hash(unit_y).abs() <= 0.42 + f32::EPSILON);
}

#[test]
fn jitter_stays_inside_cell() {
    star_jitter_stays_within_gridcell();
}

#[test]
fn renderer_applies_placed_coordinate_chebyshev_cap_by_default() {
    let scene = spiral_scene();
    assert_eq!(scene.options.max_hyperlane_chebyshev, Some(4));
}
