//! RF-CONSERVATION-ORACLE-0 — independent conservation oracle live validation.
//!
//! Validates the oracle against the CURRENT executed flat path
//! (`SimSession::open_from_spec` + `ResourceFlowOptInMode::FlatStarOptIn`),
//! mirroring ct_2a_intrinsic_flow / ct_2c_category_economy posture.
//!
//! Independence fence: this file must not import `owner_silo_recursive_rf_source`
//! or the recursive branch of `runtime_rf_tick_source`.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::{
    AccumulatorOp, AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, ColumnIndex,
    CombineFn, ConsumeMode, DimensionRegistry, EmlExpressionRegistry, GateSpec, LogTier, ScaleSpec,
    SimThing, SimThingKind, SlotIndex, SourceSpec, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    allocator_from_disbursements, build_execution_plan, check_allocator_step, check_conservation,
    check_recipe_exact, flat_star_observations, plan_arena_allocation,
    register_child_share_formula, resolve_node_columns, run_arena_allocation_oracle,
    AllocatorConservationViolation, ArenaStructuralEvidence, RecipeInvocationObservation, Scenario,
    SimSession,
};
use simthing_gpu::{build_governed_pairs, GpuContext, SlotAllocator};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowOptInMode, ResourceFlowSpec, SpecVersion,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

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

fn register_food_flow(reg: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "simthing".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "ct2a_food".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "ct2a_food".into(),
                },
            ),
            balance_rate_subfield(),
            balance_subfield(),
        ],
    };
    compile_property(&spec, reg).expect("register food_flow");
}

fn flat_star_scenario(hosted_count: usize) -> Scenario {
    let mut registry = DimensionRegistry::new();
    register_food_flow(&mut registry);
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..hosted_count {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    Scenario {
        name: "rf_conservation_flat".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn flat_star_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "rf_conservation_flat".into(),
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
            arenas: vec![ArenaSpec {
                name: "ct2a_food".into(),
                flow_property: PropertyKey::new("simthing", "food_flow"),
                balance_property: Some(PropertyKey::new("simthing", "food_flow")),
                max_participants: 16,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: Vec::new(),
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
    }
}

fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap().raw(), c.id.raw()))
        .collect();
    game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = participants;
}

fn open_from_spec_fail_closed(scenario: Scenario, game_mode: &GameModeSpec) -> SimSession {
    SimSession::open_from_spec(scenario, game_mode)
        .expect("RF conservation live proof requires a supported GPU adapter")
}

fn gpu_gate_fail_closed() -> std::sync::MutexGuard<'static, ()> {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    assert!(
        std::env::var_os("SIMTHING_RF_FORCE_NO_ADAPTER").is_none(),
        "simulated NoAdapter: RF conservation live proof fails closed"
    );
    GpuContext::new_blocking()
        .expect("RF conservation live proof requires a supported GPU adapter");
    guard
}

fn idx(slot: SlotIndex, col: u32, n_dims: u32) -> usize {
    (slot.raw() * n_dims + col) as usize
}

fn cell(values: &[f32], slot: SlotIndex, col: u32, n_dims: u32) -> f32 {
    values[idx(slot, col, n_dims)]
}

#[derive(Debug)]
struct LiveFlatStarObservation {
    root: SlotIndex,
    leaves: Vec<SlotIndex>,
    root_intrinsic: f32,
    leaf_alloc: Vec<f32>,
    measured_root_balance_rate: f32,
    measured_root_balance_delta: f32,
    measured_leaf_balance_deltas: Vec<Option<f32>>,
}

