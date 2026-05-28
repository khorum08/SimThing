//! Phase M-2: generic cadence scheduler and dirty macro-region skip.

use simthing_driver::{
    count_cadence_due_ticks, execute_single_scheduled_stencil_region, visit_scheduled_regions,
    DirtyRegionState, FieldCadence, FieldDispatchDecision, FieldDispatchReason,
    FieldDispatchSchedule, FieldGridDescriptor, FieldId, FieldRegionId, FieldRegionRegistration,
    FieldScheduleState, FieldScheduler, FieldSchedulerError, ScheduledStencilExecutionError,
};
use simthing_gpu::{
    GpuContext, StructuredFieldExecutionOptions, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn assert_no_false_skips(
    scheduler: &FieldScheduler,
    tick: u32,
    decisions: &[simthing_driver::FieldDispatchDecision],
) {
    for decision in decisions {
        if !matches!(decision.schedule, FieldDispatchSchedule::Skip) {
            continue;
        }
        let region = scheduler
            .regions()
            .iter()
            .find(|r| r.field_id == decision.field_id && r.region_id == decision.region_id)
            .expect("region");
        let field = scheduler
            .fields()
            .iter()
            .find(|f| f.field_id == region.field_id)
            .expect("field");
        let must = FieldScheduler::region_must_schedule(
            &region.dirty,
            field.cadence,
            tick,
            field.event_pending,
        )
        .unwrap();
        assert!(!must, "false skip on region {:?}", decision.region_id);
    }
}

#[test]
fn test_a_cadence_determinism_and_replay() {
    let ticks = 120;
    assert_eq!(
        count_cadence_due_ticks(FieldCadence::EveryTick, ticks, &[]).unwrap(),
        120
    );
    assert_eq!(
        count_cadence_due_ticks(FieldCadence::EveryN { n: 4 }, ticks, &[]).unwrap(),
        30
    );
    assert_eq!(
        count_cadence_due_ticks(FieldCadence::EveryN { n: 10 }, ticks, &[]).unwrap(),
        12
    );
    assert_eq!(
        count_cadence_due_ticks(FieldCadence::EveryN { n: 60 }, ticks, &[]).unwrap(),
        2
    );
    let event_ticks = [7u32, 23, 89];
    assert_eq!(
        count_cadence_due_ticks(FieldCadence::OnEvent, ticks, &event_ticks).unwrap(),
        3
    );

    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(0),
        cadence: FieldCadence::EveryN { n: 4 },
        event_pending: false,
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(0),
        dirty: DirtyRegionState::default(),
    });

    let (a, _) = scheduler.decide_tick(8).unwrap();
    let (b, _) = scheduler.decide_tick(8).unwrap();
    assert_eq!(a, b, "replay must be deterministic");
}

#[test]
fn test_b_invalid_cadence_rejected() {
    assert_eq!(
        FieldCadence::EveryN { n: 0 }.validate(),
        Err(FieldSchedulerError::InvalidEveryNZero)
    );
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(0),
        cadence: FieldCadence::EveryN { n: 0 },
        event_pending: false,
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(0),
        dirty: DirtyRegionState::default(),
    });
    assert_eq!(
        scheduler.decide_tick(0).unwrap_err(),
        FieldSchedulerError::InvalidEveryNZero
    );
}

#[test]
fn test_c_dirty_skip_correctness_zero_false_skips() {
    let tick = 1;
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(0),
        cadence: FieldCadence::EveryN { n: 4 },
        event_pending: false,
    });
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(1),
        cadence: FieldCadence::EveryTick,
        event_pending: false,
    });

    let cases = [
        (FieldRegionId(0), FieldId(0), DirtyRegionState::default(), FieldDispatchSchedule::Skip),
        (
            FieldRegionId(1),
            FieldId(0),
            DirtyRegionState {
                dirty_source_present: true,
                ..Default::default()
            },
            FieldDispatchSchedule::Dispatch,
        ),
        (
            FieldRegionId(2),
            FieldId(0),
            DirtyRegionState {
                dirty_neighbor_present: true,
                ..Default::default()
            },
            FieldDispatchSchedule::Dispatch,
        ),
        (
            FieldRegionId(3),
            FieldId(0),
            DirtyRegionState {
                residual_present: true,
                ..Default::default()
            },
            FieldDispatchSchedule::Dispatch,
        ),
        (
            FieldRegionId(4),
            FieldId(0),
            DirtyRegionState {
                topology_generation: 2,
                last_topology_generation: 1,
                ..Default::default()
            },
            FieldDispatchSchedule::Dispatch,
        ),
        (
            FieldRegionId(5),
            FieldId(0),
            DirtyRegionState {
                operator_generation: 5,
                last_operator_generation: 2,
                ..Default::default()
            },
            FieldDispatchSchedule::Dispatch,
        ),
        (
            FieldRegionId(6),
            FieldId(1),
            DirtyRegionState::default(),
            FieldDispatchSchedule::Dispatch,
        ),
    ];

    for (region_id, field_id, dirty, _) in cases {
        scheduler.register_region(FieldRegionRegistration {
            region_id,
            field_id,
            dirty,
        });
    }

    let (decisions, report) = scheduler.decide_tick(tick).unwrap();
    assert_eq!(report.false_skip_count, 0);
    assert_no_false_skips(&scheduler, tick, &decisions);

    for (region_id, field_id, _, expected) in cases {
        let decision = decisions
            .iter()
            .find(|d| d.region_id == region_id && d.field_id == field_id)
            .expect("decision");
        assert_eq!(decision.schedule, expected, "region {:?}", region_id);
    }
}

