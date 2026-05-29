use serde::{Deserialize, Serialize};

/// Designer-authored sparse RegionCell field declaration (Phase M-3).
///
/// Grid size is square-only in v1: `grid_size = N` derives `width = height = N`.
/// Spec presence alone does not enable GPU mapping execution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionFieldSpec {
    pub name: String,
    /// Square grid edge length. Designer does not author separate width/height in v1.
    pub grid_size: u32,
    pub n_dims: u32,
    pub source_col: u32,
    pub target_col: u32,
    pub operator: RegionFieldOperatorSpec,
    pub horizon: u32,
    #[serde(default)]
    pub allow_extended_horizon: bool,
    pub alpha_self: f32,
    pub gamma_neighbor: f32,
    #[serde(default)]
    pub source_cap: Option<f32>,
    #[serde(default)]
    pub source_policy: RegionFieldSourcePolicySpec,
    pub cadence: RegionFieldCadenceSpec,
    #[serde(default)]
    pub grid_profile: RegionFieldGridProfile,
    #[serde(default)]
    pub reduction: Option<RegionFieldReductionSpec>,
    #[serde(default)]
    pub parent_formula: Option<RegionFieldFormulaBindingSpec>,
    #[serde(default)]
    pub commitment: Option<FirstSliceCommitmentSpec>,
    /// Atlas batching is provisional (M-4) and rejected at admission in M-3.
    #[serde(default)]
    pub request_atlas_batching: bool,
    /// Optional designer-facing VRAM budget cap (bytes). Rejects compile preview when estimate exceeds cap.
    #[serde(default)]
    pub max_region_field_vram_bytes: Option<u64>,
    /// Summary validity policy when dense field execution is skipped (Phase M SummaryValidity V1).
    #[serde(default)]
    pub summary_policy: RegionFieldSummaryPolicySpec,
}

/// Square grid admission profile (designer/spec layer).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionFieldGridProfile {
    /// Default v1/first-slice target: max grid 10.
    #[default]
    StandardSquare,
    /// Extended square grids up to 32 when explicitly authored.
    ExtendedSquare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionFieldOperatorSpec {
    Normalized,
    SourceCappedNormalized,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionFieldSourcePolicySpec {
    #[default]
    CallerManagedOneShotSeedThenZero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionFieldCadenceSpec {
    EveryTick,
    EveryN(u32),
    OnEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionFieldReductionSpec {
    pub child_slot_start: u32,
    pub child_slot_count: u32,
    pub child_col: u32,
    pub parent_slot: u32,
    pub parent_col: u32,
    pub order_band: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionFieldFormulaBindingSpec {
    pub formula_class: String,
    #[serde(default)]
    pub tree_id: Option<u32>,
}

/// First-slice designer-authored threshold binding over parent `field_urgency`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirstSliceCommitmentSpec {
    pub source_formula_class: String,
    pub parent_slot: u32,
    pub urgency_col: u32,
    pub threshold: f32,
    pub direction: FirstSliceCommitmentDirectionSpec,
    pub event_kind: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FirstSliceCommitmentDirectionSpec {
    Upward,
}

/// V1 summary validity policy for skipped/clean RegionField ticks (designer/spec layer).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegionFieldSummaryPolicySpec {
    /// Retain last GPU-resident parent summary when clean/skipped; zero-initial before first execution.
    #[default]
    CachedUntilDirtyWithZeroInitial,
}

/// Admitted summary policy (compile layer).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CompiledRegionFieldSummaryPolicy {
    #[default]
    CachedUntilDirtyWithZeroInitial,
}

/// Mapping execution opt-in profile (structure only in M-3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MappingExecutionProfile {
    #[default]
    Disabled,
    /// Admit/compile sparse RegionCell field structure; does not wire session runtime.
    SparseRegionFieldV1,
}

impl MappingExecutionProfile {
    pub fn enables_execution(self) -> bool {
        matches!(self, Self::SparseRegionFieldV1)
    }
}
