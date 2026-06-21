//! STUDIO-RUNTIME-SAVELOAD-STATUS-CACHE-0 — cached runtime save/load status refresh proofs.

use std::fs;
use std::path::PathBuf;

use simthing_mapeditor::{
    apply_runtime_saveload_status_to_cache,
    build_studio_scenario_runtime_saveload_status_from_json_str,
    clear_runtime_saveload_status_cache, load_studio_session_from_scenario_path,
    refresh_runtime_saveload_status_if_needed, reopen_candidate_scenario_for_studio_session,
    runtime_saveload_refresh_decision, save_candidate_scenario_for_studio_create_new,
    studio_scenario_runtime_saveload_non_authority_boundary, RuntimeSaveloadRefreshDecision,
    RuntimeSaveloadStatusCacheMut, StudioScenarioRuntimeSaveLoadStatus,
};
use simthing_spec::evaluate_planet_child_locations;
use tempfile::TempDir;

const OWNER_SILO_FIXTURE: &str = "owner_silo_disburse_down_scoped.simthing-scenario.json";

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_owner_silo_fixture_json() -> String {
    fs::read_to_string(corpus_path(OWNER_SILO_FIXTURE))
        .unwrap_or_else(|_| panic!("missing corpus {OWNER_SILO_FIXTURE}"))
}

fn status_fixture() -> StudioScenarioRuntimeSaveLoadStatus {
    build_studio_scenario_runtime_saveload_status_from_json_str(
        "owner_silo_corpus",
        &load_owner_silo_fixture_json(),
    )
    .expect("status")
}

fn cache_mut<'a>(
    status: &'a mut Option<StudioScenarioRuntimeSaveLoadStatus>,
    dirty: &'a mut bool,
    source_digest: &'a mut Option<u64>,
    refresh_in_progress: &'a mut bool,
    last_refresh_ms: &'a mut Option<u128>,
) -> RuntimeSaveloadStatusCacheMut<'a> {
    RuntimeSaveloadStatusCacheMut {
        status,
        dirty,
        source_digest,
        refresh_in_progress,
        last_refresh_ms,
    }
}

#[test]
fn studio_runtime_saveload_refresh_decision_uses_cache_when_clean() {
    assert_eq!(
        runtime_saveload_refresh_decision(true, false, false, Some(42), Some(42)),
        RuntimeSaveloadRefreshDecision::UseCache
    );
}

#[test]
fn studio_runtime_saveload_refresh_decision_refreshes_when_dirty() {
    assert_eq!(
        runtime_saveload_refresh_decision(true, true, false, Some(42), Some(42)),
        RuntimeSaveloadRefreshDecision::Refresh
    );
}

#[test]
fn studio_runtime_saveload_refresh_decision_refreshes_when_digest_changed() {
    assert_eq!(
        runtime_saveload_refresh_decision(true, false, false, Some(1), Some(2)),
        RuntimeSaveloadRefreshDecision::Refresh
    );
}

#[test]
fn studio_runtime_saveload_refresh_decision_refreshes_when_forced() {
    assert_eq!(
        runtime_saveload_refresh_decision(true, false, true, Some(42), Some(42)),
        RuntimeSaveloadRefreshDecision::Refresh
    );
}

#[test]
fn studio_runtime_saveload_refresh_decision_clears_without_session() {
    assert_eq!(
        runtime_saveload_refresh_decision(false, true, true, Some(42), Some(42)),
        RuntimeSaveloadRefreshDecision::Clear
    );
}

#[test]
fn studio_runtime_saveload_status_does_not_refresh_every_frame() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let mut status = None;
    let mut dirty = true;
    let mut source_digest = None;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = None;
    let scenario = &session.scenario_authority;

    assert!(refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(scenario),
        "owner_silo_corpus",
        false,
        None,
    ));
    let first_ms = last_refresh_ms.expect("timed first refresh");
    assert!(!dirty);

    assert!(!refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(scenario),
        "owner_silo_corpus",
        false,
        None,
    ));
    assert_eq!(last_refresh_ms, Some(first_ms));
}

#[test]
fn studio_runtime_saveload_status_refreshes_on_load_event() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let mut status = None;
    let mut dirty = true;
    let mut source_digest = None;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = None;

    assert!(refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "studio_loaded_session",
        false,
        None,
    ));
    let loaded = status.expect("status");
    assert!(loaded.stead_validation_ready);
    assert!(loaded.runtime_report_chain_ready);
    assert_eq!(source_digest, loaded.loaded_scenario_digest);
}

