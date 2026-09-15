//! SLOT-LOGICAL-IDENTITY-0 — forced epoch-rebind witnesses.
//!
//! StemThing §3.1 shape (a): `SlotIndex` is stable logical identity; physical
//! rows rebind only at a recorded boundary remap through the ONE
//! `AnchorLocusRemap` history (`AnchorRemapOperation::EpochRebind`, DA
//! `5194703997`). These witnesses force a physical-row scramble with logical
//! ids fixed and prove:
//!
//! 1. multi-generation CPU and GPU reduce-up outputs are bit-identical to the
//!    never-scrambled run (order-bearing paths are authored-key derived);
//! 2. the historical physical-row-order defect — sorting reduction child
//!    blocks by slot index, the exact line deleted from
//!    `TopologyState::build` — REDs through the REAL production reducers on
//!    both arms when planted;
//! 3. pre-remap replay records resolve against post-remap state through the
//!    single canonical remap chain, and the anchor-table row-move transport
//!    (CPU + GPU `KIND_ROW_MOVE`) preserves columns by construction.

use std::collections::BTreeMap;

use simthing_core::{
    apply_anchor_remaps_to_table, mint_anchor_table_from_admission, resolve_slot_through_chain,
    AnchorIdentity, AnchorRemapOperation, AnchoredLocusMap, BindingTableSnapshot, ColumnIndex,
    DimensionRegistry, ReductionRule, RemapSubject, SimProperty, SimPropertyId, SimThing,
    SimThingId, SimThingKind, SlotIndex,
};
use simthing_driver::{apply_spec_delta, SpecDelta, SpecSessionState};
use simthing_gpu::{
    apply_epoch_rebind_to_values, build_column_rule_descriptors, cpu_reduce_oracle,
    encode_column_rules, GpuContext, Pipelines, SlotAllocator, Topology, TopologyState,
    WorldGpuState,
};
use simthing_spec::{
    CompiledTrigger, EventKey, EventPriority, ScriptPredicate, ScriptedEventDefinition,
};

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn node(tag: &str) -> SimThing {
    SimThing::new(SimThingKind::Custom(tag.into()), 0)
}

/// col 0: Sum (order-sensitive under f32 reassociation); col 1: Mean.
fn witness_registry() -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    let mut mass = SimProperty::simple("core", "mass", 0);
    mass.layout.sub_fields[0].reduction_override = Some(ReductionRule::Sum);
    registry.register(mass);
    let mut mood = SimProperty::simple("core", "mood", 0);
    mood.layout.sub_fields[0].reduction_override = Some(ReductionRule::Mean);
    registry.register(mood);
    registry
}

struct Fixture {
    root: SimThing,
    /// Authored per-object values, keyed by logical identity.
    authored: BTreeMap<SimThingId, [f32; 2]>,
    a_kids: [SimThingId; 3],
    a_id: SimThingId,
    root_id: SimThingId,
}

/// root → { a(a1,a2,a3), b(b1,b2,b3), c }. `a`'s kids carry (1e8, 1.0,
/// -1e8): authored-order left-to-right Sum = 0.0; the (a1,a3,a2)
/// arrangement sums to 1.0 — one whole unit of reassociation drift the
/// witness must never see from production.
fn fixture() -> Fixture {
    let mut root = node("root");
    let mut a = node("a");
    let mut b = node("b");
    let c = node("c");
    let (a1, a2, a3) = (node("a1"), node("a2"), node("a3"));
    let (b1, b2, b3) = (node("b1"), node("b2"), node("b3"));

    let mut authored = BTreeMap::new();
    authored.insert(root.id, [0.25_f32, 0.5_f32]);
    authored.insert(a.id, [0.125, 0.25]);
    authored.insert(b.id, [0.0625, 0.125]);
    authored.insert(c.id, [7.5, 1.5]);
    authored.insert(a1.id, [1.0e8, 0.75]);
    authored.insert(a2.id, [1.0, 0.375]);
    authored.insert(a3.id, [-1.0e8, 0.1875]);
    authored.insert(b1.id, [3.5, 2.5]);
    authored.insert(b2.id, [-0.5, 4.5]);
    authored.insert(b3.id, [12.25, 6.5]);

    let a_kids = [a1.id, a2.id, a3.id];
    let a_id = a.id;
    let root_id = root.id;
    a.add_child(a1);
    a.add_child(a2);
    a.add_child(a3);
    b.add_child(b1);
    b.add_child(b2);
    b.add_child(b3);
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);
    Fixture {
        root,
        authored,
        a_kids,
        a_id,
        root_id,
    }
}

