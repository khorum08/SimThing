//! Read-only observability query for a single SimThing.
//!
//! Answers the §13 design question "why is X high on Y?" by decomposing:
//! - Current sub-field values (from the CPU values shadow).
//! - Which overlays affect each property, attributed to ancestor vs local.
//!
//! ## Shadow fidelity
//!
//! Between day boundaries the shadow reflects CPU-side patches (after any GPU
//! row sync for RMW) and the last boundary GPU readback but not mid-tick
//! velocity integration on untouched rows. At boundary time
//! `BoundaryProtocol::execute` reads GPU values back before mutating, so calling
//! `observe` right after a boundary gives fully current values.
//! Within a day the shadow is still the right source — reading full GPU state
//! every tick is too expensive for what is fundamentally a diagnostic path.

use simthing_core::{
    DimensionRegistry, OverlayId, OverlaySource, SimPropertyId, SimThing, SimThingId,
    SubFieldRole, TransformOp,
};
use simthing_gpu::SlotAllocator;

// ── Report types ──────────────────────────────────────────────────────────────

/// Snapshot of one sub-field's current value.
#[derive(Clone, Debug)]
pub struct SubFieldObservation {
    pub role:  SubFieldRole,
    pub value: f32,
}

/// One overlay's contribution to a property on the queried SimThing.
#[derive(Clone, Debug)]
pub struct OverlayContribution {
    pub overlay_id: OverlayId,
    pub source:     OverlaySource,
    /// The transform ops this overlay applies to the property's sub-fields.
    pub deltas:     Vec<(SubFieldRole, TransformOp)>,
    /// True when the overlay lives on an ancestor node, not the queried node.
    pub inherited:  bool,
}

/// All observations for one property on the queried SimThing.
#[derive(Clone, Debug)]
pub struct PropertyObservation {
    pub property_id:   SimPropertyId,
    /// `"namespace::name"` display label.
    pub property_name: String,
    pub sub_fields:    Vec<SubFieldObservation>,
    pub overlay_contributions: Vec<OverlayContribution>,
}

/// Full observability report for one SimThing.
#[derive(Clone, Debug)]
pub struct ObservabilityReport {
    pub sim_thing_id: SimThingId,
    pub properties:   Vec<PropertyObservation>,
}

// ── Core query ────────────────────────────────────────────────────────────────

/// Build an observability report for `target` using the CPU values shadow.
///
/// Returns `None` when `target` is not in the tree or has no allocated slot.
pub fn observe(
    root:      &SimThing,
    registry:  &DimensionRegistry,
    allocator: &SlotAllocator,
    shadow:    &[f32],
    n_dims:    usize,
    target:    SimThingId,
) -> Option<ObservabilityReport> {
    let path = find_path(root, target)?;
    let node = *path.last()?;

    let slot = allocator.slot_of(node.id)? as usize;
    let base = slot * n_dims;

    let mut properties = Vec::new();

    for (&pid, _) in &node.properties {
        if !registry.is_active(pid) {
            continue;
        }
        let prop   = registry.property(pid);
        let range  = registry.column_range(pid);
        let layout = &prop.layout;

        // Sub-field values from shadow.
        let sub_fields: Vec<SubFieldObservation> = layout
            .sub_fields
            .iter()
            .filter_map(|spec| {
                let col = range.col_for_role(&spec.role, layout)?;
                Some(SubFieldObservation {
                    role:  spec.role.clone(),
                    value: shadow.get(base + col).copied().unwrap_or(0.0),
                })
            })
            .collect();

        // Overlay contributions: walk ancestor chain then local overlays.
        let mut contributions = Vec::new();
        for (depth, &ancestor) in path.iter().enumerate() {
            let inherited = depth < path.len() - 1;
            for overlay in &ancestor.overlays {
                if overlay.transform.property_id != pid {
                    continue;
                }
                contributions.push(OverlayContribution {
                    overlay_id: overlay.id,
                    source:     overlay.source.clone(),
                    deltas:     overlay.transform.sub_field_deltas.clone(),
                    inherited,
                });
            }
        }

        properties.push(PropertyObservation {
            property_id:           pid,
            property_name:         format!("{}::{}", prop.namespace, prop.name),
            sub_fields,
            overlay_contributions: contributions,
        });
    }

    Some(ObservabilityReport {
        sim_thing_id: target,
        properties,
    })
}

// ── Tree walk ─────────────────────────────────────────────────────────────────

