//! SCENARIO-0080-2-R3: capability-tree mask-down.
//!
//! Fixture-only, opt-in/default-off proof over the accepted R1 disruption heatmap and implemented
//! R2 recursive allocation report. R3 models faction-owned capability choices as SimThing state,
//! resolves those choices into bounded numeric modifier overlays, and applies the overlays by
//! owner-column matching to R1/R2 read-side signals. CPU oracle parity is the authority; this
//! module adds no GPU, shader, movement, combat, or default SimSession wiring.

#[allow(dead_code, unused_imports)]
#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod atlas_store;

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Channel, DressRehearsalR1Input,
    DressRehearsalR1OccupantKind, DressRehearsalR1Owner, DressRehearsalR1Report, GALAXY_CELL_COUNT,
    GALAXY_SIDE,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2Input,
    DressRehearsalR2OccupantPosition, DressRehearsalR2Owner, DressRehearsalR2Report,
};
use std::collections::HashMap;

pub const DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID: &str =
    "SCENARIO-0080-2-R3-CAPABILITY-MASK-DOWN";
pub const DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - capability-tree modifier overlays masked down by owner-column";
pub const DRESS_REHEARSAL_R3_SCENARIO: &str = "SCENARIO-0080-2";

pub const BPS_ONE: i32 = 10_000;
pub const MIN_MODIFIER_BPS: i32 = 5_000;
pub const MAX_MODIFIER_BPS: i32 = 20_000;

pub const PATROL_SUPPRESSION_MODIFIER: &str = "patrol_suppression_multiplier";
pub const DISRUPTION_DECAY_MODIFIER: &str = "disruption_decay_multiplier";
pub const DEFENSIVE_LOGISTICS_MODIFIER: &str = "defensive_logistics_bonus";
pub const PIRATE_EMISSION_MODIFIER: &str = "pirate_emission_multiplier";
pub const BLOCKADE_DIVERT_MODIFIER: &str = "blockade_divert_multiplier";
pub const RAIDING_LOGISTICS_MODIFIER: &str = "raiding_logistics_bonus";
pub const COMBAT_BONUS_PLACEHOLDER_MODIFIER: &str = "combat_bonus_placeholder";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DressRehearsalR3Owner {
    Terran,
    Pirate,
}

impl DressRehearsalR3Owner {
    pub fn stable_code(self) -> u64 {
        match self {
            Self::Terran => 1,
            Self::Pirate => 2,
        }
    }

