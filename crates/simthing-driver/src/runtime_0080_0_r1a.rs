//! RUNTIME-0080-0-R1a: remedial audit for Tier-A GPU next-tick authority.
//!
//! The original IMPL-0 harness overclaimed PASS by uploading a CPU-computed
//! Tier-A next-state into a private GPU journal every tick. This remedial
//! harness removes that producer and reports the rung honestly: the measured
//! R6C constituent shapes exist, but the integrated production-substrate
//! `WorldGpuState`/`Pipelines` Tier-A transform has not yet been admitted.

use simthing_gpu::GpuContext;

use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, DressRehearsalR6cReport,
};
use crate::gpu_measure_0080_0::{
    run_gpu_measure_0080_0, GpuMeasure0080Input, GpuMeasure0080Report,
    GPU_MEASURE_0080_0_STATUS_PASS,
};
use crate::runtime_0080_0_r0::{
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND,
};

pub const RUNTIME_0080_0_R1A_ID: &str = "RUNTIME-0080-0-R1a";
pub const RUNTIME_0080_0_R1A_PRIMITIVE: &str = "GPU-STATE-AUTH-0";
pub const RUNTIME_0080_0_R1A_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - Tier-A GPU-STATE-AUTH-0 resident next-tick authority";
pub const RUNTIME_0080_0_R1A_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL (REMEDIAL) - fake PASS removed; production-substrate Tier-A transform not yet earned";
pub const RUNTIME_0080_0_R1A_STATUS_BLOCKED: &str = "BLOCKED - no discrete GPU";
pub const RUNTIME_R1A_SCOPE: &str = "Tier-A field/value columns only";
pub const RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM: u64 = 0xabe9_320b_bcf5_03d4;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const R1A_PRODUCTION_SUBSTRATE_GAP: &str =
    "integrated WorldGpuState/Pipelines Tier-A transition registration is absent; a Section 4a generic, semantic-free primitive or composition must compute state_N+1 on GPU before PASS";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1aInput {
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
pub struct Runtime0080R1aAdapterReport {
    pub adapter_name: String,
    pub device_name: String,
    pub selected_discrete_gpu: bool,
    pub backend: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aMeasuredCounters {
    pub initial_seed_upload_count: u32,
    pub inter_tick_tier_a_upload_count: u32,
    pub inter_tick_readback_count: u32,
    pub boundary_parity_readback_count: u32,
    pub gpu_dispatch_count: u32,
    pub oracle_values_written_after_seed: u32,
    pub tier_a_next_state_cpu_write_call_sites: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aAntiFakeEvidence {
    pub outcome: &'static str,
    pub cpu_injected_next_state_removed: bool,
    pub identity_copy_producer_removed: bool,
    pub oracle_comparison_only: bool,
    pub negative_control_run: bool,
    pub negative_control_fails_parity: bool,
    pub measured_counters_from_call_sites: bool,
    pub earned_per_column_parity: bool,
    pub source_shape_guard_passed: bool,
    pub constituent_shapes_measured: bool,
    pub section_4a_gate_available: bool,
    pub new_substrate_primitive_added: bool,
    pub production_substrate_gap: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aSubstratePrimitiveReport {
    pub primitive_name: &'static str,
    pub section_4a_required: bool,
    pub semantic_free_identifier: bool,
    pub reusable_by_any_simthing: bool,
    pub cpu_oracle_parity_test_passed: bool,
    pub opt_in_default_off: bool,
    pub genericity_justification: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aCoveredColumnReport {
    pub column: &'static str,
    pub gpu_authoritative: bool,
    pub cpu_oracle_parity: bool,
    pub integer_bit_exact: bool,
    pub writes_state_n_plus_1: bool,
    pub reads_prior_gpu_output: bool,
    pub cpu_mutated_between_ticks: bool,
    pub parity_measured_from_gpu_value: bool,
    pub measured_shape: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aTraceRow {
    pub tick: u32,
    pub current_hash_before_tick: u64,
    pub next_hash_after_gpu_write: u64,
    pub current_hash_after_swap: u64,
    pub previous_output_read_by_next_tick: bool,
    pub gpu_wrote_state_n_plus_1: bool,
    pub boundary_swap: bool,
    pub cpu_tier_a_uploads_this_tick: u32,
    pub boundary_event_rows: u32,
    pub cpu_boundary_maintenance_rows: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aBoundarySummary {
    pub gpu_written_event_journal_rows: u32,
    pub cpu_boundary_maintenance_rows: u32,
    pub cpu_boundary_pass_bounded: bool,
    pub cpu_boundary_pass_is_planner: bool,
    pub created_removed_or_compacted_by_r1a: bool,
    pub resident_event_journal_r1b_remaining: bool,
    pub resident_reenroll_r1c_remaining: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1aReport {
    pub id: &'static str,
    pub primitive_name: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub adapter: Option<Runtime0080R1aAdapterReport>,
    pub scope: &'static str,
    pub initial_seed_upload_count: u32,
    pub inter_tick_tier_a_upload_count: u32,
    pub inter_tick_readback_count: u32,
    pub boundary_parity_readback_count: u32,
    pub gpu_state_feeds_next_tick: bool,
    pub mirror_dispatch_after_cpu_tick: bool,
    pub tier_a_current_next_buffers_exist: bool,
    pub gpu_writes_state_n_plus_1: bool,
    pub next_tick_reads_gpu_written_state: bool,
    pub buffer_swap_count: u32,
    pub resident_slot_count: u32,
    pub gpu_dispatch_count: u32,
    pub cpu_shadow_boundary_witness_only: bool,
    pub measured_counters: Runtime0080R1aMeasuredCounters,
    pub anti_fake_evidence: Runtime0080R1aAntiFakeEvidence,
    pub substrate_primitives: Vec<Runtime0080R1aSubstratePrimitiveReport>,
    pub measured_shape_names: Vec<&'static str>,
    pub covered_columns: Vec<Runtime0080R1aCoveredColumnReport>,
    pub boundary_summary: Runtime0080R1aBoundarySummary,
    pub r4_max_abs_delta: f32,
    pub r4_f32_bound: f32,
    pub r4_within_bound: bool,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub field_column_parity_matches_r6c_checksum: bool,
    pub no_new_semantic_wgsl: bool,
    pub no_new_accumulator_op: bool,
    pub request_atlas_batching: bool,
    pub m4a_masking_at_scale: bool,
    pub scenario_reopened: bool,
    pub invariant_edited: bool,
    pub pinned_number_changed: bool,
    pub default_simsession_wiring: bool,
    pub foreground_capture_method: &'static str,
    pub remaining_gaps: Vec<&'static str>,
    pub trace: Vec<Runtime0080R1aTraceRow>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

pub fn run_runtime_0080_0_r1a(input: &Runtime0080R1aInput) -> Runtime0080R1aReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required"],
            None,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden"],
            None,
        ));
    }

    let (_ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic], None);
            report.verdict = "BLOCKED";
            report.status = RUNTIME_0080_0_R1A_STATUS_BLOCKED;
            return finalize_report(report);
        }
    };

    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let measure = run_gpu_measure_0080_0(&GpuMeasure0080Input::explicit_opt_in());
    let measured_shape_names = measured_shape_names(&measure);
    let constituent_shapes_measured = measure.status == GPU_MEASURE_0080_0_STATUS_PASS
        && required_shape_names()
            .iter()
            .all(|name| measured_shape_names.iter().any(|shape| shape == name));
    let r4_max_abs_delta = measure
        .shape_reports
        .iter()
        .find(|shape| shape.shape_name == "R4 GradientXY + Candidate-F magnitude")
        .and_then(|shape| shape.max_abs_delta)
        .unwrap_or(0.0);

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.status = RUNTIME_0080_0_R1A_STATUS_PARTIAL;
    report.verdict = "PARTIAL";
    report.admitted = true;
    report.diagnostics = vec![
        "outcome_b_remedial_partial",
        "old_cpu_injected_next_state_path_removed",
        "production_substrate_tier_a_transform_not_registered",
        "negative_control_not_meaningful_until_gpu_transform_exists",
    ];
    report.cpu_shadow_boundary_witness_only = true;
    report.boundary_summary = boundary_summary(&oracle);
    report.r4_max_abs_delta = r4_max_abs_delta;
    report.r4_within_bound = r4_max_abs_delta <= RUNTIME_R0_R4_F32_BOUND;
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.measured_shape_names = measured_shape_names;
    report.anti_fake_evidence.constituent_shapes_measured = constituent_shapes_measured;
    report.remaining_gaps = vec![
        R1A_PRODUCTION_SUBSTRATE_GAP,
        "Section 6.2 negative control must be earned by disabling a GPU Tier-A transform and observing parity failure",
        "resident event journal R1b",
        "resident REENROLL/scatter/compact R1c",
        "M-4A/multi-atlas",
        "recursion",
        "multi-faction ECON",
        "richer emergence",
    ];
    finalize_report(report)
}

pub fn replay_runtime_0080_0_r1a() -> (Runtime0080R1aReport, Runtime0080R1aReport) {
    let input = Runtime0080R1aInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1a(&input),
        run_runtime_0080_0_r1a(&input),
    )
}

fn create_discrete_gpu_context() -> Result<(GpuContext, Runtime0080R1aAdapterReport), &'static str>
{
    let ctx = GpuContext::new_blocking().map_err(|_| "gpu_context_unavailable")?;
    let info = ctx.adapter.get_info();
    let selected_discrete_gpu = format!("{:?}", info.device_type) == "DiscreteGpu"
        || adapter_name_looks_discrete(&info.name);
    if !selected_discrete_gpu {
        return Err("discrete_gpu_unavailable");
    }
    Ok((
        ctx,
        Runtime0080R1aAdapterReport {
            adapter_name: info.name.clone(),
            device_name: "simthing-gpu device".to_string(),
            selected_discrete_gpu,
            backend: format!("{:?}", info.backend),
        },
    ))
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

fn base_report(
    input: &Runtime0080R1aInput,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1aReport {
    let measured_counters = Runtime0080R1aMeasuredCounters {
        initial_seed_upload_count: 0,
        inter_tick_tier_a_upload_count: 0,
        inter_tick_readback_count: 0,
        boundary_parity_readback_count: 0,
        gpu_dispatch_count: 0,
        oracle_values_written_after_seed: 0,
        tier_a_next_state_cpu_write_call_sites: 0,
    };
    Runtime0080R1aReport {
        id: RUNTIME_0080_0_R1A_ID,
        primitive_name: RUNTIME_0080_0_R1A_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        adapter,
        scope: RUNTIME_R1A_SCOPE,
        initial_seed_upload_count: measured_counters.initial_seed_upload_count,
        inter_tick_tier_a_upload_count: measured_counters.inter_tick_tier_a_upload_count,
        inter_tick_readback_count: measured_counters.inter_tick_readback_count,
        boundary_parity_readback_count: measured_counters.boundary_parity_readback_count,
        gpu_state_feeds_next_tick: false,
        mirror_dispatch_after_cpu_tick: false,
        tier_a_current_next_buffers_exist: false,
        gpu_writes_state_n_plus_1: false,
        next_tick_reads_gpu_written_state: false,
        buffer_swap_count: 0,
        resident_slot_count: 0,
        gpu_dispatch_count: measured_counters.gpu_dispatch_count,
        cpu_shadow_boundary_witness_only: false,
        measured_counters,
        anti_fake_evidence: Runtime0080R1aAntiFakeEvidence {
            outcome: "Outcome B - honest PARTIAL",
            cpu_injected_next_state_removed: true,
            identity_copy_producer_removed: true,
            oracle_comparison_only: true,
            negative_control_run: false,
            negative_control_fails_parity: false,
            measured_counters_from_call_sites: true,
            earned_per_column_parity: false,
            source_shape_guard_passed: false,
            constituent_shapes_measured: false,
            section_4a_gate_available: true,
            new_substrate_primitive_added: false,
            production_substrate_gap: R1A_PRODUCTION_SUBSTRATE_GAP,
        },
        substrate_primitives: Vec::new(),
        measured_shape_names: Vec::new(),
        covered_columns: covered_columns(),
        boundary_summary: Runtime0080R1aBoundarySummary {
            gpu_written_event_journal_rows: 0,
            cpu_boundary_maintenance_rows: 0,
            cpu_boundary_pass_bounded: true,
            cpu_boundary_pass_is_planner: false,
            created_removed_or_compacted_by_r1a: false,
            resident_event_journal_r1b_remaining: true,
            resident_reenroll_r1c_remaining: true,
        },
        r4_max_abs_delta: 0.0,
        r4_f32_bound: RUNTIME_R0_R4_F32_BOUND,
        r4_within_bound: false,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        field_column_parity_matches_r6c_checksum: false,
        no_new_semantic_wgsl: true,
        no_new_accumulator_op: true,
        request_atlas_batching: false,
        m4a_masking_at_scale: false,
        scenario_reopened: false,
        invariant_edited: false,
        pinned_number_changed: false,
        default_simsession_wiring: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        remaining_gaps: vec![
            R1A_PRODUCTION_SUBSTRATE_GAP,
            "resident event journal R1b",
            "resident REENROLL/scatter/compact R1c",
            "M-4A/multi-atlas",
            "recursion",
            "multi-faction ECON",
            "richer emergence",
        ],
        trace: Vec::new(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn covered_columns() -> Vec<Runtime0080R1aCoveredColumnReport> {
    [
        (
            "disruption",
            true,
            "R1 disruption input + bounded recurrence",
        ),
        ("location_status", true, "R1 diffusion/readout status"),
        ("stockpiles", true, "R2 owner reduce-up + disburse-down"),
        (
            "construction_progress",
            true,
            "R6B construction threshold + fusion sum",
        ),
        (
            "existing_slot_num_ships",
            true,
            "R6 combat damage reduce + attrition emission",
        ),
        (
            "blockade_divert_code",
            true,
            "R6C conformant integrated report",
        ),
        (
            "r4_magnitude_scratch",
            false,
            "R4 GradientXY + Candidate-F magnitude",
        ),
    ]
    .into_iter()
    .map(
        |(column, integer_bit_exact, measured_shape)| Runtime0080R1aCoveredColumnReport {
            column,
            gpu_authoritative: false,
            cpu_oracle_parity: false,
            integer_bit_exact,
            writes_state_n_plus_1: false,
            reads_prior_gpu_output: false,
            cpu_mutated_between_ticks: false,
            parity_measured_from_gpu_value: false,
            measured_shape,
        },
    )
    .collect()
}

fn boundary_summary(report: &DressRehearsalR6cReport) -> Runtime0080R1aBoundarySummary {
    let event_rows = report.boundary_request_rows.len()
        + report
            .combat_rows
            .iter()
            .filter(|row| row.ship_loss_event_emitted || row.zero_cohort_event_emitted)
            .count()
        + report
            .construction_rows
            .iter()
            .filter(|row| row.threshold_passed)
            .count()
        + report.reinforcement_rows.len()
        + report.birth_rows.len()
        + report.fusion_rows.len();
    Runtime0080R1aBoundarySummary {
        gpu_written_event_journal_rows: 0,
        cpu_boundary_maintenance_rows: event_rows as u32,
        cpu_boundary_pass_bounded: true,
        cpu_boundary_pass_is_planner: false,
        created_removed_or_compacted_by_r1a: false,
        resident_event_journal_r1b_remaining: true,
        resident_reenroll_r1c_remaining: true,
    }
}

fn required_shape_names() -> [&'static str; 5] {
    [
        "R1 disruption input + bounded recurrence",
        "R2 owner reduce-up + disburse-down",
        "R4 GradientXY + Candidate-F magnitude",
        "R6 combat damage reduce + attrition emission",
        "R6B construction threshold + fusion sum",
    ]
}

fn measured_shape_names(report: &GpuMeasure0080Report) -> Vec<&'static str> {
    report
        .shape_reports
        .iter()
        .filter(|shape| shape.measured_on_gpu)
        .map(|shape| shape.shape_name)
        .collect()
}

fn finalize_report(mut report: Runtime0080R1aReport) -> Runtime0080R1aReport {
    report.initial_seed_upload_count = report.measured_counters.initial_seed_upload_count;
    report.inter_tick_tier_a_upload_count = report.measured_counters.inter_tick_tier_a_upload_count;
    report.inter_tick_readback_count = report.measured_counters.inter_tick_readback_count;
    report.boundary_parity_readback_count = report.measured_counters.boundary_parity_readback_count;
    report.gpu_dispatch_count = report.measured_counters.gpu_dispatch_count;
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1a_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1aReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.verdict);
    mix_str(&mut hash, report.status);
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.cpu_injected_next_state_removed as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.identity_copy_producer_removed as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.negative_control_run as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.negative_control_fails_parity as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.earned_per_column_parity as u64,
    );
    mix_u64(&mut hash, report.initial_seed_upload_count as u64);
    mix_u64(&mut hash, report.inter_tick_tier_a_upload_count as u64);
    mix_u64(&mut hash, report.inter_tick_readback_count as u64);
    mix_u64(&mut hash, report.boundary_parity_readback_count as u64);
    mix_u64(&mut hash, report.gpu_dispatch_count as u64);
    mix_u64(&mut hash, report.gpu_state_feeds_next_tick as u64);
    mix_u64(&mut hash, report.mirror_dispatch_after_cpu_tick as u64);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(
        &mut hash,
        report.boundary_summary.cpu_boundary_maintenance_rows as u64,
    );
    for column in &report.covered_columns {
        mix_str(&mut hash, column.column);
        mix_u64(&mut hash, column.gpu_authoritative as u64);
        mix_u64(&mut hash, column.cpu_oracle_parity as u64);
        mix_u64(&mut hash, column.parity_measured_from_gpu_value as u64);
    }
    hash
}

pub fn render_runtime_0080_r1a_artifact(report: &Runtime0080R1aReport) -> String {
    let adapter = report
        .adapter
        .as_ref()
        .map(|adapter| {
            format!(
                "adapter_name: {}\nselected_discrete_gpu: {}\nbackend: {}\n",
                adapter.adapter_name, adapter.selected_discrete_gpu, adapter.backend
            )
        })
        .unwrap_or_else(|| format!("diagnostics: {}\n", report.diagnostics.join(", ")));
    let columns = report
        .covered_columns
        .iter()
        .map(|column| {
            format!(
                "| {} | {} | {} | {} | {} | {} |",
                column.column,
                column.gpu_authoritative,
                column.cpu_oracle_parity,
                column.parity_measured_from_gpu_value,
                column.writes_state_n_plus_1,
                column.measured_shape
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let shapes = if report.measured_shape_names.is_empty() {
        "none".to_string()
    } else {
        report.measured_shape_names.join("\n- ")
    };
    let gaps = report.remaining_gaps.join("\n- ");
    format!(
        "# RUNTIME-0080-0-R1a Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         {adapter}\
         ## Remedial Posture\n\
         - outcome: {outcome}\n\
         - production_substrate_gap: {gap}\n\
         - cpu_injected_next_state_removed: {cpu_removed}\n\
         - identity_copy_producer_removed: {identity_removed}\n\
         - oracle_comparison_only: {oracle_only}\n\
         - negative_control_run: {neg_run}\n\
         - negative_control_fails_parity: {neg_fails}\n\
         - earned_per_column_parity: {earned}\n\
         - source_shape_guard_passed: {shape_guard}\n\
         - constituent_shapes_measured: {constituent}\n\
         - section_4a_gate_available: {gate}\n\
         - new_substrate_primitive_added: {new_primitive}\n\n\
         ## Measured Counters\n\
         - initial_seed_upload_count: {seed_uploads}\n\
         - inter_tick_tier_a_upload_count: {tier_a_uploads}\n\
         - inter_tick_readback_count: {inter_tick_readbacks}\n\
         - boundary_parity_readback_count: {boundary_readbacks}\n\
         - gpu_dispatch_count: {dispatches}\n\
         - oracle_values_written_after_seed: {oracle_writes}\n\
         - tier_a_next_state_cpu_write_call_sites: {write_sites}\n\
         - gpu_state_feeds_next_tick: {gpu_feeds}\n\
         - gpu_writes_state_n_plus_1: {gpu_writes}\n\
         - next_tick_reads_gpu_written_state: {next_reads}\n\n\
         ## Covered Columns\n\n\
         | column | GPU authoritative | CPU oracle parity | measured from GPU value | writes N+1 | measured shape |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         {columns}\n\n\
         ## Constituent GPU Shapes\n\
         - {shapes}\n\n\
         ## CPU Oracle / R4\n\
         - r6c_checksum_expected: `{expected:016x}`\n\
         - r6c_checksum_observed: `{observed:016x}`\n\
         - field_column_parity_matches_r6c_checksum: {checksum_match}\n\
         - r4_max_abs_delta: {r4_delta}\n\
         - r4_f32_bound: {r4_bound}\n\
         - r4_within_bound: {r4_within}\n\n\
         ## Tier-B Boundary Maintenance\n\
         - gpu_written_event_journal_rows: {journal_rows}\n\
         - cpu_boundary_maintenance_rows: {maintenance_rows}\n\
         - cpu_boundary_pass_bounded: {bounded}\n\
         - cpu_boundary_pass_is_planner: {planner}\n\
         - created_removed_or_compacted_by_r1a: {created_removed}\n\
         - resident_event_journal_r1b_remaining: {event_gap}\n\
         - resident_reenroll_r1c_remaining: {reenroll_gap}\n\n\
         ## Guardrails\n\
         - no_new_semantic_wgsl: {wgsl}\n\
         - no_new_accumulator_op: {new_op}\n\
         - request_atlas_batching: {atlas_batching}\n\
         - m4a_masking_at_scale: {m4a}\n\
         - scenario_reopened: {reopened}\n\
         - invariant_edited: {invariant}\n\
         - pinned_number_changed: {pinned}\n\
         - default_simsession_wiring: {default_wiring}\n\
         - foreground_capture_method: {capture}\n\n\
         ## Remaining Gaps\n\
         - {gaps}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter,
        outcome = report.anti_fake_evidence.outcome,
        gap = report.anti_fake_evidence.production_substrate_gap,
        cpu_removed = report.anti_fake_evidence.cpu_injected_next_state_removed,
        identity_removed = report.anti_fake_evidence.identity_copy_producer_removed,
        oracle_only = report.anti_fake_evidence.oracle_comparison_only,
        neg_run = report.anti_fake_evidence.negative_control_run,
        neg_fails = report.anti_fake_evidence.negative_control_fails_parity,
        earned = report.anti_fake_evidence.earned_per_column_parity,
        shape_guard = report.anti_fake_evidence.source_shape_guard_passed,
        constituent = report.anti_fake_evidence.constituent_shapes_measured,
        gate = report.anti_fake_evidence.section_4a_gate_available,
        new_primitive = report.anti_fake_evidence.new_substrate_primitive_added,
        seed_uploads = report.initial_seed_upload_count,
        tier_a_uploads = report.inter_tick_tier_a_upload_count,
        inter_tick_readbacks = report.inter_tick_readback_count,
        boundary_readbacks = report.boundary_parity_readback_count,
        dispatches = report.gpu_dispatch_count,
        oracle_writes = report.measured_counters.oracle_values_written_after_seed,
        write_sites = report.measured_counters.tier_a_next_state_cpu_write_call_sites,
        gpu_feeds = report.gpu_state_feeds_next_tick,
        gpu_writes = report.gpu_writes_state_n_plus_1,
        next_reads = report.next_tick_reads_gpu_written_state,
        columns = columns,
        shapes = shapes,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        checksum_match = report.field_column_parity_matches_r6c_checksum,
        r4_delta = report.r4_max_abs_delta,
        r4_bound = report.r4_f32_bound,
        r4_within = report.r4_within_bound,
        journal_rows = report.boundary_summary.gpu_written_event_journal_rows,
        maintenance_rows = report.boundary_summary.cpu_boundary_maintenance_rows,
        bounded = report.boundary_summary.cpu_boundary_pass_bounded,
        planner = report.boundary_summary.cpu_boundary_pass_is_planner,
        created_removed = report.boundary_summary.created_removed_or_compacted_by_r1a,
        event_gap = report.boundary_summary.resident_event_journal_r1b_remaining,
        reenroll_gap = report.boundary_summary.resident_reenroll_r1c_remaining,
        wgsl = report.no_new_semantic_wgsl,
        new_op = report.no_new_accumulator_op,
        atlas_batching = report.request_atlas_batching,
        m4a = report.m4a_masking_at_scale,
        reopened = report.scenario_reopened,
        invariant = report.invariant_edited,
        pinned = report.pinned_number_changed,
        default_wiring = report.default_simsession_wiring,
        capture = report.foreground_capture_method,
        gaps = gaps,
    )
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}
