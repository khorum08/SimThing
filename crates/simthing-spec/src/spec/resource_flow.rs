use crate::spec::install_target::InstallTargetSpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};

/// Authored Resource Flow arena admission graph (E-10).
///
/// Declares explicit arena participation, caps, coupling edges, and fission policy.
/// Property `accumulator_spec` metadata is validated against this graph at session build.
///
/// `opt_in_mode` controls **GPU execution** for Resource Flow (RF-T1). Presence of arenas
/// alone does not enable `use_accumulator_resource_flow`; scenarios must opt in explicitly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourceFlowSpec {
    /// Explicit production execution opt-in for E-11 flat-star Resource Flow GPU sync.
    ///
    /// Authored arena/coupling content is compiled regardless; only execution requires opt-in.
    #[serde(default)]
    pub opt_in_mode: ResourceFlowOptInMode,
    #[serde(default)]
    pub arenas: Vec<ArenaSpec>,
    #[serde(default)]
    pub couplings: Vec<CouplingSpec>,
}

/// Resource Flow GPU execution opt-in (RF-T1). Mirrors `ResourceEconomyOptInMode` posture.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceFlowOptInMode {
    /// Compile/install Resource Flow artifacts; do not enable GPU Resource Flow sync.
    #[default]
    Disabled,
    /// Enable E-11 flat-star D=2 GPU path for this scenario/game mode only.
    FlatStarOptIn,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArenaSpec {
    pub name: String,
    pub flow_property: PropertyKey,
    #[serde(default)]
    pub balance_property: Option<PropertyKey>,
    pub max_participants: u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    #[serde(default)]
    pub fission_policy: FissionPolicySpec,
    /// Structural OrderBand reservation for future E-11 allocation (0 until enrolled).
    #[serde(default)]
    pub reserved_orderband_depth: u32,
    /// Reserved child slots per intermediate for fission contiguity (E-10R).
    #[serde(default)]
    pub reserved_gap_per_intermediate: u32,
    /// Expected max children per intermediate — must fit in reserved gap (E-10R).
    #[serde(default)]
    pub expected_max_children_per_intermediate: u32,
    #[serde(default)]
    pub explicit_participants: Vec<ExplicitParticipantSpec>,
    /// E-2B: authored enrollment selector resolved to `explicit_participants` at session install.
    #[serde(default)]
    pub enrollment: Option<EnrollmentSelectorSpec>,
    #[serde(default)]
    pub wildcard_admission: Option<WildcardAdmissionSpec>,
}

/// Resource Flow arena enrollment selector (E-2B).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EnrollmentSelectorSpec {
    /// Use authored `explicit_participants` only (default when `enrollment` is omitted).
    #[default]
    ExplicitOnly,
    /// Resolve live session install targets into `explicit_participants` before E-10R preflight.
    InstallTarget(InstallTargetSpec),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExplicitParticipantSpec {
    pub slot: u32,
    /// Session-local SimThing identity (raw id assigned at install).
    pub subtree_root_id: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WildcardAdmissionSpec {
    /// Declared upper bound on selector expansion. `None` is rejected at compile.
    pub max_expansion: Option<u32>,
    /// Pre-computed expansion count at session build (designer/compiler supplied).
    #[serde(default)]
    pub expanded_count: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FissionPolicySpec {
    Inherit,
    #[default]
    Reevaluate,
    Reject,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CouplingSpec {
    pub from_arena: String,
    pub to_arena: String,
    pub delay: CouplingDelaySpec,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CouplingDelaySpec {
    Algebraic,
    OneTickDelay,
    BoundaryStage { stage: u32 },
    AccumulatorState { property: PropertyKey },
}
