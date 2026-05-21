//! SimThing CLI — record and inspect LDJSON replays.

use std::env;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

use simthing_driver::{Scenario, SimSession};
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayReader};

fn usage() -> &'static str {
    "simthing record --scenario <file.ron> --out <file.ldjson> [--days N]\n\
     simthing replay  --in <file.ldjson>\n\
     simthing bench   --scenario <file.ron> [--days N] [--check]"
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", usage());
        process::exit(1);
    }

    match args[1].as_str() {
        "record" => cmd_record(&args[2..]),
        "replay" => cmd_replay(&args[2..]),
        "bench" => cmd_bench(&args[2..]),
        other => {
            eprintln!("unknown subcommand {other:?}\n{}", usage());
            process::exit(1);
        }
    }
}

fn cmd_record(args: &[String]) {
    let mut scenario_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut days: Option<u32> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(args.get(i).expect("--scenario needs a path")));
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(args.get(i).expect("--out needs a path")));
            }
            "--days" => {
                i += 1;
                days = Some(
                    args.get(i)
                        .expect("--days needs a value")
                        .parse()
                        .expect("--days must be a u32"),
                );
            }
            flag => {
                eprintln!("unknown flag {flag:?}\n{}", usage());
                process::exit(1);
            }
        }
        i += 1;
    }

    let scenario_path = scenario_path.unwrap_or_else(|| {
        eprintln!("--scenario is required\n{}", usage());
        process::exit(1);
    });
    let out_path = out_path.unwrap_or_else(|| {
        eprintln!("--out is required\n{}", usage());
        process::exit(1);
    });

    let scenario = Scenario::from_ron_path(&scenario_path).unwrap_or_else(|e| {
        eprintln!("failed to load scenario: {e}");
        process::exit(1);
    });
    let max_days = days.unwrap_or(scenario.max_days);

    let mut session = SimSession::open(scenario).unwrap_or_else(|e| {
        eprintln!("failed to open session: {e}");
        process::exit(1);
    });

    let summary = session
        .record_to_path(&out_path, max_days)
        .unwrap_or_else(|e| {
            eprintln!("recording failed: {e}");
            process::exit(1);
        });

    println!(
        "recorded {} → {} frames ({} ticks, {} fission events)",
        out_path.display(),
        summary.frames_written,
        summary.ticks_run,
        summary.fission_events,
    );
}

