//! V7.8-MET-SCENARIO-0: named consumer scenarios for promoted M/E/T lines.
//!
//! This module records the minimum product scenarios that can later unblock the
//! v7.8 promoted capability lines. It is metadata/admission only: no E-11B,
//! D-2a, M-4/M-4A, ClauseThing, ClauseScript, GPU dispatch, or production
//! `SimSession` wiring is created here.

use serde::{Deserialize, Serialize};

use super::diagnostic::{
    designer_admission_diagnostic, DesignerAdmissionDiagnostic, DesignerAdmissionDiagnosticCode,
};

pub const V78_MET_SCENARIO_PACK_ID: &str = "v7_8_met_consumer_scenario_pack";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum V78PromotedLine {
    LineA,
    LineB,
    LineC,
}

impl V78PromotedLine {
    pub const fn capability_label(self) -> &'static str {
        match self {
            Self::LineA => "E / Line A - Nested Resource Flow",
            Self::LineB => "T / Line B - Hard-currency ordering",
            Self::LineC => "M / Line C - Multi-theater atlas mapping",
        }
    }

    pub const fn promoted_from(self) -> &'static str {
        match self {
            Self::LineA => "E-11B / E-11B-5",
            Self::LineB => "D-2 / D-2a",
            Self::LineC => "M-4 / M-4A",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum V78LineScenario {
    NestedResourceFlowDepthFanout,
    HardCurrencyContentionOrdering,
    MultiTheaterAtlasMapping,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum V78LineGateStatus {
    Parked,
    NamedScenarioProposed,
    NamedScenarioAccepted,
    ImplementationAuthorized,
    Implemented,
    Accepted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78LineScenarioPack {
    pub pack_id: String,
    pub clause_spec_layer: String,
    pub scenarios: Vec<V78NamedConsumerScenario>,
}

impl V78LineScenarioPack {
    pub fn scenario_for_line(&self, line: V78PromotedLine) -> Option<&V78NamedConsumerScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.promoted_line == line)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78NamedConsumerScenario {
    pub scenario_id: String,
    pub promoted_line: V78PromotedLine,
    pub scenario: V78LineScenario,
    pub status: V78LineGateStatus,
    pub named_scenario_condition_satisfied: bool,
    pub implementation_authorized: bool,
    pub first_implementation_gate_after_acceptance: String,
    pub still_rejected_until_acceptance: Vec<String>,
    pub claim: V78LineScenarioClaim,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum V78LineScenarioClaim {
    NestedResourceFlowDepthFanout(V78NestedResourceFlowDepthFanoutClaim),
    HardCurrencyContentionOrdering(V78HardCurrencyContentionOrderingClaim),
    MultiTheaterAtlasMapping(V78MultiTheaterAtlasMappingClaim),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78NestedResourceFlowDepthFanoutClaim {
    pub faction_count: u32,
    pub planet_count: u32,
    pub district_count: u32,
    pub factory_count: u32,
    pub depth_required: u32,
    pub flat_star_insufficient: bool,
    pub requires_nested_resource_flow: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78HardCurrencyContentionOrderingClaim {
    pub multi_transaction_workload: bool,
    pub requires_sequential_cross_band_ordering: bool,
    pub discrete_accumulator_path_insufficient_at_scale: bool,
    pub contention_scale_declared: bool,
    pub boundary_or_hot_pool_contention_declared: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78MultiTheaterAtlasMappingClaim {
    pub theater_count: u32,
    pub single_32x32_theater_insufficient: bool,
    pub requires_atlas_batching: bool,
    pub vram_budget_declared: bool,
    pub vram_budget: V78AtlasVramBudget,
    pub preferred_isolation: String,
    pub fallback_isolation: String,
    pub requires_full_tile_protocol_oracle_parity: bool,
}

/// 1.5 GiB — the default C-0 atlas VRAM ceiling (commodity-GPU starting budget).
pub const V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES: u64 = 1_610_612_736;

/// VRAM budget term for the Line C atlas gate (`C-0`), set by design authority/product.
///
/// The default ceiling is **1.5 GiB** so the first multi-theater slice fits commodity GPUs,
/// but the budget is **configurable** with **no architectural hard cap**: dedicated/headless
/// servers and larger-VRAM cards raise `max_bytes` far beyond the default. VRAM-multiplier
/// reporting is mandatory (algebraic tile-local mask G=0 ≈ 1.0×; physical gutter G≥H ≈ 6.76×),
/// so admitted atlas occupancy is always checked against the *active* budget, not a constant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct V78AtlasVramBudget {
    /// Active ceiling in bytes. Default `V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES` (1.5 GiB); a
    /// deployment profile may set this far higher.
    pub max_bytes: u64,
    /// The budget is a profile/config parameter, raised for bigger-VRAM / headless deployments.
    pub configurable: bool,
    /// No fixed architectural ceiling; only the active (raisable) `max_bytes` binds.
    pub architectural_hard_cap: bool,
    /// VRAM-multiplier reporting against the active budget is mandatory (admission + runtime).
    pub multiplier_reporting_required: bool,
}

impl V78AtlasVramBudget {
    /// Default C-0 budget: 1.5 GiB ceiling, configurable, no architectural hard cap, reporting on.
    pub fn default_1p5_gib() -> Self {
        Self {
            max_bytes: V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
            configurable: true,
            architectural_hard_cap: false,
            multiplier_reporting_required: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V78LineScenarioPackAdmission {
    pub pack_id: String,
    pub admitted: bool,
    pub line_statuses: Vec<V78LineScenarioStatusRecord>,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct V78LineScenarioStatusRecord {
    pub promoted_line: V78PromotedLine,
    pub scenario: V78LineScenario,
    pub status: V78LineGateStatus,
    pub implementation_authorized: bool,
    pub still_rejected_until_acceptance: Vec<String>,
}

pub fn v7_8_met_consumer_scenario_pack() -> V78LineScenarioPack {
    V78LineScenarioPack {
        pack_id: V78_MET_SCENARIO_PACK_ID.into(),
        clause_spec_layer: "simthing-spec designer_admission / accepted CLAUSE-SPEC substrate"
            .into(),
        scenarios: vec![
            V78NamedConsumerScenario {
                scenario_id: "v7_8_line_a_nested_resource_flow_depth_fanout".into(),
                promoted_line: V78PromotedLine::LineA,
                scenario: V78LineScenario::NestedResourceFlowDepthFanout,
                status: V78LineGateStatus::NamedScenarioProposed,
                named_scenario_condition_satisfied: true,
                implementation_authorized: false,
                first_implementation_gate_after_acceptance:
                    "A-0 nested-arena first slice, not full production default-on Resource Flow"
                        .into(),
                still_rejected_until_acceptance: vec![
                    DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario
                        .as_str()
                        .into(),
                    DesignerAdmissionDiagnosticCode::E11B5RequestedWithoutNamedScenario
                        .as_str()
                        .into(),
                    DesignerAdmissionDiagnosticCode::DefaultOnRejected
                        .as_str()
                        .into(),
                ],
                claim: V78LineScenarioClaim::NestedResourceFlowDepthFanout(
                    V78NestedResourceFlowDepthFanoutClaim {
                        faction_count: 1,
                        planet_count: 100,
                        district_count: 1000,
                        factory_count: 100000,
                        depth_required: 4,
                        flat_star_insufficient: true,
                        requires_nested_resource_flow: true,
                    },
                ),
            },
            V78NamedConsumerScenario {
                scenario_id: "v7_8_line_b_hard_currency_contention_ordering".into(),
                promoted_line: V78PromotedLine::LineB,
                scenario: V78LineScenario::HardCurrencyContentionOrdering,
                status: V78LineGateStatus::NamedScenarioProposed,
                named_scenario_condition_satisfied: true,
                implementation_authorized: false,
                first_implementation_gate_after_acceptance:
                    "B-0 narrow driver-only D-2a slice, not a global hard-currency scheduler"
                        .into(),
                still_rejected_until_acceptance: vec![
                    DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario
                        .as_str()
                        .into(),
                    DesignerAdmissionDiagnosticCode::ResourceFlowBypassRejected
                        .as_str()
                        .into(),
                ],
                claim: V78LineScenarioClaim::HardCurrencyContentionOrdering(
                    V78HardCurrencyContentionOrderingClaim {
                        multi_transaction_workload: true,
                        requires_sequential_cross_band_ordering: true,
                        discrete_accumulator_path_insufficient_at_scale: true,
                        contention_scale_declared: true,
                        boundary_or_hot_pool_contention_declared: true,
                    },
                ),
            },
            V78NamedConsumerScenario {
                scenario_id: "v7_8_line_c_multi_theater_atlas_mapping".into(),
                promoted_line: V78PromotedLine::LineC,
                scenario: V78LineScenario::MultiTheaterAtlasMapping,
                status: V78LineGateStatus::NamedScenarioProposed,
                named_scenario_condition_satisfied: true,
                implementation_authorized: false,
                first_implementation_gate_after_acceptance:
                    "C-0 first Section 11-gate M-4 slice after scenario, VRAM budget, and M-4 PR approval"
                        .into(),
                still_rejected_until_acceptance: vec![
                    DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate
                        .as_str()
                        .into(),
                    DesignerAdmissionDiagnosticCode::ActiveMaskRequestedWithoutGate
                        .as_str()
                        .into(),
                    DesignerAdmissionDiagnosticCode::SourceIdentityRequestedWithoutGate
                        .as_str()
                        .into(),
                ],
                claim: V78LineScenarioClaim::MultiTheaterAtlasMapping(
                    V78MultiTheaterAtlasMappingClaim {
                        theater_count: 4,
                        single_32x32_theater_insufficient: true,
                        requires_atlas_batching: true,
                        vram_budget_declared: true,
                        vram_budget: V78AtlasVramBudget::default_1p5_gib(),
                        preferred_isolation: "AlgebraicTileLocalMaskG0".into(),
                        fallback_isolation: "PhysicalGutterGteH".into(),
                        requires_full_tile_protocol_oracle_parity: true,
                    },
                ),
            },
        ],
    }
}

pub fn admit_v7_8_line_scenario_pack(
    pack: &V78LineScenarioPack,
) -> V78LineScenarioPackAdmission {
    let mut diagnostics = Vec::new();

    if pack.pack_id.trim().is_empty() {
        diagnostics.push(malformed("scenario pack id must be non-empty"));
    }
    for line in [
        V78PromotedLine::LineA,
        V78PromotedLine::LineB,
        V78PromotedLine::LineC,
    ] {
        match pack.scenario_for_line(line) {
            Some(scenario) => validate_line_scenario(scenario, &mut diagnostics),
            None => diagnostics.push(malformed(format!(
                "missing named consumer scenario for {}",
                line.capability_label()
            ))),
        }
    }

    let line_statuses = pack
        .scenarios
        .iter()
        .map(|scenario| V78LineScenarioStatusRecord {
            promoted_line: scenario.promoted_line,
            scenario: scenario.scenario,
            status: scenario.status,
            implementation_authorized: scenario.implementation_authorized,
            still_rejected_until_acceptance: scenario.still_rejected_until_acceptance.clone(),
        })
        .collect::<Vec<_>>();

    V78LineScenarioPackAdmission {
        pack_id: pack.pack_id.clone(),
        admitted: diagnostics.is_empty(),
        line_statuses,
        diagnostics,
    }
}

fn validate_line_scenario(
    scenario: &V78NamedConsumerScenario,
    diagnostics: &mut Vec<DesignerAdmissionDiagnostic>,
) {
    if scenario.scenario_id.trim().is_empty() {
        diagnostics.push(malformed("line scenario id must be non-empty"));
    }
    if scenario.status != V78LineGateStatus::NamedScenarioProposed {
        diagnostics.push(malformed(
            "V7.8-MET-SCENARIO-0 may only propose named scenarios",
        ));
    }
    if scenario.implementation_authorized {
        diagnostics.push(malformed(
            "named consumer scenarios do not authorize implementation",
        ));
    }
    if !scenario.named_scenario_condition_satisfied {
        diagnostics.push(malformed(
            "scenario must name the constitutional need before becoming a gate candidate",
        ));
    }

    match (&scenario.promoted_line, &scenario.scenario, &scenario.claim) {
        (
            V78PromotedLine::LineA,
            V78LineScenario::NestedResourceFlowDepthFanout,
            V78LineScenarioClaim::NestedResourceFlowDepthFanout(claim),
        ) => {
            if claim.depth_required <= 2
                || !claim.flat_star_insufficient
                || !claim.requires_nested_resource_flow
            {
                diagnostics.push(malformed(
                    "Line A scenario must require depth > 2 nested Resource Flow",
                ));
            }
        }
        (
            V78PromotedLine::LineB,
            V78LineScenario::HardCurrencyContentionOrdering,
            V78LineScenarioClaim::HardCurrencyContentionOrdering(claim),
        ) => {
            if !claim.multi_transaction_workload
                || !claim.requires_sequential_cross_band_ordering
                || !claim.discrete_accumulator_path_insufficient_at_scale
                || !claim.contention_scale_declared
                || !claim.boundary_or_hot_pool_contention_declared
            {
                diagnostics.push(malformed(
                    "Line B scenario must require sequential hard-currency ordering",
                ));
            }
        }
        (
            V78PromotedLine::LineC,
            V78LineScenario::MultiTheaterAtlasMapping,
            V78LineScenarioClaim::MultiTheaterAtlasMapping(claim),
        ) => {
            if claim.theater_count <= 1
                || !claim.single_32x32_theater_insufficient
                || !claim.requires_atlas_batching
                || !claim.vram_budget_declared
                || !claim.requires_full_tile_protocol_oracle_parity
            {
                diagnostics.push(malformed(
                    "Line C scenario must require multi-theater atlas batching and VRAM review",
                ));
            }
            // VRAM budget must be a real, configurable term with no architectural hard cap and
            // mandatory multiplier reporting (default 1.5 GiB; raisable for headless/big-VRAM).
            if claim.vram_budget.max_bytes == 0
                || !claim.vram_budget.configurable
                || claim.vram_budget.architectural_hard_cap
                || !claim.vram_budget.multiplier_reporting_required
            {
                diagnostics.push(malformed(
                    "Line C VRAM budget must be a configurable, hard-cap-free term with multiplier reporting",
                ));
            }
        }
        _ => diagnostics.push(malformed(
            "line, scenario, and claim kind must match the v7.8 promoted-line mapping",
        )),
    }
}

fn malformed(message: impl Into<String>) -> DesignerAdmissionDiagnostic {
    designer_admission_diagnostic(
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
        message,
        Some("V7.8-MET-SCENARIO-0 names gate candidates only; implementation remains parked"),
    )
}
