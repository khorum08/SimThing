//! E-1 — EmitOnThreshold builder parity and regression tests.

use std::path::Path;

use simthing_core::{
    debt_band_next_threshold, emit_on_threshold, emit_on_threshold_registration_to_op,
    rebuild_emit_on_threshold_event_kinds, rebuild_emit_on_threshold_ops,
    refresh_emit_on_threshold_debt_band, AccumulatorOpBuilder, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, ThresholdDirection,
};
use simthing_gpu::{
    emit_on_threshold_registrations_to_gpu, emit_on_threshold_registrations_to_ops,
    execute_threshold_ops_cpu, threshold_registrations_to_ops, AccumulatorOpSession, EncodeError,
    GpuContext, PackedThresholdUpload, ThresholdRegistration, WorldGpuState, DIR_DOWNWARD,
    DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT, THRESH_BUF_VALUES,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn manual_c1_threshold_emit_op(
    slot: u32,
    col: u32,
    threshold: f32,
    direction: ThresholdDirection,
) -> simthing_core::AccumulatorOp {
    AccumulatorOpBuilder::emit_on_threshold(slot, col, threshold, direction)
}

fn run_cpu_threshold_crossing(
    previous: &[f32],
    mut values: Vec<f32>,
    regs: &[EmitOnThresholdRegistration],
) -> Vec<(u32, u32, f32, u32)> {
    let ops = rebuild_emit_on_threshold_ops(regs);
    let kinds = rebuild_emit_on_threshold_event_kinds(regs);
    let emissions =
        execute_threshold_ops_cpu(previous, &mut values, &ops, 1).expect("cpu threshold oracle");
    emissions
        .into_iter()
        .map(|e| (e.slot, e.col, e.value, kinds[e.reg_idx as usize]))
        .collect()
}

#[test]
fn e1_builder_matches_existing_threshold_emit_op_shape() {
    let reg = EmitOnThresholdRegistration {
        slot: 3,
        col: 1,
        threshold: 0.75,
        direction: ThresholdDirection::Either,
        event_kind: 99,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let builder_op = emit_on_threshold_registration_to_op(&reg);
    let gpu_regs = emit_on_threshold_registrations_to_gpu(std::slice::from_ref(&reg));
    let (canonical_ops, kinds) = threshold_registrations_to_ops(&gpu_regs).unwrap();
    assert_eq!(canonical_ops.len(), 1);
    assert_eq!(kinds, vec![99]);
    assert_eq!(builder_op, canonical_ops[0]);

    let (bridge_ops, bridge_kinds) =
        emit_on_threshold_registrations_to_ops(std::slice::from_ref(&reg)).unwrap();
    assert_eq!(bridge_ops, vec![builder_op.clone()]);
    assert_eq!(bridge_kinds, vec![99]);

    let manual = manual_c1_threshold_emit_op(3, 1, 0.75, ThresholdDirection::Either);
    assert_eq!(builder_op, manual);
}

#[test]
fn e1_emit_on_threshold_upward_crossing_emits_once() {
    let reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: 7,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let events = run_cpu_threshold_crossing(&[0.25], vec![0.75], std::slice::from_ref(&reg));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], (0, 0, 0.75, 7));
}

#[test]
fn e1_emit_on_threshold_downward_crossing_emits_once() {
    let reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Downward,
        event_kind: 8,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let events = run_cpu_threshold_crossing(&[0.75], vec![0.25], std::slice::from_ref(&reg));
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], (0, 0, 0.25, 8));
}

#[test]
fn e1_emit_on_threshold_either_direction_supports_up_and_down() {
    let reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Either,
        event_kind: 9,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let up = run_cpu_threshold_crossing(&[0.25], vec![0.75], std::slice::from_ref(&reg));
    assert_eq!(up.len(), 1);
    let down = run_cpu_threshold_crossing(&[0.75], vec![0.25], std::slice::from_ref(&reg));
    assert_eq!(down.len(), 1);
}

#[test]
fn e1_emit_on_threshold_no_crossing_emits_nothing() {
    let reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: 1,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let events = run_cpu_threshold_crossing(&[0.25], vec![0.4], std::slice::from_ref(&reg));
    assert!(events.is_empty());
}

