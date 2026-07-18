//! RF-EXECUTE-RECURSIVE-DEFAULT-0 — default admitted Arena RF execution proof.
//!
//! §12 homing: the named source/Owner fixture is workshop-only. Production is exercised
//! solely through `SimSession::open_from_spec` and ordinary `step_once`.

use std::collections::HashMap;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    allocator_from_disbursements, build_execution_plan, check_allocator_step, resolve_node_columns,
    ResourceFlowFlagSource, Scenario, SimSession,
};
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
    ResourceFlowSpec, SpecVersion,
};

const ROOT_BUDGET: f32 = 12.0;
const NAMED_CHILD_MARGINAL: f32 = 5.5;

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
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

fn balance_rate_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance_rate".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance_rate".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

fn balance_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance".into(),
        display_range: None,
        governed_by: Some(SubFieldRole::Named("balance_rate".into())),
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role: AccumulatorRole::Balance(BalanceSpec::default()),
            log_tier: LogTier::Summary,
        }),
    }
}

fn register_flow(registry: &mut DimensionRegistry) {
    let property = PropertySpec {
        id: "foundry_flow".into(),
        namespace: "workshop".into(),
        name: "foundry_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "foundry".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "foundry".into(),
                },
            ),
            balance_rate_subfield(),
            balance_subfield(),
        ],
    };
    compile_property(&property, registry).expect("register workshop flow property");
}

struct AuthoredFixture {
    scenario: Scenario,
    game_mode: GameModeSpec,
    session_root: SimThingId,
    owner_aggregate: SimThingId,
    named_child: SimThingId,
}

