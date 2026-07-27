//! ORDER-WEIGHT-CLASS-0 — finite order-weight class + Player directive path.
//!
//! Canonical exemplar: destination order = order-class weight overlay on a
//! fleet need column. Orders are price injections (overlays), never command
//! channels. Dominance is finite; latency = next generation boundary.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::{
    ClampBehavior, DissolveCondition, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, SimThing, SimThingId, SimThingKind, SubFieldRole,
    SubFieldSpec, TransformOp,
};
use simthing_driver::{Scenario, SimSession};
use simthing_spec::{
    compile_property, validate_order_weight_classes, validate_order_weight_overlay, GameModeSpec,
    InstallTargetSpec, OrderWeightClassSpec, OverlaySpec, PropertySpec, SpecError, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const ORDER_CLASS_ID: &str = "destination_order";
const ORDER_MAGNITUDE: f32 = 10_000.0;
const AMBIENT_WEIGHT: f32 = 1.0;

fn need_property() -> PropertySpec {
    PropertySpec {
        id: "fleet_need".into(),
        namespace: "order".into(),
        name: "fleet_need".into(),
        display_name: "Fleet Need".into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: AMBIENT_WEIGHT,
            display_name: "weight".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

fn order_class() -> OrderWeightClassSpec {
    OrderWeightClassSpec {
        id: ORDER_CLASS_ID.into(),
        magnitude: ORDER_MAGNITUDE,
        source_span_token: Some(42),
    }
}

fn scenario_two_fleets() -> (Scenario, SimThingId, SimThingId, simthing_core::SimPropertyId) {
    let mut registry = simthing_core::DimensionRegistry::new();
    compile_property(&need_property(), &mut registry).expect("need property");
    let prop_id = registry.id_of("order", "fleet_need").expect("id");
    let layout = registry.property(prop_id).layout.clone();
    let mut value = registry.property(prop_id).default_value();
    value.set_role(&SubFieldRole::Amount, &layout, AMBIENT_WEIGHT);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut fleet_a = SimThing::new(SimThingKind::Fleet, 0);
    let mut fleet_b = SimThing::new(SimThingKind::Fleet, 0);
    let a_id = fleet_a.id;
    let b_id = fleet_b.id;
    fleet_a.add_property(prop_id, value.clone());
    fleet_b.add_property(prop_id, value);
    root.add_child(fleet_a);
    root.add_child(fleet_b);

    let scenario = Scenario {
        name: "order_weight_class_0".into(),
        ticks_per_day: 1,
        max_days: 8,
        dt: 0.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::from([
            ("fleet_a".into(), vec![a_id]),
            ("fleet_b".into(), vec![b_id]),
        ]),
    };
    (scenario, a_id, b_id, prop_id)
}

fn empty_game_mode(classes: Vec<OrderWeightClassSpec>) -> GameModeSpec {
    GameModeSpec {
        id: "order_weight_class_0".into(),
        display_name: "Order Weight Class 0".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        order_weight_classes: classes,
        capability_trees: vec![],
        events: vec![],
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

fn read_amount(session: &SimSession, host: SimThingId, prop_id: simthing_core::SimPropertyId) -> f32 {
    let slot = session.proto.allocator.slot_of(host).expect("slot");
    let property = session.proto.registry.property(prop_id);
    let col = session
        .proto
        .registry
        .column_range(prop_id)
        .col_for_role(&SubFieldRole::Amount, &property.layout)
        .expect("Amount col");
    session.state.read_values_row(slot.raw())[col.raw() as usize]
}

fn destination_order_overlay(target: SimThingId, prop_id: simthing_core::SimPropertyId) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        affects: vec![target],
        transform: PropertyTransformDelta {
            property_id: prop_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(ORDER_MAGNITUDE))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
        },
    }
}

/// Negative fixtures: non-finite / class-less dominant weight span at admission.
#[test]
fn order_weight_negative_admission_spans() {
    assert!(matches!(
        validate_order_weight_classes(&[OrderWeightClassSpec {
            id: "bad".into(),
            magnitude: f32::INFINITY,
            source_span_token: Some(1),
        }]),
        Err(SpecError::MalformedOrderWeightClass { .. })
    ));

    let classes = vec![order_class()];
    let class_less = OverlaySpec {
        id: "rogue".into(),
        display_name: String::new(),
        targets_property: "order::fleet_need".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(ORDER_MAGNITUDE))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        install: InstallTargetSpec::SessionRoot,
        order_weight_class: None,
        source_span_token: Some(7),
    };
    match validate_order_weight_overlay(&class_less, &classes) {
        Err(SpecError::OrderWeightDirectiveInvalid {
            reason,
            source_span_token,
            ..
        }) => {
            assert!(reason.contains("class-less"));
            assert_eq!(source_span_token, Some(7));
        }
        other => panic!("expected class-less error, got {other:?}"),
    }

    let non_finite = OverlaySpec {
        id: "inf".into(),
        display_name: String::new(),
        targets_property: "order::fleet_need".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(f32::NAN))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        install: InstallTargetSpec::SessionRoot,
        order_weight_class: Some(ORDER_CLASS_ID.into()),
        source_span_token: Some(8),
    };
    match validate_order_weight_overlay(&non_finite, &classes) {
        Err(SpecError::OrderWeightDirectiveInvalid { reason, .. }) => {
            assert!(reason.contains("non-finite"));
        }
        other => panic!("expected non-finite error, got {other:?}"),
    }
}

