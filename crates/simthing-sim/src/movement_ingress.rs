//! MOVEMENT-DECISION-INGRESS-0 — sealed field crossing to one boundary reparent.
//!
//! This module deliberately owns no destination planner, path, predecessor,
//! or physical-row instruction. A sealed structural commitment identifies one
//! admitted field cell. Admission proves that cell is one N4 edge from the
//! mover's current parent; boundary execution then reuses ordinary reparent,
//! routed overlay delivery, owner binding, and CostBand machinery.

use crate::sim_runtime_tree::SimRuntimeTree;
use crate::threshold_registry::CostBandSemantic;
use crate::tree_mutation::apply_structural_mutations;
use simthing_core::owner_channel::{bind_owner, resolve_owner, OwnerResolutionError};
use simthing_core::{
    admit_dispatch_minted_overlay, cost_band_quantize, dispatch_until_dissolved, CostBandDraw,
    DimensionRegistry, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, SimPropertyId, SimThing, SimThingId, SubFieldRole,
    TransformOp,
};
use simthing_feeder::{BoundaryRequest, MaintainerOutcome};
use simthing_gpu::SlotAllocator;
use simthing_kernel::StructuralCommitment;
use thiserror::Error;

/// One already-admitted identity binding for a logical field cell.
///
/// `slot`/`value_col` are the sealed decision locus. `grid_row`/`grid_col`
/// carry only admitted N4 topology; no field magnitude is available here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovementFieldLocus {
    pub slot: u32,
    pub value_col: u32,
    pub grid_row: u32,
    pub grid_col: u32,
    pub cell: SimThingId,
}

/// Ordinary overlay transform carried while the mover crosses the edge.
#[derive(Clone, Debug, PartialEq)]
pub struct MovementOverlayEffect {
    pub property_id: SimPropertyId,
    pub deltas: Vec<(SubFieldRole, TransformOp)>,
}

/// Boundary-ready movement. Fields stay private so callers cannot replace the
/// field-derived cell with a privileged destination after admission.
#[derive(Clone, Debug, PartialEq)]
pub struct MovementCommitment {
    commitment: StructuralCommitment,
    actor: SimThingId,
    from_cell: SimThingId,
    to_cell: SimThingId,
    field_width: u32,
    value_col: u32,
    from_row: u32,
    from_col: u32,
    to_row: u32,
    to_col: u32,
    cost_band: CostBandSemantic,
    unit_cost: f32,
    draw: CostBandDraw,
    overlay: Overlay,
}

