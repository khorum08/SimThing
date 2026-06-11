// Vendored from github.com/rakaly/jomini @ v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
// License: MIT - see crates/simthing-clausething/src/jomini/LICENSE
// MODIFIED: text-path-only crate root; binary/save/melting/serde paths excluded.
//! Vendored jomini text-path subset for ClauseScript parsing (CT-0a).
//!
//! Upstream: https://github.com/rakaly/jomini
//! Vendored version: v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
//! License: MIT - see `LICENSE` in this directory.
//! MODIFIED: text-path-only trim (no binary lexer, envelope, melting, serde derive, or save I/O).

#![allow(missing_docs, clippy::all)]

pub mod binary;
pub mod common;
mod data;
mod encoding;
mod errors;
mod scalar;
pub mod text;
mod util;

pub use self::encoding::*;
pub use self::errors::*;
pub use self::scalar::{Scalar, ScalarError};
pub use self::text::{TextTape, TextToken, TextWriter, TextWriterBuilder};
