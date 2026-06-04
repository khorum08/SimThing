use crate::{
    export_gameplay_0080_text, observe_gameplay_0080_0, DefaultSchedule0080Input,
    DefaultSchedule0080Surface, Gameplay0080ObservationInput, Gameplay0080ObservationReport,
    Gameplay0080ObservationSurface,
};

pub const CONTROL_0080_0_ID: &str = "CONTROL-0080-0";
pub const CONTROL_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - bounded Local Patrol Economy command admission";
pub const CONTROL_0080_0_SCENARIO: &str = "Local Patrol Economy";

const MAX_STEP_COUNT: u32 = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Control0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Control0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0080Surface {
    pub gate: Control0080Gate,
    pub direct_movement_control: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
}

impl Control0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Control0080Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Control0080Command {
    SetSourceDisruption(i64),
    SetDestinationDisruption(i64),
    SetSourceSupply(i64),
    SetDestinationSupply(i64),
    SetSourceLocalSecurity(i64),
    SetDestinationLocalSecurity(i64),
    SetStepCount(u32),
    SetPatrolDisruptionReduction(i64),
    RunObservedScenario,
    ExportTranscript,
    DirectPatrolMove,
    DirectPirateMove,
    ExternalBoundaryRequest,
    CpuPlannerOrCommitment,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0080CommandBatch {
    pub commands: Vec<Control0080Command>,
}

impl Control0080CommandBatch {
    pub fn canonical_run() -> Self {
        Self {
            commands: vec![
                Control0080Command::SetStepCount(3),
                Control0080Command::RunObservedScenario,
                Control0080Command::ExportTranscript,
            ],
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0080ForbiddenRequests {
    pub direct_patrol_move: bool,
    pub direct_pirate_move: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_or_commitment: bool,
    pub player_command_loop: bool,
    pub ui_framework: bool,
    pub realtime_loop: bool,
    pub global_default_schedule: bool,
    pub semantic_or_raw_wgsl: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub closed_ladder_reopen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0080RejectedCommand {
    pub command_index: usize,
    pub diagnostic: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Control0080AdmissionInput {
    pub surface: Control0080Surface,
    pub base_schedule_input: DefaultSchedule0080Input,
    pub commands: Control0080CommandBatch,
    pub forbidden: Control0080ForbiddenRequests,
}

impl Control0080AdmissionInput {
    pub fn default_simsession() -> Self {
        Self {
            surface: Control0080Surface::default_simsession(),
            base_schedule_input: DefaultSchedule0080Input::default_simsession(),
            commands: Control0080CommandBatch::default(),
            forbidden: Control0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Control0080Surface::with_explicit_opt_in(),
            base_schedule_input: DefaultSchedule0080Input::explicit_opt_in(),
            commands: Control0080CommandBatch::canonical_run(),
            forbidden: Control0080ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Control0080AdmissionReport {
    pub control_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub bounded_command_admission_only: bool,
    pub direct_movement_control: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub cpu_planner_used: bool,
    pub external_boundary_request_emitted: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency_present: bool,
    pub closed_ladders_reopened: bool,

    pub applied_command_count: u32,
    pub rejected_commands: Vec<Control0080RejectedCommand>,
    pub schedule_input: DefaultSchedule0080Input,
    pub observation_report: Option<Gameplay0080ObservationReport>,
    pub text_export: String,
    pub deterministic_replay_checksum: u64,
}

pub fn admit_control_0080_0(input: &Control0080AdmissionInput) -> Control0080AdmissionReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(
            input,
            diagnostics,
            Vec::new(),
            input.base_schedule_input.clone(),
        );
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let mut schedule_input = input.base_schedule_input.clone();
    schedule_input.surface = DefaultSchedule0080Surface::with_explicit_opt_in();
    let mut rejected_commands = Vec::new();
    let mut applied_command_count = 0u32;
    let mut run_observed = false;
    let mut export_transcript = false;

    for (command_index, command) in input.commands.commands.iter().enumerate() {
        match apply_command(
            command,
            command_index,
            &mut schedule_input,
            &mut rejected_commands,
        ) {
            CommandApplyOutcome::Applied => {
                applied_command_count = applied_command_count.saturating_add(1);
            }
            CommandApplyOutcome::Rejected => {}
            CommandApplyOutcome::RunObservedScenario => {
                applied_command_count = applied_command_count.saturating_add(1);
                run_observed = true;
            }
            CommandApplyOutcome::ExportTranscript => {
                applied_command_count = applied_command_count.saturating_add(1);
                export_transcript = true;
            }
        }
    }

    if !rejected_commands.is_empty() {
        return rejected_report(
            input,
            vec!["control_0080_0_command_rejected"],
            rejected_commands,
            schedule_input,
        );
    }

    if export_transcript && !run_observed {
        run_observed = true;
    }

    let observation_report = if run_observed {
        Some(observe_gameplay_0080_0(&Gameplay0080ObservationInput {
            surface: Gameplay0080ObservationSurface::with_explicit_opt_in(),
            schedule_input: Some(schedule_input.clone()),
            schedule_report: None,
            forbidden: Default::default(),
        }))
    } else {
        None
    };

    let text_export = observation_report
        .as_ref()
        .map(export_gameplay_0080_text)
        .unwrap_or_default();

    admitted_report(
        input,
        schedule_input,
        applied_command_count,
        rejected_commands,
        observation_report,
        text_export,
        run_observed,
    )
}

pub fn replay_admit_control_0080_0() -> (Control0080AdmissionReport, Control0080AdmissionReport) {
    let input = Control0080AdmissionInput::explicit_opt_in();
    (admit_control_0080_0(&input), admit_control_0080_0(&input))
}

enum CommandApplyOutcome {
    Applied,
    Rejected,
    RunObservedScenario,
    ExportTranscript,
}

fn apply_command(
    command: &Control0080Command,
    command_index: usize,
    schedule_input: &mut DefaultSchedule0080Input,
    rejected_commands: &mut Vec<Control0080RejectedCommand>,
) -> CommandApplyOutcome {
    match command {
        Control0080Command::SetSourceDisruption(value) => {
            if *value < 0 {
                reject(
                    rejected_commands,
                    command_index,
                    "negative_disruption_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.source.disruption = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetDestinationDisruption(value) => {
            if *value < 0 {
                reject(
                    rejected_commands,
                    command_index,
                    "negative_disruption_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.destination.disruption = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetSourceSupply(value) => {
            if *value < 0 {
                reject(rejected_commands, command_index, "negative_supply_rejected");
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.source.supply = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetDestinationSupply(value) => {
            if *value < 0 {
                reject(rejected_commands, command_index, "negative_supply_rejected");
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.destination.supply = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetSourceLocalSecurity(value) => {
            if *value < 0 {
                reject(
                    rejected_commands,
                    command_index,
                    "negative_local_security_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.source.local_security = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetDestinationLocalSecurity(value) => {
            if *value < 0 {
                reject(
                    rejected_commands,
                    command_index,
                    "negative_local_security_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.scenario.destination.local_security = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetStepCount(value) => {
            if *value > MAX_STEP_COUNT {
                reject(
                    rejected_commands,
                    command_index,
                    "invalid_step_count_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.step_count = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::SetPatrolDisruptionReduction(value) => {
            if *value < 0 {
                reject(
                    rejected_commands,
                    command_index,
                    "negative_patrol_disruption_reduction_rejected",
                );
                return CommandApplyOutcome::Rejected;
            }
            schedule_input.patrol_disruption_reduction_per_step = *value;
            CommandApplyOutcome::Applied
        }
        Control0080Command::RunObservedScenario => CommandApplyOutcome::RunObservedScenario,
        Control0080Command::ExportTranscript => CommandApplyOutcome::ExportTranscript,
        Control0080Command::DirectPatrolMove => {
            reject(
                rejected_commands,
                command_index,
                "direct_patrol_move_rejected",
            );
            CommandApplyOutcome::Rejected
        }
        Control0080Command::DirectPirateMove => {
            reject(
                rejected_commands,
                command_index,
                "direct_pirate_move_rejected",
            );
            CommandApplyOutcome::Rejected
        }
        Control0080Command::ExternalBoundaryRequest => {
            reject(
                rejected_commands,
                command_index,
                "external_boundary_request_rejected",
            );
            CommandApplyOutcome::Rejected
        }
        Control0080Command::CpuPlannerOrCommitment => {
            reject(
                rejected_commands,
                command_index,
                "cpu_planner_or_commitment_rejected",
            );
            CommandApplyOutcome::Rejected
        }
    }
}

fn reject(
    rejected_commands: &mut Vec<Control0080RejectedCommand>,
    command_index: usize,
    diagnostic: &'static str,
) {
    rejected_commands.push(Control0080RejectedCommand {
        command_index,
        diagnostic,
    });
}

fn validate_surface(surface: &Control0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("control_0080_0_default_on_behavior_rejected");
    }
    if surface.direct_movement_control {
        diagnostics.push("direct_patrol_move_rejected");
    }
    if surface.player_command_loop {
        diagnostics.push("player_command_loop");
    }
    if surface.ui_framework_present {
        diagnostics.push("ui_framework");
    }
    if surface.realtime_loop_present {
        diagnostics.push("realtime_loop");
    }
    if surface.global_default_schedule_registered {
        diagnostics.push("global_default_schedule");
    }
}

fn validate_forbidden(
    forbidden: &Control0080ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.direct_patrol_move {
        diagnostics.push("direct_patrol_move_rejected");
    }
    if forbidden.direct_pirate_move {
        diagnostics.push("direct_pirate_move_rejected");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request_rejected");
    }
    if forbidden.cpu_planner_or_commitment {
        diagnostics.push("cpu_planner_or_commitment_rejected");
    }
    if forbidden.player_command_loop {
        diagnostics.push("player_command_loop");
    }
    if forbidden.ui_framework {
        diagnostics.push("ui_framework");
    }
    if forbidden.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
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
}

fn disabled_no_op_report(input: &Control0080AdmissionInput) -> Control0080AdmissionReport {
    base_report(
        input,
        Vec::new(),
        true,
        input.base_schedule_input.clone(),
        0,
        Vec::new(),
        None,
        String::new(),
        0,
    )
}

fn rejected_report(
    input: &Control0080AdmissionInput,
    diagnostics: Vec<&'static str>,
    rejected_commands: Vec<Control0080RejectedCommand>,
    schedule_input: DefaultSchedule0080Input,
) -> Control0080AdmissionReport {
    let mut report = base_report(
        input,
        diagnostics,
        false,
        schedule_input,
        0,
        rejected_commands,
        None,
        String::new(),
        0,
    );
    report.admitted = false;
    report
}

fn admitted_report(
    input: &Control0080AdmissionInput,
    schedule_input: DefaultSchedule0080Input,
    applied_command_count: u32,
    rejected_commands: Vec<Control0080RejectedCommand>,
    observation_report: Option<Gameplay0080ObservationReport>,
    text_export: String,
    run_observed: bool,
) -> Control0080AdmissionReport {
    let checksum = observation_report
        .as_ref()
        .map(|report| report.deterministic_replay_checksum)
        .unwrap_or(0);
    let mut report = base_report(
        input,
        Vec::new(),
        false,
        schedule_input,
        applied_command_count,
        rejected_commands,
        observation_report,
        text_export,
        checksum,
    );
    report.external_boundary_request_emitted = false;
    report.cpu_planner_used = false;
    if run_observed {
        report.bounded_command_admission_only = true;
    }
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn base_report(
    input: &Control0080AdmissionInput,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    schedule_input: DefaultSchedule0080Input,
    applied_command_count: u32,
    rejected_commands: Vec<Control0080RejectedCommand>,
    observation_report: Option<Gameplay0080ObservationReport>,
    text_export: String,
    deterministic_replay_checksum: u64,
) -> Control0080AdmissionReport {
    Control0080AdmissionReport {
        control_id: CONTROL_0080_0_ID,
        status: CONTROL_0080_0_STATUS_PASS,
        scenario_name: CONTROL_0080_0_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        bounded_command_admission_only: !disabled_no_op,
        direct_movement_control: input.surface.direct_movement_control,
        player_command_loop: input.surface.player_command_loop,
        ui_framework_present: input.surface.ui_framework_present,
        realtime_loop_present: input.surface.realtime_loop_present,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        semantic_or_raw_wgsl_present: false,
        cpu_planner_used: false,
        external_boundary_request_emitted: false,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        clausething_dependency_present: false,
        closed_ladders_reopened: false,
        applied_command_count,
        rejected_commands,
        schedule_input,
        observation_report,
        text_export,
        deterministic_replay_checksum,
    }
}

fn checksum_report(report: &Control0080AdmissionReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash = fnv_append_u64(hash, report.applied_command_count as u64);
    hash = fnv_append_u64(hash, report.rejected_commands.len() as u64);
    hash = fnv_append_u64(hash, report.schedule_input.step_count as u64);
    hash = fnv_append_u64(
        hash,
        report.schedule_input.scenario.source.disruption as u64,
    );
    hash = fnv_append_u64(
        hash,
        report.schedule_input.scenario.destination.disruption as u64,
    );
    hash = fnv_append_u64(
        hash,
        report
            .observation_report
            .as_ref()
            .map(|observation| observation.deterministic_replay_checksum)
            .unwrap_or(0),
    );
    for byte in report.text_export.as_bytes() {
        hash = fnv_append_u64(hash, *byte as u64);
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
