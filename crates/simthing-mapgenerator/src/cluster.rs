//! Bounded producer-side cluster assignment (PR7 — grouping metadata only).
//!
//! Cluster identity is producer-side only. Cross-cluster couplings are emitted as existing
//! `add_hyperlane` endpoint pairs via partition bridge selection when clusters differ.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::pair_candidates::collect_farthest_pairs_with_filter;
use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::strategy::{PlacedSystemSeed, ShapePlacement};
use crate::topology::{
    canonical_pair, grid_chebyshev_distance, lowered_grid_position, system_id_scalar, HyperlaneEdge,
};

/// Stable cluster identifier (producer-side only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClusterId(pub u32);

/// One system's cluster bucket assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterAssignment {
    pub system_id: String,
    pub cluster_id: ClusterId,
}

/// Bounded cluster assignment options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterOptions {
    pub target_cluster_count: u32,
    pub cluster_radius: u32,
    pub cluster_distance_from_core: u32,
}

impl ClusterOptions {
    pub fn from_params(params: &MapGeneratorParams) -> Self {
        let requested = params
            .clustering
            .cluster_count
            .unwrap_or(params.clustering.cluster_count_value.floor() as u32);
        let capped = params
            .clustering
            .cluster_count_max
            .map(|max| requested.min(max))
            .unwrap_or(requested);
        Self {
            target_cluster_count: capped.max(1),
            cluster_radius: params.clustering.cluster_radius.floor().max(1.0) as u32,
            cluster_distance_from_core: params
                .clustering
                .cluster_distance_from_core
                .floor()
                .max(0.0) as u32,
        }
    }
}

/// Bounded cluster assignment report (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClusterReport {
    pub target_cluster_count: u32,
    pub assigned_cluster_count: u32,
    pub rejected_out_of_radius: u32,
    pub cluster_bridge_count: u32,
    pub examined_pairs: u64,
    pub candidate_cap_hit: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ClusterError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("target cluster count must be positive")]
    ZeroClusterCount,
    #[error("cannot assign {target_cluster_count} clusters to {system_count} systems")]
    UnsatisfiedClusterCount {
        target_cluster_count: u32,
        system_count: u32,
    },
    #[error("system '{system_id}' is beyond cluster radius {cluster_radius} from all anchors")]
    UnsatisfiedClusterRadius {
        system_id: String,
        cluster_radius: u32,
    },
    #[error(
        "selected cluster bridge count {selected_count} is below required minimum {min_bridges}"
    )]
    UnsatisfiedClusterBridgeCount {
        selected_count: u32,
        min_bridges: u32,
    },
}

pub fn validate_cluster_options(options: &ClusterOptions) -> Result<(), ClusterError> {
    if options.target_cluster_count == 0 {
        return Err(ClusterError::ZeroClusterCount);
    }
    Ok(())
}

/// Deterministically group placed systems into bounded clusters on integer lattice coords.
pub fn assign_clusters(
    placement: &ShapePlacement,
    options: &ClusterOptions,
) -> Result<(Vec<ClusterAssignment>, ClusterReport), ClusterError> {
    if placement.systems.is_empty() {
        return Err(ClusterError::EmptyPlacement);
    }
    validate_cluster_options(options)?;

    let system_count = placement.systems.len() as u32;
    if options.target_cluster_count > system_count {
        return Err(ClusterError::UnsatisfiedClusterCount {
            target_cluster_count: options.target_cluster_count,
            system_count,
        });
    }

    let mut ordered: Vec<&PlacedSystemSeed> = placement.systems.iter().collect();
    ordered.sort_by_key(|system| system.id);

    let anchors: Vec<(ClusterId, (u32, u32))> = ordered
        .iter()
        .take(options.target_cluster_count as usize)
        .enumerate()
        .map(|(index, system)| (ClusterId(index as u32), lattice_coord(system)))
        .collect();

    let mut assignments = Vec::with_capacity(placement.systems.len());
    let mut rejected_out_of_radius = 0u32;
    let mut used_clusters = BTreeSet::new();

    for system in ordered {
        let position = lattice_coord(system);
        let mut best: Option<(ClusterId, u32)> = None;
        for (cluster_id, anchor) in &anchors {
            let distance = grid_chebyshev_distance(position, *anchor);
            if distance > options.cluster_radius {
                continue;
            }
            if best.is_none() || distance < best.unwrap().1 {
                best = Some((*cluster_id, distance));
            } else if best.unwrap().1 == distance && *cluster_id < best.unwrap().0 {
                best = Some((*cluster_id, distance));
            }
        }

        let Some((cluster_id, _)) = best else {
            rejected_out_of_radius += 1;
            return Err(ClusterError::UnsatisfiedClusterRadius {
                system_id: system_id_scalar(system),
                cluster_radius: options.cluster_radius,
            });
        };

        used_clusters.insert(cluster_id);
        assignments.push(ClusterAssignment {
            system_id: system_id_scalar(system),
            cluster_id,
        });
    }

    let _ = options.cluster_distance_from_core;

    Ok((
        assignments,
        ClusterReport {
            target_cluster_count: options.target_cluster_count,
            assigned_cluster_count: used_clusters.len() as u32,
            rejected_out_of_radius,
            cluster_bridge_count: 0,
            examined_pairs: 0,
            candidate_cap_hit: false,
        },
    ))
}