fn execute_live_flat_star(connect_root_balance: bool) -> LiveFlatStarObservation {
    let scenario = flat_star_scenario(8);
    let mut game_mode = flat_star_game_mode();
    fill_explicit_participants(&mut game_mode, &scenario);
    // Mirror ct_2a: scenario registry carries columns; game-mode properties cleared
    // after participants are filled so open_from_spec avoids duplicate compile.
    assert_eq!(
        game_mode.resource_flow.as_ref().unwrap().opt_in_mode,
        ResourceFlowOptInMode::FlatStarOptIn
    );
    let mut session = open_from_spec_fail_closed(scenario, &game_mode);
    assert!(
        session.proto.flags.use_accumulator_resource_flow,
        "FlatStarOptIn must enable accumulator RF"
    );

    let flow_id = session
        .proto
        .registry
        .id_of("simthing", "food_flow")
        .expect("food_flow registered");
    let cols = resolve_node_columns(
        &session.proto.registry.property(flow_id).layout,
        "ct2a_food",
    )
    .expect("column refs");
    let balance_col = cols.balance_col.expect("fixture must expose Balance");
    let balance_rate_col = session
        .proto
        .registry
        .property(flow_id)
        .layout
        .offset_of(&SubFieldRole::Named("balance_rate".into()))
        .expect("balance_rate column")
        .lane() as u32;
    assert!(
        game_mode.resource_flow.as_ref().unwrap().arenas[0]
            .balance_property
            .is_some(),
        "live fixture must declare its Balance property"
    );
    // Plan from the *admitted* runtime tree (install materializes ArenaParticipant
    // nodes onto proto.root, not scenario.root).
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena");

    let root = layout.participant_roots[0].participant_slot;
    let leaves: Vec<SlotIndex> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    assert_eq!(leaves.len(), 7, "flat-star D=2 expects seven leaves");

    let root_intrinsic = 10.0_f32;
    let leaf_weights = [1.0_f32; 7];
    let mut inputs = HashMap::from([
        ((root, cols.intrinsic_flow_col), root_intrinsic),
        // The live residual path begins with the measured allocator budget and
        // subtracts the executed child disbursements before Balance integration.
        ((root, balance_rate_col), root_intrinsic),
    ]);
    for (slot, &weight) in leaves.iter().zip(leaf_weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }

    let n_dims = session.proto.registry.total_columns as u32;
    let mut flat = session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    let root_balance_before = cell(&flat, root, balance_col, n_dims);
    let leaf_balance_before: Vec<f32> = leaves
        .iter()
        .map(|&leaf| cell(&flat, leaf, balance_col, n_dims))
        .collect();
    session.state.install_resolved_values_at_boundary(&flat);

    let mut oracle_cells = inputs.clone();
    let trace = run_arena_allocation_oracle(&layout, &mut oracle_cells, 1.0);
    assert_eq!(trace.disbursements.len(), 7, "seven leaf disbursements");

    // Execute the admitted flat RF plan (current path — not recursive). Seven
    // existing scaled SlotValue/AddToTarget ops subtract the seven executed
    // child disbursements from the seeded budget on distinct bands. The
    // ordinary governed_by integration is dispatched once, after those
    // subtractions. The negative fixture omits only that final root dispatch;
    // allocation, weights, topology, and balance-rate arithmetic are identical.
    let mut plan = plan_arena_allocation(
        &layout,
        &build_governed_pairs(&session.proto.registry),
        session.state.n_slots,
    )
    .expect("flat allocation plan");
    let root_balance_target = (root, ColumnIndex::new(balance_col as usize));
    let root_balance_rate_target = (root, ColumnIndex::new(balance_rate_col as usize));
    let mut root_governed_op = None;
    for op in &plan.cpu_ops {
        if matches!(&op.combine, CombineFn::IntegrateWithClamp { .. })
            && op.targets.contains(&root_balance_target)
        {
            assert_eq!(
                op.source,
                SourceSpec::SlotValue {
                    slot: root,
                    col: root_balance_rate_target.1,
                }
            );
            let mut governed = op.clone();
            governed.targets.push(root_balance_rate_target);
            governed.gate = GateSpec::OrderBand(0);
            assert!(root_governed_op.replace(governed).is_none());
        }
    }
    let root_governed_op =
        root_governed_op.expect("fixture must expose exactly one root governed Balance pair");
    plan.cpu_ops
        .retain(|op| !matches!(&op.combine, CombineFn::IntegrateWithClamp { .. }));
    let residual_band_start = plan.integration_band;
    for (offset, &leaf) in leaves.iter().enumerate() {
        plan.cpu_ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: leaf,
                col: ColumnIndex::new(cols.allocated_flow_col as usize),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(residual_band_start + offset as u32),
            scale: ScaleSpec::Constant(-1.0),
            consume: ConsumeMode::AddToTarget,
            targets: vec![(root, ColumnIndex::new(balance_rate_col as usize))],
        });
    }
    let n_bands = plan.n_bands.max(residual_band_start + leaves.len() as u32);
    let mut eml = EmlExpressionRegistry::new();
    register_child_share_formula(&mut eml, cols).expect("child-share EML");
    session
        .state
        .sync_resource_flow_ops_from_cpu(&plan.cpu_ops, n_bands, &eml)
        .expect("flat RF + residual arithmetic upload");
    session.state.run_resource_flow_bands(n_bands, 1.0);
    if connect_root_balance {
        session
            .state
            .sync_resource_flow_ops_from_cpu(&[root_governed_op], 1, &eml)
            .expect("root governed Balance upload");
        session.state.run_resource_flow_bands(1, 1.0);
    }
    let gpu_out = session.state.read_values();

    // GPU leaf allocated must match CPU allocation oracle (admitted RF).
    for &leaf in &leaves {
        let cpu = oracle_cells
            .get(&(leaf, cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, cols.allocated_flow_col, n_dims)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {leaf} admitted RF / allocation-oracle parity"
        );
    }

    let leaf_alloc: Vec<f32> = leaves
        .iter()
        .map(|&leaf| cell(&gpu_out, leaf, cols.allocated_flow_col, n_dims))
        .collect();
    let measured_root_balance_delta =
        cell(&gpu_out, root, balance_col, n_dims) - root_balance_before;
    let measured_root_balance_rate = cell(&gpu_out, root, balance_rate_col, n_dims);
    let measured_leaf_balance_deltas: Vec<Option<f32>> = leaves
        .iter()
        .zip(leaf_balance_before.iter())
        .map(|(&leaf, &before)| Some(cell(&gpu_out, leaf, balance_col, n_dims) - before))
        .collect();
    drop(session);

    LiveFlatStarObservation {
        root,
        leaves,
        root_intrinsic,
        leaf_alloc,
        measured_root_balance_rate,
        measured_root_balance_delta,
        measured_leaf_balance_deltas,
    }
}

