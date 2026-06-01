//! MOBILITY-SCENARIO-0: v7.9 mobility/transfer scenario admission packet.
//!
//! This module names and bounds the first parked v7.9 mobility scenario. It is
//! metadata/admission only: no allocator, reparenting, routing, economy,
//! owner-overlay, GPU kernel, or production `SimSession` wiring is created here.

use serde::{Deserialize, Serialize};

use super::diagnostic::{
    designer_admission_diagnostic, designer_admission_diagnostic_for_rejection,
    DesignerAdmissionDiagnostic, DesignerAdmissionDiagnosticCode,
};

pub const MOBILITY_SCENARIO0_ID: &str = "mobility_scenario0_v7_9_first_slice";
pub const MOBILITY_SCENARIO0_ENTITY_TARGET: u32 = 34_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityScenario0Status {
    ScenarioAdmissionProposed,
    ScenarioAccepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityTheaterScale {
    SingleTheaterMultiCell,
    MultiSystem,
    SectorScale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityRoutingMode {
    Adversarial,
    Cooperative,
    Directed,
    ArgmaxTriage,
    Proportional,
    NarrowedAdversarialFirstSlice,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityOwnerRelationKind {
    Faction,
    Species,
    Blueprint,
    Policy,
    Tech,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityOwnerRelationDiscipline {
    FlowPooling,
    DownBroadcastOverlay,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityScenario0Packet {
    pub scenario_id: String,
    pub status: MobilityScenario0Status,
    #[serde(default)]
    pub implementation_authorized: bool,
    #[serde(default)]
    pub enabled_by_default: bool,
    pub theater: MobilityTheaterShape,
    pub identity_channels: MobilityIdentityChannelBudget,
    pub allocation: MobilityAllocationBounds,
    pub identity_boundary: MobilityIdentityBoundary,
    #[serde(default)]
    pub owner_columns: Vec<MobilityOwnerColumn>,
    pub quantity_classes: MobilityQuantityClasses,
    pub supply_scope: MobilitySupplyScope,
    pub blockade: MobilityBlockadeSemantics,
    pub routing: MobilityRoutingPolicy,
    pub soak: MobilitySoakProfile,
    #[serde(default)]
    pub guardrails: MobilityScenario0GuardrailRequests,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityTheaterShape {
    pub sectors: u32,
    pub systems: u32,
    pub cells: u32,
    pub spatial_depth: u32,
    pub scale: MobilityTheaterScale,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityIdentityChannelBudget {
    pub max_factions_per_cell: u32,
    pub local_identity_channels: u32,
    pub routing_eml_node_budget: u32,
    pub first_slice_expected_peak_factions_per_cell: u32,
    pub sufficiency_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityAllocationBounds {
    pub max_fleet_density_per_cell: u32,
    pub moving_entity_block_size: u32,
    pub reserved_headroom_per_cell: u32,
    pub overflow_rejects_or_narrows: bool,
    pub slab_block_first: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityIdentityBoundary {
    pub simthing_slots: Vec<String>,
    pub count_columns: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityOwnerColumn {
    pub relation: MobilityOwnerRelationKind,
    pub column: String,
    pub discipline: MobilityOwnerRelationDiscipline,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityQuantityClasses {
    pub hard_fixed_point_band_alpha: Vec<String>,
    pub soft_float_band_beta: Vec<String>,
    pub hard_and_soft_never_silently_mix: bool,
    pub float_values_gate_structural_transitions: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilitySupplyScope {
    pub sector_cell_edges_are_resource_flow_couplings: bool,
    pub sector_cell_edges_are_spatial_structure: bool,
    pub subsidiarity_balance_depth: String,
    pub default_on_resource_flow: bool,
    pub hard_currency_routes_through_resource_flow: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityBlockadeSemantics {
    pub cut_flows: Vec<String>,
    pub blockade_immune_overlays: Vec<String>,
    pub cpu_planner: bool,
    pub cpu_urgency: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityRoutingPolicy {
    pub mode: MobilityRoutingMode,
    pub identity_is_column_not_tree: bool,
    pub uses_arrival_order_as_replay_ordering: bool,
    pub silent_hybrid_strata_rebind: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilitySoakProfile {
    pub entity_count: u32,
    pub churn_rate_per_boundary_bps: u32,
    pub movement_rate_per_boundary_bps: u32,
    pub capture_cadence_boundaries: u32,
    pub unlock_cadence_boundaries: u32,
    pub stress_mix: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobilityScenario0GuardrailRequests {
    pub owner_entities_as_spatial_parents: bool,
    pub capture_as_reparenting: bool,
    pub semantic_wgsl: bool,
    pub gpu_allocator_semaphore: bool,
    pub indirection_buffer_before_slab: bool,
    pub reopen_clausething_l3: bool,
    pub reopen_closed_ladders: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityScenario0Admission {
    pub scenario_id: String,
    pub admitted: bool,
    pub implementation_authorized: bool,
    pub status: MobilityScenario0Status,
    pub parameter_summary: MobilityScenario0ParameterSummary,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityScenario0ParameterSummary {
    pub theater_scale: MobilityTheaterScale,
    pub spatial_depth: u32,
    pub cells: u32,
    pub max_factions_per_cell: u32,
    pub routing_eml_node_budget: u32,
    pub moving_entity_block_size: u32,
    pub simthing_slot_kinds: Vec<String>,
    pub count_columns: Vec<String>,
    pub owner_column_count: u32,
    pub hard_quantity_count: u32,
    pub soft_quantity_count: u32,
    pub soak_entity_count: u32,
}

pub fn mobility_scenario0_packet() -> MobilityScenario0Packet {
    MobilityScenario0Packet {
        scenario_id: MOBILITY_SCENARIO0_ID.into(),
        status: MobilityScenario0Status::ScenarioAdmissionProposed,
        implementation_authorized: false,
        enabled_by_default: false,
        theater: MobilityTheaterShape {
            sectors: 1,
            systems: 3,
            cells: 48,
            spatial_depth: 4,
            scale: MobilityTheaterScale::SingleTheaterMultiCell,
        },
        identity_channels: MobilityIdentityChannelBudget {
            max_factions_per_cell: 4,
            local_identity_channels: 4,
            routing_eml_node_budget: 16,
            first_slice_expected_peak_factions_per_cell: 3,
            sufficiency_note:
                "First slice covers two-faction battles plus one transient third-party/neutral cell occupant; cap 4 matches the local Hybrid Strata channel count."
                    .into(),
        },
        allocation: MobilityAllocationBounds {
            max_fleet_density_per_cell: 64,
            moving_entity_block_size: 96,
            reserved_headroom_per_cell: 32,
            overflow_rejects_or_narrows: true,
            slab_block_first: true,
        },
        identity_boundary: MobilityIdentityBoundary {
            simthing_slots: vec![
                "cell".into(),
                "fleet".into(),
                "ship_class_cohort".into(),
                "pop_cohort".into(),
            ],
            count_columns: vec![
                "fighter_count".into(),
                "fighter_hp_pool".into(),
                "population_count".into(),
            ],
            examples: vec![
                "fleet is a SimThing slot; individual fighters are count columns on ship_class_cohort".into(),
                "pop cohort is a SimThing slot; population members are a count column".into(),
            ],
        },
        owner_columns: vec![
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Faction,
                column: "faction_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::FlowPooling,
            },
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Species,
                column: "species_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::DownBroadcastOverlay,
            },
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Blueprint,
                column: "blueprint_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::DownBroadcastOverlay,
            },
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Tech,
                column: "tech_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::DownBroadcastOverlay,
            },
        ],
        quantity_classes: MobilityQuantityClasses {
            hard_fixed_point_band_alpha: vec![
                "hard_currency".into(),
                "munition_stock".into(),
                "supply_balance_test".into(),
            ],
            soft_float_band_beta: vec![
                "damage_rate".into(),
                "repair_rate".into(),
                "morale_pressure".into(),
            ],
            hard_and_soft_never_silently_mix: true,
            float_values_gate_structural_transitions: false,
        },
        supply_scope: MobilitySupplyScope {
            sector_cell_edges_are_resource_flow_couplings: false,
            sector_cell_edges_are_spatial_structure: true,
            subsidiarity_balance_depth:
                "balance at the lowest spatial node where hard Band Alpha supply covers demand; escalate residual only"
                    .into(),
            default_on_resource_flow: false,
            hard_currency_routes_through_resource_flow: false,
        },
        blockade: MobilityBlockadeSemantics {
            cut_flows: vec!["per_tick_supply".into(), "munitions_resupply".into()],
            blockade_immune_overlays: vec![
                "species_trait_modifier".into(),
                "tech_modifier".into(),
                "policy_modifier".into(),
            ],
            cpu_planner: false,
            cpu_urgency: false,
        },
        routing: MobilityRoutingPolicy {
            mode: MobilityRoutingMode::NarrowedAdversarialFirstSlice,
            identity_is_column_not_tree: true,
            uses_arrival_order_as_replay_ordering: false,
            silent_hybrid_strata_rebind: false,
        },
        soak: MobilitySoakProfile {
            entity_count: MOBILITY_SCENARIO0_ENTITY_TARGET,
            churn_rate_per_boundary_bps: 75,
            movement_rate_per_boundary_bps: 250,
            capture_cadence_boundaries: 20,
            unlock_cadence_boundaries: 50,
            stress_mix: vec![
                "ALLOC: pop/fleet churn with bounded block headroom".into(),
                "REENROLL: burst fleet moves between cells".into(),
                "IDROUTE: contested cells at k<=4 identities".into(),
                "ECON: blockaded supply residuals".into(),
                "OWNER: DirtyOnly modifier refresh on capture/unlock".into(),
            ],
        },
        guardrails: MobilityScenario0GuardrailRequests::default(),
    }
}

pub fn admit_mobility_scenario0_packet(
    packet: &MobilityScenario0Packet,
) -> MobilityScenario0Admission {
    let mut diagnostics = Vec::new();
    validate_packet(packet, &mut diagnostics);

    MobilityScenario0Admission {
        scenario_id: packet.scenario_id.clone(),
        admitted: diagnostics.is_empty(),
        implementation_authorized: packet.implementation_authorized,
        status: packet.status,
        parameter_summary: MobilityScenario0ParameterSummary {
            theater_scale: packet.theater.scale,
            spatial_depth: packet.theater.spatial_depth,
            cells: packet.theater.cells,
            max_factions_per_cell: packet.identity_channels.max_factions_per_cell,
            routing_eml_node_budget: packet.identity_channels.routing_eml_node_budget,
            moving_entity_block_size: packet.allocation.moving_entity_block_size,
            simthing_slot_kinds: packet.identity_boundary.simthing_slots.clone(),
            count_columns: packet.identity_boundary.count_columns.clone(),
            owner_column_count: packet.owner_columns.len() as u32,
            hard_quantity_count: packet.quantity_classes.hard_fixed_point_band_alpha.len() as u32,
            soft_quantity_count: packet.quantity_classes.soft_float_band_beta.len() as u32,
            soak_entity_count: packet.soak.entity_count,
        },
        diagnostics,
    }
}

fn validate_packet(
    packet: &MobilityScenario0Packet,
    diagnostics: &mut Vec<DesignerAdmissionDiagnostic>,
) {
    if packet.scenario_id.trim().is_empty() {
        diagnostics.push(malformed("MOBILITY-SCENARIO-0 id must be non-empty"));
    }
    if packet.status != MobilityScenario0Status::ScenarioAdmissionProposed {
        diagnostics.push(malformed(
            "MOBILITY-SCENARIO-0 lands scenario/admission metadata only; acceptance is a later design-authority/product ruling",
        ));
    }
    if packet.implementation_authorized {
        diagnostics.push(malformed(
            "MOBILITY-SCENARIO-0 does not authorize implementation",
        ));
    }
    if packet.enabled_by_default {
        diagnostics.push(diag(DesignerAdmissionDiagnosticCode::DefaultOnRejected));
    }

    validate_bounds(packet, diagnostics);
    validate_required_guardrails(packet, diagnostics);
}

fn validate_bounds(
    packet: &MobilityScenario0Packet,
    diagnostics: &mut Vec<DesignerAdmissionDiagnostic>,
) {
    if packet.theater.cells == 0 || packet.theater.systems == 0 || packet.theater.spatial_depth < 2
    {
        diagnostics.push(malformed(
            "theater must declare non-zero systems/cells and spatial depth >= 2",
        ));
    }
    if packet.identity_channels.local_identity_channels
        != packet.identity_channels.max_factions_per_cell
    {
        diagnostics.push(malformed(
            "local identity channel count must equal max_factions_per_cell",
        ));
    }
    if packet
        .identity_channels
        .first_slice_expected_peak_factions_per_cell
        > packet.identity_channels.max_factions_per_cell
    {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityMaxFactionsPerCellExceeded,
        ));
    }
    let required_nodes = packet
        .identity_channels
        .max_factions_per_cell
        .saturating_mul(4);
    if packet.identity_channels.routing_eml_node_budget < required_nodes {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityRoutingEmlNodeBudgetExceeded,
        ));
    }
    if packet.allocation.max_fleet_density_per_cell == 0
        || packet.allocation.moving_entity_block_size == 0
        || packet.allocation.moving_entity_block_size < packet.allocation.max_fleet_density_per_cell
        || !packet.allocation.overflow_rejects_or_narrows
        || !packet.allocation.slab_block_first
    {
        diagnostics.push(malformed(
            "allocation bounds must use slab/block-first reservation with visible overflow narrowing/rejection",
        ));
    }
    if packet.identity_boundary.simthing_slots.is_empty()
        || packet.identity_boundary.count_columns.is_empty()
    {
        diagnostics.push(malformed(
            "identity boundary must declare SimThing slots and count columns",
        ));
    }
    if !packet
        .owner_columns
        .iter()
        .any(|owner| owner.discipline == MobilityOwnerRelationDiscipline::FlowPooling)
        || !packet
            .owner_columns
            .iter()
            .any(|owner| owner.discipline == MobilityOwnerRelationDiscipline::DownBroadcastOverlay)
    {
        diagnostics.push(malformed(
            "owner columns must include both flow-pooling and down-broadcast overlay relations",
        ));
    }
    if !packet.quantity_classes.hard_and_soft_never_silently_mix {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityHardSoftMixedPassRejected,
        ));
    }
    if packet
        .quantity_classes
        .float_values_gate_structural_transitions
    {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityFloatStructuralGateRejected,
        ));
    }
    if packet.supply_scope.default_on_resource_flow {
        diagnostics.push(diag(DesignerAdmissionDiagnosticCode::DefaultOnRejected));
    }
    if packet
        .supply_scope
        .hard_currency_routes_through_resource_flow
    {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityHardCurrencyThroughResourceFlowRejected,
        ));
    }
    if packet.blockade.cpu_planner {
        diagnostics.push(diag(DesignerAdmissionDiagnosticCode::CpuPlannerRejected));
    }
    if packet.blockade.cpu_urgency {
        diagnostics.push(diag(DesignerAdmissionDiagnosticCode::CpuUrgencyRejected));
    }
    if !packet.routing.identity_is_column_not_tree {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityOwnerSpatialParentRejected,
        ));
    }
    if packet.routing.uses_arrival_order_as_replay_ordering {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityArrivalOrderReplayOrderingRejected,
        ));
    }
    if packet.routing.silent_hybrid_strata_rebind {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityHybridStrataSilentRebindRejected,
        ));
    }
    if packet.soak.entity_count != MOBILITY_SCENARIO0_ENTITY_TARGET {
        diagnostics.push(malformed(
            "MOBILITY-SCENARIO-0 must record the 34k soak target",
        ));
    }
}

