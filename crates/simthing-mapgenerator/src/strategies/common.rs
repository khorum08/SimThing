//! Shared producer-side placement helpers (PR8 — float sampling quantizes to integer cells).

use std::f64::consts::PI;

use crate::lattice::{CoreMask, LatticeCoord, SquareLattice};
use crate::rng::MapGenRng;
use crate::strategy::{
    PlacedSystemSeed, ShapePlacement, ShapePlacementError, ShapeStrategyContext,
};

pub fn shape_param_f64(params: &crate::params::ShapeParams, key: &str, default: f64) -> f64 {
    params
        .shape_params
        .get(key)
        .copied()
        .filter(|v| v.is_finite())
        .unwrap_or(default)
        .max(0.0)
}

pub fn chebyshev_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
    a.col.abs_diff(b.col).max(a.row.abs_diff(b.row))
}

pub fn squared_distance(a: LatticeCoord, b: LatticeCoord) -> u32 {
    let dc = a.col.abs_diff(b.col);
    let dr = a.row.abs_diff(b.row);
    dc.saturating_mul(dc).saturating_add(dr.saturating_mul(dr))
}

pub fn shuffle_coords(coords: &mut [LatticeCoord], rng: &mut MapGenRng) {
    for i in (1..coords.len()).rev() {
        let j = rng.gen_index((i + 1) as u32) as usize;
        coords.swap(i, j);
    }
}

