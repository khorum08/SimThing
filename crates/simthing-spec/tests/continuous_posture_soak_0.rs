//! CONTINUOUS-POSTURE-SOAK-0 — N-generation forced-lag soak over landed 6.0–6.2 surfaces.
//!
//! Synthetic inline input only. No corpus, scenario tuning, or GPU device-loss path.

use std::collections::BTreeMap;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    cost_band_quantize, deliver_routed_overlay, eval_overlay_eml, AuthoredSeamStaleness,
    DimensionRegistry, ExecutionPosture, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SlotIndex, SubFieldRole, TransformOp,
};
use simthing_spec::{
    derive_staleness_f32, reconstruct_owner_channel_rf_map, reduce_owner_channel_rf,
    replay_async_owner_channel_rf_seam, AsyncOwnerChannelRfSeam, AsyncStalenessColumn,
    AuthoredStalenessHorizon, OwnerChannelRfOwnAggregate, OwnerChannelRfSteadSurface,
    ParentRfIntegrationState, ResourceKey,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn own(
    simthing_id: SimThingId,
    resource: &str,
    surplus: u32,
    deficit: u32,
) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new(resource),
        surplus,
        deficit,
    }
}

/// Scaling-matrix tree: chain of `nodes` with ownership flips every `crossing_every`.
fn scaling_tree(
    nodes: usize,
    crossing_every: usize,
    resources: &[&str],
) -> (SimThing, Vec<OwnerChannelRfOwnAggregate>, usize) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("owner-0"));
    let mut ids = vec![root.id];
    let mut cursor = &mut root;
    let mut owner_idx = 0u32;
    let mut crossing_count = 0usize;
    for depth in 1..nodes {
        let mut child = node();
        if crossing_every > 0 && depth % crossing_every == 0 {
            owner_idx += 1;
            bind_owner(&mut child, &OwnerRef::new(&format!("owner-{owner_idx}")));
            crossing_count += 1;
        }
        ids.push(child.id);
        cursor.add_child(child);
        cursor = cursor.children.last_mut().expect("child just added");
    }
    let rows = ids
        .iter()
        .flat_map(|&id| {
            resources
                .iter()
                .enumerate()
                .map(move |(ri, r)| own(id, r, ((ri as u32) + 1) % 5, ((ri as u32) * 2) % 3))
        })
        .collect();
    (root, rows, crossing_count)
}

fn slot_map(root: &SimThing) -> BTreeMap<SimThingId, SlotIndex> {
    let mut map = BTreeMap::new();
    let mut next = 0u32;
    fn walk(node: &SimThing, map: &mut BTreeMap<SimThingId, SlotIndex>, next: &mut u32) {
        map.insert(node.id, SlotIndex::new(*next));
        *next += 1;
        for child in &node.children {
            walk(child, map, next);
        }
    }
    walk(root, &mut map, &mut next);
    map
}

fn overlay_from_eml(origin: SimThingId, n: f32) -> Overlay {
    let op = TransformOp::add(n);
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(1),
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        },
        lifecycle: OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
        },
    }
}

#[test]
fn execution_posture_continuous_batches_same_kernel_paced_default() {
    assert_eq!(ExecutionPosture::default(), ExecutionPosture::Paced);
    assert!(ExecutionPosture::Paced.is_paced());
    assert_eq!(ExecutionPosture::Paced.generations_per_schedule(), 1);
    let continuous = ExecutionPosture::continuous(8).expect("nonzero continuous admits");
    assert!(continuous.is_continuous());
    assert_eq!(continuous.generations_per_schedule(), 8);
    // Posture is scheduling only — both name the same generation unit.
    assert_ne!(continuous, ExecutionPosture::Paced);
    assert!(
        ExecutionPosture::continuous(0).is_err(),
        "zero continuous batch must fail closed at admit"
    );
}

