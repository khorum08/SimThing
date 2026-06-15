//! Annular ring placement (PR8).

use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

use super::common::{
    collect_annulus_candidates, fringe_bucket, max_radius_cells, place_from_candidates,
    shape_param_f64,
};

pub struct RingStrategy;

impl ShapeStrategy for RingStrategy {
    fn name(&self) -> &str {
        "ring"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let center = ctx.lattice.center();
        let max_r = max_radius_cells(ctx.lattice);
        let ring_radius = shape_param_f64(&ctx.params.shape, "ring_radius", max_r * 0.55);
        let band = shape_param_f64(&ctx.params.shape, "arm_width", 2.0).max(shape_param_f64(
            &ctx.params.shape,
            "band_width",
            2.0,
        ));
        let jitter = shape_param_f64(&ctx.params.shape, "jitter", 0.0);
        let inner = (ring_radius - band * 0.5).max(1.0);
        let outer = (ring_radius + band * 0.5).min(max_r);
        let candidates = collect_annulus_candidates(
            ctx.lattice,
            ctx.core_mask,
            center,
            inner,
            outer,
            jitter,
            ctx.rng,
        );
        place_from_candidates(ctx, candidates, fringe_bucket(ctx))
    }
}
