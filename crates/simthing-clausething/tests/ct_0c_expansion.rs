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
fn scope_like_text_is_not_resolved_by_expansion() {
    let document = parse(SCOPE_UNTOUCHED);
    let expanded =
        expand_document(&document, &ExpansionInput::default()).expect("expand scope fixture");
    assert_eq!(
        to_canonical_json(&document).unwrap(),
        to_canonical_json(&expanded).unwrap(),
        "expansion must not rewrite scope-like text"
    );
    let actual = to_canonical_json(&expanded).unwrap();
    let expected = golden_json("expand_scope_untouched").trim_end().to_string();
    assert_eq!(actual, expected, "scope-untouched golden mismatch");
}

#[test]
fn parameters_expand_inside_included_content() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let pairs = pairs_of(&expanded);
    assert!(
        pairs.contains(&("wave_strength".to_string(), "3".to_string())),
        "call parameter SEVERITY must substitute inside included content: {pairs:?}"
    );
}

#[test]
fn provided_parameter_selects_present_branch() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let keys: Vec<String> = pairs_of(&expanded).into_iter().map(|(k, _)| k).collect();
    assert!(keys.contains(&"throughput_mult".to_string()));
    assert!(!keys.contains(&"spread_mult".to_string()));
}

#[test]
fn omitted_parameter_selects_absent_branch() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("no")).expect("expand plague");
    let keys: Vec<String> = pairs_of(&expanded).into_iter().map(|(k, _)| k).collect();
    assert!(keys.contains(&"spread_mult".to_string()));
    assert!(!keys.contains(&"throughput_mult".to_string()));
    assert!(!keys.contains(&"lockdown".to_string()));
}

#[test]
fn included_content_preserves_call_site_order() {
    let input = ExpansionInput {
        inline_scripts: library(&[("simthing/order_probe", ORDER_LIB)]),
        ..ExpansionInput::default()
    };
    let expanded = expand_document(&parse(ORDER_MAIN), &input).expect("expand order fixture");
    let RawValue::Block(root) = &expanded.root else {
        panic!("expected block root");
    };
    let keys: Vec<&str> = root
        .properties
        .iter()
        .map(|p| p.key.text.as_str())
        .collect();
    assert_eq!(
        keys,
        vec!["before_marker", "probe_alpha", "probe_beta", "after_marker"],
        "include must splice at the call site in source order"
    );
}

#[test]
fn ordered_duplication_survives_expansion() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let pairs = pairs_of(&expanded);
    let decay_rates: Vec<&str> = pairs
        .iter()
        .filter(|(k, _)| k == "rate")
        .map(|(_, v)| v.as_str())
        .collect();
    assert_eq!(
        decay_rates,
        vec!["0.04", "0.01"],
        "decay duplicates: {pairs:?}"
    );
    let tiers: Vec<&str> = pairs
        .iter()
        .filter(|(k, _)| k == "tier")
        .map(|(_, v)| v.as_str())
        .collect();
    assert_eq!(
        tiers,
        vec!["3", "fallback"],
        "include duplicates: {pairs:?}"
    );
}

#[test]
fn scripted_variable_defined_locally_substitutes_and_strips() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let pairs = pairs_of(&expanded);
    assert!(pairs.contains(&("echo_rate".to_string(), "0.04".to_string())));
    let keys: Vec<String> = pairs.into_iter().map(|(k, _)| k).collect();
    assert!(
        !keys.iter().any(|k| k == "@blight_base_rate"),
        "definition property must be stripped: {keys:?}"
    );
}

#[test]
fn document_local_variable_overrides_synthetic_and_unknown_stays_symbolic() {
    let text = "@speed = 9\nuses = @speed\nmystery = @unknown_form";
    let mut input = ExpansionInput::default();
    input
        .variables
        .insert("@speed".to_string(), "1".to_string());
    let expanded = expand_document(&parse(text), &input).expect("expand vars");
    let pairs = pairs_of(&expanded);
    assert!(pairs.contains(&("uses".to_string(), "9".to_string())));
    assert!(pairs.contains(&("mystery".to_string(), "@unknown_form".to_string())));
}

#[test]
fn value_reference_remains_symbolic() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let RawValue::Block(root) = &expanded.root else {
        panic!("expected block root");
    };
    let RawValue::Block(outbreak) = &root.properties[0].value else {
        panic!("expected outbreak block");
    };
    let mortality = outbreak
        .properties
        .iter()
        .find(|p| p.key.text == "mortality")
        .expect("mortality property");
    let RawValue::Scalar(scalar) = &mortality.value else {
        panic!("expected scalar mortality");
    };
    assert_eq!(scalar.text, "value:blight_mortality");
    assert_eq!(scalar.form, ScalarForm::Unquoted);
    assert!(is_value_reference(scalar));
}

#[test]
fn inline_math_is_preserved_not_evaluated() {
    let expanded =
        expand_document(&parse(PLAGUE_MAIN), &plague_input("yes")).expect("expand plague");
    let pairs = pairs_of(&expanded);
    assert!(
        pairs.contains(&("casualty_estimate".to_string(), "@[ 3 * 12 ]".to_string())),
        "spaced inline math must keep substituted symbolic text: {pairs:?}"
    );
    assert!(
        pairs.contains(&("surge".to_string(), "@[100*2]".to_string())),
        "tight inline math must keep substituted symbolic text: {pairs:?}"
    );
    let RawValue::Block(root) = &expanded.root else {
        panic!("expected block root");
    };
    let RawValue::Block(outbreak) = &root.properties[0].value else {
        panic!("expected outbreak block");
    };
    let surge = outbreak
        .properties
        .iter()
        .find(|p| p.key.text == "surge")
        .expect("surge property");
    let RawValue::Scalar(scalar) = &surge.value else {
        panic!("expected scalar surge");
    };
    assert!(is_inline_math(scalar));
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
fn inline_depth_cap_is_enforced() {
    let mut input = ExpansionInput {
        inline_scripts: library(&[
            ("chain/a", "inline_script = \"chain/b\""),
            ("chain/b", "inline_script = \"chain/c\""),
            ("chain/c", "leaf = yes"),
        ]),
        ..ExpansionInput::default()
    };
    input.options.max_inline_depth = 2;
    let err = expand_document(&parse("inline_script = \"chain/a\""), &input)
        .expect_err("depth cap must trip");
    assert!(
        err.message.contains("depth cap 2 exceeded"),
        "{}",
        err.message
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
