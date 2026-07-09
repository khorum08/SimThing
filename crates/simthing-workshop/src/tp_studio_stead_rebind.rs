//! Workshop-homed StructuralRebindReady candidate for TP Studio wiring.
//!
//! TP-STUDIO-STEAD-REBIND-0 — converts AuthorityTreeCandidate Spec (empty STEAD grid)
//! into StructuralRebindReady by binding map_container_id / placements / links to
//! existing authority-tree node ids. Not a production mapeditor API.

use std::collections::BTreeMap;

use simthing_clausething::HydratedScenarioPack;
use simthing_core::SimThingKind;
use simthing_spec::{
    game_session_galaxy_map, gridcell_structural_col, gridcell_structural_row, is_galaxy_map_entity,
    structural_property_value_u32, validate_scenario_links, validate_stead_mapping_consistency,
    SimThingScenarioLink, SimThingScenarioSpec, SimThingStructuralGridPlacement,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
};
use thiserror::Error;

use crate::tp_studio_clause_ingest::{
    project_tp_pack_to_scenario_spec, TpStudioClauseIngestError,
};

/// Projection mode labels (readiness report contract).
pub const PROJECTION_MODE_AUTHORITY_TREE_CANDIDATE: &str = "AuthorityTreeCandidate";
pub const PROJECTION_MODE_STRUCTURAL_REBIND_READY: &str = "StructuralRebindReady";

