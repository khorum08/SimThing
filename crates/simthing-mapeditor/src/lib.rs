//! SimThing Studio — Bevy presentation/authoring shell over MapGenerator producer output.
//!
//! ClauseThing/MapGenerator generates structural galaxy data; the editor breathes SimThing as a
//! render/UI metatable. Bevy transforms and visual z-height are render-only — never structural truth.

pub mod camera_control;
pub mod clause_scenario_ingest;
pub mod clause_scenario_picker;
pub mod dialog;
pub mod falloff_metric;
pub mod falloff_ruler_overlay;
pub mod generation;
pub mod hydration;
pub mod hyperlane_buckets;
pub mod hyperlane_ribbon;
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
pub mod studio_aa_test_pattern;
pub mod studio_admission_report;
pub mod studio_antialiasing;
pub mod studio_config;
pub mod studio_frame_phase_gpu_telemetry;
pub mod studio_fleet_presence;
pub mod studio_frosted_glass;
pub mod studio_performance_telemetry;
pub mod studio_planet_child_location;
pub mod studio_render_loop_dirty_gate;
pub mod studio_scenario_document;
pub mod studio_scenario_library_ui;
pub mod studio_scenario_load;
pub mod studio_screenshot;
pub mod studio_faction_nameplates;
pub mod studio_live_observe;
pub mod studio_live_session_bridge;
pub mod studio_sim_clock;
pub mod studio_sim_clock_ui;
pub mod studio_structural_edit;
pub mod studio_typeface_shell;
pub mod terran_pirate_skeleton;
pub mod tp_base_disc;
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

