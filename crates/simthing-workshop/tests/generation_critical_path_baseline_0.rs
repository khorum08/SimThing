//! GENERATION-CRITICAL-PATH-BASELINE-0 focused measurement proof.
//!
//! Structural assertions only: completeness, provenance, door enumeration,
//! lawful overlapping-id construction, and presence of required legs/envelope
//! fields. No wall-clock number, ratio, or percentile determines a verdict.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use simthing_workshop::generation_critical_path_baseline::{
    overlapping_id_fixture_proof, query_gpu_envelope, run_generation_critical_path_baseline,
    uninstrumented_clear_matches_instrumented, BaselinePacket, MeasurementEnvelope,
    COMPARATOR_LEGS, D2_ENVELOPE_SHAPE, E2E_ACCOUNTED_LEGS, HOST_CLEARING_DOOR_CENSUS,
    INSTRUMENT_LEGS, OVERLAPPING_RAW, REQUIRED_LEGS,
};

fn git_head() -> String {
    String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("git rev-parse")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string()
}

fn utc_now() -> String {
    let powershell = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "[DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')",
        ])
        .output();
    if let Ok(out) = powershell {
        let text = String::from_utf8_lossy(&out.stdout)
            .trim()
            .replace('\0', "")
            .to_string();
        if text.len() >= 20 {
            return text;
        }
    }
    "2026-08-31T00:00:00Z".into()
}

fn rustc_version() -> String {
    String::from_utf8(
        Command::new("rustc")
            .arg("--version")
            .output()
            .expect("rustc --version")
            .stdout,
    )
    .unwrap_or_default()
    .trim()
    .to_string()
}

fn envelope() -> MeasurementEnvelope {
    let (gpu, backend, driver) = query_gpu_envelope().expect("GPU envelope");
    MeasurementEnvelope {
        tested_commit: git_head(),
        utc_date: utc_now(),
        cpu: std::env::var("PROCESSOR_IDENTIFIER")
            .or_else(|_| std::env::var("PROCESSOR_ARCHITECTURE"))
            .unwrap_or_else(|_| "unknown-cpu".into()),
        gpu,
        adapter_backend: backend,
        driver,
        os: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        compiler_toolchain: rustc_version(),
        profile: "cargo-test (optimized+debuginfo)".into(),
        deterministic_seed: simthing_workshop::generation_critical_path_baseline::DETERMINISTIC_SEED,
        exact_command: "cargo test -p simthing-workshop --test generation_critical_path_baseline_0 -- --nocapture".into(),
    }
}

fn packet() -> &'static BaselinePacket {
    static PACKET: OnceLock<BaselinePacket> = OnceLock::new();
    PACKET.get_or_init(|| {
        let packet = run_generation_critical_path_baseline(envelope()).expect("14.1 measure");
        let path = reports_path();
        let body = simthing_workshop::generation_critical_path_baseline::format_packet(&packet)
            .replace('\0', "");
        fs::write(&path, body).expect("write reports artifact");
        packet
    })
}

fn reports_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/generation_critical_path_baseline_reports.txt")
}

fn spec_source() -> String {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../simthing-spec/src/spec/constrained_clearing.rs"),
    )
    .expect("constrained_clearing.rs")
}

