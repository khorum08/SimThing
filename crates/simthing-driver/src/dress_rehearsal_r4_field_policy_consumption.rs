//! SCENARIO-0080-2-R4: FIELD_POLICY field-consumption + exact sqrt (EC2).
//!
//! Fixture-only, opt-in/default-off proof that child movers read the parent galactic grid
//! field at their own cells, apply R3 masked-down disposition weights, compute GradientXY,
//! derive exact pre-sqrt mag2 and Candidate-F exact Euclidean magnitude, and threshold-gate
//! a sit-still vs step-opportunity FIELD_POLICY decision. CPU oracle parity is the authority; no
//! relocation, REENROLL, BoundaryRequest, or default SimSession wiring.

#[allow(dead_code, unused_imports)]
#[path = "dress_rehearsal_atlas_batch_0_store.rs"]
mod atlas_store;

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1CellInput, DressRehearsalR1Input,
    DressRehearsalR1OccupantKind, DressRehearsalR1Report, GALAXY_CELL_COUNT, GALAXY_SIDE,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2Input, DressRehearsalR2Report,
};
use crate::dress_rehearsal_r3_capability_mask_down::{
    apply_modifier_bps, run_dress_rehearsal_r3_capability_mask_down, DressRehearsalR3Input,
    DressRehearsalR3ModifierOverlayRow, DressRehearsalR3Owner, DressRehearsalR3Report,
    BLOCKADE_DIVERT_MODIFIER, DEFENSIVE_LOGISTICS_MODIFIER, DISRUPTION_DECAY_MODIFIER,
    PATROL_SUPPRESSION_MODIFIER, PIRATE_EMISSION_MODIFIER, RAIDING_LOGISTICS_MODIFIER,
};
use simthing_gpu::{
    cpu_horizon, params_from_config, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOperator,
    StructuredFieldStencilSourcePolicy,
};
use simthing_spec::{
    MAG2_Q16_SCALE, SQRT_F_ARTIFACT_HASH, SQRT_F_ARTIFACT_PATH, SQRT_F_ENTRYPOINT,
};
use std::collections::HashMap;

pub const DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_ID: &str =
    "SCENARIO-0080-2-R4-FIELD_POLICY-FIELD-CONSUMPTION";
pub const DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - FIELD_POLICY field-consumption + exact sqrt EC2";
pub const DRESS_REHEARSAL_R4_SCENARIO: &str = "SCENARIO-0080-2";

