//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser.
//! CT-0b: lossless raw model, JSON goldens, and safe synthetic round-trip.
//! CT-0c: expansion passes (@vars, inline_script, $PARAM$, [[PARAM]] conditionals)
//! with `@[ ]` inline math preserved symbolically and `value:` left symbolic.
//! No hydration to `simthing-spec`, no runtime wiring, default-off.

pub mod error;
pub mod expand;
pub mod jomini;
pub mod raw;

mod emit;
mod json;
mod parse;

pub use emit::emit_text;
pub use error::{EmitError, ExpandError, ParseError};
pub use expand::{
    ExpansionInput, ExpansionOptions, expand_document, is_inline_math, is_value_reference,
};
pub use jomini::{TextTape, TextToken};
pub use json::to_canonical_json;
pub use parse::parse_raw_document;
pub use raw::RawDocument;
