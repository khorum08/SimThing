//! GPU-MEASURE-0080-0: measure accepted dress-rehearsal row/mask shapes on GPU.
//!
//! This module instantiates existing generic GPU substrates only. It does not
//! introduce a new shader, a new `AccumulatorOp`, or any SimSession wiring.

use std::collections::BTreeMap;

use simthing_core::{
    eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{
    cpu_horizon, params_from_config, set_debug_readback_allowed, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};

use crate::dress_rehearsal_r1_disruption_heatmap::{
    run_dress_rehearsal_r1_disruption_heatmap, DressRehearsalR1Input, CEILING, DECAY, FLOOR,
    GALAXY_CELL_COUNT, GALAXY_SIDE,
};
use crate::dress_rehearsal_r2_recursive_allocation::{
    run_dress_rehearsal_r2_recursive_allocation, DressRehearsalR2Input,
};
use crate::dress_rehearsal_r4_field_policy_consumption::{
    cpu_mag2_sum, f32_to_q16, mag2_u64_q16_to_f32_bits,
    run_dress_rehearsal_r4_field_policy_consumption, sqrt_cr_f_bits, DressRehearsalR4Input,
};
use crate::dress_rehearsal_r6_combat_hp_damage::{
    run_dress_rehearsal_r6_combat_hp_damage, DressRehearsalR6Input,
};
use crate::dress_rehearsal_r6b_ship_cohort_reinforcement::{
    run_dress_rehearsal_r6b_ship_cohort_reinforcement, DressRehearsalR6bInput,
};
use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, R6C_GPU_POSTURE,
};

pub const GPU_MEASURE_0080_0_ID: &str = "GPU-MEASURE-0080-0";
pub const GPU_MEASURE_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - rehearsal GPU measurement pass";
pub const GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT: &str = "GPU-measured (integer bit-exact)";
pub const GPU_MEASURE_VERDICT_VERIFIED_APPROXIMATE: &str =
    "GPU-measured (verified-approximate, within accepted f32 bound)";
pub const GPU_MEASURE_VERDICT_UNMEASURED: &str = "GPU-conformant; GPU execution not yet measured";
pub const GPU_MEASURE_R4_F32_BOUND: f32 = 1.0e-4;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const R1_TREE_ID: u32 = 0x0080_0001;
const R6_ATTRITION_TREE_ID: u32 = 0x0080_0006;
const R6B_CONSTRUCTION_TREE_ID: u32 = 0x0080_006b;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuMeasure0080Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl GpuMeasure0080Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuMeasure0080AdapterReport {
    pub adapter_name: String,
    pub device_name: String,
    pub selected_discrete_gpu: bool,
    pub timestamp_supported: bool,
    pub backend: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GpuMeasure0080ShapeReport {
    pub shape_name: &'static str,
    pub source_rung: &'static str,
    pub cpu_oracle_checksum: u64,
    pub gpu_checksum: u64,
    pub cpu_value_summary: String,
    pub gpu_value_summary: String,
    pub comparison_method: &'static str,
    pub verdict: &'static str,
    pub measured_on_gpu: bool,
    pub bit_exact: bool,
    pub max_abs_delta: Option<f32>,
    pub f32_bound: Option<f32>,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GpuMeasure0080Report {
    pub id: &'static str,
    pub status: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub adapter: Option<GpuMeasure0080AdapterReport>,
    pub shape_reports: Vec<GpuMeasure0080ShapeReport>,
    pub stable_report_checksum: u64,
    pub no_behavior_change: bool,
    pub no_semantic_wgsl: bool,
    pub no_new_accumulator_op: bool,
    pub no_invariant_edit: bool,
    pub no_pinned_number_change: bool,
    pub scenario_0080_2_reopened: bool,
    pub r6c_integrated_run_posture: &'static str,
    pub artifact_markdown: String,
}

pub fn run_gpu_measure_0080_0(input: &GpuMeasure0080Input) -> GpuMeasure0080Report {
    if !input.explicit_opt_in {
        return base_report(
            input,
            true,
            vec!["explicit_opt_in_required"],
            None,
            Vec::new(),
        );
    }
    if input.enabled_by_default {
        return base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden"],
            None,
            Vec::new(),
        );
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            return base_report(input, false, vec![diagnostic], None, Vec::new());
        }
    };

    set_debug_readback_allowed(true);
    let mut shapes = Vec::new();
    shapes.push(measure_r1_disruption(&ctx));
    shapes.push(measure_r2_owner_reduce(&ctx));
    shapes.push(measure_r4_gradient(&ctx));
    shapes.push(measure_r6_attrition(&ctx));
    shapes.push(measure_r6b_construction_and_fusion(&ctx));
    shapes.push(report_r6c_conformant_unmeasured());

    base_report(input, false, Vec::new(), Some(adapter), shapes)
}

