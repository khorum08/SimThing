//! Phase M — resource economy authoring preview and admission diagnostics.

#[path = "support/resource_economy_compile.rs"]
mod support;

use simthing_core::{DimensionRegistry, EmlExpressionRegistry, SubFieldRole};
use simthing_spec::{
    compile_game_mode_resource_economy_authoring_preview, compile_property,
    compile_resource_economy_authoring_preview, deserialize_game_mode_ron,
    ResourceEconomyOptInMode, ResourceEconomySpec, ResourceRecipeSpec, ResourceTransferSpec,
    SpecError,
};
use support::{amount_property, exact_eml_registry, register_amount_property};

const SURPLUS_RON: &str =
    include_str!("../../simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron");
const DEFICIT_RON: &str =
    include_str!("../../simthing-driver/tests/fixtures/daily_economy_banking_deficit_scenario.ron");

fn treasury_static_net(preview: &simthing_spec::ResourceEconomyAuthoringPreview) -> Option<f32> {
    preview
        .report
        .simple_static_nets
        .iter()
        .find(|n| n.namespace == "core" && n.name == "treasury" && n.role == "amount")
        .map(|n| n.net_per_boundary)
}

#[test]
fn surplus_fixture_authoring_preview() {
    let game_mode = deserialize_game_mode_ron(SURPLUS_RON).expect("surplus fixture parses");
    let preview = compile_game_mode_resource_economy_authoring_preview(
        &game_mode,
        &EmlExpressionRegistry::new(),
    )
    .expect("surplus fixture preview compiles");

    let report = &preview.report;
    assert!(game_mode.resource_economy.is_some());
    assert!(!report.resource_flow_enabled);
    assert_eq!(report.opt_in_mode, ResourceEconomyOptInMode::TransferOnly);
    assert_eq!(report.transfer_count, 2);
    assert_eq!(report.recipe_count, 1);
    assert_eq!(report.threshold_emit_count, 0);
    assert_eq!(report.order_bands, vec![0, 1]);

    let transfer_ids: Vec<_> = report.transfers.iter().map(|t| t.id.as_str()).collect();
    assert!(transfer_ids.contains(&"bank_daily_income"));
    assert!(transfer_ids.contains(&"daily_upkeep"));

    let recipe_ids: Vec<_> = report.recipes.iter().map(|r| r.id.as_str()).collect();
    assert!(recipe_ids.contains(&"daily_income"));

    assert_eq!(treasury_static_net(&preview), Some(7.0));
    assert!(report.warnings.is_empty());

    // R2 ergonomics: schedule_lines helper exposes intended transfers clearly for designers
    assert!(report
        .schedule_lines
        .iter()
        .any(|s| s.contains("bank_daily_income")));
    assert!(report
        .schedule_lines
        .iter()
        .any(|s| s.contains("daily_upkeep")));
    assert!(!report.schedule_lines.is_empty());
}

#[test]
fn deficit_fixture_authoring_preview() {
    let game_mode = deserialize_game_mode_ron(DEFICIT_RON).expect("deficit fixture parses");
    let preview = compile_game_mode_resource_economy_authoring_preview(
        &game_mode,
        &EmlExpressionRegistry::new(),
    )
    .expect("deficit fixture preview compiles");

    let report = &preview.report;
    assert!(!report.resource_flow_enabled);
    assert_eq!(report.transfer_count, 2);
    assert_eq!(report.recipe_count, 0);
    assert_eq!(report.threshold_emit_count, 1);

    let transfer_ids: Vec<_> = report.transfers.iter().map(|t| t.id.as_str()).collect();
    assert!(transfer_ids.contains(&"bank_daily_income"));
    assert!(transfer_ids.contains(&"daily_upkeep"));

    let threshold_ids: Vec<_> = report
        .threshold_emits
        .iter()
        .map(|e| e.id.as_str())
        .collect();
    assert!(threshold_ids.contains(&"low_storage_event"));

    assert_eq!(treasury_static_net(&preview), Some(-6.0));
    assert!(report.warnings.is_empty());

    // R2 ergonomics coverage
    assert!(report
        .schedule_lines
        .iter()
        .any(|s| s.contains("low_storage_event")));
    assert!(!report.schedule_lines.is_empty());
}

#[test]
fn preview_binds_resources_and_order_bands_from_game_mode_properties() {
    let game_mode = deserialize_game_mode_ron(SURPLUS_RON).expect("surplus fixture parses");
    let mut registry = DimensionRegistry::new();
    for property in &game_mode.properties {
        compile_property(property, &mut registry).expect("property compiles");
    }
    let economy = game_mode.resource_economy.as_ref().unwrap();
    let preview = compile_resource_economy_authoring_preview(
        economy,
        &registry,
        &EmlExpressionRegistry::new(),
        false,
    )
    .expect("inline preview compiles");

    assert!(preview
        .report
        .resources_bound
        .iter()
        .any(|b| b.namespace == "core" && b.name == "treasury" && b.role == "amount"));
}