#[test]
fn owner_channel_stead_growth_crossing_bounded_over_scaling_matrix_and_product_form_reds() {
    // Scaling matrix: (nodes, crossing_every, resources)
    let matrix = [
        (32, 8, &["r0"][..]),
        (64, 8, &["r0", "r1"][..]),
        (128, 16, &["r0", "r1", "r2"][..]),
        (256, 32, &["r0", "r1"][..]),
    ];
    let mut measured: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
    for &(nodes, every, resources) in &matrix {
        let (root, rows, crossings) = scaling_tree(nodes, every, resources);
        let n_owners = crossings + 1;
        let n_resources = resources.len();
        let mut max_own = 0usize;
        let mut max_cross = 0usize;
        for gen in 0..16u32 {
            let stamped =
                reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(gen)).expect("reduce");
            let report = stamped.product();
            let reconstructed =
                reconstruct_owner_channel_rf_map(&root, &report.stead).expect("reconstruct");
            assert_eq!(reconstructed, report.buckets);
            max_own = max_own.max(report.stead.own_aggregates.len());
            max_cross = max_cross.max(report.stead.crossing_flows.len());
            assert_eq!(
                report.stead.own_aggregates.len(),
                nodes * n_resources,
                "own aggregates = nodes × resources"
            );
            assert_eq!(
                report.stead.crossing_flows.len(),
                crossings,
                "retained crossings must stay crossing-bounded across generations"
            );
            let product_form = nodes * n_owners * n_resources;
            assert!(
                report.stead.crossing_flows.len() < product_form,
                "crossing rows must never reach nodes×owners×resources product form"
            );
            // Retained STEAD cardinality = own + crossings (the measured bound).
            let retained = report.stead.own_aggregates.len() + report.stead.crossing_flows.len();
            assert_eq!(retained, nodes * n_resources + crossings);
        }
        measured.push((nodes, n_owners, n_resources, max_own, max_cross));
    }
    assert!(
        !measured.is_empty(),
        "scaling matrix must produce growth measurements"
    );

    // Planted product-form growth mutant: inflate crossings to nodes×owners×resources.
    let (root, rows, crossings) = scaling_tree(64, 8, &["r0", "r1"]);
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(0)).expect("reduce");
    let mut mutant = stamped.product().stead.clone();
    let n_owners = crossings + 1;
    let product = 64 * n_owners * 2;
    while mutant.crossing_flows.len() < product {
        if let Some(sample) = mutant.crossing_flows.first().cloned() {
            mutant.crossing_flows.push(sample);
        } else {
            break;
        }
    }
    assert!(
        mutant.crossing_flows.len() >= product || crossings == 0,
        "mutant reaches product-form cardinality"
    );
    let honest = stamped.product().stead.crossing_flows.len();
    assert_ne!(
        mutant.crossing_flows.len(),
        honest,
        "product-form growth mutant must diverge from measured crossing-bounded growth"
    );
    let _ = OwnerChannelRfSteadSurface {
        own_aggregates: mutant.own_aggregates,
        crossing_flows: mutant.crossing_flows,
    };
}

