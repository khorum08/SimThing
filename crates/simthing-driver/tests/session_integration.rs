//! End-to-end GPU tests for `SimSession`. Skips cleanly when no adapter.

use std::collections::HashMap;
use std::io::BufReader;
use std::time::Instant;

use simthing_driver::{check_bench_ceiling, Scenario, SimSession};
use simthing_gpu::GpuContext;
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayReader};

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn run_bench_scenario(ron: &str) -> (f64, simthing_driver::RunSummary, String) {
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let name = scenario.name.clone();
    let max_days = scenario.max_days;
    let mut session = SimSession::open(scenario).expect("session open");
    let started = Instant::now();
    let summary = session.run(max_days).expect("session run");
    (started.elapsed().as_secs_f64() * 1000.0, summary, name)
}

#[test]
fn rebellion_demo_ron_runs_fission_via_sim_session() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.run(4).expect("session run");

    assert_eq!(summary.boundaries_run, 4);
    assert_eq!(summary.ticks_run, 4);
    assert!(
        summary.fission_events >= 1,
        "rebellion demo should fission within 4 days, got {}",
        summary.fission_events
    );
    assert_eq!(
        session.proto.root.subtree_size(),
        4,
        "world + location + cohort + one fission child"
    );
    assert_eq!(session.proto.fission_lineage().len(), 1);
}

#[test]
fn record_rebellion_demo_replay_round_trips_structural_state() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("session.replay.ldjson");

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.record_to_path(&path, 4).expect("record session");
    assert_eq!(summary.frames_written, 4);
    assert!(summary.fission_events >= 1);

    let file = std::fs::File::open(&path).expect("replay file");
    let mut reader = ReplayReader::new(BufReader::new(file));
    let mut driver = ReplayDriver::from_snapshot(reader.read_snapshot().expect("snapshot"));

    let mut frames = 0u32;
    let mut entry_counts: HashMap<&'static str, u32> = HashMap::new();
    while let Some(frame) = reader.next_frame().expect("read frame") {
        for entry in &frame.entries {
            *entry_counts.entry(replay_entry_kind(entry)).or_default() += 1;
        }
        driver.apply_frame(frame);
        frames += 1;
    }

    assert_eq!(frames, 4);
    assert_eq!(driver.day, 4);
    assert_eq!(driver.root.subtree_size(), 4);
    assert_eq!(driver.fission_lineage.len(), 1);
    assert_eq!(
        driver.fission_lineage.len(),
        session.proto.fission_lineage().len()
    );
    assert!(
        entry_counts.get("FissionOccurred").copied().unwrap_or(0) >= 1,
        "expected FissionOccurred in replay log, got {entry_counts:?}"
    );
    assert!(
        entry_counts
            .get("FissionLineageAdded")
            .copied()
            .unwrap_or(0)
            >= 1,
        "expected FissionLineageAdded in replay log, got {entry_counts:?}"
    );
}

#[test]
fn bench_stress_scenarios_within_ceiling() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    for (ron, label) in [
        (
            include_str!("../../../scenarios/intent_stress.ron"),
            "intent_stress",
        ),
        (
            include_str!("../../../scenarios/fission_stress.ron"),
            "fission_stress",
        ),
    ] {
        let (elapsed_ms, summary, name) = run_bench_scenario(ron);
        assert_eq!(name, label);
        check_bench_ceiling(&name, elapsed_ms, &summary).unwrap_or_else(|err| {
            panic!("{label} bench ceiling: {err} (elapsed_ms={elapsed_ms:.3})");
        });
    }
}

fn replay_entry_kind(entry: &BoundaryDeltaEntry) -> &'static str {
    match entry {
        BoundaryDeltaEntry::OverlayAttached { .. } => "OverlayAttached",
        BoundaryDeltaEntry::OverlayDissolved { .. } => "OverlayDissolved",
        BoundaryDeltaEntry::OverlayActivated { .. } => "OverlayActivated",
        BoundaryDeltaEntry::OverlaySuspended { .. } => "OverlaySuspended",
        BoundaryDeltaEntry::SimThingAdded { .. } => "SimThingAdded",
        BoundaryDeltaEntry::SimThingRemoved { .. } => "SimThingRemoved",
        BoundaryDeltaEntry::DimensionAdded { .. } => "DimensionAdded",
        BoundaryDeltaEntry::FissionOccurred { .. } => "FissionOccurred",
        BoundaryDeltaEntry::FusionOccurred { .. } => "FusionOccurred",
        BoundaryDeltaEntry::PropertyExpired { .. } => "PropertyExpired",
        BoundaryDeltaEntry::SimThingReparented { .. } => "SimThingReparented",
        BoundaryDeltaEntry::VelocityAlert { .. } => "VelocityAlert",
        BoundaryDeltaEntry::AggregateAlert { .. } => "AggregateAlert",
        BoundaryDeltaEntry::FissionLineageAdded { .. } => "FissionLineageAdded",
        BoundaryDeltaEntry::FissionLineageRemoved { .. } => "FissionLineageRemoved",
    }
}
