//! Default Resource Flow admission derived from populated SimThings.
//!
//! Authored `ResourceFlowSpec` remains an override surface. When a registered
//! accumulator property is present on SimThings, its arena identity plus
//! resource-parent edges are sufficient to produce the same explicit plan the
//! downstream compiler and GPU path already consume.

use simthing_core::{
    AccumulatorRole, DimensionRegistry, SimPropertyId, SimThing, SimThingId, SlotIndex,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey, ResourceFlowSpec,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArenaAdmissionOrigin {
    Derived,
    AuthoredOverride,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivedArenaParticipation {
    pub arena: String,
    pub property: PropertyKey,
    pub origin: ArenaAdmissionOrigin,
    pub participants: Vec<DerivedParticipant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivedParticipant {
    pub simthing_id: SimThingId,
    pub slot: SlotIndex,
    pub parent: Option<SimThingId>,
    pub source_span_token: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceFlowDerivationReport {
    pub arenas: Vec<DerivedArenaParticipation>,
}

#[derive(Clone, Debug)]
pub struct ResolvedResourceFlowAdmission {
    pub spec: Option<ResourceFlowSpec>,
    pub report: ResourceFlowDerivationReport,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResourceFlowDerivationError {
    #[error(
        "resource property `{property}` resolves to multiple arenas {arenas:?} (span_token={span_token:?})"
    )]
    AmbiguousPropertyArena {
        property: String,
        arenas: Vec<String>,
        span_token: Option<usize>,
    },
    #[error(
        "arena `{arena}` is derived from multiple populated resource properties {properties:?} (span_token={span_token:?})"
    )]
    AmbiguousArenaProperty {
        arena: String,
        properties: Vec<String>,
        span_token: Option<usize>,
    },
    #[error(
        "arena `{arena}` authored flow property `{authored_property}` conflicts with populated resource property `{derived_property}` (span_token={span_token:?})"
    )]
    AuthoredArenaPropertyConflict {
        arena: String,
        authored_property: String,
        derived_property: String,
        span_token: Option<usize>,
    },
    #[error(
        "SimThing {simthing_id:?} has multiple parent edges for resource property `{property}` (span_tokens={span_tokens:?})"
    )]
    AmbiguousParentEdge {
        property: String,
        simthing_id: SimThingId,
        span_tokens: Vec<Option<usize>>,
    },
    #[error(
        "SimThing {simthing_id:?} resource parent {parent_id:?} does not possess property `{property}` (span_token={span_token:?})"
    )]
    ParentMissingResourceProperty {
        property: String,
        simthing_id: SimThingId,
        parent_id: SimThingId,
        span_token: Option<usize>,
    },
    #[error(
        "SimThing {simthing_id:?} with resource property `{property}` has no allocated slot (span_token={span_token:?})"
    )]
    ParticipantMissingSlot {
        property: String,
        simthing_id: SimThingId,
        span_token: Option<usize>,
    },
}

struct PropertyArenaCandidate {
    property_id: SimPropertyId,
    property: PropertyKey,
    arena: String,
    has_balance: bool,
}

struct PopulatedNode<'a> {
    node: &'a SimThing,
    physical_parent: Option<SimThingId>,
}

