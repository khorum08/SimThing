//! ACTIONBAND-SPATIAL-VENDORIZATION-0 — born-mortal pure consumer of ActionBand
//! spatial progress.
//!
//! This leaf module re-proves §13.1 fences as a disposable workshop witness. It
//! does **not** define a production Destination, path, predecessor, planner,
//! crossing detector, or peer movement executor. Deleting it cannot remove the
//! graduated ActionBand facility in production crates.
//!
//! The sealed locus arrives already authorized (ActionBand + Phase-5
//! `BandCrossingDelta` / `StructuralCommitment`). This consumer only:
//! - reattaches the sealed `(slot, col)` to exactly one admitted logical cell,
//! - fails closed on zero or multiple loci (physical-row order never chooses),
//! - requires exactly one N4 adjacent structural edge,
//! - quantizes consumption through ordinary CostBand algebra,
//! - mints an ordinary overlay with a real origin and lawful lifecycle.
//!
//! Human-readable “movement” language is proof prose only.

use simthing_core::{
    admit_dispatch_minted_overlay, cost_band_quantize, CostBandDraw, DissolveCondition, Overlay,
    OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimPropertyId,
    SimThingId, SubFieldRole, TransformOp,
};
use thiserror::Error;

/// One admitted synthetic topology cell. Coordinates are N4 topology only;
/// magnitudes never live here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedTopologyCell {
    pub slot: u32,
    pub value_col: u32,
    pub grid_row: u32,
    pub grid_col: u32,
    pub cell: SimThingId,
}

/// Ordinary overlay transform carried while the actor crosses the edge.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialStepOverlayEffect {
    pub property_id: SimPropertyId,
    pub deltas: Vec<(SubFieldRole, TransformOp)>,
}

/// Boundary-ready spatial step admitted from a sealed ActionBand locus.
///
/// Destination is not an argument: it is reattached from the unique sealed
/// `(slot, value_col)` binding. Callers cannot replace the field-derived cell
/// after admission.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialVendorizationStep {
    actor: SimThingId,
    from_cell: SimThingId,
    to_cell: SimThingId,
    field_width: u32,
    value_col: u32,
    from_row: u32,
    from_col: u32,
    to_row: u32,
    to_col: u32,
    sealed_slot: u32,
    sealed_col: u32,
    sealed_value: f32,
    event_kind: u32,
    unit_cost: f32,
    is_sink: bool,
    throttle: Option<u32>,
    draw: CostBandDraw,
    overlay: Overlay,
}

impl SpatialVendorizationStep {
    /// Admit exactly one field-derived N4 edge from a sealed ActionBand locus.
    #[allow(clippy::too_many_arguments)]
    pub fn admit(
        sealed_slot: u32,
        sealed_col: u32,
        sealed_value: f32,
        event_kind: u32,
        actor: SimThingId,
        actor_parent: SimThingId,
        field_width: u32,
        cells: &[AdmittedTopologyCell],
        effect: SpatialStepOverlayEffect,
        is_sink: bool,
        unit_cost: f32,
        throttle: Option<u32>,
    ) -> Result<Self, SpatialVendorizationError> {
        validate_topology(field_width, cells)?;

        let to = resolve_authoritative_cell(cells, sealed_slot, sealed_col)?;
        let from = resolve_authoritative_parent(cells, actor_parent)?;
        if from.cell == to.cell || manhattan(from, to) != 1 {
            return Err(SpatialVendorizationError::NotOneN4Edge {
                from: from.cell,
                to: to.cell,
            });
        }

        let draw = cost_band_quantize(sealed_value, unit_cost, is_sink, throttle)
            .map_err(|error| SpatialVendorizationError::CostBand(error.to_string()))?;
        if is_sink && draw.n != 1 {
            return Err(SpatialVendorizationError::CostBandDidNotAuthorize {
                completed: draw.n,
            });
        }
        if !is_sink && (draw.n != 0 || draw.r.to_bits() != draw.v.to_bits()) {
            return Err(SpatialVendorizationError::FreeRepositionConsumed);
        }

        // Lawful lifecycle: automatic dissolve after one tick. Arrival residency
        // is proven by the structural Reparent (ChildOf(destination)), not by a
        // peer movement dissolution predicate.
        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Instruction,
            source: OverlaySource::System,
            origin: to.cell,
            affects: vec![actor],
            transform: PropertyTransformDelta {
                property_id: effect.property_id,
                sub_field_deltas: effect.deltas,
            },
            lifecycle: OverlayLifecycle::UntilDissolvedWith {
                dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
            },
        };
        admit_dispatch_minted_overlay(&overlay)
            .map_err(|error| SpatialVendorizationError::Overlay(error.to_string()))?;

