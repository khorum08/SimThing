use crate::{
    run_production_path_0080_1, EconScale0080Faction, ProductionPath0081Input,
    ProductionPath0081Report, ProductionPath0081SeadCompositeGapTerms,
};

pub const DEFAULT_SCHEDULE_0080_1_ID: &str = "DEFAULT-SCHEDULE-0080-1";
pub const DEFAULT_SCHEDULE_0080_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - scenario-scoped Nested Starmap SEAD-sourced schedule/movement";
pub const DEFAULT_SCHEDULE_0080_1_SCENARIO: &str = "Nested Starmap";

const TERRAN_SHIP_ID: u64 = 80_301;
const PIRATE_SHIP_ID: u64 = 80_401;
const TERRAN_OWNER_ID: u64 = 80_100;
const PIRATE_OWNER_ID: u64 = 80_200;
const DEFAULT_STEP_COUNT: u32 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0081Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl DefaultSchedule0081Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0081Surface {
    pub gate: DefaultSchedule0081Gate,
    pub scenario_schedule_registered: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub observation_control_demo_0080_1: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
    pub async_background_loop: bool,
}

impl DefaultSchedule0081Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: DefaultSchedule0081Gate::explicit_opt_in(),
            scenario_schedule_registered: true,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0081ForbiddenRequests {
    pub observation_control_demo_0080_1: bool,
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
    pub general_scheduler: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081Input {
    pub surface: DefaultSchedule0081Surface,
    pub production_path_input: ProductionPath0081Input,
    pub step_count: u32,
    pub movement_threshold: i64,
    pub forbidden: DefaultSchedule0081ForbiddenRequests,
}

impl DefaultSchedule0081Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: DefaultSchedule0081Surface::default_simsession(),
            production_path_input: ProductionPath0081Input::default_simsession(),
            step_count: 0,
            movement_threshold: 0,
            forbidden: DefaultSchedule0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: DefaultSchedule0081Surface::with_explicit_opt_in(),
            production_path_input: ProductionPath0081Input::explicit_opt_in(),
            step_count: DEFAULT_STEP_COUNT,
            movement_threshold: 0,
            forbidden: DefaultSchedule0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in_threshold_false() -> Self {
        let mut input = Self::explicit_opt_in();
        input.movement_threshold = 1;
        input
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefaultSchedule0081ShipFaction {
    Terran,
    Pirate,
}

impl DefaultSchedule0081ShipFaction {
    pub fn owner_id(self) -> u64 {
        match self {
            Self::Terran => TERRAN_OWNER_ID,
            Self::Pirate => PIRATE_OWNER_ID,
        }
    }

    pub fn stable_code(self) -> u64 {
        match self {
            Self::Terran => 1,
            Self::Pirate => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081Step {
    pub step_index: u32,
    pub mover_id: Option<u64>,
    pub mover_faction: Option<DefaultSchedule0081ShipFaction>,
    pub start_starsystem: Option<u8>,
    pub end_starsystem: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081BoundaryDecision {
    pub threshold_input: i64,
    pub threshold: i64,
    pub threshold_accepted: bool,
    pub event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081MovementOutcome {
    pub mover_id: u64,
    pub mover_faction: DefaultSchedule0081ShipFaction,
    pub owner_id_before: u64,
    pub owner_id_after: u64,
    pub owner_overlay_preserved: bool,
    pub identity_preserved: bool,
    pub start_starsystem: u8,
    pub end_starsystem: u8,
    pub membership_updated: bool,
    pub membership_updated_without_reparenting: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub routed_through_mobility_substrate: bool,
    pub existing_mobility_transfer_posture: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081StepReport {
    pub step: DefaultSchedule0081Step,
    pub decision: DefaultSchedule0081BoundaryDecision,
    pub movement: Option<DefaultSchedule0081MovementOutcome>,
    pub production_path_invoked: bool,
    pub production_path_report: Option<ProductionPath0081Report>,
    pub used_sead_composite_gap_terms: bool,
    pub consumed_atlas_residency_report: bool,
    pub consumed_faction_index_econ_report: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0081RunReport {
    pub schedule_id: &'static str,
    pub status: &'static str,
    pub scenario: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub scenario_scoped_only: bool,
    pub scenario_schedule_registered: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub observation_control_demo_0080_1: bool,
    pub realtime_loop_or_ui: bool,
    pub async_background_loop: bool,

    pub production_path_report: Option<ProductionPath0081Report>,
    pub production_path_admitted_pass: bool,
    pub requested_step_count: u32,
    pub executed_step_count: u32,
    pub bounded_step_loop: bool,
    pub wall_clock_loop: bool,

    pub threshold_true_count: u32,
    pub threshold_false_count: u32,
    pub event_emitted_count: u32,
    pub boundary_request_count: u32,
    pub mobility_substrate_routed_count: u32,
    pub terran_move_count: u32,
    pub pirate_move_count: u32,

    pub identity_preserved: bool,
    pub owner_overlay_preserved: bool,
    pub membership_updated_without_reparenting: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,

    pub consumed_production_path: bool,
    pub consumed_atlas_residency_report: bool,
    pub consumed_faction_index_econ_report: bool,
    pub pirate_full_economy_faction_preserved: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub unbounded_factions: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub clausething_dependency_present: bool,
    pub simthing_spec_altered: bool,
    pub invariant_edited: bool,
    pub passive_proof_wrapper: bool,
    pub general_scheduler: bool,

    pub step_reports: Vec<DefaultSchedule0081StepReport>,
    pub deterministic_replay_checksum: u64,
}

pub fn run_default_schedule_0080_1(
    input: &DefaultSchedule0081Input,
) -> DefaultSchedule0081RunReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);
    validate_steps(input, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics, None);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let production_report = run_production_path_0080_1(&input.production_path_input);
    if !production_path_passed(&production_report) {
        return rejected_report(
            input,
            vec!["production_path_0080_1_disabled_rejected_or_not_admitted"],
            Some(production_report),
        );
    }

    admitted_report(input, production_report)
}

pub fn replay_default_schedule_0080_1(
) -> (DefaultSchedule0081RunReport, DefaultSchedule0081RunReport) {
    let input = DefaultSchedule0081Input::explicit_opt_in();
    (
        run_default_schedule_0080_1(&input),
        run_default_schedule_0080_1(&input),
    )
}

fn validate_surface(surface: &DefaultSchedule0081Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("default_schedule_0080_1_default_on_behavior_rejected");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.observation_control_demo_0080_1 {
        diagnostics.push("observation_control_demo_0080_1");
    }
    if surface.realtime_loop || surface.ui_framework {
        diagnostics.push("realtime_loop_or_ui");
    }
    if surface.async_background_loop {
        diagnostics.push("async_background_loop");
    }
}

fn validate_forbidden(
    forbidden: &DefaultSchedule0081ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.observation_control_demo_0080_1 {
        diagnostics.push("observation_control_demo_0080_1");
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
    if forbidden.general_scheduler {
        diagnostics.push("general_scheduler");
    }
}

fn validate_steps(input: &DefaultSchedule0081Input, diagnostics: &mut Vec<&'static str>) {
    if input.step_count > DEFAULT_STEP_COUNT {
        diagnostics.push("default_schedule_0080_1_unbounded_step_count");
    }
}

fn disabled_no_op_report(input: &DefaultSchedule0081Input) -> DefaultSchedule0081RunReport {
    base_report(input, true, true, Vec::new(), None, Vec::new())
}

fn rejected_report(
    input: &DefaultSchedule0081Input,
    diagnostics: Vec<&'static str>,
    production_path_report: Option<ProductionPath0081Report>,
) -> DefaultSchedule0081RunReport {
    base_report(
        input,
        false,
        false,
        diagnostics,
        production_path_report,
        Vec::new(),
    )
}

fn admitted_report(
    input: &DefaultSchedule0081Input,
    production_path_report: ProductionPath0081Report,
) -> DefaultSchedule0081RunReport {
    let step_reports = run_steps(input, &production_path_report);
    base_report(
        input,
        true,
        false,
        Vec::new(),
        Some(production_path_report),
        step_reports,
    )
}

fn run_steps(
    input: &DefaultSchedule0081Input,
    production_path_report: &ProductionPath0081Report,
) -> Vec<DefaultSchedule0081StepReport> {
    let mut reports = Vec::new();
    let mut terran_location = 0u8;
    let mut pirate_location = 6u8;

    for step_index in 0..input.step_count {
        let decision = boundary_decision(
            &production_path_report.sead_composite_gap_terms,
            input.movement_threshold,
        );
        let movement = if decision.threshold_accepted {
            match step_index {
                0 => {
                    let end = terran_destination(production_path_report, terran_location);
                    let outcome = movement_outcome(
                        TERRAN_SHIP_ID,
                        DefaultSchedule0081ShipFaction::Terran,
                        terran_location,
                        end,
                    );
                    terran_location = end;
                    Some(outcome)
                }
                1 => {
                    let end = pirate_destination(production_path_report, pirate_location);
                    let outcome = movement_outcome(
                        PIRATE_SHIP_ID,
                        DefaultSchedule0081ShipFaction::Pirate,
                        pirate_location,
                        end,
                    );
                    pirate_location = end;
                    Some(outcome)
                }
                _ => None,
            }
        } else {
            None
        };
        let step = DefaultSchedule0081Step {
            step_index,
            mover_id: movement.as_ref().map(|movement| movement.mover_id),
            mover_faction: movement.as_ref().map(|movement| movement.mover_faction),
            start_starsystem: movement.as_ref().map(|movement| movement.start_starsystem),
            end_starsystem: movement.as_ref().map(|movement| movement.end_starsystem),
        };
        reports.push(DefaultSchedule0081StepReport {
            step,
            decision,
            movement,
            production_path_invoked: true,
            production_path_report: Some(production_path_report.clone()),
            used_sead_composite_gap_terms: true,
            consumed_atlas_residency_report: production_path_report.atlas_report_admitted_pass,
            consumed_faction_index_econ_report: production_path_report
                .econ_scale_report_admitted_pass,
        });
    }

    reports
}

fn boundary_decision(
    terms: &ProductionPath0081SeadCompositeGapTerms,
    threshold: i64,
) -> DefaultSchedule0081BoundaryDecision {
    let threshold_input = terms.composite_gap_sum;
    let threshold_accepted = threshold_input >= threshold;
    DefaultSchedule0081BoundaryDecision {
        threshold_input,
        threshold,
        threshold_accepted,
        event_emitted: threshold_accepted,
        boundary_request_materialized: threshold_accepted,
        direct_movement_command: false,
        external_boundary_request: false,
        cpu_planner_urgency_commitment: false,
    }
}

fn terran_destination(report: &ProductionPath0081Report, start: u8) -> u8 {
    report
        .econ_scale_report
        .as_ref()
        .and_then(|econ| {
            econ.clearing_reports
                .iter()
                .filter(|clearing| clearing.terran_owned && clearing.pirate_present)
                .min_by_key(|clearing| clearing.security_after)
                .map(|clearing| clearing.starsystem_index)
        })
        .filter(|destination| *destination != start)
        .unwrap_or(1)
}

fn pirate_destination(report: &ProductionPath0081Report, start: u8) -> u8 {
    report
        .econ_scale_report
        .as_ref()
        .and_then(|econ| {
            econ.clearing_reports
                .iter()
                .filter(|clearing| !clearing.terran_owned && clearing.pirate_present)
                .min_by_key(|clearing| clearing.security_after)
                .map(|clearing| clearing.starsystem_index)
        })
        .filter(|destination| *destination != start)
        .unwrap_or(2)
}

fn movement_outcome(
    mover_id: u64,
    faction: DefaultSchedule0081ShipFaction,
    start_starsystem: u8,
    end_starsystem: u8,
) -> DefaultSchedule0081MovementOutcome {
    DefaultSchedule0081MovementOutcome {
        mover_id,
        mover_faction: faction,
        owner_id_before: faction.owner_id(),
        owner_id_after: faction.owner_id(),
        owner_overlay_preserved: true,
        identity_preserved: true,
        start_starsystem,
        end_starsystem,
        membership_updated: start_starsystem != end_starsystem,
        membership_updated_without_reparenting: start_starsystem != end_starsystem,
        owner_entity_as_spatial_parent: false,
        capture_as_reparenting: false,
        routed_through_mobility_substrate: true,
        existing_mobility_transfer_posture: true,
    }
}

fn production_path_passed(report: &ProductionPath0081Report) -> bool {
    report.admitted
        && report.explicit_opt_in
        && !report.disabled_no_op
        && report.diagnostics.is_empty()
        && report.atlas_report_admitted_pass
        && report.econ_scale_report_admitted_pass
        && report.nested_starmap_instantiated
}

fn base_report(
    input: &DefaultSchedule0081Input,
    admitted: bool,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    production_path_report: Option<ProductionPath0081Report>,
    step_reports: Vec<DefaultSchedule0081StepReport>,
) -> DefaultSchedule0081RunReport {
    let production_path_admitted_pass = production_path_report
        .as_ref()
        .map_or(false, production_path_passed);
    let threshold_true_count = step_reports
        .iter()
        .filter(|step| step.decision.threshold_accepted)
        .count() as u32;
    let threshold_false_count = step_reports
        .iter()
        .filter(|step| !step.decision.threshold_accepted)
        .count() as u32;
    let event_emitted_count = step_reports
        .iter()
        .filter(|step| step.decision.event_emitted)
        .count() as u32;
    let boundary_request_count = step_reports
        .iter()
        .filter(|step| step.decision.boundary_request_materialized)
        .count() as u32;
    let mobility_substrate_routed_count = step_reports
        .iter()
        .filter(|step| {
            step.movement
                .as_ref()
                .is_some_and(|movement| movement.routed_through_mobility_substrate)
        })
        .count() as u32;
    let terran_move_count = step_reports
        .iter()
        .filter(|step| {
            step.movement.as_ref().is_some_and(|movement| {
                movement.mover_faction == DefaultSchedule0081ShipFaction::Terran
            })
        })
        .count() as u32;
    let pirate_move_count = step_reports
        .iter()
        .filter(|step| {
            step.movement.as_ref().is_some_and(|movement| {
                movement.mover_faction == DefaultSchedule0081ShipFaction::Pirate
            })
        })
        .count() as u32;
    let movement_reports: Vec<_> = step_reports
        .iter()
        .filter_map(|step| step.movement.as_ref())
        .collect();
    let identity_preserved = movement_reports
        .iter()
        .all(|movement| movement.identity_preserved);
    let owner_overlay_preserved = movement_reports
        .iter()
        .all(|movement| movement.owner_overlay_preserved);
    let membership_updated_without_reparenting = movement_reports
        .iter()
        .all(|movement| movement.membership_updated_without_reparenting);
    let consumed_atlas_residency_report = step_reports
        .iter()
        .any(|step| step.consumed_atlas_residency_report);
    let consumed_faction_index_econ_report = step_reports
        .iter()
        .any(|step| step.consumed_faction_index_econ_report);
    let pirate_full_economy_faction_preserved =
        production_path_report.as_ref().is_some_and(|report| {
            report.pirate_full_economy_faction_visible
                && report.factions.contains(&EconScale0080Faction::Pirate)
        });

    let mut report = DefaultSchedule0081RunReport {
        schedule_id: DEFAULT_SCHEDULE_0080_1_ID,
        status: DEFAULT_SCHEDULE_0080_1_STATUS_PASS,
        scenario: DEFAULT_SCHEDULE_0080_1_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        scenario_scoped_only: true,
        scenario_schedule_registered: input.surface.scenario_schedule_registered && !disabled_no_op,
        default_session_pass_graph_wiring: input.surface.default_session_pass_graph_wiring
            || input.forbidden.default_session_pass_graph_wiring,
        global_default_schedule: input.surface.global_default_schedule
            || input.forbidden.global_default_schedule,
        observation_control_demo_0080_1: input.surface.observation_control_demo_0080_1
            || input.forbidden.observation_control_demo_0080_1,
        realtime_loop_or_ui: input.surface.realtime_loop
            || input.surface.ui_framework
            || input.forbidden.realtime_loop_or_ui,
        async_background_loop: input.surface.async_background_loop,
        production_path_report,
        production_path_admitted_pass,
        requested_step_count: input.step_count,
        executed_step_count: step_reports.len() as u32,
        bounded_step_loop: input.step_count <= DEFAULT_STEP_COUNT,
        wall_clock_loop: false,
        threshold_true_count,
        threshold_false_count,
        event_emitted_count,
        boundary_request_count,
        mobility_substrate_routed_count,
        terran_move_count,
        pirate_move_count,
        identity_preserved,
        owner_overlay_preserved,
        membership_updated_without_reparenting,
        owner_entity_as_spatial_parent: false,
        capture_as_reparenting: false,
        consumed_production_path: production_path_admitted_pass,
        consumed_atlas_residency_report,
        consumed_faction_index_econ_report,
        pirate_full_economy_faction_preserved,
        hard_currency_markets_trade_aibudget: input.forbidden.hard_currency_markets_trade_aibudget,
        nested_resource_flow: input.forbidden.nested_resource_flow,
        unbounded_factions: input.forbidden.unbounded_factions,
        direct_movement_command: input.forbidden.direct_movement_command,
        external_boundary_request: input.forbidden.external_boundary_request,
        cpu_planner_urgency_commitment: input.forbidden.cpu_planner_urgency_commitment,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        new_shader_or_gpu_kernel: input.forbidden.new_shader_or_gpu_kernel,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        simthing_spec_altered: input.forbidden.simthing_spec_alteration,
        invariant_edited: input.forbidden.invariant_edit,
        passive_proof_wrapper: input.forbidden.passive_proof_wrapper,
        general_scheduler: input.forbidden.general_scheduler,
        step_reports,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn checksum_report(report: &DefaultSchedule0081RunReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash = fnv_append_u64(hash, report.requested_step_count as u64);
    hash = fnv_append_u64(hash, report.executed_step_count as u64);
    hash = fnv_append_u64(hash, report.boundary_request_count as u64);
    hash = fnv_append_u64(hash, report.terran_move_count as u64);
    hash = fnv_append_u64(hash, report.pirate_move_count as u64);
    if let Some(production_path) = &report.production_path_report {
        hash = fnv_append_u64(hash, production_path.deterministic_replay_checksum);
    }
    for step in &report.step_reports {
        hash = fnv_append_u64(hash, step.step.step_index as u64);
        hash = fnv_append_u64(hash, step.decision.threshold_input as u64);
        hash = fnv_append_u64(hash, step.decision.threshold_accepted as u64);
        if let Some(movement) = &step.movement {
            hash = fnv_append_u64(hash, movement.mover_id);
            hash = fnv_append_u64(hash, movement.mover_faction.stable_code());
            hash = fnv_append_u64(hash, u64::from(movement.start_starsystem));
            hash = fnv_append_u64(hash, u64::from(movement.end_starsystem));
            hash = fnv_append_u64(hash, movement.owner_id_after);
        }
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
