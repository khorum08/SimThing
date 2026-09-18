//! Authoritative per-generation recipe-unit cap: native authoring and the
//! canonical interchange (DA, relay 5735839909). The throttle hint stays a hint.
use simthing_clausething::{hydrate_scenario, parse_raw_document, HydrateError};
use simthing_spec::ResourceRecipeSpec;
use std::num::NonZeroU32;

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

fn foundry(cap: &str) -> Result<ResourceRecipeSpec, HydrateError> {
    let text = SCENARIO.replace("CAP", cap);
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    let pack = hydrate_scenario(&document)?;
    Ok(pack
        .game_mode
        .resource_economy
        .expect("resource economy")
        .recipes
        .into_iter()
        .find(|recipe| recipe.id.ends_with("ridge_foundry"))
        .expect("foundry recipe"))
}

/// catches: the historical throttle hint silently promoted to authority, an
/// authored cap dropped at lowering, or the canonical interchange changing
/// for recipes that author no cap.
#[test]
fn authored_unit_cap_is_authority_and_the_hint_stays_a_hint() {
    let legacy = foundry("").expect("uncapped building hydrates");
    assert_eq!(legacy.max_units_per_generation, None);
    assert_eq!(legacy.throttle_hint_max_per_tick, 3);
    let legacy_json = serde_json::to_string(&legacy).unwrap();
    assert!(
        !legacy_json.contains("max_units_per_generation"),
        "an uncapped recipe's canonical JSON is unchanged: {legacy_json}"
    );

    let capped = foundry("max_units_per_generation = 1").expect("capped building hydrates");
    assert_eq!(capped.max_units_per_generation, NonZeroU32::new(1));
    assert_eq!(capped.throttle_hint_max_per_tick, 3, "the hint is not promoted");
    let capped_json = serde_json::to_string(&capped).unwrap();
    assert!(capped_json.contains("\"max_units_per_generation\":1"));
    assert_eq!(
        serde_json::from_str::<ResourceRecipeSpec>(&capped_json).unwrap(),
        capped,
        "the canonical interchange round-trips the authority"
    );
}

/// catches: zero, negative, fractional, or repeated cap authority being
/// rounded, defaulted, or last-wins instead of refused before activation — in
/// native source and in the canonical interchange alike.
#[test]
fn malformed_unit_cap_refuses_before_activation() {
    for (cap, expected) in [
        ("max_units_per_generation = 0", "must be a positive integer"),
        ("max_units_per_generation = -1", "must be a non-negative integer literal"),
        ("max_units_per_generation = 1.5", "must be a non-negative integer literal"),
        (
            "max_units_per_generation = 1\n            max_units_per_generation = 2",
            "duplicate production_building field `max_units_per_generation`",
        ),
    ] {
        let error = foundry(cap).expect_err("malformed cap must refuse");
        assert!(error.message.contains(expected), "{cap}: {}", error.message);
        assert!(error.span.is_some(), "{cap}: refusal must carry a span");
    }

    let json = serde_json::to_string(&foundry("max_units_per_generation = 1").unwrap()).unwrap();
    for bad in ["0", "-1", "1.5"] {
        let tampered = json.replace(
            "\"max_units_per_generation\":1",
            &format!("\"max_units_per_generation\":{bad}"),
        );
        assert_ne!(tampered, json);
        assert!(
            serde_json::from_str::<ResourceRecipeSpec>(&tampered).is_err(),
            "interchange cap {bad} must refuse"
        );
    }
}