fn cmd_bench(args: &[String]) {
    let mut scenario_path: Option<PathBuf> = None;
    let mut days: Option<u32> = None;
    let mut check_ceiling = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--scenario" => {
                i += 1;
                scenario_path = Some(PathBuf::from(args.get(i).expect("--scenario needs a path")));
            }
            "--days" => {
                i += 1;
                days = Some(
                    args.get(i)
                        .expect("--days needs a value")
                        .parse()
                        .expect("--days must be a u32"),
                );
            }
            "--check" => {
                check_ceiling = true;
            }
            flag => {
                eprintln!("unknown flag {flag:?}\n{}", usage());
                process::exit(1);
            }
        }
        i += 1;
    }

    let scenario_path = scenario_path.unwrap_or_else(|| {
        eprintln!("--scenario is required\n{}", usage());
        process::exit(1);
    });
    let scenario = Scenario::from_ron_path(&scenario_path).unwrap_or_else(|e| {
        eprintln!("failed to load scenario: {e}");
        process::exit(1);
    });
    let max_days = days.unwrap_or(scenario.max_days);
    let mut session = SimSession::open(scenario).unwrap_or_else(|e| {
        eprintln!("failed to open session: {e}");
        process::exit(1);
    });

    let n_slots_start = session.coord.n_slots();
    let n_dims = session.coord.n_dims();
    let ticks_per_day = session.coord.ticks_per_day();
    let started = Instant::now();
    let summary = session.run(max_days).unwrap_or_else(|e| {
        eprintln!("benchmark failed: {e}");
        process::exit(1);
    });
    let elapsed = started.elapsed();
    let ms_total = elapsed.as_secs_f64() * 1000.0;
    let ms_per_tick = if summary.ticks_run == 0 {
        0.0
    } else {
        ms_total / summary.ticks_run as f64
    };
    let ms_per_day = if summary.boundaries_run == 0 {
        0.0
    } else {
        ms_total / summary.boundaries_run as f64
    };
    let avg_tick = |ms: f64| {
        if summary.ticks_run == 0 {
            0.0
        } else {
            ms / summary.ticks_run as f64
        }
    };
    let avg_boundary = |ms: f64| {
        if summary.boundaries_run == 0 {
            0.0
        } else {
            ms / summary.boundaries_run as f64
        }
    };

    println!("benchmark {}", session.scenario.name);
    println!("  n_slots_start: {n_slots_start}");
    println!("  n_slots_end: {}", session.coord.n_slots());
    println!("  n_dims: {n_dims}");
    println!("  ticks_per_day: {ticks_per_day}");
    println!("  ticks_run: {}", summary.ticks_run);
    println!("  days_run: {}", summary.boundaries_run);
    println!("  fission_events: {}", summary.fission_events);
    println!("  elapsed_ms: {:.3}", ms_total);
    println!("  ms_per_tick: {:.6}", ms_per_tick);
    println!("  ms_per_sim_day: {:.6}", ms_per_day);
    println!("  tick_measured_ms: {:.3}", summary.tick_total_ms);
    println!(
        "  tick_measured_ms_per_tick: {:.6}",
        avg_tick(summary.tick_total_ms)
    );
    println!(
        "  tick_submit_patches_ms: {:.3}",
        summary.submit_tick_patches_ms
    );
    println!("  tick_drain_ms: {:.3}", summary.tick_drain_ms);
    println!(
        "  tick_intent_upload_ms: {:.3}",
        summary.tick_intent_upload_ms
    );
    println!(
        "  tick_dirty_upload_ms: {:.3}",
        summary.tick_dirty_upload_ms
    );
    println!(
        "  tick_gpu_pipeline_submit_ms: {:.3}",
        summary.tick_gpu_pipeline_ms
    );
    println!(
        "  tick_event_readback_ms: {:.3}",
        summary.tick_event_readback_ms
    );
    println!(
        "  tick_event_readback_bytes: {}",
        summary.tick_event_readback_bytes
    );
    println!("  boundary_ms: {:.3}", summary.boundary_total_ms);
    println!(
        "  boundary_ms_per_day: {:.6}",
        avg_boundary(summary.boundary_total_ms)
    );
    println!(
        "  boundary_value_readback_ms: {:.3}",
        summary.boundary_value_readback_ms
    );
    println!(
        "  boundary_alert_collect_ms: {:.3}",
        summary.boundary_alert_collect_ms
    );
    println!(
        "  boundary_lifecycle_ms: {:.3}",
        summary.boundary_lifecycle_ms
    );
    println!("  boundary_expiry_ms: {:.3}", summary.boundary_expiry_ms);
    println!(
        "  boundary_pregrow_fission_ms: {:.3}",
        summary.boundary_pregrow_fission_ms
    );
    println!("  boundary_fission_ms: {:.3}", summary.boundary_fission_ms);
    println!("  boundary_lineage_ms: {:.3}", summary.boundary_lineage_ms);
    println!(
        "  boundary_request_drain_ms: {:.3}",
        summary.boundary_request_drain_ms
    );
    println!(
        "  boundary_pregrow_add_child_ms: {:.3}",
        summary.boundary_pregrow_add_child_ms
    );
    println!(
        "  boundary_structural_ms: {:.3}",
        summary.boundary_structural_ms
    );
    println!(
        "  boundary_dimension_rebuild_ms: {:.3}",
        summary.boundary_dimension_rebuild_ms
    );
    println!(
        "  boundary_final_capacity_ms: {:.3}",
        summary.boundary_final_capacity_ms
    );
    println!(
        "  boundary_gpu_sync_ms: {:.3}",
        summary.boundary_gpu_sync_ms
    );
    println!(
        "  boundary_delta_log_ms: {:.3}",
        summary.boundary_delta_log_ms
    );
    println!("  boundaries_skipped: {}", summary.boundaries_skipped);
    println!("  rmw_rows_synced: {}", summary.rmw_rows_synced);
    println!("  rmw_readback_bytes: {}", summary.rmw_readback_bytes);
    println!(
        "  intent_deltas_uploaded: {}",
        summary.intent_deltas_uploaded
    );
    println!("  intent_delta_bytes: {}", summary.intent_delta_bytes);
    println!(
        "  boundary_readback_bytes: {}",
        summary.boundary_readback_bytes
    );
    println!("  boundary_upload_bytes: {}", summary.boundary_upload_bytes);
    println!(
        "  boundary_value_rows_uploaded: {}",
        summary.boundary_value_rows_uploaded
    );
    println!(
        "  boundary_full_value_uploads: {}",
        summary.boundary_full_value_uploads
    );
    println!(
        "  overlay_deltas_uploaded: {}",
        summary.overlay_deltas_uploaded
    );
    println!(
        "  threshold_regs_uploaded: {}",
        summary.threshold_regs_uploaded
    );
    println!(
        "  reduction_edges_uploaded: {}",
        summary.reduction_edges_uploaded
    );
    println!(
        "  reduction_slots_uploaded: {}",
        summary.reduction_slots_uploaded
    );
    println!(
        "  reduction_depths_total: {}",
        summary.reduction_depths_total
    );
    println!("  reduction_depths_max: {}", summary.reduction_depths_max);
    println!(
        "  final_gpu_buffer_bytes: {}",
        session.state.total_buffer_bytes()
    );

    if check_ceiling {
        match simthing_driver::check_bench_ceiling(&session.scenario.name, ms_total, &summary) {
            Ok(ms_per_day) => {
                println!("  bench_check: pass ({ms_per_day:.3} ms/sim-day)");
            }
            Err(err) => {
                eprintln!("bench check failed: {err}");
                process::exit(1);
            }
        }
    }
}

