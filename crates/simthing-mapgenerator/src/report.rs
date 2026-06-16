//! Deterministic machine-readable generation report (`mapgenerator.report.v1`).

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use serde::Serialize;

use crate::coupling::CouplingEdgeKind;
use crate::params::MapGeneratorParams;
use crate::strategy::ShapePlacement;
use crate::topology::{system_id_scalar, HyperlaneEdge};
use crate::GalaxyGenerationResult;

pub const REPORT_SCHEMA_VERSION: &str = "mapgenerator.report.v1";

/// Topology target ratio below this fails map quality (`topology_target_ratio`).
pub const TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD: f64 = 0.50;
/// Connectivity bridge share above this warns (`connectivity_bridge_ratio`).
pub const CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD: f64 = 0.25;
/// Connectivity bridge share above this fails map quality.
pub const CONNECTIVITY_BRIDGE_RATIO_FAIL_THRESHOLD: f64 = 0.50;
/// Average degree below this warns on dense preview maps (`star_count` or target ≥ 1000).
pub const DENSE_PREVIEW_AVG_DEGREE_WARN_THRESHOLD: f64 = 2.5;
/// Longest connectivity-bridge Chebyshev span above this warns (documented producer threshold).
pub const LONGEST_BRIDGE_CHEBYSHEV_WARN_THRESHOLD: u32 = 32;

