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
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingKind, SlotIndex, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    allocator_from_disbursements, build_execution_plan, check_allocator_step, check_conservation,
    check_recipe_exact, flat_star_observations, leaf_allocated_from_cells, resolve_node_columns,
    run_arena_allocation_oracle, RecipeInvocationObservation, Scenario, SessionError, SimSession,
};
use simthing_gpu::{GpuContext, GpuInitError, SlotAllocator};
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
                balance_property: None,
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

fn open_from_spec_or_skip(scenario: Scenario, game_mode: &GameModeSpec) -> Option<SimSession> {
    match SimSession::open_from_spec(scenario, game_mode) {
        Ok(session) => Some(session),
        Err(SessionError::Gpu(GpuInitError::NoAdapter)) => {
            eprintln!("skipping: no GPU");
            None
        }
        Err(err) => panic!("open_from_spec: {err}"),
    }
}

fn gpu_gate() -> Option<std::sync::MutexGuard<'static, ()>> {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    if GpuContext::new_blocking().is_err() {
        eprintln!("skipping: no GPU");
        return None;
    }
    Some(guard)
}

fn idx(slot: SlotIndex, col: u32, n_dims: u32) -> usize {
    (slot.raw() * n_dims + col) as usize
}

/// Oracle agrees with admitted flat-opt-in RF (ct_2a posture): zero false positives.
#[test]
fn oracle_agrees_with_flat_star_opt_in_executed_rf() {
    let Some(guard) = gpu_gate() else {
        return;
    };
    let ctx = GpuContext::new_blocking().expect("gpu gate already checked adapter");

    let scenario = flat_star_scenario(3);
    let mut game_mode = flat_star_game_mode();
    fill_explicit_participants(&mut game_mode, &scenario);
    // Mirror ct_2a: scenario registry carries columns; game-mode properties cleared
    // after participants are filled so open_from_spec avoids duplicate compile.
    assert_eq!(
        game_mode.resource_flow.as_ref().unwrap().opt_in_mode,
        ResourceFlowOptInMode::FlatStarOptIn
    );
    let Some(mut session) = open_from_spec_or_skip(scenario, &game_mode) else {
        return;
    };
    assert!(
        session.proto.flags.use_accumulator_resource_flow,
        "FlatStarOptIn must enable accumulator RF"
    );
    session
        .sync_resource_flow_if_enabled()
        .expect("resource flow sync");

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
    assert_eq!(leaves.len(), 2, "flat-star D=2 expects two leaves");

    let root_intrinsic = 10.0_f32;
    let leaf_weights = [1.0_f32, 3.0];
    let mut inputs = HashMap::from([((root, cols.intrinsic_flow_col), root_intrinsic)]);
    for (slot, &weight) in leaves.iter().zip(leaf_weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }

    let n_dims = session.proto.registry.total_columns as u32;
    let mut flat = session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    session.state.install_resolved_values_at_boundary(&flat);

    let mut oracle_cells = inputs.clone();
    let trace = run_arena_allocation_oracle(&layout, &mut oracle_cells, 1.0);
    assert_eq!(trace.disbursements.len(), 2, "two leaf disbursements");

    // Execute admitted flat RF bands (current path — not recursive).
    session
        .state
        .run_resource_flow_bands(session.state.accumulator_resource_flow_bands, 1.0);
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

    // Conservation oracle over the admitted allocation — must PASS (no false positive).
    let leaf_alloc =
        leaf_allocated_from_cells(&oracle_cells, &leaves, cols.allocated_flow_col);
    let sum_alloc: f32 = leaf_alloc.iter().sum();
    let alloc_obs = allocator_from_disbursements(root_intrinsic, leaf_alloc.clone());
    assert!(
        check_allocator_step(&alloc_obs).is_ok(),
        "allocator conservation must hold on admitted flat RF: budget={root_intrinsic} sum={sum_alloc} residual={}",
        root_intrinsic - sum_alloc
    );

    let (a, arena) = flat_star_observations(
        root.raw() as u64,
        &leaves.iter().map(|s| s.raw() as u64).collect::<Vec<_>>(),
        root_intrinsic,
        &leaf_alloc,
        0.0,
        &vec![0.0; leaf_alloc.len()],
        0.0,
        0.0,
    );
    let report = check_conservation(&[], &[a], &[arena]);
    assert!(
        report.all_pass(),
        "conservation oracle must agree with admitted flat RF (zero false positives): {:?}",
        report
    );

    // Recipe family still exact on a synthetic conjunctive debit matching ADR identity.
    let recipe = RecipeInvocationObservation {
        need_deltas: vec![-20.0],
        unit_costs: vec![5.0],
        emit_count: 4,
    };
    assert!(check_recipe_exact(&recipe).is_ok());

    drop(session);
    drop(ctx);
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
        0.0,
        &[0.0, 0.0],
        0.0,
        0.0,
    );
    let pass = check_conservation(&[], &[a_ok], &[arena_ok]);
    assert!(pass.all_pass(), "conservative must PASS: {:?}", pass);

    // Non-conservative: disburse breaks O(ε·n) bound (stolen mass not in Balance).
    let a_bad = simthing_driver::AllocatorStepObservation {
        budget: 10.0,
        disbursed: vec![1.0, 2.0],
        balance_residual: 7.0,
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
                balance_delta: 0.0,
                has_declared_lineage: true,
            },
            simthing_driver::ArenaParticipantObservation {
                id: 77,
                is_leaf: true,
                intrinsic_flow: 0.0,
                allocated_flow: 4.0,
                balance_delta: 0.0,
                has_declared_lineage: false,
            },
        ],
        inbound_coupling: 0.0,
        emission_consumption: 0.0,
    };
    let orphan_fail = check_conservation(&[], &[], &[arena_orphan]);
    assert!(!orphan_fail.all_pass());
    assert!(!orphan_fail.structural_ok);
}
