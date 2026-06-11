//! Lab-only aggregate frequency scanner for `scopes.log` (CT-0d).
//!
//! Never commits or persists raw lab text. Requires `CLAUSER_LAB_DIR`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Aggregate-only frequency evidence from a lab `scopes.log` scan.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabFrequencyReport {
    pub scopes_log_found: bool,
    pub total_scope_names: usize,
    pub output_scope_counts: BTreeMap<String, usize>,
    pub supported_relation_count: usize,
    pub malformed_line_count: usize,
    pub unhandled_line_count: usize,
    pub top_scope_names: Vec<(String, usize)>,
}

/// Scan lab `scopes.log` under `CLAUSER_LAB_DIR` and return aggregate counts only.
pub fn scan_lab_scopes(lab_dir: &Path) -> LabFrequencyReport {
    let mut report = LabFrequencyReport::default();
    let Some(path) = locate_scopes_log(lab_dir) else {
        return report;
    };
    report.scopes_log_found = true;

    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => {
            report.unhandled_line_count += 1;
            return report;
        }
    };

    let mut current_name: Option<String> = None;
    let mut name_counts: BTreeMap<String, usize> = BTreeMap::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(name) = trimmed.strip_suffix(':') {
            if is_scope_name(name) {
                current_name = Some(name.to_string());
                *name_counts.entry(name.to_string()).or_insert(0) += 1;
                report.total_scope_names += 1;
                continue;
            }
        }

        if let Some(rest) = trimmed.strip_prefix("Output Scope:") {
            let output = rest.trim();
            if output.is_empty() {
                report.malformed_line_count += 1;
                continue;
            }
            *report
                .output_scope_counts
                .entry(output.to_string())
                .or_insert(0) += 1;
            if current_name.is_none() {
                report.malformed_line_count += 1;
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("Supported Scopes:") {
            let supported = rest.trim();
            if supported.is_empty() {
                report.malformed_line_count += 1;
                continue;
            }
            report.supported_relation_count += supported
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .count();
            if current_name.is_none() {
                report.malformed_line_count += 1;
            }
            continue;
        }

        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        report.unhandled_line_count += 1;
    }

    report.top_scope_names = top_n(&name_counts, 10);
    report
}

fn locate_scopes_log(lab_dir: &Path) -> Option<PathBuf> {
    let candidates = [
        lab_dir.join("Paradox/script_documentation/scopes.log"),
        lab_dir.join("script_documentation/scopes.log"),
        lab_dir.join("scopes.log"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn top_n(counts: &BTreeMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = counts
        .iter()
        .map(|(name, count)| (name.clone(), *count))
        .collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    entries.truncate(n);
    entries
}

fn is_scope_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}
