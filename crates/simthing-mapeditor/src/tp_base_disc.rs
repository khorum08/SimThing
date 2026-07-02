use std::collections::BTreeMap;

use simthing_clausething::{parse_stellaris_star_name_catalog, StellarisStarNameError};
use simthing_core::{SimThing, SimThingId};
use simthing_mapgenerator::{
    build_generation_report, generate_galaxy_with_structure, structure_options_from_params,
    MapGeneratorParams, ReportArtifacts, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
    MAP_QUALITY_PASS,
};
use simthing_spec::{
    save_scenario_spec_to_canonical_json, SimThingScenarioProvenance, SimThingScenarioSpec,
};
use thiserror::Error;

use crate::generation::{run_generation, GenerationError, GenerationPreset, GenerationProfile};
use crate::hydration::{generate_simthing_spec_scenario_with_star_names, StudioHydrationError};

pub const TP_BASE_DISC_RUNG_ID: &str = "TP-BASE-DISC-GEN-0";
pub const TP_BASE_DISC_SCENARIO_ID: &str = "tp_base_disc_1500";
pub const TP_BASE_DISC_SEED: u64 = 770_421;
pub const TP_BASE_DISC_STARS: u32 = 1500;
pub const TP_BASE_DISC_LATTICE_EDGE: u32 = 300;
pub const TP_BASE_DISC_NAME_CORPUS_SOURCE: &str = "simthing-authored:tp_base_disc_star_names_v1";
pub const TP_BASE_DISC_NAME_ASSIGNMENT_MODE: &str =
    "stellaris_star_names_seeded_shuffle_no_replacement_cycle";

#[derive(Debug, Clone)]
pub struct TpBaseDiscArtifact {
    pub scenario: SimThingScenarioSpec,
    pub canonical_json: String,
    pub authority_digest: u64,
    pub byte_len: u32,
    pub map_quality_status: &'static str,
}

