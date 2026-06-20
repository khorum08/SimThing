//! Authored SimThing specification layer.
//!
//! This crate compiles external RON-facing game data into native SimThing
//! runtime artifacts. It owns authored schemas, validation, diagnostics,
//! logical keys, compile-time conversion, runtime definition types, Script IR,
//! event/trigger/effect templates, boundary handlers, and impact preview.
//!
//! It does **not** execute the simulation, own GPU state, or orchestrate the
//! day-boundary protocol. The driver (`simthing-driver`) installs compiled
//! artifacts into `SpecSessionState` and invokes boundary handlers through
//! a generic sim-side hook after GPU value readback.
//!
//! ## Scope (PRs 1–11)
//!
//! - **Authoring structs** (`spec::*`): properties, overlays, capability trees,
//!   events, triggers, effects, Script IR.
//! - **Compilers** (`compile::*`): `compile_property`, `compile_overlay`,
//!   `CapabilityTreeBuilder`, `compile_event`, trigger/effect compilers.
//! - **Runtime artifacts** (`runtime::*`): `CapabilityTreeDefinition`,
//!   capability/session state types, `ScriptedEventDefinition`.
//! - **Boundary handlers** (`boundary::*`): capability unlock / player
//!   selection, scripted-event predicate + threshold dispatch (called by the
//!   driver hook — not embedded in `simthing-sim::BoundaryProtocol`).
//! - **Preview** (`preview::*`): `preview_capability_effect`.
//! - **RON loaders**, validation, diagnostics, logical keys.
//!
//! ## Out of scope / deferred
//!
//! - RON-driven session open from `GameModeSpec` (manual `install_spec_state`
//!   today — see progress log § Open work O1).
//! - Replay serialization of spec runtime state (O2).
//! - B2 append-only integration for external capability/scripted threshold
//!   registrations on growth boundaries (helpers exist; wiring deferred).
//! - EML backend, Studio GUI, full scenario RON expansion.
//!
//! ## Crate boundary
//!
//! Production code depends on `simthing-core` and `simthing-feeder` only.
//! Integration tests may use `simthing-gpu` / `simthing-sim` as dev-dependencies.
//! Fired GPU threshold events are resolved by the caller via
//! `ThresholdRegistry::extract_capability_unlocks` / `extract_scripted_event_triggers`
//! before reaching spec boundary handlers.

pub mod boundary;
pub mod compile;
pub mod designer_admission;
pub mod diagnostics;
pub mod error;
pub mod keys;
pub mod metadata;
pub mod preview;
pub mod ron;
pub mod runtime;
pub mod spec;
pub mod validate;
pub mod version;

