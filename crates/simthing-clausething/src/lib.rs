//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser only.
//! No hydration to `simthing-spec`, no runtime wiring, default-off.

pub mod jomini;

pub use jomini::{TextTape, TextToken};