pub fn quantize_polar(
    lattice: &SquareLattice,
    center: LatticeCoord,
    radius: f64,
    angle: f64,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Option<LatticeCoord> {
    let jx = (rng.next_f64() - 0.5) * jitter;
    let jy = (rng.next_f64() - 0.5) * jitter;
    let col = center.col as f64 + radius * angle.cos() + jx;
    let row = center.row as f64 + radius * angle.sin() + jy;
    let coord = LatticeCoord {
        col: col.round().max(0.0) as u32,
        row: row.round().max(0.0) as u32,
    };
    if lattice.contains(coord) {
        Some(coord)
    } else {
        None
    }
}

pub fn collect_unique_placeable(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    mut coords: Vec<LatticeCoord>,
) -> Vec<LatticeCoord> {
    coords.retain(|c| lattice.contains(*c) && core_mask.is_placeable(*c));
    coords.sort_by_key(|c| (c.col, c.row));
    coords.dedup();
    coords
}

pub fn place_from_candidates(
    ctx: &mut ShapeStrategyContext<'_>,
    mut candidates: Vec<LatticeCoord>,
    bucket: Option<String>,
) -> Result<ShapePlacement, ShapePlacementError> {
    let num_stars = ctx.params.scale_core.num_stars as usize;
    shuffle_coords(&mut candidates, ctx.rng);
    if candidates.len() < num_stars {
        return Err(ShapePlacementError::InsufficientCandidates {
            requested: ctx.params.scale_core.num_stars,
            available: candidates.len(),
        });
    }
    let mut systems = Vec::with_capacity(num_stars);
    for (id, coord) in candidates.into_iter().take(num_stars).enumerate() {
        let placed = ctx
            .occupancy
            .insert_or_relocate(coord, ctx.rng)
            .map_err(ShapePlacementError::Occupancy)?;
        systems.push(PlacedSystemSeed {
            id: id as u32,
            coord: placed,
            bucket: bucket.clone(),
        });
    }
    Ok(ShapePlacement { systems })
}

pub fn max_radius_cells(lattice: &SquareLattice) -> f64 {
    (lattice.edge() / 2).max(1) as f64 * 0.85
}

pub fn collect_spiral_candidates(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    center: LatticeCoord,
    num_arms: u32,
    num_stars: u32,
    arm_tightness: f64,
    arm_width: f64,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Vec<LatticeCoord> {
    let max_r = max_radius_cells(lattice);
    let tightness = arm_tightness.max(0.1).min(4.0);
    let width = arm_width.max(0.5);
    let oversample = (num_stars as usize).saturating_mul(6).max(24);
    let mut raw = Vec::with_capacity(oversample);
    for i in 0..oversample {
        let arm = (i % num_arms as usize) as u32;
        let t = (i / num_arms as usize) as f64 / (oversample / num_arms as usize).max(1) as f64;
        let r = 2.0 + t * max_r;
        let theta = (arm as f64) * 2.0 * PI / num_arms as f64 + t * max_r * 0.12 * tightness;
        let perp = (rng.next_f64() - 0.5) * width;
        let angle = theta + perp * 0.08;
        if let Some(coord) = quantize_polar(lattice, center, r, angle, jitter, rng) {
            raw.push(coord);
        }
    }
    collect_unique_placeable(lattice, core_mask, raw)
}

pub fn collect_annulus_candidates(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    center: LatticeCoord,
    inner_radius: f64,
    outer_radius: f64,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Vec<LatticeCoord> {
    let inner_sq = (inner_radius * inner_radius).floor() as u32;
    let outer_sq = (outer_radius * outer_radius).ceil() as u32;
    let mut raw: Vec<LatticeCoord> = lattice
        .iter_coords()
        .filter(|coord| {
            core_mask.is_placeable(*coord) && {
                let dist_sq = squared_distance(*coord, center);
                dist_sq > inner_sq && dist_sq <= outer_sq
            }
        })
        .collect();
    if jitter > 0.0 {
        shuffle_coords(&mut raw, rng);
        let keep = raw.len().saturating_mul(7) / 10;
        raw.truncate(keep.max(1));
    }
    raw
}

pub fn collect_bar_candidates(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    center: LatticeCoord,
    bar_length: f64,
    bar_width: f64,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Vec<LatticeCoord> {
    let half_len = bar_length.max(2.0) / 2.0;
    let half_w = bar_width.max(1.0) / 2.0;
    let mut raw = Vec::new();
    let steps = (half_len * 2.0).ceil() as i32;
    for i in -steps..=steps {
        for w in -(half_w.ceil() as i32)..=(half_w.ceil() as i32) {
            let col = center.col as i64 + i as i64;
            let row = center.row as i64 + w as i64;
            if col < 0 || row < 0 {
                continue;
            }
            let mut coord = LatticeCoord {
                col: col as u32,
                row: row as u32,
            };
            if jitter > 0.0 {
                let j = (rng.next_f64() - 0.5) * jitter;
                coord.col = (coord.col as f64 + j).round().max(0.0) as u32;
            }
            if lattice.contains(coord) && core_mask.is_placeable(coord) {
                raw.push(coord);
            }
        }
    }
    collect_unique_placeable(lattice, core_mask, raw)
}

pub fn collect_radial_spoke_candidates(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    center: LatticeCoord,
    num_spokes: u32,
    max_radius: f64,
    hub_radius: f64,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Vec<LatticeCoord> {
    let spokes = num_spokes.max(3);
    let mut raw = Vec::new();
    let steps = max_radius.ceil() as u32;
    for spoke in 0..spokes {
        let angle = (spoke as f64) * 2.0 * PI / spokes as f64;
        for step in 0..=steps {
            let r = hub_radius + step as f64;
            if r > max_radius {
                break;
            }
            if let Some(coord) = quantize_polar(lattice, center, r, angle, jitter, rng) {
                if core_mask.is_placeable(coord) {
                    raw.push(coord);
                }
            }
        }
    }
    collect_unique_placeable(lattice, core_mask, raw)
}

pub fn collect_cartwheel_candidates(
    lattice: &SquareLattice,
    core_mask: &CoreMask,
    center: LatticeCoord,
    ring_radius: f64,
    num_spokes: u32,
    jitter: f64,
    rng: &mut MapGenRng,
) -> Vec<LatticeCoord> {
    let max_r = max_radius_cells(lattice);
    let band = ring_radius.max(2.0).min(max_r * 0.4);
    let inner = (ring_radius - band * 0.5).max(1.0);
    let outer = (ring_radius + band * 0.5).min(max_r);
    let mut raw = collect_annulus_candidates(lattice, core_mask, center, inner, outer, jitter, rng);
    raw.extend(collect_radial_spoke_candidates(
        lattice, core_mask, center, num_spokes, max_r, 1.0, jitter, rng,
    ));
    collect_unique_placeable(lattice, core_mask, raw)
}

pub fn fringe_bucket(ctx: &ShapeStrategyContext<'_>) -> Option<String> {
    Some(ctx.params.initializers.initializer_bucket_fringe.clone())
}

pub fn arm_bucket(ctx: &ShapeStrategyContext<'_>) -> Option<String> {
    Some(ctx.params.initializers.initializer_bucket_arm.clone())
}
