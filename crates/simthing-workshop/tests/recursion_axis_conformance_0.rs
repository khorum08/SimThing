//! RECURSION-AXIS-CONFORMANCE-0 production-resident referee.

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    DimensionRegistry, GenerationStamp, IntegrationSchedule, PersistenceDeformationProgram,
    ResidencyCapacityPartition, SimThing, SimThingId, TransformOp, TreeRealmId,
};
use simthing_driver::produce_runtime_rf_next_generation_demands_for_tick;
use simthing_driver::resident_clearing_runtime::{
    ResidentAuthoredDemand, ResidentClearingBatchBinding, ResidentClearingRuntime,
    ResidentClearingRuntimeError, ResidentPersistenceDeformationBinding,
    ResidentSpatialClaimBinding, ResidentTemporalExecutionBinding,
};
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_spec::{
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim, ConstrainedSupply,
    OwnerChannelScopeKey, PersistenceDeformationBinding, PersistenceDeformationBindings,
    ResourceKey, RuntimeOwnerSiloDemandBucket, RuntimeRfDemandGenerationAuthority, ScopeId,
};

const ROOT: u32 = 7;
const CHILD: u32 = 8;
const DESCENDANT: u32 = 9;

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

fn admit_runtime(
    gpu: &GpuContext,
    realm: u128,
    generation: u32,
    lane_capacity: u32,
    deformation: Option<PersistenceDeformationProgram>,
) -> (ResidentClearingRuntime, IntegrationSchedule) {
    let tree = loaded_tree(generation);
    let registry = DimensionRegistry::new();
    let mut residency = SlotAllocator::new();
    residency
        .install_initial_tree(&tree)
        .expect("tree-local residency");
    let mut schedule = IntegrationSchedule::new();
    schedule
        .admit_resident_live_head(16)
        .expect("bounded resident live head");
    let bindings = deformation
        .map(|program| {
            vec![ResidentPersistenceDeformationBinding {
                source_simthing_id: id(CHILD),
                program,
            }]
        })
        .unwrap_or_default();
    let runtime = ResidentClearingRuntime::admit_with_persistence_deformations(
        gpu,
        TreeRealmId::from_u128(realm).expect("nonzero realm"),
        &tree,
        &registry,
        &residency,
        &schedule,
        GenerationStamp::new(generation),
        lane_capacity,
        &bindings,
    )
    .expect("qualified production resident executor");
    (runtime, schedule)
}

fn economic(products: Vec<simthing_gpu::ResidentConstrainedProduct>) -> Vec<(u32, u32, u32, u32)> {
    products
        .into_iter()
        .map(|product| {
            (
                product.source_simthing_id().raw(),
                product.granted(),
                product.unresolved(),
                product.generation().get(),
            )
        })
        .collect()
}

fn run_immediate(
    gpu: &GpuContext,
    realm: u128,
    rows: &[ResidentClearingBatchBinding],
) -> Vec<(u32, u32, u32, u32)> {
    let (mut runtime, mut schedule) = admit_runtime(gpu, realm, 41, rows.len() as u32, None);
    let ticket = runtime
        .dispatch(&mut schedule, id(ROOT), GenerationStamp::new(41), rows)
        .expect("production resident immediate dispatch");
    economic(runtime.materialize(&mut schedule, ticket).unwrap())
}

