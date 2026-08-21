use simthing_core::{
    admit_overlay_lifecycle, establish_overlay_deadline, rebase_routed_overlay_duration,
    DimensionRegistry, DissolveCondition, GenerationStamp, OverlayLifecycle,
    OverlayLifecycleAdmitError, RoutedGenerationDuration, SimProperty,
};
use simthing_kernel::accumulator_op::{
    OverlayLifecycleProjectionBinding, OverlayLifecycleProjectionPlan,
    OverlayLifecycleProjectionSeed, OverlayLifecycleStateGpu, THRESH_BUF_OWNING_GENERATION,
};
use simthing_kernel::{
    AccumulatorOpSession, GpuContext, PackedThresholdUpload, ThresholdRegistration, WorldGpuState,
    DIR_UPWARD, THRESH_BUF_VALUES,
};
use std::time::Instant;

#[test]
fn real_phase5_crossings_project_conjunctive_lifecycle_state() {
    let ctx = GpuContext::new_blocking()
        .expect("GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 requires a real GPU adapter");
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("proof", "amount", 0));
    let state = WorldGpuState::new(ctx, &registry, 1);
    // PropertyReaches is already satisfied in both resident planes. The GPU
    // lifecycle level mode must resolve it without inventing a new edge.
    state.install_resolved_previous_values_at_boundary(&[2.0, 0.0, 0.0]);
    state.install_resolved_values_at_boundary(&[2.0, 0.0, 0.0]);

    let registrations = [
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 10,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 4.0,
            direction: DIR_UPWARD,
            event_kind: 11,
            buffer: THRESH_BUF_OWNING_GENERATION,
        },
    ];
    let plan = OverlayLifecycleProjectionPlan {
        rows: vec![OverlayLifecycleProjectionSeed::pending(0b11)],
        bindings: vec![
            OverlayLifecycleProjectionBinding {
                registration_index: 0,
                row: 0,
                condition_bit: 0,
            },
            OverlayLifecycleProjectionBinding {
                registration_index: 1,
                row: 0,
                condition_bit: 1,
            },
        ],
    };

    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 4);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&registrations).unwrap(),
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
    assert_eq!(rows[0].satisfied_mask(), 0b11);
    assert!(rows[0].is_dissolved());
    assert_eq!(rows[0].generation(), 5);

    // Biting production-path mutant: misbind the deadline to the already-used
    // property bit. The same hardware dispatch must remain pending.
    let mut mutant = AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 4);
    mutant
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&registrations).unwrap(),
        )
        .unwrap();
    mutant.bind_generation_authority(5);
    let mut mutant_plan = plan.clone();
    mutant_plan.bindings[1].condition_bit = 0;
    mutant
        .configure_overlay_lifecycle_projection(&state.ctx, &mutant_plan)
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut mutant)
        .unwrap();
    let mutant_rows = mutant
        .readback_overlay_lifecycle_states(&state.ctx)
        .unwrap();
    assert_eq!(mutant_rows[0].satisfied_mask(), 0b01);
    assert!(!mutant_rows[0].is_dissolved());

    // Biting threshold-bypass mutant: replace the real resident property
    // observation with the owning-generation source and an unreachable
    // threshold. The real Phase-5 dispatch must leave the property bit clear.
    let mut bypass_registrations = registrations;
    bypass_registrations[0].buffer = THRESH_BUF_OWNING_GENERATION;
    bypass_registrations[0].threshold = 99.0;
    let mut bypass = AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 4);
    bypass
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&bypass_registrations).unwrap(),
        )
        .unwrap();
    bypass.bind_generation_authority(5);
    bypass
        .configure_overlay_lifecycle_projection(&state.ctx, &plan)
        .unwrap();
    state
        .dispatch_accumulator_threshold_scan(&mut bypass)
        .unwrap();
    let bypass_rows = bypass
        .readback_overlay_lifecycle_states(&state.ctx)
        .unwrap();
    assert_eq!(bypass_rows[0].satisfied_mask(), 0b10);
    assert!(!bypass_rows[0].is_dissolved());

    // The real session admission door freezes capacity: a mid-session semantic
    // mint cannot silently grow the resident facility.
    let mut grown = plan.clone();
    grown
        .rows
        .push(OverlayLifecycleProjectionSeed::pending(0b1));
    assert!(mutant
        .configure_overlay_lifecycle_projection(&state.ctx, &grown)
        .is_err());

    let mut over_capacity = plan.clone();
    over_capacity
        .rows
        .extend([OverlayLifecycleProjectionSeed::pending(0b1); 4]);
    assert!(mutant
        .configure_overlay_lifecycle_projection(&state.ctx, &over_capacity)
        .is_err());

    assert_eq!(
        admit_overlay_lifecycle(&OverlayLifecycle::Transient {
            dissolution_conditions: vec![DissolveCondition::OverrideReceived],
        }),
        Err(OverlayLifecycleAdmitError::OverrideReceivedForbidden)
    );
    assert_eq!(
        establish_overlay_deadline(GenerationStamp::new(u32::MAX), 1),
        Err(OverlayLifecycleAdmitError::DeadlineOverflow {
            activation: u32::MAX,
            duration: 1,
        })
    );
    let routed = RoutedGenerationDuration::new(4, GenerationStamp::new(900));
    assert_eq!(
        rebase_routed_overlay_duration(routed, GenerationStamp::new(7)).unwrap(),
        GenerationStamp::new(11)
    );
    assert_ne!(
        rebase_routed_overlay_duration(routed, GenerationStamp::new(7)).unwrap(),
        GenerationStamp::new(904),
        "foreign absolute/global generation mutant must be RED under skew"
    );

    for row_count in [1usize, 64, 256] {
        let mut carry_session =
            AccumulatorOpSession::new_attached(&state.ctx, 1, state.n_dims, 256);
        carry_session
            .upload_packed_threshold_ops(
                &state.ctx,
                &PackedThresholdUpload::from_registrations(&registrations[..1]).unwrap(),
            )
            .unwrap();
        let carry_plan = OverlayLifecycleProjectionPlan {
            rows: vec![OverlayLifecycleProjectionSeed::pending(0b1); row_count],
            bindings: vec![OverlayLifecycleProjectionBinding {
                registration_index: 0,
                row: 0,
                condition_bit: 0,
            }],
        };
        carry_session
            .configure_overlay_lifecycle_projection(&state.ctx, &carry_plan)
            .unwrap();
        let started = Instant::now();
        state
            .dispatch_accumulator_threshold_scan(&mut carry_session)
            .unwrap();
        let elapsed_ns = started.elapsed().as_nanos();
        println!(
            "CARRY-MEASUREMENT rows={row_count} bytes={} dispatch_and_submit_ns={elapsed_ns}",
            row_count * std::mem::size_of::<OverlayLifecycleStateGpu>()
        );
    }
}
