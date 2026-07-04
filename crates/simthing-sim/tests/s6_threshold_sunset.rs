use std::path::Path;

use simthing_core::{
    DimensionRegistry, Direction, FissionTemplate, FissionThreshold, PropertyValue, SimProperty,
    SimThing, SimThingKind, SimThingKindTag, SubFieldRole,
};
use simthing_feeder::DispatchCoordinator;
use simthing_gpu::{
    AccumulatorOpSession, GpuContext, PackedThresholdUpload, SlotAllocator, ThresholdRegistration,
    WorldGpuState, DIR_UPWARD, THRESH_BUF_VALUES,
};
use simthing_sim::{BoundaryProtocol, PipelineFlags, SimRuntimeTree};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
fn s6_threshold_events_match_cpu_golden() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "amount", 0));
    let state = WorldGpuState::new(ctx, &reg, 1);
    state.install_resolved_previous_values_at_boundary(&[0.25, 0.0, 0.0]);
    state.install_resolved_values_at_boundary(&[0.75, 0.0, 0.0]);
    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 0.5,
        direction: DIR_UPWARD,
        event_kind: 42,
        buffer: THRESH_BUF_VALUES,
    }];
    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 1);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut session)
        .unwrap();
    let events = session.readback_threshold_events(&state.ctx).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].slot(), 0);
    assert_eq!(events[0].col(), 0);
    assert_eq!(events[0].event_kind(), 42);
    assert_eq!(events[0].value().to_bits(), 0.75_f32.to_bits());
}
