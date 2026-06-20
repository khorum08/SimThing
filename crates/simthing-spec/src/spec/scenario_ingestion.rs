//! GENERAL-SCENARIO-INGESTION-ADMISSION-0 — arbitrary Scenario ingestion and typed admission reports.
//!
//! Spec-owned ingestion authority: parse, validate, classify, and report deferrals without driver
//! compile or Studio presentation ownership.

use simthing_core::SimThingKind;

use super::local_participant_effects::evaluate_local_participant_effects;
use super::owner_silo_disburse_down::owner_silo_demand_buckets_from_planet_child_rf;
use super::owner_silo_runtime_writeback::{
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario,
};
use super::planet_child_location::{
    evaluate_planet_child_locations, PlanetChildLocationAdmissionClassification,
    PlanetChildLocationAdmissionErrorKind, PlanetChildLocationAdmissionReport,
};
use super::planet_child_rf::{
    evaluate_planet_child_rf_admission, evaluate_planet_child_rf_reduce_up,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionReport,
    PlanetChildRfReduceUpReport,
};
use super::runtime_local_allocation::apply_runtime_local_allocations_from_disburse_down;
use super::runtime_rf_tick::evaluate_runtime_rf_tick;
use super::runtime_tick_history::replay_runtime_tick_history;
use super::runtime_tick_shell::{evaluate_runtime_tick_shell, RuntimeTickId};
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
    PlanetSimulationDeferred,
    UnsupportedChildLocationRole,
    PlanetOwnershipResolutionDeferred,
    PlanetNonGridChildSimulationDeferred,
    PlanetNonGridChildUnsupportedKind,
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
    /// Admitted owner-silo participants can lower to existing AccumulatorOp/GPU surfaces.
    pub owner_silo_gpu_participant_accumulation_ready: bool,
    /// Full owner-silo state mutation (reduce-up/disburse-down writes) remains deferred.
    pub owner_silo_full_state_mutation_deferred: bool,
    /// Admitted planet/non-grid child RF participants can lower to existing AccumulatorOp/GPU surfaces.
    pub planet_child_rf_gpu_participant_accumulation_ready: bool,
    /// Scoped planet child RF reduce-up oracle is available for admitted participants.
    pub planet_child_rf_reduce_up_ready: bool,
    /// Runtime owner-silo writeback from scoped reduce-up is ready (scenario authority unchanged).
    pub owner_silo_runtime_writeback_ready: bool,
    /// Runtime writeback defers scenario authority mutation and disburse-down.
    pub owner_silo_runtime_writeback_deferred: bool,
    /// Runtime owner-silo disburse-down allocation oracle is ready (scenario authority unchanged).
    pub owner_silo_disburse_down_ready: bool,
    /// Disburse-down defers allocation application and scenario authority mutation.
    pub owner_silo_disburse_down_deferred: bool,
    /// Runtime-local allocation application from disburse-down is ready (scenario authority unchanged).
    pub runtime_local_allocation_ready: bool,
    /// Full economy execution and participant property mutation remain deferred.
    pub runtime_local_allocation_deferred: bool,
    /// Composed runtime RF tick report is ready (scenario authority unchanged).
    pub runtime_rf_tick_ready: bool,
    /// Economy/local effects and Scenario authority mutation remain deferred at tick boundary.
    pub runtime_rf_tick_deferred: bool,
    /// Runtime tick execution shell can evaluate the composed RF tick plan.
    pub runtime_tick_shell_ready: bool,
    /// Economy/local effects and Scenario authority mutation remain deferred at tick shell.
    pub runtime_tick_shell_deferred: bool,
    /// Local participant effect previews can be evaluated under the tick shell.
    pub local_participant_effects_ready: bool,
    /// Economy execution, participant property mutation, and Scenario authority mutation remain deferred.
    pub local_participant_effects_deferred: bool,
    /// Runtime tick history entry and replay report can be evaluated.
    pub runtime_tick_history_ready: bool,
    /// Persistent history, economy/local effects, and Scenario authority mutation remain deferred.
    pub runtime_tick_history_deferred: bool,
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
    pub planet_child_location: Option<PlanetChildLocationAdmissionReport>,
    pub planet_child_rf: Option<PlanetChildRfAdmissionReport>,
    pub planet_child_rf_reduce_up: Option<PlanetChildRfReduceUpReport>,
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
        planet_child_location: None,
        planet_child_rf: None,
        planet_child_rf_reduce_up: None,
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
        }
    }

    integrate_planet_child_locations(spec, result);
    integrate_planet_child_rf(spec, result);
    integrate_planet_child_rf_reduce_up(spec, result);
    integrate_owner_silo_runtime_writeback(spec, result);
    integrate_owner_silo_disburse_down(spec, result);
    integrate_runtime_local_allocation(spec, result);
    integrate_runtime_rf_tick(spec, result);
    integrate_runtime_tick_shell(spec, result);
    integrate_local_participant_effects(spec, result);
    integrate_runtime_tick_history(spec, result);

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