pub use dialog::{SettingsDialogModel, StudioAction, TelemetryDialogModel, WarningDialogModel};
pub use falloff_metric::{
    compute_map_radius_falloff_context, map_radius_progress, map_radius_progress_percent,
    origin_source_label, plateau_falloff_t, plateau_falloff_t_percent, plateau_interpolate,
    world_position_map_progress_percent, MapPlaneBounds, MapViewOriginSource,
    StudioMapRadiusFalloffContext, FALLOFF_MODE_CAMERA_DISTANCE, FALLOFF_MODE_MAP_RADIUS,
    FALLOFF_MODE_VISUAL_HORIZON,
};
pub use clause_scenario_ingest::{
    ingest_clause_scenario_bytes, ingest_clause_scenario_path,
    load_clause_studio_session_from_path, load_studio_session_from_clause_ingest_result,
    save_clause_scenario_authority_to_path, ClauseScenarioIngestError, ClauseScenarioIngestOptions,
    ClauseScenarioIngestResult, ClauseScenarioSourceResolver,
};
pub use clause_scenario_picker::{
    clause_picker_menu_label, default_clause_picker_start_directory, format_clause_picker_error,
    open_clause_scenario_with_picker, parse_clause_resolver_entries, run_clause_picker_action,
    run_clause_picker_ingest_then_session, validate_clause_path, ClauseFilePicker,
    ClausePickerActionResult, ClausePickerSelection, FakeClauseFilePicker, NativeClauseFilePicker,
    OPEN_CLAUSE_SCENARIO_ACTION_LABEL,
};
pub use falloff_ruler_overlay::{draw_falloff_ruler_overlay, FalloffRulerOverlayParams};
pub use generation::{GenerationPreset, GenerationProfile, GenerationRunOutput};
// Re-export projection mode from clausething for callers.
pub use simthing_clausething::{
    ClauseScenarioProjectionMode, ClauseScenarioProjectionReport,
};
pub use hydration::{
    generate_simthing_spec_scenario, heatmap_readiness_from_simthing_spec,
    hydrate_generation_into_studio_grid, rf_accumulator_readiness_from_simthing_spec,
    studio_projection_from_scenario_authority, studio_projection_from_simthing_spec,
    StudioHeatmapReadiness, StudioHeatmapReadinessKind, StudioHydrationBoundary,
    StudioHydrationError, StudioRfAccumulatorReadiness,
};
pub use hyperlane_ribbon::{
    compute_camera_facing_width_dir, count_non_finite_vertex_positions, hyperlane_mesh_is_valid,
    hyperlane_rebuild_is_valid, hyperlane_ribbon_width_dir, is_valid_width_dir, HyperlaneMeshStats,
    HyperlaneRibbonBasis, HyperlaneRibbonCamera, HyperlaneWidthDirOutcome,
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
    apply_runtime_saveload_status_to_cache,
    build_studio_scenario_runtime_saveload_status_from_json_str,
    canonical_json_from_loaded_scenario_authority, clear_runtime_saveload_status_cache,
    refresh_runtime_saveload_status_from_session, refresh_runtime_saveload_status_if_needed,
    reopen_candidate_scenario_for_studio, reopen_candidate_scenario_for_studio_session,
    runtime_saveload_refresh_decision, save_candidate_scenario_for_studio_create_new,
    studio_scenario_runtime_saveload_non_authority_boundary, RuntimeSaveloadRefreshDecision,
    RuntimeSaveloadStatusCacheMut, StudioReopenCandidateAdoptionResult,
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
pub use studio_frame_phase_gpu_telemetry::{
    apply_diagnostic_minimal_render, capture_normal_render_snapshot, format_present_mode_label,
    frame_phase_settings_lines, gpu_context_settings_lines, instrumented_render_loop_ms,
    performance_capture_steps_lines, read_frame_time_ms_from_diagnostics,
    restore_normal_render_from_snapshot, studio_build_profile_label, unexplained_frame_ms,
    vram_tracked_asset_lines, PerformanceDiagnosticFlags, PerformanceNormalRenderSnapshot,
    DIAGNOSTIC_MINIMAL_RENDER_BUTTON, RESTORE_NORMAL_RENDER_BUTTON,
};
pub use studio_fleet_presence::{
    studio_fleet_presence_map_from_session, studio_fleet_presence_map_from_snapshot,
    StudioFleetPresenceMap,
};
pub use studio_frosted_glass::{
    FrostedGlassFrameTelemetry, FrostedGlassPanelRegistry, FrostedGlassRenderPlan,
    FrostedGlassSettings, StudioFrostedGlassPlugin, FROSTED_GLASS_BLUR_PASS_COUNT,
    FROSTED_GLASS_DOWNSAMPLE_FACTOR, FROSTED_GLASS_MAX_PANELS,
    FROSTED_GLASS_SHARED_TARGET_COUNT,
};
pub use studio_performance_telemetry::{
    bytes_to_vram_mb, estimate_image_vram_bytes, estimate_mesh_vram_bytes,
    estimate_studio_allocated_vram_bytes, format_fps_label, format_vram_mb_label,
    hyperlane_debug_lines, performance_settings_section_lines, read_fps_from_diagnostics,
    render_loop_diagnostics_lines, StudioPerformanceTelemetry,
};
pub use studio_planet_child_location::{
    studio_apply_planet_child_location_command, StudioPlanetChildLocationError,
    StudioPlanetChildLocationOutcome,
};
pub use studio_render_loop_dirty_gate::{
    billboard_should_sync, hyperlane_basis_mismatch_angle_deg,
    hyperlane_basis_mismatch_exceeds_epsilon, hyperlane_camera_basis_from_transform,
    hyperlane_render_settings_key, hyperlane_render_should_rebuild,
    picking_projection_should_rebuild, quantize_billboard_camera_key,
    quantize_hyperlane_camera_key, quantize_picking_projection_key, quantize_star_depth_percent,
    render_loop_telemetry_record_timing, star_falloff_settings_key,
    star_visual_per_star_should_write, star_visuals_should_sync, BillboardCameraKey,
    BillboardSyncCacheState, HyperlaneCameraBasis, HyperlaneCameraKey, HyperlaneRenderCacheState,
    HyperlaneRenderSettingsKey, PickingProjectionCacheState, PickingProjectionKey,
    StarFalloffSettingsKey, StarVisualAppliedKey, StarVisualSyncCacheState, StarVisualSyncKey,
    StudioRenderLoopCaches, HYPERLANE_BASIS_MISMATCH_REBUILD_EPSILON_DEG,
};
pub use studio_scenario_document::{
    build_studio_scenario_document, build_studio_scenario_document_with_admission,
    load_canonical_studio_document_from_path, save_studio_scenario_with_document_roundtrip,
    studio_galaxy_map_gridcells_from_spec, StudioGalaxyMapView, StudioGameSessionView,
    StudioGridcellRole, StudioGridcellView, StudioOwnerView, StudioPlanetChildView,
    StudioPlanetNonGridChildView, StudioReceiverCellView, StudioScenarioAuthorityKind,
    StudioScenarioDocument, StudioScenarioDocumentError,
};
pub use studio_scenario_library_ui::{
    build_blank_studio_scenario_spec, build_studio_scenario_telemetry_readout,
    create_blank_studio_session, StudioScenarioLibraryCreateError, StudioScenarioLibraryModel,
    StudioScenarioLibraryTab, StudioScenarioTelemetryReadout,
    STUDIO_SCENARIO_LIBRARY_CREATE_PROVENANCE, STUDIO_SCENARIO_LIBRARY_DEFAULT_CREATE_ID,
};
pub use studio_screenshot::{next_screenshot_filename, parse_screenshot_index};
pub use studio_faction_nameplates::{
    fallback_simthing_nameplate_id, nameplate_rgba_from_color_rgb, owner_color_rgb_map_from_authority,
    owned_star_highlight_system_ids, selected_owner_id_for_system, star_nameplate_presentations,
    star_nameplate_rgba_for_gridcell, star_nameplate_rgba_for_placement, star_owner_id_by_system_id,
    star_owner_id_for_placement, star_ownership_presentations, star_visual_selected_for_owned_set,
    StarOwnershipPresentation, NEUTRAL_NAMEPLATE_RGBA,
};
pub use studio_live_observe::{
    build_studio_live_observation_readout, observe_module_source_forbids_workshop_residue,
    StudioLiveObservationReadout, StudioLiveObservationSourceKind,
};
pub use studio_live_session_bridge::{
    apply_live_bridge_reset_before_tick, bridge_module_source_forbids_workshop_residue,
    driver_scenario_from_authority, request_live_bridge_reset_after_session_replacement,
    revalidate_authority_stead, studio_summary_identity_eq, BridgeOpenIdentity,
    StudioLiveSessionBridge, StudioLiveSessionBridgeError, StudioLiveSessionBridgeReadout,
    StudioLiveSessionBridgeStatus,
};
pub use studio_sim_clock::{
    StudioSimClock, StudioSimClockError, StudioSimClockRate, STUDIO_SIM_CLOCK_DEFAULT_MAX_TPS,
    STUDIO_SIM_CLOCK_RATE_RATIO_TOLERANCE,
};
pub use studio_sim_clock_ui::{
    rate_label, StudioSimClockReadout, StudioSimClockTransport, StudioSimClockTransportCommand,
};
pub use studio_structural_edit::{
    studio_apply_structural_placement_command, StudioStructuralEditError,
    StudioStructuralEditOutcome,
};
pub use studio_typeface_shell::{
    mount_studio_typeface_plugins, typeface_fixture_font_bytes, StudioTypefaceShellMounted,
    StudioTypefaceShellPlugin,
};
pub use terran_pirate_skeleton::{
    terran_pirate_skeleton_dense_inputs, terran_pirate_skeleton_scenario_spec,
    TERRAN_PIRATE_SKELETON_PROVENANCE_SOURCE, TERRAN_PIRATE_SKELETON_SCENARIO_ID,
};
pub use tp_base_disc::{
    canonicalize_tp_base_disc_scenario, generate_tp_base_disc_artifact,
    generate_tp_base_disc_artifact_from_recorded_metadata, stamp_tp_base_disc_metadata,
    tp_base_disc_generation_profile, tp_base_disc_star_name_corpus_source, TpBaseDiscArtifact,
    TpBaseDiscError, TP_BASE_DISC_LATTICE_EDGE, TP_BASE_DISC_NAME_ASSIGNMENT_MODE,
    TP_BASE_DISC_NAME_CORPUS_SOURCE, TP_BASE_DISC_RUNG_ID, TP_BASE_DISC_SCENARIO_ID,
    TP_BASE_DISC_SEED, TP_BASE_DISC_STARS,
};
pub use view_model::{
    StudioGalaxyRenderMeta, StudioGalaxyViewModel, StudioHyperlaneView, StudioStarView,
};