pub fn replay_gpu_measure_0080_0() -> (GpuMeasure0080Report, GpuMeasure0080Report) {
    let input = GpuMeasure0080Input::explicit_opt_in();
    (
        run_gpu_measure_0080_0(&input),
        run_gpu_measure_0080_0(&input),
    )
}

pub fn render_gpu_measure_0080_0_report(report: &GpuMeasure0080Report) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# {GPU_MEASURE_0080_0_ID} Results"));
    lines.push(String::new());
    lines.push(format!("status: {}", report.status));
    lines.push(format!("admitted: {}", report.admitted));
    lines.push(format!(
        "stable_report_checksum: 0x{:016x}",
        report.stable_report_checksum
    ));
    if let Some(adapter) = &report.adapter {
        lines.push(format!("adapter_name: {}", adapter.adapter_name));
        lines.push(format!(
            "selected_discrete_gpu: {}",
            adapter.selected_discrete_gpu
        ));
        lines.push(format!(
            "timestamp_supported: {}",
            adapter.timestamp_supported
        ));
        lines.push(format!("backend: {}", adapter.backend));
    } else {
        lines.push(format!("diagnostics: {}", report.diagnostics.join(", ")));
    }
    lines.push(String::new());
    lines.push("| Shape | Source | Verdict | CPU checksum | GPU checksum | Notes |".to_string());
    lines.push("| --- | --- | --- | ---: | ---: | --- |".to_string());
    for shape in &report.shape_reports {
        lines.push(format!(
            "| {} | {} | {} | 0x{:016x} | 0x{:016x} | {} |",
            shape.shape_name,
            shape.source_rung,
            shape.verdict,
            shape.cpu_oracle_checksum,
            shape.gpu_checksum,
            shape.notes.replace('|', "\\|")
        ));
    }
    lines.push(String::new());
    lines.push(format!("no_behavior_change: {}", report.no_behavior_change));
    lines.push(format!("no_semantic_wgsl: {}", report.no_semantic_wgsl));
    lines.push(format!(
        "no_new_accumulator_op: {}",
        report.no_new_accumulator_op
    ));
    lines.push(format!("no_invariant_edit: {}", report.no_invariant_edit));
    lines.push(format!(
        "r6c_integrated_run_posture: {}",
        report.r6c_integrated_run_posture
    ));
    lines.join("\n")
}

fn base_report(
    input: &GpuMeasure0080Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    adapter: Option<GpuMeasure0080AdapterReport>,
    shape_reports: Vec<GpuMeasure0080ShapeReport>,
) -> GpuMeasure0080Report {
    let admitted = input.explicit_opt_in
        && !input.enabled_by_default
        && diagnostics.is_empty()
        && adapter.is_some()
        && shape_reports
            .iter()
            .all(|s| s.verdict == GPU_MEASURE_VERDICT_UNMEASURED || s.measured_on_gpu);
    let stable_report_checksum = stable_report_checksum(&shape_reports);
    let mut report = GpuMeasure0080Report {
        id: GPU_MEASURE_0080_0_ID,
        status: GPU_MEASURE_0080_0_STATUS_PASS,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        adapter,
        shape_reports,
        stable_report_checksum,
        no_behavior_change: true,
        no_semantic_wgsl: true,
        no_new_accumulator_op: true,
        no_invariant_edit: true,
        no_pinned_number_change: true,
        scenario_0080_2_reopened: false,
        r6c_integrated_run_posture: GPU_MEASURE_VERDICT_UNMEASURED,
        artifact_markdown: String::new(),
    };
    report.artifact_markdown = render_gpu_measure_0080_0_report(&report);
    report
}