/// Resolve the default admission plan. Explicit enrollment and wildcard rows
/// remain authoritative overrides; an arena override with no rows supplies
/// caps/policy while its participants are still derived.
pub fn derive_resource_flow_admission(
    authored: Option<&ResourceFlowSpec>,
    registry: &DimensionRegistry,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<ResolvedResourceFlowAdmission, ResourceFlowDerivationError> {
    let populated = collect_populated_nodes(root);
    let mut candidates = Vec::new();

    for (property_index, property) in registry.properties.iter().enumerate() {
        let property_id = SimPropertyId(property_index as u32);
        if !populated
            .iter()
            .any(|entry| entry.node.properties.contains_key(&property_id))
        {
            continue;
        }

        let mut arena_names = BTreeSet::new();
        let mut has_balance = false;
        for sub_field in &property.layout.sub_fields {
            let Some(accumulator) = sub_field.accumulator_spec.as_ref() else {
                continue;
            };
            match &accumulator.role {
                AccumulatorRole::AllocatedFlow { arena }
                | AccumulatorRole::AllocatorWeight { arena } => {
                    arena_names.insert(arena.clone());
                }
                AccumulatorRole::Balance(_) => has_balance = true,
                AccumulatorRole::IntrinsicFlow => {}
            }
        }
        if arena_names.is_empty() {
            continue;
        }
        let key = PropertyKey::new(&property.namespace, &property.name);
        if arena_names.len() != 1 {
            return Err(ResourceFlowDerivationError::AmbiguousPropertyArena {
                property: format_property_key(&key),
                arenas: arena_names.into_iter().collect(),
                span_token: first_property_edge_span(&populated, &key),
            });
        }
        candidates.push(PropertyArenaCandidate {
            property_id,
            property: key,
            arena: arena_names.into_iter().next().expect("one arena"),
            has_balance,
        });
    }

    let mut properties_by_arena: BTreeMap<&str, Vec<&PropertyArenaCandidate>> = BTreeMap::new();
    for candidate in &candidates {
        properties_by_arena
            .entry(candidate.arena.as_str())
            .or_default()
            .push(candidate);
    }
    for (arena, properties) in &properties_by_arena {
        if properties.len() > 1 {
            return Err(ResourceFlowDerivationError::AmbiguousArenaProperty {
                arena: (*arena).to_string(),
                properties: properties
                    .iter()
                    .map(|candidate| format_property_key(&candidate.property))
                    .collect(),
                span_token: properties.iter().find_map(|candidate| {
                    first_property_edge_span(&populated, &candidate.property)
                }),
            });
        }
    }

    let mut resolved = authored.cloned().unwrap_or_default();
    let mut report = ResourceFlowDerivationReport::default();

    for candidate in candidates {
        let override_index = resolved
            .arenas
            .iter()
            .position(|arena| arena.name == candidate.arena);
        if let Some(index) = override_index {
            let arena = &resolved.arenas[index];
            if arena.flow_property != candidate.property {
                return Err(ResourceFlowDerivationError::AuthoredArenaPropertyConflict {
                    arena: candidate.arena,
                    authored_property: format_property_key(&arena.flow_property),
                    derived_property: format_property_key(&candidate.property),
                    span_token: first_property_edge_span(&populated, &candidate.property),
                });
            }
            if !arena.explicit_participants.is_empty()
                || arena.enrollment.is_some()
                || arena.wildcard_admission.is_some()
            {
                report.arenas.push(DerivedArenaParticipation {
                    arena: candidate.arena,
                    property: candidate.property,
                    origin: ArenaAdmissionOrigin::AuthoredOverride,
                    participants: arena
                        .explicit_participants
                        .iter()
                        .map(|participant| DerivedParticipant {
                            simthing_id: SimThingId::from_session_raw(participant.subtree_root_id),
                            slot: SlotIndex::new(participant.slot),
                            parent: participant
                                .parent_subtree_root_id
                                .map(|raw| SimThingId::from_session_raw(raw as u32)),
                            source_span_token: None,
                        })
                        .collect(),
                });
                continue;
            }
        }

        let participants = derive_participants(&candidate, &populated, allocator)?;
        let explicit_participants = participants
            .iter()
            .map(|participant| match participant.parent {
                Some(parent) => ExplicitParticipantSpec::nested(
                    participant.slot.raw(),
                    participant.simthing_id.raw(),
                    u64::from(parent.raw()),
                ),
                None => ExplicitParticipantSpec::flat(
                    participant.slot.raw(),
                    participant.simthing_id.raw(),
                ),
            })
            .collect();

        if let Some(index) = override_index {
            resolved.arenas[index].explicit_participants = explicit_participants;
        } else {
            let participant_cap = (participants.len() as u32).max(1).next_power_of_two();
            resolved.arenas.push(ArenaSpec {
                name: candidate.arena.clone(),
                flow_property: candidate.property.clone(),
                balance_property: candidate.has_balance.then(|| candidate.property.clone()),
                max_participants: participant_cap,
                max_coupling_fanout: participant_cap / 2,
                max_orderband_depth: participant_cap.saturating_mul(2),
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                explicit_participants,
                enrollment: None,
                wildcard_admission: None,
            });
        }
        report.arenas.push(DerivedArenaParticipation {
            arena: candidate.arena,
            property: candidate.property,
            origin: ArenaAdmissionOrigin::Derived,
            participants,
        });
    }

    let spec = if authored.is_some() || !resolved.arenas.is_empty() {
        Some(resolved)
    } else {
        None
    };
    Ok(ResolvedResourceFlowAdmission { spec, report })
}

fn derive_participants(
    candidate: &PropertyArenaCandidate,
    populated: &[PopulatedNode<'_>],
    allocator: &SlotAllocator,
) -> Result<Vec<DerivedParticipant>, ResourceFlowDerivationError> {
    let participant_ids: HashSet<SimThingId> = populated
        .iter()
        .filter(|entry| entry.node.properties.contains_key(&candidate.property_id))
        .map(|entry| entry.node.id)
        .collect();
    let property_label = format_property_key(&candidate.property);
    let mut out = Vec::with_capacity(participant_ids.len());

    for entry in populated
        .iter()
        .filter(|entry| participant_ids.contains(&entry.node.id))
    {
        let matching_edges: Vec<_> = entry
            .node
            .resource_parent_edges
            .iter()
            .filter(|edge| {
                edge.property_namespace == candidate.property.namespace
                    && edge.property_name == candidate.property.name
            })
            .collect();
        if matching_edges.len() > 1 {
            return Err(ResourceFlowDerivationError::AmbiguousParentEdge {
                property: property_label,
                simthing_id: entry.node.id,
                span_tokens: matching_edges
                    .iter()
                    .map(|edge| edge.source_span_token)
                    .collect(),
            });
        }
        let (parent, span_token) = match matching_edges.as_slice() {
            [edge] => (Some(edge.parent), edge.source_span_token),
            [] => (
                entry
                    .physical_parent
                    .filter(|parent| participant_ids.contains(parent)),
                None,
            ),
            _ => unreachable!("ambiguity rejected"),
        };
        if let Some(parent_id) = parent {
            if !participant_ids.contains(&parent_id) {
                return Err(ResourceFlowDerivationError::ParentMissingResourceProperty {
                    property: property_label,
                    simthing_id: entry.node.id,
                    parent_id,
                    span_token,
                });
            }
        }
        let slot = allocator.slot_of(entry.node.id).ok_or_else(|| {
            ResourceFlowDerivationError::ParticipantMissingSlot {
                property: property_label.clone(),
                simthing_id: entry.node.id,
                span_token,
            }
        })?;
        out.push(DerivedParticipant {
            simthing_id: entry.node.id,
            slot,
            parent,
            source_span_token: span_token,
        });
    }
    Ok(out)
}

fn collect_populated_nodes(root: &SimThing) -> Vec<PopulatedNode<'_>> {
    fn visit<'a>(node: &'a SimThing, parent: Option<SimThingId>, out: &mut Vec<PopulatedNode<'a>>) {
        out.push(PopulatedNode {
            node,
            physical_parent: parent,
        });
        for child in &node.children {
            visit(child, Some(node.id), out);
        }
    }
    let mut out = Vec::new();
    visit(root, None, &mut out);
    out
}