// Each simple property occupies 3 columns (Amount, Velocity, Intensity):
// mass Amount = col 0 (Sum), mood Amount = col 3 (Mean).
const N_DIMS: usize = 6;
const MASS_COL: usize = 0;
const MOOD_COL: usize = 3;
const GENERATIONS: usize = 3;

fn base_row(vals: &[f32; 2]) -> [f32; N_DIMS] {
    let mut row = [0.0_f32; N_DIMS];
    row[MASS_COL] = vals[0];
    row[MOOD_COL] = vals[1];
    row
}

fn flat_values(alloc: &SlotAllocator, authored: &BTreeMap<SimThingId, [f32; 2]>) -> Vec<f32> {
    let mut flat = vec![0.0_f32; alloc.capacity() * N_DIMS];
    for (&id, vals) in authored {
        let row = alloc.slot_of(id).expect("live row").as_usize();
        flat[row * N_DIMS..row * N_DIMS + N_DIMS].copy_from_slice(&base_row(vals));
    }
    flat
}

/// The production topology-upload recipe (mirrors `gpu_sync`): build from the
/// tree + THE binding table, pad CSR offsets to the state's slot count.
fn upload_production_topology(
    state: &mut WorldGpuState,
    root: &SimThing,
    alloc: &SlotAllocator,
    registry: &DimensionRegistry,
) -> Topology {
    let topo = TopologyState::build(root, alloc).flatten();
    let descriptors = build_column_rule_descriptors(registry, N_DIMS);
    let rules_u32 = encode_column_rules(&descriptors);
    let mut depth_slots: Vec<u32> = Vec::new();
    let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
    for bucket in &topo.depth_buckets {
        let offset = depth_slots.len() as u32;
        depth_slots.extend_from_slice(bucket);
        depth_ranges.push((offset, bucket.len() as u32));
    }
    let n_slots = state.n_slots as usize;
    let mut child_starts = topo.child_starts.clone();
    if child_starts.len() < n_slots + 1 {
        let last = *child_starts.last().unwrap_or(&0);
        child_starts.resize(n_slots + 1, last);
    }
    state.upload_reduction_topology(
        &child_starts,
        &topo.child_indices,
        &rules_u32,
        &depth_slots,
        depth_ranges,
    );
    topo
}

/// One CPU generation: reduce, then evolve every live row from its own
/// output (id-keyed — identical arithmetic in every run regardless of
/// physical placement).
fn cpu_generation(
    topo: &Topology,
    registry: &DimensionRegistry,
    alloc: &SlotAllocator,
    authored: &BTreeMap<SimThingId, [f32; 2]>,
    values: &mut Vec<f32>,
) -> Vec<f32> {
    let descriptors = build_column_rule_descriptors(registry, N_DIMS);
    let mut output = vec![0.0_f32; values.len()];
    cpu_reduce_oracle(topo, &descriptors, N_DIMS, values, &mut output);
    for (&id, base) in authored {
        let row = alloc.slot_of(id).expect("live row").as_usize();
        let base = base_row(base);
        for c in 0..N_DIMS {
            values[row * N_DIMS + c] = output[row * N_DIMS + c] * 0.5 + base[c];
        }
    }
    output
}

/// Per-id projection of a slot-major plane — the logical-identity view every
/// comparison uses.
fn by_id(
    plane: &[f32],
    alloc: &SlotAllocator,
    ids: &BTreeMap<SimThingId, [f32; 2]>,
) -> Vec<(SimThingId, [f32; 2])> {
    ids.keys()
        .map(|&id| {
            let row = alloc.slot_of(id).expect("live row").as_usize();
            (
                id,
                [
                    plane[row * N_DIMS + MASS_COL],
                    plane[row * N_DIMS + MOOD_COL],
                ],
            )
        })
        .collect()
}

/// The forced scramble: every live row moves, and `a`'s kids land so that
/// ascending-slot order becomes (a1, a3, a2) — the arrangement whose Sum
/// differs from the authored order by exactly 1.0 if any production path
/// iterates physical rows.
fn scramble_assignment(alloc: &SlotAllocator, fx: &Fixture) -> BindingTableSnapshot {
    let pre = alloc.binding_table_snapshot();
    let capacity = alloc.capacity() as u32;
    let mut taken: Vec<u32> = Vec::new();
    let mut assignment = BindingTableSnapshot::new();
    // Pin the three interesting rows first.
    let [a1, a2, a3] = fx.a_kids;
    assignment.insert(a1, SlotIndex::new(1));
    assignment.insert(a3, SlotIndex::new(3));
    assignment.insert(a2, SlotIndex::new(5));
    taken.extend([1, 3, 5]);
    // Everyone else: highest free row first — every remaining row moves.
    let mut free: Vec<u32> = (0..capacity).filter(|r| !taken.contains(r)).collect();
    for (&id, _) in pre.iter() {
        if assignment.contains_key(&id) {
            continue;
        }
        let row = free.pop().expect("capacity holds all live rows");
        assignment.insert(id, SlotIndex::new(row));
    }
    assignment
}

