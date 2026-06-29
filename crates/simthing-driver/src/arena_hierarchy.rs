//! E-11 — arena participant hierarchy and band layout (driver-only).

use simthing_core::{
    expand_arena_internal_columns, AccumulatorRole, DimensionRegistry, PropertyLayout,
    SimPropertyId, SimThing, SimThingId, SlotIndex, SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::SimRuntimeTree;
use std::collections::HashMap;
use thiserror::Error;

use crate::arena_participant::{
    arena_participant_sibling_slots_runtime, slots_are_contiguous, ArenaParticipantScaffold,
};
use crate::arena_registry::{ArenaIdx, GpuArenaDescriptor, SlotId};

/// E-11 child-share EML tree id (one registration per session).
pub const CHILD_SHARE_FORMULA_TREE_ID: u32 = 0xE11_0001;

/// `total_bands(D) = 3·D − 1` (count of OrderBand indices `0 ..= integration_band`).
pub fn total_bands_for_depth(max_depth: u32) -> u32 {
    3u32.saturating_mul(max_depth).saturating_sub(1)
}

/// Integration band index: `D + 2·(D−1) = 3·D − 2`.
pub fn integration_band_for_depth(max_depth: u32) -> u32 {
    max_depth.saturating_add(2 * max_depth.saturating_sub(1))
}

#[derive(Clone, Debug, Default)]
pub struct ArenaExecutionPlan {
    pub arenas: Vec<ArenaTreeLayout>,
    pub arena_participant_index: HashMap<(SimThingId, ArenaIdx), SlotId>,
    pub generation: u64,
}

#[derive(Clone, Debug)]
pub struct ArenaTreeLayout {
    pub arena_idx: ArenaIdx,
    pub arena_root_simthing: SimThingId,
    pub arena_root_slot: SlotId,
    pub participant_roots: Vec<HierarchyNode>,
    pub max_depth: u32,
    pub max_children_per_intermediate: u32,
    pub interior_count: u32,
    pub band_layout: ArenaBandLayout,
    pub reserved_gap_per_intermediate: u32,
    pub flow_property_id: SimPropertyId,
}

#[derive(Clone, Debug)]
pub struct HierarchyNode {
    pub participant_slot: SlotId,
    pub hosted_simthing_id: SimThingId,
    pub depth: u32,
    pub children: Vec<HierarchyNode>,
    pub cols: NodeColumnRefs,
    pub gap_used: u32,
}

impl HierarchyNode {
    pub fn is_interior(&self) -> bool {
        !self.children.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeColumnRefs {
    pub intrinsic_flow_col: u32,
    pub intrinsic_flow_sum_col: u32,
    pub allocated_flow_col: u32,
    pub balance_col: Option<u32>,
    pub weight_col: u32,
    pub weight_sum_col: u32,
    pub propagated_intrinsic_flow_col: u32,
    pub propagated_allocated_flow_col: u32,
    pub propagated_weight_sum_col: u32,
    pub hosted_simthing_id_col: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaBandLayout {
    pub reset_band: u32,
    pub upsweep_band_base: u32,
    pub upsweep_band_count: u32,
    pub downsweep_band_base: u32,
    pub downsweep_band_count: u32,
    pub integration_band: u32,
    pub total_bands_used: u32,
}

impl ArenaBandLayout {
    pub fn for_depth(max_depth: u32) -> Self {
        let total = total_bands_for_depth(max_depth);
        let integration = if max_depth <= 1 {
            total.saturating_sub(1)
        } else {
            integration_band_for_depth(max_depth)
        };
        Self {
            reset_band: 0,
            upsweep_band_base: 1,
            upsweep_band_count: max_depth.saturating_sub(1),
            downsweep_band_base: max_depth,
            downsweep_band_count: 2 * max_depth.saturating_sub(1),
            integration_band: integration,
            total_bands_used: total,
        }
    }

    pub fn broadcast_band(&self, parent_depth: u32, max_depth: u32) -> u32 {
        max_depth.saturating_add(2 * parent_depth)
    }

    pub fn disburse_band(&self, parent_depth: u32, max_depth: u32) -> u32 {
        self.broadcast_band(parent_depth, max_depth)
            .saturating_add(1)
    }

    pub fn upsweep_band(&self, parent_depth: u32, max_depth: u32) -> u32 {
        max_depth.saturating_sub(1).saturating_sub(parent_depth)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum HierarchyError {
    #[error("arena `{arena}` orderband depth budget exceeded: need {needed}, max {max}")]
    OrderBandDepthExceeded {
        arena: String,
        needed: u32,
        max: u32,
    },
    #[error("arena `{arena}` missing AllocatedFlow role on flow property")]
    MissingAllocatedFlow { arena: String },
    #[error("arena `{arena}` missing AllocatorWeight role on flow property")]
    MissingAllocatorWeight { arena: String },
    #[error("arena `{arena}` missing IntrinsicFlow role on flow property")]
    MissingIntrinsicFlow { arena: String },
    #[error("non-contiguous participant children for parent slot {parent_slot}")]
    NonContiguousChildren { parent_slot: SlotId },
    #[error("arena `{arena}` has no participant slots")]
    EmptyParticipants { arena: String },
}

impl ArenaTreeLayout {
    pub fn iter_all(&self) -> Vec<&HierarchyNode> {
        let mut out = Vec::new();
        for root in &self.participant_roots {
            root.walk_subtree(&mut out);
        }
        out
    }

    pub fn iter_at_depth(&self, depth: u32) -> Vec<&HierarchyNode> {
        self.iter_all()
            .into_iter()
            .filter(|n| n.depth == depth)
            .collect()
    }

    pub fn find_node_by_slot(&self, slot: SlotId) -> Option<&HierarchyNode> {
        self.iter_all()
            .into_iter()
            .find(|node| node.participant_slot == slot)
    }

    pub fn interior_participant_slots(&self) -> Vec<SlotId> {
        self.iter_all()
            .into_iter()
            .filter(|node| node.is_interior())
            .map(|node| node.participant_slot)
            .collect()
    }

    pub fn participant_slots(&self) -> Vec<SlotId> {
        self.iter_all()
            .into_iter()
            .map(|n| n.participant_slot)
            .collect()
    }
}

impl HierarchyNode {
    pub fn walk_subtree<'a>(&'a self, out: &mut Vec<&'a HierarchyNode>) {
        out.push(self);
        for child in &self.children {
            child.walk_subtree(out);
        }
    }

    pub fn verify_child_contiguity(&self) -> Result<(), HierarchyError> {
        if self.children.is_empty() {
            return Ok(());
        }
        let slots: Vec<SlotId> = self.children.iter().map(|c| c.participant_slot).collect();
        if !slots_are_contiguous(&slots) {
            return Err(HierarchyError::NonContiguousChildren {
                parent_slot: self.participant_slot,
            });
        }
        for child in &self.children {
            child.verify_child_contiguity()?;
        }
        Ok(())
    }

    pub fn active_child_slots(&self) -> Vec<SlotId> {
        self.children.iter().map(|c| c.participant_slot).collect()
    }
}

pub fn resolve_node_columns(
    layout: &PropertyLayout,
    arena_name: &str,
) -> Result<NodeColumnRefs, HierarchyError> {
    let expanded = expand_arena_internal_columns(layout.clone());
    let arena = arena_name.to_string();

    let intrinsic_flow_col =
        find_role_col(&expanded, |r| matches!(r, AccumulatorRole::IntrinsicFlow)).ok_or_else(
            || HierarchyError::MissingIntrinsicFlow {
                arena: arena.clone(),
            },
        )?;
    let allocated_flow_col = find_role_col(
        &expanded,
        |r| matches!(r, AccumulatorRole::AllocatedFlow { arena: a } if a == arena_name),
    )
    .ok_or_else(|| HierarchyError::MissingAllocatedFlow {
        arena: arena.clone(),
    })?;
    let weight_col = find_role_col(
        &expanded,
        |r| matches!(r, AccumulatorRole::AllocatorWeight { arena: a } if a == arena_name),
    )
    .ok_or_else(|| HierarchyError::MissingAllocatorWeight {
        arena: arena.clone(),
    })?;
    let balance_col = find_role_col(&expanded, |r| matches!(r, AccumulatorRole::Balance(_)));

    let named = |s: &str| {
        expanded
            .offset_of(&SubFieldRole::Named(s.into()))
            .map(|o| o.lane() as u32)
    };
    Ok(NodeColumnRefs {
        intrinsic_flow_col,
        intrinsic_flow_sum_col: named("intrinsic_flow_sum").expect("E-8R column") as u32,
        allocated_flow_col,
        balance_col,
        weight_col,
        weight_sum_col: named("weight_sum").expect("E-8R column") as u32,
        propagated_intrinsic_flow_col: named("propagated_intrinsic_flow").expect("E-8R column")
            as u32,
        propagated_allocated_flow_col: named("propagated_allocated_flow").expect("E-8R column")
            as u32,
        propagated_weight_sum_col: named("propagated_weight_sum").expect("E-8R column") as u32,
        hosted_simthing_id_col: named("hosted_simthing_id").expect("E-8R column") as u32,
    })
}

fn find_role_col(layout: &PropertyLayout, pred: impl Fn(&AccumulatorRole) -> bool) -> Option<u32> {
    layout.sub_fields.iter().enumerate().find_map(|(i, sf)| {
        sf.accumulator_spec
            .as_ref()
            .filter(|s| pred(&s.role))
            .map(|_| i as u32)
    })
}

/// Build a D=2 star hierarchy: first sibling participant is root, remainder are leaves.
pub fn build_flat_star_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    cols: NodeColumnRefs,
    root: &SimRuntimeTree,
    allocator: &SlotAllocator,
    scaffold: &ArenaParticipantScaffold,
    index: &HashMap<(SimThingId, ArenaIdx), SlotId>,
) -> Result<ArenaTreeLayout, HierarchyError> {
    let arena_root_id = *scaffold.arena_root_ids.get(&arena_idx).ok_or_else(|| {
        HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        }
    })?;
    let arena_root_slot = allocator
        .slot_of(arena_root_id)
        .expect("arena root allocated");

    let sibling_slots = arena_participant_sibling_slots_runtime(root, arena_root_id, allocator);
    if sibling_slots.is_empty() {
        return Err(HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        });
    }
    if !slots_are_contiguous(&sibling_slots) {
        return Err(HierarchyError::NonContiguousChildren {
            parent_slot: arena_root_slot,
        });
    }

    let max_depth = if sibling_slots.len() <= 1 { 1 } else { 2 };
    let bands = ArenaBandLayout::for_depth(max_depth);
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
    }

    let root_slot = sibling_slots[0];
    let hosted_root = hosted_for_slot(index, arena_idx, root_slot);
    let leaves: Vec<HierarchyNode> = sibling_slots
        .iter()
        .skip(1)
        .map(|&slot| HierarchyNode {
            participant_slot: slot,
            hosted_simthing_id: hosted_for_slot(index, arena_idx, slot),
            depth: 1,
            children: Vec::new(),
            cols,
            gap_used: 0,
        })
        .collect();

    let root_node = HierarchyNode {
        participant_slot: root_slot,
        hosted_simthing_id: hosted_root,
        depth: 0,
        children: leaves,
        cols,
        gap_used: 0,
    };
    root_node.verify_child_contiguity()?;

    let interior_count = if root_node.is_interior() { 1 } else { 0 };
    Ok(ArenaTreeLayout {
        arena_idx,
        arena_root_simthing: arena_root_id,
        arena_root_slot,
        participant_roots: vec![root_node],
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        reserved_gap_per_intermediate: 0,
        flow_property_id: arena.flow_property_id,
    })
}

/// Build a nested participant hierarchy from already-materialized ArenaParticipant
/// SimThing topology. SlotRange reductions require each parent's direct
/// ArenaParticipant children to occupy a contiguous slot group.
pub fn build_nested_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    cols: NodeColumnRefs,
    root: &SimRuntimeTree,
    allocator: &SlotAllocator,
    scaffold: &ArenaParticipantScaffold,
    index: &HashMap<(SimThingId, ArenaIdx), SlotId>,
) -> Result<ArenaTreeLayout, HierarchyError> {
    let arena_root_id = *scaffold.arena_root_ids.get(&arena_idx).ok_or_else(|| {
        HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        }
    })?;
    let arena_root_slot = allocator
        .slot_of(arena_root_id)
        .expect("arena root allocated");
    let arena_root =
        root.snapshot_node(arena_root_id)
            .ok_or_else(|| HierarchyError::EmptyParticipants {
                arena: arena.name.clone(),
            })?;

    let participant_roots: Vec<HierarchyNode> = arena_root
        .children
        .iter()
        .filter(|&&child_id| root.node_is_arena_participant(child_id))
        .map(|&child_id| build_nested_node(root, child_id, arena_idx, cols, allocator, index, 0))
        .collect::<Result<Vec<_>, _>>()?;
    if participant_roots.is_empty() {
        return Err(HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        });
    }

    let max_depth = max_node_depth(&participant_roots).saturating_add(1);
    let bands = ArenaBandLayout::for_depth(max_depth);
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
    }
    for root in &participant_roots {
        root.verify_child_contiguity()?;
    }

    let interior_count = participant_roots.iter().map(count_interiors).sum::<u32>();

    Ok(ArenaTreeLayout {
        arena_idx,
        arena_root_simthing: arena_root_id,
        arena_root_slot,
        participant_roots,
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        reserved_gap_per_intermediate: 0,
        flow_property_id: arena.flow_property_id,
    })
}

