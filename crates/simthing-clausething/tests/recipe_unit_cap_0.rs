//! Authoritative per-generation recipe-unit cap: native authoring lowers it to a
//! BUDGET, never an op field (DA, relay 5735839909; StemThing law 3 per live
//! Board 5879126789). The throttle hint stays a hint.
use simthing_clausething::{hydrate_scenario, parse_raw_document, HydrateError};
use simthing_core::{SubFieldRole, TransformOp};
use simthing_spec::{GameModeSpec, InstallTargetSpec, ResourceRecipeSpec};

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
            input = { resource = "ore" amount = 2 }
            output = { resource = "tools" coefficient = 1 }
            throttle_hint_max_per_tick = 3
            CAP
        }
    }
}
"#;

const CAPACITY: &str = "ridge_ridge_foundry_capacity_units";

fn game_mode(cap: &str) -> Result<GameModeSpec, HydrateError> {
    let text = SCENARIO.replace("CAP", cap);
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    Ok(hydrate_scenario(&document)?.game_mode)
}

fn foundry(game_mode: &GameModeSpec) -> &ResourceRecipeSpec {
    game_mode
        .resource_economy
        .as_ref()
        .expect("resource economy")
        .recipes
        .iter()
        .find(|recipe| recipe.id.ends_with("ridge_foundry"))
        .expect("foundry recipe")
}

/// catches: the historical throttle hint silently promoted to authority, an
/// authored cap dropped at lowering or lowered to anything but a budget (a
/// capacity input at unit cost 1 opening at the cap, refilled by a standing Set
/// at the building), or budget data appearing for recipes that author no cap.
#[test]
fn authored_unit_cap_is_authority_and_the_hint_stays_a_hint() {
    let legacy = game_mode("").expect("uncapped building hydrates");
    assert_eq!(foundry(&legacy).throttle_hint_max_per_tick, 3);
    assert!(
        foundry(&legacy)
            .inputs
            .iter()
            .all(|input| input.property.name != CAPACITY),
        "an uncapped recipe spends no budget"
    );
    assert!(legacy.properties.iter().all(|p| p.name != CAPACITY));
    assert!(legacy
        .overlays
        .iter()
        .all(|overlay| overlay.targets_property != format!("forge::{CAPACITY}")));

    for units in [1u32, 2] {
        let capped = game_mode(&format!("max_units_per_generation = {units}"))
            .expect("capped building hydrates");
        let recipe = foundry(&capped);
        assert_eq!(
            recipe.throttle_hint_max_per_tick, 3,
            "the hint is not promoted"
        );
        let budget: Vec<_> = recipe
            .inputs
            .iter()
            .filter(|input| input.property.name == CAPACITY)
            .collect();
        assert_eq!(budget.len(), 1, "cap {units}: one capacity input");
        assert_eq!(budget[0].property.namespace, "forge");
        assert_eq!(budget[0].role, SubFieldRole::Amount);
        assert_eq!(budget[0].unit_cost, 1.0, "one unit of budget per unit");
        assert_eq!(budget[0].host_entity.as_deref(), Some("ridge"));

        let property = capped
            .properties
            .iter()
            .find(|p| p.name == CAPACITY)
            .expect("the budget is a declared property");
        let amount = property
            .sub_fields
            .iter()
            .find(|sub_field| sub_field.role == SubFieldRole::Amount)
            .expect("an Amount lane");
        assert_eq!(amount.default, units as f32, "the budget opens at the cap");

        let refills: Vec<_> = capped
            .overlays
            .iter()
            .filter(|overlay| overlay.targets_property == format!("forge::{CAPACITY}"))
            .collect();
        assert_eq!(refills.len(), 1, "cap {units}: one standing refill");
        assert_eq!(
            refills[0].sub_field_deltas,
            vec![(SubFieldRole::Amount, TransformOp::set(units as f32))],
            "the refill restores the cap, never banks unused budget"
        );
        assert!(matches!(
            &refills[0].install,
            InstallTargetSpec::ScenarioListed { target_id } if target_id == "ridge"
        ));

        let json = serde_json::to_string(&capped).unwrap();
        assert_eq!(
            budget_of(&serde_json::from_str::<GameModeSpec>(&json).unwrap()),
            budget_of(&capped),
            "the canonical interchange round-trips the budget as ordinary data"
        );
    }
}

/// The recipe, the capacity's opening Amount, and the refill's deltas.
fn budget_of(
    game_mode: &GameModeSpec,
) -> (ResourceRecipeSpec, f32, Vec<(SubFieldRole, TransformOp)>) {
    let amount = game_mode
        .properties
        .iter()
        .find(|p| p.name == CAPACITY)
        .and_then(|p| {
            p.sub_fields
                .iter()
                .find(|sub_field| sub_field.role == SubFieldRole::Amount)
        })
        .expect("the budget's Amount lane")
        .default;
    let refill = game_mode
        .overlays
        .iter()
        .find(|overlay| overlay.targets_property == format!("forge::{CAPACITY}"))
        .expect("the standing refill")
        .sub_field_deltas
        .clone();
    (foundry(game_mode).clone(), amount, refill)
}

/// catches: zero, negative, fractional, or repeated cap authority being
/// rounded, defaulted, or last-wins instead of refused before activation; and
/// the retired op-level cap field re-entering through the interchange.
#[test]
fn malformed_unit_cap_refuses_before_activation() {
    for (cap, expected) in [
        ("max_units_per_generation = 0", "must be a positive integer"),
        (
            "max_units_per_generation = -1",
            "must be a non-negative integer literal",
        ),
        (
            "max_units_per_generation = 1.5",
            "must be a non-negative integer literal",
        ),
        (
            "max_units_per_generation = 1\n            max_units_per_generation = 2",
            "duplicate production_building field `max_units_per_generation`",
        ),
    ] {
        let error = game_mode(cap).expect_err("malformed cap must refuse");
        assert!(error.message.contains(expected), "{cap}: {}", error.message);
        assert!(error.span.is_some(), "{cap}: refusal must carry a span");
    }

    let recipe =
        serde_json::to_string(foundry(&game_mode("max_units_per_generation = 1").unwrap()))
            .unwrap();
    let tampered = recipe.replacen('{', "{\"max_units_per_generation\":1,", 1);
    assert_ne!(tampered, recipe);
    assert!(
        serde_json::from_str::<ResourceRecipeSpec>(&tampered).is_err(),
        "a recipe carries no cap field"
    );
}
