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
//! `Topology::child_indices`. The CPU builder writes children in
//! **ascending slot index** order, regardless of their position in
//! `SimThing::children`. The GPU consumer iterates the same buffer in the
//! same order. Float sums and means are therefore bit-exact between CPU and
//! GPU.
//!
//! `depth_buckets` exists for the GPU dispatch: one compute dispatch per
//! depth, deepest first. The CPU oracle uses the same bucket ordering so
//! intermediate `output_vectors` rows are produced in the same sequence.

use simthing_core::{
    DimensionRegistry, ReductionRule, SimPropertyId, SimThing, SlotIndex, SubFieldRole,
};

use crate::slot::SlotAllocator;
use crate::world_state::{encode_rule, WEIGHT_COL_NONE};

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
    /// Flat list of child slot indices, packed in canonical (ascending) slot
    /// order within each parent block.
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
/// - `per_slot_children[i]` holds child slot indices in strictly ascending
///   order. The flattened CSR inherits this canonical iteration order,
///   which Pass 4–6 reduction and the CPU oracle both depend on for
///   bit-exact `f32` parity.
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
            0,
            allocator,
            &mut state.per_slot_children,
            &mut state.depths,
        );
        // Sort each parent's children by slot index — canonical iteration
        // order. (Walk visits children in tree order, which is not
        // necessarily slot order.)
        for v in &mut state.per_slot_children {
            v.sort_unstable();
        }
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
    /// Used by B2 Approach C on pure-fission growth boundaries: the
    /// `SlotAllocator` hands out monotonically increasing indices, so a
    /// newly-spawned child has the highest slot in the world. Pushing onto
    /// `per_slot_children[parent_slot]` preserves the ascending-slot
    /// invariant without re-sorting — but the assertion guards against
    /// the (currently impossible) case where slot reuse breaks that.
    ///
    /// Caller must ensure `ensure_capacity` covers both slots first.
    pub fn add_child(&mut self, parent_slot: SlotIndex, child_slot: SlotIndex) {
        let parent_idx = parent_slot.as_usize();
        let kids = &mut self.per_slot_children[parent_idx];
        if let Some(&last) = kids.last() {
            debug_assert!(
                child_slot.raw() > last,
                "TopologyState::add_child: child_slot {} <= existing last child {last} \
                 (parent_slot {}); ascending-slot invariant violated",
                child_slot.raw(),
                parent_slot.raw(),
            );
        }
        kids.push(child_slot.raw());
        if let Some(Some(parent_depth)) = self.depths.get(parent_idx).copied() {
            self.depths[child_slot.as_usize()] = Some(parent_depth + 1);
        }
    }

    /// Flatten the per-slot state into the CSR + depth-bucket form that
    /// `WorldGpuState::upload_reduction_topology` consumes. Cheap — no
    /// sorting (state already sorted by construction).
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
    depth: u32,
    allocator: &SlotAllocator,
    per_slot_children: &mut [Vec<u32>],
    depths: &mut [Option<u32>],
) {
    let Some(slot) = allocator.slot_of(node.id) else {
        return;
    };
    depths[slot.as_usize()] = Some(depth);
    for child in &node.children {
        if let Some(child_slot) = allocator.slot_of(child.id) {
            per_slot_children[slot.as_usize()].push(child_slot.raw());
        }
        walk(child, depth + 1, allocator, per_slot_children, depths);
    }
}

// ── CPU oracle ────────────────────────────────────────────────────────────────

use std::sync::atomic::{AtomicU32, Ordering};

static CPU_REDUCE_ORACLE_CALLS: AtomicU32 = AtomicU32::new(0);

/// Test-only probe: count `cpu_reduce_oracle` invocations.
pub fn cpu_reduce_oracle_call_count() -> u32 {
    CPU_REDUCE_ORACLE_CALLS.load(Ordering::Relaxed)
}

/// Reset the test-only `cpu_reduce_oracle` call counter.
pub fn reset_cpu_reduce_oracle_call_count() {
    CPU_REDUCE_ORACLE_CALLS.store(0, Ordering::Relaxed);
}

/// Reduce a SimThing tree on the CPU, matching the GPU reduction shader
/// bit-exactly. Operates on flat `values` (post-Pass-3) and writes to a flat
/// `output` of the same shape.
///
/// Both buffers are `n_slots × n_dims` row-major. Slots not present in any
/// depth bucket are left untouched in `output`.
pub fn cpu_reduce_oracle(
    topology: &Topology,
    descriptors: &[ColumnRuleDescriptor],
    n_dims: usize,
    values: &[f32],
    output: &mut [f32],
) {
    CPU_REDUCE_ORACLE_CALLS.fetch_add(1, Ordering::Relaxed);
    assert_eq!(descriptors.len(), n_dims);
    assert_eq!(values.len(), output.len());

    // Process depths from deepest to shallowest. Leaves first.
    for depth_idx in (0..topology.depth_buckets.len()).rev() {
        for &slot in &topology.depth_buckets[depth_idx] {
            reduce_one_slot(slot, topology, descriptors, n_dims, values, output);
        }
    }
}