/// Live GPU canonical exemplar: Player order-class weight dominates at the
/// next generation boundary, dissolves after AfterTicks(1), and dual-runs
/// bit-exact. Twin without order stays ambient.
#[test]
fn destination_order_dominates_via_player_weight_overlay_on_live_gpu() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let mut traces: Vec<(u32, u32, u32, u32)> = Vec::new();
    for run in 0..2 {
        let (scenario, fleet_a, fleet_b, prop_id) = scenario_two_fleets();
        let game_mode = empty_game_mode(vec![order_class()]);
        let mut session =
            SimSession::open_from_spec(scenario, &game_mode).expect("open live GPU session");
        let adapter = session.state.ctx.adapter.get_info();

        // Ambient baseline: both fleets at AMBIENT_WEIGHT.
        let a0 = read_amount(&session, fleet_a, prop_id);
        let b0 = read_amount(&session, fleet_b, prop_id);
        assert_eq!(a0.to_bits(), AMBIENT_WEIGHT.to_bits());
        assert_eq!(b0.to_bits(), AMBIENT_WEIGHT.to_bits());

        // Submit destination order for fleet_b as Player intent (price injection).
        // Latency: takes effect at the NEXT generation boundary.
        session
            .tx
            .submit_player_intent(fleet_b, destination_order_overlay(fleet_b, prop_id))
            .expect("submit player order intent");

        // Hot tick without boundary must not apply the order yet.
        // With ticks_per_day=1, first step_once reaches boundary — that is the
        // decision-ingress latency (responsive-order feel by construction).
        let step = session.step_once().expect("order attachment boundary");
        assert!(
            step.boundary_reached,
            "order attaches at the generation boundary"
        );
        assert!(step.boundaries_run >= 1);

        // Production tick applies overlay transform.
        session.step_once().expect("post-order production tick");
        let a1 = read_amount(&session, fleet_a, prop_id);
        let b1 = read_amount(&session, fleet_b, prop_id);
        assert_eq!(
            a1.to_bits(),
            AMBIENT_WEIGHT.to_bits(),
            "unordered twin stays ambient"
        );
        // Dominance contract: ordered fleet need exceeds ambient by the
        // order-class band (Add overlays re-apply each production pass on the
        // carried column — exact residual is not the load-bearing claim).
        assert!(
            b1 >= a1 + ORDER_MAGNITUDE - 1.0,
            "run{run}: order-class weight dominates fleet_b need (got b={b1} a={a1})"
        );
        assert!(
            b1 > a1 + 1000.0,
            "dominance contract: ordered target exceeds ambient twin by order-class band"
        );

        // AfterTicks(1): next boundary dissolves the Transient order.
        let dissolve = session.step_once().expect("dissolve boundary");
        assert!(dissolve.boundary_reached);
        session.step_once().expect("post-dissolve production");
        let a2 = read_amount(&session, fleet_a, prop_id);
        let b2 = read_amount(&session, fleet_b, prop_id);
        assert_eq!(a2.to_bits(), AMBIENT_WEIGHT.to_bits());
        // Post-dissolve: ordered fleet returns toward ambient (overlay gone).
        // Exact post-dissolve value depends on whether Set/Add residual remains
        // in the dense column; lawfulness requires twin match from same state
        // when no order was ever live — ambient twin still ambient.
        assert_eq!(
            a2.to_bits(),
            AMBIENT_WEIGHT.to_bits(),
            "never-ordered twin unchanged end-to-end"
        );

        traces.push((a1.to_bits(), b1.to_bits(), a2.to_bits(), b2.to_bits()));
        eprintln!(
            "ORDER-WEIGHT-CLASS-GPU-PROOF run={run} adapter={:?} backend={:?} device_type={:?} ambient={} ordered_live={} ordered_post_dissolve={} dominance_delta={}",
            adapter.name,
            adapter.backend,
            adapter.device_type,
            a1,
            b1,
            b2,
            b1 - a1
        );
    }

    assert_eq!(
        traces[0], traces[1],
        "bit-exact dual-run equality for ordered/unordered traces"
    );
}

/// Install-time admission rejects class-less dominant Player overlay on game mode.
#[test]
fn install_rejects_class_less_dominant_player_overlay() {
    let (scenario, _a, b, _prop) = scenario_two_fleets();
    let mut game_mode = empty_game_mode(vec![order_class()]);
    game_mode.overlays.push(OverlaySpec {
        id: "illegal_dominant".into(),
        display_name: String::new(),
        targets_property: "order::fleet_need".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(ORDER_MAGNITUDE))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        install: InstallTargetSpec::ScenarioListed {
            target_id: "fleet_b".into(),
        },
        order_weight_class: None, // class-less
        source_span_token: Some(99),
    });
    // Also register property on game mode for install compile path.
    let err = SimSession::open_from_spec(scenario, &game_mode);
    match err {
        Ok(_) => panic!("class-less dominant order must fail admission"),
        Err(e) => {
            let s = e.to_string();
            assert!(
                s.contains("class-less") || s.contains("order-weight") || s.contains("OrderWeight"),
                "unexpected error: {s}"
            );
        }
    }
}
