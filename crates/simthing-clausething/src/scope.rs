//! CT-0d symbolic scope-chain extraction over expanded raw documents.
//!
//! Designer-layer only: no runtime slot resolution, no `simthing-spec` hydration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::expand::{is_inline_math, is_value_reference};
use crate::raw::{RawArray, RawBlock, RawDocument, RawScalar, RawSpan, RawValue};

/// One symbolic step in a scope navigation chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScopeAtomKind {
    This,
    Root,
    From { repeat: usize },
    Prev { repeat: usize },
    Domain { name: String },
    EventTarget { name: String },
    Unknown { text: String },
}

/// A parsed scope atom with preserved raw text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeAtom {
    pub kind: ScopeAtomKind,
    pub raw_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<RawSpan>,
}

/// Dot-path or navigation chain (e.g. `root.owner.capital_relay`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeChain {
    pub atoms: Vec<ScopeAtom>,
    pub raw_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<RawSpan>,
}

/// Where a scope reference was observed in the expanded document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReferenceRole {
    BlockScopeKey,
    ScalarPath,
    EventTargetValue,
}

/// One extracted scope reference in source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReference {
    pub role: ScopeReferenceRole,
    pub chain: ScopeChain,
    pub context_path: Vec<String>,
}

/// Deterministic extraction/validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiagnostic {
    pub kind: ScopeDiagnosticKind,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<RawSpan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDiagnosticKind {
    MalformedChain,
    UnknownDomainScope,
    UnsupportedForm,
}

/// Aggregate extraction output for a document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeExtractionReport {
    pub references: Vec<ScopeReference>,
    pub diagnostics: Vec<ScopeDiagnostic>,
}

/// Supported/Output metadata for safe synthetic scope validation (always-on tests).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeTableEntry {
    pub output: String,
    pub supported: Vec<String>,
}

/// Designer-layer scope table used for CT-0d validation only.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScopeTable {
    pub scopes: BTreeMap<String, ScopeTableEntry>,
}

impl ScopeTable {
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        output: impl Into<String>,
        supported: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.scopes.insert(
            name.into(),
            ScopeTableEntry {
                output: output.into(),
                supported: supported.into_iter().map(Into::into).collect(),
            },
        );
    }
}

/// Default synthetic scope table for always-on CT-0d tests.
pub fn synthetic_scope_table() -> ScopeTable {
    let mut table = ScopeTable::default();
    table.insert("owner", "country", ["planet", "ship", "fleet"]);
    table.insert("capital_scope", "planet", ["country", "owner"]);
    table.insert("station", "starbase", ["solar_system", "planet"]);
    table.insert("solar_system", "solar_system", ["galaxy", "country"]);
    table
}

/// Extract symbolic scope references from an expanded [`RawDocument`] in source order.
pub fn extract_scopes(document: &RawDocument) -> ScopeExtractionReport {
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut path = Vec::new();
    walk_value(
        &document.root,
        &mut path,
        &mut references,
        &mut diagnostics,
        None,
    );
    ScopeExtractionReport {
        references,
        diagnostics,
    }
}

/// Extract scopes and validate domain block keys against a scope table.
pub fn extract_scopes_validated(
    document: &RawDocument,
    table: &ScopeTable,
) -> ScopeExtractionReport {
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut path = Vec::new();
    walk_value(
        &document.root,
        &mut path,
        &mut references,
        &mut diagnostics,
        Some(table),
    );
    ScopeExtractionReport {
        references,
        diagnostics,
    }
}

fn walk_value(
    value: &RawValue,
    path: &mut Vec<String>,
    references: &mut Vec<ScopeReference>,
    diagnostics: &mut Vec<ScopeDiagnostic>,
    table: Option<&ScopeTable>,
) {
    match value {
        RawValue::Block(block) => walk_block(block, path, references, diagnostics, table),
        RawValue::Array(array) => {
            for item in &array.items {
                walk_value(item, path, references, diagnostics, table);
            }
        }
        RawValue::Header(header) => {
            walk_value(&header.payload, path, references, diagnostics, table);
        }
        RawValue::Scalar(_) => {}
    }
}

