//! Limited production ClauseScript scenario composition API (StructuralRebindReady only).
//!
//! TP-STUDIO-CLAUSE-API-1 — caller-supplied path/bytes + source resolver; no scenario defaults.
//! Composes clausething parse/hydrate + generic rebind + mapeditor scenario_io / session.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document,
    project_pack_to_authority_tree_candidate, rebind_pack_to_structural_rebind_ready,
    ClauseScenarioProjectionError, ClauseScenarioProjectionMode, ClauseScenarioProjectionReport,
    HydrateError, HydratedScenarioPack, ParseError,
};
use simthing_spec::SimThingScenarioSpec;
use thiserror::Error;

use crate::generation::GenerationProfile;
use crate::scenario_io::{
    load_studio_session_from_scenario_path, save_scenario_authority_to_path, ScenarioIoError,
};
use crate::session::StudioSession;

/// Caller-supplied placeholder → filesystem path map for clause source rewrite before parse.
///
/// Keys are exact tokens appearing in clause text (e.g. `"{{FIXTURE_JSON}}"`).
/// Production never invents defaults when a token is missing.
#[derive(Debug, Clone, Default)]
pub struct ClauseScenarioSourceResolver {
    pub placeholder_paths: BTreeMap<String, PathBuf>,
}

impl ClauseScenarioSourceResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_placeholder(mut self, token: impl Into<String>, path: PathBuf) -> Self {
        self.placeholder_paths.insert(token.into(), path);
        self
    }

    pub fn insert(&mut self, token: impl Into<String>, path: PathBuf) {
        self.placeholder_paths.insert(token.into(), path);
    }
}

#[derive(Debug, Clone)]
pub struct ClauseScenarioIngestOptions {
    pub projection_mode: ClauseScenarioProjectionMode,
    pub source_resolver: ClauseScenarioSourceResolver,
}

