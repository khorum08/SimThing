//! Production ClauseScript source and reproducible cache ingestion (StructuralRebindReady).
//!
//! TP-STUDIO-CLAUSE-API-1 — caller-supplied path/bytes + source resolver; no scenario defaults.
//! Composes clausething parse/hydrate + generic rebind + mapeditor scenario_io / session.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use simthing_clausething::{
    expand_document, hydrate_scenario_with_source_base, parse_raw_document,
    project_pack_to_authority_tree_candidate, rebind_pack_to_structural_rebind_ready,
    ClauseScenarioProjectionError, ClauseScenarioProjectionMode, ClauseScenarioProjectionReport,
    ExpansionInput, HydrateError, HydratedScenarioPack, ParseError,
};
use simthing_spec::SimThingScenarioSpec;
use thiserror::Error;

use crate::generation::GenerationProfile;
use crate::scenario_io::{save_scenario_authority_to_path, ScenarioIoError};
use crate::session::StudioSession;
use crate::studio_live_session_bridge::authored_live_profile_from_pack;

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
    /// Source/cache provenance, never a second hydrated or executable representation.
    pub source_cache: Option<ClauseScenarioCache>,
}

pub const CLAUSE_CACHE_FORMAT: &str = "simthing-clause-cache-v1";

/// Reproducible raw-model cache with a declared, content-pinned source bundle.
/// All paths are relative to the native source, except source_path (relative to the cache).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClauseScenarioCache {
    pub format: String,
    pub source_path: String,
    pub source_identity: String,
    pub document: simthing_clausething::raw::RawDocument,
    pub dependencies: BTreeMap<String, String>,
    pub resolver_entries: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct StudioSourceCacheProvenance {
    pub cache: ClauseScenarioCache,
    pub scenario_identity: String,
    pub profile_identity: String,
}

