//! ACTIONBAND-SPATIAL-VENDORIZATION-0 — born-mortal pure consumer of ActionBand
//! spatial progress.
//!
//! This leaf module re-proves §13.1 fences as a disposable workshop witness. It
//! does **not** define a production Destination, path, predecessor, planner,
//! crossing detector, or peer movement executor. Deleting it cannot remove the
//! graduated ActionBand facility in production crates.
//!
//! Authority boundary: spatial admission consumes the existing sealed
//! [`StructuralCommitment`] minted by the graduated ActionBand + Phase-5 path.
//! Raw slot/col/value/event integers are not an admission door.
//!
//! This consumer only:
//! - reattaches the sealed locus to exactly one admitted logical cell,
//! - fails closed on zero or multiple loci (physical-row order never chooses),
//! - requires exactly one N4 adjacent structural edge,
//! - quantizes consumption through ordinary CostBand algebra,
//! - mints an ordinary overlay with a real origin and lawful lifecycle.
//!
//! Slot identity is an opaque sealed key in an admitted mapping table — never
//! required to equal `row * width + col`. Structural `(row,col)` + logical cell
//! identity govern spatial meaning.

use simthing_core::{
    admit_dispatch_minted_overlay, cost_band_quantize, CostBandDraw, DissolveCondition, Overlay,
    OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimPropertyId,
    SimThingId, SubFieldRole, TransformOp,
};
use simthing_kernel::StructuralCommitment;
use thiserror::Error;

/// One admitted synthetic topology cell.
///
/// `slot` is the opaque sealed ActionBand/Phase-5 locus key from the admitted
/// mapping — it is **not** structural coordinates and need not be row-major.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedTopologyCell {
    /// Opaque sealed locus key (ActionBand / GPU slot identity).
    pub sealed_slot: u32,
    /// Sealed value-plane column expected on the commitment.
    pub sealed_col: u32,
    /// Structural N4 coordinates (authoritative spatial geometry).
    pub grid_row: u32,
    pub grid_col: u32,
    /// Logical cell identity.
    pub cell: SimThingId,
}

/// Ordinary overlay transform carried while the actor crosses the edge.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialStepOverlayEffect {
    pub property_id: SimPropertyId,
    pub deltas: Vec<(SubFieldRole, TransformOp)>,
}

/// Boundary-ready spatial step admitted from a sealed ActionBand commitment.
///
/// Destination is not an argument: it is reattached from the unique sealed
/// locus on the typed commitment. Callers cannot replace the field-derived
/// cell after admission.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialVendorizationStep {
    actor: SimThingId,
    from_cell: SimThingId,
    to_cell: SimThingId,
    from_row: u32,
    from_col: u32,
    to_row: u32,
    to_col: u32,
    commitment: StructuralCommitment,
    unit_cost: f32,
    is_sink: bool,
    throttle: Option<u32>,
    draw: CostBandDraw,
    overlay: Overlay,
}

