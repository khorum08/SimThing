//! ORDER-WEIGHT-CLASS-0 — finite order-weight class + Player directive path.
//!
//! Canonical exemplar (remand 5097206874): RF two-destination weight allocation
//! under ordinary arena normalization. Orders are price injections
//! (`OverlaySource::Player` overlays via the typed class-bound feeder path),
//! never command channels. Dominance is finite and arena-grounded
//! (class magnitude > ambient_ceiling). Latency = next generation boundary.
//! Arrival dissolve uses declarative `PropertyReaches`; twin + injection-log
//! replay prove reversibility.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DissolveCondition, LogTier, Overlay,
    OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimThing,
    SimThingId, SimThingKind, SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_driver::{
    build_execution_plan, check_conservation, flat_star_observations, resolve_node_columns,
    OrderDirectiveRequest, Scenario, SimSession,
};
use simthing_spec::{
    compile_property, validate_order_weight_classes, validate_order_weight_overlay, ArenaSpec,
    DomainPackSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec, InstallTargetSpec,
    OrderWeightClassSpec, OverlaySpec, PropertyKey, PropertySpec, ResourceFlowOptInMode,
    ResourceFlowSpec, SpecError, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const ORDER_CLASS_ID: &str = "destination_order";
/// Arena ambient weight ceiling (authored envelope for both destinations).
const AMBIENT_CEILING: f32 = 2.0;
/// Class magnitude — strictly dominates ambient under 2-way proportional allocation.
const ORDER_MAGNITUDE: f32 = 20.0;
const ROOT_INTRINSIC: f32 = 30.0;
const ARENA: &str = "order_dest";

fn order_class() -> OrderWeightClassSpec {
    OrderWeightClassSpec {
        id: ORDER_CLASS_ID.into(),
        magnitude: ORDER_MAGNITUDE,
        ambient_ceiling: AMBIENT_CEILING,
        source_span_token: Some(42),
    }
}

fn flow_subfield(name: &str, role: AccumulatorRole, default: f32) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn arrival_property() -> PropertySpec {
    PropertySpec {
        id: "arrival".into(),
        namespace: "order".into(),
        name: "arrival".into(),
        display_name: "Arrival".into(),
        description: String::new(),
        sub_fields: vec![SubFieldSpec {
            role: SubFieldRole::Amount,
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: "progress".into(),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        }],
    }
}

fn food_flow_property() -> PropertySpec {
    PropertySpec {
        id: "food_flow".into(),
        namespace: "order".into(),
        name: "food_flow".into(),
        display_name: "Food Flow".into(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow, 0.0),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: ARENA.into(),
                },
                0.0,
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: ARENA.into(),
                },
                0.0,
            ),
        ],
    }
}

