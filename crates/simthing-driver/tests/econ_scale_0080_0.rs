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
fn econ_scale_0080_0_explicit_opt_in_only() {
    let disabled = run_econ_scale_0080_0(&EconScale0080Input::default_simsession());
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(disabled.clearing_reports.is_empty());

    let mut default_on = EconScale0080Input::explicit_opt_in();
    default_on.surface.gate.enabled_by_default = true;
    let rejected = run_econ_scale_0080_0(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"econ_scale_0080_0_default_on_behavior_rejected"));

    let admitted = report();
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.explicit_opt_in);
    assert!(admitted.default_off);
    assert!(!admitted.disabled_no_op);
}

#[test]
fn econ_scale_0080_0_default_path_single_owner_unchanged() {
    let disabled = run_econ_scale_0080_0(&EconScale0080Input::default_simsession());
    assert!(disabled.single_owner_default_unchanged);
    assert!(disabled.disabled_no_op);
    assert!(disabled.clearing_reports.is_empty());
    // The opt-in surface also leaves the default ECON path conceptually single-owner (scenario-scoped).
    assert!(report().single_owner_default_unchanged);
}

#[test]
fn econ_scale_0080_0_bounded_fixed_faction_count() {
    let admitted = report();
    assert!(admitted.bounded_faction_count);
    assert_eq!(admitted.faction_count, ECON_SCALE_0080_0_FACTION_COUNT);
    assert_eq!(admitted.faction_count, 2);

    let rejected = rejected_with(|f| f.unbounded_faction_fanout = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"unbounded_faction_fanout"));
}

#[test]
fn econ_scale_0080_0_faction_indexed_participation() {
    let admitted = report();
    assert!(admitted.faction_indexed_participation);
    // At least one starsystem distinguishes both faction indices (0 = Terran, 1 = Pirate).
    let both = admitted
        .clearing_reports
        .iter()
        .find(|r| r.terran_present && r.pirate_present)
        .expect("a contended starsystem");
    assert!(both.faction_indices_present.contains(&0));
    assert!(both.faction_indices_present.contains(&1));
    assert!(both.terran_extraction >= 0 && both.pirate_extraction >= 0);
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

#[test]
fn econ_scale_0080_0_no_hard_currency_markets_trade_aibudget() {
    let admitted = report();
    assert!(!admitted.hard_currency_markets_trade_aibudget);

    let hard = rejected_with(|f| f.hard_currency = true);
    assert!(!hard.admitted);
    assert!(hard.diagnostics.contains(&"hard_currency"));

    let markets = rejected_with(|f| f.markets_trade_aibudget = true);
    assert!(!markets.admitted);
    assert!(markets.diagnostics.contains(&"markets_trade_aibudget"));
}

#[test]
fn econ_scale_0080_0_no_nested_resource_flow() {
    let admitted = report();
    assert!(!admitted.nested_resource_flow);
    assert!(admitted.flat_star_posture_preserved);

    let rejected = rejected_with(|f| f.nested_resource_flow = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"nested_resource_flow"));
}

#[test]
fn econ_scale_0080_0_subsidiarity_preserved() {
    let admitted = report();
    assert!(admitted.subsidiarity_preserved);

    let rejected = rejected_with(|f| f.replace_subsidiarity = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"replace_subsidiarity"));
}

#[test]
fn econ_scale_0080_0_no_semantic_or_raw_wgsl() {
    let admitted = report();
    assert!(!admitted.semantic_or_raw_wgsl_present);

    let rejected = rejected_with(|f| f.semantic_or_raw_wgsl = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));

    let shader = rejected_with(|f| f.semantically_named_shader = true);
    assert!(!shader.admitted);
    assert!(shader.diagnostics.contains(&"semantically_named_shader"));
}

#[test]
fn econ_scale_0080_0_no_clausething_dependency() {
    let admitted = report();
    assert!(!admitted.clausething_dependency_present);

    let rejected = rejected_with(|f| f.clausething_dependency = true);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"clausething_dependency"));
}

#[test]
fn econ_scale_0080_0_docs_status_matches_gate() {
    let admitted = report();
    assert_eq!(admitted.econ_scale_id, ECON_SCALE_0080_0_ID);
    assert_eq!(admitted.status, ECON_SCALE_0080_0_STATUS_PASS);
    assert_eq!(admitted.scenario_name, ECON_SCALE_0080_0_SCENARIO);
    assert!(!admitted.production_path_0080_1_implemented);
}

// --- scenario-lock tests ---

#[test]
fn econ_scale_0080_0_pirate_is_full_economy_faction() {
    let admitted = report();
    assert!(admitted.pirate_is_full_economy_faction);
    assert!(EconScale0080Faction::Pirate.is_full_economy_faction());
    // The pirate actually extracts (full economy participant), not merely disrupts.
    assert!(admitted
        .clearing_reports
        .iter()
        .any(|r| r.pirate_present && r.pirate_extraction > 0));
}

#[test]
fn econ_scale_0080_0_pirate_enters_starsystem_as_adversarial_participant() {
    let admitted = report();
    let entered = admitted
        .clearing_reports
        .iter()
        .find(|r| r.pirate_present)
        .expect("a starsystem with a pirate participant");
    assert!(entered.adversarial);
    assert!(entered.faction_indices_present.contains(&1));
    // A neutral starsystem the pirate entered still clears pirate participation (owns no star there).
    assert!(admitted
        .clearing_reports
        .iter()
        .any(|r| !r.terran_owned && r.pirate_present && r.pirate_extraction > 0));
}

#[test]
fn econ_scale_0080_0_terran_and_pirate_factions_are_bounded() {
    let admitted = report();
    assert_eq!(admitted.faction_count, 2);
    assert!(admitted.factions.contains(&EconScale0080Faction::Terran));
    assert!(admitted.factions.contains(&EconScale0080Faction::Pirate));
    assert!(admitted.bounded_faction_count);
}