#[test]
fn e1_re_registration_preserves_next_threshold_band() {
    let unit_cost = 20.0;
    let mut reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: debt_band_next_threshold(10, unit_cost),
        direction: ThresholdDirection::Downward,
        event_kind: 42,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let first = run_cpu_threshold_crossing(&[-170.0], vec![-190.0], std::slice::from_ref(&reg));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].3, 42);

    reg = refresh_emit_on_threshold_debt_band(&reg, 8, unit_cost);
    assert_eq!(reg.threshold, debt_band_next_threshold(8, unit_cost));

    let no_dup = run_cpu_threshold_crossing(&[-190.0], vec![-195.0], std::slice::from_ref(&reg));
    assert!(no_dup.is_empty());

    let second = run_cpu_threshold_crossing(&[-130.0], vec![-150.0], std::slice::from_ref(&reg));
    assert_eq!(second.len(), 1);
}

#[test]
fn e1_no_legacy_threshold_shader_and_routes_through_accumulator_op() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/threshold_scan.wgsl");
    assert!(
        !path.exists(),
        "legacy threshold shader still exists: {path:?}"
    );

    let op = emit_on_threshold(0, 0, 0.5, ThresholdDirection::Upward);
    op.validate().expect("E-1 op validates as AccumulatorOp");

    let Some(ctx) = try_gpu() else {
        eprintln!("skipping GPU portion: no GPU");
        return;
    };

    use simthing_core::{DimensionRegistry, SimProperty};

    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "amount", 0));
    let state = WorldGpuState::new(ctx, &reg, 1);
    let row_len = state.values_len();
    let mut prev = vec![0.0_f32; row_len];
    let mut curr = vec![0.0_f32; row_len];
    prev[0] = 0.25;
    curr[0] = 0.75;
    state.write_previous_values(&prev);
    state.write_values(&curr);

    let gpu_regs = vec![ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: THRESH_BUF_VALUES,
    }];
    let builder_regs = vec![EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: 1,
        buffer: EmitOnThresholdBuffer::Values,
    }];
    assert_eq!(
        emit_on_threshold_registrations_to_gpu(&builder_regs),
        gpu_regs
    );

    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, 1, 1);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&gpu_regs).unwrap(),
        )
        .expect("upload via AccumulatorOp session");
    session
        .dispatch_threshold_scan(&state.ctx, &state.values, &state.previous_values)
        .expect("dispatch threshold scan");
    let events = session
        .readback_threshold_events(&state.ctx)
        .expect("readback");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_kind, 1);
}

#[test]
fn e1_output_buffer_registration_preserved_in_gpu_bridge() {
    let reg = EmitOnThresholdRegistration {
        slot: 2,
        col: 1,
        threshold: 0.25,
        direction: ThresholdDirection::Upward,
        event_kind: 55,
        buffer: EmitOnThresholdBuffer::Output,
    };
    let gpu = emit_on_threshold_registrations_to_gpu(std::slice::from_ref(&reg));
    assert_eq!(gpu.len(), 1);
    assert_eq!(gpu[0].buffer, THRESH_BUF_OUTPUT);
    assert_eq!(gpu[0].slot, 2);
    assert_eq!(gpu[0].col, 1);
    assert_eq!(gpu[0].event_kind, 55);
}

#[test]
fn e1_output_buffer_rejected_by_plain_ops_helper() {
    let reg = EmitOnThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: ThresholdDirection::Upward,
        event_kind: 1,
        buffer: EmitOnThresholdBuffer::Output,
    };
    assert!(matches!(
        emit_on_threshold_registrations_to_ops(std::slice::from_ref(&reg)),
        Err(EncodeError::Unsupported(_))
    ));
}

#[test]
fn e1_builder_direction_constants_match_gpu_dirs() {
    let cases = [
        (ThresholdDirection::Upward, DIR_UPWARD),
        (ThresholdDirection::Downward, DIR_DOWNWARD),
        (ThresholdDirection::Either, DIR_EITHER),
    ];
    for (dir, gpu_dir) in cases {
        let reg = EmitOnThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: dir,
            event_kind: 0,
            buffer: EmitOnThresholdBuffer::Values,
        };
        let gpu = emit_on_threshold_registrations_to_gpu(std::slice::from_ref(&reg));
        assert_eq!(gpu[0].direction, gpu_dir);
    }
}
