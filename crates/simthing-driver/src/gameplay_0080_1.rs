use crate::{
    run_default_schedule_0080_1, Atlas0080TheaterId, DefaultSchedule0081Input,
    DefaultSchedule0081RunReport, DefaultSchedule0081ShipFaction, ProductionPath0081Report,
    DEFAULT_SCHEDULE_0080_1_ID,
};

pub const GAMEPLAY_0080_1_ID: &str = "GAMEPLAY-0080-1";
pub const GAMEPLAY_0080_1_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - read-only Nested Starmap observation/export";
pub const GAMEPLAY_0080_1_SCENARIO: &str = "Nested Starmap";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0081Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Gameplay0081Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0081Surface {
    pub gate: Gameplay0081Gate,
    pub control_input_present: bool,
    pub command_input_present: bool,
    pub player_command_loop_present: bool,
    pub demo_packaging_present: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
}

impl Gameplay0081Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Gameplay0081Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gameplay0081ForbiddenRequests {
    pub control_or_command_input: bool,
    pub demo_packaging: bool,
    pub player_command_loop: bool,
    pub ui_framework: bool,
    pub realtime_loop: bool,
    pub global_default_schedule: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
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
    pub general_gameplay_framework: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081Input {
    pub surface: Gameplay0081Surface,
    pub schedule_input: Option<DefaultSchedule0081Input>,
    pub schedule_report: Option<DefaultSchedule0081RunReport>,
    pub forbidden: Gameplay0081ForbiddenRequests,
}

impl Gameplay0081Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: Gameplay0081Surface::default_simsession(),
            schedule_input: None,
            schedule_report: None,
            forbidden: Gameplay0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Gameplay0081Surface::with_explicit_opt_in(),
            schedule_input: Some(DefaultSchedule0081Input::explicit_opt_in()),
            schedule_report: None,
            forbidden: Gameplay0081ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in_from_report(report: DefaultSchedule0081RunReport) -> Self {
        Self {
            surface: Gameplay0081Surface::with_explicit_opt_in(),
            schedule_input: None,
            schedule_report: Some(report),
            forbidden: Gameplay0081ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081StarmapShape {
    pub starmap_side: u32,
    pub starsystem_count: usize,
    pub starsystem_side: u32,
    pub planets_per_starsystem: u32,
    pub planet_side: u32,
    pub logical_location_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081AtlasSummary {
    pub active_theaters: Vec<Atlas0080TheaterId>,
    pub resident_theaters: Vec<Atlas0080TheaterId>,
    pub sparse_residency_composed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081FactionEconSummary {
    pub fixed_terran_pirate_faction_set: bool,
    pub pirate_full_economy_participation: bool,
    pub contended_econ_visible: bool,
    pub faction_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081Summary {
    pub starmap_shape: Gameplay0081StarmapShape,
    pub atlas: Gameplay0081AtlasSummary,
    pub faction_econ: Gameplay0081FactionEconSummary,
    pub owner_overlay_inheritance_summary: String,
    pub ownership_up_aggregation_summary: String,
    pub sead_movement_trace_included: bool,
    pub terran_movement_rows: u32,
    pub pirate_movement_rows: u32,
    pub no_mover_rows: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081MovementRow {
    pub step_index: u32,
    pub mover_id: Option<u64>,
    pub mover_faction: Option<DefaultSchedule0081ShipFaction>,
    pub start_starsystem: Option<u8>,
    pub start_theater: Option<String>,
    pub end_starsystem: Option<u8>,
    pub end_theater: Option<String>,
    pub threshold_accepted: bool,
    pub event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub identity_preserved: bool,
    pub owner_overlay_preserved: bool,
    pub membership_updated_without_reparenting: bool,
    pub replay_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081Transcript {
    pub rows: Vec<Gameplay0081MovementRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gameplay0081ObservationReport {
    pub observation_id: &'static str,
    pub status: &'static str,
    pub scenario_id: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub read_only: bool,
    pub schedule_report_consumed: bool,
    pub schedule_invoked_by_observer: bool,
    pub observer_emitted_events: bool,
    pub observer_materialized_boundary_requests: bool,

    pub control_or_command_input_present: bool,
    pub demo_packaging_present: bool,
    pub player_command_loop_present: bool,
    pub ui_framework_present: bool,
    pub realtime_loop_present: bool,
    pub global_default_schedule_registered: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub unbounded_factions: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub capture_as_reparenting: bool,
    pub clausething_dependency_present: bool,
    pub simthing_spec_altered: bool,
    pub invariant_edited: bool,
    pub passive_proof_wrapper_present: bool,
    pub general_gameplay_framework_present: bool,

    pub schedule_id: &'static str,
    pub schedule_admitted: bool,
    pub executed_step_count: u32,
    pub replay_checksum: u64,
    pub summary: Gameplay0081Summary,
    pub transcript: Gameplay0081Transcript,
    pub text_export: String,
}

pub fn observe_gameplay_0080_1(input: &Gameplay0081Input) -> Gameplay0081ObservationReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let schedule_invoked_by_observer = input.schedule_report.is_none();
    let schedule_report = resolve_schedule_report(input);
    admitted_report(input, schedule_report, schedule_invoked_by_observer)
}

pub fn replay_observe_gameplay_0080_1(
) -> (Gameplay0081ObservationReport, Gameplay0081ObservationReport) {
    let input = Gameplay0081Input::explicit_opt_in();
    (
        observe_gameplay_0080_1(&input),
        observe_gameplay_0080_1(&input),
    )
}

pub fn export_gameplay_0080_1_text(report: &Gameplay0081ObservationReport) -> String {
    report.text_export.clone()
}

fn resolve_schedule_report(input: &Gameplay0081Input) -> DefaultSchedule0081RunReport {
    if let Some(report) = &input.schedule_report {
        return report.clone();
    }
    let schedule_input = input
        .schedule_input
        .clone()
        .unwrap_or_else(DefaultSchedule0081Input::explicit_opt_in);
    run_default_schedule_0080_1(&schedule_input)
}

fn validate_surface(surface: &Gameplay0081Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("gameplay_0080_1_default_on_behavior_rejected");
    }
    if surface.control_input_present || surface.command_input_present {
        diagnostics.push("control_or_command_input");
    }
    if surface.demo_packaging_present {
        diagnostics.push("demo_packaging");
    }
    if surface.player_command_loop_present {
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
    forbidden: &Gameplay0081ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.control_or_command_input {
        diagnostics.push("control_or_command_input");
    }
    if forbidden.demo_packaging {
        diagnostics.push("demo_packaging");
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
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
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
    if forbidden.general_gameplay_framework {
        diagnostics.push("general_gameplay_framework");
    }
}

fn disabled_no_op_report(input: &Gameplay0081Input) -> Gameplay0081ObservationReport {
    base_report(
        input,
        true,
        true,
        Vec::new(),
        empty_summary(),
        Gameplay0081Transcript { rows: Vec::new() },
        String::new(),
        false,
        0,
    )
}

fn rejected_report(
    input: &Gameplay0081Input,
    diagnostics: Vec<&'static str>,
) -> Gameplay0081ObservationReport {
    base_report(
        input,
        false,
        false,
        diagnostics,
        empty_summary(),
        Gameplay0081Transcript { rows: Vec::new() },
        String::new(),
        false,
        0,
    )
}

fn admitted_report(
    input: &Gameplay0081Input,
    schedule_report: DefaultSchedule0081RunReport,
    schedule_invoked_by_observer: bool,
) -> Gameplay0081ObservationReport {
    let production_path = schedule_report.production_path_report.as_ref();
    let transcript = build_transcript(&schedule_report);
    let summary = build_summary(production_path, &transcript);
    let text_export = render_text_export(&schedule_report, &summary, &transcript);
    let checksum = schedule_report.deterministic_replay_checksum;
    base_report(
        input,
        schedule_report.admitted
            && !schedule_report.disabled_no_op
            && schedule_report.diagnostics.is_empty(),
        false,
        schedule_report.diagnostics,
        summary,
        transcript,
        text_export,
        schedule_invoked_by_observer,
        checksum,
    )
}

fn base_report(
    input: &Gameplay0081Input,
    admitted: bool,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    summary: Gameplay0081Summary,
    transcript: Gameplay0081Transcript,
    text_export: String,
    schedule_invoked_by_observer: bool,
    replay_checksum: u64,
) -> Gameplay0081ObservationReport {
    Gameplay0081ObservationReport {
        observation_id: GAMEPLAY_0080_1_ID,
        status: GAMEPLAY_0080_1_STATUS_PASS,
        scenario_id: "SCENARIO-0080-1",
        scenario_name: GAMEPLAY_0080_1_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        read_only: true,
        schedule_report_consumed: admitted && !disabled_no_op,
        schedule_invoked_by_observer,
        observer_emitted_events: false,
        observer_materialized_boundary_requests: false,
        control_or_command_input_present: input.surface.control_input_present
            || input.surface.command_input_present
            || input.forbidden.control_or_command_input,
        demo_packaging_present: input.surface.demo_packaging_present
            || input.forbidden.demo_packaging,
        player_command_loop_present: input.surface.player_command_loop_present
            || input.forbidden.player_command_loop,
        ui_framework_present: input.surface.ui_framework_present || input.forbidden.ui_framework,
        realtime_loop_present: input.surface.realtime_loop_present || input.forbidden.realtime_loop,
        global_default_schedule_registered: input.surface.global_default_schedule_registered
            || input.forbidden.global_default_schedule,
        direct_movement_command: input.forbidden.direct_movement_command,
        external_boundary_request: input.forbidden.external_boundary_request,
        cpu_planner_urgency_commitment: input.forbidden.cpu_planner_urgency_commitment,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        new_shader_or_gpu_kernel: input.forbidden.new_shader_or_gpu_kernel,
        hard_currency_markets_trade_aibudget: input.forbidden.hard_currency_markets_trade_aibudget,
        nested_resource_flow: input.forbidden.nested_resource_flow,
        unbounded_factions: input.forbidden.unbounded_factions,
        owner_entity_as_spatial_parent: input.forbidden.owner_entity_as_spatial_parent,
        capture_as_reparenting: input.forbidden.capture_as_reparenting,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        simthing_spec_altered: input.forbidden.simthing_spec_alteration,
        invariant_edited: input.forbidden.invariant_edit,
        passive_proof_wrapper_present: input.forbidden.passive_proof_wrapper,
        general_gameplay_framework_present: input.forbidden.general_gameplay_framework,
        schedule_id: DEFAULT_SCHEDULE_0080_1_ID,
        schedule_admitted: admitted && !disabled_no_op,
        executed_step_count: transcript.rows.len() as u32,
        replay_checksum,
        summary,
        transcript,
        text_export,
    }
}

fn build_transcript(schedule_report: &DefaultSchedule0081RunReport) -> Gameplay0081Transcript {
    let rows = schedule_report
        .step_reports
        .iter()
        .map(|step| {
            let movement = step.movement.as_ref();
            Gameplay0081MovementRow {
                step_index: step.step.step_index,
                mover_id: step.step.mover_id,
                mover_faction: step.step.mover_faction,
                start_starsystem: step.step.start_starsystem,
                start_theater: step.step.start_starsystem.map(starsystem_theater_label),
                end_starsystem: step.step.end_starsystem,
                end_theater: step.step.end_starsystem.map(starsystem_theater_label),
                threshold_accepted: step.decision.threshold_accepted,
                event_emitted: step.decision.event_emitted,
                boundary_request_materialized: step.decision.boundary_request_materialized,
                identity_preserved: movement.map_or(true, |movement| movement.identity_preserved),
                owner_overlay_preserved: movement
                    .map_or(true, |movement| movement.owner_overlay_preserved),
                membership_updated_without_reparenting: movement.map_or(true, |movement| {
                    movement.membership_updated_without_reparenting
                }),
                replay_checksum: schedule_report.deterministic_replay_checksum,
            }
        })
        .collect();
    Gameplay0081Transcript { rows }
}

fn build_summary(
    production_path: Option<&ProductionPath0081Report>,
    transcript: &Gameplay0081Transcript,
) -> Gameplay0081Summary {
    let terran_movement_rows = transcript
        .rows
        .iter()
        .filter(|row| row.mover_faction == Some(DefaultSchedule0081ShipFaction::Terran))
        .count() as u32;
    let pirate_movement_rows = transcript
        .rows
        .iter()
        .filter(|row| row.mover_faction == Some(DefaultSchedule0081ShipFaction::Pirate))
        .count() as u32;
    let no_mover_rows = transcript
        .rows
        .iter()
        .filter(|row| row.mover_id.is_none())
        .count() as u32;

    if let Some(path) = production_path {
        return Gameplay0081Summary {
            starmap_shape: Gameplay0081StarmapShape {
                starmap_side: path.starmap_side,
                starsystem_count: path.starsystem_count,
                starsystem_side: path.starsystem_side,
                planets_per_starsystem: path.planets_per_starsystem,
                planet_side: path.planet_side,
                logical_location_count: path.logical_location_count,
            },
            atlas: Gameplay0081AtlasSummary {
                active_theaters: path.active_theaters.clone(),
                resident_theaters: path.resident_theaters.clone(),
                sparse_residency_composed: path.sparse_residency_composed,
            },
            faction_econ: Gameplay0081FactionEconSummary {
                fixed_terran_pirate_faction_set: path.fixed_terran_pirate_faction_set,
                pirate_full_economy_participation: path.pirate_full_economy_faction_visible,
                contended_econ_visible: path.contended_clearing_reports_visible,
                faction_count: path.factions.len(),
            },
            owner_overlay_inheritance_summary: format!(
                "owners terran={} pirate={} inherit_location_weights={} inherit_ship_weights={} owner_as_spatial_parent={}",
                path.owner_overlay_summary.terran_owner_id,
                path.owner_overlay_summary.pirate_owner_id,
                path.owner_overlay_summary.location_owner_overlays_inherit_numeric_weights,
                path.owner_overlay_summary.ship_owner_overlays_inherit_faction_weights,
                path.owner_overlay_summary.owner_entity_as_spatial_parent,
            ),
            ownership_up_aggregation_summary: format!(
                "planet_to_starsystem={} terran_owned_planets={} terran_owned_starsystems={} neutral_starsystems={} capture_as_reparenting={}",
                path.ownership_aggregation_summary.planet_to_starsystem_up_aggregation,
                path.ownership_aggregation_summary.terran_owned_planets,
                path.ownership_aggregation_summary.terran_owned_starsystems_derived,
                path.ownership_aggregation_summary.neutral_starsystems,
                path.ownership_aggregation_summary.capture_as_reparenting,
            ),
            sead_movement_trace_included: !transcript.rows.is_empty(),
            terran_movement_rows,
            pirate_movement_rows,
            no_mover_rows,
        };
    }

    Gameplay0081Summary {
        starmap_shape: empty_shape(),
        atlas: Gameplay0081AtlasSummary {
            active_theaters: Vec::new(),
            resident_theaters: Vec::new(),
            sparse_residency_composed: false,
        },
        faction_econ: Gameplay0081FactionEconSummary {
            fixed_terran_pirate_faction_set: false,
            pirate_full_economy_participation: false,
            contended_econ_visible: false,
            faction_count: 0,
        },
        owner_overlay_inheritance_summary: String::new(),
        ownership_up_aggregation_summary: String::new(),
        sead_movement_trace_included: false,
        terran_movement_rows,
        pirate_movement_rows,
        no_mover_rows,
    }
}

fn render_text_export(
    schedule_report: &DefaultSchedule0081RunReport,
    summary: &Gameplay0081Summary,
    transcript: &Gameplay0081Transcript,
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "GAMEPLAY-0080-1|scenario_id=SCENARIO-0080-1|scenario_name={}|schedule_id={}|schedule_status={}|starmap={}x{}|starsystems={}|starsystem_side={}|planet_side={}|logical_locations={}|active_theaters={}|resident_theaters={}|fixed_factions={}|pirate_full_economy={}|contended_econ={}|checksum={}",
        schedule_report.scenario,
        schedule_report.schedule_id,
        schedule_report.status,
        summary.starmap_shape.starmap_side,
        summary.starmap_shape.starmap_side,
        summary.starmap_shape.starsystem_count,
        summary.starmap_shape.starsystem_side,
        summary.starmap_shape.planet_side,
        summary.starmap_shape.logical_location_count,
        theater_list(&summary.atlas.active_theaters),
        theater_list(&summary.atlas.resident_theaters),
        summary.faction_econ.fixed_terran_pirate_faction_set,
        summary.faction_econ.pirate_full_economy_participation,
        summary.faction_econ.contended_econ_visible,
        schedule_report.deterministic_replay_checksum,
    ));
    lines.push(format!(
        "OWNER|overlay={}|up_aggregation={}",
        summary.owner_overlay_inheritance_summary, summary.ownership_up_aggregation_summary
    ));
    for row in &transcript.rows {
        lines.push(format!(
            "MOVE|step={}|mover_id={}|mover_faction={}|start_starsystem={}|start_theater={}|end_starsystem={}|end_theater={}|threshold_accepted={}|event_emitted={}|boundary_request_materialized={}|identity_preserved={}|owner_overlay_preserved={}|membership_updated_without_reparenting={}|checksum={}",
            row.step_index,
            row.mover_id.map_or("none".to_string(), |id| id.to_string()),
            row.mover_faction.map_or("none".to_string(), faction_label),
            row.start_starsystem.map_or("none".to_string(), |id| id.to_string()),
            row.start_theater.as_deref().unwrap_or("none"),
            row.end_starsystem.map_or("none".to_string(), |id| id.to_string()),
            row.end_theater.as_deref().unwrap_or("none"),
            row.threshold_accepted,
            row.event_emitted,
            row.boundary_request_materialized,
            row.identity_preserved,
            row.owner_overlay_preserved,
            row.membership_updated_without_reparenting,
            row.replay_checksum,
        ));
    }
    lines.join("\n")
}

fn empty_summary() -> Gameplay0081Summary {
    Gameplay0081Summary {
        starmap_shape: empty_shape(),
        atlas: Gameplay0081AtlasSummary {
            active_theaters: Vec::new(),
            resident_theaters: Vec::new(),
            sparse_residency_composed: false,
        },
        faction_econ: Gameplay0081FactionEconSummary {
            fixed_terran_pirate_faction_set: false,
            pirate_full_economy_participation: false,
            contended_econ_visible: false,
            faction_count: 0,
        },
        owner_overlay_inheritance_summary: String::new(),
        ownership_up_aggregation_summary: String::new(),
        sead_movement_trace_included: false,
        terran_movement_rows: 0,
        pirate_movement_rows: 0,
        no_mover_rows: 0,
    }
}

fn empty_shape() -> Gameplay0081StarmapShape {
    Gameplay0081StarmapShape {
        starmap_side: 0,
        starsystem_count: 0,
        starsystem_side: 0,
        planets_per_starsystem: 0,
        planet_side: 0,
        logical_location_count: 0,
    }
}

fn starsystem_theater_label(index: u8) -> String {
    format!("starsystem-{index}")
}

fn theater_list(theaters: &[Atlas0080TheaterId]) -> String {
    theaters
        .iter()
        .map(|theater| match theater {
            Atlas0080TheaterId::Starmap => "starmap".to_string(),
            Atlas0080TheaterId::Starsystem { index } => format!("starsystem-{index}"),
            Atlas0080TheaterId::Planet { starsystem_index } => {
                format!("planet-starsystem-{starsystem_index}")
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn faction_label(faction: DefaultSchedule0081ShipFaction) -> String {
    match faction {
        DefaultSchedule0081ShipFaction::Terran => "Terran".to_string(),
        DefaultSchedule0081ShipFaction::Pirate => "Pirate".to_string(),
    }
}
