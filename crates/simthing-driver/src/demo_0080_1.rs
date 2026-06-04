//! `DEMO-0080-1` — Nested Starmap headless demo/export library helper. Opt-in/default-off.
//! No CLI binary. Pure read/orchestration: applies a canonical bounded `CONTROL-0080-1`
//! command batch and runs the existing control → schedule → observation/export path.
//! Adds no simulation behavior and no decision logic; SEAD remains the sole mover-decision source.

use crate::{
    admit_control_0080_1, Control0081AdmissionInput, Control0081AdmissionReport,
    Control0081BoundedConfig, Control0081CommandBatch, Control0081ForbiddenRequests,
    Control0081Surface, DefaultSchedule0081Input, DefaultSchedule0081ShipFaction,
    CONTROL_0080_1_ID, CONTROL_0080_1_STATUS_PASS,
};

pub const DEMO_0080_1_ID: &str = "DEMO-0080-1";
pub const DEMO_0080_1_SCENARIO: &str = "Nested Starmap";
pub const DEMO_0080_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - headless Nested Starmap demo/export library helper";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Demo0081Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Demo0081Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Demo0081Surface {
    pub gate: Demo0081Gate,
    pub cli_binary_present: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub player_command_loop: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub sead_bypass: bool,
    pub global_default_schedule: bool,
}

impl Demo0081Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Demo0081Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Demo0081ForbiddenRequests {
    pub cli_binary: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub sead_bypass: bool,
    pub cpu_planner_or_commitment: bool,
    pub player_command_loop: bool,
    pub ui_framework: bool,
    pub realtime_loop: bool,
    pub global_default_schedule: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub general_command_framework: bool,
    pub general_gameplay_framework: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demo0081Input {
    pub surface: Demo0081Surface,
    pub control_input: Control0081AdmissionInput,
    pub forbidden: Demo0081ForbiddenRequests,
}

impl Demo0081Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: Demo0081Surface::default_simsession(),
            control_input: Control0081AdmissionInput::default_simsession(),
            forbidden: Demo0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Demo0081Surface::with_explicit_opt_in(),
            control_input: Control0081AdmissionInput::explicit_opt_in(),
            forbidden: Demo0081ForbiddenRequests::default(),
        }
    }
}

/// A command transcript row in the demo report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demo0081CommandRow {
    pub command_index: usize,
    pub command: &'static str,
    pub accepted: bool,
    pub target_bounded_field: &'static str,
    pub old_value: i64,
    pub new_value: i64,
}

/// A movement row in the demo report, built from the observation transcript.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demo0081MovementRow {
    pub step_index: u32,
    pub mover_faction: &'static str,
    pub start_starsystem: Option<u8>,
    pub end_starsystem: Option<u8>,
    pub threshold_accepted: bool,
    pub event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub identity_preserved: bool,
    pub owner_overlay_preserved: bool,
    pub membership_updated_without_reparenting: bool,
}

fn faction_label(faction: Option<DefaultSchedule0081ShipFaction>) -> &'static str {
    match faction {
        Some(DefaultSchedule0081ShipFaction::Terran) => "Terran",
        Some(DefaultSchedule0081ShipFaction::Pirate) => "Pirate",
        None => "none",
    }
}

/// The compact demo/export report emitted by `run_demo_0080_1`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Demo0081Report {
    pub demo_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    // Guardrail confirmations.
    pub no_cli_binary: bool,
    pub no_direct_movement_command: bool,
    pub no_external_boundary_request: bool,
    pub no_sead_bypass: bool,
    pub no_cpu_planner_or_commitment: bool,
    pub no_player_command_loop: bool,
    pub no_ui_framework: bool,
    pub no_realtime_loop: bool,
    pub no_global_default_schedule: bool,
    pub no_semantic_or_raw_wgsl: bool,
    pub no_new_shader_or_gpu_kernel: bool,
    pub no_hard_currency_markets_trade_aibudget: bool,
    pub no_nested_resource_flow: bool,
    pub no_clausething_dependency: bool,

    // Control layer.
    pub control_id: &'static str,
    pub control_admitted: bool,
    pub applied_command_count: u32,
    pub command_transcript: Vec<Demo0081CommandRow>,

    // Schedule/observation layer (from the embedded control/observation report).
    pub control_ran_schedule: bool,
    pub executed_step_count: u32,
    pub terran_move_count: u32,
    pub pirate_move_count: u32,
    pub boundary_request_count: u32,
    pub identity_preserved: bool,
    pub owner_overlay_preserved: bool,
    pub membership_updated_without_reparenting: bool,

    // Summary presence flags.
    pub atlas_residency_summary_present: bool,
    pub faction_index_econ_summary_present: bool,
    pub owner_overlay_summary_present: bool,
    pub ownership_up_aggregation_summary_present: bool,
    pub sead_movement_trace_present: bool,

    pub movement_rows: Vec<Demo0081MovementRow>,

    pub demo_text_export: String,
    pub deterministic_replay_checksum: u64,
}