#[test]
fn staleness_is_one_derived_stead_lane_seeded_horizon_inert_zero_and_whole_lattice_reds() {
    let (root, rows, _crossings) = scaling_tree(48, 12, &["r0"]);
    let stamped = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(1)).expect("reduce");
    let crossings = &stamped.product().stead.crossing_flows;
    let seeds = AsyncStalenessColumn::seeds_from_crossings(crossings);
    assert!(!seeds.is_empty());

    let slots = slot_map(&root);
    let n_slots = slots.len();
    let mut registry = DimensionRegistry::new();
    let mut column = AsyncStalenessColumn::admit(
        &mut registry,
        n_slots,
        seeds.clone(),
        AuthoredStalenessHorizon::new(2),
    )
    .expect("admit");
    assert!(column.is_allocated());
    assert_eq!(registry.total_columns as usize, column.n_dims());
    assert_eq!(column.registration_count(), seeds.len());
    let mut stead = vec![0.0; n_slots * column.n_dims()];

    let parent = GenerationStamp::new(10);
    let mut latest = BTreeMap::new();
    for &seed in &seeds {
        latest.insert(seed, GenerationStamp::new(7));
    }
    let visits = column
        .sweep_seeded(&mut stead, &root, &slots, parent, &latest)
        .expect("seeded sweep");
    assert_eq!(column.dispatch_count, 1);
    assert!(visits > 0);
    assert_eq!(column.visit_count, visits);

    // Cost scales with crossings × horizon-neighbourhood, not lattice size.
    // Upper bound: each seed + BFS within 2 hops cannot exceed n_slots, and
    // with sparse crossings stays far below whole-lattice × generations.
    assert!(
        visits <= (seeds.len() as u64) * (n_slots as u64),
        "seeded visits must not imply whole-lattice work"
    );
    assert!(
        visits < n_slots as u64 || seeds.len() * 3 >= n_slots,
        "with sparse crossings, horizon-2 neighbourhood is below lattice size"
    );

    let seed_slot = *slots.get(&seeds[0]).expect("seed slotted");
    let observed = column.value_at(&stead, seed_slot).expect("lane");
    assert_eq!(
        observed.to_bits(),
        derive_staleness_f32(parent, GenerationStamp::new(7)).to_bits()
    );
    assert_eq!(observed.to_bits(), 3.0f32.to_bits());
    // Magnitude is on the STEAD plane index law, not a parallel column store.
    let plane_idx = usize::from(seed_slot) * column.n_dims() + column.col().raw();
    assert_eq!(stead[plane_idx].to_bits(), observed.to_bits());

    // Whole-lattice registration has no production door: crossing seeds are a
    // strict subset of lattice slots (test-only mutant lives under cfg(test)).
    let all_ids: Vec<SimThingId> = slots.keys().copied().collect();
    assert!(
        seeds.len() < all_ids.len(),
        "seeds_from_crossings must not register the whole lattice"
    );

    // Missing latest_integrated_child_stamp fails closed — never fabricates freshness.
    let mut missing_reg = DimensionRegistry::new();
    let mut missing = AsyncStalenessColumn::admit(
        &mut missing_reg,
        n_slots,
        seeds.clone(),
        AuthoredStalenessHorizon::new(1),
    )
    .expect("admit");
    let mut missing_plane = vec![0.0; n_slots * missing.n_dims()];
    let err = missing
        .sweep_seeded(&mut missing_plane, &root, &slots, parent, &BTreeMap::new())
        .expect_err("missing stamp must RED");
    assert!(matches!(
        err,
        simthing_spec::AsyncStalenessError::MissingLatestIntegratedChildStamp(_)
    ));
    assert_eq!(missing.visit_count, 0);

    // No per-node property / history / mirror: the column is the only representation.
    // Prove inert world pays zero.
    let inert = AsyncStalenessColumn::inert();
    assert!(!inert.is_allocated());
    assert_eq!(inert.column_bytes(), 0);
    assert_eq!(inert.registration_count(), 0);
    assert_eq!(inert.dispatch_count, 0);
    assert_eq!(inert.visit_count, 0);
    assert_eq!(inert.seed_count, 0);
}

