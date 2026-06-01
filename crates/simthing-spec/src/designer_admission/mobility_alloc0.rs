//! MOBILITY-ALLOC-0: deterministic slab + bulk-accounting allocator substrate.
//!
//! This is a named, metadata/testable substrate only. It does not implement
//! re-enrollment, routing, economy, owner overlays, GPU kernels, default-on
//! behavior, or production `SimSession` wiring.

use std::collections::{BTreeMap, BTreeSet};

pub const MOBILITY_ALLOC0_ID: &str = "mobility_alloc0_deterministic_slab_allocator";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MobilityAlloc0ParentKey {
    pub parent_id: u64,
    pub key_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0BlockSpec {
    pub parent_key: MobilityAlloc0ParentKey,
    pub start_slot: u32,
    pub slot_count: u32,
    pub reserved_headroom: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0LiveSlice {
    pub entity_id: u64,
    pub parent_key: MobilityAlloc0ParentKey,
    pub slot: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobilityAlloc0BoundaryEventKind {
    Arrival,
    Departure,
    ParentRemoved,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0BoundaryEvent {
    pub kind: MobilityAlloc0BoundaryEventKind,
    pub parent_key: MobilityAlloc0ParentKey,
    pub entity_id: Option<u64>,
    pub arrival_order: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MobilityAlloc0ForbiddenPathRequests {
    pub live_compaction: bool,
    pub arrival_order_replay_significance: bool,
    pub gpu_semaphore_or_atomic_path: bool,
    pub indirection_list_slotrange: bool,
    pub reenroll_idroute_econ_owner: bool,
    pub production_simsession_wiring: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0PlanInput {
    pub blocks: Vec<MobilityAlloc0BlockSpec>,
    pub live_slices: Vec<MobilityAlloc0LiveSlice>,
    pub events: Vec<MobilityAlloc0BoundaryEvent>,
    pub forbidden: MobilityAlloc0ForbiddenPathRequests,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0Assignment {
    pub entity_id: u64,
    pub parent_key: MobilityAlloc0ParentKey,
    pub slot: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAlloc0PlanReport {
    pub substrate_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub assignments: Vec<MobilityAlloc0Assignment>,
    pub final_live_slices: Vec<MobilityAlloc0LiveSlice>,
    pub reclaimed_blocks: Vec<MobilityAlloc0ParentKey>,
    pub touched_block_count: u32,
    pub boundary_event_count: u32,
    pub bulk_accounting_group_count: u32,
    pub peak_live_slots: u32,
    pub total_declared_slots: u32,
    pub wasted_slots: u32,
    pub arrival_order_used_for_assignment: bool,
    pub runtime_implementation_authorized: bool,
}

#[derive(Clone, Debug, Default)]
struct GroupedEvents {
    arrivals: BTreeSet<u64>,
    departures: BTreeSet<u64>,
    parent_removed: bool,
}

pub fn plan_mobility_alloc0(input: &MobilityAlloc0PlanInput) -> MobilityAlloc0PlanReport {
    let mut diagnostics = Vec::new();
    validate_forbidden(&input.forbidden, &mut diagnostics);

    let blocks_by_key = block_index(&input.blocks, &mut diagnostics);
    let total_declared_slots = input
        .blocks
        .iter()
        .map(|block| block.slot_count)
        .sum::<u32>();
    let mut live_by_key = live_index(&input.live_slices, &blocks_by_key, &mut diagnostics);
    let grouped_events = group_events(&input.events, &blocks_by_key, &mut diagnostics);

    let mut assignments = Vec::new();
    let mut reclaimed_blocks = Vec::new();
    let mut peak_live_slots = live_by_key
        .values()
        .map(|entries| entries.len() as u32)
        .sum::<u32>();

    if diagnostics.is_empty() {
        for (key, group) in &grouped_events {
            let Some(block) = blocks_by_key.get(key) else {
                continue;
            };
            let live = live_by_key.entry(*key).or_default();

            for entity_id in &group.departures {
                live.remove(entity_id);
            }

            if group.parent_removed {
                if !live.is_empty() || !group.arrivals.is_empty() {
                    diagnostics.push("parent removal requires no live slices and no arrivals");
                    continue;
                }
                reclaimed_blocks.push(*key);
                continue;
            }

            let mut occupied = live.values().copied().collect::<BTreeSet<_>>();
            let mut free_slots = (block.start_slot..block.start_slot + block.slot_count)
                .filter(|slot| !occupied.contains(slot))
                .collect::<BTreeSet<_>>();

            for entity_id in &group.arrivals {
                if live.contains_key(entity_id) {
                    diagnostics.push("arrival entity already live in parent/key block");
                    continue;
                }
                let Some(slot) = free_slots.pop_first() else {
                    diagnostics.push("parent/key block capacity exceeded");
                    continue;
                };
                occupied.insert(slot);
                live.insert(*entity_id, slot);
                assignments.push(MobilityAlloc0Assignment {
                    entity_id: *entity_id,
                    parent_key: *key,
                    slot,
                });
            }

            peak_live_slots =
                peak_live_slots.max(live_by_key.values().map(|m| m.len() as u32).sum());
        }
    }

    let mut final_live_slices = live_by_key
        .iter()
        .flat_map(|(parent_key, live)| {
            live.iter()
                .map(|(entity_id, slot)| MobilityAlloc0LiveSlice {
                    entity_id: *entity_id,
                    parent_key: *parent_key,
                    slot: *slot,
                })
        })
        .collect::<Vec<_>>();
    final_live_slices.sort_by_key(|slice| (slice.parent_key, slice.slot, slice.entity_id));
    assignments
        .sort_by_key(|assignment| (assignment.parent_key, assignment.slot, assignment.entity_id));
    reclaimed_blocks.sort();

    let final_live_count = final_live_slices.len() as u32;
    let wasted_slots = total_declared_slots.saturating_sub(final_live_count);

    MobilityAlloc0PlanReport {
        substrate_id: MOBILITY_ALLOC0_ID,
        admitted: diagnostics.is_empty(),
        diagnostics,
        assignments,
        final_live_slices,
        reclaimed_blocks,
        touched_block_count: grouped_events.len() as u32,
        boundary_event_count: input.events.len() as u32,
        bulk_accounting_group_count: grouped_events.len() as u32,
        peak_live_slots,
        total_declared_slots,
        wasted_slots,
        arrival_order_used_for_assignment: false,
        runtime_implementation_authorized: false,
    }
}

pub fn mobility_alloc0_layout_checksum_cpu(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_alloc0_layout_checksum(slices)
}

pub fn mobility_alloc0_layout_checksum_gpu_proxy(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
    mobility_alloc0_layout_checksum(slices)
}

fn mobility_alloc0_layout_checksum(slices: &[MobilityAlloc0LiveSlice]) -> u64 {
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
    forbidden: &MobilityAlloc0ForbiddenPathRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.live_compaction {
        diagnostics.push("live compaction is rejected");
    }
    if forbidden.arrival_order_replay_significance {
        diagnostics.push("arrival-order replay-significant assignment is rejected");
    }
    if forbidden.gpu_semaphore_or_atomic_path {
        diagnostics.push("GPU semaphore or nondeterministic atomic allocator path is rejected");
    }
    if forbidden.indirection_list_slotrange {
        diagnostics.push("indirection-list SlotRange is rejected");
    }
    if forbidden.reenroll_idroute_econ_owner {
        diagnostics.push("REENROLL/IDROUTE/ECON/OWNER remains parked");
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
    let mut occupied_ranges = BTreeSet::new();
    for block in blocks {
        if block.slot_count == 0 {
            diagnostics.push("parent/key block must have non-zero capacity");
        }
        if block.reserved_headroom > block.slot_count {
            diagnostics.push("reserved headroom may not exceed block capacity");
        }
        if by_key.insert(block.parent_key, block.clone()).is_some() {
            diagnostics.push("duplicate parent/key block");
        }
        for slot in block.start_slot..block.start_slot + block.slot_count {
            if !occupied_ranges.insert(slot) {
                diagnostics.push("overlapping parent/key blocks are rejected");
                break;
            }
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
    let mut occupied = BTreeSet::new();
    for slice in live_slices {
        let Some(block) = blocks_by_key.get(&slice.parent_key) else {
            diagnostics.push("live slice references an undeclared parent/key block");
            continue;
        };
        if slice.slot < block.start_slot || slice.slot >= block.start_slot + block.slot_count {
            diagnostics.push("live slice falls outside its parent/key block");
            continue;
        }
        if !occupied.insert(slice.slot) {
            diagnostics.push("duplicate live slot");
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

fn group_events(
    events: &[MobilityAlloc0BoundaryEvent],
    blocks_by_key: &BTreeMap<MobilityAlloc0ParentKey, MobilityAlloc0BlockSpec>,
    diagnostics: &mut Vec<&'static str>,
) -> BTreeMap<MobilityAlloc0ParentKey, GroupedEvents> {
    let mut grouped = BTreeMap::new();
    for event in events {
        if !blocks_by_key.contains_key(&event.parent_key) {
            diagnostics.push("boundary event references an undeclared parent/key block");
            continue;
        }
        let group: &mut GroupedEvents = grouped.entry(event.parent_key).or_default();
        match event.kind {
            MobilityAlloc0BoundaryEventKind::Arrival => match event.entity_id {
                Some(entity_id) => {
                    group.arrivals.insert(entity_id);
                }
                None => diagnostics.push("arrival event requires entity id"),
            },
            MobilityAlloc0BoundaryEventKind::Departure => match event.entity_id {
                Some(entity_id) => {
                    group.departures.insert(entity_id);
                }
                None => diagnostics.push("departure event requires entity id"),
            },
            MobilityAlloc0BoundaryEventKind::ParentRemoved => {
                group.parent_removed = true;
            }
        }
    }
    grouped
}