fn integrate_planet_child_locations(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    let planet_report = evaluate_planet_child_locations(spec);
    for err in &planet_report.errors {
        push_error(
            result,
            "planet_child_location",
            format!(
                "{}: {}",
                planet_child_location_error_kind_label(err.kind),
                err.message
            ),
        );
    }
    for deferral in &planet_report.deferrals {
        push_deferral(
            result,
            map_planet_deferral_kind(deferral.kind),
            deferral.path.clone(),
            deferral.simthing_id_raw,
            &deferral.reason,
            true,
            true,
        );
    }
    result.planet_child_location = Some(planet_report);
}

fn map_planet_deferral_kind(kind: PlanetChildLocationAdmissionErrorKind) -> ScenarioDeferralKind {
    match kind {
        PlanetChildLocationAdmissionErrorKind::PlanetSimulationDeferred => {
            ScenarioDeferralKind::PlanetSimulationDeferred
        }
        PlanetChildLocationAdmissionErrorKind::UnsupportedChildLocationRole => {
            ScenarioDeferralKind::UnsupportedChildLocationRole
        }
        PlanetChildLocationAdmissionErrorKind::DeepPlanetChildDeferred => {
            ScenarioDeferralKind::UnsupportedChildLocationDepth
        }
        PlanetChildLocationAdmissionErrorKind::PlanetOwnershipResolutionDeferred => {
            ScenarioDeferralKind::PlanetOwnershipResolutionDeferred
        }
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildSimulationDeferred => {
            ScenarioDeferralKind::PlanetNonGridChildSimulationDeferred
        }
        PlanetChildLocationAdmissionErrorKind::PlanetNonGridChildUnsupportedKind => {
            ScenarioDeferralKind::PlanetNonGridChildUnsupportedKind
        }
        _ => ScenarioDeferralKind::UnsupportedChildLocationRole,
    }
}

fn planet_child_location_error_kind_label(
    kind: PlanetChildLocationAdmissionErrorKind,
) -> &'static str {
    super::planet_child_location::planet_child_location_error_kind_label(kind)
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

fn integrate_planet_child_rf(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    let rf_report = evaluate_planet_child_rf_admission(spec);
    result.planet_child_rf = Some(rf_report.clone());

    if rf_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        for err in &rf_report.errors {
            push_error(
                result,
                "planet_child_rf",
                format!("{:?}: {}", err.kind, err.message),
            );
        }
        return;
    }

    result
        .compile_readiness
        .planet_child_rf_gpu_participant_accumulation_ready = rf_report.total_participant_count > 0
        && rf_report.classification != PlanetChildRfAdmissionClassification::Unsupported;
}

fn integrate_planet_child_rf_reduce_up(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    let reduce_up_report = evaluate_planet_child_rf_reduce_up(spec);
    result.planet_child_rf_reduce_up = Some(reduce_up_report.clone());

    if reduce_up_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        for err in &reduce_up_report.errors {
            push_error(
                result,
                "planet_child_rf_reduce_up",
                format!("{:?}: {}", err.kind, err.message),
            );
        }
        return;
    }

    result.compile_readiness.planet_child_rf_reduce_up_ready = reduce_up_report.bucket_count > 0
        && reduce_up_report.classification != PlanetChildRfAdmissionClassification::Unsupported;
    result
        .compile_readiness
        .owner_silo_full_state_mutation_deferred = true;
}

fn integrate_owner_silo_runtime_writeback(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    result
        .compile_readiness
        .owner_silo_runtime_writeback_deferred = true;

    let reduce_up = evaluate_planet_child_rf_reduce_up(spec);
    if reduce_up.classification == PlanetChildRfAdmissionClassification::Rejected {
        return;
    }
    if runtime_owner_silo_states_from_scenario(spec).is_err() {
        return;
    }
    if owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up).is_err() {
        return;
    }

    result.compile_readiness.owner_silo_runtime_writeback_ready = true;
    result
        .compile_readiness
        .owner_silo_runtime_writeback_deferred = true;
}

