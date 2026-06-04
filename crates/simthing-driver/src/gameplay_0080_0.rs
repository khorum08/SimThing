use crate::{
    run_default_schedule_0080_0, DefaultSchedule0080Input, DefaultSchedule0080Location,
    DefaultSchedule0080RunReport, DefaultSchedule0080StepReport, LocalPatrolEconomyScenario,
    DEFAULT_SCHEDULE_0080_0_SCENARIO,
};

pub const GAMEPLAY_0080_0_ID: &str = "GAMEPLAY-0080-0";
pub const GAMEPLAY_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - read-only Local Patrol Economy observation export";
pub const GAMEPLAY_0080_0_SCENARIO: &str = "Local Patrol Economy";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0080ObservationGate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Gameplay0080ObservationGate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0080ObservationSurface {
    pub gate: Gameplay0080ObservationGate,
    pub player_commands_registered: bool,
    pub command_input_present: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub gameplay_scheduler_registered: bool,
    pub global_default_schedule_registered: bool,
}

impl Gameplay0080ObservationSurface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Gameplay0080ObservationGate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0080ForbiddenRequests {
    pub player_commands: bool,
    pub command_input: bool,
    pub ui_framework: bool,
    pub realtime_loop: bool,
    pub gameplay_scheduler: bool,
    pub global_default_schedule: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_or_external_move_script: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub closed_ladder_reopen: bool,
    pub passive_proof_wrapper: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Gameplay0080ObservationInput {
    pub surface: Gameplay0080ObservationSurface,
    pub schedule_input: Option<DefaultSchedule0080Input>,
    pub schedule_report: Option<DefaultSchedule0080RunReport>,
    pub forbidden: Gameplay0080ForbiddenRequests,
}

impl Gameplay0080ObservationInput {
    pub fn default_simsession() -> Self {
        Self {
            surface: Gameplay0080ObservationSurface::default_simsession(),
            schedule_input: None,
            schedule_report: None,
            forbidden: Gameplay0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Gameplay0080ObservationSurface::with_explicit_opt_in(),
            schedule_input: Some(DefaultSchedule0080Input::explicit_opt_in()),
            schedule_report: None,
            forbidden: Gameplay0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in_from_report(report: DefaultSchedule0080RunReport) -> Self {
        Self {
            surface: Gameplay0080ObservationSurface::with_explicit_opt_in(),
            schedule_input: None,
            schedule_report: Some(report),
            forbidden: Gameplay0080ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0080LocationSummary {
    pub location_label: &'static str,
    pub supply: i64,
    pub maintenance: i64,
    pub local_output: i64,
    pub local_security: i64,
    pub disruption: i64,
    pub patrol_participation_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0080StepTranscript {
    pub step_index: u32,
    pub source: Gameplay0080LocationSummary,
    pub destination: Gameplay0080LocationSummary,
    pub patrol_entity_id: u64,
    pub patrol_owner_id: u64,
    pub patrol_at_source_before_step: bool,
    pub patrol_at_destination_before_step: bool,
    pub patrol_relocated: bool,
    pub pirate_entity_id: u64,
    pub pirate_location_before: Option<&'static str>,
    pub pirate_location_after: Option<&'static str>,
    pub pirate_relocated: bool,
    pub pirate_supply_drained: i64,
    pub pirate_disruption_added: i64,
    pub threshold_accepted: bool,
    pub event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub production_path_invoked: bool,
    pub source_target_score: Option<i64>,
    pub destination_target_score: Option<i64>,
    pub local_security_evasion_observed: bool,
    pub cat_and_mouse_step_observed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0080Transcript {
    pub steps: Vec<Gameplay0080StepTranscript>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0080ObservationReport {
    pub observation_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub read_only: bool,
    pub player_commands_present: bool,
    pub command_input_present: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub gameplay_scheduler_present: bool,
    pub global_default_schedule_registered: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub cpu_planner_used: bool,
    pub external_move_script_used: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency_present: bool,
    pub closed_ladders_reopened: bool,
    pub passive_proof_wrapper_present: bool,

    pub schedule_id: &'static str,
    pub schedule_status: &'static str,
    pub schedule_admitted: bool,
    pub executed_step_count: u32,
    pub boundary_request_count: u32,
    pub production_path_invocation_count: u32,
    pub pirate_relocation_count: u32,
    pub pirate_supply_drained_total: i64,
    pub pirate_disruption_added_total: i64,
    pub cat_and_mouse_pattern_observed: bool,
    pub deterministic_replay_checksum: u64,

    pub transcript: Gameplay0080Transcript,
    pub text_export: String,
}

pub fn observe_gameplay_0080_0(
    input: &Gameplay0080ObservationInput,
) -> Gameplay0080ObservationReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let schedule_report = resolve_schedule_report(input);
    admitted_report(input, schedule_report)
}

pub fn replay_observe_gameplay_0080_0(
) -> (Gameplay0080ObservationReport, Gameplay0080ObservationReport) {
    let input = Gameplay0080ObservationInput::explicit_opt_in();
    (
        observe_gameplay_0080_0(&input),
        observe_gameplay_0080_0(&input),
    )
}

pub fn export_gameplay_0080_text(report: &Gameplay0080ObservationReport) -> String {
    report.text_export.clone()
}

fn resolve_schedule_report(input: &Gameplay0080ObservationInput) -> DefaultSchedule0080RunReport {
    if let Some(report) = &input.schedule_report {
        return report.clone();
    }
    let schedule_input = input
        .schedule_input
        .clone()
        .unwrap_or_else(DefaultSchedule0080Input::explicit_opt_in);
    run_default_schedule_0080_0(&schedule_input)
}

fn validate_surface(surface: &Gameplay0080ObservationSurface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("gameplay_0080_0_default_on_behavior_rejected");
    }
    if surface.player_commands_registered {
        diagnostics.push("player_commands");
    }
    if surface.command_input_present {
        diagnostics.push("command_input");
    }
    if surface.ui_framework_present {
        diagnostics.push("ui_framework");
    }
    if surface.realtime_loop_present {
        diagnostics.push("realtime_loop");
    }
    if surface.gameplay_scheduler_registered {
        diagnostics.push("gameplay_scheduler");
    }
    if surface.global_default_schedule_registered {
        diagnostics.push("global_default_schedule");
    }
}

fn validate_forbidden(
    forbidden: &Gameplay0080ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.player_commands {
        diagnostics.push("player_commands");
    }
    if forbidden.command_input {
        diagnostics.push("command_input");
    }
    if forbidden.ui_framework {
        diagnostics.push("ui_framework");
    }
    if forbidden.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if forbidden.gameplay_scheduler {
        diagnostics.push("gameplay_scheduler");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
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
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
    if forbidden.passive_proof_wrapper {
        diagnostics.push("passive_proof_wrapper");
    }
}

fn disabled_no_op_report(input: &Gameplay0080ObservationInput) -> Gameplay0080ObservationReport {
    base_report(
        input,
        Vec::new(),
        true,
        empty_schedule_report(),
        Gameplay0080Transcript { steps: Vec::new() },
        String::new(),
    )
}

fn rejected_report(
    input: &Gameplay0080ObservationInput,
    diagnostics: Vec<&'static str>,
) -> Gameplay0080ObservationReport {
    let mut report = base_report(
        input,
        diagnostics,
        false,
        empty_schedule_report(),
        Gameplay0080Transcript { steps: Vec::new() },
        String::new(),
    );
    report.admitted = false;
    report
}

fn admitted_report(
    input: &Gameplay0080ObservationInput,
    schedule_report: DefaultSchedule0080RunReport,
) -> Gameplay0080ObservationReport {
    let transcript = build_transcript(&schedule_report);
    let text_export = render_text_export(&schedule_report, &transcript);
    base_report(
        input,
        Vec::new(),
        false,
        schedule_report,
        transcript,
        text_export,
    )
}

fn base_report(
    input: &Gameplay0080ObservationInput,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    schedule_report: DefaultSchedule0080RunReport,
    transcript: Gameplay0080Transcript,
    text_export: String,
) -> Gameplay0080ObservationReport {
    Gameplay0080ObservationReport {
        observation_id: GAMEPLAY_0080_0_ID,
        status: GAMEPLAY_0080_0_STATUS_PASS,
        scenario_name: GAMEPLAY_0080_0_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        read_only: true,
        player_commands_present: input.surface.player_commands_registered,
        command_input_present: input.surface.command_input_present,
        ui_framework_present: input.surface.ui_framework_present,
        realtime_loop_present: input.surface.realtime_loop_present,
        gameplay_scheduler_present: input.surface.gameplay_scheduler_registered,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        semantic_or_raw_wgsl_present: false,
        cpu_planner_used: schedule_report.cpu_planner_used,
        external_move_script_used: schedule_report.external_move_script_used,
        hard_currency_markets_trade_aibudget: schedule_report.hard_currency_markets_trade_aibudget,
        nested_resource_flow: schedule_report.nested_resource_flow,
        clausething_dependency_present: schedule_report.clausething_dependency_present,
        closed_ladders_reopened: schedule_report.closed_ladders_reopened,
        passive_proof_wrapper_present: false,
        schedule_id: schedule_report.schedule_id,
        schedule_status: schedule_report.status,
        schedule_admitted: schedule_report.admitted && !schedule_report.disabled_no_op,
        executed_step_count: schedule_report.executed_step_count,
        boundary_request_count: schedule_report.boundary_request_count,
        production_path_invocation_count: schedule_report.production_path_invocation_count,
        pirate_relocation_count: schedule_report.pirate_relocation_count,
        pirate_supply_drained_total: schedule_report.pirate_supply_drained_total,
        pirate_disruption_added_total: schedule_report.pirate_disruption_added_total,
        cat_and_mouse_pattern_observed: schedule_report.cat_and_mouse_pattern_observed,
        deterministic_replay_checksum: schedule_report.deterministic_replay_checksum,
        transcript,
        text_export,
    }
}

fn build_transcript(schedule_report: &DefaultSchedule0080RunReport) -> Gameplay0080Transcript {
    let steps = schedule_report
        .step_reports
        .iter()
        .map(build_step_transcript)
        .collect();
    Gameplay0080Transcript { steps }
}

fn build_step_transcript(step: &DefaultSchedule0080StepReport) -> Gameplay0080StepTranscript {
    let production = step.production_path_report.as_ref();
    let pirate = step.pirate_report.as_ref();
    let canonical = LocalPatrolEconomyScenario::canonical();

    let patrol_entity_id = production
        .map(|report| report.patrol_entity_id_before)
        .unwrap_or(canonical.patrol_entity_id);
    let patrol_owner_id = production
        .map(|report| report.owner_id_before)
        .unwrap_or(canonical.owner_id);
    let patrol_at_source_before = production
        .map(|report| report.source_membership_before)
        .unwrap_or(true);
    let patrol_at_destination_before = production
        .map(|report| report.destination_membership_before)
        .unwrap_or(false);
    let patrol_relocated = step.production_path_invoked;

    let pirate_disruption_added = pirate
        .map(|report| {
            report
                .local_disruption_after
                .saturating_sub(report.local_disruption_before)
        })
        .unwrap_or(0);

    let cat_and_mouse_step_observed = step.production_path_invoked
        && pirate.is_some_and(|report| {
            report.location_before != report.location_after
                && report.used_local_security_evasion_term
        });

    Gameplay0080StepTranscript {
        step_index: step.step.step_index,
        source: Gameplay0080LocationSummary {
            location_label: "source",
            supply: step.step.source_supply,
            maintenance: canonical.source.maintenance,
            local_output: canonical.source.local_output,
            local_security: step.step.source_local_security,
            disruption: step.step.source_disruption_after,
            patrol_participation_count: production
                .map(|report| report.source_patrol_count_after)
                .unwrap_or(if patrol_at_source_before { 1 } else { 0 }),
        },
        destination: Gameplay0080LocationSummary {
            location_label: "destination",
            supply: step.step.destination_supply,
            maintenance: canonical.destination.maintenance,
            local_output: canonical.destination.local_output,
            local_security: step.step.destination_local_security,
            disruption: step.step.destination_disruption_after,
            patrol_participation_count: production
                .map(|report| report.destination_patrol_count_after)
                .unwrap_or(if patrol_at_destination_before { 1 } else { 0 }),
        },
        patrol_entity_id,
        patrol_owner_id,
        patrol_at_source_before_step: patrol_at_source_before,
        patrol_at_destination_before_step: patrol_at_destination_before,
        patrol_relocated,
        pirate_entity_id: pirate.map(|report| report.pirate_entity_id).unwrap_or(0),
        pirate_location_before: pirate.map(|report| location_label(&report.location_before)),
        pirate_location_after: pirate.map(|report| location_label(&report.location_after)),
        pirate_relocated: pirate
            .is_some_and(|report| report.location_before != report.location_after),
        pirate_supply_drained: pirate.map(|report| report.supply_drained).unwrap_or(0),
        pirate_disruption_added,
        threshold_accepted: step.sead_threshold_accepted,
        event_emitted: step.sead_emit_event_emitted,
        boundary_request_materialized: step.boundary_request_materialized,
        production_path_invoked: step.production_path_invoked,
        source_target_score: pirate.map(|report| report.source_target_score),
        destination_target_score: pirate.map(|report| report.destination_target_score),
        local_security_evasion_observed: pirate
            .is_some_and(|report| report.used_local_security_evasion_term),
        cat_and_mouse_step_observed,
    }
}

fn location_label(location: &DefaultSchedule0080Location) -> &'static str {
    match location {
        DefaultSchedule0080Location::Source => "source",
        DefaultSchedule0080Location::Destination => "destination",
    }
}

fn render_text_export(
    schedule_report: &DefaultSchedule0080RunReport,
    transcript: &Gameplay0080Transcript,
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "GAMEPLAY-0080-0|scenario={}|schedule_id={}|schedule_status={}|steps={}|boundary_requests={}|production_path_invocations={}|pirate_relocations={}|pirate_supply_drained_total={}|pirate_disruption_added_total={}|cat_and_mouse={}|checksum={}",
        schedule_report.scenario,
        schedule_report.schedule_id,
        schedule_report.status,
        schedule_report.executed_step_count,
        schedule_report.boundary_request_count,
        schedule_report.production_path_invocation_count,
        schedule_report.pirate_relocation_count,
        schedule_report.pirate_supply_drained_total,
        schedule_report.pirate_disruption_added_total,
        schedule_report.cat_and_mouse_pattern_observed,
        schedule_report.deterministic_replay_checksum,
    ));

    for step in &transcript.steps {
        lines.push(format!(
            "STEP|index={}|source_supply={}|source_security={}|source_disruption={}|destination_supply={}|destination_security={}|destination_disruption={}|patrol_entity={}|patrol_owner={}|patrol_relocated={}|pirate_entity={}|pirate_before={}|pirate_after={}|pirate_relocated={}|pirate_supply_drained={}|pirate_disruption_added={}|threshold={}|event={}|boundary={}|production_path={}|source_score={}|destination_score={}|local_security_evasion={}|cat_and_mouse_step={}",
            step.step_index,
            step.source.supply,
            step.source.local_security,
            step.source.disruption,
            step.destination.supply,
            step.destination.local_security,
            step.destination.disruption,
            step.patrol_entity_id,
            step.patrol_owner_id,
            step.patrol_relocated,
            step.pirate_entity_id,
            step.pirate_location_before.unwrap_or("none"),
            step.pirate_location_after.unwrap_or("none"),
            step.pirate_relocated,
            step.pirate_supply_drained,
            step.pirate_disruption_added,
            step.threshold_accepted,
            step.event_emitted,
            step.boundary_request_materialized,
            step.production_path_invoked,
            step.source_target_score.unwrap_or(-1),
            step.destination_target_score.unwrap_or(-1),
            step.local_security_evasion_observed,
            step.cat_and_mouse_step_observed,
        ));
    }

    lines.join("\n")
}

fn empty_schedule_report() -> DefaultSchedule0080RunReport {
    DefaultSchedule0080RunReport {
        schedule_id: "",
        status: "",
        scenario: DEFAULT_SCHEDULE_0080_0_SCENARIO,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: false,
        default_off: true,
        disabled_no_op: true,
        scenario_scoped_only: true,
        scenario_schedule_registered: false,
        global_default_schedule_registered: false,
        gameplay_surface_present: false,
        semantic_or_raw_wgsl_present: false,
        cpu_planner_used: false,
        external_move_script_used: false,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        clausething_dependency_present: false,
        closed_ladders_reopened: false,
        pirate_behavior_implemented: false,
        pirate_entity_id: 0,
        patrol_identity_lane: 0,
        pirate_identity_lane: 0,
        pirate_is_second_identity: false,
        pirate_is_second_economy_owner: false,
        pirate_relocation_count: 0,
        pirate_supply_drained_total: 0,
        pirate_disruption_added_total: 0,
        local_security_evasion_term_implemented: false,
        cat_and_mouse_pattern_observed: false,
        requested_step_count: 0,
        executed_step_count: 0,
        production_path_invocation_count: 0,
        boundary_request_count: 0,
        bounded_local_economy_values: Vec::new(),
        bounded_local_economy_only: true,
        step_reports: Vec::new(),
        deterministic_replay_checksum: 0,
    }
}
