//! Bounded producer-side partition assignment and cross-partition bridge selection (PR7).
//!
//! Partition identity and bridge structure are producer reports only — emission uses existing
//! `add_hyperlane` endpoint pairs. No route/path/predecessor/movement semantics.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use thiserror::Error;

use crate::params::{MapGeneratorParams, PartitionMethod};
use crate::rng::MapGenRng;
use crate::strategy::ShapePlacement;
use crate::topology::{
    canonical_pair, grid_chebyshev_distance, lowered_grid_position, system_id_scalar,
    HyperlaneEdge, DEFAULT_MAX_PER_NODE_FANOUT,
};

/// Producer-side partition kind (reporting only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionKind {
    HomeSystemPartition,
    OpenSpacePartition,
    ClusterPartition,
}

/// Stable partition identifier (producer-side only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartitionId(pub u32);

/// One system's partition bucket assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionAssignment {
    pub system_id: String,
    pub partition_id: PartitionId,
    pub kind: PartitionKind,
}

/// One cross-partition bridge endpoint pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeEdge {
    pub from: String,
    pub to: String,
    pub from_partition: PartitionId,
    pub to_partition: PartitionId,
}

impl BridgeEdge {
    pub fn to_hyperlane_edge(&self) -> HyperlaneEdge {
        HyperlaneEdge {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

/// Bounded partition/bridge generation options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionOptions {
    pub fixture_lattice_edge: u32,
    pub home_system_partitions: u32,
    pub open_space_partitions: u32,
    pub min_systems: u32,
    pub max_systems: u32,
    pub min_bridges: u32,
    pub max_bridges: u32,
    pub method: PartitionMethod,
    pub max_per_node_fanout: u32,
}

impl PartitionOptions {
    pub fn from_params(params: &MapGeneratorParams, fixture_lattice_edge: u32) -> Self {
        Self {
            fixture_lattice_edge: fixture_lattice_edge.max(1),
            home_system_partitions: params.partitioning.home_system_partitions,
            open_space_partitions: params.partitioning.open_space_partitions,
            min_systems: params.partitioning.partition_min_systems,
            max_systems: params.partitioning.partition_max_systems,
            min_bridges: params.partitioning.partition_min_bridges,
            max_bridges: params.partitioning.partition_max_bridges,
            method: params.partitioning.partition_method,
            max_per_node_fanout: DEFAULT_MAX_PER_NODE_FANOUT,
        }
    }

    pub fn partition_count(&self) -> u32 {
        self.home_system_partitions + self.open_space_partitions
    }
}

/// Bounded partition/bridge generation report (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartitionReport {
    pub partition_count: u32,
    pub assignment_count: u32,
    pub bridge_candidate_count: u32,
    pub bridge_count: u32,
    pub rejected_fanout: u32,
    pub rejected_duplicate: u32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PartitionError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("fixture lattice edge must be positive")]
    InvalidFixtureEdge,
    #[error("max_per_node_fanout must be positive")]
    InvalidFanoutCap,
    #[error("partition system bounds invalid: min={min_systems}, max={max_systems}")]
    InvalidSystemBounds { min_systems: u32, max_systems: u32 },
    #[error("partition bridge bounds invalid: min={min_bridges}, max={max_bridges}")]
    InvalidBridgeBounds { min_bridges: u32, max_bridges: u32 },
    #[error("partition count must be positive")]
    ZeroPartitionCount,
    #[error(
        "cannot satisfy partition structure for {system_count} systems across {partition_count} partitions with min={min_systems} max={max_systems}"
    )]
    UnsatisfiedPartitionStructure {
        system_count: u32,
        partition_count: u32,
        min_systems: u32,
        max_systems: u32,
    },
    #[error("selected bridge count {selected_count} is below required minimum {min_bridges}")]
    UnsatisfiedBridgeCount {
        selected_count: u32,
        min_bridges: u32,
    },
    #[error("unknown system id '{id}'")]
    UnknownSystemId { id: String },
    #[error("bridge self-link for system '{id}'")]
    SelfLink { id: String },
    #[error("duplicate bridge edge ({from}, {to})")]
    DuplicateEdge { from: String, to: String },
}

pub fn validate_partition_options(options: &PartitionOptions) -> Result<(), PartitionError> {
    if options.fixture_lattice_edge == 0 {
        return Err(PartitionError::InvalidFixtureEdge);
    }
    if options.max_per_node_fanout == 0 {
        return Err(PartitionError::InvalidFanoutCap);
    }
    if options.min_systems > options.max_systems {
        return Err(PartitionError::InvalidSystemBounds {
            min_systems: options.min_systems,
            max_systems: options.max_systems,
        });
    }
    if options.min_bridges > options.max_bridges {
        return Err(PartitionError::InvalidBridgeBounds {
            min_bridges: options.min_bridges,
            max_bridges: options.max_bridges,
        });
    }
    Ok(())
}

