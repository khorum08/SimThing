use simthing_spec::{
    admit_v7_8_line_scenario_pack, deserialize_v7_8_line_scenario_pack_ron,
    serialize_v7_8_line_scenario_pack_ron, v7_8_met_consumer_scenario_pack,
    DesignerAdmissionDiagnosticCode, V78LineGateStatus, V78LineScenario, V78LineScenarioClaim,
    V78PromotedLine,
};

fn pack() -> simthing_spec::V78LineScenarioPack {
    v7_8_met_consumer_scenario_pack()
}

fn scenario(line: V78PromotedLine) -> simthing_spec::V78NamedConsumerScenario {
    pack()
        .scenario_for_line(line)
        .expect("line scenario exists")
        .clone()
}

#[test]
fn v7_8_met_scenario_pack_names_all_promoted_lines() {
    let pack = pack();
    let ron = serialize_v7_8_line_scenario_pack_ron(&pack).unwrap();
    let parsed = deserialize_v7_8_line_scenario_pack_ron(&ron).unwrap();
    let admission = admit_v7_8_line_scenario_pack(&parsed);

    assert!(admission.admitted, "{:?}", admission.diagnostics);
    assert_eq!(parsed.scenarios.len(), 3);
    assert!(parsed.scenario_for_line(V78PromotedLine::LineA).is_some());
    assert!(parsed.scenario_for_line(V78PromotedLine::LineB).is_some());
    assert!(parsed.scenario_for_line(V78PromotedLine::LineC).is_some());
}

#[test]
fn v7_8_line_a_nested_resource_flow_scenario_matches_constitution_gate() {
    let scenario = scenario(V78PromotedLine::LineA);
    assert_eq!(
        scenario.scenario,
        V78LineScenario::NestedResourceFlowDepthFanout
    );
    assert_eq!(scenario.status, V78LineGateStatus::NamedScenarioProposed);
    assert_eq!(scenario.promoted_line.promoted_from(), "E-11B / E-11B-5");

    let V78LineScenarioClaim::NestedResourceFlowDepthFanout(claim) = scenario.claim else {
        panic!("wrong claim kind");
    };
    assert_eq!(claim.faction_count, 1);
    assert_eq!(claim.planet_count, 100);
    assert_eq!(claim.district_count, 1000);
    assert_eq!(claim.factory_count, 100000);
    assert!(claim.depth_required > 2);
    assert!(claim.flat_star_insufficient);
    assert!(claim.requires_nested_resource_flow);
}

#[test]
fn v7_8_line_b_hard_currency_scenario_matches_constitution_gate() {
    let scenario = scenario(V78PromotedLine::LineB);
    assert_eq!(
        scenario.scenario,
        V78LineScenario::HardCurrencyContentionOrdering
    );
    assert_eq!(scenario.status, V78LineGateStatus::NamedScenarioProposed);
    assert_eq!(scenario.promoted_line.promoted_from(), "D-2 / D-2a");

    let V78LineScenarioClaim::HardCurrencyContentionOrdering(claim) = scenario.claim else {
        panic!("wrong claim kind");
    };
    assert!(claim.multi_transaction_workload);
    assert!(claim.requires_sequential_cross_band_ordering);
    assert!(claim.discrete_accumulator_path_insufficient_at_scale);
    assert!(claim.contention_scale_declared);
    assert!(claim.boundary_or_hot_pool_contention_declared);
}