fn lattice_coord(system: &PlacedSystemSeed) -> (u32, u32) {
    (system.coord.col, system.coord.row)
}

/// One cross-cluster bridge endpoint pair (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterBridgeEdge {
    pub from: String,
    pub to: String,
    pub from_cluster: ClusterId,
    pub to_cluster: ClusterId,
}

impl ClusterBridgeEdge {
    pub fn to_hyperlane_edge(&self) -> HyperlaneEdge {
        HyperlaneEdge {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

/// Select bounded cross-cluster bridge endpoint pairs for existing `add_hyperlane` emission.
pub fn generate_cluster_bridges(
    placement: &ShapePlacement,
    assignments: &[ClusterAssignment],
    fixture_lattice_edge: u32,
    min_bridges: u32,
    max_bridges: u32,
    max_per_node_fanout: u32,
    existing_edges: &[(String, String)],
    rng: &mut MapGenRng,
) -> Result<(Vec<ClusterBridgeEdge>, ClusterReport), ClusterError> {
    if placement.systems.is_empty() {
        return Err(ClusterError::EmptyPlacement);
    }
    if max_bridges == 0 {
        return Ok((Vec::new(), ClusterReport::default()));
    }
    if max_per_node_fanout == 0 {
        return Err(ClusterError::ZeroClusterCount);
    }

    let cluster_by_system: BTreeMap<String, ClusterId> = assignments
        .iter()
        .map(|assignment| (assignment.system_id.clone(), assignment.cluster_id))
        .collect();

    let ids: Vec<String> = placement.systems.iter().map(system_id_scalar).collect();
    let positions: Vec<(u32, u32)> = (0..placement.systems.len())
        .map(|index| lowered_grid_position(index, fixture_lattice_edge.max(1)))
        .collect();

    let occupied: BTreeSet<(String, String)> = existing_edges
        .iter()
        .map(|(from, to)| canonical_pair(from, to))
        .collect();

    let mut fanout: BTreeMap<String, u32> = BTreeMap::new();
    for (from, to) in existing_edges {
        *fanout.entry(from.clone()).or_insert(0) += 1;
        *fanout.entry(to.clone()).or_insert(0) += 1;
    }

    let mut candidates = Vec::new();
    let (pair_rows, pair_stats) =
        collect_farthest_pairs_with_filter(&positions, |left, right, _distance| {
            let left_cluster = cluster_by_system
                .get(&ids[left])
                .copied()
                .unwrap_or(ClusterId(0));
            let right_cluster = cluster_by_system
                .get(&ids[right])
                .copied()
                .unwrap_or(ClusterId(0));
            left_cluster != right_cluster
        });
    for (distance, left, right) in pair_rows {
        let left_cluster = cluster_by_system
            .get(&ids[left])
            .copied()
            .unwrap_or(ClusterId(0));
        let right_cluster = cluster_by_system
            .get(&ids[right])
            .copied()
            .unwrap_or(ClusterId(0));
        let pair = canonical_pair(&ids[left], &ids[right]);
        candidates.push((distance, pair.0, pair.1, left_cluster, right_cluster));
    }

    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    let mut ordered: Vec<(String, String, ClusterId, ClusterId)> = candidates
        .into_iter()
        .map(|(_, from, to, from_cluster, to_cluster)| (from, to, from_cluster, to_cluster))
        .collect();

    if ordered.len() > 1 {
        for index in (1..ordered.len()).rev() {
            let swap_with = rng.gen_index(index as u32 + 1) as usize;
            ordered.swap(index, swap_with);
        }
    }

    let target = max_bridges as usize;
    let mut selected = Vec::new();
    let mut seen_pairs = BTreeSet::new();
    for (from, to, from_cluster, to_cluster) in ordered {
        if selected.len() >= target {
            break;
        }
        let pair = canonical_pair(&from, &to);
        if occupied.contains(&pair) || !seen_pairs.insert(pair) {
            continue;
        }
        if fanout.get(&from).copied().unwrap_or(0) >= max_per_node_fanout
            || fanout.get(&to).copied().unwrap_or(0) >= max_per_node_fanout
        {
            continue;
        }
        *fanout.entry(from.clone()).or_insert(0) += 1;
        *fanout.entry(to.clone()).or_insert(0) += 1;
        selected.push(ClusterBridgeEdge {
            from,
            to,
            from_cluster,
            to_cluster,
        });
    }

    if (selected.len() as u32) < min_bridges {
        return Err(ClusterError::UnsatisfiedClusterBridgeCount {
            selected_count: selected.len() as u32,
            min_bridges,
        });
    }

    let bridge_count = selected.len() as u32;
    Ok((
        selected,
        ClusterReport {
            cluster_bridge_count: bridge_count,
            examined_pairs: pair_stats.examined_pairs,
            candidate_cap_hit: pair_stats.capped,
            ..ClusterReport::default()
        },
    ))
}
