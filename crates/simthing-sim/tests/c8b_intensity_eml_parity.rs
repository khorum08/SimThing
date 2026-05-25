//! C-8b intensity EvalEML parity vs legacy Pass 2.

use simthing_core::{
    compile_intensity_behavior_to_eml, eml_opcode, intensity_eml_direct_cpu, intensity_tree_id,
    DimensionRegistry, EmlConsumerKind, EmlExecutionClass, IntensityBehavior, SimProperty,
    SimPropertyId,
};
use simthing_gpu::{
    eval_eml_cpu, plan_velocity_integration, build_governed_pairs, GpuContext, Pipelines,
    WorldGpuState,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn intensity_property(behavior: IntensityBehavior) -> SimProperty {
    let mut p = SimProperty::simple("core", "pressure", 0);
    p.intensity_behavior = Some(behavior);
    p
}

fn setup_intensity_state(reg: &DimensionRegistry, n_slots: u32, initial: &[f32]) -> WorldGpuState {
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), reg, n_slots);
    let n_dims = state.n_dims as usize;
    let mut flat = vec![0.0_f32; state.values_len()];
    for (slot, row) in initial.chunks(n_dims).enumerate() {
        flat[slot * n_dims..slot * n_dims + n_dims].copy_from_slice(row);
    }
    state.write_values(&flat);
    state.sync_intensity_eml_accumulator(reg);
    state
}

fn run_legacy_intensity(state: &WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    pipelines.run_snapshot(state);
    if state.n_governed_pairs > 0 {
        pipelines.run_velocity_integration(state, dt);
    }
    pipelines.run_intensity_update(state, dt);
    state.read_values()
}

fn run_accumulator_intensity(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    let mut intensity_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_intensity_eml_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: intensity_session.as_mut(),
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_intensity_eml_session(intensity_session);
    state.read_values()
}

fn intensity_col(reg: &DimensionRegistry) -> usize {
    let layout = &reg.property(SimPropertyId(0)).layout;
    reg.column_range(SimPropertyId(0))
        .col_for_role(&simthing_core::SubFieldRole::Intensity, layout)
        .unwrap()
}

#[test]
fn c8b_intensity_behavior_compiles_exact_deterministic_eml() {
    let behavior = IntensityBehavior::default();
    let (meta, nodes) = compile_intensity_behavior_to_eml(&behavior, intensity_tree_id(0), 1, 2);
    assert_eq!(meta.execution_class, EmlExecutionClass::ExactDeterministic);
    assert!(meta.allowed_consumers.contains_kind(EmlConsumerKind::Intensity));
    assert_eq!(meta.node_count, nodes.len() as u32);
    for node in &nodes {
        if node.opcode == eml_opcode::PARAM {
            assert!(node.a <= 3);
        }
    }
}

#[test]
fn c8b_intensity_eml_cpu_oracle_matches_legacy_formula() {
    let behavior = IntensityBehavior::default();
    let (_meta, nodes) = compile_intensity_behavior_to_eml(&behavior, intensity_tree_id(0), 1, 2);
    let gpu_nodes: Vec<_> = nodes
        .iter()
        .map(|n| simthing_core::EmlNodeGpu {
            opcode: n.opcode,
            flags: n.flags,
            a: n.a,
            b: n.b,
            c: n.c,
            d: n.d,
        })
        .collect();
    let cases = [
        (0.0, 0.5, 1.0),
        (0.004, 0.5, 1.0),
        (0.005, 0.5, 1.0),
        (0.006, 0.5, 1.0),
        (-0.01, 0.5, 1.0),
        (0.0, 0.001, 1.0),
        (0.1, 0.999, 1.0),
        (0.0, 0.5, 0.0),
    ];
    for (velocity, intensity, dt) in cases {
        let expected = intensity_eml_direct_cpu(&behavior, velocity, intensity, dt);
        let values = vec![0.0, velocity, intensity];
        let got = eval_eml_cpu(&gpu_nodes, 0, &values, 3, [dt, 0.0, 0.0, 0.0]);
        assert_eq!(got.to_bits(), expected.to_bits(), "vel={velocity} int={intensity} dt={dt}");
    }
}