/// Oracle agrees with admitted flat-opt-in RF (ct_2a posture): zero false positives.
#[test]
fn oracle_agrees_with_flat_star_opt_in_executed_rf() {
    let guard = gpu_gate_fail_closed();

    let connected = execute_live_flat_star(true);
    let sum_alloc: f32 = connected.leaf_alloc.iter().sum();
    let arithmetic_residual = connected.root_intrinsic - sum_alloc;
    let bound =
        simthing_driver::allocator_eps_bound(connected.leaf_alloc.len(), connected.root_intrinsic);
    eprintln!(
        "RF-MEASURED-BALANCE: budget={} sum_disbursed={sum_alloc} residual={arithmetic_residual} bound={bound} root_rate={} root_delta={} leaf_deltas={:?}",
        connected.root_intrinsic,
        connected.measured_root_balance_rate,
        connected.measured_root_balance_delta,
        connected.measured_leaf_balance_deltas
    );
    assert_ne!(
        arithmetic_residual.to_bits(),
        0.0_f32.to_bits(),
        "live governed-Balance proof requires a deterministic non-zero f32 residual"
    );
    assert_ne!(
        connected.measured_root_balance_delta, 0.0,
        "executed governed_by must write the non-zero residual into root Balance"
    );
    assert!(
        (connected.measured_root_balance_delta - arithmetic_residual).abs() <= bound,
        "actual root Balance delta must match arithmetic residual within bound: residual={arithmetic_residual} measured={} bound={bound}",
        connected.measured_root_balance_delta
    );
    let reported_root_balance_delta = if std::env::var_os("SIMTHING_RF_BALANCE_DRIFT").is_some() {
        connected.measured_root_balance_delta + 1.0
    } else {
        connected.measured_root_balance_delta
    };
    let alloc_obs = allocator_from_disbursements(
        connected.root_intrinsic,
        connected.leaf_alloc.clone(),
        Some(reported_root_balance_delta),
    );
    let allocator_result = check_allocator_step(&alloc_obs);
    assert!(
        allocator_result.is_ok(),
        "allocator conservation must hold on admitted flat RF: budget={} sum={sum_alloc} residual={} result={allocator_result:?}",
        connected.root_intrinsic,
        arithmetic_residual
    );

    let (a, mut arena) = flat_star_observations(
        connected.root.raw() as u64,
        &connected
            .leaves
            .iter()
            .map(|s| s.raw() as u64)
            .collect::<Vec<_>>(),
        connected.root_intrinsic,
        &connected.leaf_alloc,
        Some(reported_root_balance_delta),
        &connected.measured_leaf_balance_deltas,
        0.0,
        0.0,
    );
    if std::env::var_os("SIMTHING_RF_ORPHAN_DRIFT").is_some() {
        arena
            .structural_evidence
            .parent_disbursement_recipient_ids
            .pop();
    }
    let report = check_conservation(&[], &[a], &[arena]);
    assert!(
        report.all_pass(),
        "conservation oracle must agree with admitted flat RF (zero false positives): {:?}",
        report
    );

    // Load-bearing paired runtime negative: execute the identical admitted
    // allocation and residual arithmetic with only the root governed Balance
    // integration removed. The actual GPU readout remains zero and must bite.
    let disconnected = execute_live_flat_star(false);
    assert_eq!(disconnected.root, connected.root);
    assert_eq!(disconnected.leaves, connected.leaves);
    assert_eq!(
        disconnected.root_intrinsic.to_bits(),
        connected.root_intrinsic.to_bits()
    );
    assert_eq!(disconnected.leaf_alloc.len(), connected.leaf_alloc.len());
    for (actual, expected) in disconnected.leaf_alloc.iter().zip(&connected.leaf_alloc) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(
        disconnected.measured_root_balance_rate.to_bits(),
        connected.measured_root_balance_rate.to_bits(),
        "positive and negative fixtures must execute identical residual arithmetic"
    );
    assert_ne!(
        disconnected.measured_root_balance_rate, 0.0,
        "negative must preserve the executed non-zero residual rate"
    );
    assert_eq!(
        disconnected.measured_root_balance_delta.to_bits(),
        0.0_f32.to_bits(),
        "disconnected governed path must leave the observed Balance cell unchanged"
    );
    let disconnected_observation = allocator_from_disbursements(
        disconnected.root_intrinsic,
        disconnected.leaf_alloc.clone(),
        Some(disconnected.measured_root_balance_delta),
    );
    let disconnected_error = check_allocator_step(&disconnected_observation)
        .expect_err("actual GPU Balance readout must expose disconnected governed integration");
    assert!(matches!(
        disconnected_error,
        AllocatorConservationViolation::ResidualNotIntegrated { .. }
    ));
    eprintln!(
        "RF-RUNTIME-BALANCE-REMOVED: budget={} sum_disbursed={} residual={} root_rate={} actual_root_delta={} result={disconnected_error:?}",
        disconnected.root_intrinsic,
        disconnected.leaf_alloc.iter().sum::<f32>(),
        disconnected.root_intrinsic - disconnected.leaf_alloc.iter().sum::<f32>(),
        disconnected.measured_root_balance_rate,
        disconnected.measured_root_balance_delta
    );

    // Secondary observation negatives remain: corruption after readout and a
    // missing observation both fail, but neither substitutes for the runtime
    // path falsifier above.
    for measured in [Some(connected.measured_root_balance_delta + 1.0), None] {
        let corrupted = allocator_from_disbursements(
            connected.root_intrinsic,
            connected.leaf_alloc.clone(),
            measured,
        );
        assert!(matches!(
            check_allocator_step(&corrupted),
            Err(AllocatorConservationViolation::ResidualNotIntegrated { .. })
        ));
    }

    // Recipe family still exact on a synthetic conjunctive debit matching ADR identity.
    let recipe = RecipeInvocationObservation {
        need_deltas: vec![-20.0],
        unit_costs: vec![5.0],
        emit_count: 4,
    };
    assert!(check_recipe_exact(&recipe).is_ok());

    drop(guard);
}

