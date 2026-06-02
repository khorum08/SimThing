use crate::{
    run_production_path_0080_0, LocalPatrolEconomyScenario, ProductionPath0080ForbiddenRequests,
    ProductionPath0080Input, ProductionPath0080Report, ProductionPath0080Surface,
    PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES,
};

pub const DEFAULT_SCHEDULE_0080_0_ID: &str = "DEFAULT-SCHEDULE-0080-0";
pub const DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS: &str =
    "IMPLEMENTED / PASS - 1A scenario-scoped schedule + patrol loop";
pub const DEFAULT_SCHEDULE_0080_0_SCENARIO: &str = "Local Patrol Economy";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl DefaultSchedule0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0080Surface {
    pub gate: DefaultSchedule0080Gate,
    pub scenario_schedule_registered: bool,
    pub global_default_schedule_registered: bool,
}

impl DefaultSchedule0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: DefaultSchedule0080Gate::explicit_opt_in(),
            scenario_schedule_registered: true,
            global_default_schedule_registered: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DefaultSchedule0080ForbiddenRequests {
    pub global_default_schedule: bool,
    pub gameplay_surface: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_or_external_move_script: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub capture_as_reparenting: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub clausething_dependency: bool,
    pub closed_ladder_reopen: bool,
    pub pirate_behavior: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0080Step {
    pub step_index: u32,
    pub source_disruption_before: i64,
    pub source_disruption_after: i64,
    pub destination_disruption_before: i64,
    pub destination_disruption_after: i64,
    pub source_supply: i64,
    pub destination_supply: i64,
    pub source_local_security: i64,
    pub destination_local_security: i64,
    pub threshold_crossed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultSchedule0080StepReport {
    pub step: DefaultSchedule0080Step,
    pub sead_threshold_accepted: bool,
    pub sead_emit_event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub production_path_invoked: bool,
    pub production_path_report: Option<ProductionPath0080Report>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefaultSchedule0080Input {
    pub surface: DefaultSchedule0080Surface,
    pub scenario: LocalPatrolEconomyScenario,
    pub step_count: u32,
    pub patrol_disruption_reduction_per_step: i64,
    pub forbidden: DefaultSchedule0080ForbiddenRequests,
}

impl DefaultSchedule0080Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: DefaultSchedule0080Surface::default_simsession(),
            scenario: LocalPatrolEconomyScenario::canonical(),
            step_count: 0,
            patrol_disruption_reduction_per_step: 1,
            forbidden: DefaultSchedule0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: DefaultSchedule0080Surface::with_explicit_opt_in(),
            scenario: LocalPatrolEconomyScenario::canonical(),
            step_count: 3,
            patrol_disruption_reduction_per_step: 1,
            forbidden: DefaultSchedule0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in_threshold_false() -> Self {
        let mut input = Self::explicit_opt_in();
        input.scenario.source.disruption = input.scenario.disruption_threshold.saturating_sub(2);
        input.scenario.source.local_security = input.scenario.local_security_floor + 2;
        input
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultSchedule0080RunReport {
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
    pub global_default_schedule_registered: bool,
    pub gameplay_surface_present: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub cpu_planner_used: bool,
    pub external_move_script_used: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency_present: bool,
    pub closed_ladders_reopened: bool,
    pub pirate_behavior_implemented: bool,

    pub requested_step_count: u32,
    pub executed_step_count: u32,
    pub production_path_invocation_count: u32,
    pub boundary_request_count: u32,
    pub bounded_local_economy_values: Vec<&'static str>,
    pub bounded_local_economy_only: bool,
    pub step_reports: Vec<DefaultSchedule0080StepReport>,
    pub deterministic_replay_checksum: u64,
}

pub fn run_default_schedule_0080_0(
    input: &DefaultSchedule0080Input,
) -> DefaultSchedule0080RunReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let mut scenario = input.scenario.clone();
    let mut step_reports = Vec::new();
    let mut production_path_invocation_count = 0u32;
    let mut boundary_request_count = 0u32;

    for step_index in 0..input.step_count {
        let source_disruption_before = scenario.source.disruption;
        let destination_disruption_before = scenario.destination.disruption;
        let threshold_crossed = patrol_threshold_crossed(&scenario);
        let source_disruption_after = source_disruption_before
            .saturating_sub(input.patrol_disruption_reduction_per_step)
            .max(0);
        scenario.source.disruption = source_disruption_after;

        let production_path_report = if threshold_crossed {
            let mut production_input = ProductionPath0080Input {
                surface: ProductionPath0080Surface::with_explicit_opt_in(),
                scenario: scenario.clone(),
                forbidden: ProductionPath0080ForbiddenRequests::default(),
            };
            production_input.scenario.source.disruption = source_disruption_before;
            let report = run_production_path_0080_0(&production_input);
            production_path_invocation_count = production_path_invocation_count.saturating_add(1);
            boundary_request_count = boundary_request_count.saturating_add(1);
            scenario.source.patrol_participation_count = report.source_patrol_count_after;
            scenario.destination.patrol_participation_count = report.destination_patrol_count_after;
            Some(report)
        } else {
            None
        };

        step_reports.push(DefaultSchedule0080StepReport {
            step: DefaultSchedule0080Step {
                step_index,
                source_disruption_before,
                source_disruption_after,
                destination_disruption_before,
                destination_disruption_after: scenario.destination.disruption,
                source_supply: scenario.source.supply,
                destination_supply: scenario.destination.supply,
                source_local_security: scenario.source.local_security,
                destination_local_security: scenario.destination.local_security,
                threshold_crossed,
            },
            sead_threshold_accepted: threshold_crossed,
            sead_emit_event_emitted: threshold_crossed,
            boundary_request_materialized: threshold_crossed,
            production_path_invoked: production_path_report.is_some(),
            production_path_report,
        });

        if threshold_crossed {
            scenario.source.disruption = scenario.disruption_threshold.saturating_sub(1);
            scenario.source.local_security = scenario.local_security_floor.saturating_add(1);
        }
    }

    admitted_report(
        input,
        step_reports,
        production_path_invocation_count,
        boundary_request_count,
    )
}

pub fn replay_default_schedule_0080_0(
) -> (DefaultSchedule0080RunReport, DefaultSchedule0080RunReport) {
    let input = DefaultSchedule0080Input::explicit_opt_in();
    (
        run_default_schedule_0080_0(&input),
        run_default_schedule_0080_0(&input),
    )
}

fn patrol_threshold_crossed(scenario: &LocalPatrolEconomyScenario) -> bool {
    scenario.source.disruption >= scenario.disruption_threshold
        || scenario.source.local_security <= scenario.local_security_floor
        || scenario.source.supply < scenario.source.maintenance
        || scenario.destination.disruption >= scenario.disruption_threshold
}

fn validate_surface(surface: &DefaultSchedule0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("default_schedule_0080_0_default_on_behavior_rejected");
    }
    if surface.global_default_schedule_registered {
        diagnostics.push("global_default_schedule");
    }
}

fn validate_forbidden(
    forbidden: &DefaultSchedule0080ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.gameplay_surface {
        diagnostics.push("gameplay_surface");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.cpu_planner_or_external_move_script {
        diagnostics.push("cpu_planner_or_external_move_script");
    }
    if forbidden.hard_currency_markets_trade_aibudget {
        diagnostics.push("hard_currency_markets_trade_aibudget");
    }
    if forbidden.nested_resource_flow {
        diagnostics.push("nested_resource_flow");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.owner_entity_as_spatial_parent {
        diagnostics.push("owner_entity_as_spatial_parent");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
    if forbidden.pirate_behavior {
        diagnostics.push("pirate_behavior_not_implemented_in_1a");
    }
}

fn disabled_no_op_report(input: &DefaultSchedule0080Input) -> DefaultSchedule0080RunReport {
    base_report(input, Vec::new(), true, Vec::new(), 0, 0)
}

fn rejected_report(
    input: &DefaultSchedule0080Input,
    diagnostics: Vec<&'static str>,
) -> DefaultSchedule0080RunReport {
    let mut report = base_report(input, diagnostics, false, Vec::new(), 0, 0);
    report.admitted = false;
    report
}

fn admitted_report(
    input: &DefaultSchedule0080Input,
    step_reports: Vec<DefaultSchedule0080StepReport>,
    production_path_invocation_count: u32,
    boundary_request_count: u32,
) -> DefaultSchedule0080RunReport {
    let mut report = base_report(
        input,
        Vec::new(),
        false,
        step_reports,
        production_path_invocation_count,
        boundary_request_count,
    );
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn base_report(
    input: &DefaultSchedule0080Input,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    step_reports: Vec<DefaultSchedule0080StepReport>,
    production_path_invocation_count: u32,
    boundary_request_count: u32,
) -> DefaultSchedule0080RunReport {
    DefaultSchedule0080RunReport {
        schedule_id: DEFAULT_SCHEDULE_0080_0_ID,
        status: DEFAULT_SCHEDULE_0080_0_STATUS_1A_PASS,
        scenario: DEFAULT_SCHEDULE_0080_0_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        scenario_scoped_only: true,
        scenario_schedule_registered: input.surface.scenario_schedule_registered && !disabled_no_op,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        gameplay_surface_present: false,
        semantic_or_raw_wgsl_present: false,
        cpu_planner_used: false,
        external_move_script_used: false,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        clausething_dependency_present: false,
        closed_ladders_reopened: false,
        pirate_behavior_implemented: false,
        requested_step_count: input.step_count,
        executed_step_count: step_reports.len() as u32,
        production_path_invocation_count,
        boundary_request_count,
        bounded_local_economy_values: PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES.to_vec(),
        bounded_local_economy_only: true,
        step_reports,
        deterministic_replay_checksum: 0,
    }
}

fn checksum_report(report: &DefaultSchedule0080RunReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash = fnv_append_u64(hash, report.requested_step_count as u64);
    hash = fnv_append_u64(hash, report.executed_step_count as u64);
    hash = fnv_append_u64(hash, report.production_path_invocation_count as u64);
    hash = fnv_append_u64(hash, report.boundary_request_count as u64);
    for step in &report.step_reports {
        hash = fnv_append_u64(hash, step.step.step_index as u64);
        hash = fnv_append_u64(hash, step.step.source_disruption_before as u64);
        hash = fnv_append_u64(hash, step.step.source_disruption_after as u64);
        hash = fnv_append_u64(hash, step.step.destination_disruption_before as u64);
        hash = fnv_append_u64(hash, step.step.destination_disruption_after as u64);
        hash = fnv_append_u64(hash, step.production_path_invoked as u64);
        if let Some(production) = &step.production_path_report {
            hash = fnv_append_u64(hash, production.deterministic_replay_checksum);
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
