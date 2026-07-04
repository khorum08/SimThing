use simthing_driver::{
    replay_econ_scale_0080_0, run_econ_scale_0080_0, EconScale0080Faction,
    EconScale0080ForbiddenRequests, EconScale0080Input, ECON_SCALE_0080_0_FACTION_COUNT,
    ECON_SCALE_0080_0_ID, ECON_SCALE_0080_0_SCENARIO, ECON_SCALE_0080_0_STATUS_PASS,
};

fn report() -> simthing_driver::EconScale0080RunReport {
    run_econ_scale_0080_0(&EconScale0080Input::explicit_opt_in())
}

fn rejected_with(
    mutate: impl FnOnce(&mut EconScale0080ForbiddenRequests),
) -> simthing_driver::EconScale0080RunReport {
    let mut input = EconScale0080Input::explicit_opt_in();
    mutate(&mut input.forbidden);
    run_econ_scale_0080_0(&input)
}

#[test]
fn econ_scale_0080_0_adversarial_contended_clearing_deterministic() {
    let admitted = report();
    assert!(admitted.adversarial_contended_clearing);
    let contended = admitted
        .clearing_reports
        .iter()
        .find(|r| r.adversarial && r.terran_present && r.pirate_present)
        .expect("a contended starsystem");
    // Pirate adversarial pressure drains supply, raises contention and disruption.
    assert!(contended.supply_after < contended.supply_before);
    assert!(contended.contention_after > contended.contention_before);
    assert!(contended.disruption_after > contended.disruption_before);
    assert!(contended.pirate_extraction > 0);

    // Deterministic: re-running yields an identical report.
    let again = report();
    assert_eq!(admitted, again);
}

#[test]
fn econ_scale_0080_0_parity_bit_exact() {
    let admitted = report();
    assert!(admitted.parity_bit_exact);
    assert!(admitted.clearing_reports.iter().all(|r| r.parity_bit_exact));
}

#[test]
fn econ_scale_0080_0_replay_deterministic() {
    let (a, b) = replay_econ_scale_0080_0();
    assert_eq!(a, b);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

// --- scenario-lock tests ---