fn validate_required_guardrails(
    packet: &MobilityScenario0Packet,
    diagnostics: &mut Vec<DesignerAdmissionDiagnostic>,
) {
    if packet.guardrails.owner_entities_as_spatial_parents {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityOwnerSpatialParentRejected,
        ));
    }
    if packet.guardrails.capture_as_reparenting {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityCaptureAsReparentingRejected,
        ));
    }
    if packet.guardrails.semantic_wgsl {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected,
        ));
    }
    if packet.guardrails.gpu_allocator_semaphore {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityGpuAllocatorSemaphoreRejected,
        ));
    }
    if packet.guardrails.indirection_buffer_before_slab {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityIndirectionBeforeSlabRejected,
        ));
    }
    if packet.guardrails.reopen_clausething_l3 {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::ClauseThingRuntimeRequestParked,
        ));
    }
    if packet.guardrails.reopen_closed_ladders {
        diagnostics.push(diag(
            DesignerAdmissionDiagnosticCode::MobilityClosedLadderReopenRejected,
        ));
    }
}

fn diag(code: DesignerAdmissionDiagnosticCode) -> DesignerAdmissionDiagnostic {
    designer_admission_diagnostic_for_rejection(code.rejection_kind())
}

fn malformed(message: impl Into<String>) -> DesignerAdmissionDiagnostic {
    designer_admission_diagnostic(
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
        message,
        Some("provide a bounded MOBILITY-SCENARIO-0 metadata packet; do not authorize implementation"),
    )
}
