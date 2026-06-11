//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser.
//! CT-0b: lossless raw model, JSON goldens, and safe synthetic round-trip.
//! CT-0c: expansion passes (@vars, inline_script, $PARAM$, [[PARAM]] conditionals)
//! with `@[ ]` inline math preserved symbolically and `value:` left symbolic.
//! CT-0d: symbolic scope-chain extraction and lab-only frequency evidence.
//! No hydration to `simthing-spec`, no runtime wiring, default-off.

pub mod error;
pub mod expand;
pub mod jomini;
pub mod raw;
pub mod scope;

mod emit;
mod json;
mod parse;
mod scope_json;
mod scope_lab;

pub use emit::emit_text;
pub use error::{EmitError, ExpandError, ParseError};
pub use expand::{
    ExpansionInput, ExpansionOptions, expand_document, is_inline_math, is_value_reference,
};
pub use jomini::{TextTape, TextToken};
pub use json::to_canonical_json;
pub use parse::parse_raw_document;
pub use raw::RawDocument;
pub use scope::{
    ScopeAtom, ScopeAtomKind, ScopeChain, ScopeDiagnostic, ScopeDiagnosticKind,
    ScopeExtractionReport, ScopeReference, ScopeReferenceRole, ScopeTable, extract_scopes,
    extract_scopes_validated, parse_scope_chain, synthetic_scope_table,
};
pub use scope_json::scope_report_to_json;
pub use scope_lab::{LabFrequencyReport, scan_lab_scopes};