/// Pure closed-form bite without GPU: non-conservative fails, conservative passes.
#[test]
fn oracle_bite_nonconservative_fails_conservative_passes() {
    // Conservative flat star.
    let (a_ok, arena_ok) = flat_star_observations(
        10,
        &[11, 12],
        10.0,
        &[2.5, 7.5],
        Some(0.0),
        &[Some(0.0), Some(0.0)],
        0.0,
        0.0,
    );
    let pass = check_conservation(&[], &[a_ok], &[arena_ok]);
    assert!(pass.all_pass(), "conservative must PASS: {:?}", pass);

    // Non-conservative: disburse breaks O(ε·n) bound (stolen mass not in Balance).
    let a_bad = simthing_driver::AllocatorStepObservation {
        budget: 10.0,
        disbursed: vec![1.0, 2.0],
        balance_residual: Some(7.0),
    };
    let fail = check_conservation(&[], &[a_bad], &[]);
    assert!(!fail.all_pass(), "non-conservative must FAIL");
    assert!(!fail.allocator_ok);

    // Orphan participant fails structural.
    let arena_orphan = simthing_driver::ArenaConservationSnapshot {
        participants: vec![
            simthing_driver::ArenaParticipantObservation {
                id: 1,
                is_leaf: false,
                intrinsic_flow: 4.0,
                allocated_flow: 0.0,
                balance_delta: Some(0.0),
            },
            simthing_driver::ArenaParticipantObservation {
                id: 77,
                is_leaf: true,
                intrinsic_flow: 0.0,
                allocated_flow: 4.0,
                balance_delta: Some(0.0),
            },
        ],
        structural_evidence: ArenaStructuralEvidence {
            declared_intrinsic_source_ids: vec![1],
            inbound_coupling_endpoint_ids: Vec::new(),
            parent_disbursement_recipient_ids: Vec::new(),
        },
        inbound_coupling: 0.0,
        emission_consumption: 0.0,
    };
    let orphan_fail = check_conservation(&[], &[], &[arena_orphan]);
    assert!(!orphan_fail.all_pass());
    assert!(!orphan_fail.structural_ok);
}
