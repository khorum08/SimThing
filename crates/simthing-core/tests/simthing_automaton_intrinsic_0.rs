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