pub const MAP_QUALITY_PASS: &str = "PASS";
pub const MAP_QUALITY_WARN: &str = "WARN";
pub const MAP_QUALITY_FAIL: &str = "FAIL";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GenerationReport {
    pub schema_version: &'static str,
    pub generator: GeneratorMeta,
    pub request: RequestSection,
    pub output: OutputSection,
    pub artifacts: ArtifactsSection,
    pub constitution: ConstitutionSection,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GeneratorMeta {
    #[serde(rename = "crate")]
    pub crate_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_version: Option<&'static str>,
    pub track: &'static str,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RequestSection {
    pub shape: String,
    pub star_count: u32,
    pub lattice_width: u32,
    pub lattice_height: u32,
    pub target_hyperlanes: u32,
    pub ensure_connected: bool,
    pub allow_disconnected: bool,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub shape_params: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BoundingBox {
    pub min_col: u32,
    pub max_col: u32,
    pub min_row: u32,
    pub max_row: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OutputSection {
    pub system_count: u32,
    /// Total base hyperlane edges after connectivity repair (topology + connectivity bridges).
    pub base_hyperlane_count: u32,
    /// Pre-connectivity bounded topology edges (local adjacency heuristic only).
    #[serde(alias = "topology_hyperlane_count")]
    pub actual_topology_hyperlanes: u32,
    pub special_route_count: u32,
    pub partition_bridge_count: u32,
    pub cluster_bridge_count: u32,
    /// Connectivity-repair bridges only (not partition/cluster/special routes).
    #[serde(alias = "bridge_count")]
    pub connectivity_bridge_count: u32,
    pub component_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components_before: Option<u32>,
    pub isolated_system_count: u32,
    pub min_degree: u32,
    pub max_degree: u32,
    pub average_degree: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest_bridge_chebyshev: Option<u32>,
    pub bounding_box: BoundingBox,
    pub occupied_cell_count: u32,
    pub duplicate_cell_count: u32,
    /// Requested `--num-hyperlanes` / `num_hyperlanes_default` (before clamp).
    pub requested_target_hyperlanes: u32,
    /// Same as `base_hyperlane_count` (explicit alias for editor consumption).
    pub actual_base_hyperlanes: u32,
    /// Effective topology target after clamping to `[num_hyperlanes_min, num_hyperlanes_max]`.
    pub effective_target_hyperlanes: u32,
    pub topology_target_satisfied: bool,
    pub topology_target_deficit: u32,
    pub topology_target_ratio: f64,
    pub connectivity_bridge_ratio: f64,
    pub map_quality_status: &'static str,
    pub map_quality_warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ArtifactsSection {
    pub scenario_path: Option<String>,
    pub png_path: Option<String>,
    pub report_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ConstitutionSection {
    pub structural_coordinates: &'static str,
    pub render_coordinates_authoritative: bool,
    pub uses_native_sqrt_authority: bool,
    pub uses_pathfinding_semantics: bool,
    pub uses_runtime_simulation: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error("failed to write generation report: {0}")]
    Write(#[from] std::io::Error),
    #[error("failed to serialize generation report: {0}")]
    Serialize(#[from] serde_json::Error),
}

pub struct ReportArtifacts<'a> {
    pub scenario_path: Option<&'a Path>,
    pub png_path: Option<&'a Path>,
    pub report_path: Option<&'a Path>,
}

impl<'a> ReportArtifacts<'a> {
    pub fn new() -> Self {
        Self {
            scenario_path: None,
            png_path: None,
            report_path: None,
        }
    }
}

impl Default for ReportArtifacts<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn build_generation_report(
    params: &MapGeneratorParams,
    result: &GalaxyGenerationResult,
    artifacts: ReportArtifacts<'_>,
) -> GenerationReport {
    let edge = result.lattice.edge();
    let placement_stats = placement_stats(&result.placement);
    let degree_stats = degree_stats(&result.placement, &result.base_hyperlane_edges);
    let coupling_counts = coupling_counts(&result.classified_edges);
    let connectivity_bridge_count = result
        .connectivity
        .map(|report| report.bridges_added)
        .unwrap_or(0);
    let actual_topology_hyperlanes = result.pre_connectivity_topology_count;
    let actual_base_hyperlanes = result.base_hyperlane_edges.len() as u32;
    let requested_target_hyperlanes = params.hyperlane.num_hyperlanes_default;
    let effective_target_hyperlanes = if result.effective_target_hyperlanes > 0 {
        result.effective_target_hyperlanes
    } else {
        requested_target_hyperlanes.clamp(
            params.hyperlane.num_hyperlanes_min,
            params.hyperlane.num_hyperlanes_max,
        )
    };
    let component_count = result
        .connectivity
        .map(|c| c.components_after)
        .unwrap_or_else(|| count_components(&result.placement, &result.base_hyperlane_edges));

    let topology_target_deficit =
        requested_target_hyperlanes.saturating_sub(actual_topology_hyperlanes);
    let topology_target_ratio = if requested_target_hyperlanes == 0 {
        1.0
    } else {
        (actual_topology_hyperlanes as f64) / (requested_target_hyperlanes as f64)
    };
    let topology_target_ratio = round_ratio(topology_target_ratio);
    let topology_target_satisfied = actual_topology_hyperlanes >= effective_target_hyperlanes;
    let connectivity_bridge_ratio = if actual_base_hyperlanes == 0 {
        0.0
    } else {
        round_ratio(connectivity_bridge_count as f64 / actual_base_hyperlanes as f64)
    };

    let quality = evaluate_map_quality(MapQualityInput {
        requested_star_count: params.scale_core.num_stars,
        system_count: result.placement.systems.len() as u32,
        duplicate_cell_count: placement_stats.duplicate_cell_count,
        ensure_connected: params.hyperlane.ensure_connected,
        component_count,
        isolated_system_count: degree_stats.isolated,
        requested_target_hyperlanes,
        effective_target_hyperlanes,
        actual_topology_hyperlanes,
        topology_target_ratio,
        connectivity_bridge_ratio,
        average_degree: degree_stats.average,
        longest_bridge_chebyshev: result.connectivity.map(|c| c.max_bridge_chebyshev),
    });

    GenerationReport {
        schema_version: REPORT_SCHEMA_VERSION,
        generator: GeneratorMeta {
            crate_name: env!("CARGO_PKG_NAME"),
            crate_version: Some(env!("CARGO_PKG_VERSION")),
            track: "MapGeneratorCLI",
            seed: result.seed,
        },
        request: RequestSection {
            shape: params.shape.shape.clone(),
            star_count: params.scale_core.num_stars,
            lattice_width: edge,
            lattice_height: edge,
            target_hyperlanes: requested_target_hyperlanes,
            ensure_connected: params.hyperlane.ensure_connected,
            allow_disconnected: !params.hyperlane.ensure_connected,
            shape_params: params.shape.shape_params.clone(),
        },
        output: OutputSection {
            system_count: result.placement.systems.len() as u32,
            base_hyperlane_count: actual_base_hyperlanes,
            actual_topology_hyperlanes,
            special_route_count: coupling_counts.special_route,
            partition_bridge_count: coupling_counts.partition_bridge,
            cluster_bridge_count: coupling_counts.cluster_bridge,
            connectivity_bridge_count,
            component_count,
            components_before: result.connectivity.map(|c| c.components_before),
            isolated_system_count: degree_stats.isolated,
            min_degree: degree_stats.min,
            max_degree: degree_stats.max,
            average_degree: degree_stats.average,
            longest_bridge_chebyshev: result.connectivity.map(|c| c.max_bridge_chebyshev),
            bounding_box: placement_stats.bounding_box,
            occupied_cell_count: placement_stats.occupied_cell_count,
            duplicate_cell_count: placement_stats.duplicate_cell_count,
            requested_target_hyperlanes,
            actual_base_hyperlanes,
            effective_target_hyperlanes,
            topology_target_satisfied,
            topology_target_deficit,
            topology_target_ratio,
            connectivity_bridge_ratio,
            map_quality_status: quality.status,
            map_quality_warnings: quality.warnings,
        },
        artifacts: ArtifactsSection {
            scenario_path: path_to_report_string(artifacts.scenario_path),
            png_path: path_to_report_string(artifacts.png_path),
            report_path: path_to_report_string(artifacts.report_path),
        },
        constitution: ConstitutionSection {
            structural_coordinates: "authored_gridcell",
            render_coordinates_authoritative: false,
            uses_native_sqrt_authority: false,
            uses_pathfinding_semantics: false,
            uses_runtime_simulation: false,
        },
    }
}

pub fn generation_report_to_json(report: &GenerationReport) -> Result<String, ReportError> {
    Ok(serde_json::to_string_pretty(report)?)
}

pub fn write_generation_report_json(
    report: &GenerationReport,
    path: impl AsRef<Path>,
) -> Result<(), ReportError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, generation_report_to_json(report)?)?;
    Ok(())
}

/// Compare report content ignoring artifact path strings (for deterministic tests).
pub fn normalized_report_json(report: &GenerationReport) -> Result<String, ReportError> {
    let mut copy = report.clone();
    copy.artifacts = ArtifactsSection {
        scenario_path: None,
        png_path: None,
        report_path: None,
    };
    generation_report_to_json(&copy)
}

struct PlacementStats {
    bounding_box: BoundingBox,
    occupied_cell_count: u32,
    duplicate_cell_count: u32,
}

struct DegreeStats {
    min: u32,
    max: u32,
    average: f64,
    isolated: u32,
}

struct CouplingCounts {
    special_route: u32,
    partition_bridge: u32,
    cluster_bridge: u32,
}

struct MapQualityInput {
    requested_star_count: u32,
    system_count: u32,
    duplicate_cell_count: u32,
    ensure_connected: bool,
    component_count: u32,
    isolated_system_count: u32,
    requested_target_hyperlanes: u32,
    effective_target_hyperlanes: u32,
    actual_topology_hyperlanes: u32,
    topology_target_ratio: f64,
    connectivity_bridge_ratio: f64,
    average_degree: f64,
    longest_bridge_chebyshev: Option<u32>,
}

struct MapQualityEvaluation {
    status: &'static str,
    warnings: Vec<String>,
}

fn round_ratio(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

fn evaluate_map_quality(input: MapQualityInput) -> MapQualityEvaluation {
    let mut fails = Vec::new();
    let mut warns = Vec::new();

    if input.system_count != input.requested_star_count {
        fails.push(format!(
            "system_count {0} != requested star_count {1}",
            input.system_count, input.requested_star_count
        ));
    }
    if input.duplicate_cell_count > 0 {
        fails.push(format!(
            "duplicate_cell_count {} > 0",
            input.duplicate_cell_count
        ));
    }
    if input.ensure_connected && input.component_count != 1 {
        fails.push(format!(
            "component_count {} != 1 with ensure_connected",
            input.component_count
        ));
    }
    if input.ensure_connected && input.isolated_system_count > 0 {
        fails.push(format!(
            "isolated_system_count {} > 0 with ensure_connected",
            input.isolated_system_count
        ));
    }
    if input.requested_target_hyperlanes > input.effective_target_hyperlanes {
        warns.push(format!(
            "topology target clamped from {} to {} by num_hyperlanes_max/min",
            input.requested_target_hyperlanes, input.effective_target_hyperlanes
        ));
    }
    if input.topology_target_ratio < TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD {
        fails.push(format!(
            "topology_target_ratio {0:.4} < {1} (actual_topology_hyperlanes={2}, requested_target_hyperlanes={3})",
            input.topology_target_ratio,
            TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD,
            input.actual_topology_hyperlanes,
            input.requested_target_hyperlanes
        ));
    }
    if input.connectivity_bridge_ratio > CONNECTIVITY_BRIDGE_RATIO_FAIL_THRESHOLD {
        fails.push(format!(
            "connectivity_bridge_ratio {0:.4} > {1}",
            input.connectivity_bridge_ratio, CONNECTIVITY_BRIDGE_RATIO_FAIL_THRESHOLD
        ));
    } else if input.connectivity_bridge_ratio > CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD {
        warns.push(format!(
            "connectivity_bridge_ratio {0:.4} > {1}",
            input.connectivity_bridge_ratio, CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD
        ));
    }
    let dense_preview =
        input.requested_star_count >= 1000 || input.requested_target_hyperlanes >= 1000;
    if dense_preview && input.average_degree < DENSE_PREVIEW_AVG_DEGREE_WARN_THRESHOLD {
        warns.push(format!(
            "average_degree {0:.2} < {1} for dense preview map",
            input.average_degree, DENSE_PREVIEW_AVG_DEGREE_WARN_THRESHOLD
        ));
    }
    if let Some(longest) = input.longest_bridge_chebyshev {
        if longest > LONGEST_BRIDGE_CHEBYSHEV_WARN_THRESHOLD {
            warns.push(format!(
                "longest_bridge_chebyshev {longest} > {LONGEST_BRIDGE_CHEBYSHEV_WARN_THRESHOLD}"
            ));
        }
    }

    let status = if !fails.is_empty() {
        MAP_QUALITY_FAIL
    } else if !warns.is_empty() {
        MAP_QUALITY_WARN
    } else {
        MAP_QUALITY_PASS
    };
    let mut warnings = fails;
    warnings.extend(warns);
    MapQualityEvaluation { status, warnings }
}

pub fn map_quality_is_acceptable_for_editor(report: &GenerationReport) -> bool {
    report.output.map_quality_status == MAP_QUALITY_PASS
}

fn path_to_report_string(path: Option<&Path>) -> Option<String> {
    path.map(|p| p.to_string_lossy().into_owned())
}

fn placement_stats(placement: &ShapePlacement) -> PlacementStats {
    if placement.systems.is_empty() {
        return PlacementStats {
            bounding_box: BoundingBox {
                min_col: 0,
                max_col: 0,
                min_row: 0,
                max_row: 0,
            },
            occupied_cell_count: 0,
            duplicate_cell_count: 0,
        };
    }
    let mut min_col = u32::MAX;
    let mut max_col = 0u32;
    let mut min_row = u32::MAX;
    let mut max_row = 0u32;
    let mut seen = HashMap::<(u32, u32), u32>::new();
    for system in &placement.systems {
        min_col = min_col.min(system.coord.col);
        max_col = max_col.max(system.coord.col);
        min_row = min_row.min(system.coord.row);
        max_row = max_row.max(system.coord.row);
        *seen
            .entry((system.coord.col, system.coord.row))
            .or_insert(0) += 1;
    }
    let duplicate_cell_count = seen.values().filter(|&&count| count > 1).count() as u32;
    PlacementStats {
        bounding_box: BoundingBox {
            min_col,
            max_col,
            min_row,
            max_row,
        },
        occupied_cell_count: seen.len() as u32,
        duplicate_cell_count,
    }
}

fn degree_stats(placement: &ShapePlacement, edges: &[HyperlaneEdge]) -> DegreeStats {
    let mut degree: HashMap<String, u32> = placement
        .systems
        .iter()
        .map(|system| (system_id_scalar(system), 0))
        .collect();
    for edge in edges {
        *degree.entry(edge.from.clone()).or_insert(0) += 1;
        *degree.entry(edge.to.clone()).or_insert(0) += 1;
    }
    if degree.is_empty() {
        return DegreeStats {
            min: 0,
            max: 0,
            average: 0.0,
            isolated: 0,
        };
    }
    let mut min = u32::MAX;
    let mut max = 0u32;
    let mut total = 0u64;
    let mut isolated = 0u32;
    for value in degree.values() {
        min = min.min(*value);
        max = max.max(*value);
        total += *value as u64;
        if *value == 0 {
            isolated += 1;
        }
    }
    let average = (total as f64) / (degree.len() as f64);
    DegreeStats {
        min,
        max,
        average: (average * 100.0).round() / 100.0,
        isolated,
    }
}

fn coupling_counts(classified: &[crate::coupling::ClassifiedCouplingEdge]) -> CouplingCounts {
    let mut counts = CouplingCounts {
        special_route: 0,
        partition_bridge: 0,
        cluster_bridge: 0,
    };
    for entry in classified {
        match entry.kind {
            CouplingEdgeKind::BaseHyperlane => {}
            CouplingEdgeKind::SpecialRouteCoupling => counts.special_route += 1,
            CouplingEdgeKind::PartitionBridgeCoupling => counts.partition_bridge += 1,
            CouplingEdgeKind::ClusterBridgeCoupling => counts.cluster_bridge += 1,
        }
    }
    counts
}

fn count_components(placement: &ShapePlacement, edges: &[HyperlaneEdge]) -> u32 {
    let n = placement.systems.len();
    if n == 0 {
        return 0;
    }
    let index_of: HashMap<String, usize> = placement
        .systems
        .iter()
        .enumerate()
        .map(|(i, s)| (system_id_scalar(s), i))
        .collect();
    let mut parent: Vec<usize> = (0..n).collect();
    let find = |x: usize, parent: &mut Vec<usize>| -> usize {
        let mut root = x;
        while parent[root] != root {
            parent[root] = parent[parent[root]];
            root = parent[root];
        }
        root
    };
    for edge in edges {
        if let (Some(&i), Some(&j)) = (index_of.get(&edge.from), index_of.get(&edge.to)) {
            let ri = find(i, &mut parent);
            let rj = find(j, &mut parent);
            if ri != rj {
                parent[rj] = ri;
            }
        }
    }
    let mut roots = std::collections::BTreeSet::new();
    for i in 0..n {
        roots.insert(find(i, &mut parent));
    }
    roots.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::canonical_pair;

    pub fn canonical_pair_set(edges: &[HyperlaneEdge]) -> BTreeMap<(String, String), ()> {
        edges
            .iter()
            .map(|edge| canonical_pair(&edge.from, &edge.to))
            .map(|pair| (pair, ()))
            .collect()
    }
}
