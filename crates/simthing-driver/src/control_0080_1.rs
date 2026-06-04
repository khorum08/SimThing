use crate::{
    export_gameplay_0080_1_text, observe_gameplay_0080_1, DefaultSchedule0081Input,
    DefaultSchedule0081Surface, Gameplay0081Input, Gameplay0081ObservationReport,
    Gameplay0081Surface,
};

pub const CONTROL_0080_1_ID: &str = "CONTROL-0080-1";
pub const CONTROL_0080_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - bounded Nested Starmap command admission";
pub const CONTROL_0080_1_SCENARIO: &str = "Nested Starmap";

const MAX_STEP_COUNT: u32 = 3;
const MAX_STARSYSTEM_INDEX: u8 = 9;
const MIN_BOUNDED_VALUE: i64 = -32;
const MAX_BOUNDED_VALUE: i64 = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Control0081Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Control0081Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0081Surface {
    pub gate: Control0081Gate,
    pub direct_movement_control: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
    pub demo_packaging_present: bool,
}

impl Control0081Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Control0081Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Control0081Command {
    SetStepCount(u32),
    SetTerranThreshold(i64),
    SetPirateThreshold(i64),
    SetTerranSourceStarsystem(u8),
    SetTerranCandidateStarsystem(u8),
    SetPirateSourceStarsystem(u8),
    SetPirateCandidateStarsystem(u8),
    SetSupplySecurityGap(i64),
    SetBilateralRelationalGap(i64),
    SetCompositeGapSum(i64),
    RunObservedScenario,
    ExportTranscript,
    DirectTerranMove,
    DirectPirateMove,
    ExternalBoundaryRequest,
    SeadBypass,
    CpuPlannerOrCommitment,
    GeneralCommandSystem,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0081CommandBatch {
    pub commands: Vec<Control0081Command>,
}

impl Control0081CommandBatch {
    pub fn canonical_run() -> Self {
        Self {
            commands: vec![
                Control0081Command::SetStepCount(3),
                Control0081Command::SetTerranThreshold(0),
                Control0081Command::SetPirateThreshold(0),
                Control0081Command::RunObservedScenario,
                Control0081Command::ExportTranscript,
            ],
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Control0081ForbiddenRequests {
    pub direct_terran_move: bool,
    pub direct_pirate_move: bool,
    pub external_boundary_request: bool,
    pub sead_bypass: bool,
    pub cpu_planner_or_commitment: bool,
    pub player_command_loop: bool,
    pub ui_framework: bool,
    pub realtime_loop: bool,
    pub global_default_schedule: bool,
    pub demo_packaging: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub closed_ladder_reopen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0081RejectedCommand {
    pub command_index: usize,
    pub diagnostic: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0081CommandTranscriptRow {
    pub command_index: usize,
    pub command: &'static str,
    pub accepted: bool,
    pub target_bounded_field: &'static str,
    pub old_value: i64,
    pub new_value: i64,
    pub run_observed: bool,
    pub export_produced: bool,
    pub replay_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0081BoundedConfig {
    pub terran_threshold: i64,
    pub pirate_threshold: i64,
    pub terran_source_starsystem: u8,
    pub terran_candidate_starsystem: u8,
    pub pirate_source_starsystem: u8,
    pub pirate_candidate_starsystem: u8,
    pub supply_security_gap: i64,
    pub bilateral_relational_gap: i64,
    pub composite_gap_sum: i64,
}

impl Default for Control0081BoundedConfig {
    fn default() -> Self {
        Self {
            terran_threshold: 0,
            pirate_threshold: 0,
            terran_source_starsystem: 0,
            terran_candidate_starsystem: 1,
            pirate_source_starsystem: 6,
            pirate_candidate_starsystem: 2,
            supply_security_gap: -8,
            bilateral_relational_gap: 5,
            composite_gap_sum: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0081AdmissionInput {
    pub surface: Control0081Surface,
    pub base_schedule_input: DefaultSchedule0081Input,
    pub commands: Control0081CommandBatch,
    pub forbidden: Control0081ForbiddenRequests,
}

impl Control0081AdmissionInput {
    pub fn default_simsession() -> Self {
        Self {
            surface: Control0081Surface::default_simsession(),
            base_schedule_input: DefaultSchedule0081Input::default_simsession(),
            commands: Control0081CommandBatch::default(),
            forbidden: Control0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Control0081Surface::with_explicit_opt_in(),
            base_schedule_input: DefaultSchedule0081Input::explicit_opt_in(),
            commands: Control0081CommandBatch::canonical_run(),
            forbidden: Control0081ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Control0081AdmissionReport {
    pub control_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub bounded_command_admission_only: bool,
    pub command_writes_existing_bounded_values_only: bool,
    pub command_moved_ship: bool,
    pub command_emitted_boundary_request: bool,
    pub command_bypassed_sead: bool,
    pub direct_movement_control: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
    pub demo_packaging_present: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub cpu_planner_used: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency_present: bool,
    pub closed_ladders_reopened: bool,

    pub applied_command_count: u32,
    pub rejected_commands: Vec<Control0081RejectedCommand>,
    pub command_transcript: Vec<Control0081CommandTranscriptRow>,
    pub schedule_input: DefaultSchedule0081Input,
    pub bounded_config: Control0081BoundedConfig,
    pub observation_report: Option<Gameplay0081ObservationReport>,
    pub text_export: String,
    pub deterministic_replay_checksum: u64,
}

pub fn admit_control_0080_1(input: &Control0081AdmissionInput) -> Control0081AdmissionReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(
            input,
            diagnostics,
            Vec::new(),
            Vec::new(),
            input.base_schedule_input.clone(),
            Control0081BoundedConfig::default(),
        );
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let mut schedule_input = input.base_schedule_input.clone();
    schedule_input.surface = DefaultSchedule0081Surface::with_explicit_opt_in();
    schedule_input.production_path_input = crate::ProductionPath0081Input::explicit_opt_in();
    let mut bounded_config = Control0081BoundedConfig::default();
    let mut rejected_commands = Vec::new();
    let mut command_transcript = Vec::new();
    let mut applied_command_count = 0u32;
    let mut run_observed = false;
    let mut export_transcript = false;

    for (command_index, command) in input.commands.commands.iter().enumerate() {
        match apply_command(
            command,
            command_index,
            &mut schedule_input,
            &mut bounded_config,
            &mut rejected_commands,
            &mut command_transcript,
        ) {
            CommandApplyOutcome::Applied => {
                applied_command_count = applied_command_count.saturating_add(1);
            }
            CommandApplyOutcome::Rejected => {}
            CommandApplyOutcome::RunObservedScenario => {
                applied_command_count = applied_command_count.saturating_add(1);
                run_observed = true;
                command_transcript.push(transcript_row(
                    command_index,
                    "run_observed_scenario",
                    true,
                    "GAMEPLAY-0080-1",
                    0,
                    1,
                    true,
                    false,
                ));
            }
            CommandApplyOutcome::ExportTranscript => {
                applied_command_count = applied_command_count.saturating_add(1);
                export_transcript = true;
                command_transcript.push(transcript_row(
                    command_index,
                    "export_transcript",
                    true,
                    "text_export",
                    0,
                    1,
                    false,
                    true,
                ));
            }
        }
    }

    if !rejected_commands.is_empty() {
        return rejected_report(
            input,
            vec!["control_0080_1_command_rejected"],
            rejected_commands,
            command_transcript,
            schedule_input,
            bounded_config,
        );
    }

    if export_transcript && !run_observed {
        run_observed = true;
    }

    let observation_report = if run_observed {
        Some(observe_gameplay_0080_1(&Gameplay0081Input {
            surface: Gameplay0081Surface::with_explicit_opt_in(),
            schedule_input: Some(schedule_input.clone()),
            schedule_report: None,
            forbidden: Default::default(),
        }))
    } else {
        None
    };

    let text_export = if export_transcript {
        observation_report
            .as_ref()
            .map(export_gameplay_0080_1_text)
            .unwrap_or_default()
    } else {
        String::new()
    };

    admitted_report(
        input,
        schedule_input,
        bounded_config,
        applied_command_count,
        rejected_commands,
        command_transcript,
        observation_report,
        text_export,
    )
}

pub fn replay_admit_control_0080_1() -> (Control0081AdmissionReport, Control0081AdmissionReport) {
    let input = Control0081AdmissionInput::explicit_opt_in();
    (admit_control_0080_1(&input), admit_control_0080_1(&input))
}

enum CommandApplyOutcome {
    Applied,
    Rejected,
    RunObservedScenario,
    ExportTranscript,
}

fn apply_command(
    command: &Control0081Command,
    command_index: usize,
    schedule_input: &mut DefaultSchedule0081Input,
    bounded_config: &mut Control0081BoundedConfig,
    rejected_commands: &mut Vec<Control0081RejectedCommand>,
    command_transcript: &mut Vec<Control0081CommandTranscriptRow>,
) -> CommandApplyOutcome {
    match command {
        Control0081Command::SetStepCount(value) => {
            if *value > MAX_STEP_COUNT {
                return reject_command(
                    rejected_commands,
                    command_transcript,
                    command_index,
                    "set_step_count",
                    "invalid_step_count_rejected",
                );
            }
            let old = i64::from(schedule_input.step_count);
            schedule_input.step_count = *value;
            push_applied(
                command_transcript,
                command_index,
                "set_step_count",
                "DefaultSchedule0081Input.step_count",
                old,
                i64::from(*value),
            );
            CommandApplyOutcome::Applied
        }
        Control0081Command::SetTerranThreshold(value) => {
            if !bounded_i64(*value) {
                return reject_command(
                    rejected_commands,
                    command_transcript,
                    command_index,
                    "set_terran_threshold",
                    "bounded_threshold_rejected",
                );
            }
            let old = bounded_config.terran_threshold;
            bounded_config.terran_threshold = *value;
            write_schedule_threshold(schedule_input, bounded_config);
            push_applied(
                command_transcript,
                command_index,
                "set_terran_threshold",
                "DefaultSchedule0081Input.movement_threshold",
                old,
                *value,
            );
            CommandApplyOutcome::Applied
        }
        Control0081Command::SetPirateThreshold(value) => {
            if !bounded_i64(*value) {
                return reject_command(
                    rejected_commands,
                    command_transcript,
                    command_index,
                    "set_pirate_threshold",
                    "bounded_threshold_rejected",
                );
            }
            let old = bounded_config.pirate_threshold;
            bounded_config.pirate_threshold = *value;
            write_schedule_threshold(schedule_input, bounded_config);
            push_applied(
                command_transcript,
                command_index,
                "set_pirate_threshold",
                "DefaultSchedule0081Input.movement_threshold",
                old,
                *value,
            );
            CommandApplyOutcome::Applied
        }
        Control0081Command::SetTerranSourceStarsystem(value) => apply_starsystem(
            *value,
            command_index,
            "set_terran_source_starsystem",
            "bounded_config.terran_source_starsystem",
            &mut bounded_config.terran_source_starsystem,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetTerranCandidateStarsystem(value) => apply_starsystem(
            *value,
            command_index,
            "set_terran_candidate_starsystem",
            "bounded_config.terran_candidate_starsystem",
            &mut bounded_config.terran_candidate_starsystem,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetPirateSourceStarsystem(value) => apply_starsystem(
            *value,
            command_index,
            "set_pirate_source_starsystem",
            "bounded_config.pirate_source_starsystem",
            &mut bounded_config.pirate_source_starsystem,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetPirateCandidateStarsystem(value) => apply_starsystem(
            *value,
            command_index,
            "set_pirate_candidate_starsystem",
            "bounded_config.pirate_candidate_starsystem",
            &mut bounded_config.pirate_candidate_starsystem,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetSupplySecurityGap(value) => apply_i64_config(
            *value,
            command_index,
            "set_supply_security_gap",
            "bounded_config.supply_security_gap",
            &mut bounded_config.supply_security_gap,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetBilateralRelationalGap(value) => apply_i64_config(
            *value,
            command_index,
            "set_bilateral_relational_gap",
            "bounded_config.bilateral_relational_gap",
            &mut bounded_config.bilateral_relational_gap,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::SetCompositeGapSum(value) => apply_i64_config(
            *value,
            command_index,
            "set_composite_gap_sum",
            "bounded_config.composite_gap_sum",
            &mut bounded_config.composite_gap_sum,
            rejected_commands,
            command_transcript,
        ),
        Control0081Command::RunObservedScenario => CommandApplyOutcome::RunObservedScenario,
        Control0081Command::ExportTranscript => CommandApplyOutcome::ExportTranscript,
        Control0081Command::DirectTerranMove => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "direct_terran_move",
            "direct_terran_move_rejected",
        ),
        Control0081Command::DirectPirateMove => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "direct_pirate_move",
            "direct_pirate_move_rejected",
        ),
        Control0081Command::ExternalBoundaryRequest => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "external_boundary_request",
            "external_boundary_request_rejected",
        ),
        Control0081Command::SeadBypass => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "sead_bypass",
            "sead_bypass_rejected",
        ),
        Control0081Command::CpuPlannerOrCommitment => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "cpu_planner_or_commitment",
            "cpu_planner_or_commitment_rejected",
        ),
        Control0081Command::GeneralCommandSystem => reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            "general_command_system",
            "general_command_system_rejected",
        ),
    }
}

fn apply_starsystem(
    value: u8,
    command_index: usize,
    command: &'static str,
    field: &'static str,
    target: &mut u8,
    rejected_commands: &mut Vec<Control0081RejectedCommand>,
    command_transcript: &mut Vec<Control0081CommandTranscriptRow>,
) -> CommandApplyOutcome {
    if value > MAX_STARSYSTEM_INDEX {
        return reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            command,
            "starsystem_index_out_of_bounds_rejected",
        );
    }
    let old = i64::from(*target);
    *target = value;
    push_applied(
        command_transcript,
        command_index,
        command,
        field,
        old,
        i64::from(value),
    );
    CommandApplyOutcome::Applied
}

fn apply_i64_config(
    value: i64,
    command_index: usize,
    command: &'static str,
    field: &'static str,
    target: &mut i64,
    rejected_commands: &mut Vec<Control0081RejectedCommand>,
    command_transcript: &mut Vec<Control0081CommandTranscriptRow>,
) -> CommandApplyOutcome {
    if !bounded_i64(value) {
        return reject_command(
            rejected_commands,
            command_transcript,
            command_index,
            command,
            "bounded_config_value_rejected",
        );
    }
    let old = *target;
    *target = value;
    push_applied(
        command_transcript,
        command_index,
        command,
        field,
        old,
        value,
    );
    CommandApplyOutcome::Applied
}

fn push_applied(
    command_transcript: &mut Vec<Control0081CommandTranscriptRow>,
    command_index: usize,
    command: &'static str,
    field: &'static str,
    old_value: i64,
    new_value: i64,
) {
    command_transcript.push(transcript_row(
        command_index,
        command,
        true,
        field,
        old_value,
        new_value,
        false,
        false,
    ));
}

fn reject_command(
    rejected_commands: &mut Vec<Control0081RejectedCommand>,
    command_transcript: &mut Vec<Control0081CommandTranscriptRow>,
    command_index: usize,
    command: &'static str,
    diagnostic: &'static str,
) -> CommandApplyOutcome {
    rejected_commands.push(Control0081RejectedCommand {
        command_index,
        diagnostic,
    });
    command_transcript.push(transcript_row(
        command_index,
        command,
        false,
        diagnostic,
        0,
        0,
        false,
        false,
    ));
    CommandApplyOutcome::Rejected
}

fn transcript_row(
    command_index: usize,
    command: &'static str,
    accepted: bool,
    target_bounded_field: &'static str,
    old_value: i64,
    new_value: i64,
    run_observed: bool,
    export_produced: bool,
) -> Control0081CommandTranscriptRow {
    Control0081CommandTranscriptRow {
        command_index,
        command,
        accepted,
        target_bounded_field,
        old_value,
        new_value,
        run_observed,
        export_produced,
        replay_checksum: 0,
    }
}

fn bounded_i64(value: i64) -> bool {
    (MIN_BOUNDED_VALUE..=MAX_BOUNDED_VALUE).contains(&value)
}

fn write_schedule_threshold(
    schedule_input: &mut DefaultSchedule0081Input,
    bounded_config: &Control0081BoundedConfig,
) {
    schedule_input.movement_threshold = bounded_config
        .terran_threshold
        .max(bounded_config.pirate_threshold);
}

fn validate_surface(surface: &Control0081Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("control_0080_1_default_on_behavior_rejected");
    }
    if surface.direct_movement_control {
        diagnostics.push("direct_movement_control_rejected");
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
    if surface.demo_packaging_present {
        diagnostics.push("demo_packaging");
    }
}

fn validate_forbidden(
    forbidden: &Control0081ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.direct_terran_move {
        diagnostics.push("direct_terran_move_rejected");
    }
    if forbidden.direct_pirate_move {
        diagnostics.push("direct_pirate_move_rejected");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request_rejected");
    }
    if forbidden.sead_bypass {
        diagnostics.push("sead_bypass_rejected");
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
    if forbidden.demo_packaging {
        diagnostics.push("demo_packaging");
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
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
}

fn disabled_no_op_report(input: &Control0081AdmissionInput) -> Control0081AdmissionReport {
    base_report(
        input,
        Vec::new(),
        true,
        input.base_schedule_input.clone(),
        Control0081BoundedConfig::default(),
        0,
        Vec::new(),
        Vec::new(),
        None,
        String::new(),
    )
}

fn rejected_report(
    input: &Control0081AdmissionInput,
    diagnostics: Vec<&'static str>,
    rejected_commands: Vec<Control0081RejectedCommand>,
    command_transcript: Vec<Control0081CommandTranscriptRow>,
    schedule_input: DefaultSchedule0081Input,
    bounded_config: Control0081BoundedConfig,
) -> Control0081AdmissionReport {
    let mut report = base_report(
        input,
        diagnostics,
        false,
        schedule_input,
        bounded_config,
        0,
        rejected_commands,
        command_transcript,
        None,
        String::new(),
    );
    report.admitted = false;
    report.deterministic_replay_checksum = checksum_report(&report);
    stamp_transcript_checksums(&mut report);
    report
}

fn admitted_report(
    input: &Control0081AdmissionInput,
    schedule_input: DefaultSchedule0081Input,
    bounded_config: Control0081BoundedConfig,
    applied_command_count: u32,
    rejected_commands: Vec<Control0081RejectedCommand>,
    command_transcript: Vec<Control0081CommandTranscriptRow>,
    observation_report: Option<Gameplay0081ObservationReport>,
    text_export: String,
) -> Control0081AdmissionReport {
    let mut report = base_report(
        input,
        Vec::new(),
        false,
        schedule_input,
        bounded_config,
        applied_command_count,
        rejected_commands,
        command_transcript,
        observation_report,
        text_export,
    );
    report.deterministic_replay_checksum = checksum_report(&report);
    stamp_transcript_checksums(&mut report);
    report
}

fn base_report(
    input: &Control0081AdmissionInput,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    schedule_input: DefaultSchedule0081Input,
    bounded_config: Control0081BoundedConfig,
    applied_command_count: u32,
    rejected_commands: Vec<Control0081RejectedCommand>,
    command_transcript: Vec<Control0081CommandTranscriptRow>,
    observation_report: Option<Gameplay0081ObservationReport>,
    text_export: String,
) -> Control0081AdmissionReport {
    Control0081AdmissionReport {
        control_id: CONTROL_0080_1_ID,
        status: CONTROL_0080_1_STATUS_PASS,
        scenario_name: CONTROL_0080_1_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        bounded_command_admission_only: !disabled_no_op,
        command_writes_existing_bounded_values_only: !disabled_no_op,
        command_moved_ship: false,
        command_emitted_boundary_request: false,
        command_bypassed_sead: false,
        direct_movement_control: input.surface.direct_movement_control,
        player_command_loop: input.surface.player_command_loop,
        ui_framework_present: input.surface.ui_framework_present,
        realtime_loop_present: input.surface.realtime_loop_present,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        demo_packaging_present: input.surface.demo_packaging_present,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        new_shader_or_gpu_kernel: input.forbidden.new_shader_or_gpu_kernel,
        cpu_planner_used: false,
        hard_currency_markets_trade_aibudget: input.forbidden.hard_currency_markets_trade_aibudget,
        nested_resource_flow: input.forbidden.nested_resource_flow,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        closed_ladders_reopened: input.forbidden.closed_ladder_reopen,
        applied_command_count,
        rejected_commands,
        command_transcript,
        schedule_input,
        bounded_config,
        observation_report,
        text_export,
        deterministic_replay_checksum: 0,
    }
}

fn stamp_transcript_checksums(report: &mut Control0081AdmissionReport) {
    for row in &mut report.command_transcript {
        row.replay_checksum = report.deterministic_replay_checksum;
    }
}

fn checksum_report(report: &Control0081AdmissionReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash = fnv_append_u64(hash, report.admitted as u64);
    hash = fnv_append_u64(hash, report.applied_command_count as u64);
    hash = fnv_append_u64(hash, report.rejected_commands.len() as u64);
    hash = fnv_append_u64(hash, report.schedule_input.step_count as u64);
    hash = fnv_append_u64(hash, report.schedule_input.movement_threshold as u64);
    hash = fnv_append_u64(hash, report.bounded_config.terran_threshold as u64);
    hash = fnv_append_u64(hash, report.bounded_config.pirate_threshold as u64);
    hash = fnv_append_u64(
        hash,
        u64::from(report.bounded_config.terran_source_starsystem),
    );
    hash = fnv_append_u64(
        hash,
        u64::from(report.bounded_config.terran_candidate_starsystem),
    );
    hash = fnv_append_u64(
        hash,
        u64::from(report.bounded_config.pirate_source_starsystem),
    );
    hash = fnv_append_u64(
        hash,
        u64::from(report.bounded_config.pirate_candidate_starsystem),
    );
    hash = fnv_append_u64(hash, report.bounded_config.supply_security_gap as u64);
    hash = fnv_append_u64(hash, report.bounded_config.bilateral_relational_gap as u64);
    hash = fnv_append_u64(hash, report.bounded_config.composite_gap_sum as u64);
    hash = fnv_append_u64(
        hash,
        report
            .observation_report
            .as_ref()
            .map(|observation| observation.replay_checksum)
            .unwrap_or(0),
    );
    for byte in report.text_export.as_bytes() {
        hash = fnv_append_u64(hash, u64::from(*byte));
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
