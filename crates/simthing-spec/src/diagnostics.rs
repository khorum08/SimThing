use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecDiagnostic {
    pub severity:    DiagnosticSeverity,
    pub code:        String,
    pub message:     String,
    #[serde(default)]
    pub source_path: Option<String>,
    #[serde(default)]
    pub hint:        Option<String>,
}

impl SpecDiagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity:    DiagnosticSeverity::Error,
            code:        code.into(),
            message:     message.into(),
            source_path: None,
            hint:        None,
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity:    DiagnosticSeverity::Warning,
            code:        code.into(),
            message:     message.into(),
            source_path: None,
            hint:        None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecDiagnostics {
    pub diagnostics: Vec<SpecDiagnostic>,
}

impl SpecDiagnostics {
    pub fn push(&mut self, diagnostic: SpecDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error)
    }

    pub fn merge(&mut self, other: SpecDiagnostics) {
        self.diagnostics.extend(other.diagnostics);
    }
}

pub type SpecResult<T> = Result<(T, SpecDiagnostics), SpecError>;

use crate::error::SpecError;
