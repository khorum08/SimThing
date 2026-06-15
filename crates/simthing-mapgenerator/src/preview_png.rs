//! Producer-side PNG preview for generated galaxy scenarios (render-only aesthetics).

use std::path::Path;

use png::{BitDepth, ColorType, Encoder};

use crate::coupling::{ClassifiedCouplingEdge, CouplingEdgeKind};
use crate::lattice::{CoreMask, SquareLattice};
use crate::nebula::NebulaField;
use crate::strategy::ShapePlacement;
use crate::topology::{grid_chebyshev_distance, system_id_scalar, HyperlaneEdge};

const PREVIEW_MARGIN: f32 = 0.04;
const JITTER_FRAC: f32 = 0.42;
const DEFAULT_PREVIEW_MAX_HYPERLANE_CHEBYSHEV: u32 = 4;

/// Default PNG edge length for producer preview artifacts.
pub const GALAXY_PREVIEW_PNG_SIZE: u32 = 1000;

/// Which hyperlane couplings to draw in the preview (default: base topology only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlanePreviewFilter {
    BaseOnly,
    AllCouplings,
}

/// Render-only preview options (does not affect scenario emission or topology).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GalaxyPreviewOptions {
    pub seed: u64,
    pub png_size: u32,
    pub jitter_stars: bool,
    pub draw_nebulas: bool,
    pub draw_core_mask: bool,
    pub hyperlane_filter: HyperlanePreviewFilter,
    /// Render-only: skip hyperlane segments whose placed lattice Chebyshev distance exceeds this.
    pub max_hyperlane_chebyshev: Option<u32>,
    /// Render-only RGBA for hyperlane (starlane) segments.
    pub hyperlane_rgba: [u8; 4],
    /// Render-only: paint a bright galactic-core glow over the (inaccessible) core void.
    pub draw_core_glow: bool,
}

/// Default faint starlane colour (preserves prior preview output).
pub const DEFAULT_HYPERLANE_RGBA: [u8; 4] = [45, 50, 58, 90];

impl Default for GalaxyPreviewOptions {
    fn default() -> Self {
        Self {
            seed: 0,
            png_size: 1000,
            jitter_stars: true,
            draw_nebulas: false,
            draw_core_mask: false,
            hyperlane_filter: HyperlanePreviewFilter::BaseOnly,
            max_hyperlane_chebyshev: Some(DEFAULT_PREVIEW_MAX_HYPERLANE_CHEBYSHEV),
            hyperlane_rgba: DEFAULT_HYPERLANE_RGBA,
            draw_core_glow: false,
        }
    }
}

/// Inputs for preview rendering (placement + topology; no runtime semantics).
#[derive(Debug, Clone)]
pub struct GalaxyPreviewScene {
    pub seed: u64,
    pub options: GalaxyPreviewOptions,
    pub lattice: SquareLattice,
    pub core_mask: CoreMask,
    pub placement: ShapePlacement,
    pub base_hyperlane_edges: Vec<HyperlaneEdge>,
    pub classified_edges: Vec<ClassifiedCouplingEdge>,
    pub nebulas: Vec<NebulaField>,
}

