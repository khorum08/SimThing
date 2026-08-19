//! E-11 — arena participant hierarchy and band layout (driver-only).

use simthing_core::{
    expand_arena_internal_columns, AccumulatorRole, ColumnIndex, DimensionRegistry,
    PropertyColumnRange, PropertyLayout, SimPropertyId, SimThingId, SubFieldRole,
};
use std::collections::HashMap;
use thiserror::Error;

use crate::arena_registry::{ArenaIdx, ArenaMember, ArenaRegistry, GpuArenaDescriptor, SlotId};

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
    pub member_index: HashMap<(SimThingId, ArenaIdx), SlotId>,
    pub generation: u64,
}

#[derive(Clone, Debug)]
pub struct ArenaTreeLayout {
    pub arena_idx: ArenaIdx,
    pub participant_roots: Vec<HierarchyNode>,
    pub max_depth: u32,
    pub max_children_per_intermediate: u32,
    pub interior_count: u32,
    pub band_layout: ArenaBandLayout,
    pub flow_property_id: SimPropertyId,
}

#[derive(Clone, Debug)]
pub struct HierarchyNode {
    pub participant_slot: SlotId,
    pub hosted_simthing_id: SimThingId,
    pub depth: u32,
    pub children: Vec<HierarchyNode>,
    pub cols: NodeColumnRefs,
}

impl HierarchyNode {
    pub fn is_interior(&self) -> bool {
        !self.children.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeColumnRefs {
    pub intrinsic_flow_col: ColumnIndex,
    pub intrinsic_flow_sum_col: ColumnIndex,
    pub allocated_flow_col: ColumnIndex,
    pub balance_col: Option<ColumnIndex>,
    pub balance_governing_col: Option<ColumnIndex>,
    pub weight_col: ColumnIndex,
    pub weight_sum_col: ColumnIndex,
    pub propagated_intrinsic_flow_col: ColumnIndex,
    pub propagated_allocated_flow_col: ColumnIndex,
    pub propagated_weight_sum_col: ColumnIndex,
    pub hosted_simthing_id_col: ColumnIndex,
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
        Self::for_depth_with_residual_closure(max_depth, false)
    }

    pub fn for_depth_with_residual_closure(max_depth: u32, has_residual_closure: bool) -> Self {
        let total = total_bands_for_depth(max_depth);
        let base_integration = if max_depth <= 1 {
            total.saturating_sub(1)
        } else {
            integration_band_for_depth(max_depth)
        };
        let residual_bands = u32::from(has_residual_closure && max_depth > 1) * 4;
        let integration = base_integration.saturating_add(residual_bands);
        Self {
            reset_band: 0,
            upsweep_band_base: 1,
            upsweep_band_count: max_depth.saturating_sub(1),
            downsweep_band_base: max_depth,
            downsweep_band_count: 2 * max_depth.saturating_sub(1),
            integration_band: integration,
            total_bands_used: total.saturating_add(residual_bands),
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
    #[error("resource-parent edge names unknown arena member {member:?}")]
    UnknownMember { member: SimThingId },
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

    pub fn active_child_slots(&self) -> Vec<SlotId> {
        self.children.iter().map(|c| c.participant_slot).collect()
    }
}

pub fn resolve_node_columns(
    range: &PropertyColumnRange,
    layout: &PropertyLayout,
    arena_name: &str,
) -> Result<NodeColumnRefs, HierarchyError> {
    let expanded = expand_arena_internal_columns(layout.clone());
    // Authoritative registry start — never fabricate a zero-start range as a
    // substitute for registry.column_range(...).
    let range = PropertyColumnRange {
        start: range.start,
        stride: expanded.stride(),
    };
    let arena = arena_name.to_string();

    let intrinsic_flow_col = find_role_col(&range, &expanded, |r| {
        matches!(r, AccumulatorRole::IntrinsicFlow)
    })
    .ok_or_else(|| HierarchyError::MissingIntrinsicFlow {
        arena: arena.clone(),
    })?;
    let allocated_flow_col = find_role_col(
        &range,
        &expanded,
        |r| matches!(r, AccumulatorRole::AllocatedFlow { arena: a } if a == arena_name),
    )
    .ok_or_else(|| HierarchyError::MissingAllocatedFlow {
        arena: arena.clone(),
    })?;
    let weight_col = find_role_col(
        &range,
        &expanded,
        |r| matches!(r, AccumulatorRole::AllocatorWeight { arena: a } if a == arena_name),
    )
    .ok_or_else(|| HierarchyError::MissingAllocatorWeight {
        arena: arena.clone(),
    })?;
    let balance_subfield = expanded.sub_fields.iter().find(|subfield| {
        subfield
            .accumulator_spec
            .as_ref()
            .is_some_and(|spec| matches!(&spec.role, AccumulatorRole::Balance(_)))
    });
    let balance_col =
        balance_subfield.and_then(|subfield| range.col_for_role(&subfield.role, &expanded));
    let balance_governing_col = balance_subfield
        .and_then(|subfield| subfield.governed_by.as_ref())
        .and_then(|role| range.col_for_role(role, &expanded));

    let named = |s: &str| {
        range
            .col_for_role(&SubFieldRole::Named(s.into()), &expanded)
            .expect("E-8R column")
    };
    Ok(NodeColumnRefs {
        intrinsic_flow_col,
        intrinsic_flow_sum_col: named("intrinsic_flow_sum"),
        allocated_flow_col,
        balance_col,
        balance_governing_col,
        weight_col,
        weight_sum_col: named("weight_sum"),
        propagated_intrinsic_flow_col: named("propagated_intrinsic_flow"),
        propagated_allocated_flow_col: named("propagated_allocated_flow"),
        propagated_weight_sum_col: named("propagated_weight_sum"),
        hosted_simthing_id_col: named("hosted_simthing_id"),
    })
}

/// Resolve arena node columns from the registry-owned property range + layout.
pub fn resolve_node_columns_for_property(
    registry: &DimensionRegistry,
    flow_property_id: SimPropertyId,
    arena_name: &str,
) -> Result<NodeColumnRefs, HierarchyError> {
    resolve_node_columns(
        registry.column_range(flow_property_id),
        &registry.property(flow_property_id).layout,
        arena_name,
    )
}

fn find_role_col(
    range: &PropertyColumnRange,
    layout: &PropertyLayout,
    pred: impl Fn(&AccumulatorRole) -> bool,
) -> Option<ColumnIndex> {
    layout.sub_fields.iter().find_map(|sf| {
        sf.accumulator_spec
            .as_ref()
            .filter(|s| pred(&s.role))
            .and_then(|_| range.col_for_role(&sf.role, layout))
    })
}

/// Build a D=2 star hierarchy: the first admitted member is root, remainder are leaves.
pub fn build_flat_star_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    cols: NodeColumnRefs,
    members: &[ArenaMember],
) -> Result<ArenaTreeLayout, HierarchyError> {
    if members.is_empty() {
        return Err(HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        });
    }

    let max_depth = if members.len() <= 1 { 1 } else { 2 };
    let bands = ArenaBandLayout::for_depth_with_residual_closure(
        max_depth,
        cols.balance_governing_col.is_some(),
    );
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
    }

