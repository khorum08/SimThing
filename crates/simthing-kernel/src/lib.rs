//! KERNEL-DISPATCH-INCRATE-0 — authoritative runtime admission surface.
//!
//! Owns sealed resolved-state buffers, GPU dispatch/encode/readback, decision/emission records,
//! spatial participation proofs, and high-level tick entry points. Consumers observe via read-only
//! views and sealed `read_*()` outputs; producers route through AccumulatorOp / BoundaryProtocol.

#![forbid(unsafe_code)]

pub mod accumulator_op;
pub mod candidate_f_magnitude;
pub mod context;
pub mod decision_ingress;
pub mod eml_opcode_gate;
pub mod exact_magnitude_gate;
pub mod cpu_oracle;
pub mod emission_accumulator;
pub mod emission_oracle;
pub mod gpu_readback;
pub mod indexed_scatter;
pub mod intensity_accumulator;
pub mod overlay_orderband;
pub mod overlay_prep;
pub mod participation;
pub mod passes;
pub mod projection;
pub mod readback;
pub mod reduction;
pub mod reduction_orderband;
pub mod registration;
pub mod resolved;
pub mod sealed;
pub mod slot;
pub mod transfer_accumulator;
pub mod velocity_accumulator;
pub mod world_state;

pub use accumulator_op::{
    ao_wgsl0_fast_path_compatible, classify_ao_wgsl0_plan, debug_readback_allowed,
    emit_on_threshold_registrations_to_gpu, emit_on_threshold_registrations_to_ops, eval_eml_cpu,
    execute_intent_deltas_cpu, execute_ops_cpu, execute_threshold_ops_cpu,
    scoped_debug_readback_allowed, set_debug_readback_allowed, summaries_from_values,
    threshold_registrations_to_ops, validate_intent_deltas_no_duplicate_cells, AccumulatorInputGpu,
    AccumulatorInputListTable, AccumulatorOpGpu, AccumulatorOpSession, AccumulatorOpSessionError,
    AoWgsl0Compatibility, AoWgsl0FallbackReason, AoWgsl0PlanShape, DebugReadbackGuard,
    EmissionOpPlanSignature, EmlGpuProgramTable, EmlTreeRangeGpu, EmlUploadError, EncodeError,
    ExactnessClass, InputListRange, InputListUploadError, IntensityEmlOpPlanSignature,
    LegacyOracleFamily, OpSetHandle, OperationFamily, OverlayCompileCache, PackedAccumulatorUpload,
    PackedIntentUpload, PackedThresholdUpload, SlotSummary, TransferOpPlanSignature,
    WorldAccumulatorRuntime, WorldSummaryRuntime, AO_WGSL0_ENTRY_POINT, DEFAULT_EML_NODE_CAPACITY,
    DEFAULT_EML_TREE_CAPACITY, DEFAULT_INPUT_LIST_CAPACITY, DEFAULT_THRESHOLD_EMISSION_CAPACITY,
    WORKGROUP_SIZE,
};
pub use candidate_f_magnitude::{
    max_candidate_f_magnitude_bits, CandidateFMagnitudeError, CandidateFMagnitudeReport,
    CandidateFMagnitudeRequest, GradientPairGpu,
};
pub use decision_ingress::{
    ApproximateDecisionDiagnostic, BoundaryEmissionToken, CpuDiagnosticDecision,
    DecisionIngressError, EmissionToken, StructuralCommitment, ThresholdCrossingToken,
};
pub use eml_opcode_gate::{
    combine_in_closed_vocabulary, opcode_in_closed_vocabulary, AdmittedEvalEmlCombine,
    AdmittedEvalEmlOpcode, CombineRegistrationRequest, CpuOracleParityProof, EvalEmlCombine,
    EvalEmlOpcode, EvalEmlVocabulary, GenericPrimitiveRegistration, OpcodeGateError,
    OpcodeRegistrationGate, OpcodeRegistrationRequest, SemanticOpcodeRegistration,
    SoftStepPolicyConditional,
};
pub use exact_magnitude_gate::{
    exact_mag2_bits_q16, mint_exact_magnitude_proof_candidate_f,
    mint_exact_magnitude_proof_candidate_f_cpu, sqrt_cr_f_bits, ApproximateDiagnostic,
    CommitmentRegistration, ExactMagnitudeProof,
};
pub use context::{GpuContext, GpuInitError};
pub use cpu_oracle::{execute_ops_cpu_with_emissions, CpuOracleError};
pub use emission_accumulator::{
    cpu_oracle_emission_records, emission_plan_signature_fields, encode_emission_plan,
    plan_emission_ops, EmissionFormula, EmissionPlan, EmissionPlanError, EmissionRegistration,
    EmissionSyncError, FORMULA_KIND_CONSTANT, FORMULA_KIND_EVAL_EML, FORMULA_KIND_IDENTITY_FLOOR,
    NO_CONSTANT, NO_MAX_EMIT, NO_TREE_ID,
};
pub use emission_oracle::{EmissionOracleError, EmissionOracleFormula, EmissionOracleRegistration};
pub use gpu_readback::{
    EmissionRecordReadback, KernelReadbackError, ThresholdEmissionReadback,
    ThresholdEventCandidatesReadback,
};
pub use indexed_scatter::{
    cpu_scatter_indexed, validate_scatter_entries, IndexedScatterError, IndexedScatterOp,
    ScatterEntry,
};
pub use intensity_accumulator::{
    build_intensity_eml_entries, plan_intensity_eml_ops, register_intensity_eml_formulas,
    IntensityEmlEntry, IntensityEmlPlan,
};
pub use overlay_orderband::{plan_overlay_orderband, OverlayOrderBandPlan};
pub use overlay_prep::build_overlay_deltas;
pub use participation::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, PlacedParticipant,
    PlacedParticipantValidationError, StructuralGridPlacement,
};
pub use passes::{AccumulatorPipelineSessions, Pipelines};
pub use projection::project_tree_to_values;
pub use reduction::{
    build_column_rule_descriptors, build_column_rules, build_topology, cpu_reduce_oracle,
    cpu_reduce_oracle_call_count, encode_column_rules, reset_cpu_reduce_oracle_call_count,
    ColumnRuleDescriptor, Topology, TopologyState,
};
pub use reduction_orderband::{
    plan_reduction_orderband, reduction_soft_band_for_depth_bucket, ReductionOrderBandPlan,
    ReductionPlanError,
};
pub use registration::{
    ThresholdRegistration, DIR_DOWNWARD, DIR_EITHER, DIR_UPWARD, THRESH_BUF_OUTPUT,
    THRESH_BUF_VALUES,
};
pub use resolved::ResolvedGpuBuffers;
pub use sealed::{
    cpu_oracle_threshold_events, EmissionRecord, EmissionRecordGpu, ResolvedWriteAuthority,
    ThresholdEmission, ThresholdEmissionGpu, ThresholdEvent, ThresholdEventGpu,
    DEFAULT_EMISSION_CAPACITY,
};
pub use slot::{SlotAllocError, SlotAllocator};
pub use transfer_accumulator::{
    conjunctive_recipe_registration_to_transfer, conjunctive_recipe_registrations_to_transfer,
    discrete_transfer_registration_to_transfer, discrete_transfer_registrations_to_transfer,
    encode_transfer_plan, plan_transfer_ops, TransferInputRef, TransferPlan, TransferPlanError,
    TransferRegistration, TransferSyncError,
};
pub use velocity_accumulator::{
    plan_governed_integration, plan_governed_integration_at_band, plan_velocity_integration,
    GovernedIntegrationPlan, PlannerError, VelocityAccumulatorPlan,
};
pub use world_state::{
    build_governed_pairs, encode_rule, governed_pairs_for_property, GovernedPair, IntentDelta,
    OverlayDelta, SlotDeltaRange, WorldGpuState, CLAMP_BOUNDED, CLAMP_FLOORED, CLAMP_UNBOUNDED,
    OP_ADD, OP_MULTIPLY, OP_SET, RULE_FIRST, RULE_MAX, RULE_MEAN, RULE_MIN, RULE_SUM,
    RULE_WEIGHTED_MEAN, WEIGHT_COL_NONE,
};

#[cfg(test)]
mod dependency_budget;