/// Depth-first search returning the path from `root` to `target` (inclusive).
fn find_path<'a>(root: &'a SimThing, target: SimThingId) -> Option<Vec<&'a SimThing>> {
    if root.id == target {
        return Some(vec![root]);
    }
    for child in &root.children {
        if let Some(mut path) = find_path(child, target) {
            path.insert(0, root);
            return Some(path);
        }
    }
    None
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
        PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingKind, SubFieldRole,
        TransformOp,
    };
    use simthing_gpu::SlotAllocator;

    fn fixture() -> (DimensionRegistry, SimPropertyId) {
        let mut reg = DimensionRegistry::new();
        let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
        (reg, pid)
    }

    fn make_overlay(pid: SimPropertyId, role: SubFieldRole, op: TransformOp, source: OverlaySource, inherited: bool) -> Overlay {
        let _ = inherited; // only used by tests as a label
        Overlay {
            id:        OverlayId::new(),
            kind:      OverlayKind::Policy,
            source,
            affects:   vec![],
            transform: PropertyTransformDelta {
                property_id:      pid,
                sub_field_deltas: vec![(role, op)],
            },
            lifecycle: OverlayLifecycle::Permanent,
        }
    }

    #[test]
    fn observe_returns_none_for_unknown_target() {
        let (reg, _) = fixture();
        let root  = SimThing::new(SimThingKind::World, 0);
        let alloc = SlotAllocator::new();
        let ghost = SimThing::new(SimThingKind::Cohort, 0).id;
        assert!(observe(&root, &reg, &alloc, &[], 0, ghost).is_none());
    }

    #[test]
    fn observe_reports_sub_field_values_from_shadow() {
        let (reg, pid) = fixture();
        let layout  = reg.property(pid).layout.clone();
        let n_dims  = reg.total_columns;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&layout));
        let cohort_id = cohort.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let slot   = alloc.slot_of(cohort_id).unwrap() as usize;
        let mut shadow = vec![0.0f32; 2 * n_dims];

        // Set Amount (col 0 relative to property start) = 0.5.
        let range      = reg.column_range(pid);
        let amount_col = range.col_for_role(&SubFieldRole::Amount, &layout).unwrap();
        shadow[slot * n_dims + amount_col] = 0.5;

        let report = observe(&root, &reg, &alloc, &shadow, n_dims, cohort_id).unwrap();
        assert_eq!(report.sim_thing_id, cohort_id);
        assert_eq!(report.properties.len(), 1);

        let obs = &report.properties[0];
        assert_eq!(obs.property_name, "core::loyalty");
        let amount_sf = obs.sub_fields.iter().find(|sf| sf.role == SubFieldRole::Amount).unwrap();
        assert_eq!(amount_sf.value.to_bits(), 0.5f32.to_bits());
    }

    #[test]
    fn local_overlay_is_not_inherited() {
        let (reg, pid) = fixture();
        let n_dims = reg.total_columns;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        let overlay = make_overlay(pid, SubFieldRole::Amount, TransformOp::Multiply(0.9), OverlaySource::Player, false);
        let overlay_id = overlay.id;
        cohort.add_overlay(overlay);
        let cohort_id = cohort.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let shadow = vec![0.0f32; 2 * n_dims];

        let report = observe(&root, &reg, &alloc, &shadow, n_dims, cohort_id).unwrap();
        let obs = &report.properties[0];
        assert_eq!(obs.overlay_contributions.len(), 1);
        assert!(!obs.overlay_contributions[0].inherited);
        assert_eq!(obs.overlay_contributions[0].overlay_id, overlay_id);
    }

    #[test]
    fn ancestor_overlay_is_marked_inherited() {
        let (reg, pid) = fixture();
        let n_dims = reg.total_columns;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        let cohort_id = cohort.id;

        // Overlay on the parent (world), not on the cohort.
        let ancestor_overlay = make_overlay(pid, SubFieldRole::Velocity, TransformOp::Add(-0.05), OverlaySource::System, true);
        let ancestor_id = ancestor_overlay.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_overlay(ancestor_overlay);
        root.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let shadow = vec![0.0f32; 2 * n_dims];

        let report = observe(&root, &reg, &alloc, &shadow, n_dims, cohort_id).unwrap();
        let obs = &report.properties[0];
        assert_eq!(obs.overlay_contributions.len(), 1);
        assert!(obs.overlay_contributions[0].inherited, "overlay from ancestor must be flagged inherited");
        assert_eq!(obs.overlay_contributions[0].overlay_id, ancestor_id);
    }

    #[test]
    fn inherited_and_local_overlays_both_reported_in_path_order() {
        let (reg, pid) = fixture();
        let n_dims = reg.total_columns;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
        let local_overlay = make_overlay(pid, SubFieldRole::Amount, TransformOp::Set(0.8), OverlaySource::Player, false);
        let local_id = local_overlay.id;
        cohort.add_overlay(local_overlay);
        let cohort_id = cohort.id;

        let ancestor_overlay = make_overlay(pid, SubFieldRole::Velocity, TransformOp::Multiply(0.5), OverlaySource::Ai, true);
        let ancestor_id = ancestor_overlay.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_overlay(ancestor_overlay);
        root.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let shadow = vec![0.0f32; 2 * n_dims];

        let report = observe(&root, &reg, &alloc, &shadow, n_dims, cohort_id).unwrap();
        let contribs = &report.properties[0].overlay_contributions;
        assert_eq!(contribs.len(), 2);
        // Ancestor first (root is earlier in path), then local.
        assert_eq!(contribs[0].overlay_id, ancestor_id);
        assert!(contribs[0].inherited);
        assert_eq!(contribs[1].overlay_id, local_id);
        assert!(!contribs[1].inherited);
    }

    #[test]
    fn overlays_on_unrelated_properties_are_excluded() {
        let mut reg = DimensionRegistry::new();
        let pid_a = reg.register(SimProperty::simple("core", "loyalty", 0));
        let pid_b = reg.register(SimProperty::simple("core", "morale", 0));
        let n_dims = reg.total_columns;

        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        cohort.add_property(pid_a, PropertyValue::from_layout(&reg.property(pid_a).layout));
        // Overlay targets pid_b, which cohort doesn't even have — should be absent.
        let unrelated = make_overlay(pid_b, SubFieldRole::Amount, TransformOp::Add(1.0), OverlaySource::System, false);
        cohort.add_overlay(unrelated);
        let cohort_id = cohort.id;

        let mut root = SimThing::new(SimThingKind::World, 0);
        root.add_child(cohort);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&root);
        let shadow = vec![0.0f32; 2 * n_dims];

        let report = observe(&root, &reg, &alloc, &shadow, n_dims, cohort_id).unwrap();
        // Only pid_a is in the report (the node doesn't have pid_b).
        assert_eq!(report.properties.len(), 1);
        assert_eq!(report.properties[0].property_id, pid_a);
        assert!(report.properties[0].overlay_contributions.is_empty());
    }
}
