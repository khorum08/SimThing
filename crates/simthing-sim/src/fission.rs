//! Fission and fusion execution — step 6 of the day boundary.
//!
//! Per design_v4.md §7:
//!
//! ## Fission
//!
//! When a `ThresholdSemantic::FissionTrigger` event fires:
//! 1. Locate the parent SimThing by `sim_thing_id`.
//! 2. Check the `SecondaryCondition` (if any) against current GPU values.
//!    If the secondary condition is not met, skip (the primary threshold may
//!    have been transiently crossed; the secondary guards against false positives).
//! 3. Spawn a new `SimThing` child of `FissionTemplate::child_kind`.
//! 4. Seed the child's initial property values from the parent's GPU row.
//!    The Amount sub-field of the activating property is split: parent retains
//!    its Amount, child starts at 0 (it represents the newly-expressing force).
//! 5. Emit a `FissionLineageRecord` onto `FissionOutcome::lineage_added`.
//!    `BoundaryProtocol` accumulates it and `ThresholdBuilder::build_with_lineage`
//!    registers the child's `FusionTrigger` on the next boundary sync.
//!
//! ## Fusion
//!
//! When a `ThresholdSemantic::FusionTrigger` fires:
//! 1. Locate parent + child by their stored ids.
//! 2. Apply the fusion scar: multiply the parent's activating-property Amount
//!    by `(1 - fusion_scar_coefficient)` in the values shadow.
//! 3. Remove the child from its parent's children list.
//! 4. Tombstone the child's slot.
//! 5. Append the lineage entry to `lineage_removed` so `BoundaryProtocol`
//!    can drop it from its persistent lineage vec.
//!
//! Lineage records (`FissionLineageRecord`) are emitted by `execute_fission`
//! and consumed by `ThresholdBuilder::build_with_lineage` to register the
//! `FusionTrigger` watching the child's activating-property Intensity. Each
//! lineage entry is registered every boundary until the child fuses or one
//! of the two nodes tombstones (Remove).
//!
//! ## Idempotency guard
//!
//! Multiple events can fire for the same (SimThing, template) pair in one
//! boundary tick if both threshold and secondary are met simultaneously on
//! several columns. `FissionExecutor` deduplicates by (sim_thing_id, template_idx)
//! before executing, keeping only the first.
//!
//! **Recurring rebellions:** across days/ticks there is no suppression — if
//! Amount re-crosses the fission threshold later, a new child may spawn. That
//! is intentional (see `docs/state-authority.md`).

use crate::fission_clone_source_view::fission_clone_source_children;
use crate::growth_entitlement::{
    OrdinaryGrowthCandidate, OrdinaryGrowthOrigin, VerifiedGrowthResidencyCommit,
};
use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
use crate::tree_index::{node_at_path, node_at_path_mut};
use serde::{Deserialize, Serialize};
use simthing_core::{
    DimensionRegistry, PropertyValue, ResolvedFissionChildBlueprint, SecondaryCondition,
    SimPropertyId, SimThing, SimThingId, SubFieldRole,
};
use simthing_gpu::{GrowthResidencyCommit, SlotAllocator, ThresholdEvent};
use std::collections::{BTreeMap, HashMap, HashSet};

/// One spawned child's lineage back to its parent + activating template.
///
/// Recorded at fission time and replayed at each subsequent boundary's
/// threshold-registration step so that the child carries a `FusionTrigger`
/// registration watching its activating-property Intensity. Once fusion
/// fires (or either node tombstones), the record is dropped.
///
/// Serializable so it can be embedded in `BoundaryDeltaEntry` and survive
/// LDJSON round-trips in the replay log.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FissionLineageRecord {
    pub parent_id: SimThingId,
    pub child_id: SimThingId,
    pub property_id: SimPropertyId,
    pub template_idx: usize,
}