pub(crate) fn authored_profile_content_identity(
    profile: &crate::studio_live_session_bridge::StudioAuthoredLiveProfile,
) -> Result<String, serde_json::Error> {
    use serde::ser::Error;
    use simthing_core::{Overlay, ResourceParentEdge, SimThing};

    // A serialization-only namespace, rooted in the existing ordered tree. Never
    // change runtime IDs or infer references from arbitrary numeric property data.
    fn index_tree(node: &SimThing, ids: &mut BTreeMap<u64, u32>) -> Result<(), serde_json::Error> {
        let ordinal = u32::try_from(ids.len() + 1).map_err(serde_json::Error::custom)?;
        if ids.insert(u64::from(node.id.raw()), ordinal).is_some() {
            return Err(serde_json::Error::custom(
                "duplicate profile SimThing identity",
            ));
        }
        for child in &node.children {
            index_tree(child, ids)?;
        }
        Ok(())
    }

    fn reference(ids: &BTreeMap<u64, u32>, raw: u64) -> Result<u32, serde_json::Error> {
        ids.get(&raw).copied().ok_or_else(|| {
            serde_json::Error::custom(format!(
                "profile reference {raw} is outside the session tree"
            ))
        })
    }

    fn tree_value(
        node: &SimThing,
        ids: &BTreeMap<u64, u32>,
    ) -> Result<serde_json::Value, serde_json::Error> {
        // Exhaustive destructuring keeps additions to the identity domain visible
        // to the compiler. All fields and sequence order survive serialization.
        let SimThing {
            id,
            kind,
            properties,
            resource_parent_edges,
            overlays,
            children,
            spawned_generation,
            declared_specializations,
        } = node;
        let properties: Vec<_> = properties
            .iter()
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect();
        let edges = resource_parent_edges
            .iter()
            .map(|edge| {
                let ResourceParentEdge {
                    property_namespace,
                    property_name,
                    parent,
                    source_span_token,
                } = edge;
                Ok(serde_json::json!({
                    "property_namespace": property_namespace,
                    "property_name": property_name,
                    "parent": reference(ids, u64::from(parent.raw()))?,
                    "source_span_token": source_span_token,
                }))
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()?;
        let overlays = overlays
            .iter()
            .map(|overlay| {
                let Overlay {
                    id,
                    kind,
                    source,
                    origin,
                    affects,
                    transform,
                    lifecycle,
                } = overlay;
                let affects = affects
                    .iter()
                    .map(|id| reference(ids, u64::from(id.raw())))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(serde_json::json!({
                    "id": id,
                    "kind": kind,
                    "source": source,
                    "origin": reference(ids, u64::from(origin.raw()))?,
                    "affects": affects,
                    "transform": transform,
                    "lifecycle": lifecycle,
                }))
            })
            .collect::<Result<Vec<_>, serde_json::Error>>()?;
        let children = children
            .iter()
            .map(|child| tree_value(child, ids))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(serde_json::json!({
            "id": reference(ids, u64::from(id.raw()))?,
            "kind": kind,
            "properties": properties,
            "resource_parent_edges": edges,
            "overlays": overlays,
            "children": children,
            "spawned_generation": spawned_generation,
            "declared_specializations": declared_specializations,
        }))
    }

    let mut ids = BTreeMap::new();
    index_tree(&profile.session_root, &mut ids)?;
    let tree = tree_value(&profile.session_root, &ids)?;
    let targets = profile
        .install_targets
        .iter()
        .map(|(name, members)| {
            let members = members
                .iter()
                .map(|id| reference(&ids, u64::from(id.raw())))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((name, members))
        })
        .collect::<Result<BTreeMap<_, _>, serde_json::Error>>()?;

    // Explicit RF overrides carry session-node references as raw integer fields.
    // Remap only those declared references; slots, capacities and all other
    // GameMode data retain their original values.
    let mut game_mode = profile.game_mode.clone();
    if let Some(flow) = &mut game_mode.resource_flow {
        for arena in &mut flow.arenas {
            for participant in &mut arena.explicit_participants {
                participant.subtree_root_id =
                    reference(&ids, u64::from(participant.subtree_root_id))?;
                participant.parent_subtree_root_id = participant
                    .parent_subtree_root_id
                    .map(|id| reference(&ids, id).map(u64::from))
                    .transpose()?;
            }
        }
    }
    Ok(clause_source_content_identity(&serde_json::to_vec(&(
        game_mode, tree, targets,
    ))?))
}

/// Stable byte identity for source/cache freshness, not an authentication primitive.
pub fn clause_source_content_identity(bytes: &[u8]) -> String {
    let hash = bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    });
    format!("fnv1a64:{hash:016x}:{}", bytes.len())
}

fn source_failure(path: &Path, error: impl std::fmt::Display) -> ClauseScenarioIngestError {
    ClauseScenarioIngestError::SourceResolution(format!(
        "0088-INGRESS-FIDELITY-0 refusal before activation; file {}: {error}",
        path.display(),
    ))
}

