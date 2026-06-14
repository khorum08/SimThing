//! Minimal deterministic elliptical disc sampling (PR3 seam — not final galaxy quality).

use crate::lattice::LatticeCoord;
use crate::rng::MapGenRng;
use crate::strategy::{
    PlacedSystemSeed, ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext,
};

pub struct EllipticalStrategy;

impl ShapeStrategy for EllipticalStrategy {
    fn name(&self) -> &str {
        "elliptical"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let num_stars = ctx.params.scale_core.num_stars;
        let jitter = ctx
            .params
            .shape
            .shape_params
            .get("jitter")
            .copied()
            .unwrap_or(0.0)
            .max(0.0);

        let center = ctx.lattice.center();
        let (semi_a, semi_b) = ellipse_semi_axes(ctx.lattice, ctx.params.scale_core.radius, jitter);

        let mut candidates =
            collect_ellipse_candidates(ctx.lattice, ctx.core_mask, center, semi_a, semi_b);
        shuffle_coords(&mut candidates, ctx.rng);

        if candidates.len() < num_stars as usize {
            return Err(ShapePlacementError::InsufficientCandidates {
                requested: num_stars,
                available: candidates.len(),
            });
        }

        let bucket = Some(ctx.params.initializers.initializer_bucket_fringe.clone());
        let mut systems = Vec::with_capacity(num_stars as usize);

        for (id, coord) in candidates.into_iter().take(num_stars as usize).enumerate() {
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
}

fn ellipse_semi_axes(
    lattice: &crate::lattice::SquareLattice,
    radius: f64,
    jitter: f64,
) -> (f64, f64) {
    let half = (lattice.edge() / 2).max(1) as f64;
    let scale = if radius.is_finite() && radius > 0.0 {
        (half * 0.85).min(half)
    } else {
        half * 0.85
    };
    let jitter_scale = 1.0 + jitter.min(0.5) * 0.1;
    (scale * jitter_scale, scale / jitter_scale.max(1.0))
}

fn collect_ellipse_candidates(
    lattice: &crate::lattice::SquareLattice,
    core_mask: &crate::lattice::CoreMask,
    center: LatticeCoord,
    semi_a: f64,
    semi_b: f64,
) -> Vec<LatticeCoord> {
    let a_sq = semi_a * semi_a;
    let b_sq = semi_b * semi_b;
    lattice
        .iter_coords()
        .filter(|coord| {
            core_mask.is_placeable(*coord) && {
                let dc = coord.col as f64 - center.col as f64;
                let dr = coord.row as f64 - center.row as f64;
                (dc * dc) / a_sq + (dr * dr) / b_sq <= 1.0
            }
        })
        .collect()
}

fn shuffle_coords(coords: &mut [LatticeCoord], rng: &mut MapGenRng) {
    for i in (1..coords.len()).rev() {
        let j = rng.gen_index((i + 1) as u32) as usize;
        coords.swap(i, j);
    }
}
