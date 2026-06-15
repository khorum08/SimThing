//! Radial starburst placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    collect_radial_spoke_candidates, fringe_bucket, max_radius_cells, place_from_candidates,
    shape_param_f64,
};

pub struct StarburstStrategy;

impl ShapeStrategy for StarburstStrategy {
    fn name(&self) -> &str {
        "starburst"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let max_r = max_radius_cells(ctx.lattice);
        let num_spokes = shape_param_f64(&ctx.params.shape, "num_arms", 8.0).round() as u32;
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let candidates = collect_radial_spoke_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            num_spokes.max(3),
            max_r,
            1.0,
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, fringe_bucket(ctx))
    }
}
