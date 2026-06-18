//! One-shot helper to emit `tests/fixtures/runtime_vertical_seed.simthing-scenario.json`.

use std::path::PathBuf;

use simthing_mapeditor::runtime_vertical_seed_scenario_spec;
use simthing_spec::serialize_scenario_authority;

fn main() {
    let scenario = runtime_vertical_seed_scenario_spec();
    let json = serialize_scenario_authority(&scenario).expect("serialize seed");
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/runtime_vertical_seed.simthing-scenario.json");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixtures dir");
    }
    std::fs::write(&path, json).expect("write fixture");
    println!("wrote {}", path.display());
}
