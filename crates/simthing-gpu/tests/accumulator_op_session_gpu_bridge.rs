//! AccumulatorOpSession generic GPU buffer bridge helpers (M-first-slice-R2).

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{
    accumulator_op::set_debug_readback_allowed, AccumulatorOpSession, AccumulatorOpSessionError,
    GpuContext,
};
use std::sync::Mutex;
use wgpu::util::DeviceExt;
use wgpu::BufferUsages;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

#[test]
fn test_r2_bridge_copy_and_slot_writes() {
    with_gpu(|ctx| {
        set_debug_readback_allowed(true);
        let n_slots = 4u32;
        let n_dims = 2u32;
        let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);

        let src_data = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let src = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bridge_src"),
            contents: bytemuck::cast_slice(&src_data),
            usage: BufferUsages::COPY_SRC,
        });

        session.zero_values_buffer(ctx);
        session
            .copy_values_prefix_from_buffer(ctx, &src, 0, 0, 32)
            .unwrap();
        session
            .write_slot_col_values(ctx, &[(3, 1, 9.0)])
            .unwrap();

        let vals = session.readback_full(ctx).unwrap();
        assert_eq!(vals, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0]);

        let sum_op = AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: 0,
                count: 3,
                col: 0,
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(3, 0)],
        };
        session.upload_ops(ctx, &[sum_op]).unwrap();
        session.tick(ctx, 0).unwrap();
        let out = session.readback_full(ctx).unwrap();
        assert!((out[idx(3, 0, n_dims)] - 9.0).abs() < 1e-5);
    });
}

#[test]
fn test_r2_bridge_bounds_validation() {
    with_gpu(|ctx| {
        let session = AccumulatorOpSession::new(ctx, 2, 2);

        let err = session
            .copy_values_prefix_from_buffer(ctx, session.values_buffer(), 0, 0, 32)
            .unwrap_err();
        assert!(matches!(err, AccumulatorOpSessionError::CopyOutOfBounds { .. }));

        let err = session
            .write_slot_col_values(ctx, &[(2, 0, 1.0)])
            .unwrap_err();
        assert!(matches!(err, AccumulatorOpSessionError::InvalidSlot { .. }));

        let err = session
            .write_slot_col_values(ctx, &[(0, 2, 1.0)])
            .unwrap_err();
        assert!(matches!(err, AccumulatorOpSessionError::InvalidColumn { .. }));
    });
}