fn authored_fixture() -> AuthoredFixture {
    let mut registry = DimensionRegistry::new();
    register_flow(&mut registry);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let session_root = SimThing::new(SimThingKind::Cohort, 0);
    let owner_aggregate = SimThing::new(SimThingKind::Cohort, 0);
    let named_child = SimThing::new(SimThingKind::Cohort, 0);
    let ids = [session_root.id, owner_aggregate.id, named_child.id];
    root.add_child(session_root);
    root.add_child(owner_aggregate);
    root.add_child(named_child);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot = |id| allocator.slot_of(id).expect("hosted slot").raw();
    let participants = vec![
        ExplicitParticipantSpec::flat(slot(ids[0]), ids[0].raw()),
        ExplicitParticipantSpec::nested(slot(ids[1]), ids[1].raw(), ids[0].raw() as u64),
        ExplicitParticipantSpec::nested(slot(ids[2]), ids[2].raw(), ids[1].raw() as u64),
    ];

    let game_mode = GameModeSpec {
        id: "rf_recursive_default".into(),
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
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            arenas: vec![ArenaSpec {
                name: "foundry".into(),
                flow_property: PropertyKey::new("workshop", "foundry_flow"),
                balance_property: Some(PropertyKey::new("workshop", "foundry_flow")),
                max_participants: 8,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: participants,
                enrollment: None,
                wildcard_admission: None,
            }],
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    AuthoredFixture {
        scenario: Scenario {
            name: "rf_recursive_default".into(),
            ticks_per_day: 8,
            max_days: 1,
            dt: 1.0,
            n_slots: 32,
            registry,
            root,
            shadow_seeds: vec![],
            tick_patches: vec![],
            install_targets: HashMap::new(),
        },
        game_mode,
        session_root: ids[0],
        owner_aggregate: ids[1],
        named_child: ids[2],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExecutedObservation {
    owner_aggregate_bits: u32,
    owner_allocation_bits: u32,
    named_child_allocation_bits: u32,
    n_bands: u32,
}

fn cell(values: &[f32], slot: SlotIndex, col: u32, n_dims: u32) -> f32 {
    values[(slot.raw() * n_dims + col) as usize]
}

fn execute_default_step(named_child_intrinsic: f32) -> ExecutedObservation {
    let fixture = authored_fixture();
    assert_eq!(
        fixture.game_mode.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow,
        "RF-2 flips the authored execution-profile default"
    );
    assert_eq!(
        fixture
            .game_mode
            .resource_flow
            .as_ref()
            .unwrap()
            .opt_in_mode,
        ResourceFlowOptInMode::Disabled,
        "proof must use the default profile, not legacy FlatStarOptIn"
    );

    let mut session = SimSession::open_from_spec(fixture.scenario, &fixture.game_mode)
        .expect("default admitted Arena RF requires a supported GPU adapter");
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn
    );
    assert!(session.state.accumulator_resource_flow_active);

    let flow_id = session
        .proto
        .registry
        .id_of("workshop", "foundry_flow")
        .expect("flow property");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "foundry")
        .expect("arena columns");
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("recursive execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena");
    assert_eq!(layout.max_depth, 3, "fixture must execute nested D=3 RF");

    let participant_slot = |hosted| {
        *session
            .spec_state
            .arena_participant_scaffold
            .index
            .by_host_and_arena
            .get(&(hosted, 0))
            .expect("admitted participant slot")
    };
    let root_slot = participant_slot(fixture.session_root);
    let owner_slot = participant_slot(fixture.owner_aggregate);
    let child_slot = participant_slot(fixture.named_child);

    let n_dims = session.proto.registry.total_columns as u32;
    let mut values = session.state.read_values();
    values[(root_slot.raw() * n_dims + cols.intrinsic_flow_col) as usize] = ROOT_BUDGET;
    values[(owner_slot.raw() * n_dims + cols.weight_col) as usize] = 1.0;
    values[(child_slot.raw() * n_dims + cols.weight_col) as usize] = 1.0;
    values[(child_slot.raw() * n_dims + cols.intrinsic_flow_col) as usize] = named_child_intrinsic;
    session.state.install_resolved_values_at_boundary(&values);

    let outcome = session.step_once().expect("ordinary admitted step_once");
    assert_eq!(outcome.ticks_run, 1);
    assert!(!outcome.boundary_reached);
    let actual = session.state.read_values();
    let owner_aggregate = cell(&actual, owner_slot, cols.intrinsic_flow_sum_col, n_dims);
    let owner_allocation = cell(&actual, owner_slot, cols.allocated_flow_col, n_dims);
    let named_child_allocation = cell(&actual, child_slot, cols.allocated_flow_col, n_dims);

    check_allocator_step(&allocator_from_disbursements(
        ROOT_BUDGET,
        vec![owner_allocation],
        Some(ROOT_BUDGET - owner_allocation),
    ))
    .expect("RF-1 allocator invariant at root");
    check_allocator_step(&allocator_from_disbursements(
        owner_aggregate + owner_allocation,
        vec![named_child_allocation],
        Some(owner_aggregate + owner_allocation - named_child_allocation),
    ))
    .expect("RF-1 allocator invariant at named Owner aggregate");

    ExecutedObservation {
        owner_aggregate_bits: owner_aggregate.to_bits(),
        owner_allocation_bits: owner_allocation.to_bits(),
        named_child_allocation_bits: named_child_allocation.to_bits(),
        n_bands: session.state.accumulator_resource_flow_bands,
    }
}

#[test]
fn default_step_once_recursively_reduces_named_child_and_writes_local_allocation() {
    GpuContext::new_blocking()
        .expect("RF-2 live execution proof fails closed without a supported GPU adapter");

    let with_child = execute_default_step(NAMED_CHILD_MARGINAL);
    let replay = execute_default_step(NAMED_CHILD_MARGINAL);
    let without_child = execute_default_step(0.0);

    assert_eq!(with_child, replay, "default execution must be bit-exact");
    let with_aggregate = f32::from_bits(with_child.owner_aggregate_bits);
    let without_aggregate = f32::from_bits(without_child.owner_aggregate_bits);
    let with_local = f32::from_bits(with_child.named_child_allocation_bits);
    let without_local = f32::from_bits(without_child.named_child_allocation_bits);
    assert_eq!(without_aggregate.to_bits(), 0.0_f32.to_bits());
    assert_eq!(
        (with_aggregate - without_aggregate).to_bits(),
        NAMED_CHILD_MARGINAL.to_bits(),
        "disabling only the named child must remove exactly its ancestor marginal"
    );
    assert_eq!(
        (with_local - without_local).to_bits(),
        NAMED_CHILD_MARGINAL.to_bits(),
        "runtime local-allocation writeback must carry the same marginal"
    );
    assert!(with_child.n_bands >= 8, "D=3 requires recursive OrderBands");

    eprintln!(
        "RF2-EXECUTED-DEFAULT: depth=3 bands={} named_child_marginal={} owner_aggregate_with={} owner_aggregate_without={} local_allocation_with={} local_allocation_without={} deterministic_bits=PASS economy_execution_deferred=false",
        with_child.n_bands,
        NAMED_CHILD_MARGINAL,
        with_aggregate,
        without_aggregate,
        with_local,
        without_local,
    );
}
