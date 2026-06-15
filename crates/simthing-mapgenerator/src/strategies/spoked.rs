//! Hub-and-spoke placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    arm_bucket, collect_radial_spoke_candidates, max_radius_cells, place_from_candidates,
    shape_param_f64,
};

pub struct SpokedStrategy;

impl ShapeStrategy for SpokedStrategy {
    fn name(&self) -> &str {
        "spoked"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let max_r = max_radius_cells(ctx.lattice);
        let num_spokes = shape_param_f64(&ctx.params.shape, "num_arms", 6.0).round() as u32;
        let hub = shape_param_f64(&ctx.params.shape, "core_radius", 1.0).max(0.0);
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let candidates = collect_radial_spoke_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            num_spokes.max(3),
            max_r,
            hub.max(1.0),
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, arm_bucket(ctx))
    }
}
