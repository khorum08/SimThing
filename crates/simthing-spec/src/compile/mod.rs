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
pub mod event;
pub mod overlay;
pub mod property;
pub mod resource_economy;
pub mod region_field_admission;
pub mod resource_flow_admission;
pub mod trigger;

pub use capability::{CapabilityTreeBuildOutput, CapabilityTreeBuilder};
pub use context::CompileContext;
pub use effect::compile_effect;
pub use event::compile_event;
pub use overlay::compile_overlay;
pub use property::compile_property;
pub use resource_economy::{
    compile_resource_economy, CompiledEmitOnThreshold, CompiledEmissionFormula,
    CompiledResourceEconomy, CompiledResourceEmission, CompiledResourceRecipe,
    CompiledResourceRecipeInput, CompiledResourceTransfer, ResourceEconomyDiagnostic,
    ResourceEconomyExpansionReport,
};
pub use region_field_admission::{
    compile_region_field_preview, compile_region_field_stencil_config, admit_region_field_formula_class,
    CompiledFieldCadence, CompiledRegionFieldBoundaryMode, CompiledRegionFieldMaskMode,
    CompiledRegionFieldOperator, CompiledRegionFieldPreview, CompiledRegionFieldSourcePolicy,
    CompiledRegionFieldStencilSpec, ADMITTED_REGION_FIELD_FORMULA_CLASSES,
    REGION_FIELD_DEFAULT_HORIZON_CAP, REGION_FIELD_EXTENDED_HORIZON_CAP,
    REGION_FIELD_EXTENDED_MAX_GRID, REGION_FIELD_MAX_CELL_COUNT,
    REGION_FIELD_STANDARD_MAX_GRID,
};
pub use resource_flow_admission::{
    compile_resource_flow_admission, CompiledArenaAdmission, CompiledCouplingAdmission,
    CompiledCouplingDelay, CompiledResourceFlowAdmission, ResourceFlowDiagnostic,
    ResourceFlowExpansionReport,
};
pub use trigger::compile_trigger;
