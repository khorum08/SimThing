//! RESIDENCY-TIER-VOCABULARY-0 — the StemThing-A residency tier vocabulary.
//!
//! **A tier is a price vector, never a category** (StemThing §5):
//!
//! ```text
//! tier = { lane set, residency_class, adjacency participation, churn class, unit cost }
//! ```
//!
//! The engine vocabulary beneath authored tiers is SMALL, GENERIC, and CLOSED
//! (the enums in this module). Authored tier rows are DATA that only compose
//! it: authoring may mint rows freely (open set, no taxonomy pressure), the
//! session's tier set is FROZEN at admission ([`SessionTierSet::admit`] — the
//! moment budget arithmetic becomes statically solvable), and the engine
//! never branches on authored tier identity — a tier's `name` is authored
//! label for diagnostics and duplicate detection at admission, and nothing
//! else. Consumption ([`resolve_residency_draw`]) reads ONLY the priced
//! components; two rows with identical vectors are indistinguishable to the
//! engine regardless of their names.
//!
//! The Owner-gated mid-session tier door (dynamic ontogeny) is chartered but
//! DOES NOT EXIST; until it is designed the freeze is law and "schema
//! evolution" means usage evolution over the admitted vocabulary
//! (StemThing §5, Owner ruling 2026-08-03).
//!
//! **Capacity is quantity; extents/placement remain kernel physics**
//! (StemThing §3): [`ResidencyCapacityPartition`] carries the exact
//! hard-currency invariant `free + in_flight + occupied = capacity` with the
//! in-flight seam holding account inside the judged universe. Discrete-exact
//! transitions only — the continuous approximate allocator path is forbidden
//! for slots.
//!
//! **Census is perception, never a directory** (StemThing §6):
//! [`SparseGrantingCensus`] materializes fixed-width-per-session aggregate
//! lanes ONLY on granting-active nodes — non-granting nodes allocate zero
//! census columns/bytes (absent, never zero-filled rows), so the cost is
//! never `O(nodes × authored tiers)`.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::ids::SimThingId;

// ── The closed generic engine vocabulary ─────────────────────────────────────

/// Closed lane vocabulary — the four legs (StemThing §2). A tier's lane set
/// answers what a descendant can DO; membership is data, not behavior.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct LaneSet {
    pub participate: bool,
    pub act: bool,
    pub originate: bool,
    pub receive: bool,
}

impl LaneSet {
    pub const fn all() -> Self {
        Self {
            participate: true,
            act: true,
            originate: true,
            receive: true,
        }
    }

    pub fn is_empty(self) -> bool {
        !(self.participate || self.act || self.originate || self.receive)
    }
}

/// Closed residency shape vocabulary — what memory shape a draw receives
/// (tile dimensions: spatial block or compact row). Generic; scale is a draw
/// quantity, never a shape property (StemThing §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResidencyShapeClass {
    SpatialBlock,
    CompactRow,
}

/// Closed adjacency-participation vocabulary — whether drawn rows join grid
/// adjacency tables. Names the participation, never the topology instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AdjacencyParticipation {
    Absent,
    GridN4,
}

/// Closed churn vocabulary — how a tier's slots behave over the session.
/// Classes name dynamism only; they carry no movement or placement meaning
/// and never reintroduce physical-row identity (6.4 law preserved).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ResidencyChurnClass {
    /// Rows persist for the session (no recycling pressure).
    Static,
    /// Rows tombstone and recycle through the ordinary doors.
    Recyclable,
    /// Rows grow/shrink against authored horizons.
    Elastic,
}

// ── Authored tier rows (open across authoring; data, never taxonomy) ────────

/// One authored tier row — a PRICE VECTOR composing exactly the sealed
/// dimensions. `name` is an authored label: admission uses it for duplicate
/// detection and diagnostics; no engine path branches on it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyTierRow {
    pub name: String,
    pub lanes: LaneSet,
    pub shape: ResidencyShapeClass,
    pub adjacency: AdjacencyParticipation,
    pub churn: ResidencyChurnClass,
    /// CostBand unit cost `C` in rows: drawing `N` units grants `N·C` rows.
    pub unit_cost_rows: u32,
}

