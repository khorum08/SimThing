//! Static / arbitrary_static in-memory passthrough (PR3 — no file parsing).

use crate::strategy::{
    PlacedSystemSeed, ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext,
};

pub struct StaticArbitraryStrategy;

impl ShapeStrategy for StaticArbitraryStrategy {
    fn name(&self) -> &str {
        "static"
    }

    fn place(
        &self,
        ctx: &mut ShapeStrategyContext<'_>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let shape = ctx.params.shape.shape.clone();
        let cells =
            ctx.explicit_cells
                .ok_or_else(|| ShapePlacementError::ExplicitCellsRequired {
                    shape: shape.clone(),
                })?;

        let bucket = Some(ctx.params.initializers.initializer_bucket_core.clone());
        let mut systems = Vec::with_capacity(cells.len());

        for (id, &coord) in cells.iter().enumerate() {
            ctx.occupancy
                .try_insert(coord)
                .map_err(ShapePlacementError::Occupancy)?;
            systems.push(PlacedSystemSeed {
                id: id as u32,
                coord,
                bucket: bucket.clone(),
            });
        }

        Ok(ShapePlacement { systems })
    }
}