impl MovementCommitment {
    /// Admit exactly one field-derived N4 edge from a sealed commitment.
    ///
    /// The destination is not an argument: it is reattached from the unique
    /// `(slot, value_col)` binding selected by the sealed GPU crossing.
    #[allow(clippy::too_many_arguments)]
    pub fn admit(
        commitment: StructuralCommitment,
        actor: SimThingId,
        actor_parent: SimThingId,
        field_width: u32,
        loci: &[MovementFieldLocus],
        effect: MovementOverlayEffect,
        cost_band: CostBandSemantic,
        unit_cost: f32,
    ) -> Result<Self, MovementIngressError> {
        if field_width == 0 {
            return Err(MovementIngressError::InvalidFieldTopology);
        }
        for locus in loci {
            let expected = locus
                .grid_row
                .checked_mul(field_width)
                .and_then(|v| v.checked_add(locus.grid_col))
                .ok_or(MovementIngressError::InvalidFieldTopology)?;
            if locus.grid_col >= field_width || locus.slot != expected {
                return Err(MovementIngressError::InvalidFieldTopology);
            }
        }

        let mut selected = loci
            .iter()
            .filter(|locus| locus.slot == commitment.slot() && locus.value_col == commitment.col());
        let to = *selected
            .next()
            .ok_or(MovementIngressError::UnboundDecisionLocus {
                slot: commitment.slot(),
                col: commitment.col(),
            })?;
        if selected.next().is_some() {
            return Err(MovementIngressError::AmbiguousDecisionLocus {
                slot: commitment.slot(),
                col: commitment.col(),
            });
        }

        let mut origins = loci.iter().filter(|locus| locus.cell == actor_parent);
        let from = *origins
            .next()
            .ok_or(MovementIngressError::ActorParentOutsideField { actor_parent })?;
        if origins.next().is_some() {
            return Err(MovementIngressError::AmbiguousActorParent { actor_parent });
        }
        if from.cell == to.cell || manhattan(from, to) != 1 {
            return Err(MovementIngressError::NotOneN4Edge {
                from: from.cell,
                to: to.cell,
            });
        }

        let draw = cost_band_quantize(
            commitment.value(),
            unit_cost,
            cost_band.is_sink,
            cost_band.throttle_hint_max_per_tick,
        )
        .map_err(|error| MovementIngressError::CostBand(error.to_string()))?;
        if cost_band.is_sink && draw.n != 1 {
            return Err(MovementIngressError::CostBandDidNotAuthorize { completed: draw.n });
        }
        if !cost_band.is_sink && (draw.n != 0 || draw.r.to_bits() != draw.v.to_bits()) {
            return Err(MovementIngressError::FreeRepositionConsumed);
        }

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
            lifecycle: dispatch_until_dissolved(vec![DissolveCondition::ArrivedAt {
                destination: to.cell,
            }])
            .expect("arrival is one non-empty dissolution condition"),
        };
        admit_dispatch_minted_overlay(&overlay)
            .map_err(|error| MovementIngressError::Overlay(error.to_string()))?;

        let request = Self {
            commitment,
            actor,
            from_cell: from.cell,
            to_cell: to.cell,
            field_width,
            value_col: to.value_col,
            from_row: from.grid_row,
            from_col: from.grid_col,
            to_row: to.grid_row,
            to_col: to.grid_col,
            cost_band,
            unit_cost,
            draw,
            overlay,
        };
        request.validate_integrity()?;
        Ok(request)
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

    pub fn cost_band_draw(&self) -> CostBandDraw {
        self.draw
    }

    pub fn overlay_id(&self) -> OverlayId {
        self.overlay.id
    }

    fn validate_integrity(&self) -> Result<(), MovementIngressError> {
        let selected_slot = self
            .to_row
            .checked_mul(self.field_width)
            .and_then(|v| v.checked_add(self.to_col))
            .ok_or(MovementIngressError::InvalidFieldTopology)?;
        if self.to_col >= self.field_width
            || self.from_col >= self.field_width
            || selected_slot != self.commitment.slot()
            || self.value_col != self.commitment.col()
        {
            return Err(MovementIngressError::DecisionLocusDrift);
        }
        let distance = self.from_row.abs_diff(self.to_row) + self.from_col.abs_diff(self.to_col);
        if self.from_cell == self.to_cell || distance != 1 {
            return Err(MovementIngressError::NotOneN4Edge {
                from: self.from_cell,
                to: self.to_cell,
            });
        }
        validate_movement_overlay(self.actor, self.to_cell, &self.overlay)?;
        validate_movement_cost_band(self.commitment, self.cost_band, self.unit_cost, self.draw)?;
        Ok(())
    }
}

/// Production validator used both at admission and immediately before apply.
/// It rejects missing/synthesized origins and every non-arrival lifecycle.
pub fn validate_movement_overlay(
    actor: SimThingId,
    deciding_cell: SimThingId,
    overlay: &Overlay,
) -> Result<(), MovementIngressError> {
    if overlay.origin != deciding_cell || overlay.affects != vec![actor] {
        return Err(MovementIngressError::OverlayOriginDrift);
    }
    match &overlay.lifecycle {
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        } if dissolution_conditions
            == &vec![DissolveCondition::ArrivedAt {
                destination: deciding_cell,
            }] => {}
        _ => return Err(MovementIngressError::ArrivalLifecycleRequired),
    }
    admit_dispatch_minted_overlay(overlay)
        .map_err(|error| MovementIngressError::Overlay(error.to_string()))?;
    Ok(())
}

