//! Elongated bar placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    arm_bucket, collect_bar_candidates, max_radius_cells, place_from_candidates, shape_param_f64,
};

pub struct BarStrategy;

impl ShapeStrategy for BarStrategy {
    fn name(&self) -> &str {
        "bar"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let max_r = max_radius_cells(ctx.lattice);
        let bar_length = shape_param_f64(&ctx.params.shape, "bar_length", max_r * 0.9);
        let bar_width = shape_param_f64(&ctx.params.shape, "bar_width", 2.0);
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let candidates = collect_bar_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            bar_length,
            bar_width,
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, arm_bucket(ctx))
    }
}
