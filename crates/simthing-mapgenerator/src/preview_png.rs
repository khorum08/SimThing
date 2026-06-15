//! Producer-side galaxy preview rasterization (UI handoff — not runtime semantics).
//!
//! Renders in-memory lattice placement, bounded hyperlane feedstock, and nebula centers into a fixed
//! 1000×1000 PNG. Integer lattice coords only; no Euclidean authority in output semantics.

use std::collections::BTreeMap;
use std::path::Path;

use thiserror::Error;

use crate::lattice::{CoreMask, LatticeCoord, SquareLattice};
use crate::nebula::NebulaField;
use crate::strategy::ShapePlacement;
use crate::topology::{system_id_scalar, HyperlaneEdge};

/// Stellaris-style map box size for UI preview (producer-only, not sim authority).
pub const GALAXY_PREVIEW_PNG_SIZE: u32 = 1000;

const PREVIEW_MARGIN_PX: u32 = 24;

#[derive(Debug, Clone)]
pub struct GalaxyPreviewScene {
    pub lattice: SquareLattice,
    pub core_mask: CoreMask,
    pub placement: ShapePlacement,
    pub hyperlane_edges: Vec<HyperlaneEdge>,
    pub nebulas: Vec<NebulaField>,
}

#[derive(Debug, Error)]
pub enum PreviewPngError {
    #[error("preview PNG must be {GALAXY_PREVIEW_PNG_SIZE}x{GALAXY_PREVIEW_PNG_SIZE}")]
    InvalidDimensions,
    #[error("hyperlane endpoint '{0}' not found in placement")]
    UnknownEndpoint(String),
    #[error("png encode failed: {0}")]
    Encode(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Render a 1000×1000 RGB PNG into memory.
pub fn render_galaxy_preview_png(scene: &GalaxyPreviewScene) -> Result<Vec<u8>, PreviewPngError> {
    let mut rgba = vec![0u8; (GALAXY_PREVIEW_PNG_SIZE * GALAXY_PREVIEW_PNG_SIZE * 4) as usize];
    fill_background(&mut rgba);
    paint_core_mask(&mut rgba, &scene.lattice, &scene.core_mask);
    paint_nebulas(&mut rgba, &scene.lattice, &scene.nebulas);
    paint_hyperlanes(
        &mut rgba,
        &scene.lattice,
        &scene.placement,
        &scene.hyperlane_edges,
    )?;
    paint_systems(&mut rgba, &scene.lattice, &scene.placement);
    encode_png_rgb(&rgba)
}

pub fn write_galaxy_preview_png(
    scene: &GalaxyPreviewScene,
    path: impl AsRef<Path>,
) -> Result<(), PreviewPngError> {
    let bytes = render_galaxy_preview_png(scene)?;
    std::fs::write(path, bytes)?;
    Ok(())
}

fn fill_background(rgba: &mut [u8]) {
    for chunk in rgba.chunks_exact_mut(4) {
        chunk[0] = 8;
        chunk[1] = 10;
        chunk[2] = 24;
        chunk[3] = 255;
    }
}

fn paint_core_mask(rgba: &mut [u8], lattice: &SquareLattice, core_mask: &CoreMask) {
    for coord in lattice.iter_coords() {
        if core_mask.is_masked(coord) {
            set_pixel(
                rgba,
                lattice_to_pixel(coord, lattice.edge()),
                [20, 15, 30, 255],
            );
        }
    }
}

fn paint_nebulas(rgba: &mut [u8], lattice: &SquareLattice, nebulas: &[NebulaField]) {
    for nebula in nebulas {
        let center = lattice_to_pixel(nebula.center, lattice.edge());
        let radius_px =
            ((nebula.radius_cells as u32 + 1) * GALAXY_PREVIEW_PNG_SIZE / lattice.edge()).max(2);
        stamp_disc(rgba, center, radius_px, [90, 45, 130, 180]);
    }
}

fn paint_hyperlanes(
    rgba: &mut [u8],
    lattice: &SquareLattice,
    placement: &ShapePlacement,
    edges: &[HyperlaneEdge],
) -> Result<(), PreviewPngError> {
    let coords = system_coord_map(placement);
    for edge in edges {
        let from = coords
            .get(&edge.from)
            .ok_or_else(|| PreviewPngError::UnknownEndpoint(edge.from.clone()))?;
        let to = coords
            .get(&edge.to)
            .ok_or_else(|| PreviewPngError::UnknownEndpoint(edge.to.clone()))?;
        draw_line(
            rgba,
            lattice_to_pixel(*from, lattice.edge()),
            lattice_to_pixel(*to, lattice.edge()),
            [70, 110, 170, 220],
        );
    }
    Ok(())
}

fn paint_systems(rgba: &mut [u8], lattice: &SquareLattice, placement: &ShapePlacement) {
    for system in &placement.systems {
        let center = lattice_to_pixel(system.coord, lattice.edge());
        stamp_disc(rgba, center, 2, [235, 245, 255, 255]);
    }
}

fn system_coord_map(placement: &ShapePlacement) -> BTreeMap<String, LatticeCoord> {
    placement
        .systems
        .iter()
        .map(|system| (system_id_scalar(system), system.coord))
        .collect()
}

fn lattice_to_pixel(coord: LatticeCoord, edge: u32) -> (i32, i32) {
    let drawable = GALAXY_PREVIEW_PNG_SIZE.saturating_sub(2 * PREVIEW_MARGIN_PX);
    let max_index = edge.saturating_sub(1).max(1) as i32;
    let x = PREVIEW_MARGIN_PX as i32
        + (coord.col as i32 * (drawable.saturating_sub(1) as i32)) / max_index;
    let y = PREVIEW_MARGIN_PX as i32
        + (coord.row as i32 * (drawable.saturating_sub(1) as i32)) / max_index;
    (x, y)
}

fn set_pixel(rgba: &mut [u8], (x, y): (i32, i32), color: [u8; 4]) {
    if x < 0 || y < 0 || x >= GALAXY_PREVIEW_PNG_SIZE as i32 || y >= GALAXY_PREVIEW_PNG_SIZE as i32
    {
        return;
    }
    let idx = ((y as u32 * GALAXY_PREVIEW_PNG_SIZE + x as u32) * 4) as usize;
    blend_pixel(&mut rgba[idx..idx + 4], color);
}

fn blend_pixel(dst: &mut [u8], src: [u8; 4]) {
    let alpha = src[3] as f32 / 255.0;
    if alpha <= 0.0 {
        return;
    }
    if alpha >= 1.0 {
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
        dst[3] = 255;
        return;
    }
    for channel in 0..3 {
        dst[channel] = ((1.0 - alpha) * dst[channel] as f32 + alpha * src[channel] as f32) as u8;
    }
    dst[3] = 255;
}

fn stamp_disc(rgba: &mut [u8], center: (i32, i32), radius: u32, color: [u8; 4]) {
    let r = radius as i32;
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                set_pixel(rgba, (center.0 + dx, center.1 + dy), color);
            }
        }
    }
}

