//! MOBILITY-GPU-KERNEL-4: 34k composition-derived projection soak through the
//! registered-node mobility column kernel path.
//!
//! Test/support only. This scales the KERNEL-3 composition projection to the
//! accepted 34k scenario shape without adding default scheduling, gameplay, or
//! designer-authored shader input.

#[path = "mobility_gpu_kernel3_projection_fixture.rs"]
mod mobility_gpu_kernel3_projection_fixture;

use mobility_gpu_kernel3_projection_fixture::{
    projection_row_checksum, run_mobility_gpu_kernel3_fixture, MobilityGpuKernel3FixtureInput,
    MobilityGpuKernel3ForbiddenPathRequests, MobilityGpuKernel3Gate,
    MobilityGpuKernel3ProjectionReport,
};

pub use mobility_gpu_kernel3_projection_fixture::{
    cpu_column_transform_oracle, encode_parent_key_for_projection,
    project_runtime_composition_input_to_columns, MobilityGpuKernel0ColumnProbe,
    MobilityGpuKernel0ParityClassification, MobilityRuntime1aDriverFixtureInput,
    MobilityRuntime1bPassgraphFixtureInput, MobilityRuntime1bPassgraphGate,
    MOBILITY_GPU_KERNEL0_KERNEL_ID, MOBILITY_GPU_KERNEL1_FIXTURE_ID,
    MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};

use simthing_spec::{
    IdentityLane, MobilityAlloc0BlockSpec, MobilityAlloc0ForbiddenPathRequests,
    MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey, MobilityAlloc0PlanInput,
    MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord, MobilityEcon0PlanInput,
    MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord, MobilityIdroute0PlanInput,
    MobilityOwner0ColumnKind, MobilityOwner0ColumnValue, MobilityOwner0ForbiddenPathRequests,
    MobilityOwner0LocalRecord, MobilityOwner0Overlay, MobilityOwner0PlanInput,
    MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move, MobilityReenroll0PlanInput,
    MobilityReenroll0RegistryState, MobilityRuntime0CompositionInput,
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig,
};

pub const MOBILITY_GPU_KERNEL4_FIXTURE_ID: &str =
    "mobility_gpu_kernel4_34k_composition_projection_soak_fixture";
pub const MOBILITY_GPU_KERNEL4_NAMED_GATE: &str =
    "mobility_gpu_kernel4_34k_projection_explicit_opt_in_gate";
pub const MOBILITY_GPU_KERNEL4_ROW_COUNT: usize = 34_000;
pub const MOBILITY_GPU_KERNEL4_BLOCK_COUNT: usize = 340;
pub const MOBILITY_GPU_KERNEL4_ROWS_PER_BLOCK: usize = 100;
pub const MOBILITY_GPU_KERNEL4_SLOTS_PER_BLOCK: u32 = 256;
pub const MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY: u64 = 50_001;
pub const MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY: u64 = 50_002;
pub const MOBILITY_GPU_KERNEL4_REPEATED_DEST_SLOTS: u32 = 4_096;
pub const MOBILITY_GPU_KERNEL4_NEW_SHADER_TEXT_ADDED: bool = false;

pub const MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START: usize = 10_000;
pub const MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END: usize = 10_050;
pub const MOBILITY_GPU_KERNEL4_SPARSE_STRIDE: usize = 1_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MobilityGpuKernel4Gate {
    pub registration_gate_enabled: bool,
    pub dispatch_gate_enabled: bool,
    pub enabled_by_default: bool,
}

