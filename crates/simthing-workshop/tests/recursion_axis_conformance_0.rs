//! RECURSION-AXIS-CONFORMANCE-0 focused production-resident referee.

use simthing_core::{
    DimensionRegistry, GenerationStamp, IntegrationSchedule, SimThing, SimThingId, TreeRealmId,
};
use simthing_driver::resident_clearing_runtime::{
    ResidentClearingBatchBinding, ResidentClearingRuntime,
};
use simthing_gpu::{GpuContext, SlotAllocator};

fn loaded_tree(generation: u32) -> SimThing {
    serde_json::from_str(&format!(
        r#"{{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [],
            "spawned_generation": {generation}
        }}"#
    ))
    .expect("persisted resident fixture")
}

fn admit_runtime(
    gpu: &GpuContext,
    realm: u128,
    generation: u32,
    lane_capacity: u32,
) -> (ResidentClearingRuntime, IntegrationSchedule) {
    let tree = loaded_tree(generation);
    let registry = DimensionRegistry::new();
    let mut residency = SlotAllocator::new();
    residency
        .install_initial_tree(&tree)
        .expect("tree-local residency");
    let mut schedule = IntegrationSchedule::new();
    schedule
        .admit_resident_live_head(lane_capacity * 2)
        .expect("bounded resident live head");
    let runtime = ResidentClearingRuntime::admit(
        gpu,
        TreeRealmId::from_u128(realm).expect("nonzero realm"),
        &tree,
        &registry,
        &residency,
        &schedule,
        GenerationStamp::new(generation),
        lane_capacity,
    )
    .expect("qualified production resident executor");
    (runtime, schedule)
}

fn run_root(
    gpu: &GpuContext,
    realm: u128,
    generation: u32,
    rows: &[ResidentClearingBatchBinding],
) -> Vec<(u32, u32, u32)> {
    let (mut runtime, mut schedule) = admit_runtime(gpu, realm, generation, rows.len() as u32);
    let ticket = runtime
        .dispatch(
            &mut schedule,
            SimThingId::from_session_raw(7),
            GenerationStamp::new(generation),
            Some(rows),
        )
        .expect("production resident root dispatch");
    runtime
        .materialize(&mut schedule, ticket)
        .expect("resident result materializes")
        .into_iter()
        .map(|product| {
            (
                product.source_simthing_id().raw(),
                product.granted(),
                product.unresolved(),
            )
        })
        .collect()
}

#[test]
fn e6_zero_basis_high_band_reproduces_feasible_supply_stranding() {
    let gpu = GpuContext::new_blocking().expect("real GPU for E6 falsifier");
    let products = run_root(
        &gpu,
        0x15_05_e6_01,
        41,
        &[ResidentClearingBatchBinding {
            source_simthing_id: SimThingId::from_session_raw(8),
            requested: 4,
            available: 4,
            precedence: 0,
            continuous_weight: 0.0,
        }],
    );

    assert_eq!(products, vec![(8, 0, 4)]);
    println!("E6 zero-basis-high-band reproduced: available=4 products={products:?} stranded=4");
}

#[test]
fn e6_mixed_bands_reproduce_request_reserved_stranding() {
    let gpu = GpuContext::new_blocking().expect("real GPU for E6 falsifier");
    let products = run_root(
        &gpu,
        0x15_05_e6_02,
        43,
        &[
            ResidentClearingBatchBinding {
                source_simthing_id: SimThingId::from_session_raw(8),
                requested: 4,
                available: 4,
                precedence: 0,
                continuous_weight: 0.0,
            },
            ResidentClearingBatchBinding {
                source_simthing_id: SimThingId::from_session_raw(9),
                requested: 4,
                available: 4,
                precedence: 1,
                continuous_weight: 1.0,
            },
        ],
    );

    assert_eq!(products, vec![(8, 0, 4), (9, 0, 4)]);
    println!(
        "E6 mixed-band stranding reproduced: available=4 products={products:?} stranded=4 despite serviceable lower band"
    );
}
