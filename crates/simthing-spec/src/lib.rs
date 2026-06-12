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
    compile_resource_economy_authoring_preview, compile_resource_flow_admission, compile_trigger,
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
    EmlGadgetCompileOptions, EmlGadgetCompositionPlan, EmlGadgetDiagnostic, EmlGadgetKind,
    EmlGadgetPreviewReport, EmlGadgetRegistry, EventAuthorityContract,
    EventBucketReductionInputAuthority, EventBucketReductionOrderAuthority,
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
    REGION_FIELD_STANDARD_MAX_GRID, SQRT_F_ARTIFACT_HASH, SQRT_F_ARTIFACT_PATH,
    SQRT_F_DESCRIPTOR_ID, SQRT_F_DOMAIN, SQRT_F_ENTRYPOINT, SQRT_F_IO_CONTRACT,
    SQRT_F_PROOF_REPORT,
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
pub use spec::overlay::OverlaySpec;
pub use spec::property::PropertySpec;
pub use spec::region_field::{
    ArenaPressureBindingSpec, CompiledRegionFieldSummaryPolicy, FirstSliceCommitmentDirectionSpec,
    FirstSliceCommitmentSpec, GradientAxisSpec, MappingExecutionProfile, PressurePlacementSpec,
    PressureSourceSpec, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, RegionFieldSummaryPolicySpec,
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
pub use spec::script::{
    PropertyKey, ScopeRef, ScriptEvalContext, ScriptEvalError, ScriptExpr, ScriptPredicate,
};
pub use spec::trigger::{TriggerDirection, TriggerSpec};
pub use validate::validate_capability_tree;
pub use version::SpecVersion;