/// Exact magnitude threshold (f32 bits compared against Candidate-F output).
pub const MOVEMENT_THRESHOLD_MAG_BITS: u32 = 0x3c23_d70a; // 0.01f32

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DressRehearsalR4Owner {
    Terran,
    Pirate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DressRehearsalR4Decision {
    SitStill,
    StepOpportunity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4CompositeComponentRow {
    pub cell_index: u32,
    pub disruption: f32,
    pub location_status: f32,
    pub patrol_count: u32,
    pub pirate_count: u32,
    pub economy_signal: f32,
    pub disposition_weight_bps: i32,
    pub composite_opportunity: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4MoverDecisionRow {
    pub mover_id: String,
    pub owner: DressRehearsalR4Owner,
    pub occupant_kind: &'static str,
    pub cell_index: u32,
    pub x: u32,
    pub y: u32,
    pub disruption_at_cell: f32,
    pub location_status_at_cell: f32,
    pub patrol_count_at_cell: u32,
    pub pirate_count_at_cell: u32,
    pub economy_signal_at_cell: f32,
    pub disposition_weights: Vec<(&'static str, i32)>,
    pub composite_components: Vec<DressRehearsalR4CompositeComponentRow>,
    pub gradient_dx_f32: f32,
    pub gradient_dy_f32: f32,
    pub gradient_dx_fixed: i32,
    pub gradient_dy_fixed: i32,
    pub exact_mag2_u64: u64,
    pub exact_mag2_bits: u32,
    pub candidate_f_exact_mag_bits: u32,
    pub approximate_diagnostic_mag_bits: u32,
    pub movement_threshold_mag_bits: u32,
    pub threshold_passed: bool,
    pub decision: DressRehearsalR4Decision,
    pub candidate_target_x: Option<u32>,
    pub candidate_target_y: Option<u32>,
    pub candidate_target_cell_index: Option<u32>,
    pub candidate_direction: Option<&'static str>,
    pub movement_applied: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4ExactMagnitudeRow {
    pub mover_id: String,
    pub dx_fixed: i32,
    pub dy_fixed: i32,
    pub exact_mag2_u64: u64,
    pub exact_mag2_bits: u32,
    pub candidate_f_exact_mag_bits: u32,
    pub approximate_diagnostic_mag_bits: u32,
    pub commitment_uses_candidate_f: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalR4Summary {
    pub mover_count: usize,
    pub sit_still_count: usize,
    pub step_opportunity_count: usize,
    pub gradientxy_consumed: bool,
    pub stable_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4Artifact {
    pub mover_rows: Vec<DressRehearsalR4MoverDecisionRow>,
    pub composite_field_rows: Vec<DressRehearsalR4CompositeComponentRow>,
    pub exact_magnitude_rows: Vec<DressRehearsalR4ExactMagnitudeRow>,
    pub summary: DressRehearsalR4Summary,
    pub cpu_oracle_parity: bool,
    pub markdown: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4Oracle {
    pub mover_rows: Vec<DressRehearsalR4MoverDecisionRow>,
    pub composite_field_rows: Vec<DressRehearsalR4CompositeComponentRow>,
    pub exact_magnitude_rows: Vec<DressRehearsalR4ExactMagnitudeRow>,
    pub summary: DressRehearsalR4Summary,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub movement_threshold_mag_bits: u32,
    pub r1_report: Option<DressRehearsalR1Report>,
    pub r2_report: Option<DressRehearsalR2Report>,
    pub r3_report: Option<DressRehearsalR3Report>,
}

impl DressRehearsalR4Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            movement_threshold_mag_bits: MOVEMENT_THRESHOLD_MAG_BITS,
            r1_report: None,
            r2_report: None,
            r3_report: None,
        }
    }

    pub fn explicit_opt_in() -> Self {
        let r1_report =
            run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in());
        let r2_report = run_dress_rehearsal_r2_recursive_allocation(
            &DressRehearsalR2Input::with_r1_report(r1_report.clone()),
        );
        let r3_report = run_dress_rehearsal_r3_capability_mask_down(
            &DressRehearsalR3Input::with_reports(r1_report.clone(), r2_report.clone()),
        );
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            movement_threshold_mag_bits: MOVEMENT_THRESHOLD_MAG_BITS,
            r1_report: Some(r1_report),
            r2_report: Some(r2_report),
            r3_report: Some(r3_report),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DressRehearsalR4Report {
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
    pub r3_contract_consumed: bool,
    pub r3_contract_checksum: u64,
    pub r3_cpu_oracle_parity: bool,
    pub store_owner_layout_consumed: bool,

    pub galaxy_side: u32,
    pub single_galactic_tier: bool,
    pub gradientxy_consumed: bool,
    pub exact_sqrt_artifact_hash: &'static str,
    pub exact_sqrt_artifact_path: &'static str,
    pub exact_sqrt_entrypoint: &'static str,

    pub mover_rows: Vec<DressRehearsalR4MoverDecisionRow>,
    pub composite_field_rows: Vec<DressRehearsalR4CompositeComponentRow>,
    pub exact_magnitude_rows: Vec<DressRehearsalR4ExactMagnitudeRow>,
    pub artifact: DressRehearsalR4Artifact,
    pub summary: DressRehearsalR4Summary,

    pub occupant_positions_before: Vec<(String, u32, u32, u32)>,
    pub occupant_positions_after: Vec<(String, u32, u32, u32)>,
    pub boundary_request_emitted: bool,
    pub movement_applied: bool,
    pub reenroll_emitted: bool,
    pub reparented_occupant_count: usize,
    pub combat_resolution_events: usize,
    pub new_shader_or_wgsl: bool,
    pub default_simsession_pass_graph_change: bool,
    pub cpu_planner_used: bool,
    pub gpu_diagnostic_run: bool,

    pub cpu_oracle_parity: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_dress_rehearsal_r4_field_policy_consumption(
    input: &DressRehearsalR4Input,
) -> DressRehearsalR4Report {
    let mut diagnostics = Vec::new();
    validate_input(input, &mut diagnostics);

    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, false);
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, false);
    }

    let r1_report = input.r1_report.as_ref().expect("validated R1");
    let r2_report = input.r2_report.as_ref().expect("validated R2");
    let r3_report = input.r3_report.as_ref().expect("validated R3");
    let execution = execute_model(
        r1_report,
        r2_report,
        r3_report,
        input.movement_threshold_mag_bits,
    );
    let oracle = cpu_oracle_dress_rehearsal_r4_field_policy_consumption(input);
    let parity = execution.mover_rows == oracle.mover_rows
        && execution.composite_field_rows == oracle.composite_field_rows
        && execution.exact_magnitude_rows == oracle.exact_magnitude_rows
        && execution.summary == oracle.summary;
    base_report(input, false, Vec::new(), Some(execution), parity)
}

pub fn replay_dress_rehearsal_r4_field_policy_consumption(
) -> (DressRehearsalR4Report, DressRehearsalR4Report) {
    let input = DressRehearsalR4Input::explicit_opt_in();
    (
        run_dress_rehearsal_r4_field_policy_consumption(&input),
        run_dress_rehearsal_r4_field_policy_consumption(&input),
    )
}

pub fn cpu_oracle_dress_rehearsal_r4_field_policy_consumption(
    input: &DressRehearsalR4Input,
) -> DressRehearsalR4Oracle {
    if !input.explicit_opt_in || input.enabled_by_default {
        return empty_oracle();
    }
    let Some(r1_report) = input.r1_report.as_ref() else {
        return empty_oracle();
    };
    let Some(r2_report) = input.r2_report.as_ref() else {
        return empty_oracle();
    };
    let Some(r3_report) = input.r3_report.as_ref() else {
        return empty_oracle();
    };
    if !r1_report.admitted
        || !r1_report.cpu_oracle_parity
        || !r2_report.admitted
        || !r2_report.cpu_oracle_parity
        || !r3_report.admitted
        || !r3_report.cpu_oracle_parity
    {
        return empty_oracle();
    }
    let execution = execute_model(
        r1_report,
        r2_report,
        r3_report,
        input.movement_threshold_mag_bits,
    );
    DressRehearsalR4Oracle {
        mover_rows: execution.mover_rows,
        composite_field_rows: execution.composite_field_rows,
        exact_magnitude_rows: execution.exact_magnitude_rows,
        summary: execution.summary,
    }
}

pub fn render_dress_rehearsal_r4_artifact(report: &DressRehearsalR4Report) -> String {
    report.artifact.markdown.clone()
}

pub fn sqrt_cr_f_bits(x_bits: u32) -> u32 {
    const F_QNAN: u32 = 0x7FC0_0000;
    const F_PINF: u32 = 0x7F80_0000;

    fn sqrt_cr_f_core(m: f32) -> f32 {
        let y0 = m.sqrt();
        let y_hi = f32::from_bits(f32::to_bits(y0) & 0xFFFF_F000);
        let y_lo = y0 - y_hi;
        let p = y0 * y0;
        let yhi_yhi = y_hi * y_hi;
        let yhi_ylo = y_hi * y_lo;
        let two_yhi_ylo = yhi_ylo + yhi_ylo;
        let ylo_ylo = y_lo * y_lo;
        let e0 = yhi_yhi - p;
        let e1 = e0 + two_yhi_ylo;
        let e = e1 + ylo_ylo;
        let sp = m - p;
        let r = sp - e;
        let y_up = f32::from_bits(f32::to_bits(y0) + 1);
        let y_dn = f32::from_bits(f32::to_bits(y0) - 1);
        let u_up = y_up - y0;
        let u_dn = y0 - y_dn;
        let t_up = y0 * u_up + 0.25 * u_up * u_up;
        let t_dn = y0 * u_dn - 0.25 * u_dn * u_dn;
        if r > t_up {
            return y_up;
        }
        if r < -t_dn {
            return y_dn;
        }
        y0
    }

    let sign = x_bits >> 31;
    let exp = (x_bits >> 23) & 0xFF;
    let mant = x_bits & 0x007F_FFFF;

    if exp == 0xFF {
        if mant != 0 {
            return F_QNAN;
        }
        if sign == 0 {
            return F_PINF;
        }
        return F_QNAN;
    }
    if x_bits == 0x0000_0000 {
        return 0x0000_0000;
    }
    if x_bits == 0x8000_0000 {
        return 0x8000_0000;
    }
    if sign == 1 {
        return F_QNAN;
    }

    let (m2_bits, e2) = if exp == 0 {
        let lz = mant.leading_zeros();
        let sh = lz.saturating_sub(8);
        let frac = (mant << sh) & 0x007F_FFFF;
        let m2_bits = 0x3F80_0000 | frac;
        let e2 = -118 - (lz as i32);
        (m2_bits, e2)
    } else {
        (0x3F80_0000 | mant, exp as i32 - 127)
    };

    let k = e2 >> 1;
    let parity = (e2 as u32) & 1;
    let m = f32::from_bits(m2_bits) * (1u32 << parity) as f32;
    let root = sqrt_cr_f_core(m);
    let root_bits = f32::to_bits(root);
    let final_exp = ((root_bits >> 23) & 0xFF) as i32 + k;
    ((final_exp as u32) << 23) | (root_bits & 0x007F_FFFF)
}

pub fn f32_to_q16(v: f32) -> i32 {
    (v * MAG2_Q16_SCALE as f32).round() as i32
}

pub fn cpu_mag2_sum(dx_fixed: i32, dy_fixed: i32) -> u64 {
    let dx = i64::from(dx_fixed);
    let dy = i64::from(dy_fixed);
    (dx * dx + dy * dy) as u64
}

pub fn mag2_u64_q16_to_f32_bits(sum: u64) -> u32 {
    let lo = sum as u32;
    let hi = (sum >> 32) as u32;
    f32::to_bits((hi as f32) + (lo as f32 / 4_294_967_296.0))
}

pub fn exact_mag2_bits_from_fixed(dx_fixed: i32, dy_fixed: i32) -> u32 {
    mag2_u64_q16_to_f32_bits(cpu_mag2_sum(dx_fixed, dy_fixed))
}

fn validate_input(input: &DressRehearsalR4Input, diagnostics: &mut Vec<&'static str>) {
    if input.enabled_by_default {
        diagnostics.push("r4_default_on_rejected");
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
    let Some(r3_report) = input.r3_report.as_ref() else {
        diagnostics.push("r3_report_missing");
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
    if !r3_report.admitted {
        diagnostics.push("r3_report_not_admitted");
    }
    if !r3_report.cpu_oracle_parity {
        diagnostics.push("r3_cpu_oracle_parity_missing");
    }
    if r3_report.r1_contract_checksum != r1_report.starmap_summary.stable_checksum {
        diagnostics.push("r1_r3_checksum_mismatch");
    }
    if r3_report.r2_contract_checksum != r2_report.summary.stable_checksum {
        diagnostics.push("r2_r3_checksum_mismatch");
    }
}

struct Execution {
    mover_rows: Vec<DressRehearsalR4MoverDecisionRow>,
    composite_field_rows: Vec<DressRehearsalR4CompositeComponentRow>,
    exact_magnitude_rows: Vec<DressRehearsalR4ExactMagnitudeRow>,
    summary: DressRehearsalR4Summary,
}

fn empty_oracle() -> DressRehearsalR4Oracle {
    DressRehearsalR4Oracle {
        mover_rows: Vec::new(),
        composite_field_rows: Vec::new(),
        exact_magnitude_rows: Vec::new(),
        summary: DressRehearsalR4Summary {
            mover_count: 0,
            sit_still_count: 0,
            step_opportunity_count: 0,
            gradientxy_consumed: false,
            stable_checksum: 0,
        },
    }
}

fn execute_model(
    r1_report: &DressRehearsalR1Report,
    r2_report: &DressRehearsalR2Report,
    r3_report: &DressRehearsalR3Report,
    movement_threshold_mag_bits: u32,
) -> Execution {
    let cell_inputs = cell_input_map(&r1_report.cell_inputs);
    let economy_by_cell = economy_signal_by_cell(r2_report);
    let modifier_lookup = build_modifier_lookup(&r3_report.modifier_overlay_rows);

    let pirate_field = build_composite_field(
        DressRehearsalR4Owner::Pirate,
        r1_report,
        &cell_inputs,
        &economy_by_cell,
        &modifier_lookup,
    );
    let patrol_field = build_composite_field(
        DressRehearsalR4Owner::Terran,
        r1_report,
        &cell_inputs,
        &economy_by_cell,
        &modifier_lookup,
    );

    let movers = canonical_movers_with_gradient(r1_report, &pirate_field, &patrol_field);
    let mut mover_rows = Vec::new();
    let mut exact_magnitude_rows = Vec::new();

    for (mover_id, owner, kind, x, y, cell_index) in movers {
        let field = match owner {
            DressRehearsalR4Owner::Pirate => &pirate_field,
            DressRehearsalR4Owner::Terran => &patrol_field,
        };
        let (gradient_dx_f32, gradient_dy_f32) = gradient_xy_at_cell(field, x, y, GALAXY_SIDE);
        let dx_fixed = f32_to_q16(gradient_dx_f32);
        let dy_fixed = f32_to_q16(gradient_dy_f32);
        let exact_mag2_u64 = cpu_mag2_sum(dx_fixed, dy_fixed);
        let exact_mag2_bits = exact_mag2_bits_from_fixed(dx_fixed, dy_fixed);
        let candidate_f_exact_mag_bits = sqrt_cr_f_bits(exact_mag2_bits);
        let approximate_diagnostic_mag_bits = {
            let raw = gradient_dx_f32 * gradient_dx_f32 + gradient_dy_f32 * gradient_dy_f32;
            raw.sqrt().to_bits()
        };
        let threshold_passed = if movement_threshold_mag_bits == 0 {
            candidate_f_exact_mag_bits > 0
        } else {
            candidate_f_exact_mag_bits >= movement_threshold_mag_bits
        };
        let decision = if threshold_passed {
            DressRehearsalR4Decision::StepOpportunity
        } else {
            DressRehearsalR4Decision::SitStill
        };
        let (candidate_target_x, candidate_target_y, candidate_target_cell_index, direction) =
            if threshold_passed {
                greedy_target(
                    cell_index,
                    x,
                    y,
                    gradient_dx_f32,
                    gradient_dy_f32,
                    field,
                    GALAXY_SIDE,
                )
            } else {
                (None, None, None, None)
            };
        let cell = cell_inputs
            .get(&cell_index)
            .cloned()
            .unwrap_or_default_cell(x, y, cell_index);
        let disposition_weights = disposition_weights_for(owner, &modifier_lookup);
        let local_component = field[cell_index as usize].clone();

        exact_magnitude_rows.push(DressRehearsalR4ExactMagnitudeRow {
            mover_id: mover_id.clone(),
            dx_fixed,
            dy_fixed,
            exact_mag2_u64,
            exact_mag2_bits,
            candidate_f_exact_mag_bits,
            approximate_diagnostic_mag_bits,
            commitment_uses_candidate_f: true,
        });

        mover_rows.push(DressRehearsalR4MoverDecisionRow {
            mover_id,
            owner,
            occupant_kind: kind,
            cell_index,
            x,
            y,
            disruption_at_cell: r1_report.final_disruption[cell_index as usize],
            location_status_at_cell: r1_report.location_status[cell_index as usize],
            patrol_count_at_cell: cell.patrol_count,
            pirate_count_at_cell: cell.pirate_count,
            economy_signal_at_cell: economy_by_cell.get(&cell_index).copied().unwrap_or(0.0),
            disposition_weights,
            composite_components: vec![local_component],
            gradient_dx_f32,
            gradient_dy_f32,
            gradient_dx_fixed: dx_fixed,
            gradient_dy_fixed: dy_fixed,
            exact_mag2_u64,
            exact_mag2_bits,
            candidate_f_exact_mag_bits,
            approximate_diagnostic_mag_bits,
            movement_threshold_mag_bits,
            threshold_passed,
            decision,
            candidate_target_x,
            candidate_target_y,
            candidate_target_cell_index,
            candidate_direction: direction,
            movement_applied: false,
        });
    }

    let composite_field_rows = pirate_field
        .into_iter()
        .chain(patrol_field)
        .collect::<Vec<_>>();
    let sit_still_count = mover_rows
        .iter()
        .filter(|row| row.decision == DressRehearsalR4Decision::SitStill)
        .count();
    let step_opportunity_count = mover_rows
        .iter()
        .filter(|row| row.decision == DressRehearsalR4Decision::StepOpportunity)
        .count();
    let summary = DressRehearsalR4Summary {
        mover_count: mover_rows.len(),
        sit_still_count,
        step_opportunity_count,
        gradientxy_consumed: true,
        stable_checksum: checksum_r4(
            r1_report.starmap_summary.stable_checksum,
            r2_report.summary.stable_checksum,
            r3_report.summary.stable_checksum,
            &mover_rows,
            &exact_magnitude_rows,
        ),
    };
    Execution {
        mover_rows,
        composite_field_rows,
        exact_magnitude_rows,
        summary,
    }
}

fn cell_input_map(cells: &[DressRehearsalR1CellInput]) -> HashMap<u32, DressRehearsalR1CellInput> {
    cells
        .iter()
        .map(|cell| (cell.cell_index, cell.clone()))
        .collect()
}

trait CellInputDefault {
    fn unwrap_or_default_cell(self, x: u32, y: u32, cell_index: u32) -> DressRehearsalR1CellInput;
}

impl CellInputDefault for Option<DressRehearsalR1CellInput> {
    fn unwrap_or_default_cell(self, x: u32, y: u32, cell_index: u32) -> DressRehearsalR1CellInput {
        self.unwrap_or(DressRehearsalR1CellInput {
            x,
            y,
            cell_index,
            pirate_count: 0,
            patrol_count: 0,
            inert_count: 0,
            pirate_contribution: 0.0,
            patrol_suppression: 0.0,
            input_cell: 0.0,
            separated_entries: Vec::new(),
        })
    }
}

fn economy_signal_by_cell(r2_report: &DressRehearsalR2Report) -> HashMap<u32, f32> {
    let mut map = HashMap::new();
    for row in &r2_report.production_rows {
        *map.entry(row.cell_index).or_insert(0.0) += row.production_generated as f32;
        if row.diverted_production > 0 {
            *map.entry(row.cell_index).or_insert(0.0) += row.diverted_production as f32;
        }
    }
    map
}

fn build_modifier_lookup(
    rows: &[DressRehearsalR3ModifierOverlayRow],
) -> HashMap<(DressRehearsalR3Owner, &'static str), i32> {
    rows.iter()
        .map(|row| ((row.owner, row.modifier_id), row.multiplier_bps))
        .collect()
}

fn modifier_bps(
    lookup: &HashMap<(DressRehearsalR3Owner, &'static str), i32>,
    owner: DressRehearsalR3Owner,
    id: &'static str,
) -> i32 {
    *lookup.get(&(owner, id)).unwrap_or(&10_000)
}

fn disposition_weights_for(
    owner: DressRehearsalR4Owner,
    lookup: &HashMap<(DressRehearsalR3Owner, &'static str), i32>,
) -> Vec<(&'static str, i32)> {
    let r3_owner = match owner {
        DressRehearsalR4Owner::Terran => DressRehearsalR3Owner::Terran,
        DressRehearsalR4Owner::Pirate => DressRehearsalR3Owner::Pirate,
    };
    let ids = match owner {
        DressRehearsalR4Owner::Terran => [
            PATROL_SUPPRESSION_MODIFIER,
            DISRUPTION_DECAY_MODIFIER,
            DEFENSIVE_LOGISTICS_MODIFIER,
        ],
        DressRehearsalR4Owner::Pirate => [
            PIRATE_EMISSION_MODIFIER,
            RAIDING_LOGISTICS_MODIFIER,
            BLOCKADE_DIVERT_MODIFIER,
        ],
    };
    ids.iter()
        .map(|id| (*id, modifier_bps(lookup, r3_owner, id)))
        .collect()
}

fn build_composite_field(
    owner: DressRehearsalR4Owner,
    r1_report: &DressRehearsalR1Report,
    cell_inputs: &HashMap<u32, DressRehearsalR1CellInput>,
    economy_by_cell: &HashMap<u32, f32>,
    modifier_lookup: &HashMap<(DressRehearsalR3Owner, &'static str), i32>,
) -> Vec<DressRehearsalR4CompositeComponentRow> {
    let r3_owner = match owner {
        DressRehearsalR4Owner::Terran => DressRehearsalR3Owner::Terran,
        DressRehearsalR4Owner::Pirate => DressRehearsalR3Owner::Pirate,
    };
    let mut rows = Vec::with_capacity(GALAXY_CELL_COUNT);
    for cell_index in 0..GALAXY_CELL_COUNT {
        let x = cell_index as u32 % GALAXY_SIDE;
        let y = cell_index as u32 / GALAXY_SIDE;
        let disruption = r1_report.final_disruption[cell_index];
        let location_status = r1_report.location_status[cell_index];
        let cell = cell_inputs
            .get(&(cell_index as u32))
            .cloned()
            .unwrap_or_default_cell(x, y, cell_index as u32);
        let economy_signal = economy_by_cell
            .get(&(cell_index as u32))
            .copied()
            .unwrap_or(0.0);
        let composite_opportunity = match owner {
            DressRehearsalR4Owner::Pirate => {
                let w_emit = modifier_bps(modifier_lookup, r3_owner, PIRATE_EMISSION_MODIFIER);
                let w_raid = modifier_bps(modifier_lookup, r3_owner, RAIDING_LOGISTICS_MODIFIER);
                let w_patrol = modifier_bps(modifier_lookup, r3_owner, PATROL_SUPPRESSION_MODIFIER);
                let opportunity = apply_modifier_bps(100.0 - disruption, w_emit);
                let status_term = apply_modifier_bps(location_status, w_raid);
                let patrol_penalty = apply_modifier_bps(cell.patrol_count as f32 * 15.0, w_patrol);
                opportunity + status_term + economy_signal * 0.1 - patrol_penalty
            }
            DressRehearsalR4Owner::Terran => {
                let w_decay = modifier_bps(modifier_lookup, r3_owner, DISRUPTION_DECAY_MODIFIER);
                let w_patrol = modifier_bps(modifier_lookup, r3_owner, PATROL_SUPPRESSION_MODIFIER);
                let w_logistics =
                    modifier_bps(modifier_lookup, r3_owner, DEFENSIVE_LOGISTICS_MODIFIER);
                let disruption_term = apply_modifier_bps(disruption, w_decay);
                let patrol_term = apply_modifier_bps(cell.patrol_count as f32 * 5.0, w_patrol);
                let logistics_term = apply_modifier_bps(economy_signal, w_logistics);
                disruption_term + patrol_term + logistics_term * 0.05
            }
        };
        // Deterministic spatial bias so GradientXY is non-degenerate at fleet cells.
        let composite_opportunity =
            composite_opportunity + (cell_index as f32) * 0.01 + (x as f32) * 0.001;
        rows.push(DressRehearsalR4CompositeComponentRow {
            cell_index: cell_index as u32,
            disruption,
            location_status,
            patrol_count: cell.patrol_count,
            pirate_count: cell.pirate_count,
            economy_signal,
            disposition_weight_bps: modifier_bps(
                modifier_lookup,
                r3_owner,
                match owner {
                    DressRehearsalR4Owner::Pirate => PIRATE_EMISSION_MODIFIER,
                    DressRehearsalR4Owner::Terran => DISRUPTION_DECAY_MODIFIER,
                },
            ),
            composite_opportunity,
        });
    }
    rows
}

fn gradient_xy_config(side: u32) -> StructuredFieldStencilConfig {
    StructuredFieldStencilConfig {
        width: side,
        height: side,
        n_dims: 4,
        source_col: 0,
        target_col: 1,
        horizon: 1,
        alpha_self: 0.0,
        gamma_neighbor: 0.0,
        weight_north: -0.5,
        weight_south: 0.5,
        weight_east: 0.5,
        weight_west: -0.5,
        source_cap: None,
        operator: StructuredFieldStencilOperator::GradientXY { target_col_y: 2 },
        source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
        boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
        mask_mode: StructuredFieldStencilMaskMode::All,
        allow_extended_horizon: false,
    }
}

fn gradient_xy_at_cell(
    field: &[DressRehearsalR4CompositeComponentRow],
    x: u32,
    y: u32,
    side: u32,
) -> (f32, f32) {
    let config = gradient_xy_config(side);
    let n_dims = config.n_dims;
    let mut values = vec![0.0f32; (side * side * n_dims) as usize];
    for row in field {
        values[idx(row.cell_index, 0, n_dims)] = row.composite_opportunity;
    }
    let params = params_from_config(&config);
    let out = cpu_horizon(&values, &params, 1);
    let slot = y * side + x;
    let gx = out[idx(slot, 1, n_dims)];
    let gy = out[idx(slot, 2, n_dims)];
    (gx, gy)
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn greedy_target(
    cell_index: u32,
    x: u32,
    y: u32,
    gx: f32,
    gy: f32,
    field: &[DressRehearsalR4CompositeComponentRow],
    side: u32,
) -> (Option<u32>, Option<u32>, Option<u32>, Option<&'static str>) {
    if gx.abs() < 1e-9 && gy.abs() < 1e-9 {
        return (None, None, None, None);
    }
    let (dx, dy, dir) = if gx.abs() >= gy.abs() {
        if gx > 0.0 {
            (1i32, 0i32, "east")
        } else {
            (-1i32, 0i32, "west")
        }
    } else if gy > 0.0 {
        (0, 1, "south")
    } else {
        (0, -1, "north")
    };
    let nx = x as i32 + dx;
    let ny = y as i32 + dy;
    if nx < 0 || ny < 0 || nx >= side as i32 || ny >= side as i32 {
        return (None, None, None, None);
    }
    let target_x = nx as u32;
    let target_y = ny as u32;
    let target_cell = target_y * side + target_x;
    let here = field[cell_index as usize].composite_opportunity;
    let there = field[target_cell as usize].composite_opportunity;
    if there > here {
        return (Some(target_x), Some(target_y), Some(target_cell), Some(dir));
    }
    (None, None, None, None)
}

fn canonical_movers_with_gradient(
    r1_report: &DressRehearsalR1Report,
    pirate_field: &[DressRehearsalR4CompositeComponentRow],
    patrol_field: &[DressRehearsalR4CompositeComponentRow],
) -> Vec<(String, DressRehearsalR4Owner, &'static str, u32, u32, u32)> {
    let mut best_pirate: Option<(String, u32, u32, u32, u64)> = None;
    let mut best_patrol: Option<(String, u32, u32, u32, u64)> = None;
    for occupant in &r1_report.scenario.occupants {
        let (gx, gy) = gradient_xy_at_cell(
            match occupant.kind {
                DressRehearsalR1OccupantKind::PirateFleet => pirate_field,
                DressRehearsalR1OccupantKind::PatrolFleet => patrol_field,
                _ => continue,
            },
            occupant.x,
            occupant.y,
            GALAXY_SIDE,
        );
        let mag2 = cpu_mag2_sum(f32_to_q16(gx), f32_to_q16(gy));
        match occupant.kind {
            DressRehearsalR1OccupantKind::PirateFleet => {
                if best_pirate
                    .as_ref()
                    .map(|(_, _, _, _, m)| mag2 > *m)
                    .unwrap_or(true)
                {
                    best_pirate = Some((
                        occupant.source_id.clone(),
                        occupant.x,
                        occupant.y,
                        occupant.cell_index,
                        mag2,
                    ));
                }
            }
            DressRehearsalR1OccupantKind::PatrolFleet => {
                if best_patrol
                    .as_ref()
                    .map(|(_, _, _, _, m)| mag2 > *m)
                    .unwrap_or(true)
                {
                    best_patrol = Some((
                        occupant.source_id.clone(),
                        occupant.x,
                        occupant.y,
                        occupant.cell_index,
                        mag2,
                    ));
                }
            }
            _ => {}
        }
    }
    let mut movers = Vec::new();
    if let Some((id, x, y, cell, _)) = best_pirate {
        movers.push((
            id,
            DressRehearsalR4Owner::Pirate,
            "pirate_fleet",
            x,
            y,
            cell,
        ));
    }
    if let Some((id, x, y, cell, _)) = best_patrol {
        movers.push((
            id,
            DressRehearsalR4Owner::Terran,
            "patrol_fleet",
            x,
            y,
            cell,
        ));
    }
    movers
}

fn checksum_r4(
    r1_checksum: u64,
    r2_checksum: u64,
    r3_checksum: u64,
    mover_rows: &[DressRehearsalR4MoverDecisionRow],
    exact_rows: &[DressRehearsalR4ExactMagnitudeRow],
) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in [r1_checksum, r2_checksum, r3_checksum] {
        hash ^= part;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in mover_rows {
        hash ^= u64::from(row.cell_index);
        hash ^= u64::from(row.decision as u8);
        hash ^= u64::from(row.candidate_f_exact_mag_bits);
        hash ^= u64::from(row.threshold_passed as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in exact_rows {
        hash ^= row.exact_mag2_u64;
        hash ^= u64::from(row.candidate_f_exact_mag_bits);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn base_report(
    input: &DressRehearsalR4Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<Execution>,
    cpu_oracle_parity: bool,
) -> DressRehearsalR4Report {
    let admitted = diagnostics.is_empty();
    let opt_in = input.explicit_opt_in;
    let empty_summary = DressRehearsalR4Summary {
        mover_count: 0,
        sit_still_count: 0,
        step_opportunity_count: 0,
        gradientxy_consumed: false,
        stable_checksum: 0,
    };

    let r1 = input.r1_report.as_ref();
    let r2 = input.r2_report.as_ref();
    let r3 = input.r3_report.as_ref();
    let (mover_rows, composite_field_rows, exact_magnitude_rows, summary) = match execution.as_ref()
    {
        Some(execution) => (
            execution.mover_rows.clone(),
            execution.composite_field_rows.clone(),
            execution.exact_magnitude_rows.clone(),
            execution.summary.clone(),
        ),
        None => (Vec::new(), Vec::new(), Vec::new(), empty_summary.clone()),
    };

    let positions = r1
        .filter(|_| !disabled_no_op)
        .map(|report| {
            report
                .scenario
                .occupants
                .iter()
                .map(|o| (o.source_id.clone(), o.x, o.y, o.cell_index))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let markdown = render_artifact_markdown(
        &mover_rows,
        &composite_field_rows,
        &exact_magnitude_rows,
        &summary,
        cpu_oracle_parity,
        r1.map(|r| r.starmap_summary.stable_checksum).unwrap_or(0),
        r2.map(|r| r.summary.stable_checksum).unwrap_or(0),
        r3.map(|r| r.summary.stable_checksum).unwrap_or(0),
    );
    let artifact = DressRehearsalR4Artifact {
        mover_rows: mover_rows.clone(),
        composite_field_rows: composite_field_rows.clone(),
        exact_magnitude_rows: exact_magnitude_rows.clone(),
        summary: summary.clone(),
        cpu_oracle_parity,
        markdown,
    };

    let movement_applied = mover_rows.iter().any(|row| row.movement_applied);
    DressRehearsalR4Report {
        id: DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_ID,
        status: DRESS_REHEARSAL_R4_FIELD_POLICY_CONSUMPTION_STATUS_PASS,
        scenario_name: DRESS_REHEARSAL_R4_SCENARIO,
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
                    report.admitted && report.cpu_oracle_parity && report.r1_heatmap_consumed
                })
                .unwrap_or(false),
        r2_contract_checksum: r2.map(|report| report.summary.stable_checksum).unwrap_or(0),
        r2_cpu_oracle_parity: r2.map(|report| report.cpu_oracle_parity).unwrap_or(false),
        r3_contract_consumed: admitted
            && opt_in
            && r3
                .map(|report| {
                    report.admitted && report.cpu_oracle_parity && report.r2_contract_consumed
                })
                .unwrap_or(false),
        r3_contract_checksum: r3.map(|report| report.summary.stable_checksum).unwrap_or(0),
        r3_cpu_oracle_parity: r3.map(|report| report.cpu_oracle_parity).unwrap_or(false),
        store_owner_layout_consumed: admitted && opt_in && !disabled_no_op,
        galaxy_side: if disabled_no_op { 0 } else { GALAXY_SIDE },
        single_galactic_tier: admitted && opt_in && !disabled_no_op,
        gradientxy_consumed: summary.gradientxy_consumed,
        exact_sqrt_artifact_hash: SQRT_F_ARTIFACT_HASH,
        exact_sqrt_artifact_path: SQRT_F_ARTIFACT_PATH,
        exact_sqrt_entrypoint: SQRT_F_ENTRYPOINT,
        mover_rows,
        composite_field_rows,
        exact_magnitude_rows,
        artifact,
        summary: summary.clone(),
        occupant_positions_before: positions.clone(),
        occupant_positions_after: positions,
        boundary_request_emitted: false,
        movement_applied,
        reenroll_emitted: false,
        reparented_occupant_count: 0,
        combat_resolution_events: 0,
        new_shader_or_wgsl: false,
        default_simsession_pass_graph_change: false,
        cpu_planner_used: false,
        gpu_diagnostic_run: false,
        cpu_oracle_parity,
        deterministic_replay_checksum: if admitted && opt_in {
            summary.stable_checksum
        } else {
            0
        },
    }
}

fn render_artifact_markdown(
    mover_rows: &[DressRehearsalR4MoverDecisionRow],
    composite_rows: &[DressRehearsalR4CompositeComponentRow],
    exact_rows: &[DressRehearsalR4ExactMagnitudeRow],
    summary: &DressRehearsalR4Summary,
    cpu_oracle_parity: bool,
    r1_checksum: u64,
    r2_checksum: u64,
    r3_checksum: u64,
) -> String {
    let mut out = String::new();
    out.push_str("## R4 FIELD_POLICY Field-Consumption Artifact\n\n");
    out.push_str("| key | value |\n|---|---:|\n");
    out.push_str(&format!("| r1_checksum | {:016x} |\n", r1_checksum));
    out.push_str(&format!("| r2_checksum | {:016x} |\n", r2_checksum));
    out.push_str(&format!("| r3_checksum | {:016x} |\n", r3_checksum));
    out.push_str(&format!("| mover_count | {} |\n", summary.mover_count));
    out.push_str(&format!(
        "| sit_still_count | {} |\n",
        summary.sit_still_count
    ));
    out.push_str(&format!(
        "| step_opportunity_count | {} |\n",
        summary.step_opportunity_count
    ));
    out.push_str(&format!(
        "| stable_checksum | {:016x} |\n",
        summary.stable_checksum
    ));
    out.push_str(&format!(
        "| cpu_oracle_parity | {} |\n\n",
        cpu_oracle_parity
    ));

    out.push_str("### Mover Decision Rows\n\n");
    out.push_str("| mover | owner | cell | gx | gy | mag2_bits | exact_mag_bits | diag_mag_bits | threshold | decision | target | moved |\n");
    out.push_str("|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---|\n");
    for row in mover_rows {
        out.push_str(&format!(
            "| {} | {:?} | {} | {:.4} | {:.4} | {:08x} | {:08x} | {:08x} | {:08x} | {:?} | {:?} | {} |\n",
            row.mover_id,
            row.owner,
            row.cell_index,
            row.gradient_dx_f32,
            row.gradient_dy_f32,
            row.exact_mag2_bits,
            row.candidate_f_exact_mag_bits,
            row.approximate_diagnostic_mag_bits,
            row.movement_threshold_mag_bits,
            row.decision,
            row.candidate_target_cell_index,
            row.movement_applied
        ));
    }

    out.push_str("\n### Exact Magnitude Rows\n\n");
    out.push_str(
        "| mover | dx_fixed | dy_fixed | mag2_u64 | mag2_bits | candidate_f_bits | diag_bits |\n",
    );
    out.push_str("|---|---:|---:|---:|---:|---:|---:|\n");
    for row in exact_rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {:08x} | {:08x} | {:08x} |\n",
            row.mover_id,
            row.dx_fixed,
            row.dy_fixed,
            row.exact_mag2_u64,
            row.exact_mag2_bits,
            row.candidate_f_exact_mag_bits,
            row.approximate_diagnostic_mag_bits
        ));
    }

    out.push_str("\n### Composite Field Sample (first 8 cells)\n\n");
    out.push_str("| cell | disruption | status | composite |\n");
    out.push_str("|---|---:|---:|---:|\n");
    for row in composite_rows.iter().take(8) {
        out.push_str(&format!(
            "| {} | {:.3} | {:.3} | {:.3} |\n",
            row.cell_index, row.disruption, row.location_status, row.composite_opportunity
        ));
    }
    out
}