fn reduce_one_slot(
    slot: u32,
    topology: &Topology,
    descriptors: &[ColumnRuleDescriptor],
    n_dims: usize,
    values: &[f32],
    output: &mut [f32],
) {
    let base = slot as usize * n_dims;
    let start = topology.child_starts[slot as usize] as usize;
    let end = topology.child_starts[slot as usize + 1] as usize;
    let n_children = end - start;

    if n_children == 0 {
        // Leaf: copy values → output.
        output[base..base + n_dims].copy_from_slice(&values[base..base + n_dims]);
        return;
    }

    // Inner: reduce each column independently.
    for col in 0..n_dims {
        let desc = descriptors[col];
        let v = reduce_column(
            desc,
            col,
            n_dims,
            &topology.child_indices[start..end],
            output,
        );
        output[base + col] = v;
    }
}

fn reduce_column(
    desc: ColumnRuleDescriptor,
    col: usize,
    n_dims: usize,
    child_slots: &[u32],
    output: &[f32],
) -> f32 {
    if child_slots.is_empty() {
        return 0.0;
    }
    // Iterate in the order recorded in child_indices — canonical order.
    // For floating-point determinism we accumulate left-to-right with no
    // tree reduction.
    let read = |s: u32, c: usize| output[s as usize * n_dims + c];

    match desc.rule {
        ReductionRule::Sum => {
            let mut acc = 0.0_f32;
            for &s in child_slots {
                acc += read(s, col);
            }
            acc
        }
        ReductionRule::Mean => {
            let mut acc = 0.0_f32;
            for &s in child_slots {
                acc += read(s, col);
            }
            acc / child_slots.len() as f32
        }
        ReductionRule::Max => {
            let mut acc = read(child_slots[0], col);
            for &s in &child_slots[1..] {
                let v = read(s, col);
                if v > acc {
                    acc = v;
                }
            }
            acc
        }
        ReductionRule::Min => {
            let mut acc = read(child_slots[0], col);
            for &s in &child_slots[1..] {
                let v = read(s, col);
                if v < acc {
                    acc = v;
                }
            }
            acc
        }
        ReductionRule::First => read(child_slots[0], col),
        ReductionRule::WeightedMean { .. } => {
            let wcol = desc.weight_col as usize;
            let mut weighted_sum = 0.0_f32;
            let mut weight_total = 0.0_f32;
            for &s in child_slots {
                let w = read(s, wcol);
                let v = read(s, col);
                let scaled = v * w;
                weighted_sum += scaled;
                weight_total += w;
            }
            if weight_total == 0.0 {
                0.0
            } else {
                weighted_sum / weight_total
            }
        }
    }
}

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
        alloc.populate_from_tree(&world);

        (reg, lid, world, alloc)
    }

    /// B2 Approach C safety guard: `TopologyState::build(...).flatten()` must
    /// produce a `Topology` that is bit-identical to `build_topology(...)`.
    /// CPU/GPU reduction parity (Pass 4–6) depends on the canonical
    /// ascending-slot child order baked into the CSR; any drift here
    /// breaks `f32`-associative reduction sums.
    #[test]
    fn topology_state_flatten_matches_build_topology() {
        let (_reg, _lid, world, alloc) = small_tree();
        let direct = build_topology(&world, &alloc);
        let via_state = TopologyState::build(&world, &alloc).flatten();
        assert_eq!(direct.child_starts, via_state.child_starts);
        assert_eq!(direct.child_indices, via_state.child_indices);
        assert_eq!(direct.depth_buckets, via_state.depth_buckets);
    }

    /// B2 Approach C critical safety guard: applying an incremental
    /// `add_child` to a cached state must produce the same CSR as a full
    /// rebuild from the post-fission tree. Identical bytes uploaded → same
    /// reduction output.
    #[test]
    fn topology_state_incremental_add_child_matches_full_rebuild() {
        let (reg, _lid, mut world, mut alloc) = small_tree();

        // Snapshot the cache at the original tree shape.
        let mut state = TopologyState::build(&world, &alloc);

        // Capture the pre-fission parent ids before we mutate the tree.
        let parent_a_id = world.children[0].children[0].id;
        let parent_b_id = world.children[0].children[1].id;

        // Now simulate fission spawning two new cohorts: one under each
        // existing cohort. The allocator hands out monotonically increasing
        // slot indices, so each new child ends up at the tail of its
        // parent's child block (the invariant `add_child` relies on).
        let new_a = SimThing::new(SimThingKind::Cohort, 0);
        let new_b = SimThing::new(SimThingKind::Cohort, 0);
        let new_a_id = new_a.id;
        let new_b_id = new_b.id;
        world.children[0].children[0].add_child(new_a);
        world.children[0].children[1].add_child(new_b);
        let new_a_slot = alloc.alloc(new_a_id);
        let new_b_slot = alloc.alloc(new_b_id);

        // Incremental: extend cache to new capacity, then add edges.
        state.ensure_capacity(alloc.capacity());
        state.add_child(alloc.slot_of(parent_a_id).unwrap(), new_a_slot);
        state.add_child(alloc.slot_of(parent_b_id).unwrap(), new_b_slot);
        let via_incremental = state.flatten();

        // Ground truth: full rebuild from the post-fission tree.
        let direct = build_topology(&world, &alloc);

        assert_eq!(
            direct.child_starts, via_incremental.child_starts,
            "child_starts must match full rebuild"
        );
        assert_eq!(
            direct.child_indices, via_incremental.child_indices,
            "child_indices must match full rebuild — canonical ascending-slot order"
        );
        assert_eq!(
            direct.depth_buckets, via_incremental.depth_buckets,
            "depth_buckets must match full rebuild"
        );

        // And the corollary: the CPU reduction oracle produces identical
        // f32 output through both topologies.
        let n_dims = reg.total_columns;
        let descriptors = build_column_rule_descriptors(&reg, n_dims);
        let values = vec![0.0_f32; alloc.capacity() * n_dims];
        let mut out_direct = vec![0.0_f32; values.len()];
        let mut out_incr = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&direct, &descriptors, n_dims, &values, &mut out_direct);
        cpu_reduce_oracle(
            &via_incremental,
            &descriptors,
            n_dims,
            &values,
            &mut out_incr,
        );
        for (a, b) in out_direct.iter().zip(out_incr.iter()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn topology_csr_and_depth_buckets() {
        let (_reg, _lid, world, alloc) = small_tree();
        let topo = build_topology(&world, &alloc);

        // 4 slots total. depth_buckets: [world], [loc], [c1, c2] (in slot order).
        assert_eq!(topo.depth_buckets.len(), 3);
        assert_eq!(topo.depth_buckets[0].len(), 1); // world
        assert_eq!(topo.depth_buckets[1].len(), 1); // loc
        assert_eq!(topo.depth_buckets[2].len(), 2); // 2 cohorts

        let world_slot = alloc.slot_of(world.id).unwrap();
        let world_kids_start = topo.child_starts[world_slot.as_usize()] as usize;
        let world_kids_end = topo.child_starts[world_slot.as_usize() + 1] as usize;
        assert_eq!(world_kids_end - world_kids_start, 1, "world has 1 child");
    }

    #[test]
    fn cpu_oracle_mean_intensity_max() {
        let (reg, lid, world, alloc) = small_tree();
        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let n_dims = reg.total_columns;
        let topo = build_topology(&world, &alloc);
        let descriptors = build_column_rule_descriptors(&reg, n_dims);

        // Project leaves into flat values (only cohort rows have data).
        let mut values = vec![0.0_f32; alloc.capacity() * n_dims];
        crate::projection::project_tree_to_values(&world, &reg, &alloc, n_dims, &mut values);

        let mut output = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &values, &mut output);

        // Location's reduced row: amount = mean(0.40, 0.60) = 0.50, intensity = max(0.10, 0.80) = 0.80.
        let loc_id = world.children[0].id;
        let loc_slot = alloc.slot_of(loc_id).unwrap().as_usize();
        let range = reg.column_range(lid);
        assert_eq!(
            output[loc_slot * n_dims + range.start + a_off.lane()].to_bits(),
            0.50_f32.to_bits()
        );
        assert_eq!(
            output[loc_slot * n_dims + range.start + i_off.lane()].to_bits(),
            0.80_f32.to_bits()
        );

        // World's reduced row equals location's (single child, mean of one = identity, max of one = identity).
        let world_slot = alloc.slot_of(world.id).unwrap().as_usize();
        for col in 0..n_dims {
            assert_eq!(
                output[world_slot * n_dims + col].to_bits(),
                output[loc_slot * n_dims + col].to_bits(),
                "world should mirror its single child at col {col}"
            );
        }

        // Leaves: output rows match input values bit-exactly.
        for cohort in &world.children[0].children {
            let s = alloc.slot_of(cohort.id).unwrap().as_usize();
            for col in 0..n_dims {
                assert_eq!(
                    output[s * n_dims + col].to_bits(),
                    values[s * n_dims + col].to_bits(),
                    "leaf slot {s} col {col} should be identity"
                );
            }
        }
    }

    #[test]
    fn column_rules_respect_override() {
        let mut reg = DimensionRegistry::new();

        // Property with one Amount sub-field overridden to Sum.
        let layout = PropertyLayout {
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "n".into(),
                display_range: None,
                governed_by: None,
                reduction_override: Some(ReductionRule::Sum),
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        };
        let mut prop = SimProperty::simple("core", "headcount", 0);
        prop.layout = layout;
        reg.register(prop);

        let rules = build_column_rules(&reg, reg.total_columns);
        assert_eq!(rules[0], ReductionRule::Sum);
    }

    #[test]
    fn sum_rule_sums_children() {
        // Custom property with Sum override on the Amount sub-field.
        let layout = PropertyLayout {
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "n".into(),
                display_range: None,
                governed_by: None,
                reduction_override: Some(ReductionRule::Sum),
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        };
        let mut reg = DimensionRegistry::new();
        let mut prop = SimProperty::simple("core", "headcount", 0);
        prop.layout = layout.clone();
        let pid = reg.register(prop);

        // World with 3 children, each carrying values [1.0, 2.5, 3.25].
        let mut world = SimThing::new(SimThingKind::World, 0);
        for v in [1.0f32, 2.5, 3.25] {
            let mut child = SimThing::new(SimThingKind::Cohort, 0);
            let mut pv = PropertyValue::from_layout(&layout);
            let amount_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
            pv.set_lane_at_offset(amount_off, v);
            child.add_property(pid, pv);
            world.add_child(child);
        }

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let n_dims = reg.total_columns;
        let topo = build_topology(&world, &alloc);
        let descriptors = build_column_rule_descriptors(&reg, n_dims);
        let mut values = vec![0.0_f32; alloc.capacity() * n_dims];
        crate::projection::project_tree_to_values(&world, &reg, &alloc, n_dims, &mut values);

        let mut output = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &values, &mut output);

        let world_slot = alloc.slot_of(world.id).unwrap().as_usize();
        // 1.0 + 2.5 + 3.25 = 6.75
        assert_eq!(output[world_slot * n_dims].to_bits(), 6.75_f32.to_bits());
    }

    fn population_property() -> SimProperty {
        SimProperty::simple("demo", "population", 0)
    }

    #[test]
    fn weighted_mean_uses_child_amount_as_weight() {
        let mut reg = DimensionRegistry::new();
        let pop_id = reg.register(population_property());
        let pop_layout = reg.property(pop_id).layout.clone();
        let pop_a_off = pop_layout.offset_of(&SubFieldRole::Amount).unwrap();

        let mut loyalty = SimProperty::simple("core", "loyalty", 0);
        let loyalty_layout = loyalty.layout.clone();
        let loyalty_a_off = loyalty_layout.offset_of(&SubFieldRole::Amount).unwrap();
        loyalty.layout.sub_fields[0].reduction_override =
            Some(ReductionRule::WeightedMean { by: pop_id });
        let lid = reg.register(loyalty);

        let mut world = SimThing::new(SimThingKind::World, 0);
        let mut loc = SimThing::new(SimThingKind::Location, 0);

        for (loyalty_amt, pop_amt) in [(0.40f32, 100.0), (0.80, 300.0)] {
            let mut c = SimThing::new(SimThingKind::Cohort, 0);
            let mut lpv = PropertyValue::from_layout(&loyalty_layout);
            lpv.set_lane_at_offset(loyalty_a_off, loyalty_amt);
            c.add_property(lid, lpv);

            let mut ppv = PropertyValue::from_layout(&pop_layout);
            ppv.set_lane_at_offset(pop_a_off, pop_amt);
            c.add_property(pop_id, ppv);

            loc.add_child(c);
        }
        world.add_child(loc);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let n_dims = reg.total_columns;
        let topo = build_topology(&world, &alloc);
        let descriptors = build_column_rule_descriptors(&reg, n_dims);

        let loyalty_range = reg.column_range(lid);
        assert_eq!(
            descriptors[loyalty_range.start + loyalty_a_off.lane()].rule,
            ReductionRule::WeightedMean { by: pop_id },
        );
        assert_eq!(
            descriptors[loyalty_range.start + loyalty_a_off.lane()].weight_col as usize,
            reg.column_range(pop_id).start + pop_a_off.lane(),
        );

        let mut values = vec![0.0_f32; alloc.capacity() * n_dims];
        crate::projection::project_tree_to_values(&world, &reg, &alloc, n_dims, &mut values);

        let mut output = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&topo, &descriptors, n_dims, &values, &mut output);

        // (0.40*100 + 0.80*300) / 400 = 0.70
        let loc_id = world.children[0].id;
        let loc_slot = alloc.slot_of(loc_id).unwrap().as_usize();
        let col = loyalty_range.start + loyalty_a_off.lane();
        assert_eq!(
            output[loc_slot * n_dims + col].to_bits(),
            0.70_f32.to_bits(),
        );
    }
}
