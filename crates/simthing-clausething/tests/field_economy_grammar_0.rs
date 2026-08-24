//! Scenario-neutral field-economy grammar and lowering tests.

use simthing_clausething::{hydrate_scenario, parse_raw_document};
use simthing_core::TransformOp;
use simthing_spec::{EmissionFormulaSpec, EmlGadgetInstanceSpec, InstallTargetSpec};

const FIELD_ECONOMY_SEMANTIC_CASES: &[&str] = &[
    "well-formed-existing-surfaces",
    "second-synthetic-vocabulary",
    "unsupported-output-yield",
    "location-enrollment",
    "unsupported-silo-capacity",
    "silo-owner-current",
    "malformed-spanned-admission",
    "missing-output-coefficient",
    "missing-flow-coupling",
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
    let amount = overlay.sub_field_deltas[0]
        .1
        .as_add_literal()
        .unwrap_or_else(|| {
            panic!(
                "expected add overlay {id}, got {:?}",
                overlay.sub_field_deltas[0].1
            )
        });
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
            output = { resource = "tools" coefficient = 1.5 }
            throttle_hint_max_per_tick = 3
        }
        stockpile_silo = guild_ore {
            owner = "guild"
            resource = "ore"
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
        flow_coupling = smoke_suppresses_tools {
            source = { location = "ridge" resource = "tools" unit_cost = 1 }
            pressure = { location = "basin" resource = "smoke" unit_cost = 1 }
            weight = { owner = "guild" resource = "ore" unit_cost = 1 }
            sink = { location = "ridge" resource = "spoiled_tools" }
            output_coefficient = 1
            order_band = 1
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
            output = { resource = "pressure" coefficient = 1.25 }
            throttle_hint_max_per_tick = 1
        }
        stockpile_silo = council_gate {
            owner = "council"
            resource = "gate"
            current = 1
        }
        disruption_presence = spring_silt {
            location = "spring"
            resource = "silt"
            amount = 1
            threshold = 0.5
            event_kind = 78
        }
        flow_coupling = silt_suppresses_pressure {
            source = { location = "spring" resource = "pressure" unit_cost = 1 }
            pressure = { location = "spring" resource = "silt" unit_cost = 1 }
            weight = { owner = "council" resource = "gate" unit_cost = 1 }
            sink = { location = "spring" resource = "lost_pressure" }
            output_coefficient = 1
            order_band = 1
        }
        weight_profile = manufacturing_need {
            profile = "manufacturing-need"
            input = { input_col = 2 weight_col = 5 }
            output_col = 6
        }
    }
}
"#;
/// catches: unsupported authored production yield being flattened onto unrelated runtime records.
#[test]
fn production_output_amount_is_spanned_unsupported_authoring_error() {
    let unsupported = AQUEDUCT_SCENARIO.replace(
        "output = { resource = \"pressure\" coefficient = 1.25 }",
        "output = { resource = \"pressure\" coefficient = 1.25 amount = 2 }",
    );
    let document = parse_raw_document(unsupported.as_bytes()).expect("parse ClauseScript");
    let err = hydrate_scenario(&document).expect_err("must reject unsupported output amount");
    assert!(err.message.contains("unsupported output field `amount`"));
    assert!(
        err.span.is_some(),
        "unsupported output amount must carry a source span"
    );
}

/// catches: omitted coefficient silently falling back to unit output.
#[test]
fn production_output_coefficient_is_required_and_spanned() {
    let missing = AQUEDUCT_SCENARIO.replace(
        "output = { resource = \"pressure\" coefficient = 1.25 }",
        "output = { resource = \"pressure\" }",
    );
    let document = parse_raw_document(missing.as_bytes()).expect("parse ClauseScript");
    let err = hydrate_scenario(&document).expect_err("missing coefficient must fail closed");
    assert!(err.message.contains("coefficient"), "{}", err.message);
    assert!(
        err.span.is_some(),
        "missing coefficient must carry a source span"
    );
}
/// catches: unsupported silo capacity being flattened onto unrelated runtime records.
#[test]
fn stockpile_capacity_is_spanned_unsupported_authoring_error() {
    let unsupported =
        FOUNDRY_SCENARIO.replace("current = 20", "capacity = 100\n            current = 20");
    let document = parse_raw_document(unsupported.as_bytes()).expect("parse ClauseScript");
    let err = hydrate_scenario(&document).expect_err("must reject unsupported silo capacity");
    assert!(err
        .message
        .contains("unsupported stockpile_silo field `capacity`"));
    assert!(
        err.span.is_some(),
        "unsupported silo capacity must carry a source span"
    );
}
/// catches: malformed field-economy authoring becoming a runtime branch instead of admission error.
#[test]
fn malformed_field_economy_is_spanned_hard_error_at_admission() {
    let malformed = FOUNDRY_SCENARIO.replace("current = 20", "current = -1");
    let document = parse_raw_document(malformed.as_bytes()).expect("parse ClauseScript");
    let err = hydrate_scenario(&document).expect_err("must reject at admission");
    assert!(err
        .message
        .contains("`stockpile_silo.current` must be non-negative"));
    assert!(
        err.span.is_some(),
        "admission error must carry a source span"
    );
}
