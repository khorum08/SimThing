//! Scenario-neutral field-economy grammar and lowering tests.

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_spec::{EmlGadgetInstanceSpec, ResourceEconomyOptInMode};

fn hydrate(text: &str) -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    hydrate_scenario(&document).expect("hydrate scenario")
}

const FOUNDRY_SCENARIO: &str = r#"
scenario = foundry_valley {
    metadata = {
        display_name = "Foundry Valley"
    }
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge {
        display_name = "Ridge"
    }
    location = basin {
        display_name = "Basin"
    }
    field_economy = valley_economy {
        namespace = "forge"
        field_resource_quantity = ridge_ore {
            location = "ridge"
            resource = "ore"
            amount = 12
        }
        production_building = ridge_foundry {
            location = "ridge"
            input = { resource = "ore" amount = 2 }
            output = { resource = "tools" amount = 1 }
            throttle_hint_max_per_tick = 3
        }
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
            capacity = 100
            current = 20
        }
        disruption_presence = basin_smoke {
            location = "basin"
            resource = "smoke"
            amount = 4
            threshold = 2
            direction = Rising
            event_kind = 77
        }
        owner_policy_overlay = guild_tools {
            owner = "guild"
            targets_property = "forge::tools_quantity"
            amount_mult = 1.25
        }
        weight_profile = expansion_need {
            profile = "expansion-need"
            input = { input_col = 0 weight_col = 10 }
            input = { input_col = 1 weight_col = 11 }
            output_col = 12
        }
    }
}
"#;

const AQUEDUCT_SCENARIO: &str = r#"
scenario = aqueduct_delta {
    owner = council {
        owner_key = "council"
        display_name = "Council"
        archetype = "civic"
    }
    location = spring {
        display_name = "Spring"
    }
    field_economy = waterworks {
        namespace = "civic"
        field_resource_quantity = spring_water {
            location = "spring"
            resource = "water"
            amount = 30
        }
        production_building = pump_house {
            location = "spring"
            input = { resource = "water" amount = 5 }
            output = { resource = "pressure" amount = 2 }
            throttle_hint_max_per_tick = 1
        }
        weight_profile = manufacturing_need {
            profile = "manufacturing-need"
            input = { input_col = 2 weight_col = 5 }
            output_col = 6
        }
    }
}
"#;

/// catches: lowering drift away from existing ResourceEconomySpec, OverlaySpec, and EML profile surfaces.
#[test]
fn field_economy_well_formed_hydrates_to_existing_surfaces() {
    let pack = hydrate(FOUNDRY_SCENARIO);
    let economy = pack.field_economy.as_ref().expect("field economy");
    assert_eq!(economy.id, "valley_economy");
    assert_eq!(economy.weight_profiles.len(), 1);
    match &economy.weight_profiles[0].stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            output_col,
            ..
        } => {
            assert_eq!(input_cols, &vec![0, 1]);
            assert_eq!(weight_cols, &vec![10, 11]);
            assert_eq!(*output_col, Some(12));
        }
        other => panic!("expected WeightedAccumulator, got {other:?}"),
    }

    let resource_economy = pack
        .game_mode
        .resource_economy
        .as_ref()
        .expect("resource economy");
    assert_eq!(
        resource_economy.opt_in_mode,
        ResourceEconomyOptInMode::TransferAndEmission
    );
    assert_eq!(resource_economy.recipes.len(), 1);
    assert_eq!(
        resource_economy.recipes[0].id,
        "valley_economy_recipe_ridge_foundry"
    );
    assert_eq!(resource_economy.transfers.len(), 1);
    assert_eq!(resource_economy.emissions.len(), 2);
    assert_eq!(resource_economy.emit_on_threshold.len(), 1);
    assert_eq!(resource_economy.emit_on_threshold[0].event_kind, 77);

    assert!(
        pack.game_mode
            .properties
            .iter()
            .any(|property| property.namespace == "forge" && property.name == "tools_quantity")
    );
    assert!(
        pack.game_mode
            .properties
            .iter()
            .any(|property| property.namespace == "forge" && property.name == "smoke_presence")
    );
    assert_eq!(pack.game_mode.overlays.len(), 1);
    assert_eq!(
        pack.game_mode.overlays[0].targets_property,
        "forge::tools_quantity"
    );
}

/// catches: scenario-specific keying hidden in the grammar by running a different vocabulary through it.
#[test]
fn second_synthetic_scenario_uses_same_field_economy_grammar() {
    let pack = hydrate(AQUEDUCT_SCENARIO);
    let economy = pack.field_economy.as_ref().expect("field economy");
    assert_eq!(economy.namespace, "civic");
    assert_eq!(economy.production_buildings[0].output_resource, "pressure");
    assert_eq!(economy.weight_profiles[0].profile, "manufacturing-need");
    let resource_economy = pack.game_mode.resource_economy.as_ref().unwrap();
    assert_eq!(resource_economy.recipes[0].inputs[0].unit_cost, 5.0);
    assert_eq!(
        resource_economy.emissions[0].id,
        "waterworks_quantity_emission_spring_water"
    );
}

/// catches: malformed field-economy authoring becoming a runtime branch instead of admission error.
#[test]
fn malformed_field_economy_is_spanned_hard_error_at_admission() {
    let malformed = FOUNDRY_SCENARIO.replace("current = 20", "current = 120");
    let document = parse_raw_document(malformed.as_bytes()).expect("parse ClauseScript");
    let err = hydrate_scenario(&document).expect_err("must reject at admission");
    assert!(err.message.contains("current 120 exceeds capacity 100"));
    assert!(
        err.span.is_some(),
        "admission error must carry a source span"
    );
}
