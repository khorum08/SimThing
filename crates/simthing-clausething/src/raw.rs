//! CT-0b lossless ClauseScript raw model (import layer only; no hydration).

use serde::{Deserialize, Serialize};

/// Source location hint for later diagnostics (token index in the parsed tape).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawSpan {
    pub token_index: usize,
}

/// Observable scalar lexical form from the text parser path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScalarForm {
    Quoted,
    Unquoted,
    Parameter,
    UndefinedParameter,
    Header,
}

/// A symbolic scalar value as written or tokenized — never evaluated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawScalar {
    pub form: ScalarForm,
    pub text: String,
    pub span: RawSpan,
}

/// Assignment or comparison operator when exposed by the parser path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RawOperator {
    Equal,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    NotEqual,
    Exact,
    Exists,
}

/// One ordered key/operator/value field (duplicate keys remain distinct entries).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawProperty {
    pub key: RawScalar,
    pub operator: Option<RawOperator>,
    pub value: RawValue,
}

/// Braced object body preserving insertion order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawBlock {
    pub properties: Vec<RawProperty>,
    pub mixed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<RawArray>,
}

/// Braced array body preserving element order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawArray {
    pub items: Vec<RawValue>,
    pub mixed: bool,
}

/// Header token plus payload (e.g. `rgb { 100 200 50 }`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawHeaderValue {
    pub header: RawScalar,
    pub payload: Box<RawValue>,
}

/// Structural value variants at CT-0b (no macro/scope/formula semantics).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RawValue {
    Scalar(RawScalar),
    Block(RawBlock),
    Array(RawArray),
    Header(RawHeaderValue),
}

/// Parsed document root (implicit top-level object).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawDocument {
    pub root: RawValue,
}
