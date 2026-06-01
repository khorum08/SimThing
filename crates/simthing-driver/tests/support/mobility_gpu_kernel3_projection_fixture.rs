//! MOBILITY-GPU-KERNEL-3: project accepted RUNTIME composition outputs into generic GPU column
//! buffers and dispatch through MOBILITY-GPU-KERNEL-1 registered-node path.
//!
//! Test/support only — not default production scheduling, not gameplay path.

#[path = "mobility_gpu_kernel1_dispatch_fixture.rs"]
mod mobility_gpu_kernel1_dispatch_fixture;

use mobility_gpu_kernel1_dispatch_fixture::{
    run_mobility_gpu_kernel1_fixture, MobilityGpuKernel1FixtureInput, MobilityGpuKernel1FixtureReport,
    MobilityGpuKernel1ForbiddenPathRequests, MobilityGpuKernel1Gate,
};

use simthing_spec::{
    compose_mobility_runtime0, MobilityAlloc0ParentKey, MobilityReenroll0CommittedMove,
    MobilityRuntime0CompositionInput, MobilityRuntime0CompositionReport,
};

pub use mobility_gpu_kernel1_dispatch_fixture::{
    cpu_column_transform_oracle, MobilityGpuKernel0ColumnProbe, MobilityGpuKernel0FixtureInput,
    MobilityGpuKernel0Gate, MobilityGpuKernel0ParityClassification,
    MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bPassgraphFixtureInput,
    MobilityRuntime1bPassgraphGate, MOBILITY_GPU_KERNEL0_KERNEL_ID,
    MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

pub const MOBILITY_GPU_KERNEL3_FIXTURE_ID: &str =
    "mobility_gpu_kernel3_runtime_composition_column_projection_fixture";
pub const MOBILITY_GPU_KERNEL3_NAMED_GATE: &str =
    "mobility_gpu_kernel3_projection_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL3_NEW_SHADER_TEXT_ADDED: bool = false;

/// Generic column vocabulary projected from composition (no owner/econ semantics).
pub const MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS: [&str; 6] = [
    "entity_id", "src_parent", "dst_parent", "move_mask", "out_parent", "out_changed",
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel3Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel3Gate {
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
pub struct MobilityGpuKernel3ForbiddenPathRequests {
    pub semantic_or_raw_wgsl: bool,
    pub designer_authored_shader_input: bool,
    pub default_on_behavior: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel3FixtureInput {
    pub gate: MobilityGpuKernel3Gate,
    pub forbidden: MobilityGpuKernel3ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
}

impl MobilityGpuKernel3FixtureInput {
    pub fn default_projection_probe(passgraph: MobilityRuntime1bPassgraphFixtureInput) -> Self {
        Self {
            gate: MobilityGpuKernel3Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel3ForbiddenPathRequests::default(),
            passgraph,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel3ProjectionReport {
    pub composition_admitted: bool,
    pub row_count: usize,
    pub moved_entity_count: u32,
    pub unmoved_entity_count: u32,
    pub generic_column_vocabulary_only: bool,
    pub owner_econ_semantics_in_shader: bool,
    pub projection_checksum: u64,
    pub columns: MobilityGpuKernel0ColumnProbe,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel3FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
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
    pub registration_non_executing: bool,
    pub delegates_to_kernel1: bool,
    pub kernel1_fixture_id: &'static str,
    pub kernel0_kernel_id: &'static str,
    pub new_shader_text_added: bool,
    pub composition_projected: bool,

    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,

    pub cpu_oracle_complete: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,

    pub projection: Option<MobilityGpuKernel3ProjectionReport>,
    pub kernel1_report: Option<MobilityGpuKernel1FixtureReport>,
}

/// Deterministic neutral parent-key encoding for generic GPU column buffers.
pub fn encode_parent_key_for_projection(key: &MobilityAlloc0ParentKey) -> u32 {
    key.parent_id
        .wrapping_mul(1_000_003)
        .wrapping_add(key.key_id) as u32
}

fn move_index(moves: &[MobilityReenroll0CommittedMove]) -> std::collections::BTreeMap<u64, &MobilityReenroll0CommittedMove> {
    moves
        .iter()
        .map(|mv| (mv.entity_id, mv))
        .collect()
}

/// Project accepted RUNTIME-0 composition reenroll output into generic mobility columns.
/// Uses only reenroll `final_live_slices` and `committed_moves`; owner/econ reports are ignored.
pub fn project_runtime_composition_to_columns(
    composition: &MobilityRuntime0CompositionReport,
) -> Result<MobilityGpuKernel0ColumnProbe, &'static str> {
    if !composition.admitted {
        return Err("composition_not_admitted");
    }
    let reenroll = composition
        .reenroll_report
        .as_ref()
        .ok_or("reenroll_report_missing")?;
    if !reenroll.admitted {
        return Err("reenroll_not_admitted");
    }

    let moves = move_index(&reenroll.committed_moves);
    let mut src_parent = Vec::with_capacity(reenroll.final_live_slices.len());
    let mut dst_parent = Vec::with_capacity(reenroll.final_live_slices.len());
    let mut entity_id = Vec::with_capacity(reenroll.final_live_slices.len());
    let mut move_mask = Vec::with_capacity(reenroll.final_live_slices.len());

    for slice in &reenroll.final_live_slices {
        let eid = slice.entity_id;
        entity_id.push(eid as u32);
        if let Some(mv) = moves.get(&eid) {
            src_parent.push(encode_parent_key_for_projection(&mv.origin));
            dst_parent.push(encode_parent_key_for_projection(&mv.destination));
            move_mask.push(1);
        } else {
            let current = encode_parent_key_for_projection(&slice.parent_key);
            src_parent.push(current);
            dst_parent.push(current);
            move_mask.push(0);
        }
    }

    Ok(MobilityGpuKernel0ColumnProbe {
        src_parent,
        dst_parent,
        entity_id,
        move_mask,
    })
}

pub fn project_runtime_composition_input_to_columns(
    input: &MobilityRuntime0CompositionInput,
) -> Result<MobilityGpuKernel0ColumnProbe, Vec<&'static str>> {
    let composition = compose_mobility_runtime0(input);
    project_runtime_composition_to_columns(&composition).map_err(|reason| vec![reason])
}

pub fn projection_row_checksum(columns: &MobilityGpuKernel0ColumnProbe) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for values in [
        &columns.entity_id,
        &columns.src_parent,
        &columns.dst_parent,
        &columns.move_mask,
    ] {
        for value in values {
            hash = fnv_append_u32(hash, *value);
        }
    }
    hash
}

fn fnv_append_u32(mut hash: u64, value: u32) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn projection_report_from_columns(columns: &MobilityGpuKernel0ColumnProbe) -> MobilityGpuKernel3ProjectionReport {
    let moved_entity_count = columns.move_mask.iter().filter(|&&m| m != 0).count() as u32;
    let unmoved_entity_count = columns.move_mask.len() as u32 - moved_entity_count;
    MobilityGpuKernel3ProjectionReport {
        composition_admitted: true,
        row_count: columns.entity_id.len(),
        moved_entity_count,
        unmoved_entity_count,
        generic_column_vocabulary_only: true,
        owner_econ_semantics_in_shader: false,
        projection_checksum: projection_row_checksum(columns),
        columns: columns.clone(),
    }
}

fn to_kernel1_gate(gate: MobilityGpuKernel3Gate) -> MobilityGpuKernel1Gate {
    MobilityGpuKernel1Gate {
        registration_gate_enabled: gate.registration_gate_enabled,
        dispatch_gate_enabled: gate.dispatch_gate_enabled,
        enabled_by_default: gate.enabled_by_default,
    }
}

fn to_kernel1_forbidden(
    forbidden: &MobilityGpuKernel3ForbiddenPathRequests,
) -> MobilityGpuKernel1ForbiddenPathRequests {
    MobilityGpuKernel1ForbiddenPathRequests {
        semantic_or_raw_wgsl: forbidden.semantic_or_raw_wgsl,
        designer_authored_shader_input: forbidden.designer_authored_shader_input,
        default_on_behavior: forbidden.default_on_behavior,
    }
}

fn kernel1_input(
    input: &MobilityGpuKernel3FixtureInput,
    columns: Option<MobilityGpuKernel0ColumnProbe>,
) -> MobilityGpuKernel1FixtureInput {
    MobilityGpuKernel1FixtureInput {
        gate: to_kernel1_gate(input.gate),
        forbidden: to_kernel1_forbidden(&input.forbidden),
        passgraph: input.passgraph.clone(),
        kernel0: MobilityGpuKernel0FixtureInput {
            gate: MobilityGpuKernel0Gate::explicit_opt_in(),
            forbidden: Default::default(),
            columns: columns.unwrap_or_else(MobilityGpuKernel0ColumnProbe::default_probe),
        },
    }
}

pub fn run_mobility_gpu_kernel3_fixture(
    input: &MobilityGpuKernel3FixtureInput,
) -> MobilityGpuKernel3FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel3_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    let projection = if input.gate.dispatch_gate_enabled {
        match project_runtime_composition_input_to_columns(&input.passgraph.driver.composition) {
            Ok(columns) => Some(projection_report_from_columns(&columns)),
            Err(diagnostics) => return rejected_report(input, diagnostics),
        }
    } else {
        None
    };

    let columns = projection.as_ref().map(|p| p.columns.clone());
    let kernel1_report = run_mobility_gpu_kernel1_fixture(&kernel1_input(input, columns));

    map_kernel1_report(input, projection, kernel1_report)
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel3ForbiddenPathRequests,
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
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn shell(input: &MobilityGpuKernel3FixtureInput) -> MobilityGpuKernel3FixtureReport {
    MobilityGpuKernel3FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL3_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL3_NAMED_GATE,
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
        registration_non_executing: true,
        delegates_to_kernel1: false,
        kernel1_fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        kernel0_kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        new_shader_text_added: MOBILITY_GPU_KERNEL3_NEW_SHADER_TEXT_ADDED,
        composition_projected: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        cpu_oracle_complete: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        projection: None,
        kernel1_report: None,
    }
}

fn disabled_no_op_report(input: &MobilityGpuKernel3FixtureInput) -> MobilityGpuKernel3FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel3FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel3FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report
}

fn map_kernel1_report(
    input: &MobilityGpuKernel3FixtureInput,
    projection: Option<MobilityGpuKernel3ProjectionReport>,
    kernel1: MobilityGpuKernel1FixtureReport,
) -> MobilityGpuKernel3FixtureReport {
    let mut report = shell(input);
    report.admitted = kernel1.admitted;
    report.diagnostics = kernel1.diagnostics.clone();
    report.explicit_opt_in = kernel1.explicit_opt_in;
    report.disabled_no_op = kernel1.disabled_no_op;
    report.uses_registered_node = kernel1.uses_registered_node;
    report.registration_non_executing = kernel1.registration_non_executing;
    report.delegates_to_kernel1 = kernel1.admitted;
    report.composition_projected = projection.is_some();
    report.gpu_dispatch_occurred = kernel1.gpu_dispatch_occurred;
    report.cpu_oracle_checksum = kernel1.cpu_oracle_checksum;
    report.gpu_result_checksum = kernel1.gpu_result_checksum;
    report.parity_classification = kernel1.parity_classification;
    report.live_slot_compaction = kernel1.live_slot_compaction;
    report.gpu_allocator_used = kernel1.gpu_allocator_used;
    report.nondeterministic_atomics_used = kernel1.nondeterministic_atomics_used;
    report.cpu_planner_urgency_commitment = kernel1.cpu_planner_urgency_commitment;
    report.default_production_scheduling_wired = kernel1.default_production_scheduling_wired;
    report.hybrid_strata_or_faction_index_scaling = kernel1.hybrid_strata_or_faction_index_scaling;
    report.cpu_oracle_complete =
        input.gate.dispatch_gate_enabled && kernel1.admitted && kernel1.cpu_oracle_checksum != 0;
    report.projection = projection;
    report.kernel1_report = Some(kernel1);
    report
}