fn walk_block(
    block: &RawBlock,
    path: &mut Vec<String>,
    references: &mut Vec<ScopeReference>,
    diagnostics: &mut Vec<ScopeDiagnostic>,
    table: Option<&ScopeTable>,
) {
    for property in &block.properties {
        let key_text = property.key.text.as_str();

        if let RawValue::Block(_) = &property.value {
            // Top-level entity/template keys are not domain scope transitions.
            if !path.is_empty() && is_domain_scope_key(key_text) {
                let chain = ScopeChain {
                    atoms: vec![ScopeAtom {
                        kind: ScopeAtomKind::Domain {
                            name: key_text.to_string(),
                        },
                        raw_text: key_text.to_string(),
                        span: Some(property.key.span.clone()),
                    }],
                    raw_text: key_text.to_string(),
                    span: Some(property.key.span.clone()),
                };
                references.push(ScopeReference {
                    role: ScopeReferenceRole::BlockScopeKey,
                    chain,
                    context_path: path.clone(),
                });
                if let Some(table) = table {
                    validate_domain_key(key_text, &property.key.span, diagnostics, table);
                }
            }
        }

        if let RawValue::Scalar(scalar) = &property.value {
            extract_scalar_reference(scalar, path, references, diagnostics);
        }

        path.push(key_text.to_string());
        walk_value(&property.value, path, references, diagnostics, table);
        path.pop();
    }

    if let Some(tail) = &block.tail {
        walk_array(tail, path, references, diagnostics, table);
    }
}

fn walk_array(
    array: &RawArray,
    path: &mut Vec<String>,
    references: &mut Vec<ScopeReference>,
    diagnostics: &mut Vec<ScopeDiagnostic>,
    table: Option<&ScopeTable>,
) {
    for item in &array.items {
        walk_value(item, path, references, diagnostics, table);
    }
}

fn extract_scalar_reference(
    scalar: &RawScalar,
    path: &[String],
    references: &mut Vec<ScopeReference>,
    diagnostics: &mut Vec<ScopeDiagnostic>,
) {
    if is_value_reference(scalar) || is_inline_math(scalar) {
        return;
    }

    if let Some(target) = parse_event_target_scalar(scalar.text.as_str()) {
        references.push(ScopeReference {
            role: ScopeReferenceRole::EventTargetValue,
            chain: ScopeChain {
                atoms: vec![ScopeAtom {
                    kind: ScopeAtomKind::EventTarget {
                        name: target.to_string(),
                    },
                    raw_text: scalar.text.clone(),
                    span: Some(scalar.span.clone()),
                }],
                raw_text: scalar.text.clone(),
                span: Some(scalar.span.clone()),
            },
            context_path: path.to_vec(),
        });
        return;
    }

    if !looks_like_scope_chain(scalar.text.as_str()) {
        return;
    }

    match parse_scope_chain(scalar.text.as_str(), Some(scalar.span.clone())) {
        Ok(chain) => references.push(ScopeReference {
            role: ScopeReferenceRole::ScalarPath,
            chain,
            context_path: path.to_vec(),
        }),
        Err(message) => diagnostics.push(ScopeDiagnostic {
            kind: ScopeDiagnosticKind::MalformedChain,
            message,
            span: Some(scalar.span.clone()),
            raw_text: Some(scalar.text.clone()),
        }),
    }
}

fn validate_domain_key(
    key: &str,
    span: &RawSpan,
    diagnostics: &mut Vec<ScopeDiagnostic>,
    table: &ScopeTable,
) {
    if table.scopes.contains_key(key) {
        return;
    }
    diagnostics.push(ScopeDiagnostic {
        kind: ScopeDiagnosticKind::UnknownDomainScope,
        message: format!("unknown domain scope `{key}` (not in validation table)"),
        span: Some(span.clone()),
        raw_text: Some(key.to_string()),
    });
}

