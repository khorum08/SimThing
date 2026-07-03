//! CT-0b lossless raw model: JSON goldens and parse → emit → reparse round-trip.
//!
//! All fixtures are original SimThing-authored ClauseScript-shaped text.

use simthing_clausething::{emit_text, parse_raw_document, to_canonical_json};

const FIXTURES: &[(&str, &str)] = &[
    (
        "duplicate_keys",
        include_str!("fixtures/duplicate_keys.clause"),
    ),
    (
        "quoted_scalars",
        include_str!("fixtures/quoted_scalars.clause"),
    ),
    (
        "nested_blocks",
        include_str!("fixtures/nested_blocks.clause"),
    ),
    (
        "mixed_siblings",
        include_str!("fixtures/mixed_siblings.clause"),
    ),
    (
        "repeated_nested",
        include_str!("fixtures/repeated_nested.clause"),
    ),
    ("operators", include_str!("fixtures/operators.clause")),
    ("header_value", include_str!("fixtures/header_value.clause")),
    (
        "mixed_container",
        include_str!("fixtures/mixed_container.clause"),
    ),
];

fn golden_json(name: &str) -> String {
    let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read golden {path}: {err}"))
}

fn canonical_from_fixture(text: &str) -> String {
    let document = parse_raw_document(text.as_bytes()).expect("parse fixture");
    to_canonical_json(&document).expect("serialize fixture")
}

#[test]
fn parse_matches_json_golden() {
    for (name, text) in FIXTURES {
        let actual = canonical_from_fixture(text);
        let expected = golden_json(name).trim_end().to_string();
        assert_eq!(actual, expected, "JSON golden mismatch for {name}");
    }
}

#[test]
fn parse_emit_reparse_matches_canonical_json() {
    for (name, text) in FIXTURES {
        let first = canonical_from_fixture(text);
        let document = parse_raw_document(text.as_bytes()).expect("parse fixture");
        let emitted = emit_text(&document).expect("emit fixture");
        let reparsed = canonical_from_fixture(std::str::from_utf8(&emitted).expect("utf8 emit"));
        assert_eq!(first, reparsed, "round-trip JSON mismatch for {name}");
    }
}

#[test]
#[ignore = "developer utility: regenerate JSON goldens locally"]
fn write_goldens() {
    for (name, text) in FIXTURES {
        let json = canonical_from_fixture(text);
        let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::write(&path, format!("{json}\n")).expect("write golden");
        eprintln!("wrote {path}");
    }
}
