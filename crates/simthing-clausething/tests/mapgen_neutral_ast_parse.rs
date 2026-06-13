//! MapGen PR2 — neutral-AST parse-only adapter tests (M1).

use simthing_clausething::raw::{RawBlock, RawProperty, RawValue};
use simthing_clausething::{MapGenNeutralDocument, parse_mapgen_neutral_document};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

const FORBIDDEN_SEMANTIC_TYPE_NAMES: &[&str] = &[
    "SystemNode",
    "Hyperlane",
    "Deposit",
    "Initializer",
    "SetupScenario",
    "GridCell",
    "RegionCell",
    "MovementFront",
    "PalmaReach",
    "ResourceArena",
    "Commitment",
];

const FORBIDDEN_ENGINE_API_NAMES: &[&str] = &[
    "route",
    "pathfinding",
    "predecessor",
    "movement_order",
    "border_service",
    "frontline",
    "cpu_planner",
    "fleet_path",
    "SEAD",
];

fn root_block(document: &MapGenNeutralDocument) -> &RawBlock {
    let RawValue::Block(block) = &document.document.root else {
        panic!("expected root block");
    };
    block
}

fn block_property<'a>(block: &'a RawBlock, key: &str) -> &'a RawProperty {
    block
        .properties
        .iter()
        .find(|property| property.key.text == key)
        .unwrap_or_else(|| panic!("missing property {key}"))
}

fn block_value<'a>(block: &'a RawBlock, key: &str) -> &'a RawValue {
    &block_property(block, key).value
}

fn property_keys(block: &RawBlock) -> Vec<&str> {
    block
        .properties
        .iter()
        .map(|property| property.key.text.as_str())
        .collect()
}

fn property_values_matching<'a>(block: &'a RawBlock, key: &str) -> Vec<&'a RawValue> {
    block
        .properties
        .iter()
        .filter(|property| property.key.text == key)
        .map(|property| &property.value)
        .collect()
}

fn scalar_text(value: &RawValue) -> &str {
    let RawValue::Scalar(scalar) = value else {
        panic!("expected scalar value, got {value:?}");
    };
    scalar.text.as_str()
}

#[test]
fn parses_tiny_raw_mapgen_fixture() {
    let parsed = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    assert_eq!(parsed.source_byte_len, RAW_FIXTURE.len());
    assert_eq!(
        property_keys(root_block(&parsed)),
        vec!["tiny_pentad_hub_slice_raw"]
    );
}

#[test]
fn preserves_repeated_keys() {
    let parsed = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    let root = root_block(&parsed);
    let RawValue::Block(scenario) = block_value(root, "tiny_pentad_hub_slice_raw") else {
        panic!("expected slice block");
    };
    let RawValue::Block(static_galaxy) = block_value(scenario, "static_galaxy_scenario") else {
        panic!("expected static_galaxy_scenario block");
    };

    let system_ids: Vec<_> = property_values_matching(static_galaxy, "system")
        .into_iter()
        .map(|value| {
            let RawValue::Block(system) = value else {
                panic!("expected system block");
            };
            scalar_text(block_value(system, "id")).to_string()
        })
        .collect();
    assert_eq!(system_ids, vec!["0", "9", "31", "2", "15"]);

    let hyperlanes: Vec<_> = property_values_matching(static_galaxy, "add_hyperlane")
        .into_iter()
        .map(|value| {
            let RawValue::Block(link) = value else {
                panic!("expected hyperlane block");
            };
            (
                scalar_text(block_value(link, "from")).to_string(),
                scalar_text(block_value(link, "to")).to_string(),
            )
        })
        .collect();
    assert_eq!(
        hyperlanes,
        vec![
            ("0".to_string(), "9".to_string()),
            ("0".to_string(), "31".to_string()),
            ("0".to_string(), "2".to_string()),
            ("9".to_string(), "15".to_string()),
            ("31".to_string(), "15".to_string()),
        ]
    );
}

#[test]
fn preserves_nesting() {
    let parsed = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    let root = root_block(&parsed);
    let RawValue::Block(slice) = block_value(root, "tiny_pentad_hub_slice_raw") else {
        panic!("expected slice block");
    };
    let RawValue::Block(static_galaxy) = block_value(slice, "static_galaxy_scenario") else {
        panic!("expected static_galaxy_scenario block");
    };
    let RawValue::Block(first_system) = property_values_matching(static_galaxy, "system")[0] else {
        panic!("expected first system block");
    };
    let RawValue::Block(position) = block_value(first_system, "position") else {
        panic!("expected position block");
    };
    assert_eq!(property_keys(position), vec!["x", "y", "z"]);
    assert_eq!(scalar_text(block_value(position, "x")), "0");

    let RawValue::Block(initializer) = block_value(slice, "example_rim_initializer") else {
        panic!("expected initializer block");
    };
    let RawValue::Block(deposit) = block_value(initializer, "deposit") else {
        panic!("expected deposit block");
    };
    let RawValue::Block(resources) = block_value(deposit, "resources") else {
        panic!("expected resources block");
    };
    assert_eq!(scalar_text(block_value(resources, "minerals")), "4");
}

#[test]
fn preserves_sibling_order_and_count() {
    let parsed = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    let root = root_block(&parsed);
    let RawValue::Block(slice) = block_value(root, "tiny_pentad_hub_slice_raw") else {
        panic!("expected slice block");
    };
    let RawValue::Block(static_galaxy) = block_value(slice, "static_galaxy_scenario") else {
        panic!("expected static_galaxy_scenario block");
    };

    assert_eq!(static_galaxy.properties.len(), 13);
    assert_eq!(
        property_keys(static_galaxy),
        vec![
            "name",
            "random_hyperlanes",
            "system",
            "system",
            "system",
            "system",
            "system",
            "add_hyperlane",
            "add_hyperlane",
            "add_hyperlane",
            "add_hyperlane",
            "add_hyperlane",
            "nebula",
        ]
    );
}

#[test]
fn adapter_output_is_raw_document_only() {
    let parsed = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse fixture");
    assert!(matches!(parsed.document.root, RawValue::Block(_)));
    assert_eq!(parsed.source_byte_len, RAW_FIXTURE.len());
}

#[test]
fn adapter_source_has_no_semantic_mapping_calls() {
    let source = include_str!("../src/mapgen_neutral_ast.rs");
    for forbidden in ["hydrate_scenario", "HydratedScenarioPack", "GameModeSpec"] {
        assert!(
            !source.contains(forbidden),
            "mapgen_neutral_ast.rs must not reference {forbidden}"
        );
    }
}

#[test]
fn forbidden_semantic_type_names_absent_from_module_source() {
    let source = include_str!("../src/mapgen_neutral_ast.rs");
    for name in FORBIDDEN_SEMANTIC_TYPE_NAMES {
        assert!(
            !source.contains(name),
            "forbidden semantic type name present in mapgen_neutral_ast.rs: {name}"
        );
    }
}

#[test]
fn forbidden_engine_vocabulary_absent_from_public_adapter_source() {
    let sources = [
        include_str!("../src/mapgen_neutral_ast.rs"),
        include_str!("../src/lib.rs"),
    ];
    for source in sources {
        for name in FORBIDDEN_ENGINE_API_NAMES {
            assert!(
                !source.contains(name),
                "forbidden engine vocabulary present in adapter source: {name}"
            );
        }
    }
}
