//! E-10R2/E-10R3 — ArenaParticipant SimThing scaffold (driver/session topology only).
//!
//! Materializes dedicated arena-participant wrapper nodes for admitted explicit
//! participants, proves sibling contiguity for SlotRange reductions, and tracks
//! arena-local reserved-gap blocks for fission-spawned participant children.

use simthing_core::{
    DimensionRegistry, PropertyLayout, PropertyValue, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SubFieldRole,
};
use simthing_gpu::{SlotAllocError, SlotAllocator};
use simthing_spec::{PropertyKey, ResourceFlowSpec, SpecError};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

use crate::arena_registry::{ArenaIdx, ArenaRegistry, ArenaRegistryError, FissionPolicy, SlotId};
use crate::install::InstallError;

/// `(hosted SimThing id, arena idx) → arena-participant slot`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArenaParticipantIndex {
    pub by_host_and_arena: HashMap<(SimThingId, ArenaIdx), SlotId>,
}

impl ArenaParticipantIndex {
    pub fn participant_slot(&self, hosted: SimThingId, arena_idx: ArenaIdx) -> Option<SlotId> {
        self.by_host_and_arena.get(&(hosted, arena_idx)).copied()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArenaParticipantAllocationReport {
    pub arena: String,
    pub root_slot: SlotId,
    pub participant_count: u32,
    pub reserved_gap_per_intermediate: u32,
    pub max_children_per_intermediate: u32,
    /// First slot of the contiguous participant sibling block under the arena root.
    pub participant_sibling_first: Option<SlotId>,
    /// First slot of the arena-local reserved gap block (if any).
    pub gap_block_first: Option<SlotId>,
}

/// LIFO pool of exclusively reserved tombstoned slots in the arena-local gap block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReservedGapPool {
    pub parent_participant_slot: SlotId,
    /// Ascending reservation order; consumed LIFO (pop from end).
    available: Vec<SlotId>,
}

impl ReservedGapPool {
    pub fn remaining(&self) -> u32 {
        self.available.len() as u32
    }