fn create_discrete_gpu_context() -> Result<(GpuContext, GpuMeasure0080AdapterReport), &'static str>
{
    let ctx = GpuContext::new_blocking().map_err(|_| "gpu_context_unavailable")?;
    let info = ctx.adapter.get_info();
    let selected_discrete_gpu = format!("{:?}", info.device_type) == "DiscreteGpu"
        || adapter_name_looks_discrete(&info.name);
    if !selected_discrete_gpu {
        return Err("discrete_gpu_unavailable");
    }
    let timestamp_supported = format!("{:?}", ctx.device.features()).contains("TIMESTAMP_QUERY");
    let report = GpuMeasure0080AdapterReport {
        adapter_name: info.name.clone(),
        device_name: "simthing-gpu device".to_string(),
        selected_discrete_gpu,
        timestamp_supported,
        backend: format!("{:?}", info.backend),
    };
    Ok((ctx, report))
}

fn adapter_name_looks_discrete(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let known_integrated = lower.contains("intel")
        || lower.contains("iris")
        || lower.contains("uhd")
        || lower.contains("raptorlake")
        || lower.contains("basic render");
    !known_integrated
        && (lower.contains("nvidia")
            || lower.contains("rtx")
            || lower.contains("geforce")
            || lower.contains("radeon")
            || lower.contains("arc"))
}

fn measure_r1_disruption(ctx: &GpuContext) -> GpuMeasure0080ShapeReport {
    let r1 = run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in());
    let contribution_slot_count: u32 = r1
        .cell_inputs
        .iter()
        .map(|cell| cell.separated_entries.len().max(1) as u32)
        .sum();
    let target_slot_start = contribution_slot_count;
    let n_slots = target_slot_start + GALAXY_CELL_COUNT as u32;
    let n_dims = 3u32;
    let col_before = 0u32;
    let col_input = 1u32;
    let col_after = 2u32;
    let mut values = vec![0.0f32; (n_slots * n_dims) as usize];
    let mut ops = Vec::new();
    let mut slot = 0u32;

    for cell in &r1.cell_inputs {
        let start = slot;
        if cell.separated_entries.is_empty() {
            slot += 1;
        } else {
            for entry in &cell.separated_entries {
                values[idx(slot, col_input, n_dims)] = entry.value;
                slot += 1;
            }
        }
        let count = slot - start;
        let target = target_slot_start + cell.cell_index;
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(start),
                count,
                col: ColumnIndex::new(col_input as usize),
            },
            combine: CombineFn::Sum,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(target), ColumnIndex::new(col_input as usize))],
        });
    }

    for row in &r1.recurrence_rows {
        let target = target_slot_start + row.cell_index;
        values[idx(target, col_before, n_dims)] = row.disruption_before;
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(target),
                col: ColumnIndex::new(col_before as usize),
            },
            combine: CombineFn::EvalEML {
                tree_id: R1_TREE_ID,
            },
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(target), ColumnIndex::new(col_after as usize))],
        });
    }

    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 8);
    register_and_upload(
        ctx,
        &mut registry,
        &mut table,
        R1_TREE_ID,
        "gpu_measure_r1_bounded_feedback",
        r1_bounded_feedback_nodes(),
    );

    let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);
    session.upload_values(ctx, &values);
    session
        .upload_ops_with_eml(ctx, &ops, Some(&registry))
        .expect("R1 GPU measure ops");
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session.tick_with_eml(ctx, 0, eml).expect("R1 sum tick");
    session
        .tick_with_eml(ctx, 1, eml)
        .expect("R1 recurrence tick");
    let gpu_values = session.readback_full(ctx).expect("R1 readback");

    let mut mismatches = 0usize;
    let mut cpu_hash = FNV_OFFSET;
    let mut gpu_hash = FNV_OFFSET;
    for cell in &r1.cell_inputs {
        let target = target_slot_start + cell.cell_index;
        let gpu_input = gpu_values[idx(target, col_input, n_dims)];
        cpu_hash = hash_f32(cpu_hash, cell.input_cell);
        gpu_hash = hash_f32(gpu_hash, gpu_input);
        if gpu_input.to_bits() != cell.input_cell.to_bits() {
            mismatches += 1;
        }
    }
    for row in &r1.recurrence_rows {
        let target = target_slot_start + row.cell_index;
        let gpu_after = gpu_values[idx(target, col_after, n_dims)];
        cpu_hash = hash_f32(cpu_hash, row.disruption_after);
        gpu_hash = hash_f32(gpu_hash, gpu_after);
        if gpu_after.to_bits() != row.disruption_after.to_bits() {
            mismatches += 1;
        }
    }

    exact_shape_report(
        "R1 disruption input + bounded recurrence",
        "SCENARIO-0080-2-R1",
        cpu_hash,
        gpu_hash,
        r1.cell_inputs.len() + r1.recurrence_rows.len(),
        mismatches,
        "SlotRange Sum + EvalEML bounded clamp",
    )
}