impl MobilityGpuKernel4Gate {
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
pub struct MobilityGpuKernel4ForbiddenPathRequests {
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel4FixtureInput {
    pub gate: MobilityGpuKernel4Gate,
    pub forbidden: MobilityGpuKernel4ForbiddenPathRequests,
    pub passgraph: MobilityRuntime1bPassgraphFixtureInput,
}

impl MobilityGpuKernel4FixtureInput {
    pub fn default_34k_projection_soak() -> Self {
        Self {
            gate: MobilityGpuKernel4Gate::registration_and_dispatch(),
            forbidden: MobilityGpuKernel4ForbiddenPathRequests::default(),
            passgraph: passgraph_input_34k(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MobilityGpuKernel4FixtureReport {
    pub fixture_id: &'static str,
    pub named_gate: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub row_count: usize,
    pub generated_composition_row_count: usize,

    pub confined_to_driver_test_support: bool,
    pub default_simsession_lib_path_wired: bool,
    pub default_schedule_unchanged: bool,
    pub gameplay_facing_path: bool,

    pub uses_registered_node: bool,
    pub registration_non_executing: bool,
    pub delegates_to_kernel3: bool,
    pub delegates_to_kernel1: bool,
    pub kernel1_fixture_id: &'static str,
    pub kernel0_kernel_id: &'static str,
    pub new_shader_text_added: bool,
    pub composition_projected: bool,
    pub generic_column_vocabulary_only: bool,
    pub owner_econ_semantics_in_shader: bool,

    pub semantic_or_raw_wgsl_present: bool,
    pub designer_shader_input_present: bool,
    pub live_slot_compaction: bool,
    pub gpu_allocator_used: bool,
    pub nondeterministic_atomics_used: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_production_scheduling_wired: bool,
    pub hybrid_strata_or_faction_index_scaling: bool,
    pub closed_ladders_reopened: bool,

    pub moved_entity_count: u32,
    pub unmoved_entity_count: u32,
    pub cpu_oracle_complete: bool,
    pub gpu_dispatch_occurred: bool,
    pub cpu_oracle_checksum: u64,
    pub gpu_result_checksum: Option<u64>,
    pub parity_classification: MobilityGpuKernel0ParityClassification,
    pub projection_checksum: u64,
    pub projection: Option<MobilityGpuKernel3ProjectionReport>,
}

pub fn run_mobility_gpu_kernel4_fixture(
    input: &MobilityGpuKernel4FixtureInput,
) -> MobilityGpuKernel4FixtureReport {
    if input.gate.enabled_by_default {
        return rejected_report(input, vec!["mobility_gpu_kernel4_default_on_rejected"]);
    }

    if let Some(diagnostics) = validate_forbidden(&input.forbidden) {
        return rejected_report(input, diagnostics);
    }

    if !input.gate.registration_gate_enabled && !input.gate.dispatch_gate_enabled {
        return disabled_no_op_report(input);
    }

    let kernel3_report = run_mobility_gpu_kernel3_fixture(&kernel3_input(input));
    let projection = kernel3_report.projection.clone();
    let mut report = shell(input);
    report.admitted = kernel3_report.admitted;
    report.diagnostics = kernel3_report.diagnostics;
    report.explicit_opt_in = kernel3_report.explicit_opt_in;
    report.disabled_no_op = kernel3_report.disabled_no_op;
    report.uses_registered_node = kernel3_report.uses_registered_node;
    report.registration_non_executing = kernel3_report.registration_non_executing;
    report.delegates_to_kernel3 = kernel3_report.admitted;
    report.delegates_to_kernel1 = kernel3_report.delegates_to_kernel1;
    report.composition_projected = kernel3_report.composition_projected;
    report.gpu_dispatch_occurred = kernel3_report.gpu_dispatch_occurred;
    report.cpu_oracle_checksum = kernel3_report.cpu_oracle_checksum;
    report.gpu_result_checksum = kernel3_report.gpu_result_checksum;
    report.parity_classification = kernel3_report.parity_classification;
    report.live_slot_compaction = kernel3_report.live_slot_compaction;
    report.gpu_allocator_used = kernel3_report.gpu_allocator_used;
    report.nondeterministic_atomics_used = kernel3_report.nondeterministic_atomics_used;
    report.cpu_planner_urgency_commitment = kernel3_report.cpu_planner_urgency_commitment;
    report.default_production_scheduling_wired = kernel3_report.default_production_scheduling_wired;
    report.hybrid_strata_or_faction_index_scaling =
        kernel3_report.hybrid_strata_or_faction_index_scaling;
    report.cpu_oracle_complete = kernel3_report.cpu_oracle_complete;

    if let Some(projection) = projection {
        report.row_count = projection.row_count;
        report.moved_entity_count = projection.moved_entity_count;
        report.unmoved_entity_count = projection.unmoved_entity_count;
        report.generic_column_vocabulary_only = projection.generic_column_vocabulary_only;
        report.owner_econ_semantics_in_shader = projection.owner_econ_semantics_in_shader;
        report.projection_checksum = projection.projection_checksum;
        report.projection = Some(projection);
    }

    report
}

pub fn generate_34k_runtime_composition_input() -> MobilityRuntime0CompositionInput {
    let mut blocks = Vec::with_capacity(MOBILITY_GPU_KERNEL4_BLOCK_COUNT + 2);
    let mut live_slices = Vec::with_capacity(MOBILITY_GPU_KERNEL4_ROW_COUNT);

    for block_index in 0..MOBILITY_GPU_KERNEL4_BLOCK_COUNT {
        let key_id = source_key_for_block(block_index);
        let start_slot = (block_index as u32) * MOBILITY_GPU_KERNEL4_SLOTS_PER_BLOCK;
        blocks.push(block(
            key_id,
            start_slot,
            MOBILITY_GPU_KERNEL4_SLOTS_PER_BLOCK,
        ));
        for local_row in 0..MOBILITY_GPU_KERNEL4_ROWS_PER_BLOCK {
            let row = block_index * MOBILITY_GPU_KERNEL4_ROWS_PER_BLOCK + local_row;
            live_slices.push(live(
                key_id,
                entity_for_row(row),
                start_slot + local_row as u32,
            ));
        }
    }
    let repeated_start =
        (MOBILITY_GPU_KERNEL4_BLOCK_COUNT as u32) * MOBILITY_GPU_KERNEL4_SLOTS_PER_BLOCK;
    blocks.push(block(
        MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY,
        repeated_start,
        MOBILITY_GPU_KERNEL4_REPEATED_DEST_SLOTS,
    ));
    blocks.push(block(
        MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY,
        repeated_start + MOBILITY_GPU_KERNEL4_REPEATED_DEST_SLOTS,
        MOBILITY_GPU_KERNEL4_SLOTS_PER_BLOCK,
    ));

    let mut moves = Vec::new();
    for row in 0..MOBILITY_GPU_KERNEL4_ROW_COUNT {
        if move_mask_for_row(row) {
            let origin_key = source_key_for_block(row / MOBILITY_GPU_KERNEL4_ROWS_PER_BLOCK);
            let destination_key = if row % 17 == 0 {
                MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY
            } else {
                MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY
            };
            moves.push(mv(row, origin_key, destination_key));
        }
    }

    MobilityRuntime0CompositionInput {
        config: MobilityRuntime0HarnessConfig::opt_in_test_harness(),
        alloc: MobilityAlloc0PlanInput {
            blocks: blocks.clone(),
            live_slices: live_slices.clone(),
            events: vec![],
            forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
        },
        reenroll: MobilityReenroll0PlanInput {
            registry: MobilityReenroll0RegistryState {
                blocks,
                live_slices,
                origin_generations: Default::default(),
                destination_generations: Default::default(),
            },
            moves,
            forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
        },
        idroute: MobilityIdroute0PlanInput {
            records: vec![
                idrec(entity_for_row(0), source_key_for_block(0), 1, 10, 1.0),
                idrec(
                    entity_for_row(MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START),
                    MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY,
                    2,
                    6,
                    0.5,
                ),
                idrec(
                    entity_for_row(MOBILITY_GPU_KERNEL4_ROW_COUNT - 1),
                    339,
                    3,
                    3,
                    0.25,
                ),
            ],
            max_factions_per_cell: 4,
            forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
        },
        econ: MobilityEcon0PlanInput {
            records: vec![
                erec(MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY, 7, 10, 6, 1.0, 1),
                erec(source_key_for_block(0), 7, 4, 8, 0.5, 2),
                erec(MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY, 7, 1, 1, 0.25, 3),
            ],
            forbidden: MobilityEcon0ForbiddenPathRequests::default(),
        },
        owner: MobilityOwner0PlanInput {
            records: vec![
                orec(
                    entity_for_row(0),
                    MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY,
                    vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                ),
                orec(
                    entity_for_row(MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START),
                    MOBILITY_GPU_KERNEL4_REPEATED_DEST_KEY,
                    vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                ),
                orec(
                    entity_for_row(MOBILITY_GPU_KERNEL4_ROW_COUNT - 1),
                    MOBILITY_GPU_KERNEL4_ALTERNATE_DEST_KEY,
                    vec![owner(MobilityOwner0ColumnKind::Species, 7)],
                ),
            ],
            overlays: vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 42, 11)],
            owner_changes: vec![],
            forbidden: MobilityOwner0ForbiddenPathRequests::default(),
        },
        forbidden: MobilityRuntime0ForbiddenPathRequests::default(),
    }
}

pub fn generate_permuted_34k_runtime_composition_input() -> MobilityRuntime0CompositionInput {
    let mut input = generate_34k_runtime_composition_input();
    input.alloc.blocks.reverse();
    input.alloc.live_slices.reverse();
    input.reenroll.registry.blocks.reverse();
    input.reenroll.registry.live_slices.reverse();
    input.reenroll.moves.reverse();
    input.idroute.records.reverse();
    input.econ.records.reverse();
    input.owner.records.reverse();
    input.owner.overlays.reverse();
    input
}

pub fn projected_34k_columns() -> MobilityGpuKernel0ColumnProbe {
    project_runtime_composition_input_to_columns(&generate_34k_runtime_composition_input())
        .expect("34k runtime composition projection should be admitted")
}

pub fn move_mask_for_row(row: usize) -> bool {
    if row == 0 || row + 1 == MOBILITY_GPU_KERNEL4_ROW_COUNT {
        return true;
    }
    if row % MOBILITY_GPU_KERNEL4_SPARSE_STRIDE == 0 {
        return true;
    }
    if (MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_START..MOBILITY_GPU_KERNEL4_DENSE_CLUSTER_END)
        .contains(&row)
    {
        return true;
    }
    if (20_000..20_100).contains(&row) {
        return row % 2 == 0;
    }
    row % 11 == 0
}

pub fn entity_for_row(row: usize) -> u64 {
    100_000 + row as u64
}

pub fn source_key_for_block(block_index: usize) -> u64 {
    1_000 + block_index as u64
}

fn passgraph_input_34k() -> MobilityRuntime1bPassgraphFixtureInput {
    MobilityRuntime1bPassgraphFixtureInput {
        gate: MobilityRuntime1bPassgraphGate::explicit_opt_in(),
        driver: MobilityRuntime1aDriverFixtureInput {
            session: Default::default(),
            composition: generate_34k_runtime_composition_input(),
            forbidden: Default::default(),
        },
    }
}

fn kernel3_input(input: &MobilityGpuKernel4FixtureInput) -> MobilityGpuKernel3FixtureInput {
    MobilityGpuKernel3FixtureInput {
        gate: MobilityGpuKernel3Gate {
            registration_gate_enabled: input.gate.registration_gate_enabled,
            dispatch_gate_enabled: input.gate.dispatch_gate_enabled,
            enabled_by_default: input.gate.enabled_by_default,
        },
        forbidden: MobilityGpuKernel3ForbiddenPathRequests {
            semantic_or_raw_wgsl: input.forbidden.semantic_or_raw_wgsl,
            designer_authored_shader_input: input.forbidden.designer_authored_shader_input,
            default_on_behavior: input.forbidden.default_on_behavior,
        },
        passgraph: input.passgraph.clone(),
    }
}

fn validate_forbidden(
    forbidden: &MobilityGpuKernel4ForbiddenPathRequests,
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
    if diagnostics.is_empty() {
        None
    } else {
        Some(diagnostics)
    }
}

fn shell(input: &MobilityGpuKernel4FixtureInput) -> MobilityGpuKernel4FixtureReport {
    MobilityGpuKernel4FixtureReport {
        fixture_id: MOBILITY_GPU_KERNEL4_FIXTURE_ID,
        named_gate: MOBILITY_GPU_KERNEL4_NAMED_GATE,
        admitted: false,
        diagnostics: Vec::new(),
        explicit_opt_in: input.gate.dispatch_gate_enabled,
        default_off: !input.gate.enabled_by_default,
        disabled_no_op: false,
        row_count: 0,
        generated_composition_row_count: if input.gate.dispatch_gate_enabled {
            MOBILITY_GPU_KERNEL4_ROW_COUNT
        } else {
            0
        },
        confined_to_driver_test_support: true,
        default_simsession_lib_path_wired: false,
        default_schedule_unchanged: true,
        gameplay_facing_path: false,
        uses_registered_node: false,
        registration_non_executing: true,
        delegates_to_kernel3: false,
        delegates_to_kernel1: false,
        kernel1_fixture_id: MOBILITY_GPU_KERNEL1_FIXTURE_ID,
        kernel0_kernel_id: MOBILITY_GPU_KERNEL0_KERNEL_ID,
        new_shader_text_added: MOBILITY_GPU_KERNEL4_NEW_SHADER_TEXT_ADDED,
        composition_projected: false,
        generic_column_vocabulary_only: true,
        owner_econ_semantics_in_shader: false,
        semantic_or_raw_wgsl_present: false,
        designer_shader_input_present: false,
        live_slot_compaction: false,
        gpu_allocator_used: false,
        nondeterministic_atomics_used: false,
        cpu_planner_urgency_commitment: false,
        default_production_scheduling_wired: false,
        hybrid_strata_or_faction_index_scaling: false,
        closed_ladders_reopened: false,
        moved_entity_count: 0,
        unmoved_entity_count: 0,
        cpu_oracle_complete: false,
        gpu_dispatch_occurred: false,
        cpu_oracle_checksum: 0,
        gpu_result_checksum: None,
        parity_classification: MobilityGpuKernel0ParityClassification::GpuUnavailable,
        projection_checksum: 0,
        projection: None,
    }
}

fn disabled_no_op_report(
    input: &MobilityGpuKernel4FixtureInput,
) -> MobilityGpuKernel4FixtureReport {
    let mut report = shell(input);
    report.admitted = true;
    report.disabled_no_op = true;
    report.generated_composition_row_count = 0;
    report
}

fn rejected_report(
    input: &MobilityGpuKernel4FixtureInput,
    diagnostics: Vec<&'static str>,
) -> MobilityGpuKernel4FixtureReport {
    let mut report = shell(input);
    report.diagnostics = diagnostics;
    report.generated_composition_row_count = 0;
    report
}

fn key(key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id,
    }
}

fn block(key_id: u64, start_slot: u32, slot_count: u32) -> MobilityAlloc0BlockSpec {
    MobilityAlloc0BlockSpec {
        parent_key: key(key_id),
        start_slot,
        slot_count,
        reserved_headroom: slot_count / 2,
    }
}

fn live(key_id: u64, entity_id: u64, slot: u32) -> MobilityAlloc0LiveSlice {
    MobilityAlloc0LiveSlice {
        entity_id,
        parent_key: key(key_id),
        slot,
    }
}

fn mv(row: usize, origin_key: u64, destination_key: u64) -> MobilityReenroll0Move {
    MobilityReenroll0Move {
        entity_id: entity_for_row(row),
        origin: key(origin_key),
        destination: key(destination_key),
        arrival_order: row as u64,
    }
}

fn idrec(
    entity_id: u64,
    cell_key: u64,
    identity: u32,
    hard_value: i64,
    soft_value: f32,
) -> MobilityIdroute0LocalRecord {
    MobilityIdroute0LocalRecord {
        entity_id,
        parent_key: key(cell_key),
        identity: IdentityLane(identity),
        hard_value,
        soft_value,
    }
}

fn erec(
    cell_key: u64,
    resource_id: u64,
    hard_available: i64,
    hard_need: i64,
    soft_beta_signal: f32,
    arrival_order: u64,
) -> MobilityEcon0LocalCellRecord {
    MobilityEcon0LocalCellRecord {
        session_id: 1,
        cell_key: key(cell_key),
        resource_id,
        hard_available,
        hard_need,
        soft_beta_signal,
        arrival_order,
    }
}

fn owner(kind: MobilityOwner0ColumnKind, owner_id: u64) -> MobilityOwner0ColumnValue {
    MobilityOwner0ColumnValue { kind, owner_id }
}

fn orec(
    entity_id: u64,
    cell_key: u64,
    owner_columns: Vec<MobilityOwner0ColumnValue>,
) -> MobilityOwner0LocalRecord {
    MobilityOwner0LocalRecord {
        entity_id,
        cell_key: key(cell_key),
        cohort_count: 1,
        owner_columns,
        generation: 0,
        blocked_by_blockade: false,
        arrival_order: entity_id,
    }
}

fn overlay(
    kind: MobilityOwner0ColumnKind,
    owner_id: u64,
    modifier_id: u64,
    modifier_amount: i64,
) -> MobilityOwner0Overlay {
    MobilityOwner0Overlay {
        owner: owner(kind, owner_id),
        modifier_id,
        modifier_amount,
        arrival_order: 0,
    }
}

#[allow(dead_code)]
pub fn projected_34k_checksum() -> u64 {
    projection_row_checksum(&projected_34k_columns())
}
