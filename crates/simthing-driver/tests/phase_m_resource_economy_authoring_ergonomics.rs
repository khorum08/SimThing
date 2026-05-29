//! Phase M Resource Economy Authoring Ergonomics V1 — driver-level fixture + doctrine tests.

#[path = "support/daily_economy_session.rs"]
mod daily_economy;

use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_game_mode_resource_economy_authoring_preview, deserialize_game_mode_ron,
    MappingExecutionProfile, ResourceFlowOptInMode,
};

#[test]
fn surplus_daily_economy_fixture_authoring_preview() {
    let game_mode = daily_economy::surplus_game_mode();
    let preview = compile_game_mode_resource_economy_authoring_preview(
        &game_mode,
        &simthing_core::EmlExpressionRegistry::new(),
    )
    .expect("surplus preview");

    assert!(!preview.report.resource_flow_enabled);
    assert_eq!(preview.report.transfer_count, 2);
    assert_eq!(preview.report.recipe_count, 1);
    assert_eq!(preview.report.order_bands, vec![0, 1]);

    let treasury_net = preview
        .report
        .simple_static_nets
        .iter()
        .find(|n| n.name == "treasury")
        .map(|n| n.net_per_boundary);
    assert_eq!(treasury_net, Some(daily_economy::SURPLUS_DAILY_NET));
}

#[test]
fn deficit_daily_economy_fixture_authoring_preview() {
    let game_mode = daily_economy::deficit_game_mode();
    let preview = compile_game_mode_resource_economy_authoring_preview(
        &game_mode,
        &simthing_core::EmlExpressionRegistry::new(),
    )
    .expect("deficit preview");

    assert!(!preview.report.resource_flow_enabled);
    assert_eq!(preview.report.threshold_emit_count, 1);
    assert!(
        preview
            .report
            .threshold_emits
            .iter()
            .any(|e| e.id == "low_storage_event")
    );

    let treasury_net = preview
        .report
        .simple_static_nets
        .iter()
        .find(|n| n.name == "treasury")
        .map(|n| n.net_per_boundary);
    assert_eq!(treasury_net, Some(daily_economy::DEFICIT_DAILY_NET));
}

#[test]
fn doctrine_posture_preserved_for_authoring_ergonomics_track() {
    let sim_sources = [
        include_str!("../../simthing-sim/src/lib.rs"),
        include_str!("../../simthing-sim/src/boundary.rs"),
    ];
    for text in sim_sources {
        assert!(
            !text.contains("DailyResolutionBoundary"),
            "forbidden DailyResolutionBoundary in simthing-sim"
        );
        assert!(
            !text.contains("struct Calendar")
                && !text.contains("enum Calendar")
                && !text.contains("struct Season")
                && !text.contains("enum Season"),
            "forbidden calendar/season types in simthing-sim"
        );
    }

    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
    assert_eq!(ResourceFlowOptInMode::default(), ResourceFlowOptInMode::Disabled);
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);

    let surplus = daily_economy::surplus_game_mode();
    let deficit = daily_economy::deficit_game_mode();
    for mode in [&surplus, &deficit] {
        assert_eq!(
            mode.resource_flow
                .as_ref()
                .map(|rf| rf.opt_in_mode)
                .unwrap_or(ResourceFlowOptInMode::Disabled),
            ResourceFlowOptInMode::Disabled
        );
    }

    let surplus_ron = daily_economy::SURPLUS_RON;
    assert!(surplus_ron.contains("day_index") || surplus_ron.contains("ticks_per_day") || surplus_ron.contains("daily"));
    assert!(!surplus_ron.contains("DailyResolutionBoundary"));
}

#[test]
fn example_fixtures_do_not_enable_resource_flow_e11() {
    for ron in [daily_economy::SURPLUS_RON, daily_economy::DEFICIT_RON] {
        let game_mode = deserialize_game_mode_ron(ron).expect("fixture parses");
        let preview = compile_game_mode_resource_economy_authoring_preview(
            &game_mode,
            &simthing_core::EmlExpressionRegistry::new(),
        )
        .expect("preview compiles");
        assert!(!preview.report.resource_flow_enabled);
    }
}