fn draw_line(rgba: &mut [u8], start: (i32, i32), end: (i32, i32), color: [u8; 4]) {
    let mut x0 = start.0;
    let mut y0 = start.1;
    let x1 = end.0;
    let y1 = end.1;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        set_pixel(rgba, (x0, y0), color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn encode_png_rgb(rgba: &[u8]) -> Result<Vec<u8>, PreviewPngError> {
    if rgba.len() != (GALAXY_PREVIEW_PNG_SIZE * GALAXY_PREVIEW_PNG_SIZE * 4) as usize {
        return Err(PreviewPngError::InvalidDimensions);
    }
    let mut rgb =
        Vec::with_capacity((GALAXY_PREVIEW_PNG_SIZE * GALAXY_PREVIEW_PNG_SIZE * 3) as usize);
    for chunk in rgba.chunks_exact(4) {
        rgb.extend_from_slice(&chunk[..3]);
    }
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, GALAXY_PREVIEW_PNG_SIZE, GALAXY_PREVIEW_PNG_SIZE);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|err| PreviewPngError::Encode(err.to_string()))?;
    writer
        .write_image_data(&rgb)
        .map_err(|err| PreviewPngError::Encode(err.to_string()))?;
    writer
        .finish()
        .map_err(|err| PreviewPngError::Encode(err.to_string()))?;
    Ok(out)
}
