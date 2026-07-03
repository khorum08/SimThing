//! RF-BASE-INTRINSIC-0 - install-consumed base intrinsic-flow obligations.

mod support;

use std::collections::HashMap;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, install_atomic, resolve_node_columns, run_arena_allocation_oracle,
    Scenario, SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec,
    ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec, InstallTargetSpec, PropertyKey,
    PropertySpec, ResourceFlowOptInMode, ResourceFlowSpec, SpecVersion,
};

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

fn register_food_flow(registry: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow, 0.0),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "food".into(),
                },
                0.0,
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "food".into(),
                },
                1.0,
            ),
        ],
    };
    compile_property(&spec, registry).expect("register food flow");
}

fn scenario(hosted_count: usize) -> (Scenario, Vec<SimThingId>) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut ids = Vec::with_capacity(hosted_count);
    for _ in 0..hosted_count {
        let child = SimThing::new(SimThingKind::Cohort, 0);
        ids.push(child.id);
        root.add_child(child);
    }

    let mut registry = DimensionRegistry::new();
    register_food_flow(&mut registry);

    let mut install_targets = HashMap::new();
    install_targets.insert("producer".into(), vec![ids[0]]);
    if hosted_count > 2 {
        install_targets.insert("outside".into(), vec![ids[hosted_count - 1]]);
    }

    (
        Scenario {
            name: "rf_base_intrinsic".into(),
            ticks_per_day: 1,
            max_days: 1,
            dt: 1.0,
            n_slots: 64,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets,
        },
        ids,
    )
}

fn explicit_participants(scenario: &Scenario, count: usize) -> Vec<ExplicitParticipantSpec> {
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    scenario
        .root
        .children
        .iter()
        .take(count)
        .map(|child| {
            ExplicitParticipantSpec::flat(allocator.slot_of(child.id).unwrap(), child.id.raw())
        })
        .collect()
}

fn obligation(
    id: &str,
    target_id: &str,
    direction: BaseFlowDirectionSpec,
    rate: f32,
) -> BaseFlowObligationSpec {
    BaseFlowObligationSpec {
        id: id.into(),
        arena: "food".into(),
        install: InstallTargetSpec::ScenarioListed {
            target_id: target_id.into(),
        },
        direction,
        rate,
    }
}

fn game_mode(
    scenario: &Scenario,
    participant_count: usize,
    opt_in: ResourceFlowOptInMode,
) -> GameModeSpec {
    GameModeSpec {
        id: "rf_base_intrinsic".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            opt_in_mode: opt_in,
            arenas: vec![ArenaSpec {
                name: "food".into(),
                flow_property: PropertyKey::new("core", "food_flow"),
                balance_property: None,
                max_participants: 16,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: explicit_participants(scenario, participant_count),
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: vec![],
            base_obligations: vec![
                obligation(
                    "producer_food",
                    "producer",
                    BaseFlowDirectionSpec::Produce,
                    10.0,
                ),
                obligation(
                    "producer_upkeep",
                    "producer",
                    BaseFlowDirectionSpec::Upkeep,
                    2.0,
                ),
            ],
            gated_rates: vec![],
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

fn cell(values: &[f32], slot: u32, col: u32, n_dims: u32) -> f32 {
    values[slot as usize * n_dims as usize + col as usize]
}

#[test]
fn install_consumes_base_intrinsic_obligations_without_manual_side_channel() {
    let Some(_gpu) = support::e11_flat_star::try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (scenario, _) = scenario(3);
    let mode = game_mode(&scenario, 3, ResourceFlowOptInMode::Disabled);
    let session = SimSession::open_from_spec(scenario, &mode).expect("open_from_spec");
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(!session.state.accumulator_resource_flow_active);
    assert_eq!(session.spec_state.arena_registry.participants.len(), 3);

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("cols");
    let layout = build_execution_plan_from_authoring(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("plan")
    .arenas
    .into_iter()
    .next()
    .expect("arena");
    let root_slot = layout.participant_roots[0].participant_slot;
    let n_dims = session.coord.n_dims();
    assert_eq!(
        cell(
            &session.coord.shadow,
            root_slot,
            cols.intrinsic_flow_col,
            n_dims
        )
        .to_bits(),
        8.0_f32.to_bits()
    );
}

#[test]
fn base_intrinsic_obligation_target_must_be_admitted_to_arena() {
    let (scenario, _) = scenario(3);
    let mut mode = game_mode(&scenario, 2, ResourceFlowOptInMode::Disabled);
    mode.resource_flow.as_mut().unwrap().base_obligations = vec![obligation(
        "outside_food",
        "outside",
        BaseFlowDirectionSpec::Produce,
        1.0,
    )];

    let mut registry = scenario.registry.clone();
    let mut root = scenario.root.clone();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let err = install_atomic(&mode, &scenario, &mut registry, &mut root, &mut allocator)
        .expect_err("outside target must not seed an unadmitted participant");
    assert!(
        err.to_string().contains("not admitted"),
        "expected not-admitted error, got {err}"
    );
}
