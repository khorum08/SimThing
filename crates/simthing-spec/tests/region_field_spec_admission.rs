//! Phase M-3 — RegionFieldSpec RON roundtrip and mapping admission framework.

use simthing_core::ColumnAwareReductionCombine;
use simthing_driver::field_scheduler::FieldCadence;
use simthing_gpu::{
    StructuredFieldStencilBoundaryMode, StructuredFieldStencilConfig,
    StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};
use simthing_sim::PipelineFlags;
use simthing_spec::{
    admit_region_field_formula_class, compile_region_field_frame_preview,
    compile_region_field_preview, compile_region_field_stencil_config,
    deserialize_region_field_ron, validate_region_field_frame_gradient_sinks, CompiledFieldCadence,
    CompiledRegionFieldMaskMode, CompiledRegionFieldOperator, CompiledRegionFieldSourcePolicy,
    FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec, GameModeSpec, GradientAxisSpec,
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, ResourceFlowExecutionProfile, SpecError,
};

fn standard_suppression_field() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "suppression_field".into(),
        grid_size: 10,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SourceCappedNormalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(500.0),
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

fn to_gpu_stencil_config(
    compiled: &simthing_spec::CompiledRegionFieldStencilSpec,
) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: compiled.width,
        height: compiled.height,
        n_dims: compiled.n_dims,
        source_col: compiled.source_col,
        target_col: compiled.target_col,
        horizon: compiled.horizon,
        alpha_self: compiled.alpha_self,
        gamma_neighbor: compiled.gamma_neighbor,
        weight_north: compiled.weight_north,
        weight_south: compiled.weight_south,
        weight_east: compiled.weight_east,
        weight_west: compiled.weight_west,
        source_cap: compiled.source_cap,
        operator: match compiled.operator {
            CompiledRegionFieldOperator::Normalized => StructuredFieldStencilOperator::Normalized,
            CompiledRegionFieldOperator::SourceCappedNormalized => {
                StructuredFieldStencilOperator::SourceCappedNormalized
            }
            CompiledRegionFieldOperator::Gradient { axis } => match axis {
                simthing_spec::CompiledGradientAxis::X => StructuredFieldStencilOperator::GradientX,
                simthing_spec::CompiledGradientAxis::Y => StructuredFieldStencilOperator::GradientY,
            },
            CompiledRegionFieldOperator::SaturatingFlux {
                u_sat,
                chi,
                choke_output_col,
            } => StructuredFieldStencilOperator::SaturatingFlux {
                u_sat,
                chi,
                choke_output_col,
            },
        },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: compiled.allow_extended_horizon,
    }
}

fn to_field_cadence(compiled: CompiledFieldCadence) -> FieldCadence {
    match compiled {
        CompiledFieldCadence::EveryTick => FieldCadence::EveryTick,
        CompiledFieldCadence::EveryN { n } => FieldCadence::EveryN { n },
        CompiledFieldCadence::OnEvent => FieldCadence::OnEvent,
    }
}

fn assert_region_field_err(spec: &RegionFieldSpec, reason_substr: &str) {
    let err = compile_region_field_preview(spec).expect_err("expected admission failure");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}


fn gradient_field(axis: GradientAxisSpec, output_col: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: "grad_field".into(),
        grid_size: 3,
        n_dims: 4,
        source_col: 0,
        target_col: output_col,
        operator: RegionFieldOperatorSpec::Gradient { axis, output_col },
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}


fn assert_frame_gradient_sink_err(fields: &[&RegionFieldSpec], reason_substr: &str) {
    let err = validate_region_field_frame_gradient_sinks(fields).expect_err("expected rejection");
    match err {
        SpecError::RegionFieldAdmission { reason, .. } => {
            assert!(
                reason.contains(reason_substr),
                "expected `{reason_substr}` in `{reason}`"
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