/// Admitted tier identity — the frozen session index of one admitted row.
/// Session-scoped; never authored, never serialized past its session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TierId(u16);

impl TierId {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Spanned admission failure over the authored tier input.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TierAdmissionError {
    #[error("authored tier row {row_index} (`{name}`) has zero unit cost — slots are hard currency; a free draw is not a price")]
    ZeroUnitCost { row_index: usize, name: String },
    #[error("authored tier row {row_index} (`{name}`) has an empty lane set — a tier that answers nothing prices nothing")]
    EmptyLaneSet { row_index: usize, name: String },
    #[error("authored tier rows {first_row_index} and {row_index} both name `{name}` — authored identity must be unambiguous at admission")]
    DuplicateName {
        first_row_index: usize,
        row_index: usize,
        name: String,
    },
    #[error(
        "authored tier set of {attempted} rows exceeds the session admission width limit {limit}"
    )]
    TooManyTiers { attempted: usize, limit: usize },
    #[error(
        "mid-session tier mint refused: the session tier set is FROZEN at admission ({admitted} rows); the Owner-gated epoch-boundary dynamic-tier door is chartered but does not exist (StemThing §5, Owner ruling 2026-08-03); attempted to admit {attempted} rows mid-session"
    )]
    MidSessionTierMintRefused { admitted: usize, attempted: usize },
}

/// The FROZEN session tier set. No mutation surface exists — the only mint is
/// [`SessionTierSet::admit`] at session construction, and the session door
/// ([`crate::residency_tier`] consumers) refuses a second admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionTierSet {
    rows: Vec<ResidencyTierRow>,
}

/// Census width is session-fixed; `u16` bounds the admitted set.
pub const SESSION_TIER_WIDTH_LIMIT: usize = u16::MAX as usize;

impl SessionTierSet {
    /// Session admission: validate and FREEZE the authored tier rows. Open
    /// across authoring — any number of entity names may later share these
    /// rows — but after this call the session's vocabulary is fixed.
    pub fn admit(rows: Vec<ResidencyTierRow>) -> Result<Self, TierAdmissionError> {
        if rows.len() > SESSION_TIER_WIDTH_LIMIT {
            return Err(TierAdmissionError::TooManyTiers {
                attempted: rows.len(),
                limit: SESSION_TIER_WIDTH_LIMIT,
            });
        }
        let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
        for (row_index, row) in rows.iter().enumerate() {
            if row.unit_cost_rows == 0 {
                return Err(TierAdmissionError::ZeroUnitCost {
                    row_index,
                    name: row.name.clone(),
                });
            }
            if row.lanes.is_empty() {
                return Err(TierAdmissionError::EmptyLaneSet {
                    row_index,
                    name: row.name.clone(),
                });
            }
            if let Some(&first_row_index) = seen.get(row.name.as_str()) {
                return Err(TierAdmissionError::DuplicateName {
                    first_row_index,
                    row_index,
                    name: row.name.clone(),
                });
            }
            seen.insert(row.name.as_str(), row_index);
        }
        Ok(Self { rows })
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Session-fixed census width (lanes per granting-active node).
    pub fn census_width(&self) -> u16 {
        self.rows.len() as u16
    }

    pub fn rows(&self) -> &[ResidencyTierRow] {
        &self.rows
    }

    pub fn tier(&self, id: TierId) -> Option<&ResidencyTierRow> {
        self.rows.get(id.index())
    }

    /// Resolve an authored label to its frozen id — the ONLY name-aware
    /// lookup, used by authoring-side binding (entity names → tier rows are
    /// pure data); engine consumption is id/component-driven past this point.
    pub fn tier_id_by_name(&self, name: &str) -> Option<TierId> {
        self.rows
            .iter()
            .position(|row| row.name == name)
            .map(|i| TierId(i as u16))
    }

    pub fn tier_ids(&self) -> impl Iterator<Item = TierId> + '_ {
        (0..self.rows.len()).map(|i| TierId(i as u16))
    }
}

// ── Tier consumption (identity-blind by construction) ───────────────────────

