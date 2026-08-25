//! Sealed write-impact band-crossing deltas (WRITE-DOOR-BAND-DELTA-0).
//!
//! Minted only from fused-pass threshold emissions joined with registration
//! sidecars under the canonical DimensionRegistry Anchored filter. External
//! crates cannot forge write-impact evidence.

use serde::{Deserialize, Serialize};
use simthing_core::{
    ColumnIndex, DimensionRegistry, PropertyLayout, SimPropertyId, SimThingId, SlotIndex,
    SubFieldRole,
};

use crate::registration::{ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD};
use crate::sealed::{ThresholdEmission, ThresholdEvent};
use crate::slot::SlotAllocator;
use crate::wgsl_encode::column_from_wire;

/// Deterministic direction of a crossed band edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BandCrossingDirection {
    Rising,
    Falling,
}

/// One sealed write-impact band-crossing delta derived in the fused GPU pass.
///
/// External crates cannot forge band deltas directly:
///
/// ```compile_fail,E0616
/// fn external_band_crossing_delta_field_access(delta: &simthing_kernel::BandCrossingDelta) {
///     let _ = delta.reg_idx;
/// }
/// ```
///
/// External crates cannot forge band deltas via a public named constructor:
///
/// ```compile_fail,E0624
/// fn external_band_crossing_delta_named_forge() {
///     let _ = simthing_kernel::BandCrossingDelta::from_fused_threshold_emission(
///         /* emission */ unimplemented!(),
///         /* reg */ unimplemented!(),
///         /* sim_thing_id */ unimplemented!(),
///         /* property_id */ unimplemented!(),
///         /* role */ unimplemented!(),
///         /* slot */ unimplemented!(),
///         /* col */ unimplemented!(),
///     );
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandCrossingDelta {
    generation: u32,
    reg_idx: u32,
    sim_thing_id: SimThingId,
    property_id: SimPropertyId,
    role: SubFieldRole,
    slot: SlotIndex,
    col: ColumnIndex,
    threshold: f32,
    direction: BandCrossingDirection,
    post_value: f32,
    event_kind: u32,
}

impl BandCrossingDelta {
    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn reg_idx(&self) -> u32 {
        self.reg_idx
    }

    pub fn sim_thing_id(&self) -> SimThingId {
        self.sim_thing_id
    }

    pub fn property_id(&self) -> SimPropertyId {
        self.property_id
    }

    pub fn role(&self) -> &SubFieldRole {
        &self.role
    }

    pub fn slot(&self) -> SlotIndex {
        self.slot
    }

