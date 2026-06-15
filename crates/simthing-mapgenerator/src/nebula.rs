//! Producer-side nebula field placement (PR9 — declarative feedstock only).
//!
//! Deterministic integer-lattice placement with Chebyshev min-distance between centers.
//! No Euclidean authority, no runtime field execution, no GPU work.

use thiserror::Error;

use crate::lattice::{LatticeCoord, SquareLattice};
use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::strategy::ShapePlacement;
use crate::topology::{grid_chebyshev_distance, system_id_scalar};

/// One producer-side nebula declaration (maps to closed `nebula = { name radius }` feedstock).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NebulaField {
    pub index: u32,
    pub center: LatticeCoord,
    pub radius_cells: u32,
    pub name: String,
    pub affected_system_ids: Vec<String>,
}

/// Bounded nebula placement options derived from high-level params.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NebulaOptions {
    pub count: u32,
    pub radius_cells: u32,
    pub min_center_distance: u32,
}

impl NebulaOptions {
    pub fn from_params(params: &MapGeneratorParams) -> Self {
        Self {
            count: params.nebula.num_nebulas,
            radius_cells: params.nebula.nebula_size.round().max(1.0) as u32,
            min_center_distance: params.nebula.nebula_min_dist.round() as u32,
        }
    }
}

/// Compact nebula placement report (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NebulaReport {
    pub requested_count: u32,
    pub placed_count: u32,
    pub rejected_min_distance: u32,
    pub affected_system_count: u32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NebulaError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("nebula count must be bounded")]
    ZeroCountWhenRequested,
    #[error(
        "cannot place {requested} nebulas with min center distance {min_distance} on {candidate_count} candidates"
    )]
    ImpossibleRequest {
        requested: u32,
        min_distance: u32,
        candidate_count: usize,
    },
}

/// Deterministically place bounded nebula centers on the placed-system lattice.
pub fn place_nebulas(
    placement: &ShapePlacement,
    lattice: &SquareLattice,
    options: NebulaOptions,
    rng: &mut MapGenRng,
) -> Result<(Vec<NebulaField>, NebulaReport), NebulaError> {
    if options.count == 0 {
        return Ok((Vec::new(), NebulaReport::default()));
    }
    if placement.systems.is_empty() {
        return Err(NebulaError::EmptyPlacement);
    }

    let mut candidates = collect_candidate_centers(placement, lattice);
    let candidate_count = candidates.len();
    shuffle_coords(&mut candidates, rng);

    let mut selected: Vec<LatticeCoord> = Vec::new();
    let mut rejected_min_distance = 0_u32;
    'outer: for center in candidates {
        for existing in &selected {
            if grid_chebyshev_distance((center.col, center.row), (existing.col, existing.row))
                < options.min_center_distance
            {
                rejected_min_distance += 1;
                continue 'outer;
            }
        }
        selected.push(center);
        if selected.len() as u32 >= options.count {
            break;
        }
    }

    if (selected.len() as u32) < options.count {
        return Err(NebulaError::ImpossibleRequest {
            requested: options.count,
            min_distance: options.min_center_distance,
            candidate_count,
        });
    }

    selected.sort_by_key(|coord| (coord.col, coord.row));
    let mut nebulas = Vec::with_capacity(selected.len());
    let mut affected_system_count = 0_usize;
    for (index, center) in selected.into_iter().enumerate() {
        let affected_system_ids = affected_systems(placement, center, options.radius_cells);
        affected_system_count += affected_system_ids.len();
        nebulas.push(NebulaField {
            index: index as u32,
            center,
            radius_cells: options.radius_cells,
            name: format!("generated_nebula_{index}"),
            affected_system_ids,
        });
    }

    let placed_count = nebulas.len() as u32;
    Ok((
        nebulas,
        NebulaReport {
            requested_count: options.count,
            placed_count,
            rejected_min_distance,
            affected_system_count: affected_system_count as u32,
        },
    ))
}

fn collect_candidate_centers(
    placement: &ShapePlacement,
    lattice: &SquareLattice,
) -> Vec<LatticeCoord> {
    let mut candidates: Vec<LatticeCoord> = placement
        .systems
        .iter()
        .map(|system| system.coord)
        .collect();
    let center = LatticeCoord {
        col: lattice.edge().saturating_sub(1) / 2,
        row: lattice.edge().saturating_sub(1) / 2,
    };
    candidates.push(center);
    candidates.sort_by_key(|coord| (coord.col, coord.row));
    candidates.dedup();
    candidates
}

fn shuffle_coords(coords: &mut [LatticeCoord], rng: &mut MapGenRng) {
    for i in (1..coords.len()).rev() {
        let j = rng.gen_index((i + 1) as u32) as usize;
        coords.swap(i, j);
    }
}

fn affected_systems(
    placement: &ShapePlacement,
    center: LatticeCoord,
    radius_cells: u32,
) -> Vec<String> {
    let mut affected = Vec::new();
    for system in &placement.systems {
        if grid_chebyshev_distance(
            (center.col, center.row),
            (system.coord.col, system.coord.row),
        ) <= radius_cells
        {
            affected.push(system_id_scalar(system));
        }
    }
    affected.sort();
    affected.dedup();
    affected
}

/// Apply cluster-bucket initializer refs to systems assigned to non-anchor clusters.
pub fn apply_cluster_initializer_buckets(
    placement: &mut ShapePlacement,
    assignments: &[crate::cluster::ClusterAssignment],
    cluster_bucket: &str,
) {
    if cluster_bucket.is_empty() {
        return;
    }
    for system in &mut placement.systems {
        let id = system_id_scalar(system);
        if assignments
            .iter()
            .any(|assignment| assignment.system_id == id && assignment.cluster_id.0 > 0)
        {
            system.bucket = Some(cluster_bucket.to_string());
        }
    }
}
