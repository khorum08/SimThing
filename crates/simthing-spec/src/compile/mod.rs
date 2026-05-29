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
pub use region_field_admission::{
    admit_region_field_formula_class, compile_region_field_preview,
    compile_region_field_stencil_config, CompiledFieldCadence,
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
