//! MOBILITY-AUDIT-0 owner OrderBand depth budget tests.

use simthing_spec::{
    audit_mobility_owner_band_budget, audit_mobility_owner_band_budget_with_ceiling,
    mobility_audit0_packet_matches_accepted_constants, mobility_audit0_required_orderband_depth,
    mobility_scenario0_packet, MobilityAudit0CirculationFamily, MobilityAudit0Verdict,
    MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH, MOBILITY_AUDIT0_ID,
};

#[test]
fn mobility_audit0_accepts_current_first_slice_if_depth_fits() {
    let packet = mobility_scenario0_packet();
    let report = audit_mobility_owner_band_budget(&packet);

    assert_eq!(report.audit_id, MOBILITY_AUDIT0_ID);
    assert_eq!(report.verdict, MobilityAudit0Verdict::Pass);
    assert_eq!(report.verdict.as_str(), "PASS");
    assert_eq!(report.required_orderband_depth, 13);
    assert_eq!(
        report.max_orderband_depth,
        MOBILITY_AUDIT0_CURRENT_MAX_ORDERBAND_DEPTH
    );
    assert_eq!(report.slack_orderbands, 3);
    assert!(mobility_audit0_packet_matches_accepted_constants(&packet));
}

#[test]
fn mobility_audit0_reports_required_depth_for_all_circulations() {
    let packet = mobility_scenario0_packet();
    let report = audit_mobility_owner_band_budget(&packet);

    assert_eq!(mobility_audit0_required_orderband_depth(&packet), 13);
    assert_eq!(report.family_budgets.len(), 7);
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::ModifierDown && budget.required_bands == 1
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::EconomyUp && budget.required_bands == 3
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::EconomyDown && budget.required_bands == 3
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::ResearchUp && budget.required_bands == 3
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::Thresholds && budget.required_bands == 1
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::HardFixedPointBandAlpha
            && budget.required_bands == 1
    }));
    assert!(report.family_budgets.iter().any(|budget| {
        budget.family == MobilityAudit0CirculationFamily::SoftFloatBandBeta
            && budget.required_bands == 1
    }));
}

#[test]
fn mobility_audit0_rejects_or_narrows_when_depth_exceeds_ceiling() {
    let packet = mobility_scenario0_packet();
    let narrowed = audit_mobility_owner_band_budget_with_ceiling(&packet, 12);
    assert_eq!(narrowed.verdict, MobilityAudit0Verdict::PassWithNarrowing);
    assert_eq!(narrowed.verdict.as_str(), "PASS WITH NARROWING");
    assert!(narrowed.narrowing.is_some());

    let blocked = audit_mobility_owner_band_budget_with_ceiling(&packet, 7);
    assert_eq!(blocked.verdict, MobilityAudit0Verdict::FailBlocked);
    assert_eq!(blocked.verdict.as_str(), "FAIL-BLOCKED");
}

#[test]
fn mobility_audit0_keeps_alloc_reenroll_idroute_econ_owner_parked() {
    let report = audit_mobility_owner_band_budget(&mobility_scenario0_packet());
    assert!(report.alloc_reenroll_idroute_econ_owner_parked);
}

#[test]
fn mobility_audit0_does_not_authorize_runtime_implementation() {
    let report = audit_mobility_owner_band_budget(&mobility_scenario0_packet());
    assert!(!report.runtime_implementation_authorized);
}

#[test]
fn mobility_audit0_alpha_precedes_beta() {
    let report = audit_mobility_owner_band_budget(&mobility_scenario0_packet());
    let alpha_index = report
        .family_budgets
        .iter()
        .position(|budget| {
            budget.family == MobilityAudit0CirculationFamily::HardFixedPointBandAlpha
        })
        .expect("alpha");
    let beta_index = report
        .family_budgets
        .iter()
        .position(|budget| budget.family == MobilityAudit0CirculationFamily::SoftFloatBandBeta)
        .expect("beta");

    assert!(report.alpha_precedes_beta);
    assert!(alpha_index < beta_index);
}

#[test]
fn mobility_audit0_no_hard_soft_silent_mix() {
    let report = audit_mobility_owner_band_budget(&mobility_scenario0_packet());
    assert!(!report.hard_soft_silent_mix);
}

#[test]
fn mobility_audit0_no_owner_spatial_parent_assumption() {
    let report = audit_mobility_owner_band_budget(&mobility_scenario0_packet());
    assert!(!report.owner_spatial_parent_assumption);
}
