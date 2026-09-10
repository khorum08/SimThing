//! Native source/cache boundary and declared dependency witnesses.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use simthing_core::SubFieldRole;
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot};
use simthing_mapeditor::clause_scenario_ingest::{
    ingest_clause_scenario_path, load_clause_studio_session_from_path,
    load_studio_session_from_clause_ingest_result, save_clause_scenario_cache_to_path,
    ClauseScenarioIngestOptions,
};
use simthing_mapeditor::{
    load_studio_session_from_scenario_path, StudioLiveSessionBridge, StudioSession,
};
use simthing_spec::PropertyKey;

fn copy_bundle(directory: &Path) -> PathBuf {
    let sources = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    for suffix in ["clause", "base.json", "dependencies.json"] {
        let name = format!("rehearsal_ingress_cache.{suffix}");
        std::fs::copy(sources.join(&name), directory.join(name)).unwrap();
    }
    directory.join("rehearsal_ingress_cache.clause")
}

fn born_values(session: &StudioSession) -> BTreeMap<String, f32> {
    let mut bridge = StudioLiveSessionBridge::default();
    bridge.open_from_loaded_studio_session(session).unwrap();
    bridge.consume_scheduled_ticks(1).unwrap();
    let sim = bridge.sim_session().unwrap();
    let snapshot = AnchorTableSnapshot::from_session(sim);
    let profile = session
        .authored_live_profile
        .as_ref()
        .expect("cache must preserve the authored program");
    let mut values = BTreeMap::new();
    for owner in ["alpha", "beta"] {
        for index in 0..5 {
            let entity = format!("{owner}_{index}");
            let property = format!("{entity}_energy_quantity");
            let value = observe_hosted_property_cell(
                &sim.proto.registry,
                &sim.proto.allocator,
                &snapshot,
                profile.install_targets[&entity][0],
                &PropertyKey::new("cache_economy", property),
                &SubFieldRole::Amount,
            )
            .unwrap();
            values.insert(entity, value);
        }
        let value = observe_hosted_property_cell(
            &sim.proto.registry,
            &sim.proto.allocator,
            &snapshot,
            profile.install_targets[owner][0],
            &PropertyKey::new("cache_economy", format!("{owner}_energy_stockpile")),
            &SubFieldRole::Amount,
        )
        .unwrap();
        values.insert(owner.into(), value);
    }
    values
}

#[test]
fn rehearsal_ingress_native_cache_preserves_born_values_and_policy_changes() {
    let dir = tempfile::tempdir().unwrap();
    let source = copy_bundle(dir.path());
    let cache = dir.path().join("cache.simthing-scenario.json");
    let options = ClauseScenarioIngestOptions::default();
    let (ingest, native) =
        load_clause_studio_session_from_path(&source, &options, &cache, None).unwrap();
    let first_bytes = std::fs::read(&cache).unwrap();
    save_clause_scenario_cache_to_path(&cache, &ingest).unwrap();
    assert_eq!(
        std::fs::read(&cache).unwrap(),
        first_bytes,
        "cache bytes are reproducible"
    );
    let baseline = born_values(&native);
    let saved = dir.path().join("saved.simthing-scenario.json");
    simthing_mapeditor::save_current_session_scenario_to_path(&native, &saved).unwrap();
    assert_eq!(std::fs::read(&saved).unwrap(), first_bytes);
    let mut edited = native.clone();
    simthing_spec::set_owner_display_name(
        &mut edited.scenario_authority,
        "alpha",
        "Edited outside native source",
    )
    .unwrap();
    let refused_save = dir.path().join("must-not-be-written.json");
    assert!(
        simthing_mapeditor::save_current_session_scenario_to_path(&edited, &refused_save).is_err()
    );
    assert!(!refused_save.exists());
    let mut edited = native.clone();
    edited
        .authored_live_profile
        .as_mut()
        .unwrap()
        .game_mode
        .overlays
        .clear();
    assert!(
        simthing_mapeditor::save_current_session_scenario_to_path(&edited, &refused_save).is_err()
    );
    assert!(!refused_save.exists());
    let cached = load_studio_session_from_scenario_path(&cache, None).unwrap();
    assert_eq!(born_values(&cached), baseline);
    assert_eq!(baseline.len(), 12);
    assert_eq!(
        baseline["alpha_4"], 0.0,
        "zero-valued present quantity remains observable"
    );
    assert_eq!(baseline["beta_4"], 0.0);
    let mut tampered: serde_json::Value = serde_json::from_slice(&first_bytes).unwrap();
    tampered["document"]["root"]["properties"][0]["value"]["text"] = "999".into();
    std::fs::write(&cache, serde_json::to_vec(&tampered).unwrap()).unwrap();
    assert!(
        load_studio_session_from_scenario_path(&cache, None).is_err(),
        "the cache cannot author independent economics"
    );
    for nonportable in ["C:source.clause", "C:/source.clause", "/source.clause"] {
        let mut tampered: serde_json::Value = serde_json::from_slice(&first_bytes).unwrap();
        tampered["source_path"] = nonportable.into();
        std::fs::write(&cache, serde_json::to_vec(&tampered).unwrap()).unwrap();
        let error = load_studio_session_from_scenario_path(&cache, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("non-relative native source"), "{error}");
    }
    std::fs::write(&cache, &first_bytes).unwrap();

    let text = std::fs::read_to_string(&source).unwrap();
    for (changed, locus) in [
        (
            text.replace(
                "location = \"beta_3\" resource = \"energy\" amount = 8",
                "location = \"beta_3\" resource = \"energy\" amount = 20",
            ),
            "beta_3",
        ),
        (text.replace("@beta_policy = 3", "@beta_policy = 7"), "beta"),
    ] {
        std::fs::write(&source, changed).unwrap();
        assert!(
            load_studio_session_from_scenario_path(&cache, None).is_err(),
            "stale cache refuses source changes"
        );
        let (_, native) =
            load_clause_studio_session_from_path(&source, &options, &cache, None).unwrap();
        let current = born_values(&native);
        eprintln!("native/cache baseline={baseline:?}; variant {locus}={current:?}");
        assert_ne!(
            current[locus], baseline[locus],
            "authored change must reach born output"
        );
        assert_eq!(
            born_values(&load_studio_session_from_scenario_path(&cache, None).unwrap()),
            current
        );
    }
}

