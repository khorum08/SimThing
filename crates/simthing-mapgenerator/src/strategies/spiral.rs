//! Multi-arm spiral placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    collect_spiral_candidates, fringe_bucket, place_from_candidates, shape_param_f64,
};

pub struct SpiralStrategy {
    pub arms: u32,
}

impl ShapeStrategy for SpiralStrategy {
    fn name(&self) -> &str {
        match self.arms {
            2 => "spiral_2",
            3 => "spiral_3",
            4 => "spiral_4",
            6 => "spiral_6",
            _ => "spiral",
        }
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let arm_tightness = shape_param_f64(&ctx.params.shape, "arm_tightness", 1.0);
        let arm_width = shape_param_f64(&ctx.params.shape, "arm_width", 1.0);
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let candidates = collect_spiral_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            self.arms,
            ctx.params.scale_core.num_stars,
            arm_tightness,
            arm_width,
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, fringe_bucket(ctx))
    }
}
