//! CONTINUOUS-POSTURE-SOAK-0 — async staleness as a derived STEAD field column.
//!
//! Staleness is `parent_generation - latest_integrated_child_stamp` (pure
//! arithmetic over operands that already exist). Its only runtime home is ONE
//! f32 STEAD lane per admitted slot (`slot * n_dims + col`).
//!
//! - Never a per-node property, history log, CPU mirror, or health-monitor service.
//! - Seeded only at retained ownership-crossing `boundary_simthing_id`s.
//! - Horizon-bounded: work/visit counts scale with `crossings × horizon-neighbourhood`.
//! - Inert by default: no async seam ⇒ zero registration, zero column bytes,
//!   zero dispatches, no retained side state.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use simthing_core::{ColumnIndex, GenerationStamp, SimThing, SimThingId, SlotIndex};
use thiserror::Error;

use super::owner_channel_rf::OwnerChannelRfCrossingFlow;

/// Authored horizon (tree hops from each seed) bounding the staleness sweep.
///
/// There is deliberately no `Default`: every async seam that registers staleness
/// must state how far the seeded magnitude may diffuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthoredStalenessHorizon {
    hops: u32,
}

impl AuthoredStalenessHorizon {
    pub const fn new(hops: u32) -> Self {
        Self { hops }
    }

    pub const fn hops(self) -> u32 {
        self.hops
    }
}