#[test]
fn test_m2_1_region_identity_includes_field_identity() {
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(1),
        cadence: FieldCadence::EveryTick,
        event_pending: false,
    });
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(2),
        cadence: FieldCadence::EveryTick,
        event_pending: false,
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(1),
        dirty: DirtyRegionState::default(),
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(2),
        dirty: DirtyRegionState::default(),
    });
    assert_eq!(scheduler.regions().len(), 2);
    let (decisions, _) = scheduler.decide_tick(0).unwrap();
    assert_eq!(decisions.len(), 2);
}

#[test]
fn test_m2_1_same_field_region_replacement() {
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(1),
        cadence: FieldCadence::EveryN { n: 100 },
        event_pending: false,
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(1),
        dirty: DirtyRegionState::default(),
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(1),
        dirty: DirtyRegionState {
            dirty_source_present: true,
            ..Default::default()
        },
    });
    assert_eq!(scheduler.regions().len(), 1);
    let (decisions, _) = scheduler.decide_tick(1).unwrap();
    assert_eq!(decisions.len(), 1);
    assert!(matches!(
        decisions[0].schedule,
        FieldDispatchSchedule::Dispatch
    ));
}

#[test]
fn test_m2_1_scheduled_visitor_does_not_execute_skipped() {
    let decisions = [
        FieldDispatchDecision {
            region_id: FieldRegionId(1),
            field_id: FieldId(0),
            schedule: FieldDispatchSchedule::Skip,
            reasons: vec![],
        },
        FieldDispatchDecision {
            region_id: FieldRegionId(2),
            field_id: FieldId(0),
            schedule: FieldDispatchSchedule::Dispatch,
            reasons: vec![FieldDispatchReason::DirtySource],
        },
        FieldDispatchDecision {
            region_id: FieldRegionId(3),
            field_id: FieldId(1),
            schedule: FieldDispatchSchedule::Dispatch,
            reasons: vec![FieldDispatchReason::CadenceDue],
        },
    ];
    let mut calls = 0u32;
    let executed = visit_scheduled_regions(&decisions, |_d| {
        calls += 1;
        Ok::<(), ()>(())
    })
    .unwrap();
    assert_eq!(calls, 2);
    assert_eq!(
        executed,
        vec![(FieldId(0), FieldRegionId(2)), (FieldId(1), FieldRegionId(3))]
    );
}

#[test]
fn test_m2_1_single_op_guard_rejects_multiple_scheduled() {
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(0),
        cadence: FieldCadence::EveryTick,
        event_pending: false,
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(0),
        field_id: FieldId(0),
        dirty: DirtyRegionState::default(),
    });
    scheduler.register_region(FieldRegionRegistration {
        region_id: FieldRegionId(1),
        field_id: FieldId(0),
        dirty: DirtyRegionState::default(),
    });
    let (decisions, _) = scheduler.decide_tick(0).unwrap();
    with_gpu(|ctx| {
        let config = StructuredFieldStencilConfig {
            width: 3,
            height: 3,
            n_dims: 4,
            source_col: 0,
            target_col: 0,
            horizon: 1,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let err = execute_single_scheduled_stencil_region(
            ctx,
            &op,
            &decisions,
            StructuredFieldExecutionOptions::default(),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ScheduledStencilExecutionError::MultipleScheduledRegionsForSingleOp { count: 2 }
        ));
    });
}

#[test]
fn test_d_scheduler_report_metrics() {
    let mut scheduler = FieldScheduler::new();
    scheduler.register_field(FieldScheduleState {
        field_id: FieldId(0),
        cadence: FieldCadence::EveryN { n: 100 },
        event_pending: false,
    });
    for id in 0..10u32 {
        let dirty = if id < 4 {
            DirtyRegionState {
                dirty_source_present: true,
                ..Default::default()
            }
        } else {
            DirtyRegionState::default()
        };
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(id),
            field_id: FieldId(0),
            dirty,
        });
    }

    let (decisions, report) = scheduler.decide_tick(7).unwrap();
    assert_eq!(report.total_regions, 10);
    assert_eq!(report.scheduled_regions, 4);
    assert_eq!(report.skipped_regions, 6);
    assert!((report.skip_ratio - 0.6).abs() < 1e-6);
    assert_eq!(report.false_skip_count, 0);
    assert_eq!(
        decisions
            .iter()
            .filter(|d| matches!(d.schedule, FieldDispatchSchedule::Dispatch))
            .count(),
        4
    );
}

