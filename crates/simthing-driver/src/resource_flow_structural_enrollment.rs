//! Structural-addition dynamic Resource Flow admission (DA, relay 5743461789).
//!
//! An ordinary successful structural addition, meaning the boundary's own
//! `maintainer.allocated` record (for example an ActionBand `AddChild` birth),
//! is admitted to EXISTING arenas by the same definition session-build
//! derivation uses, scoped to the added subtree:
//! - a node participates in an arena only if it carries that arena's flow
//!   property: the parent's arenas are never inherited wholesale;
//! - one matching resource-parent edge is authoritative, and two refuse;
//! - absent an edge, the physical parent is the resource parent only when it
//!   is itself a member of that arena; otherwise the node joins flat;
//! - the node's slot is the one the boundary already committed.
//!
//! Only DERIVED arenas take structural additions. Derived membership is a
//! definition ("every carrier of the flow property"), so a newly added carrier
//! belongs by that same definition. Authored-row arenas (explicit, enrollment
//! or wildcard) are an enumeration and stay closed: a carrier there is
//! reported, never admitted. The whole batch is preflighted, participant
//! capacity and OrderBand depth budget included, before any mutation, and any
//! refusal admits nothing. A successful
//! batch bumps the registry generation exactly once and the caller syncs once.
//! Fission keeps its own policy path.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{DimensionRegistry, ObjectResidencyRelation, SimThingId};
use simthing_gpu::SlotAllocator;
use simthing_sim::SimRuntimeTree;