/// What a residency draw at a tier yields — priced COMPONENTS only. There is
/// deliberately no tier id or name here: downstream machinery receives shape,
/// not identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResidencyDrawShape {
    /// `N·C` — the CostBand grant in rows.
    pub rows: u64,
    pub lanes: LaneSet,
    pub shape: ResidencyShapeClass,
    pub adjacency: AdjacencyParticipation,
    pub churn: ResidencyChurnClass,
}

/// The production tier-consumption path: resolve a draw of `n_units` at one
/// admitted tier into its priced shape. Pure composition over the closed
/// vocabulary — the row's `name` is not read, so engine behavior cannot
/// depend on authored tier identity (two rows with equal vectors resolve
/// identically whatever they are called).
pub fn resolve_residency_draw(tier: &ResidencyTierRow, n_units: u32) -> ResidencyDrawShape {
    ResidencyDrawShape {
        rows: u64::from(n_units) * u64::from(tier.unit_cost_rows),
        lanes: tier.lanes,
        shape: tier.shape,
        adjacency: tier.adjacency,
        churn: tier.churn,
    }
}

// ── Exact hard-currency capacity partition (StemThing §3) ───────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CapacityPartitionError {
    #[error("issue of {requested} rows exceeds free {free}")]
    InsufficientFree { requested: u64, free: u64 },
    #[error("delivery of {requested} rows exceeds in_flight {in_flight}")]
    InsufficientInFlight { requested: u64, in_flight: u64 },
    #[error("release of {requested} rows exceeds occupied {occupied}")]
    InsufficientOccupied { requested: u64, occupied: u64 },
    #[error(
        "residency partition violated: free {free} + in_flight {in_flight} + occupied {occupied} != capacity {capacity} — a slot leak is a conservation violation, not a bookkeeping bug"
    )]
    PartitionNotExact {
        free: u64,
        in_flight: u64,
        occupied: u64,
        capacity: u64,
    },
}

/// The exact residency partition `free + in_flight + occupied = capacity`.
/// `in_flight` is the seam holding account for grants issued but not yet
/// delivered — inside the judged universe, never a side ledger. All
/// transitions are discrete-exact; every mutation re-verifies the invariant
/// so an omitted or double-counted term REDs at the door that made it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidencyCapacityPartition {
    capacity: u64,
    free: u64,
    in_flight: u64,
    occupied: u64,
}

impl ResidencyCapacityPartition {
    pub fn new(capacity: u64) -> Self {
        Self {
            capacity,
            free: capacity,
            in_flight: 0,
            occupied: 0,
        }
    }

