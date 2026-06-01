//! MOBILITY-REENROLL-0: bilateral arena re-enrollment substrate.
//!
//! Spatial movement as a bilateral arena operation built on MOBILITY-ALLOC-0.
//! This is a named, metadata/testable substrate only. It does not implement
//! IDROUTE, ECON, OWNER, routing/economy/owner-overlay runtime, GPU kernels,
//! default-on behavior, or production `SimSession` wiring.

use std::collections::{BTreeMap, BTreeSet};

use super::mobility_alloc0::{
    mobility_alloc0_layout_checksum_cpu, mobility_alloc0_layout_checksum_gpu_proxy,
    plan_mobility_alloc0, MobilityAlloc0BlockSpec, MobilityAlloc0BoundaryEvent,
    MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice,
    MobilityAlloc0ParentKey, MobilityAlloc0PlanInput, MOBILITY_ALLOC0_ID,
};

pub const MOBILITY_REENROLL0_ID: &str = "mobility_reenroll0_bilateral_arena_reenrollment";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityReenroll0Move {
    pub entity_id: u64,
    pub origin: MobilityAlloc0ParentKey,
    pub destination: MobilityAlloc0ParentKey,
    pub arrival_order: u64,
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
    pub blocks: Vec<MobilityAlloc0BlockSpec>,
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
    pub alloc_substrate_id: &'static str,
    pub alloc_assignments: Vec<super::mobility_alloc0::MobilityAlloc0Assignment>,
    pub touched_block_count: u32,
    pub boundary_event_count: u32,
    pub bulk_accounting_group_count: u32,
    pub peak_live_slots: u32,
    pub peak_pending_buffer_entries: u32,
    pub arrival_order_used_for_assignment: bool,
    pub runtime_implementation_authorized: bool,
}

pub fn plan_mobility_reenroll0(input: &MobilityReenroll0PlanInput) -> MobilityReenroll0PlanReport {
    let snapshot = input.registry.clone();
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);

    let canonical_moves = canonicalize_moves(&input.moves);
    let blocks_by_key = block_index(&snapshot.blocks, &mut diagnostics);
    let live_by_key = live_index(&snapshot.live_slices, &blocks_by_key, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(&snapshot, diagnostics);
    }

    let mut departures: BTreeMap<MobilityAlloc0ParentKey, BTreeSet<u64>> = BTreeMap::new();
    let mut arrivals: BTreeMap<MobilityAlloc0ParentKey, BTreeSet<u64>> = BTreeMap::new();
    let mut move_entities = BTreeSet::new();

    for mv in &canonical_moves {
        if mv.origin == mv.destination {
            diagnostics.push("spatial transfer requires distinct origin and destination");
            continue;
        }
        if mv.origin.parent_id != mv.destination.parent_id {
            diagnostics.push("flat-star cell arenas reject nested parent/key reparenting");
            continue;
        }
        if !blocks_by_key.contains_key(&mv.origin) {
            diagnostics.push("origin parent/key block is undeclared");
            continue;
        }
        if !blocks_by_key.contains_key(&mv.destination) {
            diagnostics.push("destination parent/key block is undeclared");
            continue;
        }
        if !move_entities.insert(mv.entity_id) {
            diagnostics.push("duplicate entity in move batch");
            continue;
        }

        let origin_live = live_by_key.get(&mv.origin);
        if !origin_live.is_some_and(|live| live.contains_key(&mv.entity_id)) {
            diagnostics.push("origin live slice missing for entity");
            continue;
        }

        let also_departing_dest = departures
            .get(&mv.destination)
            .is_some_and(|deps| deps.contains(&mv.entity_id));
        if live_by_key
            .get(&mv.destination)
            .is_some_and(|live| live.contains_key(&mv.entity_id))
            && !also_departing_dest
        {
            diagnostics.push("entity already live at destination parent/key block");
            continue;
        }

        departures
            .entry(mv.origin)
            .or_default()
            .insert(mv.entity_id);
        arrivals
            .entry(mv.destination)
            .or_default()
            .insert(mv.entity_id);
    }

    let peak_pending_buffer_entries = departures.values().map(|s| s.len()).sum::<usize>() as u32
        + arrivals.values().map(|s| s.len()).sum::<usize>() as u32;

    for (key, block) in &blocks_by_key {
        let live_count = live_by_key.get(key).map(|live| live.len()).unwrap_or(0) as u32;
        let dep_count = departures.get(key).map(|set| set.len()).unwrap_or(0) as u32;
        let arr_count = arrivals.get(key).map(|set| set.len()).unwrap_or(0) as u32;
        let after_departures = live_count.saturating_sub(dep_count);
        if after_departures + arr_count > block.slot_count {
            diagnostics.push("destination parent/key block capacity exceeded");
        }
    }

    if !diagnostics.is_empty() {
        return rejected_report(&snapshot, diagnostics);
    }

    let events = build_boundary_events(&canonical_moves);
    let alloc_input = MobilityAlloc0PlanInput {
        blocks: snapshot.blocks.clone(),
        live_slices: snapshot.live_slices.clone(),
        events,
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    };
    let alloc_report = plan_mobility_alloc0(&alloc_input);

    if !alloc_report.admitted {
        let mut merged = diagnostics;
        merged.extend(alloc_report.diagnostics);
        return rejected_report(&snapshot, merged);
    }

    let mut origin_generations = snapshot.origin_generations.clone();
    let mut destination_generations = snapshot.destination_generations.clone();
    let mut touched_blocks = BTreeSet::new();

    let mut committed_moves = Vec::with_capacity(canonical_moves.len());
    for mv in &canonical_moves {
        let assignment = alloc_report
            .assignments
            .iter()
            .find(|a| a.entity_id == mv.entity_id && a.parent_key == mv.destination)
            .expect("alloc assignment for committed move");
        committed_moves.push(MobilityReenroll0CommittedMove {
            entity_id: mv.entity_id,
            origin: mv.origin,
            destination: mv.destination,
            destination_slot: assignment.slot,
        });
        touched_blocks.insert(mv.origin);
        touched_blocks.insert(mv.destination);
    }
    committed_moves.sort_by_key(|mv| (mv.entity_id, mv.origin, mv.destination));

    for key in touched_blocks {
        *origin_generations.entry(key).or_insert(0) += 1;
        *destination_generations.entry(key).or_insert(0) += 1;
    }

    MobilityReenroll0PlanReport {
        substrate_id: MOBILITY_REENROLL0_ID,
        admitted: true,
        diagnostics,
        committed_moves,
        final_live_slices: alloc_report.final_live_slices.clone(),
        origin_generations,
        destination_generations,
        alloc_substrate_id: MOBILITY_ALLOC0_ID,
        alloc_assignments: alloc_report.assignments.clone(),
        touched_block_count: alloc_report.touched_block_count,
        boundary_event_count: alloc_report.boundary_event_count,
        bulk_accounting_group_count: alloc_report.bulk_accounting_group_count,
        peak_live_slots: alloc_report.peak_live_slots,
        peak_pending_buffer_entries,
        arrival_order_used_for_assignment: false,
        runtime_implementation_authorized: false,
    }
}

