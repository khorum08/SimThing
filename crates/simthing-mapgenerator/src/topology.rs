//! Bounded producer-side hyperlane topology (PR6 — declarative endpoint pairs only).
//!
//! Producer-side hyperlane **adjacency heuristic**. **Follow-up (STEAD-CONTRACT-0 objection):** this
//! currently uses *lowered index-order* positions, but since STEAD-PRIVILEGE-0 the closed `mapgen_lattice`
//! honors the **emitted authored** positions as the authoritative structural `(col,row)` layout — so this
//! heuristic should migrate to the authored positions for spatial fidelity (a producer behavior change
//! with its own tests). Until then, the emitted `add_hyperlane` pairs still lower correctly (the lowerer
//! classifies them on the structural grid); only the producer's *which-pairs* heuristic is on stale coords.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::pair_candidates::{collect_pairs_within_chebyshev, PRODUCER_PAIR_CANDIDATE_CAP};
use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::strategy::{PlacedSystemSeed, ShapePlacement};

/// Default per-node fanout cap aligned with closed MapGen PR3 link fanout.
pub const DEFAULT_MAX_PER_NODE_FANOUT: u32 = 4;

/// One declarative hyperlane edge between generated system ids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperlaneEdge {
    pub from: String,
    pub to: String,
}

/// Deterministic hyperlane topology output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperlaneTopology {
    pub edges: Vec<HyperlaneEdge>,
}

/// Bounded hyperlane generation options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperlaneOptions {
    pub fixture_lattice_edge: u32,
    pub max_hyperlane_distance: u32,
    pub min_edge_count: u32,
    pub max_edge_count: u32,
    pub target_edge_count: u32,
    pub random_hyperlanes: bool,
    pub prevent_pairs: Vec<(String, String)>,
    pub max_per_node_fanout: u32,
}

impl HyperlaneOptions {
    pub fn from_params(params: &MapGeneratorParams, fixture_lattice_edge: u32) -> Self {
        let min = params.hyperlane.num_hyperlanes_min;
        let max = params.hyperlane.num_hyperlanes_max;
        let target = params.hyperlane.num_hyperlanes_default.clamp(min, max);
        let max_dist = params.hyperlane.max_hyperlane_distance.floor().max(1.0) as u32;
        Self {
            fixture_lattice_edge: fixture_lattice_edge.max(1),
            max_hyperlane_distance: max_dist,
            min_edge_count: min,
            max_edge_count: max,
            target_edge_count: target,
            random_hyperlanes: params.hyperlane.random_hyperlanes,
            prevent_pairs: Vec::new(),
            max_per_node_fanout: DEFAULT_MAX_PER_NODE_FANOUT,
        }
    }
}

/// Bounded hyperlane generation report (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HyperlaneGenerationReport {
    pub candidate_count: u32,
    pub selected_count: u32,
    pub rejected_prevent: u32,
    pub rejected_fanout: u32,
    pub examined_pairs: u64,
    pub candidate_cap_hit: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HyperlaneError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("fixture lattice edge must be positive")]
    InvalidFixtureEdge,
    #[error(
        "hyperlane edge counts invalid: min={min_edge_count}, max={max_edge_count}, target={target_edge_count}"
    )]
    InvalidEdgeCounts {
        min_edge_count: u32,
        max_edge_count: u32,
        target_edge_count: u32,
    },
    #[error("max_per_node_fanout must be positive")]
    InvalidFanoutCap,
    #[error(
        "selected hyperlane count {selected_count} is below required minimum {min_edge_count}"
    )]
    UnsatisfiedMinEdgeCount {
        selected_count: u32,
        min_edge_count: u32,
    },
    #[error("unknown system id '{id}'")]
    UnknownSystemId { id: String },
    #[error("hyperlane self-link for system '{id}'")]
    SelfLink { id: String },
    #[error("duplicate hyperlane edge ({from}, {to})")]
    DuplicateEdge { from: String, to: String },
    #[error(
        "fixture lattice edge overflow for {system_count} systems (capacity would exceed u32::MAX)"
    )]
    FixtureLatticeOverflow { system_count: usize },
    #[error(
        "hyperlane candidate cap {cap} exceeded before selection (examined {examined_pairs} pairs)"
    )]
    CandidateCapExceeded { cap: usize, examined_pairs: u64 },
}

/// Fail-closed validation for directly constructed [`HyperlaneOptions`].
pub fn validate_hyperlane_options(options: &HyperlaneOptions) -> Result<(), HyperlaneError> {
    if options.fixture_lattice_edge == 0 {
        return Err(HyperlaneError::InvalidFixtureEdge);
    }
    if options.min_edge_count > options.max_edge_count {
        return Err(HyperlaneError::InvalidEdgeCounts {
            min_edge_count: options.min_edge_count,
            max_edge_count: options.max_edge_count,
            target_edge_count: options.target_edge_count,
        });
    }
    if options.max_per_node_fanout == 0 {
        return Err(HyperlaneError::InvalidFanoutCap);
    }
    Ok(())
}

/// Smallest square edge whose capacity fits `system_count` lowered grid slots.
pub fn fixture_lattice_edge_for_system_count(system_count: usize) -> Result<u32, HyperlaneError> {
    if system_count == 0 {
        return Ok(1);
    }
    let mut edge = 1u64;
    while edge.saturating_mul(edge) < system_count as u64 {
        edge += 1;
        if edge > u32::MAX as u64 {
            return Err(HyperlaneError::FixtureLatticeOverflow { system_count });
        }
    }
    Ok(edge as u32)
}

