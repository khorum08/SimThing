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
use std::collections::HashMap;
use thiserror::Error;

use crate::arena_registry::{ArenaIdx, FissionPolicy, SlotId};
use crate::install::InstallError;

/// `(hosted SimThing id, arena idx) → arena-participant slot`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArenaParticipantIndex {
    pub by_host_and_arena: HashMap<(SimThingId, ArenaIdx), SlotId>,
}

impl ArenaParticipantIndex {
    pub fn participant_slot(&self, hosted: SimThingId, arena_idx: ArenaIdx) -> Option<SlotId> {
        self.by_host_and_arena
            .get(&(hosted, arena_idx))
            .copied()
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

        let mut arena_root = SimThing::new(
            SimThingKind::Custom(format!("arena_root:{}", arena.name)),
            0,
        );
        let arena_root_id = arena_root.id;
        let mut hosted_ids: Vec<SimThingId> = Vec::new();

        for participant in &arena.explicit_participants {
            let hosted_id = SimThingId::from_session_raw(participant.subtree_root_id);
            let mut node = SimThing::new(SimThingKind::ArenaParticipant, 0);
            seed_participant_property(&mut node, flow_property_id, registry, hosted_id);
            arena_root.add_child(node);
            hosted_ids.push(hosted_id);
        }

        root.add_child(arena_root);
        allocator.alloc(arena_root_id);
        let arena_root_slot = allocator
            .slot_of(arena_root_id)
            .expect("arena root just allocated");

        let arena_root_ref = find_child(root, arena_root_id).expect("arena root attached");
        let mut participant_slots = Vec::new();
        for child in &arena_root_ref.children {
            if child.kind != SimThingKind::ArenaParticipant {
                continue;
            }
            allocator.alloc(child.id);
            participant_slots.push(allocator.slot_of(child.id).expect("participant allocated"));
        }

        for (hosted_id, participant_slot) in hosted_ids.iter().zip(participant_slots.iter()) {
            scaffold
                .index
                .by_host_and_arena
                .insert((*hosted_id, arena_idx), *participant_slot);
        }

        let gap_k = arena.reserved_gap_per_intermediate;
        let gap_block_first = if gap_k > 0 && !participant_slots.is_empty() {
            let total_gaps = gap_k.saturating_mul(participant_slots.len() as u32);
            let gap_block = allocator.reserve_exclusive_gap_block(total_gaps);
            for (i, participant_slot) in participant_slots.iter().enumerate() {
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
        } else {
            None
        };

        scaffold.arena_root_ids.insert(arena_idx, arena_root_id);
        scaffold.reports.push(ArenaParticipantAllocationReport {
            arena: arena.name.clone(),
            root_slot: arena_root_slot,
            participant_count: participant_slots.len() as u32,
            reserved_gap_per_intermediate: arena.reserved_gap_per_intermediate,
            max_children_per_intermediate: arena.expected_max_children_per_intermediate,
            participant_sibling_first: participant_slots.first().copied(),
            gap_block_first,
        });
    }

    Ok(scaffold)
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
    let pool = scaffold
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
    let parent_id = allocator.owner_of(parent_participant_slot).expect("parent slot");
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