pub fn mobility_reenroll0_layout_checksum_cpu(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_alloc0_layout_checksum_cpu(slices)
}

pub fn mobility_reenroll0_layout_checksum_gpu_proxy(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_alloc0_layout_checksum_gpu_proxy(slices)
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
        alloc_substrate_id: MOBILITY_ALLOC0_ID,
        alloc_assignments: Vec::new(),
        touched_block_count: 0,
        boundary_event_count: 0,
        bulk_accounting_group_count: 0,
        peak_live_slots: snapshot.live_slices.len() as u32,
        peak_pending_buffer_entries: 0,
        arrival_order_used_for_assignment: false,
        runtime_implementation_authorized: false,
    }
}

fn canonicalize_moves(moves: &[MobilityReenroll0Move]) -> Vec<MobilityReenroll0Move> {
    let mut ordered = moves.to_vec();
    ordered.sort_by_key(|mv| {
        (
            mv.entity_id,
            mv.origin.parent_id,
            mv.origin.key_id,
            mv.destination.parent_id,
            mv.destination.key_id,
        )
    });
    ordered
}

fn build_boundary_events(moves: &[MobilityReenroll0Move]) -> Vec<MobilityAlloc0BoundaryEvent> {
    let mut events = Vec::with_capacity(moves.len() * 2);
    for mv in moves {
        events.push(MobilityAlloc0BoundaryEvent {
            kind: MobilityAlloc0BoundaryEventKind::Departure,
            parent_key: mv.origin,
            entity_id: Some(mv.entity_id),
            arrival_order: mv.arrival_order,
        });
        events.push(MobilityAlloc0BoundaryEvent {
            kind: MobilityAlloc0BoundaryEventKind::Arrival,
            parent_key: mv.destination,
            entity_id: Some(mv.entity_id),
            arrival_order: mv.arrival_order,
        });
    }
    events
}

fn validate_forbidden(
    forbidden: &MobilityReenroll0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture-as-reparenting is rejected");
    }
    if forbidden.owner_as_spatial_parent {
        diagnostics.push("owner-entity as spatial parent is rejected");
    }
    if forbidden.nested_arena_reparenting {
        diagnostics.push("nested arena reparenting requires a separate gate");
    }
    if forbidden.idroute_econ_owner {
        diagnostics.push("IDROUTE/ECON/OWNER remains parked");
    }
    if forbidden.production_simsession_wiring {
        diagnostics.push("production SimSession wiring is not authorized");
    }
    if forbidden.default_on_behavior {
        diagnostics.push("default-on behavior is not authorized");
    }
}

fn block_index(
    blocks: &[MobilityAlloc0BlockSpec],
    diagnostics: &mut Vec<&'static str>,
) -> BTreeMap<MobilityAlloc0ParentKey, MobilityAlloc0BlockSpec> {
    let mut by_key = BTreeMap::new();
    for block in blocks {
        if by_key.insert(block.parent_key, block.clone()).is_some() {
            diagnostics.push("duplicate parent/key block");
        }
    }
    by_key
}

fn live_index(
    live_slices: &[MobilityAlloc0LiveSlice],
    blocks_by_key: &BTreeMap<MobilityAlloc0ParentKey, MobilityAlloc0BlockSpec>,
    diagnostics: &mut Vec<&'static str>,
) -> BTreeMap<MobilityAlloc0ParentKey, BTreeMap<u64, u32>> {
    let mut by_key: BTreeMap<MobilityAlloc0ParentKey, BTreeMap<u64, u32>> = BTreeMap::new();
    for slice in live_slices {
        if !blocks_by_key.contains_key(&slice.parent_key) {
            diagnostics.push("live slice references an undeclared parent/key block");
            continue;
        }
        if by_key
            .entry(slice.parent_key)
            .or_default()
            .insert(slice.entity_id, slice.slot)
            .is_some()
        {
            diagnostics.push("duplicate live entity in parent/key block");
        }
    }
    by_key
}
