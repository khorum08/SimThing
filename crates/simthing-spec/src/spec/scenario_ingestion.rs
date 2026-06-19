//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion and typed admission reports.
//!
//! Spec-owned ingestion authority: parse, validate, classify, and report deferrals without driver
//! compile or Studio presentation ownership.

use simthing_core::SimThingKind;

use super::scenario::{
    galaxy_map_id, game_session_child, game_session_galaxy_map, game_session_owners, gridcell_role,
    is_galaxy_map_entity, is_owner_entity_kind, owner_entity_id, owner_has_silo_metadata,
    resolve_map_container, scenario_metadata_seed, validate_legacy_world_root_compatibility,
    validate_scenario_game_session_child, validate_scenario_links,
    validate_scenario_root_authority, validate_session_galaxy_map, validate_session_owner_entities,
    validate_stead_mapping_consistency, ScenarioRootError, ScenarioRootValidationMode,
    ScenarioSerdeError, SimThingScenarioSpec, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
};
use super::session_resource_flow::{
    evaluate_owner_silo_flow, owner_silo_flow_suppresses_ingestion_deferral,
    OwnerSiloAdmissionClassification, OwnerSiloAdmissionReport,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScenarioIngestionProfile {
    pub require_canonical_tree: bool,
    pub admit_legacy_world_root: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioIngestionClassification {
    Admitted,
    PartiallyAdmitted,
    Rejected,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioDeferralKind {
    LegacyWorldRootCompatibility,
    PlanetsNotYetAdmitted,
    OwnerResourceFlowNotYetExecuted,
    CapabilityTreeNotYetExecuted,
    StudioStructuralPlacementEditNotYetSupported,
    MappingPlanCompileDeferred,
    GpuResidentExecutionDeferred,
    UnsupportedGridcellRole,
    UnsupportedChildLocationDepth,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioDeferral {
    pub kind: ScenarioDeferralKind,
    pub path: Option<String>,
    pub simthing_id_raw: Option<u32>,
    pub reason: String,
    pub scenario_remains_valid: bool,
    pub compile_can_continue: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioIngestionError {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScenarioValidationReport {
    pub json_parse_ok: bool,
    pub root_kind: Option<String>,
    pub canonical_validation_ok: bool,
    pub legacy_compat_ok: bool,
    pub seed_metadata_ok: bool,
    pub gamesession_ok: bool,
    pub owners_ok: bool,
    pub galaxy_map_ok: bool,
    pub stead_mapping_ok: bool,
    pub links_ok: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScenarioTreeAdmissionReport {
    pub has_game_session: bool,
    pub owner_count: u32,
    pub has_galaxy_map: bool,
    pub gridcell_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnerAdmissionReport {
    pub admitted_owner_ids: Vec<String>,
    pub owner_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GalaxyMapAdmissionReport {
    pub galaxy_map_id: Option<String>,
    pub gridcell_inert_count: u32,
    pub gridcell_star_system_count: u32,
    pub gridcell_unknown_role_count: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StructuralAdmissionReport {
    pub map_container_resolved: bool,
    pub placement_count: u32,
    pub stead_valid: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScenarioCompileReadinessReport {
    pub structural_n4_ready: bool,
    pub structural_n4_deferred: bool,
    pub mapping_plan_ready: bool,
    pub mapping_plan_deferred: bool,
    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioFingerprint {
    pub scenario_id: String,
    pub root_kind: String,
    pub subtree_size: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScenarioIngestionResult {
    pub source_name: String,
    pub scenario_fingerprint: Option<ScenarioFingerprint>,
    pub classification: ScenarioIngestionClassification,
    pub validation: ScenarioValidationReport,
    pub canonical_tree: ScenarioTreeAdmissionReport,
    pub owner_admission: OwnerAdmissionReport,
    pub galaxy_map_admission: GalaxyMapAdmissionReport,
    pub structural_admission: StructuralAdmissionReport,
    pub compile_readiness: ScenarioCompileReadinessReport,
    pub owner_silo: Option<OwnerSiloAdmissionReport>,
    pub deferrals: Vec<ScenarioDeferral>,
    pub errors: Vec<ScenarioIngestionError>,
}

pub fn ingest_scenario_from_str(
    source_name: &str,
    json: &str,
    profile: ScenarioIngestionProfile,
) -> (ScenarioIngestionResult, Option<SimThingScenarioSpec>) {
    let spec = match serde_json::from_str::<SimThingScenarioSpec>(json) {
        Ok(spec) => spec,
        Err(err) => {
            let mut result = empty_result(source_name);
            result.validation.json_parse_ok = false;
            push_error(
                &mut result,
                "json_parse",
                format!("failed to deserialize scenario authority: {err}"),
            );
            result.classification = ScenarioIngestionClassification::Rejected;
            return (result, None);
        }
    };
    let result = ingest_scenario(source_name, &spec, profile);
    (result, Some(spec))
}

pub fn ingest_scenario(
    source_name: &str,
    spec: &SimThingScenarioSpec,
    profile: ScenarioIngestionProfile,
) -> ScenarioIngestionResult {
    let mut result = empty_result(source_name);
    result.validation.json_parse_ok = true;
    result.validation.root_kind = Some(format!("{:?}", spec.root.kind));

    result.scenario_fingerprint = Some(ScenarioFingerprint {
        scenario_id: spec.scenario_id.clone(),
        root_kind: format!("{:?}", spec.root.kind),
        subtree_size: spec.root.subtree_size() as u32,
    });

    match spec.root.kind {
        SimThingKind::Scenario => ingest_canonical(spec, profile, &mut result),
        SimThingKind::World => ingest_legacy_world(spec, profile, &mut result),
        ref other => {
            push_error(
                &mut result,
                "invalid_root_kind",
                format!("scenario root kind {other:?} is not Scenario or legacy World"),
            );
        }
    }

    result.compile_readiness.mapping_plan_deferred = true;
    result.compile_readiness.note = Some(
        "driver structural N4 / mapping-plan compile evaluation belongs in simthing-driver".into(),
    );
    finalize_classification(&mut result);
    result
}

fn empty_result(source_name: &str) -> ScenarioIngestionResult {
    ScenarioIngestionResult {
        source_name: source_name.to_string(),
        scenario_fingerprint: None,
        classification: ScenarioIngestionClassification::Rejected,
        validation: ScenarioValidationReport::default(),
        canonical_tree: ScenarioTreeAdmissionReport::default(),
        owner_admission: OwnerAdmissionReport::default(),
        galaxy_map_admission: GalaxyMapAdmissionReport::default(),
        structural_admission: StructuralAdmissionReport::default(),
        compile_readiness: ScenarioCompileReadinessReport {
            mapping_plan_deferred: true,
            ..Default::default()
        },
        owner_silo: None,
        deferrals: Vec::new(),
        errors: Vec::new(),
    }
}

fn ingest_canonical(
    spec: &SimThingScenarioSpec,
    profile: ScenarioIngestionProfile,
    result: &mut ScenarioIngestionResult,
) {
    if profile.require_canonical_tree {
        validate_step(
            result,
            "canonical_validation",
            validate_scenario_root_authority(spec, ScenarioRootValidationMode::Canonical),
            |r| r.validation.canonical_validation_ok = true,
        );
    } else {
        validate_step(
            result,
            "canonical_validation",
            validate_scenario_root_authority(spec, ScenarioRootValidationMode::Canonical),
            |r| r.validation.canonical_validation_ok = true,
        );
    }

    result.validation.seed_metadata_ok =
        scenario_metadata_seed(&spec.root).is_some() || spec.provenance.generator_seed != 0;

    validate_step(
        result,
        "gamesession",
        validate_scenario_game_session_child(spec),
        |r| r.validation.gamesession_ok = true,
    );
    validate_step(
        result,
        "owners",
        validate_session_owner_entities(spec),
        |r| r.validation.owners_ok = true,
    );
    validate_step(
        result,
        "galaxy_map",
        validate_session_galaxy_map(spec),
        |r| r.validation.galaxy_map_ok = true,
    );
    validate_step(
        result,
        "stead_mapping",
        validate_stead_mapping_consistency(spec),
        |r| {
            r.validation.stead_mapping_ok = true;
            r.structural_admission.stead_valid = true;
        },
    );
    validate_step(result, "links", validate_scenario_links(spec), |r| {
        r.validation.links_ok = true
    });

    if result.errors.is_empty() {
        populate_canonical_reports(spec, result);
        integrate_owner_silo_flow(spec, result);
    } else if result.validation.gamesession_ok {
        populate_partial_canonical_reports(spec, result);
    }
}

fn ingest_legacy_world(
    spec: &SimThingScenarioSpec,
    profile: ScenarioIngestionProfile,
    result: &mut ScenarioIngestionResult,
) {
    if !profile.admit_legacy_world_root {
        push_error(
            result,
            "legacy_world_rejected",
            "legacy World-root scenarios require admit_legacy_world_root in ingestion profile"
                .into(),
        );
        return;
    }

    validate_step(
        result,
        "legacy_compat",
        validate_legacy_world_root_compatibility(spec),
        |r| r.validation.legacy_compat_ok = true,
    );
    validate_step(
        result,
        "stead_mapping",
        validate_stead_mapping_consistency(spec),
        |r| {
            r.validation.stead_mapping_ok = true;
            r.structural_admission.stead_valid = true;
        },
    );
    validate_step(result, "links", validate_scenario_links(spec), |r| {
        r.validation.links_ok = true
    });

    if result.errors.is_empty() {
        push_deferral(
            result,
            ScenarioDeferralKind::LegacyWorldRootCompatibility,
            None,
            None,
            "legacy World-root fixture admitted through explicit compatibility path only",
            true,
            true,
        );
        populate_legacy_reports(spec, result);
    }
}

fn validate_step(
    result: &mut ScenarioIngestionResult,
    code: &str,
    outcome: Result<(), impl std::error::Error>,
    on_ok: impl FnOnce(&mut ScenarioIngestionResult),
) {
    match outcome {
        Ok(()) => on_ok(result),
        Err(err) => push_error(result, code, err.to_string()),
    }
}

fn push_error(result: &mut ScenarioIngestionResult, code: &str, message: String) {
    result.errors.push(ScenarioIngestionError {
        code: code.to_string(),
        message,
    });
}

fn push_deferral(
    result: &mut ScenarioIngestionResult,
    kind: ScenarioDeferralKind,
    path: Option<String>,
    simthing_id_raw: Option<u32>,
    reason: &str,
    scenario_remains_valid: bool,
    compile_can_continue: bool,
) {
    result.deferrals.push(ScenarioDeferral {
        kind,
        path,
        simthing_id_raw,
        reason: reason.to_string(),
        scenario_remains_valid,
        compile_can_continue,
    });
}

fn populate_canonical_reports(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    if let Ok(game_session) = game_session_child(spec) {
        result.canonical_tree.has_game_session = true;
        result.canonical_tree.owner_count = game_session
            .children
            .iter()
            .filter(|c| is_owner_entity_kind(&c.kind))
            .count() as u32;
        result.canonical_tree.has_galaxy_map = game_session
            .children
            .iter()
            .any(|c| is_galaxy_map_entity(c));
    }

    if let Ok(owners) = game_session_owners(spec) {
        result.owner_admission.owner_count = owners.len() as u32;
        result.owner_admission.admitted_owner_ids =
            owners.iter().filter_map(|o| owner_entity_id(o)).collect();
    }

    if let Ok(galaxy_map) = game_session_galaxy_map(spec) {
        result.galaxy_map_admission.galaxy_map_id = galaxy_map_id(galaxy_map);
        let gridcells: Vec<_> = galaxy_map
            .children
            .iter()
            .filter(|c| c.kind == SimThingKind::Location && !is_galaxy_map_entity(c))
            .collect();
        result.canonical_tree.gridcell_count = gridcells.len() as u32;
        for gridcell in gridcells {
            classify_gridcell_role(gridcell, result);
            scan_gridcell_children(gridcell, result);
        }
    }

    result.structural_admission.placement_count = spec.structural_grid.placements.len() as u32;
    result.structural_admission.map_container_resolved = resolve_map_container(spec).is_ok();

    scan_owner_subtrees(spec, result);
}

fn populate_partial_canonical_reports(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    if let Ok(game_session) = game_session_child(spec) {
        result.canonical_tree.has_game_session = true;
        result.canonical_tree.owner_count = game_session
            .children
            .iter()
            .filter(|c| is_owner_entity_kind(&c.kind))
            .count() as u32;
    }
}

fn populate_legacy_reports(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    result.structural_admission.placement_count = spec.structural_grid.placements.len() as u32;
    result.structural_admission.map_container_resolved = resolve_map_container(spec).is_ok();
    if let Ok(map) = resolve_map_container(spec) {
        let gridcells: Vec<_> = map
            .children
            .iter()
            .filter(|c| c.kind == SimThingKind::Location)
            .collect();
        result.canonical_tree.gridcell_count = gridcells.len() as u32;
        for gridcell in gridcells {
            classify_gridcell_role(gridcell, result);
        }
    }
    push_deferral(
        result,
        ScenarioDeferralKind::MappingPlanCompileDeferred,
        None,
        None,
        "canonical Scenario tree not present; mapping plan compile uses legacy spatial path only",
        true,
        true,
    );
}

fn classify_gridcell_role(
    gridcell: &simthing_core::SimThing,
    result: &mut ScenarioIngestionResult,
) {
    match gridcell_role(gridcell).as_deref() {
        Some(GALAXY_GRIDCELL_ROLE_INERT) => {
            result.galaxy_map_admission.gridcell_inert_count += 1;
        }
        Some(GALAXY_GRIDCELL_ROLE_STAR_SYSTEM) => {
            result.galaxy_map_admission.gridcell_star_system_count += 1;
        }
        Some(role) => {
            result.galaxy_map_admission.gridcell_unknown_role_count += 1;
            push_deferral(
                result,
                ScenarioDeferralKind::UnsupportedGridcellRole,
                Some(format!("gridcell/role:{role}")),
                Some(gridcell.id.raw()),
                &format!("gridcell role `{role}` is not yet admitted by current engine surfaces"),
                true,
                true,
            );
        }
        None => {
            result.galaxy_map_admission.gridcell_unknown_role_count += 1;
        }
    }
}

fn scan_gridcell_children(
    gridcell: &simthing_core::SimThing,
    result: &mut ScenarioIngestionResult,
) {
    for child in &gridcell.children {
        if child.kind == SimThingKind::Location {
            push_deferral(
                result,
                ScenarioDeferralKind::PlanetsNotYetAdmitted,
                Some("gridcell/child_location".into()),
                Some(child.id.raw()),
                "nested Location child under gridcell represents deferred planet/child-location depth",
                true,
                true,
            );
        } else if matches!(child.kind, SimThingKind::Custom(ref name) if name == "Planet") {
            push_deferral(
                result,
                ScenarioDeferralKind::PlanetsNotYetAdmitted,
                Some("gridcell/planet".into()),
                Some(child.id.raw()),
                "planet entities are valid schema but not yet admitted by current engine surfaces",
                true,
                true,
            );
        }
    }
}

fn scan_owner_subtrees(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    let Ok(owners) = game_session_owners(spec) else {
        return;
    };
    for owner in owners {
        for child in &owner.children {
            if matches!(child.kind, SimThingKind::Custom(ref name) if name == "CapabilityTree") {
                push_deferral(
                    result,
                    ScenarioDeferralKind::CapabilityTreeNotYetExecuted,
                    Some(format!(
                        "owner/{}/capability_tree",
                        owner_entity_id(owner).unwrap_or_default()
                    )),
                    Some(child.id.raw()),
                    "owner capability/talent subtree placeholders are not yet executed",
                    true,
                    true,
                );
            }
        }
    }
}

fn integrate_owner_silo_flow(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    let silo_report = evaluate_owner_silo_flow(spec);
    let suppress = owner_silo_flow_suppresses_ingestion_deferral(&silo_report);
    result.owner_silo = Some(silo_report.clone());

    if silo_report.classification == OwnerSiloAdmissionClassification::Rejected {
        for err in &silo_report.errors {
            push_error(
                result,
                "owner_silo",
                format!("{:?}: {}", err.kind, err.message),
            );
        }
        return;
    }

    if suppress {
        return;
    }

    if silo_report.silo_owner_count > 0 && silo_report.participant_count == 0 {
        if let Ok(owners) = game_session_owners(spec) {
            for owner in owners {
                if owner_has_silo_metadata(owner) {
                    push_deferral(
                        result,
                        ScenarioDeferralKind::OwnerResourceFlowNotYetExecuted,
                        Some(format!(
                            "owner/{}",
                            owner_entity_id(owner).unwrap_or_default()
                        )),
                        Some(owner.id.raw()),
                        "owner silo metadata present without admitted participant flow properties",
                        true,
                        true,
                    );
                }
            }
        }
    }
}

fn finalize_classification(result: &mut ScenarioIngestionResult) {
    if !result.errors.is_empty() {
        result.classification = ScenarioIngestionClassification::Rejected;
        return;
    }

    if result
        .deferrals
        .iter()
        .any(|d| d.kind == ScenarioDeferralKind::LegacyWorldRootCompatibility)
    {
        result.classification = ScenarioIngestionClassification::PartiallyAdmitted;
        return;
    }

    let feature_deferrals: Vec<_> = result
        .deferrals
        .iter()
        .filter(|d| {
            matches!(
                d.kind,
                ScenarioDeferralKind::PlanetsNotYetAdmitted
                    | ScenarioDeferralKind::CapabilityTreeNotYetExecuted
                    | ScenarioDeferralKind::OwnerResourceFlowNotYetExecuted
                    | ScenarioDeferralKind::UnsupportedGridcellRole
                    | ScenarioDeferralKind::UnsupportedChildLocationDepth
            )
        })
        .collect();

    if feature_deferrals.is_empty() {
        result.classification = ScenarioIngestionClassification::Admitted;
        return;
    }

    if !feature_deferrals.is_empty()
        && feature_deferrals.len() == result.deferrals.len()
        && result
            .deferrals
            .iter()
            .all(|d| matches!(d.kind, ScenarioDeferralKind::PlanetsNotYetAdmitted))
    {
        result.classification = ScenarioIngestionClassification::Unsupported;
        return;
    }

    if !feature_deferrals.is_empty()
        && feature_deferrals
            .iter()
            .all(|d| matches!(d.kind, ScenarioDeferralKind::UnsupportedGridcellRole))
    {
        result.classification = ScenarioIngestionClassification::PartiallyAdmitted;
        return;
    }

    result.classification = ScenarioIngestionClassification::PartiallyAdmitted;
}

pub fn ingestion_error_from_root(err: ScenarioRootError) -> ScenarioIngestionError {
    ScenarioIngestionError {
        code: "scenario_root".into(),
        message: err.to_string(),
    }
}

pub fn ingestion_error_from_serde(err: ScenarioSerdeError) -> ScenarioIngestionError {
    ScenarioIngestionError {
        code: "scenario_serde".into(),
        message: err.to_string(),
    }
}