#[test]
fn e6_immediate_flow_is_work_conserving_and_commitment_alone_reserves() {
    let gpu = GpuContext::new_blocking().expect("real GPU for E6 referee");
    let rows = [
        ResidentClearingBatchBinding {
            source_simthing_id: id(CHILD),
            requested: 4,
            available: 4,
            precedence: 0,
            continuous_weight: 0.0,
        },
        ResidentClearingBatchBinding {
            source_simthing_id: id(DESCENDANT),
            requested: 4,
            available: 4,
            precedence: 1,
            continuous_weight: 1.0,
        },
    ];
    let work_conserving = run_immediate(&gpu, 0x15_05_e6_01, &rows);
    assert_eq!(work_conserving, vec![(8, 0, 4, 41), (9, 4, 0, 41)]);

    let mixed_band = run_immediate(
        &gpu,
        0x15_05_e6_03,
        &[
            ResidentClearingBatchBinding {
                source_simthing_id: id(ROOT),
                requested: 100,
                available: 10,
                precedence: 0,
                continuous_weight: 0.0,
            },
            ResidentClearingBatchBinding {
                source_simthing_id: id(CHILD),
                requested: 1,
                available: 10,
                precedence: 0,
                continuous_weight: 1.0,
            },
            ResidentClearingBatchBinding {
                source_simthing_id: id(DESCENDANT),
                requested: 9,
                available: 10,
                precedence: 1,
                continuous_weight: 9.0,
            },
        ],
    );
    assert_eq!(
        mixed_band,
        vec![(7, 0, 100, 41), (8, 1, 0, 41), (9, 9, 0, 41)],
        "a zero-basis sibling's request cannot reserve inside a serviceable equality band"
    );

    let (mut runtime, mut schedule) = admit_runtime(&gpu, 0x15_05_e6_02, 41, 2, None);
    let mut commitment = ResidencyCapacityPartition::new(4);
    commitment.issue(3).expect("exact in-flight commitment");
    let reserved = runtime
        .dispatch_with_commitment_partition(
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(41),
            &rows,
            &commitment,
        )
        .expect("one projection over free supply");
    let reserved = economic(runtime.materialize(&mut schedule, reserved).unwrap());
    assert_eq!(reserved, vec![(8, 0, 4, 41), (9, 1, 3, 41)]);

    println!(
        "E6 PASS no-commitment={work_conserving:?} mixed-band={mixed_band:?} in_flight=3 reserved={reserved:?} law=S_next=S-sum(G)"
    );
}

fn cpu_effective_demand(deformation: Option<PersistenceDeformationProgram>) -> u32 {
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("axis-referee"),
        resource_key: ResourceKey::new("quanta"),
        scope_id: ScopeId::new("root-market"),
    };
    let remainder = ClearingRemainderAuthority {
        granter: id(ROOT),
        generation: GenerationStamp::new(50),
    };
    let authority = match deformation {
        Some(program) => RuntimeRfDemandGenerationAuthority::with_persistence_deformations(
            remainder,
            PersistenceDeformationBindings::admit([PersistenceDeformationBinding::new(
                scope.clone(),
                id(CHILD),
                program,
            )])
            .unwrap(),
        ),
        None => RuntimeRfDemandGenerationAuthority::new(remainder),
    };
    let current = RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested: 10,
        priority: 0,
        source_simthing_id_raw: Some(CHILD),
    };
    let authored = RuntimeOwnerSiloDemandBucket {
        requested: 2,
        ..current.clone()
    };
    let claim = ConstrainedClaim::from_runtime_demand(&current, 1.0).unwrap();
    let (cleared, next) = produce_runtime_rf_next_generation_demands_for_tick(
        &authority,
        &[ConstrainedSupply {
            scope,
            available: 4,
        }],
        &[claim],
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        vec![authored],
    )
    .unwrap();
    assert_eq!(
        (cleared[0].granted_total, cleared[0].unresolved_total),
        (4, 6)
    );
    next[0].product().requested
}

#[test]
fn canonical_cross_product_separates_spatial_and_temporal_axes() {
    let gpu = GpuContext::new_blocking().expect("real GPU for canonical referee");
    let (mut runtime, mut schedule) = admit_runtime(&gpu, 0x15_05_c0, 50, 2, None);
    let root = runtime
        .dispatch(
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(50),
            &[ResidentClearingBatchBinding {
                source_simthing_id: id(CHILD),
                requested: 10,
                available: 4,
                precedence: 0,
                continuous_weight: 10.0,
            }],
        )
        .unwrap();
    let child = runtime
        .dispatch_spatial(
            &mut schedule,
            &root,
            id(CHILD),
            GenerationStamp::new(50),
            &[ResidentSpatialClaimBinding {
                source_simthing_id: id(DESCENDANT),
                requested: 4,
                precedence: 0,
                continuous_weight: 4.0,
            }],
        )
        .unwrap();
    let demand = runtime
        .prepare_temporal_demands(
            &root,
            GenerationStamp::new(51),
            &[ResidentAuthoredDemand {
                source_simthing_id: id(CHILD),
                quantity: 2,
            }],
        )
        .unwrap();
    let next = runtime
        .dispatch_temporal(
            &mut schedule,
            &demand,
            id(ROOT),
            GenerationStamp::new(51),
            &[ResidentTemporalExecutionBinding {
                source_simthing_id: id(CHILD),
                available: 5,
                precedence: 0,
                continuous_weight: 8.0,
            }],
        )
        .unwrap();

    let minted = runtime
        .readback_temporal_demands_for_proof(&demand)
        .unwrap();
    assert_eq!(minted.len(), 1);
    assert!(minted[0].is_successful());
    assert_eq!(
        (minted[0].quantity(), minted[0].generation().get()),
        (8, 51)
    );
    assert_eq!(cpu_effective_demand(None), 8);
    assert_ne!(
        root.submission().authority_granter(),
        child.submission().authority_granter()
    );
    assert_ne!(root.semantic_scope_owner(), child.semantic_scope_owner());

    let root_products = economic(runtime.materialize(&mut schedule, root).unwrap());
    let child_products = economic(runtime.materialize(&mut schedule, child).unwrap());
    let next_products = economic(runtime.materialize(&mut schedule, next).unwrap());
    assert_eq!(root_products, vec![(8, 4, 6, 50)]);
    assert_eq!(child_products, vec![(9, 4, 0, 50)]);
    assert_eq!(next_products, vec![(8, 5, 3, 51)]);

    println!(
        "15.5 CROSS-PRODUCT PASS T_s=(G4,U6,N50) child=(granter8,source9,G4,N50) authored_N1=2 effective_N1=8 executes_with_N1_supply=5"
    );
}

