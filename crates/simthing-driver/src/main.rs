//! SimThing CLI — record and inspect LDJSON replays.

use std::env;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

use simthing_driver::{Scenario, SimSession};
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayReader};

fn usage() -> &'static str {
    "simthing record --scenario <file.ron> --out <file.ldjson> [--days N]\n\
     simthing replay  --in <file.ldjson>"
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

    let summary = session.record_to_path(&out_path, max_days).unwrap_or_else(|e| {
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
    let mut entry_counts: std::collections::HashMap<&'static str, u32> = std::collections::HashMap::new();

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
    println!("  fission lineage records: {}", driver.fission_lineage.len());
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
        BoundaryDeltaEntry::SimThingAdded { .. } => "SimThingAdded",
        BoundaryDeltaEntry::SimThingRemoved { .. } => "SimThingRemoved",
        BoundaryDeltaEntry::DimensionAdded { .. } => "DimensionAdded",
        BoundaryDeltaEntry::FissionOccurred { .. } => "FissionOccurred",
        BoundaryDeltaEntry::FusionOccurred { .. } => "FusionOccurred",
        BoundaryDeltaEntry::PropertyExpired { .. } => "PropertyExpired",
        BoundaryDeltaEntry::SimThingReparented { .. } => "SimThingReparented",
        BoundaryDeltaEntry::VelocityAlert { .. } => "VelocityAlert",
        BoundaryDeltaEntry::FissionLineageAdded { .. } => "FissionLineageAdded",
        BoundaryDeltaEntry::FissionLineageRemoved { .. } => "FissionLineageRemoved",
    }
}
