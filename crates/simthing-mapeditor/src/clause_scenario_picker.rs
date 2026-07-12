//! TP-STUDIO-CLAUSE-PICKER-0 — narrow Studio UI affordance for `.clause` open.
//!
//! Thin caller of production `clause_scenario_ingest` only. No parse/rebind duplicate,
//! no TP/fixture defaults, no GameMode/RF/live-run/closeout.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::clause_scenario_ingest::{
    load_clause_studio_session_from_path, load_studio_session_from_clause_ingest_result,
    ClauseScenarioIngestError, ClauseScenarioIngestOptions, ClauseScenarioIngestResult,
    ClauseScenarioSourceResolver,
};
use crate::generation::GenerationProfile;
use crate::session::StudioSession;
use crate::studio_scenario_load::{
    canonicalize_scenario_display_path, default_picker_start_directory, ScenarioPickerOutcome,
};

pub const OPEN_CLAUSE_SCENARIO_ACTION_LABEL: &str = "Open ClauseScript Scenario...";
pub const CLAUSE_DIALOG_TITLE: &str = "Open ClauseScript Scenario";
pub const CLAUSE_DIALOG_FILTER_NAME: &str = "ClauseScript";
pub const CLAUSE_DIALOG_FILTER_EXT: &str = "clause";
pub const CLAUSE_FILE_SUFFIX: &str = ".clause";

