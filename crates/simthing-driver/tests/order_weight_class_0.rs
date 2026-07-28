//! ORDER-WEIGHT-CLASS-0 — finite order-weight class + Player directive path.
//!
//! Canonical exemplar: a TP fleet chooses between canonical STEAD destination
//! gridcells through ordinary RF OrderBand normalization. Orders are price injections
//! (`OverlaySource::Player` overlays via the typed class-bound feeder path),
//! never command channels. Dominance is finite and arena-grounded
//! (class magnitude > install-derived ambient sum). Latency = next generation boundary.
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
    build_execution_plan, check_conservation, flat_star_observations, resolve_node_columns_for_property,
    OrderDirectiveRequest, Scenario, SimSession,
};
use simthing_spec::{
    compile_property, validate_order_weight_classes, validate_order_weight_overlay, ArenaSpec,
    ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec, InstallTargetSpec,
    OrderWeightClassSpec, OverlaySpec, PropertyKey, PropertySpec, ResourceFlowOptInMode,
    ResourceFlowSpec, SpecError, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const ORDER_CLASS_ID: &str = "destination_order";
/// Per-destination ambient weight authored by the canonical fixture.
const AMBIENT_WEIGHT_EACH: f32 = 2.0;
/// Class magnitude — strictly dominates ambient under 2-way proportional allocation.
const ORDER_MAGNITUDE: f32 = 20.0;
const ROOT_INTRINSIC: f32 = 30.0;
const ARENA: &str = "order_dest";

fn order_class() -> OrderWeightClassSpec {
    OrderWeightClassSpec {
        id: ORDER_CLASS_ID.into(),
        magnitude: ORDER_MAGNITUDE,
        arena: ARENA.into(),
        property: PropertyKey::new("order", "food_flow"),
        sub_field: SubFieldRole::Named("weight".into()),
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

fn food_flow_property() -> PropertySpec {
    PropertySpec {
        id: "food_flow".into(),
        namespace: "order".into(),
        name: "food_flow".into(),
        display_name: "Food Flow".into(),
        description: String::new(),
        admission_disposition: Default::default(),
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

fn canonical_tp_stead_authority() -> (SimThing, SimThingId, SimThingId, SimThingId) {
    let fixture_json = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/");
    let source = include_str!(
        "../../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause"
    )
    .replace("{{FIXTURE_JSON}}", &fixture_json);
    let document =
        simthing_clausething::parse_raw_document(source.as_bytes()).expect("parse canonical TP");
    let pack = simthing_clausething::hydrate_scenario(&document).expect("hydrate canonical TP");
    assert!(
        pack.commitment.is_some(),
        "canonical TP STEAD commitment feedstock"
    );
    assert_eq!(
        pack.metadata
            .get("fleet_movement_profile")
            .map(String::as_str),
        Some("palma_d_gradient_reparent")
    );
    let fleet_id = first_kind(
        pack.authority_root.as_ref().expect("TP authority root"),
        &SimThingKind::Fleet,
    )
    .expect("canonical TP fleet");
    let mut destinations: Vec<_> = pack
        .install_targets
        .iter()
        .filter(|(name, _)| name.starts_with("tp_base::"))
        .flat_map(|(_, ids)| ids.iter().copied())
        .collect();
    destinations.sort_by_key(|id| id.raw());
    destinations.dedup();
    let [dest_a, dest_b, ..] = destinations.as_slice() else {
        panic!("canonical TP must expose at least two STEAD destination gridcells");
    };
    let mut root = pack.authority_root.expect("TP authority root");
    clear_runtime_values(&mut root);
    (root, fleet_id, *dest_a, *dest_b)
}

fn first_kind(node: &SimThing, kind: &SimThingKind) -> Option<SimThingId> {
    if &node.kind == kind {
        return Some(node.id);
    }
    node.children
        .iter()
        .find_map(|child| first_kind(child, kind))
}

fn clear_runtime_values(node: &mut SimThing) {
    node.properties.clear();
    node.overlays.clear();
    node.resource_parent_edges.clear();
    for child in &mut node.children {
        clear_runtime_values(child);
    }
}

fn find_mut(node: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_mut(child, id))
}

/// Canonical TP fleet + two STEAD grid destinations in an RF star. Ambient prices are Permanent
/// System Set overlays so post-dissolve weights re-anchor to the admitted ambient state.
fn tp_stead_destination_fixture() -> (Scenario, GameModeSpec, SimThingId, SimThingId, SimThingId) {
    let mut registry = simthing_core::DimensionRegistry::new();
    compile_property(&food_flow_property(), &mut registry).expect("food_flow");
    let flow_id = registry.id_of("order", "food_flow").expect("flow id");
    let flow_layout = registry.property(flow_id).layout.clone();

    let mut flow_default = registry.property(flow_id).default_value();
    flow_default.set_role(
        &SubFieldRole::Named("weight".into()),
        &flow_layout,
        AMBIENT_WEIGHT_EACH,
    );
    let (mut root, root_id, a_id, b_id) = canonical_tp_stead_authority();

    // Root carries intrinsic pool; leaves carry weights + allocated.
    let mut root_flow = registry.property(flow_id).default_value();
    root_flow.set_role(&SubFieldRole::Named("flow".into()), &flow_layout, 0.0);
    root_flow.set_role(&SubFieldRole::Named("weight".into()), &flow_layout, 0.0);
    find_mut(&mut root, root_id)
        .expect("TP fleet")
        .add_property(flow_id, root_flow);

    find_mut(&mut root, a_id)
        .expect("TP destination A")
        .add_property(flow_id, flow_default.clone());
    find_mut(&mut root, b_id)
        .expect("TP destination B")
        .add_property(flow_id, flow_default);
    let ambient_overlay = |target: SimThingId| Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        affects: vec![target],
        transform: PropertyTransformDelta {
            property_id: flow_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named("weight".into()),
                TransformOp::Set(AMBIENT_WEIGHT_EACH),
            )],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    find_mut(&mut root, a_id)
        .expect("TP destination A")
        .add_overlay(ambient_overlay(a_id));
    find_mut(&mut root, b_id)
        .expect("TP destination B")
        .add_overlay(ambient_overlay(b_id));

    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let participants = vec![
        ExplicitParticipantSpec::flat(
            allocator.slot_of(root_id).expect("fleet slot").raw(),
            root_id.raw(),
        ),
        ExplicitParticipantSpec::nested(
            allocator.slot_of(a_id).expect("destination A slot").raw(),
            a_id.raw(),
            root_id.raw() as u64,
        ),
        ExplicitParticipantSpec::nested(
            allocator.slot_of(b_id).expect("destination B slot").raw(),
            b_id.raw(),
            root_id.raw() as u64,
        ),
    ];

    let scenario = Scenario {
        name: "order_weight_class_0_rf".into(),
        ticks_per_day: 1,
        max_days: 16,
        dt: 1.0,
        n_slots: root.subtree_size() as u32 + 16,
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

    let game_mode = GameModeSpec {
        id: "order_weight_class_0_rf".into(),
        display_name: "Order Weight Class 0 RF".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
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
            arena: ARENA.into(),
            property: PropertyKey::new("order", "food_flow"),
            sub_field: SubFieldRole::Named("weight".into()),
            source_span_token: Some(1),
        }]),
        Err(SpecError::MalformedOrderWeightClass { .. })
    ));
    let (scenario, mut game_mode, ..) = tp_stead_destination_fixture();
    game_mode.order_weight_classes[0].magnitude = 2.0;
    game_mode.order_weight_classes[0].source_span_token = Some(2);
    assert!(
        SimSession::open_from_spec(scenario, &game_mode).is_err(),
        "install must derive the real arena ambient sum and reject a non-dominating class"
    );

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
    let game_mode = simthing_spec::ron::deserialize_game_mode_ron(&text)
        .expect("parse authored RON GameModeSpec");
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
            let add_scalar = text.find("Add(20.0)").expect("offending scalar") + "Add(".len();
            assert_eq!(source_span_token, Some(add_scalar));
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
    let (scenario, game_mode, root_id, dest_a, dest_b) = tp_stead_destination_fixture();
    let twin_scenario = scenario.clone();
    let twin_game_mode = game_mode.clone();
    let recorded_scenario = scenario.clone();
    let recorded_game_mode = game_mode.clone();
    let replay_scenario = scenario.clone();
    let replay_game_mode = game_mode.clone();
    let mut ordered =
        SimSession::open_from_spec(scenario, &game_mode).expect("open live GPU session");
    let adapter = ordered.state.ctx.adapter.get_info();
    let flow_id = ordered
        .proto
        .registry
        .id_of("order", "food_flow")
        .expect("flow");

    // Seed root intrinsic into dense values (property default + one production).
    // Explicit participant RF uses buffer values; ensure pool is present.
    let root_slot = ordered.proto.allocator.slot_of(root_id).expect("root slot");
    let cols = resolve_node_columns_for_property(&ordered.proto.registry, flow_id, ARENA)
        .expect("flow columns");
    let n_dims = ordered.state.n_dims;
    // Ambient baseline weights via Permanent System Set overlays (applied at
    // first production after open/sync). Advance one boundary+production so
    // ambient overlays are live before the order attaches.
    let warm = ordered.step_once().expect("warm boundary");
    assert!(warm.boundary_reached);
    ordered.step_once().expect("warm production");

    let w_a0 = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b0 = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    assert!(
        (w_a0 - AMBIENT_WEIGHT_EACH).abs() < 1e-3 && (w_b0 - AMBIENT_WEIGHT_EACH).abs() < 1e-3,
        "ambient weights must equal the admitted fixture state before order (a={w_a0} b={w_b0})"
    );
    let arrival_before = read_named(&ordered, dest_b, "order", "food_flow", "allocated");
    assert_eq!(
        arrival_before.to_bits(),
        0.0_f32.to_bits(),
        "destination must be live and not arrived before the ordered path runs"
    );
    let mut seed = ordered.state.read_values();
    let root_flow_idx =
        (root_slot.raw() * n_dims + cols.intrinsic_flow_col.raw_u32()) as usize;
    seed[root_flow_idx] = ROOT_INTRINSIC;
    ordered.state.install_resolved_values_at_boundary(&seed);

    // Class-bound directive via typed API — no raw ORDER_MAGNITUDE bypass.
    ordered
        .submit_order_directive(OrderDirectiveRequest {
            class_id: ORDER_CLASS_ID.into(),
            target: dest_b,
            property_id: flow_id,
            sub_field: SubFieldRole::Named("weight".into()),
            dissolve: DissolveCondition::PropertyReaches {
                property: flow_id,
                sub_field: SubFieldRole::Named("allocated".into()),
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
        ordered.submit_player_intent_gated(dest_a, bypass).is_err(),
        "raw dominant Player overlay must not bypass class law"
    );

    // Order attaches at next generation boundary (decision-ingress latency).
    let attach = ordered.step_once().expect("order attach boundary");
    assert!(
        attach.boundary_reached,
        "order attaches at generation boundary"
    );

    // Production: ambient Set then order Add on dest_b; RF allocates by weight.
    ordered.step_once().expect("ordered live production");
    let w_a_live = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b_live = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    let alloc_a_live = read_named(&ordered, dest_a, "order", "food_flow", "allocated");
    let alloc_b_live = read_named(&ordered, dest_b, "order", "food_flow", "allocated");
    let arrival_live = read_named(&ordered, dest_b, "order", "food_flow", "allocated");

    assert!(
        (w_a_live - AMBIENT_WEIGHT_EACH).abs() < 1e-2,
        "unordered dest stays ambient weight (got {w_a_live})"
    );
    assert!(
        w_b_live + 1e-2 >= AMBIENT_WEIGHT_EACH + ORDER_MAGNITUDE
            || w_b_live + 1e-2 >= ORDER_MAGNITUDE,
        "ordered dest weight must carry class magnitude (got w_b={w_b_live})"
    );
    assert!(
        alloc_b_live > alloc_a_live,
        "ordered destination must dominate RF allocation (a={alloc_a_live} b={alloc_b_live})"
    );
    // Arena-grounded dominance: ordered share > ambient twin share under
    // proportional normalization with the loader-derived ambient envelope.
    assert!(
        alloc_b_live >= alloc_a_live + 1.0,
        "dominance margin under ambient_weight_each={AMBIENT_WEIGHT_EACH} mag={ORDER_MAGNITUDE}"
    );
    assert!(
        arrival_live + 1e-3 >= 1.0,
        "selected-destination allocation must causally advance arrival (got {arrival_live})"
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

    // ── Arrival dissolve at generation boundary ───────────────────────────
    let dissolve = ordered.step_once().expect("arrival dissolve boundary");
    assert!(dissolve.boundary_reached);
    ordered.step_once().expect("post-dissolve production");
    let w_a_post = read_named(&ordered, dest_a, "order", "food_flow", "weight");
    let w_b_post = read_named(&ordered, dest_b, "order", "food_flow", "weight");
    let alloc_a_post = read_named(&ordered, dest_a, "order", "food_flow", "allocated");
    let alloc_b_post = read_named(&ordered, dest_b, "order", "food_flow", "allocated");
    assert!(
        (w_a_post - AMBIENT_WEIGHT_EACH).abs() < 1e-2
            && (w_b_post - AMBIENT_WEIGHT_EACH).abs() < 1e-2,
        "post-dissolve weights re-anchor to ambient via Permanent System Set (a={w_a_post} b={w_b_post})"
    );
    assert!(
        (alloc_a_post - alloc_b_post).abs() < 1e-2,
        "post-dissolve allocations must re-equalize (a={alloc_a_post} b={alloc_b_post})"
    );

    // ── Never-ordered twin forked from same pre-order fixture ─────────────
    let root_id_t = root_id;
    let dest_a_t = dest_a;
    let dest_b_t = dest_b;
    let mut twin = SimSession::open_from_spec(twin_scenario, &twin_game_mode)
        .expect("open same-checkpoint twin GPU session");
    let root_slot_t = twin.proto.allocator.slot_of(root_id_t).expect("root");
    let flow_id_t = twin.proto.registry.id_of("order", "food_flow").unwrap();
    let cols_t =
        resolve_node_columns_for_property(&twin.proto.registry, flow_id_t, ARENA).unwrap();
    let n_dims_t = twin.state.n_dims;
    // Same schedule as ordered branch, but never submit the order.
    twin.step_once().expect("twin warm boundary");
    twin.step_once().expect("twin warm production");
    let mut seed_t = twin.state.read_values();
    seed_t[(root_slot_t.raw() * n_dims_t + cols_t.intrinsic_flow_col.raw_u32()) as usize] =
        ROOT_INTRINSIC;
    twin.state.install_resolved_values_at_boundary(&seed_t);
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
    assert_eq!(
        values_bits(&ordered),
        values_bits(&twin),
        "complete dense state must converge at the first post-dissolve checkpoint"
    );
    for generation in 0..3 {
        ordered.step_once().expect("ordered convergence step");
        twin.step_once().expect("twin convergence step");
        assert_eq!(
            values_bits(&ordered),
            values_bits(&twin),
            "complete dense trajectory diverged after dissolve at convergence generation {generation}"
        );
        let ordered_snapshot =
            serde_json::to_vec(&ordered.proto.snapshot(ordered.coord.day_index() as u32))
                .expect("ordered structural checkpoint");
        let twin_snapshot = serde_json::to_vec(&twin.proto.snapshot(twin.coord.day_index() as u32))
            .expect("twin structural checkpoint");
        assert_eq!(
            ordered_snapshot, twin_snapshot,
            "tree/overlay lifecycle checkpoint diverged at convergence generation {generation}"
        );
    }

    // ── Injection-log style re-injection replay (same directive at same gen) ─
    let root_id_r = root_id;
    let dest_b_r = dest_b;
    let mut recorded = SimSession::open_from_spec(recorded_scenario, &recorded_game_mode)
        .expect("open recording session");
    let mut replay = SimSession::open_from_spec(replay_scenario, &replay_game_mode)
        .expect("open same-checkpoint replay session");
    let root_slot_r = recorded.proto.allocator.slot_of(root_id_r).expect("root");
    let flow_id_r = recorded.proto.registry.id_of("order", "food_flow").unwrap();
    let cols_r =
        resolve_node_columns_for_property(&recorded.proto.registry, flow_id_r, ARENA).unwrap();
    let n_dims_r = recorded.state.n_dims;
    recorded.step_once().expect("record warm boundary");
    recorded.step_once().expect("record warm production");
    replay.step_once().expect("replay warm boundary");
    replay.step_once().expect("replay warm production");
    let mut seed_r = recorded.state.read_values();
    seed_r[(root_slot_r.raw() * n_dims_r + cols_r.intrinsic_flow_col.raw_u32()) as usize] =
        ROOT_INTRINSIC;
    recorded.state.install_resolved_values_at_boundary(&seed_r);
    replay.state.install_resolved_values_at_boundary(&seed_r);
    recorded
        .submit_order_directive(OrderDirectiveRequest {
            class_id: ORDER_CLASS_ID.into(),
            target: dest_b_r,
            property_id: flow_id_r,
            sub_field: SubFieldRole::Named("weight".into()),
            dissolve: DissolveCondition::PropertyReaches {
                property: flow_id_r,
                sub_field: SubFieldRole::Named("allocated".into()),
                value: 1.0,
            },
        })
        .expect("record directive ingress");
    let replay_path = std::env::temp_dir().join(format!(
        "simthing-order-weight-{}-{}.jsonl",
        std::process::id(),
        recorded.coord.day_index()
    ));
    recorded
        .record_to_path(&replay_path, 2)
        .expect("record through existing replay writer");
    let loaded =
        simthing_driver::read_spec_replay_file(&replay_path).expect("read directive replay log");
    let mut injection_count = 0;
    for (frame, _) in loaded.frames {
        for injection in simthing_driver::order_directive_injections_from_frame(&frame)
            .expect("decode typed directive ingress")
        {
            assert_eq!(
                injection.generation,
                replay.coord.day_index(),
                "directive must re-enter at its recorded generation"
            );
            replay
                .submit_order_directive(injection.request)
                .expect("replay logged directive");
            injection_count += 1;
        }
        replay.step_once().expect("replay logged frame");
    }
    std::fs::remove_file(&replay_path).expect("remove temporary replay");
    assert_eq!(injection_count, 1, "one real directive ingress record");
    assert_eq!(
        values_bits(&recorded),
        values_bits(&replay),
        "existing replay-log re-injection must reproduce the complete values checkpoint bit-exact"
    );

    eprintln!(
        "ORDER-WEIGHT-CLASS-GPU-PROOF adapter={:?} backend={:?} device_type={:?} \
         ambient_weight_each={AMBIENT_WEIGHT_EACH} class_magnitude={ORDER_MAGNITUDE} \
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
    let _plan = build_execution_plan(&ordered.proto.registry, &ordered.spec_state.arena_registry)
        .expect("execution plan");
}

/// Install-time admission rejects class-less dominant Player overlay.
#[test]
fn install_rejects_class_less_dominant_player_overlay() {
    let (scenario, mut game_mode, _r, _a, _b) = tp_stead_destination_fixture();
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

#[test]
fn raw_public_sender_cannot_bypass_canonical_player_drain_gate() {
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let (scenario, game_mode, _root, dest_a, _dest_b) = tp_stead_destination_fixture();
    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open session");
    let flow_id = session.proto.registry.id_of("order", "food_flow").unwrap();
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
    session
        .tx
        .submit_player_intent(dest_a, bypass)
        .expect("raw public sender accepts queue work");
    let error = session
        .step_once()
        .expect_err("canonical drain must reject raw dominant Player overlay");
    assert!(
        error.to_string().contains("class-less dominant"),
        "unexpected drain rejection: {error}"
    );
    assert!(
        session.patcher.take_player_intents().is_empty(),
        "rejected intent must be neither folded nor parked"
    );

    let wrong_target = session.submit_order_directive(OrderDirectiveRequest {
        class_id: ORDER_CLASS_ID.into(),
        target: SimThingId::new(),
        property_id: flow_id,
        sub_field: SubFieldRole::Named("weight".into()),
        dissolve: DissolveCondition::AfterTicks { remaining: 1 },
    });
    assert!(
        wrong_target.is_err(),
        "class binding cannot be reused outside its admitted arena participants"
    );

    // Individually sub-dominant raw additions cannot defeat the class by accumulation.
    let (scenario, game_mode, _root, dest_a, _dest_b) = tp_stead_destination_fixture();
    let mut aggregate = SimSession::open_from_spec(scenario, &game_mode).expect("open session");
    let flow_id = aggregate
        .proto
        .registry
        .id_of("order", "food_flow")
        .unwrap();
    for _ in 0..2 {
        aggregate
            .tx
            .submit_player_intent(
                dest_a,
                Overlay {
                    id: OverlayId::new(),
                    kind: OverlayKind::Instruction,
                    source: OverlaySource::Player,
                    affects: vec![dest_a],
                    transform: PropertyTransformDelta {
                        property_id: flow_id,
                        sub_field_deltas: vec![(
                            SubFieldRole::Named("weight".into()),
                            TransformOp::Add(8.0),
                        )],
                    },
                    lifecycle: OverlayLifecycle::Transient {
                        dissolution_conditions: vec![DissolveCondition::AfterTicks {
                            remaining: 1,
                        }],
                    },
                },
            )
            .expect("queue individually sub-dominant raw addition");
    }
    let error = aggregate
        .step_once()
        .expect_err("aggregate raw additions must not reach the admitted class magnitude");
    assert!(
        error
            .to_string()
            .contains("aggregate Player ambient envelope"),
        "unexpected aggregate drain rejection: {error}"
    );
}
