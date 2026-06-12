//! Spec → runtime compilation.
//!
//! Compilers turn authored `*Spec` structures into live SimThing primitives:
//!
//! - [`compile_property`] registers a `SimProperty` with a `DimensionRegistry`.
//! - [`compile_overlay`] builds an `Overlay` instance (caller attaches it).
//! - [`CapabilityTreeBuilder`] compiles a full capability tree spec into a
//!   template `SimThing`, a `CapabilityTreeDefinition`, and the unlock
//!   registrations PR 4 will hand to the feeder.
//!
//! [`CompileContext`] threads the registry through batch compilation of multiple
//! specs from the same `DomainPackSpec` / `GameModeSpec`.

pub mod capability;
pub mod context;
pub mod effect;
pub mod eml_gadget;
pub mod event;
pub mod first_slice_scenario_admission;
pub mod jit_exact_sqrt_artifact_admission;
pub mod jit_kernel_cohort_preview;
pub mod jit_kernel_descriptor_admission;
pub mod jit_kernel_graph_admission;
pub mod jit_kernel_graph_identity;
pub mod jit_kernel_production_registry_shell;
pub mod jit_kernel_registry_preview;
pub mod overlay;
pub mod property;
pub mod region_field_admission;
pub mod region_field_budget;
pub mod resource_economy;
pub mod resource_economy_admission;
pub mod resource_flow_admission;
pub mod stress_compose_admission;
pub mod trigger;
pub mod w_impedance_compose_admission;

