//! MOBILITY-GPU-KERNEL-3 — runtime composition to generic GPU column projection tests.

#[path = "support/mobility_gpu_kernel3_projection_fixture.rs"]
mod mobility_gpu_kernel3_projection_fixture;

use mobility_gpu_kernel3_projection_fixture::{
    cpu_column_transform_oracle, encode_parent_key_for_projection,
    project_runtime_composition_input_to_columns, projection_row_checksum,
    run_mobility_gpu_kernel3_fixture, MobilityGpuKernel0ParityClassification,
    MobilityGpuKernel3FixtureInput, MobilityGpuKernel3ForbiddenPathRequests,
    MobilityGpuKernel3Gate, MobilityRuntime1aDriverFixtureInput,
    MobilityRuntime1bPassgraphFixtureInput, MobilityRuntime1bPassgraphGate,
    MOBILITY_GPU_KERNEL1_FIXTURE_ID, MOBILITY_GPU_KERNEL3_FIXTURE_ID,
    MOBILITY_GPU_KERNEL3_GENERIC_COLUMNS, MOBILITY_GPU_KERNEL3_NAMED_GATE,
    MOBILITY_GPU_KERNEL3_NEW_SHADER_TEXT_ADDED, MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
};
use simthing_spec::{
    compose_mobility_runtime0, IdentityLane, MobilityAlloc0BlockSpec,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput, MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord,
    MobilityEcon0PlanInput, MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord,
    MobilityIdroute0PlanInput, MobilityOwner0ColumnKind, MobilityOwner0ColumnValue,
    MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord, MobilityOwner0Overlay,
    MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move,
    MobilityReenroll0PlanInput, MobilityReenroll0RegistryState, MobilityRuntime0CompositionInput,
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig,
};

fn key(parent_id: u64, key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey { parent_id, key_id }
}

fn block(parent_id: u64, key_id: u64, start_slot: u32, slot_count: u32) -> MobilityAlloc0BlockSpec {
    MobilityAlloc0BlockSpec {
        parent_key: key(parent_id, key_id),
        start_slot,
        slot_count,
        reserved_headroom: slot_count / 2,
    }
}

fn live(parent_id: u64, key_id: u64, entity_id: u64, slot: u32) -> MobilityAlloc0LiveSlice {
    MobilityAlloc0LiveSlice {
        entity_id,
        parent_key: key(parent_id, key_id),
        slot,
    }
}

fn mv(
    entity_id: u64,
    origin_key: u64,
    destination_key: u64,
    arrival_order: u64,
) -> MobilityReenroll0Move {
    MobilityReenroll0Move {
        entity_id,
        origin: key(1, origin_key),
        destination: key(1, destination_key),
        arrival_order,
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
        parent_key: key(1, cell_key),
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
        cell_key: key(1, cell_key),
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
    cohort_count: u32,
    owner_columns: Vec<MobilityOwner0ColumnValue>,
) -> MobilityOwner0LocalRecord {
    MobilityOwner0LocalRecord {
        entity_id,
        cell_key: key(1, cell_key),
        cohort_count,
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

fn composition_fixture() -> MobilityRuntime0CompositionInput {
    let blocks = vec![
        block(1, 10, 0, 8),
        block(1, 20, 8, 8),
        block(1, 30, 16, 2),
        block(1, 31, 18, 2),
    ];
    let live_slices = vec![
        live(1, 10, 100, 0),
        live(1, 10, 101, 1),
        live(1, 30, 2, 16),
        live(1, 31, 3, 18),
    ];

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
            moves: vec![mv(100, 10, 20, 9)],
            forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
        },
        idroute: MobilityIdroute0PlanInput {
            records: vec![
                idrec(100, 20, 0, 10, 1.0),
                idrec(101, 10, 1, 6, 0.5),
                idrec(2, 30, 0, 2, 0.25),
            ],
            max_factions_per_cell: 4,
            forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
        },
        econ: MobilityEcon0PlanInput {
            records: vec![
                erec(20, 7, 10, 6, 1.0, 1),
                erec(10, 7, 4, 8, 0.5, 2),
                erec(30, 7, 1, 1, 0.25, 3),
            ],
            forbidden: MobilityEcon0ForbiddenPathRequests::default(),
        },
        owner: MobilityOwner0PlanInput {
            records: vec![
                orec(
                    100,
                    20,
                    1,
                    vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                ),
                orec(2, 30, 1, vec![owner(MobilityOwner0ColumnKind::Faction, 7)]),
                orec(3, 31, 1, vec![owner(MobilityOwner0ColumnKind::Species, 7)]),
            ],
            overlays: vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 42, 11)],
            owner_changes: vec![],
            forbidden: MobilityOwner0ForbiddenPathRequests::default(),
        },
        forbidden: MobilityRuntime0ForbiddenPathRequests::default(),
    }
}

fn permuted_composition_fixture() -> MobilityRuntime0CompositionInput {
    let mut input = composition_fixture();
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

fn passgraph_input() -> MobilityRuntime1bPassgraphFixtureInput {
    MobilityRuntime1bPassgraphFixtureInput {
        gate: MobilityRuntime1bPassgraphGate::explicit_opt_in(),
        driver: MobilityRuntime1aDriverFixtureInput {
            session: Default::default(),
            composition: composition_fixture(),
            forbidden: Default::default(),
        },
    }
}

fn fixture_input() -> MobilityGpuKernel3FixtureInput {
    MobilityGpuKernel3FixtureInput::default_projection_probe(passgraph_input())
}

fn rejected_with(
    forbidden: MobilityGpuKernel3ForbiddenPathRequests,
) -> mobility_gpu_kernel3_projection_fixture::MobilityGpuKernel3FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_gpu_kernel3_fixture(&input)
}

#[test]
fn mobility_gpu_kernel3_projection_preserves_deterministic_row_order() {
    let composition = compose_mobility_runtime0(&composition_fixture());
    let reenroll = composition.reenroll_report.unwrap();
    let columns = project_runtime_composition_input_to_columns(&composition_fixture()).unwrap();
    let expected_entity_order: Vec<u32> = reenroll
        .final_live_slices
        .iter()
        .map(|slice| slice.entity_id as u32)
        .collect();
    assert_eq!(columns.entity_id, expected_entity_order);
}

#[test]
fn mobility_gpu_kernel3_projection_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_gpu_kernel3_fixture(&fixture_input());
    assert!(matches!(
        report.parity_classification,
        MobilityGpuKernel0ParityClassification::ExactParity
            | MobilityGpuKernel0ParityClassification::GpuUnavailable
    ));
    if report.parity_classification == MobilityGpuKernel0ParityClassification::ExactParity {
        assert_eq!(
            report.cpu_oracle_checksum,
            report.gpu_result_checksum.unwrap()
        );
    }
}

#[test]
fn mobility_gpu_kernel3_projection_no_gpu_allocator_or_nondeterministic_atomics() {
    let report = run_mobility_gpu_kernel3_fixture(&fixture_input());
    assert!(!report.gpu_allocator_used);
    assert!(!report.nondeterministic_atomics_used);
}
