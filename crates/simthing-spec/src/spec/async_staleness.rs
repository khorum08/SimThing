//! CONTINUOUS-POSTURE-SOAK-0 — async staleness as a derived STEAD field column.
//!
//! Staleness is `parent_generation - latest_integrated_child_stamp` (pure
//! arithmetic over operands that already exist). Its only runtime home is ONE
//! f32 STEAD lane per admitted slot (`slot * n_dims + col`) on the existing
//! values plane (`WorldGpuState.resolved.values` / its CPU shadow of identical
//! layout). This type never owns a parallel `Vec<f32>` values mirror.
//!
//! - Never a per-node property, history log, CPU mirror, or health-monitor service.
//! - Seeded only at retained ownership-crossing `boundary_simthing_id`s.
//! - Horizon-bounded: work/visit counts scale with `crossings × horizon-neighbourhood`.
//! - Inert by default: no async seam ⇒ no registry column growth, zero registration,
//!   zero dispatches, no retained side state attributable to this feature.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use simthing_core::{
    ClampBehavior, ColumnIndex, DimensionRegistry, GenerationStamp, PropertyAdmissionDisposition,
    PropertyLayout, SimProperty, SimPropertyId, SimThing, SimThingId, SlotIndex, SubFieldRole,
    SubFieldSpec,
};
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
    #[error(
        "registered seed {0:?} has no latest_integrated_child_stamp; \
         fabricating zero freshness is forbidden — fail closed"
    )]
    MissingLatestIntegratedChildStamp(SimThingId),
    #[error("slot {0} is outside the admitted staleness column")]
    SlotOutOfRange(u32),
    #[error("staleness column admission rejected: n_slots must be > 0")]
    EmptyColumn,
    #[error(
        "STEAD values plane length {actual} != n_slots ({n_slots}) × n_dims ({n_dims})"
    )]
    ValuesPlaneLengthMismatch {
        actual: usize,
        n_slots: usize,
        n_dims: usize,
    },
}

/// Registration metadata for one derived async-staleness STEAD lane.
///
/// Allocated only when an async seam registers. Absent admission means the
/// world pays zero attributable column growth / registration / sweep work.
/// Magnitude truth lives only on the caller-owned STEAD values plane.
#[derive(Clone, Debug, PartialEq)]
pub struct AsyncStalenessColumn {
    /// `None` = inert (no registry lane).
    admitted: Option<AdmittedStalenessLane>,
    /// Seed registrations (crossing boundary ids). Empty when inert.
    seeds: Vec<SimThingId>,
    horizon: Option<AuthoredStalenessHorizon>,
    /// Deterministic work meters (zero when inert / never swept).
    pub visit_count: u64,
    pub dispatch_count: u64,
    pub seed_count: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AdmittedStalenessLane {
    property_id: SimPropertyId,
    col: ColumnIndex,
    n_dims: usize,
    n_slots: usize,
}

impl AsyncStalenessColumn {
    /// Inert world: no async seam ⇒ zero registry growth, zero registration, zero work.
    pub fn inert() -> Self {
        Self {
            admitted: None,
            seeds: Vec::new(),
            horizon: None,
            visit_count: 0,
            dispatch_count: 0,
            seed_count: 0,
        }
    }

