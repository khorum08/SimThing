//! MOBILITY-GPU-KERNEL-7: deterministic replay soak for repeated explicit
//! dispatch of the semantic-free KERNEL-6 chain.
//!
//! Driver test/support only. This fixture repeats the ordered KERNEL-0 ->
//! KERNEL-5 chain over the same 34k projection without adding shader text,
//! default scheduling, gameplay, designer-authored input, or semantic WGSL.

#[path = "mobility_gpu_kernel6_chain_fixture.rs"]
mod mobility_gpu_kernel6_chain_fixture;

use mobility_gpu_kernel6_chain_fixture::{
    run_mobility_gpu_kernel6_fixture, MobilityGpuKernel6FixtureInput,
    MobilityGpuKernel6ForbiddenPathRequests, MobilityGpuKernel6Gate, MOBILITY_GPU_KERNEL6_CHAIN_ID,
    MOBILITY_GPU_KERNEL6_FIXTURE_ID,
};

pub use mobility_gpu_kernel6_chain_fixture::{
    cpu_chain_oracle, mobility_gpu_kernel6_chain_shader_text_has_domain_terms,
    permuted_projected_34k_columns_for_kernel6, projected_34k_columns_for_kernel6,
    MobilityGpuKernel0ParityClassification, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL7_FIXTURE_ID: &str =
    "mobility_gpu_kernel7_deterministic_replay_soak_fixture";
pub const MOBILITY_GPU_KERNEL7_NAMED_GATE: &str =
    "mobility_gpu_kernel7_replay_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL7_REPLAY_ID: &str =
    "mobility_gpu_kernel7_kernel6_repeated_dispatch_replay_soak";
pub const MOBILITY_GPU_KERNEL7_MIN_ITERATIONS: usize = 8;
pub const MOBILITY_GPU_KERNEL7_NEW_SHADER_TEXT_ADDED: bool = false;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel7Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel7Gate {
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
pub struct MobilityGpuKernel7ForbiddenPathRequests {
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
pub struct MobilityGpuKernel7FixtureInput {
    pub gate: MobilityGpuKernel7Gate,
    pub forbidden: MobilityGpuKernel7ForbiddenPathRequests,
    pub iterations: usize,
}

impl MobilityGpuKernel7FixtureInput {
    pub fn default_replay_soak() -> Self {
        Self {
            gate: MobilityGpuKernel7Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel7ForbiddenPathRequests::default(),
            iterations: MOBILITY_GPU_KERNEL7_MIN_ITERATIONS,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MobilityGpuKernel7IterationReport {
    pub iteration: usize,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub projection_checksum: u64,
    pub gpu_dispatch_occurred: bool,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel7FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub replay_id: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub iteration_count: usize,
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

    pub cpu_oracle_stable_across_iterations: bool,
    pub gpu_checksums_stable_or_unavailable: bool,
    pub source_projection_unchanged: bool,
    pub permutation_stable_oracle: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_chain_checksum: u64,
    pub gpu_chain_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub iterations: Vec<MobilityGpuKernel7IterationReport>,
}

pub fn run_mobility_gpu_kernel7_fixture(
    input: &MobilityGpuKernel7FixtureInput,
) -> MobilityGpuKernel7FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel7_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    if !input.gate.dispatch_gate_enabled {
        let kernel6 = run_mobility_gpu_kernel6_fixture(&kernel6_input(input));
        let mut report = shell(input);
        report.admitted = kernel6.admitted;
        report.diagnostics = kernel6.diagnostics;
        report.uses_registered_node = kernel6.uses_registered_node;
        report.registration_non_executing = true;
        report.reuses_kernel6_chain = kernel6.admitted;
        return report;
    }

    let source_before = projected_34k_columns_for_kernel6();
    let permuted = permuted_projected_34k_columns_for_kernel6();
    let permutation_stable_oracle = cpu_chain_oracle(&source_before) == cpu_chain_oracle(&permuted);
    let iteration_count = input.iterations.max(MOBILITY_GPU_KERNEL7_MIN_ITERATIONS);
    let mut iteration_reports = Vec::with_capacity(iteration_count);
    let mut diagnostics = Vec::new();
    let mut row_count = 0;
    let mut uses_registered_node = false;
    let mut reuses_kernel6_chain = false;
    let mut any_gpu_dispatch = false;

    for iteration in 0..iteration_count {
        let kernel6 = run_mobility_gpu_kernel6_fixture(&kernel6_input(input));
        if !kernel6.admitted {
            diagnostics.extend(kernel6.diagnostics);
            break;
        }
        row_count = kernel6.row_count;
        uses_registered_node |= kernel6.uses_registered_node;
        reuses_kernel6_chain = true;
        any_gpu_dispatch |= kernel6.gpu_dispatch_occurred;
        iteration_reports.push(MobilityGpuKernel7IterationReport {
            iteration,
            cpu_chain_checksum: kernel6.cpu_chain_checksum,
            gpu_chain_checksum: kernel6.gpu_chain_checksum,
            projection_checksum: kernel6.projection_checksum,
            gpu_dispatch_occurred: kernel6.gpu_dispatch_occurred,
            parity_classification: kernel6.parity_classification,
        });
    }

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    let source_after = projected_34k_columns_for_kernel6();
    let cpu_chain_checksum = iteration_reports
        .first()
        .map(|report| report.cpu_chain_checksum)
        .unwrap_or(0);
    let gpu_chain_checksum = iteration_reports
        .first()
        .and_then(|report| report.gpu_chain_checksum);
    let cpu_oracle_stable = cpu_chain_checksum != 0
        && iteration_reports
            .iter()
            .all(|report| report.cpu_chain_checksum == cpu_chain_checksum);
    let gpu_checksums_stable_or_unavailable =
        iteration_reports
            .iter()
            .all(|report| match report.parity_classification {
                MobilityGpuKernel0ParityClassification::ExactParity => {
                    report.gpu_chain_checksum == gpu_chain_checksum && gpu_chain_checksum.is_some()
                }
                MobilityGpuKernel0ParityClassification::GpuUnavailable => {
                    report.gpu_chain_checksum.is_none() && !report.gpu_dispatch_occurred
                }
                MobilityGpuKernel0ParityClassification::GpuExecutionFailed => false,
            });
    let parity_classification = if iteration_reports.iter().all(|report| {
        report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity
    }) {
        MobilityGpuKernel0ParityClassification::ExactParity
    } else if iteration_reports.iter().all(|report| {
        report.parity_classification == MobilityGpuKernel0ParityClassification::GpuUnavailable
    }) {
        MobilityGpuKernel0ParityClassification::GpuUnavailable
    } else {
        MobilityGpuKernel0ParityClassification::GpuExecutionFailed
    };

    let mut report = shell(input);
    report.admitted = true;
    report.explicit_opt_in = true;
    report.registration_non_executing = false;
    report.iteration_count = iteration_reports.len();
    report.row_count = row_count;
    report.uses_registered_node = uses_registered_node;
    report.reuses_kernel6_chain = reuses_kernel6_chain;
    report.cpu_oracle_stable_across_iterations = cpu_oracle_stable;
    report.gpu_checksums_stable_or_unavailable = gpu_checksums_stable_or_unavailable;
    report.source_projection_unchanged = source_before == source_after;
    report.permutation_stable_oracle = permutation_stable_oracle;
    report.gpu_dispatch_occurred = any_gpu_dispatch;
    report.cpu_chain_checksum = cpu_chain_checksum;
    report.gpu_chain_checksum = gpu_chain_checksum;
    report.parity_classification = parity_classification;
    report.iterations = iteration_reports;
    report
}

pub fn mobility_gpu_kernel7_replay_shader_text_has_domain_terms() -> bool {
    mobility_gpu_kernel6_chain_shader_text_has_domain_terms()
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel7ForbiddenPathRequests,
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

fn kernel6_input(input: &MobilityGpuKernel7FixtureInput) -> MobilityGpuKernel6FixtureInput {
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
        columns_override: None,
    }
}

fn shell(input: &MobilityGpuKernel7FixtureInput) -> MobilityGpuKernel7FixtureReport {
    MobilityGpuKernel7FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL7_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL7_NAMED_GATE,
        replay_id: MOBILITY_GPU_KERNEL7_REPLAY_ID,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        iteration_count: 0,
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
        shader_text_has_domain_terms: mobility_gpu_kernel7_replay_shader_text_has_domain_terms(),
        new_shader_text_added: MOBILITY_GPU_KERNEL7_NEW_SHADER_TEXT_ADDED,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        cpu_oracle_stable_across_iterations: false,
        gpu_checksums_stable_or_unavailable: false,
        source_projection_unchanged: false,
        permutation_stable_oracle: false,
        gpu_dispatch_occurred: false,
        cpu_chain_checksum: 0,
        gpu_chain_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        iterations: Vec::new(),
    }
}

fn disabled_no_op_report(
    input: &MobilityGpuKernel7FixtureInput,
) -> MobilityGpuKernel7FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel7FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel7FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}