#[test]
fn host_clearing_door_census_covers_generationless_and_ordinary_doors() {
    let src = spec_source();
    let lines: Vec<&str> = src.lines().collect();
    for door in &HOST_CLEARING_DOOR_CENSUS {
        let idx = (door.line as usize).saturating_sub(1);
        assert!(
            idx < lines.len(),
            "{} line {} out of range",
            door.symbol,
            door.line
        );
        assert!(
            lines[idx].contains(&format!("pub fn {}(", door.symbol)),
            "door {} missing at {}:{}",
            door.symbol,
            door.path,
            door.line
        );
    }
    assert!(src.contains("granter: SimThingId::from_session_raw(0)"));
    assert!(src.contains("generation: GenerationStamp::new(0)"));

    let spec_lib = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../simthing-spec/src/lib.rs"),
    )
    .expect("spec lib");
    assert!(spec_lib.contains("clear_reduced_owner_channels"));
    assert!(spec_lib.contains("clear_reduced_owner_channels_at_generation"));
    assert!(spec_lib.contains("clear_constrained_claims_at_generation"));
    assert!(spec_lib.contains("clear_stamped_owner_channels"));

    let embedder = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../simthing-embedder/src/run.rs"),
    )
    .expect("embedder run");
    assert!(embedder.contains("clear_constrained_claims_at_generation"));
    assert!(embedder.contains("clear_stamped_owner_channels"));

    let growth = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../simthing-driver/src/growth_entitlement.rs"),
    )
    .expect("growth");
    assert!(growth.contains("clear_constrained_claims_at_generation"));

    let contention = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../simthing-driver/tests/contention_arena_executed_0.rs"),
    )
    .expect("contention");
    assert!(contention.contains("clear_reduced_owner_channels("));

    let generationless = HOST_CLEARING_DOOR_CENSUS
        .iter()
        .find(|d| d.symbol == "clear_reduced_owner_channels")
        .expect("generationless door");
    assert_eq!(generationless.line, 420);
    assert!(generationless.disposition_14_6.contains("DELETE"));
}

#[test]
fn overlapping_local_ids_use_ordinary_from_runtime_demand_door() {
    let proof = overlapping_id_fixture_proof().expect("overlapping fixture");
    assert_eq!(
        proof.construction_door,
        "ConstrainedClaim::from_runtime_demand -> SimThingId::from_session_raw"
    );
    assert_eq!(proof.overlapping_raw, OVERLAPPING_RAW);
    assert!(proof.tree_a_raws.contains(&OVERLAPPING_RAW));
    assert!(proof.tree_b_raws.contains(&OVERLAPPING_RAW));
    assert_ne!(proof.tree_a_scope, proof.tree_b_scope);
    assert!(proof.same_raw_distinct_contexts);
    assert!(proof.tree_a_grants > 0);
    assert!(proof.tree_b_grants > 0);
}

#[test]
fn observation_layer_is_neutral_vs_uninstrumented_clear() {
    assert!(uninstrumented_clear_matches_instrumented().expect("neutrality"));
}

