//! SimThing Studio — Bevy presentation/authoring shell over MapGenerator producer output.
//!
//! ClauseThing/MapGenerator generates structural galaxy data; the editor breathes SimThing as a
//! render/UI metatable. Bevy transforms and visual z-height are render-only — never structural truth.

pub mod camera_control;
pub mod dialog;
pub mod generation;
pub mod hydration;
pub mod hyperlane_buckets;
pub mod panel_layout;
pub mod runtime_vertical_seed;
pub mod scenario_io;
pub mod scenario_projection;
pub mod scenario_runtime_saveload_ui;
pub mod selection;
pub mod session;
pub mod settings;
pub mod shape_params;
pub mod star_render;
pub mod starburst;
pub mod studio_admission_report;
pub mod studio_config;
pub mod studio_planet_child_location;
pub mod studio_scenario_document;
pub mod studio_scenario_load;
pub mod studio_structural_edit;
pub mod terran_pirate_skeleton;
pub mod view_model;

#[cfg(windows)]
pub mod app;

#[cfg(windows)]
pub use app::scenario_io::{
    load_scenario_manual_path_action, load_scenario_with_picker, open_native_scenario_load_picker,
    set_programmatic_scenario_path, ScenarioPickerActionResult,
};

#[cfg(windows)]
pub fn run() {
    app::run_studio();
}

#[cfg(not(windows))]
pub fn run() {
    eprintln!("SimThing Studio PR1 requires Windows.");
    std::process::exit(1);
}

pub use dialog::{StudioAction, WarningDialogModel};
pub use generation::{GenerationPreset, GenerationProfile, GenerationRunOutput};
pub use hydration::{
    generate_simthing_spec_scenario, heatmap_readiness_from_simthing_spec,
    hydrate_generation_into_studio_grid, rf_accumulator_readiness_from_simthing_spec,
    studio_projection_from_scenario_authority, studio_projection_from_simthing_spec,
    StudioHeatmapReadiness, StudioHeatmapReadinessKind, StudioHydrationBoundary,
    StudioHydrationError, StudioRfAccumulatorReadiness,
};
pub use runtime_vertical_seed::{
    runtime_vertical_seed_scenario_spec, RUNTIME_VERTICAL_SEED_PROVENANCE_SOURCE,
    RUNTIME_VERTICAL_SEED_SCENARIO_ID,
};
pub use scenario_io::{
    load_scenario_authority_from_path, load_studio_session_from_scenario_path,
    save_current_session_scenario_to_path, save_scenario_authority_to_path, ScenarioIoError,
    SCENARIO_FILE_SUFFIX, SCENARIO_TMP_SUFFIX,
};
pub use scenario_projection::{
    build_gpu_residency_readiness_from_scenario, build_gpu_structural_upload_packet_from_scenario,
    build_structural_projection, prove_gpu_buffer_residency_blocking,
    prove_gpu_structural_validation_blocking, to_structural_gpu_rows,
    StudioGpuBufferResidencyProof, StudioGpuLinkRow, StudioGpuLocationRow,
    StudioGpuResidencyReadiness, StudioGpuStructuralFrameRow, StudioGpuStructuralUploadError,
    StudioGpuStructuralUploadPacket, StudioGpuStructuralValidationProof, StudioLinkIndexRow,
    StudioLocationIndexRow, StudioStructuralProjection,
};
pub use scenario_runtime_saveload_ui::{
    build_studio_scenario_runtime_saveload_status_from_json_str,
    canonical_json_from_loaded_scenario_authority, refresh_runtime_saveload_status_from_session,
    reopen_candidate_scenario_for_studio, reopen_candidate_scenario_for_studio_session,
    save_candidate_scenario_for_studio_create_new,
    studio_scenario_runtime_saveload_non_authority_boundary, StudioReopenCandidateAdoptionResult,
    StudioReopenCandidateResult, StudioSaveCandidateResult,
    StudioScenarioRuntimeSaveLoadNonAuthorityBoundary, StudioScenarioRuntimeSaveLoadStatus,
};
pub use selection::{SelectedSystemDetails, StudioSelectionState};
pub use session::{StudioScenarioSummary, StudioSession, StudioSessionSource};
pub use settings::{EditorSettings, PersistedCameraState, WindowModeSetting};
pub use studio_admission_report::{
    build_studio_admission_summary_from_ingestion, build_studio_admission_summary_from_spec,
    studio_ingest_scenario_text_for_report, studio_scenario_authority_snapshot,
    StudioCompileReadinessSummary, StudioOwnerSiloSummary, StudioScenarioAdmissionSummary,
    StudioScenarioDeferralSummary, StudioScenarioErrorSummary,
};
pub use studio_config::{
    SimThingStudioConfig, StudioConfigError, StudioConfigLoadOutcome, STUDIO_CONFIG_FILE_NAME,
    STUDIO_CONFIG_SCHEMA_VERSION,
};
pub use studio_planet_child_location::{
    studio_apply_planet_child_location_command, StudioPlanetChildLocationError,
    StudioPlanetChildLocationOutcome,
};
pub use studio_scenario_document::{
    build_studio_scenario_document, build_studio_scenario_document_with_admission,
    load_canonical_studio_document_from_path, save_studio_scenario_with_document_roundtrip,
    studio_galaxy_map_gridcells_from_spec, StudioGalaxyMapView, StudioGameSessionView,
    StudioGridcellRole, StudioGridcellView, StudioOwnerView, StudioPlanetChildView,
    StudioPlanetNonGridChildView, StudioReceiverCellView, StudioScenarioAuthorityKind,
    StudioScenarioDocument, StudioScenarioDocumentError,
};
pub use studio_structural_edit::{
    studio_apply_structural_placement_command, StudioStructuralEditError,
    StudioStructuralEditOutcome,
};
pub use terran_pirate_skeleton::{
    terran_pirate_skeleton_dense_inputs, terran_pirate_skeleton_scenario_spec,
    TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE, TERRAN_PIRATE_SKELETON_SCENARIO_ID,
};
pub use view_model::{
    StudioGalaxyRenderMeta, StudioGalaxyViewModel, StudioHyperlaneView, StudioStarView,
};