/// Flat family for the GPU OrderBand arm: root with SIX kids; kids carry the
/// order-sensitive Sum triple (1e8, 1.0, -1e8) followed by (3.5, -0.5,
/// 12.25). Authored-order Sum = 15.25; the k2/k3-interleaved physical order
/// sums to 16.25.
fn flat_family_fixture() -> (SimThing, BTreeMap<SimThingId, [f32; 2]>, [SimThingId; 6]) {
    let mut root = node("froot");
    let kids: Vec<SimThing> = (0..6).map(|i| node(&format!("k{i}"))).collect();
    let kid_vals: [[f32; 2]; 6] = [
        [1.0e8, 0.75],
        [1.0, 0.375],
        [-1.0e8, 0.1875],
        [3.5, 2.5],
        [-0.5, 4.5],
        [12.25, 6.5],
    ];
    let mut authored = BTreeMap::new();
    authored.insert(root.id, [0.25_f32, 0.5_f32]);
    let mut kid_ids = [SimThingId::default(); 6];
    for (i, kid) in kids.into_iter().enumerate() {
        kid_ids[i] = kid.id;
        authored.insert(kid.id, kid_vals[i]);
        root.add_child(kid);
    }
    (root, authored, kid_ids)
}

/// Every live row moves; the child block stays contiguous in authored order
/// (kids to rows 0..=5, root to row 6) — the compaction shape of a lawful
/// epoch rebind.
fn contiguous_family_rebind(alloc: &SlotAllocator, root: &SimThing) -> BindingTableSnapshot {
    let mut assignment = BindingTableSnapshot::new();
    assignment.insert(root.id, SlotIndex::new(6));
    for (i, kid) in root.children.iter().enumerate() {
        assignment.insert(kid.id, SlotIndex::new(i as u32));
    }
    // Sanity: every row moves (root was 0; kids were 1..=6).
    for (&id, &slot) in &assignment {
        assert_ne!(
            alloc.slot_of(id),
            Some(slot),
            "forced scramble moves every row"
        );
    }
    assignment
}

/// Legacy CSR upload (depth buckets) + the production OrderBand plan/ops.
fn upload_flattened_topology(
    state: &mut WorldGpuState,
    topology_state: &TopologyState,
    registry: &DimensionRegistry,
) {
    let topo = topology_state.flatten();
    let descriptors = build_column_rule_descriptors(registry, N_DIMS);
    let rules_u32 = encode_column_rules(&descriptors);
    let mut depth_slots: Vec<u32> = Vec::new();
    let mut depth_ranges: Vec<(u32, u32)> = Vec::new();
    for bucket in &topo.depth_buckets {
        let offset = depth_slots.len() as u32;
        depth_slots.extend_from_slice(bucket);
        depth_ranges.push((offset, bucket.len() as u32));
    }
    let n_slots = state.n_slots as usize;
    let mut child_starts = topo.child_starts.clone();
    if child_starts.len() < n_slots + 1 {
        let last = *child_starts.last().unwrap_or(&0);
        child_starts.resize(n_slots + 1, last);
    }
    state.upload_reduction_topology(
        &child_starts,
        &topo.child_indices,
        &rules_u32,
        &depth_slots,
        depth_ranges,
    );
}

/// Mirror of the production `upload_accumulator_reduction_plan` recipe.
fn upload_orderband_plan(
    state: &mut WorldGpuState,
    root: &SimThing,
    alloc: &SlotAllocator,
    registry: &DimensionRegistry,
) -> Result<(), simthing_gpu::ReductionPlanError> {
    let topology_state = TopologyState::build(root, alloc);
    upload_flattened_topology(state, &topology_state, registry);
    let descriptors = build_column_rule_descriptors(registry, N_DIMS);
    let plan =
        simthing_gpu::plan_reduction_orderband(&topology_state, &descriptors, N_DIMS as u32)?;
    state.ensure_reduction_soft_accumulator();
    state
        .upload_reduction_soft_ops_with_bands(&plan.ops, plan.n_bands)
        .expect("reduction op upload");
    Ok(())
}

