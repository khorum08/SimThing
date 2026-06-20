//! SCENARIO-STEAD-MAP-ROUNDTRIP-0 — STEAD IDs, links, RF metadata, and spatial tree roundtrip proof.

use simthing_core::SimThingKind;

use crate::error::SpecError;

use super::planet_child_location::planet_id;
use super::scenario::{
    galaxy_map_id, gridcell_role, is_galaxy_map_entity, is_owner_entity_kind, owner_entity_id,
    owner_flow_deficit, owner_flow_demand, owner_flow_owner_ref, owner_flow_resource_key,
    owner_flow_surplus, resolve_map_container, scenario_metadata_string, validate_scenario_links,
    validate_scenario_root_authority, validate_session_owner_entities,
    validate_stead_mapping_consistency, ScenarioRootValidationMode, SimThingScenarioLink,
    SimThingScenarioSpec, GALAXY_GRIDCELL_ROLE_PROPERTY_ID,
};
use super::scenario_canonical_io::{
    load_scenario_spec_from_json_str, prove_scenario_canonical_load_save_roundtrip,
    save_scenario_spec_to_canonical_json,
};
use super::scenario_ingestion::{
    ingest_scenario, studio_canonical_ingestion_profile, ScenarioIngestionClassification,
};
use super::spatial_local_grid::{interior_local_grid_frame_for_gridcell, local_gridcell_role};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScenarioSteadIdRow {
    pub simthing_id_raw: u32,
    pub stable_id: String,
    pub kind: String,
    pub parent_id_raw: Option<u32>,
    pub owner_ref: Option<String>,
    pub is_location: bool,
    pub is_spatial_gridcell: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScenarioSteadLinkRow {
    pub source_id_raw: u32,
    pub target_id_raw: u32,
    pub link_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScenarioSpatialTreeRow {
    pub simthing_id_raw: u32,
    pub parent_id_raw: Option<u32>,
    pub depth: u32,
    pub is_location: bool,
    pub has_interior_grid: bool,
    pub interior_grid_width: Option<u32>,
    pub interior_grid_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScenarioRfMetadataRow {
    pub simthing_id_raw: u32,
    pub owner_ref: Option<String>,
    pub resource_key: Option<String>,
    pub scope_id: Option<String>,
    pub parent_id_raw: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioSteadMapRoundtripReport {
    pub source_label: String,
    pub initial_authority_digest: u64,
    pub canonical_roundtrip_digest: u64,
    pub digest_stable: bool,

    pub stead_id_rows_before: Vec<ScenarioSteadIdRow>,
    pub stead_id_rows_after: Vec<ScenarioSteadIdRow>,
    pub stead_ids_stable: bool,

    pub link_rows_before: Vec<ScenarioSteadLinkRow>,
    pub link_rows_after: Vec<ScenarioSteadLinkRow>,
    pub links_stable: bool,

    pub spatial_tree_rows_before: Vec<ScenarioSpatialTreeRow>,
    pub spatial_tree_rows_after: Vec<ScenarioSpatialTreeRow>,
    pub spatial_tree_stable: bool,

    pub rf_metadata_rows_before: Vec<ScenarioRfMetadataRow>,
    pub rf_metadata_rows_after: Vec<ScenarioRfMetadataRow>,
    pub rf_metadata_stable: bool,

    pub owner_metadata_not_spatial_parentage: bool,
    pub local_rf_parent_node_resolution_prerequisites_present: bool,
    pub studio_projection_rebuild_ready: bool,
}

pub fn evaluate_scenario_stead_map_roundtrip_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioSteadMapRoundtripReport, SpecError> {
    let roundtrip = prove_scenario_canonical_load_save_roundtrip(source_label, json)?;
    let (initial_scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let canonical_save = save_scenario_spec_to_canonical_json(&initial_scenario)?;
    let (roundtrip_scenario, _) =
        load_scenario_spec_from_json_str(source_label, &canonical_save.canonical_json)?;

    let stead_id_rows_before = extract_scenario_stead_id_rows(&initial_scenario)?;
    let stead_id_rows_after = extract_scenario_stead_id_rows(&roundtrip_scenario)?;
    let link_rows_before = extract_scenario_stead_link_rows(&initial_scenario)?;
    let link_rows_after = extract_scenario_stead_link_rows(&roundtrip_scenario)?;
    let spatial_tree_rows_before = extract_scenario_spatial_tree_rows(&initial_scenario)?;
    let spatial_tree_rows_after = extract_scenario_spatial_tree_rows(&roundtrip_scenario)?;
    let rf_metadata_rows_before = extract_scenario_rf_metadata_rows(&initial_scenario)?;
    let rf_metadata_rows_after = extract_scenario_rf_metadata_rows(&roundtrip_scenario)?;

    let stead_ids_stable = stead_id_rows_before == stead_id_rows_after;
    let links_stable = link_rows_before == link_rows_after;
    let spatial_tree_stable = spatial_tree_rows_before == spatial_tree_rows_after;
    let rf_metadata_stable = rf_metadata_rows_before == rf_metadata_rows_after;
    let digest_stable = roundtrip.digest_stable
        && roundtrip.initial_digest == roundtrip.roundtrip_digest
        && roundtrip.scenario_authority_preserved;

    Ok(ScenarioSteadMapRoundtripReport {
        source_label: source_label.to_string(),
        initial_authority_digest: roundtrip.initial_digest,
        canonical_roundtrip_digest: roundtrip.roundtrip_digest,
        digest_stable,
        stead_id_rows_before,
        stead_id_rows_after,
        stead_ids_stable,
        link_rows_before,
        link_rows_after,
        links_stable,
        spatial_tree_rows_before,
        spatial_tree_rows_after,
        spatial_tree_stable,
        rf_metadata_rows_before,
        rf_metadata_rows_after,
        rf_metadata_stable,
        owner_metadata_not_spatial_parentage: prove_owner_metadata_not_spatial_parentage(
            &roundtrip_scenario,
        )?,
        local_rf_parent_node_resolution_prerequisites_present:
            prove_local_rf_parent_node_resolution_prerequisites(&roundtrip_scenario)?,
        studio_projection_rebuild_ready: prove_studio_projection_rebuild_ready(
            &roundtrip_scenario,
        )?,
    })
}

pub fn extract_scenario_stead_id_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<ScenarioSteadIdRow>, SpecError> {
    let mut rows = Vec::new();
    collect_stead_id_rows(&scenario.root, None, &mut rows)?;
    rows.sort();
    Ok(rows)
}

pub fn extract_scenario_stead_link_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<ScenarioSteadLinkRow>, SpecError> {
    let system_to_raw: std::collections::BTreeMap<String, u32> = scenario
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.system_id.to_string(), placement.simthing_id_raw))
        .collect();

    let mut rows = Vec::new();
    for link in &scenario.links {
        rows.push(link_row_from_spec_link(link, &system_to_raw)?);
    }
    rows.sort();
    Ok(rows)
}

pub fn extract_scenario_spatial_tree_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<ScenarioSpatialTreeRow>, SpecError> {
    let mut rows = Vec::new();
    collect_spatial_tree_rows(&scenario.root, None, 0, &mut rows)?;
    rows.sort();
    Ok(rows)
}

pub fn extract_scenario_rf_metadata_rows(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<ScenarioRfMetadataRow>, SpecError> {
    let mut rows = Vec::new();
    collect_rf_metadata_rows(&scenario.root, None, &mut rows)?;
    rows.sort();
    Ok(rows)
}

fn collect_stead_id_rows(
    thing: &simthing_core::SimThing,
    parent_id_raw: Option<u32>,
    rows: &mut Vec<ScenarioSteadIdRow>,
) -> Result<(), SpecError> {
    let stable_id = stable_id_for_simthing(thing)?;
    let owner_ref = owner_flow_owner_ref(thing);
    let is_location = thing.kind == SimThingKind::Location;
    let is_spatial_gridcell = is_location && is_spatial_gridcell_location(thing);

    rows.push(ScenarioSteadIdRow {
        simthing_id_raw: thing.id.raw(),
        stable_id,
        kind: simthing_kind_label(&thing.kind),
        parent_id_raw,
        owner_ref,
        is_location,
        is_spatial_gridcell,
    });

    for child in &thing.children {
        collect_stead_id_rows(child, Some(thing.id.raw()), rows)?;
    }
    Ok(())
}

fn collect_spatial_tree_rows(
    thing: &simthing_core::SimThing,
    parent_id_raw: Option<u32>,
    depth: u32,
    rows: &mut Vec<ScenarioSpatialTreeRow>,
) -> Result<(), SpecError> {
    let is_location = thing.kind == SimThingKind::Location;
    let (has_interior_grid, interior_grid_width, interior_grid_height) =
        if is_location && is_spatial_gridcell_location(thing) {
            let frame = interior_local_grid_frame_for_gridcell(thing)
                .map_err(|_| SpecError::ValidationFailed)?;
            (true, Some(frame.cols), Some(frame.rows))
        } else {
            (false, None, None)
        };

    rows.push(ScenarioSpatialTreeRow {
        simthing_id_raw: thing.id.raw(),
        parent_id_raw,
        depth,
        is_location,
        has_interior_grid,
        interior_grid_width,
        interior_grid_height,
    });

    for child in &thing.children {
        collect_spatial_tree_rows(child, Some(thing.id.raw()), depth.saturating_add(1), rows)?;
    }
    Ok(())
}

fn collect_rf_metadata_rows(
    thing: &simthing_core::SimThing,
    parent_id_raw: Option<u32>,
    rows: &mut Vec<ScenarioRfMetadataRow>,
) -> Result<(), SpecError> {
    if has_rf_channel_metadata(thing) {
        let resource_key = owner_flow_resource_key(thing);
        let resource_key = if resource_key.trim().is_empty() {
            None
        } else {
            Some(resource_key)
        };
        rows.push(ScenarioRfMetadataRow {
            simthing_id_raw: thing.id.raw(),
            owner_ref: owner_flow_owner_ref(thing),
            resource_key,
            scope_id: rf_scope_id_for_simthing(thing, parent_id_raw),
            parent_id_raw,
        });
    }

    for child in &thing.children {
        collect_rf_metadata_rows(child, Some(thing.id.raw()), rows)?;
    }
    Ok(())
}

fn has_rf_channel_metadata(thing: &simthing_core::SimThing) -> bool {
    owner_flow_owner_ref(thing).is_some()
        || owner_flow_surplus(thing).is_some()
        || owner_flow_deficit(thing).is_some()
        || owner_flow_demand(thing).is_some()
        || scenario_metadata_string(thing, super::scenario::OWNER_FLOW_RESOURCE_KEY_PROPERTY_ID)
            .is_some()
}

fn rf_scope_id_for_simthing(
    thing: &simthing_core::SimThing,
    parent_id_raw: Option<u32>,
) -> Option<String> {
    planet_id(thing)
        .or_else(|| local_gridcell_role(thing))
        .map(|role| {
            if let Some(parent) = parent_id_raw {
                format!("{parent}:{role}")
            } else {
                role
            }
        })
}

fn stable_id_for_simthing(thing: &simthing_core::SimThing) -> Result<String, SpecError> {
    if thing.kind == SimThingKind::Scenario {
        return Ok(
            scenario_metadata_string(thing, super::scenario::SCENARIO_ID_PROPERTY_ID)
                .filter(|id| !id.trim().is_empty())
                .unwrap_or_else(|| format!("scenario_raw_{}", thing.id.raw())),
        );
    }
    if is_owner_entity_kind(&thing.kind) {
        return Ok(
            owner_entity_id(thing).unwrap_or_else(|| format!("owner_raw_{}", thing.id.raw()))
        );
    }
    if is_galaxy_map_entity(thing) {
        return Ok(
            galaxy_map_id(thing).unwrap_or_else(|| format!("galaxy_map_raw_{}", thing.id.raw()))
        );
    }
    if let Some(planet) = planet_id(thing) {
        return Ok(planet);
    }
    if let Some(system_id) = super::scenario::gridcell_generated_system_id(thing) {
        return Ok(format!("system_{system_id}"));
    }
    if let Some(role) = gridcell_role(thing) {
        return Ok(format!("gridcell_{}_{}", role, thing.id.raw()));
    }
    Ok(format!(
        "{}_{}",
        simthing_kind_label(&thing.kind),
        thing.id.raw()
    ))
}

fn is_spatial_gridcell_location(thing: &simthing_core::SimThing) -> bool {
    if thing.kind != SimThingKind::Location {
        return false;
    }
    gridcell_role(thing).is_some()
        || thing
            .properties
            .contains_key(&GALAXY_GRIDCELL_ROLE_PROPERTY_ID)
        || super::scenario::gridcell_generated_system_id(thing).is_some()
        || is_galaxy_map_entity(thing)
}

fn simthing_kind_label(kind: &SimThingKind) -> String {
    match kind {
        SimThingKind::Custom(name) => name.clone(),
        other => format!("{other:?}"),
    }
}

fn link_row_from_spec_link(
    link: &SimThingScenarioLink,
    system_to_raw: &std::collections::BTreeMap<String, u32>,
) -> Result<ScenarioSteadLinkRow, SpecError> {
    let source_id_raw = system_to_raw
        .get(&link.from_system_id)
        .copied()
        .ok_or(SpecError::ValidationFailed)?;
    let target_id_raw = system_to_raw
        .get(&link.to_system_id)
        .copied()
        .ok_or(SpecError::ValidationFailed)?;
    Ok(ScenarioSteadLinkRow {
        source_id_raw,
        target_id_raw,
        link_kind: "structural_hyperlane".to_string(),
    })
}

fn prove_owner_metadata_not_spatial_parentage(
    scenario: &SimThingScenarioSpec,
) -> Result<bool, SpecError> {
    validate_session_owner_entities(scenario).map_err(|_| SpecError::ValidationFailed)?;
    let owners_ok = !owner_is_spatial_parent_in_tree(&scenario.root);
    let refs_ok = !owner_ref_implies_spatial_parentage(&scenario.root);
    Ok(owners_ok && refs_ok)
}

fn owner_is_spatial_parent_in_tree(thing: &simthing_core::SimThing) -> bool {
    if is_owner_entity_kind(&thing.kind)
        && thing.children.iter().any(|child| {
            child.kind == SimThingKind::Location
                || child.kind == SimThingKind::World
                || child.kind == SimThingKind::GameSession
        })
    {
        return true;
    }
    thing.children.iter().any(owner_is_spatial_parent_in_tree)
}

fn owner_ref_implies_spatial_parentage(thing: &simthing_core::SimThing) -> bool {
    if let Some(owner_ref) = owner_flow_owner_ref(thing) {
        if thing.children.iter().any(|child| {
            is_owner_entity_kind(&child.kind)
                && owner_entity_id(child).as_deref() == Some(owner_ref.as_str())
        }) {
            return true;
        }
    }
    thing
        .children
        .iter()
        .any(owner_ref_implies_spatial_parentage)
}

fn prove_local_rf_parent_node_resolution_prerequisites(
    scenario: &SimThingScenarioSpec,
) -> Result<bool, SpecError> {
    let map_container = resolve_map_container(scenario).map_err(|_| SpecError::ValidationFailed)?;
    let has_location_tree = map_container.kind == SimThingKind::Location;
    let mut spatial_gridcell_with_interior = false;
    let mut parent_location_arena = false;
    visit_locations(map_container, 0, &mut |location, depth| {
        if depth > 0 {
            parent_location_arena = true;
        }
        if is_spatial_gridcell_location(location) {
            if interior_local_grid_frame_for_gridcell(location).is_ok() {
                spatial_gridcell_with_interior = true;
            }
        }
    });
    let rf_rows = extract_scenario_rf_metadata_rows(scenario)?;
    let has_rf_channel_keys = rf_rows
        .iter()
        .any(|row| row.owner_ref.is_some() || row.resource_key.is_some() || row.scope_id.is_some());
    Ok(has_location_tree
        && spatial_gridcell_with_interior
        && parent_location_arena
        && has_rf_channel_keys)
}

fn visit_locations(
    thing: &simthing_core::SimThing,
    depth: u32,
    f: &mut impl FnMut(&simthing_core::SimThing, u32),
) {
    if thing.kind == SimThingKind::Location {
        f(thing, depth);
        for child in &thing.children {
            if child.kind == SimThingKind::Location {
                visit_locations(child, depth.saturating_add(1), f);
            }
        }
    } else {
        for child in &thing.children {
            visit_locations(child, depth, f);
        }
    }
}

fn prove_studio_projection_rebuild_ready(
    scenario: &SimThingScenarioSpec,
) -> Result<bool, SpecError> {
    validate_scenario_root_authority(scenario, ScenarioRootValidationMode::Canonical)
        .map_err(|_| SpecError::ValidationFailed)?;
    validate_stead_mapping_consistency(scenario).map_err(|_| SpecError::ValidationFailed)?;
    validate_scenario_links(scenario).map_err(|_| SpecError::ValidationFailed)?;
    scenario
        .reserve_loaded_simthing_ids()
        .map_err(|_| SpecError::ValidationFailed)?;

    let ingestion = ingest_scenario(
        "stead_roundtrip_projection",
        scenario,
        studio_canonical_ingestion_profile(),
    );
    let ingestion_ready = matches!(
        ingestion.classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
    ) && ingestion.validation.json_parse_ok;

    let structural_ready = !scenario.structural_grid.placements.is_empty()
        || scenario.structural_grid.map_container_id.trim().is_empty();

    Ok(ingestion_ready && structural_ready)
}
