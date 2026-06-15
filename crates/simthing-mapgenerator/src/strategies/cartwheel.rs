//! Cartwheel (ring + spokes) placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    collect_cartwheel_candidates, fringe_bucket, max_radius_cells, place_from_candidates,
    shape_param_f64,
};

pub struct CartwheelStrategy;

impl ShapeStrategy for CartwheelStrategy {
    fn name(&self) -> &str {
        "cartwheel"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let max_r = max_radius_cells(ctx.lattice);
        let ring_radius = shape_param_f64(&ctx.params.shape, "ring_radius", max_r * 0.5);
        let num_spokes = shape_param_f64(&ctx.params.shape, "num_arms", 6.0).round() as u32;
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let candidates = collect_cartwheel_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            ring_radius,
            num_spokes.max(3),
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, fringe_bucket(ctx))
    }
}