fn relative_source_path(path: &Path, base: &Path) -> Result<String, ClauseScenarioIngestError> {
    let path = path.canonicalize()?;
    let base = base.canonicalize()?;
    let path_parts: Vec<_> = path.components().collect();
    let base_parts: Vec<_> = base.components().collect();
    let common = path_parts
        .iter()
        .zip(&base_parts)
        .take_while(|(a, b)| a == b)
        .count();
    if common == 0 {
        return Err(source_failure(
            &path,
            "G4: source and cache must share a portable filesystem root",
        ));
    }
    let mut relative = PathBuf::new();
    for _ in common..base_parts.len() {
        relative.push("..");
    }
    for part in &path_parts[common..] {
        relative.push(part.as_os_str());
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn declared_dependencies(
    document: &simthing_clausething::raw::RawDocument,
    source: &Path,
) -> Result<BTreeMap<String, String>, ClauseScenarioIngestError> {
    use simthing_clausething::raw::RawValue;
    fn collect(value: &RawValue, paths: &mut Vec<String>) {
        match value {
            RawValue::Header(header) => collect(&header.payload, paths),
            RawValue::Block(block) => {
                for property in &block.properties {
                    if property.key.text == "static_galaxy_scenario" {
                        let body = match &property.value {
                            RawValue::Header(header) => header.payload.as_ref(),
                            other => other,
                        };
                        if let RawValue::Block(body) = body {
                            for field in &body.properties {
                                if matches!(field.key.text.as_str(), "source_json" | "include_json")
                                {
                                    if let RawValue::Scalar(value) = &field.value {
                                        paths.push(value.text.clone());
                                    }
                                }
                            }
                        }
                    }
                    collect(&property.value, paths);
                }
            }
            RawValue::Array(array) => {
                for value in &array.items {
                    collect(value, paths);
                }
            }
            RawValue::Scalar(_) => {}
        }
    }
    let base = source.parent().unwrap_or_else(|| Path::new("."));
    let mut paths = Vec::new();
    collect(&document.root, &mut paths);
    let mut dependencies = BTreeMap::new();
    for raw in paths {
        let resolved = simthing_clausething::resolve_clause_source_path(&raw, Some(base));
        let bytes = std::fs::read(&resolved)
            .map_err(|e| source_failure(source, format!("dependency `{raw}`: {e}")))?;
        dependencies.insert(
            relative_source_path(&resolved, base)?,
            clause_source_content_identity(&bytes),
        );
    }
    // Existing source_json declarations normalize to this same manifest without author rewrites.
    // A shipped bundle may pin them explicitly in the sibling <stem>.dependencies.json file.
    let manifest_path = source.with_extension("dependencies.json");
    if manifest_path.exists() {
        let bytes = std::fs::read(&manifest_path)?;
        let expected: BTreeMap<String, String> =
            serde_json::from_slice(&bytes).map_err(|e| source_failure(&manifest_path, e))?;
        if expected != dependencies {
            return Err(source_failure(
                &manifest_path,
                "G4: declared dependency identities do not match the source bundle",
            ));
        }
        dependencies.insert(
            relative_source_path(&manifest_path, base)?,
            clause_source_content_identity(&bytes),
        );
    }
    Ok(dependencies)
}

pub fn save_clause_scenario_cache_to_path(
    path: &Path,
    result: &ClauseScenarioIngestResult,
) -> Result<(), ClauseScenarioIngestError> {
    let cache = result
        .source_cache
        .as_ref()
        .ok_or_else(|| source_failure(path, "a cache requires a native source path"))?;
    write_clause_source_cache(path, cache)
}

pub(crate) fn write_clause_source_cache(
    path: &Path,
    source_cache: &ClauseScenarioCache,
) -> Result<(), ClauseScenarioIngestError> {
    let mut cache = source_cache.clone();
    let source = Path::new(&source_cache.source_path);
    if clause_source_content_identity(&std::fs::read(source)?) != cache.source_identity {
        return Err(source_failure(
            source,
            "source changed since this session loaded; reload before saving its cache",
        ));
    }
    for (relative, expected) in &cache.dependencies {
        let dependency = source.parent().unwrap().join(relative);
        if clause_source_content_identity(&std::fs::read(&dependency)?) != *expected {
            return Err(source_failure(
                &dependency,
                "dependency changed since this session loaded",
            ));
        }
    }
    cache.source_path = relative_source_path(
        source,
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new(".")),
    )?;
    let json = serde_json::to_string_pretty(&cache).map_err(|e| source_failure(path, e))?;
    crate::scenario_io::atomic_write(path, &json)?;
    Ok(())
}

pub fn ingest_clause_scenario_cache_path(
    path: &Path,
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    let cache: ClauseScenarioCache =
        serde_json::from_slice(&std::fs::read(path)?).map_err(|e| source_failure(path, e))?;
    if cache.format != CLAUSE_CACHE_FORMAT || !is_portable_relative_path(&cache.source_path) {
        return Err(source_failure(
            path,
            "G4: invalid cache format or non-relative native source",
        ));
    }
    let source = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(&cache.source_path);
    let source_bytes = std::fs::read(&source).map_err(|e| source_failure(&source, e))?;
    if clause_source_content_identity(&source_bytes) != cache.source_identity {
        return Err(source_failure(
            &source,
            "canonical JSON cache is stale; regenerate it from the native source",
        ));
    }
    let mut options = ClauseScenarioIngestOptions::default();
    for (token, relative) in &cache.resolver_entries {
        if !is_portable_relative_path(relative) {
            return Err(source_failure(
                path,
                "G4: non-relative cache resolver dependency",
            ));
        }
        options
            .source_resolver
            .insert(token.clone(), source.parent().unwrap().join(relative));
    }
    // The native loader owns parse/expand/hydrate/rebind on BOTH routes.
    let result = ingest_clause_scenario_path(&source, &options)?;
    let regenerated = result
        .source_cache
        .as_ref()
        .expect("path ingest carries source cache");
    if regenerated.document != cache.document || regenerated.dependencies != cache.dependencies {
        return Err(source_failure(path, "canonical JSON document or dependency manifest differs from its native source; regenerate the cache"));
    }
    Ok(result)
}

fn is_portable_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && !path.contains(':')
        && !Path::new(path).components().any(|component| {
            matches!(
                component,
                std::path::Component::Prefix(_) | std::path::Component::RootDir
            )
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClauseScenarioIngestStage {
    Resolve,
    Parse,
    Hydrate,
    Rebind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseScenarioIngestStageEvent {
    Running(ClauseScenarioIngestStage),
    Passed {
        stage: ClauseScenarioIngestStage,
        elapsed: Duration,
    },
    Failed {
        stage: ClauseScenarioIngestStage,
        elapsed: Duration,
        message: String,
    },
}

fn observe_ingest_stage<T>(
    stage: ClauseScenarioIngestStage,
    observer: &mut impl FnMut(ClauseScenarioIngestStageEvent),
    action: impl FnOnce() -> Result<T, ClauseScenarioIngestError>,
) -> Result<T, ClauseScenarioIngestError> {
    observer(ClauseScenarioIngestStageEvent::Running(stage));
    let started = Instant::now();
    match action() {
        Ok(value) => {
            observer(ClauseScenarioIngestStageEvent::Passed {
                stage,
                elapsed: started.elapsed(),
            });
            Ok(value)
        }
        Err(error) => {
            observer(ClauseScenarioIngestStageEvent::Failed {
                stage,
                elapsed: started.elapsed(),
                message: error.status_message(),
            });
            Err(error)
        }
    }
}

/// Ingest a `.clause` path with caller-supplied resolver; emit StructuralRebindReady Spec.
///
/// Relative `source_json` / `include_json` paths resolve against the **clause file directory**
/// (`source_base`), not process CWD (STUDIO-CLAUSE-LOADER-SIMPLIFY-0 / 11.1 residual).
pub fn ingest_clause_scenario_path(
    path: &Path,
    options: &ClauseScenarioIngestOptions,
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    ingest_clause_scenario_path_staged(path, options, &mut |_| {})
}

/// Production path with presentation-only observation at the existing real call boundaries.
pub fn ingest_clause_scenario_path_staged(
    path: &Path,
    options: &ClauseScenarioIngestOptions,
    observer: &mut impl FnMut(ClauseScenarioIngestStageEvent),
) -> Result<ClauseScenarioIngestResult, ClauseScenarioIngestError> {
    let canonical_path = path.canonicalize().map_err(|e| source_failure(path, e))?;
    let path = canonical_path.as_path();
    let source_base = path.parent().map(Path::to_path_buf);
    let (raw, source) = observe_ingest_stage(ClauseScenarioIngestStage::Resolve, observer, || {
        if options.projection_mode != ClauseScenarioProjectionMode::StructuralRebindReady {
            return Err(ClauseScenarioIngestError::UnsupportedProjectionMode);
        }
        let raw = std::fs::read_to_string(path)?;
        let resolved = apply_source_resolver(&raw, &options.source_resolver)?;
        Ok((raw, resolved))
    })?;
    let document = observe_ingest_stage(ClauseScenarioIngestStage::Parse, observer, || {
        let parsed = parse_raw_document(source.as_bytes()).map_err(|e| source_failure(path, e))?;
        expand_document(&parsed, &ExpansionInput::default()).map_err(|e| source_failure(path, e))
    })?;
    let dependencies = declared_dependencies(&document, path)?;
    let pack = observe_ingest_stage(ClauseScenarioIngestStage::Hydrate, observer, || {
        hydrate_scenario_with_source_base(&document, source_base.as_deref())
            .map_err(|e| source_failure(path, e))
    })?;
    let (scenario, report) =
        observe_ingest_stage(ClauseScenarioIngestStage::Rebind, observer, || {
            rebind_pack_to_structural_rebind_ready(&pack).map_err(|e| source_failure(path, e))
        })?;
    if declared_dependencies(&document, path)? != dependencies {
        return Err(source_failure(
            path,
            "dependency changed during source admission",
        ));
    }
    let resolver_entries = options
        .source_resolver
        .placeholder_paths
        .iter()
        .filter(|(token, _)| raw.contains(token.as_str()))
        .map(|(token, resolved)| {
            Ok((
                token.clone(),
                relative_source_path(resolved, path.parent().unwrap())?,
            ))
        })
        .collect::<Result<_, ClauseScenarioIngestError>>()?;
    let source_cache = ClauseScenarioCache {
        format: CLAUSE_CACHE_FORMAT.into(),
        source_path: path.to_string_lossy().into_owned(),
        source_identity: clause_source_content_identity(raw.as_bytes()),
        document: parse_raw_document(raw.as_bytes()).map_err(|e| source_failure(path, e))?,
        dependencies,
        resolver_entries,
    };
    Ok(ClauseScenarioIngestResult {
        source_path: Some(path.to_path_buf()),
        pack,
        scenario,
        report,
        source_cache: Some(source_cache),
    })
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
    let document = expand_document(&document, &ExpansionInput::default())
        .map_err(|e| source_failure(Path::new("<source-bytes>"), e))?;
    let pack = hydrate_scenario_with_source_base(&document, source_base)?;
    let (scenario, report) = rebind_pack_to_structural_rebind_ready(&pack)?;
    Ok(ClauseScenarioIngestResult {
        source_path: None,
        pack,
        scenario,
        report,
        source_cache: None,
    })
}

/// Save produced Spec through existing Studio authority path helper.
pub fn save_clause_scenario_authority_to_path(
    path: &Path,
    scenario: &SimThingScenarioSpec,
) -> Result<(), ClauseScenarioIngestError> {
    Ok(save_scenario_authority_to_path(path, scenario)?)
}

/// Ingest the native source, persist its reproducible cache, and build the admitted Studio session.
///
/// Attaches an authored live profile from the hydrate pack so the live bridge can open
/// the field-bearing `open_from_spec` path without workshop residue.
pub fn load_clause_studio_session_from_path(
    clause_path: &Path,
    options: &ClauseScenarioIngestOptions,
    scenario_json_path: &Path,
    profile_hint: Option<GenerationProfile>,
) -> Result<(ClauseScenarioIngestResult, StudioSession), ClauseScenarioIngestError> {
    let ingest = ingest_clause_scenario_path(clause_path, options)?;
    save_clause_scenario_cache_to_path(scenario_json_path, &ingest)?;
    let session = load_studio_session_from_clause_ingest_result(
        &ingest,
        scenario_json_path.to_path_buf(),
        profile_hint,
    )?;
    Ok((ingest, session))
}

/// Load Studio session from an already-produced StructuralRebindReady Spec authority.
///
/// Retains the hydrate pack as an authored live profile for field-bearing open.
pub fn load_studio_session_from_clause_ingest_result(
    result: &ClauseScenarioIngestResult,
    scenario_path_label: PathBuf,
    profile_hint: Option<GenerationProfile>,
) -> Result<StudioSession, ClauseScenarioIngestError> {
    let mut profile = authored_live_profile_from_pack(&result.pack)
        .map_err(ClauseScenarioIngestError::SourceResolution)?;
    if let Some(cache) = &result.source_cache {
        profile.source_cache_provenance = Some(StudioSourceCacheProvenance {
            cache: cache.clone(),
            scenario_identity: simthing_spec::scenario_authority_digest(&result.scenario)
                .map_err(|e| source_failure(&scenario_path_label, e.message))?,
            profile_identity: authored_profile_content_identity(&profile)
                .map_err(|e| source_failure(&scenario_path_label, e))?,
        });
    }
    Ok(StudioSession::from_loaded_scenario(
        result.scenario.clone(),
        scenario_path_label,
        profile_hint,
    )
    .map_err(ScenarioIoError::from)?
    .with_authored_live_profile(profile))
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
