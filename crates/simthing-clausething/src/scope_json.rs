//! Deterministic JSON for CT-0d scope extraction reports.

use crate::error::ParseError;
use crate::scope::ScopeExtractionReport;

pub fn scope_report_to_json(report: &ScopeExtractionReport) -> Result<String, ParseError> {
    serde_json::to_string_pretty(report)
        .map_err(|err| ParseError::new(format!("scope report JSON serialization failed: {err}")))
}