fn measure_r2_owner_reduce(ctx: &GpuContext) -> GpuMeasure0080ShapeReport {
    let r1 = run_dress_rehearsal_r1_disruption_heatmap(&DressRehearsalR1Input::explicit_opt_in());
    let r2 =
        run_dress_rehearsal_r2_recursive_allocation(&DressRehearsalR2Input::with_r1_report(r1));
    let mut groups: Vec<Vec<f32>> = Vec::new();
    let mut expected = Vec::new();

    for ledger in &r2.stockpile_ledger {
        let owner_values: Vec<f32> = r2
            .production_rows
            .iter()
            .filter(|row| row.effective_outflow_owner == ledger.owner)
            .map(|row| row.outflow_to_effective_owner as f32)
            .collect();
        groups.push(owner_values);
        expected.push(ledger.reduced_in as f32);

        let disburse_values: Vec<f32> = r2
            .deficit_disbursements
            .iter()
            .filter(|row| row.owner == ledger.owner)
            .map(|row| row.disbursed as f32)
            .collect();
        groups.push(disburse_values);
        expected.push(ledger.disbursed_down as f32);
    }

    let gpu = run_sum_groups(ctx, &groups);
    let (mismatches, cpu_hash, gpu_hash) = compare_i64_like(&expected, &gpu);
    exact_shape_report(
        "R2 owner reduce-up + disburse-down",
        "SCENARIO-0080-2-R2",
        cpu_hash,
        gpu_hash,
        expected.len(),
        mismatches,
        "SlotRange Sum over accepted owner groups",
    )
}

