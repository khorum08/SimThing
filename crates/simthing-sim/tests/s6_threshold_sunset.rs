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
fn s6_no_legacy_threshold_shader_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-gpu/src/shaders/threshold_scan.wgsl");
    assert!(
        !path.exists(),
        "legacy threshold shader still exists: {path:?}"
    );
}

#[test]
fn s6_accumulator_threshold_is_default_path() {
    assert!(PipelineFlags::default().use_accumulator_threshold_scan);
}

#[test]
fn s6_threshold_disabled_rejects_threshold_workload() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let mut reg = DimensionRegistry::new();
    let mut prop = SimProperty::simple("stress", "pressure", 0);
    prop.fission_templates = vec![FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.5,
        direction: Direction::Falling,
        template: FissionTemplate {
            child_kind: SimThingKindTag::Cohort,
            fusion_intensity_threshold: 0.9,
            fusion_scar_coefficient: 0.0,
            resolution_label: "resolved".into(),
            clone_capability_children: false,
            capability_container_kinds: Vec::new(),
        },
        secondary: None,
    }];
    let pid = reg.register(prop);
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    child.add_property(pid, PropertyValue::from_layout(&reg.property(pid).layout));
    world.add_child(child);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&world);
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let coord = DispatchCoordinator::new(alloc.capacity() as u32, reg.total_columns as u32, 1);
    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(world), reg, alloc);
    proto.flags.use_accumulator_threshold_scan = false;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        proto.initial_gpu_sync(&coord, &mut state);
    }));
    assert!(result.is_err());
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
    state.write_previous_values(&[0.25, 0.0, 0.0]);
    state.write_values(&[0.75, 0.0, 0.0]);
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
    session
        .dispatch_threshold_scan(&state.ctx, &state.values, &state.previous_values)
        .unwrap();
    let events = session.readback_threshold_events(&state.ctx).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].slot, 0);
    assert_eq!(events[0].col, 0);
    assert_eq!(events[0].event_kind, 42);
    assert_eq!(events[0].value.to_bits(), 0.75_f32.to_bits());
}