impl Default for ClauseScenarioIngestOptions {
    fn default() -> Self {
        Self {
            projection_mode: ClauseScenarioProjectionMode::StructuralRebindReady,
            source_resolver: ClauseScenarioSourceResolver::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClauseScenarioIngestError {
    #[error("clause scenario IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("clause scenario parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("clause scenario hydrate error: {0}")]
    Hydrate(#[from] HydrateError),
    #[error("clause scenario projection error: {0}")]
    Projection(#[from] ClauseScenarioProjectionError),
    #[error("clause scenario source resolution error: {0}")]
    SourceResolution(String),
    #[error("unsupported clause scenario projection mode")]
    UnsupportedProjectionMode,
    #[error("studio scenario IO error: {0}")]
    ScenarioIo(#[from] ScenarioIoError),
}

impl ClauseScenarioIngestError {
    pub fn status_message(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ClauseScenarioIngestResult {
    pub source_path: Option<PathBuf>,
    pub pack: HydratedScenarioPack,
    pub scenario: SimThingScenarioSpec,
    pub report: ClauseScenarioProjectionReport,
}

/// Ingest a `.clause` path with caller-supplied resolver; emit StructuralRebindReady Spec.
///
/// Relative `source_json` / `include_json` paths resolve against the **clause file directory**
/// (`source_base`), not process CWD (STUDIO-CLAUSE-LOADER-SIMPLIFY-0 / 11.1 residual).
pub fn ingest_clause_scenario_path(
    path: &Path,
    options: &ClauseScenarioIngestOptions,
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    let raw = std::fs::read_to_string(path)?;
    let source_base = path.parent();
    let mut result = ingest_clause_scenario_text(&raw, options, source_base)?;
    result.source_path = Some(path.to_path_buf());
    Ok(result)
}

/// Ingest clause source bytes with caller-supplied resolver (no clause path → no source_base).
pub fn ingest_clause_scenario_bytes(
    bytes: &[u8],
    options: &ClauseScenarioIngestOptions,
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    let raw = std::str::from_utf8(bytes).map_err(|e| {
        ClauseScenarioIngestError::SourceResolution(format!("clause source is not UTF-8: {e}"))
    })?;
    ingest_clause_scenario_text(raw, options, None)
}

fn ingest_clause_scenario_text(
    raw: &str,
    options: &ClauseScenarioIngestOptions,
    source_base: Option<&Path>,
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    if options.projection_mode != ClauseScenarioProjectionMode::StructuralRebindReady {
        return Err(ClauseScenarioIngestError::UnsupportedProjectionMode);
    }
    let source = apply_source_resolver(raw, &options.source_resolver)?;
    let document = parse_raw_document(source.as_bytes())?;
    let pack = hydrate_scenario_with_source_base(&document, source_base)?;
    let (scenario, report) = rebind_pack_to_structural_rebind_ready(&pack)?;
    Ok(ClauseScenarioIngestResult {
        source_path: None,
        pack,
        scenario,
        report,
    })
}

/// Save produced Spec through existing Studio authority path helper.
pub fn save_clause_scenario_authority_to_path(
    path: &Path,
    scenario: &SimThingScenarioSpec,
) -> Result<(), ClauseScenarioIngestError> {
    Ok(save_scenario_authority_to_path(path, scenario)?)
}

/// Ingest clause path, save Spec JSON, then load Studio session via existing session path.
pub fn load_clause_studio_session_from_path(
    clause_path: &Path,
    options: &ClauseScenarioIngestOptions,
    scenario_json_path: &Path,
    profile_hint: Option<GenerationProfile>,
) -> Result<(ClauseScenarioIngestResult, StudioSession), ClauseScenarioIngestError> {
    let ingest = ingest_clause_scenario_path(clause_path, options)?;
    save_clause_scenario_authority_to_path(scenario_json_path, &ingest.scenario)?;
    let session = load_studio_session_from_scenario_path(scenario_json_path, profile_hint)?;
    Ok((ingest, session))
}

/// Load Studio session from an already-produced StructuralRebindReady Spec authority.
pub fn load_studio_session_from_clause_ingest_result(
    result: &ClauseScenarioIngestResult,
    scenario_path_label: PathBuf,
    profile_hint: Option<GenerationProfile>,
) -> Result<StudioSession, ClauseScenarioIngestError> {
    Ok(StudioSession::from_loaded_scenario(
        result.scenario.clone(),
        scenario_path_label,
        profile_hint,
    )
    .map_err(ScenarioIoError::from)?)
}

fn apply_source_resolver(
    raw: &str,
    resolver: &ClauseScenarioSourceResolver,
) -> Result<String, ClauseScenarioIngestError> {
    let mut out = raw.to_string();
    for (token, path) in &resolver.placeholder_paths {
        if !out.contains(token) {
            continue;
        }
        if !path.is_file() {
            return Err(ClauseScenarioIngestError::SourceResolution(format!(
                "resolver path for `{token}` does not exist: {}",
                path.display()
            )));
        }
        let path_str = path.to_string_lossy().replace('\\', "/");
        out = out.replace(token, &path_str);
    }
    // Any remaining `{{...}}` placeholders are unresolved — hard error (no silent defaults).
    if let Some(start) = out.find("{{") {
        if let Some(end_rel) = out[start..].find("}}") {
            let token = &out[start..start + end_rel + 2];
            return Err(ClauseScenarioIngestError::SourceResolution(format!(
                "unresolved clause source placeholder `{token}`; caller must supply source_resolver entry"
            )));
        }
    }
    Ok(out)
}

/// Internal: authority-tree candidate only (not a production open mode).
#[allow(dead_code)]
pub(crate) fn project_authority_tree_candidate_for_tests(
    pack: &HydratedScenarioPack,
) -> Result<SimThingScenarioSpec, ClauseScenarioIngestError> {
    Ok(project_pack_to_authority_tree_candidate(pack)?)
}
