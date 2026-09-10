//! Current recursive resident causal interval through existing public seams.
use simthing_core::{
    DimensionRegistry, GenerationStamp, IntegrationSchedule, SimThing, SimThingId, TreeRealmId,
};
use simthing_driver::resident_clearing_runtime::{
    build_default_resident_arena_registry, install_default_resident_rf_property,
    ResidentAuthoredDemand, ResidentClearingBatchBinding, ResidentClearingRuntime,
    ResidentMarketQualification, ResidentSpatialClaimBinding, ResidentTemporalExecutionBinding,
};
use simthing_driver::sync_resource_flow_accumulator;
use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
use simthing_workshop::rehearsal_perf_tr_composite_0::GpuTimeline;
use std::time::Instant;

fn id(raw: u32) -> SimThingId {
    SimThingId::from_session_raw(raw)
}
fn loaded_tree(generation: u32) -> SimThing {
    serde_json::from_str(&format!(
        r#"{{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [{{
                "id": 8,
                "kind": "Owner",
                "properties": [],
                "resource_parent_edges": [],
                "overlays": [],
                "children": [{{
                    "id": 9,
                    "kind": "Cohort",
                    "properties": [],
                    "resource_parent_edges": [],
                    "overlays": [],
                    "children": [],
                    "spawned_generation": {generation}
                }}],
                "spawned_generation": {generation}
            }}],
            "spawned_generation": {generation}
        }}"#
    ))
    .expect("persisted resident fixture")
}

struct Tree {
    runtime: ResidentClearingRuntime,
    state: WorldGpuState,
    qualification: ResidentMarketQualification,
    schedule: IntegrationSchedule,
    bands: u32,
}

fn admit(ctx: &GpuContext, realm: u128) -> Tree {
    let tree_setup = Instant::now();
    let mut tree = loaded_tree(50);
    let mut registry = DimensionRegistry::new();
    let property = install_default_resident_rf_property(&mut registry, &mut tree);
    let mut slots = SlotAllocator::new();
    slots.install_initial_tree(&tree).unwrap();
    let arenas = build_default_resident_arena_registry(property, &tree, &slots, 3).unwrap();
    let mut state = WorldGpuState::new(ctx.clone(), &registry, slots.capacity() as u32);
    let mut initial = vec![0.0; state.values_len()];
    simthing_gpu::project_tree_to_values(
        &tree,
        &registry,
        &slots,
        state.n_dims as usize,
        &mut initial,
    );
    state.install_resolved_values_at_boundary(&initial);
    let flow = sync_resource_flow_accumulator(&mut state, &registry, &arenas, &[], &[], &std::collections::BTreeMap::new()).unwrap();
    let mut schedule = IntegrationSchedule::new();
    schedule.admit_resident_live_head(32).unwrap();
    let runtime = ResidentClearingRuntime::admit_with_persistence_deformations(
        ctx,
        TreeRealmId::from_u128(realm).unwrap(),
        &tree,
        &registry,
        &arenas,
        &slots,
        &schedule,
        GenerationStamp::new(50),
        2,
        &[],
    )
    .expect("REFERENCE-QUALIFICATION-REFUSAL");
    let qualification = runtime.market_qualification();
    println!(
        "M3_SETUP {}",
        serde_json::json!({"realm": realm, "setup_ns": tree_setup.elapsed().as_nanos(),
        "active_tree_rows": 3, "reserved_instance_rows": state.n_slots, "dimensions": state.n_dims,
        "rf_bands": flow.n_bands, "world_buffer_bytes_partial_inventory": state.total_buffer_bytes(),
        "qualification": format!("{:?}", runtime.qualification())})
    );
    Tree {
        runtime,
        state,
        qualification,
        schedule,
        bands: flow.n_bands,
    }
}

