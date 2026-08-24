//! **Populate** — tree/RF authoring through existing admission.

pub use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, CostBandAdmissionError,
    CostBandResourceMarker, DimensionRegistry, LogTier, OwnerBoundaryValidationError, OwnerRef,
    PropertyValue, SimProperty, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
    TransformOp,
};
pub use simthing_driver::Scenario;
pub use simthing_gpu::SlotAllocator;
pub use simthing_sim::CostBandSemantic;
pub use simthing_spec::{
    compile_property, ArenaPressureBindingSpec, ArenaSpec, ExplicitParticipantSpec,
    FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec, FissionPolicySpec, GameModeSpec,
    MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec, PropertyKey, PropertySpec,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec, RegionFieldSummaryPolicySpec, ResourceFlowOptInMode, ResourceFlowSpec,
};

/// Bind a subtree ownership boundary. Descendants inherit by absence.
pub fn owner(node: &mut SimThing, owner: &OwnerRef) {
    simthing_core::bind_owner(node, owner);
}

/// Admit only genuine owner crossings; redundant inherited-owner stamps fail.
pub fn ownership(root: &SimThing) -> Result<(), OwnerBoundaryValidationError> {
    simthing_core::validate_owner_binding_boundaries(root)
}

/// Author a queued CostBand shape as `(unit_cost, semantics)` data.
///
/// Runtime changes the available/depth operand; this shape is admitted once
/// and is not re-hydrated for each draw.
pub fn queued_cost_band(
    unit_cost: f32,
    throttle_hint_max_per_tick: Option<u32>,
    resource: Option<CostBandResourceMarker>,
) -> Result<(f32, CostBandSemantic), CostBandAdmissionError> {
    simthing_core::cost_band_quantize(0.0, unit_cost, true, throttle_hint_max_per_tick)?;
    let semantics = CostBandSemantic::admit_sink(throttle_hint_max_per_tick, resource)?;
    Ok((unit_cost, semantics))
}
