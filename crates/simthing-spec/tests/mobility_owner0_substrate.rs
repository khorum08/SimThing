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
fn owner_capture_is_column_flip_not_reparenting() {
    let report = plan_mobility_owner0(&MobilityOwner0PlanInput {
        records: vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            3,
            false,
            0,
        )],
        overlays: vec![],
        owner_changes: vec![MobilityOwner0OwnerChange {
            entity_id: 1,
            kind: MobilityOwner0ColumnKind::Faction,
            from_owner_id: 7,
            to_owner_id: 9,
            changed_count: 10,
            new_entity_id: None,
            capture: true,
            arrival_order: 0,
        }],
        forbidden: MobilityOwner0ForbiddenPathRequests::default(),
    });

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.generation_resyncs.len(), 1);
    assert_eq!(report.generation_resyncs[0].old_generation, 3);
    assert_eq!(report.generation_resyncs[0].new_generation, 4);
    assert_eq!(
        report.generation_resyncs[0].changed_owner,
        owner(MobilityOwner0ColumnKind::Faction, 9)
    );
    assert!(!report.capture_reparented);
    assert!(!report.owner_columns_are_spatial_parents);
}

#[test]
fn owner_latched_modifier_overlay_persists() {
    let report = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Tech, 5)],
            0,
            false,
            0,
        )],
        vec![overlay(MobilityOwner0ColumnKind::Tech, 5, 42, 11, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.applied_overlays.len(), 1);
    assert_eq!(report.applied_overlays[0].modifier_amount, 11);
    assert_eq!(report.dirtyonly_noop_count, 1);
}

#[test]
fn owner_blockade_immune_modifier_stays_latched() {
    let report = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Species, 8)],
            0,
            true,
            0,
        )],
        vec![overlay(MobilityOwner0ColumnKind::Species, 8, 1, 3, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.applied_overlays.len(), 1);
    assert!(report.applied_overlays[0].blocked_by_blockade);
    assert!(!report.blockade_dropped_latched_modifier);
}

#[test]
fn owner_down_broadcast_does_not_spawn_arena_columns() {
    let report = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Blueprint, 3)],
            0,
            false,
            0,
        )],
        vec![overlay(MobilityOwner0ColumnKind::Blueprint, 3, 1, 2, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.spawned_arena_column_count, 0);
    assert_eq!(report.spawned_aggregation_column_count, 0);
}

#[test]
fn owner_down_broadcast_reaches_every_owned_including_isolated() {
    let faction_owner = owner(MobilityOwner0ColumnKind::Faction, 7);
    let records = vec![
        rec(1, 100, 16, vec![faction_owner], 0, false, 10),
        rec(2, 801, 1, vec![faction_owner], 0, false, 0),
        rec(
            3,
            802,
            1,
            vec![owner(MobilityOwner0ColumnKind::Species, 7)],
            0,
            false,
            1,
        ),
    ];
    let overlays = vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 42, 11, 0)];
    let steady_report = plan_mobility_owner0(&input(records.clone(), overlays.clone()));

    assert!(steady_report.admitted, "{:?}", steady_report.diagnostics);
    assert_eq!(steady_report.applied_overlays.len(), 2);
    assert!(steady_report
        .applied_overlays
        .iter()
        .any(|applied| applied.entity_id == 1 && applied.cell_key == key(100, 0)));
    assert!(steady_report
        .applied_overlays
        .iter()
        .any(|applied| applied.entity_id == 2
            && applied.cell_key == key(801, 0)
            && applied.owner == faction_owner
            && applied.modifier_id == 42
            && applied.modifier_amount == 11));
    assert!(!steady_report
        .applied_overlays
        .iter()
        .any(|applied| applied.entity_id == 3));
    assert_eq!(steady_report.spawned_arena_column_count, 0);
    assert_eq!(steady_report.spawned_aggregation_column_count, 0);
    assert!(!steady_report.owner_columns_are_spatial_parents);
    assert!(!steady_report.capture_reparented);
    assert_eq!(steady_report.modifier_dispersal_count, 0);
    assert_eq!(steady_report.dirtyonly_noop_count, 2);

    let dirty_report = plan_mobility_owner0(&MobilityOwner0PlanInput {
        records,
        overlays,
        owner_changes: vec![MobilityOwner0OwnerChange {
            entity_id: 1,
            kind: MobilityOwner0ColumnKind::Faction,
            from_owner_id: 7,
            to_owner_id: 9,
            changed_count: 16,
            new_entity_id: None,
            capture: false,
            arrival_order: 0,
        }],
        forbidden: MobilityOwner0ForbiddenPathRequests::default(),
    });

    assert!(dirty_report.admitted, "{:?}", dirty_report.diagnostics);
    assert_eq!(dirty_report.applied_overlays.len(), 2);
    assert_eq!(dirty_report.modifier_dispersal_count, 2);
    assert_eq!(dirty_report.dirtyonly_noop_count, 0);
    assert_eq!(dirty_report.spawned_arena_column_count, 0);
    assert_eq!(dirty_report.spawned_aggregation_column_count, 0);
}

