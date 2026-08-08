//! MOVEMENT-DECISION-INGRESS-0 production session witness.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, PropertyValue, SimProperty, SimThing, SimThingId, SimThingKind,
    SubFieldRole, TransformOp,
};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    ArenaPressureBindingSpec, ArenaSpec, CommitmentEffectLifecycleSpec, CommitmentEffectSpec,
    ExplicitParticipantSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    FissionPolicySpec, GameModeSpec, MappingExecutionProfile, PressurePlacementSpec,
    PressureSourceSpec, PropertyKey, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, RegionFieldSummaryPolicySpec,
    ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy)]
struct MovementIds {
    actor: SimThingId,
    a: SimThingId,
    b: SimThingId,
    c: SimThingId,
}

fn add_potential_overlay(
    root: &mut SimThing,
    target: SimThingId,
    flow_property: simthing_core::SimPropertyId,
) {
    let origin = root.id;
    let node = find_node_mut(root, target).expect("weighted field cell");
    node.add_overlay(Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin,
        affects: vec![target],
        transform: PropertyTransformDelta {
            property_id: flow_property,
            sub_field_deltas: vec![(SubFieldRole::Named("flow".into()), TransformOp::add(2.0))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    });
}

fn find_node_mut(node: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_node_mut(child, id))
}

fn movement_fixture() -> (
    Scenario,
    GameModeSpec,
    MovementIds,
    simthing_core::SimPropertyId,
) {
    let mut registry = DimensionRegistry::new();
    simthing_driver::resource_flow_opt_in_burn_in::register_food_flow(&mut registry);
    let flow_property = registry.id_of("core", "food_flow").unwrap();
    let movement_property = registry.register(SimProperty::simple("move", "pressure", 0));
    let flow_layout = registry.property(flow_property).layout.clone();

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut a = SimThing::new(SimThingKind::Location, 0);
    let mut b = SimThing::new(SimThingKind::Location, 0);
    let mut c = SimThing::new(SimThingKind::Location, 0);
    for cell in [&mut a, &mut b, &mut c] {
        cell.add_property(flow_property, PropertyValue::from_layout(&flow_layout));
    }
    let (a_id, b_id, c_id) = (a.id, b.id, c.id);
    let mut actor = SimThing::new(SimThingKind::Cohort, 0);
    actor.add_property(
        movement_property,
        registry.property(movement_property).default_value(),
    );
    let actor_id = actor.id;
    a.add_child(actor);
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let participants = [a_id, b_id, c_id]
        .into_iter()
        .map(|id| {
            ExplicitParticipantSpec::flat(
                allocator.slot_of(id).expect("cell owns a stable row").raw(),
                id.raw(),
            )
        })
        .collect();
    let install_targets = HashMap::from([
        ("cell-a".into(), vec![a_id]),
        ("cell-b".into(), vec![b_id]),
        ("cell-c".into(), vec![c_id]),
        ("actor".into(), vec![actor_id]),
    ]);
    let scenario = Scenario {
        name: "movement_decision_ingress_0".into(),
        ticks_per_day: 1,
        max_days: 2,
        dt: 0.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    };
    let field = RegionFieldSpec {
        name: "movement_potential".into(),
        grid_size: 3,
        n_dims: 8,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::Normalized,
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.01,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 0,
            child_slot_count: 9,
            child_col: 0,
            parent_slot: 9,
            parent_col: 0,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: None,
            weight_pressure: Some(1.0),
            weight_resource: Some(0.0),
        }),
        commitment: Some(FirstSliceCommitmentSpec {
            source_formula_class: "field_urgency".into(),
            parent_slot: 9,
            urgency_col: 4,
            threshold: 0.00005,
            direction: FirstSliceCommitmentDirectionSpec::Upward,
            event_kind: 0x4d4f_5645,
            effect: Some(CommitmentEffectSpec {
                target_id: "actor".into(),
                targets_property: "move::pressure".into(),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.25))],
                lifecycle: CommitmentEffectLifecycleSpec::UntilDissolved,
                once: true,
            }),
        }),
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: RegionFieldSummaryPolicySpec::default(),
        pressure_binding: Some(ArenaPressureBindingSpec {
            arena: "food".into(),
            source: PressureSourceSpec::IntrinsicFlow,
            placements: vec![
                PressurePlacementSpec {
                    target_id: "cell-a".into(),
                    row: 1,
                    col: 1,
                },
                PressurePlacementSpec {
                    target_id: "cell-b".into(),
                    row: 1,
                    col: 2,
                },
                PressurePlacementSpec {
                    target_id: "cell-c".into(),
                    row: 2,
                    col: 1,
                },
            ],
        }),
    };
    let game_mode = GameModeSpec {
        id: "movement_decision_ingress_0".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "food".into(),
                flow_property: PropertyKey::new("core", "food_flow"),
                balance_property: None,
                max_participants: 8,
                max_coupling_fanout: 4,
                max_orderband_depth: 8,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                explicit_participants: participants,
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: Vec::new(),
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: ResourceFlowExecutionProfile::DefaultDisabled,
        region_fields: vec![field],
        mapping_execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
    };
    (
        scenario,
        game_mode,
        MovementIds {
            actor: actor_id,
            a: a_id,
            b: b_id,
            c: c_id,
        },
        flow_property,
    )
}

fn destination_with_weighted_cell(
    base: &Scenario,
    game_mode: &GameModeSpec,
    ids: MovementIds,
    flow_property: simthing_core::SimPropertyId,
    weighted: SimThingId,
) -> SimThingId {
    let mut scenario = base.clone();
    add_potential_overlay(&mut scenario.root, weighted, flow_property);
    let mut session = SimSession::open_from_spec(scenario, game_mode)
        .expect("movement session requires the production GPU path");
    assert!(session.state.accumulator_overlay_add_active);
    let actor_slot = session.proto.allocator.slot_of(ids.actor).unwrap();
    for _ in 0..2 {
        session.step_once().expect("production movement cycle");
    }
    assert_eq!(session.proto.allocator.slot_of(ids.actor), Some(actor_slot));
    assert_eq!(session.mapping_commitments.len(), 1);
    assert_eq!(
        session.mapping_commitments[0].commitment.slot(),
        if weighted == ids.b { 5 } else { 7 }
    );
    session
        .proto
        .root
        .parent_id_of(ids.actor)
        .expect("actor remains spatially attached")
}

#[test]
fn ordinary_overlay_weighting_changes_the_sealed_field_destination_without_editing_identity() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|error| error.into_inner());
    let (scenario, game_mode, ids, flow_property) = movement_fixture();
    assert_eq!(scenario.root.children[0].id, ids.a);

    let toward_b = destination_with_weighted_cell(&scenario, &game_mode, ids, flow_property, ids.b);
    let toward_c = destination_with_weighted_cell(&scenario, &game_mode, ids, flow_property, ids.c);
    assert_eq!(toward_b, ids.b);
    assert_eq!(toward_c, ids.c);
}