pub use boundary::{
    CapabilityBoundaryContext, CapabilityTreeBoundaryHandler, CapabilityTreeError,
    ScriptedEventBoundaryContext, ScriptedEventBoundaryHandler, ScriptedEventDiagnostic,
    ScriptedEventDiagnosticKind,
};
pub use compile::jit_kernel_production_registry_shell::{
    ProductionKernelRegistryShell, ProductionKernelRegistryShellConfig,
    RegisteredProductionCandidate,
};
pub use compile::{
    admit_region_field_formula_class, compile_effect, compile_eml_gadget, compile_eml_gadget_stack,
    compile_event, compile_first_slice_scenario_preview,
    compile_game_mode_resource_economy_authoring_preview, compile_overlay, compile_property,
    compile_region_field_frame_preview, compile_region_field_preview,
    compile_region_field_stencil_config, compile_resource_economy,
    compile_resource_economy_authoring_preview, compile_resource_flow_admission,
    compile_stress_compose_preview, compile_trigger, compile_w_impedance_compose_preview,
    estimate_region_field_budget, eval_eml_postfix, exact_sqrt_f_artifact_descriptor,
    field_policy_act0_numeric_proposals_kernel_descriptor,
    field_policy_act1_phase_e_proposal_consumer_kernel_descriptor,
    field_policy_act2_proposal_admission_records_kernel_descriptor,
    field_policy_act3_economic_fixture_records_kernel_descriptor,
    field_policy_event0_compaction_kernel_descriptor,
    field_policy_event1_code_bucketing_kernel_descriptor,
    field_policy_event2_bucket_reductions_kernel_descriptor,
    field_policy_obs0_overlay_score_kernel_descriptor,
    field_policy_obs2_multilayer_overlay_score_kernel_descriptor,
    field_policy_obs3_multilayer_fixed_score_kernel_descriptor,
    field_policy_obs4_threshold_event_kernel_descriptor,
    field_policy_pipe0_observer_event_pipeline_kernel_descriptor, fnv1a64_hex,
    is_exact_mag2_fixed_descriptor, is_exact_mag_f_from_mag2_descriptor,
    is_exact_sqrt_f_descriptor, is_field_policy_act0_numeric_proposals_descriptor,
    is_field_policy_act1_phase_e_proposal_consumer_descriptor,
    is_field_policy_act2_proposal_admission_records_descriptor,
    is_field_policy_act3_economic_fixture_records_descriptor,
    is_field_policy_event0_compaction_descriptor, is_field_policy_event1_code_bucketing_descriptor,
    is_field_policy_event2_bucket_reductions_descriptor,
    is_field_policy_obs0_overlay_score_descriptor,
    is_field_policy_obs2_multilayer_overlay_score_descriptor,
    is_field_policy_obs3_multilayer_fixed_score_descriptor,
    is_field_policy_obs4_threshold_event_descriptor,
    is_field_policy_pipe0_observer_event_pipeline_descriptor, is_mag_f_dxdy_probe_descriptor,
    landed_jit_kernel_descriptors, mag2_fixed_exact_kernel_descriptor,
    mag_f_from_dxdy_probe_kernel_descriptor, mag_f_from_exact_mag2_kernel_descriptor,
    oracle_acceleration, oracle_bounded_feedback, oracle_decay, oracle_ema, oracle_field_sampler,
    oracle_hysteresis, oracle_soft_step, oracle_velocity_monitor, oracle_weighted_accumulator,
    preview_kernel_graph_cohorts, preview_kernel_graph_identity, preview_kernel_registry_manifest,
    preview_production_candidate_registry_entry, region_field_isolation_multiplier,
    reject_unknown_gadget_kind, sqrt_f_exact_kernel_descriptor, validate_exact_kernel_inputs,
    validate_exact_pre_sqrt_contract, validate_exact_sqrt_artifact_admission,
    validate_exact_sqrt_artifact_binding, validate_field_policy_act0_numeric_proposals_contract,
    validate_field_policy_act1_phase_e_proposal_consumer_contract,
    validate_field_policy_act2_proposal_admission_records_contract,
    validate_field_policy_act3_economic_fixture_records_contract,
    validate_field_policy_event0_compaction_contract,
    validate_field_policy_event1_code_bucketing_contract,
    validate_field_policy_event2_bucket_reductions_contract,
    validate_field_policy_obs0_overlay_score_contract,
    validate_field_policy_obs2_multilayer_overlay_score_contract,
    validate_field_policy_obs3_multilayer_fixed_score_contract,
    validate_field_policy_obs4_threshold_event_contract,
    validate_field_policy_pipe0_observer_event_pipeline_contract,
    validate_kernel_descriptor_admission, validate_kernel_graph_admission,
    validate_kernel_registry_manifest_preview, validate_mag2_source_contract,
    validate_production_candidate_preview_entry, validate_region_field_frame_gradient_sinks,
    validate_score_authority_contract, CapabilityTreeBuildOutput, CapabilityTreeBuilder,
    CompileContext, CompiledArenaAdmission, CompiledCouplingAdmission, CompiledCouplingDelay,
    CompiledEmissionFormula, CompiledEmitOnThreshold, CompiledEmlGadget, CompiledEmlGadgetStack,
    CompiledFieldCadence, CompiledFirstSliceCommitmentDirection,
    CompiledFirstSliceCommitmentThreshold, CompiledFirstSliceScenarioPreview, CompiledGradientAxis,
    CompiledRegionFieldBoundaryMode, CompiledRegionFieldMaskMode, CompiledRegionFieldOperator,
    CompiledRegionFieldPreview, CompiledRegionFieldSourcePolicy, CompiledRegionFieldStencilSpec,
    CompiledResourceEconomy, CompiledResourceEmission, CompiledResourceFlowAdmission,
    CompiledResourceRecipe, CompiledResourceRecipeInput, CompiledResourceTransfer,
    CompiledStressCompose, CompiledStressComposeProfile, CompiledWImpedanceCompose,
    CompiledWImpedanceComposeProfile, EmlGadgetCompileOptions, EmlGadgetCompositionPlan,
    EmlGadgetDiagnostic, EmlGadgetKind, EmlGadgetPreviewReport, EmlGadgetRegistry,
    EventAuthorityContract, EventBucketReductionInputAuthority, EventBucketReductionOrderAuthority,
    EventCodeBucketMembershipAuthority, EventCodeBucketOrderAuthority,
    EventCompactionMembershipAuthority, EventCompactionOrderAuthority, ExactPreSqrtInputContract,
    ExactSqrtArtifactDescriptor, ExactSqrtAuthorityClass, KernelDescriptorSpec,
    KernelGraphCohortPreview, KernelGraphCohortPreviewSet, KernelGraphEdgeSpec,
    KernelGraphIdentity, KernelGraphRequestSpec, KernelGraphSpec, KernelLane, KernelOutputSpec,
    KernelRegistryEntryPreview, KernelRegistryLane, KernelRegistryManifestPreview,
    Mag2SourceContract, NativeMathClass, NumericProposalMembershipAuthority,
    NumericProposalOrderAuthority, OutputAuthority, PhaseEEconomicFixtureRecordAuthority,
    PhaseEFixtureProposalAdmissionAuthority, PhaseEProposalConsumerInputAuthority,
    PhaseEProposalSummaryOrderAuthority, RecipePreview, RegionFieldBudgetError,
    RegionFieldBudgetEstimate, RegionFieldBudgetSpec, RegionFieldIsolationPolicyEstimate,
    ResourceBindingPreview, ResourceEconomyAuthoringPreview, ResourceEconomyDiagnostic,
    ResourceEconomyExpansionReport, ResourceEconomyPreviewReport, ResourceFlowDiagnostic,
    ResourceFlowExpansionReport, ScoreAuthorityContract, StaticPropertyNetPreview,
    ThresholdAuthorityContract, ThresholdEmitPreview, TransferPreview,
    ADMITTED_REGION_FIELD_FORMULA_CLASSES, DEFERRED_GADGET_KINDS, FIELD_POLICY_ACT0_CODE_COUNT,
    FIELD_POLICY_ACT0_DESCRIPTOR_ID, FIELD_POLICY_ACT0_LABEL,
    FIELD_POLICY_ACT1_ADMITTED_TABLE_SIZE, FIELD_POLICY_ACT1_DESCRIPTOR_ID,
    FIELD_POLICY_ACT1_LABEL, FIELD_POLICY_ACT2_ADMISSION_RECORD_STRIDE,
    FIELD_POLICY_ACT2_DESCRIPTOR_ID, FIELD_POLICY_ACT2_LABEL, FIELD_POLICY_ACT3_DESCRIPTOR_ID,
    FIELD_POLICY_ACT3_FIXTURE_RECORD_STRIDE, FIELD_POLICY_ACT3_LABEL,
    FIELD_POLICY_EVENT0_DESCRIPTOR_ID, FIELD_POLICY_EVENT0_LABEL, FIELD_POLICY_EVENT1_CODE_COUNT,
    FIELD_POLICY_EVENT1_DESCRIPTOR_ID, FIELD_POLICY_EVENT1_LABEL, FIELD_POLICY_EVENT2_CODE_COUNT,
    FIELD_POLICY_EVENT2_DESCRIPTOR_ID, FIELD_POLICY_EVENT2_LABEL, FIELD_POLICY_OBS0_DESCRIPTOR_ID,
    FIELD_POLICY_OBS0_LABEL, FIELD_POLICY_OBS2_DESCRIPTOR_ID, FIELD_POLICY_OBS2_LABEL,
    FIELD_POLICY_OBS2_LAYER_COUNT, FIELD_POLICY_OBS3_DESCRIPTOR_ID, FIELD_POLICY_OBS3_LABEL,
    FIELD_POLICY_OBS3_LAYER_COUNT, FIELD_POLICY_OBS4_DESCRIPTOR_ID, FIELD_POLICY_OBS4_LABEL,
    FIELD_POLICY_OBS4_LAYER_COUNT, FIELD_POLICY_PIPE0_DESCRIPTOR_ID, FIELD_POLICY_PIPE0_LABEL,
    FIELD_POLICY_PIPE0_LAYER_COUNT, FIRST_SLICE_FIELD_URGENCY_COL, MAG2_FIXED_DESCRIPTOR_ID,
    MAG2_FIXED_LABEL, MAG2_Q16_COMPONENT_MAX, MAG2_Q16_FRAC_BITS, MAG2_Q16_SCALE,
    MAG2_Q16_SCALE_SQ, MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID, MAG_F_FROM_DXDY_PROBE_LABEL,
    MAG_F_FROM_MAG2_DESCRIPTOR_ID, MAG_F_FROM_MAG2_LABEL, REGION_FIELD_DEFAULT_HORIZON_CAP,
    REGION_FIELD_EXTENDED_HORIZON_CAP, REGION_FIELD_EXTENDED_MAX_GRID, REGION_FIELD_MAX_CELL_COUNT,
    REGION_FIELD_STANDARD_MAX_GRID, SATURATING_FLUX_CHI_CFL_MAX, SQRT_F_ARTIFACT_HASH,
    SQRT_F_ARTIFACT_PATH, SQRT_F_DESCRIPTOR_ID, SQRT_F_DOMAIN, SQRT_F_ENTRYPOINT,
    SQRT_F_IO_CONTRACT, SQRT_F_PROOF_REPORT,
};
pub use designer_admission::{
    accepted_frontier_v2_artifact_target_ids,
    accepted_frontier_v2_artifact_targets,
    admit_clause_spec_frontier_v2_scenario,
    admit_mobility_scenario0_packet,
    admit_v7_8_line_scenario_pack,
    all_designer_admission_diagnostic_codes,
    audit_mobility_owner_band_budget,
    audit_mobility_owner_band_budget_with_ceiling,
    compose_mobility_runtime0,
    designer_admission_diagnostic,
    designer_admission_diagnostic_for_rejection,
    evaluate_designer_admission_request,
    mobility_alloc0_layout_checksum_cpu,
    mobility_alloc0_layout_checksum_gpu_proxy,
    mobility_audit0_family_budgets,
    mobility_audit0_packet_matches_accepted_constants,
    mobility_audit0_required_orderband_depth,
    mobility_econ0_layout_checksum_cpu,
    mobility_econ0_layout_checksum_gpu_proxy,
    mobility_idroute0_layout_checksum_cpu,
    mobility_idroute0_layout_checksum_gpu_proxy,
    mobility_owner0_layout_checksum_cpu,
    mobility_owner0_layout_checksum_gpu_proxy,
    mobility_reenroll0_layout_checksum_cpu,
    mobility_reenroll0_layout_checksum_gpu_proxy,
    mobility_scenario0_packet,
    plan_mobility_alloc0,
    plan_mobility_econ0,
    plan_mobility_idroute0,
    plan_mobility_owner0,
    plan_mobility_reenroll0,
    preview_accepted_artifact_targets,
    preview_designer_admission_preflight,
    resolve_frontier_artifact_target_id,
    run_mobility_runtime1a_production_fixture,
    v7_8_met_consumer_scenario_pack,
    AcceptedFrontierArtifactTarget,
    // C-2 atlas admission
    AtlasAdmissionDecision,
    AtlasAdmissionProfile,
    AtlasAdmissionSpec,
    AtlasIsolationAdmissionMode,
    ClauseSpecArtifactTargets,
    ClauseSpecFaction,
    ClauseSpecFrontierV2Admission,
    ClauseSpecFrontierV2LoweringSummary,
    ClauseSpecFrontierV2Scenario,
    ClauseSpecGrid,
    ClauseSpecMapping,
    ClauseSpecMovementFeedback,
    ClauseSpecMovementFeedbackMode,
    ClauseSpecResourceFlow,
    ClauseSpecResourceFlowRoute,
    ClauseSpecStructuralFeedback,
    ClauseSpecStructuralFeedbackMode,
    DesignerAdmissionDiagnostic,
    DesignerAdmissionDiagnosticCode,
    DesignerAdmissionPreflightManifest,
    DesignerAdmissionPreflightReport,
    DesignerAdmissionPreviewReport,
    DesignerAdmissionRejectionKind,
    DesignerAdmissionRequest,
    DesignerFacingGuardrailClass,
    DirectedDisburse,
    FieldPolicyLadderStage,
    IdentityLane,
    MobilityAlloc0Assignment,
    MobilityAlloc0BlockSpec,
    MobilityAlloc0BoundaryEvent,
    MobilityAlloc0BoundaryEventKind,
    MobilityAlloc0ForbiddenPathRequests,
    MobilityAlloc0LiveSlice,
    MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput,
    MobilityAlloc0PlanReport,
    MobilityAllocationBounds,
    MobilityAudit0CirculationFamily,
    MobilityAudit0FamilyBudget,
    MobilityAudit0Report,
    MobilityAudit0ScenarioConstants,
    MobilityAudit0Verdict,
    MobilityBlockadeSemantics,
    MobilityEcon0DownDisburse,
    MobilityEcon0ForbiddenPathRequests,
    MobilityEcon0LocalCellRecord,
    MobilityEcon0PlanInput,
    MobilityEcon0PlanReport,
    MobilityEcon0SessionAggregate,
    MobilityEcon0SessionResourceKey,
    MobilityIdentityBoundary,
    MobilityIdentityChannelBudget,
    MobilityIdroute0ForbiddenPathRequests,
    MobilityIdroute0LocalRecord,
    MobilityIdroute0PlanInput,
    MobilityIdroute0PlanReport,
    MobilityOwner0AppliedOverlay,
    MobilityOwner0ColumnKind,
    MobilityOwner0ColumnValue,
    MobilityOwner0FissionResult,
    MobilityOwner0ForbiddenPathRequests,
    MobilityOwner0GenerationResync,
    MobilityOwner0LocalRecord,
    MobilityOwner0Overlay,
    MobilityOwner0OwnerChange,
    MobilityOwner0PlanInput,
    MobilityOwner0PlanReport,
    MobilityOwnerColumn,
    MobilityOwnerRelationDiscipline,
    MobilityOwnerRelationKind,
    MobilityQuantityClasses,
    MobilityReenroll0CommittedMove,
    MobilityReenroll0ForbiddenPathRequests,
    MobilityReenroll0Move,
    MobilityReenroll0PlanInput,
    MobilityReenroll0PlanReport,
    MobilityReenroll0RegistryState,
    MobilityRoutingMode,
    MobilityRoutingPolicy,
    MobilityRuntime0CompositionInput,
    MobilityRuntime0CompositionReport,
    MobilityRuntime0ForbiddenPathRequests,
    MobilityRuntime0HarnessConfig,
    MobilityRuntime1aFixtureGate,
    MobilityRuntime1aForbiddenPathRequests,
    MobilityRuntime1aProductionFixtureInput,
    MobilityRuntime1aProductionFixtureReport,
    MobilityRuntime1aSimSessionSurface,
    MobilityScenario0Admission,
    MobilityScenario0GuardrailRequests,
    MobilityScenario0Packet,
    MobilityScenario0ParameterSummary,
    MobilityScenario0Status,
    MobilitySoakProfile,
    MobilitySupplyScope,
    MobilityTheaterScale,
    MobilityTheaterShape,
    PerIdentitySum,
    V78AtlasVramBudget,
    V78HardCurrencyContentionOrderingClaim,
    V78LineGateStatus,
    V78LineScenario,
    V78LineScenarioClaim,
    V78LineScenarioPack,
    V78LineScenarioPackAdmission,
    V78LineScenarioStatusRecord,
    V78MultiTheaterAtlasMappingClaim,
    V78NamedConsumerScenario,
    V78NestedResourceFlowDepthFanoutClaim,
    V78PromotedLine,
    CLAUSE_SPEC_FRONTIER_V2_GRID_CAP,
    CLAUSE_SPEC_FRONTIER_V2_MIN_TICKS,
    CLAUSE_SPEC_FRONTIER_V2_PROFILE,
    MOBILITY_ALLOC0_ID,
    MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH,
    MOBILITY_AUDIT0_ID,
    MOBILITY_AUDIT0_NARROWING_CEILING,
    MOBILITY_ECON0_ID,
    MOBILITY_IDROUTE0_ID,
    MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH,
    MOBILITY_OWNER0_ID,
    MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH,
    MOBILITY_REENROLL0_ID,
    MOBILITY_RUNTIME0_ID,
    MOBILITY_RUNTIME0_ORDER,
    MOBILITY_RUNTIME1A_ID,
    MOBILITY_RUNTIME1A_NAMED_GATE,
    MOBILITY_RUNTIME1A_RUNTIME_FIXTURE_GATE,
    MOBILITY_SCENARIO0_ENTITY_TARGET,
    MOBILITY_SCENARIO0_ID,
    V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
    V78_MET_SCENARIO_PACK_ID,
};
pub use diagnostics::{DiagnosticSeverity, SpecDiagnostic, SpecDiagnostics, SpecResult};
pub use error::SpecError;
pub use keys::{CapabilityEffectKey, CapabilityEntryKey, CapabilityTreeKey, CategoryKey};
pub use metadata::DisplayMeta;
pub use preview::{
    preview_capability_effect, CapabilityPreviewDelta, CapabilityPreviewInput,
    CapabilityPreviewOverlayBreakdown, CapabilityPreviewReport,
};
pub use ron::{
    deserialize_capability_tree_ron, deserialize_clause_spec_frontier_v2_scenario_ron,
    deserialize_designer_admission_preflight_manifest_ron, deserialize_eml_gadget_stack_ron,
    deserialize_first_slice_scenario_ron, deserialize_game_mode_ron,
    deserialize_mobility_scenario0_packet_ron, deserialize_region_field_ron,
    deserialize_v7_8_line_scenario_pack_ron, serialize_clause_spec_frontier_v2_scenario_ron,
    serialize_designer_admission_preflight_manifest_ron, serialize_mobility_scenario0_packet_ron,
    serialize_v7_8_line_scenario_pack_ron,
};
pub use runtime::{
    CapabilityCategoryDefinition, CapabilityDefinition, CapabilityPrereq, CapabilityTreeDefinition,
    CapabilityTreeDefinitionId, CapabilityTreeDiagnostic, CapabilityTreeInstance,
    CapabilityTreeNotification, CapabilityTreeState, CapabilityUnlockRegistration, CompiledEffect,
    CompiledThresholdTrigger, CompiledTrigger, ScriptedEventDefinition, ScriptedEventDefinitionId,
    ScriptedEventInstance, ScriptedEventInstanceKey,
};
pub use spec::capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
    CapabilitySpec, CapabilityTreeSpec, EffectTarget, MaxActivePolicy, ReplacementPolicy,
};
pub use spec::domain_pack::DomainPackSpec;
pub use spec::effect::EffectSpec;
pub use spec::eml_gadget::{EmlGadgetInstanceSpec, EmlGadgetStackSpec};
pub use spec::event::{CooldownSpec, EventKey, EventPriority, EventSpec};
pub use spec::first_slice_scenario::FirstSliceScenarioSpec;
pub use spec::game_mode::GameModeSpec;
pub use spec::install_target::InstallTargetSpec;
pub use spec::loaded_scenario_recursive_rf_runtime::{
    evaluate_loaded_scenario_recursive_rf_runtime_from_json_str,
    prove_loaded_scenario_recursive_rf_runtime_preserves_authority,
    LoadedScenarioRecursiveRfRuntimeReport, LoadedScenarioRecursiveRfRuntimeSource,
    LoadedScenarioRfChannelRow, LoadedScenarioRfParentArenaRow, LoadedScenarioRfParticipantRow,
};
pub use spec::loaded_scenario_runtime_report_chain::{
    evaluate_loaded_scenario_runtime_report_chain_from_json_str,
    prove_loaded_scenario_runtime_report_chain_preserves_authority,
    LoadedScenarioRuntimeReportChainReport, LoadedScenarioRuntimeReportChainSource,
    LoadedScenarioRuntimeReportChainStage,
};
pub use spec::loaded_scenario_studio_session_envelope::{
    evaluate_loaded_scenario_studio_session_envelope_from_json_str,
    prove_loaded_scenario_session_envelope_preserves_authority_boundaries,
    LoadedScenarioAuthorityEnvelope, LoadedScenarioRuntimeSidecarEnvelope,
    LoadedScenarioSessionSource, LoadedScenarioStudioSessionEnvelope,
};
pub use spec::local_allocation_recursive_rf_source::{
    evaluate_runtime_local_allocation_with_rf_source,
    prove_local_allocation_recursive_source_preserves_authority,
    runtime_local_allocation_from_owner_silo_disburse_report, LocalAllocationRecursiveSourceError,
    LocalAllocationRecursiveSourceErrorKind, LocalAllocationRfSourceMode,
    LocalAllocationRfSourceSelection, RuntimeLocalAllocationReport,
    RuntimeLocalAllocationRfSourceReport,
};
pub use spec::local_effect_application::{
    apply_runtime_local_effect_records, evaluate_runtime_local_effect_application,
    local_effect_application_aggregate_totals, prove_local_effect_application_preserves_authority,
    LocalEffectApplicationAuthorityProof, LocalEffectApplicationDeferral,
    LocalEffectApplicationDeferralKind, LocalEffectApplicationError,
    LocalEffectApplicationErrorKind, RuntimeLocalEffectApplicationRecord,
    RuntimeLocalEffectApplicationReport,
};
pub use spec::local_effect_recursive_rf_source::{
    evaluate_local_effect_application_with_rf_source,
    local_effect_application_from_participant_effects_report,
    local_participant_effects_from_runtime_local_allocation_report,
    prove_local_effect_recursive_source_preserves_authority, LocalEffectApplicationRfSourceReport,
    LocalEffectRecursiveSourceError, LocalEffectRecursiveSourceErrorKind, LocalEffectRfSourceMode,
    LocalEffectRfSourceSelection,
};
pub use spec::local_participant_effects::{
    evaluate_local_participant_effects, local_participant_effects_aggregate_totals,
    local_participant_effects_from_allocations, LocalParticipantEffectsDeferral,
    LocalParticipantEffectsDeferralKind, LocalParticipantEffectsError,
    LocalParticipantEffectsErrorKind, LocalParticipantEffectsReport, RuntimeLocalParticipantEffect,
};
pub use spec::overlay::OverlaySpec;
pub use spec::owner_silo_disburse_down::{
    apply_owner_silo_runtime_disburse_down_cpu, owner_silo_demand_aggregate_totals,
    owner_silo_demand_buckets_from_planet_child_rf, RuntimeOwnerSiloDemandBucket,
    RuntimeOwnerSiloDisburseDownAllocation, RuntimeOwnerSiloDisburseDownError,
    RuntimeOwnerSiloDisburseDownErrorKind, RuntimeOwnerSiloDisburseDownInput,
    RuntimeOwnerSiloDisburseDownResult,
};
pub use spec::owner_silo_recursive_rf_source::{
    evaluate_owner_silo_disburse_down_with_rf_source,
    owner_silo_demand_buckets_from_recursive_local_rf,
    prove_owner_silo_recursive_source_preserves_authority, OwnerSiloDisburseDownReport,
    OwnerSiloRecursiveSourceError, OwnerSiloRecursiveSourceErrorKind,
    OwnerSiloRfSourceDisburseReport, OwnerSiloRfSourceMode, OwnerSiloRfSourceSelection,
};
pub use spec::owner_silo_runtime_writeback::{
    apply_owner_silo_runtime_writeback_cpu,
    owner_silo_writeback_inputs_from_planet_child_reduce_up, read_owner_silo_capacity_from_owner,
    read_owner_silo_current_from_owner, runtime_owner_silo_states_from_scenario,
    RuntimeOwnerSiloState, RuntimeOwnerSiloWritebackError, RuntimeOwnerSiloWritebackErrorKind,
    RuntimeOwnerSiloWritebackInput, RuntimeOwnerSiloWritebackResult,
};
pub use spec::planet_child_location::{
    all_planet_child_locations, all_planet_gridcells, apply_local_gridcell_metadata,
    apply_planet_child_location_command, apply_planet_child_metadata,
    apply_planet_gridcell_metadata, apply_planet_local_grid_command,
    apply_star_system_local_grid_frame_metadata, child_location_role, collect_local_receiver_cells,
    collect_planet_non_grid_children, evaluate_planet_child_locations,
    is_admitted_planet_non_grid_child, is_local_gridcell, is_planet_child_location,
    is_planet_gridcell, local_gridcell_col, local_gridcell_role, local_gridcell_row,
    make_local_inert_gridcell, make_planet_child_location, make_planet_gridcell,
    planet_child_location_classification_label, planet_child_location_error_kind_label,
    planet_child_locations, planet_display_name, planet_gridcell_interior_frame, planet_gridcells,
    planet_id, planet_non_grid_child_kind_label, planet_non_grid_child_owner_ref, planet_owner_ref,
    star_system_gridcells, star_system_local_grid_frame, validate_planet_child_locations,
    LocalGridcellRoleEdit, PlanetChildLocationAdmissionClassification,
    PlanetChildLocationAdmissionError, PlanetChildLocationAdmissionErrorKind,
    PlanetChildLocationAdmissionReport, PlanetChildLocationCommand, PlanetChildLocationDeferral,
    PlanetChildLocationEditError, PlanetChildLocationEditErrorKind, PlanetChildLocationEditReport,
    PlanetLocalGridCommand, PlanetNonGridChildEntry,
};
pub use spec::planet_child_rf::{
    evaluate_planet_child_rf_admission, evaluate_planet_child_rf_reduce_up,
    planet_child_rf_admission_classification_label, planet_child_rf_participant_inputs,
    scope_key_from_participant, PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionError,
    PlanetChildRfAdmissionErrorKind, PlanetChildRfAdmissionReport, PlanetChildRfDeferral,
    PlanetChildRfDeferralKind, PlanetChildRfParticipantInput, PlanetChildRfReduceUpBucket,
    PlanetChildRfReduceUpReport, PlanetChildRfScopeKey, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
pub use spec::property::PropertySpec;
pub use spec::recursive_local_rf::{
    evaluate_recursive_local_rf, prove_recursive_local_rf_preserves_authority,
    recursive_local_rf_aggregate_source_rows, recursive_local_rf_arena_aggregate_totals,
    recursive_local_rf_participant_rows_from_planet_child_inputs,
    recursive_local_rf_report_matches_planet_child_compatibility_slice, LocalRfArenaKey,
    LocalRfArenaSettlement, LocalRfChildOutputRow, LocalRfParticipantRow, LocationRfArenaReport,
    RecursiveLocalRfAggregateSourceKind, RecursiveLocalRfAggregateSourceRow,
    RecursiveLocalRfAuthorityProof, RecursiveLocalRfCompatibilityReport, RecursiveLocalRfDeferral,
    RecursiveLocalRfDeferralKind, RecursiveLocalRfError, RecursiveLocalRfErrorKind,
    RecursiveLocalRfEvaluationReport,
};
pub use spec::recursive_rf_reconciliation::{
    project_planet_child_rf_ladder_rows, project_recursive_local_rf_rows,
    prove_recursive_rf_reconciliation_preserves_authority,
    reconcile_planet_child_rf_with_recursive_local_rf, PlanetChildRfProjectionRow,
    RecursiveRfProjectionRow, RecursiveRfReconciliationBucket, RecursiveRfReconciliationDeferral,
    RecursiveRfReconciliationDeferralKind, RecursiveRfReconciliationError,
    RecursiveRfReconciliationErrorKind, RecursiveRfReconciliationMismatch,
    RecursiveRfReconciliationMismatchKind, RecursiveRfReconciliationReport,
};
pub use spec::region_field::{
    ArenaPressureBindingSpec, CommitmentEffectLifecycleSpec, CommitmentEffectSpec,
    CompiledRegionFieldSummaryPolicy, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    GradientAxisSpec, MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec, RegionFieldSummaryPolicySpec,
};
pub use spec::resource_economy::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, RecipeInputSpec,
    ResourceEconomyOptInMode, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
pub use spec::resource_flow::{
    ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec, CouplingDelaySpec, CouplingSpec,
    EnrollmentSelectorSpec, ExplicitParticipantSpec, FissionPolicySpec, GatedRateOpSpec,
    GatedRateSpec, GatedRateTriggerSpec, RateFormulaOp, RateFormulaOpSpec, RateFormulaOperandSpec,
    RateFormulaSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec,
    WildcardAdmissionSpec,
};
pub use spec::runtime_local_allocation::{
    apply_runtime_local_allocations_from_disburse_down, runtime_local_allocation_aggregate_totals,
    RuntimeLocalAllocationApplicationError, RuntimeLocalAllocationApplicationErrorKind,
    RuntimeLocalAllocationApplicationReport, RuntimeLocalAllocationState,
};
pub use spec::runtime_participant_property_mutation_boundary::{
    evaluate_runtime_participant_property_mutation_boundary,
    prove_runtime_participant_property_mutation_boundary_preserves_authority,
    replay_runtime_participant_property_mutation_boundary,
    RuntimeParticipantPropertyMutationBoundaryError,
    RuntimeParticipantPropertyMutationBoundaryErrorKind,
    RuntimeParticipantPropertyMutationBoundaryRecord,
    RuntimeParticipantPropertyMutationBoundaryReplayReport,
    RuntimeParticipantPropertyMutationBoundaryReport, RuntimeParticipantPropertyMutationSourceMode,
    RuntimeParticipantPropertyViewRow, MAX_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT,
    MIN_RUNTIME_PARTICIPANT_PROPERTY_MUTATION_REPLAY_COUNT,
};
pub use spec::runtime_participant_state_mutation::{
    evaluate_runtime_participant_state_mutation,
    prove_runtime_participant_state_mutation_preserves_authority,
    replay_runtime_participant_state_mutation, RuntimeParticipantStateMutationError,
    RuntimeParticipantStateMutationErrorKind, RuntimeParticipantStateMutationKind,
    RuntimeParticipantStateMutationRecord, RuntimeParticipantStateMutationReplayReport,
    RuntimeParticipantStateMutationReport, RuntimeParticipantStateMutationSourceMode,
    RuntimeParticipantStateRow, MAX_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT,
    MIN_RUNTIME_PARTICIPANT_STATE_REPLAY_COUNT,
};
pub use spec::runtime_rf_tick::{
    evaluate_runtime_rf_tick, RuntimeRfTickDeferral, RuntimeRfTickDeferralKind, RuntimeRfTickError,
    RuntimeRfTickErrorKind, RuntimeRfTickReport,
};
pub use spec::runtime_rf_tick_source::{
    evaluate_runtime_rf_tick_source_comparison, evaluate_runtime_rf_tick_source_preview,
    evaluate_runtime_rf_tick_source_selection, prove_runtime_rf_tick_source_preserves_authority,
    prove_runtime_rf_tick_source_selection_preserves_authority, RuntimeRfTickSelectedSourceReport,
    RuntimeRfTickSourceComparisonReport, RuntimeRfTickSourceDelta, RuntimeRfTickSourceDeltaKind,
    RuntimeRfTickSourceError, RuntimeRfTickSourceErrorKind, RuntimeRfTickSourceKind,
    RuntimeRfTickSourceMode, RuntimeRfTickSourceSelectionGate, RuntimeRfTickSourceSelectionMode,
    RuntimeRfTickSourceSummary,
};
pub use spec::runtime_tick_history::{
    evaluate_runtime_tick_history_entry, replay_runtime_tick_history, scenario_authority_digest,
    RuntimeTickHistoryEntry, RuntimeTickHistoryError, RuntimeTickHistoryErrorKind,
    RuntimeTickReplayMismatch, RuntimeTickReplayReport, MAX_RUNTIME_TICK_REPLAY_COUNT,
};
pub use spec::runtime_tick_shell::{
    evaluate_runtime_tick_shell, runtime_tick_shell_stage_order, RuntimeTickExecutionReport,
    RuntimeTickId, RuntimeTickShellDeferral, RuntimeTickShellDeferralKind, RuntimeTickShellError,
    RuntimeTickShellErrorKind, RuntimeTickStage,
};
pub use spec::scenario::{
    apply_galaxy_map_metadata, apply_gridcell_property_edit, apply_gridcell_role_metadata,
    apply_owner_entity_metadata, apply_owner_silo_metadata,
    apply_participant_owner_flow_demand_metadata, apply_participant_owner_flow_metadata,
    apply_participant_owner_flow_resource_key_metadata, apply_scenario_metadata_to_root,
    canonical_scenario_link_key, canonical_scenario_link_pair, deserialize_scenario_authority,
    galaxy_map_display_name, galaxy_map_id, galaxy_map_role, game_session_child,
    game_session_galaxy_map, game_session_galaxy_maps, game_session_owners,
    gridcell_generated_system_id, gridcell_role, gridcell_structural_col, gridcell_structural_row,
    is_galaxy_map_entity, is_owner_entity_kind, make_galaxy_map, make_owner_entity,
    owner_archetype, owner_color_index, owner_display_name, owner_entity_id, owner_flow_deficit,
    owner_flow_demand, owner_flow_owner_ref, owner_flow_priority, owner_flow_resource_key,
    owner_flow_surplus, owner_has_silo_metadata, owner_silo_capacity, owner_silo_current,
    owner_silo_marker, property_u32, reserve_simthing_ids_from_scenario, resolve_map_container,
    resolve_map_container_mut, scenario_metadata_seed, scenario_metadata_seed_value,
    scenario_metadata_string, scenario_metadata_string_value, scenario_metadata_u32,
    scenario_metadata_u32_value, serialize_scenario_authority, set_galaxy_map_display_name,
    set_owner_display_name, spatial_authority_root, structural_property_value_u32,
    sync_root_metadata_from_sidecar, sync_sidecar_from_root_metadata,
    validate_legacy_world_root_compatibility, validate_scenario_game_session_child,
    validate_scenario_links, validate_scenario_root_authority, validate_session_galaxy_map,
    validate_session_owner_entities, validate_stead_mapping_consistency, ScenarioEditError,
    ScenarioLinkError, ScenarioRootError, ScenarioRootValidationMode, ScenarioSerdeError,
    SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement, SteadMappingError,
    GALAXY_CHILD_LOCATION_ROLE_MOON, GALAXY_CHILD_LOCATION_ROLE_PLANET,
    GALAXY_CHILD_LOCATION_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID, GALAXY_MAP_ID_PROPERTY_ID, GALAXY_MAP_ROLE_CANONICAL,
    GALAXY_MAP_ROLE_PROPERTY_ID, OWNER_ARCHETYPE_PROPERTY_ID, OWNER_COLOR_INDEX_PROPERTY_ID,
    OWNER_DISPLAY_NAME_PROPERTY_ID, OWNER_FLOW_DEFAULT_PRIORITY, OWNER_FLOW_DEFAULT_RESOURCE_KEY,
    OWNER_FLOW_DEFICIT_PROPERTY_ID, OWNER_FLOW_DEMAND_PROPERTY_ID,
    OWNER_FLOW_OWNER_REF_PROPERTY_ID, OWNER_FLOW_PRIORITY_PROPERTY_ID,
    OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID, OWNER_FLOW_SURPLUS_PROPERTY_ID, OWNER_ID_PROPERTY_ID,
    OWNER_SILO_CAPACITY_PROPERTY_ID, OWNER_SILO_CURRENT_PROPERTY_ID, OWNER_SILO_MARKER_PROPERTY_ID,
    PLANET_CLASS_PROPERTY_ID, PLANET_DISPLAY_NAME_PROPERTY_ID, PLANET_ID_PROPERTY_ID,
    PLANET_ORBIT_INDEX_PROPERTY_ID, PLANET_OWNER_REF_PROPERTY_ID,
    RUNTIME_PREVIEW_APPLIED_SIM_PROPERTY_ID, RUNTIME_PREVIEW_SATISFIED_SIM_PROPERTY_ID,
    RUNTIME_PREVIEW_SHORTFALL_SIM_PROPERTY_ID, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
    SCENARIO_GENERATOR_SEED_PROPERTY_ID, SCENARIO_GENERATOR_SHAPE_PROPERTY_ID,
    SCENARIO_ID_PROPERTY_ID, SCENARIO_RENDER_WORLD_X_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_Y_PROPERTY_ID, SCENARIO_RENDER_WORLD_Z_PROPERTY_ID,
    SCENARIO_SCHEMA_VERSION, SCENARIO_SCHEMA_VERSION_PROPERTY_ID,
    SCENARIO_SOURCE_LABEL_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_INTEGER_MAX, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    SIMTHING_SCENARIO_AUTHORITY_LABEL,
};
pub use spec::scenario::{
    LOCAL_GRIDCELL_COL_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_INERT, LOCAL_GRIDCELL_ROLE_PLANET,
    LOCAL_GRIDCELL_ROLE_PROPERTY_ID, LOCAL_GRIDCELL_ROLE_RECEIVER, LOCAL_GRIDCELL_ROW_PROPERTY_ID,
    LOCAL_GRID_DEFAULT_COLS, LOCAL_GRID_DEFAULT_ROWS, LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    LOCAL_GRID_FRAME_ROWS_PROPERTY_ID, STAR_SYSTEM_LOCAL_GRID_DEFAULT_COLS,
    STAR_SYSTEM_LOCAL_GRID_DEFAULT_ROWS, STAR_SYSTEM_LOCAL_GRID_FRAME_COLS_PROPERTY_ID,
    STAR_SYSTEM_LOCAL_GRID_FRAME_ROWS_PROPERTY_ID,
};
pub use spec::scenario_candidate_from_runtime::{
    evaluate_scenario_candidate_from_runtime_from_json_str,
    prove_scenario_candidate_from_runtime_preserves_original_authority,
    ScenarioCandidateFromRuntimeReport, ScenarioCandidateFromRuntimeSource,
    ScenarioCandidatePropertyMutationRecord,
};
pub use spec::scenario_candidate_save_reopen::{
    candidate_scenario_write_policy_report, candidate_scenario_write_temp_path,
    evaluate_scenario_candidate_save_reopen_from_json_str,
    prove_scenario_candidate_save_reopen_digest_stability,
    write_candidate_scenario_canonical_json_atomic, CandidateScenarioWritePolicy,
    ScenarioCandidateReopenReport, ScenarioCandidateSaveReopenReport,
    ScenarioCandidateSaveReopenSource, ScenarioCandidateSaveReport,
    ScenarioCandidateWritePolicyReport,
};
pub use spec::scenario_canonical_io::{
    load_scenario_spec_from_json_str, prove_scenario_canonical_load_save_roundtrip,
    save_scenario_spec_to_canonical_json, ScenarioCanonicalLoadReport,
    ScenarioCanonicalRoundtripReport, ScenarioCanonicalSaveReport,
};
pub use spec::scenario_ingestion::{
    ingest_scenario, ingest_scenario_from_str, ingestion_error_from_root,
    ingestion_error_from_serde, scenario_deferral_kind_label,
    scenario_ingestion_classification_label, studio_canonical_ingestion_profile,
    GalaxyMapAdmissionReport, OwnerAdmissionReport, ScenarioCompileReadinessReport,
    ScenarioDeferral, ScenarioDeferralKind, ScenarioFingerprint, ScenarioIngestionClassification,
    ScenarioIngestionError, ScenarioIngestionProfile, ScenarioIngestionResult,
    ScenarioTreeAdmissionReport, ScenarioValidationReport, StructuralAdmissionReport,
};
pub use spec::scenario_property_mutation_authority_boundary::{
    clone_scenario_candidate_with_runtime_property_view,
    evaluate_scenario_property_mutation_authority_boundary,
    prove_scenario_property_mutation_boundary_preserves_original_authority,
    replay_scenario_property_mutation_authority_boundary,
    ScenarioPropertyMutationAuthorityBoundaryError,
    ScenarioPropertyMutationAuthorityBoundaryErrorKind,
    ScenarioPropertyMutationAuthorityBoundaryReplayReport,
    ScenarioPropertyMutationAuthorityBoundaryReport, ScenarioPropertyMutationRecord,
    ScenarioPropertyMutationSourceMode, MAX_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT,
    MIN_SCENARIO_PROPERTY_MUTATION_REPLAY_COUNT,
};
pub use spec::scenario_stead_map_roundtrip::{
    evaluate_scenario_stead_map_roundtrip_from_json_str, extract_scenario_rf_metadata_rows,
    extract_scenario_spatial_tree_rows, extract_scenario_stead_id_rows,
    extract_scenario_stead_link_rows, ScenarioRfMetadataRow, ScenarioSpatialTreeRow,
    ScenarioSteadIdRow, ScenarioSteadLinkRow, ScenarioSteadMapRoundtripReport,
};
pub use spec::script::{
    PropertyKey, ScopeRef, ScriptEvalContext, ScriptEvalError, ScriptExpr, ScriptPredicate,
};
pub use spec::semantic_effect_execution_boundary::{
    evaluate_semantic_effect_execution_boundary,
    prove_semantic_effect_execution_boundary_preserves_authority,
    SemanticEffectExecutionBoundaryError, SemanticEffectExecutionBoundaryErrorKind,
    SemanticEffectExecutionBoundaryReport, SemanticEffectExecutionKind,
    SemanticEffectExecutionRecord, SemanticEffectExecutionSourceMode,
};
pub use spec::semantic_local_effects::{
    evaluate_semantic_local_effects, prove_semantic_local_effects_preserve_authority,
    semantic_local_effects_aggregate_totals, semantic_local_effects_from_application,
    SemanticLocalEffectAuthorityProof, SemanticLocalEffectDeferral,
    SemanticLocalEffectDeferralKind, SemanticLocalEffectError, SemanticLocalEffectErrorKind,
    SemanticLocalEffectKind, SemanticLocalEffectOutput, SemanticLocalEffectReport,
};
pub use spec::semantic_local_effects_recursive_rf_source::{
    evaluate_semantic_local_effects_with_rf_source,
    prove_semantic_local_effects_recursive_source_preserves_authority,
    semantic_local_effects_from_local_effect_application_report,
    SemanticLocalEffectRecursiveSourceError, SemanticLocalEffectRecursiveSourceErrorKind,
    SemanticLocalEffectRfSourceMode, SemanticLocalEffectRfSourceReport,
    SemanticLocalEffectRfSourceSelection,
};
pub use spec::semantic_participant_delta_preview::{
    evaluate_semantic_participant_delta_preview,
    prove_semantic_participant_delta_preview_preserves_authority, ParticipantDeltaPreviewKind,
    ParticipantDeltaPreviewSourceMode, ParticipantPropertyDeltaPreviewRecord,
    SemanticParticipantDeltaPreviewError, SemanticParticipantDeltaPreviewErrorKind,
    SemanticParticipantDeltaPreviewReport, RUNTIME_PREVIEW_APPLIED_PROPERTY_ID,
    RUNTIME_PREVIEW_SATISFIED_PROPERTY_ID, RUNTIME_PREVIEW_SHORTFALL_PROPERTY_ID,
};
pub use spec::session_resource_flow::{
    evaluate_owner_silo_flow, owner_silo_admission_classification_label,
    owner_silo_flow_participant_inputs, owner_silo_flow_participant_roots,
    owner_silo_flow_suppresses_ingestion_deferral, OwnerSiloAdmissionClassification,
    OwnerSiloAdmissionError, OwnerSiloAdmissionErrorKind, OwnerSiloAdmissionReport,
    OwnerSiloDeferral, OwnerSiloDeferralKind, OwnerSiloFlowParticipantInput,
};
pub use spec::spatial_local_grid::{
    default_local_grid_frame_for_spatial_gridcell, explicit_local_grid_frame_for_spatial_gridcell,
    implicit_receiver_cell_for_gridcell, interior_local_grid_frame_for_gridcell,
    is_local_coordinate_in_frame, is_receiver_local_gridcell,
    local_grid_frame_for_spatial_gridcell, LocalGridFrame, LocalGridFrameError,
    LocalGridFrameErrorKind, LocalReceiverCell, LocalReceiverCellRole,
};
pub use spec::stress_compose::{
    StressComposeProfileSpec, StressComposeSpec, StressOperatorSpec,
    STRESS_COMPOSE_MAX_INPUT_FIELDS, STRESS_COMPOSE_MAX_PROFILES, STRESS_OP_MISMATCH,
    STRESS_OP_OVERLAP, STRESS_OP_VELOCITY, STRESS_OP_WEIGHTED,
};
pub use spec::structural_edit::{
    apply_structural_placement_command, validate_structural_placements_under_galaxymap,
    GridcellRoleEdit, StructuralPlacementCommand, StructuralPlacementEditError,
    StructuralPlacementEditErrorKind, StructuralPlacementEditReport,
    StructuralPlacementEditWarning,
};
pub use spec::trigger::{TriggerDirection, TriggerSpec};
pub use spec::w_impedance_compose::{
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec, CT_4B_LOCAL_AUTOMATA_W_FEEDSTOCK,
    W_IMPEDANCE_COMPOSE_MAX_PROFILES,
};
pub use validate::validate_capability_tree;
pub use version::SpecVersion;