fn gpu_state_with_orderband(
    root: &SimThing,
    alloc: &SlotAllocator,
    registry: &DimensionRegistry,
) -> WorldGpuState {
    let mut state = WorldGpuState::new(
        GpuContext::new_blocking().expect("gpu"),
        registry,
        alloc.capacity() as u32,
    );
    upload_orderband_plan(&mut state, root, alloc, registry).expect("plannable placement");
    state
}

/// One production tick with the reduction-soft session attached (the
/// dispatcher wiring), reading nothing but the state itself.
fn run_orderband_tick(state: &mut WorldGpuState) {
    let pipelines = Pipelines::new(&state.ctx);
    let mut session = state
        .accumulator_runtime
        .as_mut()
        .expect("runtime")
        .take_reduction_soft_session();
    pipelines.run_tick_pipeline_with_accumulators(
        state,
        1.0,
        simthing_gpu::AccumulatorPipelineSessions {
            intent: None,
            overlay_add: None,
            threshold: None,
            reduction_soft: session.as_mut(),
            velocity: None,
            intensity_eml: None,
            transfer: None,
            emission: None,
            encode_world_summary: false,
        },
    );
    state
        .accumulator_runtime
        .as_mut()
        .expect("runtime")
        .restore_reduction_soft_session(session);
}

/// One GPU generation: install → tick → read → id-keyed evolve.
fn gpu_generation(
    state: &mut WorldGpuState,
    values: &mut Vec<f32>,
    alloc: &SlotAllocator,
    authored: &BTreeMap<SimThingId, [f32; 2]>,
) -> Vec<f32> {
    state.install_resolved_values_at_boundary(values);
    run_orderband_tick(state);
    let out = state.read_output_vectors();
    for (&id, base) in authored {
        let row = alloc.slot_of(id).expect("live").as_usize();
        let base = base_row(base);
        for c in 0..N_DIMS {
            values[row * N_DIMS + c] = out[row * N_DIMS + c] * 0.5 + base[c];
        }
    }
    out
}