#[test]
fn deformation_reads_u_inside_once_mint_and_matches_cpu() {
    let half = PersistenceDeformationProgram::admit(TransformOp::multiply(0.5), 10).unwrap();
    let gpu = GpuContext::new_blocking().expect("real GPU for temporal parity");
    let (mut runtime, mut schedule) = admit_runtime(&gpu, 0x15_05_d0, 50, 1, Some(half.clone()));
    let root = runtime
        .dispatch(
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(50),
            &[ResidentClearingBatchBinding {
                source_simthing_id: id(CHILD),
                requested: 10,
                available: 4,
                precedence: 0,
                continuous_weight: 10.0,
            }],
        )
        .unwrap();
    let demand = runtime
        .prepare_temporal_demands(
            &root,
            GenerationStamp::new(51),
            &[ResidentAuthoredDemand {
                source_simthing_id: id(CHILD),
                quantity: 2,
            }],
        )
        .unwrap();
    let next = runtime
        .dispatch_temporal(
            &mut schedule,
            &demand,
            id(ROOT),
            GenerationStamp::new(51),
            &[ResidentTemporalExecutionBinding {
                source_simthing_id: id(CHILD),
                available: 0,
                precedence: 0,
                continuous_weight: 5.0,
            }],
        )
        .unwrap();
    let minted = runtime
        .readback_temporal_demands_for_proof(&demand)
        .unwrap();
    assert_eq!(minted[0].quantity(), 5);
    assert_eq!(cpu_effective_demand(Some(half)), 5);
    assert_eq!(
        economic(runtime.materialize(&mut schedule, root).unwrap()),
        vec![(8, 4, 6, 50)]
    );
    assert_eq!(
        economic(runtime.materialize(&mut schedule, next).unwrap()),
        vec![(8, 0, 5, 51)]
    );
}