/// Same uploaded RF program and sealed values as the ordinary helper, encoded on
/// its existing public seam. No helper-level device-wide wait between RF and exact.
fn encode_r(tree: &mut Tree, timeline: &GpuTimeline, start: u32, end: u32) {
    let mut runtime = tree.state.accumulator_runtime.take().expect("RF runtime");
    let mut session = runtime
        .take_resource_flow_session()
        .expect("uploaded RF session");
    let mut encoder =
        tree.state
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("rehearsal existing RF seam"),
            });
    timeline.encode_mark(&mut encoder, start);
    tree.state.encode_accumulator_orderband_into(
        &mut session,
        &mut encoder,
        tree.bands,
        1.0,
        runtime.eml_program_table(),
        false,
    );
    timeline.encode_mark(&mut encoder, end);
    tree.state.ctx.queue.submit(Some(encoder.finish()));
    runtime.restore_resource_flow_session(Some(session));
    tree.state.accumulator_runtime = Some(runtime);
}

#[test]
fn per_tree_composite_keeps_barrier_next_execution_and_observation_separate() {
    let ctx = GpuContext::new_blocking().expect("REFERENCE-ADAPTER-UNAVAILABLE");
    assert_eq!(
        ctx.adapter.get_info().name,
        "NVIDIA GeForce RTX 4080 Laptop GPU"
    );
    let mut trees = [admit(&ctx, 0x88_02_01), admit(&ctx, 0x88_02_02)];
    // Independent realms intentionally reuse local IDs; neither tree's observer
    // waits until both trees' complete N/N+1 causal chains have been submitted.
    for repetition in 0..5u32 {
        let mut pending = Vec::new();
        for (tree_index, tree) in trees.iter_mut().enumerate() {
            let generation = GenerationStamp::new(50 + repetition * 2);
            let next_generation = GenerationStamp::new(generation.get() + 1);
            let begin = Instant::now();
            let mut permit = tree.runtime.begin_generation(generation).unwrap();
            let intake_permit_ns = begin.elapsed().as_nanos();
            let timeline = GpuTimeline::new(&ctx.device, 9).unwrap();
            permit.authorize_economics().unwrap();
            let causal = Instant::now();
            let r_launch = Instant::now();
            encode_r(tree, &timeline, 0, 1);
            let r_launch_ns = r_launch.elapsed().as_nanos();
            let exact_launch = Instant::now();
            let root = tree
                .runtime
                .dispatch(
                    &tree.state,
                    &tree.qualification,
                    &permit,
                    &mut tree.schedule,
                    id(7),
                    generation,
                    &[ResidentClearingBatchBinding {
                        source_simthing_id: id(8),
                        rf_participant: id(8),
                        requested: 10,
                        available: 4,
                        precedence: 0,
                    }],
                )
                .unwrap();
            let exact_launch_ns = exact_launch.elapsed().as_nanos();
            timeline.mark(&ctx.device, &ctx.queue, 2);
            let spatial_launch = Instant::now();
            let child = tree
                .runtime
                .dispatch_spatial(
                    &tree.state,
                    &tree.qualification,
                    &permit,
                    &mut tree.schedule,
                    &root,
                    id(8),
                    generation,
                    &[ResidentSpatialClaimBinding {
                        source_simthing_id: id(9),
                        rf_participant: id(9),
                        requested: 4,
                        precedence: 0,
                    }],
                )
                .unwrap();
            let spatial_launch_ns = spatial_launch.elapsed().as_nanos();
            timeline.mark(&ctx.device, &ctx.queue, 3);
            let temporal_launch = Instant::now();
            let demands = tree
                .runtime
                .prepare_temporal_demands(
                    &tree.state,
                    &tree.qualification,
                    &permit,
                    &root,
                    next_generation,
                    &[ResidentAuthoredDemand {
                        source_simthing_id: id(8),
                        quantity: 2,
                    }],
                )
                .unwrap();
            let temporal_launch_ns = temporal_launch.elapsed().as_nanos();
            timeline.mark(&ctx.device, &ctx.queue, 4);
            let tr_host_submission_ns = causal.elapsed().as_nanos();
            let barrier = Instant::now();
            tree.runtime
                .finish_generation(&mut permit, next_generation)
                .unwrap();
            let mut next_permit = tree.runtime.begin_generation(next_generation).unwrap();
            let barrier_host_ns = barrier.elapsed().as_nanos();
            timeline.mark(&ctx.device, &ctx.queue, 5);
            let next_launch = Instant::now();
            next_permit.authorize_economics().unwrap();
            encode_r(tree, &timeline, 6, 7);
            let next = tree
                .runtime
                .dispatch_temporal(
                    &tree.state,
                    &tree.qualification,
                    &next_permit,
                    &mut tree.schedule,
                    &demands,
                    id(7),
                    next_generation,
                    &[ResidentTemporalExecutionBinding {
                        source_simthing_id: id(8),
                        rf_participant: id(8),
                        available: 5,
                        precedence: 0,
                    }],
                )
                .unwrap();
            let n1_launch_ns = next_launch.elapsed().as_nanos();
            timeline.mark(&ctx.device, &ctx.queue, 8);
            tree.runtime
                .finish_generation(
                    &mut next_permit,
                    GenerationStamp::new(next_generation.get() + 1),
                )
                .unwrap();
            pending.push((timeline, root, child, next, demands, serde_json::json!({"tree": tree_index,
                "repetition": repetition, "phase": if repetition < 2 {"warmup"} else {"sample"},
                "generation": generation.get(), "intake_permit_host_ns": intake_permit_ns,
                "tr_host_submission_ns": tr_host_submission_ns, "r_launch_host_ns": r_launch_ns,
                "exact_launch_host_ns": exact_launch_ns, "spatial_launch_host_ns": spatial_launch_ns,
                "temporal_launch_host_ns": temporal_launch_ns, "barrier_host_ns": barrier_host_ns,
                "n1_launch_host_ns": n1_launch_ns})));
        }
        for (tree, (timeline, root, child, next, demands, mut row)) in trees.iter_mut().zip(pending)
        {
            let egress = Instant::now();
            let t = timeline.read_ns(&ctx.device, &ctx.queue).unwrap();
            row["timestamp_egress_host_ns"] = serde_json::json!(egress.elapsed().as_nanos());
            let materialize = Instant::now();
            let mut products = Vec::new();
            for ticket in [&root, &child, &next] {
                let p = tree
                    .runtime
                    .materialize(&tree.state, &tree.qualification, &mut tree.schedule, ticket)
                    .unwrap();
                assert_eq!(p.len(), 1);
                products.push((p[0].granted(), p[0].unresolved(), p[0].generation().get()));
            }
            let g = row["generation"].as_u64().unwrap() as u32;
            assert_eq!(products, [(4, 6, g), (4, 0, g), (5, 3, g + 1)]);
            let minted = tree
                .runtime
                .readback_temporal_demands_for_proof(&tree.state, &tree.qualification, &demands)
                .unwrap();
            assert!(minted[0].is_successful());
            assert_eq!(minted[0].quantity(), 8);
            row["economic_proof_egress_host_ns"] =
                serde_json::json!(materialize.elapsed().as_nanos());
            row["gpu_intervals_ns"] = serde_json::json!({"tr_enclosing": t[4]-t[0], "r": t[1]-t[0],
                "exact_projection_and_append": t[2]-t[1], "spatial_settlement_consumption_and_append": t[3]-t[2],
                "temporal_preparation": t[4]-t[3], "permit_boundary_queue_gap": t[5]-t[4],
                "n1_execution": t[8]-t[5], "n1_r": t[7]-t[6]});
            row["signed_host_submission_minus_tr_ns"] = serde_json::json!(
                row["tr_host_submission_ns"].as_u64().unwrap() as f64 - (t[4] - t[0])
            );
            row["timestamps_relative_ns"] = serde_json::json!(t);
            row["products"] = serde_json::json!(products);
            println!("M3_RAW {}", row);
        }
    }
}