#[test]
fn studio_runtime_saveload_status_refreshes_on_manual_refresh() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let baseline = status_fixture();
    let mut status = Some(baseline.clone());
    let mut dirty = false;
    let mut source_digest = baseline.loaded_scenario_digest;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = Some(1);

    assert!(refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "owner_silo_corpus",
        true,
        None,
    ));
    assert!(last_refresh_ms.unwrap() > 0);
    assert_eq!(status.as_ref(), Some(&baseline));
}

#[test]
fn studio_runtime_saveload_status_refreshes_once_after_save_candidate() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let output_path = temp_dir.path().join("candidate.simthing-scenario.json");

    let mut status = None;
    let mut dirty = true;
    let mut source_digest = None;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = None;
    refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "owner_silo_corpus",
        false,
        None,
    );
    let before_ms = last_refresh_ms;

    let save_result =
        save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &output_path)
            .expect("save");
    assert!(save_result.saved);

    dirty = true;
    assert!(refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "owner_silo_corpus",
        false,
        None,
    ));
    assert_ne!(last_refresh_ms, before_ms);
    assert!(!refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "owner_silo_corpus",
        false,
        None,
    ));
}

#[test]
fn studio_runtime_saveload_status_refreshes_once_after_reopen_candidate_adoption() {
    let json = load_owner_silo_fixture_json();
    let temp_dir = TempDir::new().expect("temp");
    let candidate_path = temp_dir.path().join("candidate.simthing-scenario.json");
    save_candidate_scenario_for_studio_create_new("owner_silo_corpus", &json, &candidate_path)
        .expect("save");

    let adoption = reopen_candidate_scenario_for_studio_session(&candidate_path).expect("adopt");
    let adopted_status = adoption.status.expect("status");
    let session = adoption.session.expect("session");

    let mut status = None;
    let mut dirty = false;
    let mut source_digest = None;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = None;
    apply_runtime_saveload_status_to_cache(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        adopted_status.clone(),
        Some(0),
    );
    assert_eq!(status.as_ref(), Some(&adopted_status));
    assert!(!dirty);

    assert!(!refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "studio_reopened_candidate",
        false,
        None,
    ));
}

#[test]
fn studio_runtime_saveload_status_uses_cached_status_when_not_dirty() {
    let session = load_studio_session_from_scenario_path(&corpus_path(OWNER_SILO_FIXTURE), None)
        .expect("session");
    let baseline = status_fixture();
    let mut status = Some(baseline.clone());
    let mut dirty = false;
    let mut source_digest = baseline.loaded_scenario_digest;
    let mut refresh_in_progress = false;
    let mut last_refresh_ms = Some(99);

    assert!(!refresh_runtime_saveload_status_if_needed(
        cache_mut(
            &mut status,
            &mut dirty,
            &mut source_digest,
            &mut refresh_in_progress,
            &mut last_refresh_ms,
        ),
        Some(&session.scenario_authority),
        "owner_silo_corpus",
        false,
        None,
    ));
    assert_eq!(status.as_ref(), Some(&baseline));
    assert_eq!(last_refresh_ms, Some(99));

    clear_runtime_saveload_status_cache(cache_mut(
        &mut status,
        &mut dirty,
        &mut source_digest,
        &mut refresh_in_progress,
        &mut last_refresh_ms,
    ));
    assert!(status.is_none());
    assert!(!dirty);
}

#[test]
fn studio_runtime_saveload_status_preserves_non_authority_boundary() {
    let boundary = studio_scenario_runtime_saveload_non_authority_boundary();
    assert!(!boundary.ui_state_is_authority);
    assert!(!boundary.bevy_state_is_authority);
    assert!(!boundary.runtime_reports_are_authority);
    assert!(!boundary.gpu_buffers_are_authority);
    assert!(boundary.persistent_history_deferred);
    assert!(boundary.gpu_dispatch_deferred);
}

#[test]
fn studio_runtime_saveload_status_preserves_surface_gridcell_tier() {
    let json = load_owner_silo_fixture_json();
    let (spec, _) =
        simthing_spec::load_scenario_spec_from_json_str("owner_silo", &json).expect("load");
    assert!(evaluate_planet_child_locations(&spec).surface_gridcell_tier_present);
    let status = status_fixture();
    assert!(status.loaded_scenario_digest.is_some());
}