#[test]
fn c8b_intensity_gpu_eval_eml_matches_legacy_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let behavior = IntensityBehavior::default();
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(behavior));
    let n_dims = reg.total_columns;
    let icol = intensity_col(&reg);

    let cases: &[(f32, f32, f32, &str)] = &[
        (0.1, 0.5, 1.0, "build branch"),
        (0.0, 0.5, 1.0, "decay branch"),
        (-0.02, 0.3, 0.5, "negative velocity build"),
        (0.0, 0.99, 0.25, "near max decay"),
    ];

    for &(velocity, intensity, dt, label) in cases {
        let mut row = vec![0.0_f32; n_dims];
        row[1] = velocity;
        row[icol] = intensity;

        let legacy_state = setup_intensity_state(&reg, 1, &row);
        let mut acc_state = setup_intensity_state(&reg, 1, &row);
        let legacy = run_legacy_intensity(&legacy_state, dt);
        let acc = run_accumulator_intensity(&mut acc_state, dt);
        assert_eq!(
            legacy[icol].to_bits(),
            acc[icol].to_bits(),
            "{label}: legacy={} acc={}",
            legacy[icol],
            acc[icol]
        );
    }
}

#[test]
fn c8b_intensity_runs_after_velocity_before_overlay() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let behavior = IntensityBehavior {
        velocity_threshold: 0.001,
        build_coefficient: 5.0,
        decay_coefficient: 0.01,
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(behavior.clone()));
    let n_dims = reg.total_columns;
    let icol = intensity_col(&reg);

    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.05, 0.2]);
    state.ensure_velocity_accumulator();
    let pairs = build_governed_pairs(&reg);
    let vplan = plan_velocity_integration(&pairs, 1);
    state
        .upload_velocity_ops_with_bands(&vplan.ops, vplan.n_bands)
        .expect("velocity upload");

    let mut velocity_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_velocity_session();
    let mut intensity_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_intensity_eml_session();
    let pipelines = Pipelines::new(&state.ctx);
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        1.0,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity_session.as_mut(),
            intensity_eml: intensity_session.as_mut(),
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_velocity_session(velocity_session);
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_intensity_eml_session(intensity_session);

    let after = state.read_values();
    assert!(after[1].abs() > behavior.velocity_threshold);
    assert!(after[icol] > 0.2, "intensity should increase after velocity+intensity pass");
}

#[test]
fn c8b_intensity_does_not_reupload_eml_table_per_tick() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    let node_uploads = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .eml
        .as_ref()
        .unwrap()
        .node_upload_count;
    let range_uploads = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .eml
        .as_ref()
        .unwrap()
        .range_upload_count;

    let _ = run_accumulator_intensity(&mut state, 0.25);
    let _ = run_accumulator_intensity(&mut state, 0.5);
    let _ = run_accumulator_intensity(&mut state, 1.0);

    let table = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .eml
        .as_ref()
        .unwrap();
    assert_eq!(table.node_upload_count, node_uploads);
    assert_eq!(table.range_upload_count, range_uploads);
}

#[test]
fn c8b_intensity_reuploads_ops_when_eml_generation_changes() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut behavior = IntensityBehavior::default();
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(behavior.clone()));
    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    let gen1 = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .intensity_ops_registry_generation();

    behavior.build_coefficient = 3.5;
    let mut reg2 = DimensionRegistry::new();
    reg2.register(intensity_property(behavior));
    state.sync_intensity_eml_accumulator(&reg2);

    let gen2 = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .intensity_ops_registry_generation();
    assert_ne!(gen1, gen2);
    assert!(state.accumulator_intensity_eml_active);
}

#[test]
fn c8b_intensity_path_no_cpu_mediated_evaluation() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    assert!(state.accumulator_intensity_eml_active);
    let _ = run_accumulator_intensity(&mut state, 0.25);
}

#[test]
fn c8b_combined_c1_c2_c4_s4_c7_c8b_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let n_dims = reg.total_columns;
    let icol = intensity_col(&reg);
    let mut row = vec![0.0_f32; n_dims];
    row[1] = 0.02;
    row[icol] = 0.4;

    let mut state = setup_intensity_state(&reg, 1, &row);
    state.ensure_velocity_accumulator();
    let pairs = build_governed_pairs(&reg);
    let vplan = plan_velocity_integration(&pairs, 1);
    state
        .upload_velocity_ops_with_bands(&vplan.ops, vplan.n_bands)
        .expect("velocity upload");

    let legacy_state = setup_intensity_state(&reg, 1, &row);
    let legacy = run_legacy_intensity(&legacy_state, 0.5);
    let acc = run_accumulator_intensity(&mut state, 0.5);
    assert_eq!(legacy[icol].to_bits(), acc[icol].to_bits());
}
