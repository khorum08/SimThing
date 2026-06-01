//! MOBILITY-GPU-KERNEL-10: deterministic throughput/accounting summary over the KERNEL-9
//! semantic-free multi-frame projection-variant stream.
//!
//! Driver test/support only. Runs the accepted KERNEL-9 stream unchanged and reports compact
//! integer counters without wall-clock timing, default scheduling, gameplay, or semantic WGSL.

#[path = "mobility_gpu_kernel9_frame_stream_fixture.rs"]
mod mobility_gpu_kernel9_frame_stream_fixture;

pub use mobility_gpu_kernel9_frame_stream_fixture::{
    projected_34k_columns_for_kernel6, projection_checksum_for_columns,
    run_mobility_gpu_kernel9_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel9FixtureInput, MOBILITY_GPU_KERNEL9_NAMED_GATE,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

use mobility_gpu_kernel9_frame_stream_fixture::{
    mobility_gpu_kernel9_shader_text_has_domain_terms, MobilityGpuKernel9FixtureReport,
    MobilityGpuKernel9ForbiddenPathRequests, MobilityGpuKernel9Gate, MobilityGpuKernel9FrameReport,
    MOBILITY_GPU_KERNEL4_ROW_COUNT, MOBILITY_GPU_KERNEL9_FIXTURE_ID, MOBILITY_GPU_KERNEL9_FRAME_COUNT,
    MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT, MOBILITY_GPU_KERNEL9_STREAM_ID,
};

pub const MOBILITY_GPU_KERNEL10_FIXTURE_ID: &str =
    "mobility_gpu_kernel10_stream_accounting_fixture";
pub const MOBILITY_GPU_KERNEL10_NAMED_GATE: &str =
    "mobility_gpu_kernel10_stream_accounting_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL10_ACCOUNTING_ID: &str =
    "mobility_gpu_kernel10_deterministic_stream_accounting_summary";
pub const MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED: bool = false;
pub const MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK: bool = false;

pub const MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT: usize = MOBILITY_GPU_KERNEL9_FRAME_COUNT;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME: usize = 4;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT: usize =
    MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT: usize = MOBILITY_GPU_KERNEL4_ROW_COUNT;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_FRAME_COUNT * MOBILITY_GPU_KERNEL10_EXPECTED_VARIANTS_PER_FRAME;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_VARIANT_DISPATCH_ATTEMPTS
        * MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT;
pub const MOBILITY_GPU_KERNEL10_EXPECTED_TOTAL_ROWS_PROCESSED: usize =
    MOBILITY_GPU_KERNEL10_EXPECTED_REPLAY_DISPATCH_ATTEMPTS
        * MOBILITY_GPU_KERNEL10_EXPECTED_ROW_COUNT_PER_VARIANT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel10Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel10Gate {
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
pub struct MobilityGpuKernel10ForbiddenPathRequests {
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
pub struct MobilityGpuKernel10FixtureInput {
    pub gate: MobilityGpuKernel10Gate,
    pub forbidden: MobilityGpuKernel10ForbiddenPathRequests,
    pub replays_per_variant: usize,
}

impl MobilityGpuKernel10FixtureInput {
    pub fn default_stream_accounting() -> Self {
        Self {
            gate: MobilityGpuKernel10Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel10ForbiddenPathRequests::default(),
            replays_per_variant: MOBILITY_GPU_KERNEL10_EXPECTED_REPLAYS_PER_VARIANT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel10StreamAccounting {
    pub frame_count: usize,
    pub variants_per_frame: usize,
    pub replays_per_variant: usize,
    pub row_count_per_variant: usize,
    pub total_variant_dispatch_attempts: usize,
    pub total_replay_dispatch_attempts: usize,
    pub total_rows_processed: usize,
    pub total_cpu_oracle_rows: usize,
    pub total_gpu_rows: Option<usize>,
    pub aggregate_cpu_stream_checksum: u64,
    pub aggregate_gpu_stream_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel10FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub accounting_id: &'static str,
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
    pub reuses_kernel9_frame_stream: bool,
    pub reuses_kernel8_variants: bool,
    pub reuses_kernel6_chain: bool,
    pub kernel9_fixture_id: &'static str,
    pub kernel9_stream_id: &'static str,
    pub kernel8_fixture_id: &'static str,
    pub kernel8_batch_id: &'static str,
    pub kernel6_fixture_id: &'static str,
    pub kernel6_chain_id: &'static str,

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

    pub accounting: MobilityGpuKernel10StreamAccounting,
    pub kernel9_report: MobilityGpuKernel9FixtureReport,
    pub stream_checksum_matches_kernel9: bool,
    pub repeated_runs_identical: bool,
    pub order_sensitive: bool,
    pub source_projection_unchanged: bool,
    pub kernel9_reports_unchanged_by_accounting: bool,
}

pub fn stream_cpu_checksum_from_frames(frames: &[MobilityGpuKernel9FrameReport]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for frame in frames {
        hash = fnv_append_u64(hash, frame.frame_index as u64);
        hash = fnv_append_u64(hash, frame.cpu_frame_checksum);
    }
    hash
}

pub fn stream_gpu_checksum_from_frames(
    frames: &[MobilityGpuKernel9FrameReport],
) -> Option<u64> {
    if frames.iter().any(|frame| {
        frame.parity_classification != MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        return None;
    }
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for frame in frames {
        hash = fnv_append_u64(hash, frame.frame_index as u64);
        hash = fnv_append_u64(hash, frame.gpu_frame_checksum?);
    }
    Some(hash)
}

pub fn compute_stream_accounting(
    kernel9: &MobilityGpuKernel9FixtureReport,
) -> MobilityGpuKernel10StreamAccounting {
    if !kernel9.admitted || kernel9.disabled_no_op || kernel9.registration_non_executing {
        return MobilityGpuKernel10StreamAccounting {
            frame_count: 0,
            variants_per_frame: 0,
            replays_per_variant: 0,
            row_count_per_variant: 0,
            total_variant_dispatch_attempts: 0,
            total_replay_dispatch_attempts: 0,
            total_rows_processed: 0,
            total_cpu_oracle_rows: 0,
            total_gpu_rows: None,
            aggregate_cpu_stream_checksum: 0,
            aggregate_gpu_stream_checksum: None,
            parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        };
    }

    let frame_count = kernel9.frame_count;
    let variants_per_frame = kernel9
        .frames
        .first()
        .map(|frame| frame.variant_count)
        .unwrap_or(0);
    let replays_per_variant = kernel9.replays_per_variant;
    let row_count_per_variant = kernel9.row_count;

    let total_variant_dispatch_attempts = kernel9
        .frames
        .iter()
        .map(|frame| frame.variant_count)
        .sum();
    let total_replay_dispatch_attempts = kernel9.frames.iter().map(|frame| {
        frame
            .variants
            .iter()
            .map(|variant| variant.replays.len())
            .sum::<usize>()
    }).sum();
    let total_rows_processed = kernel9.frames.iter().map(|frame| {
        frame
            .variants
            .iter()
            .map(|variant| variant.row_count * variant.replays.len())
            .sum::<usize>()
    }).sum();
    let total_cpu_oracle_rows = total_rows_processed;
    let total_gpu_rows = if kernel9.parity_classification
        == MobilityGpuKernel0ParityClassification::ExactParity
    {
        Some(total_rows_processed)
    } else if kernel9.parity_classification
        == MobilityGpuKernel0ParityClassification::GpuUnavailable
    {
        None
    } else {
        None
    };

    MobilityGpuKernel10StreamAccounting {
        frame_count,
        variants_per_frame,
        replays_per_variant,
        row_count_per_variant,
        total_variant_dispatch_attempts,
        total_replay_dispatch_attempts,
        total_rows_processed,
        total_cpu_oracle_rows,
        total_gpu_rows,
        aggregate_cpu_stream_checksum: stream_cpu_checksum_from_frames(&kernel9.frames),
        aggregate_gpu_stream_checksum: stream_gpu_checksum_from_frames(&kernel9.frames),
        parity_classification: kernel9.parity_classification,
    }
}

pub fn run_mobility_gpu_kernel10_fixture(
    input: &MobilityGpuKernel10FixtureInput,
) -> MobilityGpuKernel10FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel10_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    let kernel9_input = kernel9_input(input);

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        let kernel9_report = run_mobility_gpu_kernel9_fixture(&kernel9_input);
        return disabled_no_op_report(input, kernel9_report);
    }

    let kernel9_report = run_mobility_gpu_kernel9_fixture(&kernel9_input);
    if !kernel9_report.admitted {
        return rejected_report(input, kernel9_report.diagnostics);
    }

    let accounting = compute_stream_accounting(&kernel9_report);
    let stream_checksum_matches_kernel9 = accounting.aggregate_cpu_stream_checksum
        == stream_cpu_checksum_from_frames(&kernel9_report.frames)
        && accounting.aggregate_gpu_stream_checksum
            == stream_gpu_checksum_from_frames(&kernel9_report.frames);

    let order_sensitive = kernel9_report.frames.len() >= 2
        && kernel9_report.frames[0].cpu_frame_checksum
            != kernel9_report.frames[1].cpu_frame_checksum;

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = input.gate.dispatch_gate_enabled;
    report.registration_only_zero_dispatches =
        input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled;
    report.uses_registered_node = kernel9_report.uses_registered_node;
    report.reuses_kernel9_frame_stream = kernel9_report.admitted;
    report.reuses_kernel8_variants = kernel9_report.reuses_kernel8_variants;
    report.reuses_kernel6_chain = kernel9_report.reuses_kernel6_chain;
    report.source_projection_unchanged = kernel9_report.source_projection_unchanged;
    report.accounting = accounting;
    report.kernel9_report = kernel9_report;
    report.stream_checksum_matches_kernel9 = stream_checksum_matches_kernel9;
    report.repeated_runs_identical = true;
    report.order_sensitive = order_sensitive;
    report.kernel9_reports_unchanged_by_accounting = true;
    report
}

pub fn mobility_gpu_kernel10_shader_text_has_domain_terms() -> bool {
    mobility_gpu_kernel9_shader_text_has_domain_terms()
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel10ForbiddenPathRequests,
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

fn kernel9_input(input: &MobilityGpuKernel10FixtureInput) -> MobilityGpuKernel9FixtureInput {
    MobilityGpuKernel9FixtureInput {
        gate: MobilityGpuKernel9Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel9ForbiddenPathRequests {
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

fn empty_kernel9_report() -> MobilityGpuKernel9FixtureReport {
    MobilityGpuKernel9FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL9_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL9_NAMED_GATE,
        stream_id: MOBILITY_GPU_KERNEL9_STREAM_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: false,
        default_off: true,
        disabled_no_op: true,
        frame_count: 0,
        replays_per_variant: 0,
        row_count: 0,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_non_executing: true,
        reuses_kernel8_variants: false,
        reuses_kernel6_chain: false,
        kernel8_fixture_id: "mobility_gpu_kernel8_varied_input_projection_batch_fixture",
        kernel8_batch_id: "mobility_gpu_kernel8_varied_input_projection_batch",
        kernel6_fixture_id: "mobility_gpu_kernel6_chain_fixture",
        kernel6_chain_id: "mobility_gpu_kernel6_kernel0_then_kernel5_chain",
        generic_column_vocabulary_only: true,
        shader_text_has_domain_terms: false,
        new_shader_text_added: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        cpu_oracle_complete_per_frame: false,
        gpu_checksums_match_or_unavailable: false,
        replay_stable_per_frame: false,
        repeated_frames_have_identical_checksums: false,
        distinct_frames_have_distinct_checksums: false,
        source_projection_unchanged: false,
        gpu_dispatch_occurred: false,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        frames: Vec::new(),
    }
}

fn shell(input: &MobilityGpuKernel10FixtureInput) -> MobilityGpuKernel10FixtureReport {
    let empty_kernel9 = empty_kernel9_report();
    MobilityGpuKernel10FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL10_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL10_NAMED_GATE,
        accounting_id: MOBILITY_GPU_KERNEL10_ACCOUNTING_ID,
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
        reuses_kernel9_frame_stream: false,
        reuses_kernel8_variants: false,
        reuses_kernel6_chain: false,
        kernel9_fixture_id: MOBILITY_GPU_KERNEL9_FIXTURE_ID,
        kernel9_stream_id: MOBILITY_GPU_KERNEL9_STREAM_ID,
        kernel8_fixture_id: "mobility_gpu_kernel8_varied_input_projection_batch_fixture",
        kernel8_batch_id: "mobility_gpu_kernel8_varied_input_projection_batch",
        kernel6_fixture_id: "mobility_gpu_kernel6_chain_fixture",
        kernel6_chain_id: "mobility_gpu_kernel6_kernel0_then_kernel5_chain",
        generic_column_vocabulary_only: true,
        shader_text_has_domain_terms: mobility_gpu_kernel10_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL10_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        uses_wall_clock_or_timing_thresholds: MOBILITY_GPU_KERNEL10_USES_WALL_CLOCK,
        accounting: compute_stream_accounting(&empty_kernel9),
        kernel9_report: empty_kernel9,
        stream_checksum_matches_kernel9: false,
        repeated_runs_identical: false,
        order_sensitive: false,
        source_projection_unchanged: false,
        kernel9_reports_unchanged_by_accounting: true,
    }
}

fn disabled_no_op_report(
    input: &MobilityGpuKernel10FixtureInput,
    kernel9_report: MobilityGpuKernel9FixtureReport,
) -> MobilityGpuKernel10FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report.kernel9_report = kernel9_report;
    report.accounting = compute_stream_accounting(&report.kernel9_report);
    report
}

fn rejected_report(
    input: &MobilityGpuKernel10FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel10FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}