/// Deterministically assign placed systems to home/open partition buckets.
pub fn assign_partitions(
    placement: &ShapePlacement,
    options: &PartitionOptions,
) -> Result<(Vec<PartitionAssignment>, PartitionReport), PartitionError> {
    if placement.systems.is_empty() {
        return Err(PartitionError::EmptyPlacement);
    }
    validate_partition_options(options)?;

    let partition_count = options.partition_count();
    if partition_count == 0 {
        return Err(PartitionError::ZeroPartitionCount);
    }

    let system_count = placement.systems.len() as u32;
    if system_count < partition_count * options.min_systems {
        return Err(PartitionError::UnsatisfiedPartitionStructure {
            system_count,
            partition_count,
            min_systems: options.min_systems,
            max_systems: options.max_systems,
        });
    }
    if system_count > partition_count * options.max_systems {
        return Err(PartitionError::UnsatisfiedPartitionStructure {
            system_count,
            partition_count,
            min_systems: options.min_systems,
            max_systems: options.max_systems,
        });
    }

    let ordered = ordered_system_indices(placement, options);
    let chunks = split_ordered_indices(&ordered, partition_count as usize);

    let mut assignments = Vec::with_capacity(placement.systems.len());
    for (partition_index, chunk) in chunks.into_iter().enumerate() {
        let kind = if (partition_index as u32) < options.home_system_partitions {
            PartitionKind::HomeSystemPartition
        } else {
            PartitionKind::OpenSpacePartition
        };
        let partition_id = PartitionId(partition_index as u32);
        for system_index in chunk {
            assignments.push(PartitionAssignment {
                system_id: system_id_scalar(&placement.systems[system_index]),
                partition_id,
                kind,
            });
        }
    }

    let report = PartitionReport {
        partition_count,
        assignment_count: assignments.len() as u32,
        ..PartitionReport::default()
    };
    Ok((assignments, report))
}

/// Select bounded cross-partition bridge endpoint pairs for existing `add_hyperlane` emission.
pub fn generate_partition_bridges(
    placement: &ShapePlacement,
    assignments: &[PartitionAssignment],
    options: &PartitionOptions,
    existing_edges: &[(String, String)],
    rng: &mut MapGenRng,
) -> Result<(Vec<BridgeEdge>, PartitionReport), PartitionError> {
    if placement.systems.is_empty() {
        return Err(PartitionError::EmptyPlacement);
    }
    validate_partition_options(options)?;

    if options.max_bridges == 0 {
        return Ok((Vec::new(), PartitionReport::default()));
    }

    let partition_by_system: BTreeMap<String, (PartitionId, PartitionKind)> = assignments
        .iter()
        .map(|assignment| {
            (
                assignment.system_id.clone(),
                (assignment.partition_id, assignment.kind),
            )
        })
        .collect();

    let ids: Vec<String> = placement.systems.iter().map(system_id_scalar).collect();
    let positions: Vec<(u32, u32)> = (0..placement.systems.len())
        .map(|index| lowered_grid_position(index, options.fixture_lattice_edge))
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
    for left in 0..ids.len() {
        for right in left + 1..ids.len() {
            let left_partition = partition_by_system
                .get(&ids[left])
                .map(|(id, _)| *id)
                .unwrap_or(PartitionId(0));
            let right_partition = partition_by_system
                .get(&ids[right])
                .map(|(id, _)| *id)
                .unwrap_or(PartitionId(0));
            if left_partition == right_partition {
                continue;
            }
            let distance = grid_chebyshev_distance(positions[left], positions[right]);
            let pair = canonical_pair(&ids[left], &ids[right]);
            candidates.push((distance, pair.0, pair.1, left_partition, right_partition));
        }
    }

    let bridge_candidate_count = candidates.len() as u32;
    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    let mut ordered: Vec<(String, String, PartitionId, PartitionId)> = candidates
        .into_iter()
        .map(|(_, from, to, from_partition, to_partition)| (from, to, from_partition, to_partition))
        .collect();
    fisher_yates_bridge_shuffle(&mut ordered, rng);

    let target = options.max_bridges as usize;
    let mut selected = Vec::new();
    let mut rejected_fanout = 0u32;
    let mut rejected_duplicate = 0u32;
    let mut seen_pairs: BTreeSet<(String, String)> = BTreeSet::new();

    for (from, to, from_partition, to_partition) in ordered {
        if selected.len() >= target {
            break;
        }
        let pair = canonical_pair(&from, &to);
        if occupied.contains(&pair) || !seen_pairs.insert(pair.clone()) {
            rejected_duplicate += 1;
            continue;
        }
        if fanout.get(&from).copied().unwrap_or(0) >= options.max_per_node_fanout
            || fanout.get(&to).copied().unwrap_or(0) >= options.max_per_node_fanout
        {
            rejected_fanout += 1;
            continue;
        }
        *fanout.entry(from.clone()).or_insert(0) += 1;
        *fanout.entry(to.clone()).or_insert(0) += 1;
        selected.push(BridgeEdge {
            from,
            to,
            from_partition,
            to_partition,
        });
    }

    let bridge_count = selected.len() as u32;
    if bridge_count < options.min_bridges {
        return Err(PartitionError::UnsatisfiedBridgeCount {
            selected_count: bridge_count,
            min_bridges: options.min_bridges,
        });
    }

    let report = PartitionReport {
        partition_count: options.partition_count(),
        assignment_count: assignments.len() as u32,
        bridge_candidate_count,
        bridge_count,
        rejected_fanout,
        rejected_duplicate,
    };
    Ok((selected, report))
}

