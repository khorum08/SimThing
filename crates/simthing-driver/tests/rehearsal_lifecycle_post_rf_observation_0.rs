//! rehearsal_lifecycle_post_rf_observation_0 — first synthetic consumer of the
//! post-RF canonical-observation publication admission (DA capability
//! increment, orchestrator relay 5618867588; rung `0088-INGRESS-FIDELITY-0`).
//!
//! LAW: completed RF band writes publish through the ONE canonical
//! AnchorTable observation authority even when the session carries ZERO
//! need-threshold bindings. "No thresholds to rescan" never meant "no
//! observation to publish" (P0: observation is free, deferral is loud). The
//! mechanism is the EXISTING `run_anchor_table_magnitude_maintain` scheduled
//! at the post-RF rescan point — one authority, no second observation path.
//!
//! The application-level first consumer
//! (`rehearsal_ingress_post_rf_observation_matches_born_allocations`, Astra,
//! rung 1.1) exercises the same law through the ordinary resident Studio
//! session; this witness pins the contract at its owning kernel surface.

use simthing_core::{
    mint_anchor_table_from_admission, AnchorIdentity, AnchoredLocusMap, ColumnIndex,
    DimensionRegistry, SimProperty, SimThing, SimThingKind,
};
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};

// One simple property occupies 3 columns (Amount, Velocity, Intensity).
const N_DIMS: usize = 3;
const STOCK_COL: usize = 0;

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

#[test]
fn post_rf_rescan_with_zero_need_regs_publishes_canonical_observation() {
    let Some(ctx) = try_gpu() else {
        return;
    };
    let mut registry = DimensionRegistry::new();
    let stock = registry.register(SimProperty::simple("rf", "stock", 0));
    let root = SimThing::new(SimThingKind::World, 0);
    let root_id = root.id;
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&root);
    let slot = alloc.slot_of(root_id).expect("root row");
    let col = ColumnIndex::try_from_admitted_authored(STOCK_COL as u32, N_DIMS as u32)
        .expect("stock col");
    let mut loci = AnchoredLocusMap::new();
    loci.insert((root_id, stock), (slot, col));

    let pre = 3.25_f32;
    let post = 7.875_f32; // exactly representable — bit-compared below
    let mut values = vec![0.0_f32; alloc.capacity() * N_DIMS];
    values[slot.as_usize() * N_DIMS + STOCK_COL] = pre;

    let mut state = WorldGpuState::new(ctx, &registry, alloc.capacity() as u32);
    let table = mint_anchor_table_from_admission(&root, &registry, &loci, &values, N_DIMS);
    state.upload_typed_anchor_table(&table);
    let minted = state
        .read_typed_anchor_table(&registry)
        .get(AnchorIdentity::new(root_id, stock))
        .expect("anchored row minted")
        .observed_value;
    assert_eq!(minted.to_bits(), pre.to_bits(), "minted observation = pre value");

    // The RF bands' completed writes land in the values plane; install the
    // post-RF value through the existing boundary installer.
    values[slot.as_usize() * N_DIMS + STOCK_COL] = post;
    state.install_resolved_values_at_boundary(&values);

    // ZERO need registrations — the exact corner the ordinary application
    // session exposed: no threshold work, yet observation MUST still publish.
    state.set_post_rf_need_threshold_regs(Vec::new());
    state
        .rescan_accumulator_thresholds_after_resource_flow()
        .expect("empty-need post-RF rescan is Ok");

    let observed = state
        .read_typed_anchor_table(&registry)
        .get(AnchorIdentity::new(root_id, stock))
        .expect("anchored row survives")
        .observed_value;
    assert_eq!(
        observed.to_bits(),
        post.to_bits(),
        "post-RF canonical observation must equal the completed values-plane write \
         (stale pre-RF observation = the silent deferral P0 forbids)"
    );
}

#[test]
fn post_rf_rescan_with_zero_anchor_rows_stays_a_lawful_no_op() {
    let Some(ctx) = try_gpu() else {
        return;
    };
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("rf", "stock", 0));
    let root = SimThing::new(SimThingKind::World, 0);
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&root);
    // NO anchor table uploaded: n_anchor_rows stays 0 — the maintain must
    // early-return and the rescan stays a lawful no-op.
    let mut state = WorldGpuState::new(ctx, &registry, alloc.capacity() as u32);
    state.set_post_rf_need_threshold_regs(Vec::new());
    state
        .rescan_accumulator_thresholds_after_resource_flow()
        .expect("no anchors + no need regs remains Ok");
}
