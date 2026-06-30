use crate::spec::install_target::InstallTargetSpec;
use crate::spec::script::PropertyKey;
use serde::{Deserialize, Serialize};
use simthing_core::PlacedParticipant;

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
    #[serde(default)]
    pub base_obligations: Vec<BaseFlowObligationSpec>,
    /// CT-RF-EML-RATE-0: trigger-gated rate contributions evaluated per tick
    /// by an `EvalEML` effective-rate band ordered before the arena reduce
    /// bands — `intrinsic = (base + Σ add×gate) × (1 + Σ mult×gate)`,
    /// gate = `trigger_property ≥ at_least`. Rising and falling edges are
    /// exact by per-tick recomputation from the base column; per-tick
    /// transforms directly on rate columns are rejected (compounding).
    #[serde(default)]
    pub gated_rates: Vec<GatedRateSpec>,
}

/// One dynamic rate contribution (CT-RF-EML-RATE-0). With a `trigger` the
/// term is gated (`× gate`); without one it is an always-on dynamic term.
/// With a `rate_formula` the magnitude is a `value:` formula tree evaluated
/// per tick over explicit columns; otherwise the literal `rate` applies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatedRateSpec {
    pub id: String,
    pub arena: String,
    pub install: InstallTargetSpec,
    pub direction: BaseFlowDirectionSpec,
    pub op: GatedRateOpSpec,
    /// Non-negative magnitude for `Add` (sign from `direction`); fractional
    /// bonus for `Mult` (additive-in-effect across gated mults). Ignored when
    /// `rate_formula` is present.
    pub rate: f32,
    #[serde(default)]
    pub trigger: Option<GatedRateTriggerSpec>,
    /// `value:` formula tree (CT-3b+4a `value:` lowering): evaluated on the
    /// effective-rate EvalEML band, never CPU-side, never cached.
    #[serde(default)]
    pub rate_formula: Option<RateFormulaSpec>,
}

/// Authored `value:` script-value formula: `base`, then ordered operations
/// applied left to right. Flat (no script-value recursion in v1; recursive
/// references are a hard hydration error naming this limit).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateFormulaSpec {
    pub base: f32,
    pub ops: Vec<RateFormulaOpSpec>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateFormulaOpSpec {
    pub op: RateFormulaOp,
    pub operand: RateFormulaOperandSpec,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RateFormulaOp {
    Add,
    Mult,
    /// Result is at least the operand (Paradox script-value `min`).
    FloorAt,
    /// Result is at most the operand (Paradox script-value `max`).
    CeilAt,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RateFormulaOperandSpec {
    Literal(f32),
    /// Per-tick read of the participant's registered property column
    /// (Amount sub-field) — `value:` formulas consume live game state.
    Property(PropertyKey),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GatedRateOpSpec {
    Add,
    Mult,
}

/// Same-scope threshold gate over an explicit registered property column.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatedRateTriggerSpec {
    pub property: PropertyKey,
    pub at_least: f32,
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

/// RF-T4 — scenario-class / execution-profile enablement for flat-star Resource Flow GPU path.
///
/// Distinct from `ResourceFlowOptInMode`: profile enablement applies at session open when
/// spec `opt_in_mode` is `Disabled` or omitted. Does not flip global `PipelineFlags` default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceFlowExecutionProfile {
    /// No scenario-class Resource Flow GPU enablement.
    #[default]
    DefaultDisabled,
    /// Enable E-11 flat-star D=2 GPU path via named execution profile (RF-T4).
    FlatStarResourceFlow,
}

impl ResourceFlowExecutionProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultDisabled => "DefaultDisabled",
            Self::FlatStarResourceFlow => "FlatStarResourceFlow",
        }
    }

    pub fn enables_flat_star_resource_flow(self) -> bool {
        matches!(self, Self::FlatStarResourceFlow)
    }
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

/// Install-consumed base intrinsic-flow obligation for admitted arena participants.
///
/// This is the Resource Flow substrate's base-rate authoring surface. It seeds the
/// arena participant's `AccumulatorRole::IntrinsicFlow` sub-field during install and
/// is intentionally distinct from overlay modifiers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseFlowObligationSpec {
    pub id: String,
    pub arena: String,
    pub install: InstallTargetSpec,
    pub direction: BaseFlowDirectionSpec,
    pub rate: f32,
}

impl BaseFlowObligationSpec {
    pub fn signed_rate(&self) -> f32 {
        self.direction.sign() * self.rate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BaseFlowDirectionSpec {
    Produce,
    Upkeep,
}

impl BaseFlowDirectionSpec {
    pub fn sign(self) -> f32 {
        match self {
            Self::Produce => 1.0,
            Self::Upkeep => -1.0,
        }
    }
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
    /// When set, this participant is a child of the explicit participant whose
    /// `subtree_root_id` equals this value within the same arena list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_subtree_root_id: Option<u64>,
}

impl ExplicitParticipantSpec {
    /// Flat-star participant (direct child of the arena root) — **non-spatial RF channel identity**.
    ///
    /// Spatially-bound RF arenas over gridcell `Location`s must enroll via [`Self::spatial`] with a
    /// sealed [`PlacedParticipant`] proof minted by structural/boundary validation.
    pub fn flat(slot: u32, subtree_root_id: u32) -> Self {
        Self {
            slot,
            subtree_root_id,
            parent_subtree_root_id: None,
        }
    }

    /// Spatial arena participant — requires a sealed [`PlacedParticipant`] placement proof.
    ///
    /// External code cannot enroll a spatial gridcell `Location` with channel identity alone:
    ///
    /// ```compile_fail
    /// use simthing_core::{SimThingId, StructuralCoord};
    /// use simthing_spec::ExplicitParticipantSpec;
    /// fn spatial_arena_participant_without_placed_proof() {
    ///     let _coord = StructuralCoord::new(0, 0);
    ///     let _id = SimThingId::from_session_raw(42);
    ///     let _ = ExplicitParticipantSpec::from_spatial_channel_identity(0, _id, _coord);
    /// }
    /// ```
    pub fn spatial(slot: u32, placed: PlacedParticipant) -> Self {
        Self {
            slot,
            subtree_root_id: placed.participant().raw(),
            parent_subtree_root_id: None,
        }
    }

    /// Nested participant child of another explicit participant in the same arena.
    pub fn nested(slot: u32, subtree_root_id: u32, parent_subtree_root_id: u64) -> Self {
        Self {
            slot,
            subtree_root_id,
            parent_subtree_root_id: Some(parent_subtree_root_id),
        }
    }

    /// Nested spatial arena participant — requires a sealed [`PlacedParticipant`] placement proof.
    pub fn spatial_nested(
        slot: u32,
        placed: PlacedParticipant,
        parent_subtree_root_id: u64,
    ) -> Self {
        Self {
            slot,
            subtree_root_id: placed.participant().raw(),
            parent_subtree_root_id: Some(parent_subtree_root_id),
        }
    }
}

/// Build explicit participants for a spatially-bound RF arena from sealed placement proofs.
pub fn spatial_arena_explicit_participants(
    slot_and_placed: &[(u32, PlacedParticipant)],
) -> Vec<ExplicitParticipantSpec> {
    slot_and_placed
        .iter()
        .map(|(slot, placed)| ExplicitParticipantSpec::spatial(*slot, *placed))
        .collect()
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
