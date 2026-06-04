//! MOBILITY-GPU-KERNEL-11: deterministic budget-envelope assertions over KERNEL-10 stream
//! accounting.
//!
//! Driver test/support only. Evaluates integer-only budget envelopes over the accepted KERNEL-10
//! accounting summary without wall-clock timing, default scheduling, gameplay, or semantic WGSL.

#[path = "mobility_gpu_kernel10_stream_accounting_fixture.rs"]
mod mobility_gpu_kernel10_stream_accounting_fixture;

pub use mobility_gpu_kernel10_stream_accounting_fixture::{
    mobility_gpu_kernel10_shader_text_has_domain_terms, projected_34k_columns_for_kernel6,
    projection_checksum_for_columns, run_mobility_gpu_kernel10_fixture,
    stream_cpu_checksum_from_frames, stream_gpu_checksum_from_frames,
    MobilityGpuKernel0ParityClassification, MobilityGpuKernel10FixtureInput,
    MobilityGpuKernel10FixtureReport, MobilityGpuKernel10StreamAccounting,
    MOBILITY_GPU_KERNEL10_ACCOUNTING_ID, MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT,
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS,
    MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT,
    MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED,
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME,
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS, MOBILITY_GPU_KERNEL10_FIXTURE_ID,
    MOBILITY_GPU_KERNEL10_NAMED_GATE, MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED,
    MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

use mobility_gpu_kernel10_stream_accounting_fixture::{
    MobilityGpuKernel10ForbiddenPathRequests, MobilityGpuKernel10Gate,
};

pub const MOBILITY_GPU_KERNEL11_FIXTURE_ID: &str = "mobility_gpu_kernel11_budget_envelope_fixture";
pub const MOBILITY_GPU_KERNEL11_NAMED_GATE: &str =
    "mobility_gpu_kernel11_budget_envelope_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL11_BUDGET_ID: &str =
    "mobility_gpu_kernel11_deterministic_stream_budget_envelope";
pub const MOBILITY_GPU_KERNEL11_NEW_SHADER_TEXT_ADDED: bool = false;
pub const MOBILITY_GPU_KERNEL11_USES_WALL_CLOCK: bool = false;

pub const MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANTS_PER_FRAME: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_ROW_COUNT_PER_VARIANT: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANT_DISPATCH_ATTEMPTS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAY_DISPATCH_ATTEMPTS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_CPU_ORACLE_ROWS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED;
pub const MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_GPU_ROWS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel11Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel11Gate {
    pub fn registration_only() -> Self {
        Self {
            registration_gate_enabled: true,
            dispatch_gate_enabled: false,
            enabled_by_default: false,
        }
    }

    pub fn registration_and_dispatch() -> Self {
        Self {
            registration_gate_enabled: true,
            dispatch_gate_enabled: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel11ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
    pub default_schedule: bool,
    pub default_simsession_path: bool,
    pub gameplay_path: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_or_nondeterministic_atomics: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladder_reopen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel11FixtureInput {
    pub gate: MobilityGpuKernel11Gate,
    pub forbidden: MobilityGpuKernel11ForbiddenPathRequests,
    pub replays_per_variant: usize,
}

impl MobilityGpuKernel11FixtureInput {
    pub fn default_budget_envelope() -> Self {
        Self {
            gate: MobilityGpuKernel11Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel11ForbiddenPathRequests::default(),
            replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel11StreamBudgetEnvelope {
    pub frame_count: usize,
    pub variants_per_frame: usize,
    pub replays_per_variant: usize,
    pub row_count_per_variant: usize,
    pub total_variant_dispatch_attempts: usize,
    pub total_replay_dispatch_attempts: usize,
    pub total_rows_processed: usize,
    pub total_cpu_oracle_rows: usize,
    pub total_gpu_rows_exact: Option<usize>,
    pub zero_cost: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel11BudgetEvaluation {
    pub within_envelope: bool,
    pub diagnostics: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel11FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub budget_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_only_zero_dispatches: bool,
    pub reuses_kernel10_accounting: bool,
    pub reuses_kernel9_frame_stream: bool,
    pub kernel10_fixture_id: &'static str,
    pub kernel10_accounting_id: &'static str,

    pub generic_column_vocabulary_only: bool,
    pub shader_text_has_domain_terms: bool,
    pub new_shader_text_added: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladders_reopened: bool,
    pub uses_wall_clock_or_timing_thresholds: bool,

    pub envelope: MobilityGpuKernel11StreamBudgetEnvelope,
    pub budget_evaluation: MobilityGpuKernel11BudgetEvaluation,
    pub kernel10_report: MobilityGpuKernel10FixtureReport,
    pub preserves_kernel10_checksums: bool,
    pub repeated_runs_identical: bool,
    pub order_sensitive: bool,
    pub evaluation_does_not_mutate_accounting: bool,
}

pub fn active_stream_budget_envelope() -> MobilityGpuKernel11StreamBudgetEnvelope {
    MobilityGpuKernel11StreamBudgetEnvelope {
        frame_count: MOBILITY_GPU_KERNEL11_ENVELOPE_FRAME_COUNT,
        variants_per_frame: MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANTS_PER_FRAME,
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
        row_count_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_ROW_COUNT_PER_VARIANT,
        total_variant_dispatch_attempts: MOBILITY_GPU_KERNEL11_ENVELOPE_VARIANT_DISPATCH_ATTEMPTS,
        total_replay_dispatch_attempts: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAY_DISPATCH_ATTEMPTS,
        total_rows_processed: MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_ROWS_PROCESSED,
        total_cpu_oracle_rows: MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_CPU_ORACLE_ROWS,
        total_gpu_rows_exact: Some(MOBILITY_GPU_KERNEL11_ENVELOPE_TOTAL_GPU_ROWS),
        zero_cost: false,
    }
}

pub fn zero_cost_budget_envelope() -> MobilityGpuKernel11StreamBudgetEnvelope {
    MobilityGpuKernel11StreamBudgetEnvelope {
        frame_count: 0,
        variants_per_frame: 0,
        replays_per_variant: 0,
        row_count_per_variant: 0,
        total_variant_dispatch_attempts: 0,
        total_replay_dispatch_attempts: 0,
        total_rows_processed: 0,
        total_cpu_oracle_rows: 0,
        total_gpu_rows_exact: None,
        zero_cost: true,
    }
}

pub fn evaluate_stream_budget_envelope(
    accounting: &MobilityGpuKernel10StreamAccounting,
    envelope: &MobilityGpuKernel11StreamBudgetEnvelope,
) -> MobilityGpuKernel11BudgetEvaluation {
    let mut diagnostics = Vec::new();

    if envelope.zero_cost {
        if accounting.frame_count != 0 {
            diagnostics.push("kernel11_budget_zero_cost_frame_count_nonzero");
        }
        if accounting.total_variant_dispatch_attempts != 0 {
            diagnostics.push("kernel11_budget_zero_cost_variant_dispatch_nonzero");
        }
        if accounting.total_replay_dispatch_attempts != 0 {
            diagnostics.push("kernel11_budget_zero_cost_replay_dispatch_nonzero");
        }
        if accounting.total_rows_processed != 0 {
            diagnostics.push("kernel11_budget_zero_cost_rows_nonzero");
        }
        if accounting.total_cpu_oracle_rows != 0 {
            diagnostics.push("kernel11_budget_zero_cost_cpu_oracle_rows_nonzero");
        }
        if accounting.total_gpu_rows.is_some() {
            diagnostics.push("kernel11_budget_zero_cost_gpu_rows_present");
        }
        return MobilityGpuKernel11BudgetEvaluation {
            within_envelope: diagnostics.is_empty(),
            diagnostics,
        };
    }

    if accounting.frame_count != envelope.frame_count {
        diagnostics.push("kernel11_budget_frame_count_over_envelope");
    }
    if accounting.variants_per_frame != envelope.variants_per_frame {
        diagnostics.push("kernel11_budget_variants_per_frame_over_envelope");
    }
    if accounting.replays_per_variant != envelope.replays_per_variant {
        diagnostics.push("kernel11_budget_replays_per_variant_over_envelope");
    }
    if accounting.row_count_per_variant != envelope.row_count_per_variant {
        diagnostics.push("kernel11_budget_row_count_per_variant_over_envelope");
    }
    if accounting.total_variant_dispatch_attempts > envelope.total_variant_dispatch_attempts {
        diagnostics.push("kernel11_budget_variant_dispatch_over_envelope");
    } else if accounting.total_variant_dispatch_attempts != envelope.total_variant_dispatch_attempts
    {
        diagnostics.push("kernel11_budget_variant_dispatch_under_envelope");
    }
    if accounting.total_replay_dispatch_attempts > envelope.total_replay_dispatch_attempts {
        diagnostics.push("kernel11_budget_replay_dispatch_over_envelope");
    } else if accounting.total_replay_dispatch_attempts != envelope.total_replay_dispatch_attempts {
        diagnostics.push("kernel11_budget_replay_dispatch_under_envelope");
    }
    if accounting.total_rows_processed > envelope.total_rows_processed {
        diagnostics.push("kernel11_budget_rows_over_envelope");
    } else if accounting.total_rows_processed != envelope.total_rows_processed {
        diagnostics.push("kernel11_budget_rows_under_envelope");
    }
    if accounting.total_cpu_oracle_rows > envelope.total_cpu_oracle_rows {
        diagnostics.push("kernel11_budget_cpu_oracle_rows_over_envelope");
    } else if accounting.total_cpu_oracle_rows != envelope.total_cpu_oracle_rows {
        diagnostics.push("kernel11_budget_cpu_oracle_rows_under_envelope");
    }

    match (
        accounting.parity_classification,
        accounting.total_gpu_rows,
        envelope.total_gpu_rows_exact,
    ) {
        (MobilityGpuKernel0ParityClassification::ExactParity, Some(gpu_rows), Some(expected)) => {
            if gpu_rows > expected {
                diagnostics.push("kernel11_budget_gpu_rows_over_envelope");
            } else if gpu_rows != expected {
                diagnostics.push("kernel11_budget_gpu_rows_under_envelope");
            }
        }
        (MobilityGpuKernel0ParityClassification::GpuUnavailable, None, Some(_)) => {}
        (MobilityGpuKernel0ParityClassification::GpuUnavailable, None, None) => {}
        (_, Some(_), None) => diagnostics.push("kernel11_budget_gpu_rows_present_in_zero_envelope"),
        (_, None, Some(_)) => diagnostics.push("kernel11_budget_gpu_rows_missing_for_exact_parity"),
        (MobilityGpuKernel0ParityClassification::GpuExecutionFailed, _, _) => {
            diagnostics.push("kernel11_budget_gpu_execution_failed");
        }
        _ => diagnostics.push("kernel11_budget_gpu_rows_parity_mismatch"),
    }

    MobilityGpuKernel11BudgetEvaluation {
        within_envelope: diagnostics.is_empty(),
        diagnostics,
    }
}

pub fn fake_over_budget_rows_accounting(
    base: &MobilityGpuKernel10StreamAccounting,
) -> MobilityGpuKernel10StreamAccounting {
    let mut fake = base.clone();
    fake.total_rows_processed = base.total_rows_processed.saturating_add(1);
    fake.total_cpu_oracle_rows = fake.total_rows_processed;
    if fake.total_gpu_rows.is_some() {
        fake.total_gpu_rows = Some(fake.total_rows_processed);
    }
    fake
}

pub fn fake_over_budget_dispatches_accounting(
    base: &MobilityGpuKernel10StreamAccounting,
) -> MobilityGpuKernel10StreamAccounting {
    let mut fake = base.clone();
    fake.total_replay_dispatch_attempts = base.total_replay_dispatch_attempts.saturating_add(1);
    fake.total_variant_dispatch_attempts = base.total_variant_dispatch_attempts.saturating_add(1);
    fake
}

pub fn run_mobility_gpu_kernel11_fixture(
    input: &MobilityGpuKernel11FixtureInput,
) -> MobilityGpuKernel11FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel11_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    let kernel10_input = kernel10_input(input);
    let kernel10_report = run_mobility_gpu_kernel10_fixture(&kernel10_input);

    if !kernel10_report.admitted {
        return rejected_report(input, kernel10_report.diagnostics);
    }

    let accounting_before = kernel10_report.accounting.clone();
    let envelope =
        if kernel10_report.disabled_no_op || kernel10_report.registration_only_zero_dispatches {
            zero_cost_budget_envelope()
        } else {
            active_stream_budget_envelope()
        };
    let budget_evaluation = evaluate_stream_budget_envelope(&accounting_before, &envelope);
    let evaluation_does_not_mutate_accounting = accounting_before == kernel10_report.accounting;

    let preserves_kernel10_checksums = kernel10_report.stream_checksum_matches_kernel9
        && kernel10_report.accounting.aggregate_cpu_stream_checksum
            == stream_cpu_checksum_from_frames(&kernel10_report.kernel9_report.frames)
        && kernel10_report.accounting.aggregate_gpu_stream_checksum
            == stream_gpu_checksum_from_frames(&kernel10_report.kernel9_report.frames);

    let first_eval = budget_evaluation.clone();
    let second_eval = evaluate_stream_budget_envelope(&kernel10_report.accounting, &envelope);
    let repeated_runs_identical = first_eval == second_eval;

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = input.gate.dispatch_gate_enabled;
    report.disabled_no_op = kernel10_report.disabled_no_op;
    report.registration_only_zero_dispatches = kernel10_report.registration_only_zero_dispatches;
    report.uses_registered_node = kernel10_report.uses_registered_node;
    report.reuses_kernel10_accounting = kernel10_report.admitted;
    report.reuses_kernel9_frame_stream = kernel10_report.reuses_kernel9_frame_stream;
    report.envelope = envelope;
    report.budget_evaluation = budget_evaluation;
    report.kernel10_report = kernel10_report;
    report.preserves_kernel10_checksums = preserves_kernel10_checksums;
    report.repeated_runs_identical = repeated_runs_identical;
    report.order_sensitive = report.kernel10_report.order_sensitive;
    report.evaluation_does_not_mutate_accounting = evaluation_does_not_mutate_accounting;
    report
}

pub fn mobility_gpu_kernel11_shader_text_has_domain_terms() -> bool {
    mobility_gpu_kernel10_shader_text_has_domain_terms()
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel11ForbiddenPathRequests,
) -> Option<Vec<&'static str>> {
    let mut diagnostics = Vec::new();
    if forbidden.default_on_behavior {
        diagnostics.push("default_on_behavior");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.designer_authored_shader_input {
        diagnostics.push("designer_authored_shader_input");
    }
    if forbidden.default_schedule {
        diagnostics.push("default_schedule");
    }
    if forbidden.default_simsession_path {
        diagnostics.push("default_simsession_path");
    }
    if forbidden.gameplay_path {
        diagnostics.push("gameplay_path");
    }
    if forbidden.live_slot_compaction {
        diagnostics.push("live_slot_compaction");
    }
    if forbidden.gpu_allocator_or_nondeterministic_atomics {
        diagnostics.push("gpu_allocator_or_nondeterministic_atomics");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.hybrid_strata_or_faction_index_scaling {
        diagnostics.push("hybrid_strata_or_faction_index_scaling");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn kernel10_input(input: &MobilityGpuKernel11FixtureInput) -> MobilityGpuKernel10FixtureInput {
    MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel10ForbiddenPathRequests {
            semantic_or_raw_wgsl: input.forbidden.semantic_or_raw_wgsl,
            designer_authored_shader_input: input.forbidden.designer_authored_shader_input,
            default_on_behavior: input.forbidden.default_on_behavior,
            default_schedule: input.forbidden.default_schedule,
            default_simsession_path: input.forbidden.default_simsession_path,
            gameplay_path: input.forbidden.gameplay_path,
            live_slot_compaction: input.forbidden.live_slot_compaction,
            gpu_allocator_or_nondeterministic_atomics: input
                .forbidden
                .gpu_allocator_or_nondeterministic_atomics,
            cpu_planner_urgency_commitment: input.forbidden.cpu_planner_urgency_commitment,
            hybrid_strata_or_faction_index_scaling: input
                .forbidden
                .hybrid_strata_or_faction_index_scaling,
            closed_ladder_reopen: input.forbidden.closed_ladder_reopen,
        },
        replays_per_variant: input.replays_per_variant,
    }
}

fn shell(input: &MobilityGpuKernel11FixtureInput) -> MobilityGpuKernel11FixtureReport {
    let kernel10_report = run_mobility_gpu_kernel10_fixture(&MobilityGpuKernel10FixtureInput {
        gate: MobilityGpuKernel10Gate::default(),
        forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
        replays_per_variant: MOBILITY_GPU_KERNEL11_ENVELOPE_REPLAYS_PER_VARIANT,
    });
    MobilityGpuKernel11FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL11_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL11_NAMED_GATE,
        budget_id: MOBILITY_GPU_KERNEL11_BUDGET_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_only_zero_dispatches: false,
        reuses_kernel10_accounting: false,
        reuses_kernel9_frame_stream: false,
        kernel10_fixture_id: MOBILITY_GPU_KERNEL10_FIXTURE_ID,
        kernel10_accounting_id: MOBILITY_GPU_KERNEL10_ACCOUNTING_ID,
        generic_column_vocabulary_only: true,
        shader_text_has_domain_terms: mobility_gpu_kernel11_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL11_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        uses_wall_clock_or_timing_thresholds: MOBILITY_GPU_KERNEL11_USES_WALL_CLOCK,
        envelope: zero_cost_budget_envelope(),
        budget_evaluation: MobilityGpuKernel11BudgetEvaluation {
            within_envelope: false,
            diagnostics: Vec::new(),
        },
        kernel10_report,
        preserves_kernel10_checksums: false,
        repeated_runs_identical: false,
        order_sensitive: false,
        evaluation_does_not_mutate_accounting: true,
    }
}

fn rejected_report(
    input: &MobilityGpuKernel11FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel11FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}