pub use capability::{CapabilityTreeBuildOutput, CapabilityTreeBuilder};
pub use context::CompileContext;
pub use effect::compile_effect;
pub use eml_gadget::{
    compile_eml_gadget, compile_eml_gadget_stack, eval_eml_postfix, oracle_acceleration,
    oracle_bounded_feedback, oracle_decay, oracle_ema, oracle_field_sampler, oracle_hysteresis,
    oracle_soft_step, oracle_velocity_monitor, oracle_weighted_accumulator,
    reject_unknown_gadget_kind, CompiledEmlGadget, CompiledEmlGadgetStack, EmlGadgetCompileOptions,
    EmlGadgetCompositionPlan, EmlGadgetDiagnostic, EmlGadgetKind, EmlGadgetPreviewReport,
    EmlGadgetRegistry, DEFERRED_GADGET_KINDS,
};
pub use event::compile_event;
pub use first_slice_scenario_admission::{
    compile_first_slice_scenario_preview, CompiledFirstSliceScenarioPreview,
};
pub use jit_exact_sqrt_artifact_admission::{
    exact_sqrt_f_artifact_descriptor, field_policy_act0_numeric_proposals_kernel_descriptor,
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
    mag2_fixed_exact_kernel_descriptor, mag_f_from_dxdy_probe_kernel_descriptor,
    mag_f_from_exact_mag2_kernel_descriptor, sqrt_f_exact_kernel_descriptor,
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
    validate_field_policy_pipe0_observer_event_pipeline_contract, validate_mag2_source_contract,
    validate_score_authority_contract, EventAuthorityContract, EventBucketReductionInputAuthority,
    EventBucketReductionOrderAuthority, EventCodeBucketMembershipAuthority,
    EventCodeBucketOrderAuthority, EventCompactionMembershipAuthority,
    EventCompactionOrderAuthority, ExactPreSqrtInputContract, ExactSqrtArtifactDescriptor,
    ExactSqrtAuthorityClass, Mag2SourceContract, NumericProposalMembershipAuthority,
    NumericProposalOrderAuthority, PhaseEEconomicFixtureRecordAuthority,
    PhaseEFixtureProposalAdmissionAuthority, PhaseEProposalConsumerInputAuthority,
    PhaseEProposalSummaryOrderAuthority, ScoreAuthorityContract, ThresholdAuthorityContract,
    FIELD_POLICY_ACT0_CODE_COUNT, FIELD_POLICY_ACT0_DESCRIPTOR_ID, FIELD_POLICY_ACT0_LABEL,
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
    FIELD_POLICY_PIPE0_LAYER_COUNT, MAG2_FIXED_DESCRIPTOR_ID, MAG2_FIXED_LABEL,
    MAG2_Q16_COMPONENT_MAX, MAG2_Q16_FRAC_BITS, MAG2_Q16_SCALE, MAG2_Q16_SCALE_SQ,
    MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID, MAG_F_FROM_DXDY_PROBE_LABEL,
    MAG_F_FROM_MAG2_DESCRIPTOR_ID, MAG_F_FROM_MAG2_LABEL, SQRT_F_ARTIFACT_HASH,
    SQRT_F_ARTIFACT_PATH, SQRT_F_DESCRIPTOR_ID, SQRT_F_DOMAIN, SQRT_F_ENTRYPOINT,
    SQRT_F_IO_CONTRACT, SQRT_F_PROOF_REPORT,
};
pub use jit_kernel_cohort_preview::{
    preview_kernel_graph_cohorts, KernelGraphCohortPreview, KernelGraphCohortPreviewSet,
    KernelGraphRequestSpec,
};
pub use jit_kernel_descriptor_admission::{
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, KernelDescriptorSpec, KernelLane, KernelOutputSpec,
    NativeMathClass, OutputAuthority,
};
pub use jit_kernel_graph_admission::{
    validate_kernel_graph_admission, KernelGraphEdgeSpec, KernelGraphSpec,
};
pub use jit_kernel_graph_identity::{preview_kernel_graph_identity, KernelGraphIdentity};
pub use jit_kernel_production_registry_shell::{
    ProductionKernelRegistryShell, ProductionKernelRegistryShellConfig,
    RegisteredProductionCandidate,
};
pub use jit_kernel_registry_preview::{
    preview_kernel_registry_manifest, preview_production_candidate_registry_entry,
    validate_kernel_registry_manifest_preview, validate_production_candidate_preview_entry,
    KernelRegistryEntryPreview, KernelRegistryLane, KernelRegistryManifestPreview,
};
pub use overlay::compile_overlay;
pub use property::compile_property;
pub use region_field_admission::{
    admit_region_field_formula_class, compile_region_field_frame_preview,
    compile_region_field_preview, compile_region_field_stencil_config,
    validate_region_field_frame_gradient_sinks, CompiledFieldCadence,
    CompiledFirstSliceCommitmentDirection, CompiledFirstSliceCommitmentThreshold,
    CompiledGradientAxis, CompiledRegionFieldBoundaryMode, CompiledRegionFieldMaskMode,
    CompiledRegionFieldOperator, CompiledRegionFieldPreview, CompiledRegionFieldSourcePolicy,
    CompiledRegionFieldStencilSpec, CompiledRegionFieldSummaryPolicy,
    ADMITTED_REGION_FIELD_FORMULA_CLASSES, FIRST_SLICE_FIELD_URGENCY_COL,
    REGION_FIELD_DEFAULT_HORIZON_CAP, REGION_FIELD_EXTENDED_HORIZON_CAP,
    REGION_FIELD_EXTENDED_MAX_GRID, REGION_FIELD_MAX_CELL_COUNT, REGION_FIELD_STANDARD_MAX_GRID,
    SATURATING_FLUX_CHI_CFL_MAX,
};
pub use region_field_budget::{
    estimate_region_field_budget, region_field_isolation_multiplier, RegionFieldBudgetError,
    RegionFieldBudgetEstimate, RegionFieldBudgetSpec, RegionFieldIsolationPolicyEstimate,
};
pub use resource_economy::{
    compile_resource_economy, CompiledEmissionFormula, CompiledEmitOnThreshold,
    CompiledResourceEconomy, CompiledResourceEmission, CompiledResourceRecipe,
    CompiledResourceRecipeInput, CompiledResourceTransfer, ResourceEconomyDiagnostic,
    ResourceEconomyExpansionReport,
};
pub use resource_economy_admission::{
    compile_game_mode_resource_economy_authoring_preview,
    compile_resource_economy_authoring_preview, RecipePreview, ResourceBindingPreview,
    ResourceEconomyAuthoringPreview, ResourceEconomyPreviewReport, StaticPropertyNetPreview,
    ThresholdEmitPreview, TransferPreview,
};
pub use resource_flow_admission::{
    compile_resource_flow_admission, CompiledArenaAdmission, CompiledCouplingAdmission,
    CompiledCouplingDelay, CompiledResourceFlowAdmission, ResourceFlowDiagnostic,
    ResourceFlowExpansionReport,
};
pub use stress_compose_admission::{
    compile_stress_compose_preview, CompiledStressCompose, CompiledStressComposeProfile,
};
pub use trigger::compile_trigger;
pub use w_impedance_compose_admission::{
    compile_w_impedance_compose_preview, CompiledWImpedanceCompose,
    CompiledWImpedanceComposeProfile,
};
