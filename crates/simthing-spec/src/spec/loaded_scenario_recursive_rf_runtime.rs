//! LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0 — recursive Accumulator RF runtime surface for loaded ScenarioSpec trees.
//!
//! GPU-residency doctrine: runtime RF aggregation lowers to flat GPU-compatible rows/tables.
//! CPU space is limited to deterministic oracle/reference validation, semantic-side bookkeeping,
//! compile-plan construction, and owner/user-facing reports — not production simulation authority.

use std::collections::BTreeMap;

use simthing_core::{SimThing, SimThingKind};

use crate::error::SpecError;

use super::channel_key::{OwnerRef, ParentLocationId, ResourceKey, ScopeId};
use super::loaded_scenario_studio_session_envelope::evaluate_loaded_scenario_studio_session_envelope_from_json_str;
use super::planet_child_location::{
    is_planet_gridcell, is_surface_gridcell, local_gridcell_role, planet_id,
};
use super::recursive_local_rf::{
    evaluate_recursive_local_rf, prove_recursive_local_rf_preserves_authority,
    LocationRfArenaReport, RecursiveLocalRfError,
};
use super::scenario::is_galaxy_map_entity;
use super::scenario_canonical_io::load_scenario_spec_from_json_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadedScenarioRecursiveRfRuntimeSource {
    LoadedScenarioStudioSessionEnvelope,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRfParticipantRow {
    pub simthing_id_raw: u32,
    pub parent_location_id: ParentLocationId,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub scope_id: Option<ScopeId>,
    pub requested_amount: f64,
    pub available_amount: f64,
    pub is_location: bool,
    pub is_spatial_gridcell: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRfParentArenaRow {
    pub parent_location_id: ParentLocationId,
    pub depth: u32,
    pub child_count: u32,
    pub participant_count: u32,
    pub local_requested_total: f64,
    pub local_available_total: f64,
    pub local_satisfied_total: f64,
    pub local_unmet_total: f64,
    pub local_surplus_total: f64,
    pub net_upward_requested: f64,
    pub net_upward_available: f64,
    pub local_settlement_applied_before_upward_bubbling: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRfChannelRow {
    pub parent_location_id: ParentLocationId,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub scope_id: Option<ScopeId>,
    pub requested_total: f64,
    pub available_total: f64,
    pub satisfied_total: f64,
    pub unmet_total: f64,
    pub surplus_total: f64,
    pub net_upward_delta: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedScenarioRecursiveRfRuntimeReport {
    pub source: LoadedScenarioRecursiveRfRuntimeSource,
    pub source_label: String,
    pub scenario_authority_digest: u64,
    pub loaded_session_envelope_ready: bool,
    pub recursive_rf_runtime_ready: bool,
    pub participant_rows: Vec<LoadedScenarioRfParticipantRow>,
    pub parent_arena_rows: Vec<LoadedScenarioRfParentArenaRow>,
    pub channel_rows: Vec<LoadedScenarioRfChannelRow>,
    pub parent_location_arena_count: u32,
    pub participant_row_count: u32,
    pub channel_row_count: u32,
    pub local_parent_node_resolution_first: bool,
    pub sibling_settlement_before_upward_bubbling: bool,
    pub owner_scope_not_spatial_parentage: bool,
    pub surface_arena_count: u32,
    pub gameplay_rows_parented_to_surface: bool,
    pub surface_to_planet_bubbling_present: bool,
    pub gpu_compatible_row_table_surface: bool,
    pub cpu_oracle_only: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub runtime_mutation_deferred: bool,
    pub semantic_execution_deferred: bool,
    pub savefile_persistence_deferred: bool,
    pub persistent_history_deferred: bool,
    pub studio_ui_wiring_deferred: bool,
    pub gpu_dispatch_deferred: bool,
}

/// Evaluate loaded scenario recursive RF runtime report from canonical JSON, composing #836 session envelope.
pub fn evaluate_loaded_scenario_recursive_rf_runtime_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<LoadedScenarioRecursiveRfRuntimeReport, SpecError> {
    let envelope =
        evaluate_loaded_scenario_studio_session_envelope_from_json_str(source_label, json)?;
    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let rf_report = evaluate_recursive_local_rf(&scenario).map_err(map_recursive_rf_error)?;

    let scope_by_simthing = collect_rf_scope_ids(&scenario.root);
    let flags_by_simthing = collect_simthing_location_flags(&scenario.root);

    let mut participant_rows = Vec::new();
    for arena in &rf_report.arena_reports {
        for row in &arena.participant_rows {
            let (is_location, is_spatial_gridcell) = flags_by_simthing
                .get(&row.source_simthing_id_raw)
                .copied()
                .unwrap_or((false, false));
            participant_rows.push(LoadedScenarioRfParticipantRow {
                simthing_id_raw: row.source_simthing_id_raw,
                parent_location_id: row.parent_location_id,
                owner_ref: Some(row.owner_ref.clone()),
                resource_key: Some(row.resource_key.clone()),
                scope_id: scope_by_simthing
                    .get(&row.source_simthing_id_raw)
                    .cloned()
                    .map(ScopeId::new),
                requested_amount: f64::from(row.demand),
                available_amount: f64::from(row.surplus),
                is_location,
                is_spatial_gridcell,
            });
        }
    }
    sort_participant_rows(&mut participant_rows);
    validate_finite_f64_participants(&participant_rows)?;

    let mut parent_arena_rows = Vec::new();
    let mut channel_rows = Vec::new();
    for arena in &rf_report.arena_reports {
        parent_arena_rows.push(build_parent_arena_row(arena));
        for settlement in &arena.settlements {
            channel_rows.push(LoadedScenarioRfChannelRow {
                parent_location_id: ParentLocationId::new(settlement.arena_location_id_raw),
                owner_ref: Some(settlement.owner_ref.clone()),
                resource_key: Some(settlement.resource_key.clone()),
                scope_id: Some(ScopeId::new(format!(
                    "location/{}",
                    settlement.arena_location_id_raw
                ))),
                requested_total: f64::from(settlement.total_demand),
                available_total: f64::from(settlement.total_surplus),
                satisfied_total: f64::from(settlement.locally_matched_total),
                unmet_total: f64::from(settlement.net_deficit_to_parent),
                surplus_total: f64::from(settlement.net_surplus_to_parent),
                net_upward_delta: f64::from(settlement.net_surplus_to_parent)
                    - f64::from(settlement.net_deficit_to_parent),
            });
        }
    }
    sort_parent_arena_rows(&mut parent_arena_rows);
    sort_channel_rows(&mut channel_rows);
    validate_finite_f64_arenas(&parent_arena_rows)?;
    validate_finite_f64_channels(&channel_rows)?;

    let local_parent_node_resolution_first =
        prove_local_parent_node_resolution_first(&rf_report.arena_reports);
    let sibling_settlement_before_upward_bubbling =
        prove_sibling_settlement_before_upward_bubbling(&rf_report.arena_reports);
    let surface_arena_count = count_surface_arenas(&scenario.root);
    let gameplay_rows_parented_to_surface =
        prove_gameplay_rows_parented_to_surface(&scenario.root, &participant_rows);
    let surface_to_planet_bubbling_present =
        prove_surface_to_planet_bubbling(&rf_report.arena_reports, &scenario.root);

    let loaded_session_envelope_ready = envelope.authority.scenario_import_ready
        && envelope.authority.recursive_rf_prerequisites_ready;
    let recursive_rf_runtime_ready = loaded_session_envelope_ready
        && !participant_rows.is_empty()
        && !parent_arena_rows.is_empty()
        && local_parent_node_resolution_first
        && sibling_settlement_before_upward_bubbling;

    Ok(LoadedScenarioRecursiveRfRuntimeReport {
        source: LoadedScenarioRecursiveRfRuntimeSource::LoadedScenarioStudioSessionEnvelope,
        source_label: source_label.to_string(),
        scenario_authority_digest: envelope.authority.scenario_authority_digest,
        loaded_session_envelope_ready,
        recursive_rf_runtime_ready,
        parent_location_arena_count: parent_arena_rows.len() as u32,
        participant_row_count: participant_rows.len() as u32,
        channel_row_count: channel_rows.len() as u32,
        local_parent_node_resolution_first,
        sibling_settlement_before_upward_bubbling,
        owner_scope_not_spatial_parentage: envelope.authority.owner_metadata_not_spatial_parentage,
        surface_arena_count,
        gameplay_rows_parented_to_surface,
        surface_to_planet_bubbling_present,
        gpu_compatible_row_table_surface: true,
        cpu_oracle_only: true,
        scenario_authority_mutation_deferred: true,
        runtime_mutation_deferred: envelope.runtime_sidecar.runtime_mutation_deferred,
        semantic_execution_deferred: envelope.runtime_sidecar.semantic_execution_deferred,
        savefile_persistence_deferred: envelope.runtime_sidecar.savefile_persistence_deferred,
        persistent_history_deferred: envelope.runtime_sidecar.persistent_history_deferred,
        studio_ui_wiring_deferred: envelope.runtime_sidecar.studio_ui_wiring_deferred,
        gpu_dispatch_deferred: envelope.runtime_sidecar.gpu_dispatch_deferred,
        participant_rows,
        parent_arena_rows,
        channel_rows,
    })
}

/// Prove loaded scenario recursive RF runtime preserves ScenarioSpec authority.
pub fn prove_loaded_scenario_recursive_rf_runtime_preserves_authority(
    source_label: &str,
    json: &str,
) -> Result<bool, SpecError> {
    let report = evaluate_loaded_scenario_recursive_rf_runtime_from_json_str(source_label, json)?;
    let (scenario, _) = load_scenario_spec_from_json_str(source_label, json)?;
    let proof =
        prove_recursive_local_rf_preserves_authority(&scenario).map_err(map_recursive_rf_error)?;

    Ok(proof.scenario_authority_unchanged
        && report.scenario_authority_mutation_deferred
        && report.runtime_mutation_deferred
        && report.semantic_execution_deferred
        && report.savefile_persistence_deferred
        && report.persistent_history_deferred
        && report.studio_ui_wiring_deferred
        && report.gpu_dispatch_deferred
        && report.cpu_oracle_only)
}

fn build_parent_arena_row(arena: &LocationRfArenaReport) -> LoadedScenarioRfParentArenaRow {
    let mut local_requested_total = 0.0;
    let mut local_available_total = 0.0;
    let mut local_satisfied_total = 0.0;
    let mut local_unmet_total = 0.0;
    let mut local_surplus_total = 0.0;
    let mut net_upward_requested = 0.0;
    let mut net_upward_available = 0.0;
    let mut local_settlement_applied_before_upward_bubbling = true;

    for settlement in &arena.settlements {
        local_requested_total += f64::from(settlement.total_demand);
        local_available_total += f64::from(settlement.total_surplus);
        local_satisfied_total += f64::from(settlement.locally_matched_total);
        local_unmet_total += f64::from(settlement.net_deficit_to_parent);
        local_surplus_total += f64::from(settlement.net_surplus_to_parent);
        net_upward_requested += f64::from(settlement.net_deficit_to_parent);
        net_upward_available += f64::from(settlement.net_surplus_to_parent);

        let expected_match = settlement.total_surplus.min(settlement.total_demand);
        if settlement.locally_matched_total != expected_match
            || settlement.net_surplus_to_parent
                != settlement
                    .total_surplus
                    .saturating_sub(settlement.locally_matched_total)
            || settlement.net_deficit_to_parent
                != settlement
                    .total_demand
                    .saturating_sub(settlement.locally_matched_total)
        {
            local_settlement_applied_before_upward_bubbling = false;
        }
    }

    LoadedScenarioRfParentArenaRow {
        parent_location_id: ParentLocationId::new(arena.location_id_raw),
        depth: arena.depth,
        child_count: arena.child_location_count,
        participant_count: arena.participant_count,
        local_requested_total,
        local_available_total,
        local_satisfied_total,
        local_unmet_total,
        local_surplus_total,
        net_upward_requested,
        net_upward_available,
        local_settlement_applied_before_upward_bubbling,
    }
}

fn prove_local_parent_node_resolution_first(arenas: &[LocationRfArenaReport]) -> bool {
    arenas.iter().all(|arena| {
        arena.settlements.iter().all(|settlement| {
            let expected_match = settlement.total_surplus.min(settlement.total_demand);
            settlement.locally_matched_total == expected_match
        })
    })
}

fn prove_sibling_settlement_before_upward_bubbling(arenas: &[LocationRfArenaReport]) -> bool {
    arenas.iter().all(|arena| {
        arena.settlements.iter().all(|settlement| {
            let expected_match = settlement.total_surplus.min(settlement.total_demand);
            settlement.locally_matched_total == expected_match
                && settlement.net_surplus_to_parent
                    == settlement
                        .total_surplus
                        .saturating_sub(settlement.locally_matched_total)
                && settlement.net_deficit_to_parent
                    == settlement
                        .total_demand
                        .saturating_sub(settlement.locally_matched_total)
        })
    })
}

fn collect_rf_scope_ids(root: &SimThing) -> BTreeMap<u32, String> {
    let mut rows = BTreeMap::new();
    collect_rf_scope_ids_recursive(root, None, &mut rows);
    rows
}

fn collect_rf_scope_ids_recursive(
    thing: &SimThing,
    parent_id_raw: Option<u32>,
    rows: &mut BTreeMap<u32, String>,
) {
    if let Some(scope_id) = rf_scope_id_for_simthing(thing, parent_id_raw) {
        rows.insert(thing.id.raw(), scope_id);
    }
    for child in &thing.children {
        collect_rf_scope_ids_recursive(child, Some(thing.id.raw()), rows);
    }
}

fn rf_scope_id_for_simthing(thing: &SimThing, parent_id_raw: Option<u32>) -> Option<String> {
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

fn collect_simthing_location_flags(root: &SimThing) -> BTreeMap<u32, (bool, bool)> {
    let mut rows = BTreeMap::new();
    collect_simthing_location_flags_recursive(root, &mut rows);
    rows
}

fn collect_simthing_location_flags_recursive(
    thing: &SimThing,
    rows: &mut BTreeMap<u32, (bool, bool)>,
) {
    let is_location = thing.kind == SimThingKind::Location;
    let is_spatial_gridcell = is_location
        && (is_planet_gridcell(thing)
            || is_surface_gridcell(thing)
            || is_galaxy_map_entity(thing)
            || local_gridcell_role(thing).is_some());
    rows.insert(thing.id.raw(), (is_location, is_spatial_gridcell));
    for child in &thing.children {
        collect_simthing_location_flags_recursive(child, rows);
    }
}

fn sort_participant_rows(rows: &mut [LoadedScenarioRfParticipantRow]) {
    rows.sort_by(|a, b| {
        (
            a.parent_location_id,
            a.owner_ref.as_ref().map(|o| o.as_str()).unwrap_or(""),
            a.resource_key.as_ref().map(|k| k.as_str()).unwrap_or(""),
            a.simthing_id_raw,
        )
            .cmp(&(
                b.parent_location_id,
                b.owner_ref.as_ref().map(|o| o.as_str()).unwrap_or(""),
                b.resource_key.as_ref().map(|k| k.as_str()).unwrap_or(""),
                b.simthing_id_raw,
            ))
    });
}

fn sort_parent_arena_rows(rows: &mut [LoadedScenarioRfParentArenaRow]) {
    rows.sort_by(|a, b| (a.depth, a.parent_location_id).cmp(&(b.depth, b.parent_location_id)));
}

fn sort_channel_rows(rows: &mut [LoadedScenarioRfChannelRow]) {
    rows.sort_by(|a, b| {
        (
            a.parent_location_id,
            a.owner_ref.as_ref().map(|o| o.as_str()).unwrap_or(""),
            a.resource_key.as_ref().map(|k| k.as_str()).unwrap_or(""),
            a.scope_id.as_ref().map(|s| s.as_str()).unwrap_or(""),
        )
            .cmp(&(
                b.parent_location_id,
                b.owner_ref.as_ref().map(|o| o.as_str()).unwrap_or(""),
                b.resource_key.as_ref().map(|k| k.as_str()).unwrap_or(""),
                b.scope_id.as_ref().map(|s| s.as_str()).unwrap_or(""),
            ))
    });
}

fn validate_finite_f64_participants(
    rows: &[LoadedScenarioRfParticipantRow],
) -> Result<(), SpecError> {
    for row in rows {
        if !row.requested_amount.is_finite() || !row.available_amount.is_finite() {
            return Err(SpecError::ValidationFailed);
        }
    }
    Ok(())
}

fn validate_finite_f64_arenas(rows: &[LoadedScenarioRfParentArenaRow]) -> Result<(), SpecError> {
    for row in rows {
        for value in [
            row.local_requested_total,
            row.local_available_total,
            row.local_satisfied_total,
            row.local_unmet_total,
            row.local_surplus_total,
            row.net_upward_requested,
            row.net_upward_available,
        ] {
            if !value.is_finite() {
                return Err(SpecError::ValidationFailed);
            }
        }
    }
    Ok(())
}

fn validate_finite_f64_channels(rows: &[LoadedScenarioRfChannelRow]) -> Result<(), SpecError> {
    for row in rows {
        for value in [
            row.requested_total,
            row.available_total,
            row.satisfied_total,
            row.unmet_total,
            row.surplus_total,
            row.net_upward_delta,
        ] {
            if !value.is_finite() {
                return Err(SpecError::ValidationFailed);
            }
        }
    }
    Ok(())
}

fn count_surface_arenas(root: &SimThing) -> u32 {
    let mut count = 0u32;
    count_surface_arenas_recursive(root, &mut count);
    count
}

fn count_surface_arenas_recursive(thing: &SimThing, count: &mut u32) {
    if is_surface_gridcell(thing) {
        *count = count.saturating_add(1);
    }
    for child in &thing.children {
        count_surface_arenas_recursive(child, count);
    }
}

fn prove_gameplay_rows_parented_to_surface(
    root: &SimThing,
    participant_rows: &[LoadedScenarioRfParticipantRow],
) -> bool {
    let surface_ids = collect_surface_gridcell_ids(root);
    if surface_ids.is_empty() {
        return false;
    }
    let planet_surface_gameplay = collect_planet_surface_gameplay_parent_ids(root);
    if planet_surface_gameplay.is_empty() {
        return true;
    }
    planet_surface_gameplay
        .values()
        .all(|parent_id| surface_ids.contains_key(parent_id))
        && participant_rows
            .iter()
            .filter(|row| planet_surface_gameplay.contains_key(&row.simthing_id_raw))
            .all(|row| surface_ids.contains_key(&row.parent_location_id.raw()))
}

fn collect_surface_gridcell_ids(root: &SimThing) -> BTreeMap<u32, ()> {
    let mut ids = BTreeMap::new();
    collect_surface_gridcell_ids_recursive(root, &mut ids);
    ids
}

fn collect_surface_gridcell_ids_recursive(thing: &SimThing, ids: &mut BTreeMap<u32, ()>) {
    if is_surface_gridcell(thing) {
        ids.insert(thing.id.raw(), ());
    }
    for child in &thing.children {
        collect_surface_gridcell_ids_recursive(child, ids);
    }
}

fn collect_planet_surface_gameplay_parent_ids(root: &SimThing) -> BTreeMap<u32, u32> {
    let mut rows = BTreeMap::new();
    collect_planet_surface_gameplay_parent_ids_recursive(root, &mut rows);
    rows
}

fn collect_planet_surface_gameplay_parent_ids_recursive(
    thing: &SimThing,
    rows: &mut BTreeMap<u32, u32>,
) {
    if super::planet_child_location::is_surface_gridcell(thing) {
        for child in &thing.children {
            if child.kind != SimThingKind::Location
                && super::planet_child_location::is_admitted_planet_non_grid_child(&child.kind)
            {
                rows.insert(child.id.raw(), thing.id.raw());
            }
        }
    }
    for child in &thing.children {
        collect_planet_surface_gameplay_parent_ids_recursive(child, rows);
    }
}

fn prove_surface_to_planet_bubbling(arenas: &[LocationRfArenaReport], root: &SimThing) -> bool {
    let surface_to_planet: BTreeMap<u32, u32> = collect_surface_to_planet_links(root);
    if surface_to_planet.is_empty() {
        return true;
    }
    surface_to_planet.iter().all(|(surface_id, planet_id)| {
        let surface_has_arena = arenas
            .iter()
            .any(|arena| arena.location_id_raw == *surface_id);
        let planet_has_arena = arenas
            .iter()
            .any(|arena| arena.location_id_raw == *planet_id);
        surface_has_arena && planet_has_arena
    })
}

fn collect_surface_to_planet_links(root: &SimThing) -> BTreeMap<u32, u32> {
    let mut links = BTreeMap::new();
    collect_surface_to_planet_links_recursive(root, &mut links);
    links
}

fn collect_surface_to_planet_links_recursive(thing: &SimThing, links: &mut BTreeMap<u32, u32>) {
    if is_planet_gridcell(thing) {
        if let Some(surface) = super::planet_child_location::planet_surface_gridcell(thing) {
            links.insert(surface.id.raw(), thing.id.raw());
        }
    }
    for child in &thing.children {
        collect_surface_to_planet_links_recursive(child, links);
    }
}

fn map_recursive_rf_error(_error: RecursiveLocalRfError) -> SpecError {
    SpecError::ValidationFailed
}