pub fn validate_bridge_edges(
    placement: &ShapePlacement,
    edges: &[BridgeEdge],
) -> Result<(), PartitionError> {
    let ids: BTreeSet<String> = placement.systems.iter().map(system_id_scalar).collect();
    let mut seen = BTreeSet::new();
    for edge in edges {
        if edge.from == edge.to {
            return Err(PartitionError::SelfLink {
                id: edge.from.clone(),
            });
        }
        if !ids.contains(&edge.from) {
            return Err(PartitionError::UnknownSystemId {
                id: edge.from.clone(),
            });
        }
        if !ids.contains(&edge.to) {
            return Err(PartitionError::UnknownSystemId {
                id: edge.to.clone(),
            });
        }
        let pair = canonical_pair(&edge.from, &edge.to);
        if !seen.insert(pair.clone()) {
            return Err(PartitionError::DuplicateEdge {
                from: pair.0,
                to: pair.1,
            });
        }
    }
    Ok(())
}

fn ordered_system_indices(placement: &ShapePlacement, options: &PartitionOptions) -> Vec<usize> {
    let count = placement.systems.len();
    let positions: Vec<(u32, u32)> = (0..count)
        .map(|index| lowered_grid_position(index, options.fixture_lattice_edge))
        .collect();

    let mut adjacency = vec![Vec::new(); count];
    for left in 0..count {
        for right in left + 1..count {
            if grid_chebyshev_distance(positions[left], positions[right]) <= 2 {
                adjacency[left].push(right);
                adjacency[right].push(left);
            }
        }
    }
    for neighbors in &mut adjacency {
        neighbors.sort_unstable();
    }

    match options.method {
        PartitionMethod::BreadthFirst => breadth_first_order(count, &adjacency),
        PartitionMethod::DepthFirst => depth_first_order(count, &adjacency),
    }
}

fn breadth_first_order(count: usize, adjacency: &[Vec<usize>]) -> Vec<usize> {
    let mut visited = vec![false; count];
    let mut order = Vec::with_capacity(count);
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    while let Some(index) = queue.pop_front() {
        order.push(index);
        for &neighbor in &adjacency[index] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    for index in 0..count {
        if !visited[index] {
            order.push(index);
        }
    }
    order
}

fn depth_first_order(count: usize, adjacency: &[Vec<usize>]) -> Vec<usize> {
    let mut visited = vec![false; count];
    let mut order = Vec::with_capacity(count);
    depth_first_visit(0, adjacency, &mut visited, &mut order);
    for index in 0..count {
        if !visited[index] {
            depth_first_visit(index, adjacency, &mut visited, &mut order);
        }
    }
    order
}

fn depth_first_visit(
    index: usize,
    adjacency: &[Vec<usize>],
    visited: &mut [bool],
    order: &mut Vec<usize>,
) {
    visited[index] = true;
    order.push(index);
    for &neighbor in &adjacency[index] {
        if !visited[neighbor] {
            depth_first_visit(neighbor, adjacency, visited, order);
        }
    }
}

fn split_ordered_indices(ordered: &[usize], partitions: usize) -> Vec<Vec<usize>> {
    let total = ordered.len();
    let base = total / partitions;
    let remainder = total % partitions;
    let mut chunks = Vec::with_capacity(partitions);
    let mut cursor = 0usize;
    for partition_index in 0..partitions {
        let size = base + usize::from(partition_index < remainder);
        chunks.push(ordered[cursor..cursor + size].to_vec());
        cursor += size;
    }
    chunks
}

fn fisher_yates_bridge_shuffle(
    items: &mut [(String, String, PartitionId, PartitionId)],
    rng: &mut MapGenRng,
) {
    if items.len() <= 1 {
        return;
    }
    for index in (1..items.len()).rev() {
        let swap_with = rng.gen_index(index as u32 + 1) as usize;
        items.swap(index, swap_with);
    }
}
