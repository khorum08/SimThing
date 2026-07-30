//! C-8b intensity EvalEML parity vs CPU/EML golden oracle.

use simthing_core::{
    compile_intensity_behavior_to_eml, eml_opcode, intensity_eml_direct_cpu, intensity_tree_id,
    DimensionRegistry, EmlConsumerKind, EmlExecutionClass, IntensityBehavior, SimProperty,
    SimPropertyId,
};
use simthing_gpu::{
    build_governed_pairs, eval_eml_cpu, plan_velocity_integration, GpuContext, Pipelines,
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
    state.install_resolved_values_at_boundary(&flat);
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
        .raw()
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
    assert!(
        after[icol] > 0.2,
        "intensity should increase after velocity+intensity pass"
    );
}
