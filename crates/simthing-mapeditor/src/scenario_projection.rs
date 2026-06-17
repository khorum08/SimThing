//! Scenario-derived structural projection and GPU-resident readiness manifests.
//!
//! These types are projections/caches over `SimThingScenarioSpec` — not model authority and not
//! GPU buffers. They provide deterministic dense indices for future GPU upload planning.

use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, ScenarioLinkError,
    SimThingScenarioSpec,
};

use crate::hydration::{
    heatmap_readiness_from_simthing_spec, rf_accumulator_readiness_from_simthing_spec,
    StudioHeatmapReadinessKind, StudioHydrationError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioLocationIndexRow {
    pub dense_index: u32,
    pub simthing_id_raw: u32,
    pub system_id: u32,
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioLinkIndexRow {
    pub from_dense_index: u32,
    pub to_dense_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioStructuralProjection {
    pub location_indices: Vec<StudioLocationIndexRow>,
    pub link_indices: Vec<StudioLinkIndexRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGpuResidencyReadiness {
    pub grid_width: u32,
    pub grid_height: u32,
    pub occupied_cells: u64,
    pub location_count: u64,
    pub link_count: u64,
    pub dense_location_index_ready: bool,
    pub structural_placements_ready: bool,
    pub rf_accumulator_ready: bool,
    pub heatmap_ready: StudioHeatmapReadinessKind,
    pub atlas_required: bool,
    pub deferred_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioStructuralProjectionError {
    InvalidLinkEndpoint { from: String, to: String },
    SteadMapping(String),
}

impl From<StudioStructuralProjectionError> for StudioHydrationError {
    fn from(err: StudioStructuralProjectionError) -> Self {
        match err {
            StudioStructuralProjectionError::InvalidLinkEndpoint { from, to } => {
                StudioHydrationError::HyperlaneEndpointMissing { from, to }
            }
            StudioStructuralProjectionError::SteadMapping(message) => {
                StudioHydrationError::SteadMappingInconsistent(message)
            }
        }
    }
}

pub fn build_structural_projection(
    scenario: &SimThingScenarioSpec,
) -> Result<StudioStructuralProjection, StudioStructuralProjectionError> {
    validate_stead_mapping_consistency(scenario)
        .map_err(|err| StudioStructuralProjectionError::SteadMapping(err.to_string()))?;
    validate_scenario_links(scenario).map_err(|err| match err {
        ScenarioLinkError::InvalidEndpoint { from, to } => {
            StudioStructuralProjectionError::InvalidLinkEndpoint { from, to }
        }
    })?;

    let mut placements: Vec<_> = scenario.structural_grid.placements.iter().collect();
    placements.sort_by(|left, right| {
        left.row
            .cmp(&right.row)
            .then_with(|| left.col.cmp(&right.col))
            .then_with(|| left.system_id.cmp(&right.system_id))
            .then_with(|| left.simthing_id_raw.cmp(&right.simthing_id_raw))
    });

    let location_indices: Vec<StudioLocationIndexRow> = placements
        .iter()
        .enumerate()
        .map(|(dense_index, placement)| StudioLocationIndexRow {
            dense_index: dense_index as u32,
            simthing_id_raw: placement.simthing_id_raw,
            system_id: placement.system_id,
            row: placement.row,
            col: placement.col,
        })
        .collect();

    let system_to_dense: std::collections::BTreeMap<String, u32> = location_indices
        .iter()
        .map(|row| (row.system_id.to_string(), row.dense_index))
        .collect();

    let mut link_indices = Vec::with_capacity(scenario.links.len());
    for link in &scenario.links {
        let Some(from_dense_index) = system_to_dense.get(&link.from_system_id) else {
            return Err(StudioStructuralProjectionError::InvalidLinkEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        };
        let Some(to_dense_index) = system_to_dense.get(&link.to_system_id) else {
            return Err(StudioStructuralProjectionError::InvalidLinkEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        };
        link_indices.push(StudioLinkIndexRow {
            from_dense_index: *from_dense_index,
            to_dense_index: *to_dense_index,
        });
    }

    Ok(StudioStructuralProjection {
        location_indices,
        link_indices,
    })
}

pub fn build_gpu_residency_readiness(
    scenario: &SimThingScenarioSpec,
    projection: &StudioStructuralProjection,
) -> StudioGpuResidencyReadiness {
    let frame = &scenario.structural_grid.frame;
    let stead_valid = validate_stead_mapping_consistency(scenario).is_ok();
    let rf = rf_accumulator_readiness_from_simthing_spec(scenario);
    let heatmap = heatmap_readiness_from_simthing_spec(scenario);
    let placements_ready = stead_valid
        && projection.location_indices.len() == scenario.structural_grid.placements.len();
    let dense_ready = placements_ready && !projection.location_indices.is_empty();
    let atlas_required = heatmap.readiness == StudioHeatmapReadinessKind::AtlasRequired;
    let deferred_reason = if !stead_valid {
        Some("invalid STEAD mapping".to_string())
    } else if !dense_ready {
        Some("structural dense index projection incomplete".to_string())
    } else if atlas_required {
        Some("dense Movement-Front execution requires atlas scheduling".to_string())
    } else {
        None
    };

    StudioGpuResidencyReadiness {
        grid_width: frame.width,
        grid_height: frame.height,
        occupied_cells: frame.occupied_cells,
        location_count: projection.location_indices.len() as u64,
        link_count: projection.link_indices.len() as u64,
        dense_location_index_ready: dense_ready,
        structural_placements_ready: placements_ready,
        rf_accumulator_ready: rf.ready_for_spatial_rf_over_locations,
        heatmap_ready: heatmap.readiness,
        atlas_required,
        deferred_reason,
    }
}

pub fn build_gpu_residency_readiness_from_scenario(
    scenario: &SimThingScenarioSpec,
) -> Result<StudioGpuResidencyReadiness, StudioStructuralProjectionError> {
    let projection = build_structural_projection(scenario)?;
    Ok(build_gpu_residency_readiness(scenario, &projection))
}

#[cfg(test)]
mod tests {
    use simthing_core::{SimThing, SimThingKind};
    use simthing_spec::{
        structural_property_value_u32, SimThingScenarioGrid, SimThingScenarioLink,
        SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
        SimThingStructuralGridPlacement, SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
        SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
    };

    use super::*;

    fn small_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let map_raw = map.id.raw();
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(3),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(2),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(1),
        );
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
            links: vec![SimThingScenarioLink {
                from_system_id: "1".to_string(),
                to_system_id: "1".to_string(),
            }],
            provenance: SimThingScenarioProvenance::default(),
        }
    }

    #[test]
    fn structural_projection_derives_from_scenario_authority() {
        let scenario = small_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert_eq!(projection.location_indices.len(), 1);
        assert_eq!(projection.link_indices.len(), 1);
    }

    #[test]
    fn structural_projection_has_deterministic_dense_indices() {
        let scenario = small_scenario();
        let first = build_structural_projection(&scenario).expect("first");
        let second = build_structural_projection(&scenario).expect("second");
        assert_eq!(first, second);
        assert_eq!(first.location_indices[0].dense_index, 0);
    }

    #[test]
    fn structural_projection_uses_structural_coords_not_render_coords() {
        let scenario = small_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let row = &projection.location_indices[0];
        assert_eq!(row.col, 3);
        assert_eq!(row.row, 2);
    }

    #[test]
    fn structural_projection_rejects_missing_placement() {
        let mut scenario = small_scenario();
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        let err = build_structural_projection(&scenario).expect_err("missing placement");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::SteadMapping(_)
        ));
    }

    #[test]
    fn structural_projection_rejects_invalid_link_endpoint() {
        let mut scenario = small_scenario();
        scenario.links[0].to_system_id = "999".to_string();
        let err = build_structural_projection(&scenario).expect_err("invalid link");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::InvalidLinkEndpoint { .. }
        ));
    }

    #[test]
    fn structural_projection_link_indices_use_dense_location_indices() {
        let scenario = small_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert_eq!(projection.link_indices[0].from_dense_index, 0);
        assert_eq!(projection.link_indices[0].to_dense_index, 0);
    }

    #[test]
    fn gpu_residency_readiness_derives_from_scenario_authority() {
        let scenario = small_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.dense_location_index_ready);
        assert_eq!(readiness.location_count, 1);
    }

    #[test]
    fn gpu_residency_readiness_reports_rf_readiness() {
        let scenario = small_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.rf_accumulator_ready);
    }

    #[test]
    fn gpu_residency_readiness_reports_heatmap_readiness() {
        let scenario = small_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert_eq!(
            readiness.heatmap_ready,
            StudioHeatmapReadinessKind::BoundedTheaterEligible
        );
    }

    #[test]
    fn gpu_residency_readiness_contains_no_render_metadata() {
        let scenario = small_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        let encoded = format!("{readiness:?}");
        assert!(!encoded.contains("world_x"));
        assert!(!encoded.contains("render_meta"));
        assert!(!encoded.contains("sprite_scale"));
    }

    #[test]
    fn gpu_residency_readiness_reports_atlas_required_for_oversized_valid_grid() {
        let mut scenario = small_scenario();
        scenario.structural_grid.frame.width = 64;
        scenario.structural_grid.frame.height = 64;
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.atlas_required);
        assert_eq!(
            readiness.heatmap_ready,
            StudioHeatmapReadinessKind::AtlasRequired
        );
    }

    #[test]
    fn gpu_residency_readiness_rejects_invalid_stead() {
        let mut scenario = small_scenario();
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        let err = build_gpu_residency_readiness_from_scenario(&scenario).expect_err("invalid");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::SteadMapping(_)
        ));
    }
}
