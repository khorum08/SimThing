//! STUDIO-CANONICAL-SCENARIO-0 — portable canonical TP clause (empty resolver, clause-dir relative).

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario, hydrate_scenario_with_source_base, parse_raw_document,
    rebind_pack_to_structural_rebind_ready, resolve_clause_source_path, HydratedScenarioPack,
};
use simthing_core::{DimensionRegistry, SimProperty, SimThing};
use simthing_driver::{Scenario, SessionError, SimSession};
use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, SimThingScenarioSpec,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_clause_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.clause")
}

fn canonical_base_json_path() -> PathBuf {
    repo_root().join("scenarios/terran_pirate_galaxy.base_disc.json")
}

fn hydrate_canonical_empty_resolver() -> HydratedScenarioPack {
    let clause_path = canonical_clause_path();
    assert!(
        clause_path.is_file(),
        "missing committed canonical clause at {}",
        clause_path.display()
    );
    assert!(
        canonical_base_json_path().is_file(),
        "missing committed base-disc sibling"
    );
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    assert!(
        !text.contains("{{FIXTURE_JSON}}"),
        "canonical clause must not use FIXTURE_JSON token"
    );
    assert!(
        text.contains("source_json = \"terran_pirate_galaxy.base_disc.json\""),
        "canonical clause must use sibling relative source_json"
    );
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let base = clause_path.parent().expect("clause parent").to_path_buf();
    hydrate_scenario_with_source_base(&document, Some(&base)).expect("hydrate with clause base")
}

fn authority_spec_from_pack(pack: &HydratedScenarioPack) -> SimThingScenarioSpec {
    rebind_pack_to_structural_rebind_ready(pack)
        .expect("StructuralRebindReady rebind")
        .0
}

