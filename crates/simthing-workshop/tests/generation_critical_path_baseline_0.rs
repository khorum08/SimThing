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
    HOST_CLEARING_DOOR_CENSUS, OVERLAPPING_RAW, REQUIRED_LEGS,
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
    assert_eq!(packet.leg_definitions.len(), 11);
    for required in REQUIRED_LEGS {
        assert!(
            packet
                .leg_definitions
                .iter()
                .any(|leg| leg.name == required),
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

    for workload in &packet.workloads {
        assert_eq!(workload.legs.len(), 11);
        for required in REQUIRED_LEGS {
            let leg = workload
                .legs
                .iter()
                .find(|leg| leg.name == required)
                .unwrap_or_else(|| {
                    panic!(
                        "workload {} missing {required}",
                        workload.cardinalities.name
                    )
                });
            assert_eq!(leg.sample_ns.len(), workload.sample_count);
            assert!(!leg.isolation.is_empty());
            if required == "gpu_to_host_synchronization_readback"
                || required == "host_to_gpu_upload"
            {
                assert_eq!(leg.bytes_read_back, 0);
                assert_eq!(leg.bytes_uploaded, 0);
                assert!(leg.sample_ns.iter().all(|v| *v == 0));
            }
        }
        assert!(workload.isolation_matches_production);
        assert!(workload.warm_up_count >= 1 || workload.cardinalities.claims_total >= 100_000);
        assert!(workload.sample_count >= 3);
        assert!(workload.enclosing_clear_ns.sample_ns.len() == workload.sample_count);
        assert!(workload.end_to_end_ns.sample_ns.len() == workload.sample_count);
        assert!(workload.neutrality_clears_identical);
    }

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