/// Build an arbitrary hierarchy tree for multi-level tests (slots must be pre-validated).
pub fn build_custom_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    _cols: NodeColumnRefs,
    arena_root_id: SimThingId,
    arena_root_slot: SlotId,
    roots: Vec<HierarchyNode>,
) -> Result<ArenaTreeLayout, HierarchyError> {
    let max_depth = {
        let mut nodes = Vec::new();
        for root in &roots {
            root.walk_subtree(&mut nodes);
        }
        nodes
            .iter()
            .map(|n| n.depth)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    };
    let bands = ArenaBandLayout::for_depth(max_depth);
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
    }
    for root in &roots {
        root.verify_child_contiguity()?;
    }
    let interior_count = {
        let mut nodes = Vec::new();
        for root in &roots {
            root.walk_subtree(&mut nodes);
        }
        nodes.iter().filter(|n| n.is_interior()).count() as u32
    };
    Ok(ArenaTreeLayout {
        arena_idx,
        arena_root_simthing: arena_root_id,
        arena_root_slot,
        participant_roots: roots,
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        reserved_gap_per_intermediate: 0,
        flow_property_id: arena.flow_property_id,
    })
}

pub fn build_execution_plan(
    registry: &DimensionRegistry,
    arena_registry: &[GpuArenaDescriptor],
    root: &SimRuntimeTree,
    allocator: &SlotAllocator,
    scaffold: &ArenaParticipantScaffold,
    generation: u64,
) -> Result<ArenaExecutionPlan, HierarchyError> {
    let mut arenas = Vec::new();
    let index = scaffold.index.by_host_and_arena.clone();

    for (arena_idx, arena_desc) in arena_registry.iter().enumerate() {
        let arena_idx = arena_idx as ArenaIdx;
        let layout = registry
            .property(arena_desc.flow_property_id)
            .layout
            .clone();
        let cols = resolve_node_columns(&layout, &arena_desc.name)?;
        let tree = if has_nested_participants(root, scaffold, arena_idx) {
            build_nested_layout(
                arena_idx, arena_desc, cols, root, allocator, scaffold, &index,
            )?
        } else {
            build_flat_star_layout(
                arena_idx, arena_desc, cols, root, allocator, scaffold, &index,
            )?
        };
        arenas.push(tree);
    }

    Ok(ArenaExecutionPlan {
        arenas,
        arena_participant_index: index,
        generation,
    })
}

