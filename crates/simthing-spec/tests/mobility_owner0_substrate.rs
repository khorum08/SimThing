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
fn owner_rejects_owner_as_spatial_parent() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.owner_as_spatial_parent = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"owner_as_spatial_parent"));
}

#[test]
fn owner_rejects_capture_as_reparenting() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.capture_as_reparenting = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"capture_as_reparenting"));
}

#[test]
fn owner_rejects_nested_arena_reparenting() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.nested_arena_reparenting = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"nested_arena_reparenting"));
}

#[test]
fn owner_rejects_default_on_resource_flow() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.default_on_resource_flow = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_resource_flow"));
}

#[test]
fn owner_rejects_hard_currency_through_resource_flow() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.hard_currency_through_resource_flow = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hard_currency_through_resource_flow"));
}

#[test]
fn owner_rejects_production_simsession_wiring() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.production_simsession_wiring = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"production_simsession_wiring"));
    assert!(!report.runtime_implementation_authorized);
}

#[test]
fn owner_rejects_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn owner_rejects_cpu_planner_urgency_commitment() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn owner_rejects_hybrid_strata_or_faction_index_scaling_layer() {
    let mut forbidden = MobilityOwner0ForbiddenPathRequests::default();
    forbidden.hybrid_strata_or_faction_index_scaling_layer = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hybrid_strata_or_faction_index_scaling_layer"));
    assert!(report.later_econ_scaling_parked);
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

#[test]
fn owner_scale_soak_34k() {
    let records = (0..34_000u64)
        .map(|i| {
            rec(
                100_000 + i,
                100 + (i % 48),
                1,
                vec![
                    owner(MobilityOwner0ColumnKind::Faction, 7 + (i % 4)),
                    owner(MobilityOwner0ColumnKind::Species, 20 + (i % 3)),
                ],
                i % 5,
                i % 97 == 0,
                i,
            )
        })
        .collect();
    let overlays = vec![
        overlay(MobilityOwner0ColumnKind::Faction, 7, 1, 1, 0),
        overlay(MobilityOwner0ColumnKind::Faction, 8, 1, 1, 1),
        overlay(MobilityOwner0ColumnKind::Faction, 9, 1, 1, 2),
        overlay(MobilityOwner0ColumnKind::Faction, 10, 1, 1, 3),
        overlay(MobilityOwner0ColumnKind::Species, 20, 2, 1, 4),
        overlay(MobilityOwner0ColumnKind::Species, 21, 2, 1, 5),
        overlay(MobilityOwner0ColumnKind::Species, 22, 2, 1, 6),
    ];
    let report = plan_mobility_owner0(&input(records, overlays));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.peak_local_records, 34_000);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.touched_owner_count, 7);
    assert_eq!(report.applied_overlays.len(), 68_000);
}