    pub fn col(&self) -> ColumnIndex {
        self.col
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    pub fn direction(&self) -> BandCrossingDirection {
        self.direction
    }

    pub fn post_value(&self) -> f32 {
        self.post_value
    }

    pub fn event_kind(&self) -> u32 {
        self.event_kind
    }

    /// Sealed operands for CostBand quantize: `post_value` = V, `threshold` = C.
    /// Production authority for sink/throttle is the CPU `event_kind` semantic
    /// table (`ThresholdRegistry::resolve_cost_band_draw_from_delta`) — not a
    /// caller-supplied `is_sink` flag on this method.
    pub fn cost_band_operands(&self) -> (f32, f32) {
        (self.post_value(), self.threshold())
    }

    pub(crate) fn from_fused_threshold_emission(
        emission: &ThresholdEmission,
        reg: &ThresholdRegistration,
        sim_thing_id: SimThingId,
        property_id: SimPropertyId,
        role: SubFieldRole,
        slot: SlotIndex,
        col: ColumnIndex,
    ) -> Option<Self> {
        if emission.slot() != reg.slot || emission.col() != reg.col {
            return None;
        }
        if emission.slot() != slot.raw() || emission.col() as usize != col.raw() {
            return None;
        }
        let direction = match reg.direction {
            DIR_UPWARD => BandCrossingDirection::Rising,
            DIR_DOWNWARD => BandCrossingDirection::Falling,
            DIR_EITHER => {
                // Either-direction registrations still emit one evidence row; rising
                // when post is above the edge, otherwise falling.
                if emission.value() > reg.threshold {
                    BandCrossingDirection::Rising
                } else {
                    BandCrossingDirection::Falling
                }
            }
            _ => return None,
        };
        Some(Self {
            generation: emission.generation(),
            reg_idx: emission.reg_idx(),
            sim_thing_id,
            property_id,
            role,
            slot,
            col,
            threshold: reg.threshold,
            direction,
            post_value: emission.value(),
            event_kind: reg.event_kind,
        })
    }
}

fn role_at_offset(layout: &PropertyLayout, offset: usize) -> Option<SubFieldRole> {
    let mut at = 0usize;
    for sf in &layout.sub_fields {
        if offset < at + sf.width {
            return Some(sf.role.clone());
        }
        at += sf.width;
    }
    None
}

/// Resolve admitted Anchored identity for a fused emission locus.
///
/// Unobserved / inactive / unknown columns return `None` (no write-impact row).
fn resolve_anchored_identity(
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    slot: u32,
    col: u32,
) -> Option<(
    SimThingId,
    SimPropertyId,
    SubFieldRole,
    SlotIndex,
    ColumnIndex,
)> {
    let slot_idx = SlotIndex::new(slot);
    let sim_thing_id = allocator.owner_of(slot_idx)?;
    let (property_id, offset) = *registry.column_owners.get(col as usize)?;
    let prop = registry.try_property(property_id)?;
    if !registry.is_active(property_id) || !prop.admission_disposition.is_anchored() {
        return None;
    }
    let role = role_at_offset(&prop.layout, offset)?;
    // Sole production rematerialize door for GPU wire columns.
    let col_idx = column_from_wire(col);
    Some((sim_thing_id, property_id, role, slot_idx, col_idx))
}

/// Join fused-pass threshold emissions with registration sidecars into sealed
/// write-impact deltas. Order follows emission append order (`reg_idx` ladder).
///
/// Anchored eligibility is derived from the canonical registry — callers cannot
/// supply a column allowlist.
pub fn apply_band_crossing_deltas_from_fused_emissions(
    emissions: &[ThresholdEmission],
    regs: &[ThresholdRegistration],
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Vec<BandCrossingDelta> {
    let mut out = Vec::with_capacity(emissions.len());
    for emission in emissions {
        let Some(reg) = regs.get(emission.reg_idx() as usize) else {
            continue;
        };
        let Some((sim_thing_id, property_id, role, slot, col)) =
            resolve_anchored_identity(registry, allocator, emission.slot(), emission.col())
        else {
            continue;
        };
        if let Some(delta) = BandCrossingDelta::from_fused_threshold_emission(
            emission,
            reg,
            sim_thing_id,
            property_id,
            role,
            slot,
            col,
        ) {
            out.push(delta);
        }
    }
    out
}

/// Mint sealed write-impact deltas from already-read threshold events + regs.
///
/// Production boundary path: events arrive from the fused pass; this door joins
/// them to registrations under the canonical Anchored registry filter without a
/// new public band-delta readback API.
pub fn apply_band_crossing_deltas_from_threshold_events(
    events: &[ThresholdEvent],
    regs: &[ThresholdRegistration],
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Vec<BandCrossingDelta> {
    let mut emissions = Vec::with_capacity(events.len());
    for event in events {
        let Some((reg_idx, _)) = regs.iter().enumerate().find(|(_, reg)| {
            reg.event_kind == event.event_kind()
                && reg.slot == event.slot()
                && reg.col == event.col()
        }) else {
            continue;
        };
        emissions.push(ThresholdEmission::from_cpu_oracle(
            reg_idx as u32,
            event.slot(),
            event.col(),
            event.value(),
            event.generation(),
        ));
    }
    apply_band_crossing_deltas_from_fused_emissions(&emissions, regs, registry, allocator)
}

/// CPU-oracle twin: derive sealed deltas from previous/current buffers using the
/// same crossing predicate as Pass 7 / `cpu_oracle_threshold_events`.
///
/// Anchored eligibility comes from the canonical registry (no caller column filter).
pub fn cpu_oracle_band_crossing_deltas(
    previous_values: &[f32],
    values: &[f32],
    previous_output: &[f32],
    output: &[f32],
    n_dims: u32,
    regs: &[ThresholdRegistration],
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> Vec<BandCrossingDelta> {
    use crate::registration::THRESH_BUF_OUTPUT;
    use crate::sealed::ThresholdEmission;

    let mut emissions = Vec::new();
    for (reg_idx, r) in regs.iter().enumerate() {
        // Registry filter first — Unobserved / non-anchored never invent evidence.
        if resolve_anchored_identity(registry, allocator, r.slot, r.col).is_none() {
            continue;
        }
        let addr = (r.slot * n_dims + r.col) as usize;
        let (prev, curr) = if r.buffer == THRESH_BUF_OUTPUT {
            (previous_output[addr], output[addr])
        } else {
            (previous_values[addr], values[addr])
        };
        let up = prev <= r.threshold && curr > r.threshold;
        let down = prev >= r.threshold && curr < r.threshold;
        let crossed = match r.direction {
            DIR_UPWARD => up,
            DIR_DOWNWARD => down,
            _ => up || down,
        };
        if crossed {
            emissions.push(ThresholdEmission::from_cpu_oracle(
                reg_idx as u32,
                r.slot,
                r.col,
                curr,
                0,
            ));
        }
    }
    apply_band_crossing_deltas_from_fused_emissions(&emissions, regs, registry, allocator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::THRESH_BUF_VALUES;
    use crate::sealed::ThresholdEmission;
    use simthing_core::{PropertyAdmissionDisposition, SimProperty, SimThing, SimThingKind};

    fn anchored_registry_and_allocator(n_slots: u32) -> (DimensionRegistry, SlotAllocator) {
        let mut registry = DimensionRegistry::new();
        let _ = registry.register(SimProperty::simple("test", "anchored", 1));
        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        for _ in 1..n_slots {
            root.add_child(SimThing::new(SimThingKind::Location, 0));
        }
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        assert_eq!(allocator.capacity() as u32, n_slots);
        (registry, allocator)
    }

    #[test]
    fn rising_edge_mints_rising_delta() {
        let (registry, allocator) = anchored_registry_and_allocator(2);
        let owner = allocator.owner_of(SlotIndex::new(1)).expect("slot 1 owner");
        let reg = ThresholdRegistration {
            slot: 1,
            col: 0,
            threshold: 10.0,
            direction: DIR_UPWARD,
            event_kind: 7,
            buffer: THRESH_BUF_VALUES,
        };
        let emission = ThresholdEmission::from_cpu_oracle(0, 1, 0, 10.5, 0);
        let deltas = apply_band_crossing_deltas_from_fused_emissions(
            &[emission],
            &[reg],
            &registry,
            &allocator,
        );
        assert_eq!(deltas.len(), 1);
        let delta = &deltas[0];
        assert_eq!(delta.direction(), BandCrossingDirection::Rising);
        assert_eq!(delta.post_value(), 10.5);
        assert_eq!(delta.threshold(), 10.0);
        assert_eq!(delta.event_kind(), 7);
        assert_eq!(delta.slot(), SlotIndex::new(1));
        assert_eq!(delta.col().raw(), 0);
        assert_eq!(delta.sim_thing_id(), owner);
    }

    #[test]
    fn exact_edge_landing_is_not_a_rising_crossing() {
        let (registry, allocator) = anchored_registry_and_allocator(1);
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 5.0,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        let prev = [5.0f32];
        let curr = [5.0f32];
        let deltas = cpu_oracle_band_crossing_deltas(
            &prev,
            &curr,
            &[],
            &[],
            1,
            &regs,
            &registry,
            &allocator,
        );
        assert!(deltas.is_empty());
    }

    #[test]
    fn multi_edge_jump_emits_ordered_deltas() {
        let (registry, allocator) = anchored_registry_and_allocator(1);
        let regs = [
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 1.0,
                direction: DIR_UPWARD,
                event_kind: 10,
                buffer: THRESH_BUF_VALUES,
            },
            ThresholdRegistration {
                slot: 0,
                col: 0,
                threshold: 2.0,
                direction: DIR_UPWARD,
                event_kind: 11,
                buffer: THRESH_BUF_VALUES,
            },
        ];
        let prev = [0.5f32];
        let curr = [2.5f32];
        let deltas = cpu_oracle_band_crossing_deltas(
            &prev,
            &curr,
            &[],
            &[],
            1,
            &regs,
            &registry,
            &allocator,
        );
        assert_eq!(deltas.len(), 2);
        assert_eq!(deltas[0].reg_idx(), 0);
        assert_eq!(deltas[1].reg_idx(), 1);
        assert_eq!(deltas[0].event_kind(), 10);
        assert_eq!(deltas[1].event_kind(), 11);
    }

    #[test]
    fn unobserved_columns_are_filtered_without_caller_allowlist() {
        let mut registry = DimensionRegistry::new();
        let mut unobserved = SimProperty::simple("test", "dark", 1);
        unobserved.admission_disposition = PropertyAdmissionDisposition::Unobserved {
            reason: "fixture".into(),
            source_span_token: 0,
        };
        let _ = registry.register(unobserved);
        let root = SimThing::new(SimThingKind::GameSession, 0);
        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);

        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        let prev = [0.0f32];
        let curr = [2.0f32];
        let deltas = cpu_oracle_band_crossing_deltas(
            &prev,
            &curr,
            &[],
            &[],
            1,
            &regs,
            &registry,
            &allocator,
        );
        assert!(deltas.is_empty());
    }
}
