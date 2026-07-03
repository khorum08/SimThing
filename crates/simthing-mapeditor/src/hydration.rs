//! Studio projection boundary over generated SimThing-Spec scenario authority.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use simthing_core::{PropertyValue, SimThing, SimThingKind};
use simthing_mapgenerator::{GalaxyGenerationResult, GenerationReport};
use simthing_spec::{
    apply_gridcell_property_edit, apply_star_system_display_name_metadata, resolve_map_container,
    star_system_display_name, structural_property_value_u32, validate_stead_mapping_consistency,
    SimThingScenarioGrid, SimThingScenarioLink, SimThingScenarioProvenance, SimThingScenarioSpec,
    SimThingStructuralGridFrame, SimThingStructuralGridPlacement,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, SIMTHING_SCENARIO_AUTHORITY_LABEL,
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

impl StudioHydrationBoundary {
    pub fn from_scenario(
        scenario: &SimThingScenarioSpec,
        report: &GenerationReport,
    ) -> Result<Self, StudioHydrationError> {
        studio_projection_from_simthing_spec(scenario, report)
    }
}

pub const LOADED_SCENARIO_MAP_QUALITY_STATUS: &str = "LOADED";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioHydrationSourceKind {
    MapGeneratorLibrary,
    LoadedScenarioAuthority,
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

impl StudioRfAccumulatorReadiness {
    pub fn from_scenario(scenario: &SimThingScenarioSpec) -> Result<Self, StudioHydrationError> {
        validate_stead_mapping_consistency(scenario)
            .map_err(|err| StudioHydrationError::SteadMappingInconsistent(err.to_string()))?;
        let participant_count = scenario.gridcell_locations().count() as u64;
        Ok(Self {
            participant_kind: "Location".to_string(),
            grid_width: scenario.structural_grid.frame.width,
            grid_height: scenario.structural_grid.frame.height,
            occupied_cells: scenario.structural_grid.frame.occupied_cells,
            participant_count,
            all_participants_have_structural_placements: participant_count
                == scenario.structural_grid.placements.len() as u64,
            ready_for_spatial_rf_over_locations: participant_count > 0
                && participant_count == scenario.structural_grid.placements.len() as u64,
            deferred_reason: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioHeatmapReadinessKind {
    BoundedTheaterEligible,
    AtlasRequired,
    InvalidSteadMapping,
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

impl StudioHeatmapReadiness {
    pub fn is_ready(&self) -> bool {
        !matches!(
            self.readiness,
            StudioHeatmapReadinessKind::InvalidSteadMapping
        )
    }

    pub fn from_scenario(scenario: &SimThingScenarioSpec) -> Result<Self, StudioHydrationError> {
        validate_stead_mapping_consistency(scenario)
            .map_err(|err| StudioHydrationError::SteadMappingInconsistent(err.to_string()))?;
        Ok(heatmap_readiness_from_valid_scenario(scenario))
    }
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
    #[error("studio scenario authority failed STEAD mapping validation: {0}")]
    SteadMappingInconsistent(String),
    #[error("failed to read Stellaris star-name corpus `{path}`: {message}")]
    StarNameCorpusRead { path: String, message: String },
    #[error("failed to parse Stellaris star-name corpus `{path}`: {message}")]
    StarNameCorpusParse { path: String, message: String },
}

pub fn generate_simthing_spec_scenario(
    output: &GenerationRunOutput,
) -> Result<SimThingScenarioSpec, StudioHydrationError> {
    hydrate_mapgen_result_into_simthing_spec(&output.result, &output.report)
}

pub fn generate_simthing_spec_scenario_with_star_names(
    output: &GenerationRunOutput,
    star_names: &BTreeMap<u32, String>,
) -> Result<SimThingScenarioSpec, StudioHydrationError> {
    hydrate_mapgen_result_into_simthing_spec_with_star_names(
        &output.result,
        &output.report,
        Some(star_names),
    )
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
    hydrate_mapgen_result_into_simthing_spec_with_star_names(result, report, None)
}

pub fn hydrate_mapgen_result_into_simthing_spec_with_star_names(
    result: &GalaxyGenerationResult,
    report: &GenerationReport,
    star_names: Option<&BTreeMap<u32, String>>,
) -> Result<SimThingScenarioSpec, StudioHydrationError> {
    if result.placement.systems.is_empty() {
        return Err(StudioHydrationError::EmptyGeneration);
    }

    let mut seen_system_ids = BTreeSet::new();
    let mut seen_coordinates = BTreeSet::new();
    let mut map_container = SimThing::new(SimThingKind::Location, 0);
    let map_container_raw = map_container.id.raw();
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
        add_u32_property(
            &mut gridcell,
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            system.id,
        );
        add_u32_property(
            &mut gridcell,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            system.coord.col,
        );
        add_u32_property(
            &mut gridcell,
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            system.coord.row,
        );
        if let Some(display_name) = star_names
            .and_then(|names| names.get(&system.id))
            .filter(|name| !name.trim().is_empty())
        {
            apply_star_system_display_name_metadata(&mut gridcell, display_name);
        }
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        add_u32_property(
            &mut payload,
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            system.id,
        );
        gridcell.add_child(payload);
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

    let scenario = SimThingScenarioSpec {
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
            map_container_id: map_container_raw.to_string(),
            placements,
        },
        links,
        provenance: SimThingScenarioProvenance {
            source: "MapGeneratorLibrary".to_string(),
            generator_seed: report.generator.seed,
            generator_shape: report.request.shape.clone(),
            ..SimThingScenarioProvenance::default()
        },
    };
    validate_stead_mapping_consistency(&scenario)
        .map_err(|err| StudioHydrationError::SteadMappingInconsistent(err.to_string()))?;
    Ok(scenario)
}

pub fn studio_projection_from_scenario_authority(
    scenario: &SimThingScenarioSpec,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    let placement_count = scenario.structural_grid.placements.len() as u32;
    studio_projection_with_summary(
        scenario,
        StudioHydrationReportSummary {
            seed: scenario.provenance.generator_seed,
            shape: scenario.provenance.generator_shape.clone(),
            requested_systems: placement_count,
            hydrated_systems: placement_count,
            base_hyperlane_count: scenario.links.len() as u32,
            map_quality_status: LOADED_SCENARIO_MAP_QUALITY_STATUS,
        },
        StudioHydrationSourceKind::LoadedScenarioAuthority,
    )
}

pub fn studio_projection_from_simthing_spec(
    scenario: &SimThingScenarioSpec,
    report: &GenerationReport,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    studio_projection_with_summary(
        scenario,
        StudioHydrationReportSummary {
            seed: report.generator.seed,
            shape: report.request.shape.clone(),
            requested_systems: report.request.star_count,
            hydrated_systems: scenario.structural_grid.placements.len() as u32,
            base_hyperlane_count: report.output.base_hyperlane_count,
            map_quality_status: report.output.map_quality_status,
        },
        StudioHydrationSourceKind::MapGeneratorLibrary,
    )
}

fn studio_projection_with_summary(
    scenario: &SimThingScenarioSpec,
    report_summary: StudioHydrationReportSummary,
    source_kind: StudioHydrationSourceKind,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    validate_stead_mapping_consistency(scenario)
        .map_err(|err| StudioHydrationError::SteadMappingInconsistent(err.to_string()))?;
    let map_container = resolve_map_container(scenario)
        .map_err(|err| StudioHydrationError::SteadMappingInconsistent(err.to_string()))?;
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
            display_name: star_system_display_name(gridcell)
                .unwrap_or_else(|| format!("System {} Gridcell", placement.system_id)),
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
        source_kind,
        report_summary: StudioHydrationReportSummary {
            seed: report_summary.seed,
            shape: report_summary.shape,
            requested_systems: report_summary.requested_systems,
            hydrated_systems: gridcells.len() as u32,
            base_hyperlane_count: report_summary.base_hyperlane_count,
            map_quality_status: report_summary.map_quality_status,
        },
        grid: StudioHydratedGrid {
            root_id: STUDIO_WORLD_ROOT_ID.to_string(),
            root_kind: scenario.root.kind.clone(),
            map_container_id: scenario.structural_grid.map_container_id.clone(),
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
            SIMTHING_SCENARIO_AUTHORITY_LABEL,
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
    StudioRfAccumulatorReadiness::from_scenario(scenario).unwrap_or_else(|err| {
        StudioRfAccumulatorReadiness {
            participant_kind: "Location".to_string(),
            grid_width: scenario.structural_grid.frame.width,
            grid_height: scenario.structural_grid.frame.height,
            occupied_cells: scenario.structural_grid.frame.occupied_cells,
            participant_count: scenario.gridcell_locations().count() as u64,
            all_participants_have_structural_placements: false,
            ready_for_spatial_rf_over_locations: false,
            deferred_reason: Some(err.to_string()),
        }
    })
}

pub fn heatmap_readiness_from_simthing_spec(
    scenario: &SimThingScenarioSpec,
) -> StudioHeatmapReadiness {
    match validate_stead_mapping_consistency(scenario) {
        Ok(()) => heatmap_readiness_from_valid_scenario(scenario),
        Err(err) => {
            let width = scenario.structural_grid.frame.width;
            let height = scenario.structural_grid.frame.height;
            StudioHeatmapReadiness {
                grid_width: width,
                grid_height: height,
                occupied_cells: scenario.structural_grid.frame.occupied_cells,
                placement_count: scenario.structural_grid.placements.len() as u64,
                readiness: StudioHeatmapReadinessKind::InvalidSteadMapping,
                reason: err.to_string(),
            }
        }
    }
}

fn heatmap_readiness_from_valid_scenario(
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
        StudioHeatmapReadinessKind::InvalidSteadMapping => {
            "invalid STEAD mapping must not reach valid-scenario heatmap classification".to_string()
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

fn add_u32_property(thing: &mut SimThing, property_id: simthing_core::SimPropertyId, value: u32) {
    thing.add_property(
        property_id,
        PropertyValue::from_raw_lanes(vec![value as f32]),
    );
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
        let map_raw = map.id.raw();
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        add_u32_property(&mut cell, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, 1);
        add_u32_property(&mut cell, SCENARIO_STRUCTURAL_COL_PROPERTY_ID, 3);
        add_u32_property(&mut cell, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, 2);
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        add_u32_property(&mut payload, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, 1);
        cell.add_child(payload);
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
                map_container_id: map_raw.to_string(),
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
                ..SimThingScenarioProvenance::default()
            },
        }
    }

    fn scenario_with_missing_placement() -> SimThingScenarioSpec {
        let mut scenario = small_simthing_spec_scenario();
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        scenario
    }

    #[test]
    fn simthing_spec_scenario_serializes_and_deserializes() {
        let (_output, scenario, _hydration) = authority_output();
        let json = serde_json::to_string(&scenario).expect("serialize scenario authority");
        let round: SimThingScenarioSpec =
            serde_json::from_str(&json).expect("deserialize scenario authority");
        assert_eq!(round.scenario_id, scenario.scenario_id);
        assert_eq!(round.root.kind, SimThingKind::World);
        assert_eq!(round.structural_grid, scenario.structural_grid);
        assert_eq!(round.links, scenario.links);
    }

    #[test]
    fn simthing_spec_roundtrip_preserves_root_tree() {
        let (_output, scenario, _hydration) = authority_output();
        let json = serde_json::to_string(&scenario).expect("serialize scenario authority");
        let round: SimThingScenarioSpec =
            serde_json::from_str(&json).expect("deserialize scenario authority");
        assert_eq!(round.root.subtree_size(), scenario.root.subtree_size());
        assert_eq!(
            round.root.max_id_in_subtree(),
            scenario.root.max_id_in_subtree()
        );
    }

    #[test]
    fn simthing_spec_roundtrip_preserves_structural_grid() {
        let (_output, scenario, _hydration) = authority_output();
        let json = serde_json::to_string(&scenario).expect("serialize scenario authority");
        let round: SimThingScenarioSpec =
            serde_json::from_str(&json).expect("deserialize scenario authority");
        assert_eq!(round.structural_grid, scenario.structural_grid);
        simthing_spec::validate_stead_mapping_consistency(&round).expect("valid mapping");
    }

    #[test]
    fn simthing_spec_roundtrip_preserves_links() {
        let (_output, scenario, _hydration) = authority_output();
        let json = serde_json::to_string(&scenario).expect("serialize scenario authority");
        let round: SimThingScenarioSpec =
            serde_json::from_str(&json).expect("deserialize scenario authority");
        assert_eq!(round.links, scenario.links);
    }

    #[test]
    fn simthing_spec_roundtrip_preserves_gridcell_children() {
        let (_output, scenario, _hydration) = authority_output();
        let json = serde_json::to_string(&scenario).expect("serialize scenario authority");
        let round: SimThingScenarioSpec =
            serde_json::from_str(&json).expect("deserialize scenario authority");
        assert!(round
            .gridcell_locations()
            .all(|gridcell| !gridcell.children.is_empty()));
    }

    #[test]
    fn loaded_scenario_reserves_existing_simthing_ids() {
        let mut scenario = small_simthing_spec_scenario();
        scenario.root.id = simthing_core::SimThingId::from_session_raw(2_000_000);
        simthing_spec::reserve_simthing_ids_from_scenario(&scenario).expect("reserve");

        let spawned = SimThing::new(SimThingKind::Cohort, 0);

        assert!(spawned.id.raw() > 2_000_000);
    }

    #[test]
    fn new_simthing_after_loaded_scenario_does_not_collide() {
        let scenario = small_simthing_spec_scenario();
        let existing: BTreeSet<u32> = scenario
            .gridcell_locations()
            .map(|gridcell| gridcell.id.raw())
            .collect();
        simthing_spec::reserve_simthing_ids_from_scenario(&scenario).expect("reserve");

        let spawned = SimThing::new(SimThingKind::Location, 0);

        assert!(!existing.contains(&spawned.id.raw()));
    }

    #[test]
    fn stead_mapping_validator_accepts_valid_scenario() {
        let (_output, scenario, _hydration) = authority_output();
        simthing_spec::validate_stead_mapping_consistency(&scenario).expect("valid mapping");
    }

    #[test]
    fn gridcell_structural_properties_match_structural_grid_if_mirrored() {
        let (_output, scenario, _hydration) = authority_output();
        let map = scenario.galaxy_map_container().expect("map");
        for placement in &scenario.structural_grid.placements {
            let gridcell = map
                .children
                .iter()
                .find(|child| child.id.raw() == placement.simthing_id_raw)
                .expect("gridcell");
            assert_eq!(
                gridcell
                    .property(SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
                    .expect("col")
                    .raw_lanes()[0] as u32,
                placement.col
            );
            assert_eq!(
                gridcell
                    .property(SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
                    .expect("row")
                    .raw_lanes()[0] as u32,
                placement.row
            );
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
        let (output, scenario, hydration) = authority_output();
        assert_eq!(hydration.grid.root_id, STUDIO_WORLD_ROOT_ID);
        assert_eq!(hydration.grid.root_kind, SimThingKind::World);
        assert_eq!(
            hydration.grid.map_container_id,
            scenario.structural_grid.map_container_id
        );
        assert_eq!(hydration.grid.map_container_kind, SimThingKind::Location);
        let _ = output;
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
    fn bevy_render_metadata_not_written_to_scenario_authority() {
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
                "scenario authority leaked render key {render_key}"
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
    fn rf_readiness_uses_simthing_spec_scenario() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness =
            StudioRfAccumulatorReadiness::from_scenario(&scenario).expect("rf readiness");

        assert_eq!(
            readiness.participant_count,
            scenario.gridcell_locations().count() as u64
        );
        assert_eq!(
            readiness.occupied_cells,
            scenario.structural_grid.frame.occupied_cells
        );
    }

    #[test]
    fn rf_readiness_participant_count_matches_location_gridcells() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness =
            StudioRfAccumulatorReadiness::from_scenario(&scenario).expect("rf readiness");

        assert_eq!(
            readiness.participant_count,
            scenario.gridcell_locations().count() as u64
        );
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
    fn heatmap_readiness_uses_simthing_spec_scenario() {
        let scenario = small_simthing_spec_scenario();
        let readiness = StudioHeatmapReadiness::from_scenario(&scenario).expect("heatmap");

        assert_eq!(readiness.grid_width, scenario.structural_grid.frame.width);
        assert_eq!(
            readiness.placement_count,
            scenario.structural_grid.placements.len() as u64
        );
    }

    #[test]
    fn heatmap_readiness_uses_structural_frame_not_render_data() {
        let scenario = small_simthing_spec_scenario();
        let readiness = StudioHeatmapReadiness::from_scenario(&scenario).expect("heatmap");
        let debug = format!("{readiness:?}");

        assert_eq!(readiness.grid_width, 8);
        for render_key in ["world_position", "render_height", "camera"] {
            assert!(!debug.contains(render_key));
        }
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
    fn heatmap_readiness_reports_atlas_required_for_oversized_grid() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = StudioHeatmapReadiness::from_scenario(&scenario).expect("heatmap");

        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
    }

    #[test]
    fn heatmap_readiness_does_not_mark_atlas_required_as_layout_failure() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = StudioHeatmapReadiness::from_scenario(&scenario).expect("heatmap");

        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
        assert!(readiness.reason.contains("structural layout is valid"));
        assert!(!readiness.reason.contains("layout failure"));
    }

    #[test]
    fn studio_projection_rebuilds_from_scenario_authority() {
        let (output, scenario, _hydration) = authority_output();
        let rebuilt =
            StudioHydrationBoundary::from_scenario(&scenario, &output.report).expect("projection");

        assert_eq!(rebuilt.simthing_spec_scenario_id, scenario.scenario_id);
        assert_eq!(
            rebuilt.grid.gridcells.len(),
            scenario.structural_grid.placements.len()
        );
    }

    #[test]
    fn model_change_applies_to_scenario_before_projection() {
        let (output, mut scenario, _hydration) = authority_output();
        let (new_col, raw, location_id) = {
            let placement = scenario
                .structural_grid
                .placements
                .get_mut(0)
                .expect("placement");
            placement.col = placement.col.saturating_add(1);
            (
                placement.col,
                placement.simthing_id_raw,
                placement.location_id.clone(),
            )
        };
        let gridcell = scenario
            .root
            .children
            .get_mut(0)
            .expect("map")
            .children
            .iter_mut()
            .find(|child| child.id.raw() == raw)
            .expect("gridcell");
        add_u32_property(gridcell, SCENARIO_STRUCTURAL_COL_PROPERTY_ID, new_col);

        let rebuilt =
            StudioHydrationBoundary::from_scenario(&scenario, &output.report).expect("projection");
        let cell = rebuilt
            .grid
            .gridcells
            .iter()
            .find(|cell| cell.simthing_id == location_id)
            .expect("rebuilt cell");

        assert_eq!(cell.structural_col, new_col);
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

    #[test]
    fn heatmap_readiness_atlas_required_is_not_layout_failure() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
        assert!(readiness.reason.contains("structural layout is valid"));
    }

    #[test]
    fn heatmap_readiness_valid_small_grid_is_bounded_theater_eligible() {
        let scenario = small_simthing_spec_scenario();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::BoundedTheaterEligible
        );
    }

    #[test]
    fn heatmap_readiness_valid_oversized_grid_is_atlas_required() {
        let (_output, scenario, _hydration) = authority_output();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
    }

    #[test]
    fn heatmap_readiness_uses_declared_structural_grid_frame() {
        let scenario = small_simthing_spec_scenario();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(readiness.grid_width, scenario.structural_grid.frame.width);
        assert_eq!(readiness.grid_height, scenario.structural_grid.frame.height);
    }

    #[test]
    fn heatmap_readiness_atlas_required_is_valid_structure() {
        let (_output, scenario, _hydration) = authority_output();
        simthing_spec::validate_stead_mapping_consistency(&scenario).expect("valid STEAD");
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        assert_eq!(
            readiness.readiness,
            StudioHeatmapReadinessKind::AtlasRequired
        );
    }

    #[test]
    fn heatmap_readiness_does_not_use_render_metadata() {
        let scenario = small_simthing_spec_scenario();
        let readiness = heatmap_readiness_from_simthing_spec(&scenario);
        let debug = format!("{readiness:?}");
        for render_key in ["world_position", "render_height", "camera", "opacity"] {
            assert!(!debug.contains(render_key));
        }
    }

    #[test]
    fn rf_readiness_uses_declared_map_container() {
        let scenario = small_simthing_spec_scenario();
        let map = simthing_spec::resolve_map_container(&scenario).expect("map");
        let readiness =
            StudioRfAccumulatorReadiness::from_scenario(&scenario).expect("rf readiness");
        assert_eq!(
            readiness.participant_count,
            map.children
                .iter()
                .filter(|child| child.kind == SimThingKind::Location)
                .count() as u64
        );
    }

    #[test]
    fn projection_rebuild_reflects_model_edit() {
        let (output, mut scenario, _hydration) = authority_output();
        let cell_raw = scenario.structural_grid.placements[0].simthing_id_raw;
        let location_id = scenario.structural_grid.placements[0].location_id.clone();
        let new_col = scenario.structural_grid.placements[0].col.saturating_add(1);
        scenario.structural_grid.placements[0].col = new_col;
        apply_gridcell_property_edit(
            &mut scenario,
            cell_raw,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(new_col),
        )
        .expect("model edit");

        let rebuilt =
            StudioHydrationBoundary::from_scenario(&scenario, &output.report).expect("projection");
        let cell = rebuilt
            .grid
            .gridcells
            .iter()
            .find(|cell| cell.simthing_id == location_id)
            .expect("rebuilt cell");
        assert_eq!(cell.structural_col, new_col);
    }

    #[test]
    fn view_model_does_not_store_authoritative_edit_only() {
        let (output, scenario, _hydration) = authority_output();
        let vm = crate::view_model::StudioGalaxyViewModel::from_scenario(&scenario, &output.report);
        let debug = format!("{vm:?}");
        assert!(!debug.contains("scenario_authority"));
        assert!(!debug.contains("structural_grid"));
    }

    #[test]
    fn model_edit_applies_to_simthing_scenario_authority() {
        let mut scenario = small_simthing_spec_scenario();
        let cell_raw = scenario.structural_grid.placements[0].simthing_id_raw;
        apply_gridcell_property_edit(
            &mut scenario,
            cell_raw,
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(9),
        )
        .expect("edit authority");
        let gridcell = simthing_spec::resolve_map_container(&scenario)
            .expect("map")
            .children
            .iter()
            .find(|child| child.id.raw() == cell_raw)
            .expect("gridcell");
        assert_eq!(
            gridcell
                .property(SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
                .expect("col")
                .raw_lanes()[0] as u32,
            9
        );
    }
}