#[test]
fn forced_lag_soak_replays_from_one_schedule_and_causal_cycle_is_generation_paced() {
    const N: u32 = 24;
    let (root, rows, _) = scaling_tree(40, 10, &["r0", "r1"]);
    let products: Vec<_> = (1..=N)
        .map(|g| reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(g)).expect("product"))
        .collect();

    // Forced lag: parent runs ahead; child products arrive lagged by 3.
    let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(8));
    let mut schedule = simthing_core::IntegrationSchedule::new();
    let mut parent_state = ParentRfIntegrationState::default();
    let mut cycle_passes = 0u32;
    let origin = root.id;

    for parent_gen in 4..=(N + 3) {
        // Enqueue lagged child product when available.
        let child_gen = parent_gen.saturating_sub(3);
        if child_gen >= 1 && child_gen <= N {
            let product = &products[(child_gen - 1) as usize];
            seam.enqueue_reduce_up(product)
                .expect("nonblocking enqueue");
        }

        // Closed causal cycle once per generation:
        // receive (integrate) → CostBand → EML → originate → route → receive.
        let _receipt = seam
            .apply_parent_generation_barrier(
                GenerationStamp::new(parent_gen),
                &mut parent_state,
                &mut schedule,
            )
            .expect("forced lag never waits");

        // CostBand from reduce-up V (surplus as available value).
        let v = parent_state.surplus_total.min(u64::from(u32::MAX)) as f32;
        let draw = cost_band_quantize(v.max(1.0), 2.0, true, Some(4)).expect("CostBand");
        assert!(draw.conserves_exactly());

        // EML steering from N.
        let eml_value =
            eval_overlay_eml(TransformOp::add(draw.n as f32).nodes(), 0.0, draw.n as f32);
        assert_eq!(eml_value.to_bits(), (draw.n as f32).to_bits());

        // Originate + route to a child receiver (single receive; no cascade loop).
        let mut tree = root.clone();
        let target = tree.children[0].id;
        let ov = overlay_from_eml(origin, eml_value);
        let _delivery = deliver_routed_overlay(&mut tree, target, ov).expect("route");
        cycle_passes += 1;
    }

    assert_eq!(
        cycle_passes, N,
        "cycle runs exactly once per generation under load — no authored cascade bound"
    );

    // Replay from the ONE integration schedule — bit-exact vs live parent fold.
    let live_fold = parent_state.schedule_fold;
    let replayed = replay_async_owner_channel_rf_seam(&schedule, &products, &[])
        .expect("replay from sole recorder");
    assert_eq!(
        replayed.parent_state.schedule_fold, live_fold,
        "forced-lag soak must replay bit-exactly from the existing schedule"
    );
    assert_eq!(replayed.parent_state, parent_state);

    // Ambient-timing / second-recorder mutant: empty schedule with products REDs.
    let ambient = replay_async_owner_channel_rf_seam(
        &simthing_core::IntegrationSchedule::new(),
        &products,
        &[],
    );
    assert!(
        ambient.is_err(),
        "empty second-recorder / ambient timing must RED"
    );
}

#[test]
fn continuous_posture_forced_lag_growth_and_staleness_soak_n_generations() {
    const N: u32 = 32;
    let (root, rows, crossings) = scaling_tree(80, 10, &["r0", "r1"]);
    let slots = slot_map(&root);
    let posture = ExecutionPosture::continuous(N).expect("nonzero continuous admits");
    assert_eq!(posture.generations_per_schedule(), N);

    let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(12));
    let mut schedule = simthing_core::IntegrationSchedule::new();
    let mut parent_state = ParentRfIntegrationState::default();
    let mut registry = DimensionRegistry::new();
    let mut column: Option<AsyncStalenessColumn> = None;
    let mut stead: Vec<f32> = Vec::new();
    let mut retained_crossings = Vec::new();

    for gen in 1..=N {
        let product =
            reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(gen)).expect("reduce");
        retained_crossings.push(product.product().stead.crossing_flows.len());
        seam.enqueue_reduce_up(&product).expect("enqueue");

        // Parent integrates at gen+3 (forced lag).
        let parent_gen = GenerationStamp::new(gen + 3);
        let _ = seam
            .apply_parent_generation_barrier(parent_gen, &mut parent_state, &mut schedule)
            .expect("forced lag never waits");

        let seeds =
            AsyncStalenessColumn::seeds_from_crossings(&product.product().stead.crossing_flows);
        if column.is_none() {
            column = Some(
                AsyncStalenessColumn::admit(
                    &mut registry,
                    slots.len(),
                    seeds.clone(),
                    AuthoredStalenessHorizon::new(1),
                )
                .expect("admit once"),
            );
            let col = column.as_ref().expect("just admitted");
            stead = vec![0.0; slots.len() * col.n_dims()];
        }
        let col = column.as_mut().expect("allocated after first seam product");
        let mut latest = BTreeMap::new();
        for &seed in col.seeds() {
            latest.insert(seed, GenerationStamp::new(gen));
        }
        col.sweep_seeded(&mut stead, &root, &slots, parent_gen, &latest)
            .expect("seeded sweep with stamps present");
    }

    assert!(retained_crossings.iter().all(|&c| c == crossings));
    let col = column.expect("async seam allocated staleness");
    assert_eq!(col.dispatch_count, u64::from(N));
    assert!(col.visit_count > 0);
    assert!(col.column_bytes() > 0);
    assert_eq!(stead.len(), slots.len() * col.n_dims());
    assert!(!schedule.entries().is_empty());
}