impl SpatialVendorizationStep {
    /// Admit exactly one field-derived N4 edge from a sealed ActionBand product.
    ///
    /// Locus/value/event identity is derived only from `commitment` — the
    /// graduated typed seal. Raw integers are not accepted at this door.
    #[allow(clippy::too_many_arguments)]
    pub fn admit(
        commitment: StructuralCommitment,
        actor: SimThingId,
        actor_parent: SimThingId,
        cells: &[AdmittedTopologyCell],
        effect: SpatialStepOverlayEffect,
        is_sink: bool,
        unit_cost: f32,
        throttle: Option<u32>,
    ) -> Result<Self, SpatialVendorizationError> {
        validate_admitted_mapping(cells)?;

        let to = resolve_authoritative_cell(cells, commitment.slot(), commitment.col())?;
        let from = resolve_authoritative_parent(cells, actor_parent)?;
        if from.cell == to.cell || manhattan(from, to) != 1 {
            return Err(SpatialVendorizationError::NotOneN4Edge {
                from: from.cell,
                to: to.cell,
            });
        }

        let draw = cost_band_quantize(commitment.value(), unit_cost, is_sink, throttle)
            .map_err(|error| SpatialVendorizationError::CostBand(error.to_string()))?;
        if is_sink && draw.n != 1 {
            return Err(SpatialVendorizationError::CostBandDidNotAuthorize { completed: draw.n });
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
            from_row: from.grid_row,
            from_col: from.grid_col,
            to_row: to.grid_row,
            to_col: to.grid_col,
            commitment,
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

    pub fn commitment(&self) -> StructuralCommitment {
        self.commitment
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
            self.commitment.value(),
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

    pub fn validate_overlay(&self) -> Result<(), SpatialVendorizationError> {
        validate_spatial_overlay(self.actor, self.to_cell, &self.overlay)
    }

    fn validate_integrity(&self) -> Result<(), SpatialVendorizationError> {
        // Structural geometry only — never re-derive sealed slot from row-major.
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
    sealed_slot: u32,
    sealed_col: u32,
) -> Result<AdmittedTopologyCell, SpatialVendorizationError> {
    let mut selected = cells
        .iter()
        .filter(|cell| cell.sealed_slot == sealed_slot && cell.sealed_col == sealed_col);
    let first = *selected
        .next()
        .ok_or(SpatialVendorizationError::UnboundDecisionLocus {
            slot: sealed_slot,
            col: sealed_col,
        })?;
    if selected.next().is_some() {
        return Err(SpatialVendorizationError::AmbiguousDecisionLocus {
            slot: sealed_slot,
            col: sealed_col,
        });
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

/// Mapping validity: unique sealed keys, unique logical cells, finite coords.
/// Deliberately does **not** require `sealed_slot == row * width + col`.
fn validate_admitted_mapping(
    cells: &[AdmittedTopologyCell],
) -> Result<(), SpatialVendorizationError> {
    if cells.is_empty() {
        return Err(SpatialVendorizationError::InvalidFieldTopology);
    }
    for (i, cell) in cells.iter().enumerate() {
        for other in cells.iter().skip(i + 1) {
            if cell.sealed_slot == other.sealed_slot && cell.sealed_col == other.sealed_col {
                return Err(SpatialVendorizationError::AmbiguousDecisionLocus {
                    slot: cell.sealed_slot,
                    col: cell.sealed_col,
                });
            }
            if cell.cell == other.cell {
                return Err(SpatialVendorizationError::InvalidFieldTopology);
            }
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

    fn cell(sealed_slot: u32, row: u32, col: u32) -> AdmittedTopologyCell {
        AdmittedTopologyCell {
            sealed_slot,
            sealed_col: 0,
            grid_row: row,
            grid_col: col,
            cell: SimThingId::new(),
        }
    }

    #[test]
    fn zero_and_multiple_loci_fail_closed_independent_of_order() {
        // Non-row-major sealed keys: structural (0,1) is not sealed_slot 1.
        let a = cell(10, 0, 0);
        let b = cell(99, 0, 1);
        let mut cells = vec![a, b, b];
        assert!(matches!(
            resolve_authoritative_cell(&cells, 99, 0),
            Err(SpatialVendorizationError::AmbiguousDecisionLocus { slot: 99, col: 0 })
        ));
        cells.reverse();
        assert!(matches!(
            resolve_authoritative_cell(&cells, 99, 0),
            Err(SpatialVendorizationError::AmbiguousDecisionLocus { slot: 99, col: 0 })
        ));
        assert!(matches!(
            resolve_authoritative_cell(&cells, 7, 0),
            Err(SpatialVendorizationError::UnboundDecisionLocus { slot: 7, col: 0 })
        ));
    }

    #[test]
    fn multi_hop_and_teleport_are_red() {
        let a = cell(10, 0, 0);
        let far = cell(40, 1, 1);
        assert!(matches!(
            reject_non_adjacent(a, far),
            Err(SpatialVendorizationError::NotOneN4Edge { .. })
        ));
        assert!(matches!(
            reject_non_adjacent(a, a),
            Err(SpatialVendorizationError::NotOneN4Edge { .. })
        ));
    }

    #[test]
    fn sealed_slot_need_not_equal_row_major_index() {
        // Structural (1,0) with sealed_slot 777 — mapping is admitted, not formulaic.
        let a = cell(50, 0, 0);
        let c = cell(777, 1, 0);
        let cells = [a, c];
        let resolved = resolve_authoritative_cell(&cells, 777, 0).unwrap();
        assert_eq!(resolved.grid_row, 1);
        assert_eq!(resolved.grid_col, 0);
        assert_ne!(
            resolved.sealed_slot,
            resolved.grid_row * 2 + resolved.grid_col
        );
        assert_eq!(manhattan(a, c), 1);
    }
}
