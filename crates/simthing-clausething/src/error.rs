//! CT-0b parse/emit error types.

use std::fmt;

/// Failure while parsing ClauseScript-shaped text into the raw model.
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClauseThing raw parse error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

impl From<crate::jomini::Error> for ParseError {
    fn from(err: crate::jomini::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<crate::jomini::DeserializeError> for ParseError {
    fn from(err: crate::jomini::DeserializeError) -> Self {
        Self::new(err.to_string())
    }
}

/// Failure while re-emitting the raw model to ClauseScript-shaped text.
#[derive(Debug)]
pub struct EmitError {
    pub message: String,
}

impl EmitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for EmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClauseThing raw emit error: {}", self.message)
    }
}

impl std::error::Error for EmitError {}

impl From<crate::jomini::Error> for EmitError {
    fn from(err: crate::jomini::Error) -> Self {
        Self::new(err.to_string())
    }
}

/// Failure while running CT-0c expansion passes over the raw model.
#[derive(Debug)]
pub struct ExpandError {
    pub message: String,
    pub span: Option<crate::raw::RawSpan>,
}

impl ExpandError {
    pub fn new(message: impl Into<String>, span: Option<crate::raw::RawSpan>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for ExpandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.span {
            Some(span) => write!(
                f,
                "ClauseThing expansion error at token {}: {}",
                span.token_index, self.message
            ),
            None => write!(f, "ClauseThing expansion error: {}", self.message),
        }
    }
}

impl std::error::Error for ExpandError {}

/// Failure while hydrating a literal entity into `simthing-spec` authoring structs.
#[derive(Debug)]
pub struct HydrateError {
    pub message: String,
    pub span: Option<crate::raw::RawSpan>,
}

impl HydrateError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::new_spanned(message, None)
    }

    pub fn new_spanned(message: impl Into<String>, span: Option<crate::raw::RawSpan>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for HydrateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.span {
            Some(span) => write!(
                f,
                "ClauseThing hydration error at token {}: {}",
                span.token_index, self.message
            ),
            None => write!(f, "ClauseThing hydration error: {}", self.message),
        }
    }
}

impl std::error::Error for HydrateError {}
