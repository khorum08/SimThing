//! SIMTHING-AUTOMATON-INTRINSIC-0 synthetic contract witnesses.
//!
//! Each fixture is the smallest arbitrary tree that exposes the relevant law.
//! No shipped corpus or domain-shaped vocabulary is needed.

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    deliver_deficit_directive, deliver_predicate_broadcast, deliver_standing_directive,
    overlay_origin_structural_coord, validate_and_mint_placed_participants_by_location_id,
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, RoutedPredicate, RoutedPredicateComparison, SimProperty, SimPropertyId,
    SimThing, SimThingId, SimThingKind, StructuralCoord, StructuralGridPlacement, SubFieldRole,
    TransformOp,
};

fn registry() -> (DimensionRegistry, SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("test", "signal", 0));
    (registry, property_id)
}

fn node(kind: SimThingKind) -> SimThing {
    SimThing::new(kind, 0)
}

fn with_amount(registry: &DimensionRegistry, property_id: SimPropertyId, amount: f32) -> SimThing {
    let mut node = node(SimThingKind::Cohort);
    let property = registry.property(property_id);
    let mut value = property.default_value();
    value.set_role(&SubFieldRole::Amount, &property.layout, amount);
    node.add_property(property_id, value);
    node
}

fn overlay(
    origin: SimThingId,
    kind: OverlayKind,
    property_id: SimPropertyId,
    op: TransformOp,
) -> Overlay {
    // Policy/Governance keep unit UntilDissolved; Instruction (dispatch) needs
    // UntilDissolvedWith under EVENT-GENERATION-STAMP-0 dissolve discipline.
    let lifecycle = match kind {
        OverlayKind::Instruction | OverlayKind::Custom(_) => OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
        },
        _ => OverlayLifecycle::UntilDissolved,
    };
    Overlay {
        id: OverlayId::new(),
        kind,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        },
        lifecycle,
    }
}

fn amount(
    registry: &DimensionRegistry,
    property_id: SimPropertyId,
    root: &SimThing,
    target: SimThingId,
) -> f32 {
    let snapshot = Evaluator::new(registry, 0.0).evaluate(root, 0);
    let value = snapshot
        .get(target)
        .and_then(|entity| entity.properties.get(&property_id))
        .expect("target carries the synthetic property");
    value.get_role(
        &SubFieldRole::Amount,
        &registry.property(property_id).layout,
    )
}

fn find(root: &SimThing, target: SimThingId) -> &SimThing {
    if root.id == target {
        return root;
    }
    root.children
        .iter()
        .find_map(|child| {
            if child.id == target {
                Some(child)
            } else {
                find_optional(child, target)
            }
        })
        .expect("target is in the synthetic tree")
}

fn find_optional(root: &SimThing, target: SimThingId) -> Option<&SimThing> {
    (root.id == target).then_some(root).or_else(|| {
        root.children
            .iter()
            .find_map(|child| find_optional(child, target))
    })
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
        .expect("target is in the synthetic tree")
}

fn find_mut_optional(root: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if root.id == target {
        return Some(root);
    }
    root.children
        .iter_mut()
        .find_map(|child| find_mut_optional(child, target))
}