impl GalaxyPreviewScene {
    pub fn hyperlane_edges_for_preview(&self) -> Vec<HyperlaneEdge> {
        match self.options.hyperlane_filter {
            HyperlanePreviewFilter::BaseOnly => self.base_hyperlane_edges.clone(),
            HyperlanePreviewFilter::AllCouplings => self
                .classified_edges
                .iter()
                .map(|entry| entry.edge.clone())
                .collect(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PreviewPngError {
    #[error("preview PNG dimensions must be positive (got {0}×{0})")]
    InvalidDimensions(u32),
    #[error("preview PNG output path has no parent directory")]
    MissingParent,
    #[error("failed to create preview PNG output file: {0}")]
    CreateFile(#[from] std::io::Error),
    #[error("failed to encode preview PNG: {0}")]
    Encode(#[from] png::EncodingError),
}

/// Deterministic render-only hash in `[0, 1)`.
pub fn deterministic_unit_hash(seed: u64, system_id: u32, axis: &str) -> f32 {
    let mut state = seed ^ (system_id as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    for byte in axis.as_bytes() {
        state = state
            .wrapping_mul(0x5851_F42D_4C95_7F2D)
            .wrapping_add(*byte as u64);
    }
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afed_558c_c53);
    state ^= state >> 33;
    state = state.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    state ^= state >> 33;
    (state as f32) / (u64::MAX as f32)
}

/// Map a unit hash to `[-JITTER_FRAC, +JITTER_FRAC]`.
pub fn jitter_fraction_from_hash(unit: f32) -> f32 {
    (unit * 2.0 - 1.0) * JITTER_FRAC
}

pub fn cell_pixel_size(lattice_edge: u32, png_size: u32) -> (f32, f32) {
    let inner = png_size as f32 * (1.0 - 2.0 * PREVIEW_MARGIN);
    let cell = inner / lattice_edge as f32;
    (cell, cell)
}

pub fn cell_center_pixel(
    coord: crate::lattice::LatticeCoord,
    lattice_edge: u32,
    png_size: u32,
) -> (f32, f32) {
    let (cell_w, cell_h) = cell_pixel_size(lattice_edge, png_size);
    let margin = png_size as f32 * PREVIEW_MARGIN;
    let base_x = margin + (coord.col as f32 + 0.5) * cell_w;
    let base_y = margin + (coord.row as f32 + 0.5) * cell_h;
    (base_x, base_y)
}

/// Render-only star position (jitter stays inside the star's gridcell footprint).
pub fn rendered_star_pixel(
    seed: u64,
    system_id: u32,
    coord: crate::lattice::LatticeCoord,
    lattice_edge: u32,
    png_size: u32,
    jitter_stars: bool,
) -> (f32, f32) {
    let (base_x, base_y) = cell_center_pixel(coord, lattice_edge, png_size);
    if !jitter_stars {
        return (base_x, base_y);
    }
    let (cell_w, cell_h) = cell_pixel_size(lattice_edge, png_size);
    let jx = jitter_fraction_from_hash(deterministic_unit_hash(seed, system_id, "x")) * cell_w;
    let jy = jitter_fraction_from_hash(deterministic_unit_hash(seed, system_id, "y")) * cell_h;
    (base_x + jx, base_y + jy)
}

pub fn star_render_radius(seed: u64, system_id: u32) -> f32 {
    let unit = deterministic_unit_hash(seed, system_id, "radius");
    1.0 + unit * 1.2
}

pub fn render_galaxy_preview_png_bytes(
    scene: &GalaxyPreviewScene,
) -> Result<Vec<u8>, PreviewPngError> {
    let size = scene.options.png_size;
    if size == 0 {
        return Err(PreviewPngError::InvalidDimensions(size));
    }
    let rgba = encode_preview_rgba(scene)?;
    let mut bytes = Vec::new();
    {
        let writer = std::io::Cursor::new(&mut bytes);
        let mut encoder = Encoder::new(writer, size, size);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut png_writer = encoder.write_header()?;
        png_writer.write_image_data(&rgba)?;
    }
    Ok(bytes)
}

pub fn render_galaxy_preview_png(
    scene: &GalaxyPreviewScene,
    output: impl AsRef<Path>,
) -> Result<(), PreviewPngError> {
    let bytes = render_galaxy_preview_png_bytes(scene)?;
    let parent = output
        .as_ref()
        .parent()
        .ok_or(PreviewPngError::MissingParent)?;
    std::fs::create_dir_all(parent)?;
    std::fs::write(output, bytes)?;
    Ok(())
}

fn encode_preview_rgba(scene: &GalaxyPreviewScene) -> Result<Vec<u8>, PreviewPngError> {
    let size = scene.options.png_size;
    if size == 0 {
        return Err(PreviewPngError::InvalidDimensions(size));
    }

    let mut rgba = vec![0u8; (size as usize) * (size as usize) * 4];
    paint_black_background(&mut rgba, size);

    if scene.options.draw_core_glow {
        paint_core_glow(&mut rgba, size, &scene.lattice, &scene.core_mask);
    }

    if scene.options.draw_core_mask {
        paint_core_mask(&mut rgba, size, &scene.lattice, &scene.core_mask);
    }

    if scene.options.draw_nebulas {
        paint_nebulas(&mut rgba, size, &scene.lattice, &scene.nebulas);
    }

    let edge = scene.lattice.edge() as u32;
    let systems_by_id: std::collections::HashMap<String, &crate::strategy::PlacedSystemSeed> =
        scene
            .placement
            .systems
            .iter()
            .map(|system| (system_id_scalar(system), system))
            .collect();
    let hyperlanes = scene.hyperlane_edges_for_preview();
    let max_lane_dist = scene
        .options
        .max_hyperlane_chebyshev
        .unwrap_or(DEFAULT_PREVIEW_MAX_HYPERLANE_CHEBYSHEV);
    for edge_pair in &hyperlanes {
        if let (Some(from), Some(to)) = (
            systems_by_id.get(&edge_pair.from),
            systems_by_id.get(&edge_pair.to),
        ) {
            if grid_chebyshev_distance(
                (from.coord.col, from.coord.row),
                (to.coord.col, to.coord.row),
            ) > max_lane_dist
            {
                continue;
            }
            let (x0, y0) = rendered_star_pixel(
                scene.seed,
                from.id,
                from.coord,
                edge,
                size,
                scene.options.jitter_stars,
            );
            let (x1, y1) = rendered_star_pixel(
                scene.seed,
                to.id,
                to.coord,
                edge,
                size,
                scene.options.jitter_stars,
            );
            draw_line(
                &mut rgba,
                size,
                x0,
                y0,
                x1,
                y1,
                scene.options.hyperlane_rgba,
            );
        }
    }

    for system in &scene.placement.systems {
        let (x, y) = rendered_star_pixel(
            scene.seed,
            system.id,
            system.coord,
            edge,
            size,
            scene.options.jitter_stars,
        );
        // Scale star size with the canvas (1.0× at the canonical 1000px) so large renders stay legible.
        let scale = size as f32 / GALAXY_PREVIEW_PNG_SIZE as f32;
        let radius = star_render_radius(scene.seed, system.id) * scale;
        paint_star(&mut rgba, size, x, y, radius);
    }

    Ok(rgba)
}

#[allow(dead_code)]
pub fn write_galaxy_preview_png(
    scene: &GalaxyPreviewScene,
    output: impl AsRef<Path>,
) -> Result<(), PreviewPngError> {
    render_galaxy_preview_png(scene, output)
}

fn paint_black_background(rgba: &mut [u8], size: u32) {
    for px in rgba.chunks_exact_mut(4) {
        px[0] = 0;
        px[1] = 0;
        px[2] = 0;
        px[3] = 255;
    }
    let _ = size;
}

/// Paint a bright galactic-core glow over the inaccessible core void (warm-white centre, soft halo).
fn paint_core_glow(rgba: &mut [u8], size: u32, lattice: &SquareLattice, core_mask: &CoreMask) {
    let edge = lattice.edge() as u32;
    let core_cells = core_mask.radius_cells().max(1);
    let (cell_w, _) = cell_pixel_size(edge, size);
    let center = cell_center_pixel(core_mask.center(), edge, size);
    let core_px = core_cells as f32 * cell_w;
    let glow_px = (core_px * 1.6).max(1.0);
    let glow_sq = glow_px * glow_px;
    let x0 = (center.0 - glow_px).floor().max(0.0) as u32;
    let x1 = (center.0 + glow_px).ceil().min(size as f32) as u32;
    let y0 = (center.1 - glow_px).floor().max(0.0) as u32;
    let y1 = (center.1 + glow_px).ceil().min(size as f32) as u32;
    for py in y0..y1 {
        for px in x0..x1 {
            let dx = px as f32 + 0.5 - center.0;
            let dy = py as f32 + 0.5 - center.1;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq > glow_sq {
                continue;
            }
            let ratio = dist_sq / glow_sq; // 0 at centre .. 1 at glow edge
            let intensity = (1.0 - ratio).powf(1.8); // bright concentrated core
            let r = (255.0 * intensity).min(255.0) as u8;
            let g = (235.0 * intensity).min(255.0) as u8;
            let b = (205.0 * intensity).max(45.0 * (1.0 - ratio)).min(255.0) as u8;
            let a = (intensity * 255.0) as u8;
            blend_pixel(rgba, size, px, py, [r, g, b, a]);
        }
    }
}

fn paint_core_mask(rgba: &mut [u8], size: u32, lattice: &SquareLattice, core_mask: &CoreMask) {
    let edge = lattice.edge() as u32;
    for row in 0..edge {
        for col in 0..edge {
            let coord = crate::lattice::LatticeCoord { col, row };
            if !core_mask.is_masked(coord) {
                continue;
            }
            let (cx, cy) = cell_center_pixel(coord, edge, size);
            fill_rect(
                rgba,
                size,
                cx - 1.0,
                cy - 1.0,
                cx + 1.0,
                cy + 1.0,
                [20, 24, 36, 40],
            );
        }
    }
}

fn paint_nebulas(rgba: &mut [u8], size: u32, lattice: &SquareLattice, nebulas: &[NebulaField]) {
    let edge = lattice.edge() as u32;
    for nebula in nebulas {
        let (cx, cy) = cell_center_pixel(nebula.center, edge, size);
        let radius = (nebula.radius_cells as f32 + 0.5) * cell_pixel_size(edge, size).0;
        fill_circle(rgba, size, cx, cy, radius, [80, 40, 100, 30]);
    }
}

fn paint_star(rgba: &mut [u8], size: u32, x: f32, y: f32, radius: f32) {
    fill_circle(rgba, size, x, y, radius + 1.0, [220, 225, 255, 35]);
    fill_circle(rgba, size, x, y, radius, [245, 250, 255, 255]);
}

fn fill_circle(rgba: &mut [u8], size: u32, cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    let r2 = radius * radius;
    let min_x = (cx - radius).floor().max(0.0) as i32;
    let max_x = (cx + radius).ceil().min(size as f32 - 1.0) as i32;
    let min_y = (cy - radius).floor().max(0.0) as i32;
    let max_y = (cy + radius).ceil().min(size as f32 - 1.0) as i32;
    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let dx = px as f32 + 0.5 - cx;
            let dy = py as f32 + 0.5 - cy;
            if dx * dx + dy * dy <= r2 {
                blend_pixel(rgba, size, px as u32, py as u32, color);
            }
        }
    }
}

fn fill_rect(rgba: &mut [u8], size: u32, x0: f32, y0: f32, x1: f32, y1: f32, color: [u8; 4]) {
    let min_x = x0.floor().max(0.0) as i32;
    let max_x = x1.ceil().min(size as f32 - 1.0) as i32;
    let min_y = y0.floor().max(0.0) as i32;
    let max_y = y1.ceil().min(size as f32 - 1.0) as i32;
    for py in min_y..=max_y {
        for px in min_x..=max_x {
            blend_pixel(rgba, size, px as u32, py as u32, color);
        }
    }
}

fn draw_line(rgba: &mut [u8], size: u32, x0: f32, y0: f32, x1: f32, y1: f32, color: [u8; 4]) {
    // Stroke half-width scales with image size (0 at the canonical 1000px → 1-px line; thicker on larger
    // canvases for legibility) so the 1000px preview is byte-identical to before.
    let half_width = (size / 2000) as i32;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let steps = dx.abs().max(dy.abs()).ceil().max(1.0) as i32;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let cx = (x0 + dx * t).round() as i32;
        let cy = (y0 + dy * t).round() as i32;
        for oy in -half_width..=half_width {
            for ox in -half_width..=half_width {
                let px = cx + ox;
                let py = cy + oy;
                if px >= 0 && py >= 0 && (px as u32) < size && (py as u32) < size {
                    blend_pixel(rgba, size, px as u32, py as u32, color);
                }
            }
        }
    }
}

fn blend_pixel(rgba: &mut [u8], size: u32, x: u32, y: u32, color: [u8; 4]) {
    let idx = ((y * size + x) * 4) as usize;
    let alpha = color[3] as f32 / 255.0;
    if alpha >= 1.0 {
        rgba[idx..idx + 4].copy_from_slice(&color);
        return;
    }
    for c in 0..3 {
        let dst = rgba[idx + c] as f32 / 255.0;
        let src = color[c] as f32 / 255.0;
        rgba[idx + c] = ((src * alpha + dst * (1.0 - alpha)) * 255.0).round() as u8;
    }
    rgba[idx + 3] = 255;
}

pub fn collect_rendered_star_pixels(scene: &GalaxyPreviewScene) -> Vec<(f32, f32)> {
    let edge = scene.lattice.edge() as u32;
    scene
        .placement
        .systems
        .iter()
        .map(|system| {
            rendered_star_pixel(
                scene.seed,
                system.id,
                system.coord,
                edge,
                scene.options.png_size,
                scene.options.jitter_stars,
            )
        })
        .collect()
}

pub fn collect_cell_center_pixels(scene: &GalaxyPreviewScene) -> Vec<(f32, f32)> {
    let edge = scene.lattice.edge() as u32;
    scene
        .placement
        .systems
        .iter()
        .map(|system| cell_center_pixel(system.coord, edge, scene.options.png_size))
        .collect()
}

pub fn count_bridge_edges(scene: &GalaxyPreviewScene) -> usize {
    scene
        .classified_edges
        .iter()
        .filter(|entry| {
            matches!(
                entry.kind,
                CouplingEdgeKind::SpecialRouteCoupling
                    | CouplingEdgeKind::PartitionBridgeCoupling
                    | CouplingEdgeKind::ClusterBridgeCoupling
            )
        })
        .count()
}