/// User-selected inputs for the admitted picker action (no production defaults).
#[derive(Debug, Clone, Default)]
pub struct ClausePickerSelection {
    pub clause_path: PathBuf,
    /// Explicit placeholder token → filesystem path (e.g. `"{{FIXTURE_JSON}}"` → path).
    pub resolver_entries: BTreeMap<String, PathBuf>,
    /// Where to write intermediate ScenarioSpec JSON for session load helpers.
    /// When `None`, a sibling `*.from-clause.simthing-scenario.json` path is used.
    pub scenario_json_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum ClausePickerActionResult {
    Cancelled,
    Loaded {
        session: StudioSession,
        ingest: ClauseScenarioIngestResult,
        message: String,
    },
    InvalidPath {
        message: String,
    },
    Failed {
        message: String,
    },
}

impl ClausePickerActionResult {
    pub fn message(&self) -> &str {
        match self {
            Self::Cancelled => "ClauseScript open cancelled",
            Self::Loaded { message, .. }
            | Self::InvalidPath { message }
            | Self::Failed { message } => message,
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Loaded { .. })
    }
}

/// Injectable `.clause` path picker (native dialog or fake for tests).
pub trait ClauseFilePicker {
    fn pick_open_clause(&self, start_dir: &Path) -> ScenarioPickerOutcome;
}

pub struct NativeClauseFilePicker;

impl ClauseFilePicker for NativeClauseFilePicker {
    fn pick_open_clause(&self, start_dir: &Path) -> ScenarioPickerOutcome {
        let dialog = rfd::FileDialog::new()
            .set_title(CLAUSE_DIALOG_TITLE)
            .add_filter(CLAUSE_DIALOG_FILTER_NAME, &[CLAUSE_DIALOG_FILTER_EXT])
            .set_directory(start_dir);
        match dialog.pick_file() {
            Some(path) => ScenarioPickerOutcome::Selected(path),
            None => ScenarioPickerOutcome::Cancelled,
        }
    }
}

/// Prefer the repository/project `scenarios` directory when no prior ClauseScript path exists.
/// This keeps operator browsing away from similarly named integration fixtures without baking in
/// a scenario name or changing explicit path hints.
pub fn default_clause_picker_start_directory(start_path_hint: &str) -> PathBuf {
    let trimmed = start_path_hint.trim();
    if !trimmed.is_empty() {
        let candidate = PathBuf::from(trimmed);
        if candidate.is_dir() {
            return candidate;
        }
        if let Some(parent) = candidate.parent() {
            if !parent.as_os_str().is_empty() && parent.is_dir() {
                return parent.to_path_buf();
            }
        }
    }

    let cwd = std::env::current_dir().ok();
    let executable_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    for root in cwd.iter().chain(executable_dir.iter()) {
        for ancestor in root.ancestors() {
            let scenarios = ancestor.join("scenarios");
            if scenarios.is_dir() {
                return scenarios;
            }
        }
    }
    default_picker_start_directory(start_path_hint)
}

#[derive(Debug, Clone)]
pub struct FakeClauseFilePicker {
    pub outcome: ScenarioPickerOutcome,
}

impl ClauseFilePicker for FakeClauseFilePicker {
    fn pick_open_clause(&self, _start_dir: &Path) -> ScenarioPickerOutcome {
        self.outcome.clone()
    }
}

/// Parse UI resolver lines: `TOKEN=path` or `{{TOKEN}}=path` (one per line; `#` comments ok).
pub fn parse_clause_resolver_entries(text: &str) -> Result<BTreeMap<String, PathBuf>, String> {
    let mut out = BTreeMap::new();
    for (i, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((token_raw, path_raw)) = line.split_once('=') else {
            return Err(format!(
                "resolver line {}: expected TOKEN=path, got `{line}`",
                i + 1
            ));
        };
        let token = normalize_placeholder_token(token_raw.trim());
        let path = PathBuf::from(path_raw.trim());
        if token.is_empty() {
            return Err(format!("resolver line {}: empty token", i + 1));
        }
        if path.as_os_str().is_empty() {
            return Err(format!("resolver line {}: empty path", i + 1));
        }
        out.insert(token, path);
    }
    Ok(out)
}

fn normalize_placeholder_token(token: &str) -> String {
    let t = token.trim();
    if t.starts_with("{{") && t.ends_with("}}") {
        t.to_string()
    } else if t.is_empty() {
        String::new()
    } else {
        format!("{{{{{t}}}}}")
    }
}

pub fn validate_clause_path(path: &Path) -> Result<PathBuf, String> {
    let display = canonicalize_scenario_display_path(path);
    let lossy = display.to_string_lossy();
    if !lossy.ends_with(CLAUSE_FILE_SUFFIX) {
        return Err(format!(
            "clause path must use the {CLAUSE_FILE_SUFFIX} suffix (got {})",
            display.display()
        ));
    }
    if !display.is_file() {
        return Err(format!("clause path is not a file: {}", display.display()));
    }
    Ok(display)
}

fn default_scenario_json_path(clause_path: &Path) -> PathBuf {
    let stem = clause_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("from_clause");
    clause_path.with_file_name(format!("{stem}.from-clause.simthing-scenario.json"))
}

fn options_from_selection(selection: &ClausePickerSelection) -> ClauseScenarioIngestOptions {
    let mut resolver = ClauseScenarioSourceResolver::new();
    for (token, path) in &selection.resolver_entries {
        resolver.insert(token.clone(), path.clone());
    }
    ClauseScenarioIngestOptions {
        projection_mode: simthing_clausething::ClauseScenarioProjectionMode::StructuralRebindReady,
        source_resolver: resolver,
    }
}

/// Core picker action: user-selected clause path + explicit resolver → production API → session.
///
/// This is the CI-testable controller boundary. Native dialog is a thin caller that builds
/// [`ClausePickerSelection`] then invokes this function.
pub fn run_clause_picker_action(
    selection: &ClausePickerSelection,
    profile_hint: Option<GenerationProfile>,
) -> ClausePickerActionResult {
    let clause_path = match validate_clause_path(&selection.clause_path) {
        Ok(p) => p,
        Err(reason) => {
            return ClausePickerActionResult::InvalidPath {
                message: format!("ClauseScript open failed: {reason}"),
            };
        }
    };

    let json_path = selection
        .scenario_json_path
        .clone()
        .unwrap_or_else(|| default_scenario_json_path(&clause_path));
    let options = options_from_selection(selection);

    match load_clause_studio_session_from_path(&clause_path, &options, &json_path, profile_hint) {
        Ok((ingest, session)) => {
            let message = format!(
                "ClauseScript scenario opened: {} (StructuralRebindReady session hydrate PASS)",
                clause_path.display()
            );
            ClausePickerActionResult::Loaded {
                session,
                ingest,
                message,
            }
        }
        Err(err) => ClausePickerActionResult::Failed {
            message: format!(
                "ClauseScript open failed for {}: {}",
                clause_path.display(),
                err.status_message()
            ),
        },
    }
}

/// Ingest-only path (session via existing `from_loaded_scenario` helper) — used by proofs/tests.
pub fn run_clause_picker_ingest_then_session(
    selection: &ClausePickerSelection,
    scenario_path_label: PathBuf,
    profile_hint: Option<GenerationProfile>,
) -> Result<(ClauseScenarioIngestResult, StudioSession), ClauseScenarioIngestError> {
    let clause_path = validate_clause_path(&selection.clause_path).map_err(|e| {
        ClauseScenarioIngestError::SourceResolution(e)
    })?;
    let options = options_from_selection(selection);
    let ingest =
        crate::clause_scenario_ingest::ingest_clause_scenario_path(&clause_path, &options)?;
    let session =
        load_studio_session_from_clause_ingest_result(&ingest, scenario_path_label, profile_hint)?;
    Ok((ingest, session))
}

pub fn format_clause_picker_error(err: &ClauseScenarioIngestError) -> String {
    format!("ClauseScript open failed: {}", err.status_message())
}

/// Native dialog select → build selection with provided resolver map → run action.
pub fn open_clause_scenario_with_picker<P: ClauseFilePicker>(
    picker: &P,
    start_path_hint: &str,
    resolver_entries: BTreeMap<String, PathBuf>,
    scenario_json_path: Option<PathBuf>,
    profile_hint: Option<GenerationProfile>,
) -> ClausePickerActionResult {
    let start_dir = default_clause_picker_start_directory(start_path_hint);
    match picker.pick_open_clause(&start_dir) {
        ScenarioPickerOutcome::Cancelled => ClausePickerActionResult::Cancelled,
        ScenarioPickerOutcome::Selected(path) => {
            let selection = ClausePickerSelection {
                clause_path: path,
                resolver_entries,
                scenario_json_path,
            };
            run_clause_picker_action(&selection, profile_hint)
        }
    }
}

/// Menu/label constant used by Studio UI — presence proves user-facing affordance exists.
pub fn clause_picker_menu_label() -> &'static str {
    OPEN_CLAUSE_SCENARIO_ACTION_LABEL
}
