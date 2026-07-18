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
    allocator_eps_bound, allocator_from_disbursements, build_execution_plan, check_allocator_step,
    check_conservation, resolve_node_columns, AllocatorConservationViolation,
    ArenaConservationSnapshot, ArenaParticipantObservation, ArenaStructuralEvidence,
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
const FIXED_SIBLING_A_INTRINSIC: f32 = 2.25;
const FIXED_SIBLING_B_INTRINSIC: f32 = 3.125;
const LEAF_WEIGHTS: [f32; 3] = [3.0, 8.0, 4.0];

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

fn balance_subfield(connect_governed_rate: bool) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance".into(),
        display_range: None,
        governed_by: connect_governed_rate.then(|| SubFieldRole::Named("balance_rate".into())),
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role: AccumulatorRole::Balance(BalanceSpec::default()),
            log_tier: LogTier::Summary,
        }),
    }
}

fn register_flow(registry: &mut DimensionRegistry, connect_governed_rate: bool) {
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
            balance_subfield(connect_governed_rate),
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
    fixed_sibling_a: SimThingId,
    fixed_sibling_b: SimThingId,
}

fn authored_fixture(
    connect_governed_rate: bool,
    execution_profile: ResourceFlowExecutionProfile,
) -> AuthoredFixture {
    let mut registry = DimensionRegistry::new();
    register_flow(&mut registry, connect_governed_rate);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let session_root = SimThing::new(SimThingKind::Cohort, 0);
    let owner_aggregate = SimThing::new(SimThingKind::Cohort, 0);
    let named_child = SimThing::new(SimThingKind::Cohort, 0);
    let fixed_sibling_a = SimThing::new(SimThingKind::Cohort, 0);
    let fixed_sibling_b = SimThing::new(SimThingKind::Cohort, 0);
    let ids = [
        session_root.id,
        owner_aggregate.id,
        named_child.id,
        fixed_sibling_a.id,
        fixed_sibling_b.id,
    ];
    root.add_child(session_root);
    root.add_child(owner_aggregate);
    root.add_child(named_child);
    root.add_child(fixed_sibling_a);
    root.add_child(fixed_sibling_b);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot = |id| allocator.slot_of(id).expect("hosted slot").raw();
    let participants = vec![
        ExplicitParticipantSpec::flat(slot(ids[0]), ids[0].raw()),
        ExplicitParticipantSpec::nested(slot(ids[1]), ids[1].raw(), ids[0].raw() as u64),
        ExplicitParticipantSpec::nested(slot(ids[2]), ids[2].raw(), ids[1].raw() as u64),
        ExplicitParticipantSpec::nested(slot(ids[3]), ids[3].raw(), ids[1].raw() as u64),
        ExplicitParticipantSpec::nested(slot(ids[4]), ids[4].raw(), ids[1].raw() as u64),
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
        resource_flow_execution_profile: execution_profile,
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
        fixed_sibling_a: ids[3],
        fixed_sibling_b: ids[4],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExecutedObservation {
    owner_aggregate_bits: u32,
    owner_allocation_bits: u32,
    leaf_allocation_bits: Vec<u32>,
    root_balance_rate_bits: u32,
    owner_balance_rate_bits: u32,
    root_balance_delta_bits: u32,
    owner_balance_delta_bits: u32,
    leaf_balance_delta_bits: Vec<u32>,
    n_bands: u32,
    flag_source: ResourceFlowFlagSource,
    rf_active: bool,
}

fn cell(values: &[f32], slot: SlotIndex, col: u32, n_dims: u32) -> f32 {
    values[(slot.raw() * n_dims + col) as usize]
}

fn execute_step(
    named_child_intrinsic: f32,
    connect_governed_rate: bool,
    execution_profile: ResourceFlowExecutionProfile,
) -> (ExecutedObservation, simthing_driver::ConservationReport) {
    let fixture = authored_fixture(connect_governed_rate, execution_profile);
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
    let expected_active = execution_profile == ResourceFlowExecutionProfile::FlatStarResourceFlow;
    assert_eq!(
        session.proto.flags.use_accumulator_resource_flow,
        expected_active
    );
    assert_eq!(
        session.resource_flow_flag_source,
        if expected_active {
            ResourceFlowFlagSource::ScenarioClassDefaultOn
        } else {
            ResourceFlowFlagSource::DefaultDisabled
        }
    );
    assert_eq!(
        session.state.accumulator_resource_flow_active,
        expected_active
    );

    let flow_id = session
        .proto
        .registry
        .id_of("workshop", "foundry_flow")
        .expect("flow property");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "foundry")
        .expect("arena columns");
    let balance_col = cols.balance_col.expect("fixture Balance column");
    let balance_rate_col = session
        .proto
        .registry
        .property(flow_id)
        .layout
        .offset_of(&SubFieldRole::Named("balance_rate".into()))
        .expect("governed Balance rate column")
        .lane() as u32;
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
    let leaf_ids = [
        fixture.named_child,
        fixture.fixed_sibling_a,
        fixture.fixed_sibling_b,
    ];
    let leaf_slots = leaf_ids.map(participant_slot);

    let n_dims = session.proto.registry.total_columns as u32;
    let mut values = session.state.read_values();
    values[(root_slot.raw() * n_dims + cols.intrinsic_flow_col) as usize] = ROOT_BUDGET;
    values[(owner_slot.raw() * n_dims + cols.weight_col) as usize] = 1.0;
    let leaf_intrinsics = [
        named_child_intrinsic,
        FIXED_SIBLING_A_INTRINSIC,
        FIXED_SIBLING_B_INTRINSIC,
    ];
    for ((slot, weight), intrinsic) in leaf_slots.iter().zip(LEAF_WEIGHTS).zip(leaf_intrinsics) {
        values[(slot.raw() * n_dims + cols.weight_col) as usize] = weight;
        values[(slot.raw() * n_dims + cols.intrinsic_flow_col) as usize] = intrinsic;
    }

    // The governed rate starts at zero. Ordinary Arena OrderBands must derive the
    // allocator residual from the executed budget and disbursements before the
    // existing governed_by integration mutates Balance.
    values[(root_slot.raw() * n_dims + balance_rate_col) as usize] = 0.0;
    values[(owner_slot.raw() * n_dims + balance_rate_col) as usize] = 0.0;
    let root_balance_before = cell(&values, root_slot, balance_col, n_dims);
    let owner_balance_before = cell(&values, owner_slot, balance_col, n_dims);
    let leaf_balance_before = leaf_slots.map(|slot| cell(&values, slot, balance_col, n_dims));
    session.state.install_resolved_values_at_boundary(&values);

    let outcome = session.step_once().expect("ordinary admitted step_once");
    assert_eq!(outcome.ticks_run, 1);
    assert!(!outcome.boundary_reached);
    let actual = session.state.read_values();
    let owner_aggregate = cell(&actual, owner_slot, cols.intrinsic_flow_sum_col, n_dims);
    let owner_allocation = cell(&actual, owner_slot, cols.allocated_flow_col, n_dims);
    let leaf_allocations: Vec<f32> = leaf_slots
        .iter()
        .map(|slot| cell(&actual, *slot, cols.allocated_flow_col, n_dims))
        .collect();
    let root_balance_rate = cell(&actual, root_slot, balance_rate_col, n_dims);
    let owner_balance_rate = cell(&actual, owner_slot, balance_rate_col, n_dims);
    let root_balance_delta = cell(&actual, root_slot, balance_col, n_dims) - root_balance_before;
    let owner_balance_delta = cell(&actual, owner_slot, balance_col, n_dims) - owner_balance_before;
    let leaf_balance_deltas: Vec<f32> = leaf_slots
        .iter()
        .zip(leaf_balance_before)
        .map(|(slot, before)| cell(&actual, *slot, balance_col, n_dims) - before)
        .collect();
    let allocator_observations = vec![
        allocator_from_disbursements(
            ROOT_BUDGET,
            vec![owner_allocation],
            Some(root_balance_delta),
        ),
        allocator_from_disbursements(
            owner_aggregate + owner_allocation,
            leaf_allocations.clone(),
            Some(owner_balance_delta),
        ),
    ];
    let participants = vec![
        ArenaParticipantObservation {
            id: fixture.session_root.raw() as u64,
            is_leaf: false,
            intrinsic_flow: ROOT_BUDGET,
            allocated_flow: 0.0,
            balance_delta: Some(root_balance_delta),
        },
        ArenaParticipantObservation {
            id: fixture.owner_aggregate.raw() as u64,
            is_leaf: false,
            intrinsic_flow: 0.0,
            allocated_flow: owner_allocation,
            balance_delta: Some(owner_balance_delta),
        },
    ]
    .into_iter()
    .chain(
        leaf_ids
            .into_iter()
            .zip(leaf_intrinsics)
            .zip(
                leaf_allocations
                    .iter()
                    .copied()
                    .zip(leaf_balance_deltas.iter().copied()),
            )
            .map(|((id, intrinsic_flow), (allocated_flow, balance_delta))| {
                ArenaParticipantObservation {
                    id: id.raw() as u64,
                    is_leaf: true,
                    intrinsic_flow,
                    allocated_flow,
                    balance_delta: Some(balance_delta),
                }
            }),
    )
    .collect();
    let arena = ArenaConservationSnapshot {
        participants,
        structural_evidence: ArenaStructuralEvidence {
            declared_intrinsic_source_ids: vec![
                fixture.session_root.raw() as u64,
                fixture.named_child.raw() as u64,
                fixture.fixed_sibling_a.raw() as u64,
                fixture.fixed_sibling_b.raw() as u64,
            ],
            inbound_coupling_endpoint_ids: Vec::new(),
            parent_disbursement_recipient_ids: vec![
                fixture.owner_aggregate.raw() as u64,
                fixture.named_child.raw() as u64,
                fixture.fixed_sibling_a.raw() as u64,
                fixture.fixed_sibling_b.raw() as u64,
            ],
        },
        inbound_coupling: 0.0,
        emission_consumption: 0.0,
    };
    // No recipe executes in this authored Arena; recipe exactness is vacuous.
    let report = check_conservation(&[], &allocator_observations, &[arena]);

    let observation = ExecutedObservation {
        owner_aggregate_bits: owner_aggregate.to_bits(),
        owner_allocation_bits: owner_allocation.to_bits(),
        leaf_allocation_bits: leaf_allocations
            .iter()
            .map(|value| value.to_bits())
            .collect(),
        root_balance_rate_bits: root_balance_rate.to_bits(),
        owner_balance_rate_bits: owner_balance_rate.to_bits(),
        root_balance_delta_bits: root_balance_delta.to_bits(),
        owner_balance_delta_bits: owner_balance_delta.to_bits(),
        leaf_balance_delta_bits: leaf_balance_deltas
            .iter()
            .map(|value| value.to_bits())
            .collect(),
        n_bands: session.state.accumulator_resource_flow_bands,
        flag_source: session.resource_flow_flag_source,
        rf_active: session.state.accumulator_resource_flow_active,
    };
    (observation, report)
}

#[test]
fn default_step_once_recursively_reduces_named_child_and_writes_local_allocation() {
    GpuContext::new_blocking()
        .expect("RF-2 live execution proof fails closed without a supported GPU adapter");

    let default_profile = ResourceFlowExecutionProfile::default();
    assert_eq!(
        default_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow,
        "RF-2 must keep the admitted Arena profile as the authored default"
    );
    let (with_child, with_report) = execute_step(NAMED_CHILD_MARGINAL, true, default_profile);
    let (replay, replay_report) = execute_step(NAMED_CHILD_MARGINAL, true, default_profile);
    let (without_child, without_report) = execute_step(0.0, true, default_profile);

    assert_eq!(with_child, replay, "default execution must be bit-exact");
    assert!(
        with_report.all_pass(),
        "RF-1 must judge all invariant families: {with_report:?}"
    );
    assert!(replay_report.all_pass());
    assert!(
        without_report.all_pass(),
        "paired contribution control must remain RF-1-conservative: {without_report:?}"
    );
    let with_aggregate = f32::from_bits(with_child.owner_aggregate_bits);
    let without_aggregate = f32::from_bits(without_child.owner_aggregate_bits);
    let sibling_aggregate = FIXED_SIBLING_A_INTRINSIC + FIXED_SIBLING_B_INTRINSIC;
    assert!(without_aggregate > 0.0, "fixed siblings must contribute");
    assert_eq!(without_aggregate.to_bits(), sibling_aggregate.to_bits());
    assert_eq!(
        with_aggregate.to_bits(),
        (sibling_aggregate + NAMED_CHILD_MARGINAL).to_bits()
    );
    assert_eq!(
        (with_aggregate - without_aggregate).to_bits(),
        NAMED_CHILD_MARGINAL.to_bits(),
        "disabling only the named child must remove exactly its ancestor marginal"
    );
    let with_leaves: Vec<f32> = with_child
        .leaf_allocation_bits
        .iter()
        .map(|bits| f32::from_bits(*bits))
        .collect();
    let without_leaves: Vec<f32> = without_child
        .leaf_allocation_bits
        .iter()
        .map(|bits| f32::from_bits(*bits))
        .collect();
    assert!(without_leaves.iter().all(|allocation| *allocation > 0.0));
    let with_leaf_sum: f32 = with_leaves.iter().copied().sum();
    let without_leaf_sum: f32 = without_leaves.iter().copied().sum();
    let allocation_diff_error = (with_leaf_sum - without_leaf_sum - NAMED_CHILD_MARGINAL).abs();
    let allocation_diff_bound =
        allocator_eps_bound(LEAF_WEIGHTS.len(), ROOT_BUDGET + with_aggregate)
            + allocator_eps_bound(LEAF_WEIGHTS.len(), ROOT_BUDGET + without_aggregate);
    assert!(
        allocation_diff_error <= allocation_diff_bound,
        "downstream leaf allocation differential must retain the selected marginal: error={allocation_diff_error} bound={allocation_diff_bound}"
    );
    assert!(
        with_child.n_bands >= 12,
        "D=3 governed conservation requires recursive, residual, and integration OrderBands"
    );

    let owner_budget = ROOT_BUDGET + with_aggregate;
    let owner_residual = owner_budget - with_leaf_sum;
    let owner_bound = allocator_eps_bound(LEAF_WEIGHTS.len(), owner_budget);
    let measured_owner_delta = f32::from_bits(with_child.owner_balance_delta_bits);
    assert_ne!(
        owner_residual, 0.0,
        "live proof requires non-zero f32 residual"
    );
    assert_ne!(
        measured_owner_delta, 0.0,
        "governed Balance delta must bite"
    );
    assert!((measured_owner_delta - owner_residual).abs() <= owner_bound);
    assert_eq!(
        f32::from_bits(with_child.owner_balance_rate_bits).to_bits(),
        measured_owner_delta.to_bits()
    );
    let root_residual = ROOT_BUDGET - f32::from_bits(with_child.owner_allocation_bits);
    let measured_root_delta = f32::from_bits(with_child.root_balance_delta_bits);
    let measured_root_rate = f32::from_bits(with_child.root_balance_rate_bits);
    assert_eq!(measured_root_rate.to_bits(), root_residual.to_bits());
    assert!((measured_root_delta - root_residual).abs() <= allocator_eps_bound(1, ROOT_BUDGET));
    assert!(with_child
        .leaf_balance_delta_bits
        .iter()
        .all(|bits| *bits == 0.0_f32.to_bits()));

    let (balance_disconnected, disconnected_report) =
        execute_step(NAMED_CHILD_MARGINAL, false, default_profile);
    assert_eq!(
        balance_disconnected.owner_aggregate_bits,
        with_child.owner_aggregate_bits
    );
    assert_eq!(
        balance_disconnected.owner_allocation_bits,
        with_child.owner_allocation_bits
    );
    assert_eq!(
        balance_disconnected.leaf_allocation_bits,
        with_child.leaf_allocation_bits
    );
    assert_eq!(
        balance_disconnected.owner_balance_rate_bits,
        0.0_f32.to_bits(),
        "disconnecting governed Balance must remove the Arena-generated residual route"
    );
    assert_eq!(
        balance_disconnected.owner_balance_delta_bits,
        0.0_f32.to_bits(),
        "disconnecting only governed_by must leave actual Balance unchanged"
    );
    assert!(matches!(
        check_allocator_step(&allocator_from_disbursements(
            owner_budget,
            with_leaves.clone(),
            Some(f32::from_bits(
                balance_disconnected.owner_balance_delta_bits
            )),
        )),
        Err(AllocatorConservationViolation::ResidualNotIntegrated { .. })
    ));
    assert!(!disconnected_report.allocator_ok);
    assert!(disconnected_report.allocator_errors.iter().any(|error| {
        matches!(
            error,
            AllocatorConservationViolation::ResidualNotIntegrated { .. }
        )
    }));

    let (disabled, _disabled_report) = execute_step(
        NAMED_CHILD_MARGINAL,
        true,
        ResourceFlowExecutionProfile::DefaultDisabled,
    );
    assert_eq!(
        disabled.flag_source,
        ResourceFlowFlagSource::DefaultDisabled
    );
    assert!(!disabled.rf_active);
    assert_eq!(disabled.n_bands, 0);
    assert_eq!(disabled.owner_aggregate_bits, 0.0_f32.to_bits());
    assert_eq!(disabled.owner_allocation_bits, 0.0_f32.to_bits());
    assert!(disabled
        .leaf_allocation_bits
        .iter()
        .all(|bits| *bits == 0.0_f32.to_bits()));
    eprintln!(
        "RF2-EXECUTED-DEFAULT: depth=3 bands={} named_child_marginal={} sibling_aggregate={} owner_aggregate_with={} owner_aggregate_without={} leaf_allocations_with={:?} leaf_allocations_without={:?} owner_residual={} arena_generated_owner_rate={} owner_balance_delta={} rf1_allocator=PASS rf1_structural=PASS rf1_recipe=VACUOUS deterministic_bits=PASS economy_execution_deferred=false",
        with_child.n_bands,
        NAMED_CHILD_MARGINAL,
        sibling_aggregate,
        with_aggregate,
        without_aggregate,
        with_leaves,
        without_leaves,
        owner_residual,
        f32::from_bits(with_child.owner_balance_rate_bits),
        measured_owner_delta,
    );
    eprintln!(
        "RF2-RUNTIME-BALANCE-REMOVED: owner_budget={} leaf_allocations={:?} residual={} owner_rate={} actual_owner_delta={} result=ResidualNotIntegrated",
        owner_budget,
        with_leaves,
        owner_residual,
        f32::from_bits(balance_disconnected.owner_balance_rate_bits),
        f32::from_bits(balance_disconnected.owner_balance_delta_bits),
    );
    eprintln!(
        "RF2-DEFAULT-DISABLED: flag_source={:?} rf_active={} bands={} owner_aggregate={} owner_allocation={} leaf_allocations={:?}",
        disabled.flag_source,
        disabled.rf_active,
        disabled.n_bands,
        f32::from_bits(disabled.owner_aggregate_bits),
        f32::from_bits(disabled.owner_allocation_bits),
        disabled
            .leaf_allocation_bits
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect::<Vec<_>>(),
    );
}