    let root_member = &members[0];
    let leaves: Vec<HierarchyNode> = members
        .iter()
        .skip(1)
        .map(|member| HierarchyNode {
            participant_slot: member.slot,
            hosted_simthing_id: member.subtree_root,
            depth: 1,
            children: Vec::new(),
            cols,
        })
        .collect();

    let root_node = HierarchyNode {
        participant_slot: root_member.slot,
        hosted_simthing_id: root_member.subtree_root,
        depth: 0,
        children: leaves,
        cols,
    };
    let interior_count = if root_node.is_interior() { 1 } else { 0 };
    Ok(ArenaTreeLayout {
        arena_idx,
        participant_roots: vec![root_node],
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        flow_property_id: arena.flow_property_id,
    })
}

/// Build a nested hierarchy from the resource-parent edges carried by admitted rows.
pub fn build_nested_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    cols: NodeColumnRefs,
    members: &[ArenaMember],
) -> Result<ArenaTreeLayout, HierarchyError> {
    let by_id: HashMap<SimThingId, &ArenaMember> = members
        .iter()
        .map(|member| (member.subtree_root, member))
        .collect();
    let mut children_by_parent: HashMap<SimThingId, Vec<SimThingId>> = HashMap::new();
    for member in members {
        if let Some(parent) = member.parent {
            if !by_id.contains_key(&parent) {
                return Err(HierarchyError::UnknownMember { member: parent });
            }
            children_by_parent
                .entry(parent)
                .or_default()
                .push(member.subtree_root);
        }
    }
    let participant_roots: Vec<HierarchyNode> = members
        .iter()
        .filter(|member| member.parent.is_none())
        .map(|member| build_nested_node(member.subtree_root, cols, &by_id, &children_by_parent, 0))
        .collect::<Result<Vec<_>, _>>()?;
    if participant_roots.is_empty() {
        return Err(HierarchyError::EmptyParticipants {
            arena: arena.name.clone(),
        });
    }

    let max_depth = max_node_depth(&participant_roots).saturating_add(1);
    let bands = ArenaBandLayout::for_depth_with_residual_closure(
        max_depth,
        cols.balance_governing_col.is_some(),
    );
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
    }
    let interior_count = participant_roots.iter().map(count_interiors).sum::<u32>();

    Ok(ArenaTreeLayout {
        arena_idx,
        participant_roots,
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        flow_property_id: arena.flow_property_id,
    })
}

