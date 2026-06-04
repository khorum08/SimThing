use crate::{
    admit_control_0080_0, Control0080AdmissionInput, Control0080AdmissionReport,
    Control0080CommandBatch, DefaultSchedule0080Input, Gameplay0080ObservationReport,
    Gameplay0080StepTranscript,
};

pub const DEMO_0080_0_ID: &str = "DEMO-0080-0";
pub const DEMO_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - headless Local Patrol Economy demo/export library helper";
pub const DEMO_0080_0_SCENARIO: &str = "Local Patrol Economy";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Demo0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Demo0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Demo0080Surface {
    pub gate: Demo0080Gate,
    pub cli_binary_requested: bool,
    pub direct_movement_control: bool,
    pub external_boundary_request: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
}

impl Demo0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Demo0080Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Demo0080ForbiddenRequests {
    pub cli_binary: bool,
    pub direct_movement_control: bool,
    pub external_boundary_request: bool,
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
pub struct Demo0080MovementRecord {
    pub step_index: u32,
    pub patrol_start: &'static str,
    pub patrol_end: &'static str,
    pub patrol_relocated: bool,
    pub patrol_relocation_source: Option<&'static str>,
    pub patrol_relocation_destination: Option<&'static str>,
    pub pirate_start: Option<&'static str>,
    pub pirate_end: Option<&'static str>,
    pub pirate_relocated: bool,
    pub pirate_relocation_source: Option<&'static str>,
    pub pirate_relocation_destination: Option<&'static str>,
    pub source_supply: i64,
    pub source_disruption: i64,
    pub source_local_security: i64,
    pub destination_supply: i64,
    pub destination_disruption: i64,
    pub destination_local_security: i64,
    pub threshold_accepted: bool,
    pub event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub production_path_invoked: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demo0080MovementDay {
    pub step_index: u32,
    pub record: Demo0080MovementRecord,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Demo0080Input {
    pub surface: Demo0080Surface,
    pub control_input: Control0080AdmissionInput,
    pub forbidden: Demo0080ForbiddenRequests,
}

impl Demo0080Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: Demo0080Surface::default_simsession(),
            control_input: Control0080AdmissionInput::default_simsession(),
            forbidden: Demo0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Demo0080Surface::with_explicit_opt_in(),
            control_input: canonical_control_input(),
            forbidden: Demo0080ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Demo0080Report {
    pub demo_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub headless_library_helper_only: bool,
    pub cli_binary_present: bool,
    pub direct_movement_control: bool,
    pub external_boundary_request_emitted: bool,
    pub player_command_loop: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency_present: bool,

    pub uses_canonical_control_batch: bool,
    pub control_report: Option<Control0080AdmissionReport>,
    pub observation_report: Option<Gameplay0080ObservationReport>,
    pub movement_days: Vec<Demo0080MovementDay>,
    pub observation_export: String,
    pub demo_export: String,
    pub deterministic_replay_checksum: u64,
}

pub fn run_demo_0080_0(input: &Demo0080Input) -> Demo0080Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let uses_canonical = input.control_input.commands == Control0080CommandBatch::canonical_run();
    let control_report = admit_control_0080_0(&input.control_input);
    if !control_report.admitted {
        return rejected_report(input, control_report.diagnostics);
    }

    let observation_report = control_report.observation_report.clone();
    let observation_export = control_report.text_export.clone();
    let movement_days = observation_report
        .as_ref()
        .map(build_movement_days)
        .unwrap_or_default();
    let demo_export = render_demo_export(&observation_export, &movement_days);
    admitted_report(
        input,
        uses_canonical,
        control_report,
        observation_report,
        movement_days,
        observation_export,
        demo_export,
    )
}

pub fn replay_demo_0080_0() -> (Demo0080Report, Demo0080Report) {
    let input = Demo0080Input::explicit_opt_in();
    (run_demo_0080_0(&input), run_demo_0080_0(&input))
}

pub fn canonical_control_input() -> Control0080AdmissionInput {
    Control0080AdmissionInput {
        surface: crate::Control0080Surface::with_explicit_opt_in(),
        base_schedule_input: DefaultSchedule0080Input::explicit_opt_in(),
        commands: Control0080CommandBatch::canonical_run(),
        forbidden: Default::default(),
    }
}

fn validate_surface(surface: &Demo0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("demo_0080_0_default_on_behavior_rejected");
    }
    if surface.cli_binary_requested {
        diagnostics.push("cli_binary");
    }
    if surface.direct_movement_control {
        diagnostics.push("direct_patrol_move_rejected");
    }
    if surface.external_boundary_request {
        diagnostics.push("external_boundary_request_rejected");
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

fn validate_forbidden(forbidden: &Demo0080ForbiddenRequests, diagnostics: &mut Vec<&'static str>) {
    if forbidden.cli_binary {
        diagnostics.push("cli_binary");
    }
    if forbidden.direct_movement_control {
        diagnostics.push("direct_patrol_move_rejected");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request_rejected");
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

fn build_movement_days(observation: &Gameplay0080ObservationReport) -> Vec<Demo0080MovementDay> {
    observation
        .transcript
        .steps
        .iter()
        .map(build_movement_day)
        .collect()
}

fn build_movement_day(step: &Gameplay0080StepTranscript) -> Demo0080MovementDay {
    let patrol_start = patrol_location(
        step.patrol_at_source_before_step,
        step.patrol_at_destination_before_step,
    );
    let patrol_end = if step.patrol_relocated {
        "destination"
    } else {
        patrol_start
    };
    let pirate_start = step.pirate_location_before;
    let pirate_end = step.pirate_location_after;

    Demo0080MovementDay {
        step_index: step.step_index,
        record: Demo0080MovementRecord {
            step_index: step.step_index,
            patrol_start,
            patrol_end,
            patrol_relocated: step.patrol_relocated,
            patrol_relocation_source: step.patrol_relocated.then_some("source"),
            patrol_relocation_destination: step.patrol_relocated.then_some("destination"),
            pirate_start,
            pirate_end,
            pirate_relocated: step.pirate_relocated,
            pirate_relocation_source: step
                .pirate_relocated
                .then_some(pirate_start.unwrap_or("none")),
            pirate_relocation_destination: step
                .pirate_relocated
                .then_some(pirate_end.unwrap_or("none")),
            source_supply: step.source.supply,
            source_disruption: step.source.disruption,
            source_local_security: step.source.local_security,
            destination_supply: step.destination.supply,
            destination_disruption: step.destination.disruption,
            destination_local_security: step.destination.local_security,
            threshold_accepted: step.threshold_accepted,
            event_emitted: step.event_emitted,
            boundary_request_materialized: step.boundary_request_materialized,
            production_path_invoked: step.production_path_invoked,
        },
    }
}

fn patrol_location(at_source: bool, at_destination: bool) -> &'static str {
    if at_source {
        "source"
    } else if at_destination {
        "destination"
    } else {
        "none"
    }
}

fn render_demo_export(observation_export: &str, movement_days: &[Demo0080MovementDay]) -> String {
    let mut lines = vec![
        format!(
            "DEMO-0080-0|scenario={}|headless=true",
            DEMO_0080_0_SCENARIO
        ),
        observation_export.to_string(),
    ];
    lines.push("MOVEMENT|begin".to_string());
    for day in movement_days {
        let record = &day.record;
        lines.push(format!(
            "MOVEMENT|step={}|patrol_start={}|patrol_end={}|patrol_relocated={}|patrol_from={}|patrol_to={}|pirate_start={}|pirate_end={}|pirate_relocated={}|pirate_from={}|pirate_to={}|source_supply={}|source_disruption={}|source_local_security={}|destination_supply={}|destination_disruption={}|destination_local_security={}|threshold={}|event={}|boundary={}|production_path={}",
            record.step_index,
            record.patrol_start,
            record.patrol_end,
            record.patrol_relocated,
            record.patrol_relocation_source.unwrap_or("none"),
            record.patrol_relocation_destination.unwrap_or("none"),
            record.pirate_start.unwrap_or("none"),
            record.pirate_end.unwrap_or("none"),
            record.pirate_relocated,
            record.pirate_relocation_source.unwrap_or("none"),
            record.pirate_relocation_destination.unwrap_or("none"),
            record.source_supply,
            record.source_disruption,
            record.source_local_security,
            record.destination_supply,
            record.destination_disruption,
            record.destination_local_security,
            record.threshold_accepted,
            record.event_emitted,
            record.boundary_request_materialized,
            record.production_path_invoked,
        ));
    }
    lines.push("MOVEMENT|end".to_string());
    lines.join("\n")
}

fn disabled_no_op_report(input: &Demo0080Input) -> Demo0080Report {
    base_report(
        input,
        Vec::new(),
        true,
        false,
        None,
        None,
        Vec::new(),
        String::new(),
        String::new(),
        0,
    )
}

fn rejected_report(input: &Demo0080Input, diagnostics: Vec<&'static str>) -> Demo0080Report {
    let mut report = base_report(
        input,
        diagnostics,
        false,
        false,
        None,
        None,
        Vec::new(),
        String::new(),
        String::new(),
        0,
    );
    report.admitted = false;
    report
}

fn admitted_report(
    input: &Demo0080Input,
    uses_canonical: bool,
    control_report: Control0080AdmissionReport,
    observation_report: Option<Gameplay0080ObservationReport>,
    movement_days: Vec<Demo0080MovementDay>,
    observation_export: String,
    demo_export: String,
) -> Demo0080Report {
    let checksum = checksum_demo(&demo_export, &movement_days, &control_report);
    base_report(
        input,
        Vec::new(),
        false,
        uses_canonical,
        Some(control_report),
        observation_report,
        movement_days,
        observation_export,
        demo_export,
        checksum,
    )
}

fn base_report(
    input: &Demo0080Input,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    uses_canonical: bool,
    control_report: Option<Control0080AdmissionReport>,
    observation_report: Option<Gameplay0080ObservationReport>,
    movement_days: Vec<Demo0080MovementDay>,
    observation_export: String,
    demo_export: String,
    deterministic_replay_checksum: u64,
) -> Demo0080Report {
    Demo0080Report {
        demo_id: DEMO_0080_0_ID,
        status: DEMO_0080_0_STATUS_PASS,
        scenario_name: DEMO_0080_0_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        headless_library_helper_only: true,
        cli_binary_present: false,
        direct_movement_control: input.surface.direct_movement_control,
        external_boundary_request_emitted: false,
        player_command_loop: input.surface.player_command_loop,
        ui_framework_present: input.surface.ui_framework_present,
        realtime_loop_present: input.surface.realtime_loop_present,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        semantic_or_raw_wgsl_present: false,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        clausething_dependency_present: false,
        uses_canonical_control_batch: uses_canonical,
        control_report,
        observation_report,
        movement_days,
        observation_export,
        demo_export,
        deterministic_replay_checksum,
    }
}

fn checksum_demo(
    demo_export: &str,
    movement_days: &[Demo0080MovementDay],
    control_report: &Control0080AdmissionReport,
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for byte in demo_export.as_bytes() {
        hash = fnv_append_u64(hash, *byte as u64);
    }
    hash = fnv_append_u64(hash, movement_days.len() as u64);
    hash = fnv_append_u64(hash, control_report.deterministic_replay_checksum);
    hash = fnv_append_u64(
        hash,
        control_report
            .observation_report
            .as_ref()
            .map(|report| report.deterministic_replay_checksum)
            .unwrap_or(0),
    );
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