    /// Rehydrate the existing exact partition judge from ordinary published
    /// lanes. This creates no alternate accounting state: malformed lane
    /// values fail the same conservation check used by every transition.
    pub fn from_exact_parts(
        capacity: u64,
        free: u64,
        in_flight: u64,
        occupied: u64,
    ) -> Result<Self, CapacityPartitionError> {
        let partition = Self {
            capacity,
            free,
            in_flight,
            occupied,
        };
        partition.verify_exact()?;
        Ok(partition)
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn free(&self) -> u64 {
        self.free
    }

    pub fn in_flight(&self) -> u64 {
        self.in_flight
    }

    pub fn occupied(&self) -> u64 {
        self.occupied
    }

    /// Issue a grant: rows leave `free` and enter the `in_flight` holding
    /// account (issued, not yet delivered).
    pub fn issue(&mut self, rows: u64) -> Result<(), CapacityPartitionError> {
        if rows > self.free {
            return Err(CapacityPartitionError::InsufficientFree {
                requested: rows,
                free: self.free,
            });
        }
        self.free -= rows;
        self.in_flight += rows;
        self.verify_exact()
    }

    /// Deliver an issued grant: rows leave `in_flight` and become `occupied`.
    pub fn deliver(&mut self, rows: u64) -> Result<(), CapacityPartitionError> {
        if rows > self.in_flight {
            return Err(CapacityPartitionError::InsufficientInFlight {
                requested: rows,
                in_flight: self.in_flight,
            });
        }
        self.in_flight -= rows;
        self.occupied += rows;
        self.verify_exact()
    }

    /// Cancel an issued-but-undelivered grant: rows return from `in_flight`
    /// to `free`.
    pub fn cancel_in_flight(&mut self, rows: u64) -> Result<(), CapacityPartitionError> {
        if rows > self.in_flight {
            return Err(CapacityPartitionError::InsufficientInFlight {
                requested: rows,
                in_flight: self.in_flight,
            });
        }
        self.in_flight -= rows;
        self.free += rows;
        self.verify_exact()
    }

    /// Release occupied rows back to `free`.
    pub fn release(&mut self, rows: u64) -> Result<(), CapacityPartitionError> {
        if rows > self.occupied {
            return Err(CapacityPartitionError::InsufficientOccupied {
                requested: rows,
                occupied: self.occupied,
            });
        }
        self.occupied -= rows;
        self.free += rows;
        self.verify_exact()
    }

    /// The conservation judge: `free + in_flight + occupied = capacity`,
    /// exactly — the 8.1-class boundary oracle's operand.
    pub fn verify_exact(&self) -> Result<(), CapacityPartitionError> {
        let sum = self
            .free
            .checked_add(self.in_flight)
            .and_then(|s| s.checked_add(self.occupied));
        if sum == Some(self.capacity) {
            Ok(())
        } else {
            Err(CapacityPartitionError::PartitionNotExact {
                free: self.free,
                in_flight: self.in_flight,
                occupied: self.occupied,
                capacity: self.capacity,
            })
        }
    }
}

// ── Sparse granting-node census vocabulary (StemThing §6) ───────────────────

/// Fixed-width aggregate lanes for ONE granting-active node: counts, churn,
/// growth velocity per admitted tier — perception, never a directory. Width
/// is session-fixed by the frozen tier set.
#[derive(Clone, Debug, PartialEq)]
pub struct GrantingNodeCensusLanes {
    pub counts: Vec<u32>,
    pub churn: Vec<u32>,
    pub growth_velocity: Vec<f32>,
}

impl GrantingNodeCensusLanes {
    fn zeroed(width: u16) -> Self {
        let w = width as usize;
        Self {
            counts: vec![0; w],
            churn: vec![0; w],
            growth_velocity: vec![0.0; w],
        }
    }

    /// Bytes this node's census lanes occupy (the memory-profile witness
    /// operand).
    pub fn lane_bytes(&self) -> usize {
        self.counts.len() * std::mem::size_of::<u32>()
            + self.churn.len() * std::mem::size_of::<u32>()
            + self.growth_velocity.len() * std::mem::size_of::<f32>()
    }
}

/// The sparse census: lanes exist ONLY for granting-active nodes. A
/// non-granting node has NO entry — absent, never a zero-filled row — so the
/// cost scales with granting-active nodes, never `O(nodes × tiers)`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SparseGrantingCensus {
    width: u16,
    per_node: BTreeMap<SimThingId, GrantingNodeCensusLanes>,
}

/// The production census-materialization path: given the session's node
/// universe and the granting-active subset, allocate fixed-width lanes for
/// EXACTLY the granting-active nodes. Every other node stays absent —
/// zero census columns, zero bytes, never a zero-filled row — so cost
/// scales with granting activity, not with `nodes × tiers`.
pub fn materialize_granting_census(
    tier_set: &SessionTierSet,
    nodes: &BTreeSet<SimThingId>,
    granting_active: &BTreeSet<SimThingId>,
) -> SparseGrantingCensus {
    let width = tier_set.census_width();
    let per_node = nodes
        .iter()
        .filter(|id| granting_active.contains(id))
        .map(|&id| (id, GrantingNodeCensusLanes::zeroed(width)))
        .collect();
    SparseGrantingCensus { width, per_node }
}

impl SparseGrantingCensus {
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn granting_node_count(&self) -> usize {
        self.per_node.len()
    }

    /// Lanes for one node — `None` for every non-granting node (absence IS
    /// the sparse economics; there is no zero-filled row to return).
    pub fn lanes(&self, id: SimThingId) -> Option<&GrantingNodeCensusLanes> {
        self.per_node.get(&id)
    }

    pub fn lanes_mut(&mut self, id: SimThingId) -> Option<&mut GrantingNodeCensusLanes> {
        self.per_node.get_mut(&id)
    }

