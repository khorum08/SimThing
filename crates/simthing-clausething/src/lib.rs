//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser.
//! CT-0b: lossless raw model, JSON goldens, and safe synthetic round-trip.
//! CT-0c: expansion passes (@vars, inline_script, $PARAM$, [[PARAM]] conditionals)
//! with `@[ ]` inline math preserved symbolically and `value:` left symbolic.
//! CT-0d: symbolic scope-chain extraction and lab-only frequency evidence.
//! CT-1a: literal entity hydration into existing `simthing-spec` authoring structs.
//! CT-2a: literal `produces`/`upkeep` hydration into Resource Flow authoring.
//! CT-2c: category economy hydration into Resource Flow and ResourceEconomy authoring.
//! No runtime wiring, default-off.

pub mod error;
pub mod expand;
pub mod jomini;
pub mod raw;
pub mod scope;

mod emit;
mod hydrate;
mod hydrate_category_economy;
mod hydrate_field_operator;
mod hydrate_resource_flow;
mod json;
mod literal_install;
mod parse;
mod scope_json;
mod scope_lab;

pub use emit::emit_text;
pub use error::{EmitError, ExpandError, HydrateError, ParseError};
pub use expand::{
    ExpansionInput, ExpansionOptions, expand_document, is_inline_math, is_value_reference,
};
pub use hydrate::{HydratedEntityPack, hydrate_entity_pack};
pub use hydrate_category_economy::{
    CategoryFlowContribution, DecodedEconomicKey, EconomicAxis, EconomicOp,
    HydratedCategoryEconomyPack, decode_economic_modifier_key, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode,
};
pub use hydrate_field_operator::{
    BH3_MAX_FIELD_IMPEDANCE_PROFILES, BH3_MAX_FIELD_STRESS_PROFILES, HydratedFieldOperatorPack,
    hydrate_field_operator_pack,
};
pub use hydrate_resource_flow::{
    HydratedResourceFlowPack, hydrate_resource_flow_pack, net_intrinsic_flow,
};
pub use jomini::{TextTape, TextToken};
pub use json::to_canonical_json;
pub use literal_install::{
    LiteralInstallSnapshot, OverlaySpecFingerprint, admit_and_apply_domain_pack,
    admit_and_apply_pack,
};
pub use parse::parse_raw_document;
pub use raw::RawDocument;
pub use scope::{
    ScopeAtom, ScopeAtomKind, ScopeChain, ScopeDiagnostic, ScopeDiagnosticKind,
    ScopeExtractionReport, ScopeReference, ScopeReferenceRole, ScopeTable, extract_scopes,
    extract_scopes_validated, parse_scope_chain, synthetic_scope_table,
};
pub use scope_json::scope_report_to_json;
pub use scope_lab::{LabFrequencyReport, scan_lab_scopes};
