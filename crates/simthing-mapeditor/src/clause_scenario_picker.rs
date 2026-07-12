//! TP-STUDIO-CLAUSE-PICKER-0 — narrow Studio UI affordance for `.clause` open.
//!
//! Thin caller of production `clause_scenario_ingest` only. No parse/rebind duplicate,
//! no TP/fixture defaults, no GameMode/RF/live-run/closeout.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::clause_scenario_ingest::{
    ingest_clause_scenario_path_staged, load_studio_session_from_clause_ingest_result,
    save_clause_scenario_authority_to_path, ClauseScenarioIngestError, ClauseScenarioIngestOptions,
    ClauseScenarioIngestResult, ClauseScenarioIngestStage, ClauseScenarioIngestStageEvent,
    ClauseScenarioSourceResolver,
};
use crate::generation::GenerationProfile;
use crate::scenario_io::{load_scenario_authority_from_path, ScenarioIoError};
use crate::session::StudioSession;
use crate::studio_scenario_library_ui::{StudioLoaderStage, StudioLoaderStageEvent};
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

fn map_ingest_stage(stage: ClauseScenarioIngestStage) -> StudioLoaderStage {
    match stage {
        ClauseScenarioIngestStage::Resolve => StudioLoaderStage::Resolve,
        ClauseScenarioIngestStage::Parse => StudioLoaderStage::Parse,
        ClauseScenarioIngestStage::Hydrate => StudioLoaderStage::Hydrate,
        ClauseScenarioIngestStage::Rebind => StudioLoaderStage::Rebind,
    }
}

fn map_ingest_event(event: ClauseScenarioIngestStageEvent) -> StudioLoaderStageEvent {
    match event {
        ClauseScenarioIngestStageEvent::Running(stage) => {
            StudioLoaderStageEvent::Running(map_ingest_stage(stage))
        }
        ClauseScenarioIngestStageEvent::Passed { stage, elapsed } => {
            StudioLoaderStageEvent::Passed {
                stage: map_ingest_stage(stage),
                elapsed,
            }
        }
        ClauseScenarioIngestStageEvent::Failed {
            stage,
            elapsed,
            message,
        } => StudioLoaderStageEvent::Failed {
            stage: map_ingest_stage(stage),
            elapsed,
            message,
        },
    }
}

fn observe_loader_stage<T>(
    stage: StudioLoaderStage,
    observer: &mut impl FnMut(StudioLoaderStageEvent),
    action: impl FnOnce() -> Result<T, ClauseScenarioIngestError>,
) -> Result<T, ClauseScenarioIngestError> {
    observer(StudioLoaderStageEvent::Running(stage));
    let started = Instant::now();
    match action() {
        Ok(value) => {
            observer(StudioLoaderStageEvent::Passed {
                stage,
                elapsed: started.elapsed(),
            });
            Ok(value)
        }
        Err(error) => {
            observer(StudioLoaderStageEvent::Failed {
                stage,
                elapsed: started.elapsed(),
                message: error.status_message(),
            });
            Err(error)
        }
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
    run_clause_picker_action_staged(selection, profile_hint, &mut |_| {})
}

/// Execute the existing production composition while exposing real stage boundaries to presentation.
pub fn run_clause_picker_action_staged(
    selection: &ClausePickerSelection,
    profile_hint: Option<GenerationProfile>,
    observer: &mut impl FnMut(StudioLoaderStageEvent),
) -> ClausePickerActionResult {
    let validation_started = Instant::now();
    let clause_path = match validate_clause_path(&selection.clause_path) {
        Ok(p) => p,
        Err(reason) => {
            observer(StudioLoaderStageEvent::Running(StudioLoaderStage::Resolve));
            observer(StudioLoaderStageEvent::Failed {
                stage: StudioLoaderStage::Resolve,
                elapsed: validation_started.elapsed(),
                message: reason.clone(),
            });
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
    let result = (|| {
        let mut ingest_observer = |event| observer(map_ingest_event(event));
        let ingest =
            ingest_clause_scenario_path_staged(&clause_path, &options, &mut ingest_observer)?;
        observe_loader_stage(StudioLoaderStage::Persist, observer, || {
            save_clause_scenario_authority_to_path(&json_path, &ingest.scenario)
        })?;
        let scenario = observe_loader_stage(StudioLoaderStage::SessionBuild, observer, || {
            Ok(load_scenario_authority_from_path(&json_path)?)
        })?;
        let session = observe_loader_stage(StudioLoaderStage::Projection, observer, || {
            StudioSession::from_loaded_scenario(scenario, json_path.clone(), profile_hint)
                .map_err(ScenarioIoError::from)
                .map_err(ClauseScenarioIngestError::from)
        })?;
        Ok::<_, ClauseScenarioIngestError>((ingest, session))
    })();

    match result {
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

/// Native/injectable selection only. No ingest, persistence, or session construction occurs.
pub fn select_clause_path_with_picker<P: ClauseFilePicker>(
    picker: &P,
    start_path_hint: &str,
) -> ScenarioPickerOutcome {
    picker.pick_open_clause(&default_clause_picker_start_directory(start_path_hint))
}

/// Ingest-only path (session via existing `from_loaded_scenario` helper) — used by proofs/tests.
pub fn run_clause_picker_ingest_then_session(
    selection: &ClausePickerSelection,
    scenario_path_label: PathBuf,
    profile_hint: Option<GenerationProfile>,
) -> Result<(ClauseScenarioIngestResult, StudioSession), ClauseScenarioIngestError> {
    let clause_path = validate_clause_path(&selection.clause_path)
        .map_err(|e| ClauseScenarioIngestError::SourceResolution(e))?;
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
