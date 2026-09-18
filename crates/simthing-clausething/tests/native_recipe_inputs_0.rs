//! Native recipe-input locus law (DA, relay 5730468245): a production
//! building's authored costs lower to the COMPLETE, order-free conjunction,
//! and every ambiguous or duplicate cost refuses before activation.
use simthing_clausething::{hydrate_scenario, parse_raw_document, HydrateError};
use simthing_core::SubFieldRole;
use simthing_spec::{PropertyKey, RecipeInputSpec};

const SCENARIO: &str = r#"
scenario = foundry_valley {
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge {
        display_name = "Ridge"
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
            INPUTS
            output = { resource = "tools" coefficient = 1 }
            throttle_hint_max_per_tick = 1
        }
    }
}
"#;

const ORE: &str = r#"input = { resource = "ore" amount = 2 }"#;
const WATER: &str = r#"input = { resource = "water" amount = 3 }"#;
const POWER: &str =
    r#"input = { entity = "guild" property = "forge::power" role = balance amount = 1 }"#;

fn building(inputs: &[&str]) -> String {
    SCENARIO.replace("INPUTS", &inputs.join("\n            "))
}

fn hydrate(text: &str) -> Result<simthing_clausething::HydratedScenarioPack, HydrateError> {
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    hydrate_scenario(&document)
}

fn lowered_inputs(text: &str) -> Vec<RecipeInputSpec> {
    let pack = hydrate(text).expect("hydrate scenario");
    let economy = pack.game_mode.resource_economy.expect("resource economy");
    economy
        .recipes
        .into_iter()
        .find(|recipe| recipe.id.ends_with("ridge_foundry"))
        .expect("foundry recipe")
        .inputs
}

/// The economic identity of a lowered cost: its locus and amount. The span
/// token is provenance for refusal attribution, not economics.
fn semantics(inputs: &[RecipeInputSpec]) -> Vec<(PropertyKey, SubFieldRole, Option<String>, u32)> {
    inputs
        .iter()
        .map(|input| {
            (
                input.property.clone(),
                input.role.clone(),
                input.host_entity.clone(),
                input.unit_cost.to_bits(),
            )
        })
        .collect()
}

fn refusal(inputs: &[&str]) -> HydrateError {
    hydrate(&building(inputs)).expect_err("must refuse before activation")
}

/// catches: repeated authored costs collapsing last-wins or first-wins, field
/// order carrying economic authority, or a vector door hard-coded to two costs.
#[test]
fn authored_costs_lower_to_the_complete_order_free_conjunction() {
    let ore = (
        PropertyKey::new("forge", "ridge_ore_quantity"),
        SubFieldRole::Amount,
        Some("ridge".to_string()),
        2.0f32.to_bits(),
    );
    let power = (
        PropertyKey::new("forge", "power"),
        SubFieldRole::Named("balance".into()),
        Some("guild".to_string()),
        1.0f32.to_bits(),
    );
    let water = (
        PropertyKey::new("forge", "ridge_water_quantity"),
        SubFieldRole::Amount,
        Some("ridge".to_string()),
        3.0f32.to_bits(),
    );

    let forward = semantics(&lowered_inputs(&building(&[ORE, POWER])));
    assert_eq!(forward.len(), 2, "both authored costs survive: {forward:?}");
    assert!(forward.contains(&ore) && forward.contains(&power));
    assert_eq!(
        forward,
        semantics(&lowered_inputs(&building(&[POWER, ORE]))),
        "reordering distinct costs preserves the same conjunction"
    );

    let three = semantics(&lowered_inputs(&building(&[ORE, WATER, POWER])));
    assert_eq!(three.len(), 3, "the conjunction is vector-width: {three:?}");
    for order in [
        [ORE, POWER, WATER],
        [WATER, ORE, POWER],
        [WATER, POWER, ORE],
        [POWER, ORE, WATER],
        [POWER, WATER, ORE],
    ] {
        assert_eq!(three, semantics(&lowered_inputs(&building(&order))));
    }
    assert!(three.contains(&ore) && three.contains(&water) && three.contains(&power));
}

/// catches: the historical single-cost building changing its economic contract.
#[test]
fn single_cost_building_lowers_exactly_as_before() {
    assert_eq!(
        lowered_inputs(&building(&[ORE])),
        vec![RecipeInputSpec {
            property: PropertyKey::new("forge", "ridge_ore_quantity"),
            role: SubFieldRole::Amount,
            unit_cost: 2.0,
            host_entity: Some("ridge".into()),
            host_span_token: None,
        }]
    );
}

/// catches: an ambiguous, incomplete, or silently overwritten cost reaching
/// activation instead of a spanned authoring refusal.
#[test]
fn malformed_or_repeated_cost_fields_refuse_before_activation() {
    let cases: [(&[&str], &str); 7] = [
        (
            &[r#"input = { resource = "ore" entity = "guild" property = "forge::power" role = balance amount = 1 }"#],
            "mixes `resource` with a canonical",
        ),
        (
            &[r#"input = { entity = "guild" property = "forge::power" amount = 1 }"#],
            "canonical locus is incomplete; missing role",
        ),
        (&[r#"input = { amount = 1 }"#], "names no locus"),
        (
            &[r#"input = { entity = "guild" property = "power" role = balance amount = 1 }"#],
            "must be `namespace::name`",
        ),
        (
            &[r#"input = { resource = "ore" amount = 2 amount = 3 }"#],
            "duplicate production_building.input field `amount`",
        ),
        (
            &[ORE, r#"output = { resource = "rails" coefficient = 1 }"#],
            "duplicate production_building field `output`",
        ),
        (&[], "requires at least one `input`"),
    ];
    for (inputs, expected) in cases {
        let error = refusal(inputs);
        assert!(
            error.message.contains(expected),
            "{inputs:?}: expected `{expected}`, got `{}`",
            error.message
        );
        assert!(error.span.is_some(), "{inputs:?}: refusal must carry a span");
    }
}

/// catches: two costs on the SAME locus being merged or overwritten, including
/// a canonical cost that names a shorthand's own derived quantity locus.
#[test]
fn duplicate_cost_locus_is_refused_not_merged() {
    for inputs in [
        [ORE, r#"input = { resource = "ore" amount = 3 }"#],
        [
            POWER,
            r#"input = { entity = "guild" property = "forge::power" role = balance amount = 4 }"#,
        ],
        [
            ORE,
            r#"input = { entity = "ridge" property = "forge::ridge_ore_quantity" role = Amount amount = 1 }"#,
        ],
    ] {
        let error = refusal(&inputs);
        assert!(
            error.message.contains("authors the same input locus"),
            "{inputs:?}: {}",
            error.message
        );
        assert!(error.span.is_some(), "{inputs:?}: refusal must carry a span");
    }
}