#[derive(Debug, Error)]
pub enum TpBaseDiscError {
    #[error("generation failed: {0}")]
    Generation(#[from] GenerationError),
    #[error("star-name corpus failed: {0}")]
    StarNames(#[from] StellarisStarNameError),
    #[error("studio hydration failed: {0}")]
    Hydration(#[from] StudioHydrationError),
    #[error("scenario canonical save failed")]
    Save,
    #[error("scenario id normalization failed: {0}")]
    IdNormalization(&'static str),
    #[error("generator params JSON failed: {0}")]
    ParamsJson(#[from] serde_json::Error),
    #[error("map quality status is {status}, expected PASS: {warnings:?}")]
    MapQuality {
        status: &'static str,
        warnings: Vec<String>,
    },
    #[error("recorded generator params missing from scenario provenance")]
    MissingRecordedParams,
    #[error("recorded generator params do not match canonical seed/star/lattice/shape")]
    RecordedParamsMismatch,
}

pub fn tp_base_disc_generation_profile() -> GenerationProfile {
    let mut profile = GenerationPreset::Disc1500Connected.to_profile();
    profile.seed = TP_BASE_DISC_SEED;
    profile.star_count = TP_BASE_DISC_STARS;
    profile.lattice_edge = TP_BASE_DISC_LATTICE_EDGE;
    profile.max_hyperlane_distance = 7.0;
    profile.cluster_count = 5;
    profile.cluster_radius = 400.0;
    profile
}

pub fn tp_base_disc_star_name_corpus_source() -> String {
    let mut source = String::from("star_names = {\n");
    for id in 1..=TP_BASE_DISC_STARS {
        source.push_str(&format!("    \"Aster {id:04}\"\n"));
    }
    source.push_str("}\n");
    source
}

pub fn generate_tp_base_disc_artifact() -> Result<TpBaseDiscArtifact, TpBaseDiscError> {
    let profile = tp_base_disc_generation_profile();
    generate_tp_base_disc_artifact_from_profile(&profile)
}

pub fn generate_tp_base_disc_artifact_from_profile(
    profile: &GenerationProfile,
) -> Result<TpBaseDiscArtifact, TpBaseDiscError> {
    let output = run_generation(profile)?;
    if output.report.output.map_quality_status != MAP_QUALITY_PASS {
        return Err(TpBaseDiscError::MapQuality {
            status: output.report.output.map_quality_status,
            warnings: output.report.output.map_quality_warnings.clone(),
        });
    }

    let catalog =
        parse_stellaris_star_name_catalog(tp_base_disc_star_name_corpus_source().as_bytes())?;
    let assignments = catalog.assign_to_systems(
        profile.seed,
        output
            .result
            .placement
            .systems
            .iter()
            .map(|system| system.id),
    );
    let mut scenario = generate_simthing_spec_scenario_with_star_names(&output, &assignments)?;
    canonicalize_tp_base_disc_scenario(&mut scenario, profile)?;
    canonical_artifact(scenario, output.report.output.map_quality_status)
}

pub fn generate_tp_base_disc_artifact_from_recorded_metadata(
    provenance: &SimThingScenarioProvenance,
) -> Result<TpBaseDiscArtifact, TpBaseDiscError> {
    if provenance.generator_params_json.trim().is_empty() {
        return Err(TpBaseDiscError::MissingRecordedParams);
    }
    if provenance.generator_profile_id != GenerationPreset::Disc1500Connected.id() {
        return Err(TpBaseDiscError::RecordedParamsMismatch);
    }
    let params = MapGeneratorParams::from_json_str(&provenance.generator_params_json)?;
    validate_recorded_params(&params)?;

    let registry = ShapeRegistry::default();
    params.validate(&registry).map_err(GenerationError::from)?;
    let (hyperlane, special, _partition, cluster) =
        structure_options_from_params(&params).map_err(GenerationError::from)?;
    let result = generate_galaxy_with_structure(
        &params,
        &registry,
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        None,
        Some(cluster),
    )
    .map_err(GenerationError::from)?;
    let report = build_generation_report(&params, &result, ReportArtifacts::new());
    if report.output.map_quality_status != MAP_QUALITY_PASS {
        return Err(TpBaseDiscError::MapQuality {
            status: report.output.map_quality_status,
            warnings: report.output.map_quality_warnings.clone(),
        });
    }

    let catalog =
        parse_stellaris_star_name_catalog(tp_base_disc_star_name_corpus_source().as_bytes())?;
    let assignments = catalog.assign_to_systems(
        params.seed,
        result.placement.systems.iter().map(|system| system.id),
    );
    let output = crate::generation::GenerationRunOutput {
        result,
        report: report.clone(),
        galaxy_display_name: "Unnamed Elliptical Galaxy".to_string(),
    };
    let mut scenario = generate_simthing_spec_scenario_with_star_names(&output, &assignments)?;
    scenario.scenario_id = TP_BASE_DISC_SCENARIO_ID.to_string();
    scenario.provenance = provenance.clone();
    canonical_artifact(scenario, report.output.map_quality_status)
}

pub fn stamp_tp_base_disc_metadata(
    scenario: &mut SimThingScenarioSpec,
    profile: &GenerationProfile,
) -> Result<(), TpBaseDiscError> {
    let params_json = profile.to_map_generator_params().to_json_string_pretty()?;
    scenario.scenario_id = TP_BASE_DISC_SCENARIO_ID.to_string();
    scenario.provenance.source = "MapGeneratorLibrary".to_string();
    scenario.provenance.generator_seed = profile.seed;
    scenario.provenance.generator_shape = profile.shape.clone();
    scenario.provenance.generator_profile_id = profile.preset_id.clone();
    scenario.provenance.generator_params_json = params_json;
    scenario.provenance.name_corpus_source = TP_BASE_DISC_NAME_CORPUS_SOURCE.to_string();
    scenario.provenance.name_assignment_mode = TP_BASE_DISC_NAME_ASSIGNMENT_MODE.to_string();
    Ok(())
}

pub fn canonicalize_tp_base_disc_scenario(
    scenario: &mut SimThingScenarioSpec,
    profile: &GenerationProfile,
) -> Result<(), TpBaseDiscError> {
    stamp_tp_base_disc_metadata(scenario, profile)?;
    normalize_scenario_ids(scenario)
}

fn canonical_artifact(
    mut scenario: SimThingScenarioSpec,
    map_quality_status: &'static str,
) -> Result<TpBaseDiscArtifact, TpBaseDiscError> {
    normalize_scenario_ids(&mut scenario)?;
    let save =
        save_scenario_spec_to_canonical_json(&scenario).map_err(|_| TpBaseDiscError::Save)?;
    Ok(TpBaseDiscArtifact {
        scenario,
        canonical_json: save.canonical_json,
        authority_digest: save.authority_digest,
        byte_len: save.byte_len,
        map_quality_status,
    })
}

fn validate_recorded_params(params: &MapGeneratorParams) -> Result<(), TpBaseDiscError> {
    if params.seed != TP_BASE_DISC_SEED
        || params.scale_core.num_stars != TP_BASE_DISC_STARS
        || params.scale_core.lattice_size != Some(TP_BASE_DISC_LATTICE_EDGE)
        || params.shape.shape != "elliptical"
    {
        return Err(TpBaseDiscError::RecordedParamsMismatch);
    }
    Ok(())
}

fn normalize_scenario_ids(scenario: &mut SimThingScenarioSpec) -> Result<(), TpBaseDiscError> {
    let old_map_container_raw = scenario
        .structural_grid
        .map_container_id
        .parse::<u32>()
        .map_err(|_| TpBaseDiscError::IdNormalization("map container id is not raw u32"))?;
    let mut next_raw = 1u32;
    let mut rewrites = BTreeMap::new();
    rewrite_simthing_ids(&mut scenario.root, &mut next_raw, &mut rewrites);
    let new_map_container_raw =
        rewrites
            .get(&old_map_container_raw)
            .copied()
            .ok_or(TpBaseDiscError::IdNormalization(
                "map container id was not present in scenario tree",
            ))?;
    scenario.structural_grid.map_container_id = new_map_container_raw.to_string();
    for placement in &mut scenario.structural_grid.placements {
        placement.simthing_id_raw = rewrites.get(&placement.simthing_id_raw).copied().ok_or(
            TpBaseDiscError::IdNormalization(
                "placement simthing id was not present in scenario tree",
            ),
        )?;
    }
    Ok(())
}

fn rewrite_simthing_ids(
    node: &mut SimThing,
    next_raw: &mut u32,
    rewrites: &mut BTreeMap<u32, u32>,
) {
    let old_raw = node.id.raw();
    let new_raw = *next_raw;
    *next_raw = next_raw.saturating_add(1);
    node.id = SimThingId::from_session_raw(new_raw);
    rewrites.insert(old_raw, new_raw);
    for child in &mut node.children {
        rewrite_simthing_ids(child, next_raw, rewrites);
    }
}
