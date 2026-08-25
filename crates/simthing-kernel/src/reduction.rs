//! CPU-side reduction support — topology + per-column rule table + CPU oracle.
//!
//! Used by:
//! - The future GPU Passes 4–6 (consume `Topology` + `column_rules` directly).
//! - The CPU oracle `cpu_reduce_oracle` that GPU output is checked against
//!   bit-exactly.
//!
//! ## Determinism contract
//!
//! Both CPU and GPU iterate children in the order recorded in
//! `Topology::child_indices`. The CPU builder writes children in canonical
//! **authored tree order** (walk/attach order — logical identity, invariant
//! under physical row rebinding; 6.4 SLOT-LOGICAL-IDENTITY-0). The GPU
//! consumer iterates the same buffer in the same order. Float sums and means
//! are therefore bit-exact between CPU and GPU, and invariant under a forced
//! epoch rebind.
//!
//! `depth_buckets` exists for the GPU dispatch: one compute dispatch per
//! depth, deepest first. The CPU oracle uses the same bucket ordering so
//! intermediate `output_vectors` rows are produced in the same sequence.

use simthing_core::{
    DimensionRegistry, ObjectResidencyRequest, ReductionRule, SimPropertyId, SimThing, SubFieldRole,
};

use crate::slot::{ObjectResidency, SlotAllocator};
use crate::wgsl_encode::{encode_rule, WEIGHT_COL_NONE};

// ── Column rule table ─────────────────────────────────────────────────────────

/// Build a per-column reduction rule table sized `n_dims`. Inactive
/// (tombstoned) property columns are filled with `ReductionRule::Mean` as a
/// safe placeholder — the shader will read them but the rows are never
/// referenced by any active SimThing.
pub fn build_column_rules(registry: &DimensionRegistry, n_dims: usize) -> Vec<ReductionRule> {
    let mut rules = vec![ReductionRule::Mean; n_dims];
    for (idx, prop) in registry.properties.iter().enumerate() {
        let id = SimPropertyId(idx as u32);
        if !registry.is_active(id) {
            continue;
        }
        let range = registry.column_range(id);
        let layout = &prop.layout;
        let mut local_offset = 0usize;
        for sf in &layout.sub_fields {
            let rule = sf.resolved_reduction();
            for k in 0..sf.width {
                let col = range.start + local_offset + k;
                if col < rules.len() {
                    rules[col] = rule;
                }
            }
            local_offset += sf.width;
        }
    }
    rules
}

/// Per-column reduction descriptor for CPU oracle and GPU upload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColumnRuleDescriptor {
    pub rule: ReductionRule,
    /// Global column of the `Amount` sub-field on the weight property when
    /// `rule` is `WeightedMean`. `WEIGHT_COL_NONE` otherwise.
    pub weight_col: u32,
}

/// Build descriptors with weight columns resolved for `WeightedMean`.
pub fn build_column_rule_descriptors(
    registry: &DimensionRegistry,
    n_dims: usize,
) -> Vec<ColumnRuleDescriptor> {
    build_column_rules(registry, n_dims)
        .into_iter()
        .map(|rule| {
            let weight_col = match rule {
                ReductionRule::WeightedMean { by } => {
                    weight_col_for_property(registry, by).unwrap_or(WEIGHT_COL_NONE)
                }
                _ => WEIGHT_COL_NONE,
            };
            ColumnRuleDescriptor { rule, weight_col }
        })
        .collect()
}

/// Flat GPU table: `[rule_kind, weight_col]` per column, length `n_dims * 2`.
pub fn encode_column_rules(descriptors: &[ColumnRuleDescriptor]) -> Vec<u32> {
    descriptors
        .iter()
        .flat_map(|d| [encode_rule(d.rule), d.weight_col])
        .collect()
}

fn weight_col_for_property(registry: &DimensionRegistry, prop_id: SimPropertyId) -> Option<u32> {
    if !registry.is_active(prop_id) {
        return None;
    }
    let prop = registry.property(prop_id);
    let local = prop.layout.offset_of(&SubFieldRole::Amount)?;
    let range = registry.column_range(prop_id);
    Some((range.start + local.lane()) as u32)
}

// ── Topology ──────────────────────────────────────────────────────────────────

