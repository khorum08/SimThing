//! RESOLUTION-SITE-SPLIT-0 — one model, two resolution sites.
//!
//! The model (core design §8, the Vendorized Build Principle): SimThings resolve
//! state via field arithmetic; crossings are observations; CostBands are actions;
//! allocation happens at a barrier. The closed loop resolves in slot space and is
//! the DEFAULT placement; the CPU-authoritative system is a VENDORIZED BUILD of
//! the SAME model whose resolution products attach identity from the CPU semantic
//! table's registration-time mirror. Same math, same crossing selection, same
//! `BoundaryRequest` vocabulary, same barrier allocation — different placement.
//!
//! This module holds both placements' identity doors side by side so the split is
//! visibly a placement, never a fork:
//!
//! - **Closed loop** (default): crossings stay `{slot, col, value, event_kind}`
//!   through the loop; identity is re-attached ONLY at the barrier through the
//!   admitted slot map ([`SlotAllocator::owner_of`] — the live authority) and the
//!   registered column owners. Re-attachment is TOTAL over converted crossings and
//!   FAILS CLOSED: a slot with no admitted SimThing is an admission-integrity
//!   error, never a default or synthesized identity.
//! - **CPU-authoritative** (vendorized): the pre-split production arms
//!   ([`collect_velocity_alerts_vendorized`] / [`collect_aggregate_alerts_vendorized`])
//!   pull identity from the [`ThresholdSemantic`] entry stored at registration
//!   time — the semantic-shadow mirror, demoted from authority to mirror by the
//!   parity referees that hold both placements bit-identical.
//!
//! Closed-loop overlay origination carries `origin` in SLOT SPACE
//! ([`SlotSpaceOverlayDraft`]); the required `Overlay.origin: SimThingId` type
//! boundary (6.0b) holds at the CPU representation because a draft can only
//! become an [`Overlay`] through [`mint_attach_overlay_at_barrier`], which
//! re-attaches identity through the admitted slot map exactly like every other
//! identity re-attachment. There is no in-loop `SimThingId` and no slot→id
//! fallback.
//!
//! The 13-stage boundary pipeline is not this module's concern and is untouched:
//! genuine allocation (fission pre-grow, fission/fusion, lineage, AddChild
//! pre-grow, structural mutations, dimension rebuild, capacity) stays at the
//! barrier in BOTH placements.

use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimPropertyId, SimThingId, SlotIndex, SubFieldRole,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{SlotAllocator, ThresholdEvent};
use thiserror::Error;

use crate::threshold_registry::{
    AggregateAlertEvent, ThresholdRegistry, ThresholdSemantic, VelocityAlertEvent,
};

/// Placement of semantic resolution for converted boundary semantics.
///
/// Placement only — one semantic table, one crossing vocabulary, one barrier
/// allocation path. A second semantic vocabulary or a CPU-vs-closed-loop fork in
/// MEANING is constitutionally forbidden; the variants may only differ in where
/// identity attaches to resolution products.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResolutionSite {
    /// Default placement: crossings stay in slot space through the loop and
    /// identity is re-attached only at the barrier through the admitted slot
    /// map (the live authority).
    #[default]
    ClosedLoop,
    /// Vendorized placement: resolution products attach identity from the CPU
    /// semantic table's registration-time entries (the mirror). This is the
    /// pre-split production shape, retained as a derived instance of the same
    /// model — never a second system.
    CpuAuthoritative,
}

/// Fail-closed barrier re-attachment error. Every variant is an
/// admission-integrity failure: the answer is never a default identity.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SlotIdentityReattachError {
    #[error(
        "crossing slot {slot} (event_kind {event_kind}) has no admitted SimThing; \
         slot->id re-attachment fails closed — never a default identity"
    )]
    UnadmittedSlot { slot: u32, event_kind: u32 },
    #[error(
        "crossing col {col} (event_kind {event_kind}) has no registered owner column; \
         re-attachment fails closed"
    )]
    UnownedColumn { col: u32, event_kind: u32 },
    #[error(
        "crossing col {col} (event_kind {event_kind}) resolves no sub-field role in the \
         owning property layout; re-attachment fails closed"
    )]
    UnresolvedRole { col: u32, event_kind: u32 },
    #[error(
        "overlay draft {overlay:?} origin slot {slot} has no admitted SimThing; \
         a synthesized or default Overlay.origin is forbidden — fails closed"
    )]
    UnadmittedOriginSlot { slot: u32, overlay: OverlayId },
    #[error(
        "overlay draft {overlay:?} target slot {slot} has no admitted SimThing; \
         fails closed"
    )]
    UnadmittedTargetSlot { slot: u32, overlay: OverlayId },
}

