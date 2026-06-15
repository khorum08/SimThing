//! Producer-side galaxy connectivity pass — guarantee ONE interconnected galaxy (no island clusters).
//!
//! Stellaris-style maps are fully connected: every system is reachable. The base hyperlane heuristic links
//! nearest neighbours but gives no connectivity guarantee, so a scattered (disc) layout can leave isolated
//! local clusters. This pass adds the minimal extra `add_hyperlane` endpoint pairs needed to merge every
//! connected component into one, choosing **short bridges over AUTHORED structural gridcell coordinates**
//! (STEAD-PRIVILEGE-0 / STEAD-CONTRACT-0). It is producer-side topology only: bounded endpoint pairs, no
//! routes / predecessors / pathfinding, integer Chebyshev only (no sqrt).

use std::collections::BTreeMap;

use crate::pair_candidates::collect_pairs_within_chebyshev;
use crate::strategy::ShapePlacement;
use crate::topology::{canonical_pair, system_id_scalar, HyperlaneEdge};

/// Chebyshev windows tried (ascending) for short connectivity bridges before the guaranteed fallback.
const CONNECTIVITY_WINDOWS: [u32; 4] = [8, 16, 32, 64];

/// Report for the connectivity pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConnectivityReport {
    pub components_before: u32,
    pub components_after: u32,
    pub bridges_added: u32,
    pub max_bridge_chebyshev: u32,
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Returns true if a merge happened (the two were in different sets).
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        self.components -= 1;
        true
    }
}

fn chebyshev(a: (u32, u32), b: (u32, u32)) -> u32 {
    a.0.abs_diff(b.0).max(a.1.abs_diff(b.1))
}

/// Add the minimal bridges needed so every system is in one connected component.
///
/// `existing_edges` are the already-selected (base) hyperlane endpoint id pairs. Returns the **added**
/// bridge edges (canonical, deduplicated, no self-links) and a report. Guarantees the union of
/// `existing_edges + returned` is a single connected component over all systems.
pub fn connect_components(
    placement: &ShapePlacement,
    existing_edges: &[(String, String)],
) -> (Vec<HyperlaneEdge>, ConnectivityReport) {
    let n = placement.systems.len();
    if n <= 1 {
        return (Vec::new(), ConnectivityReport::default());
    }

    let index_of: BTreeMap<String, usize> = placement
        .systems
        .iter()
        .enumerate()
        .map(|(i, s)| (system_id_scalar(s), i))
        .collect();
    let positions: Vec<(u32, u32)> = placement
        .systems
        .iter()
        .map(|s| (s.coord.col, s.coord.row))
        .collect();

    let mut uf = UnionFind::new(n);
    for (from, to) in existing_edges {
        if let (Some(&i), Some(&j)) = (index_of.get(from), index_of.get(to)) {
            uf.union(i, j);
        }
    }
    let components_before = uf.components as u32;

    let mut added: Vec<HyperlaneEdge> = Vec::new();
    let mut max_bridge = 0u32;
    let ids: Vec<String> = placement.systems.iter().map(system_id_scalar).collect();

    // Phase 1 — short local bridges: Kruskal over candidate pairs within expanding Chebyshev windows.
    for window in CONNECTIVITY_WINDOWS {
        if uf.components == 1 {
            break;
        }
        let (mut candidates, _) = collect_pairs_within_chebyshev(&positions, window);
        candidates.sort_unstable();
        for (_, i, j) in candidates {
            if uf.components == 1 {
                break;
            }
            if uf.union(i, j) {
                let (from, to) = canonical_pair(&ids[i], &ids[j]);
                max_bridge = max_bridge.max(chebyshev(positions[i], positions[j]));
                added.push(HyperlaneEdge { from, to });
            }
        }
    }

    // Phase 2 — guaranteed completion: bridge any still-separate components by their nearest member pair.
    // (Rare for a filled disc; only triggers when components are farther apart than the largest window.)
    if uf.components > 1 {
        let mut by_root: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for i in 0..n {
            let r = uf.find(i);
            by_root.entry(r).or_default().push(i);
        }
        // Deterministic order: components keyed by their smallest member index.
        let mut comps: Vec<Vec<usize>> = by_root.into_values().collect();
        comps.sort_by_key(|c| c[0]);

        let mut merged: Vec<usize> = comps[0].clone();
        for comp in comps.into_iter().skip(1) {
            // Nearest member pair between the merged set and this component (short bridge).
            let mut best: Option<(u32, usize, usize)> = None;
            for &a in &merged {
                for &b in &comp {
                    let d = chebyshev(positions[a], positions[b]);
                    if best.map_or(true, |(bd, ..)| d < bd) {
                        best = Some((d, a, b));
                    }
                }
            }
            if let Some((_, a, b)) = best {
                if uf.union(a, b) {
                    let (from, to) = canonical_pair(&ids[a], &ids[b]);
                    max_bridge = max_bridge.max(chebyshev(positions[a], positions[b]));
                    added.push(HyperlaneEdge { from, to });
                }
            }
            merged.extend(comp);
        }
    }

    let report = ConnectivityReport {
        components_before,
        components_after: uf.components as u32,
        bridges_added: added.len() as u32,
        max_bridge_chebyshev: max_bridge,
    };
    (added, report)
}