#[test]
fn test_e_scheduled_execution_uses_no_readback_default() {
    with_gpu(|ctx| {
        let config = StructuredFieldStencilConfig {
            width: 3,
            height: 3,
            n_dims: 4,
            source_col: 0,
            target_col: 0,
            horizon: 2,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let values = vec![0.0f32; op.config().values_len()];
        op.upload_values(ctx, &values).unwrap();

        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: FieldId(0),
            cadence: FieldCadence::EveryN { n: 4 },
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(0),
            dirty: DirtyRegionState {
                dirty_source_present: true,
                ..Default::default()
            },
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(1),
            field_id: FieldId(0),
            dirty: DirtyRegionState::default(),
        });

        let (decisions, _) = scheduler.decide_tick(1).unwrap();
        let executed = execute_single_scheduled_stencil_region(
            ctx,
            &op,
            &decisions,
            StructuredFieldExecutionOptions::default(),
        )
        .unwrap()
        .expect("one scheduled region");

        assert_eq!(executed.field_id, FieldId(0));
        assert_eq!(executed.region_id, FieldRegionId(0));
        assert!(executed.report.values.is_none());
        assert_eq!(executed.report.debug.dispatch_count, 2);
    });
}

#[test]
fn test_f_readback_only_for_oracle_debug() {
    with_gpu(|ctx| {
        let config = StructuredFieldStencilConfig {
            width: 3,
            height: 3,
            n_dims: 4,
            source_col: 0,
            target_col: 0,
            horizon: 1,
            alpha_self: 1.0,
            gamma_neighbor: 0.8,
            source_cap: None,
            operator: StructuredFieldStencilOperator::Normalized,
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let op = StructuredFieldStencilOp::new(ctx, config).unwrap();
        let mut values = vec![0.0f32; op.config().values_len()];
        values[4] = 100.0;
        op.upload_values(ctx, &values).unwrap();

        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: FieldId(0),
            cadence: FieldCadence::EveryTick,
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(0),
            dirty: DirtyRegionState::default(),
        });

        let (decisions, _) = scheduler.decide_tick(0).unwrap();
        let no_readback = execute_single_scheduled_stencil_region(
            ctx,
            &op,
            &decisions,
            StructuredFieldExecutionOptions::default(),
        )
        .unwrap()
        .expect("scheduled");
        assert!(no_readback.report.values.is_none());
        assert!(!StructuredFieldExecutionOptions::default().readback_values);

        op.upload_values(ctx, &values).unwrap();
        let with_readback = execute_single_scheduled_stencil_region(
            ctx,
            &op,
            &decisions,
            StructuredFieldExecutionOptions {
                readback_values: true,
                ..Default::default()
            },
        )
        .unwrap()
        .expect("scheduled");
        assert!(with_readback.report.values.is_some());
    });
}

#[test]
fn test_g_alternate_square_size_fixtures_size_agnostic() {
    let tick = 3;
    let grids = [
        FieldGridDescriptor {
            field_id: FieldId(1),
            width: 5,
            height: 5,
        },
        FieldGridDescriptor {
            field_id: FieldId(2),
            width: 10,
            height: 10,
        },
        FieldGridDescriptor {
            field_id: FieldId(3),
            width: 20,
            height: 20,
        },
    ];

    let mut baseline: Option<Vec<FieldDispatchSchedule>> = None;
    for grid in grids {
        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: grid.field_id,
            cadence: FieldCadence::EveryN { n: 4 },
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: grid.field_id,
            dirty: DirtyRegionState {
                dirty_neighbor_present: tick % 2 == 1,
                ..Default::default()
            },
        });
        let (decisions, _) = scheduler.decide_tick(tick).unwrap();
        let schedules: Vec<_> = decisions.iter().map(|d| d.schedule).collect();
        if let Some(base) = &baseline {
            assert_eq!(
                &schedules,
                base,
                "grid {}x{} must not change scheduler logic",
                grid.width,
                grid.height
            );
        } else {
            baseline = Some(schedules);
        }
    }
}

#[test]
fn test_h_no_production_pass_graph_wiring() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let passes = include_str!("../../simthing-gpu/src/passes.rs");
    assert!(!passes.contains("FieldScheduler"));
    assert!(!passes.contains("field_scheduler"));
    assert!(!passes.contains("RegionField"));

    let session = include_str!("../../simthing-driver/src/session.rs");
    assert!(!session.contains("FieldScheduler"));
    assert!(!session.contains("field_scheduler"));
    assert!(!session.contains("RegionField"));

    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_lib.contains("FieldScheduler"));
    assert!(!sim_lib.contains("RegionField"));
    assert!(!sim_lib.contains("Mapping"));
}