#[test]
fn deficit_directive_routes_origin_to_lca_to_target_and_policy_filters_it() {
    let (registry, property_id) = registry();
    let mut root = node(SimThingKind::World);
    let mut policy_host = node(SimThingKind::Location);
    let origin = node(SimThingKind::Cohort);
    let origin_id = origin.id;
    policy_host.add_child(origin);
    let target = with_amount(&registry, property_id, 0.2);
    let target_id = target.id;
    let policy_host_id = policy_host.id;
    policy_host.add_overlay(overlay(
        policy_host_id,
        OverlayKind::Policy,
        property_id,
        TransformOp::multiply(0.5),
    ));
    let root_id = root.id;
    root.add_child(policy_host);
    root.add_child(target);

    let directive = overlay(
        origin_id,
        OverlayKind::Instruction,
        property_id,
        TransformOp::add(0.4),
    );
    let directive_id = directive.id;
    let mut direct_targeting_mutant = root.clone();
    let mut bypassed = directive.clone();
    bypassed.origin = target_id;
    bypassed.affects.push(target_id);
    find_mut(&mut direct_targeting_mutant, target_id).add_overlay(bypassed);
    assert_eq!(
        amount(&registry, property_id, &direct_targeting_mutant, target_id).to_bits(),
        0.6_f32.to_bits(),
        "planted flattened-route mutant must expose direct-target policy bypass"
    );

    let receipt = deliver_deficit_directive(&mut root, target_id, directive)
        .expect("both endpoints are admitted tree members");

    assert_eq!(
        receipt.route,
        vec![origin_id, policy_host_id, root_id, target_id],
        "mutation witness: bypassing the LCA route loses this exact path"
    );
    assert_eq!(find(&root, target_id).overlays.len(), 1);
    assert_eq!(
        amount(&registry, property_id, &root, target_id).to_bits(),
        0.3_f32.to_bits(),
        "mutation witness: direct target append yields 0.6, not policy-filtered 0.3"
    );

    // The policy stays live after delivery. A snapshotting implementation
    // would remain at 0.3 after this mutation.
    find_mut(&mut root, policy_host_id).overlays[0]
        .transform
        .sub_field_deltas[0]
        .1 = TransformOp::multiply(0.25);
    assert_eq!(find(&root, target_id).overlays[0].id, directive_id);
    assert_eq!(
        amount(&registry, property_id, &root, target_id).to_bits(),
        0.15_f32.to_bits(),
        "planted delivery-time snapshot must RED when live policy changes"
    );

    find_mut(&mut root, policy_host_id).overlays[0].lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::UntilDissolved),
    };
    assert_eq!(
        amount(&registry, property_id, &root, target_id).to_bits(),
        0.6_f32.to_bits(),
        "suspending policy must affect the delivered directive without re-delivery"
    );
    find_mut(&mut root, policy_host_id).overlays.clear();
    assert_eq!(find(&root, target_id).overlays.len(), 1);
    assert_eq!(
        amount(&registry, property_id, &root, target_id).to_bits(),
        0.6_f32.to_bits(),
        "dissolving policy must leave the original directive live and unfiltered"
    );
}

#[test]
fn standing_directive_is_inherited_without_descendant_copies_or_conservation_state() {
    let (registry, property_id) = registry();
    let mut root = node(SimThingKind::World);
    let root_id = root.id;
    let mut middle = node(SimThingKind::Location);
    let leaf = with_amount(&registry, property_id, 0.1);
    let leaf_id = leaf.id;
    middle.add_child(leaf);
    root.add_child(middle);

    deliver_standing_directive(
        &mut root,
        root_id,
        overlay(
            root_id,
            OverlayKind::Governance,
            property_id,
            TransformOp::add(0.25),
        ),
    )
    .expect("subtree root is admitted");

    assert_eq!(root.overlays.len(), 1);
    assert!(root.children[0].overlays.is_empty());
    assert_eq!(root.children[0].overlays.capacity(), 0);
    assert!(root.children[0].children[0].overlays.is_empty());
    assert_eq!(root.children[0].children[0].overlays.capacity(), 0);
    assert_eq!(
        amount(&registry, property_id, &root, leaf_id).to_bits(),
        0.35_f32.to_bits(),
        "mutation witness: stopping inherited TransformStack propagation loses reception"
    );
    assert_eq!(
        amount(&registry, property_id, find(&root, leaf_id), leaf_id).to_bits(),
        0.1_f32.to_bits(),
        "planted local-only evaluator mutant must fail standing reception"
    );

    event_coordinate_is_derived_from_origin_placement_not_stamped_on_overlay();
    inert_simthing_keeps_the_existing_zero_allocation_overlay_inbox();
}