fn integrate_owner_silo_disburse_down(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    result.compile_readiness.owner_silo_disburse_down_deferred = true;

    if !result.compile_readiness.owner_silo_runtime_writeback_ready {
        return;
    }
    if owner_silo_demand_buckets_from_planet_child_rf(spec).is_err() {
        return;
    }

    result.compile_readiness.owner_silo_disburse_down_ready = true;
    result.compile_readiness.owner_silo_disburse_down_deferred = true;
}

fn integrate_runtime_local_allocation(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    result.compile_readiness.runtime_local_allocation_deferred = true;

    if !result.compile_readiness.owner_silo_disburse_down_ready {
        return;
    }

    // Compile-readiness only: disburse-down CPU results must apply to runtime allocation state.
    use super::owner_silo_disburse_down::apply_owner_silo_runtime_disburse_down_cpu;
    use super::owner_silo_runtime_writeback::{
        apply_owner_silo_runtime_writeback_cpu,
        owner_silo_writeback_inputs_from_planet_child_reduce_up,
        runtime_owner_silo_states_from_scenario,
    };
    use super::planet_child_rf::{
        evaluate_planet_child_rf_reduce_up, PlanetChildRfAdmissionClassification,
    };

    let reduce_up = evaluate_planet_child_rf_reduce_up(spec);
    if reduce_up.classification == PlanetChildRfAdmissionClassification::Rejected {
        return;
    }
    let initial = match runtime_owner_silo_states_from_scenario(spec) {
        Ok(v) => v,
        Err(_) => return,
    };
    let inputs = match owner_silo_writeback_inputs_from_planet_child_reduce_up(&reduce_up) {
        Ok(v) => v,
        Err(_) => return,
    };
    let writeback = match apply_owner_silo_runtime_writeback_cpu(&initial, &inputs) {
        Ok(v) => v,
        Err(_) => return,
    };
    let demands = match owner_silo_demand_buckets_from_planet_child_rf(spec) {
        Ok(v) => v,
        Err(_) => return,
    };
    let disburse = if demands.is_empty() {
        Vec::new()
    } else {
        match apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands) {
            Ok(v) => v,
            Err(_) => return,
        }
    };
    if apply_runtime_local_allocations_from_disburse_down(&disburse).is_err() {
        return;
    }

    result.compile_readiness.runtime_local_allocation_ready = true;
    result.compile_readiness.runtime_local_allocation_deferred = true;
}

fn integrate_runtime_rf_tick(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    result.compile_readiness.runtime_rf_tick_deferred = true;

    if !result.compile_readiness.runtime_local_allocation_ready {
        return;
    }
    if evaluate_runtime_rf_tick(spec).is_err() {
        return;
    }

    result.compile_readiness.runtime_rf_tick_ready = true;
    result.compile_readiness.runtime_rf_tick_deferred = true;
}

fn integrate_runtime_tick_shell(spec: &SimThingScenarioSpec, result: &mut ScenarioIngestionResult) {
    result.compile_readiness.runtime_tick_shell_deferred = true;

    if !result.compile_readiness.runtime_rf_tick_ready {
        return;
    }
    if evaluate_runtime_tick_shell(spec, RuntimeTickId(1)).is_err() {
        return;
    }

    result.compile_readiness.runtime_tick_shell_ready = true;
    result.compile_readiness.runtime_tick_shell_deferred = true;
}

fn integrate_local_participant_effects(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    result.compile_readiness.local_participant_effects_deferred = true;

    if !result.compile_readiness.runtime_tick_shell_ready {
        return;
    }
    if evaluate_local_participant_effects(spec, RuntimeTickId(1)).is_err() {
        return;
    }

    result.compile_readiness.local_participant_effects_ready = true;
    result.compile_readiness.local_participant_effects_deferred = true;
}