    /// Total census lane bytes across the session (the memory-profile
    /// witness operand) — grows with granting-active nodes only.
    pub fn total_lane_bytes(&self) -> usize {
        self.per_node.values().map(|l| l.lane_bytes()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, cost: u32) -> ResidencyTierRow {
        ResidencyTierRow {
            name: name.into(),
            lanes: LaneSet {
                participate: true,
                ..LaneSet::default()
            },
            shape: ResidencyShapeClass::CompactRow,
            adjacency: AdjacencyParticipation::Absent,
            churn: ResidencyChurnClass::Recyclable,
            unit_cost_rows: cost,
        }
    }

    #[test]
    fn admission_freezes_a_validated_open_authored_set() {
        let set = SessionTierSet::admit(vec![
            row("spatial-container", 4),
            row("compact-participant", 1),
        ])
        .expect("valid rows admit");
        assert_eq!(set.census_width(), 2);
        assert_eq!(
            set.tier_id_by_name("compact-participant")
                .map(|t| t.index()),
            Some(1)
        );
        // No mutation surface exists on the frozen set (type boundary).
    }

    #[test]
    fn admission_rejects_zero_cost_empty_lanes_and_duplicate_names() {
        assert!(matches!(
            SessionTierSet::admit(vec![row("free-lunch", 0)]),
            Err(TierAdmissionError::ZeroUnitCost { row_index: 0, .. })
        ));
        let mut empty = row("mute", 1);
        empty.lanes = LaneSet::default();
        assert!(matches!(
            SessionTierSet::admit(vec![empty]),
            Err(TierAdmissionError::EmptyLaneSet { row_index: 0, .. })
        ));
        assert!(matches!(
            SessionTierSet::admit(vec![row("dup", 1), row("dup", 2)]),
            Err(TierAdmissionError::DuplicateName {
                first_row_index: 0,
                row_index: 1,
                ..
            })
        ));
    }

    #[test]
    fn consumption_is_identity_blind_and_costband_priced() {
        let mut a = row("alpha", 3);
        let mut b = row("omega", 3);
        a.shape = ResidencyShapeClass::SpatialBlock;
        b.shape = ResidencyShapeClass::SpatialBlock;
        // Identical price vectors, different authored names → identical
        // draw shapes: the engine cannot see the difference.
        assert_eq!(
            resolve_residency_draw(&a, 25),
            resolve_residency_draw(&b, 25)
        );
        assert_eq!(resolve_residency_draw(&a, 25).rows, 75);
    }

    #[test]
    fn capacity_partition_is_exact_through_the_full_grant_cycle() {
        let mut p = ResidencyCapacityPartition::new(100);
        p.issue(30).unwrap();
        assert_eq!((p.free(), p.in_flight(), p.occupied()), (70, 30, 0));
        p.deliver(20).unwrap();
        p.cancel_in_flight(10).unwrap();
        assert_eq!((p.free(), p.in_flight(), p.occupied()), (80, 0, 20));
        p.release(5).unwrap();
        assert_eq!((p.free(), p.in_flight(), p.occupied()), (85, 0, 15));
        p.verify_exact().unwrap();
        // Over-draws refuse exactly.
        assert!(matches!(
            p.issue(1_000),
            Err(CapacityPartitionError::InsufficientFree { .. })
        ));
        assert!(matches!(
            p.deliver(1),
            Err(CapacityPartitionError::InsufficientInFlight { .. })
        ));
    }

    #[test]
    fn census_is_sparse_and_absent_on_non_granting_nodes() {
        let set = SessionTierSet::admit(vec![row("spatial-container", 4), row("granting-root", 8)])
            .expect("admit");
        let nodes: BTreeSet<SimThingId> = (7..40).map(SimThingId::from_session_raw).collect();
        let granting = [SimThingId::from_session_raw(7)].into_iter().collect();
        let census = materialize_granting_census(&set, &nodes, &granting);
        assert_eq!(census.granting_node_count(), 1);
        assert_eq!(census.width(), 2);
        assert!(census.lanes(SimThingId::from_session_raw(7)).is_some());
        // Non-granting nodes: ABSENT, not zero-filled.
        assert!(census.lanes(SimThingId::from_session_raw(8)).is_none());
        let per_node = census
            .lanes(SimThingId::from_session_raw(7))
            .unwrap()
            .lane_bytes();
        assert_eq!(census.total_lane_bytes(), per_node);
    }
}
