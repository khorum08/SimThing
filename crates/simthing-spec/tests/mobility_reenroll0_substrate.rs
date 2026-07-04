//! MOBILITY-REENROLL-0 bilateral arena re-enrollment substrate tests.

use simthing_spec::{
    mobility_reenroll0_layout_checksum_cpu, mobility_reenroll0_layout_checksum_gpu_proxy,
    plan_mobility_reenroll0, MobilityAlloc0BlockSpec, MobilityAlloc0LiveSlice,
    MobilityAlloc0ParentKey, MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move,
    MobilityReenroll0PlanInput, MobilityReenroll0RegistryState, MOBILITY_ALLOC0_ID,
    MOBILITY_REENROLL0_ID,
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

fn registry(
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
) -> MobilityReenroll0RegistryState {
    MobilityReenroll0RegistryState {
        blocks,
        live_slices,
        origin_generations: Default::default(),
        destination_generations: Default::default(),
    }
}

fn input(
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
    moves: Vec<MobilityReenroll0Move>,
) -> MobilityReenroll0PlanInput {
    MobilityReenroll0PlanInput {
        registry: registry(blocks, live_slices),
        moves,
        forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
    }
}

fn mv(
    entity_id: u64,
    origin_parent: u64,
    origin_key: u64,
    dest_parent: u64,
    dest_key: u64,
    arrival_order: u64,
) -> MobilityReenroll0Move {
    MobilityReenroll0Move {
        entity_id,
        origin: key(origin_parent, origin_key),
        destination: key(dest_parent, dest_key),
        arrival_order,
    }
}

#[test]
fn reenroll_cpu_gpu_parity_layout() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0), live(1, 10, 101, 1)],
        vec![mv(100, 1, 10, 1, 20, 7), mv(101, 1, 10, 1, 20, 3)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(
        mobility_reenroll0_layout_checksum_cpu(&report.final_live_slices),
        mobility_reenroll0_layout_checksum_gpu_proxy(&report.final_live_slices)
    );
}
