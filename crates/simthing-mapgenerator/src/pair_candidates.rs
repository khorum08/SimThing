//! Bounded endpoint-pair candidate generation for producer topology passes (PR11 scale envelope).
//!
//! Uses lattice-neighborhood windows instead of full O(N²) scans where a Chebyshev radius applies.

use std::collections::BTreeMap;

/// Max Chebyshev distance considered for hyperlane candidate enumeration.
pub const PRODUCER_MAX_HYPERLANE_DISTANCE: u32 = 64;

/// Max stored endpoint-pair rows per producer topology pass (fail-closed when exceeded).
pub const PRODUCER_PAIR_CANDIDATE_CAP: usize = 65_536;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairCandidateStats {
    pub examined_pairs: u64,
    pub stored_candidates: u32,
    pub capped: bool,
}

/// Map lowered grid positions to system indices (multiple systems may share a cell).
pub fn build_position_index(positions: &[(u32, u32)]) -> BTreeMap<(u32, u32), Vec<usize>> {
    let mut index = BTreeMap::new();
    for (system_index, position) in positions.iter().enumerate() {
        index
            .entry(*position)
            .or_insert_with(Vec::new)
            .push(system_index);
    }
    index
}

/// Enumerate unordered system pairs within `max_distance` Chebyshev on lowered grid positions.
///
/// Complexity: O(N · (2D+1)²) cell lookups for D = max_distance.
pub fn collect_pairs_within_chebyshev(
    positions: &[(u32, u32)],
    max_distance: u32,
) -> (Vec<(u32, usize, usize)>, PairCandidateStats) {
    let max_distance = max_distance.min(PRODUCER_MAX_HYPERLANE_DISTANCE);
    let index = build_position_index(positions);
    let mut out = Vec::new();
    let mut examined_pairs = 0u64;
    let mut capped = false;

    for left in 0..positions.len() {
        let (row, col) = positions[left];
        for dr in 0..=max_distance {
            for dc in 0..=max_distance {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if dr.max(dc) > max_distance {
                    continue;
                }
                for (nr, nc) in neighbor_coords(row, col, dr, dc) {
                    let Some(right_indices) = index.get(&(nr, nc)) else {
                        continue;
                    };
                    for &right in right_indices {
                        if right <= left {
                            continue;
                        }
                        examined_pairs += 1;
                        let distance = dr.max(dc);
                        if out.len() >= PRODUCER_PAIR_CANDIDATE_CAP {
                            capped = true;
                            continue;
                        }
                        out.push((distance, left, right));
                    }
                }
            }
        }
    }

    let stored_candidates = out.len() as u32;
    (
        out,
        PairCandidateStats {
            examined_pairs,
            stored_candidates,
            capped,
        },
    )
}

/// Enumerate unordered pairs matching `pair_filter`, storing at most [`PRODUCER_PAIR_CANDIDATE_CAP`] rows.
///
/// `pair_filter` receives `(left_index, right_index, chebyshev_distance)`.
pub fn collect_pairs_with_filter<F>(
    positions: &[(u32, u32)],
    mut pair_filter: F,
) -> (Vec<(u32, usize, usize)>, PairCandidateStats)
where
    F: FnMut(usize, usize, u32) -> bool,
{
    let mut out = Vec::new();
    let mut examined_pairs = 0u64;
    let mut capped = false;

    for left in 0..positions.len() {
        for right in left + 1..positions.len() {
            examined_pairs += 1;
            let distance = positions[left]
                .0
                .abs_diff(positions[right].0)
                .max(positions[left].1.abs_diff(positions[right].1));
            if !pair_filter(left, right, distance) {
                continue;
            }
            if out.len() >= PRODUCER_PAIR_CANDIDATE_CAP {
                capped = true;
                continue;
            }
            out.push((distance, left, right));
        }
    }

    let stored_candidates = out.len() as u32;
    (
        out,
        PairCandidateStats {
            examined_pairs,
            stored_candidates,
            capped,
        },
    )
}

fn neighbor_coords(row: u32, col: u32, dr: u32, dc: u32) -> [(u32, u32); 4] {
    [
        (row.wrapping_add(dr), col.wrapping_add(dc)),
        (row.wrapping_add(dr), col.wrapping_sub(dc)),
        (row.wrapping_sub(dr), col.wrapping_add(dc)),
        (row.wrapping_sub(dr), col.wrapping_sub(dc)),
    ]
}

/// Enumerate pairs matching `pair_filter`, retaining up to cap farthest-distance rows.
pub fn collect_farthest_pairs_with_filter<F>(
    positions: &[(u32, u32)],
    mut pair_filter: F,
) -> (Vec<(u32, usize, usize)>, PairCandidateStats)
where
    F: FnMut(usize, usize, u32) -> bool,
{
    let mut out: Vec<(u32, usize, usize)> = Vec::new();
    let mut examined_pairs = 0u64;
    let mut capped = false;

    for left in 0..positions.len() {
        for right in left + 1..positions.len() {
            examined_pairs += 1;
            let distance = positions[left]
                .0
                .abs_diff(positions[right].0)
                .max(positions[left].1.abs_diff(positions[right].1));
            if !pair_filter(left, right, distance) {
                continue;
            }
            if out.len() < PRODUCER_PAIR_CANDIDATE_CAP {
                out.push((distance, left, right));
            } else {
                capped = true;
                if let Some((min_index, _)) = out
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, (distance, _, _))| *distance)
                {
                    if distance > out[min_index].0 {
                        out[min_index] = (distance, left, right);
                    }
                }
            }
        }
    }

    let stored_candidates = out.len() as u32;
    (
        out,
        PairCandidateStats {
            examined_pairs,
            stored_candidates,
            capped,
        },
    )
}