fn cmd_replay(args: &[String]) {
    let mut in_path: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--in" => {
                i += 1;
                in_path = Some(PathBuf::from(args.get(i).expect("--in needs a path")));
            }
            flag => {
                eprintln!("unknown flag {flag:?}\n{}", usage());
                process::exit(1);
            }
        }
        i += 1;
    }

    let in_path = in_path.unwrap_or_else(|| {
        eprintln!("--in is required\n{}", usage());
        process::exit(1);
    });

    let file = std::fs::File::open(&in_path).unwrap_or_else(|e| {
        eprintln!("failed to open {}: {e}", in_path.display());
        process::exit(1);
    });
    let mut reader = ReplayReader::new(BufReader::new(file));
    let snapshot = reader.read_snapshot().unwrap_or_else(|e| {
        eprintln!("failed to read snapshot: {e}");
        process::exit(1);
    });

    let mut driver = ReplayDriver::from_snapshot(snapshot);
    let mut frame_count = 0u32;
    let mut entry_counts: std::collections::HashMap<&'static str, u32> =
        std::collections::HashMap::new();

    while let Some(frame) = reader.next_frame().unwrap_or_else(|e| {
        eprintln!("failed to read frame: {e}");
        process::exit(1);
    }) {
        for entry in &frame.entries {
            *entry_counts.entry(entry_kind(entry)).or_default() += 1;
        }
        driver.apply_frame(frame);
        frame_count += 1;
    }

    println!("replay {}", in_path.display());
    println!("  frames applied: {frame_count}");
    println!("  final day: {}", driver.day);
    println!("  tree nodes: {}", driver.root.subtree_size());
    println!(
        "  fission lineage records: {}",
        driver.fission_lineage.len()
    );
    println!("  entry kinds:");
    let mut kinds: Vec<_> = entry_counts.into_iter().collect();
    kinds.sort_by_key(|(k, _)| *k);
    for (kind, count) in kinds {
        println!("    {kind}: {count}");
    }
}

fn entry_kind(entry: &BoundaryDeltaEntry) -> &'static str {
    match entry {
        BoundaryDeltaEntry::OverlayAttached { .. } => "OverlayAttached",
        BoundaryDeltaEntry::OverlayDissolved { .. } => "OverlayDissolved",
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
