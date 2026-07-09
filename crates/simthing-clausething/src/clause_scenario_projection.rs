//! Generic ClauseScript pack → ScenarioSpec projection and StructuralRebindReady rebind.
//!
//! Production spine for admitted StructuralRebindReady composition. No scenario-specific defaults.

use std::collections::BTreeMap;

use simthing_core::SimThingKind;
use simthing_spec::{
    game_session_galaxy_map, gridcell_structural_col, gridcell_structural_row, is_galaxy_map_entity,
    structural_property_value_u32, validate_scenario_links, validate_stead_mapping_consistency,
    SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
};

use crate::hydrate_scenario::HydratedScenarioPack;

/// Production projection mode (admitted public mode for Studio composition).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClauseScenarioProjectionMode {
    StructuralRebindReady,
}

impl ClauseScenarioProjectionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StructuralRebindReady => "StructuralRebindReady",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClauseScenarioProjectionReport {
    pub projection_mode: &'static str,
    pub map_container_id: String,
    pub placement_count: usize,
    pub link_count: usize,
    pub links_residue: Option<String>,
    pub stead_validation: String,
}

#[derive(Debug, Clone)]
pub struct ClauseScenarioProjectionError {
    pub message: String,
}

impl std::fmt::Display for ClauseScenarioProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "clause scenario projection error: {}", self.message)
    }
}

impl std::error::Error for ClauseScenarioProjectionError {}

impl ClauseScenarioProjectionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Project hydrated pack to authority-tree candidate Spec (empty STEAD grid fields).
pub fn project_pack_to_authority_tree_candidate(
    pack: &HydratedScenarioPack,
) -> Result<SimThingScenarioSpec, ClauseScenarioProjectionError> {
    let authority_root = pack.authority_root.clone().ok_or_else(|| {
        ClauseScenarioProjectionError::new(
            "hydrated pack is missing authority_root; cannot project to SimThingScenarioSpec",
        )
    })?;

    let (frame, provenance) = if let Some(embedded) = pack.embedded_static_galaxy_scenarios.first()
    {
        (
            embedded.source_structural_grid.frame,
            SimThingScenarioProvenance {
                source: embedded.provenance.source.clone(),
                generator_seed: embedded.provenance.generator_seed,
                generator_shape: embedded.provenance.generator_shape.clone(),
                generator_profile_id: embedded.provenance.generator_profile_id.clone(),
                generator_params_json: embedded.provenance.generator_params_json.clone(),
                name_corpus_source: embedded.provenance.name_corpus_source.clone(),
                name_assignment_mode: embedded.provenance.name_assignment_mode.clone(),
            },
        )
    } else {
        (
            SimThingStructuralGridFrame {
                width: 0,
                height: 0,
                occupied_cells: 0,
            },
            SimThingScenarioProvenance {
                source: format!("clause:{}", pack.scenario_id),
                ..SimThingScenarioProvenance::default()
            },
        )
    };

    Ok(SimThingScenarioSpec {
        scenario_id: pack.scenario_id.clone(),
        root: authority_root,
        structural_grid: SimThingScenarioGrid {
            frame,
            map_container_id: String::new(),
            placements: Vec::new(),
        },
        links: Vec::new(),
        provenance,
    })
}

/// Rebind authority-tree candidate Spec to StructuralRebindReady using pack embed lattice.
pub fn rebind_pack_to_structural_rebind_ready(
    pack: &HydratedScenarioPack,
) -> Result<(SimThingScenarioSpec, ClauseScenarioProjectionReport), ClauseScenarioProjectionError>
{
    let candidate = project_pack_to_authority_tree_candidate(pack)?;
    rebind_authority_tree_candidate(&candidate, pack)
}

/// Convert authority-tree candidate Spec into StructuralRebindReady.
pub fn rebind_authority_tree_candidate(
    candidate: &SimThingScenarioSpec,
    pack: &HydratedScenarioPack,
) -> Result<(SimThingScenarioSpec, ClauseScenarioProjectionReport), ClauseScenarioProjectionError>
{
    let mut scenario = candidate.clone();

    let galaxy_map = game_session_galaxy_map(&scenario).map_err(|e| {
        ClauseScenarioProjectionError::new(format!("missing GameSession GalaxyMap: {e}"))
    })?;
    if !is_galaxy_map_entity(galaxy_map) {
        return Err(ClauseScenarioProjectionError::new(
            "spatial GalaxyMap entity is not recognized as galaxy map",
        ));
    }

    let map_container_id = galaxy_map.id.raw().to_string();
    let galaxy_map_raw = galaxy_map.id.raw();

    let embedded = pack.embedded_static_galaxy_scenarios.first().ok_or_else(|| {
        ClauseScenarioProjectionError::new(
            "rebind requires embedded_static_galaxy_scenarios[0] for lattice join",
        )
    })?;

    let mut by_coord: BTreeMap<(u32, u32), &simthing_spec::SimThingStructuralGridPlacement> =
        BTreeMap::new();
    for placement in &embedded.source_structural_grid.placements {
        by_coord.insert((placement.row, placement.col), placement);
    }
    let mut system_id_by_namespaced_target: BTreeMap<String, u32> = BTreeMap::new();
    for np in &embedded.namespaced_placements {
        if let Some(source) = by_coord.get(&(np.row, np.col)) {
            system_id_by_namespaced_target.insert(np.target_id.clone(), source.system_id);
        }
    }

    let mut placements = Vec::new();
    {
        let game_session = scenario
            .root
            .children
            .iter_mut()
            .find(|c| c.kind == SimThingKind::GameSession)
            .ok_or_else(|| {
                ClauseScenarioProjectionError::new("authority root missing GameSession")
            })?;
        let galaxy = game_session
            .children
            .iter_mut()
            .find(|c| c.id.raw() == galaxy_map_raw)
            .ok_or_else(|| {
                ClauseScenarioProjectionError::new(
                    "could not locate GalaxyMap by raw id for mutation",
                )
            })?;

        for child in galaxy.children.iter_mut() {
            if child.kind != SimThingKind::Location || is_galaxy_map_entity(child) {
                continue;
            }
            let row = gridcell_structural_row(child).ok_or_else(|| {
                ClauseScenarioProjectionError::new(format!(
                    "authority gridcell raw {} missing structural_row",
                    child.id.raw()
                ))
            })?;
            let col = gridcell_structural_col(child).ok_or_else(|| {
                ClauseScenarioProjectionError::new(format!(
                    "authority gridcell raw {} missing structural_col",
                    child.id.raw()
                ))
            })?;
            let source = by_coord.get(&(row, col)).copied().ok_or_else(|| {
                ClauseScenarioProjectionError::new(format!(
                    "no embedded placement for authority gridcell raw {} at row={row} col={col}",
                    child.id.raw()
                ))
            })?;

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
        return Err(ClauseScenarioProjectionError::new(
            "rebind produced zero placements under GalaxyMap",
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
        ClauseScenarioProjectionError::new(format!(
            "validate_stead_mapping_consistency failed after rebind: {e}"
        ))
    })?;
    if link_count > 0 {
        validate_scenario_links(&scenario).map_err(|e| {
            ClauseScenarioProjectionError::new(format!(
                "validate_scenario_links failed after rebind: {e}"
            ))
        })?;
    }

    Ok((
        scenario,
        ClauseScenarioProjectionReport {
            projection_mode: ClauseScenarioProjectionMode::StructuralRebindReady.as_str(),
            map_container_id,
            placement_count,
            link_count,
            links_residue,
            stead_validation: "PASS".to_string(),
        },
    ))
}