fn is_domain_scope_key(key: &str) -> bool {
    if RESERVED_BLOCK_KEYS.contains(&key) {
        return false;
    }
    is_identifier(key)
}

const RESERVED_BLOCK_KEYS: &[&str] = &[
    "inline_script",
    "script",
    "limit",
    "modifier",
    "potential",
    "allow",
    "possible",
    "trigger",
    "effect",
    "if",
    "else",
    "else_if",
    "switch",
    "while",
    "hidden_effect",
    "custom_tooltip",
    "lockdown",
    "decay",
    "surge",
];

fn looks_like_scope_chain(text: &str) -> bool {
    if matches!(text, "this" | "root") {
        return true;
    }
    if text.starts_with("from") || text.starts_with("prev") {
        return true;
    }
    text.contains('.')
}

fn parse_event_target_scalar(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("event_target:")?;
    if rest.is_empty() || !is_identifier(rest) {
        return None;
    }
    Some(rest)
}

/// Parse a dot-separated scope path into symbolic atoms.
pub fn parse_scope_chain(text: &str, span: Option<RawSpan>) -> Result<ScopeChain, String> {
    if text.is_empty() {
        return Err("empty scope chain".to_string());
    }

    if let Some(target) = parse_event_target_scalar(text) {
        return Ok(ScopeChain {
            atoms: vec![ScopeAtom {
                kind: ScopeAtomKind::EventTarget {
                    name: target.to_string(),
                },
                raw_text: text.to_string(),
                span: span.clone(),
            }],
            raw_text: text.to_string(),
            span,
        });
    }

    let segments: Vec<&str> = text.split('.').collect();
    if segments.iter().any(|segment| segment.is_empty()) {
        return Err(format!("malformed scope chain `{text}`: empty dot segment"));
    }

    let mut atoms = Vec::with_capacity(segments.len());
    for segment in segments {
        atoms.push(parse_scope_atom(segment)?);
    }

    Ok(ScopeChain {
        atoms,
        raw_text: text.to_string(),
        span,
    })
}

fn parse_scope_atom(segment: &str) -> Result<ScopeAtom, String> {
    if segment == "this" {
        return Ok(atom(segment, ScopeAtomKind::This));
    }
    if segment == "root" {
        return Ok(atom(segment, ScopeAtomKind::Root));
    }
    if let Some(repeat) = count_repeat_prefix(segment, "from") {
        return Ok(atom(
            segment,
            ScopeAtomKind::From {
                repeat: repeat.max(1),
            },
        ));
    }
    if let Some(repeat) = count_repeat_prefix(segment, "prev") {
        return Ok(atom(
            segment,
            ScopeAtomKind::Prev {
                repeat: repeat.max(1),
            },
        ));
    }
    if let Some(name) = segment.strip_prefix("event_target:") {
        if is_identifier(name) {
            return Ok(atom(
                segment,
                ScopeAtomKind::EventTarget {
                    name: name.to_string(),
                },
            ));
        }
        return Err(format!("malformed event_target segment `{segment}`"));
    }
    if is_identifier(segment) {
        return Ok(atom(
            segment,
            ScopeAtomKind::Domain {
                name: segment.to_string(),
            },
        ));
    }
    Err(format!("unsupported scope atom `{segment}`"))
}

fn atom(raw_text: &str, kind: ScopeAtomKind) -> ScopeAtom {
    ScopeAtom {
        kind,
        raw_text: raw_text.to_string(),
        span: None,
    }
}

fn count_repeat_prefix(segment: &str, prefix: &str) -> Option<usize> {
    let mut rest = segment;
    let mut count = 0usize;
    while rest.starts_with(prefix) {
        count += 1;
        rest = &rest[prefix.len()..];
    }
    if rest.is_empty() && count > 0 {
        Some(count)
    } else {
        None
    }
}

fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
