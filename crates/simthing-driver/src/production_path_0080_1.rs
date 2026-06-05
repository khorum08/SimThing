use crate::{
    run_atlas_0080_0, run_econ_scale_0080_0, Atlas0080Input, Atlas0080Report, Atlas0080TheaterId,
    EconScale0080Faction, EconScale0080Input, EconScale0080RunReport,
    ATLAS_0080_0_LOGICAL_LOCATION_COUNT, ATLAS_0080_0_PLANET_SIDE, ATLAS_0080_0_STARMAP_SIDE,
    ATLAS_0080_0_STARSYSTEM_COUNT, ATLAS_0080_0_STARSYSTEM_SIDE,
};

pub const PRODUCTION_PATH_0080_1_ID: &str = "PRODUCTION-PATH-0080-1";
pub const PRODUCTION_PATH_0080_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - opt-in Nested Starmap production-path composition";
pub const PRODUCTION_PATH_0080_1_SCENARIO: &str = "Nested Starmap";
pub const SCENARIO_0080_1_GATE_ID: &str = "SCENARIO-0080-1";

const TERRAN_OWNER_ID: u64 = 80_100;
const PIRATE_OWNER_ID: u64 = 80_200;
const TERRAN_POLICY_WEIGHT: i64 = 7;
const PIRATE_POLICY_WEIGHT: i64 = -5;
const TERRAN_OWNED_PLANETS: u32 = 6;
const NEUTRAL_STARSYSTEMS: u32 = 4;
const TERRAN_SHIPS: u32 = 3;
const PIRATE_SHIPS: u32 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0081Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl ProductionPath0081Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0081Surface {
    pub gate: ProductionPath0081Gate,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub schedule_observation_control_demo_0080_1: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
}