/// Derived staleness magnitude written into the STEAD column.
#[inline]
pub fn derive_staleness_f32(
    parent_generation: GenerationStamp,
    latest_integrated_child_stamp: GenerationStamp,
) -> f32 {
    parent_generation.staleness_from_child(latest_integrated_child_stamp) as f32
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum AsyncStalenessError {
    #[error("async staleness column is not allocated (no async seam registration)")]
    NotAllocated,
    #[error("seed boundary_simthing_id {0:?} has no admitted slot")]
    UnadmittedSeed(SimThingId),
    #[error("slot {0} is outside the admitted staleness column")]
    SlotOutOfRange(u32),
    #[error("planted whole-lattice registration mutant is forbidden")]
    WholeLatticeRegistrationForbidden,
    #[error("staleness column admission rejected: n_slots must be > 0")]
    EmptyColumn,
}

/// One f32 STEAD lane column for derived async staleness.
///
/// Allocated only when an async seam registers. Absent allocation means the
/// world pays zero — no Vec, no side counters, no sweep work.
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncStalenessColumn {
    /// Dense STEAD lanes: `values[slot * n_dims + col]`. `None` = inert.
    values: Option<Vec<f32>>,
    n_dims: usize,
    col: ColumnIndex,
    /// Seed registrations (crossing boundary ids). Empty when inert.
    seeds: Vec<SimThingId>,
    horizon: Option<AuthoredStalenessHorizon>,
    /// Deterministic work meters (zero when inert / never swept).
    pub visit_count: u64,
    pub dispatch_count: u64,
    pub seed_count: u64,
}

impl AsyncStalenessColumn {
    /// Inert world: no async seam ⇒ zero allocation, zero registration, zero work.
    pub fn inert() -> Self {
        Self {
            values: None,
            n_dims: 0,
            // Placeholder only; inert column never indexes through this.
            col: ColumnIndex::default(),
            seeds: Vec::new(),
            horizon: None,
            visit_count: 0,
            dispatch_count: 0,
            seed_count: 0,
        }
    }

    /// Allocate the single f32 lane column for an admitted async-seam world.
    ///
    /// `n_slots` is the admitted slot count; the column uses `n_dims = 1` and
    /// admitted `col = 0` so each slot owns exactly one f32 lane.
    pub fn admit(
        n_slots: usize,
        seeds: impl IntoIterator<Item = SimThingId>,
        horizon: AuthoredStalenessHorizon,
    ) -> Result<Self, AsyncStalenessError> {
        if n_slots == 0 {
            return Err(AsyncStalenessError::EmptyColumn);
        }
        let col = ColumnIndex::try_from_admitted_authored(0, 1).expect("n_dims=1 admits col 0");
        let seeds: Vec<SimThingId> = seeds.into_iter().collect();
        let seed_count = seeds.len() as u64;
        Ok(Self {
            values: Some(vec![0.0; n_slots]),
            n_dims: 1,
            col,
            seeds,
            horizon: Some(horizon),
            visit_count: 0,
            dispatch_count: 0,
            seed_count,
        })
    }

    pub fn is_allocated(&self) -> bool {
        self.values.is_some()
    }

    pub fn column_bytes(&self) -> usize {
        self.values
            .as_ref()
            .map(|v| v.len() * std::mem::size_of::<f32>())
            .unwrap_or(0)
    }

    pub fn registration_count(&self) -> usize {
        self.seeds.len()
    }

    pub fn n_dims(&self) -> usize {
        self.n_dims
    }

    pub fn col(&self) -> ColumnIndex {
        self.col
    }

    pub fn seeds(&self) -> &[SimThingId] {
        &self.seeds
    }

    pub fn value_at(&self, slot: SlotIndex) -> Result<f32, AsyncStalenessError> {
        let values = self
            .values
            .as_ref()
            .ok_or(AsyncStalenessError::NotAllocated)?;
        let idx = usize::from(slot) * self.n_dims + self.col.raw();
        values
            .get(idx)
            .copied()
            .ok_or(AsyncStalenessError::SlotOutOfRange(slot.raw()))
    }

    /// Seed from retained ownership crossings only — never whole-lattice.
    pub fn seeds_from_crossings(crossings: &[OwnerChannelRfCrossingFlow]) -> Vec<SimThingId> {
        crossings.iter().map(|c| c.boundary_simthing_id).collect()
    }

    /// Write derived staleness into the STEAD lane for one slot.
    pub fn write_derived(
        &mut self,
        slot: SlotIndex,
        parent_generation: GenerationStamp,
        latest_integrated_child_stamp: GenerationStamp,
    ) -> Result<(), AsyncStalenessError> {
        let values = self
            .values
            .as_mut()
            .ok_or(AsyncStalenessError::NotAllocated)?;
        let idx = usize::from(slot) * self.n_dims + self.col.raw();
        let lane = values
            .get_mut(idx)
            .ok_or(AsyncStalenessError::SlotOutOfRange(slot.raw()))?;
        *lane = derive_staleness_f32(parent_generation, latest_integrated_child_stamp);
        Ok(())
    }

    /// Horizon-bounded seeded sweep over the tree neighbourhood.
    ///
    /// Visits only nodes within `horizon` hops of each crossing seed. Cost is
    /// deterministic and scales with `crossings × horizon-neighbourhood`, never
    /// lattice size. Returns the number of distinct visits this dispatch.
    ///
    /// `slot_of` is the admitted slot map (same authority closed-loop re-attachment
    /// uses); this module does not mint slots.
    pub fn sweep_seeded(
        &mut self,
        root: &SimThing,
        slot_of: &BTreeMap<SimThingId, SlotIndex>,
        parent_generation: GenerationStamp,
        latest_by_seed: &BTreeMap<SimThingId, GenerationStamp>,
    ) -> Result<u64, AsyncStalenessError> {
        if self.values.is_none() {
            return Err(AsyncStalenessError::NotAllocated);
        }
        let horizon = self
            .horizon
            .expect("allocated column always carries an authored horizon")
            .hops();
        self.dispatch_count = self.dispatch_count.saturating_add(1);

        let adjacency = tree_undirected_adjacency(root);
        let mut visited_this_dispatch = 0u64;

        for seed in self.seeds.clone() {
            let child_stamp = latest_by_seed
                .get(&seed)
                .copied()
                .unwrap_or(parent_generation);
            let Some(&seed_slot) = slot_of.get(&seed) else {
                return Err(AsyncStalenessError::UnadmittedSeed(seed));
            };
            self.write_derived(seed_slot, parent_generation, child_stamp)?;
            self.visit_count = self.visit_count.saturating_add(1);
            visited_this_dispatch = visited_this_dispatch.saturating_add(1);

            let mut queue = VecDeque::from([(seed, 0u32)]);
            let mut seen = BTreeSet::from([seed]);
            while let Some((node, depth)) = queue.pop_front() {
                if depth >= horizon {
                    continue;
                }
                let Some(neighbours) = adjacency.get(&node) else {
                    continue;
                };
                for &nbr in neighbours {
                    if !seen.insert(nbr) {
                        continue;
                    }
                    let Some(&slot) = slot_of.get(&nbr) else {
                        continue;
                    };
                    self.write_derived(slot, parent_generation, child_stamp)?;
                    self.visit_count = self.visit_count.saturating_add(1);
                    visited_this_dispatch = visited_this_dispatch.saturating_add(1);
                    queue.push_back((nbr, depth + 1));
                }
            }
        }
        Ok(visited_this_dispatch)
    }

    /// Planted whole-lattice mutant — must RED against seeded/horizon law.
    pub fn plant_whole_lattice_registration_mutant(
        &mut self,
        _all_slots: impl IntoIterator<Item = SlotIndex>,
        _parent_generation: GenerationStamp,
        _child_stamp: GenerationStamp,
    ) -> Result<(), AsyncStalenessError> {
        Err(AsyncStalenessError::WholeLatticeRegistrationForbidden)
    }
}

impl Default for AsyncStalenessColumn {
    fn default() -> Self {
        Self::inert()
    }
}

fn tree_undirected_adjacency(root: &SimThing) -> BTreeMap<SimThingId, Vec<SimThingId>> {
    let mut adj: BTreeMap<SimThingId, Vec<SimThingId>> = BTreeMap::new();
    fn walk(node: &SimThing, adj: &mut BTreeMap<SimThingId, Vec<SimThingId>>) {
        adj.entry(node.id).or_default();
        for child in &node.children {
            adj.entry(node.id).or_default().push(child.id);
            adj.entry(child.id).or_default().push(node.id);
            walk(child, adj);
        }
    }
    walk(root, &mut adj);
    adj
}

#[cfg(test)]
mod inert_proof {
    use super::*;

    #[test]
    fn inert_column_pays_zero() {
        let col = AsyncStalenessColumn::inert();
        assert!(!col.is_allocated());
        assert_eq!(col.column_bytes(), 0);
        assert_eq!(col.registration_count(), 0);
        assert_eq!(col.dispatch_count, 0);
        assert_eq!(col.visit_count, 0);
        assert_eq!(col.seed_count, 0);
        assert!(col.seeds().is_empty());
    }
}