#[test]
fn semantic_scope_and_axis_mutants_are_mechanically_red() {
    let gpu = GpuContext::new_blocking().expect("real GPU for mutant referee");
    let (mut runtime, mut schedule) = admit_runtime(&gpu, 0x15_05_f0, 60, 2, None);
    let parent = runtime
        .dispatch(
            &mut schedule,
            id(ROOT),
            GenerationStamp::new(60),
            &[
                ResidentClearingBatchBinding {
                    source_simthing_id: id(ROOT),
                    requested: 1,
                    available: 2,
                    precedence: 0,
                    continuous_weight: 1.0,
                },
                ResidentClearingBatchBinding {
                    source_simthing_id: id(CHILD),
                    requested: 1,
                    available: 2,
                    precedence: 0,
                    continuous_weight: 1.0,
                },
            ],
        )
        .unwrap();
    let descendant = [ResidentSpatialClaimBinding {
        source_simthing_id: id(DESCENDANT),
        requested: 1,
        precedence: 0,
        continuous_weight: 1.0,
    }];
    assert!(matches!(
        runtime.dispatch_spatial(
            &mut schedule,
            &parent,
            id(CHILD),
            GenerationStamp::new(61),
            &descendant,
        ),
        Err(ResidentClearingRuntimeError::LiveHead(
            simthing_gpu::ResidentLiveHeadError::SpatialGenerationMismatch { .. }
        ))
    ));
    assert!(matches!(
        runtime.dispatch_spatial(
            &mut schedule,
            &parent,
            id(ROOT),
            GenerationStamp::new(60),
            &[ResidentSpatialClaimBinding {
                source_simthing_id: id(CHILD),
                requested: 1,
                precedence: 0,
                continuous_weight: 1.0,
            }],
        ),
        Err(ResidentClearingRuntimeError::LiveHead(
            simthing_gpu::ResidentLiveHeadError::SpatialGranterRetained { .. }
        ))
    ));
    assert!(matches!(
        runtime.dispatch_spatial(
            &mut schedule,
            &parent,
            id(CHILD),
            GenerationStamp::new(60),
            &[ResidentSpatialClaimBinding {
                source_simthing_id: id(CHILD),
                requested: 1,
                precedence: 0,
                continuous_weight: 1.0,
            }],
        ),
        Err(ResidentClearingRuntimeError::SpatialClaimOutsideChildScope { .. })
    ));

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mint = std::fs::read_to_string(
        root.join("crates/simthing-kernel/src/shaders/resident_recursive_intake_transform.wgsl"),
    )
    .unwrap();
    let live_head =
        std::fs::read_to_string(root.join("crates/simthing-gpu/src/resident_clearing_runtime.rs"))
            .unwrap();
    let lawful = |mint: &str, live_head: &str| {
        mint.contains("authored.quantity + result.output")
            && !mint.contains("authored.quantity + input_words[input_base + 2u]")
            && mint.contains("const DEMAND_WORDS: u32 = 4u")
            && !mint.contains("for (var word = 0u; word < PRODUCT_WORDS")
            && live_head.contains("plan.generation() != parent.generation")
            && live_head.contains("plan.authority_granter() == parent.authority_granter")
    };
    assert!(lawful(&mint, &live_head));
    assert!(!lawful(
        &format!("{mint}\nauthored.quantity + input_words[input_base + 2u]"),
        &live_head
    ));
    assert!(!lawful(
        &mint.replace(
            "const DEMAND_WORDS: u32 = 4u",
            "const DEMAND_WORDS: u32 = 8u"
        ),
        &live_head
    ));
    assert!(!lawful(
        &mint,
        &live_head.replace(
            "plan.generation() != parent.generation",
            "plan.generation().get() != parent.generation.get() + 1"
        )
    ));
    assert!(!lawful(
        &mint,
        &live_head.replace(
            "plan.authority_granter() == parent.authority_granter",
            "false"
        )
    ));

    println!("15.5 MUTANTS RED temporal=G+f(U),pseudo-T_s spatial=gen+1,parent-granter semantic=parent-identity");
}

#[test]
fn prepared_demand_does_not_execute_until_n_plus_one_datum_arrives() {
    fn run(gpu: &GpuContext, realm: u128, n1_supply: u32) -> (u32, u32) {
        let (mut runtime, mut schedule) = admit_runtime(gpu, realm, 70, 1, None);
        let current = runtime
            .dispatch(
                &mut schedule,
                id(ROOT),
                GenerationStamp::new(70),
                &[ResidentClearingBatchBinding {
                    source_simthing_id: id(CHILD),
                    requested: 10,
                    available: 4,
                    precedence: 0,
                    continuous_weight: 10.0,
                }],
            )
            .unwrap();
        let prepared = runtime
            .prepare_temporal_demands(
                &current,
                GenerationStamp::new(71),
                &[ResidentAuthoredDemand {
                    source_simthing_id: id(CHILD),
                    quantity: 2,
                }],
            )
            .unwrap();
        assert_eq!(
            economic(runtime.materialize(&mut schedule, current).unwrap()),
            vec![(CHILD, 4, 6, 70)]
        );
        // `n1_supply` is supplied only here: preparation above cannot observe
        // it and cannot have executed N+1 economics.
        let executed = runtime
            .dispatch_temporal(
                &mut schedule,
                &prepared,
                id(ROOT),
                GenerationStamp::new(71),
                &[ResidentTemporalExecutionBinding {
                    source_simthing_id: id(CHILD),
                    available: n1_supply,
                    precedence: 0,
                    continuous_weight: 8.0,
                }],
            )
            .unwrap();
        let product = runtime.materialize(&mut schedule, executed).unwrap()[0];
        (product.granted(), product.unresolved())
    }

    let gpu = GpuContext::new_blocking().expect("real GPU for prepared/executed referee");
    assert_eq!(run(&gpu, 0x15_05_a1, 1), (1, 7));
    assert_eq!(run(&gpu, 0x15_05_a2, 6), (6, 2));
}
