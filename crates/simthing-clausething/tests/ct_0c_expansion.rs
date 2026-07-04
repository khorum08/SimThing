//! CT-0c expansion passes: worked plague golden, expansion-order pitfalls,
//! and deterministic diagnostics.
//!
//! All fixtures are original SimThing-authored ClauseScript-shaped text;
//! no Paradox or lab corpus material is used.

use std::collections::BTreeMap;

use simthing_clausething::raw::{RawValue, ScalarForm};
use simthing_clausething::{
    ExpansionInput, RawDocument, expand_document, is_inline_math, is_value_reference,
    parse_raw_document, to_canonical_json,
};

const PLAGUE_MAIN: &str = include_str!("fixtures/expand_plague_main.clause");
const PLAGUE_LIB: &str = include_str!("fixtures/expand_plague_lib_blight_wave.clause");
const SCOPE_UNTOUCHED: &str = include_str!("fixtures/expand_scope_untouched.clause");
const ORDER_MAIN: &str = include_str!("fixtures/expand_include_order_main.clause");
const ORDER_LIB: &str = include_str!("fixtures/expand_include_order_lib.clause");

fn parse(text: &str) -> RawDocument {
    parse_raw_document(text.as_bytes()).expect("parse fixture")
}

fn library(entries: &[(&str, &str)]) -> BTreeMap<String, RawDocument> {
    entries
        .iter()
        .map(|(name, text)| ((*name).to_string(), parse(text)))
        .collect()
}

fn params(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

fn plague_input(qstate: &str) -> ExpansionInput {
    ExpansionInput {
        inline_scripts: library(&[("simthing/blight_wave", PLAGUE_LIB)]),
        parameters: params(&[("WAVE_SCALE", "2"), ("QSTATE", qstate)]),
        ..ExpansionInput::default()
    }
}

fn golden_json(name: &str) -> String {
    let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read golden {path}: {err}"))
}

fn expanded_json(document: &RawDocument, input: &ExpansionInput) -> String {
    let expanded = expand_document(document, input).expect("expand document");
    to_canonical_json(&expanded).expect("serialize expanded document")
}

/// Flattened (key, scalar-value) pairs from a block property list, recursively.
fn scalar_pairs(value: &RawValue, out: &mut Vec<(String, String)>) {
    if let RawValue::Block(block) = value {
        for property in &block.properties {
            if let RawValue::Scalar(scalar) = &property.value {
                out.push((property.key.text.clone(), scalar.text.clone()));
            }
            scalar_pairs(&property.value, out);
        }
    }
}

fn pairs_of(document: &RawDocument) -> Vec<(String, String)> {
    let mut out = Vec::new();
    scalar_pairs(&document.root, &mut out);
    out
}

#[test]
fn worked_plague_quarantine_matches_golden() {
    let actual = expanded_json(&parse(PLAGUE_MAIN), &plague_input("yes"));
    let expected = golden_json("expand_plague_quarantine")
        .trim_end()
        .to_string();
    assert_eq!(actual, expected, "quarantine plague golden mismatch");
}

#[test]
fn worked_plague_open_matches_golden() {
    let actual = expanded_json(&parse(PLAGUE_MAIN), &plague_input("no"));
    let expected = golden_json("expand_plague_open").trim_end().to_string();
    assert_eq!(actual, expected, "open plague golden mismatch");
}

#[test]
fn missing_inline_script_is_a_deterministic_diagnostic() {
    let err = expand_document(&parse(ORDER_MAIN), &ExpansionInput::default())
        .expect_err("library is empty");
    assert_eq!(
        err.message,
        "inline_script target `simthing/order_probe` is not in the synthetic library"
    );
}

#[test]
#[ignore = "developer utility: regenerate CT-0c expansion goldens locally"]
fn write_expansion_goldens() {
    let cases: &[(&str, String)] = &[
        (
            "expand_plague_quarantine",
            expanded_json(&parse(PLAGUE_MAIN), &plague_input("yes")),
        ),
        (
            "expand_plague_open",
            expanded_json(&parse(PLAGUE_MAIN), &plague_input("no")),
        ),
        (
            "expand_scope_untouched",
            expanded_json(&parse(SCOPE_UNTOUCHED), &ExpansionInput::default()),
        ),
    ];
    for (name, json) in cases {
        let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::write(&path, format!("{json}\n")).expect("write golden");
        eprintln!("wrote {path}");
    }
}
