//! Studio projection boundary over generated SimThing-Spec scenario authority.

use std::collections::{BTreeSet, HashSet};

use simthing_core::{SimThing, SimThingKind};
use simthing_mapgenerator::{GalaxyGenerationResult, GenerationReport};
use simthing_spec::{
    SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
};
use thiserror::Error;

use crate::generation::GenerationRunOutput;

pub const STUDIO_HYDRATION_BOUNDARY_VERSION: &str = "studio.hydration_boundary.v0";
pub const STUDIO_WORLD_ROOT_ID: &str = "studio_world";
pub const STUDIO_MAP_CONTAINER_ID: &str = "studio_galaxy_map";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydrationBoundary {
    pub boundary_version: &'static str,
    pub simthing_spec_scenario_id: String,
    pub source_kind: StudioHydrationSourceKind,
    pub report_summary: StudioHydrationReportSummary,
    pub grid: StudioHydratedGrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioHydrationSourceKind {
    MapGeneratorLibrary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydrationReportSummary {
    pub seed: u64,
    pub shape: String,
    pub requested_systems: u32,
    pub hydrated_systems: u32,
    pub base_hyperlane_count: u32,
    pub map_quality_status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// StudioHydratedGrid is not save/load authority. It is a Studio index over the
/// SimThing-Spec authority carried by `SimThingScenarioSpec`.
pub struct StudioHydratedGrid {
    pub root_id: String,
    pub root_kind: SimThingKind,
    pub map_container_id: String,
    pub map_container_kind: SimThingKind,
    pub grid_width: u32,
    pub grid_height: u32,
    pub occupied_cells: u32,
    pub gridcells: Vec<StudioHydratedGridcell>,
    pub hyperlanes: Vec<StudioHydratedHyperlane>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydratedGridcell {
    pub simthing_id: String,
    pub kind: SimThingKind,
    pub display_name: String,
    pub system_id: u32,
    pub structural_col: u32,
    pub structural_row: u32,
    pub properties: Vec<StudioHydratedProperty>,
    pub overlays: Vec<StudioHydratedOverlay>,
    pub children: Vec<StudioHydratedChild>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydratedChild {
    pub simthing_id: String,
    pub kind: SimThingKind,
    pub display_name: String,
    pub properties: Vec<StudioHydratedProperty>,
    pub overlays: Vec<StudioHydratedOverlay>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydratedProperty {
    pub namespace: &'static str,
    pub name: &'static str,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydratedOverlay {
    pub namespace: &'static str,
    pub name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydratedHyperlane {
    pub from_system_id: String,
    pub to_system_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioSaveAuthorityManifest {
    pub future_save_authority: &'static str,
    pub required_payloads: Vec<&'static str>,
    pub presentation_only_payloads: Vec<&'static str>,
    pub deferred_features: Vec<&'static str>,
}

impl StudioSaveAuthorityManifest {
    pub fn mentions_simthing_spec_authority(&self) -> bool {
        self.future_save_authority
            .contains("SimThing-Spec-compliant scenario authority")
            || self
                .required_payloads
                .iter()
                .any(|payload| payload.contains("SimThing-Spec-compliant scenario authority"))
    }

    pub fn mentions_studio_projection(&self) -> bool {
        self.required_payloads.iter().any(|payload| {
            payload.contains("Studio projection") || payload.contains("projection/index")
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioRfAccumulatorReadiness {
    pub participant_kind: String,
    pub grid_width: u32,
    pub grid_height: u32,
    pub occupied_cells: u64,
    pub participant_count: u64,
    pub all_participants_have_structural_placements: bool,
    pub ready_for_spatial_rf_over_locations: bool,
    pub deferred_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioHeatmapReadinessKind {
    BoundedTheaterEligible,
    AtlasRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHeatmapReadiness {
    pub grid_width: u32,
    pub grid_height: u32,
    pub occupied_cells: u64,
    pub placement_count: u64,
    pub readiness: StudioHeatmapReadinessKind,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StudioHydrationError {
    #[error("studio hydration rejected generation output with no systems")]
    EmptyGeneration,
    #[error("studio hydration found duplicate generated system id {0}")]
    DuplicateSystemId(u32),
    #[error("studio hydration found duplicate gridcell coordinate ({col},{row})")]
    DuplicateGridcellCoordinate { col: u32, row: u32 },
    #[error("studio hydration found hyperlane endpoint outside the hydrated grid: {from}->{to}")]
    HyperlaneEndpointMissing { from: String, to: String },
    #[error("studio projection could not find SimThing-Spec galaxy map container")]
    MissingMapContainer,
    #[error("studio projection could not find gridcell Location SimThing `{0}`")]
    MissingGridcellLocation(String),
    #[error("studio projection found gridcell `{0}` without child payloads")]
    GridcellMissingChildren(String),
}

pub fn generate_simthing_spec_scenario(
    output: &GenerationRunOutput,
) -> Result<SimThingScenarioSpec, StudioHydrationError> {
    hydrate_mapgen_result_into_simthing_spec(&output.result, &output.report)
}

pub fn hydrate_generation_into_studio_grid(
    output: &GenerationRunOutput,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    let scenario = generate_simthing_spec_scenario(output)?;
    studio_projection_from_simthing_spec(&scenario, &output.report)
}

pub fn hydrate_generation_result_into_studio_grid(
    result: &GalaxyGenerationResult,
    report: &GenerationReport,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    let scenario = hydrate_mapgen_result_into_simthing_spec(result, report)?;
    studio_projection_from_simthing_spec(&scenario, report)
}

pub fn hydrate_mapgen_result_into_simthing_spec(
    result: &GalaxyGenerationResult,
    report: &GenerationReport,
) -> Result<SimThingScenarioSpec, StudioHydrationError> {
    if result.placement.systems.is_empty() {
        return Err(StudioHydrationError::EmptyGeneration);
    }

    let mut seen_system_ids = BTreeSet::new();
    let mut seen_coordinates = BTreeSet::new();
    let mut map_container = SimThing::new(SimThingKind::Location, 0);
    let mut placements = Vec::with_capacity(result.placement.systems.len());

    for system in &result.placement.systems {
        if !seen_system_ids.insert(system.id) {
            return Err(StudioHydrationError::DuplicateSystemId(system.id));
        }
        if !seen_coordinates.insert((system.coord.col, system.coord.row)) {
            return Err(StudioHydrationError::DuplicateGridcellCoordinate {
                col: system.coord.col,
                row: system.coord.row,
            });
        }

        let location_id = format!("studio_gridcell_system_{}", system.id);
        let mut gridcell = SimThing::new(SimThingKind::Location, 0);
        let gridcell_raw = gridcell.id.raw();
        gridcell.add_child(SimThing::new(SimThingKind::Cohort, 0));
        map_container.add_child(gridcell);
        placements.push(SimThingStructuralGridPlacement {
            location_id: location_id.clone(),
            target_id: location_id,
            system_id: system.id,
            row: system.coord.row,
            col: system.coord.col,
            simthing_id_raw: gridcell_raw,
        });
    }

    let known_ids: HashSet<String> = seen_system_ids.iter().map(u32::to_string).collect();
    let mut links = Vec::with_capacity(result.base_hyperlane_edges.len());
    for edge in &result.base_hyperlane_edges {
        if !known_ids.contains(&edge.from) || !known_ids.contains(&edge.to) {
            return Err(StudioHydrationError::HyperlaneEndpointMissing {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }
        links.push(SimThingScenarioLink {
            from_system_id: edge.from.clone(),
            to_system_id: edge.to.clone(),
        });
    }

    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(map_container);

    Ok(SimThingScenarioSpec {
        scenario_id: format!(
            "studio_generated_{}_{}",
            report.request.shape, report.generator.seed
        ),
        root,
        structural_grid: SimThingScenarioGrid {
            frame: SimThingStructuralGridFrame {
                width: result.lattice.edge(),
                height: result.lattice.edge(),
                occupied_cells: placements.len() as u64,
            },
            map_container_id: STUDIO_MAP_CONTAINER_ID.to_string(),
            placements,
        },
        links,
        provenance: SimThingScenarioProvenance {
            source: "MapGeneratorLibrary".to_string(),
            generator_seed: report.generator.seed,
            generator_shape: report.request.shape.clone(),
        },
    })
}

pub fn studio_projection_from_simthing_spec(
    scenario: &SimThingScenarioSpec,
    report: &GenerationReport,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    let map_container = scenario
        .galaxy_map_container()
        .ok_or(StudioHydrationError::MissingMapContainer)?;
    let mut seen_system_ids = BTreeSet::new();
    let mut seen_coordinates = BTreeSet::new();
    let mut gridcells = Vec::with_capacity(scenario.structural_grid.placements.len());

    for placement in &scenario.structural_grid.placements {
        if !seen_system_ids.insert(placement.system_id) {
            return Err(StudioHydrationError::DuplicateSystemId(placement.system_id));
        }
        if !seen_coordinates.insert((placement.col, placement.row)) {
            return Err(StudioHydrationError::DuplicateGridcellCoordinate {
                col: placement.col,
                row: placement.row,
            });
        }
        let gridcell = map_container
            .children
            .iter()
            .find(|child| child.id.raw() == placement.simthing_id_raw)
            .ok_or_else(|| {
                StudioHydrationError::MissingGridcellLocation(placement.location_id.clone())
            })?;
        if gridcell.kind != SimThingKind::Location {
            return Err(StudioHydrationError::MissingGridcellLocation(
                placement.location_id.clone(),
            ));
        }
        if gridcell.children.is_empty() {
            return Err(StudioHydrationError::GridcellMissingChildren(
                placement.location_id.clone(),
            ));
        }

        gridcells.push(StudioHydratedGridcell {
            simthing_id: placement.location_id.clone(),
            kind: gridcell.kind.clone(),
            display_name: format!("System {} Gridcell", placement.system_id),
            system_id: placement.system_id,
            structural_col: placement.col,
            structural_row: placement.row,
            properties: vec![
                StudioHydratedProperty {
                    namespace: "mapgen",
                    name: "generated_system_id",
                    value: placement.system_id.to_string(),
                },
                StudioHydratedProperty {
                    namespace: "stead",
                    name: "structural_col",
                    value: placement.col.to_string(),
                },
                StudioHydratedProperty {
                    namespace: "stead",
                    name: "structural_row",
                    value: placement.row.to_string(),
                },
            ],
            overlays: Vec::new(),
            children: gridcell
                .children
                .iter()
                .enumerate()
                .map(|(index, child)| StudioHydratedChild {
                    simthing_id: format!("{}_child_{}", placement.location_id, index),
                    kind: child.kind.clone(),
                    display_name: format!("System {} Payload {}", placement.system_id, index),
                    properties: vec![StudioHydratedProperty {
                        namespace: "mapgen",
                        name: "generated_system_id",
                        value: placement.system_id.to_string(),
                    }],
                    overlays: Vec::new(),
                })
                .collect(),
        });
    }

    let known_ids: HashSet<String> = seen_system_ids.iter().map(u32::to_string).collect();
    let mut hyperlanes = Vec::with_capacity(scenario.links.len());
    for link in &scenario.links {
        if !known_ids.contains(&link.from_system_id) || !known_ids.contains(&link.to_system_id) {
            return Err(StudioHydrationError::HyperlaneEndpointMissing {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        hyperlanes.push(StudioHydratedHyperlane {
            from_system_id: link.from_system_id.clone(),
            to_system_id: link.to_system_id.clone(),
        });
    }

    Ok(StudioHydrationBoundary {
        boundary_version: STUDIO_HYDRATION_BOUNDARY_VERSION,
        simthing_spec_scenario_id: scenario.scenario_id.clone(),
        source_kind: StudioHydrationSourceKind::MapGeneratorLibrary,
        report_summary: StudioHydrationReportSummary {
            seed: report.generator.seed,
            shape: report.request.shape.clone(),
            requested_systems: report.request.star_count,
            hydrated_systems: gridcells.len() as u32,
            base_hyperlane_count: report.output.base_hyperlane_count,
            map_quality_status: report.output.map_quality_status,
        },
        grid: StudioHydratedGrid {
            root_id: STUDIO_WORLD_ROOT_ID.to_string(),
            root_kind: scenario.root.kind.clone(),
            map_container_id: STUDIO_MAP_CONTAINER_ID.to_string(),
            map_container_kind: map_container.kind.clone(),
            grid_width: scenario.structural_grid.frame.width,
            grid_height: scenario.structural_grid.frame.height,
            occupied_cells: gridcells.len() as u32,
            gridcells,
            hyperlanes,
        },
    })
}

pub fn future_save_authority_manifest() -> StudioSaveAuthorityManifest {
    StudioSaveAuthorityManifest {
        future_save_authority:
            "future save/load must serialize the SimThing-Spec-compliant scenario authority",
        required_payloads: vec![
            "SimThing-Spec-compliant scenario authority",
            "generation profile",
            "map generator report summary",
            "structural grid frame and placements",
            "Studio projection/index rebuilt from authority",
        ],
        presentation_only_payloads: vec![
            "Bevy transforms",
            "render heights",
            "star billboard radii",
            "hyperlane line thickness and opacity",
            "camera-depth buckets",
        ],
        deferred_features: vec!["save/load UI", "new map flow", "live simulation adoption"],
    }
}

pub fn rf_accumulator_readiness_from_simthing_spec(
    scenario: &SimThingScenarioSpec,
) -> StudioRfAccumulatorReadiness {
    let map_container = scenario.galaxy_map_container();
    let placed_raw: HashSet<u32> = scenario
        .structural_grid
        .placements
        .iter()
        .map(|placement| placement.simthing_id_raw)
        .collect();
    let all_participants_have_structural_placements = map_container
        .map(|container| {
            container
                .children
                .iter()
                .filter(|child| child.kind == SimThingKind::Location)
                .all(|child| placed_raw.contains(&child.id.raw()))
        })
        .unwrap_or(false);
    let participant_count = scenario.structural_grid.placements.len() as u64;
    let ready = participant_count > 0 && all_participants_have_structural_placements;
    StudioRfAccumulatorReadiness {
        participant_kind: "Location".to_string(),
        grid_width: scenario.structural_grid.frame.width,
        grid_height: scenario.structural_grid.frame.height,
        occupied_cells: scenario.structural_grid.frame.occupied_cells,
        participant_count,
        all_participants_have_structural_placements,
        ready_for_spatial_rf_over_locations: ready,
        deferred_reason: if ready {
            None
        } else {
            Some("one or more Location participants lack structural placements".to_string())
        },
    }
}

pub fn heatmap_readiness_from_simthing_spec(
    scenario: &SimThingScenarioSpec,
) -> StudioHeatmapReadiness {
    let width = scenario.structural_grid.frame.width;
    let height = scenario.structural_grid.frame.height;
    let max_edge = simthing_spec::REGION_FIELD_STANDARD_MAX_GRID;
    let readiness = if width <= max_edge && height <= max_edge {
        StudioHeatmapReadinessKind::BoundedTheaterEligible
    } else {
        StudioHeatmapReadinessKind::AtlasRequired
    };
    let reason = match readiness {
        StudioHeatmapReadinessKind::BoundedTheaterEligible => {
            "structural placements fit the current bounded dense theater".to_string()
        }
        StudioHeatmapReadinessKind::AtlasRequired => {
            "structural layout is valid; dense Movement-Front execution requires atlas scheduling"
                .to_string()
        }
    };
    StudioHeatmapReadiness {
        grid_width: width,
        grid_height: height,
        occupied_cells: scenario.structural_grid.frame.occupied_cells,
        placement_count: scenario.structural_grid.placements.len() as u64,
        readiness,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::{run_generation, GenerationProfile};

    fn authority_output() -> (
        GenerationRunOutput,
        SimThingScenarioSpec,
        StudioHydrationBoundary,
    ) {
        let output = run_generation(&GenerationProfile::default_spiral_2_dense_3000())
            .expect("generate map");
        let scenario = generate_simthing_spec_scenario(&output).expect("spec authority");
        let hydration = studio_projection_from_simthing_spec(&scenario, &output.report)
            .expect("studio projection");
        (output, scenario, hydration)
    }

    fn hydrated_output() -> (GenerationRunOutput, StudioHydrationBoundary) {
        let (output, _scenario, hydration) = authority_output();
        (output, hydration)
    }

    fn small_simthing_spec_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_child(SimThing::new(SimThingKind::Cohort, 0));
        let cell_raw = cell.id.raw();
        map.add_child(cell);
        root.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "small_spec".to_string(),
            root,
            structural_grid: SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 1,
                },
                map_container_id: STUDIO_MAP_CONTAINER_ID.to_string(),
                placements: vec![SimThingStructuralGridPlacement {
                    location_id: "small_cell".to_string(),
                    target_id: "small_cell".to_string(),
                    system_id: 1,
                    row: 2,
                    col: 3,
                    simthing_id_raw: cell_raw,
                }],
            },
            links: Vec::new(),
            provenance: SimThingScenarioProvenance {
                source: "test".to_string(),
                generator_seed: 1,
                generator_shape: "test".to_string(),
            },
        }
    }

    #[test]
    fn successful_generation_produces_simthing_spec_scenario() {
        let (_output, scenario, _hydration) = authority_output();
        assert_eq!(scenario.provenance.source, "MapGeneratorLibrary");
        assert!(scenario.structural_grid.frame.occupied_cells > 0);
        assert!(!scenario.structural_grid.placements.is_empty());
    }

    #[test]
    fn simthing_spec_scenario_has_world_root() {
        let (_output, scenario, _hydration) = authority_output();
        assert_eq!(scenario.world_root().kind, SimThingKind::World);
    }

    #[test]
    fn world_root_has_galaxy_map_container_child() {
        let (_output, scenario, _hydration) = authority_output();
        let map = scenario
            .galaxy_map_container()
            .expect("map container child");
        assert_eq!(map.kind, SimThingKind::Location);
    }

    #[test]
    fn galaxy_map_container_has_one_location_gridcell_per_generated_system() {
        let (output, scenario, _hydration) = authority_output();
        let gridcell_count = scenario.gridcell_locations().count();
        assert_eq!(gridcell_count, output.result.placement.systems.len());
        assert_eq!(
            scenario.structural_grid.placements.len(),
            output.result.placement.systems.len()
        );
    }

    #[test]
    fn each_generated_system_gridcell_has_structural_col_row() {
        let (output, scenario, _hydration) = authority_output();
        for system in &output.result.placement.systems {
            let placement = scenario
                .structural_grid
                .placements
                .iter()
                .find(|placement| placement.system_id == system.id)
                .expect("placement");
            assert_eq!(placement.col, system.coord.col);
            assert_eq!(placement.row, system.coord.row);
        }
    }

    #[test]
    fn each_generated_system_gridcell_has_children() {
        let (_output, scenario, _hydration) = authority_output();
        assert!(scenario
            .gridcell_locations()
            .all(|gridcell| !gridcell.children.is_empty()));
    }

    #[test]
    fn no_duplicate_gridcell_coordinates_in_simthing_spec() {
        let (_output, scenario, _hydration) = authority_output();
        let coordinates: BTreeSet<(u32, u32)> = scenario
            .structural_grid
            .placements
            .iter()
            .map(|placement| (placement.col, placement.row))
            .collect();
        assert_eq!(coordinates.len(), scenario.structural_grid.placements.len());
    }

    #[test]
    fn no_duplicate_system_ids_in_simthing_spec() {
        let (_output, scenario, _hydration) = authority_output();
        let system_ids: BTreeSet<u32> = scenario
            .structural_grid
            .placements
            .iter()
            .map(|placement| placement.system_id)
            .collect();
        assert_eq!(system_ids.len(), scenario.structural_grid.placements.len());
    }

    #[test]
    fn studio_flat_grid_is_projection_not_authority() {
        let (_output, scenario, hydration) = authority_output();
        assert_eq!(hydration.simthing_spec_scenario_id, scenario.scenario_id);
        assert_ne!(
            std::any::type_name::<StudioHydratedGrid>(),
            std::any::type_name::<SimThingScenarioSpec>()
        );
        assert!(future_save_authority_manifest().mentions_simthing_spec_authority());
    }

    #[test]
    fn successful_generation_produces_hydration_boundary() {
        let (_output, hydration) = hydrated_output();
        assert_eq!(
            hydration.boundary_version,
            STUDIO_HYDRATION_BOUNDARY_VERSION
        );
        assert_eq!(
            hydration.source_kind,
            StudioHydrationSourceKind::MapGeneratorLibrary
        );
        assert!(hydration.grid.occupied_cells > 0);
    }

    #[test]
    fn hydrated_grid_has_world_root_and_map_container() {
        let (_output, hydration) = hydrated_output();
        assert_eq!(hydration.grid.root_id, STUDIO_WORLD_ROOT_ID);
        assert_eq!(hydration.grid.root_kind, SimThingKind::World);
        assert_eq!(hydration.grid.map_container_id, STUDIO_MAP_CONTAINER_ID);
        assert_eq!(hydration.grid.map_container_kind, SimThingKind::Location);
    }

    #[test]
    fn hydrated_grid_cells_are_location_simthings() {
        let (_output, hydration) = hydrated_output();
        assert!(hydration
            .grid
            .gridcells
            .iter()
            .all(|cell| cell.kind == SimThingKind::Location));
    }

    #[test]
    fn every_generated_system_has_one_gridcell_simthing() {
        let (output, hydration) = hydrated_output();
        let hydrated_ids: BTreeSet<u32> = hydration
            .grid
            .gridcells
            .iter()
            .map(|cell| cell.system_id)
            .collect();
        let generated_ids: BTreeSet<u32> = output
            .result
            .placement
            .systems
            .iter()
            .map(|system| system.id)
            .collect();
        assert_eq!(hydrated_ids, generated_ids);
        assert_eq!(
            hydration.grid.gridcells.len(),
            output.result.placement.systems.len()
        );
    }

    #[test]
    fn every_gridcell_has_structural_col_row() {
        let (output, hydration) = hydrated_output();
        for system in &output.result.placement.systems {
            let cell = hydration
                .grid
                .gridcells
                .iter()
                .find(|cell| cell.system_id == system.id)
                .expect("hydrated cell");
            assert_eq!(cell.structural_col, system.coord.col);
            assert_eq!(cell.structural_row, system.coord.row);
        }
    }

    #[test]
    fn every_star_gridcell_has_children() {
        let (_output, hydration) = hydrated_output();
        assert!(hydration
            .grid
            .gridcells
            .iter()
            .all(|cell| !cell.children.is_empty()));
    }

    #[test]
    fn no_duplicate_gridcell_coordinates() {
        let (_output, hydration) = hydrated_output();
        let coordinates: BTreeSet<(u32, u32)> = hydration
            .grid
            .gridcells
            .iter()
            .map(|cell| (cell.structural_col, cell.structural_row))
            .collect();
        assert_eq!(coordinates.len(), hydration.grid.gridcells.len());
    }

    #[test]
    fn no_duplicate_system_ids() {
        let (_output, hydration) = hydrated_output();
        let system_ids: BTreeSet<u32> = hydration
            .grid
            .gridcells
            .iter()
            .map(|cell| cell.system_id)
            .collect();
        assert_eq!(system_ids.len(), hydration.grid.gridcells.len());
    }

    #[test]
    fn bevy_render_metadata_not_written_to_hydration() {
        let (_output, hydration) = hydrated_output();
        let debug = format!("{hydration:?}");
        for render_key in [
            "world_position",
            "render_height",
            "sprite_scale",
            "emissive_strength",
            "depth_bucket",
            "camera",
            "visibility",
        ] {
            assert!(
                !debug.contains(render_key),
                "hydration leaked render key {render_key}"
            );
        }
    }

    #[test]
    fn bevy_render_metadata_not_written_to_simthing_spec() {
        let (_output, scenario, _hydration) = authority_output();
        let debug = format!("{scenario:?}");
        for render_key in [
            "world_position",
            "render_height",
            "sprite_scale",
            "emissive_strength",
            "depth_bucket",
            "camera",
            "visibility",
            "line_thickness",
            "opacity",
        ] {
            assert!(
                !debug.contains(render_key),
                "SimThing-Spec scenario leaked render key {render_key}"
            );
        }
    }

    #[test]
    fn future_save_authority_manifest_mentions_studio_projection() {
        let manifest = future_save_authority_manifest();
        assert!(manifest.mentions_simthing_spec_authority());
        assert!(manifest.mentions_studio_projection());
        assert!(manifest
            .presentation_only_payloads
            .iter()
            .any(|payload| payload.contains("Bevy")));
    }

    #[test]
    fn future_save_manifest_names_simthing_spec_as_authority() {
        let manifest = future_save_authority_manifest();
        assert!(manifest.mentions_simthing_spec_authority());
        assert!(manifest
            .required_payloads
            .contains(&"SimThing-Spec-compliant scenario authority"));
    }

    #[test]
    fn rf_accumulator_readiness_derives_from_simthing_spec_structural_placements() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = rf_accumulator_readiness_from_simthing_spec(&scenario);
        assert_eq!(readiness.participant_kind, "Location");
        assert_eq!(
            readiness.participant_count,
            scenario.structural_grid.placements.len() as u64
        );
        assert!(readiness.all_participants_have_structural_placements);
        assert!(readiness.ready_for_spatial_rf_over_locations);
    }

    #[test]
    fn rf_accumulator_readiness_uses_no_render_metadata() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = rf_accumulator_readiness_from_simthing_spec(&scenario);
        let debug = format!("{readiness:?}");
        for render_key in ["world", "render", "camera", "opacity", "depth"] {
            assert!(
                !debug.contains(render_key),
                "RF readiness leaked render key {render_key}"
            );
        }
    }

    #[test]
    fn heatmap_readiness_derives_from_simthing_spec_structural_frame() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(readiness.grid_width, scenario.structural_grid.frame.width);
        assert_eq!(readiness.grid_height, scenario.structural_grid.frame.height);
        assert_eq!(
            readiness.placement_count,
            scenario.structural_grid.placements.len() as u64
        );
    }

    #[test]
    fn heatmap_readiness_reports_bounded_theater_for_small_grid() {
        let scenario = small_simthing_spec_scenario();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::BoundedTheaterEligible
        );
    }

    #[test]
    fn heatmap_readiness_reports_atlas_required_for_oversized_dense_execution() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
        assert!(readiness.reason.contains("atlas"));
    }

    #[test]
    fn hydration_rejects_duplicate_coordinates() {
        let mut output =
            run_generation(&GenerationProfile::default_spiral_2_dense_3000()).expect("generate");
        output.result.placement.systems[1].coord = output.result.placement.systems[0].coord;
        assert!(matches!(
            hydrate_generation_into_studio_grid(&output),
            Err(StudioHydrationError::DuplicateGridcellCoordinate { .. })
        ));
    }
}