/// Run the headless Nested Starmap demo. Applies the canonical `CONTROL-0080-1` command batch
/// and runs the existing control → schedule → observation/export path.
/// Does not mutate simulation state beyond invoking the existing explicit opt-in path.
pub fn run_demo_0080_1(input: &Demo0081Input) -> Demo0081Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let control_report = admit_control_0080_1(&input.control_input);
    admitted_report(input, control_report)
}

pub fn replay_demo_0080_1() -> (Demo0081Report, Demo0081Report) {
    let input = Demo0081Input::explicit_opt_in();
    (run_demo_0080_1(&input), run_demo_0080_1(&input))
}

/// Returns the canonical control input used by the demo's default run.
pub fn canonical_control_input_0080_1() -> Control0081AdmissionInput {
    Control0081AdmissionInput {
        surface: Control0081Surface::with_explicit_opt_in(),
        base_schedule_input: DefaultSchedule0081Input::explicit_opt_in(),
        commands: Control0081CommandBatch::canonical_run(),
        forbidden: Control0081ForbiddenRequests::default(),
    }
}

fn validate_surface(surface: &Demo0081Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("demo_0080_1_default_on_behavior_rejected");
    }
    if surface.cli_binary_present {
        diagnostics.push("cli_binary_not_authorized");
    }
    if surface.ui_framework_present {
        diagnostics.push("ui_framework");
    }
    if surface.realtime_loop_present {
        diagnostics.push("realtime_loop");
    }
    if surface.player_command_loop {
        diagnostics.push("player_command_loop");
    }
    if surface.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if surface.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if surface.sead_bypass {
        diagnostics.push("sead_bypass");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
}

