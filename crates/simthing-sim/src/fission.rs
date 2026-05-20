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
//! 5. Fusion lineage threshold registration is still a follow-up; fission
//!    currently mutates and seeds the tree only.
//!
//! ## Fusion
//!
//! When a `ThresholdSemantic::FusionTrigger` fires:
//! 1. Locate parent + child by their stored ids.
//! 2. Remove the child from its parent's children list.
//! 3. Tombstone the child's slot.
//!
//! Fusion scar application and automatic fusion threshold registration are not
//! wired yet.
//!
//! ## Idempotency guard
//!
//! Multiple events can fire for the same (SimThing, template) pair in one
//! boundary tick if both threshold and secondary are met simultaneously on
//! several columns. `FissionExecutor` deduplicates by (sim_thing_id, template_idx)
//! before executing, keeping only the first.

use simthing_core::{
    DimensionRegistry, PropertyValue, SecondaryCondition, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SimThingKindTag, SubFieldRole,
};
use simthing_gpu::{SlotAllocator, ThresholdEvent};
use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};
use std::collections::HashSet;

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
}

/// Execute all fission and fusion events for one boundary.
pub fn resolve_fission_fusion(
    root:          &mut SimThing,
    registry:      &DimensionRegistry,
    allocator:     &mut SlotAllocator,
    events:        &[ThresholdEvent],
    cpu_reg:       &ThresholdRegistry,
    values_shadow: &mut [f32],
    n_dims:        usize,
    current_day:   u32,
) -> FissionOutcome {
    let mut out = FissionOutcome::default();

    // Deduplicate fission triggers.
    let mut seen_fissions: HashSet<(SimThingId, usize)> = HashSet::new();

    for event in events {
        let Some(sem) = cpu_reg.get(event.event_kind) else { continue };
        match sem {
            ThresholdSemantic::FissionTrigger { sim_thing_id, property_id, template_idx } => {
                let key = (*sim_thing_id, *template_idx);
                if seen_fissions.contains(&key) {
                    out.fissions_skipped_duplicate += 1;
                    continue;
                }
                seen_fissions.insert(key);

                let stid = *sim_thing_id;
                let pid  = *property_id;
                let idx  = *template_idx;

                if execute_fission(
                    root, registry, allocator,
                    stid, pid, idx, values_shadow, n_dims, current_day, &mut out,
                ) {
                    out.fissions_executed += 1;
                }
            }
            ThresholdSemantic::FusionTrigger { child_id, parent_id, property_id, template_idx } => {
                let cid = *child_id;
                let par = *parent_id;
                let pid = *property_id;
                let idx = *template_idx;

                execute_fusion(root, registry, allocator, cid, par, pid, idx, &mut out);
            }
            _ => {}
        }
    }

    out
}

fn execute_fission(
    root:          &mut SimThing,
    registry:      &DimensionRegistry,
    allocator:     &mut SlotAllocator,
    stid:          SimThingId,
    pid:           SimPropertyId,
    template_idx:  usize,
    values_shadow: &mut [f32],
    n_dims:        usize,
    current_day:   u32,
    out:           &mut FissionOutcome,
) -> bool {
    // Verify secondary condition before mutating the tree.
    let ok = {
        let node = find_node(root, stid);
        let slot = node.and_then(|n| allocator.slot_of(n.id));
        match (node, slot) {
            (Some(_n), Some(s)) => {
                let prop = registry.property(pid);
                if template_idx >= prop.fission_templates.len() { return false; }
                let ft = &prop.fission_templates[template_idx];
                check_secondary(ft.secondary.as_ref(), pid, registry, values_shadow, s, n_dims)
            }
            _ => false,
        }
    };

    if !ok {
        out.fissions_skipped_secondary += 1;
        return false;
    }

    // Spawn the child.
    let prop      = registry.property(pid);
    let ft        = &prop.fission_templates[template_idx];
    let child_kind = kind_tag_to_kind(&ft.template.child_kind);
    let mut new_child = SimThing::new(child_kind, current_day);
    let new_id        = new_child.id;
    let new_slot      = allocator.alloc(new_id);

    if let Some(parent) = find_node(root, stid) {
        if let Some(parent_slot) = allocator.slot_of(parent.id) {
            seed_fission_child(parent, &mut new_child, registry, pid, parent_slot, new_slot, values_shadow, n_dims);
        }
    }

    let parent = find_node_mut(root, stid);
    if let Some(p) = parent {
        p.add_child(new_child);
        out.fission_pairs.push((stid, new_id));
        true
    } else {
        // Parent disappeared between the check and the mutation — extremely
        // unlikely but defensive.
        allocator.tombstone(new_id);
        false
    }
}

