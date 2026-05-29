//! Phase M Boundary Resolution Doctrine — audit tests confirming abstract
//! tick/boundary cadence is expressible through existing substrate machinery.
//!
//! This is an audit pass only: no new `DailyResolutionBoundary` runtime primitive.
//! Historical API field names include "day"; constitutional meaning is boundary index.

use simthing_core::{DimensionRegistry, SimProperty, SimThing, SimThingKind};
use simthing_feeder::{feeder_channel, DispatchCoordinator, TransformPatcher};
use simthing_gpu::{GpuContext, Pipelines, SlotAllocator, WorldGpuState};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn minimal_fixture() -> (
    DimensionRegistry,
    SlotAllocator,
    u32,
) {
    let mut reg = DimensionRegistry::new();
    reg.register(SimProperty::simple("core", "loyalty", 0));
    let mut alloc = SlotAllocator::new();
    let a = SimThing::new(SimThingKind::Cohort, 0).id;
    let b = SimThing::new(SimThingKind::Cohort, 0).id;
    alloc.alloc(a);
    alloc.alloc(b);
    let n_dims = reg.total_columns as u32;
    (reg, alloc, n_dims)
}

#[test]
fn doctrine_no_daily_resolution_boundary_primitive() {
    let sources = [
        include_str!("../../simthing-feeder/src/dispatcher.rs"),
        include_str!("../../simthing-feeder/src/lib.rs"),
        include_str!("../../simthing-sim/src/boundary.rs"),
        include_str!("../../simthing-sim/src/lib.rs"),
        include_str!("../src/session.rs"),
        include_str!("../src/lib.rs"),
    ];
    for text in sources {
        assert!(
            !text.contains("DailyResolutionBoundary"),
            "forbidden DailyResolutionBoundary runtime primitive"
        );
    }
}

#[test]
fn doctrine_pause_is_host_non_advancement() {
    let coord = DispatchCoordinator::new(2, 6, 4);
    assert_eq!(coord.tick_index(), 0);
    assert_eq!(coord.day_index(), 0);
    assert_eq!(coord.tick_in_day(), 0);
    // Sim does not autonomously advance; host must invoke tick()/run().
}

#[test]
fn ticks_per_day_one_boundary_every_tick() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, n_dims) = minimal_fixture();
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(alloc.capacity());
    let mut coord = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 1);
    let (_tx, rx) = feeder_channel();

    for i in 1..=3 {
        let out = coord.tick(&rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0);
        assert_eq!(out.tick_index, i);
        assert!(
            out.boundary_reached,
            "ticks_per_day=1 → every tick is a boundary (tick {i})"
        );
        assert_eq!(out.day_index, i, "day_index advances each tick when ticks_per_day=1");
    }
}

#[test]
fn ticks_per_day_four_one_boundary_after_four_ticks() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, n_dims) = minimal_fixture();
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(alloc.capacity());
    let mut coord = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 4);
    let (_tx, rx) = feeder_channel();

    for i in 1..=4 {
        let out = coord.tick(&rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0);
        assert_eq!(out.tick_index, i);
        if i < 4 {
            assert!(!out.boundary_reached, "tick {i} should not signal boundary");
            assert_eq!(out.day_index, 0);
        } else {
            assert!(out.boundary_reached, "tick 4 must signal boundary");
            assert_eq!(out.day_index, 1);
        }
    }
}

#[test]
fn host_pause_preserves_state_after_partial_advancement() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let (reg, alloc, n_dims) = minimal_fixture();
    let mut state = WorldGpuState::new(ctx, &reg, alloc.capacity() as u32);
    let pipelines = Pipelines::new(&state.ctx);
    let mut patcher = TransformPatcher::new(alloc.capacity());
    let mut coord = DispatchCoordinator::new(alloc.capacity() as u32, n_dims, 4);
    let (_tx, rx) = feeder_channel();

    let out = coord.tick(&rx, &mut patcher, &reg, &alloc, &pipelines, &mut state, 0.0);
    assert_eq!(out.tick_index, 1);
    assert!(!out.boundary_reached);

    let frozen_tick = coord.tick_index();
    let frozen_day = coord.day_index();
    let frozen_in_day = coord.tick_in_day();
    // Host chooses not to request the next tick — state remains unchanged.
    assert_eq!(coord.tick_index(), frozen_tick);
    assert_eq!(coord.day_index(), frozen_day);
    assert_eq!(coord.tick_in_day(), frozen_in_day);
}

#[test]
fn daily_resource_economy_fixture_uses_ticks_per_day_one() {
    let session_support = include_str!("support/resource_economy_session.rs");
    let daily_economy_fixture = include_str!("phase_m_daily_economy_fixture.rs");
    let designer = include_str!("resource_economy_designer_ron_session.rs");
    assert!(
        session_support.contains("ticks_per_day: 1"),
        "resource economy session fixtures must use ticks_per_day: 1 for daily cadence"
    );
    assert!(
        daily_economy_fixture.contains("daily_economy_scenario(1,")
            && daily_economy_fixture.contains("open_daily_economy_session(&game_mode, 1,"),
        "daily economy banking fixture must use ticks_per_day: 1"
    );
    assert!(
        designer.contains("ticks_per_day: 1"),
        "resource economy designer RON session must use ticks_per_day: 1"
    );
    assert!(
        session_support.contains("ResourceEconomySpec"),
        "daily banking uses discrete ResourceEconomySpec, not Resource Flow substrate"
    );
    assert!(
        session_support.contains("ResourceTransferSpec"),
        "example discrete banking fixtures use discrete transfers"
    );
}

#[test]
fn doctrine_active_guidance_avoids_canonical_day_overclaims() {
    let guidance_sources = [
        include_str!("../../../docs/workshop/mapping_current_guidance.md"),
        include_str!("../../../docs/workshop/workshop_current_state.md"),
    ];
    let forbidden = [
        "boundary == day",
        "SimThing day primitive",
        "Calendar semantic",
        "canonical play structure",
        "Daily banking is the recommended model",
        "Clausewitz-style daily resolution is represented",
        "Clausewitz-style 1 tick/day resolution is represented",
    ];
    for text in guidance_sources {
        for phrase in forbidden {
            assert!(
                !text.contains(phrase),
                "active guidance must not overclaim day semantics: found `{phrase}`"
            );
        }
    }
}
