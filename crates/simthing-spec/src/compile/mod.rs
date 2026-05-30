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
pub mod jit_kernel_descriptor_admission;
pub mod jit_kernel_graph_admission;
pub mod jit_kernel_graph_identity;
pub mod jit_kernel_cohort_preview;
pub mod jit_kernel_production_registry_shell;
pub mod jit_kernel_registry_preview;
pub mod overlay;
pub mod property;
pub mod region_field_admission;
pub mod region_field_budget;
pub mod resource_economy;
pub mod resource_economy_admission;
pub mod resource_flow_admission;
pub mod trigger;

pub use capability::{CapabilityTreeBuildOutput, CapabilityTreeBuilder};
pub use context::CompileContext;
pub use effect::compile_effect;
pub use event::compile_event;
pub use overlay::compile_overlay;
pub use property::compile_property;
pub use first_slice_scenario_admission::{
    compile_first_slice_scenario_preview, CompiledFirstSliceScenarioPreview,
};
pub use jit_exact_sqrt_artifact_admission::{
    exact_sqrt_f_artifact_descriptor, fnv1a64_hex, is_exact_mag2_fixed_descriptor,
    is_exact_mag_f_from_mag2_descriptor,
    is_exact_sqrt_f_descriptor, is_mag_f_dxdy_probe_descriptor, is_sead_obs0_overlay_score_descriptor,
    is_sead_obs2_multilayer_overlay_score_descriptor,
    is_sead_obs3_multilayer_fixed_score_descriptor,
    is_sead_obs4_threshold_event_descriptor,
    is_sead_event0_compaction_descriptor,
    is_sead_pipe0_observer_event_pipeline_descriptor,
    is_sead_event1_code_bucketing_descriptor,
    is_sead_event2_bucket_reductions_descriptor,
    mag2_fixed_exact_kernel_descriptor, mag_f_from_dxdy_probe_kernel_descriptor,
    mag_f_from_exact_mag2_kernel_descriptor, sead_obs0_overlay_score_kernel_descriptor,
    sead_obs2_multilayer_overlay_score_kernel_descriptor,
    sead_obs3_multilayer_fixed_score_kernel_descriptor,
    sead_obs4_threshold_event_kernel_descriptor,
    sead_event0_compaction_kernel_descriptor,
    sead_pipe0_observer_event_pipeline_kernel_descriptor,
    sead_event1_code_bucketing_kernel_descriptor,
    sead_event2_bucket_reductions_kernel_descriptor,
    sqrt_f_exact_kernel_descriptor, validate_exact_pre_sqrt_contract, validate_mag2_source_contract,
    validate_score_authority_contract,
    validate_sead_obs0_overlay_score_contract,
    validate_sead_obs2_multilayer_overlay_score_contract,
    validate_sead_obs3_multilayer_fixed_score_contract,
    validate_sead_obs4_threshold_event_contract,
    validate_sead_event0_compaction_contract,
    validate_sead_pipe0_observer_event_pipeline_contract,
    validate_sead_event1_code_bucketing_contract,
    validate_sead_event2_bucket_reductions_contract,
    validate_exact_sqrt_artifact_admission, validate_exact_sqrt_artifact_binding,
    ExactPreSqrtInputContract, ExactSqrtArtifactDescriptor, ExactSqrtAuthorityClass,
    Mag2SourceContract, ScoreAuthorityContract, MAG2_FIXED_DESCRIPTOR_ID, MAG2_FIXED_LABEL,
    EventAuthorityContract, EventCompactionMembershipAuthority, EventCompactionOrderAuthority,
    EventCodeBucketMembershipAuthority, EventCodeBucketOrderAuthority,
    EventBucketReductionInputAuthority, EventBucketReductionOrderAuthority,
    ThresholdAuthorityContract,
    MAG2_Q16_COMPONENT_MAX, MAG2_Q16_FRAC_BITS, MAG2_Q16_SCALE, MAG2_Q16_SCALE_SQ,
    MAG_F_FROM_DXDY_PROBE_DESCRIPTOR_ID, MAG_F_FROM_DXDY_PROBE_LABEL,
    MAG_F_FROM_MAG2_DESCRIPTOR_ID, MAG_F_FROM_MAG2_LABEL, SEAD_OBS0_DESCRIPTOR_ID,
    SEAD_OBS0_LABEL, SEAD_OBS2_DESCRIPTOR_ID, SEAD_OBS2_LABEL, SEAD_OBS2_LAYER_COUNT,
    SEAD_OBS3_DESCRIPTOR_ID, SEAD_OBS3_LABEL, SEAD_OBS3_LAYER_COUNT,
    SEAD_OBS4_DESCRIPTOR_ID, SEAD_OBS4_LABEL, SEAD_OBS4_LAYER_COUNT,
    SEAD_EVENT0_DESCRIPTOR_ID, SEAD_EVENT0_LABEL,
    SEAD_PIPE0_DESCRIPTOR_ID, SEAD_PIPE0_LABEL, SEAD_PIPE0_LAYER_COUNT,
    SEAD_EVENT1_DESCRIPTOR_ID, SEAD_EVENT1_LABEL, SEAD_EVENT1_CODE_COUNT,
    SEAD_EVENT2_DESCRIPTOR_ID, SEAD_EVENT2_LABEL, SEAD_EVENT2_CODE_COUNT,
    SQRT_F_ARTIFACT_HASH,
    SQRT_F_ARTIFACT_PATH, SQRT_F_DESCRIPTOR_ID, SQRT_F_DOMAIN, SQRT_F_ENTRYPOINT,
    SQRT_F_IO_CONTRACT, SQRT_F_PROOF_REPORT,
};
pub use jit_kernel_descriptor_admission::{
    landed_jit_kernel_descriptors, validate_exact_kernel_inputs,
    validate_kernel_descriptor_admission, KernelDescriptorSpec, KernelLane, KernelOutputSpec,
    NativeMathClass, OutputAuthority,
};
pub use jit_kernel_graph_admission::{
    validate_kernel_graph_admission, KernelGraphEdgeSpec, KernelGraphSpec,
};
pub use jit_kernel_graph_identity::{
    preview_kernel_graph_identity, KernelGraphIdentity,
};
pub use jit_kernel_cohort_preview::{
    preview_kernel_graph_cohorts, KernelGraphCohortPreview, KernelGraphCohortPreviewSet,
    KernelGraphRequestSpec,
};
pub use jit_kernel_registry_preview::{
    preview_kernel_registry_manifest, preview_production_candidate_registry_entry,
    validate_kernel_registry_manifest_preview, validate_production_candidate_preview_entry,
    KernelRegistryEntryPreview, KernelRegistryLane,
    KernelRegistryManifestPreview,
};
pub use jit_kernel_production_registry_shell::{
    ProductionKernelRegistryShell, ProductionKernelRegistryShellConfig,
    RegisteredProductionCandidate,
};
pub use region_field_admission::{
    admit_region_field_formula_class, compile_region_field_preview,
    compile_region_field_stencil_config, validate_region_field_frame_gradient_sinks,
    compile_region_field_frame_preview, CompiledFieldCadence,
    CompiledFirstSliceCommitmentDirection, CompiledFirstSliceCommitmentThreshold,
    CompiledRegionFieldBoundaryMode, CompiledRegionFieldMaskMode, CompiledRegionFieldOperator,
    CompiledRegionFieldPreview, CompiledRegionFieldSourcePolicy, CompiledRegionFieldStencilSpec,
    CompiledGradientAxis,
    CompiledRegionFieldSummaryPolicy, ADMITTED_REGION_FIELD_FORMULA_CLASSES,
    FIRST_SLICE_FIELD_URGENCY_COL, REGION_FIELD_DEFAULT_HORIZON_CAP,
    REGION_FIELD_EXTENDED_HORIZON_CAP, REGION_FIELD_EXTENDED_MAX_GRID,
    REGION_FIELD_MAX_CELL_COUNT, REGION_FIELD_STANDARD_MAX_GRID,
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
pub use eml_gadget::{
    compile_eml_gadget, compile_eml_gadget_stack, eval_eml_postfix,
    oracle_field_sampler, oracle_soft_step, oracle_weighted_accumulator,
    oracle_velocity_monitor, oracle_decay, oracle_ema, oracle_bounded_feedback,
    oracle_hysteresis, oracle_acceleration, reject_unknown_gadget_kind,
    CompiledEmlGadget, CompiledEmlGadgetStack, DEFERRED_GADGET_KINDS,
    EmlGadgetCompileOptions, EmlGadgetDiagnostic, EmlGadgetCompositionPlan,
    EmlGadgetKind, EmlGadgetPreviewReport, EmlGadgetRegistry,
};
pub use trigger::compile_trigger;
