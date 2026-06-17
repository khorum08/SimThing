//! Scenario-derived structural projection and GPU-resident readiness manifests.
//!
//! These types are projections/caches over `SimThingScenarioSpec` — not model authority and not
//! GPU buffers. They provide deterministic dense indices for future GPU upload planning.

use simthing_spec::{
    canonical_scenario_link_key, validate_scenario_links, validate_stead_mapping_consistency,
    ScenarioLinkError, SimThingScenarioSpec,
};

use simthing_gpu::{
    readback_matches_source, readback_structural_upload_blocking, upload_structural_rows_to_gpu,
    StructuralFrameGpuRow, StructuralLinkGpuRow, StructuralLocationGpuRow,
    StructuralUploadGpuReport, StructuralUploadReadback, StructuralUploadRows,
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StudioGpuStructuralFrameRow {
    pub width: u32,
    pub height: u32,
    pub occupied_cells: u32,
    pub location_count: u32,
    pub link_count: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StudioGpuLocationRow {
    pub dense_index: u32,
    pub simthing_id_raw: u32,
    pub system_id: u32,
    pub row: u32,
    pub col: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StudioGpuLinkRow {
    pub from_dense_index: u32,
    pub to_dense_index: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGpuStructuralUploadPacket {
    pub frame: StudioGpuStructuralFrameRow,
    pub locations: Vec<StudioGpuLocationRow>,
    pub links: Vec<StudioGpuLinkRow>,
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
    pub structural_upload_packet_ready: bool,
    pub structural_upload_packet_location_rows: u64,
    pub structural_upload_packet_link_rows: u64,
    pub structural_upload_packet_deferred_reason: Option<String>,
    pub gpu_buffer_residency_ready: bool,
    pub gpu_buffer_residency_deferred_reason: Option<String>,
    pub deferred_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioGpuBufferResidencyProof {
    pub ready: bool,
    pub deferred_reason: Option<String>,
    pub report: Option<StructuralUploadGpuReport>,
    pub readback: Option<StructuralUploadReadback>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioGpuStructuralUploadError {
    Projection(StudioStructuralProjectionError),
    CountOverflow { field: &'static str, value: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioStructuralProjectionError {
    InvalidLinkEndpoint { from: String, to: String },
    SelfLink { system_id: String },
    DuplicateLink { from: String, to: String },
    ReversedDuplicateLink { from: String, to: String },
    SteadMapping(String),
}

impl From<StudioStructuralProjectionError> for StudioHydrationError {
    fn from(err: StudioStructuralProjectionError) -> Self {
        match err {
            StudioStructuralProjectionError::InvalidLinkEndpoint { from, to }
            | StudioStructuralProjectionError::DuplicateLink { from, to }
            | StudioStructuralProjectionError::ReversedDuplicateLink { from, to } => {
                StudioHydrationError::HyperlaneEndpointMissing { from, to }
            }
            StudioStructuralProjectionError::SelfLink { system_id } => {
                StudioHydrationError::HyperlaneEndpointMissing {
                    from: system_id.clone(),
                    to: system_id,
                }
            }
            StudioStructuralProjectionError::SteadMapping(message) => {
                StudioHydrationError::SteadMappingInconsistent(message)
            }
        }
    }
}

fn map_scenario_link_error(err: ScenarioLinkError) -> StudioStructuralProjectionError {
    match err {
        ScenarioLinkError::InvalidEndpoint { from, to } => {
            StudioStructuralProjectionError::InvalidLinkEndpoint { from, to }
        }
        ScenarioLinkError::SelfLink { system_id } => {
            StudioStructuralProjectionError::SelfLink { system_id }
        }
        ScenarioLinkError::DuplicateLink { from, to } => {
            StudioStructuralProjectionError::DuplicateLink { from, to }
        }
        ScenarioLinkError::ReversedDuplicateLink { from, to } => {
            StudioStructuralProjectionError::ReversedDuplicateLink { from, to }
        }
    }
}

pub fn build_structural_projection(
    scenario: &SimThingScenarioSpec,
) -> Result<StudioStructuralProjection, StudioStructuralProjectionError> {
    validate_stead_mapping_consistency(scenario)
        .map_err(|err| StudioStructuralProjectionError::SteadMapping(err.to_string()))?;
    validate_scenario_links(scenario).map_err(map_scenario_link_error)?;

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
    let mut seen_dense_edges = std::collections::BTreeSet::new();
    for link in &scenario.links {
        canonical_scenario_link_key(link).map_err(map_scenario_link_error)?;
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
        if from_dense_index == to_dense_index {
            return Err(StudioStructuralProjectionError::SelfLink {
                system_id: link.from_system_id.clone(),
            });
        }
        let (min_dense, max_dense) = if from_dense_index < to_dense_index {
            (*from_dense_index, *to_dense_index)
        } else {
            (*to_dense_index, *from_dense_index)
        };
        if !seen_dense_edges.insert((min_dense, max_dense)) {
            return Err(StudioStructuralProjectionError::DuplicateLink {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        link_indices.push(StudioLinkIndexRow {
            from_dense_index: min_dense,
            to_dense_index: max_dense,
        });
    }
    link_indices.sort_by(|left, right| {
        left.from_dense_index
            .cmp(&right.from_dense_index)
            .then_with(|| left.to_dense_index.cmp(&right.to_dense_index))
    });

    Ok(StudioStructuralProjection {
        location_indices,
        link_indices,
    })
}

fn u32_count_or_overflow(
    field: &'static str,
    value: u64,
) -> Result<u32, StudioGpuStructuralUploadError> {
    u32::try_from(value).map_err(|_| StudioGpuStructuralUploadError::CountOverflow { field, value })
}

pub fn build_gpu_structural_upload_packet_from_projection(
    scenario: &SimThingScenarioSpec,
    projection: &StudioStructuralProjection,
) -> Result<StudioGpuStructuralUploadPacket, StudioGpuStructuralUploadError> {
    let frame = &scenario.structural_grid.frame;
    let location_count = projection.location_indices.len() as u64;
    let link_count = projection.link_indices.len() as u64;
    let occupied_cells = u32_count_or_overflow("occupied_cells", frame.occupied_cells)?;
    let location_count_u32 = u32_count_or_overflow("location_count", location_count)?;
    let link_count_u32 = u32_count_or_overflow("link_count", link_count)?;

    let locations: Vec<StudioGpuLocationRow> = projection
        .location_indices
        .iter()
        .map(|row| StudioGpuLocationRow {
            dense_index: row.dense_index,
            simthing_id_raw: row.simthing_id_raw,
            system_id: row.system_id,
            row: row.row,
            col: row.col,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
        })
        .collect();

    let links: Vec<StudioGpuLinkRow> = projection
        .link_indices
        .iter()
        .map(|row| StudioGpuLinkRow {
            from_dense_index: row.from_dense_index,
            to_dense_index: row.to_dense_index,
            reserved0: 0,
            reserved1: 0,
        })
        .collect();

    Ok(StudioGpuStructuralUploadPacket {
        frame: StudioGpuStructuralFrameRow {
            width: frame.width,
            height: frame.height,
            occupied_cells,
            location_count: location_count_u32,
            link_count: link_count_u32,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
        },
        locations,
        links,
    })
}

pub fn build_gpu_structural_upload_packet_from_scenario(
    scenario: &SimThingScenarioSpec,
) -> Result<StudioGpuStructuralUploadPacket, StudioGpuStructuralUploadError> {
    let projection = build_structural_projection(scenario)
        .map_err(StudioGpuStructuralUploadError::Projection)?;
    build_gpu_structural_upload_packet_from_projection(scenario, &projection)
}

pub fn to_structural_gpu_rows(packet: &StudioGpuStructuralUploadPacket) -> StructuralUploadRows {
    StructuralUploadRows {
        frame: StructuralFrameGpuRow {
            width: packet.frame.width,
            height: packet.frame.height,
            occupied_cells: packet.frame.occupied_cells,
            location_count: packet.frame.location_count,
            link_count: packet.frame.link_count,
            reserved0: packet.frame.reserved0,
            reserved1: packet.frame.reserved1,
            reserved2: packet.frame.reserved2,
        },
        locations: packet
            .locations
            .iter()
            .map(|row| StructuralLocationGpuRow {
                dense_index: row.dense_index,
                simthing_id_raw: row.simthing_id_raw,
                system_id: row.system_id,
                row: row.row,
                col: row.col,
                reserved0: row.reserved0,
                reserved1: row.reserved1,
                reserved2: row.reserved2,
            })
            .collect(),
        links: packet
            .links
            .iter()
            .map(|row| StructuralLinkGpuRow {
                from_dense_index: row.from_dense_index,
                to_dense_index: row.to_dense_index,
                reserved0: row.reserved0,
                reserved1: row.reserved1,
            })
            .collect(),
    }
}

pub fn prove_gpu_buffer_residency_blocking(
    device: &simthing_gpu::wgpu::Device,
    queue: &simthing_gpu::wgpu::Queue,
    packet: &StudioGpuStructuralUploadPacket,
) -> StudioGpuBufferResidencyProof {
    let rows = to_structural_gpu_rows(packet);
    match upload_structural_rows_to_gpu(device, queue, rows.frame, &rows.locations, &rows.links) {
        Ok((buffers, report)) => {
            let readback = readback_structural_upload_blocking(device, queue, &buffers, &report);
            if readback_matches_source(&readback, rows.frame, &rows.locations, &rows.links) {
                StudioGpuBufferResidencyProof {
                    ready: true,
                    deferred_reason: None,
                    report: Some(report),
                    readback: Some(readback),
                }
            } else {
                StudioGpuBufferResidencyProof {
                    ready: false,
                    deferred_reason: Some(
                        "GPU readback bytes did not match source rows".to_string(),
                    ),
                    report: Some(report),
                    readback: Some(readback),
                }
            }
        }
        Err(err) => StudioGpuBufferResidencyProof {
            ready: false,
            deferred_reason: Some(format!("GPU structural upload failed: {err}")),
            report: None,
            readback: None,
        },
    }
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
    let upload_packet = build_gpu_structural_upload_packet_from_projection(scenario, projection);
    let (structural_upload_packet_ready, structural_upload_packet_deferred_reason) =
        match &upload_packet {
            Ok(_packet) => (true, None),
            Err(StudioGpuStructuralUploadError::Projection(err)) => (
                false,
                Some(format!(
                    "structural upload packet projection failed: {err:?}"
                )),
            ),
            Err(StudioGpuStructuralUploadError::CountOverflow { field, value }) => (
                false,
                Some(format!(
                    "structural upload packet count overflow: {field}={value}"
                )),
            ),
        };
    let structural_upload_packet_location_rows = upload_packet
        .as_ref()
        .map(|packet| packet.locations.len() as u64)
        .unwrap_or(0);
    let structural_upload_packet_link_rows = upload_packet
        .as_ref()
        .map(|packet| packet.links.len() as u64)
        .unwrap_or(0);
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
        structural_upload_packet_ready,
        structural_upload_packet_location_rows,
        structural_upload_packet_link_rows,
        structural_upload_packet_deferred_reason,
        gpu_buffer_residency_ready: false,
        gpu_buffer_residency_deferred_reason: Some(
            "GPU buffer residency requires device upload context".to_string(),
        ),
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

    fn add_gridcell(
        map: &mut SimThing,
        system_id: u32,
        row: u32,
        col: u32,
    ) -> SimThingStructuralGridPlacement {
        let mut cell = SimThing::new(SimThingKind::Location, 0);
        cell.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(system_id),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_COL_PROPERTY_ID,
            structural_property_value_u32(col),
        );
        cell.add_property(
            SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
            structural_property_value_u32(row),
        );
        let mut payload = SimThing::new(SimThingKind::Cohort, 0);
        payload.add_property(
            SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID,
            structural_property_value_u32(system_id),
        );
        cell.add_child(payload);
        let cell_raw = cell.id.raw();
        let placement = SimThingStructuralGridPlacement {
            location_id: format!("cell_{system_id}"),
            target_id: format!("cell_{system_id}"),
            system_id,
            row,
            col,
            simthing_id_raw: cell_raw,
        };
        map.add_child(cell);
        placement
    }

    fn two_cell_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let map_raw = map.id.raw();
        let placement_a = add_gridcell(&mut map, 1, 2, 3);
        let placement_b = add_gridcell(&mut map, 2, 2, 4);
        root.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "two_cell_spec".to_string(),
            root,
            structural_grid: SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 2,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![placement_a, placement_b],
            },
            links: vec![SimThingScenarioLink {
                from_system_id: "1".to_string(),
                to_system_id: "2".to_string(),
            }],
            provenance: SimThingScenarioProvenance::default(),
        }
    }

    fn single_cell_scenario() -> SimThingScenarioSpec {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let mut map = SimThing::new(SimThingKind::Location, 0);
        let map_raw = map.id.raw();
        let placement = add_gridcell(&mut map, 1, 2, 3);
        root.add_child(map);
        SimThingScenarioSpec {
            scenario_id: "single_cell_spec".to_string(),
            root,
            structural_grid: SimThingScenarioGrid {
                frame: SimThingStructuralGridFrame {
                    width: 8,
                    height: 8,
                    occupied_cells: 1,
                },
                map_container_id: map_raw.to_string(),
                placements: vec![placement],
            },
            links: Vec::new(),
            provenance: SimThingScenarioProvenance::default(),
        }
    }

    #[test]
    fn structural_projection_derives_from_scenario_authority() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert_eq!(projection.location_indices.len(), 2);
        assert_eq!(projection.link_indices.len(), 1);
    }

    #[test]
    fn structural_projection_has_deterministic_dense_indices() {
        let scenario = two_cell_scenario();
        let first = build_structural_projection(&scenario).expect("first");
        let second = build_structural_projection(&scenario).expect("second");
        assert_eq!(first, second);
        assert_eq!(first.location_indices[0].dense_index, 0);
    }

    #[test]
    fn structural_projection_uses_structural_coords_not_render_coords() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let row = &projection.location_indices[0];
        assert_eq!(row.col, 3);
        assert_eq!(row.row, 2);
    }

    #[test]
    fn structural_projection_rejects_missing_placement() {
        let mut scenario = single_cell_scenario();
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
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "999".to_string();
        let err = build_structural_projection(&scenario).expect_err("invalid link");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::InvalidLinkEndpoint { .. }
        ));
    }

    #[test]
    fn structural_projection_rejects_self_link() {
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "1".to_string();
        let err = build_structural_projection(&scenario).expect_err("self link");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::SelfLink { .. }
        ));
    }

    #[test]
    fn structural_projection_rejects_direct_duplicate_link() {
        let mut scenario = two_cell_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        });
        let err = build_structural_projection(&scenario).expect_err("duplicate");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::DuplicateLink { .. }
        ));
    }

    #[test]
    fn structural_projection_rejects_reversed_duplicate_link() {
        let mut scenario = two_cell_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "2".to_string(),
            to_system_id: "1".to_string(),
        });
        let err = build_structural_projection(&scenario).expect_err("reversed duplicate");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::ReversedDuplicateLink { .. }
        ));
    }

    #[test]
    fn structural_projection_sorts_link_indices_deterministically() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert!(
            projection.link_indices[0].from_dense_index
                <= projection.link_indices[0].to_dense_index
        );
        let again = build_structural_projection(&scenario).expect("again");
        assert_eq!(projection.link_indices, again.link_indices);
    }

    #[test]
    fn structural_projection_link_indices_use_canonical_dense_pairs() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert_eq!(projection.link_indices[0].from_dense_index, 0);
        assert_eq!(projection.link_indices[0].to_dense_index, 1);
        assert!(
            projection.link_indices[0].from_dense_index < projection.link_indices[0].to_dense_index
        );
    }

    #[test]
    fn structural_projection_link_indices_use_dense_location_indices() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        assert_eq!(projection.link_indices[0].from_dense_index, 0);
        assert_eq!(projection.link_indices[0].to_dense_index, 1);
    }

    #[test]
    fn gpu_residency_readiness_derives_from_scenario_authority() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.dense_location_index_ready);
        assert_eq!(readiness.location_count, 2);
    }

    #[test]
    fn gpu_residency_readiness_reports_rf_readiness() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.rf_accumulator_ready);
    }

    #[test]
    fn gpu_residency_readiness_reports_heatmap_readiness() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert_eq!(
            readiness.heatmap_ready,
            StudioHeatmapReadinessKind::BoundedTheaterEligible
        );
    }

    #[test]
    fn gpu_residency_readiness_contains_no_render_metadata() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        let encoded = format!("{readiness:?}");
        assert!(!encoded.contains("world_x"));
        assert!(!encoded.contains("render_meta"));
        assert!(!encoded.contains("sprite_scale"));
    }

    #[test]
    fn gpu_residency_readiness_reports_atlas_required_for_oversized_valid_grid() {
        let mut scenario = two_cell_scenario();
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
        let mut scenario = single_cell_scenario();
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        let err = build_gpu_residency_readiness_from_scenario(&scenario).expect_err("invalid");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::SteadMapping(_)
        ));
    }

    #[test]
    fn gpu_residency_readiness_rejects_duplicate_or_self_links() {
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "1".to_string();
        let err = build_gpu_residency_readiness_from_scenario(&scenario).expect_err("self");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::SelfLink { .. }
        ));

        let mut scenario = two_cell_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        });
        let err = build_gpu_residency_readiness_from_scenario(&scenario).expect_err("duplicate");
        assert!(matches!(
            err,
            StudioStructuralProjectionError::DuplicateLink { .. }
        ));
    }

    fn pod_row_bytes<T: Copy>(rows: &[T]) -> Vec<u8> {
        let byte_len = rows.len() * std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts(rows.as_ptr() as *const u8, byte_len) };
        slice.to_vec()
    }

    #[test]
    fn gpu_structural_upload_row_layout_is_stable_repr_c() {
        assert_eq!(std::mem::size_of::<StudioGpuStructuralFrameRow>(), 32);
        assert_eq!(std::mem::size_of::<StudioGpuLocationRow>(), 32);
        assert_eq!(std::mem::size_of::<StudioGpuLinkRow>(), 16);
        assert_eq!(std::mem::align_of::<StudioGpuStructuralFrameRow>(), 4);
        assert_eq!(std::mem::align_of::<StudioGpuLocationRow>(), 4);
        assert_eq!(std::mem::align_of::<StudioGpuLinkRow>(), 4);
    }

    #[test]
    fn gpu_structural_upload_packet_derives_from_scenario_authority() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        assert_eq!(packet.frame.location_count, 2);
        assert_eq!(packet.frame.link_count, 1);
    }

    #[test]
    fn gpu_structural_upload_packet_uses_structural_projection() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let packet = build_gpu_structural_upload_packet_from_projection(&scenario, &projection)
            .expect("packet");
        assert_eq!(packet.locations.len(), projection.location_indices.len());
        assert_eq!(packet.links.len(), projection.link_indices.len());
    }

    #[test]
    fn gpu_structural_upload_packet_preserves_frame() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        assert_eq!(packet.frame.width, 8);
        assert_eq!(packet.frame.height, 8);
        assert_eq!(packet.frame.occupied_cells, 2);
    }

    #[test]
    fn gpu_structural_upload_packet_preserves_location_rows() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        assert_eq!(
            packet.locations[0].dense_index,
            projection.location_indices[0].dense_index
        );
        assert_eq!(
            packet.locations[0].simthing_id_raw,
            projection.location_indices[0].simthing_id_raw
        );
        assert_eq!(packet.locations[0].row, projection.location_indices[0].row);
        assert_eq!(packet.locations[0].col, projection.location_indices[0].col);
    }

    #[test]
    fn gpu_structural_upload_packet_preserves_canonical_link_rows() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        assert_eq!(
            packet.links[0].from_dense_index,
            projection.link_indices[0].from_dense_index
        );
        assert_eq!(
            packet.links[0].to_dense_index,
            projection.link_indices[0].to_dense_index
        );
    }

    #[test]
    fn gpu_structural_upload_packet_orders_locations_deterministically() {
        let scenario = two_cell_scenario();
        let first = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("first");
        let second = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("second");
        assert_eq!(first.locations, second.locations);
        assert!(first.locations[0].dense_index < first.locations[1].dense_index);
    }

    #[test]
    fn gpu_structural_upload_packet_orders_links_deterministically() {
        let scenario = two_cell_scenario();
        let first = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("first");
        let second = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("second");
        assert_eq!(first.links, second.links);
    }

    #[test]
    fn gpu_structural_upload_packet_rejects_invalid_stead() {
        let mut scenario = single_cell_scenario();
        scenario.structural_grid.placements.clear();
        scenario.structural_grid.frame.occupied_cells = 0;
        let err = build_gpu_structural_upload_packet_from_scenario(&scenario).expect_err("stead");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::Projection(
                StudioStructuralProjectionError::SteadMapping(_)
            )
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_rejects_unknown_link_endpoint() {
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "999".to_string();
        let err = build_gpu_structural_upload_packet_from_scenario(&scenario).expect_err("unknown");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::Projection(
                StudioStructuralProjectionError::InvalidLinkEndpoint { .. }
            )
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_rejects_self_link() {
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "1".to_string();
        let err = build_gpu_structural_upload_packet_from_scenario(&scenario).expect_err("self");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::Projection(
                StudioStructuralProjectionError::SelfLink { .. }
            )
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_rejects_direct_duplicate_link() {
        let mut scenario = two_cell_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "1".to_string(),
            to_system_id: "2".to_string(),
        });
        let err =
            build_gpu_structural_upload_packet_from_scenario(&scenario).expect_err("duplicate");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::Projection(
                StudioStructuralProjectionError::DuplicateLink { .. }
            )
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_rejects_reversed_duplicate_link() {
        let mut scenario = two_cell_scenario();
        scenario.links.push(SimThingScenarioLink {
            from_system_id: "2".to_string(),
            to_system_id: "1".to_string(),
        });
        let err = build_gpu_structural_upload_packet_from_scenario(&scenario)
            .expect_err("reversed duplicate");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::Projection(
                StudioStructuralProjectionError::ReversedDuplicateLink { .. }
            )
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_contains_no_render_metadata() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let encoded = format!("{packet:?}");
        assert!(!encoded.contains("world_x"));
        assert!(!encoded.contains("render_meta"));
        assert!(!encoded.contains("sprite_scale"));
    }

    #[test]
    fn gpu_structural_upload_packet_contains_no_bevy_entity_ids() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let encoded = format!("{packet:?}");
        assert!(!encoded.contains("Entity"));
        assert!(!encoded.contains("bevy"));
    }

    #[test]
    fn gpu_structural_upload_packet_count_overflow_is_error() {
        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let mut overflow_frame = scenario.clone();
        overflow_frame.structural_grid.frame.occupied_cells = u64::from(u32::MAX) + 1;
        let err = build_gpu_structural_upload_packet_from_projection(&overflow_frame, &projection)
            .expect_err("overflow");
        assert!(matches!(
            err,
            StudioGpuStructuralUploadError::CountOverflow {
                field: "occupied_cells",
                ..
            }
        ));
    }

    #[test]
    fn gpu_structural_upload_packet_row_bytes_are_deterministic() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let frame_a = pod_row_bytes(std::slice::from_ref(&packet.frame));
        let frame_b = pod_row_bytes(std::slice::from_ref(&packet.frame));
        assert_eq!(frame_a, frame_b);
        assert_eq!(
            pod_row_bytes(&packet.locations),
            pod_row_bytes(&packet.locations)
        );
        assert_eq!(pod_row_bytes(&packet.links), pod_row_bytes(&packet.links));
    }

    #[test]
    fn gpu_residency_readiness_reports_upload_packet_ready_for_valid_scenario() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.structural_upload_packet_ready);
        assert_eq!(readiness.structural_upload_packet_location_rows, 2);
        assert_eq!(readiness.structural_upload_packet_link_rows, 1);
        assert!(readiness.structural_upload_packet_deferred_reason.is_none());
    }

    #[test]
    fn gpu_residency_readiness_reports_upload_packet_not_ready_for_invalid_links() {
        let mut scenario = two_cell_scenario();
        scenario.links[0].to_system_id = "1".to_string();
        assert!(build_gpu_residency_readiness_from_scenario(&scenario).is_err());
        assert!(build_gpu_structural_upload_packet_from_scenario(&scenario).is_err());

        let valid = two_cell_scenario();
        let projection = build_structural_projection(&valid).expect("projection");
        let mut overflow = valid.clone();
        overflow.structural_grid.frame.occupied_cells = u64::from(u32::MAX) + 1;
        let readiness = build_gpu_residency_readiness(&overflow, &projection);
        assert!(!readiness.structural_upload_packet_ready);
        assert!(readiness
            .structural_upload_packet_deferred_reason
            .as_ref()
            .is_some_and(|reason| reason.contains("count overflow")));
    }

    #[test]
    fn gpu_residency_readiness_keeps_atlas_required_distinct_from_packet_invalidity() {
        let mut scenario = two_cell_scenario();
        scenario.structural_grid.frame.width = 64;
        scenario.structural_grid.frame.height = 64;
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(readiness.atlas_required);
        assert!(readiness.structural_upload_packet_ready);
        assert!(readiness.structural_upload_packet_deferred_reason.is_none());
    }

    #[test]
    fn mapeditor_packet_converts_to_gpu_rows_exactly() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let rows = to_structural_gpu_rows(&packet);
        assert_eq!(rows.frame.width, packet.frame.width);
        assert_eq!(rows.frame.location_count, 2);
        assert_eq!(
            rows.locations[0].dense_index,
            packet.locations[0].dense_index
        );
        assert_eq!(
            rows.links[0].from_dense_index,
            packet.links[0].from_dense_index
        );
    }

    #[test]
    fn gpu_rows_preserve_canonical_link_order() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let rows = to_structural_gpu_rows(&packet);
        assert_eq!(rows.links[0].from_dense_index, 0);
        assert_eq!(rows.links[0].to_dense_index, 1);
    }

    #[test]
    fn gpu_rows_contain_no_render_metadata() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let rows = to_structural_gpu_rows(&packet);
        let encoded = format!("{rows:?}");
        assert!(!encoded.contains("world_x"));
        assert!(!encoded.contains("render_meta"));
    }

    #[test]
    fn gpu_rows_contain_no_route_or_predecessor_fields() {
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let rows = to_structural_gpu_rows(&packet);
        let encoded = format!("{rows:?}");
        for forbidden in [
            "route",
            "predecessor",
            "movement_order",
            "pathfinding",
            "frontline",
        ] {
            assert!(!encoded.contains(forbidden));
        }
    }

    #[test]
    fn gpu_residency_readiness_defers_buffer_residency_without_device_context() {
        let scenario = two_cell_scenario();
        let readiness = build_gpu_residency_readiness_from_scenario(&scenario).expect("readiness");
        assert!(!readiness.gpu_buffer_residency_ready);
        assert!(readiness
            .gpu_buffer_residency_deferred_reason
            .as_ref()
            .is_some_and(|reason| reason.contains("device upload context")));
    }

    #[test]
    fn gpu_buffer_residency_proof_uploads_and_readbacks_exact_bytes() {
        use simthing_gpu::context::GpuContext;

        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let scenario = two_cell_scenario();
        let packet = build_gpu_structural_upload_packet_from_scenario(&scenario).expect("packet");
        let proof = prove_gpu_buffer_residency_blocking(&ctx.device, &ctx.queue, &packet);
        assert!(proof.ready, "{:?}", proof.deferred_reason);
        assert!(proof.report.is_some());
        assert!(proof.readback.is_some());
    }

    #[test]
    fn gpu_buffer_residency_proof_rejects_empty_location_packet() {
        use simthing_gpu::context::GpuContext;

        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let packet = StudioGpuStructuralUploadPacket {
            frame: StudioGpuStructuralFrameRow {
                width: 8,
                height: 8,
                occupied_cells: 0,
                location_count: 0,
                link_count: 0,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            locations: Vec::new(),
            links: Vec::new(),
        };
        let proof = prove_gpu_buffer_residency_blocking(&ctx.device, &ctx.queue, &packet);
        assert!(!proof.ready);
        assert!(proof
            .deferred_reason
            .as_ref()
            .is_some_and(|reason| reason.contains("location row")));
    }

    #[test]
    fn scenario_save_load_roundtrip_preserves_canonical_link_projection() {
        use crate::scenario_io::{
            load_studio_session_from_scenario_path, save_scenario_authority_to_path,
        };
        use tempfile::TempDir;

        let scenario = two_cell_scenario();
        let projection = build_structural_projection(&scenario).expect("projection");
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("canonical-link.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &scenario).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert_eq!(loaded.structural_projection, projection);
    }
}
