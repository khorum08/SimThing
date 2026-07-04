//! MOBILITY-IDROUTE-0 local D=2 identity-routing substrate tests.
//!
//! Follows the substrate pattern established by ALLOC-0 and REENROLL-0.

use simthing_spec::designer_admission::{
    mobility_idroute0_layout_checksum_cpu, mobility_idroute0_layout_checksum_gpu_proxy,
    plan_mobility_idroute0, IdentityLane, MobilityAlloc0ParentKey,
    MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord, MobilityIdroute0PlanInput,
    MOBILITY_IDROUTE0_ID,
};

fn key(parent_id: u64, key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey { parent_id, key_id }
}

fn rec(entity: u64, parent: u64, lane: u32, hard: i64, soft: f32) -> MobilityIdroute0LocalRecord {
    MobilityIdroute0LocalRecord {
        entity_id: entity,
        parent_key: key(parent, 0),
        identity: IdentityLane(lane),
        hard_value: hard,
        soft_value: soft,
    }
}

fn input(records: Vec<MobilityIdroute0LocalRecord>) -> MobilityIdroute0PlanInput {
    MobilityIdroute0PlanInput {
        records,
        max_factions_per_cell: 4,
        forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
    }
}

#[test]
fn idroute_cpu_gpu_parity_layout() {
    let records = vec![rec(1, 100, 0, 5, 0.0)];
    let cpu = mobility_idroute0_layout_checksum_cpu(&records);
    let gpu = mobility_idroute0_layout_checksum_gpu_proxy(&records);
    assert_eq!(cpu, gpu);
}
