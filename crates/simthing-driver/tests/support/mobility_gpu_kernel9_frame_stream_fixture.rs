//! MOBILITY-GPU-KERNEL-9: deterministic multi-frame projection-variant stream soak over
//! the semantic-free KERNEL-6 chain.
//!
//! Driver test/support only. Sequences KERNEL-8 generic projection variants across explicit
//! frames without adding shader text, default scheduling, gameplay, or semantic WGSL.

#[path = "mobility_gpu_kernel8_variant_batch_fixture.rs"]
mod mobility_gpu_kernel8_variant_batch_fixture;

use mobility_gpu_kernel8_variant_batch_fixture::{
    build_projection_variants, run_mobility_gpu_kernel6_fixture, MobilityGpuKernel6FixtureInput,
    MobilityGpuKernel6ForbiddenPathRequests, MobilityGpuKernel6Gate,
    MOBILITY_GPU_KERNEL6_CHAIN_ID, MOBILITY_GPU_KERNEL6_FIXTURE_ID,
    MOBILITY_GPU_KERNEL8_BATCH_ID, MOBILITY_GPU_KERNEL8_FIXTURE_ID,
    MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT,
};

pub use mobility_gpu_kernel8_variant_batch_fixture::{
    cpu_chain_checksum_for_columns, mobility_gpu_kernel8_shader_text_has_domain_terms,
    projection_checksum_for_columns, projected_34k_columns_for_kernel6,
    MobilityGpuKernel0ColumnProbe, MobilityGpuKernel0ParityClassification,
    MOBILITY_GPU_KERNEL4_ROW_COUNT, MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
    MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK, MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
    MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL9_FIXTURE_ID: &str =
    "mobility_gpu_kernel9_multi_frame_variant_stream_fixture";
pub const MOBILITY_GPU_KERNEL9_NAMED_GATE: &str =
    "mobility_gpu_kernel9_frame_stream_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL9_STREAM_ID: &str =
    "mobility_gpu_kernel9_projection_variant_frame_stream_soak";
pub const MOBILITY_GPU_KERNEL9_FRAME_COUNT: usize = 4;
pub const MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT: usize = MOBILITY_GPU_KERNEL8_MIN_REPLAYS_PER_VARIANT;
pub const MOBILITY_GPU_KERNEL9_NEW_SHADER_TEXT_ADDED: bool = false;

pub const MOBILITY_GPU_KERNEL9_FRAME_CANONICAL: &str = "frame0_canonical_batch";
pub const MOBILITY_GPU_KERNEL9_FRAME_REVERSED: &str = "frame1_reversed_batch";
pub const MOBILITY_GPU_KERNEL9_FRAME_REPEAT: &str = "frame2_repeat_canonical";
pub const MOBILITY_GPU_KERNEL9_FRAME_ALT_ORDER: &str = "frame3_alt_variant_order";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel9FrameSpec {
    pub frame_id: &'static str,
    pub variant_order: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel9Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel9Gate {
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
pub struct MobilityGpuKernel9ForbiddenPathRequests {
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
pub struct MobilityGpuKernel9FixtureInput {
    pub gate: MobilityGpuKernel9Gate,
    pub forbidden: MobilityGpuKernel9ForbiddenPathRequests,
    pub replays_per_variant: usize,
}

impl MobilityGpuKernel9FixtureInput {
    pub fn default_frame_stream() -> Self {
        Self {
            gate: MobilityGpuKernel9Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel9ForbiddenPathRequests::default(),
            replays_per_variant: MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel9VariantReplayReport {
    pub replay: usize,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub gpu_dispatch_occurred: bool,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel9FrameVariantReport {
    pub variant_id: &'static str,
    pub row_count: usize,
    pub projection_checksum: u64,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub replays: Vec<MobilityGpuKernel9VariantReplayReport>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel9FrameReport {
    pub frame_index: usize,
    pub frame_id: &'static str,
    pub variant_count: usize,
    pub cpu_frame_checksum: u64,
    pub gpu_frame_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub variants: Vec<MobilityGpuKernel9FrameVariantReport>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel9FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub stream_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub frame_count: usize,
    pub replays_per_variant: usize,
    pub row_count: usize,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_non_executing: bool,
    pub reuses_kernel8_variants: bool,
    pub reuses_kernel6_chain: bool,
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

    pub cpu_oracle_complete_per_frame: bool,
    pub gpu_checksums_match_or_unavailable: bool,
    pub replay_stable_per_frame: bool,
    pub repeated_frames_have_identical_checksums: bool,
    pub distinct_frames_have_distinct_checksums: bool,
    pub source_projection_unchanged: bool,
    pub gpu_dispatch_occurred: bool,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub frames: Vec<MobilityGpuKernel9FrameReport>,
}

pub fn build_frame_stream_specs() -> Vec<MobilityGpuKernel9FrameSpec> {
    vec![
        MobilityGpuKernel9FrameSpec {
            frame_id: MOBILITY_GPU_KERNEL9_FRAME_CANONICAL,
            variant_order: &[
                MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
                MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
                MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
                MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
            ],
        },
        MobilityGpuKernel9FrameSpec {
            frame_id: MOBILITY_GPU_KERNEL9_FRAME_REVERSED,
            variant_order: &[
                MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
                MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
                MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
                MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
            ],
        },
        MobilityGpuKernel9FrameSpec {
            frame_id: MOBILITY_GPU_KERNEL9_FRAME_REPEAT,
            variant_order: &[
                MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
                MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
                MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
                MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
            ],
        },
        MobilityGpuKernel9FrameSpec {
            frame_id: MOBILITY_GPU_KERNEL9_FRAME_ALT_ORDER,
            variant_order: &[
                MOBILITY_GPU_KERNEL8_VARIANT_SPARSE_DELTA,
                MOBILITY_GPU_KERNEL8_VARIANT_DENSE_BULK,
                MOBILITY_GPU_KERNEL8_VARIANT_PARENT_OFFSET,
                MOBILITY_GPU_KERNEL8_VARIANT_BASELINE,
            ],
        },
    ]
}

pub fn frame_cpu_checksum(variants: &[MobilityGpuKernel9FrameVariantReport]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for variant in variants {
        hash = fnv_append_u64(hash, variant.projection_checksum);
        hash = fnv_append_u64(hash, variant.cpu_chain_checksum);
    }
    hash
}

pub fn frame_gpu_checksum(variants: &[MobilityGpuKernel9FrameVariantReport]) -> Option<u64> {
    if variants.iter().any(|variant| {
        variant.parity_classification != MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        return None;
    }
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for variant in variants {
        hash = fnv_append_u64(hash, variant.projection_checksum);
        hash = fnv_append_u64(hash, variant.gpu_chain_checksum?);
    }
    Some(hash)
}

pub fn run_mobility_gpu_kernel9_fixture(
    input: &MobilityGpuKernel9FixtureInput,
) -> MobilityGpuKernel9FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel9_default_on_rejected"]);
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
        report.reuses_kernel8_variants = kernel6.admitted;
        return report;
    }

    let source_before = projected_34k_columns_for_kernel6();
    let source_checksum = projection_checksum_for_columns(&source_before);
    let variant_map: std::collections::BTreeMap<&'static str, MobilityGpuKernel0ColumnProbe> =
        build_projection_variants(&source_before)
            .into_iter()
            .collect();
    let replay_count = input
        .replays_per_variant
        .max(MOBILITY_GPU_KERNEL9_MIN_REPLAYS_PER_VARIANT);
    let frame_specs = build_frame_stream_specs();
    let mut frame_reports = Vec::with_capacity(frame_specs.len());
    let mut diagnostics = Vec::new();
    let mut uses_registered_node = false;
    let mut any_gpu_dispatch = false;

    for (frame_index, spec) in frame_specs.iter().enumerate() {
        let mut variants = Vec::with_capacity(spec.variant_order.len());
        for &variant_id in spec.variant_order {
            let columns = match variant_map.get(variant_id) {
                Some(columns) => columns.clone(),
                None => {
                    diagnostics.push("kernel9_variant_missing");
                    break;
                }
            };
            let projection_checksum = projection_checksum_for_columns(&columns);
            let expected_cpu_chain_checksum = cpu_chain_checksum_for_columns(&columns);
            let mut replays = Vec::with_capacity(replay_count);

            for replay in 0..replay_count {
                let kernel6 =
                    run_mobility_gpu_kernel6_fixture(&kernel6_input(input, Some(columns.clone())));
                if !kernel6.admitted {
                    diagnostics.extend(kernel6.diagnostics);
                    break;
                }
                uses_registered_node |= kernel6.uses_registered_node;
                any_gpu_dispatch |= kernel6.gpu_dispatch_occurred;
                replays.push(MobilityGpuKernel9VariantReplayReport {
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
            variants.push(MobilityGpuKernel9FrameVariantReport {
                variant_id,
                row_count: columns.entity_id.len(),
                projection_checksum,
                cpu_chain_checksum: expected_cpu_chain_checksum,
                gpu_chain_checksum,
                parity_classification: classify_replay_parity(&replays),
                replays,
            });
        }

        if !diagnostics.is_empty() {
            break;
        }

        let cpu_frame_checksum = frame_cpu_checksum(&variants);
        let gpu_frame_checksum = frame_gpu_checksum(&variants);
        frame_reports.push(MobilityGpuKernel9FrameReport {
            frame_index,
            frame_id: spec.frame_id,
            variant_count: variants.len(),
            cpu_frame_checksum,
            gpu_frame_checksum,
            parity_classification: classify_frame_parity(&variants),
            variants,
        });
    }

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    let source_after = projected_34k_columns_for_kernel6();
    let cpu_oracle_complete_per_frame = frame_reports.iter().all(|frame| {
        frame.variant_count > 0
            && frame.cpu_frame_checksum != 0
            && frame.variants.iter().all(|variant| {
                variant.row_count == MOBILITY_GPU_KERNEL4_ROW_COUNT
                    && variant.cpu_chain_checksum != 0
                    && variant
                        .replays
                        .iter()
                        .all(|replay| replay.cpu_chain_checksum == variant.cpu_chain_checksum)
            })
    });
    let gpu_checksums_match_or_unavailable = frame_reports.iter().all(|frame| {
        frame.variants.iter().all(|variant| {
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
        })
    });
    let replay_stable_per_frame = frame_reports.iter().all(|frame| {
        frame.variants.iter().all(|variant| {
            variant.replays.windows(2).all(|pair| {
                pair[0].cpu_chain_checksum == pair[1].cpu_chain_checksum
                    && pair[0].parity_classification == pair[1].parity_classification
                    && pair[0].gpu_chain_checksum == pair[1].gpu_chain_checksum
            })
        })
    });
    let repeated_frames_have_identical_checksums = frame_reports.len() >= 3
        && frame_reports[0].cpu_frame_checksum == frame_reports[2].cpu_frame_checksum
        && frame_reports[0].gpu_frame_checksum == frame_reports[2].gpu_frame_checksum;
    let distinct_cpu: std::collections::BTreeSet<u64> = frame_reports
        .iter()
        .map(|frame| frame.cpu_frame_checksum)
        .collect();
    let distinct_frames_have_distinct_checksums =
        distinct_cpu.len() >= 3 && frame_reports[0].cpu_frame_checksum != frame_reports[1].cpu_frame_checksum
            && frame_reports[1].cpu_frame_checksum != frame_reports[3].cpu_frame_checksum;
    let parity_classification = if frame_reports.iter().all(|frame| {
        frame.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if frame_reports.iter().all(|frame| {
        frame.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    };

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = true;
    report.registration_non_executing = false;
    report.frame_count = frame_reports.len();
    report.replays_per_variant = replay_count;
    report.row_count = frame_reports
        .first()
        .and_then(|frame| frame.variants.first())
        .map(|variant| variant.row_count)
        .unwrap_or(0);
    report.uses_registered_node = uses_registered_node;
    report.reuses_kernel8_variants = true;
    report.reuses_kernel6_chain = true;
    report.cpu_oracle_complete_per_frame = cpu_oracle_complete_per_frame;
    report.gpu_checksums_match_or_unavailable = gpu_checksums_match_or_unavailable;
    report.replay_stable_per_frame = replay_stable_per_frame;
    report.repeated_frames_have_identical_checksums = repeated_frames_have_identical_checksums;
    report.distinct_frames_have_distinct_checksums = distinct_frames_have_distinct_checksums;
    report.source_projection_unchanged =
        source_before == source_after && projection_checksum_for_columns(&source_before) == source_checksum;
    report.gpu_dispatch_occurred = any_gpu_dispatch;
    report.parity_classification = parity_classification;
    report.frames = frame_reports;
    report
}

pub fn mobility_gpu_kernel9_shader_text_has_domain_terms() -> bool {
    mobility_gpu_kernel8_shader_text_has_domain_terms()
}

fn classify_replay_parity(
    replays: &[MobilityGpuKernel9VariantReplayReport],
) -> MobilityGpuKernel0ParityClassification {
    if replays.iter().all(|replay| {
        replay.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if replays.iter().all(|replay| {
        replay.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    }
}

fn classify_frame_parity(
    variants: &[MobilityGpuKernel9FrameVariantReport],
) -> MobilityGpuKernel0ParityClassification {
    if variants.iter().all(|variant| {
        variant.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if variants.iter().all(|variant| {
        variant.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    }
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel9ForbiddenPathRequests,
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
    input: &MobilityGpuKernel9FixtureInput,
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

fn shell(input: &MobilityGpuKernel9FixtureInput) -> MobilityGpuKernel9FixtureReport {
    MobilityGpuKernel9FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL9_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL9_NAMED_GATE,
        stream_id: MOBILITY_GPU_KERNEL9_STREAM_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
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
        kernel8_fixture_id: MOBILITY_GPU_KERNEL8_FIXTURE_ID,
        kernel8_batch_id: MOBILITY_GPU_KERNEL8_BATCH_ID,
        kernel6_fixture_id: MOBILITY_GPU_KERNEL6_FIXTURE_ID,
        kernel6_chain_id: MOBILITY_GPU_KERNEL6_CHAIN_ID,
        generic_column_vocabulary_only: true,
        shader_text_has_domain_terms: mobility_gpu_kernel9_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL9_NEW_SHADER_TEXT_ADDED,
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

fn disabled_no_op_report(input: &MobilityGpuKernel9FixtureInput) -> MobilityGpuKernel9FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel9FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel9FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}