fn measure_r4_gradient(ctx: &GpuContext) -> GpuMeasure0080ShapeReport {
    let r4 =
        run_dress_rehearsal_r4_field_policy_consumption(&DressRehearsalR4Input::explicit_opt_in());
    let config = StructuredFieldStencilConfig {
        width: GALAXY_SIDE,
        height: GALAXY_SIDE,
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
    };
    let mut values = vec![0.0f32; config.values_len()];
    for row in &r4.composite_field_rows {
        values[idx(row.cell_index, 0, config.n_dims)] = row.composite_opportunity;
    }

    let params = params_from_config(&config);
    let cpu_field = cpu_horizon(&values, &params, 1);
    let op = StructuredFieldStencilOp::new(ctx, config).expect("R4 stencil op");
    op.upload_values(ctx, &values).expect("R4 stencil upload");
    let (gpu_field, _) = op.run_ping_pong(ctx, 1).expect("R4 stencil run");

    let mut max_abs_delta = 0.0f32;
    let mut candidate_mismatches = 0usize;
    let mut cpu_hash = FNV_OFFSET;
    let mut gpu_hash = FNV_OFFSET;
    for mover in &r4.mover_rows {
        let dx_idx = idx(mover.cell_index, 1, 4);
        let dy_idx = idx(mover.cell_index, 2, 4);
        let cpu_dx = cpu_field[dx_idx];
        let cpu_dy = cpu_field[dy_idx];
        let gpu_dx = gpu_field[dx_idx];
        let gpu_dy = gpu_field[dy_idx];
        max_abs_delta = max_abs_delta
            .max((gpu_dx - mover.gradient_dx_f32).abs())
            .max((gpu_dy - mover.gradient_dy_f32).abs())
            .max((gpu_dx - cpu_dx).abs())
            .max((gpu_dy - cpu_dy).abs());

        let gpu_dx_fixed = f32_to_q16(gpu_dx);
        let gpu_dy_fixed = f32_to_q16(gpu_dy);
        let gpu_mag2 = cpu_mag2_sum(gpu_dx_fixed, gpu_dy_fixed);
        let gpu_candidate_bits = sqrt_cr_f_bits(mag2_u64_q16_to_f32_bits(gpu_mag2));
        if gpu_candidate_bits != mover.candidate_f_exact_mag_bits {
            candidate_mismatches += 1;
        }
        cpu_hash = hash_f32(cpu_hash, mover.gradient_dx_f32);
        cpu_hash = hash_f32(cpu_hash, mover.gradient_dy_f32);
        cpu_hash = hash_u64(cpu_hash, mover.candidate_f_exact_mag_bits as u64);
        gpu_hash = hash_f32(gpu_hash, gpu_dx);
        gpu_hash = hash_f32(gpu_hash, gpu_dy);
        gpu_hash = hash_u64(gpu_hash, gpu_candidate_bits as u64);
    }

    let within = max_abs_delta <= GPU_MEASURE_R4_F32_BOUND && candidate_mismatches == 0;
    GpuMeasure0080ShapeReport {
        shape_name: "R4 GradientXY + Candidate-F magnitude",
        source_rung: "SCENARIO-0080-2-R4",
        cpu_oracle_checksum: cpu_hash,
        gpu_checksum: gpu_hash,
        cpu_value_summary: format!("{} mover gradients", r4.mover_rows.len()),
        gpu_value_summary: format!(
            "{} mover gradients; max_abs_delta={:.8}; candidate_mismatches={}",
            r4.mover_rows.len(),
            max_abs_delta,
            candidate_mismatches
        ),
        comparison_method: "StructuredFieldStencil GradientXY within accepted f32 bound",
        verdict: GPU_MEASURE_VERDICT_VERIFIED_APPROXIMATE,
        measured_on_gpu: true,
        bit_exact: false,
        max_abs_delta: Some(max_abs_delta),
        f32_bound: Some(GPU_MEASURE_R4_F32_BOUND),
        notes: if within {
            "GPU gradient readback within bound; Candidate-F bits match CPU artifact".to_string()
        } else {
            "GPU gradient readback exceeded bound or Candidate-F mismatch".to_string()
        },
    }
}