/// Derive `{SimThingId, SimPropertyId, SubFieldRole}` for one crossing locus from
/// the admitted slot map + registered column owners. TOTAL and FAIL-CLOSED.
fn reattach_crossing_identity(
    event: &ThresholdEvent,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Result<(SimThingId, SimPropertyId, SubFieldRole), SlotIdentityReattachError> {
    let sim_thing_id = allocator
        .owner_of(SlotIndex::new(event.slot()))
        .ok_or(SlotIdentityReattachError::UnadmittedSlot {
            slot: event.slot(),
            event_kind: event.event_kind(),
        })?;
    let (property_id, offset) = *registry.column_owners.get(event.col() as usize).ok_or(
        SlotIdentityReattachError::UnownedColumn {
            col: event.col(),
            event_kind: event.event_kind(),
        },
    )?;
    let role = registry
        .try_property(property_id)
        .and_then(|prop| role_at_offset(&prop.layout.sub_fields, offset))
        .ok_or(SlotIdentityReattachError::UnresolvedRole {
            col: event.col(),
            event_kind: event.event_kind(),
        })?;
    Ok((sim_thing_id, property_id, role))
}

fn role_at_offset(
    sub_fields: &[simthing_core::SubFieldSpec],
    offset: usize,
) -> Option<SubFieldRole> {
    let mut at = 0usize;
    for sf in sub_fields {
        if offset < at + sf.width {
            return Some(sf.role.clone());
        }
        at += sf.width;
    }
    None
}

/// Closed-loop placement: velocity-alert products re-attached at the barrier
/// through the admitted slot map. Crossing selection is IDENTICAL to the
/// vendorized arm (the one semantic table decides which kinds are velocity
/// alerts); only the identity source differs.
pub fn reattach_velocity_alerts_at_barrier(
    events: &[ThresholdEvent],
    semantics: &ThresholdRegistry,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Result<Vec<VelocityAlertEvent>, SlotIdentityReattachError> {
    let mut out = Vec::new();
    for event in events {
        let Some(ThresholdSemantic::VelocityAlert { .. }) = semantics.get(event.event_kind())
        else {
            continue;
        };
        let (sim_thing_id, property_id, sub_field) =
            reattach_crossing_identity(event, registry, allocator)?;
        out.push(VelocityAlertEvent {
            sim_thing_id,
            property_id,
            sub_field,
            value: event.value(),
        });
    }
    Ok(out)
}

/// Closed-loop placement: aggregate-alert products re-attached at the barrier
/// through the admitted slot map. Same selection, same product shape as the
/// vendorized arm.
pub fn reattach_aggregate_alerts_at_barrier(
    events: &[ThresholdEvent],
    semantics: &ThresholdRegistry,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Result<Vec<AggregateAlertEvent>, SlotIdentityReattachError> {
    let mut out = Vec::new();
    for event in events {
        let Some(ThresholdSemantic::AggregateAlert { .. }) = semantics.get(event.event_kind())
        else {
            continue;
        };
        let (sim_thing_id, property_id, sub_field) =
            reattach_crossing_identity(event, registry, allocator)?;
        out.push(AggregateAlertEvent {
            sim_thing_id,
            property_id,
            sub_field,
            value: event.value(),
        });
    }
    Ok(out)
}

/// Vendorized placement: the pre-split production velocity-alert arm. Identity
/// comes from the semantic table's registration-time entry (the mirror). Zero
/// decision logic — pure identity re-attachment, which is why the closed-loop
/// twin of this arm evaporates into [`reattach_velocity_alerts_at_barrier`]
/// rather than porting.
pub fn collect_velocity_alerts_vendorized(
    events: &[ThresholdEvent],
    semantics: &ThresholdRegistry,
) -> Vec<VelocityAlertEvent> {
    events
        .iter()
        .filter_map(|event| {
            let ThresholdSemantic::VelocityAlert {
                sim_thing_id,
                property_id,
                sub_field,
            } = semantics.get(event.event_kind())?
            else {
                return None;
            };
            Some(VelocityAlertEvent {
                sim_thing_id: *sim_thing_id,
                property_id: *property_id,
                sub_field: sub_field.clone(),
                value: event.value(),
            })
        })
        .collect()
}

/// Vendorized placement: the pre-split production aggregate-alert arm (mirror
/// identity). See [`collect_velocity_alerts_vendorized`].
pub fn collect_aggregate_alerts_vendorized(
    events: &[ThresholdEvent],
    semantics: &ThresholdRegistry,
) -> Vec<AggregateAlertEvent> {
    events
        .iter()
        .filter_map(|event| {
            let ThresholdSemantic::AggregateAlert {
                sim_thing_id,
                property_id,
                sub_field,
            } = semantics.get(event.event_kind())?
            else {
                return None;
            };
            Some(AggregateAlertEvent {
                sim_thing_id: *sim_thing_id,
                property_id: *property_id,
                sub_field: sub_field.clone(),
                value: event.value(),
            })
        })
        .collect()
}

/// Closed-loop overlay origination carrier: origin (and target) in SLOT SPACE.
///
/// Inside the loop there is no `SimThingId` in scope — the Wei automaton
/// originates overlays intrinsically from cells addressed by slot. This draft is
/// the CPU representation of that fact. It deliberately cannot satisfy 6.0b's
/// required `Overlay.origin` on its own: the ONLY way a draft becomes an
/// [`Overlay`] is [`mint_attach_overlay_at_barrier`], where identity is
/// re-attached through the admitted slot map, totally and fail-closed.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotSpaceOverlayDraft {
    pub id: OverlayId,
    pub kind: OverlayKind,
    pub source: OverlaySource,
    /// Originating cell in slot space. Never a `SimThingId`.
    pub origin_slot: SlotIndex,
    /// Delivery target in slot space.
    pub target_slot: SlotIndex,
    pub transform: PropertyTransformDelta,
    pub lifecycle: OverlayLifecycle,
}

/// Barrier door: re-attach identity for a slot-space overlay draft through the
/// admitted slot map and mint the routed attachment request.
///
/// TOTAL over admitted drafts and FAIL-CLOSED: a slot with no admitted SimThing
/// is an admission-integrity failure ([`SlotIdentityReattachError`]), never a
/// default origin. The minted request is `BoundaryRequest::AttachOverlay`, whose
/// application routes through `deliver_routed_overlay` (6.0b) — `affects` is
/// left empty here and set by routed delivery, so direct-`affects` bypass is
/// structurally impossible from this door.
pub fn mint_attach_overlay_at_barrier(
    draft: &SlotSpaceOverlayDraft,
    allocator: &SlotAllocator,
) -> Result<BoundaryRequest, SlotIdentityReattachError> {
    let origin = allocator.owner_of(draft.origin_slot).ok_or(
        SlotIdentityReattachError::UnadmittedOriginSlot {
            slot: draft.origin_slot.raw(),
            overlay: draft.id,
        },
    )?;
    let target = allocator.owner_of(draft.target_slot).ok_or(
        SlotIdentityReattachError::UnadmittedTargetSlot {
            slot: draft.target_slot.raw(),
            overlay: draft.id,
        },
    )?;
    Ok(BoundaryRequest::AttachOverlay {
        target,
        overlay: Overlay {
            id: draft.id,
            kind: draft.kind.clone(),
            source: draft.source.clone(),
            origin,
            affects: Vec::new(),
            transform: draft.transform.clone(),
            lifecycle: draft.lifecycle.clone(),
        },
    })
}

/// PLANTED MUTANT (referee support — never a production door). The forbidden
/// shape this rung's fail-closed law exists to prevent: on an unadmitted origin
/// slot, substitute a synthesized fallback origin instead of failing closed.
/// Referees prove the real door ([`mint_attach_overlay_at_barrier`]) errors on
/// exactly the input where this mutant fabricates an attributable overlay.
pub fn plant_default_origin_mutant_mint(
    draft: &SlotSpaceOverlayDraft,
    allocator: &SlotAllocator,
    fallback_origin: SimThingId,
) -> BoundaryRequest {
    let origin = allocator
        .owner_of(draft.origin_slot)
        .unwrap_or(fallback_origin);
    let target = allocator
        .owner_of(draft.target_slot)
        .unwrap_or(fallback_origin);
    BoundaryRequest::AttachOverlay {
        target,
        overlay: Overlay {
            id: draft.id,
            kind: draft.kind.clone(),
            source: draft.source.clone(),
            origin,
            affects: Vec::new(),
            transform: draft.transform.clone(),
            lifecycle: draft.lifecycle.clone(),
        },
    }
}

/// PLANTED MUTANT (referee support — never a production door). Semantic
/// divergence in the origination path: the minted overlay's transform is
/// perturbed relative to the draft, so the two placements stop producing
/// bit-identical `BoundaryRequest` streams. Parity referees must RED on this.
pub fn plant_transform_divergence_mutant_mint(
    draft: &SlotSpaceOverlayDraft,
    allocator: &SlotAllocator,
) -> Result<BoundaryRequest, SlotIdentityReattachError> {
    let mut diverged = draft.clone();
    diverged.transform.sub_field_deltas.pop();
    mint_attach_overlay_at_barrier(&diverged, allocator)
}