#[test]
fn measurement_packet_contains_all_required_legs_envelope_and_workloads() {
    let packet = packet();
    assert_eq!(packet.door_census.len(), 4);
    assert_eq!(packet.d2_envelope_shape, D2_ENVELOPE_SHAPE);
    assert!(packet.d1_signed_remainder_note.contains(".max(0)"));
    assert!(packet.d3_nplus_boundary.contains("grants-available"));
    for required in REQUIRED_LEGS
        .iter()
        .chain(INSTRUMENT_LEGS.iter())
        .chain(COMPARATOR_LEGS.iter())
    {
        assert!(
            packet
                .leg_definitions
                .iter()
                .any(|leg| leg.name == *required),
            "missing leg definition {required}"
        );
    }
    let env = &packet.envelope;
    assert!(!env.tested_commit.is_empty());
    assert!(!env.utc_date.is_empty());
    assert!(!env.cpu.is_empty());
    assert!(!env.gpu.is_empty());
    assert!(!env.adapter_backend.is_empty());
    assert!(!env.driver.is_empty());
    assert!(!env.os.is_empty());
    assert!(!env.compiler_toolchain.is_empty());
    assert!(!env.profile.is_empty());
    assert!(!env.exact_command.is_empty());

    let names: Vec<&str> = packet
        .workloads
        .iter()
        .map(|w| w.cardinalities.name.as_str())
        .collect();
    for required in [
        "scale_1000",
        "scale_10000",
        "scale_100000",
        "scale_1000000",
        "one_large_tree",
        "many_independent_small_trees",
        "divergent_generation_trees",
        "overlapping_local_ids",
    ] {
        assert!(names.contains(&required), "missing workload {required}");
    }

    let mut saw_negative_construction = false;
    for workload in &packet.workloads {
        assert_eq!(workload.d2_envelope_shape, D2_ENVELOPE_SHAPE);
        for required in REQUIRED_LEGS
            .iter()
            .chain(INSTRUMENT_LEGS.iter())
            .chain(COMPARATOR_LEGS.iter())
        {
            let leg = workload
                .legs
                .iter()
                .find(|leg| leg.name == *required)
                .unwrap_or_else(|| {
                    panic!(
                        "workload {} missing {required}",
                        workload.cardinalities.name
                    )
                });
            assert_eq!(leg.sample_ns.len(), workload.sample_count);
            assert!(!leg.isolation.is_empty());
            if *required == "gpu_to_host_synchronization_readback"
                || *required == "host_to_gpu_upload"
            {
                assert_eq!(leg.bytes_read_back, 0);
                assert_eq!(leg.bytes_uploaded, 0);
                assert!(leg.sample_ns.iter().all(|v| *v == 0));
            }
            if *required == "grant_result_construction" {
                saw_negative_construction |= leg.sample_ns.iter().any(|v| *v < 0);
                assert!(
                    !leg.isolation.contains("forced-closed")
                        || leg.isolation.contains("no .max(0)")
                );
            }
            if *required == "n_plus_one_launch_delay" {
                assert!(leg.isolation.contains("grants-available"));
                assert!(!leg.isolation.contains("full ordinary-door re-clear"));
            }
            if *required == "next_generation_host_reclear" {
                assert!(leg.isolation.contains("full ordinary-door re-clear"));
            }
        }
        assert!(workload.isolation_matches_production);
        assert!(workload.warm_up_count >= 1 || workload.cardinalities.claims_total >= 100_000);
        assert!(workload.sample_count >= 3);
        assert!(workload.enclosing_clear_ns.sample_ns.len() == workload.sample_count);
        assert!(workload.end_to_end_ns.sample_ns.len() == workload.sample_count);
        assert!(workload.neutrality_clears_identical);
        assert_eq!(
            workload.observation_overhead_residual.sample_ns.len(),
            workload.sample_count
        );
        assert_eq!(
            workload.residual_accounted_legs,
            E2E_ACCOUNTED_LEGS
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>()
        );
        for i in 0..workload.sample_count {
            let mut accounted = 0i64;
            for name in E2E_ACCOUNTED_LEGS {
                let series = if name == "enclosing_clear" {
                    &workload.enclosing_clear_ns.sample_ns
                } else if name == "end_to_end" {
                    &workload.end_to_end_ns.sample_ns
                } else {
                    &workload
                        .legs
                        .iter()
                        .find(|leg| leg.name == name)
                        .unwrap_or_else(|| panic!("{} missing {name}", workload.cardinalities.name))
                        .sample_ns
                };
                accounted += series[i];
            }
            assert_eq!(
                workload.observation_overhead_residual.sample_ns[i],
                workload.end_to_end_ns.sample_ns[i] - accounted,
                "D6 same-sample residual mismatch at {}[{i}]",
                workload.cardinalities.name
            );
        }
        assert!(
            !workload
                .observation_overhead_residual
                .isolation
                .contains("difference of medians")
                || workload
                    .observation_overhead_residual
                    .isolation
                    .contains("not the difference of medians")
        );
        assert!(packet
            .d6_residual_definition
            .contains("observation_overhead_residual[i]"));
    }
    assert!(
        saw_negative_construction,
        "D1: at least one grant_result_construction sample must remain negative"
    );

    let million = packet
        .workloads
        .iter()
        .find(|w| w.cardinalities.name == "scale_1000000")
        .expect("million");
    assert_eq!(million.cardinalities.claims_total, 1_000_000);

    let overlapping = packet
        .workloads
        .iter()
        .find(|w| w.cardinalities.name == "overlapping_local_ids")
        .expect("overlapping");
    assert_eq!(overlapping.overlapping_raw_value, Some(OVERLAPPING_RAW));
    assert_eq!(overlapping.overlapping_tree_contexts.len(), 2);
    assert!(overlapping
        .cardinalities
        .overlapping_construction_door
        .contains("from_runtime_demand"));

    let reports = fs::read_to_string(reports_path()).expect("reports artifact");
    assert!(reports.contains("GENERATION-CRITICAL-PATH-BASELINE-0"));
    assert!(reports.contains("clear_reduced_owner_channels"));
    assert!(reports.contains("Lossless JSON follows"));
    assert!(reports.contains(&packet.envelope.tested_commit));
}