/// Build an arbitrary hierarchy tree for multi-level tests (slots must be pre-validated).
pub fn build_custom_layout(
    arena_idx: ArenaIdx,
    arena: &GpuArenaDescriptor,
    cols: NodeColumnRefs,
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
    let bands = ArenaBandLayout::for_depth_with_residual_closure(
        max_depth,
        cols.balance_governing_col.is_some(),
    );
    if bands.total_bands_used > arena.max_orderband_depth {
        return Err(HierarchyError::OrderBandDepthExceeded {
            arena: arena.name.clone(),
            needed: bands.total_bands_used,
            max: arena.max_orderband_depth,
        });
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
        participant_roots: roots,
        max_depth,
        max_children_per_intermediate: arena.max_participants,
        interior_count,
        band_layout: bands,
        flow_property_id: arena.flow_property_id,
    })
}

pub fn build_execution_plan(
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
) -> Result<ArenaExecutionPlan, HierarchyError> {
    let mut arenas = Vec::new();
    let index = arena_registry.participant_index();

    for (arena_idx, arena_desc) in arena_registry.arenas.iter().enumerate() {
        let arena_idx = arena_idx as ArenaIdx;
        let members: Vec<ArenaMember> = arena_registry
            .participants
            .iter()
            .filter(|member| member.arena_idx == arena_idx)
            .cloned()
            .collect();
        let cols = resolve_node_columns_for_property(
            registry,
            arena_desc.flow_property_id,
            &arena_desc.name,
        )?;
        let tree = if members.iter().any(|member| member.parent.is_some()) {
            build_nested_layout(arena_idx, arena_desc, cols, &members)?
        } else {
            build_flat_star_layout(arena_idx, arena_desc, cols, &members)?
        };
        arenas.push(tree);
    }

    Ok(ArenaExecutionPlan {
        arenas,
        member_index: index,
        generation: arena_registry.generation,
    })
}

/// Authoring/test alias kept to make fixture intent explicit.
pub fn build_execution_plan_from_authoring(
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
) -> Result<ArenaExecutionPlan, HierarchyError> {
    build_execution_plan(registry, arena_registry)
}

fn build_nested_node(
    node_id: SimThingId,
    cols: NodeColumnRefs,
    by_id: &HashMap<SimThingId, &ArenaMember>,
    children_by_parent: &HashMap<SimThingId, Vec<SimThingId>>,
    depth: u32,
) -> Result<HierarchyNode, HierarchyError> {
    let member = by_id
        .get(&node_id)
        .copied()
        .ok_or(HierarchyError::UnknownMember { member: node_id })?;
    let children = children_by_parent
        .get(&node_id)
        .into_iter()
        .flatten()
        .map(|&child_id| build_nested_node(child_id, cols, by_id, children_by_parent, depth + 1))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HierarchyNode {
        participant_slot: member.slot,
        hosted_simthing_id: member.subtree_root,
        depth,
        children,
        cols,
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

/// Driver/test diagnostic for static nested hierarchy materialization (A-0).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedHierarchyMaterializationReport {
    pub max_depth: u32,
    pub participant_root_count: usize,
    pub total_bands: u32,
    pub integration_band: u32,
    pub resource_parent_edge_count: usize,
}

/// Summarize a nested [`ArenaTreeLayout`] for boundary/materialization reporting.
pub fn nested_hierarchy_materialization_report(
    layout: &ArenaTreeLayout,
) -> NestedHierarchyMaterializationReport {
    let resource_parent_edge_count = layout
        .iter_all()
        .into_iter()
        .map(|node| node.children.len())
        .sum();
    NestedHierarchyMaterializationReport {
        max_depth: layout.max_depth,
        participant_root_count: layout.participant_roots.len(),
        total_bands: layout.band_layout.total_bands_used,
        integration_band: layout.band_layout.integration_band,
        resource_parent_edge_count,
    }
}
