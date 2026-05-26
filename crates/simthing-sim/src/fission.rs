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

use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
use crate::tree_index::{node_at_path, node_at_path_mut};
use serde::{Deserialize, Serialize};
use simthing_core::{
    DimensionRegistry, PropertyValue, SecondaryCondition, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SimThingKindTag, SubFieldRole,
};
use simthing_gpu::{SlotAllocator, ThresholdEvent};
use std::collections::{HashMap, HashSet};

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

/// Provenance record for one fission-cloned capability subtree root.
/// Emitted per `is_capability_container` child found on the fission
/// parent and successfully cloned onto the spawned child.
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

/// Execute all fission and fusion events for one boundary.
///
/// `node_paths` must reflect `root` before any fission mutation (see
/// `tree_index::build_node_paths`). The caller rebuilds the index after
/// fission if structural mutations follow in the same boundary.
pub fn resolve_fission_fusion(
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
    let mut out = FissionOutcome::default();

    // Deduplicate fission triggers.
    let mut seen_fissions: HashSet<(SimThingId, usize)> = HashSet::new();

    for event in events {
        let Some(sem) = cpu_reg.get(event.event_kind) else {
            continue;
        };
        match sem {
            ThresholdSemantic::FissionTrigger {
                sim_thing_id,
                property_id,
                template_idx,
            } => {
                let key = (*sim_thing_id, *template_idx);
                if seen_fissions.contains(&key) {
                    out.fissions_skipped_duplicate += 1;
                    continue;
                }
                seen_fissions.insert(key);

                let stid = *sim_thing_id;
                let pid = *property_id;
                let idx = *template_idx;

                if execute_fission(
                    root,
                    registry,
                    allocator,
                    &node_paths,
                    stid,
                    pid,
                    idx,
                    values_shadow,
                    n_dims,
                    current_day,
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
            } => {
                let cid = *child_id;
                let par = *parent_id;
                let pid = *property_id;
                let idx = *template_idx;

                execute_fusion(
                    root,
                    registry,
                    allocator,
                    cid,
                    par,
                    pid,
                    idx,
                    values_shadow,
                    n_dims,
                    &mut out,
                );
            }
            _ => {}
        }
    }

    out
}

fn execute_fission(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    allocator: &mut SlotAllocator,
    node_paths: &HashMap<SimThingId, Vec<usize>>,
    stid: SimThingId,
    pid: SimPropertyId,
    template_idx: usize,
    values_shadow: &mut [f32],
    n_dims: usize,
    current_day: u32,
    out: &mut FissionOutcome,
) -> bool {
    // Verify secondary condition before mutating the tree.
    let ok = {
        let node = node_paths
            .get(&stid)
            .and_then(|path| node_at_path(root, path));
        let slot = node.and_then(|n| allocator.slot_of(n.id));
        match (node, slot) {
            (Some(_n), Some(s)) => {
                let prop = registry.property(pid);
                if template_idx >= prop.fission_templates.len() {
                    return false;
                }
                let ft = &prop.fission_templates[template_idx];
                check_secondary(
                    ft.secondary.as_ref(),
                    pid,
                    registry,
                    values_shadow,
                    s,
                    n_dims,
                )
            }
            _ => false,
        }
    };

    if !ok {
        out.fissions_skipped_secondary += 1;
        return false;
    }

    // Spawn the child.
    let prop = registry.property(pid);
    let ft = &prop.fission_templates[template_idx];
    let child_kind = kind_tag_to_kind(&ft.template.child_kind);
    let mut new_child = SimThing::new(child_kind, current_day);
    let new_id = new_child.id;
    let new_slot = allocator.alloc(new_id);

    if let Some(parent) = node_paths
        .get(&stid)
        .and_then(|path| node_at_path(root, path))
    {
        if let Some(parent_slot) = allocator.slot_of(parent.id) {
            seed_fission_child(
                parent,
                &mut new_child,
                registry,
                pid,
                parent_slot,
                new_slot,
                values_shadow,
                n_dims,
            );
        }
        if ft.template.clone_capability_children {
            let cloned_roots = clone_capability_children(
                parent,
                &mut new_child,
                allocator,
                values_shadow,
                n_dims,
                &ft.template.capability_container_kinds,
            );
            if !cloned_roots.is_empty() {
                out.cloned_capability_subtrees = true;
                out.cloned_capability_roots.extend(cloned_roots);
            }
        }
    }

    let parent = node_paths
        .get(&stid)
        .and_then(|path| node_at_path_mut(root, path));
    if let Some(p) = parent {
        p.add_child(new_child);
        out.fission_pairs.push((stid, new_id));
        out.lineage_added.push(FissionLineageRecord {
            parent_id: stid,
            child_id: new_id,
            property_id: pid,
            template_idx,
        });
        true
    } else {
        // Parent disappeared between the check and the mutation — extremely
        // unlikely but defensive.
        allocator.tombstone(new_id);
        false
    }
}

fn seed_fission_child(
    parent: &SimThing,
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
    for &prop_id in parent.properties.keys() {
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

        let mut seeded = PropertyValue {
            data: values_shadow[start..end].to_vec(),
        };
        if prop_id == activating_pid {
            if let Some(amount_off) = prop.layout.offset_of(&SubFieldRole::Amount) {
                seeded.data[amount_off] = 0.0;
            }
        }

        if child_base + range.start + seeded.data.len() <= values_shadow.len() {
            let dst = child_base + range.start;
            values_shadow[dst..dst + seeded.data.len()].copy_from_slice(&seeded.data);
        }
        child.add_property(prop_id, seeded);
    }
}

pub(crate) fn is_capability_container(kind: &SimThingKind, container_kinds: &[String]) -> bool {
    match kind {
        SimThingKind::Custom(name) => container_kinds.iter().any(|k| k == name),
        _ => false,
    }
}

/// Clone every capability container subtree from `parent` into `child`.
/// Returns one `ClonedCapabilityRoot` per cloned subtree so the driver
/// can register new `CapabilityTreeInstance`s + threshold registrations
/// (S5 follow-up — fission-spawned trees otherwise have no thresholds
/// and unlocks never fire). Empty return = no clones happened (driver
/// no-op; Approach C append remains eligible).
fn clone_capability_children(
    parent: &SimThing,
    child: &mut SimThing,
    allocator: &mut SlotAllocator,
    values_shadow: &mut [f32],
    n_dims: usize,
    container_kinds: &[String],
) -> Vec<ClonedCapabilityRoot> {
    let mut roots = Vec::new();
    for source_child in parent
        .children
        .iter()
        .filter(|node| is_capability_container(&node.kind, container_kinds))
    {
        let source_root_id = source_child.id;
        let (cloned, id_pairs, overlay_id_pairs) =
            clone_subtree_with_fresh_ids(source_child, parent.id, child.id);
        let cloned_root_id = cloned.id;
        let mut allocated_any = false;
        for (source_id, cloned_id) in id_pairs {
            let Some(source_slot) = allocator.slot_of(source_id) else {
                continue;
            };
            let cloned_slot = allocator.alloc(cloned_id);
            copy_shadow_row(source_slot, cloned_slot, values_shadow, n_dims);
            allocated_any = true;
        }
        child.add_child(cloned);
        if allocated_any {
            roots.push(ClonedCapabilityRoot {
                spawned_owner_id: child.id,
                source_root_id,
                cloned_root_id,
                overlay_id_pairs,
            });
        }
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
    let cloned = clone_subtree_with_fresh_ids_inner(
        source,
        old_owner_id,
        new_owner_id,
        &mut pairs,
        &mut overlay_pairs,
    );
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
    if remove_child_from_tree(root, child_id) {
        allocator.tombstone(child_id);
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
    let idx = parent_slot as usize * n_dims + amount_col;
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
        values_shadow.get(base + col).copied()
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

fn remove_child_from_tree(node: &mut SimThing, child_id: SimThingId) -> bool {
    if let Some(pos) = node.children.iter().position(|c| c.id == child_id) {
        node.children.remove(pos);
        return true;
    }
    for child in &mut node.children {
        if remove_child_from_tree(child, child_id) {
            return true;
        }
    }
    false
}

fn kind_tag_to_kind(tag: &SimThingKindTag) -> SimThingKind {
    match tag {
        SimThingKindTag::World => SimThingKind::World,
        SimThingKindTag::Faction => SimThingKind::Faction,
        SimThingKindTag::StarSystem => SimThingKind::StarSystem,
        SimThingKindTag::Location => SimThingKind::Location,
        SimThingKindTag::Cohort => SimThingKind::Cohort,
        SimThingKindTag::Fleet => SimThingKind::Fleet,
        SimThingKindTag::Station => SimThingKind::Station,
        SimThingKindTag::Custom(s) => SimThingKind::Custom(s.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
    use crate::tree_index::build_node_paths;
    use simthing_core::{
        DimensionRegistry, Direction, FissionTemplate, FissionThreshold, Overlay, OverlayId,
        OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, SecondaryCondition,
        SimProperty, SimThing, SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
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

    #[test]
    fn fission_spawns_child_when_secondary_met() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let pval = reg.property(pid).default_value();
        cohort.add_property(pid, pval);
        let cid = cohort.id;
        alloc.alloc(cid);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 1,
            col: 0,
            value: 0.2,
            event_kind: ek,
        }];

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

        // cohort (child[0]) now has 1 child spawned by fission
        assert_eq!(out.fissions_executed, 1);
        assert_eq!(out.fissions_skipped_secondary, 0);
        assert_eq!(root.children[0].children.len(), 1);
    }

    #[test]
    fn duplicate_fission_trigger_is_skipped() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let pval = reg.property(pid).default_value();
        cohort.add_property(pid, pval);
        let cid = cohort.id;
        alloc.alloc(cid);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        // Send the same event twice.
        let events = vec![
            simthing_gpu::ThresholdEvent {
                slot: 1,
                col: 0,
                value: 0.2,
                event_kind: ek,
            },
            simthing_gpu::ThresholdEvent {
                slot: 1,
                col: 0,
                value: 0.2,
                event_kind: ek,
            },
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
        assert_eq!(out.fissions_skipped_duplicate, 1);
        assert_eq!(root.children[0].children.len(), 1);
    }

    #[test]
    fn fission_child_inherits_parent_properties_from_shadow() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());
        let cid = cohort.id;
        let parent_slot = alloc.alloc(cid) as usize;

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let layout = reg.property(pid).layout.clone();
        let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let velocity_off = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let intensity_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();
        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 4 * n_dims];
        let parent_base = parent_slot * n_dims;
        shadow[parent_base + amount_off] = 0.24;
        shadow[parent_base + velocity_off] = -0.12;
        shadow[parent_base + intensity_off] = 0.66;

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: parent_slot as u32,
            col: amount_off as u32,
            value: 0.24,
            event_kind: ek,
        }];

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
        let child = &root.children[0].children[0];
        let seeded = child
            .property(pid)
            .expect("child inherits activating property");
        assert_eq!(seeded.data[amount_off], 0.0);
        assert_eq!(seeded.data[velocity_off].to_bits(), (-0.12f32).to_bits());
        assert_eq!(seeded.data[intensity_off].to_bits(), (0.66f32).to_bits());

        let child_slot = alloc.slot_of(child.id).unwrap() as usize;
        let child_base = child_slot * n_dims;
        assert_eq!(shadow[child_base + amount_off], 0.0);
        assert_eq!(
            shadow[child_base + velocity_off].to_bits(),
            (-0.12f32).to_bits()
        );
        assert_eq!(
            shadow[child_base + intensity_off].to_bits(),
            (0.66f32).to_bits()
        );
    }

    #[test]
    fn secondary_condition_reads_triggering_property_only() {
        let mut reg = DimensionRegistry::new();
        let mut first = make_fission_property();
        first.name = "first".into();
        first.fission_templates[0].secondary = Some(SecondaryCondition::IntensityAbove(0.8));
        let first_pid = reg.register(first);

        let mut second = make_fission_property();
        second.name = "second".into();
        second.fission_templates[0].secondary = Some(SecondaryCondition::IntensityAbove(0.8));
        let second_pid = reg.register(second);

        let mut alloc = SlotAllocator::new();
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(first_pid, reg.property(first_pid).default_value());
        cohort.add_property(second_pid, reg.property(second_pid).default_value());
        let cid = cohort.id;
        let slot = alloc.alloc(cid) as usize;

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let n_dims = reg.total_columns;
        let mut shadow = vec![0.0f32; 4 * n_dims];
        let first_intensity = reg
            .column_range(first_pid)
            .col_for_role(&SubFieldRole::Intensity, &reg.property(first_pid).layout)
            .unwrap();
        let second_intensity = reg
            .column_range(second_pid)
            .col_for_role(&SubFieldRole::Intensity, &reg.property(second_pid).layout)
            .unwrap();
        shadow[slot * n_dims + first_intensity] = 0.9;
        shadow[slot * n_dims + second_intensity] = 0.1;

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: second_pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: slot as u32,
            col: 0,
            value: 0.2,
            event_kind: ek,
        }];

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

        assert_eq!(out.fissions_executed, 0);
        assert_eq!(out.fissions_skipped_secondary, 1);
        assert!(root.children[0].children.is_empty());
    }

    #[test]
    fn fission_clone_capability_children_remaps_affects_and_copies_shadow() {
        let mut reg = DimensionRegistry::new();
        let mut prop = make_fission_property();
        prop.fission_templates[0].template.child_kind = SimThingKindTag::Faction;
        prop.fission_templates[0].template.clone_capability_children = true;
        prop.fission_templates[0]
            .template
            .capability_container_kinds
            .push("tech_tree".into());
        let pid = reg.register(prop);
        let layout = reg.property(pid).layout.clone();
        let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut faction = SimThing::new(SimThingKind::Faction, 0);
        faction.add_property(pid, reg.property(pid).default_value());
        let faction_id = faction.id;

        let mut tech_tree = SimThing::new(SimThingKind::Custom("tech_tree".into()), 0);
        tech_tree.add_property(pid, reg.property(pid).default_value());
        let tech_tree_id = tech_tree.id;
        tech_tree.add_overlay(Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::System,
            affects: vec![faction_id],
            transform: PropertyTransformDelta {
                property_id: pid,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(0.2))],
            },
            lifecycle: OverlayLifecycle::Suspended {
                when_activated: Box::new(OverlayLifecycle::Permanent),
            },
        });
        faction.add_child(tech_tree);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(faction);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let faction_slot = alloc.slot_of(faction_id).unwrap() as usize;
        let tech_slot = alloc.slot_of(tech_tree_id).unwrap() as usize;

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 16 * n_dims];
        shadow[faction_slot * n_dims + amount_off] = 0.25;
        shadow[tech_slot * n_dims + amount_off] = 0.42;

        let mut cpu_reg = ThresholdRegistry::new();
        let event_kind = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: faction_id,
            property_id: pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: faction_slot as u32,
            col: amount_off as u32,
            value: 0.25,
            event_kind,
        }];

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
        let parent_faction = &root.children[0];
        let spawned = parent_faction
            .children
            .iter()
            .find(|child| child.kind == SimThingKind::Faction)
            .expect("fission child faction");
        assert_ne!(spawned.id, faction_id);

        let cloned_tree = spawned
            .children
            .iter()
            .find(|child| child.kind == SimThingKind::Custom("tech_tree".into()))
            .expect("cloned tech tree");
        assert_ne!(cloned_tree.id, tech_tree_id);
        assert_eq!(cloned_tree.overlays[0].affects, vec![spawned.id]);

        let cloned_slot = alloc.slot_of(cloned_tree.id).unwrap() as usize;
        assert_eq!(
            shadow[cloned_slot * n_dims + amount_off].to_bits(),
            0.42f32.to_bits()
        );
    }

    #[test]
    fn clone_capability_children_empty_kinds_clones_nothing() {
        let mut reg = DimensionRegistry::new();
        let mut prop = make_fission_property();
        prop.fission_templates[0].template.child_kind = SimThingKindTag::Faction;
        prop.fission_templates[0].template.clone_capability_children = true;
        let pid = reg.register(prop);
        let layout = reg.property(pid).layout.clone();
        let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut faction = SimThing::new(SimThingKind::Faction, 0);
        faction.add_property(pid, reg.property(pid).default_value());
        let faction_id = faction.id;
        faction.add_child(SimThing::new(SimThingKind::Custom("tech_tree".into()), 0));

        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(faction);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let faction_slot = alloc.slot_of(faction_id).unwrap() as usize;

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 8 * n_dims];
        shadow[faction_slot * n_dims + amount_off] = 0.25;

        let mut cpu_reg = ThresholdRegistry::new();
        let event_kind = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: faction_id,
            property_id: pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: faction_slot as u32,
            col: amount_off as u32,
            value: 0.25,
            event_kind,
        }];

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
        let spawned = root.children[0]
            .children
            .iter()
            .find(|child| child.kind == SimThingKind::Faction)
            .expect("fission child faction");
        assert!(
            spawned.children.is_empty(),
            "empty kinds list must clone nothing"
        );
    }

    #[test]
    fn fission_emits_lineage_record_per_successful_spawn() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());
        let cid = cohort.id;
        alloc.alloc(cid);
        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id: pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 1,
            col: 0,
            value: 0.2,
            event_kind: ek,
        }];

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
        assert_eq!(out.lineage_added.len(), 1);
        let lineage = out.lineage_added[0];
        assert_eq!(lineage.parent_id, cid);
        assert_eq!(lineage.property_id, pid);
        assert_eq!(lineage.template_idx, 0);
        // child_id is the freshly spawned id; verify it's present in the tree.
        let spawned = &root.children[0].children[0];
        assert_eq!(lineage.child_id, spawned.id);
    }

    #[test]
    fn fusion_applies_scar_to_parent_amount_and_tombstones_child() {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(make_fission_property());
        // Default scar_coef = 0.05; parent amount goes 1.0 → 0.95.

        let mut parent = SimThing::new(SimThingKind::Cohort, 0);
        parent.add_property(pid, reg.property(pid).default_value());
        let parent_id = parent.id;
        let mut child = SimThing::new(SimThingKind::Cohort, 1);
        child.add_property(pid, reg.property(pid).default_value());
        let child_id = child.id;
        parent.add_child(child);
        let mut root = SimThing::new(SimThingKind::Location, 0);
        root.add_child(parent);

        let mut alloc = SlotAllocator::new();
        let root_slot = alloc.alloc(root.id);
        let parent_slot = alloc.alloc(parent_id);
        let _ = alloc.alloc(child_id);
        let _ = root_slot;

        let n_dims = reg.total_columns.max(1);
        let layout = reg.property(pid).layout.clone();
        let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut shadow = vec![0.0f32; 4 * n_dims];
        shadow[parent_slot as usize * n_dims + amount_off] = 1.0;

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FusionTrigger {
            child_id,
            parent_id,
            property_id: pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot: 0,
            col: 0,
            value: 0.9,
            event_kind: ek,
        }];

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

        assert_eq!(out.fusions_executed, 1);
        assert_eq!(out.fusions_skipped_not_found, 0);
        assert_eq!(out.lineage_removed.len(), 1);

        // Scar applied: 1.0 * (1 - 0.05) = 0.95.
        let scarred = shadow[parent_slot as usize * n_dims + amount_off];
        assert!(
            (scarred - 0.95).abs() < 1e-6,
            "expected scarred amount ≈ 0.95, got {scarred}",
        );

        // Child gone from tree + allocator.
        assert!(root.children[0].children.is_empty());
        assert!(alloc.slot_of(child_id).is_none());
    }
}