fn first_property_edge_span(
    populated: &[PopulatedNode<'_>],
    property: &PropertyKey,
) -> Option<usize> {
    populated.iter().find_map(|entry| {
        entry.node.resource_parent_edges.iter().find_map(|edge| {
            (edge.property_namespace == property.namespace && edge.property_name == property.name)
                .then_some(edge.source_span_token)
                .flatten()
        })
    })
}

fn format_property_key(key: &PropertyKey) -> String {
    format!("{}::{}", key.namespace, key.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        AccumulatorSpec, ClampBehavior, LogTier, PropertyValue, SimThingKind, SubFieldRole,
        SubFieldSpec,
    };
    use simthing_spec::{compile_property, PropertySpec};

    fn resource_property() -> PropertySpec {
        let sub_field = |name: &str, role: AccumulatorRole| SubFieldSpec {
            role: SubFieldRole::Named(name.into()),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: name.into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: Some(AccumulatorSpec {
                role,
                log_tier: LogTier::Summary,
            }),
        };
        PropertySpec {
            id: "flow".into(),
            namespace: "derive_test".into(),
            name: "flow".into(),
            display_name: "flow".into(),
            description: String::new(),
            admission_disposition: Default::default(),
            sub_fields: vec![
                sub_field("intrinsic", AccumulatorRole::IntrinsicFlow),
                sub_field(
                    "allocated",
                    AccumulatorRole::AllocatedFlow {
                        arena: "derived".into(),
                    },
                ),
                sub_field(
                    "weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: "derived".into(),
                    },
                ),
            ],
        }
    }

    fn populated_chain() -> (DimensionRegistry, SimThing, SlotAllocator) {
        let mut registry = DimensionRegistry::new();
        let (property_id, _) =
            compile_property(&resource_property(), &mut registry).expect("property");
        let value = || PropertyValue::from_layout(&registry.property(property_id).layout);

        let mut leaf = SimThing::new(SimThingKind::Custom("leaf".into()), 0);
        leaf.add_property(property_id, value());
        let mut parent = SimThing::new(SimThingKind::Custom("parent".into()), 0);
        parent.add_property(property_id, value());
        parent.add_child(leaf);
        let mut root = SimThing::new(SimThingKind::Custom("root".into()), 0);
        root.add_property(property_id, value());
        root.add_child(parent);

        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        (registry, root, allocator)
    }

    #[test]
    fn populated_property_and_parent_edges_derive_recursive_arena() {
        let (registry, root, allocator) = populated_chain();
        let admission =
            derive_resource_flow_admission(None, &registry, &root, &allocator).expect("derive");
        let report = &admission.report.arenas[0];
        assert_eq!(report.origin, ArenaAdmissionOrigin::Derived);
        assert_eq!(report.participants.len(), 3);
        assert_eq!(report.participants[0].parent, None);
        assert_eq!(
            report.participants[1].parent,
            Some(report.participants[0].simthing_id)
        );
        assert_eq!(
            report.participants[2].parent,
            Some(report.participants[1].simthing_id)
        );
    }

    #[test]
    fn ambiguous_resource_parent_edges_preserve_source_spans() {
        let (registry, mut root, allocator) = populated_chain();
        let root_id = root.id;
        let parent_id = root.children[0].id;
        let leaf = &mut root.children[0].children[0];
        leaf.add_resource_parent_edge("derive_test", "flow", root_id, Some(31));
        leaf.add_resource_parent_edge("derive_test", "flow", parent_id, Some(47));

        let error = derive_resource_flow_admission(None, &registry, &root, &allocator)
            .expect_err("ambiguous edges must fail");
        assert!(matches!(
            error,
            ResourceFlowDerivationError::AmbiguousParentEdge {
                span_tokens,
                ..
            } if span_tokens == vec![Some(31), Some(47)]
        ));
    }
}