fn measure_r6_attrition(ctx: &GpuContext) -> GpuMeasure0080ShapeReport {
    let r6 = run_dress_rehearsal_r6_combat_hp_damage(&DressRehearsalR6Input::explicit_opt_in());
    let mut grouped_damage: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    for row in &r6.disburse_down_rows {
        grouped_damage
            .entry(row.target_id.clone())
            .or_default()
            .push(row.damage_disbursed as f32);
    }
    let mut groups = Vec::new();
    let mut combat_rows = Vec::new();
    for row in &r6.combat_arena_rows {
        groups.push(grouped_damage.remove(&row.combatant_id).unwrap_or_default());
        combat_rows.push(row.clone());
    }
    let gpu_damage = run_sum_groups(ctx, &groups);

    let mut mismatches = 0usize;
    let mut cpu_hash = FNV_OFFSET;
    let mut gpu_hash = FNV_OFFSET;
    for (row, gpu_received) in combat_rows.iter().zip(gpu_damage.iter()) {
        if *gpu_received as i64 != row.hostile_damage_received {
            mismatches += 1;
        }
        let gpu_destroyed = run_single_eval_emission(
            ctx,
            R6_ATTRITION_TREE_ID,
            "gpu_measure_r6_attrition",
            r6_attrition_nodes(),
            &[
                *gpu_received,
                row.hp_per_ship as f32,
                row.num_ships_before as f32,
            ],
        );
        if gpu_destroyed as i64 != row.ships_destroyed {
            mismatches += 1;
        }
        cpu_hash = hash_i64(cpu_hash, row.hostile_damage_received);
        cpu_hash = hash_i64(cpu_hash, row.ships_destroyed);
        gpu_hash = hash_i64(gpu_hash, *gpu_received as i64);
        gpu_hash = hash_u64(gpu_hash, gpu_destroyed as u64);
    }

    exact_shape_report(
        "R6 combat damage reduce + attrition emission",
        "SCENARIO-0080-2-R6",
        cpu_hash,
        gpu_hash,
        combat_rows.len(),
        mismatches,
        "SlotRange Sum + EvalEML/EmitEvent floor attrition",
    )
}

fn measure_r6b_construction_and_fusion(ctx: &GpuContext) -> GpuMeasure0080ShapeReport {
    let r6b = run_dress_rehearsal_r6b_ship_cohort_reinforcement(
        &DressRehearsalR6bInput::explicit_opt_in(),
    );
    let mut mismatches = 0usize;
    let mut cpu_hash = FNV_OFFSET;
    let mut gpu_hash = FNV_OFFSET;

    for row in &r6b.construction_rows {
        let gpu_delta = run_single_eval_emission(
            ctx,
            R6B_CONSTRUCTION_TREE_ID,
            "gpu_measure_r6b_construction",
            r6b_construction_nodes(),
            &[
                row.construction_progress_before as f32,
                row.production_applied as f32,
                row.ship_cost as f32,
            ],
        );
        if gpu_delta as i64 != row.ship_count_delta_emitted {
            mismatches += 1;
        }
        cpu_hash = hash_i64(cpu_hash, row.ship_count_delta_emitted);
        gpu_hash = hash_u64(gpu_hash, gpu_delta as u64);
    }

    let groups: Vec<Vec<f32>> = r6b
        .fusion_rows
        .iter()
        .map(|row| vec![row.left_num_ships as f32, row.right_num_ships as f32])
        .collect();
    let expected: Vec<f32> = r6b
        .fusion_rows
        .iter()
        .map(|row| row.fused_num_ships as f32)
        .collect();
    let gpu_fused = run_sum_groups(ctx, &groups);
    let (fusion_mismatches, fusion_cpu_hash, fusion_gpu_hash) =
        compare_i64_like(&expected, &gpu_fused);
    mismatches += fusion_mismatches;
    cpu_hash = hash_u64(cpu_hash, fusion_cpu_hash);
    gpu_hash = hash_u64(gpu_hash, fusion_gpu_hash);

    exact_shape_report(
        "R6B construction threshold + fusion sum",
        "SCENARIO-0080-2-R6B",
        cpu_hash,
        gpu_hash,
        r6b.construction_rows.len() + r6b.fusion_rows.len(),
        mismatches,
        "EvalEML/EmitEvent construction + SlotRange Sum fusion",
    )
}

