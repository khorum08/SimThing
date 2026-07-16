//! Scenario-neutral field-economy grammar and lowering tests.

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::TransformOp;
use simthing_spec::{
    EmissionFormulaSpec, EmlGadgetInstanceSpec, InstallTargetSpec, ResourceEconomyOptInMode,
};

const FIELD_ECONOMY_SEMANTIC_CASES: &[&str] = &[
    "well-formed-existing-surfaces",
    "second-synthetic-vocabulary",
    "output-yield",
    "location-enrollment",
    "silo-owner-capacity",
    "malformed-spanned-admission",
];

fn hydrate(text: &str) -> simthing_clausething::HydratedScenarioPack {
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    hydrate_scenario(&document).expect("hydrate scenario")
}

fn constant_emission(pack: &simthing_clausething::HydratedScenarioPack, id: &str) -> f32 {
    let resource_economy = pack.game_mode.resource_economy.as_ref().unwrap();
    let emission = resource_economy
        .emissions
        .iter()
        .find(|entry| entry.id == id)
        .unwrap_or_else(|| panic!("missing emission {id}"));
    match &emission.formula {
        EmissionFormulaSpec::Constant(value) => *value,
        other => panic!("expected constant emission {id}, got {other:?}"),
    }
}

fn overlay_add_install(
    pack: &simthing_clausething::HydratedScenarioPack,
    id: &str,
) -> (f32, String) {
    let overlay = pack
        .game_mode
        .overlays
        .iter()
        .find(|entry| entry.id == id)
        .unwrap_or_else(|| panic!("missing overlay {id}"));
    let amount = match &overlay.sub_field_deltas[0].1 {
        TransformOp::Add(value) => *value,
        other => panic!("expected add overlay {id}, got {other:?}"),
    };
    let target_id = match &overlay.install {
        InstallTargetSpec::ScenarioListed { target_id } => target_id.clone(),
        other => panic!("expected ScenarioListed install for {id}, got {other:?}"),
    };
    (amount, target_id)
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
    owner = union {
        owner_key = "union"
        display_name = "Union"
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
            targets_property = "forge::ridge_tools_quantity"
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
    assert_eq!(FIELD_ECONOMY_SEMANTIC_CASES.len(), 6);
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
    assert_eq!(
        resource_economy.recipes[0].inputs[0].property.name,
        "ridge_ore_quantity"
    );
    assert_eq!(
        resource_economy.recipes[0].target.name,
        "ridge_tools_quantity"
    );
    assert_eq!(resource_economy.transfers.len(), 1);
    assert_eq!(
        resource_economy.transfers[0].target.name,
        "guild_ore_stockpile"
    );
    assert_eq!(resource_economy.emissions.len(), 5);
    assert_eq!(resource_economy.emit_on_threshold.len(), 1);
    assert_eq!(
        resource_economy.emit_on_threshold[0].source.name,
        "basin_smoke_presence"
    );
    assert_eq!(resource_economy.emit_on_threshold[0].event_kind, 77);
    assert_eq!(
        constant_emission(&pack, "valley_economy_production_yield_ridge_foundry"),
        1.0
    );
    assert_eq!(
        constant_emission(&pack, "valley_economy_silo_capacity_guild_ore"),
        100.0
    );

    assert!(pack
        .game_mode
        .properties
        .iter()
        .any(|property| property.namespace == "forge" && property.name == "ridge_tools_quantity"));
    assert!(pack
        .game_mode
        .properties
        .iter()
        .any(|property| property.namespace == "forge" && property.name == "basin_smoke_presence"));
    assert_eq!(pack.game_mode.overlays.len(), 4);
    assert_eq!(
        pack.game_mode.overlays[3].targets_property,
        "forge::ridge_tools_quantity"
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
        resource_economy.recipes[0].target.name,
        "spring_pressure_quantity"
    );
    assert_eq!(
        resource_economy
            .emissions
            .iter()
            .find(|entry| entry.id == "waterworks_quantity_emission_spring_water")
            .unwrap()
            .source
            .name,
        "spring_water_quantity"
    );
    assert_eq!(
        constant_emission(&pack, "waterworks_production_yield_pump_house"),
        2.0
    );
}

/// catches: authored production yield being parsed but flattened out of existing lowering surfaces.
#[test]
fn production_output_yield_changes_existing_lowered_surfaces() {
    let low_yield = AQUEDUCT_SCENARIO.replace(
        "output = { resource = \"pressure\" amount = 2 }",
        "output = { resource = \"pressure\" amount = 1 }",
    );
    let low = hydrate(&low_yield);
    let high = hydrate(AQUEDUCT_SCENARIO);

    assert_eq!(
        constant_emission(&low, "waterworks_production_yield_pump_house"),
        1.0
    );
    assert_eq!(
        constant_emission(&high, "waterworks_production_yield_pump_house"),
        2.0
    );
    assert_eq!(
        overlay_add_install(&low, "waterworks_production_location_pump_house"),
        (1.0, "spring".to_string())
    );
    assert_eq!(
        overlay_add_install(&high, "waterworks_production_location_pump_house"),
        (2.0, "spring".to_string())
    );
}

/// catches: authored spatial enrollment validating only sidecar references without changing targets.
#[test]
fn location_authoring_changes_resource_targets_and_installs() {
    let moved = FOUNDRY_SCENARIO
        .replace(
            "field_resource_quantity = ridge_ore {\n            location = \"ridge\"",
            "field_resource_quantity = ridge_ore {\n            location = \"basin\"",
        )
        .replace(
            "production_building = ridge_foundry {\n            location = \"ridge\"",
            "production_building = ridge_foundry {\n            location = \"basin\"",
        )
        .replace(
            "disruption_presence = basin_smoke {\n            location = \"basin\"",
            "disruption_presence = basin_smoke {\n            location = \"ridge\"",
        )
        .replace(
            "targets_property = \"forge::ridge_tools_quantity\"",
            "targets_property = \"forge::basin_tools_quantity\"",
        );
    let ridge = hydrate(FOUNDRY_SCENARIO);
    let basin = hydrate(&moved);
    let ridge_economy = ridge.game_mode.resource_economy.as_ref().unwrap();
    let basin_economy = basin.game_mode.resource_economy.as_ref().unwrap();

    assert_eq!(
        ridge_economy.recipes[0].inputs[0].property.name,
        "ridge_ore_quantity"
    );
    assert_eq!(
        basin_economy.recipes[0].inputs[0].property.name,
        "basin_ore_quantity"
    );
    assert_eq!(ridge_economy.recipes[0].target.name, "ridge_tools_quantity");
    assert_eq!(basin_economy.recipes[0].target.name, "basin_tools_quantity");
    assert_eq!(
        ridge_economy
            .emissions
            .iter()
            .find(|entry| entry.id == "valley_economy_quantity_emission_ridge_ore")
            .unwrap()
            .source
            .name,
        "ridge_ore_quantity"
    );
    assert_eq!(
        basin_economy
            .emissions
            .iter()
            .find(|entry| entry.id == "valley_economy_quantity_emission_ridge_ore")
            .unwrap()
            .source
            .name,
        "basin_ore_quantity"
    );
    assert_eq!(
        ridge_economy.emit_on_threshold[0].source.name,
        "basin_smoke_presence"
    );
    assert_eq!(
        basin_economy.emit_on_threshold[0].source.name,
        "ridge_smoke_presence"
    );
    assert_eq!(
        overlay_add_install(&ridge, "valley_economy_production_location_ridge_foundry").1,
        "ridge"
    );
    assert_eq!(
        overlay_add_install(&basin, "valley_economy_production_location_ridge_foundry").1,
        "basin"
    );
}

/// catches: silo owner/capacity surviving only in the hydrated sidecar.
#[test]
fn silo_owner_and_capacity_change_existing_lowered_surfaces() {
    let changed = FOUNDRY_SCENARIO
        .replace(
            "owner = \"guild\"\n            resource = \"ore\"",
            "owner = \"union\"\n            resource = \"ore\"",
        )
        .replace("capacity = 100", "capacity = 125");
    let guild = hydrate(FOUNDRY_SCENARIO);
    let union = hydrate(&changed);
    let guild_economy = guild.game_mode.resource_economy.as_ref().unwrap();
    let union_economy = union.game_mode.resource_economy.as_ref().unwrap();

    assert_eq!(guild_economy.transfers[0].source.name, "guild_ore_current");
    assert_eq!(union_economy.transfers[0].source.name, "union_ore_current");
    assert_eq!(
        guild_economy.transfers[0].target.name,
        "guild_ore_stockpile"
    );
    assert_eq!(
        union_economy.transfers[0].target.name,
        "union_ore_stockpile"
    );
    assert_eq!(
        guild_economy
            .emissions
            .iter()
            .find(|entry| entry.id == "valley_economy_silo_capacity_guild_ore")
            .unwrap()
            .source
            .name,
        "guild_ore_capacity"
    );
    assert_eq!(
        union_economy
            .emissions
            .iter()
            .find(|entry| entry.id == "valley_economy_silo_capacity_guild_ore")
            .unwrap()
            .source
            .name,
        "union_ore_capacity"
    );
    assert_eq!(
        constant_emission(&guild, "valley_economy_silo_capacity_guild_ore"),
        100.0
    );
    assert_eq!(
        constant_emission(&union, "valley_economy_silo_capacity_guild_ore"),
        125.0
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
