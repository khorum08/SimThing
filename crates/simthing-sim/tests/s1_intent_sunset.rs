use std::path::Path;

use simthing_core::{DimensionRegistry, SimProperty};
use simthing_gpu::{
    accumulator_op::execute_intent_deltas_cpu, AccumulatorOpSession, GpuContext, IntentDelta,
    PackedIntentUpload, Pipelines, WorldGpuState,
};
use simthing_sim::PipelineFlags;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
fn s1_no_legacy_intent_shader_file() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src/shaders/intent_delta.wgsl");
    assert!(
        !path.exists(),
        "legacy intent shader still exists: {path:?}"
    );
}

#[test]
fn s1_accumulator_intent_is_default_path() {
    assert!(PipelineFlags::default().use_accumulator_intent);
}

#[test]
fn s1_intent_disabled_rejects_pending_intents() {
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "value", 0));
    let ctx = try_gpu();
    if ctx.is_none() {
        eprintln!("skipping: no GPU");
        return;
    }
    let ctx = ctx.unwrap();
    let mut state = WorldGpuState::new(ctx, &reg, 1);
    let deltas = [IntentDelta {
        slot: 0,
        col: 0,
        mul: 1.0,
        add: 0.25,
    }];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let flags = PipelineFlags {
            use_accumulator_intent: false,
            ..PipelineFlags::default()
        };
        if !flags.use_accumulator_intent && !deltas.is_empty() {
            panic!(
                "Legacy intent_delta.wgsl was deleted in S-1; AccumulatorOp intent must remain enabled when player intents exist."
            );
        }
        state.upload_intent_deltas(&deltas);
    }));
    assert!(result.is_err());
}

#[test]
fn s1_intent_accumulator_matches_cpu_golden() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "value", 0));
    let mut state = WorldGpuState::new(ctx, &reg, 1);
    let initial = vec![0.5_f32, 0.0, 0.0];
    state.install_resolved_values_at_boundary(&initial);
    let deltas = [IntentDelta {
        slot: 0,
        col: 0,
        mul: 2.0,
        add: -0.25,
    }];
    let mut expected = initial.clone();
    execute_intent_deltas_cpu(&mut expected, &deltas, state.n_dims);

    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 1);
    session
        .upload_packed_intent_ops(
            &state.ctx,
            &PackedIntentUpload::from_deltas(&deltas).unwrap(),
        )
        .unwrap();
    let pipelines = Pipelines::new(&state.ctx);
    session.prepare_intent(&state.ctx);
    pipelines.run_tick_pipeline_with_accumulators(
        &mut state,
        0.0,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: Some(&mut session),
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: None,
            emission: None,
            encode_world_summary: false,
        },
    );
    session.finish_intent(&state.ctx);
    assert_eq!(state.read_values(), expected);
}
