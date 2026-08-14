use simthing_core::{
    DimensionRegistry, DissolveCondition, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimThing, SimThingKind,
    SubFieldRole, TransformOp,
};
use simthing_gpu::{
    AccumulatorOpSession, GpuContext, PackedThresholdUpload, SlotAllocator, WorldGpuState,
};
use simthing_sim::overlay_lifecycle::{
    append_overlay_lifecycle_registrations, apply_gpu_overlay_lifecycle, resolve_overlay_lifecycle,
    OverlayLifecycleAdmissionState,
};
use simthing_sim::ThresholdRegistry;
use simthing_sim::{ReplayDriver, ReplayReader};
use std::collections::HashMap;
use std::io::{BufReader, Cursor};

#[test]
fn gpu_production_decision_is_bit_identical_to_retained_cpu_oracle() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU adapter");
        return;
    };
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("proof", "amount", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_property(property, registry.property(property).default_value());
    let overlay_id = OverlayId::new();
    root.overlays.push(Overlay {
        id: overlay_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: root.id,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![
                DissolveCondition::PropertyReaches {
                    property,
                    sub_field: SubFieldRole::Amount,
                    value: 1.0,
                },
                DissolveCondition::AfterTicks { remaining: 5 },
            ],
        },
    });
    let mut oracle_root = root.clone();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let n_dims = registry.total_columns as u32;
    let state = WorldGpuState::new(ctx, &registry, 1);
    let mut previous = vec![0.0; n_dims as usize];
    let mut current = previous.clone();
    current[0] = 2.0;
    state.install_resolved_previous_values_at_boundary(&previous);
    state.install_resolved_values_at_boundary(&current);

    let mut gpu_regs = Vec::new();
    let mut cpu_registry = ThresholdRegistry::new();
    let mut admission = OverlayLifecycleAdmissionState::default();
    let (plan, targets) = append_overlay_lifecycle_registrations(
        &root,
        &registry,
        &allocator,
        GenerationStamp::new(0),
        &mut gpu_regs,
        &mut cpu_registry,
        &mut admission,
    );
    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, n_dims, 4);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&gpu_regs).unwrap(),
        )
        .unwrap();
    session.bind_generation_authority(5);
    session
        .configure_overlay_lifecycle_projection(&state.ctx, &plan)
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut session)
        .unwrap();
    let rows = session
        .readback_overlay_lifecycle_states(&state.ctx)
        .unwrap();

    let paths = HashMap::from([(root.id, Vec::new())]);
    let gpu_out = apply_gpu_overlay_lifecycle(
        &mut root,
        &registry,
        &allocator,
        &mut current,
        n_dims as usize,
        &paths,
        &targets,
        &rows,
    );

    let mut oracle_shadow = current.clone();
    let mut oracle_out = Default::default();
    for day in 0..=5 {
        oracle_out = resolve_overlay_lifecycle(
            &mut oracle_root,
            &registry,
            &allocator,
            &mut oracle_shadow,
            n_dims as usize,
            day,
            None,
        );
    }
    assert_eq!(gpu_out.dissolved_overlays, vec![(root.id, overlay_id)]);
    assert_eq!(gpu_out.dissolved_overlays, oracle_out.dissolved_overlays);
    previous[0] = current[0];
    assert_eq!(previous[0].to_bits(), oracle_shadow[0].to_bits());

    // Consume, unchanged, the LDJSON emitted by ReplayWriter on historical
    // master 9593ca2e before extraction.
    let artifact =
        include_bytes!("../../../docs/tests/gpu_overlay_lifecycle_preextract_9593ca2e.ldjson");
    let mut reader = ReplayReader::new(BufReader::new(Cursor::new(artifact)));
    let snapshot = reader.read_snapshot().unwrap();
    let recorded_target = snapshot.root.id();
    let recorded_overlay = snapshot
        .root
        .snapshot_node(recorded_target)
        .unwrap()
        .overlay_ids[0];
    let mut replay = ReplayDriver::from_snapshot(snapshot);
    replay.apply_frame(reader.next_frame().unwrap().unwrap());
    assert!(!replay.root.has_overlay(recorded_target, recorded_overlay));
    assert!(reader.next_frame().unwrap().is_none());
}
