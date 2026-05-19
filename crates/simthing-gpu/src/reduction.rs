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

use simthing_core::{DimensionRegistry, ReductionRule, SimPropertyId, SimThing};

use crate::slot::SlotAllocator;

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
    let n_slots = allocator.capacity();
    // Per-slot child list, in canonical (slot ascending) order.
    let mut per_slot_children: Vec<Vec<u32>> = vec![Vec::new(); n_slots];
    let mut depths: Vec<Option<u32>> = vec![None; n_slots];

    walk(root, 0, allocator, &mut per_slot_children, &mut depths);

    // Sort each parent's children by slot index — canonical iteration order.
    for v in &mut per_slot_children {
        v.sort_unstable();
    }

    // Flatten to CSR.
    let mut child_starts = Vec::with_capacity(n_slots + 1);
    let mut child_indices = Vec::new();
    child_starts.push(0);
    for kids in &per_slot_children {
        child_indices.extend_from_slice(kids);
        child_starts.push(child_indices.len() as u32);
    }

    // Bucket slots by depth.
    let max_depth = depths.iter().filter_map(|d| *d).max().unwrap_or(0) as usize;
    let mut depth_buckets: Vec<Vec<u32>> = vec![Vec::new(); max_depth + 1];
    for (slot, d) in depths.iter().enumerate() {
        if let Some(d) = d {
            depth_buckets[*d as usize].push(slot as u32);
        }
    }
    // Sort within each bucket so reduction is deterministic across runs.
    for b in &mut depth_buckets {
        b.sort_unstable();
    }

    Topology { child_starts, child_indices, depth_buckets }
}

fn walk(
    node: &SimThing,
    depth: u32,
    allocator: &SlotAllocator,
    per_slot_children: &mut [Vec<u32>],
    depths: &mut [Option<u32>],
) {
    let Some(slot) = allocator.slot_of(node.id) else { return; };
    depths[slot as usize] = Some(depth);
    for child in &node.children {
        if let Some(child_slot) = allocator.slot_of(child.id) {
            per_slot_children[slot as usize].push(child_slot);
        }
        walk(child, depth + 1, allocator, per_slot_children, depths);
    }
}

// ── CPU oracle ────────────────────────────────────────────────────────────────

/// Reduce a SimThing tree on the CPU, matching the GPU reduction shader
/// bit-exactly. Operates on flat `values` (post-Pass-3) and writes to a flat
/// `output` of the same shape.
///
/// Both buffers are `n_slots × n_dims` row-major. Slots not present in any
/// depth bucket are left untouched in `output`.
pub fn cpu_reduce_oracle(
    topology: &Topology,
    rules: &[ReductionRule],
    n_dims: usize,
    values: &[f32],
    output: &mut [f32],
) {
    assert_eq!(rules.len(), n_dims);
    assert_eq!(values.len(), output.len());

    // Process depths from deepest to shallowest. Leaves first.
    for depth_idx in (0..topology.depth_buckets.len()).rev() {
        for &slot in &topology.depth_buckets[depth_idx] {
            reduce_one_slot(slot, topology, rules, n_dims, values, output);
        }
    }
}

fn reduce_one_slot(
    slot: u32,
    topology: &Topology,
    rules: &[ReductionRule],
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
        let rule = rules[col];
        let v = reduce_column(rule, col, n_dims, &topology.child_indices[start..end], output);
        output[base + col] = v;
    }
}

