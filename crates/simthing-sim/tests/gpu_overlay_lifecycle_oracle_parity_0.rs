use simthing_core::{
    DimensionRegistry, DissolveCondition, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimThing, SimThingKind,
    SubFieldRole, TransformOp,
};
use simthing_feeder::{BoundaryRequest, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{
    AccumulatorOpSession, GpuContext, PackedThresholdUpload, SlotAllocator, WorldGpuState,
};
use simthing_sim::overlay_lifecycle::{
    append_overlay_lifecycle_registrations, apply_gpu_overlay_lifecycle, resolve_overlay_lifecycle,
    OverlayLifecycleAdmissionState,
};
use simthing_sim::tree_mutation::apply_structural_mutations;
use simthing_sim::{
    BoundaryProtocol, ReplayDriver, ReplayReader, SimRuntimeTree, ThresholdRegistry,
};
use std::collections::HashMap;
use std::io::{BufReader, Cursor};

#[test]
fn gpu_production_decision_is_bit_identical_to_retained_cpu_oracle() {
    let ctx = GpuContext::new_blocking()
        .expect("GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 requires a real GPU adapter");
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
    allocator.install_initial_tree(&root);
    let n_dims = registry.total_columns as u32;
    let mut state = WorldGpuState::new(ctx, &registry, 1);
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
    let mut replay = ReplayDriver::from_snapshot(snapshot).expect("replay snapshot install");
    replay.apply_frame(reader.next_frame().unwrap().unwrap());
    assert!(!replay.root.has_overlay(recorded_target, recorded_overlay));
    assert!(reader.next_frame().unwrap().is_none());

    // Production attach-door falsifiers: the request transports source
    // provenance, while the destination boundary owns deadline establishment.
    // Forced skew must yield destination 7 + duration 4 = deadline 11, never
    // the foreign absolute 904. Overflow and OverrideReceived reject before
    // the overlay reaches the authoritative tree.
    let mut routed_root = SimThing::new(SimThingKind::World, 0);
    routed_root.add_property(property, registry.property(property).default_value());
    let routed_target = routed_root.id;
    let mut routed_allocator = SlotAllocator::new();
    routed_allocator.install_initial_tree(&routed_root);
    let mut routed_runtime = SimRuntimeTree::admit(routed_root);
    let mut routed_shadow = vec![0.0; n_dims as usize];
    let mut routed_admission = OverlayLifecycleAdmissionState::default();
    let routed_id = OverlayId::new();
    let routed_overlay = Overlay {
        id: routed_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: routed_target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle: OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 4 }],
        },
    };
    let accepted = apply_structural_mutations(
        vec![BoundaryRequest::AttachOverlay {
            target: routed_target,
            overlay: routed_overlay.clone(),
            source_generation: GenerationStamp::new(900),
        }],
        &mut routed_runtime,
        &mut routed_allocator,
        &mut registry,
        &mut routed_shadow,
        n_dims as usize,
        None,
        GenerationStamp::new(7),
        &mut routed_admission,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(accepted.overlays_attached, vec![(routed_target, routed_id)]);
    assert_eq!(
        routed_admission.routed_provenance(routed_target, routed_id),
        Some(GenerationStamp::new(900))
    );
    assert_eq!(
        routed_admission.activation_generation(routed_target, routed_id),
        Some(GenerationStamp::new(7)),
        "source generation 900 must remain provenance; destination generation 7 owns the deadline"
    );

    let overflow_id = OverlayId::new();
    let mut overflow_overlay = routed_overlay.clone();
    overflow_overlay.id = overflow_id;
    overflow_overlay.lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
    };
    let overflow = apply_structural_mutations(
        vec![BoundaryRequest::AttachOverlay {
            target: routed_target,
            overlay: overflow_overlay,
            source_generation: GenerationStamp::new(3),
        }],
        &mut routed_runtime,
        &mut routed_allocator,
        &mut registry,
        &mut routed_shadow,
        n_dims as usize,
        None,
        GenerationStamp::new(u32::MAX),
        &mut routed_admission,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(overflow.rejected_overlay_lifecycle, 1);
    assert!(!routed_runtime.has_overlay(routed_target, overflow_id));

    let override_id = OverlayId::new();
    let mut override_overlay = routed_overlay;
    override_overlay.id = override_id;
    override_overlay.lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::OverrideReceived],
    };
    let override_rejected = apply_structural_mutations(
        vec![BoundaryRequest::AttachOverlay {
            target: routed_target,
            overlay: override_overlay,
            source_generation: GenerationStamp::new(7),
        }],
        &mut routed_runtime,
        &mut routed_allocator,
        &mut registry,
        &mut routed_shadow,
        n_dims as usize,
        None,
        GenerationStamp::new(7),
        &mut routed_admission,
        &std::collections::BTreeMap::new(),
    );
    assert_eq!(override_rejected.rejected_overlay_lifecycle, 1);
    assert!(!routed_runtime.has_overlay(routed_target, override_id));

    // Full production boundary witness. A suspended seed reserves one admitted
    // template/capacity row at session build without becoming resident. The
    // deliberately small threshold session makes the capacity falsifier bite
    // without a synthetic kernel-only path.
    let mut boundary_root = SimThing::new(SimThingKind::World, 0);
    boundary_root.add_property(property, registry.property(property).default_value());
    let boundary_target = boundary_root.id;
    let admitted_lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::PropertyReaches {
            property,
            sub_field: SubFieldRole::Amount,
            value: 1.0,
        }],
    };
    let below_lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::PropertyBelow {
            property,
            sub_field: SubFieldRole::Amount,
            value: 1.0,
        }],
    };
    let timed_lifecycle = OverlayLifecycle::Transient {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 4 }],
    };
    let seed_id = OverlayId::new();
    boundary_root.overlays.push(Overlay {
        id: seed_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: boundary_target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(admitted_lifecycle.clone()),
        },
    });
    let below_seed_id = OverlayId::new();
    boundary_root.overlays.push(Overlay {
        id: below_seed_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: boundary_target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(below_lifecycle),
        },
    });
    let timed_seed_id = OverlayId::new();
    boundary_root.overlays.push(Overlay {
        id: timed_seed_id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: boundary_target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle: OverlayLifecycle::Suspended {
            when_activated: Box::new(timed_lifecycle),
        },
    });
    let make_boundary_overlay = |id, lifecycle| Overlay {
        id,
        kind: OverlayKind::Transient,
        source: OverlaySource::System,
        origin: boundary_target,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle,
    };
    let mut boundary_allocator = SlotAllocator::new();
    boundary_allocator.install_initial_tree(&boundary_root);
    let mut protocol = BoundaryProtocol::new(
        SimRuntimeTree::admit(boundary_root),
        registry.clone(),
        boundary_allocator,
    );
    let mut coord = DispatchCoordinator::new(1, n_dims, 1);
    let mut patcher = TransformPatcher::new(1);
    state.ensure_threshold_accumulator(3);
    protocol
        .initial_gpu_sync(&coord, &mut state)
        .expect("valid initial projection");
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    assert_eq!(state.n_thresholds, 0);

    // The three suspended seeds consume the frozen catalogue capacity. A
    // fourth row is refused before attachment, so the authoritative tree
    // stays unchanged.
    let capacity_id = OverlayId::new();
    let capacity_request = BoundaryRequest::AttachOverlay {
        target: boundary_target,
        overlay: make_boundary_overlay(capacity_id, admitted_lifecycle.clone()),
        source_generation: GenerationStamp::new(0),
    };
    let capacity_outcome = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 0, |ctx| {
            ctx.requests.push(capacity_request.clone())
        })
        .expect("valid capacity-rejection projection");
    assert_eq!(capacity_outcome.maintainer.rejected_overlay_lifecycle, 1);
    assert!(!protocol.root.has_overlay(boundary_target, capacity_id));

    // Activation changes resident membership and must rebuild the canonical
    // Phase-5 registration/projection plan.
    let activate_seed = BoundaryRequest::ActivateOverlay {
        target: boundary_target,
        overlay_id: seed_id,
    };
    let activation = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 1, |ctx| {
            ctx.requests.push(activate_seed.clone())
        })
        .expect("valid activation projection");
    assert_eq!(
        activation.maintainer.overlays_activated,
        vec![(boundary_target, seed_id)]
    );
    assert_eq!(activation.gpu_sync.threshold_regs_uploaded, 1);
    assert_eq!(protocol.overlay_lifecycle_target_count(), 1);
    assert_eq!(state.n_thresholds, 1);

    state.install_resolved_previous_values_at_boundary(&vec![0.0; n_dims as usize]);
    let mut crossed_values = vec![0.0; n_dims as usize];
    crossed_values[0] = 2.0;
    state.install_resolved_values_at_boundary(&crossed_values);
    state.bind_production_generation(2);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let seed_dissolved = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 2)
        .expect("valid seed-dissolution projection");
    assert_eq!(
        seed_dissolved.lifecycle.dissolved_overlays,
        vec![(boundary_target, seed_id)]
    );
    assert!(!protocol.root.has_overlay(boundary_target, seed_id));
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    assert_eq!(state.n_thresholds, 0);
    assert!(state
        .readback_overlay_lifecycle_states()
        .unwrap()
        .is_empty());

    // With capacity available, a differently shaped runtime template is still
    // rejected by the frozen kernel catalogue before it reaches the tree.
    let novel_id = OverlayId::new();
    let novel_request = BoundaryRequest::AttachOverlay {
        target: boundary_target,
        overlay: make_boundary_overlay(
            novel_id,
            OverlayLifecycle::Transient {
                dissolution_conditions: vec![DissolveCondition::PropertyReaches {
                    property,
                    sub_field: SubFieldRole::Amount,
                    value: 9.0,
                }],
            },
        ),
        source_generation: GenerationStamp::new(3),
    };
    let novel = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 3, |ctx| {
            ctx.requests.push(novel_request.clone())
        })
        .expect("valid novel-template rejection projection");
    assert_eq!(novel.maintainer.rejected_overlay_lifecycle, 1);
    assert!(!protocol.root.has_overlay(boundary_target, novel_id));

    // Attach the already-admitted template at the real boundary. Step 9 must
    // install both its Phase-5 registration and resident semantic row.
    let attached_id = OverlayId::new();
    let admitted_request = BoundaryRequest::AttachOverlay {
        target: boundary_target,
        overlay: make_boundary_overlay(attached_id, admitted_lifecycle.clone()),
        source_generation: GenerationStamp::new(4),
    };
    let attached = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 4, |ctx| {
            ctx.requests.push(admitted_request.clone())
        })
        .expect("valid attachment projection");
    assert_eq!(
        attached.maintainer.overlays_attached,
        vec![(boundary_target, attached_id)]
    );
    assert_eq!(attached.gpu_sync.threshold_regs_uploaded, 1);
    assert_eq!(protocol.overlay_lifecycle_target_count(), 1);
    assert_eq!(state.readback_overlay_lifecycle_states().unwrap().len(), 1);

    state.install_resolved_previous_values_at_boundary(&vec![0.0; n_dims as usize]);
    state.install_resolved_values_at_boundary(&crossed_values);
    state.bind_production_generation(5);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let attached_dissolved = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 5)
        .expect("valid attached-dissolution projection");
    assert_eq!(
        attached_dissolved.lifecycle.dissolved_overlays,
        vec![(boundary_target, attached_id)]
    );
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    assert_eq!(state.n_thresholds, 0);

    // Suspended attachment reserves the admitted template but remains absent
    // from the resident plan; activate installs it and suspend removes it.
    let toggled_id = OverlayId::new();
    let suspended_request = BoundaryRequest::AttachOverlay {
        target: boundary_target,
        overlay: make_boundary_overlay(
            toggled_id,
            OverlayLifecycle::Suspended {
                when_activated: Box::new(admitted_lifecycle),
            },
        ),
        source_generation: GenerationStamp::new(6),
    };
    let suspended_attach = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 6, |ctx| {
            ctx.requests.push(suspended_request.clone())
        })
        .expect("valid suspended-attachment projection");
    assert_eq!(
        suspended_attach.maintainer.overlays_attached,
        vec![(boundary_target, toggled_id)]
    );
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    assert_eq!(state.n_thresholds, 0);

    let activate_toggled = BoundaryRequest::ActivateOverlay {
        target: boundary_target,
        overlay_id: toggled_id,
    };
    let toggled_active = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 7, |ctx| {
            ctx.requests.push(activate_toggled.clone())
        })
        .expect("valid toggled-activation projection");
    assert_eq!(toggled_active.maintainer.overlay_activations, 1);
    assert_eq!(protocol.overlay_lifecycle_target_count(), 1);
    assert_eq!(state.n_thresholds, 1);

    let suspend_toggled = BoundaryRequest::SuspendOverlay {
        target: boundary_target,
        overlay_id: toggled_id,
    };
    let toggled_suspended = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 8, |ctx| {
            ctx.requests.push(suspend_toggled.clone())
        })
        .expect("valid toggled-suspension projection");
    assert_eq!(toggled_suspended.maintainer.overlay_suspensions, 1);
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    assert_eq!(state.n_thresholds, 0);
    assert!(state
        .readback_overlay_lifecycle_states()
        .unwrap()
        .is_empty());

    // PropertyReaches is a level predicate. Reactivation while the resident
    // value is already above the threshold must dissolve through the real GPU
    // Phase-5 comparator even though no edge occurs after activation.
    coord.shadow[0] = 2.0;
    let level_reaches_activation = protocol
        .execute_with_boundary_hook(Vec::new(), &mut patcher, &mut coord, &mut state, 9, |ctx| {
            ctx.requests.push(activate_toggled.clone())
        })
        .expect("valid level-reaches activation projection");
    assert_eq!(level_reaches_activation.maintainer.overlay_activations, 1);
    state.install_resolved_previous_values_at_boundary(&crossed_values);
    state.install_resolved_values_at_boundary(&crossed_values);
    state.bind_production_generation(10);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let level_reaches = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 10)
        .expect("valid level-reaches projection");
    assert_eq!(
        level_reaches.lifecycle.dissolved_overlays,
        vec![(boundary_target, toggled_id)]
    );

    // PropertyBelow has the same level-at-activation contract. Both previous
    // and current values are already below, so only the admitted GPU level
    // mode inside threshold_crossed can authorize dissolution.
    coord.shadow[0] = 0.0;
    let activate_below = BoundaryRequest::ActivateOverlay {
        target: boundary_target,
        overlay_id: below_seed_id,
    };
    let below_activation = protocol
        .execute_with_boundary_hook(
            Vec::new(),
            &mut patcher,
            &mut coord,
            &mut state,
            11,
            |ctx| ctx.requests.push(activate_below.clone()),
        )
        .expect("valid below activation projection");
    assert_eq!(below_activation.maintainer.overlay_activations, 1);
    let below_values = vec![0.0; n_dims as usize];
    state.install_resolved_previous_values_at_boundary(&below_values);
    state.install_resolved_values_at_boundary(&below_values);
    state.bind_production_generation(12);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let level_below = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 12)
        .expect("valid level-below projection");
    assert_eq!(
        level_below.lifecycle.dissolved_overlays,
        vec![(boundary_target, below_seed_id)]
    );

    // AfterTicks counts active owning-tree generations. Activate for two
    // ticks, suspend across generations 16-17, then reactivate at 18. The
    // remaining two active ticks dissolve at 20: neither attachment time nor
    // the original absolute deadline can produce that result.
    let activate_timed = BoundaryRequest::ActivateOverlay {
        target: boundary_target,
        overlay_id: timed_seed_id,
    };
    let timed_activation = protocol
        .execute_with_boundary_hook(
            Vec::new(),
            &mut patcher,
            &mut coord,
            &mut state,
            13,
            |ctx| ctx.requests.push(activate_timed.clone()),
        )
        .expect("valid timed activation projection");
    assert_eq!(timed_activation.maintainer.overlay_activations, 1);
    for generation in [14u32, 15] {
        state.bind_production_generation(generation);
        let mut resident_session = state
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .take_threshold_session()
            .unwrap();
        state
            .dispatch_accumulator_threshold_scan(&mut resident_session)
            .unwrap();
        state
            .accumulator_runtime
            .as_mut()
            .unwrap()
            .restore_threshold_session(Some(resident_session));
        if generation == 14 {
            let still_active = protocol
                .execute(
                    Vec::new(),
                    &mut patcher,
                    &mut coord,
                    &mut state,
                    generation as u64,
                )
                .expect("valid still-active projection");
            assert_eq!(still_active.lifecycle.dissolved, 0);
        } else {
            let suspend_timed = BoundaryRequest::SuspendOverlay {
                target: boundary_target,
                overlay_id: timed_seed_id,
            };
            let suspended = protocol
                .execute_with_boundary_hook(
                    Vec::new(),
                    &mut patcher,
                    &mut coord,
                    &mut state,
                    generation as u64,
                    |ctx| ctx.requests.push(suspend_timed.clone()),
                )
                .expect("valid timed suspension projection");
            assert_eq!(suspended.lifecycle.dissolved, 0);
            assert_eq!(suspended.maintainer.overlay_suspensions, 1);
        }
    }
    assert_eq!(protocol.overlay_lifecycle_target_count(), 0);
    for generation in [16u32, 17] {
        let paused = protocol
            .execute(
                Vec::new(),
                &mut patcher,
                &mut coord,
                &mut state,
                generation as u64,
            )
            .expect("valid paused projection");
        assert_eq!(paused.lifecycle.dissolved, 0);
    }
    let timed_reactivation = protocol
        .execute_with_boundary_hook(
            Vec::new(),
            &mut patcher,
            &mut coord,
            &mut state,
            18,
            |ctx| ctx.requests.push(activate_timed.clone()),
        )
        .expect("valid timed reactivation projection");
    assert_eq!(timed_reactivation.maintainer.overlay_activations, 1);
    state.bind_production_generation(19);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let one_remaining = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 19)
        .expect("valid one-remaining projection");
    assert_eq!(one_remaining.lifecycle.dissolved, 0);
    state.bind_production_generation(20);
    let mut resident_session = state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_threshold_session()
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut resident_session)
        .unwrap();
    state
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_threshold_session(Some(resident_session));
    let timed_dissolved = protocol
        .execute(Vec::new(), &mut patcher, &mut coord, &mut state, 20)
        .expect("valid timed-dissolution projection");
    assert_eq!(
        timed_dissolved.lifecycle.dissolved_overlays,
        vec![(boundary_target, timed_seed_id)]
    );
}
