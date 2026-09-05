use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const COMPONENTS: &[&str] = &[
    "Cargo.lock",
    "crates/simthing-driver/src/child_share_eml.rs",
    "crates/simthing-core/src/eml_nodes.rs",
    "crates/simthing-core/src/accumulator_op.rs",
    "crates/simthing-core/src/accumulator_op_builder.rs",
    "crates/simthing-driver/src/arena_allocation_plan.rs",
    "crates/simthing-driver/src/arena_allocation_sync.rs",
    "crates/simthing-driver/src/arena_hierarchy.rs",
    "crates/simthing-kernel/src/accumulator_op/mod.rs",
    "crates/simthing-kernel/src/accumulator_op/types.rs",
    "crates/simthing-kernel/src/accumulator_op/encode.rs",
    "crates/simthing-kernel/src/accumulator_op/cpu_oracle.rs",
    "crates/simthing-kernel/src/accumulator_op/session.rs",
    "crates/simthing-kernel/src/shaders/accumulator_op.wgsl",
    "crates/simthing-kernel/src/resident_clearing_plan.rs",
    "crates/simthing-core/src/persistence_deformation.rs",
    "crates/simthing-kernel/src/resident_clearing_apportionment.rs",
    "crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl",
    "crates/simthing-kernel/src/resident_recursive_intake_transform.rs",
    "crates/simthing-kernel/src/shaders/resident_recursive_intake_transform.wgsl",
    "crates/simthing-gpu/src/resident_clearing_plan.rs",
    "crates/simthing-driver/src/resident_clearing_runtime.rs",
    "crates/simthing-driver/src/session.rs",
    "crates/simthing-driver/src/growth_entitlement.rs",
    "crates/simthing-driver/src/spec_session.rs",
    "crates/simthing-sim/src/boundary.rs",
    "crates/simthing-sim/src/sim_runtime_tree.rs",
    "crates/simthing-spec/src/spec/flow_market.rs",
    "crates/simthing-spec/src/spec/constrained_clearing.rs",
    "crates/simthing-spec/src/spec/scenario.rs",
    "crates/simthing-core/src/owner_channel.rs",
    "crates/simthing-clausething/src/hydrate_shipsize_decoder.rs",
];

fn hash(mut state: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100_0000_01b3);
    }
    state
}

fn extend_component(mut state: u64, name: &str, bytes: &[u8]) -> u64 {
    state = hash(state, &(name.len() as u64).to_le_bytes());
    state = hash(state, name.as_bytes());
    state = hash(state, &(bytes.len() as u64).to_le_bytes());
    hash(state, bytes)
}

fn workspace_root() -> PathBuf {
    Path::new(&env::var_os("CARGO_MANIFEST_DIR").expect("Cargo sets CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("simthing-gpu is under <workspace>/crates")
        .to_owned()
}

fn main() {
    let workspace = workspace_root();
    let mut cargo_features = env::vars()
        .filter_map(|(name, value)| {
            name.strip_prefix("CARGO_FEATURE_")
                .filter(|_| value == "1")
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    cargo_features.sort();
    let cargo_features = cargo_features.join(",");
    let compiler = Command::new(env::var_os("RUSTC").expect("Cargo sets RUSTC"))
        .arg("-Vv")
        .output()
        .expect("build-time rustc provenance probe must run");
    assert!(
        compiler.status.success(),
        "build-time rustc provenance probe must succeed"
    );
    let compiler = String::from_utf8(compiler.stdout)
        .expect("rustc provenance is UTF-8")
        .replace("\r\n", "\n")
        .trim()
        .to_owned();

    let mut bundle = 0xcbf2_9ce4_8422_2325;
    let mut component_records = Vec::with_capacity(COMPONENTS.len());
    for name in COMPONENTS {
        let path = workspace.join(name);
        let bytes = fs::read(&path).unwrap_or_else(|error| {
            panic!(
                "failed to read qualification component {}: {error}",
                path.display()
            )
        });
        println!("cargo:rerun-if-changed={}", path.display());
        let component_hash = hash(0xcbf2_9ce4_8422_2325, &bytes);
        bundle = extend_component(bundle, name, &bytes);
        component_records.push((*name, component_hash));
    }

    let cargo_lock = fs::read(workspace.join("Cargo.lock")).expect("workspace Cargo.lock exists");
    let cargo_lock_hash = hash(0xcbf2_9ce4_8422_2325, &cargo_lock);
    let records = component_records
        .iter()
        .map(|(name, digest)| format!("    ({name:?}, 0x{digest:016x}),\n"))
        .collect::<String>();
    let generated = format!(
        "pub const BUILD_RUSTC_PROVENANCE: &str = {compiler:?};\n\
         pub const BUILD_CARGO_FEATURES: &str = {cargo_features:?};\n\
         pub const DEPENDENCY_LOCK_HASH: u64 = 0x{cargo_lock_hash:016x};\n\
         pub const SEMANTIC_KERNEL_COMPONENTS: &[(&str, u64)] = &[\n{records}];\n\
         pub const SEMANTIC_KERNEL_BUNDLE_HASH: u64 = 0x{bundle:016x};\n"
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    fs::write(out.join("resident_clearing_build_provenance.rs"), generated)
        .expect("write generated resident qualification provenance");
}