        let step = Self {
            actor,
            from_cell: from.cell,
            to_cell: to.cell,
            field_width,
            value_col: to.value_col,
            from_row: from.grid_row,
            from_col: from.grid_col,
            to_row: to.grid_row,
            to_col: to.grid_col,
            sealed_slot,
            sealed_col,
            sealed_value,
            event_kind,
            unit_cost,
            is_sink,
            throttle,
            draw,
            overlay,
        };
        step.validate_integrity()?;
        Ok(step)
    }

    pub fn actor(&self) -> SimThingId {
        self.actor
    }

    pub fn from_cell(&self) -> SimThingId {
        self.from_cell
    }

    pub fn deciding_cell(&self) -> SimThingId {
        self.to_cell
    }

    pub fn sealed_slot(&self) -> u32 {
        self.sealed_slot
    }

    pub fn sealed_col(&self) -> u32 {
        self.sealed_col
    }

    pub fn sealed_value(&self) -> f32 {
        self.sealed_value
    }

    pub fn event_kind(&self) -> u32 {
        self.event_kind
    }

    pub fn cost_band_draw(&self) -> CostBandDraw {
        self.draw
    }

    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn overlay_id(&self) -> OverlayId {
        self.overlay.id
    }

    pub fn is_sink(&self) -> bool {
        self.is_sink
    }

    /// Recompute the admitted CostBand draw; any stored direct decrement fails.
    pub fn validate_cost_band(&self) -> Result<(), SpatialVendorizationError> {
        let expected = cost_band_quantize(
            self.sealed_value,
            self.unit_cost,
            self.is_sink,
            self.throttle,
        )
        .map_err(|error| SpatialVendorizationError::CostBand(error.to_string()))?;
        if expected != self.draw {
            return Err(SpatialVendorizationError::CostBandBypass);
        }
        if self.is_sink && self.draw.n != 1 {
            return Err(SpatialVendorizationError::CostBandDidNotAuthorize {
                completed: self.draw.n,
            });
        }
        if !self.is_sink && (self.draw.n != 0 || self.draw.r.to_bits() != self.draw.v.to_bits()) {
            return Err(SpatialVendorizationError::FreeRepositionConsumed);
        }
        Ok(())
    }

    /// Reject missing/synthesized origins and non-lawful lifecycles.
    pub fn validate_overlay(&self) -> Result<(), SpatialVendorizationError> {
        validate_spatial_overlay(self.actor, self.to_cell, &self.overlay)
    }

    fn validate_integrity(&self) -> Result<(), SpatialVendorizationError> {
        let selected_slot = self
            .to_row
            .checked_mul(self.field_width)
            .and_then(|v| v.checked_add(self.to_col))
            .ok_or(SpatialVendorizationError::InvalidFieldTopology)?;
        if self.to_col >= self.field_width
            || self.from_col >= self.field_width
            || selected_slot != self.sealed_slot
            || self.value_col != self.sealed_col
        {
            return Err(SpatialVendorizationError::DecisionLocusDrift);
        }
        let distance = self.from_row.abs_diff(self.to_row) + self.from_col.abs_diff(self.to_col);
        if self.from_cell == self.to_cell || distance != 1 {
            return Err(SpatialVendorizationError::NotOneN4Edge {
                from: self.from_cell,
                to: self.to_cell,
            });
        }
        self.validate_overlay()?;
        self.validate_cost_band()?;
        Ok(())
    }
}

/// Production-shaped overlay validator used at admission and before apply.
pub fn validate_spatial_overlay(
    actor: SimThingId,
    deciding_cell: SimThingId,
    overlay: &Overlay,
) -> Result<(), SpatialVendorizationError> {
    if overlay.origin != deciding_cell || overlay.affects != vec![actor] {
        return Err(SpatialVendorizationError::OverlayOriginDrift);
    }
    match &overlay.lifecycle {
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        } if dissolution_conditions == &vec![DissolveCondition::AfterTicks { remaining: 1 }] => {}
        _ => return Err(SpatialVendorizationError::LawfulLifecycleRequired),
    }
    admit_dispatch_minted_overlay(overlay)
        .map_err(|error| SpatialVendorizationError::Overlay(error.to_string()))?;
    Ok(())
}

/// Resolve a sealed locus to exactly one admitted cell. Physical-row or
/// iteration order cannot choose among multiples — multiplicity fails closed.
pub fn resolve_authoritative_cell(
    cells: &[AdmittedTopologyCell],
    slot: u32,
    col: u32,
) -> Result<AdmittedTopologyCell, SpatialVendorizationError> {
    let mut selected = cells
        .iter()
        .filter(|cell| cell.slot == slot && cell.value_col == col);
    let first = *selected
        .next()
        .ok_or(SpatialVendorizationError::UnboundDecisionLocus { slot, col })?;
    if selected.next().is_some() {
        return Err(SpatialVendorizationError::AmbiguousDecisionLocus { slot, col });
    }
    Ok(first)
}