/// Production CostBand validator. A stored direct decrement, altered draw, or
/// consumption on free repositioning cannot cross the boundary.
pub fn validate_movement_cost_band(
    commitment: StructuralCommitment,
    cost_band: CostBandSemantic,
    unit_cost: f32,
    draw: CostBandDraw,
) -> Result<(), MovementIngressError> {
    let expected = cost_band_quantize(
        commitment.value(),
        unit_cost,
        cost_band.is_sink,
        cost_band.throttle_hint_max_per_tick,
    )
    .map_err(|error| MovementIngressError::CostBand(error.to_string()))?;
    if expected != draw {
        return Err(MovementIngressError::CostBandBypass);
    }
    if cost_band.is_sink && draw.n != 1 {
        return Err(MovementIngressError::CostBandDidNotAuthorize { completed: draw.n });
    }
    if !cost_band.is_sink && (draw.n != 0 || draw.r.to_bits() != draw.v.to_bits()) {
        return Err(MovementIngressError::FreeRepositionConsumed);
    }
    Ok(())
}

fn manhattan(left: MovementFieldLocus, right: MovementFieldLocus) -> u32 {
    left.grid_row.abs_diff(right.grid_row) + left.grid_col.abs_diff(right.grid_col)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MovementIngressOutcome {
    pub applied: u32,
    pub rejected: u32,
    pub owner_root_binds: u32,
    pub cost_band_draws: Vec<CostBandDraw>,
    pub overlays_attached: Vec<(SimThingId, OverlayId)>,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum MovementIngressError {
    #[error("movement field topology is invalid")]
    InvalidFieldTopology,
    #[error("sealed decision locus ({slot},{col}) has no admitted field-cell identity")]
    UnboundDecisionLocus { slot: u32, col: u32 },
    #[error("sealed decision locus ({slot},{col}) maps to more than one field cell")]
    AmbiguousDecisionLocus { slot: u32, col: u32 },
    #[error("actor parent {actor_parent:?} is not an admitted field cell")]
    ActorParentOutsideField { actor_parent: SimThingId },
    #[error("actor parent {actor_parent:?} maps to more than one field cell")]
    AmbiguousActorParent { actor_parent: SimThingId },
    #[error("movement from {from:?} to {to:?} is not exactly one N4 edge")]
    NotOneN4Edge { from: SimThingId, to: SimThingId },
    #[error("movement decision locus drifted after sealed admission")]
    DecisionLocusDrift,
    #[error("movement overlay origin is not the sealed deciding cell")]
    OverlayOriginDrift,
    #[error("movement overlay requires UntilDissolvedWith one ArrivedAt condition")]
    ArrivalLifecycleRequired,
    #[error("movement CostBand admission failed: {0}")]
    CostBand(String),
    #[error("movement sink CostBand completed {completed} units instead of one")]
    CostBandDidNotAuthorize { completed: u32 },
    #[error("free repositioning consumed a CostBand unit")]
    FreeRepositionConsumed,
    #[error("movement bypassed or altered its admitted CostBand draw")]
    CostBandBypass,
    #[error("movement overlay admission failed: {0}")]
    Overlay(String),
    #[error("movement actor is no longer attached to its admitted origin cell")]
    ActorParentChanged,
    #[error("movement endpoint is missing from the authority tree")]
    MissingEndpoint,
    #[error("movement destination owner resolution failed: {0}")]
    OwnerResolution(String),
    #[error("ordinary structural movement application rejected")]
    StructuralMutationRejected,
}

/// Apply admitted movements through the ordinary structural mutation path.
/// Owner change is exactly one `bind_owner` on each successfully moved root.
pub fn apply_movement_commitments(
    movements: Vec<MovementCommitment>,
    root: &mut SimRuntimeTree,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
) -> (MaintainerOutcome, MovementIngressOutcome) {
    let mut maintainer = MaintainerOutcome::default();
    let mut outcome = MovementIngressOutcome::default();

    for movement in movements {
        let applied =
            apply_one_movement(&movement, root, allocator, registry, values_shadow, n_dims);
        match applied {
            Ok(one) => {
                merge_maintainer(&mut maintainer, one);
                outcome.applied += 1;
                outcome.owner_root_binds += 1;
                outcome.cost_band_draws.push(movement.draw);
                outcome
                    .overlays_attached
                    .push((movement.actor, movement.overlay.id));
            }
            Err(_) => outcome.rejected += 1,
        }
    }

    (maintainer, outcome)
}

fn apply_one_movement(
    movement: &MovementCommitment,
    root: &mut SimRuntimeTree,
    allocator: &mut SlotAllocator,
    registry: &mut DimensionRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
) -> Result<MaintainerOutcome, MovementIngressError> {
    movement.validate_integrity()?;
    if !root.contains_id(movement.actor)
        || !root.contains_id(movement.from_cell)
        || !root.contains_id(movement.to_cell)
    {
        return Err(MovementIngressError::MissingEndpoint);
    }
    if allocator.relation_of(movement.actor)
        != Some(simthing_core::ObjectResidencyRelation::ChildOf(
            movement.from_cell,
        ))
    {
        return Err(MovementIngressError::ActorParentChanged);
    }
    let destination_owner =
        resolve_owner(root.inner(), movement.to_cell).map_err(owner_resolution_error)?;

    let applied = apply_structural_mutations(
        vec![
            BoundaryRequest::Reparent {
                child: movement.actor,
                new_parent: movement.to_cell,
            },
            BoundaryRequest::AttachOverlay {
                target: movement.actor,
                overlay: movement.overlay.clone(),
            },
        ],
        root,
        allocator,
        registry,
        values_shadow,
        n_dims,
        None,
    );
    let moved = applied
        .reparented
        .iter()
        .any(|pair| *pair == (movement.actor, movement.to_cell));
    let attached = applied
        .overlays_attached
        .iter()
        .any(|pair| *pair == (movement.actor, movement.overlay.id));
    if !moved || !attached {
        return Err(MovementIngressError::StructuralMutationRejected);
    }

    let actor = find_node_mut(root.inner_mut(), movement.actor)
        .ok_or(MovementIngressError::MissingEndpoint)?;
    bind_owner(actor, &destination_owner);
    Ok(applied)
}

fn owner_resolution_error(error: OwnerResolutionError) -> MovementIngressError {
    MovementIngressError::OwnerResolution(error.to_string())
}

fn find_node_mut(node: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_node_mut(child, id))
}

pub(crate) fn merge_maintainer(target: &mut MaintainerOutcome, mut source: MaintainerOutcome) {
    target.adds += source.adds;
    target.removes += source.removes;
    target.reparents += source.reparents;
    target.overlays += source.overlays;
    target.overlay_activations += source.overlay_activations;
    target.overlay_suspensions += source.overlay_suspensions;
    target.dimensions += source.dimensions;
    target.rejected_unknown_target += source.rejected_unknown_target;
    target.deferred += source.deferred;
    target.allocated.append(&mut source.allocated);
    target.tombstoned.append(&mut source.tombstoned);
    target
        .overlays_attached
        .append(&mut source.overlays_attached);
    target
        .overlays_activated
        .append(&mut source.overlays_activated);
    target
        .overlays_suspended
        .append(&mut source.overlays_suspended);
    target.dimensions_added.append(&mut source.dimensions_added);
    target.reparented.append(&mut source.reparented);
}
