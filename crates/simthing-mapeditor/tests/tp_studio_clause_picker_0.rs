//! TP-STUDIO-CLAUSE-PICKER-0 — admitted narrow UI picker as production API caller.
//!
//! Native dialog is thin over `run_clause_picker_action` / `open_clause_scenario_with_picker`
//! (dependency-injected paths). Terran-Pirate fixtures are **caller-supplied test data only**.

use std::path::PathBuf;

use simthing_mapeditor::{
    clause_picker_menu_label, open_clause_scenario_with_picker, parse_clause_resolver_entries,
    run_clause_picker_action, run_clause_picker_ingest_then_session, ClausePickerActionResult,
    ClausePickerSelection, ClauseScenarioSourceResolver, FakeClauseFilePicker,
    OPEN_CLAUSE_SCENARIO_ACTION_LABEL,
};
use simthing_mapeditor::studio_scenario_load::ScenarioPickerOutcome;
use simthing_spec::validate_stead_mapping_consistency;
use tempfile::TempDir;

const PLACEHOLDER: &str = "{{FIXTURE_JSON}}";

fn clause_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause",
    )
}

fn embedded_json_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
}

fn caller_selection(json_out: PathBuf) -> ClausePickerSelection {
    let mut resolver = std::collections::BTreeMap::new();
    resolver.insert(PLACEHOLDER.to_string(), embedded_json_path());
    ClausePickerSelection {
        clause_path: clause_path(),
        resolver_entries: resolver,
        scenario_json_path: Some(json_out),
    }
}

#[test]
fn picker_0_action_invokes_production_clause_api() {
    let tmp = TempDir::new().expect("tmp");
    let json = tmp.path().join("picker.simthing-scenario.json");
    let result = run_clause_picker_action(&caller_selection(json), None);
    match result {
        ClausePickerActionResult::Loaded { ingest, .. } => {
            assert_eq!(ingest.report.projection_mode, "StructuralRebindReady");
            assert_eq!(ingest.report.stead_validation, "PASS");
        }
        other => panic!("expected Loaded, got {}", other.message()),
    }

    // Action module must call production ingest symbols, not reimplement parse/rebind.
    let picker_src = include_str!("../src/clause_scenario_picker.rs");
    assert!(
        picker_src.contains("load_clause_studio_session_from_path")
            || picker_src.contains("ingest_clause_scenario_path")
    );
    assert!(picker_src.contains("clause_scenario_ingest"));
    assert!(!picker_src.contains("parse_raw_document"));
    assert!(!picker_src.contains("rebind_pack_to_structural_rebind_ready"));
    assert!(!picker_src.contains("hydrate_scenario"));
}

#[test]
fn picker_0_session_hydrates_after_picker_flow() {
    let tmp = TempDir::new().expect("tmp");
    let json = tmp.path().join("session.simthing-scenario.json");
    let selection = caller_selection(json);
    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Loaded {
            session, ingest, ..
        } => {
            assert_eq!(
                session.scenario_authority.scenario_id,
                ingest.scenario.scenario_id
            );
            assert!(!session.scenario_authority.structural_grid.placements.is_empty());
            validate_stead_mapping_consistency(&session.scenario_authority).expect("stead");
        }
        other => panic!("{}", other.message()),
    }

    // Direct ingest → from_loaded_scenario path also PASS.
    let (ingest, session) = run_clause_picker_ingest_then_session(
        &caller_selection(tmp.path().join("label.simthing-scenario.json")),
        PathBuf::from("picker_label.simthing-scenario.json"),
        None,
    )
    .expect("ingest+session");
    assert_eq!(
        session.scenario_authority.structural_grid.map_container_id,
        ingest.scenario.structural_grid.map_container_id
    );
}

