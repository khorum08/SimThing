//! MOBILITY-ECON-0 session-clearinghouse economy substrate tests.
//!
//! Follows the substrate pattern established by ALLOC-0, REENROLL-0, and
//! IDROUTE-0. ECON-0 remains metadata/testable only: no runtime wiring.

use simthing_spec::designer_admission::{
    mobility_econ0_layout_checksum_cpu, mobility_econ0_layout_checksum_gpu_proxy,
    plan_mobility_econ0, MobilityAlloc0ParentKey, MobilityEcon0ForbiddenPathRequests,
    MobilityEcon0LocalCellRecord, MobilityEcon0PlanInput, MOBILITY_ECON0_ID,
};

fn key(parent_id: u64, key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey { parent_id, key_id }
}

fn rec(
    session: u64,
    cell: u64,
    resource: u64,
    available: i64,
    need: i64,
    soft: f32,
    arrival_order: u64,
) -> MobilityEcon0LocalCellRecord {
    MobilityEcon0LocalCellRecord {
        session_id: session,
        cell_key: key(cell, 0),
        resource_id: resource,
        hard_available: available,
        hard_need: need,
        soft_beta_signal: soft,
        arrival_order,
    }
}

fn input(records: Vec<MobilityEcon0LocalCellRecord>) -> MobilityEcon0PlanInput {
    MobilityEcon0PlanInput {
        records,
        forbidden: MobilityEcon0ForbiddenPathRequests::default(),
    }
}

fn rejected_with(
    forbidden: MobilityEcon0ForbiddenPathRequests,
) -> simthing_spec::designer_admission::MobilityEcon0PlanReport {
    plan_mobility_econ0(&MobilityEcon0PlanInput {
        records: vec![rec(1, 100, 7, 4, 3, 0.25, 0)],
        forbidden,
    })
}

#[test]
fn econ_session_clearinghouse_aggregates_local_cells() {
    let report = plan_mobility_econ0(&input(vec![
        rec(1, 100, 7, 10, 6, 1.0, 9),
        rec(1, 101, 7, 4, 8, 0.5, 2),
    ]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_id, MOBILITY_ECON0_ID);
    assert_eq!(report.session_aggregates.len(), 1);
    let aggregate = &report.session_aggregates[0];
    assert_eq!(aggregate.hard_available, 14);
    assert_eq!(aggregate.hard_need, 14);
    assert_eq!(aggregate.hard_shortfall, 0);
    assert_eq!(aggregate.hard_surplus, 0);
    assert_eq!(aggregate.soft_beta_input, 1.5);
    assert_eq!(report.touched_cell_count, 2);
}

#[test]
fn econ_subsidiarity_balance_conservation() {
    let report = plan_mobility_econ0(&input(vec![
        rec(1, 100, 7, 3, 5, 0.0, 0),
        rec(1, 101, 7, 9, 10, 0.0, 1),
    ]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.conservation_preserved);
    let disbursed = report
        .down_disburses
        .iter()
        .map(|disburse| disburse.hard_amount)
        .sum::<i64>();
    assert_eq!(disbursed, 12);
    assert_eq!(report.session_aggregates[0].hard_shortfall, 3);
}

#[test]
fn econ_hard_band_alpha_before_soft_band_beta() {
    let report = plan_mobility_econ0(&input(vec![rec(1, 100, 7, 10, 6, 0.75, 0)]));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.alpha_finalized_before_beta);
    assert!(report.beta_reads_finalized_alpha);
    assert!(!report.hard_soft_same_pass);
    assert_eq!(report.down_disburses[0].hard_amount, 6);
    assert_eq!(report.down_disburses[0].soft_beta_amount, 6.75);
}

#[test]
fn econ_rejects_hard_soft_silent_mix() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.hard_soft_silent_mix = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"hard_soft_silent_mix"));
}