#[test]
fn predicate_broadcast_is_one_subtree_walk_and_policy_rules_are_conjunctive() {
    let (registry, property_id) = registry();
    let mut root = node(SimThingKind::World);
    let origin_id = root.id;
    let accepted = with_amount(&registry, property_id, 0.75);
    let accepted_id = accepted.id;
    let ineligible = with_amount(&registry, property_id, 0.25);
    let ineligible_id = ineligible.id;

    let mut restrictive_parent = node(SimThingKind::Location);
    restrictive_parent.add_overlay(overlay(
        restrictive_parent.id,
        OverlayKind::Policy,
        property_id,
        TransformOp::set(0.0),
    ));
    let mut cannot_reopen = with_amount(&registry, property_id, 0.75);
    let cannot_reopen_id = cannot_reopen.id;
    cannot_reopen.add_overlay(overlay(
        cannot_reopen_id,
        OverlayKind::Policy,
        property_id,
        TransformOp::set(1.0),
    ));
    restrictive_parent.add_child(cannot_reopen);

    root.add_child(accepted);
    root.add_child(ineligible);
    root.add_child(restrictive_parent);
    assert_eq!(
        amount(&registry, property_id, &root, cannot_reopen_id).to_bits(),
        1.0_f32.to_bits(),
        "planted sequential-selector mutant would let the descendant reopen admission"
    );
    let template = overlay(
        origin_id,
        OverlayKind::Instruction,
        property_id,
        TransformOp::add(0.1),
    );
    let predicate = RoutedPredicate {
        property_id,
        sub_field: SubFieldRole::Amount,
        comparison: RoutedPredicateComparison::AtLeast,
        threshold: 0.5,
    };

    let receipts =
        deliver_predicate_broadcast(&mut root, origin_id, &template, &predicate, &registry)
            .expect("origin is admitted");

    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].target, accepted_id);
    assert_eq!(receipts[0].route, vec![origin_id, accepted_id]);
    assert_eq!(find(&root, accepted_id).overlays.len(), 1);
    assert!(find(&root, ineligible_id).overlays.is_empty());
    assert_eq!(
        find(&root, cannot_reopen_id).overlays.len(),
        1,
        "only the pre-existing descendant policy remains"
    );
    assert_eq!(
        amount(&registry, property_id, &root, accepted_id).to_bits(),
        0.85_f32.to_bits()
    );
    assert_eq!(
        amount(&registry, property_id, &root, cannot_reopen_id).to_bits(),
        1.0_f32.to_bits(),
        "ordinary value composition remains sequential even though selection is conjunctive"
    );

    origin_is_required_on_the_wire_without_default_or_migration_path();
}

fn origin_is_required_on_the_wire_without_default_or_migration_path() {
    let (_, property_id) = registry();
    let origin = node(SimThingKind::Cohort).id;
    let mut encoded = serde_json::to_value(overlay(
        origin,
        OverlayKind::Instruction,
        property_id,
        TransformOp::add(0.1),
    ))
    .expect("overlay serializes");
    encoded
        .as_object_mut()
        .expect("overlay is a map")
        .remove("origin");

    let error = serde_json::from_value::<Overlay>(encoded)
        .expect_err("legacy-shaped overlay must not acquire an invented origin");
    assert!(error.to_string().contains("missing field `origin`"));
}

fn event_coordinate_is_derived_from_origin_placement_not_stamped_on_overlay() {
    let (_, property_id) = registry();
    let origin = node(SimThingKind::Location).id;
    let coord = StructuralCoord::new(7, 11);
    let placed = validate_and_mint_placed_participants_by_location_id(
        &[(origin, "origin-cell")],
        &[StructuralGridPlacement {
            location_id: "origin-cell",
            coord,
        }],
    )
    .expect("placement table is complete and unique");
    let event = overlay(
        origin,
        OverlayKind::Instruction,
        property_id,
        TransformOp::add(0.1),
    );

    assert_eq!(
        overlay_origin_structural_coord(&event, &placed),
        Some(coord)
    );
}

fn inert_simthing_keeps_the_existing_zero_allocation_overlay_inbox() {
    let inert = node(SimThingKind::Location);
    assert!(inert.overlays.is_empty());
    assert_eq!(inert.overlays.capacity(), 0);
}