#[test]
fn slot_logical_identity_0_forced_epoch_rebind_is_bit_identical_cpu_gpu() {
    let fx = fixture();
    let registry = witness_registry();
    let loci = AnchoredLocusMap::new();

    // ── Baseline: never scrambled ─────────────────────────────────────────
    let mut base_alloc = SlotAllocator::new();
    base_alloc.install_initial_tree(&fx.root);
    let base_topo = TopologyState::build(&fx.root, &base_alloc).flatten();
    let mut base_values = flat_values(&base_alloc, &fx.authored);
    let mut base_outputs = Vec::new();
    for _ in 0..GENERATIONS {
        let out = cpu_generation(
            &base_topo,
            &registry,
            &base_alloc,
            &fx.authored,
            &mut base_values,
        );
        base_outputs.push(by_id(&out, &base_alloc, &fx.authored));
    }

    // ── Scrambled: rebind between generation 1 and 2 ──────────────────────
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&fx.root);
    let mut topo = TopologyState::build(&fx.root, &alloc).flatten();
    let mut values = flat_values(&alloc, &fx.authored);
    let mut outputs = Vec::new();
    outputs.push(by_id(
        &cpu_generation(&topo, &registry, &alloc, &fx.authored, &mut values),
        &alloc,
        &fx.authored,
    ));

    let assignment = scramble_assignment(&alloc, &fx);
    let pre_binding = alloc.binding_table_snapshot();
    let section = alloc
        .epoch_rebind(&assignment, &loci, &loci)
        .expect("lawful barrier rebind");
    assert_eq!(section.operation, AnchorRemapOperation::EpochRebind);
    let moved = pre_binding
        .iter()
        .filter(|(id, s)| assignment[id] != **s)
        .count();
    assert_eq!(
        section.remaps.len(),
        moved,
        "exactly one ObjectRow per moved live row — zero-anchor objects included"
    );
    assert!(section
        .remaps
        .iter()
        .all(|r| r.subject == RemapSubject::ObjectRow));
    // Bake bindings into the plane and rebuild the CSR from the tree + the
    // ONE post-rebind binding table (zero per-access indirection).
    values = apply_epoch_rebind_to_values(&values, N_DIMS, &section);
    topo = TopologyState::build(&fx.root, &alloc).flatten();

    for _ in 1..GENERATIONS {
        outputs.push(by_id(
            &cpu_generation(&topo, &registry, &alloc, &fx.authored, &mut values),
            &alloc,
            &fx.authored,
        ));
    }

    for (gen, (base, scrambled)) in base_outputs.iter().zip(outputs.iter()).enumerate() {
        for ((id_b, vals_b), (id_s, vals_s)) in base.iter().zip(scrambled.iter()) {
            assert_eq!(id_b, id_s);
            for c in 0..2 {
                assert_eq!(
                    vals_b[c].to_bits(),
                    vals_s[c].to_bits(),
                    "generation {gen}: {id_b:?} col {c} drifted under a pure physical scramble"
                );
            }
        }
    }

    // ── GPU arm: the REAL OrderBand reduce (production plan + session) ────
    // The OrderBand planner demands each parent's children CONTIGUOUS in
    // authored order (SLOT_RANGE ops) — placement law, so the GPU witness
    // uses the flat family fixture and a lawful block-preserving rebind.
    if try_gpu().is_none() {
        return;
    }
    let (flat_root, flat_authored, _flat_kids) = flat_family_fixture();

    // Baseline GPU: no rebind.
    let mut base_alloc2 = SlotAllocator::new();
    base_alloc2.install_initial_tree(&flat_root);
    let mut base_gpu_outputs = Vec::new();
    {
        let mut state = gpu_state_with_orderband(&flat_root, &base_alloc2, &registry);
        let mut values = flat_values(&base_alloc2, &flat_authored);
        for _ in 0..GENERATIONS {
            let out = gpu_generation(&mut state, &mut values, &base_alloc2, &flat_authored);
            base_gpu_outputs.push(by_id(&out, &base_alloc2, &flat_authored));
        }
    }

    // Scrambled GPU: rebind between generation 1 and 2 — every row moves,
    // the child block stays contiguous in authored order (compaction shape).
    let mut g_alloc = SlotAllocator::new();
    g_alloc.install_initial_tree(&flat_root);
    let mut gpu_outputs = Vec::new();
    {
        let mut state = gpu_state_with_orderband(&flat_root, &g_alloc, &registry);
        let mut values = flat_values(&g_alloc, &flat_authored);
        let out = gpu_generation(&mut state, &mut values, &g_alloc, &flat_authored);
        gpu_outputs.push(by_id(&out, &g_alloc, &flat_authored));

        let assignment = contiguous_family_rebind(&g_alloc, &flat_root);
        let section = g_alloc
            .epoch_rebind(&assignment, &loci, &loci)
            .expect("lawful barrier rebind");
        assert!(section
            .remaps
            .iter()
            .all(|r| r.subject == RemapSubject::ObjectRow));
        values = apply_epoch_rebind_to_values(&values, N_DIMS, &section);
        // Rebuild + re-upload every slot-bearing artifact from the ONE
        // post-rebind binding table (zero per-access indirection).
        upload_orderband_plan(&mut state, &flat_root, &g_alloc, &registry)
            .expect("post-rebind placement stays plannable");
        for _ in 1..GENERATIONS {
            let out = gpu_generation(&mut state, &mut values, &g_alloc, &flat_authored);
            gpu_outputs.push(by_id(&out, &g_alloc, &flat_authored));
        }
    }

    for (gen, (base, scrambled)) in base_gpu_outputs.iter().zip(gpu_outputs.iter()).enumerate() {
        for ((id_b, vals_b), (id_s, vals_s)) in base.iter().zip(scrambled.iter()) {
            assert_eq!(id_b, id_s);
            for c in 0..2 {
                assert_eq!(
                    vals_b[c].to_bits(),
                    vals_s[c].to_bits(),
                    "GPU generation {gen}: {id_b:?} col {c} drifted under a pure physical scramble"
                );
            }
        }
    }

    // CPU twin agrees bit-exactly with the GPU baseline, generation by
    // generation (same authored order on both arms).
    let base_topo2 = TopologyState::build(&flat_root, &base_alloc2).flatten();
    let mut cpu_values = flat_values(&base_alloc2, &flat_authored);
    for gpu in base_gpu_outputs.iter() {
        let out = cpu_generation(
            &base_topo2,
            &registry,
            &base_alloc2,
            &flat_authored,
            &mut cpu_values,
        );
        let cpu = by_id(&out, &base_alloc2, &flat_authored);
        for ((id_c, vals_c), (id_g, vals_g)) in cpu.iter().zip(gpu.iter()) {
            assert_eq!(id_c, id_g);
            for c in 0..2 {
                assert_eq!(vals_c[c].to_bits(), vals_g[c].to_bits());
            }
        }
    }
}
