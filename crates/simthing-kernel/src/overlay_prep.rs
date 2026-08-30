//! CPU preparation pass for Pass 3: iterative overlay transform application.
//!
//! Walks the SimThing tree depth-first in the same order as `Evaluator::evaluate_node`,
//! building a flat `Vec<OverlayDelta>` (ancestor stack first, then local, in evaluation
//! order) and a `Vec<SlotDeltaRange>` (one per slot, indexed by slot index).
//!
//! The GPU shader (Pass 3) walks each slot's delta range and applies ops in order —
//! same order the CPU evaluator applies them in step 5. Bit-exact parity is therefore
//! trivially preserved: no composition step, no rounding-order divergence.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use simthing_core::overlay::PropertyTransformDelta;
use simthing_core::{
    DimensionRegistry, GenerationStamp, LiveOverlayRoutes, Overlay, OverlayId, OverlayKind,
    SimPropertyId, SimThing, SimThingId, SubFieldRole,
};

use crate::derived_span_projection::{
    ChangedLocus, DerivedDependencyBinding, DerivedDependencyIndex, DerivedInvalidation,
    DerivedSpanAdmissionError, DerivedSpanProjection, EffectiveProfileId, EffectiveSpanSeed,
    LogicalRowRange, LogicalSubtreeDirectory,
};
use crate::slot::SlotAllocator;
use crate::wgsl_encode::encode_column;
use crate::world_state::{OverlayDelta, SlotDeltaRange, OP_ADD, OP_MULTIPLY, OP_SET};

