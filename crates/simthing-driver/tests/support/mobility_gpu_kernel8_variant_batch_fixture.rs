//! MOBILITY-GPU-KERNEL-8: varied-input projection-batch replay soak over the semantic-free
//! KERNEL-6 chain.
//!
//! Driver test/support only. Exercises multiple deterministic generic-column projection
//! variants through the ordered KERNEL-0 -> KERNEL-5 chain without adding shader text,
//! default scheduling, gameplay, designer-authored input, or semantic WGSL.

#[path = "mobility_gpu_kernel6_chain_fixture.rs"]
mod mobility_gpu_kernel6_chain_fixture;

use mobility_gpu_kernel6_chain_fixture::{
    cpu_chain_checksum_for_columns, run_mobility_gpu_kernel6_fixture,
    MobilityGpuKernel6FixtureInput, MobilityGpuKernel6ForbiddenPathRequests, MobilityGpuKernel6Gate,
    MOBILITY_GPU_KERNEL6_CHAIN_ID, MOBILITY_GPU_KERNEL6_FIXTURE_ID,
};

pub use mobility_gpu_kernel6_chain_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel6_chain_shader_text_has_domain_terms,
    projection_checksum_for_columns, projected_34k_columns_for_kernel6,
    MobilityGpuKernel0ColumnProbe, MobilityGpuKernel0ParityClassification,
    MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END, MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START,
    MOBILITY_GPU_KERNEL4_ROW_COUNT, MOBILITY_GPU_KERNEL4_SPARSE_STRIDE,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL8_FIXTURE_ID: &str =
    "mobility_gpu_kernel8_varied_input_projection_batch_fixture";
pub const MOBILITY_GPU_KERNEL8_NAMED_GATE: &str =
    "mobility_gpu_kernel8_variant_batch_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL8_BATCH_ID: &str =
    "mobility_gpu_kernel8_projection_variant_batch_replay_soak";
pub const MOBILITY_GPU_KERNEL8_VARIANT_COUNT: usize = 4;
pub const MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT: usize = 2;
pub const MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED: bool = false;