/// Outcome of one boundary's fission/fusion pass.
#[derive(Clone, Debug, Default)]
pub struct FissionOutcome {
    pub fissions_executed: u32,
    pub fissions_skipped_secondary: u32,
    pub fissions_skipped_duplicate: u32,
    /// Complete fission candidates that stayed U or failed physical placement.
    pub fissions_refused_entitlement: u32,
    pub fusions_executed: u32,
    pub fusions_skipped_not_found: u32,
    /// Each successful fission: `(parent_id, child_id)`.
    pub fission_pairs: Vec<(SimThingId, SimThingId)>,
    /// Each successful fusion: `(parent_id, child_id)`.
    pub fusion_pairs: Vec<(SimThingId, SimThingId)>,
    /// Full lineage records for fissions executed this boundary. The
    /// `BoundaryProtocol` appends these onto its persistent lineage vec.
    pub lineage_added: Vec<FissionLineageRecord>,
    /// Lineage records whose child fused this boundary. The
    /// `BoundaryProtocol` removes these from its persistent lineage vec.
    pub lineage_removed: Vec<FissionLineageRecord>,
    /// True if any executed fission this boundary cloned capability
    /// subtrees (`FissionTemplate.clone_capability_children` + non-empty
    /// `capability_container_kinds` produced at least one new slot under
    /// the spawned child). S5: when set, the boundary's Approach C
    /// append-only topology patch is disqualified — the patch only sees
    /// the (parent → new_child) edge from `fission_pairs` and misses the
    /// cloned subtree's internal parent→child edges. Full rebuild path
    /// is correct; conservative fix per `docs/todo.md` S5.
    pub cloned_capability_subtrees: bool,
    /// Per cloned capability subtree root: `(spawned_owner_id,
    /// source_root_id, cloned_root_id)`. The driver uses this to
    /// register new `CapabilityTreeInstance`s + threshold registrations
    /// for fission-spawned trees — otherwise unlocks on the cloned tree
    /// never fire (S5 follow-up).
    pub cloned_capability_roots: Vec<ClonedCapabilityRoot>,
}

/// Complete pre-mutation fission product. Fields stay opaque; the only public
/// observation is its entitlement candidate, and execution still requires a
/// kernel-minted residency commit.
pub struct PreparedFission {
    parent_id: SimThingId,
    property_id: SimPropertyId,
    template_idx: usize,
    child: SimThing,
    parent_slot: u32,
    parent_property_ids: Vec<SimPropertyId>,
    prepared_clones: Vec<(ClonedCapabilityRoot, Vec<(SimThingId, SimThingId)>)>,
}

impl PreparedFission {
    pub fn candidate(&self) -> OrdinaryGrowthCandidate {
        OrdinaryGrowthCandidate::new(
            self.parent_id,
            self.child.id,
            subtree_size(&self.child),
            OrdinaryGrowthOrigin::Fission,
        )
    }
}

/// Provenance record for one fission-cloned capability subtree root.
/// Emitted per resolved clone-source child found on the fission parent
/// and successfully cloned onto the spawned child.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClonedCapabilityRoot {
    /// The new SimThing the clone was attached to (i.e. the fission's
    /// spawned child).
    pub spawned_owner_id: SimThingId,
    /// The original capability-tree root the subtree was cloned from.
    pub source_root_id: SimThingId,
    /// The id of the cloned subtree root attached under `spawned_owner_id`.
    pub cloned_root_id: SimThingId,
    /// Per-overlay id remapping inside the cloned subtree:
    /// `(source_overlay_id → cloned_overlay_id)`. The driver uses this
    /// to rebuild `CapabilityTreeInstance.by_overlay` and
    /// `overlay_hosts` against the source instance's maps. Without
    /// fresh overlay ids, source and clone would share `OverlayId`s and
    /// `ActivateOverlay` would be ambiguous.
    pub overlay_id_pairs: Vec<(simthing_core::OverlayId, simthing_core::OverlayId)>,
}

