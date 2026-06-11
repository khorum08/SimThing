//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser.
//! CT-0b: lossless raw model, JSON goldens, and safe synthetic round-trip.
//! No hydration to `simthing-spec`, no runtime wiring, default-off.

pub mod error;
pub mod jomini;
pub mod raw;

mod emit;
mod json;
mod parse;

pub use emit::emit_text;
pub use error::{EmitError, ParseError};
pub use jomini::{TextTape, TextToken};
pub use json::to_canonical_json;
pub use parse::parse_raw_document;
pub use raw::RawDocument;