#[test]
fn v7_8_line_c_multi_theater_scenario_matches_constitution_gate() {
    let scenario = scenario(V78PromotedLine::LineC);
    assert_eq!(scenario.scenario, V78LineScenario::MultiTheaterAtlasMapping);
    assert_eq!(scenario.status, V78LineGateStatus::NamedScenarioProposed);
    assert_eq!(scenario.promoted_line.promoted_from(), "M-4 / M-4A");

    let V78LineScenarioClaim::MultiTheaterAtlasMapping(claim) = scenario.claim else {
        panic!("wrong claim kind");
    };
    assert!(claim.theater_count > 1);
    assert!(claim.single_32x32_theater_insufficient);
    assert!(claim.requires_atlas_batching);
    assert!(claim.vram_budget_declared);
    // VRAM budget: 1.5 GiB default ceiling, configurable, no architectural hard cap.
    assert_eq!(
        claim.vram_budget.max_bytes,
        simthing_spec::V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES
    );
    assert_eq!(claim.vram_budget.max_bytes, 1_610_612_736);
    assert!(claim.vram_budget.configurable);
    assert!(!claim.vram_budget.architectural_hard_cap);
    assert!(claim.vram_budget.multiplier_reporting_required);
    assert_eq!(claim.preferred_isolation, "AlgebraicTileLocalMaskG0");
    assert_eq!(claim.fallback_isolation, "PhysicalGutterGteH");
    assert!(claim.requires_full_tile_protocol_oracle_parity);
}

#[test]
fn v7_8_scenarios_do_not_authorize_implementation() {
    let admission = admit_v7_8_line_scenario_pack(&pack());
    assert!(admission.admitted, "{:?}", admission.diagnostics);

    for status in admission.line_statuses {
        assert_eq!(status.status, V78LineGateStatus::NamedScenarioProposed);
        assert!(!status.implementation_authorized);
    }
}

#[test]
fn v7_8_scenarios_keep_nested_e11b_d2a_and_atlas_rejected_until_acceptance() {
    let a = scenario(V78PromotedLine::LineA);
    let b = scenario(V78PromotedLine::LineB);
    let c = scenario(V78PromotedLine::LineC);

    assert!(a.still_rejected_until_acceptance.contains(
        &DesignerAdmissionDiagnosticCode::NestedE11BRequestedWithoutNamedScenario
            .as_str()
            .to_string()
    ));
    assert!(b.still_rejected_until_acceptance.contains(
        &DesignerAdmissionDiagnosticCode::D2aRequestedWithoutNamedScenario
            .as_str()
            .to_string()
    ));
    assert!(c.still_rejected_until_acceptance.contains(
        &DesignerAdmissionDiagnosticCode::AtlasRequestedWithoutGate
            .as_str()
            .to_string()
    ));
}

#[test]
fn v7_8_scenarios_do_not_open_clausething_or_clausescript() {
    let pack = pack();
    assert!(pack
        .scenarios
        .iter()
        .all(|scenario| !scenario.first_implementation_gate_after_acceptance.contains("Clause")));

    let source = include_str!("../src/designer_admission/v7_8_line_scenarios.rs");
    assert!(source.contains("no E-11B"));
    assert!(source.contains("ClauseThing"));
    assert!(source.contains("ClauseScript"));
    assert!(!source.contains("ClauseScriptParser"));
    assert!(!source.contains("ClauseThingRuntime"));
}

#[test]
fn v7_8_scenarios_do_not_create_frontierv2_5() {
    let source = include_str!("../src/designer_admission/v7_8_line_scenarios.rs");
    assert!(!source.contains("FrontierV2-5"));
    assert!(!source.contains("frontier_v2_5"));
}

#[test]
fn v7_8_scenarios_do_not_reopen_act_event_obs_pipe() {
    let source = include_str!("../src/designer_admission/v7_8_line_scenarios.rs");
    for token in ["ACT-5", "EVENT-3", "OBS-5", "PIPE-1"] {
        assert!(!source.contains(token));
    }
}

#[test]
fn v7_8_scenarios_do_not_add_runtime_wiring_or_simthing_sim_semantics() {
    let source = include_str!("../src/designer_admission/v7_8_line_scenarios.rs");
    for token in [
        "simthing_driver",
        "simthing_sim",
        "FirstSliceMappingSession",
        "CommandEncoder",
        "Queue",
        "simthing_sim",
    ] {
        assert!(
            !source.contains(token),
            "source should not contain runtime import/token {token}"
        );
    }

    let sim_source = std::fs::read_to_string("../simthing-sim/src/lib.rs").unwrap();
    for token in ["V78LineScenario", "ClauseThing", "ClauseScript", "FrontierV2"] {
        assert!(!sim_source.contains(token));
    }
}
