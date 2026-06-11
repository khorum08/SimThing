// Vendored from github.com/rakaly/jomini @ v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
// License: MIT - see crates/simthing-clausething/src/jomini/LICENSE
// MODIFIED: de/reader modules excluded (text tape + dom + writer only).
//! Types for parsing ClauseScript / Clausewitz plaintext input (vendored jomini text path).

mod dom;
mod fnv;
mod operator;
mod tape;
mod writer;

pub use self::dom::{
    ArrayReader, FieldGroupsIter, FieldsIter, GroupEntry, GroupEntryIter, ObjectReader, Reader,
    ScalarReader, ValueReader, ValuesIter,
};
pub use self::operator::*;
pub use self::tape::{TextTape, TextTapeParser, TextToken};
pub use self::writer::*;
pub use crate::jomini::{ReaderError, ReaderErrorKind};
