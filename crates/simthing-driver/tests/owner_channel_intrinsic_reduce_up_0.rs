//! OWNER-CHANNEL-INTRINSIC-0 deliverables (d)/(e).
//!
//! All feedstock is inline and synthetic. The proofs exercise arbitrary tree structure and
//! generic resource keys; no asset or corpus participates.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    compile_owner_channel_rf_gpu_proof_plan, prove_owner_channel_rf_cpu_gpu_parity,
};
use simthing_gpu::GpuContext;
use simthing_spec::{
    reconstruct_owner_channel_rf_map, reduce_owner_channel_rf, OwnerChannelRfOwnAggregate,
    ResourceKey,
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

fn three_owner_tree() -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));

    let mut inherited = node();
    let inherited_leaf = node();
    let inherited_leaf_id = inherited_leaf.id;
    inherited.add_child(inherited_leaf);

    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    let crossing_leaf = node();
    let crossing_leaf_id = crossing_leaf.id;
    crossing.add_child(crossing_leaf);

    let mut nested_crossing = node();
    bind_owner(&mut nested_crossing, &OwnerRef::new("gamma"));
    let nested_crossing_id = nested_crossing.id;
    crossing.add_child(nested_crossing);

    let root_id = root.id;
    root.add_child(inherited);
    root.add_child(crossing);

    let rows = vec![
        own(root_id, "ore", 3, 0),
        own(inherited_leaf_id, "ore", 4, 0),
        own(crossing_id, "ore", 5, 1),
        own(crossing_leaf_id, "ore", 2, 3),
        own(nested_crossing_id, "ore", 0, 6),
        own(root_id, "water", 1, 2),
        own(crossing_leaf_id, "water", 8, 1),
        own(nested_crossing_id, "water", 2, 2),
    ];
    (root, rows)
}

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(context) => Some(context),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU adapter is required for owner-channel parity")
        }
        Err(_) => None,
    }
}

#[test]
fn n_owner_container_conserves_and_reconstructs_in_canonical_bucket_order() {
    let (root, rows) = three_owner_tree();
    let report = reduce_owner_channel_rf(&root, &rows).expect("generalized reduce-up");

    assert_eq!(report.owner_count, 3, "one container must admit all owners");
    assert_eq!(report.participant_count, 5);
    assert_eq!(
        report.surplus_total,
        rows.iter().map(|row| row.surplus).sum::<u32>()
    );
    assert_eq!(
        report.deficit_total,
        rows.iter().map(|row| row.deficit).sum::<u32>()
    );

    // The key itself has exactly these three dimensions. Construction would stop compiling if
    // a retired domain-shaped field returned.
    for bucket in &report.buckets {
        let _only_key_dimensions = (
            &bucket.scope.owner_ref,
            &bucket.scope.resource_key,
            &bucket.scope.scope_id,
        );
    }
    assert!(
        report
            .buckets
            .windows(2)
            .all(|pair| pair[0].scope < pair[1].scope),
        "BTreeMap lowering must expose canonical owner/resource/ScopeId order"
    );

    assert_eq!(
        report.stead.crossing_flows.len(),
        2,
        "only beta and nested gamma are ownership crossings"
    );
    assert!(report.stead.crossing_flows.iter().all(|flow| flow
        .resources
        .windows(2)
        .all(|pair| { pair[0].resource_key < pair[1].resource_key })));
    assert_eq!(
        reconstruct_owner_channel_rf_map(&root, &report.stead).expect("reconstruct"),
        report.buckets,
        "crossings plus own aggregates must reconstruct the entire RF map"
    );
}

#[test]
fn retained_owner_state_is_bounded_by_crossings_not_nodes_owners_or_resources() {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("owner-0"));
    let mut ids = vec![root.id];
    let mut cursor = &mut root;
    for depth in 1..128 {
        let mut child = node();
        if depth == 40 {
            bind_owner(&mut child, &OwnerRef::new("owner-1"));
        }
        if depth == 80 {
            bind_owner(&mut child, &OwnerRef::new("owner-2"));
        }
        ids.push(child.id);
        cursor.add_child(child);
        cursor = cursor.children.last_mut().expect("child just added");
    }

    let rows = ids
        .iter()
        .flat_map(|&id| [own(id, "r0", 1, 0), own(id, "r1", 0, 1)])
        .collect::<Vec<_>>();
    let report = reduce_owner_channel_rf(&root, &rows).expect("bounded reduction");

    assert_eq!(report.stead.own_aggregates.len(), 128 * 2);
    assert_eq!(report.stead.crossing_flows.len(), 2);
    assert_eq!(report.owner_count, 3);
    assert_eq!(report.bucket_count, 6);
    assert_eq!(
        reconstruct_owner_channel_rf_map(&root, &report.stead).expect("bounded reconstruct"),
        report.buckets
    );
}

#[test]
fn every_owner_resource_scope_bucket_is_bit_exact_on_cpu_and_gpu() {
    let Some(ctx) = gpu_context() else {
        return;
    };
    let (root, rows) = three_owner_tree();
    let plan = compile_owner_channel_rf_gpu_proof_plan(&root, &rows).expect("compile");
    let parity = prove_owner_channel_rf_cpu_gpu_parity(&ctx, &plan).expect("parity");
    assert_eq!(parity.bucket_count, plan.reduce_up_report.bucket_count);
    assert!(parity.canonical_bucket_ordering);
    assert!(parity.cpu_gpu_bit_exact);
}