#[test]
fn picker_0_unresolved_placeholder_surfaces_error() {
    let tmp = TempDir::new().expect("tmp");
    let selection = ClausePickerSelection {
        clause_path: clause_path(),
        resolver_entries: std::collections::BTreeMap::new(),
        scenario_json_path: Some(tmp.path().join("fail.simthing-scenario.json")),
    };
    match run_clause_picker_action(&selection, None) {
        ClausePickerActionResult::Failed { message } => {
            assert!(
                message.contains("placeholder")
                    || message.contains("unresolved")
                    || message.contains("resolver"),
                "{message}"
            );
            assert!(!message.is_empty());
        }
        other => panic!("expected Failed, got {}", other.message()),
    }
}

#[test]
fn picker_0_no_tp_or_fixture_defaults() {
    let picker_src = include_str!("../src/clause_scenario_picker.rs");
    for banned in ["tp_base_disc_1500", "terran_pirate_galaxy", "TP-FULL-TRANSPILE"] {
        assert!(
            !picker_src.contains(banned),
            "picker must not hardcode {banned}"
        );
    }
    // Default selection has empty resolver — no silent defaults in type defaults.
    let empty = ClausePickerSelection::default();
    assert!(empty.resolver_entries.is_empty());
    assert!(empty.clause_path.as_os_str().is_empty());

    let parsed = parse_clause_resolver_entries("").expect("empty ok");
    assert!(parsed.is_empty());
    // Explicit empty resolver must not invent placeholders.
    let _ = ClauseScenarioSourceResolver::new();
}

#[test]
fn picker_0_no_duplicate_parse_or_rebind_path() {
    let picker_src = include_str!("../src/clause_scenario_picker.rs");
    assert!(!picker_src.contains("parse_raw_document("));
    assert!(!picker_src.contains("hydrate_scenario("));
    assert!(!picker_src.contains("rebind_pack_to_structural_rebind_ready("));
    assert!(!picker_src.contains("project_pack_to_authority_tree_candidate("));
    // Must compose production API surface only.
    assert!(
        picker_src.contains("load_clause_studio_session_from_path")
            && picker_src.contains("load_studio_session_from_clause_ingest_result")
    );
}

#[test]
fn picker_0_no_gamemode_rf_live_run_closeout() {
    let picker_src = include_str!("../src/clause_scenario_picker.rs");
    // Ban attach/live-run/closeout wiring (not incidental word fragments).
    for banned in [
        "combat_arena",
        "AttachOverlay",
        "live_run",
        "track_closeout",
        "BoundaryRequest",
        "GameModeSpec",
        "attach_rf",
        "ArenaPressureBinding",
    ] {
        assert!(
            !picker_src.contains(banned),
            "picker must not mention {banned}"
        );
    }
    let ui_src = include_str!("../src/app/ui.rs");
    // Menu affordance must exist.
    assert!(
        ui_src.contains("Open ClauseScript Scenario") || ui_src.contains("clause_picker_menu_label")
    );
    assert_eq!(clause_picker_menu_label(), OPEN_CLAUSE_SCENARIO_ACTION_LABEL);
}

#[test]
fn picker_0_injected_dialog_calls_action() {
    let tmp = TempDir::new().expect("tmp");
    let json = tmp.path().join("dialog.simthing-scenario.json");
    let mut resolver = std::collections::BTreeMap::new();
    resolver.insert(PLACEHOLDER.to_string(), embedded_json_path());
    let fake = FakeClauseFilePicker {
        outcome: ScenarioPickerOutcome::Selected(clause_path()),
    };
    let result = open_clause_scenario_with_picker(
        &fake,
        ".",
        resolver,
        Some(json),
        None,
    );
    assert!(
        result.is_success(),
        "{}",
        result.message()
    );
}

#[test]
fn picker_0_resolver_parse_accepts_token_forms() {
    let map = parse_clause_resolver_entries(
        "# comment\nFIXTURE_JSON=/tmp/a.json\n{{OTHER}}=C:/x/y.json\n",
    )
    .expect("parse");
    assert_eq!(
        map.get("{{FIXTURE_JSON}}").map(|p| p.to_string_lossy().into_owned()),
        Some("/tmp/a.json".into())
    );
    assert!(map.contains_key("{{OTHER}}"));
}
