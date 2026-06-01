//! MOBILITY-AUDIT-0: owner OrderBand depth budget for MOBILITY-SCENARIO-0.
//!
//! This module is audit/modeling only. It does not implement allocator,
//! re-enrollment, routing, economy, owner-overlay, GPU, or `SimSession` runtime
//! behavior.

use super::mobility_scenario0::{
    MobilityOwnerRelationDiscipline, MobilityRoutingMode, MobilityScenario0Packet,
    MOBILITY_SCENARIO0_ENTITY_TARGET,
};

pub const MOBILITY_AUDIT0_ID: &str = "mobility_audit0_owner_band_budget";
pub const MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH: u32 = 16;
pub const MOBILITY_AUDIT0_NARROWING_CEILING: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobilityAudit0Verdict {
    Pass,
    PassWithNarrowing,
    FailBlocked,
}

impl MobilityAudit0Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            MobilityAudit0Verdict::Pass => "PASS",
            MobilityAudit0Verdict::PassWithNarrowing => "PASS WITH NARROWING",
            MobilityAudit0Verdict::FailBlocked => "FAIL-BLOCKED",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobilityAudit0CirculationFamily {
    ModifierDown,
    EconomyUp,
    EconomyDown,
    ResearchUp,
    Thresholds,
    HardFixedPointBandAlpha,
    SoftFloatBandBeta,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAudit0FamilyBudget {
    pub family: MobilityAudit0CirculationFamily,
    pub required_bands: u32,
    pub starts_after_alpha: bool,
    pub note: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAudit0ScenarioConstants {
    pub scenario_id: String,
    pub routing: MobilityRoutingMode,
    pub spatial_depth: u32,
    pub max_factions_per_cell: u32,
    pub routing_eml_node_budget: u32,
    pub theater_cells: u32,
    pub soak_entities: u32,
    pub has_faction_flow_pooling: bool,
    pub has_down_broadcast_overlays: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityAudit0Report {
    pub audit_id: &'static str,
    pub verdict: MobilityAudit0Verdict,
    pub required_orderband_depth: u32,
    pub max_orderband_depth: u32,
    pub slack_orderbands: i32,
    pub family_budgets: Vec<MobilityAudit0FamilyBudget>,
    pub scenario_constants: MobilityAudit0ScenarioConstants,
    pub alpha_precedes_beta: bool,
    pub hard_soft_silent_mix: bool,
    pub owner_spatial_parent_assumption: bool,
    pub alloc_reenroll_idroute_econ_owner_parked: bool,
    pub runtime_implementation_authorized: bool,
    pub narrowing: Option<&'static str>,
}

pub fn audit_mobility_owner_band_budget(packet: &MobilityScenario0Packet) -> MobilityAudit0Report {
    audit_mobility_owner_band_budget_with_ceiling(
        packet,
        MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH,
    )
}

pub fn audit_mobility_owner_band_budget_with_ceiling(
    packet: &MobilityScenario0Packet,
    max_orderband_depth: u32,
) -> MobilityAudit0Report {
    let family_budgets = mobility_audit0_family_budgets(packet.theater.spatial_depth);
    let required_orderband_depth = family_budgets
        .iter()
        .map(|budget| budget.required_bands)
        .sum::<u32>();
    let slack_orderbands = max_orderband_depth as i32 - required_orderband_depth as i32;
    let narrowed_required = required_orderband_depth_at_spatial_depth(3);
    let verdict = if required_orderband_depth <= max_orderband_depth {
        MobilityAudit0Verdict::Pass
    } else if max_orderband_depth >= MOBILITY_AUDIT0_NARROWING_CEILING
        && narrowed_required <= max_orderband_depth
    {
        MobilityAudit0Verdict::PassWithNarrowing
    } else {
        MobilityAudit0Verdict::FailBlocked
    };

    MobilityAudit0Report {
        audit_id: MOBILITY_AUDIT0_ID,
        verdict,
        required_orderband_depth,
        max_orderband_depth,
        slack_orderbands,
        family_budgets,
        scenario_constants: MobilityAudit0ScenarioConstants {
            scenario_id: packet.scenario_id.clone(),
            routing: packet.routing.mode,
            spatial_depth: packet.theater.spatial_depth,
            max_factions_per_cell: packet.identity_channels.max_factions_per_cell,
            routing_eml_node_budget: packet.identity_channels.routing_eml_node_budget,
            theater_cells: packet.theater.cells,
            soak_entities: packet.soak.entity_count,
            has_faction_flow_pooling: packet
                .owner_columns
                .iter()
                .any(|owner| owner.discipline == MobilityOwnerRelationDiscipline::FlowPooling),
            has_down_broadcast_overlays: packet.owner_columns.iter().any(|owner| {
                owner.discipline == MobilityOwnerRelationDiscipline::DownBroadcastOverlay
            }),
        },
        alpha_precedes_beta: true,
        hard_soft_silent_mix: !packet.quantity_classes.hard_and_soft_never_silently_mix,
        owner_spatial_parent_assumption: packet.guardrails.owner_entities_as_spatial_parents,
        alloc_reenroll_idroute_econ_owner_parked: true,
        runtime_implementation_authorized: false,
        narrowing: match verdict {
            MobilityAudit0Verdict::Pass => None,
            MobilityAudit0Verdict::PassWithNarrowing => {
                Some("narrow spatial depth to 3 before owner/economy implementation")
            }
            MobilityAudit0Verdict::FailBlocked => {
                Some("requires separate OrderBand-depth expansion scenario")
            }
        },
    }
}

pub fn mobility_audit0_required_orderband_depth(packet: &MobilityScenario0Packet) -> u32 {
    required_orderband_depth_at_spatial_depth(packet.theater.spatial_depth)
}

pub fn mobility_audit0_family_budgets(spatial_depth: u32) -> Vec<MobilityAudit0FamilyBudget> {
    let spine_bands = spatial_depth.saturating_sub(1);
    vec![
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::ModifierDown,
            required_bands: 1,
            starts_after_alpha: false,
            note: "blockade-immune owner modifier down-broadcast overlay refresh",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::HardFixedPointBandAlpha,
            required_bands: 1,
            starts_after_alpha: false,
            note: "hard fixed-point quantities settle before any soft float reads",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::EconomyUp,
            required_bands: spine_bands,
            starts_after_alpha: true,
            note: "spatial-depth up-sweep over the accepted D=4 spine",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::EconomyDown,
            required_bands: spine_bands,
            starts_after_alpha: true,
            note: "spatial-depth down-broadcast over the accepted D=4 spine",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::ResearchUp,
            required_bands: spine_bands,
            starts_after_alpha: true,
            note: "owner research/progress aggregation up the same accepted spine",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::Thresholds,
            required_bands: 1,
            starts_after_alpha: true,
            note: "threshold evaluation after hard economy/read dependencies settle",
        },
        MobilityAudit0FamilyBudget {
            family: MobilityAudit0CirculationFamily::SoftFloatBandBeta,
            required_bands: 1,
            starts_after_alpha: true,
            note: "soft float quantities read settled hard Band Alpha state",
        },
    ]
}

fn required_orderband_depth_at_spatial_depth(spatial_depth: u32) -> u32 {
    mobility_audit0_family_budgets(spatial_depth)
        .iter()
        .map(|budget| budget.required_bands)
        .sum()
}

pub fn mobility_audit0_packet_matches_accepted_constants(packet: &MobilityScenario0Packet) -> bool {
    packet.routing.mode == MobilityRoutingMode::NarrowedAdversarialFirstSlice
        && packet.theater.spatial_depth == 4
        && packet.identity_channels.max_factions_per_cell == 4
        && packet.identity_channels.routing_eml_node_budget == 16
        && packet.theater.cells == 48
        && packet.soak.entity_count == MOBILITY_SCENARIO0_ENTITY_TARGET
}
