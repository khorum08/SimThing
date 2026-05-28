//! Phase M-3 — RegionFieldSpec admission and compile preview (spec layer).

use simthing_core::{
    ColumnAwareReductionCombine, ColumnAwareReductionSpec, WHITELISTED_FORMULA_CLASSES,
};

use crate::compile::region_field_budget::{
    estimate_region_field_budget, RegionFieldBudgetError, RegionFieldBudgetSpec,
    RegionFieldIsolationPolicyEstimate,
};
use crate::error::SpecError;
use crate::spec::region_field::{
    RegionFieldCadenceSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
};

pub const REGION_FIELD_STANDARD_MAX_GRID: u32 = 10;
pub const REGION_FIELD_EXTENDED_MAX_GRID: u32 = 32;
pub const REGION_FIELD_MAX_CELL_COUNT: u32 = 1024;
pub const REGION_FIELD_DEFAULT_HORIZON_CAP: u32 = 8;
pub const REGION_FIELD_EXTENDED_HORIZON_CAP: u32 = 16;

/// Admitted field formula classes at the designer/spec policy layer (M-3).
pub const ADMITTED_REGION_FIELD_FORMULA_CLASSES: &[&str] = &[
    "field_pressure",
    "field_urgency",
    "field_decay",
    "bounded_field_update",
    "conversion_rate",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompiledRegionFieldOperator {
    Normalized,
    SourceCappedNormalized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompiledRegionFieldSourcePolicy {
    CallerManagedOneShotSeedThenZero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompiledRegionFieldBoundaryMode {
    Zero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompiledRegionFieldMaskMode {
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompiledFieldCadence {
    EveryTick,
    EveryN { n: u32 },
    OnEvent,
}

/// Serializable intermediate stencil compile result (no `simthing-gpu` dependency).
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledRegionFieldStencilSpec {
    pub width: u32,
    pub height: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub horizon: u32,
    pub alpha_self: f32,
    pub gamma_neighbor: f32,
    pub source_cap: Option<f32>,
    pub operator: CompiledRegionFieldOperator,
    pub source_policy: CompiledRegionFieldSourcePolicy,
    pub boundary_mode: CompiledRegionFieldBoundaryMode,
    pub mask_mode: CompiledRegionFieldMaskMode,
    pub allow_extended_horizon: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledRegionFieldPreview {
    pub name: String,
    pub grid_size: u32,
    pub cell_count: u32,
    pub stencil: CompiledRegionFieldStencilSpec,
    pub cadence: CompiledFieldCadence,
    pub reduction: Option<ColumnAwareReductionSpec>,
    pub parent_formula_class: Option<String>,
}

fn field_err(field: &str, reason: impl Into<String>) -> SpecError {
    SpecError::RegionFieldAdmission {
        field: field.into(),
        reason: reason.into(),
    }
}

fn max_grid_for_profile(profile: RegionFieldGridProfile) -> u32 {
    match profile {
        RegionFieldGridProfile::StandardSquare => REGION_FIELD_STANDARD_MAX_GRID,
        RegionFieldGridProfile::ExtendedSquare => REGION_FIELD_EXTENDED_MAX_GRID,
    }
}

fn validate_grid(spec: &RegionFieldSpec) -> Result<u32, SpecError> {
    if spec.grid_size == 0 {
        return Err(field_err(&spec.name, "grid_size must be > 0"));
    }
    let max_grid = max_grid_for_profile(spec.grid_profile);
    if spec.grid_size > max_grid {
        return Err(field_err(
            &spec.name,
            format!("grid_size {} exceeds profile cap {}", spec.grid_size, max_grid),
        ));
    }
    let cells = spec
        .grid_size
        .checked_mul(spec.grid_size)
        .ok_or_else(|| field_err(&spec.name, "grid cell count overflow"))?;
    if cells > REGION_FIELD_MAX_CELL_COUNT {
        return Err(field_err(
            &spec.name,
            format!("grid cell count {cells} exceeds cap {REGION_FIELD_MAX_CELL_COUNT}"),
        ));
    }
    Ok(cells)
}

fn validate_columns(spec: &RegionFieldSpec) -> Result<(), SpecError> {
    if spec.n_dims == 0 {
        return Err(field_err(&spec.name, "n_dims must be > 0"));
    }
    if spec.source_col >= spec.n_dims || spec.target_col >= spec.n_dims {
        return Err(field_err(
            &spec.name,
            format!(
                "column out of range: source={} target={} n_dims={}",
                spec.source_col, spec.target_col, spec.n_dims
            ),
        ));
    }
    Ok(())
}

fn validate_operator_and_source(spec: &RegionFieldSpec) -> Result<(), SpecError> {
    match spec.operator {
        RegionFieldOperatorSpec::Normalized => {
            if spec.source_cap.is_some() {
                return Err(field_err(
                    &spec.name,
                    "source_cap is not allowed with Normalized",
                ));
            }
        }
        RegionFieldOperatorSpec::SourceCappedNormalized => {
            let cap = spec.source_cap.ok_or_else(|| {
                field_err(
                    &spec.name,
                    "SourceCappedNormalized requires source_cap",
                )
            })?;
            if !cap.is_finite() || cap <= 0.0 {
                return Err(field_err(
                    &spec.name,
                    "source_cap must be finite and > 0",
                ));
            }
        }
    }
    if !matches!(
        spec.source_policy,
        RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero
    ) {
        return Err(field_err(
            &spec.name,
            "only CallerManagedOneShotSeedThenZero source policy is admitted in v1",
        ));
    }
    Ok(())
}

fn validate_horizon(spec: &RegionFieldSpec) -> Result<(), SpecError> {
    if spec.horizon == 0 {
        return Err(field_err(&spec.name, "horizon must be >= 1"));
    }
    if spec.horizon > REGION_FIELD_EXTENDED_HORIZON_CAP {
        return Err(field_err(
            &spec.name,
            format!(
                "horizon {} exceeds absolute cap {}",
                spec.horizon, REGION_FIELD_EXTENDED_HORIZON_CAP
            ),
        ));
    }
    if spec.horizon > REGION_FIELD_DEFAULT_HORIZON_CAP && !spec.allow_extended_horizon {
        return Err(field_err(
            &spec.name,
            format!(
                "horizon {} exceeds default cap {} without allow_extended_horizon",
                spec.horizon, REGION_FIELD_DEFAULT_HORIZON_CAP
            ),
        ));
    }
    if spec.allow_extended_horizon
        && spec.horizon > REGION_FIELD_DEFAULT_HORIZON_CAP
        && !matches!(
            spec.operator,
            RegionFieldOperatorSpec::SourceCappedNormalized
        )
    {
        return Err(field_err(
            &spec.name,
            "extended horizon requires SourceCappedNormalized with source_cap stability contract",
        ));
    }
    if spec.allow_extended_horizon
        && spec.horizon > REGION_FIELD_DEFAULT_HORIZON_CAP
        && spec.source_cap.is_none()
    {
        return Err(field_err(
            &spec.name,
            "extended horizon requires source_cap stability contract",
        ));
    }
    Ok(())
}

fn compile_cadence(spec: &RegionFieldSpec) -> Result<CompiledFieldCadence, SpecError> {
    match spec.cadence {
        RegionFieldCadenceSpec::EveryTick => Ok(CompiledFieldCadence::EveryTick),
        RegionFieldCadenceSpec::EveryN(0) => Err(field_err(
            &spec.name,
            "EveryN cadence requires n > 0",
        )),
        RegionFieldCadenceSpec::EveryN(n) => Ok(CompiledFieldCadence::EveryN { n }),
        RegionFieldCadenceSpec::OnEvent => Ok(CompiledFieldCadence::OnEvent),
    }
}

fn compile_reduction(
    spec: &RegionFieldSpec,
    reduction: &RegionFieldReductionSpec,
) -> Result<ColumnAwareReductionSpec, SpecError> {
    if reduction.child_slot_count == 0 {
        return Err(field_err(
            &spec.name,
            "reduction child_slot_count must be > 0",
        ));
    }
    if reduction.child_col >= spec.n_dims || reduction.parent_col >= spec.n_dims {
        return Err(field_err(
            &spec.name,
            "reduction column out of range for n_dims",
        ));
    }
    Ok(ColumnAwareReductionSpec {
        child_slot_start: reduction.child_slot_start,
        child_slot_count: reduction.child_slot_count,
        child_col: reduction.child_col,
        parent_slot: reduction.parent_slot,
        parent_col: reduction.parent_col,
        combine: ColumnAwareReductionCombine::Sum,
        order_band: reduction.order_band,
    })
}

fn validate_formula_class(spec: &RegionFieldSpec, class: &str) -> Result<(), SpecError> {
    if ADMITTED_REGION_FIELD_FORMULA_CLASSES.contains(&class) {
        return Ok(());
    }
    if WHITELISTED_FORMULA_CLASSES.contains(&class) {
        return Err(field_err(
            &spec.name,
            format!("formula class `{class}` is whitelisted globally but not admitted for RegionFieldSpec"),
        ));
    }
    Err(field_err(
        &spec.name,
        format!("unknown or unbounded formula class `{class}`"),
    ))
}

/// Validate a field formula class for RegionFieldSpec parent bindings.
pub fn admit_region_field_formula_class(
    spec: &RegionFieldSpec,
    class: &str,
) -> Result<(), SpecError> {
    validate_formula_class(spec, class)
}

/// Compile only the stencil portion of a RegionFieldSpec preview.
pub fn compile_region_field_stencil_config(
    spec: &RegionFieldSpec,
) -> Result<CompiledRegionFieldStencilSpec, SpecError> {
    compile_region_field_preview(spec).map(|preview| preview.stencil)
}

/// Validate and compile a RegionFieldSpec into generic substrate preview configs.
pub fn compile_region_field_preview(
    spec: &RegionFieldSpec,
) -> Result<CompiledRegionFieldPreview, SpecError> {
    if spec.request_atlas_batching {
        return Err(field_err(
            &spec.name,
            "atlas batching is provisional (M-4) and not admitted in M-3",
        ));
    }

    let cell_count = validate_grid(spec)?;
    validate_columns(spec)?;
    validate_operator_and_source(spec)?;
    validate_horizon(spec)?;

    if !spec.alpha_self.is_finite() || !spec.gamma_neighbor.is_finite() {
        return Err(field_err(
            &spec.name,
            "alpha_self and gamma_neighbor must be finite",
        ));
    }

    let operator = match spec.operator {
        RegionFieldOperatorSpec::Normalized => CompiledRegionFieldOperator::Normalized,
        RegionFieldOperatorSpec::SourceCappedNormalized => {
            CompiledRegionFieldOperator::SourceCappedNormalized
        }
    };

    let cadence = compile_cadence(spec)?;
    let reduction = spec
        .reduction
        .as_ref()
        .map(|r| compile_reduction(spec, r))
        .transpose()?;

    let parent_formula_class = if let Some(binding) = &spec.parent_formula {
        validate_formula_class(spec, &binding.formula_class)?;
        Some(binding.formula_class.clone())
    } else {
        None
    };

    let stencil = CompiledRegionFieldStencilSpec {
        width: spec.grid_size,
        height: spec.grid_size,
        n_dims: spec.n_dims,
        source_col: spec.source_col,
        target_col: spec.target_col,
        horizon: spec.horizon,
        alpha_self: spec.alpha_self,
        gamma_neighbor: spec.gamma_neighbor,
        source_cap: spec.source_cap,
        operator,
        source_policy: CompiledRegionFieldSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: CompiledRegionFieldBoundaryMode::Zero,
        mask_mode: CompiledRegionFieldMaskMode::All,
        allow_extended_horizon: spec.allow_extended_horizon,
    };

    let preview = CompiledRegionFieldPreview {
        name: spec.name.clone(),
        grid_size: spec.grid_size,
        cell_count,
        stencil,
        cadence,
        reduction,
        parent_formula_class,
    };
    validate_budget_preview(spec, &preview)?;
    Ok(preview)
}

fn validate_budget_preview(spec: &RegionFieldSpec, preview: &CompiledRegionFieldPreview) -> Result<(), SpecError> {
    let Some(max_bytes) = spec.max_region_field_vram_bytes else {
        return Ok(());
    };
    let budget_spec = RegionFieldBudgetSpec {
        grid_size: preview.grid_size,
        column_count: preview.stencil.n_dims,
        buffer_multiplier: 2.0,
        copy_multiplier: 1.0,
        tile_count: 1,
        isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
        max_region_field_vram_bytes: Some(max_bytes),
    };
    estimate_region_field_budget(&budget_spec)
        .map(|_| ())
        .map_err(|err: RegionFieldBudgetError| {
        field_err(
            &spec.name,
            format!(
                "VRAM budget exceeded: requested {} bytes, allowed {} bytes",
                err.requested_bytes, err.allowed_bytes
            ),
        )
    })
}
