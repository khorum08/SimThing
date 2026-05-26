//! C-8b intensity EvalEML parity vs CPU/EML golden oracle.

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

fn run_accumulator_intensity(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
    pipelines.run_accumulator_intensity_eml(state, dt);
    state.read_values()
}

fn run_accumulator_intensity_with_velocity(state: &mut WorldGpuState, dt: f32) -> Vec<f32> {
    let pipelines = Pipelines::new(&state.ctx);
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
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        dt,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity_session.as_mut(),
            intensity_eml: intensity_session.as_mut(),
            transfer: None,
            emission: None,
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
    state.read_values()
}

fn intensity_col(reg: &DimensionRegistry) -> usize {
    let layout = &reg.property(SimPropertyId(0)).layout;
    reg.column_range(SimPropertyId(0))
        .col_for_role(&simthing_core::SubFieldRole::Intensity, layout)
        .unwrap()
}

fn cpu_golden_intensity(
    behavior: &IntensityBehavior,
    velocity: f32,
    intensity: f32,
    dt: f32,
) -> f32 {
    intensity_eml_direct_cpu(behavior, velocity, intensity, dt)
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
fn c8b_intensity_gpu_eval_eml_matches_cpu_golden_bit_exact() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let behavior = IntensityBehavior::default();
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(behavior.clone()));
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

        let mut acc_state = setup_intensity_state(&reg, 1, &row);
        let acc = run_accumulator_intensity(&mut acc_state, dt);
        let expected = cpu_golden_intensity(&behavior, velocity, intensity, dt);
        assert_eq!(
            acc[icol].to_bits(),
            expected.to_bits(),
            "{label}: expected={expected} acc={}",
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
            transfer: None,
            emission: None,
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
    let uploads1 = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .intensity_op_upload_count();

    behavior.build_coefficient = 3.5;
    let mut reg2 = DimensionRegistry::new();
    reg2.register(intensity_property(behavior));
    state.sync_intensity_eml_accumulator(&reg2);

    let runtime = state.accumulator_runtime.as_ref().unwrap();
    let gen2 = runtime.intensity_ops_registry_generation();
    assert_ne!(gen1, gen2);
    assert!(runtime.intensity_op_upload_count() > uploads1);
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
fn c8b_slot_growth_reuploads_intensity_ops_even_when_formula_unchanged() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let n_dims = reg.total_columns;
    let icol = intensity_col(&reg);

    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    let sig1 = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .intensity_op_plan_signature()
        .unwrap()
        .clone();
    assert_eq!(sig1.n_slots, 1);
    assert_eq!(sig1.n_ops, 1);

    state.rebuild_for_slots(2, &reg);
    let mut flat = vec![0.0_f32; state.values_len()];
    flat[0..n_dims].copy_from_slice(&[0.0, 0.1, 0.5]);
    flat[n_dims..2 * n_dims].copy_from_slice(&[0.0, 0.2, 0.3]);
    state.write_values(&flat);
    state.sync_intensity_eml_accumulator(&reg);

    let runtime = state.accumulator_runtime.as_ref().unwrap();
    let sig2 = runtime.intensity_op_plan_signature().unwrap();
    assert_eq!(sig2.n_slots, 2);
    assert_eq!(sig2.n_ops, 2);
    assert_ne!(sig1, *sig2);

    let acc = run_accumulator_intensity(&mut state, 0.5);
    assert_ne!(acc[icol].to_bits(), 0.5f32.to_bits());
    assert_ne!(acc[n_dims + icol].to_bits(), 0.3f32.to_bits());
}

#[test]
fn c8b_same_shape_same_generation_skips_intensity_op_upload() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    assert_eq!(
        state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .intensity_op_upload_count(),
        1
    );

    state.sync_intensity_eml_accumulator(&reg);
    assert_eq!(
        state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .intensity_op_upload_count(),
        1
    );
}

#[test]
fn c8b_intensity_entry_layout_change_reuploads_ops() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let mut state = setup_intensity_state(&reg, 1, &[0.0, 0.1, 0.5]);
    assert_eq!(
        state
            .accumulator_runtime
            .as_ref()
            .unwrap()
            .intensity_op_upload_count(),
        1
    );

    let mut reg2 = DimensionRegistry::new();
    reg2.register(intensity_property(IntensityBehavior::default()));
    let mut p2 = SimProperty::simple("core", "heat", 1);
    p2.intensity_behavior = Some(IntensityBehavior::default());
    reg2.register(p2);
    state.sync_intensity_eml_accumulator(&reg2);

    let runtime = state.accumulator_runtime.as_ref().unwrap();
    assert_eq!(runtime.intensity_op_upload_count(), 2);
    let sig = runtime.intensity_op_plan_signature().unwrap();
    assert_eq!(sig.n_entries, 2);
    assert_eq!(sig.n_ops, 2);
}

#[test]
fn c8b_unchanged_intensity_formula_does_not_reupload_eml_table_each_boundary() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(IntensityBehavior::default()));
    let mut state = WorldGpuState::new(GpuContext::new_blocking().expect("gpu"), &reg, 1);
    state.write_values(&[0.0, 0.1, 0.5]);
    state.sync_intensity_eml_accumulator(&reg);

    let table = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .eml
        .as_ref()
        .unwrap();
    let node_uploads = table.node_upload_count;
    let range_uploads = table.range_upload_count;
    let eml_gen = state
        .accumulator_runtime
        .as_ref()
        .unwrap()
        .eml_registry
        .generation();

    state.sync_intensity_eml_accumulator(&reg);

    let runtime = state.accumulator_runtime.as_ref().unwrap();
    let table = runtime.eml.as_ref().unwrap();
    assert_eq!(table.node_upload_count, node_uploads);
    assert_eq!(table.range_upload_count, range_uploads);
    assert_eq!(runtime.eml_registry.generation(), eml_gen);
}

#[test]
fn c8b_combined_c1_c2_c4_s4_c7_c8b_all_flags_on() {
    let Some(_ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let behavior = IntensityBehavior::default();
    let mut reg = DimensionRegistry::new();
    reg.register(intensity_property(behavior.clone()));
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

    let acc = run_accumulator_intensity_with_velocity(&mut state, 0.5);
    let expected = cpu_golden_intensity(&behavior, row[1], row[icol], 0.5);
    assert_eq!(acc[icol].to_bits(), expected.to_bits());
}