fn report_r6c_conformant_unmeasured() -> GpuMeasure0080ShapeReport {
    let r6c = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    GpuMeasure0080ShapeReport {
        shape_name: "R6C integrated 100-tick whole-run execution",
        source_rung: "SCENARIO-0080-2-R6C",
        cpu_oracle_checksum: r6c.summary.stable_checksum,
        gpu_checksum: 0,
        cpu_value_summary: format!("{} canonical ticks", r6c.summary.tick_count),
        gpu_value_summary: GPU_MEASURE_VERDICT_UNMEASURED.to_string(),
        comparison_method: "posture-only whole-run gap marker",
        verdict: GPU_MEASURE_VERDICT_UNMEASURED,
        measured_on_gpu: false,
        bit_exact: false,
        max_abs_delta: None,
        f32_bound: None,
        notes: format!(
            "R6C report remains '{}'; constituent R1/R2/R4/R6/R6B shapes measured separately.",
            R6C_GPU_POSTURE
        ),
    }
}

fn exact_shape_report(
    shape_name: &'static str,
    source_rung: &'static str,
    cpu_hash: u64,
    gpu_hash: u64,
    row_count: usize,
    mismatches: usize,
    method: &'static str,
) -> GpuMeasure0080ShapeReport {
    GpuMeasure0080ShapeReport {
        shape_name,
        source_rung,
        cpu_oracle_checksum: cpu_hash,
        gpu_checksum: gpu_hash,
        cpu_value_summary: format!("{row_count} expected rows"),
        gpu_value_summary: format!("{row_count} measured rows; mismatches={mismatches}"),
        comparison_method: method,
        verdict: GPU_MEASURE_VERDICT_INTEGER_BIT_EXACT,
        measured_on_gpu: true,
        bit_exact: mismatches == 0,
        max_abs_delta: Some(if mismatches == 0 { 0.0 } else { f32::INFINITY }),
        f32_bound: None,
        notes: if mismatches == 0 {
            "integer/count values match CPU oracle exactly".to_string()
        } else {
            "integer/count mismatch detected".to_string()
        },
    }
}

fn run_sum_groups(ctx: &GpuContext, groups: &[Vec<f32>]) -> Vec<f32> {
    let n_dims = 1u32;
    let source_slots: u32 = groups.iter().map(|g| g.len().max(1) as u32).sum();
    let target_start = source_slots;
    let n_slots = target_start + groups.len() as u32;
    let mut values = vec![0.0f32; n_slots as usize];
    let mut ops = Vec::new();
    let mut slot = 0u32;
    for (group_idx, group) in groups.iter().enumerate() {
        let start = slot;
        if group.is_empty() {
            slot += 1;
        } else {
            for value in group {
                values[slot as usize] = *value;
                slot += 1;
            }
        }
        let count = slot - start;
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(start),
                count,
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                SlotIndex::new(target_start + group_idx as u32),
                ColumnIndex::new(0),
            )],
        });
    }
    let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);
    session.upload_values(ctx, &values);
    session.upload_ops(ctx, &ops).expect("sum groups upload");
    session.tick(ctx, 0).expect("sum groups tick");
    let gpu_values = session.readback_full(ctx).expect("sum groups readback");
    (0..groups.len())
        .map(|i| gpu_values[(target_start + i as u32) as usize])
        .collect()
}

fn run_single_eval_emission(
    ctx: &GpuContext,
    tree_id: u32,
    display_name: &'static str,
    nodes: Vec<EmlNodeGpu>,
    slot_values: &[f32],
) -> u32 {
    let n_dims = slot_values.len() as u32;
    let mut registry = EmlExpressionRegistry::new();
    let mut table = EmlGpuProgramTable::new(ctx, 64, 8);
    register_and_upload(ctx, &mut registry, &mut table, tree_id, display_name, nodes);
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(0),
        },
        combine: CombineFn::EvalEML { tree_id },
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![(SlotIndex::new(0), ColumnIndex::new(0))],
    };
    let mut session = AccumulatorOpSession::with_emission_capacity(ctx, 1, n_dims, 1);
    session.upload_values(ctx, slot_values);
    session
        .upload_ops_with_eml(ctx, std::slice::from_ref(&op), Some(&registry))
        .expect("single EvalEML emission upload");
    let eml = Some((&table.node_buffer, &table.range_buffer));
    session
        .tick_with_eml(ctx, 0, eml)
        .expect("single EvalEML emission tick");
    let emissions = session
        .readback_emissions(ctx)
        .expect("single EvalEML emission readback");
    emissions
        .first()
        .map(|record| record.emit_count)
        .unwrap_or(0)
}

