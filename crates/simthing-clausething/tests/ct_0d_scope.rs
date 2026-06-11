//! CT-0d scope-chain extraction, validation, and lab-only frequency scan.
//!
//! All fixtures are original SimThing-authored ClauseScript-shaped text.

use std::collections::BTreeMap;
use std::path::Path;

use simthing_clausething::{
    ExpansionInput, ScopeDiagnosticKind, ScopeReferenceRole, expand_document, extract_scopes,
    extract_scopes_validated, parse_raw_document, scan_lab_scopes, scope_report_to_json,
    synthetic_scope_table,
};

const SCOPE_BASIC: &str = include_str!("fixtures/scope_basic.clause");
const SCOPE_CHAINS: &str = include_str!("fixtures/scope_chains.clause");
const SCOPE_EVENT_TARGET: &str = include_str!("fixtures/scope_event_target.clause");
const SCOPE_MALFORMED: &str = include_str!("fixtures/scope_malformed.clause");
const SCOPE_UNKNOWN: &str = include_str!("fixtures/scope_unknown_domain.clause");
const SCOPE_ORDER: &str = include_str!("fixtures/scope_order.clause");
const SCOPE_POST_MAIN: &str = include_str!("fixtures/scope_post_expand_main.clause");
const SCOPE_POST_LIB: &str = include_str!("fixtures/scope_post_expand_lib.clause");
const SCOPE_UNTOUCHED: &str = include_str!("fixtures/expand_scope_untouched.clause");

fn golden_json(name: &str) -> String {
    let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read golden {path}: {err}"))
}

fn expanded_post_expand() -> simthing_clausething::RawDocument {
    let document = parse_raw_document(SCOPE_POST_MAIN.as_bytes()).expect("parse post-expand main");
    let input = ExpansionInput {
        inline_scripts: BTreeMap::from([(
            "simthing/scope_wave".to_string(),
            parse_raw_document(SCOPE_POST_LIB.as_bytes()).expect("parse post-expand lib"),
        )]),
        parameters: BTreeMap::from([("WAVE".to_string(), "2".to_string())]),
        ..ExpansionInput::default()
    };
    expand_document(&document, &input).expect("expand post-expand fixture")
}

fn report_json_from_fixture(text: &str) -> String {
    let document = parse_raw_document(text.as_bytes()).expect("parse fixture");
    let report = extract_scopes_validated(&document, &synthetic_scope_table());
    scope_report_to_json(&report).expect("serialize report")
}

#[test]
fn scope_basic_matches_golden() {
    let actual = report_json_from_fixture(SCOPE_BASIC).trim_end().to_string();
    let expected = golden_json("scope_basic").trim_end().to_string();
    assert_eq!(actual, expected, "scope_basic golden mismatch");
}

#[test]
fn scope_chains_matches_golden() {
    let actual = report_json_from_fixture(SCOPE_CHAINS)
        .trim_end()
        .to_string();
    let expected = golden_json("scope_chains").trim_end().to_string();
    assert_eq!(actual, expected, "scope_chains golden mismatch");
}

#[test]
fn scope_post_expand_matches_golden() {
    let expanded = expanded_post_expand();
    let report = extract_scopes_validated(&expanded, &synthetic_scope_table());
    let actual = scope_report_to_json(&report)
        .expect("serialize post-expand report")
        .trim_end()
        .to_string();
    let expected = golden_json("scope_post_expand").trim_end().to_string();
    assert_eq!(actual, expected, "scope_post_expand golden mismatch");
}

#[test]
fn extraction_runs_after_ct_0c_expansion() {
    let pre = parse_raw_document(SCOPE_UNTOUCHED.as_bytes()).expect("parse scope untouched");
    let post = expand_document(&pre, &ExpansionInput::default()).expect("expand scope untouched");
    let pre_report = extract_scopes(&pre);
    let post_report = extract_scopes(&post);
    assert_eq!(
        scope_report_to_json(&pre_report).unwrap(),
        scope_report_to_json(&post_report).unwrap(),
        "expansion must not alter scope extraction for scope-like text"
    );
    assert!(
        !post_report.references.is_empty(),
        "post-expansion extraction should find scope references"
    );
}

