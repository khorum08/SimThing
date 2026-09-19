//! Native sub-field clamp projection (DA, relay 5745626815): an authored cell
//! projects the EXISTING core `ClampBehavior` directly. Omission stays
//! `Unbounded`, so every historical document is unchanged, and every malformed
//! bound refuses before activation with its span.
use simthing_clausething::{hydrate_scenario, parse_raw_document, HydrateError};
use simthing_core::{ClampBehavior, SubFieldRole};

const SCENARIO: &str = r#"
scenario = clamp_valley {
    owner = guild {
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }
    location = ridge {
        display_name = "Ridge"
        properties = { property = {
          id = ridge_stock
          namespace = forge
          name = stock
          display_name = "Stock"
          sub_field = { role = Amount }
          sub_field = { role = balance_rate }
          sub_field = { role = balance governed_by = balance_rate accumulator = Balance CLAMP }
        } }
    }
}
"#;

fn hydrate(clamp: &str) -> Result<Vec<(SubFieldRole, ClampBehavior)>, HydrateError> {
    let text = SCENARIO.replace("CLAMP", clamp);
    let document = parse_raw_document(text.as_bytes()).expect("parse ClauseScript");
    let pack = hydrate_scenario(&document)?;
    let property = pack
        .game_mode
        .properties
        .into_iter()
        .find(|property| property.id == "ridge_stock")
        .expect("authored property");
    Ok(property
        .sub_fields
        .into_iter()
        .map(|sub| (sub.role, sub.clamp))
        .collect())
}

fn governed_clamp(clamp: &str) -> ClampBehavior {
    let cells = hydrate(clamp).expect("authored clamp admits");
    let named = |role: &SubFieldRole| matches!(role, SubFieldRole::Named(name) if name == "balance");
    let (_, clamp) = cells
        .into_iter()
        .find(|(role, _)| named(role))
        .expect("governed balance cell");
    clamp
}

/// catches: an authored finite bound dropped, reshaped, or silently widened;
/// omission changing the historical `Unbounded` default; or a degenerate but
/// lawful `min == max` bound refused.
#[test]
fn authored_clamp_projects_the_existing_core_behaviour() {
    assert_eq!(
        governed_clamp("clamp = Bounded { min = 0 max = 24 }"),
        ClampBehavior::Bounded {
            min: 0.0,
            max: 24.0
        }
    );
    assert_eq!(
        governed_clamp("clamp = Bounded { min = 3 max = 3 }"),
        ClampBehavior::Bounded { min: 3.0, max: 3.0 },
        "a degenerate but ordered bound is lawful"
    );
    assert_eq!(
        governed_clamp("clamp = Floored { min = -2.5 }"),
        ClampBehavior::Floored { min: -2.5 }
    );
    assert_eq!(governed_clamp("clamp = Unbounded"), ClampBehavior::Unbounded);

    // Omission is the historical default, and no other cell moves.
    let omitted = hydrate("").expect("a document without clamp still admits");
    assert!(omitted
        .iter()
        .all(|(_, clamp)| *clamp == ClampBehavior::Unbounded));
    let bounded = hydrate("clamp = Bounded { min = 0 max = 24 }").expect("bounded admits");
    assert_eq!(
        omitted.iter().map(|(role, _)| role).collect::<Vec<_>>(),
        bounded.iter().map(|(role, _)| role).collect::<Vec<_>>(),
        "the bound changes only the clamp of the cell that declares it"
    );
    assert_eq!(
        bounded
            .iter()
            .filter(|(_, clamp)| *clamp == ClampBehavior::Unbounded)
            .count(),
        omitted.len() - 1
    );
}

/// catches: a reversed, partial, non-finite, misspelled, duplicated or
/// over-specified bound being defaulted, coerced or silently dropped instead of
/// refusing before activation.
#[test]
fn malformed_clamp_forms_refuse_before_activation() {
    for (clamp, expected) in [
        ("clamp = Bounded { min = 5 max = 1 }", "min <= max"),
        ("clamp = Bounded { min = 0 }", "Bounded clamp requires max"),
        ("clamp = Bounded { max = 1 }", "Bounded clamp requires min"),
        (
            "clamp = Floored { min = 0 max = 1 }",
            "Floored clamp declares min only",
        ),
        // An empty block degenerates to a scalar at the parse layer, so the
        // scalar arm refuses it; either way no bound is invented.
        ("clamp = Floored { }", "unsupported clamp `Floored`"),
        ("clamp = Ceiling { max = 1 }", "unsupported clamp `Ceiling`"),
        ("clamp = Sealed", "unsupported clamp `Sealed`"),
        (
            "clamp = Bounded { min = 0 max = 1e40 }",
            "must be finite",
        ),
        (
            "clamp = Bounded { min = 0 max = 1 lo = 2 }",
            "unsupported clamp bound `lo`",
        ),
        ("clamp = Bounded { min = 0 min = 1 }", "duplicate clamp bound"),
        (
            "clamp = Unbounded clamp = Unbounded",
            "duplicate sub_field declaration",
        ),
    ] {
        let error = match hydrate(clamp) {
            Ok(cells) => panic!("must refuse `{clamp}`, admitted {cells:?}"),
            Err(error) => error.to_string(),
        };
        println!("refused ({expected}): {error}");
        assert!(error.contains(expected), "{clamp}: {error}");
        assert!(
            error.contains("token"),
            "{clamp}: refusal carries no span: {error}"
        );
    }
}
