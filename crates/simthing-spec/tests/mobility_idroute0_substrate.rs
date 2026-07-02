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
fn idroute_masked_sum_correct() {
    let report = plan_mobility_idroute0(&input(vec![
        rec(1, 100, 0, 10, 1.0),
        rec(2, 100, 0, 20, 2.0),
        rec(3, 100, 1, 5, 0.5),
    ]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_id, MOBILITY_IDROUTE0_ID);
    let sum0 = report
        .per_identity_sums
        .iter()
        .find(|s| s.identity.0 == 0)
        .unwrap();
    assert_eq!(sum0.hard_sum, 30);
    assert_eq!(sum0.soft_sum, 3.0);
}

#[test]
fn idroute_multi_term_sum_determinism() {
    let request = input(vec![rec(2, 100, 1, 3, 0.0), rec(1, 100, 0, 7, 0.0)]);

    let a = plan_mobility_idroute0(&request);
    let b = plan_mobility_idroute0(&request);

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert_eq!(a.per_identity_sums, b.per_identity_sums);
    assert_eq!(a.cpu_gpu_parity_checksum, b.cpu_gpu_parity_checksum);
}

#[test]
fn idroute_argmax_packed_key_unique() {
    let report = plan_mobility_idroute0(&input(vec![
        rec(10, 100, 0, 100, 0.0),
        rec(20, 100, 0, 100, 0.0),
        rec(30, 100, 1, 50, 0.0),
    ]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.argmax_winner, Some((IdentityLane(0), 20)));
}

#[test]
fn idroute_directed_disburse_correct() {
    let report =
        plan_mobility_idroute0(&input(vec![rec(1, 100, 0, 8, 2.0), rec(2, 100, 1, 6, 4.0)]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.directed_disburses.len(), 2);
    assert!(report
        .directed_disburses
        .iter()
        .any(|d| d.target_entity_id == 1 && d.hard_amount == 4 && d.soft_amount == 1.0));
}

#[test]
fn idroute_directed_disburse_atomic_or_immutable() {
    let accepted =
        plan_mobility_idroute0(&input(vec![rec(1, 100, 0, 4, 0.0), rec(2, 100, 1, 6, 0.0)]));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert!(accepted.directed_disburse_immutable);
    assert!(!accepted.runtime_implementation_authorized);

    let rejected = plan_mobility_idroute0(&MobilityIdroute0PlanInput {
        records: vec![rec(1, 100, 4, 4, 0.0)],
        max_factions_per_cell: 4,
        forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
    });
    assert!(!rejected.admitted);
    assert!(rejected.directed_disburses.is_empty());
}

#[test]
fn idroute_identity_column_not_tree_structure() {
    let report = plan_mobility_idroute0(&input(vec![rec(1, 100, 0, 1, 0.0)]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.local_d2_cell_admission);
    assert!(report.identity_lanes_are_local_columns);
}

#[test]
fn idroute_cpu_gpu_parity_layout() {
    let records = vec![rec(1, 100, 0, 5, 0.0)];
    let cpu = mobility_idroute0_layout_checksum_cpu(&records);
    let gpu = mobility_idroute0_layout_checksum_gpu_proxy(&records);
    assert_eq!(cpu, gpu);
}

#[test]
fn idroute_accepts_many_cells_with_local_k_bound() {
    let mut records = Vec::new();
    for cell in 0..48u64 {
        for lane in 0..4u32 {
            records.push(rec(
                1_000 + cell * 10 + lane as u64,
                100 + cell,
                lane,
                1,
                0.25,
            ));
        }
    }

    let report = plan_mobility_idroute0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.max_local_identities_used, 4);
    assert!(report.unique_identities_used <= 4);
}

#[test]
fn idroute_keeps_econ_owner_parked() {
    let mut forbidden = MobilityIdroute0ForbiddenPathRequests::default();
    forbidden.econ_owner_runtime = true;

    let report = plan_mobility_idroute0(&MobilityIdroute0PlanInput {
        records: vec![rec(1, 100, 0, 1, 0.0)],
        max_factions_per_cell: 4,
        forbidden,
    });

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"econ_owner_runtime"));
}

#[test]
fn idroute_does_not_authorize_production_simsession_wiring() {
    let mut forbidden = MobilityIdroute0ForbiddenPathRequests::default();
    forbidden.production_simsession_wiring = true;

    let report = plan_mobility_idroute0(&MobilityIdroute0PlanInput {
        records: vec![rec(1, 100, 0, 1, 0.0)],
        max_factions_per_cell: 4,
        forbidden,
    });

    assert!(!report.admitted);
    assert!(!report.runtime_implementation_authorized);
}

#[test]
fn idroute_does_not_enable_default_on_behavior() {
    let mut forbidden = MobilityIdroute0ForbiddenPathRequests::default();
    forbidden.default_on_behavior = true;

    let report = plan_mobility_idroute0(&MobilityIdroute0PlanInput {
        records: vec![rec(1, 100, 0, 1, 0.0)],
        max_factions_per_cell: 4,
        forbidden,
    });

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_behavior"));
}

#[test]
fn idroute_d2_masked_dispatch_scale() {
    let records = (0..48u64)
        .flat_map(|cell| {
            (0..2u32)
                .map(move |lane| rec(10_000 + cell * 10 + lane as u64, 100 + cell, lane, 3, 0.5))
        })
        .collect();
    let report = plan_mobility_idroute0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.max_local_identities_used, 2);
}

#[test]
fn idroute_concentration_one_cell() {
    let records = (0..4u32)
        .flat_map(|lane| {
            (0..250u64).map(move |i| rec(20_000 + lane as u64 * 1_000 + i, 100, lane, 1, 0.1))
        })
        .collect();
    let report = plan_mobility_idroute0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_cell_count, 1);
    assert_eq!(report.max_local_identities_used, 4);
}