/// CSR-style child topology + depth buckets for tree reduction.
///
/// Sized to `allocator.capacity()`. Slots whose ids are not allocated (e.g.
/// tombstoned) appear with empty child ranges and depth 0; the reducer treats
/// them as leaves emitting zero output, which is harmless because nothing
/// references them.
#[derive(Clone, Debug)]
pub struct Topology {
    /// CSR offsets. `child_starts[i]..child_starts[i+1]` are the indices into
    /// `child_indices` that belong to parent slot `i`. Length: `n_slots + 1`.
    pub child_starts: Vec<u32>,
    /// Flat list of child slot indices, packed in canonical AUTHORED TREE
    /// ORDER within each parent block (never physical slot order — 6.4).
    pub child_indices: Vec<u32>,
    /// `depth_buckets[d]` = slots at tree depth `d`. The root sits at depth 0;
    /// reduction processes buckets in reverse order so leaves are written
    /// before their parents.
    pub depth_buckets: Vec<Vec<u32>>,
}

impl Topology {
    pub fn n_slots(&self) -> usize {
        self.child_starts.len().saturating_sub(1)
    }
}

/// Build the topology for an `allocator` of given capacity from a SimThing
/// tree. Slots not represented in the tree end up with no children and are
/// not included in any depth bucket.
pub fn build_topology(root: &SimThing, allocator: &SlotAllocator) -> Topology {
    TopologyState::build(root, allocator).flatten()
}

/// Persistent canonical source for [`Topology`]. Owned by callers that want
/// to apply incremental updates (B2 Approach C) instead of rebuilding the
/// CSR from scratch every boundary.
///
/// Invariants maintained by all mutators:
/// - `per_slot_children[i]` holds child slot indices in canonical AUTHORED
///   TREE ORDER (walk/attach order — logical, invariant under physical row
///   rebinding). The flattened CSR inherits this canonical iteration order,
///   which Pass 4–6 reduction and the CPU oracle both depend on for
///   bit-exact `f32` parity. Physical slot order is never an iteration
///   order (6.4 SLOT-LOGICAL-IDENTITY-0).
/// - `depths[i] == Some(d)` iff slot `i` is reachable from the tree root
///   at depth `d`.
/// - `per_slot_children.len() == depths.len()` and both are sized to a
///   capacity ≥ the slot allocator's capacity.
#[derive(Clone, Debug, Default)]
pub struct TopologyState {
    pub per_slot_children: Vec<Vec<u32>>,
    pub depths: Vec<Option<u32>>,
}

impl TopologyState {
    /// Empty state sized for `n_slots` slots.
    pub fn empty(n_slots: usize) -> Self {
        Self {
            per_slot_children: vec![Vec::new(); n_slots],
            depths: vec![None; n_slots],
        }
    }

    /// Full rebuild from a SimThing tree and its allocator. The same code
    /// path that `build_topology` used to inline; called by both the full
    /// rebuild path and any caller that needs a fresh state.
    pub fn build(root: &SimThing, allocator: &SlotAllocator) -> Self {
        let n_slots = allocator.capacity();
        let mut state = Self::empty(n_slots);
        walk(
            root,
            root.root_residency_request(),
            0,
            allocator,
            &mut state.per_slot_children,
            &mut state.depths,
        );
        // Canonical iteration order is AUTHORED TREE ORDER (walk order) —
        // a logical/authored-key order, invariant under physical row
        // rebinding (6.4 SLOT-LOGICAL-IDENTITY-0). Physical slot order is
        // NEVER a reduction order: sorting these blocks by slot index was
        // exactly the physical-row-order defect the epoch-rebind witness
        // REDs on. In an unchurned session the walk order coincides with
        // ascending mint order, which is why pre-6.4 goldens hold.
        state
    }

    /// Ensure both vecs cover at least `n_slots` slots, extending with
    /// empty entries. Idempotent and amortized O(n_added).
    pub fn ensure_capacity(&mut self, n_slots: usize) {
        if self.per_slot_children.len() < n_slots {
            self.per_slot_children.resize(n_slots, Vec::new());
        }
        if self.depths.len() < n_slots {
            self.depths.resize(n_slots, None);
        }
    }

    /// Incremental insertion of a single `parent_slot → child_slot` edge.
    /// Used by B2 Approach C on pure-fission growth boundaries. Appending
    /// preserves the canonical AUTHORED order (a newly-spawned child attaches
    /// at the end of its parent's child list), independent of which physical
    /// row the allocator hands out — slot reuse cannot perturb the order
    /// (6.4 SLOT-LOGICAL-IDENTITY-0).
    ///
    /// Caller must ensure `ensure_capacity` covers both slots first.
    pub fn add_child(&mut self, parent: ObjectResidency, child: ObjectResidency) {
        assert_eq!(
            child.relation(),
            simthing_core::ObjectResidencyRelation::ChildOf(parent.object()),
            "TopologyState::add_child requires a current object-issued parent relation",
        );
        let parent_slot = parent.slot();
        let child_slot = child.slot();
        let parent_idx = parent_slot.as_usize();
        let kids = &mut self.per_slot_children[parent_idx];
        kids.push(child_slot.raw());
        if let Some(Some(parent_depth)) = self.depths.get(parent_idx).copied() {
            self.depths[child_slot.as_usize()] = Some(parent_depth + 1);
        }
    }