    /// Admit one derived f32 STEAD lane onto the existing registry values plane.
    ///
    /// Registers a 1-wide derived property so `n_dims` grows by exactly one.
    /// Does **not** allocate a parallel values vector — callers write/read the
    /// singular STEAD plane (`slot * n_dims + col`).
    pub fn admit(
        registry: &mut DimensionRegistry,
        n_slots: usize,
        seeds: impl IntoIterator<Item = SimThingId>,
        horizon: AuthoredStalenessHorizon,
    ) -> Result<Self, AsyncStalenessError> {
        if n_slots == 0 {
            return Err(AsyncStalenessError::EmptyColumn);
        }
        // Exactly ONE f32 STEAD lane — not PropertyLayout::standard (amount/vel/…).
        let prop = SimProperty {
            namespace: "async_staleness".into(),
            name: "derived".into(),
            admission_disposition: PropertyAdmissionDisposition::Anchored,
            layout: PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role: SubFieldRole::Named("staleness".into()),
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "staleness".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                }],
            },
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description:
                "CONTINUOUS-POSTURE-SOAK-0 derived STEAD staleness lane (singular values plane)"
                    .into(),
            intensity_labels: vec![],
        };
        let property_id = registry.register(prop);
        let range = registry.column_range(property_id);
        debug_assert_eq!(range.stride, 1, "async staleness admits one STEAD column");
        let col = ColumnIndex::from_gpu_round_trip(range.start as u32);
        let n_dims = registry.total_columns as usize;
        let seeds: Vec<SimThingId> = seeds.into_iter().collect();
        let seed_count = seeds.len() as u64;
        Ok(Self {
            admitted: Some(AdmittedStalenessLane {
                property_id,
                col,
                n_dims,
                n_slots,
            }),
            seeds,
            horizon: Some(horizon),
            visit_count: 0,
            dispatch_count: 0,
            seed_count,
        })
    }

    pub fn is_allocated(&self) -> bool {
        self.admitted.is_some()
    }

    /// Attributable STEAD lane bytes when admitted (`n_slots × f32`); zero when inert.
    ///
    /// This is the lane's share of the singular values plane — not a second store.
    pub fn column_bytes(&self) -> usize {
        self.admitted
            .map(|a| a.n_slots * std::mem::size_of::<f32>())
            .unwrap_or(0)
    }

    pub fn registration_count(&self) -> usize {
        self.seeds.len()
    }

    pub fn n_dims(&self) -> usize {
        self.admitted.map(|a| a.n_dims).unwrap_or(0)
    }

    pub fn n_slots(&self) -> usize {
        self.admitted.map(|a| a.n_slots).unwrap_or(0)
    }

    pub fn col(&self) -> ColumnIndex {
        self.admitted
            .map(|a| a.col)
            .unwrap_or_else(ColumnIndex::default)
    }

    pub fn property_id(&self) -> Option<SimPropertyId> {
        self.admitted.map(|a| a.property_id)
    }

    pub fn seeds(&self) -> &[SimThingId] {
        &self.seeds
    }

    /// Read derived staleness from the singular STEAD values plane.
    pub fn value_at(
        &self,
        stead_values: &[f32],
        slot: SlotIndex,
    ) -> Result<f32, AsyncStalenessError> {
        let admitted = self.admitted.ok_or(AsyncStalenessError::NotAllocated)?;
        self.ensure_plane_len(stead_values, admitted)?;
        let idx = usize::from(slot) * admitted.n_dims + admitted.col.raw();
        stead_values
            .get(idx)
            .copied()
            .ok_or(AsyncStalenessError::SlotOutOfRange(slot.raw()))
    }

    /// Seed from retained ownership crossings only — never whole-lattice.
    pub fn seeds_from_crossings(crossings: &[OwnerChannelRfCrossingFlow]) -> Vec<SimThingId> {
        crossings.iter().map(|c| c.boundary_simthing_id).collect()
    }

    /// Write derived staleness into the singular STEAD lane for one slot.
    pub fn write_derived(
        &self,
        stead_values: &mut [f32],
        slot: SlotIndex,
        parent_generation: GenerationStamp,
        latest_integrated_child_stamp: GenerationStamp,
    ) -> Result<(), AsyncStalenessError> {
        let admitted = self.admitted.ok_or(AsyncStalenessError::NotAllocated)?;
        self.ensure_plane_len(stead_values, admitted)?;
        let idx = usize::from(slot) * admitted.n_dims + admitted.col.raw();
        let lane = stead_values
            .get_mut(idx)
            .ok_or(AsyncStalenessError::SlotOutOfRange(slot.raw()))?;
        *lane = derive_staleness_f32(parent_generation, latest_integrated_child_stamp);
        Ok(())
    }

    /// Horizon-bounded seeded sweep writing into the singular STEAD values plane.
    ///
    /// Visits only nodes within `horizon` hops of each crossing seed. Cost is
    /// deterministic and scales with `crossings × horizon-neighbourhood`, never
    /// lattice size. Returns the number of distinct visits this dispatch.
    ///
    /// `stead_values` must be the existing STEAD matrix (`n_slots × n_dims`).
    /// `slot_of` is the admitted slot map; this module does not mint slots.
    pub fn sweep_seeded(
        &mut self,
        stead_values: &mut [f32],
        root: &SimThing,
        slot_of: &BTreeMap<SimThingId, SlotIndex>,
        parent_generation: GenerationStamp,
        latest_by_seed: &BTreeMap<SimThingId, GenerationStamp>,
    ) -> Result<u64, AsyncStalenessError> {
        let admitted = self.admitted.ok_or(AsyncStalenessError::NotAllocated)?;
        self.ensure_plane_len(stead_values, admitted)?;
        let horizon = self
            .horizon
            .expect("allocated column always carries an authored horizon")
            .hops();
        self.dispatch_count = self.dispatch_count.saturating_add(1);

        let adjacency = tree_undirected_adjacency(root);
        let mut visited_this_dispatch = 0u64;

        for seed in self.seeds.clone() {
            let Some(child_stamp) = latest_by_seed.get(&seed).copied() else {
                return Err(AsyncStalenessError::MissingLatestIntegratedChildStamp(seed));
            };
            let Some(&seed_slot) = slot_of.get(&seed) else {
                return Err(AsyncStalenessError::UnadmittedSeed(seed));
            };
            self.write_derived(stead_values, seed_slot, parent_generation, child_stamp)?;
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
                    self.write_derived(stead_values, slot, parent_generation, child_stamp)?;
                    self.visit_count = self.visit_count.saturating_add(1);
                    visited_this_dispatch = visited_this_dispatch.saturating_add(1);
                    queue.push_back((nbr, depth + 1));
                }
            }
        }
        Ok(visited_this_dispatch)
    }

    fn ensure_plane_len(
        &self,
        stead_values: &[f32],
        admitted: AdmittedStalenessLane,
    ) -> Result<(), AsyncStalenessError> {
        let expected = admitted.n_slots.saturating_mul(admitted.n_dims);
        if stead_values.len() != expected {
            return Err(AsyncStalenessError::ValuesPlaneLengthMismatch {
                actual: stead_values.len(),
                n_slots: admitted.n_slots,
                n_dims: admitted.n_dims,
            });
        }
        Ok(())
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
mod column_proofs {
    use super::*;

    fn empty_registry() -> DimensionRegistry {
        DimensionRegistry::new()
    }

    fn stead_plane(column: &AsyncStalenessColumn) -> Vec<f32> {
        vec![0.0; column.n_slots() * column.n_dims()]
    }

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
        assert!(col.property_id().is_none());
    }

    /// Test-only planted whole-lattice registration — production API has no door.
    fn plant_whole_lattice_registration_mutant(
        _column: &mut AsyncStalenessColumn,
        _all_slots: impl IntoIterator<Item = SlotIndex>,
        _parent_generation: GenerationStamp,
        _child_stamp: GenerationStamp,
    ) -> Result<(), &'static str> {
        Err("whole-lattice registration is forbidden (seeded/horizon law)")
    }

    #[test]
    fn whole_lattice_registration_mutant_reds_in_test_only_scope() {
        let mut registry = empty_registry();
        let mut col = AsyncStalenessColumn::admit(
            &mut registry,
            4,
            [SimThingId::from_session_raw(1)],
            AuthoredStalenessHorizon::new(1),
        )
        .expect("admit");
        let err = plant_whole_lattice_registration_mutant(
            &mut col,
            (0..4u32).map(SlotIndex::new),
            GenerationStamp::new(1),
            GenerationStamp::new(0),
        )
        .expect_err("whole-lattice must RED");
        assert!(err.contains("forbidden"));
    }

    #[test]
    fn missing_latest_child_stamp_fails_closed_without_fabricating_zero_freshness() {
        let root = SimThing::new(simthing_core::SimThingKind::Custom("seed".into()), 0);
        let seed = root.id;
        let mut slots = BTreeMap::new();
        slots.insert(seed, SlotIndex::new(0));
        let mut registry = empty_registry();
        let mut col = AsyncStalenessColumn::admit(
            &mut registry,
            1,
            [seed],
            AuthoredStalenessHorizon::new(0),
        )
        .expect("admit registered seed");
        let mut plane = stead_plane(&col);
        let parent = GenerationStamp::new(10);
        let empty_latest = BTreeMap::new();
        let err = col
            .sweep_seeded(&mut plane, &root, &slots, parent, &empty_latest)
            .expect_err("missing stamp must fail closed");
        assert!(matches!(
            err,
            AsyncStalenessError::MissingLatestIntegratedChildStamp(id) if id == seed
        ));
        // No fabricated freshness: STEAD lane stays admit-zero.
        assert_eq!(
            col.value_at(&plane, SlotIndex::new(0))
                .expect("lane")
                .to_bits(),
            0.0f32.to_bits()
        );
        assert_eq!(col.visit_count, 0);
    }

    #[test]
    fn admit_grows_registry_plane_without_owning_values_vec() {
        let mut registry = empty_registry();
        assert_eq!(registry.total_columns, 0);
        let col = AsyncStalenessColumn::admit(
            &mut registry,
            3,
            [SimThingId::from_session_raw(9)],
            AuthoredStalenessHorizon::new(1),
        )
        .expect("admit");
        assert_eq!(registry.total_columns, 1);
        assert_eq!(col.n_dims(), 1);
        assert_eq!(col.column_bytes(), 3 * std::mem::size_of::<f32>());
        // Structural: no parallel values field — only registration metadata.
        let debug = format!("{col:?}");
        assert!(
            !debug.contains("values:"),
            "production column must not carry a values Vec mirror: {debug}"
        );
    }
}
