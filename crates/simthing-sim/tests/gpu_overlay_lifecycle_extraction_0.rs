//! GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 referee.

use simthing_core::{
    admit_dissolve_conditions, admit_dispatch_minted_overlay, deadline_reached, dispatch_until_dissolved,
    establish_deadline, rebase_routed_duration_at_destination, DissolveCondition, GenerationStamp,
    Overlay, OverlayKind, OverlayLifecycle, OverlayLifecycleAdmitError, OverlaySource,
    PropertyTransformDelta, RoutedDuration, SimThing, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::overlay_lifecycle_gpu::{
    apply_structural_dissolves, bind_tree_overlays, decide_dissolves, evaluate_instance,
    refuse_durable_row_capture, refuse_foreign_absolute_deadline, refuse_global_clock,
    refuse_overlay_local_eml_table, OverlayLifecycleGpuError, OverlayLifecycleInstanceGpu,
    OverlayLifecycleSession, OVERLAY_LIFECYCLE_ROW_BYTES,
};
use simthing_sim::overlay_lifecycle::resolve_overlay_lifecycle_oracle;

fn overlay_with(lifecycle: OverlayLifecycle) -> Overlay {
    Overlay {
        id: simthing_core::OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin: simthing_core::SimThingId::new(),
        affects: vec![],
        transform: PropertyTransformDelta {
            property_id: simthing_core::SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.0))],
        },
        lifecycle,
    }
}

#[test]
fn after_ticks_compares_deadline_with_zero_decrement() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    child.add_overlay(overlay_with(OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
    }));
    root.add_child(child);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let mut shadow = vec![0.0; allocator.capacity()];
    let mut deadlines = std::collections::HashMap::new();
    let first = resolve_overlay_lifecycle_oracle(
        &mut root,
        &Default::default(),
        &allocator,
        &mut shadow,
        1,
        GenerationStamp::new(76),
        &mut deadlines,
        None,
    );
    assert_eq!(first.after_ticks_decremented, 0);
    assert_eq!(first.dissolved, 0);
    let second = resolve_overlay_lifecycle_oracle(
        &mut root,
        &Default::default(),
        &allocator,
        &mut shadow,
        1,
        GenerationStamp::new(77),
        &mut deadlines,
        None,
    );
    assert_eq!(second.after_ticks_decremented, 0);
    assert_eq!(second.dissolved, 1);
}

#[test]
fn gpu_twin_matches_oracle_and_divergence_reds() {
    let generation = GenerationStamp::new(10);
    let instance = OverlayLifecycleInstanceGpu {
        host_slot: 0,
        deadline_generation: 10,
        threshold_col: OverlayLifecycleInstanceGpu::COL_NONE,
        direction: 0,
        threshold_value: 0.0,
        active: 1,
        overlay_id_raw: 1,
        _pad: 0,
    };
    let gpu = evaluate_instance(generation, &instance, None);
    let oracle = deadline_reached(generation, GenerationStamp::new(10));
    assert_eq!(gpu, oracle);
    let mut divergent = instance;
    divergent.deadline_generation = 11;
    assert_ne!(evaluate_instance(generation, &divergent, None), oracle);
}

#[test]
fn override_received_rejected_at_admission() {
    assert_eq!(
        admit_dissolve_conditions(&[DissolveCondition::OverrideReceived]),
        Err(OverlayLifecycleAdmitError::OverrideReceivedForbidden)
    );
    let overlay = overlay_with(
        dispatch_until_dissolved(vec![DissolveCondition::OverrideReceived]).unwrap(),
    );
    assert!(admit_dispatch_minted_overlay(&overlay).is_err());
}

#[test]
fn mid_session_template_mint_reds() {
    let mut session = OverlayLifecycleSession::default();
    session.freeze_templates(1);
    assert_eq!(
        session.mint_semantic_template(),
        Err(OverlayLifecycleGpuError::MidSessionTemplateMint)
    );
}

#[test]
fn planted_mutants_red_for_intended_reasons() {
    assert_eq!(
        refuse_overlay_local_eml_table(),
        OverlayLifecycleGpuError::OverlayLocalEmlTable
    );
    assert_eq!(
        refuse_durable_row_capture(7),
        OverlayLifecycleGpuError::DurableRowCapture
    );
    assert_eq!(
        refuse_foreign_absolute_deadline(),
        OverlayLifecycleGpuError::ForeignAbsoluteDeadline
    );
    assert_eq!(refuse_global_clock(), OverlayLifecycleGpuError::GlobalClock);
    let routed = RoutedDuration {
        duration: 4,
        provenance: GenerationStamp::new(1),
    };
    let dest = rebase_routed_duration_at_destination(routed, GenerationStamp::new(50)).unwrap();
    assert_eq!(dest, GenerationStamp::new(54));
    let foreign_absolute = GenerationStamp::new(1 + 4);
    assert_ne!(dest, foreign_absolute);
}

#[test]
fn pre_extraction_recording_replays_post_extraction() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    let ov = overlay_with(OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 1 }],
    });
    let oid = ov.id;
    let hid = child.id;
    child.add_overlay(ov);
    root.add_child(child);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let mut deadlines = Default::default();
    let generation0 = GenerationStamp::new(0);
    let (bindings, instances) = bind_tree_overlays(&root, &allocator, generation0, &mut deadlines);
    let first = decide_dissolves(&root, generation0, &instances, &bindings);
    assert!(first.dissolved.is_empty());
    let recording = vec![(hid, oid)];
    let generation1 = GenerationStamp::new(1);
    let second = decide_dissolves(&root, generation1, &instances, &bindings);
    assert_eq!(second.dissolved, recording);
    let count = apply_structural_dissolves(&mut root, &recording);
    assert_eq!(count, 1);
}

#[test]
fn carry_measured_before_compaction() {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    child.add_overlay(overlay_with(OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 8 }],
    }));
    root.add_child(child);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let mut deadlines = Default::default();
    let (_b, instances) = bind_tree_overlays(
        &root,
        &allocator,
        GenerationStamp::new(0),
        &mut deadlines,
    );
    let carry = instances.len() * OVERLAY_LIFECYCLE_ROW_BYTES;
    assert!(carry > 0);
    assert_eq!(carry, OVERLAY_LIFECYCLE_ROW_BYTES);
}

#[test]
fn production_src_has_no_afterticks_decrement_or_overlay_history() {
    let lifecycle = include_str!("../src/overlay_lifecycle.rs");
    assert!(
        !lifecycle.contains("*remaining -= 1"),
        "AfterTicks decrement must be gone from the production evaluator"
    );
    assert!(
        !lifecycle.contains("struct OverlayHistory"),
        "OverlayHistory is forbidden"
    );
}

#[test]
fn deadline_overflow_fails_closed() {
    assert!(establish_deadline(GenerationStamp::new(u32::MAX), 1).is_err());
}