/// Two destinations + root RF star. Ambient prices are Permanent System
/// Set(ambient_ceiling) overlays so post-dissolve weights re-anchor to ambient
/// (true reversibility vs residual Add).
fn rf_two_destination_fixture() -> (Scenario, GameModeSpec, SimThingId, SimThingId, SimThingId) {
    let mut registry = simthing_core::DimensionRegistry::new();
    compile_property(&food_flow_property(), &mut registry).expect("food_flow");
    compile_property(&arrival_property(), &mut registry).expect("arrival");
    let flow_id = registry.id_of("order", "food_flow").expect("flow id");
    let arrival_id = registry.id_of("order", "arrival").expect("arrival id");
    let flow_layout = registry.property(flow_id).layout.clone();
    let arrival_layout = registry.property(arrival_id).layout.clone();

    let mut flow_default = registry.property(flow_id).default_value();
    flow_default.set_role(
        &SubFieldRole::Named("weight".into()),
        &flow_layout,
        AMBIENT_CEILING,
    );
    let mut arrival_default = registry.property(arrival_id).default_value();
    arrival_default.set_role(&SubFieldRole::Amount, &arrival_layout, 0.0);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut dest_a = SimThing::new(SimThingKind::Fleet, 0);
    let mut dest_b = SimThing::new(SimThingKind::Fleet, 0);
    let a_id = dest_a.id;
    let b_id = dest_b.id;

    // Root carries intrinsic pool; leaves carry weights + allocated.
    let mut root_flow = registry.property(flow_id).default_value();
    root_flow.set_role(
        &SubFieldRole::Named("flow".into()),
        &flow_layout,
        ROOT_INTRINSIC,
    );
    root_flow.set_role(
        &SubFieldRole::Named("weight".into()),
        &flow_layout,
        0.0,
    );
    root.add_property(flow_id, root_flow);

    dest_a.add_property(flow_id, flow_default.clone());
    dest_b.add_property(flow_id, flow_default);
    // Arrival progress lives on ordered destination B (PropertyReaches dissolve).
    dest_b.add_property(arrival_id, arrival_default);
    root.add_child(dest_a);
    root.add_child(dest_b);
    let root_id = root.id;

    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let participants = [root_id, a_id, b_id]
        .into_iter()
        .map(|id| {
            ExplicitParticipantSpec::flat(
                allocator.slot_of(id).expect("slot").raw(),
                id.raw(),
            )
        })
        .collect();

    let scenario = Scenario {
        name: "order_weight_class_0_rf".into(),
        ticks_per_day: 1,
        max_days: 16,
        dt: 1.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::from([
            ("dest_a".into(), vec![a_id]),
            ("dest_b".into(), vec![b_id]),
            ("root".into(), vec![root_id]),
        ]),
    };

    // Standing ambient price policy: Permanent System Set(ambient) on each
    // destination weight so dissolve returns weights to ambient by overlay law.
    let ambient_overlay = |id: &str, target: &str| OverlaySpec {
        id: id.into(),
        display_name: String::new(),
        targets_property: "order::food_flow".into(),
        sub_field_deltas: vec![(
            SubFieldRole::Named("weight".into()),
            TransformOp::Set(AMBIENT_CEILING),
        )],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        install: InstallTargetSpec::ScenarioListed {
            target_id: target.into(),
        },
        order_weight_class: None,
        source_span_token: None,
    };
    // Standing arrival clock on dest_b: System Add(1) each production tick so
    // PropertyReaches(1.0) fires as declarative arrival after the first live
    // production under the order (generation-boundary dissolve).
    let arrival_clock = OverlaySpec {
        id: "arrival_clock".into(),
        display_name: String::new(),
        targets_property: "order::arrival".into(),
        sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        install: InstallTargetSpec::ScenarioListed {
            target_id: "dest_b".into(),
        },
        order_weight_class: None,
        source_span_token: None,
    };

    // Domain-pack standalone overlays install (game_mode.overlays are deferred).
    let pack = DomainPackSpec {
        id: "order_weight_ambient".into(),
        display_name: "Order Weight Ambient".into(),
        metadata: Default::default(),
        properties: vec![],
        overlays: vec![
            ambient_overlay("ambient_a", "dest_a"),
            ambient_overlay("ambient_b", "dest_b"),
            arrival_clock,
        ],
        capability_trees: vec![],
        events: vec![],
    };

    let game_mode = GameModeSpec {
        id: "order_weight_class_0_rf".into(),
        display_name: "Order Weight Class 0 RF".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![pack],
        // Properties already on scenario registry — avoid DuplicateProperty.
        properties: vec![],
        overlays: vec![],
        order_weight_classes: vec![order_class()],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: ARENA.into(),
                flow_property: PropertyKey::new("order", "food_flow"),
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
            couplings: vec![],
            opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    (scenario, game_mode, root_id, a_id, b_id)
}

fn read_named(
    session: &SimSession,
    host: SimThingId,
    prop_ns: &str,
    prop_name: &str,
    role: &str,
) -> f32 {
    let prop_id = session
        .proto
        .registry
        .id_of(prop_ns, prop_name)
        .expect("prop");
    let slot = session.proto.allocator.slot_of(host).expect("slot");
    let property = session.proto.registry.property(prop_id);
    let col = session
        .proto
        .registry
        .column_range(prop_id)
        .col_for_role(&SubFieldRole::Named(role.into()), &property.layout)
        .expect("col");
    session.state.read_values_row(slot.raw())[col.raw() as usize]
}

fn read_amount(session: &SimSession, host: SimThingId, prop_ns: &str, prop_name: &str) -> f32 {
    let prop_id = session
        .proto
        .registry
        .id_of(prop_ns, prop_name)
        .expect("prop");
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

fn values_bits(session: &SimSession) -> Vec<u32> {
    session
        .state
        .read_values()
        .iter()
        .map(|v| v.to_bits())
        .collect()
}

/// Negative fixtures: non-finite / class-less dominant weight span at admission.
#[test]
fn order_weight_negative_admission_spans() {
    assert!(matches!(
        validate_order_weight_classes(&[OrderWeightClassSpec {
            id: "bad".into(),
            magnitude: f32::INFINITY,
            ambient_ceiling: 1.0,
            source_span_token: Some(1),
        }]),
        Err(SpecError::MalformedOrderWeightClass { .. })
    ));
    assert!(matches!(
        validate_order_weight_classes(&[OrderWeightClassSpec {
            id: "bad_ambient".into(),
            magnitude: 2.0,
            ambient_ceiling: 2.0,
            source_span_token: Some(2),
        }]),
        Err(SpecError::MalformedOrderWeightClass { .. })
    ));

    let classes = vec![order_class()];
    let class_less = OverlaySpec {
        id: "rogue".into(),
        display_name: String::new(),
        targets_property: "order::food_flow".into(),
        sub_field_deltas: vec![(
            SubFieldRole::Named("weight".into()),
            TransformOp::Add(ORDER_MAGNITUDE),
        )],
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
        targets_property: "order::food_flow".into(),
        sub_field_deltas: vec![(
            SubFieldRole::Named("weight".into()),
            TransformOp::Add(f32::NAN),
        )],
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
        },
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

/// Authored RON fixture: class-less dominant directive echoes parsed source_span_token.
#[test]
fn order_weight_authored_ron_class_less_span_provenance() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/order_weight_class_less_dominant.ron"
    );
    let text = std::fs::read_to_string(path).expect("read authored RON fixture");
    let game_mode: GameModeSpec = ron::from_str(&text).expect("parse authored RON GameModeSpec");
    validate_order_weight_classes(&game_mode.order_weight_classes).expect("classes ok");
    assert_eq!(game_mode.overlays.len(), 1);
    let overlay = &game_mode.overlays[0];
    match validate_order_weight_overlay(overlay, &game_mode.order_weight_classes) {
        Err(SpecError::OrderWeightDirectiveInvalid {
            overlay_id,
            reason,
            source_span_token,
        }) => {
            assert_eq!(overlay_id, "rogue_destination_order");
            assert!(
                reason.contains("class-less") && reason.contains("order::food_flow"),
                "diagnostic must name locus + class omission: {reason}"
            );
            assert_eq!(
                source_span_token,
                Some(77),
                "authored RON source_span_token must echo in the admission diagnostic"
            );
        }
        other => panic!("expected class-less spanned error from RON fixture, got {other:?}"),
    }
}

/// Live GPU canonical exemplar: class-bound Player order weight dominates RF
/// allocation at the next generation boundary, dissolves on declarative
/// PropertyReaches arrival, matches a never-ordered twin after dissolve, and
/// injection-log re-injection replays bit-exact.
#[test]
fn destination_order_dominates_via_player_weight_overlay_on_live_gpu() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    // ── Ordered branch ────────────────────────────────────────────────────
    let (scenario, game_mode, root_id, dest_a, dest_b) = rf_two_destination_fixture();
    let mut ordered =
        SimSession::open_from_spec(scenario, &game_mode).expect("open live GPU session");
    let adapter = ordered.state.ctx.adapter.get_info();
    let flow_id = ordered
        .proto
        .registry
        .id_of("order", "food_flow")
        .expect("flow");
    let arrival_id = ordered
        .proto
        .registry
        .id_of("order", "arrival")
        .expect("arrival");

    // Seed root intrinsic into dense values (property default + one production).
    // Explicit participant RF uses buffer values; ensure pool is present.
    let root_slot = ordered
        .proto
        .allocator
        .slot_of(root_id)
        .expect("root slot");
    let cols = resolve_node_columns(
        &ordered.proto.registry.property(flow_id).layout,
        ARENA,
    )
    .expect("flow columns");
    let n_dims = ordered.state.n_dims;
    let mut seed = ordered.state.read_values();
    let root_flow_idx = (root_slot.raw() * n_dims + cols.intrinsic_flow_col) as usize;
    seed[root_flow_idx] = ROOT_INTRINSIC;
    ordered
        .state
        .install_resolved_values_at_boundary(&seed);

    // Ambient baseline weights via Permanent System Set overlays (applied at
    // first production after open/sync). Advance one boundary+production so
    // ambient overlays are live before the order attaches.
    let warm = ordered.step_once().expect("warm boundary");
    assert!(warm.boundary_reached);
    ordered.step_once().expect("warm production");

    let w_a0 = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b0 = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    assert!(
        (w_a0 - AMBIENT_CEILING).abs() < 1e-3 && (w_b0 - AMBIENT_CEILING).abs() < 1e-3,
        "ambient weights must equal ambient_ceiling before order (a={w_a0} b={w_b0})"
    );

    // Class-bound directive via typed API — no raw ORDER_MAGNITUDE bypass.
    ordered
        .submit_order_directive(OrderDirectiveRequest {
            class_id: ORDER_CLASS_ID.into(),
            target: dest_b,
            property_id: flow_id,
            sub_field: SubFieldRole::Named("weight".into()),
            dissolve: DissolveCondition::PropertyReaches {
                property: arrival_id,
                sub_field: SubFieldRole::Amount,
                value: 1.0,
            },
        })
        .expect("submit class-bound order directive");

    // Raw dominant Player bypass must be rejected by the runtime gate.
    let bypass = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        affects: vec![dest_a],
        transform: PropertyTransformDelta {
            property_id: flow_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named("weight".into()),
                TransformOp::Add(ORDER_MAGNITUDE),
            )],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
        },
    };
    assert!(
        ordered
            .submit_player_intent_gated(dest_a, bypass)
            .is_err(),
        "raw dominant Player overlay must not bypass class law"
    );

    // Order attaches at next generation boundary (decision-ingress latency).
    let attach = ordered.step_once().expect("order attach boundary");
    assert!(attach.boundary_reached, "order attaches at generation boundary");

    // Production: ambient Set then order Add on dest_b; RF allocates by weight.
    ordered.step_once().expect("ordered live production");
    let w_a_live = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b_live = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    let alloc_a_live = read_named(&ordered, dest_a, "order", "food_flow", "allocated");
    let alloc_b_live = read_named(&ordered, dest_b, "order", "food_flow", "allocated");
    let arrival_live = read_amount(&ordered, dest_b, "order", "arrival");

    assert!(
        (w_a_live - AMBIENT_CEILING).abs() < 1e-2,
        "unordered dest stays ambient weight (got {w_a_live})"
    );
    assert!(
        w_b_live + 1e-2 >= AMBIENT_CEILING + ORDER_MAGNITUDE
            || w_b_live + 1e-2 >= ORDER_MAGNITUDE,
        "ordered dest weight must carry class magnitude (got w_b={w_b_live})"
    );
    assert!(
        alloc_b_live > alloc_a_live,
        "ordered destination must dominate RF allocation (a={alloc_a_live} b={alloc_b_live})"
    );
    // Arena-grounded dominance: ordered share > ambient twin share under
    // proportional normalization with ambient_ceiling envelope.
    assert!(
        alloc_b_live >= alloc_a_live + 1.0,
        "dominance margin under ambient_ceiling={AMBIENT_CEILING} mag={ORDER_MAGNITUDE}"
    );
    assert!(
        arrival_live + 1e-3 >= 1.0,
        "arrival clock must reach threshold during ordered live production (got {arrival_live})"
    );

    // RF-1 conservation on live allocations.
    let leaf_slots = [
        ordered.proto.allocator.slot_of(dest_a).unwrap().raw() as u64,
        ordered.proto.allocator.slot_of(dest_b).unwrap().raw() as u64,
    ];
    let (alloc_obs, arena_obs) = flat_star_observations(
        root_slot.raw() as u64,
        &leaf_slots,
        ROOT_INTRINSIC,
        &[alloc_a_live, alloc_b_live],
        Some(0.0),
        &[Some(0.0), Some(0.0)],
        0.0,
        0.0,
    );
    let rf1 = check_conservation(&[], &[alloc_obs], &[arena_obs]);
    assert!(
        rf1.allocator_ok && rf1.structural_ok,
        "RF-1 must stay green under order-class weight: {rf1:?}"
    );

    let live_bits = values_bits(&ordered);

    // ── Arrival dissolve at generation boundary ───────────────────────────
    let dissolve = ordered.step_once().expect("arrival dissolve boundary");
    assert!(dissolve.boundary_reached);
    ordered.step_once().expect("post-dissolve production");
    let w_a_post = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b_post = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    let alloc_a_post = read_named(&ordered, dest_a, "order", "food_flow", "allocated");
    let alloc_b_post = read_named(&ordered, dest_b, "order", "food_flow", "allocated");
    assert!(
        (w_a_post - AMBIENT_CEILING).abs() < 1e-2 && (w_b_post - AMBIENT_CEILING).abs() < 1e-2,
        "post-dissolve weights re-anchor to ambient via Permanent System Set (a={w_a_post} b={w_b_post})"
    );
    assert!(
        (alloc_a_post - alloc_b_post).abs() < 1e-2,
        "post-dissolve allocations must re-equalize (a={alloc_a_post} b={alloc_b_post})"
    );

    // ── Never-ordered twin forked from same pre-order fixture ─────────────
    let (scenario_t, game_mode_t, root_id_t, dest_a_t, dest_b_t) = rf_two_destination_fixture();
    let mut twin =
        SimSession::open_from_spec(scenario_t, &game_mode_t).expect("open twin GPU session");
    let root_slot_t = twin
        .proto
        .allocator
        .slot_of(root_id_t)
        .expect("root");
    let flow_id_t = twin.proto.registry.id_of("order", "food_flow").unwrap();
    let cols_t = resolve_node_columns(
        &twin.proto.registry.property(flow_id_t).layout,
        ARENA,
    )
    .unwrap();
    let n_dims_t = twin.state.n_dims;
    let mut seed_t = twin.state.read_values();
    seed_t[(root_slot_t.raw() * n_dims_t + cols_t.intrinsic_flow_col) as usize] = ROOT_INTRINSIC;
    twin.state.install_resolved_values_at_boundary(&seed_t);
    // Same schedule as ordered branch, but never submit the order.
    twin.step_once().expect("twin warm boundary");
    twin.step_once().expect("twin warm production");
    twin.step_once().expect("twin skip-order boundary");
    twin.step_once().expect("twin live production");
    twin.step_once().expect("twin dissolve-time boundary");
    twin.step_once().expect("twin post production");

    let twin_w_a = read_named(&twin, dest_a_t, "order", "food_flow", "weight");
    let twin_w_b = read_named(&twin, dest_b_t, "order", "food_flow", "weight");
    let twin_alloc_a = read_named(&twin, dest_a_t, "order", "food_flow", "allocated");
    let twin_alloc_b = read_named(&twin, dest_b_t, "order", "food_flow", "allocated");
    assert_eq!(
        w_a_post.to_bits(),
        twin_w_a.to_bits(),
        "post-dissolve ordered dest_a weight must match never-ordered twin"
    );
    assert_eq!(
        w_b_post.to_bits(),
        twin_w_b.to_bits(),
        "post-dissolve ordered dest_b weight must match never-ordered twin"
    );
    assert_eq!(
        alloc_a_post.to_bits(),
        twin_alloc_a.to_bits(),
        "post-dissolve alloc_a must match twin"
    );
    assert_eq!(
        alloc_b_post.to_bits(),
        twin_alloc_b.to_bits(),
        "post-dissolve alloc_b must match twin"
    );

    // ── Injection-log style re-injection replay (same directive at same gen) ─
    let (scenario_r, game_mode_r, root_id_r, _a_r, dest_b_r) = rf_two_destination_fixture();
    let mut replay =
        SimSession::open_from_spec(scenario_r, &game_mode_r).expect("open replay session");
    let root_slot_r = replay
        .proto
        .allocator
        .slot_of(root_id_r)
        .expect("root");
    let flow_id_r = replay.proto.registry.id_of("order", "food_flow").unwrap();
    let arrival_id_r = replay.proto.registry.id_of("order", "arrival").unwrap();
    let cols_r = resolve_node_columns(
        &replay.proto.registry.property(flow_id_r).layout,
        ARENA,
    )
    .unwrap();
    let n_dims_r = replay.state.n_dims;
    let mut seed_r = replay.state.read_values();
    seed_r[(root_slot_r.raw() * n_dims_r + cols_r.intrinsic_flow_col) as usize] = ROOT_INTRINSIC;
    replay.state.install_resolved_values_at_boundary(&seed_r);
    replay.step_once().expect("replay warm boundary");
    replay.step_once().expect("replay warm production");
    // Re-inject the same class-bound directive at the same generation as the live run.
    replay
        .submit_order_directive(OrderDirectiveRequest {
            class_id: ORDER_CLASS_ID.into(),
            target: dest_b_r,
            property_id: flow_id_r,
            sub_field: SubFieldRole::Named("weight".into()),
            dissolve: DissolveCondition::PropertyReaches {
                property: arrival_id_r,
                sub_field: SubFieldRole::Amount,
                value: 1.0,
            },
        })
        .expect("replay inject order");
    replay.step_once().expect("replay attach");
    replay.step_once().expect("replay live production");
    let replay_live_bits = values_bits(&replay);
    assert_eq!(
        live_bits, replay_live_bits,
        "injection re-play must be bit-exact with the live ordered values buffer at the ordered-live checkpoint"
    );

    eprintln!(
        "ORDER-WEIGHT-CLASS-GPU-PROOF adapter={:?} backend={:?} device_type={:?} \
         ambient_ceiling={AMBIENT_CEILING} class_magnitude={ORDER_MAGNITUDE} \
         live_weights=({w_a_live},{w_b_live}) live_alloc=({alloc_a_live},{alloc_b_live}) \
         post_weights=({w_a_post},{w_b_post}) post_alloc=({alloc_a_post},{alloc_b_post}) \
         twin_match=1 replay_bitexact=1 rf1_allocator_ok={} rf1_structural_ok={} arrival_live={arrival_live}",
        adapter.name,
        adapter.backend,
        adapter.device_type,
        rf1.allocator_ok,
        rf1.structural_ok,
    );

    // Keep execution plan / column resolution exercised (arena-grounded path).
    let _plan = build_execution_plan(
        &ordered.proto.registry,
        &ordered.spec_state.arena_registry,
    )
    .expect("execution plan");
}

/// Install-time admission rejects class-less dominant Player overlay.
#[test]
fn install_rejects_class_less_dominant_player_overlay() {
    let (scenario, mut game_mode, _r, _a, _b) = rf_two_destination_fixture();
    // Top-level game_mode.overlays are validated at install even if install is deferred.
    game_mode.overlays.push(OverlaySpec {
        id: "illegal_dominant".into(),
        display_name: String::new(),
        targets_property: "order::food_flow".into(),
        sub_field_deltas: vec![(
            SubFieldRole::Named("weight".into()),
            TransformOp::Add(ORDER_MAGNITUDE),
        )],
        lifecycle: OverlayLifecycle::Permanent,
        kind: OverlayKind::Instruction,
        source: OverlaySource::Player,
        install: InstallTargetSpec::ScenarioListed {
            target_id: "dest_b".into(),
        },
        order_weight_class: None,
        source_span_token: Some(99),
    });
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