fn validate_forbidden(forbidden: &Demo0081ForbiddenRequests, diagnostics: &mut Vec<&'static str>) {
    if forbidden.cli_binary {
        diagnostics.push("cli_binary_not_authorized");
    }
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.sead_bypass {
        diagnostics.push("sead_bypass");
    }
    if forbidden.cpu_planner_or_commitment {
        diagnostics.push("cpu_planner_or_commitment");
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
    if forbidden.general_command_framework {
        diagnostics.push("general_command_framework");
    }
    if forbidden.general_gameplay_framework {
        diagnostics.push("general_gameplay_framework");
    }
}

fn disabled_no_op_report(input: &Demo0081Input) -> Demo0081Report {
    base_report(input, true, Vec::new(), empty_control_report())
}

fn rejected_report(input: &Demo0081Input, diagnostics: Vec<&'static str>) -> Demo0081Report {
    let mut report = base_report(input, false, diagnostics, empty_control_report());
    report.admitted = false;
    report
}

fn admitted_report(
    input: &Demo0081Input,
    control_report: Control0081AdmissionReport,
) -> Demo0081Report {
    base_report(input, false, Vec::new(), control_report)
}

fn base_report(
    input: &Demo0081Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    control_report: Control0081AdmissionReport,
) -> Demo0081Report {
    let opt_in = input.surface.gate.explicit_opt_in;
    let active = !disabled_no_op && opt_in;

    // Extract summaries from embedded observation report.
    let obs = control_report.observation_report.as_ref();

    let executed_step_count = obs.map(|o| o.executed_step_count).unwrap_or(0);
    let terran_move_count = obs.map(|o| o.summary.terran_movement_rows).unwrap_or(0);
    let pirate_move_count = obs.map(|o| o.summary.pirate_movement_rows).unwrap_or(0);
    let boundary_request_count = obs
        .map(|o| {
            o.transcript
                .rows
                .iter()
                .filter(|r| r.boundary_request_materialized)
                .count() as u32
        })
        .unwrap_or(0);
    let identity_preserved = obs
        .map(|o| o.transcript.rows.iter().all(|r| r.identity_preserved))
        .unwrap_or(true);
    let owner_overlay_preserved = obs
        .map(|o| o.transcript.rows.iter().all(|r| r.owner_overlay_preserved))
        .unwrap_or(true);
    let membership_without_reparenting = obs
        .map(|o| {
            o.transcript
                .rows
                .iter()
                .all(|r| r.membership_updated_without_reparenting)
        })
        .unwrap_or(true);

    let atlas_present = obs
        .map(|o| !o.summary.atlas.active_theaters.is_empty())
        .unwrap_or(false);
    let econ_present = obs
        .map(|o| o.summary.faction_econ.faction_count > 0)
        .unwrap_or(false);
    let owner_present = obs
        .map(|o| !o.summary.owner_overlay_inheritance_summary.is_empty())
        .unwrap_or(false);
    let up_agg_present = obs
        .map(|o| !o.summary.ownership_up_aggregation_summary.is_empty())
        .unwrap_or(false);
    let sead_trace_present = obs
        .map(|o| o.summary.sead_movement_trace_included)
        .unwrap_or(false);

    // Build command transcript rows.
    let command_transcript = control_report
        .command_transcript
        .iter()
        .map(|row| Demo0081CommandRow {
            command_index: row.command_index,
            command: row.command,
            accepted: row.accepted,
            target_bounded_field: row.target_bounded_field,
            old_value: row.old_value,
            new_value: row.new_value,
        })
        .collect::<Vec<_>>();

    // Build movement rows from the observation transcript.
    let movement_rows = if let Some(obs_report) = obs {
        obs_report
            .transcript
            .rows
            .iter()
            .map(|row| Demo0081MovementRow {
                step_index: row.step_index,
                mover_faction: faction_label(row.mover_faction),
                start_starsystem: row.start_starsystem,
                end_starsystem: row.end_starsystem,
                threshold_accepted: row.threshold_accepted,
                event_emitted: row.event_emitted,
                boundary_request_materialized: row.boundary_request_materialized,
                identity_preserved: row.identity_preserved,
                owner_overlay_preserved: row.owner_overlay_preserved,
                membership_updated_without_reparenting: row.membership_updated_without_reparenting,
            })
            .collect()
    } else {
        Vec::new()
    };

    let demo_text_export = if active {
        render_demo_export(&control_report, &command_transcript, &movement_rows)
    } else {
        String::new()
    };

    let mut report = Demo0081Report {
        demo_id: DEMO_0080_1_ID,
        status: DEMO_0080_1_STATUS_PASS,
        scenario_name: DEMO_0080_1_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        no_cli_binary: !input.surface.cli_binary_present && !input.forbidden.cli_binary,
        no_direct_movement_command: !input.surface.direct_movement_command
            && !input.forbidden.direct_movement_command,
        no_external_boundary_request: !input.surface.external_boundary_request
            && !input.forbidden.external_boundary_request,
        no_sead_bypass: !input.surface.sead_bypass && !input.forbidden.sead_bypass,
        no_cpu_planner_or_commitment: !input.forbidden.cpu_planner_or_commitment,
        no_player_command_loop: !input.surface.player_command_loop
            && !input.forbidden.player_command_loop,
        no_ui_framework: !input.surface.ui_framework_present && !input.forbidden.ui_framework,
        no_realtime_loop: !input.surface.realtime_loop_present && !input.forbidden.realtime_loop,
        no_global_default_schedule: !input.surface.global_default_schedule
            && !input.forbidden.global_default_schedule,
        no_semantic_or_raw_wgsl: !input.forbidden.semantic_or_raw_wgsl,
        no_new_shader_or_gpu_kernel: !input.forbidden.new_shader_or_gpu_kernel,
        no_hard_currency_markets_trade_aibudget: !input
            .forbidden
            .hard_currency_markets_trade_aibudget,
        no_nested_resource_flow: !input.forbidden.nested_resource_flow,
        no_clausething_dependency: !input.forbidden.clausething_dependency,
        control_id: CONTROL_0080_1_ID,
        control_admitted: control_report.admitted && !control_report.disabled_no_op,
        applied_command_count: control_report.applied_command_count,
        command_transcript,
        control_ran_schedule: active && control_report.observation_report.is_some(),
        executed_step_count,
        terran_move_count,
        pirate_move_count,
        boundary_request_count,
        identity_preserved,
        owner_overlay_preserved,
        membership_updated_without_reparenting: membership_without_reparenting,
        atlas_residency_summary_present: atlas_present,
        faction_index_econ_summary_present: econ_present,
        owner_overlay_summary_present: owner_present,
        ownership_up_aggregation_summary_present: up_agg_present,
        sead_movement_trace_present: sead_trace_present,
        movement_rows,
        demo_text_export,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn render_demo_export(
    control_report: &Control0081AdmissionReport,
    command_rows: &[Demo0081CommandRow],
    movement_rows: &[Demo0081MovementRow],
) -> String {
    let obs = control_report.observation_report.as_ref();
    let mut lines = Vec::new();
    lines.push(format!(
        "DEMO-0080-1|scenario={}|control_id={}|commands_applied={}|steps={}|terran_moves={}|pirate_moves={}|identity_preserved={}|owner_overlay_preserved={}|checksum={}",
        DEMO_0080_1_SCENARIO,
        CONTROL_0080_1_ID,
        control_report.applied_command_count,
        obs.map(|o| o.executed_step_count).unwrap_or(0),
        obs.map(|o| o.summary.terran_movement_rows).unwrap_or(0),
        obs.map(|o| o.summary.pirate_movement_rows).unwrap_or(0),
        obs.map(|o| o.transcript.rows.iter().all(|r| r.identity_preserved)).unwrap_or(true),
        obs.map(|o| o.transcript.rows.iter().all(|r| r.owner_overlay_preserved)).unwrap_or(true),
        control_report.deterministic_replay_checksum,
    ));

    for row in command_rows {
        lines.push(format!(
            "CMD|index={}|command={}|accepted={}|field={}|old={}|new={}",
            row.command_index,
            row.command,
            row.accepted,
            row.target_bounded_field,
            row.old_value,
            row.new_value,
        ));
    }

    for row in movement_rows {
        lines.push(format!(
            "MOVE|step={}|faction={}|start={:?}|end={:?}|threshold={}|event={}|boundary={}|identity={}|owner_overlay={}|no_reparenting={}",
            row.step_index,
            row.mover_faction,
            row.start_starsystem,
            row.end_starsystem,
            row.threshold_accepted,
            row.event_emitted,
            row.boundary_request_materialized,
            row.identity_preserved,
            row.owner_overlay_preserved,
            row.membership_updated_without_reparenting,
        ));
    }

    lines.join("\n")
}

fn checksum_report(report: &Demo0081Report) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv(hash, report.applied_command_count as u64);
    hash = fnv(hash, report.executed_step_count as u64);
    hash = fnv(hash, report.terran_move_count as u64);
    hash = fnv(hash, report.pirate_move_count as u64);
    hash = fnv(hash, report.boundary_request_count as u64);
    hash = fnv(hash, report.identity_preserved as u64);
    hash = fnv(hash, report.owner_overlay_preserved as u64);
    for row in &report.movement_rows {
        hash = fnv(hash, row.step_index as u64);
        hash = fnv(hash, row.threshold_accepted as u64);
        hash = fnv(hash, row.boundary_request_materialized as u64);
        hash = fnv(hash, row.identity_preserved as u64);
        hash = fnv(hash, row.owner_overlay_preserved as u64);
    }
    hash
}

fn fnv(mut hash: u64, value: u64) -> u64 {
    hash ^= value;
    hash.wrapping_mul(0x0000_0100_0000_01B3)
}

fn empty_control_report() -> Control0081AdmissionReport {
    Control0081AdmissionReport {
        control_id: CONTROL_0080_1_ID,
        status: CONTROL_0080_1_STATUS_PASS,
        scenario_name: DEMO_0080_1_SCENARIO,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: false,
        default_off: true,
        disabled_no_op: true,
        bounded_command_admission_only: true,
        command_writes_existing_bounded_values_only: true,
        command_moved_ship: false,
        command_emitted_boundary_request: false,
        command_bypassed_sead: false,
        direct_movement_control: false,
        player_command_loop: false,
        ui_framework_present: false,
        realtime_loop_present: false,
        global_default_schedule_registered: false,
        demo_packaging_present: false,
        semantic_or_raw_wgsl_present: false,
        new_shader_or_gpu_kernel: false,
        cpu_planner_used: false,
        hard_currency_markets_trade_aibudget: false,
        nested_resource_flow: false,
        clausething_dependency_present: false,
        closed_ladders_reopened: false,
        applied_command_count: 0,
        rejected_commands: Vec::new(),
        command_transcript: Vec::new(),
        schedule_input: DefaultSchedule0081Input::default_simsession(),
        bounded_config: Control0081BoundedConfig::default(),
        observation_report: None,
        text_export: String::new(),
        deterministic_replay_checksum: 0,
    }
}