/// Authoring/test path: plan from a core tree without sim public extraction APIs.
pub fn build_execution_plan_from_authoring(
    registry: &DimensionRegistry,
    arena_registry: &[GpuArenaDescriptor],
    root: &SimThing,
    allocator: &SlotAllocator,
    scaffold: &ArenaParticipantScaffold,
    generation: u64,
) -> Result<ArenaExecutionPlan, HierarchyError> {
    build_execution_plan(
        registry,
        arena_registry,
        &SimRuntimeTree::admit(root.clone()),
        allocator,
        scaffold,
        generation,
    )
}

fn build_nested_node(
    tree: &SimRuntimeTree,
    node_id: SimThingId,
    arena_idx: ArenaIdx,
    cols: NodeColumnRefs,
    allocator: &SlotAllocator,
    index: &HashMap<(SimThingId, ArenaIdx), SlotId>,
    depth: u32,
) -> Result<HierarchyNode, HierarchyError> {
    let participant_slot =
        allocator
            .slot_of(node_id)
            .ok_or(HierarchyError::NonContiguousChildren {
                parent_slot: SlotIndex::new(0),
            })?;
    let snapshot = tree
        .snapshot_node(node_id)
        .ok_or(HierarchyError::NonContiguousChildren {
            parent_slot: SlotIndex::new(0),
        })?;
    let children = snapshot
        .children
        .iter()
        .filter(|&&child_id| tree.node_is_arena_participant(child_id))
        .map(|&child_id| {
            build_nested_node(tree, child_id, arena_idx, cols, allocator, index, depth + 1)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HierarchyNode {
        participant_slot,
        hosted_simthing_id: hosted_for_slot(index, arena_idx, participant_slot),
        depth,
        children,
        cols,
        gap_used: 0,
    })
}

fn max_node_depth(roots: &[HierarchyNode]) -> u32 {
    roots
        .iter()
        .map(|root| {
            let mut nodes = Vec::new();
            root.walk_subtree(&mut nodes);
            nodes.iter().map(|node| node.depth).max().unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

fn count_interiors(root: &HierarchyNode) -> u32 {
    let mut nodes = Vec::new();
    root.walk_subtree(&mut nodes);
    nodes.iter().filter(|node| node.is_interior()).count() as u32
}

fn has_nested_participants(
    root: &SimRuntimeTree,
    scaffold: &ArenaParticipantScaffold,
    arena_idx: ArenaIdx,
) -> bool {
    let Some(arena_root_id) = scaffold.arena_root_ids.get(&arena_idx).copied() else {
        return false;
    };
    root.snapshot_node(arena_root_id)
        .map(|arena_root| {
            arena_root.children.iter().any(|&child_id| {
                root.node_is_arena_participant(child_id)
                    && contains_participant_child(root, child_id)
            })
        })
        .unwrap_or(false)
}

fn contains_participant_child(tree: &SimRuntimeTree, node_id: SimThingId) -> bool {
    let Some(snapshot) = tree.snapshot_node(node_id) else {
        return false;
    };
    snapshot.children.iter().any(|&child_id| {
        tree.node_is_arena_participant(child_id) || contains_participant_child(tree, child_id)
    })
}

fn hosted_for_slot(
    index: &HashMap<(SimThingId, ArenaIdx), SlotId>,
    arena_idx: ArenaIdx,
    slot: SlotId,
) -> SimThingId {
    index
        .iter()
        .find_map(|((hosted, idx), s)| (*s == slot && *idx == arena_idx).then_some(*hosted))
        .unwrap_or_default()
}

/// Driver/test diagnostic for static nested hierarchy materialization (A-0).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedHierarchyMaterializationReport {
    pub max_depth: u32,
    pub participant_root_count: usize,
    pub total_bands: u32,
    pub integration_band: u32,
    pub all_parents_contiguous: bool,
}

/// Summarize a nested [`ArenaTreeLayout`] for boundary/materialization reporting.
pub fn nested_hierarchy_materialization_report(
    layout: &ArenaTreeLayout,
) -> NestedHierarchyMaterializationReport {
    let all_parents_contiguous = layout
        .iter_all()
        .into_iter()
        .filter(|node| !node.children.is_empty())
        .all(|node| node.verify_child_contiguity().is_ok());
    NestedHierarchyMaterializationReport {
        max_depth: layout.max_depth,
        participant_root_count: layout.participant_roots.len(),
        total_bands: layout.band_layout.total_bands_used,
        integration_band: layout.band_layout.integration_band,
        all_parents_contiguous,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_layout_d2_matches_canonical_schedule() {
        let b = ArenaBandLayout::for_depth(2);
        assert_eq!(b.reset_band, 0);
        assert_eq!(b.upsweep_band_count, 1);
        assert_eq!(b.broadcast_band(0, 2), 2);
        assert_eq!(b.disburse_band(0, 2), 3);
        assert_eq!(b.integration_band, 4);
        assert_eq!(b.total_bands_used, 5);
    }

    #[test]
    fn band_layout_d3_integration_follows_deepest_disburse() {
        let b = ArenaBandLayout::for_depth(3);
        let deepest_disburse = b.disburse_band(1, 3);
        assert_eq!(b.integration_band, deepest_disburse + 1);
    }
}