    /// Flatten the per-slot state into the CSR + depth-bucket form that
    /// `WorldGpuState::upload_reduction_topology` consumes. Cheap — no
    /// sorting (state already in canonical authored order by construction).
    pub fn flatten(&self) -> Topology {
        let n_slots = self.per_slot_children.len();
        let mut child_starts = Vec::with_capacity(n_slots + 1);
        let mut child_indices = Vec::new();
        child_starts.push(0);
        for kids in &self.per_slot_children {
            child_indices.extend_from_slice(kids);
            child_starts.push(child_indices.len() as u32);
        }

        let max_depth = self.depths.iter().filter_map(|d| *d).max().unwrap_or(0) as usize;
        let mut depth_buckets: Vec<Vec<u32>> = vec![Vec::new(); max_depth + 1];
        for (slot, d) in self.depths.iter().enumerate() {
            if let Some(d) = d {
                depth_buckets[*d as usize].push(slot as u32);
            }
        }
        // Buckets are populated in ascending slot order by construction
        // (we iterate self.depths in slot order), so no sort needed.

        Topology {
            child_starts,
            child_indices,
            depth_buckets,
        }
    }
}

fn walk(
    node: &SimThing,
    request: ObjectResidencyRequest,
    depth: u32,
    allocator: &SlotAllocator,
    per_slot_children: &mut [Vec<u32>],
    depths: &mut [Option<u32>],
) {
    let Some(residency) = allocator.residency_for(&request) else {
        return;
    };
    let slot = residency.slot();
    depths[slot.as_usize()] = Some(depth);
    for child in &node.children {
        let child_request = node
            .attached_child_residency_request(child)
            .expect("tree traversal holds the attached direct child");
        if let Some(child_residency) = allocator.residency_for(&child_request) {
            per_slot_children[slot.as_usize()].push(child_residency.slot().raw());
        }
        walk(
            child,
            child_request,
            depth + 1,
            allocator,
            per_slot_children,
            depths,
        );
    }
}

// ── CPU oracle ────────────────────────────────────────────────────────────────

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        property::SubFieldSpec, ClampBehavior, DimensionRegistry, PropertyLayout, PropertyValue,
        SimProperty, SimThing, SimThingKind, SubFieldRole,
    };

    fn loyalty_property() -> SimProperty {
        SimProperty::simple("core", "loyalty", 0)
    }

    fn small_tree() -> (DimensionRegistry, SimPropertyId, SimThing, SlotAllocator) {
        let mut reg = DimensionRegistry::new();
        let lid = reg.register(loyalty_property());

        // World → 1 Location → 2 Cohorts. 4 slots total.
        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);
        let mut c1 = SimThing::new(SimThingKind::Cohort, 0);
        let mut c2 = SimThing::new(SimThingKind::Cohort, 0);

        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut pv1 = PropertyValue::from_layout(&layout);
        pv1.set_lane_at_offset(a_off, 0.40);
        pv1.set_lane_at_offset(i_off, 0.10);
        c1.add_property(lid, pv1);

        let mut pv2 = PropertyValue::from_layout(&layout);
        pv2.set_lane_at_offset(a_off, 0.60);
        pv2.set_lane_at_offset(i_off, 0.80);
        c2.add_property(lid, pv2);

        loc.add_child(c1);
        loc.add_child(c2);
        world.add_child(loc);

        let mut alloc = SlotAllocator::new();
        alloc.install_initial_tree(&world);

        (reg, lid, world, alloc)
    }

    /// B2 Approach C safety guard: `TopologyState::build(...).flatten()` must
    /// produce a `Topology` that is bit-identical to `build_topology(...)`.
    /// CPU/GPU reduction parity (Pass 4–6) depends on the canonical
    /// ascending-slot child order baked into the CSR; any drift here
    /// breaks `f32`-associative reduction sums.
    /// B2 Approach C critical safety guard: applying an incremental
    /// `add_child` to a cached state must produce the same CSR as a full
    /// rebuild from the post-fission tree. Identical bytes uploaded → same
    /// reduction output.
    fn population_property() -> SimProperty {
        SimProperty::simple("demo", "population", 0)
    }
}
