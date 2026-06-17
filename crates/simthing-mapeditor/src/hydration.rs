//! Studio hydration boundary: generated map output becomes SimThing grid authority before render.

use std::collections::{BTreeSet, HashSet};

use simthing_core::SimThingKind;
use simthing_mapgenerator::{GalaxyGenerationResult, GenerationReport};
use thiserror::Error;

use crate::generation::GenerationRunOutput;

pub const STUDIO_HYDRATION_BOUNDARY_VERSION: &str = "studio.hydration_boundary.v0";
pub const STUDIO_WORLD_ROOT_ID: &str = "studio_world";
pub const STUDIO_MAP_CONTAINER_ID: &str = "studio_galaxy_map";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioHydrationBoundary {
    pub boundary_version: &'static str,
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
    pub fn mentions_hydrated_grid(&self) -> bool {
        self.future_save_authority
            .contains("hydrated Studio SimThing grid")
            || self
                .required_payloads
                .iter()
                .any(|payload| payload.contains("hydrated Studio SimThing grid"))
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
}

pub fn hydrate_generation_into_studio_grid(
    output: &GenerationRunOutput,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    hydrate_generation_result_into_studio_grid(&output.result, &output.report)
}

pub fn hydrate_generation_result_into_studio_grid(
    result: &GalaxyGenerationResult,
    report: &GenerationReport,
) -> Result<StudioHydrationBoundary, StudioHydrationError> {
    if result.placement.systems.is_empty() {
        return Err(StudioHydrationError::EmptyGeneration);
    }

    let mut seen_system_ids = BTreeSet::new();
    let mut seen_coordinates = BTreeSet::new();
    let mut gridcells = Vec::with_capacity(result.placement.systems.len());

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

        gridcells.push(StudioHydratedGridcell {
            simthing_id: format!("studio_gridcell_system_{}", system.id),
            kind: SimThingKind::Location,
            display_name: format!("System {} Gridcell", system.id),
            system_id: system.id,
            structural_col: system.coord.col,
            structural_row: system.coord.row,
            properties: vec![
                StudioHydratedProperty {
                    namespace: "mapgen",
                    name: "generated_system_id",
                    value: system.id.to_string(),
                },
                StudioHydratedProperty {
                    namespace: "stead",
                    name: "structural_col",
                    value: system.coord.col.to_string(),
                },
                StudioHydratedProperty {
                    namespace: "stead",
                    name: "structural_row",
                    value: system.coord.row.to_string(),
                },
            ],
            overlays: Vec::new(),
            children: vec![StudioHydratedChild {
                simthing_id: format!("studio_star_payload_{}", system.id),
                kind: SimThingKind::Cohort,
                display_name: format!("System {} Star Payload", system.id),
                properties: vec![StudioHydratedProperty {
                    namespace: "mapgen",
                    name: "generated_system_id",
                    value: system.id.to_string(),
                }],
                overlays: Vec::new(),
            }],
        });
    }

    let known_ids: HashSet<String> = seen_system_ids.iter().map(u32::to_string).collect();
    let mut hyperlanes = Vec::with_capacity(result.base_hyperlane_edges.len());
    for edge in &result.base_hyperlane_edges {
        if !known_ids.contains(&edge.from) || !known_ids.contains(&edge.to) {
            return Err(StudioHydrationError::HyperlaneEndpointMissing {
                from: edge.from.clone(),
                to: edge.to.clone(),
            });
        }
        hyperlanes.push(StudioHydratedHyperlane {
            from_system_id: edge.from.clone(),
            to_system_id: edge.to.clone(),
        });
    }

    Ok(StudioHydrationBoundary {
        boundary_version: STUDIO_HYDRATION_BOUNDARY_VERSION,
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
            root_kind: SimThingKind::World,
            map_container_id: STUDIO_MAP_CONTAINER_ID.to_string(),
            map_container_kind: SimThingKind::Location,
            grid_width: result.lattice.edge(),
            grid_height: result.lattice.edge(),
            occupied_cells: gridcells.len() as u32,
            gridcells,
            hyperlanes,
        },
    })
}

pub fn future_save_authority_manifest() -> StudioSaveAuthorityManifest {
    StudioSaveAuthorityManifest {
        future_save_authority:
            "future save/load must serialize the hydrated Studio SimThing grid as authority",
        required_payloads: vec![
            "hydrated Studio SimThing grid",
            "generation profile",
            "map generator report summary",
        ],
        presentation_only_payloads: vec![
            "Bevy transforms",
            "render heights",
            "star and hyperlane runtime visibility settings",
        ],
        deferred_features: vec!["save/load UI", "new map flow", "live simulation adoption"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::{run_generation, GenerationProfile};

    fn hydrated_output() -> (GenerationRunOutput, StudioHydrationBoundary) {
        let output = run_generation(&GenerationProfile::default_spiral_2_dense_3000())
            .expect("generate map");
        let hydration = hydrate_generation_into_studio_grid(&output).expect("hydrate");
        (output, hydration)
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
    fn future_save_authority_manifest_mentions_hydrated_grid() {
        let manifest = future_save_authority_manifest();
        assert!(manifest.mentions_hydrated_grid());
        assert!(manifest
            .presentation_only_payloads
            .iter()
            .any(|payload| payload.contains("Bevy")));
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