impl ProductionPath0081Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: ProductionPath0081Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0081ForbiddenRequests {
    pub schedule_observation_control_demo_0080_1: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub realtime_loop_or_ui: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub unbounded_factions: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub clausething_dependency: bool,
    pub simthing_spec_alteration: bool,
    pub invariant_edit: bool,
    pub passive_proof_wrapper: bool,
    pub general_production_path: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081Scenario {
    pub scenario_gate_id: &'static str,
    pub scenario_name: &'static str,
    pub starmap_side: u32,
    pub starsystem_count: usize,
    pub starsystem_side: u32,
    pub planets_per_starsystem: u32,
    pub planet_side: u32,
    pub logical_location_count: u32,
    pub terran_owned_planets: u32,
    pub neutral_starsystems: u32,
    pub terran_ships: u32,
    pub pirate_ships: u32,
}

impl ProductionPath0081Scenario {
    pub fn canonical() -> Self {
        Self {
            scenario_gate_id: SCENARIO_0080_1_GATE_ID,
            scenario_name: PRODUCTION_PATH_0080_1_SCENARIO,
            starmap_side: ATLAS_0080_0_STARMAP_SIDE,
            starsystem_count: ATLAS_0080_0_STARSYSTEM_COUNT,
            starsystem_side: ATLAS_0080_0_STARSYSTEM_SIDE,
            planets_per_starsystem: 1,
            planet_side: ATLAS_0080_0_PLANET_SIDE,
            logical_location_count: ATLAS_0080_0_LOGICAL_LOCATION_COUNT,
            terran_owned_planets: TERRAN_OWNED_PLANETS,
            neutral_starsystems: NEUTRAL_STARSYSTEMS,
            terran_ships: TERRAN_SHIPS,
            pirate_ships: PIRATE_SHIPS,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081Input {
    pub surface: ProductionPath0081Surface,
    pub scenario: ProductionPath0081Scenario,
    pub atlas_input: Atlas0080Input,
    pub econ_scale_input: EconScale0080Input,
    pub forbidden: ProductionPath0081ForbiddenRequests,
}

impl ProductionPath0081Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: ProductionPath0081Surface::default_simsession(),
            scenario: ProductionPath0081Scenario::canonical(),
            atlas_input: Atlas0080Input::default_simsession(),
            econ_scale_input: EconScale0080Input::default_simsession(),
            forbidden: ProductionPath0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: ProductionPath0081Surface::with_explicit_opt_in(),
            scenario: ProductionPath0081Scenario::canonical(),
            atlas_input: Atlas0080Input::explicit_opt_in(),
            econ_scale_input: EconScale0080Input::explicit_opt_in(),
            forbidden: ProductionPath0081ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081OwnerOverlaySummary {
    pub faction_owner_simthings_are_session_siblings: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub terran_owner_id: u64,
    pub pirate_owner_id: u64,
    pub location_owner_overlays_inherit_numeric_weights: bool,
    pub ship_owner_overlays_inherit_faction_weights: bool,
    pub terran_policy_weight: i64,
    pub pirate_policy_weight: i64,
    pub no_new_owner_substrate_opened: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081OwnershipAggregationSummary {
    pub derived_owner_overlay_summary: bool,
    pub planet_to_starsystem_up_aggregation: bool,
    pub terran_owned_planets: u32,
    pub terran_owned_starsystems_derived: u32,
    pub neutral_starsystems: u32,
    pub capture_as_reparenting: bool,
    pub spatial_reparenting_used: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081FieldPolicyCompositeGapTerms {
    pub current_space_minus_inherited_setpoint: i64,
    pub supply_security_gap: i64,
    pub bilateral_relational_gap: i64,
    pub composite_gap_sum: i64,
    pub read_only_terms_only: bool,
    pub movement_execution: bool,
    pub schedule_execution: bool,
    pub direct_move_request: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_or_commitment: bool,
    pub new_field_policy_substrate: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0081Report {
    pub path_id: &'static str,
    pub scenario_gate_id: &'static str,
    pub status: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub nested_starmap_instantiated: bool,
    pub scenario_scoped_only: bool,

    pub atlas_report: Option<Atlas0080Report>,
    pub econ_scale_report: Option<EconScale0080RunReport>,
    pub atlas_report_admitted_pass: bool,
    pub econ_scale_report_admitted_pass: bool,

    pub starmap_side: u32,
    pub starsystem_count: usize,
    pub starsystem_side: u32,
    pub planets_per_starsystem: u32,
    pub planet_side: u32,
    pub logical_location_count: u32,
    pub active_theaters: Vec<Atlas0080TheaterId>,
    pub resident_theaters: Vec<Atlas0080TheaterId>,
    pub sparse_residency_composed: bool,

    pub factions: Vec<EconScale0080Faction>,
    pub fixed_terran_pirate_faction_set: bool,
    pub pirate_full_economy_faction_visible: bool,
    pub contended_clearing_reports_visible: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub unbounded_factions: bool,

    pub owner_overlay_summary: ProductionPath0081OwnerOverlaySummary,
    pub ownership_aggregation_summary: ProductionPath0081OwnershipAggregationSummary,
    pub field_policy_composite_gap_terms: ProductionPath0081FieldPolicyCompositeGapTerms,

    pub schedule_observation_control_demo_0080_1: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub realtime_loop_or_ui: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub clausething_dependency_present: bool,
    pub simthing_spec_altered: bool,
    pub invariant_edited: bool,
    pub passive_proof_wrapper: bool,
    pub general_production_path: bool,

    pub deterministic_replay_checksum: u64,
}

pub fn run_production_path_0080_1(input: &ProductionPath0081Input) -> ProductionPath0081Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_scenario(&input.scenario, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics, None, None);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let atlas_report = run_atlas_0080_0(&input.atlas_input);
    let econ_scale_report = run_econ_scale_0080_0(&input.econ_scale_input);

    if !atlas_passed(&atlas_report) {
        diagnostics.push("atlas_0080_0_disabled_rejected_or_not_admitted");
    }
    if !econ_scale_passed(&econ_scale_report) {
        diagnostics.push("econ_scale_0080_0_disabled_rejected_or_not_admitted");
    }
    if !diagnostics.is_empty() {
        return rejected_report(
            input,
            diagnostics,
            Some(atlas_report),
            Some(econ_scale_report),
        );
    }

    admitted_report(input, atlas_report, econ_scale_report)
}

pub fn replay_production_path_0080_1() -> (ProductionPath0081Report, ProductionPath0081Report) {
    let input = ProductionPath0081Input::explicit_opt_in();
    (
        run_production_path_0080_1(&input),
        run_production_path_0080_1(&input),
    )
}

fn validate_surface(surface: &ProductionPath0081Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("production_path_0080_1_default_on_behavior_rejected");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.schedule_observation_control_demo_0080_1 {
        diagnostics.push("schedule_observation_control_demo_0080_1");
    }
    if surface.realtime_loop || surface.ui_framework {
        diagnostics.push("realtime_loop_or_ui");
    }
}

fn validate_scenario(scenario: &ProductionPath0081Scenario, diagnostics: &mut Vec<&'static str>) {
    if scenario.scenario_gate_id != SCENARIO_0080_1_GATE_ID
        || scenario.scenario_name != PRODUCTION_PATH_0080_1_SCENARIO
        || scenario.starmap_side != ATLAS_0080_0_STARMAP_SIDE
        || scenario.starsystem_count != ATLAS_0080_0_STARSYSTEM_COUNT
        || scenario.starsystem_side != ATLAS_0080_0_STARSYSTEM_SIDE
        || scenario.planets_per_starsystem != 1
        || scenario.planet_side != ATLAS_0080_0_PLANET_SIDE
        || scenario.logical_location_count != ATLAS_0080_0_LOGICAL_LOCATION_COUNT
    {
        diagnostics.push("scenario_0080_1_nested_starmap_shape");
    }
}

fn validate_forbidden(
    forbidden: &ProductionPath0081ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.schedule_observation_control_demo_0080_1 {
        diagnostics.push("schedule_observation_control_demo_0080_1");
    }
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.realtime_loop_or_ui {
        diagnostics.push("realtime_loop_or_ui");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.new_shader_or_gpu_kernel {
        diagnostics.push("new_shader_or_gpu_kernel");
    }
    if forbidden.hard_currency_markets_trade_aibudget {
        diagnostics.push("hard_currency_markets_trade_aibudget");
    }
    if forbidden.nested_resource_flow {
        diagnostics.push("nested_resource_flow");
    }
    if forbidden.unbounded_factions {
        diagnostics.push("unbounded_factions");
    }
    if forbidden.owner_entity_as_spatial_parent {
        diagnostics.push("owner_entity_as_spatial_parent");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.simthing_spec_alteration {
        diagnostics.push("simthing_spec_alteration");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
    if forbidden.passive_proof_wrapper {
        diagnostics.push("passive_proof_wrapper");
    }
    if forbidden.general_production_path {
        diagnostics.push("general_production_path");
    }
}

fn disabled_no_op_report(input: &ProductionPath0081Input) -> ProductionPath0081Report {
    base_report(input, true, Vec::new(), None, None)
}

fn rejected_report(
    input: &ProductionPath0081Input,
    diagnostics: Vec<&'static str>,
    atlas_report: Option<Atlas0080Report>,
    econ_scale_report: Option<EconScale0080RunReport>,
) -> ProductionPath0081Report {
    let mut report = base_report(input, false, diagnostics, atlas_report, econ_scale_report);
    report.disabled_no_op = false;
    report
}

fn admitted_report(
    input: &ProductionPath0081Input,
    atlas_report: Atlas0080Report,
    econ_scale_report: EconScale0080RunReport,
) -> ProductionPath0081Report {
    let mut report = base_report(
        input,
        true,
        Vec::new(),
        Some(atlas_report),
        Some(econ_scale_report),
    );
    report.nested_starmap_instantiated = true;
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn base_report(
    input: &ProductionPath0081Input,
    admitted: bool,
    diagnostics: Vec<&'static str>,
    atlas_report: Option<Atlas0080Report>,
    econ_scale_report: Option<EconScale0080RunReport>,
) -> ProductionPath0081Report {
    let active = admitted && input.surface.gate.explicit_opt_in && diagnostics.is_empty();
    let atlas_report_admitted_pass = atlas_report.as_ref().map_or(false, atlas_passed);
    let econ_scale_report_admitted_pass =
        econ_scale_report.as_ref().map_or(false, econ_scale_passed);
    let active_theaters = atlas_report
        .as_ref()
        .and_then(|report| report.residency_reports.last())
        .map(|report| report.active_theaters_after.clone())
        .unwrap_or_default();
    let resident_theaters = atlas_report
        .as_ref()
        .and_then(|report| report.residency_reports.last())
        .map(|report| report.resident_theaters.clone())
        .unwrap_or_default();
    let factions = econ_scale_report
        .as_ref()
        .map(|report| report.factions.clone())
        .unwrap_or_default();
    let fixed_terran_pirate_faction_set = active
        && factions.len() == 2
        && factions.contains(&EconScale0080Faction::Terran)
        && factions.contains(&EconScale0080Faction::Pirate);
    let pirate_full_economy_faction_visible = active
        && econ_scale_report
            .as_ref()
            .map_or(false, |report| report.pirate_is_full_economy_faction);
    let contended_clearing_reports_visible = active
        && econ_scale_report
            .as_ref()
            .map_or(false, |report| !report.clearing_reports.is_empty());

    let mut report = ProductionPath0081Report {
        path_id: PRODUCTION_PATH_0080_1_ID,
        scenario_gate_id: input.scenario.scenario_gate_id,
        status: PRODUCTION_PATH_0080_1_STATUS_PASS,
        admitted,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op: admitted && !input.surface.gate.explicit_opt_in,
        nested_starmap_instantiated: false,
        scenario_scoped_only: true,
        atlas_report,
        econ_scale_report,
        atlas_report_admitted_pass,
        econ_scale_report_admitted_pass,
        starmap_side: input.scenario.starmap_side,
        starsystem_count: input.scenario.starsystem_count,
        starsystem_side: input.scenario.starsystem_side,
        planets_per_starsystem: input.scenario.planets_per_starsystem,
        planet_side: input.scenario.planet_side,
        logical_location_count: input.scenario.logical_location_count,
        active_theaters,
        resident_theaters,
        sparse_residency_composed: active && atlas_report_admitted_pass,
        fixed_terran_pirate_faction_set,
        pirate_full_economy_faction_visible,
        contended_clearing_reports_visible,
        factions,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        unbounded_factions: false,
        owner_overlay_summary: owner_overlay_summary(),
        ownership_aggregation_summary: ownership_aggregation_summary(input),
        field_policy_composite_gap_terms: field_policy_terms(),
        schedule_observation_control_demo_0080_1: input
            .surface
            .schedule_observation_control_demo_0080_1
            || input.forbidden.schedule_observation_control_demo_0080_1,
        default_session_pass_graph_wiring: input.surface.default_session_pass_graph_wiring
            || input.forbidden.default_session_pass_graph_wiring,
        global_default_schedule: input.surface.global_default_schedule
            || input.forbidden.global_default_schedule,
        realtime_loop_or_ui: input.surface.realtime_loop
            || input.surface.ui_framework
            || input.forbidden.realtime_loop_or_ui,
        direct_movement_command: input.forbidden.direct_movement_command,
        external_boundary_request: input.forbidden.external_boundary_request,
        cpu_planner_urgency_commitment: input.forbidden.cpu_planner_urgency_commitment,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        new_shader_or_gpu_kernel: input.forbidden.new_shader_or_gpu_kernel,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        simthing_spec_altered: input.forbidden.simthing_spec_alteration,
        invariant_edited: input.forbidden.invariant_edit,
        passive_proof_wrapper: input.forbidden.passive_proof_wrapper,
        general_production_path: input.forbidden.general_production_path,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn atlas_passed(report: &Atlas0080Report) -> bool {
    report.admitted
        && report.explicit_opt_in
        && !report.disabled_no_op
        && report.diagnostics.is_empty()
        && report.sparse_residency
        && report.value_noop_parity_bit_exact
}

fn econ_scale_passed(report: &EconScale0080RunReport) -> bool {
    report.admitted
        && report.explicit_opt_in
        && !report.disabled_no_op
        && report.diagnostics.is_empty()
        && report.faction_indexed_participation
        && report.pirate_is_full_economy_faction
        && report.parity_bit_exact
}

fn owner_overlay_summary() -> ProductionPath0081OwnerOverlaySummary {
    ProductionPath0081OwnerOverlaySummary {
        faction_owner_simthings_are_session_siblings: true,
        owner_entity_as_spatial_parent: false,
        terran_owner_id: TERRAN_OWNER_ID,
        pirate_owner_id: PIRATE_OWNER_ID,
        location_owner_overlays_inherit_numeric_weights: true,
        ship_owner_overlays_inherit_faction_weights: true,
        terran_policy_weight: TERRAN_POLICY_WEIGHT,
        pirate_policy_weight: PIRATE_POLICY_WEIGHT,
        no_new_owner_substrate_opened: true,
    }
}

fn ownership_aggregation_summary(
    input: &ProductionPath0081Input,
) -> ProductionPath0081OwnershipAggregationSummary {
    ProductionPath0081OwnershipAggregationSummary {
        derived_owner_overlay_summary: true,
        planet_to_starsystem_up_aggregation: true,
        terran_owned_planets: input.scenario.terran_owned_planets,
        terran_owned_starsystems_derived: input.scenario.terran_owned_planets,
        neutral_starsystems: input.scenario.neutral_starsystems,
        capture_as_reparenting: false,
        spatial_reparenting_used: false,
    }
}

fn field_policy_terms() -> ProductionPath0081FieldPolicyCompositeGapTerms {
    let current_space_minus_inherited_setpoint = 3;
    let supply_security_gap = -8;
    let bilateral_relational_gap = 5;
    ProductionPath0081FieldPolicyCompositeGapTerms {
        current_space_minus_inherited_setpoint,
        supply_security_gap,
        bilateral_relational_gap,
        composite_gap_sum: current_space_minus_inherited_setpoint
            + supply_security_gap
            + bilateral_relational_gap,
        read_only_terms_only: true,
        movement_execution: false,
        schedule_execution: false,
        direct_move_request: false,
        external_boundary_request: false,
        cpu_planner_urgency_or_commitment: false,
        new_field_policy_substrate: false,
    }
}

fn checksum_report(report: &ProductionPath0081Report) -> u64 {
    [
        report.starmap_side as u64,
        report.starsystem_count as u64,
        report.starsystem_side as u64,
        report.planet_side as u64,
        report.logical_location_count as u64,
        report
            .atlas_report
            .as_ref()
            .map(|atlas| atlas.deterministic_replay_checksum)
            .unwrap_or(0),
        report
            .econ_scale_report
            .as_ref()
            .map(|econ| econ.deterministic_replay_checksum)
            .unwrap_or(0),
        report
            .ownership_aggregation_summary
            .terran_owned_starsystems_derived as u64,
        report.field_policy_composite_gap_terms.composite_gap_sum as u64,
    ]
    .iter()
    .fold(0xcbf2_9ce4_8422_2325, |hash, value| {
        fnv_append_u64(hash, *value)
    })
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