#[test]
fn rehearsal_ingress_bundle_relocates_and_refuses_dependency_drift() {
    if let Some(cache) = std::env::var_os("SIMTHING_INGRESS_CACHE_PROBE") {
        let session = load_studio_session_from_scenario_path(Path::new(&cache), None).unwrap();
        assert_eq!(
            session
                .authored_live_profile
                .as_ref()
                .unwrap()
                .game_mode
                .properties
                .len(),
            14
        );
        return;
    }
    let original = tempfile::tempdir().unwrap();
    let source = copy_bundle(original.path());
    let cache_name = "cache.simthing-scenario.json";
    let cache = original.path().join(cache_name);
    let ingest =
        ingest_clause_scenario_path(&source, &ClauseScenarioIngestOptions::default()).unwrap();
    save_clause_scenario_cache_to_path(&cache, &ingest).unwrap();
    let relocated = tempfile::tempdir().unwrap();
    copy_bundle(relocated.path());
    std::fs::copy(&cache, relocated.path().join(cache_name)).unwrap();
    let foreign_cwd = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "rehearsal_ingress_bundle_relocates_and_refuses_dependency_drift",
            "--nocapture",
        ])
        .env(
            "SIMTHING_INGRESS_CACHE_PROBE",
            relocated.path().join(cache_name),
        )
        .current_dir(foreign_cwd.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "relocated child failed: {} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let dependency = relocated.path().join("rehearsal_ingress_cache.base.json");
    let mut bytes = std::fs::read(&dependency).unwrap();
    bytes.push(b' ');
    std::fs::write(&dependency, bytes).unwrap();
    let error = ingest_clause_scenario_path(
        &relocated.path().join("rehearsal_ingress_cache.clause"),
        &ClauseScenarioIngestOptions::default(),
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("G4") && error.contains("dependencies.json"),
        "{error}"
    );
    assert!(
        load_studio_session_from_scenario_path(&relocated.path().join(cache_name), None).is_err()
    );
}

#[test]
fn rehearsal_ingress_source_refusals_retain_file_element_and_span() {
    let dir = tempfile::tempdir().unwrap();
    let source = copy_bundle(dir.path());
    let original = std::fs::read_to_string(&source).unwrap();
    for (declaration, element) in [
        ("location = bad { overlays = { modifier = { id = bad amount_add = 1 targets_property = \"cache_economy::bad\" unsupported_modifier = yes } } }", "unsupported_modifier"),
        ("recipe = unsupported_recipe { input = energy }", "recipe"),
    ] {
        let changed = original.replacen("scenario = rehearsal_ingress_cache {", &format!("scenario = rehearsal_ingress_cache {{ {declaration}"), 1);
        std::fs::write(&source, changed).unwrap();
        let error = ingest_clause_scenario_path(&source, &ClauseScenarioIngestOptions::default()).unwrap_err().to_string();
        assert!(error.contains("0088-INGRESS-FIDELITY-0") && error.contains("rehearsal_ingress_cache.clause") && error.contains(element) && error.contains("token"), "{error}");
    }
    std::fs::write(&source, original).unwrap();
    let ingest =
        ingest_clause_scenario_path(&source, &ClauseScenarioIngestOptions::default()).unwrap();
    assert!(load_studio_session_from_clause_ingest_result(&ingest, source, None).is_ok());
}
