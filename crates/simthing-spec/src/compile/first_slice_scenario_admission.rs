//! Phase M first-slice scenario fixture admission and compile preview.

use crate::compile::region_field_admission::compile_region_field_preview;
use crate::compile::region_field_budget::{
    estimate_region_field_budget, RegionFieldBudgetSpec, RegionFieldIsolationPolicyEstimate,
};
use crate::error::SpecError;
use crate::spec::first_slice_scenario::FirstSliceScenarioSpec;
use crate::spec::region_field::MappingExecutionProfile;

use super::region_field_admission::CompiledRegionFieldPreview;

/// Admitted compile preview for a first-slice scenario fixture.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledFirstSliceScenarioPreview {
    pub name: String,
    pub mapping_execution_profile: MappingExecutionProfile,
    pub region_field: CompiledRegionFieldPreview,
    pub parent_formula_tree_id: Option<u32>,
    pub budget_estimate_bytes: Option<u64>,
    pub budget_limit_bytes: Option<u64>,
}

/// Validate and compile a first-slice scenario fixture into admitted preview structures.
pub fn compile_first_slice_scenario_preview(
    spec: &FirstSliceScenarioSpec,
) -> Result<CompiledFirstSliceScenarioPreview, SpecError> {
    if spec.name.trim().is_empty() {
        return Err(SpecError::RegionFieldAdmission {
            field: spec.name.clone(),
            reason: "scenario name must be non-empty".into(),
        });
    }

    let region_field = compile_region_field_preview(&spec.region_field)?;
    let parent_formula_tree_id = spec
        .region_field
        .parent_formula
        .as_ref()
        .and_then(|f| f.tree_id);

    let budget_limit_bytes = spec.region_field.max_region_field_vram_bytes;
    let budget = estimate_region_field_budget(&RegionFieldBudgetSpec {
        grid_size: region_field.grid_size,
        column_count: region_field.stencil.n_dims,
        buffer_multiplier: 2.0,
        copy_multiplier: 1.0,
        tile_count: 1,
        isolation_policy: RegionFieldIsolationPolicyEstimate::SingleGridNoAtlas,
        max_region_field_vram_bytes: budget_limit_bytes,
    })
    .map_err(|err| SpecError::RegionFieldAdmission {
        field: spec.name.clone(),
        reason: format!(
            "scenario budget estimate failed: requested {} bytes, allowed {} bytes",
            err.requested_bytes, err.allowed_bytes
        ),
    })?;

    Ok(CompiledFirstSliceScenarioPreview {
        name: spec.name.clone(),
        mapping_execution_profile: spec.mapping_execution_profile,
        region_field,
        parent_formula_tree_id,
        budget_estimate_bytes: Some(budget.estimated_bytes),
        budget_limit_bytes,
    })
}