/// Lowered grid row/col for a system index (matches closed `assign_system_placements`).
pub fn lowered_grid_position(index: usize, fixture_lattice_edge: u32) -> (u32, u32) {
    let index = index as u32;
    let edge = fixture_lattice_edge.max(1);
    (index / edge, index % edge)
}

/// Chebyshev distance on the lowered index-order grid (producer-side heuristic only).
pub fn grid_chebyshev_distance(left: (u32, u32), right: (u32, u32)) -> u32 {
    left.0.abs_diff(right.0).max(left.1.abs_diff(right.1))
}

pub fn canonical_pair(from: &str, to: &str) -> (String, String) {
    if from <= to {
        (from.to_string(), to.to_string())
    } else {
        (to.to_string(), from.to_string())
    }
}

pub fn system_id_scalar(system: &PlacedSystemSeed) -> String {
    system.id.to_string()
}

/// Generate bounded hyperlane edges from in-memory placements.
pub fn generate_hyperlane_topology(
    placement: &ShapePlacement,
    options: &HyperlaneOptions,
    rng: &mut MapGenRng,
) -> Result<(HyperlaneTopology, HyperlaneGenerationReport), HyperlaneError> {
    if placement.systems.is_empty() {
        return Err(HyperlaneError::EmptyPlacement);
    }
    validate_hyperlane_options(options)?;

    let ids: Vec<String> = placement.systems.iter().map(system_id_scalar).collect();
    let positions: Vec<(u32, u32)> = (0..placement.systems.len())
        .map(|index| lowered_grid_position(index, options.fixture_lattice_edge))
        .collect();

    let prevent: BTreeSet<(String, String)> = options
        .prevent_pairs
        .iter()
        .map(|(from, to)| canonical_pair(from, to))
        .collect();

    let mut candidates = Vec::new();
    let mut rejected_prevent = 0u32;
    let (pair_rows, pair_stats) =
        collect_pairs_within_chebyshev(&positions, options.max_hyperlane_distance);
    if pair_stats.capped {
        return Err(HyperlaneError::CandidateCapExceeded {
            cap: PRODUCER_PAIR_CANDIDATE_CAP,
            examined_pairs: pair_stats.examined_pairs,
        });
    }
    for (distance, left, right) in pair_rows {
        let pair = canonical_pair(&ids[left], &ids[right]);
        if prevent.contains(&pair) {
            rejected_prevent += 1;
            continue;
        }
        if candidates
            .iter()
            .any(|(_, from, to)| from == &pair.0 && to == &pair.1)
        {
            continue;
        }
        candidates.push((distance, pair.0, pair.1));
    }

    let candidate_count = candidates.len() as u32;

    candidates.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    let mut ordered: Vec<(String, String)> = candidates
        .into_iter()
        .map(|(_, from, to)| (from, to))
        .collect();

    if options.random_hyperlanes {
        fisher_yates_shuffle(&mut ordered, rng);
    }

    let target = options
        .target_edge_count
        .clamp(options.min_edge_count, options.max_edge_count) as usize;

    let mut fanout: BTreeMap<String, u32> = BTreeMap::new();
    let mut selected = Vec::new();
    let mut rejected_fanout = 0u32;
    let mut seen_selected: BTreeSet<(String, String)> = BTreeSet::new();
    for (from, to) in ordered {
        if selected.len() >= target {
            break;
        }
        let pair = canonical_pair(&from, &to);
        if !seen_selected.insert(pair) {
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
        selected.push(HyperlaneEdge { from, to });
    }

    let selected_count = selected.len() as u32;
    if selected_count < options.min_edge_count {
        return Err(HyperlaneError::UnsatisfiedMinEdgeCount {
            selected_count,
            min_edge_count: options.min_edge_count,
        });
    }

    let report = HyperlaneGenerationReport {
        candidate_count,
        selected_count,
        rejected_prevent,
        rejected_fanout,
        examined_pairs: pair_stats.examined_pairs,
        candidate_cap_hit: pair_stats.capped,
    };

    Ok((HyperlaneTopology { edges: selected }, report))
}

/// Validate a hyperlane edge list before emission.
pub fn validate_hyperlane_edges(
    placement: &ShapePlacement,
    edges: &[(String, String)],
) -> Result<(), HyperlaneError> {
    let ids: BTreeSet<String> = placement.systems.iter().map(system_id_scalar).collect();
    let mut seen = BTreeSet::new();
    for (from, to) in edges {
        if from == to {
            return Err(HyperlaneError::SelfLink { id: from.clone() });
        }
        if !ids.contains(from) {
            return Err(HyperlaneError::UnknownSystemId { id: from.clone() });
        }
        if !ids.contains(to) {
            return Err(HyperlaneError::UnknownSystemId { id: to.clone() });
        }
        let pair = canonical_pair(from, to);
        if !seen.insert(pair.clone()) {
            return Err(HyperlaneError::DuplicateEdge {
                from: pair.0,
                to: pair.1,
            });
        }
    }
    Ok(())
}

fn fisher_yates_shuffle(items: &mut [(String, String)], rng: &mut MapGenRng) {
    if items.len() <= 1 {
        return;
    }
    for index in (1..items.len()).rev() {
        let swap_with = rng.gen_index(index as u32 + 1) as usize;
        items.swap(index, swap_with);
    }
}