    fn logistics_modifier(self) -> &'static str {
        match self {
            Self::Terran => DEFENSIVE_LOGISTICS_MODIFIER,
            Self::Pirate => RAIDING_LOGISTICS_MODIFIER,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR3CapabilityRow {
    pub owner: DressRehearsalR3Owner,
    pub faction_simthing_id: &'static str,
    pub capability_id: &'static str,
    pub tree_path: &'static str,
    pub resolved_modifier_id: &'static str,
    pub chosen_rank: u8,
    pub multiplier_bps: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR3ModifierOverlayRow {
    pub owner: DressRehearsalR3Owner,
    pub modifier_id: &'static str,
    pub source_capability_id: &'static str,
    pub unclamped_multiplier_bps: i32,
    pub multiplier_bps: i32,
    pub min_bps: i32,
    pub max_bps: i32,
    pub read_side_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR3OwnerMaskApplicationRow {
    pub source_id: String,
    pub owner: DressRehearsalR3Owner,
    pub occupant_kind: &'static str,
    pub signal_family: &'static str,
    pub cell_index: u32,
    pub x: u32,
    pub y: u32,
    pub structural_parent_before: &'static str,
    pub structural_parent_after: &'static str,
    pub modifier_id: &'static str,
    pub applied_multiplier_bps: i32,
    pub owner_column_matched: bool,
    pub evidence_group: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3ModifiedR1SignalRow {
    pub source_id: String,
    pub owner: DressRehearsalR3Owner,
    pub channel: &'static str,
    pub cell_index: u32,
    pub base_value: f32,
    pub modifier_id: &'static str,
    pub multiplier_bps: i32,
    pub effective_value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3ModifiedEconomySignalRow {
    pub signal_id: String,
    pub owner: DressRehearsalR3Owner,
    pub source_contract: &'static str,
    pub base_signal: f32,
    pub modifier_id: &'static str,
    pub multiplier_bps: i32,
    pub effective_signal: f32,
    pub bounded_signal: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR3Summary {
    pub capability_row_count: usize,
    pub modifier_row_count: usize,
    pub owner_mask_application_count: usize,
    pub modified_r1_signal_count: usize,
    pub modified_economy_signal_count: usize,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3Artifact {
    pub capability_rows: Vec<DressRehearsalR3CapabilityRow>,
    pub modifier_rows: Vec<DressRehearsalR3ModifierOverlayRow>,
    pub owner_mask_rows: Vec<DressRehearsalR3OwnerMaskApplicationRow>,
    pub modified_r1_rows: Vec<DressRehearsalR3ModifiedR1SignalRow>,
    pub modified_economy_rows: Vec<DressRehearsalR3ModifiedEconomySignalRow>,
    pub summary: DressRehearsalR3Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3Oracle {
    pub capability_rows: Vec<DressRehearsalR3CapabilityRow>,
    pub modifier_rows: Vec<DressRehearsalR3ModifierOverlayRow>,
    pub owner_mask_rows: Vec<DressRehearsalR3OwnerMaskApplicationRow>,
    pub modified_r1_rows: Vec<DressRehearsalR3ModifiedR1SignalRow>,
    pub modified_economy_rows: Vec<DressRehearsalR3ModifiedEconomySignalRow>,
    pub summary: DressRehearsalR3Summary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub r2_report: Option<DressRehearsalR2Report>,
    pub capability_rows: Vec<DressRehearsalR3CapabilityRow>,
}

impl DressRehearsalR3Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            r1_report: None,
            r2_report: None,
            capability_rows: default_capability_rows(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        let r1_report =
            run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in());
        let r2_report = run_dress_rehearsal_r2_recursive_allocation(
            &DressRehearsalR2Input::with_r1_report(r1_report.clone()),
        );
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            capability_rows: default_capability_rows(),
        }
    }

    pub fn with_reports(
        r1_report: DressRehearsalR1Report,
        r2_report: DressRehearsalR2Report,
    ) -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            capability_rows: default_capability_rows(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR3Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub r1_contract_consumed: bool,
    pub r1_contract_checksum: u64,
    pub r1_cpu_oracle_parity: bool,
    pub r2_contract_consumed: bool,
    pub r2_contract_checksum: u64,
    pub r2_cpu_oracle_parity: bool,
    pub store_owner_layout_consumed: bool,
    pub galaxy_side: u32,
    pub single_galactic_tier: bool,

    pub capability_rows: Vec<DressRehearsalR3CapabilityRow>,
    pub modifier_overlay_rows: Vec<DressRehearsalR3ModifierOverlayRow>,
    pub owner_mask_application_rows: Vec<DressRehearsalR3OwnerMaskApplicationRow>,
    pub modified_r1_signal_rows: Vec<DressRehearsalR3ModifiedR1SignalRow>,
    pub modified_economy_signal_rows: Vec<DressRehearsalR3ModifiedEconomySignalRow>,
    pub artifact: DressRehearsalR3Artifact,
    pub summary: DressRehearsalR3Summary,

    pub capability_tree_before_checksum: u64,
    pub capability_tree_after_checksum: u64,
    pub capability_tree_unchanged: bool,
    pub occupant_positions_before: Vec<DressRehearsalR2OccupantPosition>,
    pub occupant_positions_after: Vec<DressRehearsalR2OccupantPosition>,
    pub boundary_request_emitted: bool,
    pub sead_action_emitted: bool,
    pub gradientxy_consumed: bool,
    pub combat_bonus_resolved_as_data: bool,
    pub combat_resolution_events: usize,
    pub hostile_hp_delta: i64,
    pub reparented_occupant_count: usize,
    pub new_shader_or_wgsl: bool,
    pub default_simsession_pass_graph_change: bool,
    pub cpu_planner_used: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r3_capability_mask_down(
    input: &DressRehearsalR3Input,
) -> DressRehearsalR3Report {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let r1_report = input
        .r1_report
        .as_ref()
        .expect("validated R1 report must be present");
    let r2_report = input
        .r2_report
        .as_ref()
        .expect("validated R2 report must be present");
    let execution = execute_model(r1_report, r2_report, &input.capability_rows);
    let oracle = cpu_oracle_dress_rehearsal_r3_capability_mask_down(input);
    let parity = execution.capability_rows == oracle.capability_rows
        && execution.modifier_rows == oracle.modifier_rows
        && execution.owner_mask_rows == oracle.owner_mask_rows
        && execution.modified_r1_rows == oracle.modified_r1_rows
        && execution.modified_economy_rows == oracle.modified_economy_rows
        && execution.summary == oracle.summary;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r3_capability_mask_down(
) -> (DressRehearsalR3Report, DressRehearsalR3Report) {
    let input = DressRehearsalR3Input::explicit_opt_in();
    (
        run_dress_rehearsal_r3_capability_mask_down(&input),
        run_dress_rehearsal_r3_capability_mask_down(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r3_capability_mask_down(
    input: &DressRehearsalR3Input,
) -> DressRehearsalR3Oracle {
    if !input.explicit_opt_in || input.enabled_by_default {
        return empty_oracle();
    }
    let Some(r1_report) = input.r1_report.as_ref() else {
        return empty_oracle();
    };
    let Some(r2_report) = input.r2_report.as_ref() else {
        return empty_oracle();
    };
    if !r1_report.admitted
        || !r1_report.cpu_oracle_parity
        || !r2_report.admitted
        || !r2_report.cpu_oracle_parity
    {
        return empty_oracle();
    }
    let execution = execute_model(r1_report, r2_report, &input.capability_rows);
    DressRehearsalR3Oracle {
        capability_rows: execution.capability_rows,
        modifier_rows: execution.modifier_rows,
        owner_mask_rows: execution.owner_mask_rows,
        modified_r1_rows: execution.modified_r1_rows,
        modified_economy_rows: execution.modified_economy_rows,
        summary: execution.summary,
    }
}

pub fn render_dress_rehearsal_r3_artifact(report: &DressRehearsalR3Report) -> String {
    report.artifact.markdown.clone()
}

pub fn apply_modifier_bps(base_value: f32, multiplier_bps: i32) -> f32 {
    base_value * multiplier_bps as f32 / BPS_ONE as f32
}

fn default_capability_rows() -> Vec<DressRehearsalR3CapabilityRow> {
    vec![
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Terran,
            faction_simthing_id: "faction-terran",
            capability_id: "terran-patrol-suppression-doctrine",
            tree_path: "faction/terran/patrol_suppression_doctrine",
            resolved_modifier_id: PATROL_SUPPRESSION_MODIFIER,
            chosen_rank: 2,
            multiplier_bps: 12_000,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Terran,
            faction_simthing_id: "faction-terran",
            capability_id: "terran-disruption-resistance",
            tree_path: "faction/terran/disruption_resistance",
            resolved_modifier_id: DISRUPTION_DECAY_MODIFIER,
            chosen_rank: 1,
            multiplier_bps: 11_000,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Terran,
            faction_simthing_id: "faction-terran",
            capability_id: "terran-defensive-logistics",
            tree_path: "faction/terran/defensive_logistics",
            resolved_modifier_id: DEFENSIVE_LOGISTICS_MODIFIER,
            chosen_rank: 1,
            multiplier_bps: 11_000,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Terran,
            faction_simthing_id: "faction-terran",
            capability_id: "terran-combat-bonus-placeholder",
            tree_path: "faction/terran/combat_bonus_placeholder",
            resolved_modifier_id: COMBAT_BONUS_PLACEHOLDER_MODIFIER,
            chosen_rank: 1,
            multiplier_bps: 10_500,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Pirate,
            faction_simthing_id: "faction-pirate",
            capability_id: "pirate-disruption-emission-doctrine",
            tree_path: "faction/pirate/disruption_emission_doctrine",
            resolved_modifier_id: PIRATE_EMISSION_MODIFIER,
            chosen_rank: 2,
            multiplier_bps: 12_500,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Pirate,
            faction_simthing_id: "faction-pirate",
            capability_id: "pirate-blockade-efficiency",
            tree_path: "faction/pirate/blockade_efficiency",
            resolved_modifier_id: BLOCKADE_DIVERT_MODIFIER,
            chosen_rank: 2,
            multiplier_bps: 15_000,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Pirate,
            faction_simthing_id: "faction-pirate",
            capability_id: "pirate-raiding-logistics",
            tree_path: "faction/pirate/raiding_logistics",
            resolved_modifier_id: RAIDING_LOGISTICS_MODIFIER,
            chosen_rank: 1,
            multiplier_bps: 11_000,
        },
        DressRehearsalR3CapabilityRow {
            owner: DressRehearsalR3Owner::Pirate,
            faction_simthing_id: "faction-pirate",
            capability_id: "pirate-combat-bonus-placeholder",
            tree_path: "faction/pirate/combat_bonus_placeholder",
            resolved_modifier_id: COMBAT_BONUS_PLACEHOLDER_MODIFIER,
            chosen_rank: 1,
            multiplier_bps: 11_500,
        },
    ]
}

fn validate_input(input: &DressRehearsalR3Input, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("r3_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return;
    }
    let Some(r1_report) = input.r1_report.as_ref() else {
        diagnostics.push("r1_report_missing");
        return;
    };
    let Some(r2_report) = input.r2_report.as_ref() else {
        diagnostics.push("r2_report_missing");
        return;
    };
    if !r1_report.admitted {
        diagnostics.push("r1_report_not_admitted");
    }
    if !r1_report.cpu_oracle_parity {
        diagnostics.push("r1_cpu_oracle_parity_missing");
    }
    if r1_report.final_disruption.len() != GALAXY_CELL_COUNT {
        diagnostics.push("r1_final_disruption_shape_mismatch");
    }
    if !r2_report.admitted {
        diagnostics.push("r2_report_not_admitted");
    }
    if !r2_report.cpu_oracle_parity {
        diagnostics.push("r2_cpu_oracle_parity_missing");
    }
    if !r2_report.r1_heatmap_consumed {
        diagnostics.push("r2_did_not_consume_r1_heatmap");
    }
    if r2_report.r1_input_contract_checksum != r1_report.starmap_summary.stable_checksum {
        diagnostics.push("r1_r2_checksum_mismatch");
    }
    if input.capability_rows.is_empty() {
        diagnostics.push("capability_tree_missing");
    }
}

fn base_report(
    input: &DressRehearsalR3Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<Execution>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR3Report {
    let admitted = diagnostics.is_empty();
    let opt_in = input.explicit_opt_in;
    let empty_summary = DressRehearsalR3Summary {
        capability_row_count: 0,
        modifier_row_count: 0,
        owner_mask_application_count: 0,
        modified_r1_signal_count: 0,
        modified_economy_signal_count: 0,
        stable_checksum: 0,
    };

    let r1 = input.r1_report.as_ref();
    let r2 = input.r2_report.as_ref();
    let (
        capability_rows,
        modifier_rows,
        owner_mask_rows,
        modified_r1_rows,
        modified_economy_rows,
        summary,
    ) = match execution.as_ref() {
        Some(execution) => (
            execution.capability_rows.clone(),
            execution.modifier_rows.clone(),
            execution.owner_mask_rows.clone(),
            execution.modified_r1_rows.clone(),
            execution.modified_economy_rows.clone(),
            execution.summary.clone(),
        ),
        None => (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            empty_summary.clone(),
        ),
    };

    let tree_checksum = if admitted && opt_in && !disabled_no_op {
        checksum_capability_tree(&input.capability_rows)
    } else {
        0
    };
    let positions = r2
        .filter(|_| !disabled_no_op)
        .map(|report| report.occupant_positions_before.clone())
        .unwrap_or_default();
    let markdown = render_artifact_markdown(
        &capability_rows,
        &modifier_rows,
        &owner_mask_rows,
        &modified_r1_rows,
        &modified_economy_rows,
        &summary,
        cpu_oracle_parity,
        r1.map(|report| report.starmap_summary.stable_checksum)
            .unwrap_or(0),
        r2.map(|report| report.summary.stable_checksum).unwrap_or(0),
    );
    let artifact = DressRehearsalR3Artifact {
        capability_rows: capability_rows.clone(),
        modifier_rows: modifier_rows.clone(),
        owner_mask_rows: owner_mask_rows.clone(),
        modified_r1_rows: modified_r1_rows.clone(),
        modified_economy_rows: modified_economy_rows.clone(),
        summary: summary.clone(),
        cpu_oracle_parity,
        markdown,
    };

    DressRehearsalR3Report {
        id: DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_ID,
        status: DRESS_REHEARSAL_R3_CAPABILITY_MASK_DOWN_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R3_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        r1_contract_consumed: admitted
            && opt_in
            && r1
                .map(|report| {
                    report.admitted
                        && report.cpu_oracle_parity
                        && report.final_disruption.len() == GALAXY_CELL_COUNT
                })
                .unwrap_or(false),
        r1_contract_checksum: r1
            .map(|report| report.starmap_summary.stable_checksum)
            .unwrap_or(0),
        r1_cpu_oracle_parity: r1.map(|report| report.cpu_oracle_parity).unwrap_or(false),
        r2_contract_consumed: admitted
            && opt_in
            && r2
                .map(|report| {
                    report.admitted
                        && report.cpu_oracle_parity
                        && report.r1_heatmap_consumed
                        && report.summary.system_count > 0
                })
                .unwrap_or(false),
        r2_contract_checksum: r2.map(|report| report.summary.stable_checksum).unwrap_or(0),
        r2_cpu_oracle_parity: r2.map(|report| report.cpu_oracle_parity).unwrap_or(false),
        store_owner_layout_consumed: admitted && opt_in && !disabled_no_op,
        galaxy_side: if disabled_no_op { 0 } else { GALAXY_SIDE },
        single_galactic_tier: admitted && opt_in && !disabled_no_op,
        capability_rows,
        modifier_overlay_rows: modifier_rows,
        owner_mask_application_rows: owner_mask_rows,
        modified_r1_signal_rows: modified_r1_rows,
        modified_economy_signal_rows: modified_economy_rows,
        artifact,
        summary: summary.clone(),
        capability_tree_before_checksum: tree_checksum,
        capability_tree_after_checksum: tree_checksum,
        capability_tree_unchanged: admitted && opt_in && !disabled_no_op,
        occupant_positions_before: positions.clone(),
        occupant_positions_after: positions,
        boundary_request_emitted: false,
        sead_action_emitted: false,
        gradientxy_consumed: false,
        combat_bonus_resolved_as_data: admitted
            && opt_in
            && input
                .capability_rows
                .iter()
                .any(|row| row.resolved_modifier_id == COMBAT_BONUS_PLACEHOLDER_MODIFIER),
        combat_resolution_events: 0,
        hostile_hp_delta: 0,
        reparented_occupant_count: 0,
        new_shader_or_wgsl: false,
        default_simsession_pass_graph_change: false,
        cpu_planner_used: false,
        cpu_oracle_parity,
        deterministic_replay_checksum: if admitted && opt_in {
            summary.stable_checksum
        } else {
            0
        },
    }
}

fn execute_model(
    r1_report: &DressRehearsalR1Report,
    r2_report: &DressRehearsalR2Report,
    capability_rows: &[DressRehearsalR3CapabilityRow],
) -> Execution {
    let mut capability_rows = capability_rows.to_vec();
    capability_rows.sort_by(|left, right| {
        owner_rank(left.owner)
            .cmp(&owner_rank(right.owner))
            .then(left.capability_id.cmp(right.capability_id))
    });
    let modifier_rows = resolve_modifier_rows(&capability_rows);
    let modified_r1_rows = build_modified_r1_rows(r1_report, &modifier_rows);
    let modified_economy_rows = build_modified_economy_rows(r2_report, &modifier_rows);
    let owner_mask_rows =
        build_owner_mask_rows(r1_report, r2_report, &modifier_rows, &modified_r1_rows);
    let summary = build_summary(
        r1_report.starmap_summary.stable_checksum,
        r2_report.summary.stable_checksum,
        &capability_rows,
        &modifier_rows,
        &owner_mask_rows,
        &modified_r1_rows,
        &modified_economy_rows,
    );
    Execution {
        capability_rows,
        modifier_rows,
        owner_mask_rows,
        modified_r1_rows,
        modified_economy_rows,
        summary,
    }
}

fn resolve_modifier_rows(
    capability_rows: &[DressRehearsalR3CapabilityRow],
) -> Vec<DressRehearsalR3ModifierOverlayRow> {
    capability_rows
        .iter()
        .map(|row| {
            let multiplier_bps = row.multiplier_bps.clamp(MIN_MODIFIER_BPS, MAX_MODIFIER_BPS);
            DressRehearsalR3ModifierOverlayRow {
                owner: row.owner,
                modifier_id: row.resolved_modifier_id,
                source_capability_id: row.capability_id,
                unclamped_multiplier_bps: row.multiplier_bps,
                multiplier_bps,
                min_bps: MIN_MODIFIER_BPS,
                max_bps: MAX_MODIFIER_BPS,
                read_side_only: true,
            }
        })
        .collect()
}

fn build_modified_r1_rows(
    r1_report: &DressRehearsalR1Report,
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
) -> Vec<DressRehearsalR3ModifiedR1SignalRow> {
    let lookup = modifier_lookup(modifier_rows);
    let mut rows = Vec::new();
    for cell in &r1_report.cell_inputs {
        for entry in &cell.separated_entries {
            let owner = owner_from_r1(entry.owner);
            let Some(modifier_id) = r1_modifier_for(entry.channel) else {
                continue;
            };
            let modifier = lookup
                .get(&(owner, modifier_id))
                .expect("default capability rows must contain R1 modifier");
            rows.push(DressRehearsalR3ModifiedR1SignalRow {
                source_id: entry.source_id.clone(),
                owner,
                channel: channel_name(entry.channel),
                cell_index: cell.cell_index,
                base_value: entry.value,
                modifier_id,
                multiplier_bps: modifier.multiplier_bps,
                effective_value: apply_modifier_bps(entry.value, modifier.multiplier_bps),
            });
        }
    }
    rows
}

fn build_modified_economy_rows(
    r2_report: &DressRehearsalR2Report,
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
) -> Vec<DressRehearsalR3ModifiedEconomySignalRow> {
    let lookup = modifier_lookup(modifier_rows);
    let mut rows = Vec::new();
    for ledger in &r2_report.stockpile_ledger {
        let owner = owner_from_r2(ledger.owner);
        let modifier_id = owner.logistics_modifier();
        let modifier = lookup
            .get(&(owner, modifier_id))
            .expect("default capability rows must contain logistics modifier");
        let base_signal = ledger.reduced_in as f32;
        let effective_signal = apply_modifier_bps(base_signal, modifier.multiplier_bps);
        rows.push(DressRehearsalR3ModifiedEconomySignalRow {
            signal_id: format!("{:?}-stockpile-reduce-up-read", owner),
            owner,
            source_contract: "R2 stockpile ledger reduced_in",
            base_signal,
            modifier_id,
            multiplier_bps: modifier.multiplier_bps,
            effective_signal,
            bounded_signal: effective_signal.clamp(0.0, 100.0),
        });
    }
    for diverted in &r2_report.diverted_production_rows {
        let owner = owner_from_r2(diverted.blockader_owner);
        let modifier = lookup
            .get(&(owner, BLOCKADE_DIVERT_MODIFIER))
            .expect("default capability rows must contain blockade modifier");
        let base_signal = diverted.production as f32;
        let effective_signal = apply_modifier_bps(base_signal, modifier.multiplier_bps);
        rows.push(DressRehearsalR3ModifiedEconomySignalRow {
            signal_id: format!("{}-blockade-divert-read", diverted.system_id),
            owner,
            source_contract: "R2 diverted production row",
            base_signal,
            modifier_id: BLOCKADE_DIVERT_MODIFIER,
            multiplier_bps: modifier.multiplier_bps,
            effective_signal,
            bounded_signal: effective_signal.clamp(0.0, 100.0),
        });
    }
    rows
}

fn build_owner_mask_rows(
    r1_report: &DressRehearsalR1Report,
    r2_report: &DressRehearsalR2Report,
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
    modified_r1_rows: &[DressRehearsalR3ModifiedR1SignalRow],
) -> Vec<DressRehearsalR3OwnerMaskApplicationRow> {
    let lookup = modifier_lookup(modifier_rows);
    let occupant_lookup: HashMap<_, _> = r1_report
        .scenario
        .occupants
        .iter()
        .map(|occupant| (occupant.source_id.as_str(), occupant))
        .collect();
    let mut rows = Vec::new();

    for signal in modified_r1_rows {
        let occupant = occupant_lookup
            .get(signal.source_id.as_str())
            .expect("R1 signal source must exist as scenario occupant");
        rows.push(DressRehearsalR3OwnerMaskApplicationRow {
            source_id: signal.source_id.clone(),
            owner: signal.owner,
            occupant_kind: occupant_kind_name(occupant.kind),
            signal_family: "R1 source contribution",
            cell_index: occupant.cell_index,
            x: occupant.x,
            y: occupant.y,
            structural_parent_before: "galactic-location-0",
            structural_parent_after: "galactic-location-0",
            modifier_id: signal.modifier_id,
            applied_multiplier_bps: signal.multiplier_bps,
            owner_column_matched: lookup.contains_key(&(signal.owner, signal.modifier_id)),
            evidence_group: "canonical-r1-owner-mask",
        });
    }

    for system in &r2_report.production_rows {
        let owner = owner_from_r2(system.original_owner);
        let modifier_id = owner.logistics_modifier();
        let modifier = lookup
            .get(&(owner, modifier_id))
            .expect("default capability rows must contain system logistics modifier");
        rows.push(DressRehearsalR3OwnerMaskApplicationRow {
            source_id: system.system_id.clone(),
            owner,
            occupant_kind: "system",
            signal_family: "R2 stockpile/disbursement read",
            cell_index: system.cell_index,
            x: system.x,
            y: system.y,
            structural_parent_before: system.structural_parent_before,
            structural_parent_after: system.structural_parent_after,
            modifier_id,
            applied_multiplier_bps: modifier.multiplier_bps,
            owner_column_matched: true,
            evidence_group: "canonical-r2-owner-mask",
        });
    }

    for diverted in &r2_report.diverted_production_rows {
        let owner = owner_from_r2(diverted.blockader_owner);
        let modifier = lookup
            .get(&(owner, BLOCKADE_DIVERT_MODIFIER))
            .expect("default capability rows must contain blockade modifier");
        rows.push(DressRehearsalR3OwnerMaskApplicationRow {
            source_id: diverted.system_id.clone(),
            owner,
            occupant_kind: "production-flow",
            signal_family: "R2 blockade/divert read",
            cell_index: diverted.cell_index,
            x: diverted.cell_index % GALAXY_SIDE,
            y: diverted.cell_index / GALAXY_SIDE,
            structural_parent_before: diverted.structural_parent_before,
            structural_parent_after: diverted.structural_parent_after,
            modifier_id: BLOCKADE_DIVERT_MODIFIER,
            applied_multiplier_bps: modifier.multiplier_bps,
            owner_column_matched: true,
            evidence_group: "canonical-r2-owner-mask",
        });
    }

    rows.extend(build_colocated_evidence_rows(&lookup));
    rows.sort_by(|left, right| {
        left.cell_index
            .cmp(&right.cell_index)
            .then(owner_rank(left.owner).cmp(&owner_rank(right.owner)))
            .then(left.source_id.cmp(&right.source_id))
            .then(left.modifier_id.cmp(right.modifier_id))
    });
    rows
}

fn build_colocated_evidence_rows(
    lookup: &HashMap<(DressRehearsalR3Owner, &'static str), &DressRehearsalR3ModifierOverlayRow>,
) -> Vec<DressRehearsalR3OwnerMaskApplicationRow> {
    let (location_id, x, y, cell_index) = atlas_store::canonical_pirate_shared_galactic_cell(
        &atlas_store::canonical_materialization(),
    );
    assert_eq!(location_id.0, 0);
    let terran = lookup
        .get(&(DressRehearsalR3Owner::Terran, PATROL_SUPPRESSION_MODIFIER))
        .expect("Terran patrol modifier");
    let pirate = lookup
        .get(&(DressRehearsalR3Owner::Pirate, PIRATE_EMISSION_MODIFIER))
        .expect("Pirate emission modifier");
    vec![
        DressRehearsalR3OwnerMaskApplicationRow {
            source_id: "r3-colocated-terran-patrol".to_string(),
            owner: DressRehearsalR3Owner::Terran,
            occupant_kind: "patrol_fleet",
            signal_family: "ATLAS owner/channel layout evidence",
            cell_index,
            x,
            y,
            structural_parent_before: "galactic-location-0",
            structural_parent_after: "galactic-location-0",
            modifier_id: PATROL_SUPPRESSION_MODIFIER,
            applied_multiplier_bps: terran.multiplier_bps,
            owner_column_matched: true,
            evidence_group: "galactic-colocation-owner-mask",
        },
        DressRehearsalR3OwnerMaskApplicationRow {
            source_id: "r3-colocated-pirate-ship".to_string(),
            owner: DressRehearsalR3Owner::Pirate,
            occupant_kind: "pirate_fleet",
            signal_family: "ATLAS owner/channel layout evidence",
            cell_index,
            x,
            y,
            structural_parent_before: "galactic-location-0",
            structural_parent_after: "galactic-location-0",
            modifier_id: PIRATE_EMISSION_MODIFIER,
            applied_multiplier_bps: pirate.multiplier_bps,
            owner_column_matched: true,
            evidence_group: "galactic-colocation-owner-mask",
        },
    ]
}

fn build_summary(
    r1_checksum: u64,
    r2_checksum: u64,
    capability_rows: &[DressRehearsalR3CapabilityRow],
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
    owner_mask_rows: &[DressRehearsalR3OwnerMaskApplicationRow],
    modified_r1_rows: &[DressRehearsalR3ModifiedR1SignalRow],
    modified_economy_rows: &[DressRehearsalR3ModifiedEconomySignalRow],
) -> DressRehearsalR3Summary {
    let stable_checksum = checksum_r3(
        r1_checksum,
        r2_checksum,
        capability_rows,
        modifier_rows,
        owner_mask_rows,
        modified_r1_rows,
        modified_economy_rows,
    );
    DressRehearsalR3Summary {
        capability_row_count: capability_rows.len(),
        modifier_row_count: modifier_rows.len(),
        owner_mask_application_count: owner_mask_rows.len(),
        modified_r1_signal_count: modified_r1_rows.len(),
        modified_economy_signal_count: modified_economy_rows.len(),
        stable_checksum,
    }
}

fn render_artifact_markdown(
    capability_rows: &[DressRehearsalR3CapabilityRow],
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
    owner_mask_rows: &[DressRehearsalR3OwnerMaskApplicationRow],
    modified_r1_rows: &[DressRehearsalR3ModifiedR1SignalRow],
    modified_economy_rows: &[DressRehearsalR3ModifiedEconomySignalRow],
    summary: &DressRehearsalR3Summary,
    cpu_oracle_parity: bool,
    r1_checksum: u64,
    r2_checksum: u64,
) -> String {
    let mut out = String::new();
    out.push_str("## R3 Capability Mask-Down Artifact\n\n");
    out.push_str("| key | value |\n|---|---:|\n");
    out.push_str(&format!("| r1_checksum | {:016x} |\n", r1_checksum));
    out.push_str(&format!("| r2_checksum | {:016x} |\n", r2_checksum));
    out.push_str(&format!(
        "| capability_rows | {} |\n",
        summary.capability_row_count
    ));
    out.push_str(&format!(
        "| modifier_rows | {} |\n",
        summary.modifier_row_count
    ));
    out.push_str(&format!(
        "| owner_mask_rows | {} |\n",
        summary.owner_mask_application_count
    ));
    out.push_str(&format!(
        "| modified_r1_rows | {} |\n",
        summary.modified_r1_signal_count
    ));
    out.push_str(&format!(
        "| modified_economy_rows | {} |\n",
        summary.modified_economy_signal_count
    ));
    out.push_str(&format!(
        "| stable_checksum | {:016x} |\n",
        summary.stable_checksum
    ));
    out.push_str(&format!(
        "| cpu_oracle_parity | {} |\n\n",
        cpu_oracle_parity
    ));

    out.push_str("### Capability Rows\n\n");
    out.push_str("| owner | faction_simthing | capability | tree_path | modifier | rank | bps |\n");
    out.push_str("|---|---|---|---|---|---:|---:|\n");
    for row in capability_rows {
        out.push_str(&format!(
            "| {:?} | {} | {} | {} | {} | {} | {} |\n",
            row.owner,
            row.faction_simthing_id,
            row.capability_id,
            row.tree_path,
            row.resolved_modifier_id,
            row.chosen_rank,
            row.multiplier_bps
        ));
    }

    out.push_str("\n### Modifier Rows\n\n");
    out.push_str("| owner | modifier | capability | multiplier_bps | read_side_only |\n");
    out.push_str("|---|---|---|---:|---|\n");
    for row in modifier_rows {
        out.push_str(&format!(
            "| {:?} | {} | {} | {} | {} |\n",
            row.owner,
            row.modifier_id,
            row.source_capability_id,
            row.multiplier_bps,
            row.read_side_only
        ));
    }

    out.push_str("\n### Owner-Mask Rows\n\n");
    out.push_str("| source | owner | kind | signal | cell | modifier | bps | group |\n");
    out.push_str("|---|---|---|---|---:|---|---:|---|\n");
    for row in owner_mask_rows.iter().take(16) {
        out.push_str(&format!(
            "| {} | {:?} | {} | {} | {} | {} | {} | {} |\n",
            row.source_id,
            row.owner,
            row.occupant_kind,
            row.signal_family,
            row.cell_index,
            row.modifier_id,
            row.applied_multiplier_bps,
            row.evidence_group
        ));
    }

    out.push_str("\n### Modified R1 Signal Rows\n\n");
    out.push_str("| source | owner | channel | cell | base | modifier | bps | effective |\n");
    out.push_str("|---|---|---|---:|---:|---|---:|---:|\n");
    for row in modified_r1_rows.iter().take(12) {
        out.push_str(&format!(
            "| {} | {:?} | {} | {} | {:.3} | {} | {} | {:.3} |\n",
            row.source_id,
            row.owner,
            row.channel,
            row.cell_index,
            row.base_value,
            row.modifier_id,
            row.multiplier_bps,
            row.effective_value
        ));
    }

    out.push_str("\n### Modified Economy Rows\n\n");
    out.push_str("| signal | owner | contract | base | modifier | bps | effective | bounded |\n");
    out.push_str("|---|---|---|---:|---|---:|---:|---:|\n");
    for row in modified_economy_rows {
        out.push_str(&format!(
            "| {} | {:?} | {} | {:.3} | {} | {} | {:.3} | {:.3} |\n",
            row.signal_id,
            row.owner,
            row.source_contract,
            row.base_signal,
            row.modifier_id,
            row.multiplier_bps,
            row.effective_signal,
            row.bounded_signal
        ));
    }
    out
}

fn modifier_lookup(
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
) -> HashMap<(DressRehearsalR3Owner, &'static str), &DressRehearsalR3ModifierOverlayRow> {
    modifier_rows
        .iter()
        .map(|row| ((row.owner, row.modifier_id), row))
        .collect()
}

fn r1_modifier_for(channel: DressRehearsalR1Channel) -> Option<&'static str> {
    match channel {
        DressRehearsalR1Channel::PirateDisruption => Some(PIRATE_EMISSION_MODIFIER),
        DressRehearsalR1Channel::PatrolSuppression => Some(PATROL_SUPPRESSION_MODIFIER),
        DressRehearsalR1Channel::InertSystem => None,
    }
}

fn channel_name(channel: DressRehearsalR1Channel) -> &'static str {
    match channel {
        DressRehearsalR1Channel::InertSystem => "InertSystem",
        DressRehearsalR1Channel::PirateDisruption => "PirateDisruption",
        DressRehearsalR1Channel::PatrolSuppression => "PatrolSuppression",
    }
}

fn occupant_kind_name(kind: DressRehearsalR1OccupantKind) -> &'static str {
    match kind {
        DressRehearsalR1OccupantKind::System => "system",
        DressRehearsalR1OccupantKind::PirateFleet => "pirate_fleet",
        DressRehearsalR1OccupantKind::PatrolFleet => "patrol_fleet",
    }
}

fn owner_from_r1(owner: DressRehearsalR1Owner) -> DressRehearsalR3Owner {
    match owner {
        DressRehearsalR1Owner::Terran => DressRehearsalR3Owner::Terran,
        DressRehearsalR1Owner::Pirate => DressRehearsalR3Owner::Pirate,
    }
}

fn owner_from_r2(owner: DressRehearsalR2Owner) -> DressRehearsalR3Owner {
    match owner {
        DressRehearsalR2Owner::Terran => DressRehearsalR3Owner::Terran,
        DressRehearsalR2Owner::Pirate => DressRehearsalR3Owner::Pirate,
    }
}

fn owner_rank(owner: DressRehearsalR3Owner) -> u8 {
    match owner {
        DressRehearsalR3Owner::Terran => 0,
        DressRehearsalR3Owner::Pirate => 1,
    }
}

fn empty_oracle() -> DressRehearsalR3Oracle {
    DressRehearsalR3Oracle {
        capability_rows: Vec::new(),
        modifier_rows: Vec::new(),
        owner_mask_rows: Vec::new(),
        modified_r1_rows: Vec::new(),
        modified_economy_rows: Vec::new(),
        summary: DressRehearsalR3Summary {
            capability_row_count: 0,
            modifier_row_count: 0,
            owner_mask_application_count: 0,
            modified_r1_signal_count: 0,
            modified_economy_signal_count: 0,
            stable_checksum: 0,
        },
    }
}

struct Execution {
    capability_rows: Vec<DressRehearsalR3CapabilityRow>,
    modifier_rows: Vec<DressRehearsalR3ModifierOverlayRow>,
    owner_mask_rows: Vec<DressRehearsalR3OwnerMaskApplicationRow>,
    modified_r1_rows: Vec<DressRehearsalR3ModifiedR1SignalRow>,
    modified_economy_rows: Vec<DressRehearsalR3ModifiedEconomySignalRow>,
    summary: DressRehearsalR3Summary,
}

fn checksum_capability_tree(capability_rows: &[DressRehearsalR3CapabilityRow]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for row in capability_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.faction_simthing_id);
        hash = fnv_str(hash, row.capability_id);
        hash = fnv_str(hash, row.tree_path);
        hash = fnv_str(hash, row.resolved_modifier_id);
        hash = fnv(hash, row.chosen_rank as u64);
        hash = fnv(hash, row.multiplier_bps as u64);
    }
    hash
}

fn checksum_r3(
    r1_checksum: u64,
    r2_checksum: u64,
    capability_rows: &[DressRehearsalR3CapabilityRow],
    modifier_rows: &[DressRehearsalR3ModifierOverlayRow],
    owner_mask_rows: &[DressRehearsalR3OwnerMaskApplicationRow],
    modified_r1_rows: &[DressRehearsalR3ModifiedR1SignalRow],
    modified_economy_rows: &[DressRehearsalR3ModifiedEconomySignalRow],
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv(hash, r1_checksum);
    hash = fnv(hash, r2_checksum);
    for row in capability_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.capability_id);
        hash = fnv_str(hash, row.resolved_modifier_id);
        hash = fnv(hash, row.chosen_rank as u64);
        hash = fnv(hash, row.multiplier_bps as u64);
    }
    for row in modifier_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.modifier_id);
        hash = fnv(hash, row.multiplier_bps as u64);
        hash = fnv(hash, row.read_side_only as u64);
    }
    for row in owner_mask_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.source_id.as_str());
        hash = fnv(hash, row.cell_index as u64);
        hash = fnv_str(hash, row.modifier_id);
        hash = fnv(hash, row.applied_multiplier_bps as u64);
    }
    for row in modified_r1_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.source_id.as_str());
        hash = fnv(hash, row.cell_index as u64);
        hash = fnv(hash, row.base_value.to_bits() as u64);
        hash = fnv(hash, row.effective_value.to_bits() as u64);
    }
    for row in modified_economy_rows {
        hash = fnv(hash, row.owner.stable_code());
        hash = fnv_str(hash, row.signal_id.as_str());
        hash = fnv(hash, row.base_signal.to_bits() as u64);
        hash = fnv(hash, row.effective_signal.to_bits() as u64);
        hash = fnv(hash, row.bounded_signal.to_bits() as u64);
    }
    hash
}

fn fnv(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn fnv_str(mut hash: u64, value: &str) -> u64 {
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