/// Build every fission candidate before any ordinary growth mutation. Trigger
/// descriptors are sorted before IDs are minted, so event order cannot change
/// candidate identity or later clearing.
pub fn prepare_fission_growth_candidates(
    root: &SimThing,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    events: &[ThresholdEvent],
    cpu_reg: &ThresholdRegistry,
    values_shadow: &[f32],
    n_dims: usize,
    current_day: u32,
) -> (
    BTreeMap<(SimThingId, usize), PreparedFission>,
    FissionOutcome,
) {
    let mut out = FissionOutcome::default();
    let mut descriptors = BTreeMap::new();
    for event in events {
        let Some(sem) = cpu_reg.get(event.event_kind()) else {
            continue;
        };
        if let ThresholdSemantic::FissionTrigger {
            sim_thing_id,
            property_id,
            template_idx,
        } = sem
        {
            let key = (*sim_thing_id, *template_idx);
            if descriptors
                .insert(key, (*property_id, *template_idx))
                .is_some()
            {
                out.fissions_skipped_duplicate += 1;
            }
        }
    }
    let mut prepared = BTreeMap::new();
    for ((parent_id, template_idx), (property_id, _)) in descriptors {
        let Some(parent) = node_paths
            .get(&parent_id)
            .and_then(|path| node_at_path(root, path))
        else {
            continue;
        };
        let Some(parent_slot) = allocator.slot_of(parent.id) else {
            continue;
        };
        let prop = registry.property(property_id);
        let Some(ft) = prop.fission_templates.get(template_idx) else {
            continue;
        };
        if !check_secondary(
            ft.secondary.as_ref(),
            property_id,
            registry,
            values_shadow,
            parent_slot.raw(),
            n_dims,
        ) {
            out.fissions_skipped_secondary += 1;
            continue;
        }
        let mut child =
            ResolvedFissionChildBlueprint::from_template(&ft.template).spawn(current_day);
        let prepared_clones = if ft.template.clone_capability_children {
            prepare_capability_children(parent, &mut child, &ft.template.capability_container_kinds)
        } else {
            Vec::new()
        };
        prepared.insert(
            (parent_id, template_idx),
            PreparedFission {
                parent_id,
                property_id,
                template_idx,
                child,
                parent_slot: parent_slot.raw(),
                parent_property_ids: parent.properties.keys().copied().collect(),
                prepared_clones,
            },
        );
    }
    (prepared, out)
}

