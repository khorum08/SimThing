//! CAPABILITY-PREREQ-DAG-ADMISSION-0 — RON authored-corpus census + builder gate.
//!
//! ClauseThing tradition trees and live GPU atomicity live in
//! `simthing-clausething/tests/capability_prereq_dag_admission_0.rs` (crate
//! dependency direction forbids clausething from being a simthing-spec
//! dev-dep).

use simthing_core::DimensionRegistry;
use simthing_spec::{
    validate_capability_tree, CapabilityTreeBuilder, CapabilityTreeSpec, GameModeSpec, SpecError,
};

const MINIMAL_TECH: &str = include_str!("fixtures/minimal_tech_tree.ron");
const MINIMAL_GAME_MODE: &str = include_str!("fixtures/minimal_game_mode.ron");
const EXAMPLE_ALL_FACTIONS: &str =
    include_str!("../../../docs/examples/game_mode_install_all_factions.ron");
const EXAMPLE_SCENARIO_LISTED: &str =
    include_str!("../../../docs/examples/game_mode_install_scenario_listed.ron");
const EXAMPLE_SESSION_ROOT: &str =
    include_str!("../../../docs/examples/game_mode_install_session_root.ron");

/// RON authored-corpus labels checked by this crate's census.
const RON_CORPUS_LABELS: &[&str] = &[
    "minimal_tech_tree.ron",
    "minimal_game_mode.ron",
    "docs/examples/game_mode_install_all_factions.ron",
    "docs/examples/game_mode_install_scenario_listed.ron",
    "docs/examples/game_mode_install_session_root.ron",
];

fn strip_ron_comments(src: &str) -> String {
    src.lines()
        .skip_while(|l| l.trim_start().starts_with("//") || l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn trees_from_game_mode(mode: &GameModeSpec) -> Vec<&CapabilityTreeSpec> {
    let mut out = Vec::new();
    for tree in &mode.capability_trees {
        out.push(tree);
    }
    for pack in &mode.domain_packs {
        for tree in &pack.capability_trees {
            out.push(tree);
        }
    }
    out
}

fn seed_mode_properties(mode: &GameModeSpec, registry: &mut DimensionRegistry) {
    for prop in &mode.properties {
        let _ = simthing_spec::compile_property(prop, registry);
    }
    for pack in &mode.domain_packs {
        for prop in &pack.properties {
            let _ = simthing_spec::compile_property(prop, registry);
        }
    }
}