pub const MOBILITY_GPU_KERNEL8_VARIANT_BASELINE: &str = "baseline_34k_projection";
pub const MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA: &str = "sparse_delta_move_mask";
pub const MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK: &str = "dense_bulk_move_mask";
pub const MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET: &str = "parent_key_offset";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel8Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel8Gate {
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
pub struct MobilityGpuKernel8ForbiddenPathRequests {
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

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel8FixtureInput {
    pub gate: MobilityGpuKernel8Gate,
    pub forbidden: MobilityGpuKernel8ForbiddenPathRequests,
    pub replays_per_variant: usize,
}

impl MobilityGpuKernel8FixtureInput {
    pub fn default_variant_batch() -> Self {
        Self {
            gate: MobilityGpuKernel8Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel8ForbiddenPathRequests::default(),
            replays_per_variant: MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel8ReplayReport {
    pub replay: usize,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub gpu_dispatch_occurred: bool,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel8VariantReport {
    pub variant_id: &'static str,
    pub row_count: usize,
    pub projection_checksum: u64,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub replays: Vec<MobilityGpuKernel8ReplayReport>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel8FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub batch_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub variant_count: usize,
    pub replays_per_variant: usize,
    pub row_count: usize,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_non_executing: bool,
    pub reuses_kernel6_chain: bool,
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

    pub cpu_oracle_complete_per_variant: bool,
    pub gpu_checksums_match_or_unavailable: bool,
    pub replay_stable_per_variant: bool,
    pub distinct_variants_have_distinct_checksums: bool,
    pub source_projection_unchanged: bool,
    pub gpu_dispatch_occurred: bool,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub variants: Vec<MobilityGpuKernel8VariantReport>,
}

pub fn build_projection_variants(
    baseline: &MobilityGpuKernel0ColumnProbe,
) -> Vec<(&'static str, MobilityGpuKernel0ColumnProbe)> {
    vec![
        (
            MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
            baseline.clone(),
        ),
        (
            MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
            sparse_delta_variant(baseline),
        ),
        (
            MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
            dense_bulk_variant(baseline),
        ),
        (
            MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
            parent_key_offset_variant(baseline),
        ),
    ]
}

pub fn sparse_delta_variant(base: &MobilityGpuKernel0ColumnProbe) -> MobilityGpuKernel0ColumnProbe {
    let mut columns = base.clone();
    for i in (0..columns.move_mask.len()).step_by(MOBILITY_GPU_KERNEL4_SPARSE_STRIDE) {
        columns.move_mask[i] ^= 1;
        if columns.move_mask[i] != 0 {
            columns.dst_parent[i] = columns.dst_parent[i].wrapping_add(1);
        }
    }
    columns
}

pub fn dense_bulk_variant(base: &MobilityGpuKernel0ColumnProbe) -> MobilityGpuKernel0ColumnProbe {
    let mut columns = base.clone();
    for i in MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START..MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END {
        columns.move_mask[i] = 1;
        columns.dst_parent[i] = columns.dst_parent[i].wrapping_add(2);
    }
    columns
}

pub fn parent_key_offset_variant(base: &MobilityGpuKernel0ColumnProbe) -> MobilityGpuKernel0ColumnProbe {
    let mut columns = base.clone();
    for i in 0..columns.src_parent.len() {
        columns.src_parent[i] = columns.src_parent[i].wrapping_add(3);
        columns.dst_parent[i] = columns.dst_parent[i].wrapping_add(7);
    }
    columns
}

pub fn run_mobility_gpu_kernel8_fixture(
    input: &MobilityGpuKernel8FixtureInput,
) -> MobilityGpuKernel8FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel8_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    if !input.gate.dispatch_gate_enabled {
        let kernel6 = run_mobility_gpu_kernel6_fixture(&kernel6_input(input, None));
        let mut report = shell(input);
        report.admitted = kernel6.admitted;
        report.diagnostics = kernel6.diagnostics;
        report.uses_registered_node = kernel6.uses_registered_node;
        report.registration_non_executing = true;
        report.reuses_kernel6_chain = kernel6.admitted;
        return report;
    }

    let source_before = projected_34k_columns_for_kernel6();
    let source_checksum = projection_checksum_for_columns(&source_before);
    let variants = build_projection_variants(&source_before);
    let replay_count = input
        .replays_per_variant
        .max(MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT);
    let mut variant_reports = Vec::with_capacity(variants.len());
    let mut diagnostics = Vec::new();
    let mut uses_registered_node = false;
    let mut any_gpu_dispatch = false;

    for (variant_id, columns) in variants {
        let projection_checksum = projection_checksum_for_columns(&columns);
        let expected_cpu_chain_checksum = cpu_chain_checksum_for_columns(&columns);
        let mut replays = Vec::with_capacity(replay_count);

        for replay in 0..replay_count {
            let kernel6 = run_mobility_gpu_kernel6_fixture(&kernel6_input(
                input,
                Some(columns.clone()),
            ));
            if !kernel6.admitted {
                diagnostics.extend(kernel6.diagnostics);
                break;
            }
            uses_registered_node |= kernel6.uses_registered_node;
            any_gpu_dispatch |= kernel6.gpu_dispatch_occurred;
            replays.push(MobilityGpuKernel8ReplayReport {
                replay,
                cpu_chain_checksum: kernel6.cpu_chain_checksum,
                gpu_chain_checksum: kernel6.gpu_chain_checksum,
                gpu_dispatch_occurred: kernel6.gpu_dispatch_occurred,
                parity_classification: kernel6.parity_classification,
            });
        }

        if !diagnostics.is_empty() {
            break;
        }

        let gpu_chain_checksum = replays.first().and_then(|report| report.gpu_chain_checksum);
        let parity_classification = classify_variant_parity(&replays);
        variant_reports.push(MobilityGpuKernel8VariantReport {
            variant_id,
            row_count: columns.entity_id.len(),
            projection_checksum,
            cpu_chain_checksum: expected_cpu_chain_checksum,
            gpu_chain_checksum,
            parity_classification,
            replays,
        });
    }

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    let source_after = projected_34k_columns_for_kernel6();
    let cpu_oracle_complete_per_variant = variant_reports.iter().all(|variant| {
        variant.row_count == MOBILITY_GPU_KERNEL4_ROW_COUNT
            && variant.cpu_chain_checksum != 0
            && variant
                .replays
                .iter()
                .all(|replay| replay.cpu_chain_checksum == variant.cpu_chain_checksum)
    });
    let gpu_checksums_match_or_unavailable = variant_reports.iter().all(|variant| {
        variant.replays.iter().all(|replay| {
            match replay.parity_classification {
                MobilityGpuKernel0ParityClassification::ExactParity => {
                    replay.gpu_chain_checksum == variant.gpu_chain_checksum
                        && replay.gpu_chain_checksum.is_some()
                }
                MobilityGpuKernel0ParityClassification::GpuUnavailable => {
                    !replay.gpu_dispatch_occurred && replay.gpu_chain_checksum.is_none()
                }
                MobilityGpuKernel0ParityClassification::GpuExecutionFailed => false,
            }
        })
    });
    let replay_stable_per_variant = variant_reports.iter().all(|variant| {
        variant.replays.windows(2).all(|pair| {
            pair[0].cpu_chain_checksum == pair[1].cpu_chain_checksum
                && pair[0].parity_classification == pair[1].parity_classification
                && pair[0].gpu_chain_checksum == pair[1].gpu_chain_checksum
        })
    });
    let distinct_projection_checksums: std::collections::BTreeSet<u64> = variant_reports
        .iter()
        .map(|variant| variant.projection_checksum)
        .collect();
    let distinct_chain_checksums: std::collections::BTreeSet<u64> = variant_reports
        .iter()
        .map(|variant| variant.cpu_chain_checksum)
        .collect();
    let distinct_variants_have_distinct_checksums = distinct_projection_checksums.len()
        == variant_reports.len()
        && distinct_chain_checksums.len() == variant_reports.len();
    let parity_classification = if variant_reports.iter().all(|variant| {
        variant.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if variant_reports.iter().all(|variant| {
        variant.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    };

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = true;
    report.registration_non_executing = false;
    report.variant_count = variant_reports.len();
    report.replays_per_variant = replay_count;
    report.row_count = variant_reports
        .first()
        .map(|variant| variant.row_count)
        .unwrap_or(0);
    report.uses_registered_node = uses_registered_node;
    report.reuses_kernel6_chain = true;
    report.cpu_oracle_complete_per_variant = cpu_oracle_complete_per_variant;
    report.gpu_checksums_match_or_unavailable = gpu_checksums_match_or_unavailable;
    report.replay_stable_per_variant = replay_stable_per_variant;
    report.distinct_variants_have_distinct_checksums = distinct_variants_have_distinct_checksums;
    report.source_projection_unchanged =
        source_before == source_after && projection_checksum_for_columns(&source_before) == source_checksum;
    report.gpu_dispatch_occurred = any_gpu_dispatch;
    report.parity_classification = parity_classification;
    report.variants = variant_reports;
    report
}

pub fn mobility_gpu_kernel8_shader_text_has_domain_terms() -> bool {
    mobility_gpu_kernel6_chain_shader_text_has_domain_terms()
}

fn classify_variant_parity(
    replays: &[MobilityGpuKernel8ReplayReport],
) -> MobilityGpuKernel0ParityClassification {
    if replays
        .iter()
        .all(|replay| replay.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity)
    {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if replays.iter().all(|replay| {
        replay.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    }
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel8ForbiddenPathRequests,
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

fn kernel6_input(
    input: &MobilityGpuKernel8FixtureInput,
    columns_override: Option<MobilityGpuKernel0ColumnProbe>,
) -> MobilityGpuKernel6FixtureInput {
    MobilityGpuKernel6FixtureInput {
        gate: MobilityGpuKernel6Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel6ForbiddenPathRequests {
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
        passgraph: MobilityGpuKernel6FixtureInput::default_chain().passgraph,
        columns_override,
    }
}

fn shell(input: &MobilityGpuKernel8FixtureInput) -> MobilityGpuKernel8FixtureReport {
    MobilityGpuKernel8FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL8_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL8_NAMED_GATE,
        batch_id: MOBILITY_GPU_KERNEL8_BATCH_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        variant_count: 0,
        replays_per_variant: 0,
        row_count: 0,
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_non_executing: true,
        reuses_kernel6_chain: false,
        kernel6_fixture_id: MOBILITY_GPU_KERNEL6_FIXTURE_ID,
        kernel6_chain_id: MOBILITY_GPU_KERNEL6_CHAIN_ID,
        generic_column_vocabulary_only: true,
        shader_text_has_domain_terms: mobility_gpu_kernel8_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL8_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        cpu_oracle_complete_per_variant: false,
        gpu_checksums_match_or_unavailable: false,
        replay_stable_per_variant: false,
        distinct_variants_have_distinct_checksums: false,
        source_projection_unchanged: false,
        gpu_dispatch_occurred: false,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        variants: Vec::new(),
    }
}

fn disabled_no_op_report(input: &MobilityGpuKernel8FixtureInput) -> MobilityGpuKernel8FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel8FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel8FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}
