//! Phase T designer-authored resource economy RON smoke coverage.

use simthing_core::{DimensionRegistry, EmlExpressionRegistry};
use simthing_spec::{
    compile_property, compile_resource_economy, deserialize_game_mode_ron, ResourceEconomyOptInMode,
};

const FIXTURE: &str = include_str!("fixtures/game_modes/resource_economy_smoke.ron");

fn compile_fixture_economy() -> (
    simthing_spec::GameModeSpec,
    simthing_spec::CompiledResourceEconomy,
) {
    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let mut registry = DimensionRegistry::new();
    for property in &game_mode.properties {
        compile_property(property, &mut registry).expect("fixture property compiles");
    }
    let compiled = compile_resource_economy(
        game_mode
            .resource_economy
            .as_ref()
            .expect("fixture has resource economy"),
        &registry,
        &EmlExpressionRegistry::new(),
    )
    .expect("fixture resource economy compiles");
    (game_mode, compiled)
}

#[test]
fn resource_economy_designer_ron_fixture_deserializes() {
    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let economy = game_mode
        .resource_economy
        .as_ref()
        .expect("fixture has resource economy");

    assert_eq!(game_mode.id, "phase_t_resource_economy_smoke");
    assert_eq!(game_mode.properties.len(), 6);
    assert_eq!(
        economy.opt_in_mode,
        ResourceEconomyOptInMode::TransferAndEmission
    );
    assert_eq!(economy.transfers.len(), 1);
    assert_eq!(economy.recipes.len(), 1);
    assert_eq!(economy.emissions.len(), 1);
}

#[test]
fn resource_economy_designer_ron_roundtrips_without_field_drop() {
    let game_mode = deserialize_game_mode_ron(FIXTURE).expect("designer RON fixture parses");
    let pretty = ron::ser::to_string_pretty(&game_mode, ron::ser::PrettyConfig::default())
        .expect("serialize fixture");
    let reparsed = deserialize_game_mode_ron(&pretty).expect("roundtripped fixture parses");

    assert_eq!(reparsed.properties.len(), game_mode.properties.len());
    assert_eq!(reparsed.resource_economy, game_mode.resource_economy);
}

#[test]
fn resource_economy_designer_ron_compile_succeeds() {
    let (_game_mode, compiled) = compile_fixture_economy();

    assert_eq!(compiled.transfers.len(), 1);
    assert_eq!(compiled.recipes.len(), 1);
    assert_eq!(compiled.recipes[0].inputs.len(), 2);
    assert_eq!(compiled.emissions.len(), 1);
}

#[test]
fn resource_economy_designer_ron_unknown_field_rejected_if_supported() {
    let invalid = FIXTURE.replace(
        "source: (namespace: \"core\", name: \"credits\")",
        "soruce_property: (namespace: \"core\", name: \"credits\")",
    );
    let err = deserialize_game_mode_ron(&invalid).expect_err("unknown transfer field rejected");
    let message = err.to_string();

    assert!(
        message.contains("soruce_property") || message.contains("unknown field"),
        "unexpected error: {message}"
    );
}