#[test]
fn owner_generation_resync_on_owner_column_change() {
    let report = plan_mobility_owner0(&MobilityOwner0PlanInput {
        records: vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            2,
            false,
            0,
        )],
        overlays: vec![],
        owner_changes: vec![MobilityOwner0OwnerChange {
            entity_id: 1,
            kind: MobilityOwner0ColumnKind::Faction,
            from_owner_id: 7,
            to_owner_id: 9,
            changed_count: 10,
            new_entity_id: None,
            capture: false,
            arrival_order: 0,
        }],
        forbidden: MobilityOwner0ForbiddenPathRequests::default(),
    });

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.generation_resyncs.len(), 1);
    assert!(report.generation_resyncs[0].no_silent_rebind);
    assert_eq!(report.generation_resyncs[0].new_generation, 3);
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

#[test]
fn owner_cohort_homogeneity_via_fission() {
    let report = plan_mobility_owner0(&MobilityOwner0PlanInput {
        records: vec![rec(
            1,
            100,
            10,
            vec![
                owner(MobilityOwner0ColumnKind::Faction, 7),
                owner(MobilityOwner0ColumnKind::Species, 8),
            ],
            0,
            false,
            0,
        )],
        overlays: vec![],
        owner_changes: vec![MobilityOwner0OwnerChange {
            entity_id: 1,
            kind: MobilityOwner0ColumnKind::Faction,
            from_owner_id: 7,
            to_owner_id: 9,
            changed_count: 4,
            new_entity_id: Some(2),
            capture: false,
            arrival_order: 0,
        }],
        forbidden: MobilityOwner0ForbiddenPathRequests::default(),
    });

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.fission_results.len(), 1);
    let fission = &report.fission_results[0];
    assert_eq!(fission.retained_count, 6);
    assert_eq!(fission.fission_count, 4);
    assert!(fission
        .retained_owner_columns
        .contains(&owner(MobilityOwner0ColumnKind::Faction, 7)));
    assert!(fission
        .fission_owner_columns
        .contains(&owner(MobilityOwner0ColumnKind::Faction, 9)));
}

#[test]
fn owner_keeps_production_runtime_integration_parked() {
    let accepted = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            0,
            false,
            0,
        )],
        vec![],
    ));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert!(accepted.production_runtime_integration_parked);

    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.production_runtime_integration = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"production_runtime_integration"));
}

#[test]
fn owner_overlay_multi_cell_scale() {
    let records = (0..48u64)
        .map(|cell| {
            rec(
                1_000 + cell,
                100 + cell,
                16,
                vec![owner(MobilityOwner0ColumnKind::Faction, 7 + (cell % 4))],
                0,
                false,
                cell,
            )
        })
        .collect();
    let overlays = (0..4u64)
        .map(|id| overlay(MobilityOwner0ColumnKind::Faction, 7 + id, id, 2, id))
        .collect();

    let report = plan_mobility_owner0(&input(records, overlays));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.applied_overlays.len(), 48);
}

#[test]
fn owner_concentration_one_owner() {
    let records = (0..1_000u64)
        .map(|i| {
            rec(
                10_000 + i,
                100 + (i % 48),
                1,
                vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                0,
                false,
                i,
            )
        })
        .collect();
    let report = plan_mobility_owner0(&input(
        records,
        vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 1, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_owner_count, 1);
    assert_eq!(report.applied_overlays.len(), 1_000);
}

#[test]
fn owner_dirtyonly_amortized() {
    let report = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            0,
            false,
            0,
        )],
        vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 5, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.modifier_dispersal_count, 0);
    assert_eq!(report.dirtyonly_noop_count, 1);
}

#[test]
fn owner_band_budget_audit() {
    let packet = mobility_scenario0_packet();
    let audit = audit_mobility_owner_band_budget(&packet);
    let report = plan_mobility_owner0(&input(
        vec![rec(
            1,
            100,
            10,
            vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
            0,
            false,
            0,
        )],
        vec![],
    ));

    assert_eq!(audit.verdict, MobilityAudit0Verdict::Pass);
    assert!(report.owner_band_budget_preserved);
    assert_eq!(
        report.required_orderband_depth,
        MOBILITY_OWNER0_REQUIRED_ORDERBAND_DEPTH
    );
    assert_eq!(
        report.max_orderband_depth,
        MOBILITY_OWNER0_CURRENT_MAX_ORDERBAND_DEPTH
    );
    assert!(report.required_orderband_depth <= report.max_orderband_depth);
}
