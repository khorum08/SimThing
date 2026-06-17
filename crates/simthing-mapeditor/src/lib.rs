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
pub mod scenario_io;
pub mod scenario_projection;
pub mod selection;
pub mod session;
pub mod settings;
pub mod shape_params;
pub mod star_render;
pub mod starburst;
pub mod studio_config;
pub mod view_model;

#[cfg(windows)]
pub mod app;

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
pub use selection::{SelectedSystemDetails, StudioSelectionState};
pub use session::{StudioScenarioSummary, StudioSession, StudioSessionSource};
pub use settings::{EditorSettings, PersistedCameraState, WindowModeSetting};
pub use studio_config::{
    SimThingStudioConfig, StudioConfigError, StudioConfigLoadOutcome, STUDIO_CONFIG_FILE_NAME,
    STUDIO_CONFIG_SCHEMA_VERSION,
};
pub use view_model::{
    StudioGalaxyRenderMeta, StudioGalaxyViewModel, StudioHyperlaneView, StudioStarView,
};
