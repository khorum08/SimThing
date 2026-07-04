//! ACCUMULATOR-DRIVER-SIM-CONVERGENCE-1 — Studio/runtime constitutional guards.

use std::fs;
use std::path::PathBuf;

use simthing_core::StructuralScalarChannel;
use simthing_driver::compile_structural_link_neighbor_sum_plan;
use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_sim::execute_accumulator_plan_tick_cpu;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_repo_file(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|err| panic!("failed to read {rel}: {err}"))
}

fn app_source_files() -> Vec<PathBuf> {
    let app_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/app");
    fs::read_dir(&app_dir)
        .expect("app dir")
        .map(|entry| entry.expect("entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .collect()
}

const FORBIDDEN_RUNTIME_TOKENS: &[&str] = &[
    "structural_link_accumulator",
    "execute_structural_link_accumulator_on_gpu",
    "accumulate_structural_rows_on_gpu",
    "prove_gpu_link_accumulator_smoke_blocking",
    "prove_runtime_vertical_seed_gpu_link_accumulator_blocking",
];

const FORBIDDEN_GPU_DISPATCH_TOKENS: &[&str] = &[
    "AccumulatorOpSession::new",
    "upload_ops_resolving_input_lists",
    "execute_accumulator_plan_tick_gpu",
    "execute_accumulator_plan_tick_with_backend",
    "SimGpuAccumulatorTickState::new",
    "SimGpuAccumulatorTickState",
    "ProofReadback",
    "set_debug_readback_allowed",
    "scoped_debug_readback_allowed",
    "DebugReadbackGuard",
];

#[test]
fn production_doc_names_terran_pirate_scenario_skeleton() {
    let doc = read_repo_file("docs/design_0_0_8_3_studio_production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("terran-pirate-scenario-skeleton-0"));
    assert!(lower.contains("terran_pirate_skeleton"));
}

#[test]
fn production_doc_names_terran_pirate_scenario_skeleton_0r() {
    let doc = read_repo_file("docs/design_0_0_8_3_studio_production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("terran-pirate-scenario-skeleton-0r"));
    assert!(lower.contains("scenarios/horizon/terran_pirate_skeleton"));
}
