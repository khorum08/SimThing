//! Bounded producer-side special-route selection (PR6b — wormhole/gateway endpoint pairs only).
//!
//! Special routes are long-range lane-coupling declarations represented as `add_hyperlane` endpoint
//! pairs at emission time. No route/path/predecessor/movement semantics.

use std::collections::{BTreeMap, BTreeSet};

use thiserror::Error;

use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::strategy::ShapePlacement;
use crate::topology::{
    canonical_pair, grid_chebyshev_distance, lowered_grid_position, system_id_scalar,
    HyperlaneEdge, DEFAULT_MAX_PER_NODE_FANOUT,
};

/// Producer-side special-route kind (reporting only — not emitted in scenario grammar).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRouteKind {
    WormholePair,
    Gateway,
}

/// One bounded special-route endpoint pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialRouteEdge {
    pub kind: SpecialRouteKind,
    pub from: String,
    pub to: String,
}

impl SpecialRouteEdge {
    pub fn to_hyperlane_edge(&self) -> HyperlaneEdge {
        HyperlaneEdge {
            from: self.from.clone(),
            to: self.to.clone(),
        }
    }
}

/// Deterministic special-route topology output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialRouteTopology {
    pub edges: Vec<SpecialRouteEdge>,
}

/// Bounded special-route generation options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialRouteOptions {
    pub fixture_lattice_edge: u32,
    pub num_wormhole_pairs: u32,
    pub num_gateways: u32,
    pub max_per_node_fanout: u32,
}

impl SpecialRouteOptions {
    pub fn from_params(params: &MapGeneratorParams, fixture_lattice_edge: u32) -> Self {
        Self {
            fixture_lattice_edge: fixture_lattice_edge.max(1),
            num_wormhole_pairs: params.special_routes.num_wormhole_pairs,
            num_gateways: params.special_routes.num_gateways,
            max_per_node_fanout: DEFAULT_MAX_PER_NODE_FANOUT,
        }
    }
}

/// Bounded special-route generation report (producer-side only).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpecialRouteReport {
    pub long_range_candidate_count: u32,
    pub wormhole_pair_count: u32,
    pub gateway_count: u32,
    pub rejected_fanout: u32,
    pub rejected_duplicate: u32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SpecialRouteError {
    #[error("placement contains no systems")]
    EmptyPlacement,
    #[error("fixture lattice edge must be positive")]
    InvalidFixtureEdge,
    #[error("max_per_node_fanout must be positive")]
    InvalidFanoutCap,
    #[error("could not satisfy {kind:?} count: requested {requested}, selected {selected}")]
    UnsatisfiedRouteCount {
        kind: SpecialRouteKind,
        requested: u32,
        selected: u32,
    },
    #[error("unknown system id '{id}'")]
    UnknownSystemId { id: String },
    #[error("special route self-link for system '{id}'")]
    SelfLink { id: String },
    #[error("duplicate special route edge ({from}, {to})")]
    DuplicateEdge { from: String, to: String },
}

/// Fail-closed validation for directly constructed [`SpecialRouteOptions`].
pub fn validate_special_route_options(
    options: &SpecialRouteOptions,
) -> Result<(), SpecialRouteError> {
    if options.fixture_lattice_edge == 0 {
        return Err(SpecialRouteError::InvalidFixtureEdge);
    }
    if options.max_per_node_fanout == 0 {
        return Err(SpecialRouteError::InvalidFanoutCap);
    }
    Ok(())
}

