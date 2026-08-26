//! MOBILITY-REENROLL-0: slot-preserving structural re-enrollment witness.
//!
//! Re-enrollment moves an already-resident object between structural parents
//! without allocating residency.  The object's stable logical slot is carried
//! through unchanged; no free-list, capacity contest, arrival order, or
//! caller-proposed destination row is represented by the input type.
//!
//! ```compile_fail,E0560
//! use simthing_spec::{
//!     MobilityAlloc0ParentKey, MobilityReenroll0Move,
//! };
//!
//! let root = MobilityAlloc0ParentKey { parent_id: 1, key_id: 0 };
//! let _ = MobilityReenroll0Move {
//!     entity_id: 7,
//!     origin: root,
//!     destination: MobilityAlloc0ParentKey { parent_id: 1, key_id: 1 },
//!     destination_slot: 0,
//!     arrival_order: 0,
//! };
//! ```

use std::collections::{BTreeMap, BTreeSet};

use super::mobility_alloc0::{MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey};

pub const MOBILITY_REENROLL0_ID: &str = "mobility_reenroll0_slot_preserving_reparent";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0Move {
    pub entity_id: u64,
    pub origin: MobilityAlloc0ParentKey,
    pub destination: MobilityAlloc0ParentKey,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityReenroll0ForbiddenPathRequests {
    pub capture_as_reparenting: bool,
    pub owner_as_spatial_parent: bool,
    pub nested_arena_reparenting: bool,
    pub idroute_econ_owner: bool,
    pub production_simsession_wiring: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0RegistryState {
    pub live_slices: Vec<MobilityAlloc0LiveSlice>,
    pub origin_generations: BTreeMap<MobilityAlloc0ParentKey, u64>,
    pub destination_generations: BTreeMap<MobilityAlloc0ParentKey, u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0PlanInput {
    pub registry: MobilityReenroll0RegistryState,
    pub moves: Vec<MobilityReenroll0Move>,
    pub forbidden: MobilityReenroll0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0CommittedMove {
    pub entity_id: u64,
    pub origin: MobilityAlloc0ParentKey,
    pub destination: MobilityAlloc0ParentKey,
    /// The pre-existing logical slot; structural movement never selects a new one.
    pub destination_slot: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0PlanReport {
    pub substrate_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub committed_moves: Vec<MobilityReenroll0CommittedMove>,
    pub final_live_slices: Vec<MobilityAlloc0LiveSlice>,
    pub origin_generations: BTreeMap<MobilityAlloc0ParentKey, u64>,
    pub destination_generations: BTreeMap<MobilityAlloc0ParentKey, u64>,
    pub touched_parent_count: u32,
    pub peak_pending_buffer_entries: u32,
    pub runtime_implementation_authorized: bool,
}

pub fn plan_mobility_reenroll0(input: &MobilityReenroll0PlanInput) -> MobilityReenroll0PlanReport {
    let snapshot = input.registry.clone();
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);

    let canonical_moves = canonicalize_moves(&input.moves);
    let live_by_entity = live_index(&snapshot.live_slices, &mut diagnostics);
    let mut move_entities = BTreeSet::new();

    for movement in &canonical_moves {
        if movement.origin == movement.destination {
            diagnostics.push("spatial transfer requires distinct origin and destination");
            continue;
        }
        if movement.origin.parent_id != movement.destination.parent_id {
            diagnostics.push("flat-star cell arenas reject nested parent/key reparenting");
            continue;
        }
        if !move_entities.insert(movement.entity_id) {
            diagnostics.push("duplicate entity in move batch");
            continue;
        }
        match live_by_entity.get(&movement.entity_id) {
            Some((parent_key, _)) if *parent_key == movement.origin => {}
            Some(_) => diagnostics.push("origin live slice does not match movement origin"),
            None => diagnostics.push("origin live slice missing for entity"),
        }
    }

    if !diagnostics.is_empty() {
        return rejected_report(&snapshot, diagnostics);
    }

    let mut final_live_slices = snapshot.live_slices.clone();
    let mut committed_moves = Vec::with_capacity(canonical_moves.len());
    let mut touched_parents = BTreeSet::new();
    for movement in &canonical_moves {
        let slice = final_live_slices
            .iter_mut()
            .find(|slice| slice.entity_id == movement.entity_id)
            .expect("validated resident movement entity");
        let stable_slot = slice.slot;
        slice.parent_key = movement.destination;
        committed_moves.push(MobilityReenroll0CommittedMove {
            entity_id: movement.entity_id,
            origin: movement.origin,
            destination: movement.destination,
            destination_slot: stable_slot,
        });
        touched_parents.insert(movement.origin);
        touched_parents.insert(movement.destination);
    }

    final_live_slices.sort_by_key(|slice| (slice.parent_key, slice.slot, slice.entity_id));
    committed_moves
        .sort_by_key(|movement| (movement.entity_id, movement.origin, movement.destination));

    let mut origin_generations = snapshot.origin_generations.clone();
    let mut destination_generations = snapshot.destination_generations.clone();
    for parent in &touched_parents {
        *origin_generations.entry(*parent).or_insert(0) += 1;
        *destination_generations.entry(*parent).or_insert(0) += 1;
    }

    MobilityReenroll0PlanReport {
        substrate_id: MOBILITY_REENROLL0_ID,
        admitted: true,
        diagnostics,
        committed_moves,
        final_live_slices,
        origin_generations,
        destination_generations,
        touched_parent_count: touched_parents.len() as u32,
        peak_pending_buffer_entries: canonical_moves.len().saturating_mul(2) as u32,
        runtime_implementation_authorized: false,
    }
}

pub fn mobility_reenroll0_layout_checksum_cpu(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_reenroll0_layout_checksum(slices)
}

pub fn mobility_reenroll0_layout_checksum_gpu_proxy(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_reenroll0_layout_checksum(slices)
}

fn mobility_reenroll0_layout_checksum(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    let mut ordered = slices.to_vec();
    ordered.sort_by_key(|slice| (slice.parent_key, slice.slot, slice.entity_id));
    ordered.iter().fold(0xcbf2_9ce4_8422_2325, |hash, slice| {
        let hash = fnv_append_u64(hash, slice.parent_key.parent_id);
        let hash = fnv_append_u64(hash, slice.parent_key.key_id);
        let hash = fnv_append_u64(hash, slice.entity_id);
        fnv_append_u64(hash, slice.slot as u64)
    })
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn validate_forbidden(
    forbidden: &MobilityReenroll0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture as reparenting is rejected");
    }
    if forbidden.owner_as_spatial_parent {
        diagnostics.push("owner as spatial parent is rejected");
    }
    if forbidden.nested_arena_reparenting {
        diagnostics.push("nested arena reparenting is rejected");
    }
    if forbidden.idroute_econ_owner {
        diagnostics.push("IDROUTE/ECON/OWNER composition is not part of re-enrollment");
    }
    if forbidden.production_simsession_wiring {
        diagnostics.push("production SimSession wiring is not authorized");
    }
    if forbidden.default_on_behavior {
        diagnostics.push("default-on behavior is not authorized");
    }
}

fn canonicalize_moves(moves: &[MobilityReenroll0Move]) -> Vec<MobilityReenroll0Move> {
    let mut canonical = moves.to_vec();
    canonical.sort_by_key(|movement| (movement.entity_id, movement.origin, movement.destination));
    canonical
}

fn live_index(
    live_slices: &[MobilityAlloc0LiveSlice],
    diagnostics: &mut Vec<&'static str>,
) -> BTreeMap<u64, (MobilityAlloc0ParentKey, u32)> {
    let mut by_entity = BTreeMap::new();
    let mut occupied = BTreeSet::new();
    for slice in live_slices {
        if !occupied.insert(slice.slot) {
            diagnostics.push("duplicate live logical slot");
        }
        if by_entity
            .insert(slice.entity_id, (slice.parent_key, slice.slot))
            .is_some()
        {
            diagnostics.push("duplicate live entity");
        }
    }
    by_entity
}

fn rejected_report(
    snapshot: &MobilityReenroll0RegistryState,
    diagnostics: Vec<&'static str>,
) -> MobilityReenroll0PlanReport {
    MobilityReenroll0PlanReport {
        substrate_id: MOBILITY_REENROLL0_ID,
        admitted: false,
        diagnostics,
        committed_moves: Vec::new(),
        final_live_slices: snapshot.live_slices.clone(),
        origin_generations: snapshot.origin_generations.clone(),
        destination_generations: snapshot.destination_generations.clone(),
        touched_parent_count: 0,
        peak_pending_buffer_entries: 0,
        runtime_implementation_authorized: false,
    }
}