pub use crate::clearing_weight_projection::{
    resolve_effective_clearing_weights, ClearingWeightOverrideSpec, ClearingWeightResolutionError,
    ClearingWeightSpanProjection,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OverlayProfileOpKind {
    Add,
    Multiply,
    Set,
}

impl OverlayProfileOpKind {
    fn wire(self) -> u32 {
        match self {
            Self::Add => OP_ADD,
            Self::Multiply => OP_MULTIPLY,
            Self::Set => OP_SET,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct OverlayProfileOp {
    host: SimThingId,
    property_id: SimPropertyId,
    role: SubFieldRole,
    kind: OverlayProfileOpKind,
    value_bits: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OverlayEffectiveDescriptor {
    sources: Arc<Vec<OverlayProfileOp>>,
    admitted_properties: Arc<Vec<SimPropertyId>>,
}

#[derive(Clone, Debug, PartialEq)]
struct OverlayNodeSnapshot {
    parent: Option<SimThingId>,
    path: Vec<usize>,
    overlays: Vec<Overlay>,
    active_overlays: Vec<Overlay>,
    admitted_properties: Arc<Vec<SimPropertyId>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OverlayDenseMaterialization {
    pub(crate) deltas: Vec<OverlayDelta>,
    pub(crate) ranges: Vec<SlotDeltaRange>,
    pub(crate) rows_materialized: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct OverlayProjectionMetrics {
    pub(crate) logical_rows: u64,
    pub(crate) profiles: u64,
    pub(crate) spans: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct OverlayProjectionRefresh {
    pub(crate) invalidation: DerivedInvalidation,
    pub(crate) semantic_spans_rebuilt: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OverlayProjectionHostChange {
    OverlayState(SimThingId),
    PropertyShape(SimThingId),
}

/// Boundary-compiled effective overlay profiles. Descriptors contain logical
/// PropertyId/role bindings and semantic order only; physical slots are read
/// solely when a dense upload cache is materialized.
///
/// The production seam deliberately exposes no runtime dependency mutation:
///
/// ```compile_fail,E0599
/// fn runtime_dependency_registry_is_not_mutable_compile_fail(
///     projection: &mut simthing_kernel::OverlaySpanProjection,
/// ) {
///     projection.insert_runtime_dependency();
/// }
/// ```
///
/// Descendant leaf stamping is likewise absent from the production type:
///
/// ```compile_fail,E0599
/// fn descendant_semantic_instances_cannot_be_stamped_compile_fail(
///     projection: &mut simthing_kernel::OverlaySpanProjection,
///     leaf: simthing_core::SimThingId,
/// ) {
///     projection.stamp_descendant_overlay(leaf);
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct OverlaySpanProjection {
    projection: DerivedSpanProjection<OverlayEffectiveDescriptor>,
    logical_members: Vec<SimThingId>,
    nodes: HashMap<SimThingId, OverlayNodeSnapshot>,
}

impl OverlaySpanProjection {
    pub fn compile(root: &SimThing) -> Result<Self, DerivedSpanAdmissionError> {
        Self::try_compile(root)
    }

    pub(crate) fn try_compile(root: &SimThing) -> Result<Self, DerivedSpanAdmissionError> {
        Self::compile_with_dependencies(root, Vec::new())
    }

    pub(crate) fn logical_directory(&self) -> &LogicalSubtreeDirectory {
        self.projection.directory()
    }

    fn compile_with_dependencies(
        root: &SimThing,
        mut dependencies: Vec<DerivedDependencyBinding>,
    ) -> Result<Self, DerivedSpanAdmissionError> {
        let mut logical_members = Vec::new();
        let mut nodes = HashMap::new();
        let mut ranges = Vec::new();
        collect_projection_topology(
            root,
            None,
            &mut Vec::new(),
            &mut logical_members,
            &mut nodes,
            &mut ranges,
        );
        let routed_targets = active_routed_targets(&nodes);
        dependencies.extend(build_frozen_overlay_dependencies(&nodes));

        let mut recipes: HashMap<SimThingId, Arc<Vec<OverlayProfileOp>>> = HashMap::new();
        let mut candidates: Vec<(
            LogicalRowRange,
            EffectiveProfileId,
            OverlayEffectiveDescriptor,
        )> = Vec::new();
        for (logical_row, id) in logical_members.iter().copied().enumerate() {
            let node = &nodes[&id];
            let sources = if routed_targets.contains(&id) {
                Arc::new(resolve_ordered_profile_ops(id, &nodes))
            } else {
                let inherited = node
                    .parent
                    .and_then(|parent| recipes.get(&parent).cloned())
                    .unwrap_or_else(|| Arc::new(Vec::new()));
                if node.active_overlays.is_empty() {
                    inherited
                } else {
                    let mut composed = inherited.as_ref().clone();
                    append_overlay_profile_ops(id, &node.active_overlays, &mut composed);
                    Arc::new(composed)
                }
            };
            recipes.insert(id, sources.clone());
            let descriptor = OverlayEffectiveDescriptor {
                sources,
                admitted_properties: node.admitted_properties.clone(),
            };
            let profile_id = profile_id_for(&descriptor);
            let one = LogicalRowRange::new(logical_row as u64, 1)?;
            if let Some((previous_range, previous_profile, previous_descriptor)) =
                candidates.last_mut()
            {
                if *previous_profile == profile_id && *previous_descriptor == descriptor {
                    *previous_range =
                        LogicalRowRange::new(previous_range.start(), previous_range.len() + 1)?;
                    continue;
                }
            }
            candidates.push((one, profile_id, descriptor));
        }
        let directory = LogicalSubtreeDirectory::admit(logical_members.len() as u64, ranges)?;
        let dependency_index = DerivedDependencyIndex::admit(dependencies)?;
        let seeds = candidates
            .into_iter()
            .map(|(range, profile, descriptor)| EffectiveSpanSeed::new(range, profile, descriptor))
            .collect();
        let projection = DerivedSpanProjection::admit(directory, seeds, dependency_index)?;
        Ok(Self {
            projection,
            logical_members,
            nodes,
        })
    }

    fn metrics(&self) -> OverlayProjectionMetrics {
        OverlayProjectionMetrics {
            logical_rows: self.logical_members.len() as u64,
            profiles: self.projection.profile_count() as u64,
            spans: self.projection.span_count() as u64,
        }
    }

    fn dependency_index(&self) -> &DerivedDependencyIndex {
        self.projection.dependency_index()
    }

    fn profile_digest_by_logical_identity(&self) -> Vec<(SimThingId, u64)> {
        self.logical_members
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(row, id)| {
                self.projection
                    .effective_profile_at(row as u64)
                    .map(|profile| (id, profile.digest()))
            })
            .collect()
    }

    pub fn profile_and_span_counts(&self) -> (u64, u64) {
        let metrics = self.metrics();
        (metrics.profiles, metrics.spans)
    }

    pub fn projection_counts(&self) -> (u64, u64, u64) {
        let metrics = self.metrics();
        (metrics.logical_rows, metrics.profiles, metrics.spans)
    }

    pub fn refresh(
        &mut self,
        root: &SimThing,
        changes: &[OverlayProjectionHostChange],
        generation: GenerationStamp,
    ) -> Result<(u64, u64), DerivedSpanAdmissionError> {
        let refresh = self.try_refresh(root, changes, generation)?;
        Ok((
            refresh.semantic_spans_rebuilt,
            refresh.invalidation.logical_member_rows_scanned,
        ))
    }

    pub fn refresh_with_metrics(
        &mut self,
        root: &SimThing,
        changes: &[OverlayProjectionHostChange],
        generation: GenerationStamp,
    ) -> Result<(u64, u64, u64, u64), DerivedSpanAdmissionError> {
        let refresh = self.try_refresh(root, changes, generation)?;
        Ok((
            refresh.semantic_spans_rebuilt,
            refresh.invalidation.dirty_span_ranges.len() as u64,
            refresh.invalidation.spans_examined,
            refresh.invalidation.logical_member_rows_scanned,
        ))
    }

    pub(crate) fn try_refresh(
        &mut self,
        root: &SimThing,
        changes: &[OverlayProjectionHostChange],
        generation: GenerationStamp,
    ) -> Result<OverlayProjectionRefresh, DerivedSpanAdmissionError> {
        let mut overlay_hosts = HashSet::new();
        let mut property_hosts = HashSet::new();
        let mut old_ops: HashMap<SimThingId, Vec<OverlayProfileOp>> = HashMap::new();
        let mut old_properties: HashMap<SimThingId, Vec<SimPropertyId>> = HashMap::new();
        for change in changes {
            let (id, property_only) = match *change {
                OverlayProjectionHostChange::OverlayState(id) => (id, false),
                OverlayProjectionHostChange::PropertyShape(id) => (id, true),
            };
            let snapshot = self
                .nodes
                .get(&id)
                .ok_or(DerivedSpanAdmissionError::UnknownLogicalIdentity(id))?;
            old_ops.insert(id, profile_ops_for_host(id, &snapshot.active_overlays));
            old_properties.insert(id, snapshot.admitted_properties.as_ref().clone());
            let live = node_at_projection_path(root, &snapshot.path)
                .ok_or(DerivedSpanAdmissionError::UnknownLogicalIdentity(id))?;
            if !same_overlay_dependency_shape(&snapshot.overlays, &live.overlays) {
                return Err(DerivedSpanAdmissionError::FrozenDependencyShapeChanged(id));
            }
            let replacement = self.nodes.get_mut(&id).expect("checked above");
            replacement.overlays = live.overlays.clone();
            replacement.active_overlays = live
                .overlays
                .iter()
                .filter(|overlay| overlay.is_active())
                .cloned()
                .collect();
            replacement.admitted_properties = Arc::new(sorted_property_ids(live));
            if property_only {
                property_hosts.insert(id);
            } else {
                overlay_hosts.insert(id);
            }
        }

        let mut loci = Vec::new();
        for &host in &overlay_hosts {
            let mut ops = old_ops.remove(&host).unwrap_or_default();
            ops.extend(profile_ops_for_host(
                host,
                &self.nodes[&host].active_overlays,
            ));
            for op in ops {
                let locus = ChangedLocus::new(host, op.property_id, op.role);
                if !loci.contains(&locus) {
                    loci.push(locus);
                }
            }
        }
        for &host in &property_hosts {
            let mut properties = self.nodes[&host].admitted_properties.as_ref().clone();
            if let Some(old) = old_properties.get(&host) {
                properties.extend(old.iter().copied());
            }
            properties.sort_unstable_by_key(|property| property.0);
            properties.dedup();
            for property_id in properties {
                loci.push(ChangedLocus::new(host, property_id, SubFieldRole::Amount));
            }
        }
        let invalidation = self.projection.invalidate(&loci, generation)?;

        let mut semantic_spans_rebuilt = 0u64;
        for dirty_range in invalidation.affected_ranges.iter().copied() {
            let replacements = self
                .projection
                .spans_in_range(dirty_range)
                .map(|span| {
                    let start = span.range().start().max(dirty_range.start());
                    let representative = self.logical_members[start as usize];
                    let descriptor = self.resolve_descriptor(representative);
                    (start, (profile_id_for(&descriptor), descriptor))
                })
                .collect::<HashMap<_, _>>();
            semantic_spans_rebuilt += self.projection.remap_range(
                dirty_range,
                generation,
                |range, prior, prior_profile| {
                    replacements
                        .get(&range.start())
                        .cloned()
                        .map(|(profile, descriptor)| (descriptor, profile))
                        .unwrap_or_else(|| (prior.clone(), prior_profile))
                },
            )?;
        }
        Ok(OverlayProjectionRefresh {
            invalidation,
            semantic_spans_rebuilt,
        })
    }

    pub fn materialize_dense(
        &self,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
    ) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>) {
        let dense = self.materialize_dense_internal(registry, allocator);
        (dense.deltas, dense.ranges)
    }

    pub(crate) fn materialize_dense_internal(
        &self,
        registry: &DimensionRegistry,
        allocator: &SlotAllocator,
    ) -> OverlayDenseMaterialization {
        let mut deltas = Vec::new();
        let mut ranges = vec![SlotDeltaRange::default(); allocator.capacity()];
        let mut spans = self.projection.iter_spans();
        let mut span = spans.next().expect("admitted projection has a span");
        for (logical_row, id) in self.logical_members.iter().copied().enumerate() {
            while span.range().end() <= logical_row as u64 {
                span = spans.next().expect("span coverage is complete");
            }
            let Some(slot) = allocator.slot_of(id) else {
                continue;
            };
            let descriptor = span.descriptor();
            let offset = deltas.len() as u32;
            for op in descriptor
                .sources
                .iter()
                .filter(|op| descriptor.admitted_properties.contains(&op.property_id))
            {
                let Some(range) = registry.try_column_range(op.property_id) else {
                    continue;
                };
                let Some(property) = registry.try_property(op.property_id) else {
                    continue;
                };
                let Some(col) = range.col_for_role(&op.role, &property.layout) else {
                    continue;
                };
                deltas.push(OverlayDelta {
                    col: encode_column(col),
                    op_kind: op.kind.wire(),
                    value: f32::from_bits(op.value_bits),
                    _pad: 0,
                });
            }
            ranges[slot.as_usize()] = SlotDeltaRange {
                offset,
                length: deltas.len() as u32 - offset,
            };
        }
        OverlayDenseMaterialization {
            deltas,
            ranges,
            rows_materialized: self.logical_members.len() as u64,
        }
    }

    fn resolve_descriptor(&self, target: SimThingId) -> OverlayEffectiveDescriptor {
        OverlayEffectiveDescriptor {
            sources: Arc::new(resolve_ordered_profile_ops(target, &self.nodes)),
            admitted_properties: self.nodes[&target].admitted_properties.clone(),
        }
    }
}

fn collect_projection_topology(
    node: &SimThing,
    parent: Option<SimThingId>,
    path: &mut Vec<usize>,
    members: &mut Vec<SimThingId>,
    nodes: &mut HashMap<SimThingId, OverlayNodeSnapshot>,
    ranges: &mut Vec<(SimThingId, LogicalRowRange)>,
) {
    let start = members.len() as u64;
    members.push(node.id);
    nodes.insert(
        node.id,
        OverlayNodeSnapshot {
            parent,
            path: path.clone(),
            overlays: node.overlays.clone(),
            active_overlays: node
                .overlays
                .iter()
                .filter(|overlay| overlay.is_active())
                .cloned()
                .collect(),
            admitted_properties: Arc::new(sorted_property_ids(node)),
        },
    );
    for (index, child) in node.children.iter().enumerate() {
        path.push(index);
        collect_projection_topology(child, Some(node.id), path, members, nodes, ranges);
        path.pop();
    }
    ranges.push((
        node.id,
        LogicalRowRange::new(start, members.len() as u64 - start)
            .expect("a collected subtree contains its root"),
    ));
}

fn sorted_property_ids(node: &SimThing) -> Vec<SimPropertyId> {
    let mut properties = node.properties.keys().copied().collect::<Vec<_>>();
    properties.sort_unstable_by_key(|property| property.0);
    properties
}

fn append_overlay_profile_ops(
    host: SimThingId,
    overlays: &[Overlay],
    out: &mut Vec<OverlayProfileOp>,
) {
    for overlay in overlays {
        for (role, transform) in &overlay.transform.sub_field_deltas {
            let (kind, value) = if let Some(value) = transform.as_add_literal() {
                (OverlayProfileOpKind::Add, value)
            } else if let Some(value) = transform.as_multiply_literal() {
                (OverlayProfileOpKind::Multiply, value)
            } else if let Some(value) = transform.as_set_literal() {
                (OverlayProfileOpKind::Set, value)
            } else {
                continue;
            };
            out.push(OverlayProfileOp {
                host,
                property_id: overlay.transform.property_id,
                role: role.clone(),
                kind,
                value_bits: value.to_bits(),
            });
        }
    }
}

fn profile_ops_for_host(host: SimThingId, overlays: &[Overlay]) -> Vec<OverlayProfileOp> {
    let mut out = Vec::new();
    append_overlay_profile_ops(host, overlays, &mut out);
    out
}

fn path_from_root(
    target: SimThingId,
    nodes: &HashMap<SimThingId, OverlayNodeSnapshot>,
) -> Option<Vec<SimThingId>> {
    nodes.get(&target)?;
    let mut path = vec![target];
    while let Some(parent) = nodes[path.last()?].parent {
        path.push(parent);
    }
    path.reverse();
    Some(path)
}

fn route(
    origin: SimThingId,
    target: SimThingId,
    nodes: &HashMap<SimThingId, OverlayNodeSnapshot>,
) -> Option<Vec<SimThingId>> {
    let origin_path = path_from_root(origin, nodes)?;
    let target_path = path_from_root(target, nodes)?;
    let common = origin_path
        .iter()
        .zip(&target_path)
        .take_while(|(left, right)| left == right)
        .count();
    if common == 0 {
        return None;
    }
    let mut routed = origin_path[common - 1..]
        .iter()
        .rev()
        .copied()
        .collect::<Vec<_>>();
    routed.extend(target_path[common..].iter().copied());
    Some(routed)
}

fn active_routed_targets(nodes: &HashMap<SimThingId, OverlayNodeSnapshot>) -> HashSet<SimThingId> {
    let mut routed_targets = HashSet::new();
    for node in nodes.values() {
        for overlay in &node.active_overlays {
            if !matches!(overlay.kind, OverlayKind::Instruction) {
                continue;
            }
            for &target in &overlay.affects {
                if overlay.origin == target {
                    continue;
                }
                routed_targets.insert(target);
            }
        }
    }
    routed_targets
}

fn build_frozen_overlay_dependencies(
    nodes: &HashMap<SimThingId, OverlayNodeSnapshot>,
) -> Vec<DerivedDependencyBinding> {
    use crate::derived_span_projection::DerivedDependencyTarget;

    let mut rows = HashSet::new();
    for (&host, node) in nodes {
        for &property_id in node.admitted_properties.iter() {
            rows.insert((
                ChangedLocus::new(host, property_id, SubFieldRole::Amount),
                DerivedDependencyTarget::LogicalMember(host),
            ));
        }
        for overlay in &node.overlays {
            for op in profile_ops_for_host(host, std::slice::from_ref(overlay)) {
                rows.insert((
                    ChangedLocus::new(host, op.property_id, op.role),
                    DerivedDependencyTarget::SpanRoot(host),
                ));
            }
            if !matches!(overlay.kind, OverlayKind::Instruction) {
                continue;
            }
            for &target in &overlay.affects {
                if overlay.origin == target {
                    continue;
                }
                for route_host in route(overlay.origin, target, nodes).into_iter().flatten() {
                    for policy in nodes[&route_host].overlays.iter().filter(|candidate| {
                        matches!(
                            candidate.kind,
                            OverlayKind::Policy | OverlayKind::Governance
                        )
                    }) {
                        for op in profile_ops_for_host(route_host, std::slice::from_ref(policy)) {
                            rows.insert((
                                ChangedLocus::new(route_host, op.property_id, op.role),
                                DerivedDependencyTarget::SpanRoot(target),
                            ));
                        }
                    }
                }
            }
        }
    }
    rows.into_iter()
        .map(|(locus, target)| DerivedDependencyBinding::new(locus, target))
        .collect()
}

fn same_overlay_dependency_shape(left: &[Overlay], right: &[Overlay]) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            left.id == right.id
                && left.kind == right.kind
                && left.origin == right.origin
                && left.affects == right.affects
                && left.transform == right.transform
        })
}

fn resolve_ordered_overlay_hosts(
    target: SimThingId,
    nodes: &HashMap<SimThingId, OverlayNodeSnapshot>,
) -> Vec<(SimThingId, &Overlay)> {
    let Some(target_path) = path_from_root(target, nodes) else {
        return Vec::new();
    };
    let ordinary = target_path
        .iter()
        .flat_map(|host| {
            nodes[host]
                .active_overlays
                .iter()
                .map(move |overlay| (*host, overlay))
        })
        .collect::<Vec<_>>();
    let routed = ordinary
        .iter()
        .copied()
        .filter(|(_, overlay)| {
            matches!(overlay.kind, OverlayKind::Instruction)
                && overlay.origin != target
                && overlay.affects.contains(&target)
        })
        .map(|(_, instruction)| {
            let policies = route(instruction.origin, target, nodes)
                .into_iter()
                .flatten()
                .flat_map(|host| {
                    nodes[&host]
                        .active_overlays
                        .iter()
                        .filter(|overlay| {
                            matches!(overlay.kind, OverlayKind::Policy | OverlayKind::Governance)
                        })
                        .map(move |overlay| (host, overlay))
                })
                .collect::<Vec<_>>();
            (instruction.id, policies)
        })
        .collect::<Vec<_>>();
    if routed.is_empty() {
        return ordinary;
    }
    let deferred = routed
        .iter()
        .flat_map(|(_, policies)| policies.iter().map(|(_, overlay)| overlay.id))
        .collect::<HashSet<OverlayId>>();
    let mut ordered = Vec::new();
    for (host, overlay) in ordinary {
        if matches!(overlay.kind, OverlayKind::Policy | OverlayKind::Governance)
            && deferred.contains(&overlay.id)
        {
            continue;
        }
        ordered.push((host, overlay));
        if let Some((_, policies)) = routed.iter().find(|(id, _)| *id == overlay.id) {
            ordered.extend(policies.iter().copied());
        }
    }
    ordered
}

fn resolve_ordered_profile_ops(
    target: SimThingId,
    nodes: &HashMap<SimThingId, OverlayNodeSnapshot>,
) -> Vec<OverlayProfileOp> {
    let mut out = Vec::new();
    for (host, overlay) in resolve_ordered_overlay_hosts(target, nodes) {
        append_overlay_profile_ops(host, std::slice::from_ref(overlay), &mut out);
    }
    out
}

fn profile_id_for(descriptor: &OverlayEffectiveDescriptor) -> EffectiveProfileId {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    fn mix(hash: &mut u64, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= u64::from(*byte);
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
    }
    let mut hash = FNV_OFFSET;
    mix(&mut hash, b"overlay-effective-descriptor-v1");
    mix(&mut hash, &(descriptor.sources.len() as u64).to_le_bytes());
    for op in descriptor.sources.iter() {
        mix(&mut hash, &op.host.raw().to_le_bytes());
        mix(&mut hash, &op.property_id.0.to_le_bytes());
        match &op.role {
            SubFieldRole::Amount => mix(&mut hash, &[0]),
            SubFieldRole::Velocity => mix(&mut hash, &[1]),
            SubFieldRole::Intensity => mix(&mut hash, &[2]),
            SubFieldRole::Named(name) => {
                mix(&mut hash, &[3]);
                mix(&mut hash, &(name.len() as u64).to_le_bytes());
                mix(&mut hash, name.as_bytes());
            }
            SubFieldRole::Custom(name) => {
                mix(&mut hash, &[4]);
                mix(&mut hash, &(name.len() as u64).to_le_bytes());
                mix(&mut hash, name.as_bytes());
            }
        }
        mix(
            &mut hash,
            &[match op.kind {
                OverlayProfileOpKind::Add => 0,
                OverlayProfileOpKind::Multiply => 1,
                OverlayProfileOpKind::Set => 2,
            }],
        );
        mix(&mut hash, &op.value_bits.to_le_bytes());
    }
    mix(
        &mut hash,
        &(descriptor.admitted_properties.len() as u64).to_le_bytes(),
    );
    for property_id in descriptor.admitted_properties.iter() {
        mix(&mut hash, &property_id.0.to_le_bytes());
    }
    EffectiveProfileId::from_semantic_digest(hash)
}

fn node_at_projection_path<'a>(root: &'a SimThing, path: &[usize]) -> Option<&'a SimThing> {
    let mut node = root;
    for &index in path {
        node = node.children.get(index)?;
    }
    Some(node)
}

/// Build the per-tick overlay delta batch for upload to `WorldGpuState`.
///
/// Mirrors `Evaluator::evaluate_node` exactly:
///   - Ancestor transforms accumulate depth-first in push order.
///   - Local overlays are appended after ancestors (same as `TransformStack::push`).
///   - Only deltas for properties the node actually has are emitted (mirrors the
///     evaluator iterating `resolved` which contains only the node's own properties).
///   - Column resolution via `col_for_role` only (Invariant I1).
///
/// `ranges` is indexed by slot index and initialized to zero-length for all slots,
/// so slots with no overlays naturally get `length = 0` and Pass 3 skips them.
pub fn build_overlay_deltas(
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
) -> (Vec<OverlayDelta>, Vec<SlotDeltaRange>) {
    let n_slots = allocator.capacity();
    let mut deltas: Vec<OverlayDelta> = Vec::new();
    let mut ranges: Vec<SlotDeltaRange> = vec![SlotDeltaRange::default(); n_slots];

    let live_routes = LiveOverlayRoutes::for_tree(root);
    build_node(
        root,
        &[],
        live_routes.as_ref(),
        registry,
        allocator,
        &mut deltas,
        &mut ranges,
    );

    (deltas, ranges)
}

/// Recursive helper. `ancestor_transforms` carries the ordered list of
/// `PropertyTransformDelta`s accumulated from the root down to the current node's
/// parent — matching `TransformStack::deltas` at the point of recursion in the evaluator.
fn build_node(
    node: &SimThing,
    ancestor_transforms: &[PropertyTransformDelta],
    live_routes: Option<&LiveOverlayRoutes<'_>>,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    deltas: &mut Vec<OverlayDelta>,
    ranges: &mut Vec<SlotDeltaRange>,
) {
    // Compose: ancestor transforms + this node's overlay transforms, in order.
    // Mirrors: local_stack = node.overlays.iter().fold(ancestors.clone(), |s, o| s.push(...))
    let mut local_transforms: Vec<PropertyTransformDelta> = ancestor_transforms.to_vec();
    for overlay in &node.overlays {
        if !overlay.is_active() {
            continue;
        }
        local_transforms.push(overlay.transform.clone());
    }

    // Emit deltas for this node's slot (if it has one).
    if let Some(slot) = allocator.slot_of(node.id) {
        let offset = deltas.len() as u32;

        // Mirrors evaluator step 5: apply local_stack to each property the node HAS.
        // Only emit a delta if node.properties contains the transform's target property.
        if let Some(overlays) =
            live_routes.and_then(|routes| routes.ordered_active_overlays(node.id))
        {
            for overlay in overlays {
                emit_transform(node, &overlay.transform, registry, deltas);
            }
        } else {
            for transform in &local_transforms {
                emit_transform(node, transform, registry, deltas);
            }
        }

        let length = deltas.len() as u32 - offset;
        ranges[slot.as_usize()] = SlotDeltaRange { offset, length };
    }

    // Recurse children with the full local_transforms (this node's overlays included).
    // Mirrors: evaluate_node(child, &local_stack, ...)
    for child in &node.children {
        build_node(
            child,
            &local_transforms,
            live_routes,
            registry,
            allocator,
            deltas,
            ranges,
        );
    }
}

fn emit_transform(
    node: &SimThing,
    transform: &PropertyTransformDelta,
    registry: &DimensionRegistry,
    deltas: &mut Vec<OverlayDelta>,
) {
    if !node.properties.contains_key(&transform.property_id) {
        return;
    }
    let range = registry.column_range(transform.property_id);
    let layout = &registry.property(transform.property_id).layout;
    for (role, op) in &transform.sub_field_deltas {
        let Some(col) = range.col_for_role(role, layout) else {
            continue;
        };
        // Degenerate Add/Mul/Set program shapes lower to GPU OrderBands.
        // Multi-node EML stays CPU/EML-path only (zero WGSL widen).
        let (op_kind, value) = if let Some(v) = op.as_multiply_literal() {
            (OP_MULTIPLY, v)
        } else if let Some(v) = op.as_add_literal() {
            (OP_ADD, v)
        } else if let Some(v) = op.as_set_literal() {
            (OP_SET, v)
        } else {
            continue;
        };
        deltas.push(OverlayDelta {
            col: encode_column(col),
            op_kind,
            value,
            _pad: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accumulator_op::OverlayCompileCache;
    use crate::overlay_orderband::plan_overlay_orderband;
    use crate::slot::SlotAllocator;
    use crate::world_state::{OP_ADD, OP_MULTIPLY, OP_SET};
    use simthing_core::deliver_routed_overlay;
    use simthing_core::ids::OverlayId;
    use simthing_core::overlay::{Overlay, OverlayKind, OverlayLifecycle, OverlaySource};
    use simthing_core::property::{SimProperty, SubFieldRole};
    use simthing_core::{DimensionRegistry, SimThing, SimThingKind, TransformOp};
    use std::collections::BTreeMap;

    fn reg_with_loyalty() -> (DimensionRegistry, simthing_core::SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 0));
        (reg, id)
    }

    fn make_overlay(
        prop_id: simthing_core::SimPropertyId,
        deltas: Vec<(SubFieldRole, TransformOp)>,
    ) -> Overlay {
        Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::Player,
            origin: simthing_core::SimThingId::new(),
            affects: vec![],
            transform: PropertyTransformDelta {
                property_id: prop_id,
                sub_field_deltas: deltas,
            },
            lifecycle: OverlayLifecycle::UntilDissolved,
        }
    }

    fn projection_registry() -> (DimensionRegistry, SimPropertyId) {
        let mut registry = DimensionRegistry::new();
        let property = registry.register(SimProperty::simple("span-proof", "signal", 0));
        (registry, property)
    }

    fn node_with_property(registry: &DimensionRegistry, property: SimPropertyId) -> SimThing {
        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        node.add_property(property, registry.property(property).default_value());
        node
    }

    fn projection_overlay(
        host: SimThingId,
        kind: OverlayKind,
        property: SimPropertyId,
        op: TransformOp,
    ) -> Overlay {
        let lifecycle = if matches!(kind, OverlayKind::Instruction | OverlayKind::Custom(_)) {
            OverlayLifecycle::UntilDissolvedWith {
                dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
            }
        } else {
            OverlayLifecycle::UntilDissolved
        };
        Overlay {
            id: OverlayId::new(),
            kind,
            source: OverlaySource::System,
            origin: host,
            affects: Vec::new(),
            transform: PropertyTransformDelta {
                property_id: property,
                sub_field_deltas: vec![(SubFieldRole::Amount, op)],
            },
            lifecycle,
        }
    }

    fn find_mut(root: &mut SimThing, target: SimThingId) -> &mut SimThing {
        if root.id == target {
            return root;
        }
        root.children
            .iter_mut()
            .find_map(|child| {
                if child.id == target {
                    Some(child)
                } else {
                    find_mut_optional(child, target)
                }
            })
            .expect("target in test tree")
    }

    fn find_mut_optional(root: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
        if root.id == target {
            return Some(root);
        }
        root.children
            .iter_mut()
            .find_map(|child| find_mut_optional(child, target))
    }

    fn ops_for(
        id: SimThingId,
        allocator: &SlotAllocator,
        dense: &OverlayDenseMaterialization,
    ) -> Vec<(u32, u32, u32)> {
        let range = dense.ranges[allocator.slot_of(id).unwrap().as_usize()];
        dense.deltas[range.offset as usize..(range.offset + range.length) as usize]
            .iter()
            .map(|delta| (delta.col, delta.op_kind, delta.value.to_bits()))
            .collect()
    }

    #[test]
    fn dense_materialization_is_deletable_cache_and_remaps_by_logical_identity() {
        let (registry, property) = projection_registry();
        let mut root = node_with_property(&registry, property);
        root.add_overlay(projection_overlay(
            root.id,
            OverlayKind::Policy,
            property,
            TransformOp::add(0.25),
        ));
        let child_a = node_with_property(&registry, property);
        let child_a_id = child_a.id;
        let child_b = node_with_property(&registry, property);
        let child_b_id = child_b.id;
        root.add_child(child_a);
        root.add_child(child_b);

        let projection = OverlaySpanProjection::try_compile(&root).unwrap();
        assert_eq!(projection.metrics().profiles, 1);
        assert_eq!(projection.metrics().spans, 1);
        let semantic_before = projection.profile_digest_by_logical_identity();

        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        let dense_before = projection.materialize_dense_internal(&registry, &allocator);
        let by_id_before = [
            (root.id, ops_for(root.id, &allocator, &dense_before)),
            (child_a_id, ops_for(child_a_id, &allocator, &dense_before)),
            (child_b_id, ops_for(child_b_id, &allocator, &dense_before)),
        ];
        let plan = plan_overlay_orderband(
            &dense_before.deltas,
            &dense_before.ranges,
            allocator.capacity() as u32,
        );
        let mut cache = OverlayCompileCache {
            compiled_at_revision: 1,
            projection,
            cached_deltas: dense_before.deltas.clone(),
            cached_ranges: dense_before.ranges.clone(),
            cached_n_bands: plan.n_bands,
            cached_op_buffer_uploaded_n_ops: plan.ops.len() as u32,
            compile_count: 1,
            upload_count: 1,
        };
        cache.drop_dense_materialization();
        assert!(cache.cached_deltas.is_empty());
        assert!(cache.cached_ranges.is_empty());
        assert_eq!(
            cache.projection.profile_digest_by_logical_identity(),
            semantic_before
        );
        let rebuilt = cache.rebuild_dense_materialization(&registry, &allocator);
        assert_eq!(rebuilt, dense_before);
        let rebuilt_again = cache.rebuild_dense_materialization(&registry, &allocator);
        assert_eq!(rebuilt, rebuilt_again);

        let pre = allocator.binding_table_snapshot();
        let mut ids = pre.keys().copied().collect::<Vec<_>>();
        let mut slots = pre.values().copied().collect::<Vec<_>>();
        ids.sort_unstable();
        slots.sort_unstable();
        slots.reverse();
        let assignment = ids.into_iter().zip(slots).collect::<BTreeMap<_, _>>();
        allocator
            .epoch_rebind(&assignment, &BTreeMap::new(), &BTreeMap::new())
            .unwrap();
        cache.drop_dense_materialization();
        let after_remap = cache.rebuild_dense_materialization(&registry, &allocator);
        for (id, expected) in by_id_before {
            assert_eq!(ops_for(id, &allocator, &after_remap), expected);
        }
        assert_eq!(
            cache.projection.profile_digest_by_logical_identity(),
            semantic_before
        );
    }

    #[test]
    fn profile_digest_covers_full_effective_descriptor_and_reuses_equal_descriptors() {
        let (registry, property) = projection_registry();

        // An empty operation stream does not erase the admitted-property shape.
        let mut property_shape_root = SimThing::new(SimThingKind::Cohort, 0);
        let property_shape_child = node_with_property(&registry, property);
        let property_shape_child_id = property_shape_child.id;
        property_shape_root.add_child(property_shape_child);
        let property_shape_projection =
            OverlaySpanProjection::compile(&property_shape_root).unwrap();
        let property_shape_digests = property_shape_projection
            .profile_digest_by_logical_identity()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        assert_ne!(
            property_shape_digests[&property_shape_root.id],
            property_shape_digests[&property_shape_child_id]
        );

        // An operation for a property absent from its host is still semantic
        // descriptor data. The two genuinely empty descriptors lawfully reuse
        // one identity even though they are separated by the non-applicable op.
        let mut no_applicable_root = SimThing::new(SimThingKind::Cohort, 0);
        let mut no_applicable_child = SimThing::new(SimThingKind::Cohort, 0);
        let no_applicable_child_id = no_applicable_child.id;
        no_applicable_child.add_overlay(projection_overlay(
            no_applicable_child_id,
            OverlayKind::Policy,
            property,
            TransformOp::add(0.25),
        ));
        let equal_empty_child = SimThing::new(SimThingKind::Cohort, 0);
        let equal_empty_child_id = equal_empty_child.id;
        no_applicable_root.add_child(no_applicable_child);
        no_applicable_root.add_child(equal_empty_child);
        let no_applicable_projection = OverlaySpanProjection::compile(&no_applicable_root).unwrap();
        let no_applicable_digests = no_applicable_projection
            .profile_digest_by_logical_identity()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            no_applicable_digests[&no_applicable_root.id],
            no_applicable_digests[&equal_empty_child_id],
            "equal descriptors reuse the interned profile identity"
        );
        assert_ne!(
            no_applicable_digests[&no_applicable_root.id],
            no_applicable_digests[&no_applicable_child_id],
            "a no-applicable-op descriptor must not collapse to empty"
        );
        assert_eq!(no_applicable_projection.profile_and_span_counts(), (2, 3));

        // Otherwise-equal local programs remain distinct when their semantic
        // overlay hosts differ.
        let mut host_root = SimThing::new(SimThingKind::Cohort, 0);
        let mut host_a = node_with_property(&registry, property);
        let host_a_id = host_a.id;
        host_a.add_overlay(projection_overlay(
            host_a_id,
            OverlayKind::Policy,
            property,
            TransformOp::multiply(0.5),
        ));
        let mut host_b = node_with_property(&registry, property);
        let host_b_id = host_b.id;
        host_b.add_overlay(projection_overlay(
            host_b_id,
            OverlayKind::Policy,
            property,
            TransformOp::multiply(0.5),
        ));
        host_root.add_child(host_a);
        host_root.add_child(host_b);
        let host_projection = OverlaySpanProjection::compile(&host_root).unwrap();
        let host_digests = host_projection
            .profile_digest_by_logical_identity()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        assert_ne!(host_digests[&host_a_id], host_digests[&host_b_id]);
    }

    #[test]
    fn standing_and_routed_projection_match_inheritance_oracle_after_local_split() {
        let (registry, property) = projection_registry();
        let mut root = node_with_property(&registry, property);
        let mut policy_host = node_with_property(&registry, property);
        let origin = node_with_property(&registry, property);
        let origin_id = origin.id;
        policy_host.add_child(origin);
        let policy_host_id = policy_host.id;
        let mut deferred_policy = projection_overlay(
            policy_host_id,
            OverlayKind::Policy,
            property,
            TransformOp::multiply(0.5),
        );
        let deferred_policy_id = deferred_policy.id;
        deferred_policy.lifecycle = OverlayLifecycle::Suspended {
            when_activated: Box::new(OverlayLifecycle::UntilDissolved),
        };
        policy_host.add_overlay(deferred_policy);
        let receiver = node_with_property(&registry, property);
        let receiver_id = receiver.id;
        root.add_child(policy_host);
        root.add_child(receiver);
        let instruction = projection_overlay(
            origin_id,
            OverlayKind::Instruction,
            property,
            TransformOp::add(0.4),
        );
        deliver_routed_overlay(&mut root, receiver_id, instruction).unwrap();

        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        let mut projection = OverlaySpanProjection::try_compile(&root).unwrap();
        assert!(projection.dependency_index().binding_count() > 0);
        let initial = projection.materialize_dense_internal(&registry, &allocator);
        let (oracle_deltas, oracle_ranges) = build_overlay_deltas(&root, &registry, &allocator);
        assert_eq!(initial.deltas, oracle_deltas);
        assert_eq!(initial.ranges, oracle_ranges);

        find_mut(&mut root, policy_host_id)
            .overlays
            .iter_mut()
            .find(|overlay| overlay.id == deferred_policy_id)
            .unwrap()
            .lifecycle = OverlayLifecycle::UntilDissolved;
        let refresh = projection
            .try_refresh(
                &root,
                &[OverlayProjectionHostChange::OverlayState(policy_host_id)],
                GenerationStamp::new(11),
            )
            .unwrap();
        assert_eq!(refresh.invalidation.logical_member_rows_scanned, 0);
        assert!(refresh.semantic_spans_rebuilt > 0);
        let incrementally_rebuilt = projection.materialize_dense_internal(&registry, &allocator);
        let (oracle_deltas, oracle_ranges) = build_overlay_deltas(&root, &registry, &allocator);
        assert_eq!(incrementally_rebuilt.deltas, oracle_deltas);
        assert_eq!(incrementally_rebuilt.ranges, oracle_ranges);

        find_mut(&mut root, policy_host_id).add_overlay(projection_overlay(
            policy_host_id,
            OverlayKind::Policy,
            property,
            TransformOp::add(0.1),
        ));
        assert!(matches!(
            projection.try_refresh(
                &root,
                &[OverlayProjectionHostChange::OverlayState(policy_host_id)],
                GenerationStamp::new(12),
            ),
            Err(DerivedSpanAdmissionError::FrozenDependencyShapeChanged(id)) if id == policy_host_id
        ));
    }
}