/// Generate bounded wormhole/gateway endpoint pairs from in-memory placements.
pub fn generate_special_routes(
    placement: &ShapePlacement,
    options: &SpecialRouteOptions,
    existing_edges: &[(String, String)],
    rng: &mut MapGenRng,
) -> Result<(SpecialRouteTopology, SpecialRouteReport), SpecialRouteError> {
    if placement.systems.is_empty() {
        return Err(SpecialRouteError::EmptyPlacement);
    }
    validate_special_route_options(options)?;

    if options.num_wormhole_pairs == 0 && options.num_gateways == 0 {
        return Ok((
            SpecialRouteTopology { edges: Vec::new() },
            SpecialRouteReport::default(),
        ));
    }

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
            let left_pos = positions[left];
            let right_pos = positions[right];
            if is_n4_neighbor(left_pos, right_pos) {
                continue;
            }
            let distance = grid_chebyshev_distance(left_pos, right_pos);
            if distance == 0 {
                continue;
            }
            let pair = canonical_pair(&ids[left], &ids[right]);
            candidates.push((distance, pair.0, pair.1));
        }
    }

    let long_range_candidate_count = candidates.len() as u32;

    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });

    let mut ordered: Vec<(String, String)> = candidates
        .into_iter()
        .map(|(_, from, to)| (from, to))
        .collect();
    fisher_yates_shuffle(&mut ordered, rng);

    let mut selected = Vec::new();
    let mut rejected_fanout = 0u32;
    let mut rejected_duplicate = 0u32;

    let wormhole_pair_count = select_special_routes(
        SpecialRouteKind::WormholePair,
        options.num_wormhole_pairs,
        &ordered,
        &occupied,
        &mut fanout,
        options.max_per_node_fanout,
        &mut selected,
        &mut rejected_fanout,
        &mut rejected_duplicate,
    )?;

    let gateway_count = select_special_routes(
        SpecialRouteKind::Gateway,
        options.num_gateways,
        &ordered,
        &occupied,
        &mut fanout,
        options.max_per_node_fanout,
        &mut selected,
        &mut rejected_fanout,
        &mut rejected_duplicate,
    )?;

    let report = SpecialRouteReport {
        long_range_candidate_count,
        wormhole_pair_count,
        gateway_count,
        rejected_fanout,
        rejected_duplicate,
    };

    Ok((SpecialRouteTopology { edges: selected }, report))
}

fn select_special_routes(
    kind: SpecialRouteKind,
    requested: u32,
    ordered: &[(String, String)],
    occupied: &BTreeSet<(String, String)>,
    fanout: &mut BTreeMap<String, u32>,
    max_per_node_fanout: u32,
    selected: &mut Vec<SpecialRouteEdge>,
    rejected_fanout: &mut u32,
    rejected_duplicate: &mut u32,
) -> Result<u32, SpecialRouteError> {
    if requested == 0 {
        return Ok(0);
    }

    let mut picked = 0u32;
    let mut seen_pairs: BTreeSet<(String, String)> = selected
        .iter()
        .map(|edge| canonical_pair(&edge.from, &edge.to))
        .collect();

    for (from, to) in ordered {
        if picked >= requested {
            break;
        }
        let pair = canonical_pair(from, to);
        if occupied.contains(&pair) || seen_pairs.contains(&pair) {
            *rejected_duplicate += 1;
            continue;
        }
        if fanout.get(from).copied().unwrap_or(0) >= max_per_node_fanout
            || fanout.get(to).copied().unwrap_or(0) >= max_per_node_fanout
        {
            *rejected_fanout += 1;
            continue;
        }
        *fanout.entry(from.clone()).or_insert(0) += 1;
        *fanout.entry(to.clone()).or_insert(0) += 1;
        seen_pairs.insert(pair);
        selected.push(SpecialRouteEdge {
            kind,
            from: from.clone(),
            to: to.clone(),
        });
        picked += 1;
    }

    if picked < requested {
        return Err(SpecialRouteError::UnsatisfiedRouteCount {
            kind,
            requested,
            selected: picked,
        });
    }

    Ok(picked)
}

/// Validate special-route endpoint pairs before emission.
pub fn validate_special_route_edges(
    placement: &ShapePlacement,
    edges: &[SpecialRouteEdge],
) -> Result<(), SpecialRouteError> {
    let ids: BTreeSet<String> = placement.systems.iter().map(system_id_scalar).collect();
    let mut seen = BTreeSet::new();
    for edge in edges {
        if edge.from == edge.to {
            return Err(SpecialRouteError::SelfLink {
                id: edge.from.clone(),
            });
        }
        if !ids.contains(&edge.from) {
            return Err(SpecialRouteError::UnknownSystemId {
                id: edge.from.clone(),
            });
        }
        if !ids.contains(&edge.to) {
            return Err(SpecialRouteError::UnknownSystemId {
                id: edge.to.clone(),
            });
        }
        let pair = canonical_pair(&edge.from, &edge.to);
        if !seen.insert(pair.clone()) {
            return Err(SpecialRouteError::DuplicateEdge {
                from: pair.0,
                to: pair.1,
            });
        }
    }
    Ok(())
}

/// Mirror closed `mapgen_links` N4 adjacency on lowered grid positions.
fn is_n4_neighbor(left: (u32, u32), right: (u32, u32)) -> bool {
    (left.0 == right.0 && left.1.abs_diff(right.1) == 1)
        || (left.1 == right.1 && left.0.abs_diff(right.0) == 1)
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
