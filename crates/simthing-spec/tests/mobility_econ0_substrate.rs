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
