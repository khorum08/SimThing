//! MOBILITY-OWNER-0 owner-relations + latched modifier overlay tests.
//!
//! OWNER-0 remains metadata/testable only: no production runtime wiring.

use simthing_spec::designer_admission::{
    audit_mobility_owner_band_budget, mobility_owner0_layout_checksum_cpu,
    mobility_owner0_layout_checksum_gpu_proxy, mobility_scenario0_packet, plan_mobility_owner0,
    MobilityAlloc0ParentKey, MobilityAudit0Verdict, MobilityOwner0ColumnKind,
    MobilityOwner0ColumnValue, MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord,
    MobilityOwner0Overlay, MobilityOwner0OwnerChange, MobilityOwner0PlanInput,
    MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH, MOBILITY_OWNER0_ID,
    MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH,
};

fn key(parent_id: u64, key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey { parent_id, key_id }
}

fn owner(kind: MobilityOwner0ColumnKind, owner_id: u64) -> MobilityOwner0ColumnValue {
    MobilityOwner0ColumnValue { kind, owner_id }
}

fn rec(
    entity_id: u64,
    cell_id: u64,
    cohort_count: u32,
    owners: Vec<MobilityOwner0ColumnValue>,
    generation: u64,
    blocked_by_blockade: bool,
    arrival_order: u64,
) -> MobilityOwner0LocalRecord {
    MobilityOwner0LocalRecord {
        entity_id,
        cell_key: key(cell_id, 0),
        cohort_count,
        owner_columns: owners,
        generation,
        blocked_by_blockade,
        arrival_order,
    }
}

fn overlay(
    kind: MobilityOwner0ColumnKind,
    owner_id: u64,
    modifier_id: u64,
    modifier_amount: i64,
    arrival_order: u64,
) -> MobilityOwner0Overlay {
    MobilityOwner0Overlay {
        owner: owner(kind, owner_id),
        modifier_id,
        modifier_amount,
        arrival_order,
    }
}

fn input(
    records: Vec<MobilityOwner0LocalRecord>,
    overlays: Vec<MobilityOwner0Overlay>,
) -> MobilityOwner0PlanInput {
    MobilityOwner0PlanInput {
        records,
        overlays,
        owner_changes: vec![],
        forbidden: MobilityOwner0ForbiddenPathRequests::default(),
    }
}

fn rejected_with(
    forbidden: MobilityOwner0ForbiddenPathRequests,
) -> simthing_spec::designer_admission::MobilityOwner0PlanReport {
    plan_mobility_owner0(&MobilityOwner0PlanInput {
        records: vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            0,
            false,
            0,
        )],
        overlays: vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 5, 0)],
        owner_changes: vec![],
        forbidden,
    })
}

#[test]
fn owner_column_overlay_applies_deterministically() {
    let a = plan_mobility_owner0(&input(
        vec![
            rec(
                2,
                101,
                1,
                vec![owner(MobilityOwner0ColumnKind::Species, 8)],
                0,
                false,
                99,
            ),
            rec(
                1,
                100,
                1,
                vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                0,
                false,
                1,
            ),
        ],
        vec![
            overlay(MobilityOwner0ColumnKind::Species, 8, 2, 4, 2),
            overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 5, 99),
        ],
    ));
    let b = plan_mobility_owner0(&input(
        vec![
            rec(
                1,
                100,
                1,
                vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                0,
                false,
                500,
            ),
            rec(
                2,
                101,
                1,
                vec![owner(MobilityOwner0ColumnKind::Species, 8)],
                0,
                false,
                0,
            ),
        ],
        vec![
            overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 5, 1000),
            overlay(MobilityOwner0ColumnKind::Species, 8, 2, 4, 1),
        ],
    ));

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert_eq!(a.substrate_id, MOBILITY_OWNER0_ID);
    assert_eq!(a.applied_overlays, b.applied_overlays);
    assert_eq!(a.cpu_gpu_parity_checksum, b.cpu_gpu_parity_checksum);
}

#[test]
fn owner_cpu_gpu_parity_layout() {
    let records = vec![rec(
        1,
        100,
        10,
        vec![
            owner(MobilityOwner0ColumnKind::Faction, 7),
            owner(MobilityOwner0ColumnKind::Species, 8),
        ],
        0,
        false,
        99,
    )];
    let cpu = mobility_owner0_layout_checksum_cpu(&records);
    let gpu = mobility_owner0_layout_checksum_gpu_proxy(&records);
    assert_eq!(cpu, gpu);
}