#[derive(Debug, Error)]
pub enum TpStudioSteadRebindError {
    #[error("TP STEAD rebind error: {0}")]
    Message(String),
    #[error("TP STEAD rebind ingest error: {0}")]
    Ingest(#[from] TpStudioClauseIngestError),
}

#[derive(Debug, Clone)]
pub struct TpStudioSteadRebindReport {
    pub projection_mode: &'static str,
    pub map_container_id: String,
    pub placement_count: usize,
    pub link_count: usize,
    pub links_residue: Option<String>,
    pub stead_validation: String,
}

#[derive(Debug, Clone)]
pub struct TpStudioSteadRebindResult {
    pub scenario: SimThingScenarioSpec,
    pub report: TpStudioSteadRebindReport,
}

/// Project pack → AuthorityTreeCandidate then rebind to StructuralRebindReady.
pub fn rebind_pack_to_structural_rebind_ready(
    pack: &HydratedScenarioPack,
) -> Result<TpStudioSteadRebindResult, TpStudioSteadRebindError> {
    let candidate = project_tp_pack_to_scenario_spec(pack)?;
    rebind_authority_tree_candidate(&candidate, pack)
}

/// Convert an AuthorityTreeCandidate Spec into StructuralRebindReady using pack embed lattice.
///
/// Policy (from readiness report):
/// - `map_container_id` = authority GalaxyMap raw id
/// - placements enumerate GalaxyMap star-system Location children with authority `simthing_id_raw`
/// - system_id / location / target ids joined from embedded source placements by (row, col)
/// - links copied from embedded source links when system_ids align; else residue recorded
/// - stamps `SCENARIO_GENERATED_SYSTEM_ID` on authority gridcells (required by STEAD validate)
pub fn rebind_authority_tree_candidate(
    candidate: &SimThingScenarioSpec,
    pack: &HydratedScenarioPack,
) -> Result<TpStudioSteadRebindResult, TpStudioSteadRebindError> {
    let mut scenario = candidate.clone();

    let galaxy_map = game_session_galaxy_map(&scenario).map_err(|e| {
        TpStudioSteadRebindError::Message(format!("missing GameSession GalaxyMap: {e}"))
    })?;
    if !is_galaxy_map_entity(galaxy_map) {
        return Err(TpStudioSteadRebindError::Message(
            "spatial GalaxyMap entity is not recognized as galaxy map".into(),
        ));
    }

    let map_container_id = galaxy_map.id.raw().to_string();
    let galaxy_map_raw = galaxy_map.id.raw();

    let embedded = pack.embedded_static_galaxy_scenarios.first().ok_or_else(|| {
        TpStudioSteadRebindError::Message(
            "rebind requires embedded_static_galaxy_scenarios[0] for lattice join".into(),
        )
    })?;

    // Join key: structural (row, col) → embedded source placement (system_id + location/target).
    let mut by_coord: BTreeMap<(u32, u32), &simthing_spec::SimThingStructuralGridPlacement> =
        BTreeMap::new();
    for placement in &embedded.source_structural_grid.placements {
        by_coord.insert((placement.row, placement.col), placement);
    }
    // namespaced target_id → system_id for link rebuild (pack stores namespaced link endpoints).
    let mut system_id_by_namespaced_target: BTreeMap<String, u32> = BTreeMap::new();
    for np in &embedded.namespaced_placements {
        if let Some(source) = by_coord.get(&(np.row, np.col)) {
            system_id_by_namespaced_target.insert(np.target_id.clone(), source.system_id);
        }
    }

    // Mutate authority tree: stamp generated_system_id on each star-system gridcell.
    // Walk via mutable galaxy map children under Scenario → GameSession → GalaxyMap.
    let mut placements = Vec::new();
    {
        let game_session = scenario
            .root
            .children
            .iter_mut()
            .find(|c| c.kind == SimThingKind::GameSession)
            .ok_or_else(|| {
                TpStudioSteadRebindError::Message("authority root missing GameSession".into())
            })?;
        let galaxy = game_session
            .children
            .iter_mut()
            .find(|c| c.id.raw() == galaxy_map_raw)
            .ok_or_else(|| {
                TpStudioSteadRebindError::Message(
                    "could not locate GalaxyMap by raw id for mutation".into(),
                )
            })?;

        for child in galaxy.children.iter_mut() {
            if child.kind != SimThingKind::Location || is_galaxy_map_entity(child) {
                continue;
            }
            let row = gridcell_structural_row(child).ok_or_else(|| {
                TpStudioSteadRebindError::Message(format!(
                    "authority gridcell raw {} missing structural_row",
                    child.id.raw()
                ))
            })?;
            let col = gridcell_structural_col(child).ok_or_else(|| {
                TpStudioSteadRebindError::Message(format!(
                    "authority gridcell raw {} missing structural_col",
                    child.id.raw()
                ))
            })?;
            let source = by_coord.get(&(row, col)).copied().ok_or_else(|| {
                TpStudioSteadRebindError::Message(format!(
                    "no embedded placement for authority gridcell raw {} at row={row} col={col}",
                    child.id.raw()
                ))
            })?;

            // STEAD requires generated_system_id property on the authority node.
            child.add_property(
                SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
                structural_property_value_u32(source.system_id),
            );

            placements.push(SimThingStructuralGridPlacement {
                location_id: source.location_id.clone(),
                target_id: source.target_id.clone(),
                system_id: source.system_id,
                row,
                col,
                simthing_id_raw: child.id.raw(),
            });
        }
    }

    if placements.is_empty() {
        return Err(TpStudioSteadRebindError::Message(
            "rebind produced zero placements under GalaxyMap".into(),
        ));
    }

    placements.sort_by_key(|p| (p.row, p.col, p.system_id));

    let mut links = Vec::new();
    let mut links_residue = None;
    if embedded.namespaced_links.is_empty() {
        links_residue = Some(
            "embedded namespaced_links empty — Spec links remain empty".to_string(),
        );
    } else {
        let mut dropped = 0usize;
        for link in &embedded.namespaced_links {
            let Some(&from_sys) = system_id_by_namespaced_target.get(&link.from) else {
                dropped += 1;
                continue;
            };
            let Some(&to_sys) = system_id_by_namespaced_target.get(&link.to) else {
                dropped += 1;
                continue;
            };
            links.push(SimThingScenarioLink {
                from_system_id: from_sys.to_string(),
                to_system_id: to_sys.to_string(),
            });
        }
        if dropped > 0 {
            links_residue = Some(format!(
                "dropped {dropped} namespaced links with endpoints outside rebind join"
            ));
        }
        if links.is_empty() {
            links_residue = Some(
                "namespaced links present but none mapped to system_id endpoints".to_string(),
            );
        }
    }

    let placement_count = placements.len();
    let link_count = links.len();
    scenario.structural_grid.map_container_id = map_container_id.clone();
    scenario.structural_grid.placements = placements;
    scenario.structural_grid.frame.occupied_cells = placement_count as u64;
    scenario.links = links;

    validate_stead_mapping_consistency(&scenario).map_err(|e| {
        TpStudioSteadRebindError::Message(format!(
            "validate_stead_mapping_consistency failed after rebind: {e}"
        ))
    })?;
    // Links validation is part of full authority admission; run when links non-empty.
    if link_count > 0 {
        validate_scenario_links(&scenario).map_err(|e| {
            TpStudioSteadRebindError::Message(format!(
                "validate_scenario_links failed after rebind: {e}"
            ))
        })?;
    }

    Ok(TpStudioSteadRebindResult {
        scenario,
        report: TpStudioSteadRebindReport {
            projection_mode: PROJECTION_MODE_STRUCTURAL_REBIND_READY,
            map_container_id,
            placement_count,
            link_count,
            links_residue,
            stead_validation: "PASS".to_string(),
        },
    })
}