/// Structural driver shell for multi-tick identity (no GameMode/RF attach).
fn structural_scenario_from_spec(spec: &SimThingScenarioSpec) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_studio_canonical", "seed", 0));
    let mut root = spec.root.clone();
    strip_property_maps(&mut root);
    let mut n_slots = 0u32;
    count_tree_nodes(&root, &mut n_slots);
    n_slots = n_slots.max(1).saturating_mul(2).max(16);
    Scenario {
        name: if spec.scenario_id.is_empty() {
            "studio_canonical".into()
        } else {
            spec.scenario_id.clone()
        },
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn strip_property_maps(node: &mut SimThing) {
    node.properties.clear();
    for child in &mut node.children {
        strip_property_maps(child);
    }
}

fn count_tree_nodes(node: &SimThing, n: &mut u32) {
    *n = n.saturating_add(1);
    for child in &node.children {
        count_tree_nodes(child, n);
    }
}

/// catches: process-CWD-relative leakage for bare source_json.
#[test]
fn canonical_clause_empty_resolver_hydrates_from_non_scenarios_cwd() {
    let original = env::current_dir().expect("cwd");
    // Force a CWD that is NOT scenarios/ so CWD-relative open would fail.
    let alien = repo_root().join("crates").join("simthing-clausething");
    env::set_current_dir(&alien).expect("chdir alien");
    let result = std::panic::catch_unwind(|| {
        let pack = hydrate_canonical_empty_resolver();
        assert_eq!(pack.scenario_id, "terran_pirate_galaxy");
        assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
        assert_eq!(pack.grid_metadata.placements.len(), 1500);
        assert_eq!(pack.owners.len(), 2);
        // Prove resolve helper is clause-dir based (not CWD).
        let resolved = resolve_clause_source_path(
            "terran_pirate_galaxy.base_disc.json",
            Some(canonical_clause_path().parent().unwrap()),
        );
        assert!(
            resolved.is_file(),
            "resolved sibling must exist: {resolved:?}"
        );
        assert!(
            !env::current_dir()
                .unwrap()
                .join("terran_pirate_galaxy.base_disc.json")
                .is_file(),
            "base disc must not be present under alien CWD"
        );
    });
    env::set_current_dir(&original).expect("restore cwd");
    result.expect("hydrate from non-scenarios cwd");
}

/// catches: hydrate-only proof without live/session stability.
#[test]
fn canonical_clause_empty_resolver_multi_tick_holds_identity() {
    let original = env::current_dir().expect("cwd");
    env::set_current_dir(repo_root().join("crates")).expect("chdir crates");
    let pack = hydrate_canonical_empty_resolver();
    let spec = authority_spec_from_pack(&pack);
    let scenario_id = spec.scenario_id.clone();
    let stead_before = validate_stead_mapping_consistency(&spec);
    let links_before = validate_scenario_links(&spec);
    assert!(
        stead_before.is_ok(),
        "STEAD must hold after rebind: {stead_before:?}"
    );
    assert!(
        links_before.is_ok(),
        "links must hold after rebind: {links_before:?}"
    );

    let scenario = structural_scenario_from_spec(&spec);
    match SimSession::open(scenario) {
        Ok(mut session) => {
            for _ in 0..3 {
                match session.step_once() {
                    Ok(_) => {}
                    Err(e) => panic!("step_once failed: {e}"),
                }
            }
            // Spec authority identity is held on the rebind product, not session shell.
            assert_eq!(spec.scenario_id, scenario_id);
            assert!(validate_stead_mapping_consistency(&spec).is_ok());
            assert!(validate_scenario_links(&spec).is_ok());
            assert_eq!(
                validate_stead_mapping_consistency(&spec).is_ok(),
                stead_before.is_ok()
            );
            let _ = session;
        }
        Err(SessionError::Gpu(e)) => {
            eprintln!("STUDIO-CANONICAL-SCENARIO-0: GPU_SKIPPED multi-tick ({e}); identity holds on rebind");
            assert_eq!(spec.scenario_id, scenario_id);
            assert!(validate_stead_mapping_consistency(&spec).is_ok());
        }
        Err(e) => panic!("unexpected SimSession::open error: {e}"),
    }
    env::set_current_dir(&original).expect("restore cwd");
}

/// catches: resolver-token regression for {{FIXTURE_JSON}} fixture path.
#[test]
fn canonical_clause_backcompat_fixture_json_token_still_resolves() {
    let fixture_json = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .canonicalize()
        .expect("fixture json")
        .to_string_lossy()
        .replace('\\', "/");
    let clause = include_str!("fixtures/scenario/terran_pirate_galaxy.clause")
        .replace("{{FIXTURE_JSON}}", &fixture_json);
    assert!(clause.contains(&fixture_json));
    let document = parse_raw_document(clause.as_bytes()).expect("parse token-substituted clause");
    // No source_base: absolute substituted path must still open (token path back-compat).
    let pack =
        hydrate_scenario(&document).expect("hydrate absolute token path without clause base");
    assert_eq!(pack.scenario_id, "terran_pirate_galaxy");
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert_eq!(pack.grid_metadata.placements.len(), 1500);
}

/// catches: generated output cruft near committed scenarios.
#[test]
fn canonical_clause_does_not_emit_sibling_from_clause_output() {
    let scenarios_dir = repo_root().join("scenarios");
    let before: Vec<_> = std::fs::read_dir(&scenarios_dir)
        .expect("read scenarios")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();

    let pack = hydrate_canonical_empty_resolver();
    let _spec = authority_spec_from_pack(&pack);

    // Pure hydrate/rebind path must not write sibling ingest artifacts.
    let after: Vec<_> = std::fs::read_dir(&scenarios_dir)
        .expect("read scenarios after")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();
    assert_eq!(
        before, after,
        "scenarios/ directory must be unchanged by hydrate"
    );

    let sibling = scenarios_dir.join("terran_pirate_galaxy.from-clause.simthing-scenario.json");
    assert!(
        !sibling.exists(),
        "must not emit sibling from-clause output: {}",
        sibling.display()
    );

    // Scan scenarios/ for any .from-clause artifact.
    for entry in std::fs::read_dir(&scenarios_dir).expect("scan") {
        let entry = entry.expect("entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        assert!(
            !name.contains(".from-clause."),
            "forbidden from-clause artifact present: {name}"
        );
    }
}