fn resolve_authoritative_parent(
    cells: &[AdmittedTopologyCell],
    actor_parent: SimThingId,
) -> Result<AdmittedTopologyCell, SpatialVendorizationError> {
    let mut origins = cells.iter().filter(|cell| cell.cell == actor_parent);
    let first = *origins
        .next()
        .ok_or(SpatialVendorizationError::ActorParentOutsideField { actor_parent })?;
    if origins.next().is_some() {
        return Err(SpatialVendorizationError::AmbiguousActorParent { actor_parent });
    }
    Ok(first)
}

fn validate_topology(
    field_width: u32,
    cells: &[AdmittedTopologyCell],
) -> Result<(), SpatialVendorizationError> {
    if field_width == 0 {
        return Err(SpatialVendorizationError::InvalidFieldTopology);
    }
    for cell in cells {
        let expected = cell
            .grid_row
            .checked_mul(field_width)
            .and_then(|v| v.checked_add(cell.grid_col))
            .ok_or(SpatialVendorizationError::InvalidFieldTopology)?;
        if cell.grid_col >= field_width || cell.slot != expected {
            return Err(SpatialVendorizationError::InvalidFieldTopology);
        }
    }
    Ok(())
}

pub fn manhattan(left: AdmittedTopologyCell, right: AdmittedTopologyCell) -> u32 {
    left.grid_row.abs_diff(right.grid_row) + left.grid_col.abs_diff(right.grid_col)
}

/// Multi-hop / non-adjacent structural consequences are red by construction.
pub fn reject_non_adjacent(
    from: AdmittedTopologyCell,
    to: AdmittedTopologyCell,
) -> Result<(), SpatialVendorizationError> {
    if from.cell == to.cell || manhattan(from, to) != 1 {
        return Err(SpatialVendorizationError::NotOneN4Edge {
            from: from.cell,
            to: to.cell,
        });
    }
    Ok(())
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SpatialVendorizationError {
    #[error("spatial field topology is invalid")]
    InvalidFieldTopology,
    #[error("sealed decision locus ({slot},{col}) has no admitted field-cell identity")]
    UnboundDecisionLocus { slot: u32, col: u32 },
    #[error("sealed decision locus ({slot},{col}) maps to more than one field cell")]
    AmbiguousDecisionLocus { slot: u32, col: u32 },
    #[error("actor parent {actor_parent:?} is not an admitted field cell")]
    ActorParentOutsideField { actor_parent: SimThingId },
    #[error("actor parent {actor_parent:?} maps to more than one field cell")]
    AmbiguousActorParent { actor_parent: SimThingId },
    #[error("spatial step from {from:?} to {to:?} is not exactly one N4 edge")]
    NotOneN4Edge { from: SimThingId, to: SimThingId },
    #[error("spatial decision locus drifted after sealed admission")]
    DecisionLocusDrift,
    #[error("spatial overlay origin is not the sealed deciding cell")]
    OverlayOriginDrift,
    #[error("spatial overlay requires UntilDissolvedWith AfterTicks{{remaining:1}}")]
    LawfulLifecycleRequired,
    #[error("spatial CostBand admission failed: {0}")]
    CostBand(String),
    #[error("spatial sink CostBand completed {completed} units instead of one")]
    CostBandDidNotAuthorize { completed: u32 },
    #[error("free repositioning consumed a CostBand unit")]
    FreeRepositionConsumed,
    #[error("spatial step bypassed or altered its admitted CostBand draw")]
    CostBandBypass,
    #[error("spatial overlay admission failed: {0}")]
    Overlay(String),
}

#[cfg(test)]
mod pure_unit {
    use super::*;
    use simthing_core::SimThingId;

    fn cell(slot: u32, row: u32, col: u32) -> AdmittedTopologyCell {
        AdmittedTopologyCell {
            slot,
            value_col: 0,
            grid_row: row,
            grid_col: col,
            cell: SimThingId::new(),
        }
    }

    #[test]
    fn zero_and_multiple_loci_fail_closed_independent_of_order() {
        let a = cell(0, 0, 0);
        let b = cell(1, 0, 1);
        let mut cells = vec![a, b, b];
        assert!(matches!(
            resolve_authoritative_cell(&cells, 1, 0),
            Err(SpatialVendorizationError::AmbiguousDecisionLocus { slot: 1, col: 0 })
        ));
        cells.reverse();
        assert!(matches!(
            resolve_authoritative_cell(&cells, 1, 0),
            Err(SpatialVendorizationError::AmbiguousDecisionLocus { slot: 1, col: 0 })
        ));
        assert!(matches!(
            resolve_authoritative_cell(&cells, 9, 0),
            Err(SpatialVendorizationError::UnboundDecisionLocus { slot: 9, col: 0 })
        ));
    }

    #[test]
    fn multi_hop_and_teleport_are_red() {
        let a = cell(0, 0, 0);
        let far = cell(2, 1, 1);
        assert!(matches!(
            reject_non_adjacent(a, far),
            Err(SpatialVendorizationError::NotOneN4Edge { .. })
        ));
        assert!(matches!(
            reject_non_adjacent(a, a),
            Err(SpatialVendorizationError::NotOneN4Edge { .. })
        ));
    }
}