fn seed_fission_child(
    parent:         &SimThing,
    child:          &mut SimThing,
    registry:       &DimensionRegistry,
    activating_pid: SimPropertyId,
    parent_slot:    u32,
    child_slot:     u32,
    values_shadow:  &mut [f32],
    n_dims:         usize,
) {
    let child_base = child_slot as usize * n_dims;
    if child_base + n_dims <= values_shadow.len() {
        values_shadow[child_base..child_base + n_dims].fill(0.0);
    }

    let parent_base = parent_slot as usize * n_dims;
    for &prop_id in parent.properties.keys() {
        if !registry.is_active(prop_id) { continue; }

        let prop  = registry.property(prop_id);
        let range = registry.column_range(prop_id);
        let start = parent_base + range.start;
        let end   = start + prop.layout.stride();
        if end > values_shadow.len() { continue; }

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

fn execute_fusion(
    root:         &mut SimThing,
    _registry:    &DimensionRegistry,
    allocator:    &mut SlotAllocator,
    child_id:     SimThingId,
    parent_id:    SimThingId,
    _pid:         SimPropertyId,
    _template_idx: usize,
    out:          &mut FissionOutcome,
) {
    // Find and remove the child from its parent's children list.
    if remove_child_from_tree(root, child_id) {
        allocator.tombstone(child_id);
        out.fusion_pairs.push((parent_id, child_id));
        out.fusions_executed += 1;
    } else {
        out.fusions_skipped_not_found += 1;
    }
}

fn check_secondary(
    secondary:     Option<&SecondaryCondition>,
    triggering_pid: SimPropertyId,
    registry:      &DimensionRegistry,
    values_shadow: &[f32],
    slot:          u32,
    n_dims:        usize,
) -> bool {
    let Some(cond) = secondary else { return true };
    let base = (slot as usize) * n_dims;

    // Helper to read amount/intensity from shadow.
    let read_role = |pid: SimPropertyId, role: &SubFieldRole| -> Option<f32> {
        if !registry.is_active(pid) { return None; }
        let range  = registry.column_range(pid);
        let layout = &registry.property(pid).layout;
        let col    = range.col_for_role(role, layout)?;
        values_shadow.get(base + col).copied()
    };

    match cond {
        SecondaryCondition::IntensityAbove(floor) => {
            read_role(triggering_pid, &SubFieldRole::Intensity)
                .map(|v| v > *floor).unwrap_or(false)
        }
        SecondaryCondition::IntensityBelow(ceil) => {
            read_role(triggering_pid, &SubFieldRole::Intensity)
                .map(|v| v < *ceil).unwrap_or(false)
        }
        SecondaryCondition::AmountAbove(floor) => {
            read_role(triggering_pid, &SubFieldRole::Amount)
                .map(|v| v > *floor).unwrap_or(false)
        }
        SecondaryCondition::AmountBelow(ceil) => {
            read_role(triggering_pid, &SubFieldRole::Amount)
                .map(|v| v < *ceil).unwrap_or(false)
        }
    }
}

fn find_node(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id { return Some(root); }
    for child in &root.children {
        if let Some(n) = find_node(child, id) { return Some(n); }
    }
    None
}

fn find_node_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id { return Some(root); }
    for child in &mut root.children {
        if let Some(n) = find_node_mut(child, id) { return Some(n); }
    }
    None
}

fn remove_child_from_tree(node: &mut SimThing, child_id: SimThingId) -> bool {
    if let Some(pos) = node.children.iter().position(|c| c.id == child_id) {
        node.children.remove(pos);
        return true;
    }
    for child in &mut node.children {
        if remove_child_from_tree(child, child_id) { return true; }
    }
    false
}

fn kind_tag_to_kind(tag: &SimThingKindTag) -> SimThingKind {
    match tag {
        SimThingKindTag::World      => SimThingKind::World,
        SimThingKindTag::Faction    => SimThingKind::Faction,
        SimThingKindTag::StarSystem => SimThingKind::StarSystem,
        SimThingKindTag::Location   => SimThingKind::Location,
        SimThingKindTag::Cohort     => SimThingKind::Cohort,
        SimThingKindTag::Fleet      => SimThingKind::Fleet,
        SimThingKindTag::Station    => SimThingKind::Station,
        SimThingKindTag::Custom(s)  => SimThingKind::Custom(s.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        Direction, DimensionRegistry, FissionTemplate, FissionThreshold, SecondaryCondition,
        SimProperty, SimThing, SimThingKind,
        SimThingKindTag, SubFieldRole,
    };
    use simthing_gpu::SlotAllocator;
    use crate::threshold_registry::{ThresholdRegistry, ThresholdSemantic};

    fn make_fission_property() -> SimProperty {
        let mut p = SimProperty::simple("core", "loyalty", 0);
        p.fission_templates = vec![FissionThreshold {
            dimension:  SimPropertyId(0),
            sub_field:  SubFieldRole::Amount,
            threshold:  0.3,
            direction:  Direction::Falling,
            template:   FissionTemplate {
                child_kind:                 SimThingKindTag::Cohort,
                fusion_intensity_threshold: 0.8,
                fusion_scar_coefficient:    0.05,
                resolution_label:           "resolved".into(),
            },
            secondary: None,
        }];
        p
    }

    #[test]
    fn fission_spawns_child_when_secondary_met() {
        let mut reg   = DimensionRegistry::new();
        let pid       = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let pval       = reg.property(pid).default_value();
        cohort.add_property(pid, pval);
        let cid = cohort.id;
        alloc.alloc(cid);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id:  pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        let events = vec![simthing_gpu::ThresholdEvent { slot: 1, col: 0, value: 0.2, event_kind: ek }];

        let out = resolve_fission_fusion(&mut root, &reg, &mut alloc, &events, &cpu_reg, &mut shadow, n_dims, 1);

        // cohort (child[0]) now has 1 child spawned by fission
        assert_eq!(out.fissions_executed, 1);
        assert_eq!(out.fissions_skipped_secondary, 0);
        assert_eq!(root.children[0].children.len(), 1);
    }

    #[test]
    fn duplicate_fission_trigger_is_skipped() {
        let mut reg   = DimensionRegistry::new();
        let pid       = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        let pval       = reg.property(pid).default_value();
        cohort.add_property(pid, pval);
        let cid = cohort.id;
        alloc.alloc(cid);

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id:  pid,
            template_idx: 0,
        });

        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 3 * n_dims];
        // Send the same event twice.
        let events = vec![
            simthing_gpu::ThresholdEvent { slot: 1, col: 0, value: 0.2, event_kind: ek },
            simthing_gpu::ThresholdEvent { slot: 1, col: 0, value: 0.2, event_kind: ek },
        ];

        let out = resolve_fission_fusion(&mut root, &reg, &mut alloc, &events, &cpu_reg, &mut shadow, n_dims, 1);

        assert_eq!(out.fissions_executed, 1);
        assert_eq!(out.fissions_skipped_duplicate, 1);
        assert_eq!(root.children[0].children.len(), 1);
    }

    #[test]
    fn fission_child_inherits_parent_properties_from_shadow() {
        let mut reg   = DimensionRegistry::new();
        let pid       = reg.register(make_fission_property());
        let mut alloc = SlotAllocator::new();

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, reg.property(pid).default_value());
        let cid = cohort.id;
        let parent_slot = alloc.alloc(cid) as usize;

        let mut root = SimThing::new(SimThingKind::Location, 0);
        alloc.alloc(root.id);
        root.add_child(cohort);

        let layout = reg.property(pid).layout.clone();
        let amount_off    = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let velocity_off  = layout.offset_of(&SubFieldRole::Velocity).unwrap();
        let intensity_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();
        let n_dims = reg.total_columns.max(1);
        let mut shadow = vec![0.0f32; 4 * n_dims];
        let parent_base = parent_slot * n_dims;
        shadow[parent_base + amount_off]    = 0.24;
        shadow[parent_base + velocity_off]  = -0.12;
        shadow[parent_base + intensity_off] = 0.66;

        let mut cpu_reg = ThresholdRegistry::new();
        let ek = cpu_reg.push(ThresholdSemantic::FissionTrigger {
            sim_thing_id: cid,
            property_id:  pid,
            template_idx: 0,
        });
        let events = vec![simthing_gpu::ThresholdEvent {
            slot:       parent_slot as u32,
            col:        amount_off as u32,
            value:      0.24,
            event_kind: ek,
        }];

        let out = resolve_fission_fusion(
            &mut root, &reg, &mut alloc, &events, &cpu_reg, &mut shadow, n_dims, 1,
        );

        assert_eq!(out.fissions_executed, 1);
        let child = &root.children[0].children[0];
        let seeded = child.property(pid).expect("child inherits activating property");
        assert_eq!(seeded.data[amount_off], 0.0);
        assert_eq!(seeded.data[velocity_off].to_bits(), (-0.12f32).to_bits());
        assert_eq!(seeded.data[intensity_off].to_bits(), (0.66f32).to_bits());

        let child_slot = alloc.slot_of(child.id).unwrap() as usize;
        let child_base = child_slot * n_dims;
        assert_eq!(shadow[child_base + amount_off], 0.0);
        assert_eq!(shadow[child_base + velocity_off].to_bits(), (-0.12f32).to_bits());
        assert_eq!(shadow[child_base + intensity_off].to_bits(), (0.66f32).to_bits());
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

        let out = resolve_fission_fusion(
            &mut root, &reg, &mut alloc, &events, &cpu_reg, &mut shadow, n_dims, 1,
        );

        assert_eq!(out.fissions_executed, 0);
        assert_eq!(out.fissions_skipped_secondary, 1);
        assert!(root.children[0].children.is_empty());
    }
}
