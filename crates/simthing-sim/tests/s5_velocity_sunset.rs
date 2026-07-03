use std::path::Path;

use simthing_core::{ClampBehavior, DimensionRegistry, SimProperty, SubFieldRole};
use simthing_gpu::{
    build_governed_pairs, plan_velocity_integration, GpuContext, Pipelines, WorldGpuState,
};
use simthing_sim::{BoundaryProtocol, PipelineFlags, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn governed_property() -> SimProperty {
    let mut p = SimProperty::simple("core", "governed", 0);
    for sf in &mut p.layout.sub_fields {
        if matches!(sf.role, SubFieldRole::Amount) {
            sf.clamp = ClampBehavior::Bounded { min: 0.0, max: 1.0 };
            sf.velocity_max = Some(1.0);
        }
    }
    p
}

#[test]
fn s5_no_legacy_velocity_shader_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/velocity_integration.wgsl");
    assert!(
        !path.exists(),
        "legacy velocity shader still exists: {path:?}"
    );
}

#[test]
fn s5_accumulator_velocity_is_default_path() {
    assert!(PipelineFlags::default().use_accumulator_velocity);
}
#[test]
fn s5_velocity_accumulator_matches_cpu_golden() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(governed_property());
    let mut state = WorldGpuState::new(ctx, &reg, 1);
    let initial = vec![0.25_f32, 0.5, 0.0];
    state.install_resolved_values_at_boundary(&initial);
    state.install_resolved_previous_values_at_boundary(&initial);

    let pairs = build_governed_pairs(&reg);
    let plan = plan_velocity_integration(&pairs, 1);
    let mut expected = initial.clone();
    expected[0] = 0.5;
    expected[1] = 0.5;

    state.ensure_velocity_accumulator();
    state
        .upload_velocity_ops_with_bands(&plan.ops, plan.n_bands)
        .unwrap();
    let pipelines = Pipelines::new(&state.ctx);
    let mut runtime = state.accumulator_runtime.take().unwrap();
    let mut velocity = runtime.take_velocity_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        0.5,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: velocity.as_mut(),
            intensity_eml: None,
            transfer: None,
            emission: None,
            encode_world_summary: false,
        },
    );
    runtime.restore_velocity_session(velocity);
    state.accumulator_runtime = Some(runtime);
    assert_eq!(state.read_values(), expected);
}