#[test]
fn econ_deterministic_up_down_disburse() {
    let a = plan_mobility_econ0(&input(vec![
        rec(1, 101, 7, 4, 8, 0.5, 99),
        rec(1, 100, 7, 10, 6, 1.0, 2),
    ]));
    let b = plan_mobility_econ0(&input(vec![
        rec(1, 100, 7, 10, 6, 1.0, 500),
        rec(1, 101, 7, 4, 8, 0.5, 1),
    ]));

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert_eq!(a.session_aggregates, b.session_aggregates);
    assert_eq!(a.down_disburses, b.down_disburses);
    assert_eq!(a.cpu_gpu_parity_checksum, b.cpu_gpu_parity_checksum);
}

#[test]
fn econ_cpu_gpu_parity_layout() {
    let records = vec![rec(1, 100, 7, 10, 5, 0.5, 99)];
    let cpu = mobility_econ0_layout_checksum_cpu(&records);
    let gpu = mobility_econ0_layout_checksum_gpu_proxy(&records);
    assert_eq!(cpu, gpu);
}

#[test]
fn econ_rejects_owner_overlay_runtime() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.owner_overlay_runtime = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"owner_overlay_runtime"));
}

#[test]
fn econ_keeps_owner_parked() {
    let accepted = plan_mobility_econ0(&input(vec![rec(1, 100, 7, 4, 3, 0.25, 0)]));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert!(accepted.owner_parked);

    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.owner_runtime = true;
    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"owner_runtime"));
}

#[test]
fn econ_rejects_default_on_resource_flow() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.default_on_resource_flow = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_resource_flow"));
}

#[test]
fn econ_rejects_hard_currency_through_resource_flow() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.hard_currency_through_resource_flow = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hard_currency_through_resource_flow"));
}

#[test]
fn econ_rejects_float_structural_gate() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.float_structural_gate = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"float_structural_gate"));
}

#[test]
fn econ_rejects_production_simsession_wiring() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.production_simsession_wiring = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(!report.runtime_implementation_authorized);
    assert!(report.diagnostics.contains(&"production_simsession_wiring"));
}

#[test]
fn econ_rejects_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn econ_rejects_cpu_planner_urgency_commitment() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn econ_rejects_owner_as_spatial_parent() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.owner_as_spatial_parent = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"owner_as_spatial_parent"));
}

#[test]
fn econ_rejects_capture_as_reparenting() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.capture_as_reparenting = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"capture_as_reparenting"));
}

#[test]
fn econ_rejects_hybrid_strata_or_faction_index_scaling_layer() {
    let mut forbidden = MobilityEcon0ForbiddenPathRequests::default();
    forbidden.hybrid_strata_or_faction_index_scaling_layer = true;

    let report = rejected_with(forbidden);

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hybrid_strata_or_faction_index_scaling_layer"));
    assert!(report.later_econ_scaling_parked);
}

#[test]
fn econ_multi_cell_clearinghouse_scale() {
    let records = (0..48u64)
        .map(|cell| rec(1, 100 + cell, 7, 2, 1 + (cell % 3) as i64, 0.25, cell))
        .collect();
    let report = plan_mobility_econ0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.boundary_group_count, 1);
    assert!(report.conservation_preserved);
}

#[test]
fn econ_concentration_one_session() {
    let records = (0..1_000u64)
        .map(|i| rec(1, 100 + (i % 48), 7 + (i % 3), 1, 1, 0.1, i))
        .collect();
    let report = plan_mobility_econ0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.touched_session_count, 1);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.touched_resource_count, 3);
}

#[test]
fn econ_scale_soak_34k() {
    let records = (0..34_000u64)
        .map(|i| rec(1 + (i % 2), 100 + (i % 48), 7 + (i % 4), 1, 1, 0.1, i))
        .collect();
    let report = plan_mobility_econ0(&input(records));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.peak_local_records, 34_000);
    assert_eq!(report.touched_cell_count, 48);
    assert_eq!(report.touched_resource_count, 4);
    assert!(report.conservation_preserved);
}