use crate::arena_hierarchy::{resolve_node_columns_for_property, ArenaBandLayout};
use crate::arena_registry::{ArenaIdx, ArenaRegistry};
use crate::resource_flow_derivation::{ArenaAdmissionOrigin, ResourceFlowDerivationReport};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralEnrollmentAdmission {
    pub simthing_id: SimThingId,
    pub arena_idx: ArenaIdx,
    pub participant_slot: u32,
    pub parent: Option<SimThingId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructuralEnrollmentRefusal {
    /// The carrier's arena enumerates authored rows; it stays closed. This is
    /// a typed non-admission, not a batch failure.
    ClosedArena {
        simthing_id: SimThingId,
        arena: String,
    },
    AmbiguousParentEdge {
        simthing_id: SimThingId,
        arena: String,
        span_tokens: Vec<Option<usize>>,
    },
    ParentNotParticipant {
        simthing_id: SimThingId,
        arena: String,
        parent: SimThingId,
        span_token: Option<usize>,
    },
    MissingSlot {
        simthing_id: SimThingId,
        arena: String,
    },
    Capacity {
        arena: String,
        declared: u32,
        computed: u32,
    },
    /// The admitted subtree would deepen the arena past its OrderBand budget.
    DepthBudget {
        arena: String,
        needed: u32,
        max: u32,
    },
}

impl StructuralEnrollmentRefusal {
    /// Whether this refusal fails the whole batch (everything but a closed arena).
    pub fn fails_batch(&self) -> bool {
        !matches!(self, Self::ClosedArena { .. })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StructuralEnrollmentReport {
    pub added_roots: Vec<SimThingId>,
    pub admissions: Vec<StructuralEnrollmentAdmission>,
    pub refusals: Vec<StructuralEnrollmentRefusal>,
    pub generation_before: u64,
    pub generation_after: u64,
}

impl StructuralEnrollmentReport {
    pub fn any_admissions(&self) -> bool {
        !self.admissions.is_empty()
    }
}

/// Admit the carriers inside the boundary's successfully added subtrees.
/// `allocated` is the boundary's own record: every node of each added
/// subtree, parents first. The walk starts only at its true roots, the nodes
/// whose committed parent was not itself allocated.
pub fn react_to_structural_resource_flow_enrollment(
    allocated: &[SimThingId],
    tree: &SimRuntimeTree,
    registry: &DimensionRegistry,
    arena_registry: &mut ArenaRegistry,
    derivation: &ResourceFlowDerivationReport,
    allocator: &SlotAllocator,
) -> StructuralEnrollmentReport {
    let generation_before = arena_registry.generation;
    let committed_parent = |id: SimThingId| match allocator.relation_of(id) {
        Some(ObjectResidencyRelation::ChildOf(parent)) => Some(parent),
        _ => None,
    };
    let in_batch: BTreeSet<SimThingId> = allocated.iter().copied().collect();
    let mut seen = BTreeSet::new();
    let added_roots: Vec<SimThingId> = allocated
        .iter()
        .copied()
        .filter(|&id| !committed_parent(id).is_some_and(|parent| in_batch.contains(&parent)))
        .filter(|&id| seen.insert(id))
        .collect();
    let mut report = StructuralEnrollmentReport {
        added_roots,
        generation_before,
        generation_after: generation_before,
        ..Default::default()
    };
    if report.added_roots.is_empty() || arena_registry.arenas.is_empty() {
        return report;
    }
    let derived: BTreeSet<&str> = derivation
        .arenas
        .iter()
        .filter(|arena| arena.origin == ArenaAdmissionOrigin::Derived)
        .map(|arena| arena.arena.as_str())
        .collect();

    // Deterministic pre-order over every added subtree, so a parent is planned
    // before its children. Only the added subtrees and each root's committed
    // structural parent are read; the global registry is never re-derived.
    let mut nodes = Vec::new();
    for &root in &report.added_roots {
        let mut pending = vec![(root, committed_parent(root))];
        while let Some((id, parent)) = pending.pop() {
            let Some(snapshot) = tree.snapshot_node(id) else {
                continue;
            };
            for &child in snapshot.children.iter().rev() {
                pending.push((child, Some(id)));
            }
            nodes.push((id, parent, snapshot.property_ids));
        }
    }

    let mut planned = Vec::new();
    let mut planned_members = BTreeSet::new();
    let mut planned_per_arena = BTreeMap::<ArenaIdx, u32>::new();
    for (index, arena) in arena_registry.arenas.iter().enumerate() {
        let arena_idx = index as ArenaIdx;
        let property = registry.property(arena.flow_property_id);
        let is_member = |who: SimThingId, planned: &BTreeSet<(SimThingId, ArenaIdx)>| {
            arena_registry.participant_slot(who, arena_idx).is_some()
                || planned.contains(&(who, arena_idx))
        };
        for (id, physical_parent, properties) in &nodes {
            if !properties.contains(&arena.flow_property_id) {
                continue;
            }
            if is_member(*id, &planned_members) {
                // A replayed or already-admitted addition never duplicates.
                continue;
            }
            if !derived.contains(arena.name.as_str()) {
                report.refusals.push(StructuralEnrollmentRefusal::ClosedArena {
                    simthing_id: *id,
                    arena: arena.name.clone(),
                });
                continue;
            }
            let edges: Vec<_> = tree
                .resource_parent_edges_of(*id)
                .unwrap_or(&[])
                .iter()
                .filter(|edge| {
                    edge.property_namespace == property.namespace
                        && edge.property_name == property.name
                })
                .collect();
            let parent = match edges.as_slice() {
                [] => physical_parent.filter(|parent| is_member(*parent, &planned_members)),
                [edge] if is_member(edge.parent, &planned_members) => Some(edge.parent),
                [edge] => {
                    report
                        .refusals
                        .push(StructuralEnrollmentRefusal::ParentNotParticipant {
                            simthing_id: *id,
                            arena: arena.name.clone(),
                            parent: edge.parent,
                            span_token: edge.source_span_token,
                        });
                    continue;
                }
                _ => {
                    report
                        .refusals
                        .push(StructuralEnrollmentRefusal::AmbiguousParentEdge {
                            simthing_id: *id,
                            arena: arena.name.clone(),
                            span_tokens: edges.iter().map(|edge| edge.source_span_token).collect(),
                        });
                    continue;
                }
            };
            let Some(slot) = allocator.slot_of(*id) else {
                report.refusals.push(StructuralEnrollmentRefusal::MissingSlot {
                    simthing_id: *id,
                    arena: arena.name.clone(),
                });
                continue;
            };
            planned_members.insert((*id, arena_idx));
            *planned_per_arena.entry(arena_idx).or_default() += 1;
            planned.push(StructuralEnrollmentAdmission {
                simthing_id: *id,
                arena_idx,
                participant_slot: slot.raw(),
                parent,
            });
        }
    }
    for (&arena_idx, &count) in &planned_per_arena {
        let arena = &arena_registry.arenas[arena_idx as usize];
        let computed = arena.participant_range.1.saturating_add(count);
        if computed > arena.max_participants {
            report.refusals.push(StructuralEnrollmentRefusal::Capacity {
                arena: arena.name.clone(),
                declared: arena.max_participants,
                computed,
            });
        }
        // The resulting tree must still fit the arena's OrderBand budget, or
        // the post-admission sync would fail after mutating the registry.
        let parents: BTreeMap<SimThingId, Option<SimThingId>> = arena_registry
            .participants
            .iter()
            .filter(|member| member.arena_idx == arena_idx)
            .map(|member| (member.subtree_root, member.parent))
            .chain(
                planned
                    .iter()
                    .filter(|admission| admission.arena_idx == arena_idx)
                    .map(|admission| (admission.simthing_id, admission.parent)),
            )
            .collect();
        let depth = |mut id: SimThingId| {
            let mut depth = 0u32;
            while let Some(Some(parent)) = parents.get(&id) {
                depth += 1;
                id = *parent;
            }
            depth
        };
        let max_depth = parents.keys().map(|&id| depth(id)).max().unwrap_or(0) + 1;
        let governed = resolve_node_columns_for_property(registry, arena.flow_property_id, &arena.name)
            .map(|cols| cols.balance_governing_col.is_some())
            .unwrap_or(false);
        let needed = ArenaBandLayout::for_depth_with_residual_closure(max_depth, governed)
            .total_bands_used;
        if needed > arena.max_orderband_depth {
            report.refusals.push(StructuralEnrollmentRefusal::DepthBudget {
                arena: arena.name.clone(),
                needed,
                max: arena.max_orderband_depth,
            });
        }
    }
    if planned.is_empty() || report.refusals.iter().any(StructuralEnrollmentRefusal::fails_batch)
    {
        return report;
    }
    for admission in &planned {
        arena_registry
            .admit_participant_runtime(
                admission.arena_idx,
                simthing_core::SlotIndex::new(admission.participant_slot),
                admission.simthing_id,
                admission.parent,
            )
            .expect("capacity and membership were preflighted for the whole batch");
    }
    arena_registry.bump_generation_after_runtime_admit();
    report.admissions = planned;
    report.generation_after = arena_registry.generation;
    report
}