fn reduce_column(
    rule: ReductionRule,
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
    let read = |s: u32| output[s as usize * n_dims + col];

    match rule {
        ReductionRule::Sum => {
            let mut acc = 0.0_f32;
            for &s in child_slots {
                acc += read(s);
            }
            acc
        }
        ReductionRule::Mean => {
            let mut acc = 0.0_f32;
            for &s in child_slots {
                acc += read(s);
            }
            acc / child_slots.len() as f32
        }
        ReductionRule::Max => {
            let mut acc = read(child_slots[0]);
            for &s in &child_slots[1..] {
                let v = read(s);
                if v > acc { acc = v; }
            }
            acc
        }
        ReductionRule::Min => {
            let mut acc = read(child_slots[0]);
            for &s in &child_slots[1..] {
                let v = read(s);
                if v < acc { acc = v; }
            }
            acc
        }
        ReductionRule::First => read(child_slots[0]),
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
        let mut loc   = SimThing::new(SimThingKind::Location, 0);
        let mut c1    = SimThing::new(SimThingKind::Cohort, 0);
        let mut c2    = SimThing::new(SimThingKind::Cohort, 0);

        let layout = reg.property(lid).layout.clone();
        let a_off = layout.offset_of(&SubFieldRole::Amount).unwrap();
        let i_off = layout.offset_of(&SubFieldRole::Intensity).unwrap();

        let mut pv1 = PropertyValue::from_layout(&layout);
        pv1.data[a_off] = 0.40;
        pv1.data[i_off] = 0.10;
        c1.add_property(lid, pv1);

        let mut pv2 = PropertyValue::from_layout(&layout);
        pv2.data[a_off] = 0.60;
        pv2.data[i_off] = 0.80;
        c2.add_property(lid, pv2);

        loc.add_child(c1);
        loc.add_child(c2);
        world.add_child(loc);

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        (reg, lid, world, alloc)
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
        let world_kids_start = topo.child_starts[world_slot as usize] as usize;
        let world_kids_end   = topo.child_starts[world_slot as usize + 1] as usize;
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
        let rules = build_column_rules(&reg, n_dims);

        // Project leaves into flat values (only cohort rows have data).
        let mut values = vec![0.0_f32; alloc.capacity() * n_dims];
        crate::projection::project_tree_to_values(&world, &reg, &alloc, n_dims, &mut values);

        let mut output = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&topo, &rules, n_dims, &values, &mut output);

        // Location's reduced row: amount = mean(0.40, 0.60) = 0.50, intensity = max(0.10, 0.80) = 0.80.
        let loc_id  = world.children[0].id;
        let loc_slot = alloc.slot_of(loc_id).unwrap() as usize;
        let range = reg.column_range(lid);
        assert_eq!(
            output[loc_slot * n_dims + range.start + a_off].to_bits(),
            0.50_f32.to_bits()
        );
        assert_eq!(
            output[loc_slot * n_dims + range.start + i_off].to_bits(),
            0.80_f32.to_bits()
        );

        // World's reduced row equals location's (single child, mean of one = identity, max of one = identity).
        let world_slot = alloc.slot_of(world.id).unwrap() as usize;
        for col in 0..n_dims {
            assert_eq!(
                output[world_slot * n_dims + col].to_bits(),
                output[loc_slot   * n_dims + col].to_bits(),
                "world should mirror its single child at col {col}"
            );
        }

        // Leaves: output rows match input values bit-exactly.
        for cohort in &world.children[0].children {
            let s = alloc.slot_of(cohort.id).unwrap() as usize;
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
                role:               SubFieldRole::Amount,
                width:              1,
                clamp:              ClampBehavior::Unbounded,
                velocity_max:       None,
                default:            0.0,
                display_name:       "n".into(),
                display_range:      None,
                governed_by:        None,
                reduction_override: Some(ReductionRule::Sum),
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
                role:               SubFieldRole::Amount,
                width:              1,
                clamp:              ClampBehavior::Unbounded,
                velocity_max:       None,
                default:            0.0,
                display_name:       "n".into(),
                display_range:      None,
                governed_by:        None,
                reduction_override: Some(ReductionRule::Sum),
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
            pv.data[0] = v;
            child.add_property(pid, pv);
            world.add_child(child);
        }

        let mut alloc = SlotAllocator::new();
        alloc.populate_from_tree(&world);

        let n_dims = reg.total_columns;
        let topo = build_topology(&world, &alloc);
        let rules = build_column_rules(&reg, n_dims);
        let mut values = vec![0.0_f32; alloc.capacity() * n_dims];
        crate::projection::project_tree_to_values(&world, &reg, &alloc, n_dims, &mut values);

        let mut output = vec![0.0_f32; values.len()];
        cpu_reduce_oracle(&topo, &rules, n_dims, &values, &mut output);

        let world_slot = alloc.slot_of(world.id).unwrap() as usize;
        // 1.0 + 2.5 + 3.25 = 6.75
        assert_eq!(output[world_slot * n_dims].to_bits(), 6.75_f32.to_bits());
    }
}
