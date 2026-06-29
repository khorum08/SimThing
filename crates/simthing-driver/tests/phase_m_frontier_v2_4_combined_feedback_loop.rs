//! FrontierV2-4 — Combined movement + structural feedback loop (Tier-2, test-only).
//!
//! Combines V2-2 own-column movement shadow and V2-3 BoundaryRequest structural shadow
//! in one bounded four-tick closed-loop fixture.

#[path = "support/e11_flat_star.rs"]
mod e11_flat_star;
#[path = "support/field_policy_v1_live_pipeline.rs"]
mod field_policy_v1_live_pipeline;
#[path = "support/field_policy_v1_route_replay.rs"]
mod field_policy_v1_route_replay;
#[path = "support/frontier_v2.rs"]
mod frontier_v2;

use std::sync::Mutex;

use e11_flat_star::{
    fill_explicit_participants, flat_star_cell_inputs, flat_star_game_mode, flat_star_scenario,
    leaf_slots, root_slot, FlatStarSession,
};
use field_policy_v1_live_pipeline::{
    cpu_pipe0_expected_records, cpu_threshold_state_event, default_admitted_count,
    default_admitted_table, frontier_field_observer_rows, pipe0_records_to_act2, rules_for_smoke,
    run_act2_chain_gpu, run_pipe0_gpu, smoke_admission_rules, verify_act2_chain_admission,
    ObserverRow,
};
use field_policy_v1_route_replay::validate_field_policy_v1_consumed;
use frontier_v2::*;
use simthing_driver::{
    build_execution_plan, compiled_stencil_to_gpu_config, resolve_node_columns,
    run_arena_allocation_oracle, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
    SimSession,
};
use simthing_gpu::{cpu_horizon, params_from_config, GpuContext, StructuredFieldStencilConfig};
use simthing_spec::{
    compile_region_field_preview, landed_jit_kernel_descriptors, MappingExecutionProfile,
    RegionFieldSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

pub const FRONTIER_V2_4_COMBINED_FEEDBACK_FINGERPRINT: &str = "dbb54b952f9face8";

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let Some(ctx) = field_policy_v1_live_pipeline::try_gpu() else {
        eprintln!("skipping GPU assertions: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn smoke_fixture() -> (FrontierV1ScenarioSkeleton, FrontierV1FixtureConfig) {
    (frontier_v2_smoke_skeleton(), frontier_v1_1_fixture_config())
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn cpu_caller_managed_field(
    config: &StructuredFieldStencilConfig,
    seeds: &[(u32, u32, f32)],
) -> Vec<f32> {
    let params = params_from_config(config);
    let mut values = vec![0.0f32; config.values_len()];
    for &(row, col, value) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = value;
    }
    values = cpu_horizon(&values, &params, 1);
    for &(row, col, _) in seeds {
        let slot = row * config.width + col;
        values[idx(slot, config.source_col, config.n_dims)] = 0.0;
    }
    cpu_horizon(&values, &params, config.horizon)
}

fn assert_gpu_field_matches_cpu_reference(
    spec: &RegionFieldSpec,
    field_values: &[f32],
    seeds: &[(u32, u32, f32)],
) {
    let preview = compile_region_field_preview(spec).expect("mapping spec admits");
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let expected = cpu_caller_managed_field(&config, seeds);
    assert_eq!(field_values.len(), expected.len());
    for (i, (&gpu, &cpu)) in field_values.iter().zip(expected.iter()).enumerate() {
        assert!(
            (gpu - cpu).abs() <= 0.0001,
            "GPU/CPU mapping parity mismatch at {i}: gpu={gpu} cpu={cpu}"
        );
    }
}

fn gpu_seeds(
    config: &FrontierV1FixtureConfig,
    movement_shadow: Option<&FrontierV2OwnColumnShadow>,
) -> Vec<FirstSliceSeed> {
    let ((row_a, col_a), (row_b, col_b)) = source_seed_placement(config, movement_shadow);
    vec![
        FirstSliceSeed {
            row: row_a,
            col: col_a,
            value: config.district_output_a as f32,
        },
        FirstSliceSeed {
            row: row_b,
            col: col_b,
            value: config.district_output_b as f32,
        },
    ]
}

fn seed_tuples(
    config: &FrontierV1FixtureConfig,
    movement_shadow: Option<&FrontierV2OwnColumnShadow>,
) -> Vec<(u32, u32, f32)> {
    let ((row_a, col_a), (row_b, col_b)) = source_seed_placement(config, movement_shadow);
    vec![
        (row_a, col_a, config.district_output_a as f32),
        (row_b, col_b, config.district_output_b as f32),
    ]
}

fn open_flat_star_gpu() -> FlatStarSession {
    let scenario = flat_star_scenario(3, 32);
    let mut game_mode = flat_star_game_mode(16);
    game_mode.resource_flow.as_mut().unwrap().opt_in_mode = ResourceFlowOptInMode::FlatStarOptIn;
    game_mode.resource_flow_execution_profile = ResourceFlowExecutionProfile::FlatStarResourceFlow;
    fill_explicit_participants(&mut game_mode, &scenario);

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert!(session.state.accumulator_resource_flow_active);

    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow registered");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("column refs");
    let layout = build_execution_plan_from_authoring(
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

    FlatStarSession {
        session,
        layout,
        cols,
    }
}

fn f32_alloc_to_u32(v: f32) -> u32 {
    assert!(v.is_finite() && v >= 0.0);
    v.round() as u32
}

fn run_gpu_flat_star_allocation(
    fx: &mut FlatStarSession,
    allocator_total: u32,
) -> GpuResourceFlowAllocationSummary {
    fx.session
        .sync_resource_flow_if_enabled()
        .expect("resource flow sync");

    let root = root_slot(&fx.layout);
    let leaves = leaf_slots(&fx.layout);
    let (weight_a, weight_b) = frontier_v1_flat_star_weights();
    let inputs = flat_star_cell_inputs(
        root,
        &leaves,
        fx.cols,
        allocator_total as f32,
        &[weight_a, weight_b],
    );

    let n_dims = fx.session.proto.registry.total_columns as u32;
    let mut flat = fx.session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    fx.session.state.write_values(&flat);

    let mut oracle = inputs.clone();
    run_arena_allocation_oracle(&fx.layout, &mut oracle, 1.0);

    fx.session
        .state
        .run_resource_flow_bands(fx.session.state.accumulator_resource_flow_bands, 1.0);

    let gpu_out = fx.session.state.read_values();
    let mut gpu_a = 0.0f32;
    let mut gpu_b = 0.0f32;
    for (i, &leaf) in leaves.iter().enumerate() {
        let cpu = oracle
            .get(&(leaf, fx.cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, fx.cols.allocated_flow_col, n_dims)];
        assert_eq!(cpu.to_bits(), gpu.to_bits(), "leaf {leaf} E-11 parity");
        if i == 0 {
            gpu_a = gpu;
        } else {
            gpu_b = gpu;
        }
    }

    GpuResourceFlowAllocationSummary {
        faction_a_allocation: f32_alloc_to_u32(gpu_a),
        faction_b_allocation: f32_alloc_to_u32(gpu_b),
        allocator_total,
        resource_overflow_flags: 0,
        allocator_route_code: FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
    }
}

fn assert_observer_rows_cpu_oracle(rows: &[ObserverRow]) {
    for (i, row) in rows.iter().enumerate() {
        let (state, event_code, score) = cpu_threshold_state_event(row);
        assert_eq!(
            event_code, FRONTIER_V1_RESOURCE_EVENT_CODE,
            "row {i} event_code"
        );
        assert_eq!(state, 1, "row {i} state");
        assert!(score >= 500, "row {i} score {score}");
    }
}

struct FrontierV2CombinedFeedbackRun {
    summary: FrontierV2CombinedFeedbackSummary,
    tick0: FrontierV2TickRun,
    tick1: FrontierV2TickRun,
    tick2: FrontierV2TickRun,
    tick3: FrontierV2TickRun,
    movement_before: FrontierV2OwnColumnShadow,
    movement_after: FrontierV2OwnColumnShadow,
    boundary_before: FrontierV2BoundaryRequestShadow,
    boundary_after: FrontierV2BoundaryRequestShadow,
    combined_feedback_code: u32,
}

fn run_single_live_tick(
    ctx: &GpuContext,
    skeleton: &FrontierV1ScenarioSkeleton,
    config: &FrontierV1FixtureConfig,
    tick_index: u32,
    source_unit_id: u32,
    movement_shadow: Option<&FrontierV2OwnColumnShadow>,
) -> FrontierV2TickRun {
    let spec = frontier_v1_mapping_field_spec();
    let mut mapping_session =
        FirstSliceMappingSession::open(ctx, MappingExecutionProfile::SparseRegionFieldV1, &spec)
            .expect("mapping session opens");
    mapping_session
        .queue_seeds(&gpu_seeds(config, movement_shadow))
        .expect("queue seeds");
    let mapping_report = mapping_session
        .tick(ctx, FirstSliceTickOptions::debug_readback(), (0.2, 0.1))
        .expect("mapping tick");
    assert!(mapping_report.enabled);
    assert!(mapping_report.reduction_executed);
    assert!(mapping_report.eml_executed);
    let field_values = mapping_report
        .field_values
        .expect("debug readback field values");
    assert_gpu_field_matches_cpu_reference(
        &spec,
        &field_values,
        &seed_tuples(config, movement_shadow),
    );

    let (threat, urgency) = mapping_session
        .diagnostic_readback_reduction_eml(ctx, (0.2, 0.1))
        .expect("reduction/eml readback");

    let mapping_hash = hash_gpu_field_values(&field_values);
    let observer_rows = frontier_field_observer_rows(urgency, threat);
    assert_observer_rows_cpu_oracle(&observer_rows);

    let pipe_capacity = observer_rows.len() as u32;
    let pipe0 = run_pipe0_gpu(ctx, &observer_rows, pipe_capacity, 1, true);
    let expected_records = cpu_pipe0_expected_records(
        &field_policy_v1_live_pipeline::cpu_event_rows(&observer_rows),
    );
    assert_eq!(pipe0.event_count(), expected_records.len() as u32);
    assert_eq!(pipe0.overflow(), 0);
    assert!(field_policy_v1_live_pipeline::cpu_pipe0_membership_exact(
        &expected_records,
        pipe0.records(),
    ));

    let compact = pipe0_records_to_act2(pipe0.records());
    let rules = rules_for_smoke();
    let admitted = default_admitted_table();
    let admitted_n = default_admitted_count();
    let adm_rules = smoke_admission_rules();
    let bucket_cap = 8u32;
    let prop_cap = 8u32;
    let act2 = run_act2_chain_gpu(
        ctx, &compact, bucket_cap, &rules, prop_cap, &admitted, admitted_n, &adm_rules, 1,
    );
    verify_act2_chain_admission(
        &act2, &compact, bucket_cap, &rules, prop_cap, &admitted, admitted_n, &adm_rules,
    );

    let cpu_output = run_frontier_v1_fixture(skeleton, config);
    let allocator_total = cpu_output
        .resource_flow
        .allocated_a
        .saturating_add(cpu_output.resource_flow.allocated_b);

    let mut fx = open_flat_star_gpu();
    let gpu_rf = run_gpu_flat_star_allocation(&mut fx, allocator_total);
    assert_eq!(
        gpu_rf.faction_a_allocation,
        cpu_output.resource_flow.allocated_a
    );
    assert_eq!(
        gpu_rf.faction_b_allocation,
        cpu_output.resource_flow.allocated_b
    );
    assert_eq!(
        gpu_rf.allocator_route_code,
        FRONTIER_V1_ALLOCATOR_ROUTE_CODE
    );

    let mut overflow_flags = 0u32;
    if cpu_output.mapping.overflow {
        overflow_flags |= 1;
    }
    if cpu_output.resource_flow.overflow {
        overflow_flags |= 2;
    }
    if pipe0.overflow() != 0 {
        overflow_flags |= 8;
    }
    if act2.proposal_overflow() != 0 {
        overflow_flags |= 16;
    }

    let dispatch_count = act2.proposal_count();
    let field_feedback_code = act2.admission().admission_code();
    let feedback = build_feedback_candidate(
        tick_index,
        source_unit_id,
        FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
        FRONTIER_V1_RESOURCE_PROPOSAL_CODE,
        dispatch_count,
        gpu_rf,
        field_feedback_code,
        overflow_flags,
    );

    let field_agent_hash = hash_live_field_agent_gpu_execution(
        pipe0.event_count(),
        pipe0.overflow(),
        act2.proposal_count(),
        act2.proposal_overflow(),
        act2.admission().admission_code(),
        act2.admission().flags(),
    );
    let proposal_dispatch_hash = hash_tick_proposal_dispatch(
        act2.proposal_count(),
        act2.admission().admission_code(),
        dispatch_count,
    );
    let movement = build_evolved_movement_candidate(&feedback, mapping_hash, urgency, tick_index);
    let structural = build_evolved_structural_candidate(&feedback, mapping_hash, tick_index);

    FrontierV2TickRun {
        tick_index,
        mapping_hash,
        field_agent_hash,
        proposal_dispatch_hash,
        feedback,
        movement,
        structural,
        threat,
        urgency,
        proposal_count: act2.proposal_count(),
    }
}

fn run_combined_feedback_loop(ctx: &GpuContext) -> FrontierV2CombinedFeedbackRun {
    let (skeleton, base_config) = smoke_fixture();
    let admission = validate_frontier_v2_admission(&skeleton);
    assert!(admission.accepted, "{:?}", admission.rejected_reasons);

    let field_policy_consumed = validate_field_policy_v1_consumed();
    assert!(field_policy_consumed.pipe0_registered);
    assert!(field_policy_consumed.act2_registered);

    let movement_before = initial_own_column_shadow(0);
    let boundary_before = empty_boundary_request_shadow(0);

    let tick0 = run_single_live_tick(ctx, &skeleton, &base_config, 0, 0, None);

    let tick1_config = apply_feedback_to_config(&base_config, &tick0.feedback);
    let tick1 = run_single_live_tick(ctx, &skeleton, &tick1_config, 1, 0, None);

    let movement_after = apply_movement_to_own_column_shadow(
        &movement_before,
        &tick1.movement,
        base_config.grid_size,
        1,
    )
    .expect("own-column movement write");
    let boundary_after = apply_structural_to_boundary_request_shadow(&tick1.structural, 0, 1)
        .expect("BoundaryRequest shadow write");

    let (tick2_config, combined_feedback_code) =
        apply_combined_feedback_to_config(&tick1_config, &tick1.feedback, &boundary_after);
    let tick2 = run_single_live_tick(ctx, &skeleton, &tick2_config, 2, 0, Some(&movement_after));

    let (tick3_config, _) =
        apply_combined_feedback_to_config(&tick2_config, &tick2.feedback, &boundary_after);
    let tick3 = run_single_live_tick(ctx, &skeleton, &tick3_config, 3, 0, Some(&movement_after));

    let combined_feedback_delta_hash = hash_combined_feedback_delta(
        movement_after,
        boundary_after,
        combined_feedback_code,
        tick1.mapping_hash,
        tick2.mapping_hash,
        tick3.mapping_hash,
    );
    let overflow_flags = tick0.feedback.overflow_flags
        | tick1.feedback.overflow_flags
        | tick2.feedback.overflow_flags
        | tick3.feedback.overflow_flags;

    let summary = FrontierV2CombinedFeedbackSummary {
        tick0_movement_hash: hash_movement_candidate(tick0.movement),
        tick1_movement_hash: hash_movement_candidate(tick1.movement),
        tick0_structural_hash: hash_structural_candidate(tick0.structural),
        tick1_structural_hash: hash_structural_candidate(tick1.structural),
        movement_shadow_before_hash: hash_own_column_shadow(movement_before),
        movement_shadow_after_hash: hash_own_column_shadow(movement_after),
        boundary_shadow_before_hash: hash_boundary_request_shadow(boundary_before),
        boundary_shadow_after_hash: hash_boundary_request_shadow(boundary_after),
        combined_feedback_code,
        tick1_mapping_hash: tick1.mapping_hash,
        tick2_mapping_hash: tick2.mapping_hash,
        tick3_mapping_hash: tick3.mapping_hash,
        tick2_proposal_dispatch_hash: tick2.proposal_dispatch_hash,
        tick3_proposal_dispatch_hash: tick3.proposal_dispatch_hash,
        combined_feedback_delta_hash,
        overflow_flags,
        tick0_resource_route_status: FrontierV2FieldStatus::GpuVerified,
        tick1_resource_route_status: FrontierV2FieldStatus::GpuVerified,
        tick2_resource_route_status: FrontierV2FieldStatus::GpuVerified,
        tick3_resource_route_status: FrontierV2FieldStatus::GpuVerified,
        movement_application_status: FrontierV2WriteClassification::OwnColumnShadowWrite,
        structural_application_status: FrontierV2WriteClassification::BoundaryRequestShadowWrite,
        combined_feedback_status: FrontierV2FieldStatus::FixtureOnly,
        clause_thing_status: FrontierV2ClauseThingStatus::NotImplemented,
        phase_closure_status: FrontierV2PhaseClosureStatus::NotDeclared,
    };

    FrontierV2CombinedFeedbackRun {
        summary,
        tick0,
        tick1,
        tick2,
        tick3,
        movement_before,
        movement_after,
        boundary_before,
        boundary_after,
        combined_feedback_code,
    }
}

#[test]
fn frontier_v2_4_happy_path_combined_feedback_loop_runs() {
    with_gpu(|ctx| {
        let (skeleton, _) = smoke_fixture();
        let run = run_combined_feedback_loop(ctx);

        assert!(validate_frontier_v2_admission(&skeleton).accepted);
        for status in [
            run.summary.tick0_resource_route_status,
            run.summary.tick1_resource_route_status,
            run.summary.tick2_resource_route_status,
            run.summary.tick3_resource_route_status,
        ] {
            assert_eq!(status, FrontierV2FieldStatus::GpuVerified);
        }
        assert_eq!(
            run.summary.movement_application_status,
            FrontierV2WriteClassification::OwnColumnShadowWrite
        );
        assert_eq!(
            run.summary.structural_application_status,
            FrontierV2WriteClassification::BoundaryRequestShadowWrite
        );
        assert_ne!(run.movement_before, run.movement_after);
        assert!(run.boundary_after.applied);
        assert!(!skeleton.enabled_by_default);

        println!(
            "frontier_v2_4_happy: fixture_id={FRONTIER_V2_4_FIXTURE_ID} fp={} movement=({}, {}) boundary={} code={}",
            run.summary.combined_hex(),
            run.movement_after.row,
            run.movement_after.col,
            run.boundary_after.boundary_request_code,
            run.combined_feedback_code,
        );
    });
}

#[test]
fn frontier_v2_4_combined_feedback_changes_downstream_tick() {
    with_gpu(|ctx| {
        let run = run_combined_feedback_loop(ctx);
        assert_ne!(run.tick2.mapping_hash, run.tick1.mapping_hash);
        assert_ne!(run.tick3.mapping_hash, run.tick2.mapping_hash);
        println!(
            "frontier_v2_4_delta: tick1_map={} tick2_map={} tick3_map={}",
            run.tick1.mapping_hash, run.tick2.mapping_hash, run.tick3.mapping_hash,
        );
    });
}

#[test]
fn frontier_v2_4_movement_write_remains_own_column_only() {
    let shadow = initial_own_column_shadow(0);
    let mut cross_entity = build_evolved_movement_candidate(
        &FrontierV1LiveFieldAgentFeedbackCandidate {
            tick_index: 1,
            source_unit_id: 1,
            route_code: FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
            proposal_code: FRONTIER_V1_RESOURCE_PROPOSAL_CODE,
            dispatch_count: 2,
            faction_a_allocation: 199,
            faction_b_allocation: 133,
            allocator_total: 332,
            field_feedback_code: 5039,
            overflow_flags: 0,
        },
        0,
        0.5,
        1,
    );
    cross_entity.source_unit_id = 1;

    let err = apply_movement_to_own_column_shadow(
        &shadow,
        &cross_entity,
        frontier_v1_1_fixture_config().grid_size,
        1,
    )
    .expect_err("cross-entity write must reject");
    assert_eq!(
        err,
        FrontierV2MovementWriteError::CrossEntityTarget {
            source_unit_id: 1,
            shadow_unit_id: 0,
        }
    );

    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    assert!(!sim_lib.contains("FrontierV2OwnColumnShadow"));

    println!("frontier_v2_4_movement: own_column_only=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_structural_write_remains_boundary_shadow_only() {
    let valid = FrontierV2StructuralCandidate {
        proposal_code: 1,
        boundary_request_code: 5039,
        route_code: FRONTIER_V1_STRUCTURAL_ROUTE_CODE,
        dispatch_count: 2,
    };
    assert!(apply_structural_to_boundary_request_shadow(&valid, 0, 1).is_ok());

    let invalid = FrontierV2StructuralCandidate {
        proposal_code: 1,
        boundary_request_code: 5039,
        route_code: FRONTIER_V1_MOVEMENT_ROUTE_CODE,
        dispatch_count: 2,
    };
    assert!(apply_structural_to_boundary_request_shadow(&invalid, 0, 1).is_err());

    let (skeleton, _) = smoke_fixture();
    let mut commitment = skeleton;
    commitment.field_policy.cpu_commitment_emission = true;
    assert!(!validate_frontier_v2_admission(&commitment).accepted);

    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    assert!(!sim_lib.contains("FrontierV2BoundaryRequestShadow"));

    println!(
        "frontier_v2_4_structural: boundary_shadow_only=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}"
    );
}

#[test]
fn frontier_v2_4_cpu_oracle_parity() {
    with_gpu(|ctx| {
        let (skeleton, base_config) = smoke_fixture();
        let run = run_combined_feedback_loop(ctx);

        let tick1_config = apply_feedback_to_config(&base_config, &run.tick0.feedback);
        let (tick2_config, code2) = apply_combined_feedback_to_config(
            &tick1_config,
            &run.tick1.feedback,
            &run.boundary_after,
        );
        let (tick3_config, _) = apply_combined_feedback_to_config(
            &tick2_config,
            &run.tick2.feedback,
            &run.boundary_after,
        );

        assert_eq!(code2, run.combined_feedback_code);

        for (tick, config, movement_shadow) in [
            (&run.tick0, base_config, None),
            (&run.tick1, tick1_config, None),
            (&run.tick2, tick2_config, Some(run.movement_after)),
            (&run.tick3, tick3_config, Some(run.movement_after)),
        ] {
            let exp_movement = build_evolved_movement_candidate(
                &tick.feedback,
                tick.mapping_hash,
                tick.urgency,
                tick.tick_index,
            );
            let exp_structural = build_evolved_structural_candidate(
                &tick.feedback,
                tick.mapping_hash,
                tick.tick_index,
            );
            assert_eq!(tick.movement, exp_movement);
            assert_eq!(tick.structural, exp_structural);

            if tick.tick_index >= 2 {
                let ((row_a, col_a), _) = source_seed_placement(&config, movement_shadow.as_ref());
                assert_eq!(row_a, run.movement_after.row);
                assert_eq!(col_a, run.movement_after.col);
            }

            let cpu = run_frontier_v1_fixture(&skeleton, &config);
            let oracle = cpu_live_field_agent_oracle(
                &skeleton,
                &config,
                tick.tick_index,
                0,
                tick.proposal_count,
                tick.feedback.field_feedback_code,
            );
            assert_eq!(tick.feedback.route_code, oracle.resource_route_code);
            assert_eq!(tick.feedback.allocator_total, oracle.allocator_total);
            assert_eq!(
                tick.feedback.faction_a_allocation,
                cpu.resource_flow.allocated_a
            );
            assert_eq!(
                tick.feedback.faction_b_allocation,
                oracle.faction_b_allocation
            );
            assert_eq!(oracle.invalid_route_count, 0);
            assert_eq!(tick.feedback.overflow_flags, oracle.overflow_flags);
        }

        println!(
            "frontier_v2_4_parity: fp={} delta={}",
            run.summary.combined_hex(),
            run.summary.combined_feedback_delta_hash,
        );
    });
}

#[test]
fn frontier_v2_4_replay_reproducibility() {
    with_gpu(|ctx| {
        let run_a = run_combined_feedback_loop(ctx);
        let run_b = run_combined_feedback_loop(ctx);

        assert_eq!(run_a.summary, run_b.summary);
        assert_eq!(run_a.tick0, run_b.tick0);
        assert_eq!(run_a.tick1, run_b.tick1);
        assert_eq!(run_a.tick2, run_b.tick2);
        assert_eq!(run_a.tick3, run_b.tick3);
        assert_eq!(run_a.movement_after, run_b.movement_after);
        assert_eq!(run_a.boundary_after, run_b.boundary_after);
        assert_eq!(run_a.summary.combined_hex(), run_b.summary.combined_hex());
        assert_eq!(
            run_a.summary.combined_hex(),
            FRONTIER_V2_4_COMBINED_FEEDBACK_FINGERPRINT
        );

        println!(
            "frontier_v2_4_replay: fp={} fixture_id={FRONTIER_V2_4_FIXTURE_ID}",
            run_a.summary.combined_hex()
        );
    });
}

#[test]
fn frontier_v2_4_defaults_remain_disabled() {
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    assert_eq!(
        ResourceFlowOptInMode::default(),
        ResourceFlowOptInMode::Disabled
    );
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );

    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    assert!(!sim_lib.contains("FrontierV1"));
    assert!(!sim_lib.contains("FrontierV2"));

    println!("frontier_v2_4_defaults: disabled=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_resource_route_stays_allocator_only() {
    let (skeleton, config) = smoke_fixture();

    let mut bypass = skeleton;
    bypass.field_policy.resource_dispatch_via_allocator = false;
    assert_eq!(
        classify_proposal_route(ProposalKind::ResourceDispatch, &bypass),
        ProposalRoute::Rejected
    );

    let mut parallel = skeleton;
    parallel.resource_flow.parallel_fixture_economy = true;
    assert!(!validate_frontier_v2_admission(&parallel).accepted);

    let mut shared_pool = skeleton;
    shared_pool.resource_flow.shared_pool_tick_writes = true;
    assert!(!validate_frontier_v2_admission(&shared_pool).accepted);

    let mut planner = skeleton;
    planner.field_policy.cpu_planner = true;
    assert!(!validate_frontier_v2_admission(&planner).accepted);

    let mut urgency = skeleton;
    urgency.field_policy.cpu_urgency = true;
    assert!(!validate_frontier_v2_admission(&urgency).accepted);

    let mut commitment = skeleton;
    commitment.field_policy.cpu_commitment_emission = true;
    assert!(!validate_frontier_v2_admission(&commitment).accepted);

    let _ = config;
    println!("frontier_v2_4_allocator: rejects=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_deferred_features_reject() {
    let deferred: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 10] = [
        ("atlas", Box::new(|s| s.theater.request_atlas = true)),
        (
            "active_mask",
            Box::new(|s| s.theater.request_active_mask = true),
        ),
        (
            "perception",
            Box::new(|s| s.theater.request_perception = true),
        ),
        (
            "source_identity",
            Box::new(|s| s.theater.request_source_identity = true),
        ),
        (
            "nested_e11b",
            Box::new(|s| s.resource_flow.nested_e11b = true),
        ),
        (
            "e11b_5",
            Box::new(|s| s.resource_flow.e11b_5_dynamic_enrollment = true),
        ),
        (
            "d2a",
            Box::new(|s| s.resource_flow.d2a_hard_currency_ordering = true),
        ),
        (
            "act5_ladder",
            Box::new(|s| s.field_policy.pipeline_version = FieldPolicyPipelineVersion::Other),
        ),
        (
            "parallel_fixture",
            Box::new(|s| s.resource_flow.parallel_fixture_economy = true),
        ),
        (
            "cpu_planner",
            Box::new(|s| s.field_policy.cpu_planner = true),
        ),
    ];
    for (label, mutate) in deferred {
        let mut skeleton = frontier_v2_smoke_skeleton();
        mutate(&mut skeleton);
        assert!(
            !validate_frontier_v2_admission(&skeleton).accepted,
            "{label} should reject"
        );
    }
    println!("frontier_v2_4_deferred: rejects=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_no_simthing_sim_semantic_awareness() {
    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    for needle in [
        "FrontierV1",
        "FrontierV2",
        "FIELD_POLICY",
        "RegionCell",
        "ArenaRegistry",
        "proposal",
        "ResourceFlow",
    ] {
        assert!(
            !sim_lib.contains(needle),
            "simthing-sim must not contain `{needle}`"
        );
    }
    println!("frontier_v2_4_sim: semantic_free=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_no_unauthorized_gpu_primitive() {
    let frontier_descriptor = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id.contains("frontier_v2_4") || d.id.contains("combined_feedback"));
    assert!(frontier_descriptor.is_none());

    let wgsl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wgsl");
    if wgsl_dir.is_dir() {
        for entry in std::fs::read_dir(&wgsl_dir).expect("read wgsl dir") {
            let path = entry.expect("entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            assert!(
                !name.contains("frontier_v2_4"),
                "no FrontierV2-4 WGSL: {}",
                path.display()
            );
        }
    }
    println!("frontier_v2_4_gpu: no_new_primitive=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}

#[test]
fn frontier_v2_4_no_implementer_self_acceptance() {
    let report_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/tests/phase_m_frontier_v2_4_combined_feedback_loop_results.md");
    let report = std::fs::read_to_string(&report_path).expect("V2-4 results report must exist");
    let forbidden = [
        "Phase M closed",
        "Phase E closed",
        "M/E closed",
        "FrontierV2 accepted",
        "ClauseThing unblocked",
    ];
    for phrase in forbidden {
        assert!(
            !report.contains(phrase),
            "report must not declare `{phrase}`"
        );
    }
    assert!(report.contains("NotImplemented") || report.contains("not implemented"));
    assert!(report.contains("OwnColumnShadowWrite"));
    assert!(report.contains("BoundaryRequestShadowWrite"));
    println!("frontier_v2_4_no_self_accept: report_ok=true fixture_id={FRONTIER_V2_4_FIXTURE_ID}");
}