/// Execute the previously completed fission batch and ordinary fusion events.
/// Every fission attachment must consume its kernel placement commit.
pub fn resolve_prepared_fission_fusion(
    root: &mut SimThing,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    events: &[ThresholdEvent],
    cpu_reg: &ThresholdRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    mut prepared: BTreeMap<(SimThingId, usize), PreparedFission>,
    commits: &BTreeMap<SimThingId, VerifiedGrowthResidencyCommit>,
    mut out: FissionOutcome,
) -> FissionOutcome {
    let mut seen_fissions = HashSet::new();
    for event in events {
        let Some(semantic) = cpu_reg.get(event.event_kind()) else {
            continue;
        };
        match semantic {
            ThresholdSemantic::FissionTrigger {
                sim_thing_id,
                template_idx,
                ..
            } => {
                let key = (*sim_thing_id, *template_idx);
                if !seen_fissions.insert(key) {
                    continue;
                }
                let Some(candidate) = prepared.remove(&key) else {
                    continue;
                };
                let Some(commit) = commits.get(&candidate.child.id).copied() else {
                    out.fissions_refused_entitlement += 1;
                    continue;
                };
                if execute_prepared_fission(
                    root,
                    registry,
                    allocator,
                    node_paths,
                    candidate,
                    commit.commit(),
                    values_shadow,
                    n_dims,
                    &mut out,
                ) {
                    out.fissions_executed += 1;
                }
            }
            ThresholdSemantic::FusionTrigger {
                child_id,
                parent_id,
                property_id,
                template_idx,
            } => execute_fusion(
                root,
                registry,
                allocator,
                *child_id,
                *parent_id,
                *property_id,
                *template_idx,
                values_shadow,
                n_dims,
                &mut out,
            ),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn resolve_fission_fusion(
    root: &mut SimThing,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    events: &[ThresholdEvent],
    cpu_reg: &ThresholdRegistry,
    values_shadow: &mut [f32],
    n_dims: usize,
    current_day: u32,
) -> FissionOutcome {
    use simthing_core::{GenerationStamp, IntegrationSchedule};
    use simthing_gpu::{ProvisionalResidencyEntitlement, ResidencyExtent};

    let generation = GenerationStamp::new(current_day);
    let granter = root.id;
    let rows = u32::try_from(values_shadow.len() / n_dims).expect("test shadow row count fits");
    allocator
        .declare_root_residency_extent(
            granter,
            ResidencyExtent::try_new(0, rows).expect("test extent is nonempty"),
        )
        .expect("test root extent admits");
    let (prepared, out) = prepare_fission_growth_candidates(
        root,
        node_paths,
        registry,
        allocator,
        events,
        cpu_reg,
        values_shadow,
        n_dims,
        current_day,
    );
    let mut schedule = IntegrationSchedule::new();
    let commits = prepared
        .values()
        .enumerate()
        .map(|(index, candidate)| {
            let growth = candidate.candidate();
            let entitlement = ProvisionalResidencyEntitlement::try_new(
                granter,
                growth.grantee(),
                u64::try_from(index).unwrap() + 1,
                growth.quantity(),
                generation,
            )
            .unwrap();
            let commit = allocator
                .realize_unattached_growth_residency(
                    entitlement,
                    growth.structural_parent(),
                    generation,
                    &mut schedule,
                )
                .unwrap();
            (
                growth.grantee(),
                VerifiedGrowthResidencyCommit::unchecked_for_test(commit),
            )
        })
        .collect();
    resolve_prepared_fission_fusion(
        root,
        node_paths,
        registry,
        allocator,
        events,
        cpu_reg,
        values_shadow,
        n_dims,
        prepared,
        &commits,
        out,
    )
}

fn execute_prepared_fission(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    prepared: PreparedFission,
    commit: GrowthResidencyCommit,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut FissionOutcome,
) -> bool {
    let PreparedFission {
        parent_id,
        property_id,
        template_idx,
        child,
        parent_slot,
        parent_property_ids,
        prepared_clones,
    } = prepared;
    let new_id = child.id;
    let Some(parent) = node_paths
        .get(&parent_id)
        .and_then(|path| node_at_path_mut(root, path))
    else {
        return false;
    };
    parent.add_child(child);
    let attached = parent.children.last().expect("fission child just attached");
    if allocator
        .realize_growth_subtree(parent, attached, commit)
        .is_err()
    {
        parent.children.pop();
        return false;
    }

    let new_slot = allocator
        .slot_of(new_id)
        .expect("attached fission child received residency");
    let attached = parent
        .children
        .last_mut()
        .expect("admitted fission child remains attached");
    seed_fission_child(
        &parent_property_ids,
        attached,
        registry,
        property_id,
        parent_slot,
        new_slot.raw(),
        values_shadow,
        n_dims,
    );

    let mut cloned_roots = Vec::new();
    for (record, id_pairs) in prepared_clones {
        let mut allocated_any = false;
        for (source_id, cloned_id) in id_pairs {
            let Some(source_slot) = allocator.slot_of(source_id) else {
                continue;
            };
            let Some(cloned_slot) = allocator.slot_of(cloned_id) else {
                continue;
            };
            copy_shadow_row(source_slot.raw(), cloned_slot.raw(), values_shadow, n_dims);
            allocated_any = true;
        }
        if allocated_any {
            cloned_roots.push(record);
        }
    }
    if !cloned_roots.is_empty() {
        out.cloned_capability_subtrees = true;
        out.cloned_capability_roots.extend(cloned_roots);
    }

    out.fission_pairs.push((parent_id, new_id));
    out.lineage_added.push(FissionLineageRecord {
        parent_id,
        child_id: new_id,
        property_id,
        template_idx,
    });
    true
}

fn subtree_size(node: &SimThing) -> u32 {
    1u32.saturating_add(
        node.children
            .iter()
            .map(subtree_size)
            .fold(0u32, u32::saturating_add),
    )
}

fn seed_fission_child(
    parent_property_ids: &[SimPropertyId],
    child: &mut SimThing,
    registry: &DimensionRegistry,
    activating_pid: SimPropertyId,
    parent_slot: u32,
    child_slot: u32,
    values_shadow: &mut [f32],
    n_dims: usize,
) {
    let child_base = child_slot as usize * n_dims;
    if child_base + n_dims <= values_shadow.len() {
        values_shadow[child_base..child_base + n_dims].fill(0.0);
    }

    let parent_base = parent_slot as usize * n_dims;
    for &prop_id in parent_property_ids {
        if !registry.is_active(prop_id) {
            continue;
        }

        let prop = registry.property(prop_id);
        let range = registry.column_range(prop_id);
        let start = parent_base + range.start;
        let end = start + prop.layout.stride();
        if end > values_shadow.len() {
            continue;
        }

        let mut seeded = PropertyValue::from_raw_lanes(values_shadow[start..end].to_vec());
        if prop_id == activating_pid {
            if let Some(amount_off) = prop.layout.offset_of(&SubFieldRole::Amount) {
                seeded.set_lane_at_offset(amount_off, 0.0);
            }
        }

        if child_base + range.start + seeded.lane_count() <= values_shadow.len() {
            let dst = child_base + range.start;
            values_shadow[dst..dst + seeded.lane_count()]
                .copy_from_slice(seeded.raw_lanes_for_serialization());
        }
        child.add_property(prop_id, seeded);
    }
}

/// Prepare every resolved clone-source subtree inside the not-yet-attached
/// fission child. Allocation is deferred until the complete subtree has been
/// attached and admitted through its real parent relation.
/// (S5 follow-up — fission-spawned trees otherwise have no thresholds
/// and unlocks never fire). Empty return = no clones happened (driver
/// no-op; Approach C append remains eligible).
fn prepare_capability_children(
    parent: &SimThing,
    child: &mut SimThing,
    container_kinds: &[String],
) -> Vec<(ClonedCapabilityRoot, Vec<(SimThingId, SimThingId)>)> {
    let mut roots = Vec::new();
    for source_child in fission_clone_source_children(parent, container_kinds) {
        let source_root_id = source_child.id;
        let (cloned, id_pairs, overlay_id_pairs) =
            clone_subtree_with_fresh_ids(source_child, parent.id, child.id);
        let cloned_root_id = cloned.id;
        child.add_child(cloned);
        roots.push((
            ClonedCapabilityRoot {
                spawned_owner_id: child.id,
                source_root_id,
                cloned_root_id,
                overlay_id_pairs,
            },
            id_pairs,
        ));
    }
    roots
}

fn clone_subtree_with_fresh_ids(
    source: &SimThing,
    old_owner_id: SimThingId,
    new_owner_id: SimThingId,
) -> (
    SimThing,
    Vec<(SimThingId, SimThingId)>,
    Vec<(simthing_core::OverlayId, simthing_core::OverlayId)>,
) {
    let mut pairs = Vec::new();
    let mut overlay_pairs = Vec::new();
    let mut cloned = clone_subtree_with_fresh_ids_inner(
        source,
        old_owner_id,
        new_owner_id,
        &mut pairs,
        &mut overlay_pairs,
    );
    let origin_id_map: HashMap<_, _> = pairs.iter().copied().collect();
    remap_overlay_origins(&mut cloned, old_owner_id, new_owner_id, &origin_id_map);
    (cloned, pairs, overlay_pairs)
}

fn clone_subtree_with_fresh_ids_inner(
    source: &SimThing,
    old_owner_id: SimThingId,
    new_owner_id: SimThingId,
    pairs: &mut Vec<(SimThingId, SimThingId)>,
    overlay_pairs: &mut Vec<(simthing_core::OverlayId, simthing_core::OverlayId)>,
) -> SimThing {
    let mut cloned = source.clone();
    let old_id = cloned.id;
    cloned.id = SimThingId::new();
    pairs.push((old_id, cloned.id));
    // Re-stamp overlay ids so the clone owns distinct `OverlayId`s from
    // the source. Without this, `ActivateOverlay { overlay_id }` would be
    // ambiguous across source and clone subtrees (S5 follow-up).
    for overlay in &mut cloned.overlays {
        let old_oid = overlay.id;
        overlay.id = simthing_core::OverlayId::new();
        overlay_pairs.push((old_oid, overlay.id));
    }
    remap_overlay_affects(&mut cloned, old_owner_id, new_owner_id);
    cloned.children = source
        .children
        .iter()
        .map(|child| {
            clone_subtree_with_fresh_ids_inner(
                child,
                old_owner_id,
                new_owner_id,
                pairs,
                overlay_pairs,
            )
        })
        .collect();
    cloned
}

fn remap_overlay_origins(
    node: &mut SimThing,
    old_owner_id: SimThingId,
    new_owner_id: SimThingId,
    id_map: &HashMap<SimThingId, SimThingId>,
) {
    for overlay in &mut node.overlays {
        if overlay.origin == old_owner_id {
            overlay.origin = new_owner_id;
        } else if let Some(cloned_origin) = id_map.get(&overlay.origin) {
            overlay.origin = *cloned_origin;
        }
    }
    for child in &mut node.children {
        remap_overlay_origins(child, old_owner_id, new_owner_id, id_map);
    }
}

fn remap_overlay_affects(node: &mut SimThing, old_id: SimThingId, new_id: SimThingId) {
    for overlay in &mut node.overlays {
        for affected in &mut overlay.affects {
            if *affected == old_id {
                *affected = new_id;
            }
        }
    }
    for child in &mut node.children {
        remap_overlay_affects(child, old_id, new_id);
    }
}

fn copy_shadow_row(source_slot: u32, target_slot: u32, values_shadow: &mut [f32], n_dims: usize) {
    let source_base = source_slot as usize * n_dims;
    let target_base = target_slot as usize * n_dims;
    if source_base + n_dims > values_shadow.len() || target_base + n_dims > values_shadow.len() {
        return;
    }
    let row: Vec<f32> = values_shadow[source_base..source_base + n_dims].to_vec();
    values_shadow[target_base..target_base + n_dims].copy_from_slice(&row);
}

fn execute_fusion(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    child_id: SimThingId,
    parent_id: SimThingId,
    pid: SimPropertyId,
    template_idx: usize,
    values_shadow: &mut [f32],
    n_dims: usize,
    out: &mut FissionOutcome,
) {
    // Apply the scar to the parent before removing the child. The scar is a
    // permanent multiplicative reduction on the parent's activating-property
    // Amount: parent.amount *= (1.0 - fusion_scar_coefficient).
    //
    // Resolved against the registry so a tombstoned property silently no-ops
    // (matches the behavior of other shadow-touching steps).
    let scar_applied = apply_fusion_scar(
        registry,
        allocator,
        parent_id,
        pid,
        template_idx,
        values_shadow,
        n_dims,
    );

    // Find and remove the child from its parent's children list.
    if let Some(removed) = remove_child_from_tree(root, child_id) {
        allocator.release_subtree(&removed);
        out.fusion_pairs.push((parent_id, child_id));
        out.fusions_executed += 1;
        // Always record the lineage_removed entry on a successful tree mutation
        // so BoundaryProtocol can prune its persistent lineage vec, even if
        // the scar lookup couldn't resolve (defensive: tombstoned property).
        out.lineage_removed.push(FissionLineageRecord {
            parent_id,
            child_id,
            property_id: pid,
            template_idx,
        });
        let _ = scar_applied;
    } else {
        out.fusions_skipped_not_found += 1;
    }
}

/// Multiply the parent's activating-property Amount by `(1 - scar_coef)` in
/// the shadow. Returns true if the write happened, false on any lookup miss.
fn apply_fusion_scar(
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    parent_id: SimThingId,
    pid: SimPropertyId,
    template_idx: usize,
    values_shadow: &mut [f32],
    n_dims: usize,
) -> bool {
    if !registry.is_active(pid) {
        return false;
    }
    let prop = registry.property(pid);
    if template_idx >= prop.fission_templates.len() {
        return false;
    }
    let ft = &prop.fission_templates[template_idx];
    let coef = ft.template.fusion_scar_coefficient.clamp(0.0, 1.0);

    let Some(parent_slot) = allocator.slot_of(parent_id) else {
        return false;
    };
    let range = registry.column_range(pid);
    let layout = &prop.layout;
    let Some(amount_col) = range.col_for_role(&SubFieldRole::Amount, layout) else {
        return false;
    };
    let idx = parent_slot.as_usize() * n_dims + amount_col.raw();
    if idx >= values_shadow.len() {
        return false;
    }
    values_shadow[idx] *= 1.0 - coef;
    true
}

fn check_secondary(
    secondary: Option<&SecondaryCondition>,
    triggering_pid: SimPropertyId,
    registry: &DimensionRegistry,
    values_shadow: &[f32],
    slot: u32,
    n_dims: usize,
) -> bool {
    let Some(cond) = secondary else { return true };
    let base = (slot as usize) * n_dims;

    // Helper to read amount/intensity from shadow.
    let read_role = |pid: SimPropertyId, role: &SubFieldRole| -> Option<f32> {
        if !registry.is_active(pid) {
            return None;
        }
        let range = registry.column_range(pid);
        let layout = &registry.property(pid).layout;
        let col = range.col_for_role(role, layout)?;
        values_shadow.get(base + col.raw()).copied()
    };

    match cond {
        SecondaryCondition::IntensityAbove(floor) => {
            read_role(triggering_pid, &SubFieldRole::Intensity)
                .map(|v| v > *floor)
                .unwrap_or(false)
        }
        SecondaryCondition::IntensityBelow(ceil) => {
            read_role(triggering_pid, &SubFieldRole::Intensity)
                .map(|v| v < *ceil)
                .unwrap_or(false)
        }
        SecondaryCondition::AmountAbove(floor) => read_role(triggering_pid, &SubFieldRole::Amount)
            .map(|v| v > *floor)
            .unwrap_or(false),
        SecondaryCondition::AmountBelow(ceil) => read_role(triggering_pid, &SubFieldRole::Amount)
            .map(|v| v < *ceil)
            .unwrap_or(false),
    }
}

fn remove_child_from_tree(node: &mut SimThing, child_id: SimThingId) -> Option<SimThing> {
    if let Some(pos) = node.children.iter().position(|c| c.id == child_id) {
        return Some(node.children.remove(pos));
    }
    for child in &mut node.children {
        if let Some(removed) = remove_child_from_tree(child, child_id) {
            return Some(removed);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
    use crate::tree_index::build_node_paths;
    use simthing_core::{
        is_fission_clone_source, prepare_fission_clone_sources_for_registry,
        stamp_fission_clone_source_label, DimensionRegistry, Direction, FissionTemplate,
        FissionThreshold, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
        PropertyTransformDelta, ResolvedFissionChildBlueprint, SecondaryCondition, SimProperty,
        SimThing, SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
        FISSION_CLONE_SOURCE_PROPERTY_ID,
    };
    use simthing_gpu::SlotAllocator;

    fn make_fission_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", 0);
        p.fission_templates = vec![FissionThreshold {
            sub_field: SubFieldRole::Amount,
            threshold: 0.3,
            direction: Direction::Falling,
            template: FissionTemplate {
                child_kind: SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.8,
                fusion_scar_coefficient: 0.05,
                resolution_label: "resolved".into(),
                clone_capability_children: false,
                capability_container_kinds: Vec::new(),
            },
            secondary: None,
        }];
        p
    }

    fn spawned_fission_child_kind(child_kind: SimThingKindTag) -> SimThingKind {
        let mut reg = DimensionRegistry::new();
        let mut prop = make_fission_property();
        prop.fission_templates[0].template.child_kind = child_kind;
        let pid = reg.register(prop);
        let template = reg.property(pid).fission_templates[0].template.clone();
        let expected = ResolvedFissionChildBlueprint::from_template(&template)
            .spawn(1)
            .kind;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());
        let cid = cohort.id;

        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(cohort);
        let mut alloc = SlotAllocator::new();
        alloc.install_initial_tree(&root);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        let events = vec![
            crate::threshold_event_test_fixtures::fixtures::upward_crossing(1, 0, 0.2, ek, n_dims),
        ];

        let paths = build_node_paths(&root);
        let out = resolve_fission_fusion(
            &mut root,
            &paths,
            &reg,
            &mut alloc,
            &events,
            &cpu_reg,
            &mut shadow,
            n_dims,
            1,
        );

        assert_eq!(out.fissions_executed, 1);
        let spawned = &root.children[0].children[0];
        assert_eq!(spawned.kind, expected);
        spawned.kind.clone()
    }

    #[test]
    fn row_slot_object_semantics_fission_clone_routes_all_rows_through_relations() {
        let mut registry = DimensionRegistry::new();
        let mut property = make_fission_property();
        property.fission_templates[0]
            .template
            .clone_capability_children = true;
        property.fission_templates[0]
            .template
            .capability_container_kinds = vec!["capability".into()];
        let property_id = registry.register(property);

        let mut parent = SimThing::new(SimThingKind::Cohort, 0);
        parent.add_property(property_id, registry.property(property_id).default_value());
        let parent_id = parent.id;
        let mut capability = SimThing::new(SimThingKind::Custom("capability".into()), 0);
        capability.add_child(SimThing::new(SimThingKind::Cohort, 0));
        parent.add_child(capability);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(parent);
        prepare_fission_clone_sources_for_registry(&mut root, &registry);

        let mut allocator = SlotAllocator::new();
        allocator.install_initial_tree(&root);
        let mut threshold_registry = ThresholdRegistry::new();
        let event_kind = threshold_registry.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: parent_id,
            property_id,
            template_idx: 0,
        });
        let n_dims = registry.total_columns.max(1);
        let mut shadow = vec![0.0; 16 * n_dims];
        let events = vec![
            crate::threshold_event_test_fixtures::fixtures::upward_crossing(
                allocator.slot_of(parent_id).unwrap().raw(),
                0,
                0.2,
                event_kind,
                n_dims,
            ),
        ];
        let paths = build_node_paths(&root);

        let outcome = resolve_fission_fusion(
            &mut root,
            &paths,
            &registry,
            &mut allocator,
            &events,
            &threshold_registry,
            &mut shadow,
            n_dims,
            1,
        );

        assert_eq!(outcome.fissions_executed, 1);
        assert!(outcome.cloned_capability_subtrees);
        assert_eq!(outcome.cloned_capability_roots.len(), 1);
        let spawned = &root.children[0].children[1];
        let cloned = &spawned.children[0];
        assert_eq!(
            allocator.relation_of(spawned.id),
            Some(simthing_core::ObjectResidencyRelation::ChildOf(parent_id))
        );
        assert_eq!(
            allocator.relation_of(cloned.id),
            Some(simthing_core::ObjectResidencyRelation::ChildOf(spawned.id))
        );
        assert_eq!(
            allocator.relation_of(cloned.children[0].id),
            Some(simthing_core::ObjectResidencyRelation::ChildOf(cloned.id))
        );
        let request = root.children[0]
            .attached_child_residency_request(spawned)
            .expect("spawned fission child is attached to its parent");
        assert!(allocator.residency_for(&request).is_some());
    }

    fn marker_lanes(node: &SimThing) -> Option<Vec<f32>> {
        node.property(FISSION_CLONE_SOURCE_PROPERTY_ID)
            .map(|value| value.raw_lanes_for_serialization().to_vec())
    }
}
