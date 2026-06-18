//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-0 — constitutional guards for structural execution convergence.

use std::fs;
use std::path::PathBuf;

use simthing_gpu::{
    ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL, ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES,
    DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE, SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE,
    STRUCTURAL_LINK_ACCUMULATOR_NOT_RUNTIME, STRUCTURAL_LINK_ACCUMULATOR_PROOF_ONLY,
    STRUCTURAL_LINK_ACCUMULATOR_SMOKE_ONLY,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_repo_file(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|err| {
        panic!(
            "failed to read {rel} from repo root {:?}: {err}",
            repo_root()
        )
    })
}

fn app_source_files() -> Vec<PathBuf> {
    let app_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/app");
    let mut files = Vec::new();
    for entry in fs::read_dir(&app_dir).expect("app dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
    files
}

const RUNTIME_BYPASS_TOKENS: &[&str] = &[
    "prove_gpu_link_accumulator_smoke_blocking",
    "prove_runtime_vertical_seed_gpu_link_accumulator_blocking",
    "execute_structural_link_accumulator_on_gpu",
    "accumulate_structural_rows_on_gpu",
    "structural_link_accumulator",
];

#[test]
fn structural_link_accumulator_marked_proof_only() {
    assert!(STRUCTURAL_LINK_ACCUMULATOR_PROOF_ONLY);
    assert!(STRUCTURAL_LINK_ACCUMULATOR_SMOKE_ONLY);
    assert!(STRUCTURAL_LINK_ACCUMULATOR_NOT_RUNTIME);
}

#[test]
fn no_studio_runtime_loop_uses_structural_link_accumulator() {
    for path in app_source_files() {
        let source = fs::read_to_string(&path).expect("read app source");
        for token in RUNTIME_BYPASS_TOKENS {
            assert!(
                !source.contains(token),
                "{} must not reference structural link accumulator smoke path ({token})",
                path.display()
            );
        }
    }
}

#[test]
fn accumulator_convergence_gap_report_exists() {
    let path = repo_root().join(ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL);
    assert!(path.is_file(), "missing gap report at {}", path.display());
}

#[test]
fn accumulator_convergence_gap_report_names_missing_generic_capability() {
    let report = read_repo_file(ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL);
    let lower = report.to_ascii_lowercase();
    for capability in ACCUMULATOR_OP_MISSING_GENERIC_CAPABILITIES {
        let needle = capability
            .split_whitespace()
            .next()
            .expect("non-empty capability");
        assert!(
            lower.contains(&needle.to_ascii_lowercase()),
            "gap report should mention capability fragment: {needle}"
        );
    }
}

#[test]
fn accumulator_convergence_gap_report_does_not_use_domain_semantics() {
    const FORBIDDEN: &[&str] = &[
        "route",
        "predecessor",
        "pathfinding",
        "movement_order",
        "fleet",
        "faction",
        "owner",
        "border",
        "frontline",
        "combat",
        "economy",
        "diplomacy",
        "pirate",
        "terran",
    ];
    let report = read_repo_file(ACCUMULATOR_CONVERGENCE_GAP_REPORT_REL);
    let stripped = report
        .split('`')
        .enumerate()
        .filter_map(|(idx, chunk)| if idx % 2 == 0 { Some(chunk) } else { None })
        .collect::<String>();
    let lower = stripped.to_ascii_lowercase();
    for token in FORBIDDEN {
        assert!(
            !lower.contains(token),
            "gap report prose must not use domain token: {token}"
        );
    }
}

#[test]
fn production_doc_names_driver_sim_as_execution_owner() {
    let doc = read_repo_file("docs/0.8.3 Simthing Studio Production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("simthing-driver"));
    assert!(lower.contains("simthing-sim"));
    assert!(lower.contains(&DRIVER_STRUCTURAL_ACCUMULATOR_COMPILE_CRATE.to_ascii_lowercase()));
    assert!(lower.contains(&SIM_STRUCTURAL_ACCUMULATOR_TICK_CRATE.to_ascii_lowercase()));
    assert!(lower.contains("accumulator-driver-sim-convergence-0"));
}

#[test]
fn production_doc_names_accumulator_op_as_convergence_target() {
    let doc = read_repo_file("docs/0.8.3 Simthing Studio Production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("accumulatorop"));
    assert!(lower.contains("ao-wgsl-0"));
    assert!(lower.contains("structural_link_accumulator"));
    assert!(lower.contains("proof"));
}