fn integrate_runtime_tick_history(
    spec: &SimThingScenarioSpec,
    result: &mut ScenarioIngestionResult,
) {
    result.compile_readiness.runtime_tick_history_deferred = true;

    if !result.compile_readiness.local_participant_effects_ready {
        return;
    }
    if replay_runtime_tick_history(spec, RuntimeTickId(1), 1).is_err() {
        return;
    }

    result.compile_readiness.runtime_tick_history_ready = true;
    result.compile_readiness.runtime_tick_history_deferred = true;
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

    result
        .compile_readiness
        .owner_silo_full_state_mutation_deferred = true;
    if suppress {
        result
            .compile_readiness
            .owner_silo_gpu_participant_accumulation_ready = true;
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
                    | ScenarioDeferralKind::PlanetSimulationDeferred
                    | ScenarioDeferralKind::UnsupportedChildLocationRole
                    | ScenarioDeferralKind::PlanetOwnershipResolutionDeferred
                    | ScenarioDeferralKind::PlanetNonGridChildSimulationDeferred
                    | ScenarioDeferralKind::PlanetNonGridChildUnsupportedKind
            )
        })
        .collect();

    if feature_deferrals.is_empty() {
        result.classification = ScenarioIngestionClassification::Admitted;
        return;
    }

    let admitted_planets = result
        .planet_child_location
        .as_ref()
        .map(|r| r.planet_gridcell_count > 0)
        .unwrap_or(false);

    if !admitted_planets
        && !feature_deferrals.is_empty()
        && feature_deferrals.len() == result.deferrals.len()
        && result.deferrals.iter().all(|d| {
            matches!(
                d.kind,
                ScenarioDeferralKind::PlanetsNotYetAdmitted
                    | ScenarioDeferralKind::UnsupportedChildLocationRole
            )
        })
    {
        result.classification = ScenarioIngestionClassification::Unsupported;
        return;
    }

    if admitted_planets
        && feature_deferrals.iter().all(|d| {
            matches!(
                d.kind,
                ScenarioDeferralKind::PlanetSimulationDeferred
                    | ScenarioDeferralKind::PlanetOwnershipResolutionDeferred
                    | ScenarioDeferralKind::UnsupportedChildLocationDepth
                    | ScenarioDeferralKind::PlanetNonGridChildSimulationDeferred
            )
        })
    {
        result.classification = ScenarioIngestionClassification::PartiallyAdmitted;
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

/// Presentation-safe canonical ingestion profile for Studio admission display.
pub fn studio_canonical_ingestion_profile() -> ScenarioIngestionProfile {
    ScenarioIngestionProfile {
        require_canonical_tree: true,
        admit_legacy_world_root: true,
    }
}

/// Stable display label for ingestion classification (presentation only).
pub fn scenario_ingestion_classification_label(
    classification: ScenarioIngestionClassification,
) -> &'static str {
    match classification {
        ScenarioIngestionClassification::Admitted => "Admitted",
        ScenarioIngestionClassification::PartiallyAdmitted => "PartiallyAdmitted",
        ScenarioIngestionClassification::Rejected => "Rejected",
        ScenarioIngestionClassification::Unsupported => "Unsupported",
    }
}

/// Stable display label for typed ingestion deferrals (presentation only).
pub fn scenario_deferral_kind_label(kind: ScenarioDeferralKind) -> &'static str {
    match kind {
        ScenarioDeferralKind::LegacyWorldRootCompatibility => "LegacyWorldRootCompatibility",
        ScenarioDeferralKind::PlanetsNotYetAdmitted => "PlanetsNotYetAdmitted",
        ScenarioDeferralKind::OwnerResourceFlowNotYetExecuted => "OwnerResourceFlowNotYetExecuted",
        ScenarioDeferralKind::CapabilityTreeNotYetExecuted => "CapabilityTreeNotYetExecuted",
        ScenarioDeferralKind::StudioStructuralPlacementEditNotYetSupported => {
            "StudioStructuralPlacementEditNotYetSupported"
        }
        ScenarioDeferralKind::MappingPlanCompileDeferred => "MappingPlanCompileDeferred",
        ScenarioDeferralKind::GpuResidentExecutionDeferred => "GpuResidentExecutionDeferred",
        ScenarioDeferralKind::UnsupportedGridcellRole => "UnsupportedGridcellRole",
        ScenarioDeferralKind::UnsupportedChildLocationDepth => "UnsupportedChildLocationDepth",
        ScenarioDeferralKind::PlanetSimulationDeferred => "PlanetSimulationDeferred",
        ScenarioDeferralKind::UnsupportedChildLocationRole => "UnsupportedChildLocationRole",
        ScenarioDeferralKind::PlanetOwnershipResolutionDeferred => {
            "PlanetOwnershipResolutionDeferred"
        }
        ScenarioDeferralKind::PlanetNonGridChildSimulationDeferred => {
            "PlanetNonGridChildSimulationDeferred"
        }
        ScenarioDeferralKind::PlanetNonGridChildUnsupportedKind => {
            "PlanetNonGridChildUnsupportedKind"
        }
    }
}