    pub fn reserved_slots(&self) -> &[SlotId] {
        &self.available
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArenaParticipantScaffold {
    pub index: ArenaParticipantIndex,
    pub reports: Vec<ArenaParticipantAllocationReport>,
    /// Parent participant slot → reserved gap pool.
    pub gap_pools: HashMap<SlotId, ReservedGapPool>,
    /// Arena root SimThing id per arena index.
    pub arena_root_ids: HashMap<ArenaIdx, SimThingId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum GapAllocError {
    #[error("reserved gap exhausted for parent participant slot {parent_slot}")]
    Exhausted { parent_slot: SlotId },
    #[error("slot allocator error: {0}")]
    Slot(#[from] SlotAllocError),
}

/// Materialize arena roots and dedicated participant SimThings for every admitted
/// explicit participant. Hosted SimThings are not replaced or re-slotted.
pub fn materialize_arena_participants(
    spec: &ResourceFlowSpec,
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<ArenaParticipantScaffold, InstallError> {
    let mut scaffold = ArenaParticipantScaffold::default();

    for (arena_idx, arena) in spec.arenas.iter().enumerate() {
        let arena_idx = arena_idx as ArenaIdx;
        let flow_property_id = resolve_flow_property(registry, &arena.flow_property)?;
        validate_explicit_participant_graph(arena)?;

        let mut arena_root = SimThing::new(
            SimThingKind::Custom(format!("arena_root:{}", arena.name)),
            0,
        );
        let arena_root_id = arena_root.id;

        let nested = arena
            .explicit_participants
            .iter()
            .any(|participant| participant.parent_subtree_root_id.is_some());

        let mut nodes_by_hosted: HashMap<u32, SimThing> = HashMap::new();
        let mut participant_node_by_hosted: HashMap<u32, SimThingId> = HashMap::new();
        for participant in &arena.explicit_participants {
            let hosted_id = SimThingId::from_session_raw(participant.subtree_root_id);
            let mut node = SimThing::new(SimThingKind::ArenaParticipant, 0);
            seed_participant_property(&mut node, flow_property_id, registry, hosted_id);
            participant_node_by_hosted.insert(participant.subtree_root_id, node.id);
            nodes_by_hosted.insert(participant.subtree_root_id, node);
        }

        let mut children_by_parent: HashMap<Option<u32>, Vec<u32>> = HashMap::new();
        for participant in &arena.explicit_participants {
            let parent = participant
                .parent_subtree_root_id
                .map(|parent_id| parent_id as u32);
            children_by_parent
                .entry(parent)
                .or_default()
                .push(participant.subtree_root_id);
        }

        for participant in &arena.explicit_participants {
            if participant.parent_subtree_root_id.is_some() {
                continue;
            }
            attach_participant_subtree(
                participant.subtree_root_id,
                &mut arena_root,
                &mut nodes_by_hosted,
                &children_by_parent,
            );
        }

        root.add_child(arena_root);
        allocator.alloc(arena_root_id);
        let arena_root_slot = allocator
            .slot_of(arena_root_id)
            .expect("arena root just allocated");

        let arena_root_ref = find_child(root, arena_root_id).expect("arena root attached");
        let mut top_level_slots = Vec::new();
        for child in &arena_root_ref.children {
            if child.kind != SimThingKind::ArenaParticipant {
                continue;
            }
            materialize_participant_subtree(child, allocator, &arena.name)?;
            top_level_slots.push(allocator.slot_of(child.id).expect("participant allocated"));
        }

        for participant in &arena.explicit_participants {
            let hosted_id = SimThingId::from_session_raw(participant.subtree_root_id);
            let participant_id = *participant_node_by_hosted
                .get(&participant.subtree_root_id)
                .expect("participant node reserved");
            let participant_slot = allocator
                .slot_of(participant_id)
                .expect("explicit participant allocated");
            scaffold
                .index
                .by_host_and_arena
                .insert((hosted_id, arena_idx), participant_slot);
        }

        let gap_k = arena.reserved_gap_per_intermediate;
        let gap_block_first = if gap_k > 0 && !top_level_slots.is_empty() {
            if nested {
                let interior_slots = interior_participant_slots(arena_root_ref, allocator);
                reserve_gap_pools_for_parent_slots(&mut scaffold, allocator, &interior_slots, gap_k)
            } else {
                let total_gaps = gap_k.saturating_mul(top_level_slots.len() as u32);
                let gap_block = allocator.reserve_exclusive_gap_block(total_gaps);
                for (i, participant_slot) in top_level_slots.iter().enumerate() {
                    let start = (i as u32 * gap_k) as usize;
                    let end = start + gap_k as usize;
                    scaffold.gap_pools.insert(
                        *participant_slot,
                        ReservedGapPool {
                            parent_participant_slot: *participant_slot,
                            available: gap_block[start..end].to_vec(),
                        },
                    );
                }
                gap_block.first().copied()
            }
        } else {
            None
        };

        scaffold.arena_root_ids.insert(arena_idx, arena_root_id);
        scaffold.reports.push(ArenaParticipantAllocationReport {
            arena: arena.name.clone(),
            root_slot: arena_root_slot,
            participant_count: arena.explicit_participants.len() as u32,
            reserved_gap_per_intermediate: arena.reserved_gap_per_intermediate,
            max_children_per_intermediate: arena.expected_max_children_per_intermediate,
            participant_sibling_first: top_level_slots.first().copied(),
            gap_block_first,
        });
    }

    Ok(scaffold)
}

fn validate_explicit_participant_graph(
    arena: &simthing_spec::ArenaSpec,
) -> Result<(), InstallError> {
    let mut by_subtree_root: HashMap<u32, usize> = HashMap::new();
    for participant in &arena.explicit_participants {
        if by_subtree_root
            .insert(
                participant.subtree_root_id,
                participant.subtree_root_id as usize,
            )
            .is_some()
        {
            return Err(InstallError::Spec(
                SpecError::DuplicateEnrollmentHostedSimThing {
                    arena: arena.name.clone(),
                    subtree_root_id: participant.subtree_root_id,
                },
            ));
        }
    }

    for participant in &arena.explicit_participants {
        let Some(parent_id) = participant.parent_subtree_root_id else {
            continue;
        };
        if parent_id > u32::MAX as u64 {
            return Err(InstallError::Spec(
                SpecError::UnknownExplicitParticipantParent {
                    arena: arena.name.clone(),
                    parent_subtree_root_id: parent_id,
                },
            ));
        }
        let parent_u32 = parent_id as u32;
        if !by_subtree_root.contains_key(&parent_u32) {
            return Err(InstallError::Spec(
                SpecError::UnknownExplicitParticipantParent {
                    arena: arena.name.clone(),
                    parent_subtree_root_id: parent_id,
                },
            ));
        }
    }

    for participant in &arena.explicit_participants {
        let mut seen = HashSet::new();
        let mut current = Some(participant.subtree_root_id);
        while let Some(subtree_root_id) = current {
            if !seen.insert(subtree_root_id) {
                return Err(InstallError::Spec(
                    SpecError::ExplicitParticipantParentCycle {
                        arena: arena.name.clone(),
                        subtree_root_id: participant.subtree_root_id,
                    },
                ));
            }
            current = parent_subtree_root_id(arena, subtree_root_id);
        }
    }

    Ok(())
}

fn parent_subtree_root_id(arena: &simthing_spec::ArenaSpec, subtree_root_id: u32) -> Option<u32> {
    arena
        .explicit_participants
        .iter()
        .find(|participant| participant.subtree_root_id == subtree_root_id)
        .and_then(|participant| participant.parent_subtree_root_id)
        .map(|parent_id| parent_id as u32)
}

fn attach_participant_subtree(
    subtree_root_id: u32,
    parent_node: &mut SimThing,
    nodes_by_hosted: &mut HashMap<u32, SimThing>,
    children_by_parent: &HashMap<Option<u32>, Vec<u32>>,
) {
    let mut node = nodes_by_hosted
        .remove(&subtree_root_id)
        .expect("participant node reserved");
    if let Some(child_ids) = children_by_parent.get(&Some(subtree_root_id)) {
        for child_id in child_ids {
            attach_participant_subtree(*child_id, &mut node, nodes_by_hosted, children_by_parent);
        }
    }
    parent_node.add_child(node);
}

fn materialize_participant_subtree(
    node: &SimThing,
    allocator: &mut SlotAllocator,
    arena_name: &str,
) -> Result<(), InstallError> {
    if allocator.slot_of(node.id).is_none() {
        allocator.alloc(node.id);
    }

    let children: Vec<&SimThing> = node
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::ArenaParticipant)
        .collect();
    if children.is_empty() {
        return Ok(());
    }

    let mut child_slots = Vec::with_capacity(children.len());
    for child in &children {
        if allocator.slot_of(child.id).is_none() {
            allocator.alloc(child.id);
        }
        child_slots.push(
            allocator
                .slot_of(child.id)
                .expect("child participant allocated"),
        );
    }
    if !slots_are_contiguous(&child_slots) {
        return Err(InstallError::Spec(
            SpecError::ExplicitParticipantAllocationNonContiguous {
                arena: arena_name.to_string(),
            },
        ));
    }

    for child in children {
        materialize_participant_subtree(child, allocator, arena_name)?;
    }
    Ok(())
}

fn interior_participant_slots(arena_root: &SimThing, allocator: &SlotAllocator) -> Vec<SlotId> {
    let mut out = Vec::new();
    collect_interior_participant_slots(arena_root, allocator, &mut out);
    out.sort_unstable();
    out.dedup();
    out
}

fn collect_interior_participant_slots(
    node: &SimThing,
    allocator: &SlotAllocator,
    out: &mut Vec<SlotId>,
) {
    if node.kind != SimThingKind::ArenaParticipant {
        for child in &node.children {
            collect_interior_participant_slots(child, allocator, out);
        }
        return;
    }

    let participant_children: Vec<&SimThing> = node
        .children
        .iter()
        .filter(|child| child.kind == SimThingKind::ArenaParticipant)
        .collect();
    if !participant_children.is_empty() {
        out.push(
            allocator
                .slot_of(node.id)
                .expect("interior participant allocated"),
        );
    }
    for child in participant_children {
        collect_interior_participant_slots(child, allocator, out);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DynamicEnrollmentError {
    #[error("arena index {0} has no materialized root")]
    UnknownArena(ArenaIdx),
    #[error("arena `{arena}` has no participant siblings to extend")]
    EmptyParticipantBlock { arena: String },
    #[error("participant sibling block for arena `{arena}` is not contiguous")]
    NonContiguousSiblings { arena: String },
    #[error("child hosted SimThing `{child:?}` already enrolled in arena `{arena}`")]
    AlreadyEnrolled { arena: String, child: SimThingId },
    #[error("contiguous slot extension failed for arena `{arena}`: {source}")]
    ContiguityExtensionFailed {
        arena: String,
        #[source]
        source: SlotAllocError,
    },
    #[error("arena registry admission failed for arena `{arena}`: {source}")]
    RegistryAdmissionFailed {
        arena: String,
        #[source]
        source: ArenaRegistryError,
    },
}

/// Prepared arena-root sibling append — all preflight checks passed; commit applies mutations.
#[derive(Clone, Debug)]
pub struct PendingDynamicArenaRootParticipant {
    pub arena_idx: ArenaIdx,
    pub arena_name: String,
    pub child_hosted_id: SimThingId,
    pub arena_root_id: SimThingId,
    pub last_sibling_slot: SlotId,
    pub participant_slot: SlotId,
    pub participant_node: SimThing,
}

/// Preflight arena-root sibling append without mutating tree/scaffold/allocator/registry.
pub fn prepare_dynamic_arena_root_append(
    scaffold: &ArenaParticipantScaffold,
    root: &SimThing,
    arena_idx: ArenaIdx,
    arena_name: &str,
    child_hosted_id: SimThingId,
    flow_property_id: SimPropertyId,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    arena_registry: &ArenaRegistry,
) -> Result<PendingDynamicArenaRootParticipant, DynamicEnrollmentError> {
    if scaffold
        .index
        .participant_slot(child_hosted_id, arena_idx)
        .is_some()
    {
        return Err(DynamicEnrollmentError::AlreadyEnrolled {
            arena: arena_name.to_string(),
            child: child_hosted_id,
        });
    }

    let arena_root_id = *scaffold
        .arena_root_ids
        .get(&arena_idx)
        .ok_or(DynamicEnrollmentError::UnknownArena(arena_idx))?;

    let sibling_slots = arena_participant_sibling_slots(root, arena_root_id, allocator);
    if sibling_slots.is_empty() {
        return Err(DynamicEnrollmentError::EmptyParticipantBlock {
            arena: arena_name.to_string(),
        });
    }
    if !slots_are_contiguous(&sibling_slots) {
        return Err(DynamicEnrollmentError::NonContiguousSiblings {
            arena: arena_name.to_string(),
        });
    }

    arena_registry
        .can_admit_participant_runtime(arena_idx)
        .map_err(|source| DynamicEnrollmentError::RegistryAdmissionFailed {
            arena: arena_name.to_string(),
            source,
        })?;

    let last_sibling = *sibling_slots.last().expect("non-empty siblings");
    let participant_slot =
        allocator
            .can_alloc_contiguous_after(last_sibling)
            .map_err(|source| DynamicEnrollmentError::ContiguityExtensionFailed {
                arena: arena_name.to_string(),
                source,
            })?;

    let mut participant_node = SimThing::new(SimThingKind::ArenaParticipant, 0);
    seed_participant_property(
        &mut participant_node,
        flow_property_id,
        registry,
        child_hosted_id,
    );

    Ok(PendingDynamicArenaRootParticipant {
        arena_idx,
        arena_name: arena_name.to_string(),
        child_hosted_id,
        arena_root_id,
        last_sibling_slot: last_sibling,
        participant_slot,
        participant_node,
    })
}

/// Commit a prepared append: allocator → registry → tree → scaffold.
pub fn commit_dynamic_arena_root_append(
    pending: PendingDynamicArenaRootParticipant,
    scaffold: &mut ArenaParticipantScaffold,
    root: &mut SimThing,
    arena_registry: &mut ArenaRegistry,
    allocator: &mut SlotAllocator,
) -> Result<SlotId, DynamicEnrollmentError> {
    let participant_id = pending.participant_node.id;
    let allocated_slot = allocator
        .try_alloc_contiguous_after(pending.last_sibling_slot, participant_id)
        .map_err(|source| DynamicEnrollmentError::ContiguityExtensionFailed {
            arena: pending.arena_name.clone(),
            source,
        })?;
    debug_assert_eq!(allocated_slot, pending.participant_slot);

    if let Err(source) = arena_registry.admit_participant_runtime(
        pending.arena_idx,
        allocated_slot,
        pending.child_hosted_id,
    ) {
        let _ = allocator.tombstone(participant_id);
        return Err(DynamicEnrollmentError::RegistryAdmissionFailed {
            arena: pending.arena_name,
            source,
        });
    }

    let arena_root = find_child_mut(root, pending.arena_root_id).expect("arena root in tree");
    arena_root.add_child(pending.participant_node);

    scaffold
        .index
        .by_host_and_arena
        .insert((pending.child_hosted_id, pending.arena_idx), allocated_slot);

    if let Some(report) = scaffold.reports.get_mut(pending.arena_idx as usize) {
        report.participant_count = report.participant_count.saturating_add(1);
    }

    Ok(allocated_slot)
}

/// Append a fission child as a new arena-root sibling participant (Policy A flat-star path).
///
/// Does not consume E-10R3 gap pools. Rejects when `last_sibling + 1` is blocked.
pub fn try_append_arena_root_sibling_participant(
    scaffold: &mut ArenaParticipantScaffold,
    root: &mut SimThing,
    arena_idx: ArenaIdx,
    arena_name: &str,
    child_hosted_id: SimThingId,
    flow_property_id: SimPropertyId,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    arena_registry: &mut ArenaRegistry,
) -> Result<SlotId, DynamicEnrollmentError> {
    let pending = prepare_dynamic_arena_root_append(
        scaffold,
        root,
        arena_idx,
        arena_name,
        child_hosted_id,
        flow_property_id,
        registry,
        allocator,
        arena_registry,
    )?;
    commit_dynamic_arena_root_append(pending, scaffold, root, arena_registry, allocator)
}

/// Consume a reserved gap slot for a fission-spawned participant child, or reject
/// when the pool is exhausted and policy is `Reject`.
pub fn try_alloc_participant_child_in_gap(
    scaffold: &mut ArenaParticipantScaffold,
    parent_participant_slot: SlotId,
    child_id: SimThingId,
    allocator: &mut SlotAllocator,
    fission_policy: FissionPolicy,
) -> Result<SlotId, GapAllocError> {
    let pool =
        scaffold
            .gap_pools
            .get_mut(&parent_participant_slot)
            .ok_or(GapAllocError::Exhausted {
                parent_slot: parent_participant_slot,
            })?;
    if let Some(slot) = pool.available.pop() {
        allocator.claim_exclusive_slot(slot, child_id)?;
        return Ok(slot);
    }
    match fission_policy {
        FissionPolicy::Reject => Err(GapAllocError::Exhausted {
            parent_slot: parent_participant_slot,
        }),
        FissionPolicy::Inherit | FissionPolicy::Reevaluate => Ok(allocator.alloc(child_id)),
    }
}

/// Minimal E-11 fission refresh: claim gap slot and attach `ArenaParticipant` under parent.
pub fn refresh_fission_participant_child(
    scaffold: &mut ArenaParticipantScaffold,
    root: &mut SimThing,
    parent_participant_slot: SlotId,
    child_hosted_id: SimThingId,
    flow_property_id: SimPropertyId,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    fission_policy: FissionPolicy,
) -> Result<SlotId, GapAllocError> {
    let mut child_participant = SimThing::new(SimThingKind::ArenaParticipant, 0);
    let child_participant_id = child_participant.id;
    seed_participant_property(
        &mut child_participant,
        flow_property_id,
        registry,
        child_hosted_id,
    );
    let slot = try_alloc_participant_child_in_gap(
        scaffold,
        parent_participant_slot,
        child_participant_id,
        allocator,
        fission_policy,
    )?;
    let parent_id = allocator
        .owner_of(parent_participant_slot)
        .expect("parent slot");
    let parent = find_child_mut(root, parent_id).expect("parent in tree");
    parent.add_child(child_participant);
    Ok(slot)
}

fn find_child_mut(node: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_child_mut(child, id) {
            return Some(found);
        }
    }
    None
}

/// Return contiguous arena-participant child slots under an arena root (topology order).
pub fn arena_participant_sibling_slots(
    root: &SimThing,
    arena_root_id: SimThingId,
    allocator: &SlotAllocator,
) -> Vec<SlotId> {
    find_child(root, arena_root_id)
        .map(|arena_root| {
            arena_root
                .children
                .iter()
                .filter(|c| c.kind == SimThingKind::ArenaParticipant)
                .map(|c| allocator.slot_of(c.id).expect("participant has slot"))
                .collect()
        })
        .unwrap_or_default()
}

/// True when `slot` falls in the half-open participant sibling `[first, first + count)`.
pub fn slot_in_participant_sibling_range(first: SlotId, count: u32, slot: SlotId) -> bool {
    count > 0 && slot >= first && slot < first.saturating_add(count)
}

/// Return all exclusively reserved gap slots across every parent pool in `scaffold`.
pub fn all_reserved_gap_slots(scaffold: &ArenaParticipantScaffold) -> Vec<SlotId> {
    let mut out = Vec::new();
    for pool in scaffold.gap_pools.values() {
        out.extend_from_slice(pool.reserved_slots());
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Reserve E-10R3 gap pools for explicit interior participant slots (nested layouts).
pub fn reserve_gap_pools_for_parent_slots(
    scaffold: &mut ArenaParticipantScaffold,
    allocator: &mut SlotAllocator,
    parent_slots: &[SlotId],
    gap_per_parent: u32,
) -> Option<SlotId> {
    if gap_per_parent == 0 || parent_slots.is_empty() {
        return None;
    }
    let total = gap_per_parent.saturating_mul(parent_slots.len() as u32);
    let gap_block = allocator.reserve_exclusive_gap_block(total);
    for (i, parent_slot) in parent_slots.iter().enumerate() {
        let start = (i as u32 * gap_per_parent) as usize;
        let end = start + gap_per_parent as usize;
        scaffold.gap_pools.insert(
            *parent_slot,
            ReservedGapPool {
                parent_participant_slot: *parent_slot,
                available: gap_block[start..end].to_vec(),
            },
        );
    }
    gap_block.first().copied()
}

/// Diagnostic report for nested fission / reserved-gap preservation (driver/test-reporting).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NestedFissionGapReport {
    pub parent_participant_slot: SlotId,
    pub active_child_slots: Vec<SlotId>,
    pub reserved_gap_slots: Vec<SlotId>,
    pub gap_outside_active_child_span: bool,
    pub gap_outside_arena_sibling_range: bool,
    pub active_children_contiguous: bool,
}

/// Build a nested fission/gap preservation report for one interior parent.
pub fn nested_fission_gap_report(
    parent_participant_slot: SlotId,
    active_child_slots: &[SlotId],
    scaffold: &ArenaParticipantScaffold,
    arena_sibling_first: Option<SlotId>,
    arena_sibling_count: u32,
) -> NestedFissionGapReport {
    let reserved_gap_slots = scaffold
        .gap_pools
        .get(&parent_participant_slot)
        .map(|pool| pool.reserved_slots().to_vec())
        .unwrap_or_default();
    let gap_outside_active_child_span = if active_child_slots.is_empty() {
        !reserved_gap_slots.is_empty()
    } else {
        let first = active_child_slots[0];
        let count = active_child_slots.len() as u32;
        reserved_gap_slots
            .iter()
            .all(|slot| !slot_in_active_child_span(first, count, *slot))
    };
    let gap_outside_arena_sibling_range = reserved_gap_slots.iter().all(|slot| {
        arena_sibling_first
            .map(|first| !slot_in_participant_sibling_range(first, arena_sibling_count, *slot))
            .unwrap_or(true)
    });
    NestedFissionGapReport {
        parent_participant_slot,
        active_child_slots: active_child_slots.to_vec(),
        reserved_gap_slots,
        gap_outside_active_child_span,
        gap_outside_arena_sibling_range,
        active_children_contiguous: slots_are_contiguous(active_child_slots),
    }
}

fn slot_in_active_child_span(first: SlotId, count: u32, slot: SlotId) -> bool {
    count > 0 && slot >= first && slot < first.saturating_add(count)
}

/// Snapshot reserved-gap pool availability for replay/determinism checks.
pub fn gap_pool_snapshot(scaffold: &ArenaParticipantScaffold) -> HashMap<SlotId, Vec<SlotId>> {
    scaffold
        .gap_pools
        .iter()
        .map(|(parent, pool)| (*parent, pool.reserved_slots().to_vec()))
        .collect()
}

/// True when `slots` form a contiguous ascending range (SlotRange precondition).
pub fn slots_are_contiguous(slots: &[SlotId]) -> bool {
    if slots.is_empty() {
        return true;
    }
    let first = slots[0];
    slots
        .iter()
        .enumerate()
        .all(|(i, &slot)| slot == first + i as u32)
}

fn resolve_flow_property(
    registry: &DimensionRegistry,
    key: &PropertyKey,
) -> Result<SimPropertyId, InstallError> {
    registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
        InstallError::Spec(SpecError::UnknownProperty {
            overlay: format!("arena:{}", key.name),
            namespace: key.namespace.clone(),
            name: key.name.clone(),
        })
    })
}

fn seed_participant_property(
    node: &mut SimThing,
    property_id: SimPropertyId,
    registry: &DimensionRegistry,
    hosted_id: SimThingId,
) {
    let layout = registry.property(property_id).layout.clone();
    let mut value = PropertyValue::from_layout(&layout);
    set_hosted_simthing_id(&mut value, &layout, hosted_id);
    node.add_property(property_id, value);
}

fn set_hosted_simthing_id(value: &mut PropertyValue, layout: &PropertyLayout, hosted: SimThingId) {
    let role = SubFieldRole::Named("hosted_simthing_id".into());
    if let Some(offset) = layout.offset_of(&role) {
        value.data[offset] = hosted.raw() as f32;
    }
}

fn find_child<'a>(root: &'a SimThing, id: SimThingId) -> Option<&'a SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_child(child, id) {
            return Some(found);
        }
    }
    None
}