#[test]
fn malformed_chains_emit_deterministic_diagnostics() {
    let document = parse_raw_document(SCOPE_MALFORMED.as_bytes()).expect("parse malformed");
    let report = extract_scopes(&document);
    assert_eq!(report.diagnostics.len(), 2);
    assert!(
        report
            .diagnostics
            .iter()
            .all(|d| d.kind == ScopeDiagnosticKind::MalformedChain)
    );
    assert_eq!(
        report.diagnostics[0].message,
        "malformed scope chain `root..owner`: empty dot segment"
    );
    assert_eq!(
        report.diagnostics[1].message,
        "malformed scope chain `.from`: empty dot segment"
    );
    assert!(report.diagnostics[0].span.is_some());
}

#[test]
fn unknown_domain_scope_is_not_silently_accepted() {
    let document = parse_raw_document(SCOPE_UNKNOWN.as_bytes()).expect("parse unknown");
    let report = extract_scopes_validated(&document, &synthetic_scope_table());
    assert_eq!(report.diagnostics.len(), 1);
    assert_eq!(
        report.diagnostics[0].kind,
        ScopeDiagnosticKind::UnknownDomainScope
    );
    assert!(
        report
            .references
            .iter()
            .any(|r| r.role == ScopeReferenceRole::BlockScopeKey)
    );
}

#[test]
fn event_target_references_are_symbolic() {
    let document =
        parse_raw_document(SCOPE_EVENT_TARGET.as_bytes()).expect("parse event_target fixture");
    let report = extract_scopes(&document);
    assert!(
        report
            .references
            .iter()
            .any(|r| r.role == ScopeReferenceRole::EventTargetValue)
    );
    assert!(
        report
            .references
            .iter()
            .any(|r| r.chain.raw_text.contains("event_target:"))
    );
}

#[test]
fn extraction_preserves_source_order() {
    let document = parse_raw_document(SCOPE_ORDER.as_bytes()).expect("parse order fixture");
    let report = extract_scopes(&document);
    let raw_texts: Vec<_> = report
        .references
        .iter()
        .map(|r| r.chain.raw_text.as_str())
        .collect();
    assert_eq!(
        raw_texts,
        vec!["this", "root", "from", "owner", "prev"],
        "source-order extraction mismatch"
    );
}

#[test]
fn dot_paths_have_no_runtime_slot_resolution() {
    let document = parse_raw_document(SCOPE_CHAINS.as_bytes()).expect("parse chains");
    let report = extract_scopes(&document);
    let relay = report
        .references
        .iter()
        .find(|r| r.chain.raw_text == "root.owner.capital_scope")
        .expect("relay chain");
    assert_eq!(relay.chain.atoms.len(), 3);
    assert!(relay.chain.atoms[1].raw_text == "owner");
}

#[test]
#[ignore = "developer utility: regenerate CT-0d scope JSON goldens locally"]
fn write_scope_goldens() {
    for (name, text) in [("scope_basic", SCOPE_BASIC), ("scope_chains", SCOPE_CHAINS)] {
        let json = report_json_from_fixture(text);
        let path = format!("{}/tests/goldens/{}.json", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::write(&path, format!("{json}\n")).expect("write golden");
        eprintln!("wrote {path}");
    }
    let expanded = expanded_post_expand();
    let report = extract_scopes_validated(&expanded, &synthetic_scope_table());
    let json = scope_report_to_json(&report).expect("serialize");
    let path = format!(
        "{}/tests/goldens/scope_post_expand.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::write(&path, format!("{json}\n")).expect("write golden");
    eprintln!("wrote {path}");
}

#[test]
#[ignore = "lab-only scopes.log aggregate scan; requires CLAUSER_LAB_DIR"]
fn lab_scopes_log_frequency_scan() {
    let lab_dir = std::env::var("CLAUSER_LAB_DIR").expect("CLAUSER_LAB_DIR must be set");
    let report = scan_lab_scopes(Path::new(&lab_dir));
    assert!(
        report.scopes_log_found,
        "scopes.log not found under CLAUSER_LAB_DIR"
    );
    assert!(
        report.total_scope_names > 0,
        "expected non-zero scope names"
    );
    assert!(
        !report.output_scope_counts.is_empty(),
        "expected output scope aggregate counts"
    );
    eprintln!("lab scope names: {}", report.total_scope_names);
    eprintln!("lab output classes: {}", report.output_scope_counts.len());
    eprintln!(
        "lab supported relations: {}",
        report.supported_relation_count
    );
}
