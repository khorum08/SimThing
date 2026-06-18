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
];

#[test]
fn no_studio_runtime_loop_uses_structural_link_accumulator() {
    for path in app_source_files() {
        let source = fs::read_to_string(&path).expect("read app source");
        for token in FORBIDDEN_RUNTIME_TOKENS {
            assert!(
                !source.contains(token),
                "{} must not reference retired bespoke accumulator path ({token})",
                path.display()
            );
        }
    }
}

#[test]
fn studio_proof_helpers_do_not_run_as_runtime() {
    let scenario_io = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/studio_scenario_load.rs"),
    )
    .expect("studio_scenario_load");
    let app_mod =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/app/mod.rs"))
            .expect("app mod");
    for token in FORBIDDEN_RUNTIME_TOKENS {
        assert!(!scenario_io.contains(token));
        assert!(!app_mod.contains(token));
    }
}

#[test]
fn studio_app_sources_do_not_construct_accumulator_op_session() {
    for path in app_source_files() {
        let source = fs::read_to_string(&path).expect("read app source");
        for token in FORBIDDEN_GPU_DISPATCH_TOKENS {
            assert!(
                !source.contains(token),
                "{} must not own GPU accumulator dispatch ({token})",
                path.display()
            );
        }
    }
}

#[test]
fn studio_load_path_does_not_execute_sim_gpu_tick() {
    let scenario_io = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/studio_scenario_load.rs"),
    )
    .expect("studio_scenario_load");
    let app_mod =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/app/mod.rs"))
            .expect("app mod");
    for token in FORBIDDEN_GPU_DISPATCH_TOKENS {
        assert!(!scenario_io.contains(token));
        assert!(!app_mod.contains(token));
    }
}

#[test]
fn studio_load_path_does_not_execute_accumulator_runtime() {
    let source = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/studio_scenario_load.rs"),
    )
    .expect("load");
    assert!(!source.contains("accumulator"));
    assert!(!source.contains("structural_link"));
}

#[test]
fn studio_remains_projection_and_proof_harness() {
    let scenario = runtime_vertical_seed_scenario_spec();
    let plan = compile_structural_link_neighbor_sum_plan(
        &scenario,
        StructuralScalarChannel(0),
        StructuralScalarChannel(1),
    )
    .expect("driver compile from scenario authority");
    let output = execute_accumulator_plan_tick_cpu(&plan, &[10.0, 20.0]).expect("sim tick");
    assert_eq!(output, vec![20.0, 10.0]);
}

#[test]
fn documentation_as_code_driver_stub_removed_or_not_counted_as_evidence() {
    let stub =
        repo_root().join("crates/simthing-driver/tests/accumulator_driver_sim_convergence_stub.rs");
    assert!(
        !stub.is_file(),
        "documentation-as-code driver stub must be removed"
    );
}

#[test]
fn accumulator_convergence_constants_removed_or_not_counted_as_evidence() {
    let module = repo_root().join("crates/simthing-gpu/src/accumulator_convergence.rs");
    assert!(
        !module.is_file(),
        "documentation-as-code convergence constants must be removed"
    );
}

#[test]
fn structural_link_accumulator_removed_or_marked_deprecated_proof_only() {
    let module = repo_root().join("crates/simthing-gpu/src/structural_link_accumulator.rs");
    let shader =
        repo_root().join("crates/simthing-gpu/src/shaders/structural_link_accumulator.wgsl");
    assert!(
        !module.is_file(),
        "bespoke structural_link_accumulator module must be deleted"
    );
    assert!(
        !shader.is_file(),
        "bespoke structural_link_accumulator shader must be deleted"
    );
}

#[test]
fn production_doc_names_driver_sim_as_execution_owner() {
    let doc = read_repo_file("docs/0.8.3 Simthing Studio Production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("simthing-driver"));
    assert!(lower.contains("simthing-sim"));
    assert!(lower.contains("accumulator-driver-sim-convergence-1"));
}

#[test]
fn production_doc_names_accumulator_op_as_convergence_target() {
    let doc = read_repo_file("docs/0.8.3 Simthing Studio Production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("sum-over-input_list"));
    assert!(lower.contains("accumulatorop"));
}

#[test]
fn production_doc_names_sim_gpu_accumulator_backend() {
    let doc = read_repo_file("docs/0.8.3 Simthing Studio Production.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower.contains("sim-gpu-accumulator-backend-0"));
    assert!(lower.contains("execute_accumulator_plan_tick_gpu"));
}