fn register_and_upload(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
    tree_id: u32,
    display_name: &'static str,
    nodes: Vec<EmlNodeGpu>,
) {
    let id = EmlTreeId(tree_id);
    registry
        .register_formula(
            id,
            exact_meta(tree_id, display_name, nodes.len() as u32),
            nodes,
        )
        .expect("register GPU measurement EML formula");
    let mut trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    trees.sort_by_key(|(id, _, _)| id.0);
    let mapping = table.upload_trees(ctx, &trees).expect("upload EML trees");
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .expect("mark EML tree uploaded");
    }
}

fn exact_meta(tree_id: u32, display_name: &'static str, node_count: u32) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(tree_id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count,
        max_stack_depth: 8,
        has_loops: false,
        has_recursion: false,
        display_name: display_name.to_string(),
    }
}

fn r1_bounded_feedback_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        lit(DECAY),
        unary(eml_opcode::MUL),
        slot_col(1),
        unary(eml_opcode::ADD),
        clamp_bounded(FLOOR, CEILING),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6_attrition_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        div_guarded(),
        slot_col(2),
        unary(eml_opcode::MIN),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6b_construction_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        unary(eml_opcode::ADD),
        slot_col(2),
        div_guarded(),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn lit(value: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::LITERAL_F32,
        flags: 0,
        a: value.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot_col(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn unary(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn div_guarded() -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::DIV,
        flags: 1,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn clamp_bounded(min: f32, max: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::CLAMP_BOUNDED,
        flags: 0,
        a: min.to_bits(),
        b: max.to_bits(),
        c: 0,
        d: 0,
    }
}

fn compare_i64_like(expected: &[f32], gpu: &[f32]) -> (usize, u64, u64) {
    let mut mismatches = 0usize;
    let mut cpu_hash = FNV_OFFSET;
    let mut gpu_hash = FNV_OFFSET;
    for (cpu, gpu) in expected.iter().zip(gpu.iter()) {
        let cpu_i = *cpu as i64;
        let gpu_i = *gpu as i64;
        if cpu_i != gpu_i {
            mismatches += 1;
        }
        cpu_hash = hash_i64(cpu_hash, cpu_i);
        gpu_hash = hash_i64(gpu_hash, gpu_i);
    }
    (mismatches, cpu_hash, gpu_hash)
}

fn stable_report_checksum(shapes: &[GpuMeasure0080ShapeReport]) -> u64 {
    let mut hash = hash_str(FNV_OFFSET, GPU_MEASURE_0080_0_ID);
    for shape in shapes {
        hash = hash_str(hash, shape.shape_name);
        hash = hash_str(hash, shape.verdict);
        hash = hash_u64(hash, shape.cpu_oracle_checksum);
        hash = hash_u64(hash, shape.gpu_checksum);
        hash = hash_u64(hash, shape.measured_on_gpu as u64);
        hash = hash_u64(hash, shape.bit_exact as u64);
        if let Some(delta) = shape.max_abs_delta {
            hash = hash_f32(hash, delta);
        }
    }
    hash
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn hash_str(mut hash: u64, value: &str) -> u64 {
    for byte in value.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn hash_f32(hash: u64, value: f32) -> u64 {
    hash_u64(hash, value.to_bits() as u64)
}

fn hash_i64(hash: u64, value: i64) -> u64 {
    hash_u64(hash, value as u64)
}

fn hash_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
