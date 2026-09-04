//! PERSISTENCE-DEFORMATION-PORT-0 terminal capability-restoration referee.

use simthing_clausething::{
    compile_persistence_deformation_script_value, parse_raw_document, raw::RawValue,
};
use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    GenerationStamp, IntegrationSchedule, PersistenceDeformationAdmissionError,
    PersistenceDeformationProgram, SimThing, SimThingId, TransformOp, TreeRealmId,
};
use simthing_driver::produce_runtime_rf_next_generation_demands_for_tick;
use simthing_driver::resident_clearing_runtime::{
    ResidentAuthoredDemand, ResidentClearingBatchBinding, ResidentClearingRuntime,
    ResidentPersistenceDeformationBinding, ResidentTemporalExecutionBinding,
};
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_spec::{
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim, ConstrainedSupply,
    OwnerChannelScopeKey, PersistenceDeformationBinding, PersistenceDeformationBindingError,
    PersistenceDeformationBindings, ResourceKey, RuntimeOwnerSiloDemandBucket,
    RuntimeRfDemandGenerationAuthority, RuntimeRfTickErrorKind, ScopeId,
};

const SOURCE: u32 = 41;

fn scope() -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("persistence-policy"),
        resource_key: ResourceKey::new("quanta"),
        scope_id: ScopeId::new("one-native-port"),
    }
}

fn demand(requested: u32) -> RuntimeOwnerSiloDemandBucket {
    let scope = scope();
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref,
        resource_key: scope.resource_key,
        scope_id: scope.scope_id,
        requested,
        priority: 0,
        source_simthing_id_raw: Some(SOURCE),
    }
}

fn cpu_step(
    generation: u32,
    current_requested: u32,
    next_authored: u32,
    program: PersistenceDeformationProgram,
) -> (u32, u32) {
    let binding =
        PersistenceDeformationBinding::new(scope(), SimThingId::from_session_raw(SOURCE), program);
    let authority = RuntimeRfDemandGenerationAuthority::with_persistence_deformations(
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: GenerationStamp::new(generation),
        },
        PersistenceDeformationBindings::admit([binding]).unwrap(),
    );
    let current = demand(current_requested);
    let claim = ConstrainedClaim::from_runtime_demand(&current, 1.0).unwrap();
    let (cleared, next) = produce_runtime_rf_next_generation_demands_for_tick(
        &authority,
        &[ConstrainedSupply {
            scope: scope(),
            available: 0,
        }],
        &[claim],
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        vec![demand(next_authored)],
    )
    .unwrap();
    (cleared[0].unresolved_total, next[0].product().requested)
}

fn loaded_tree() -> SimThing {
    serde_json::from_str(
        r#"{
            "id": 7,
            "kind": "GameSession",
            "properties": [],
            "resource_parent_edges": [],
            "overlays": [],
            "children": [],
            "spawned_generation": 30
        }"#,
    )
    .unwrap()
}

fn clause_long_chain() -> PersistenceDeformationProgram {
    let source = r#"
        script_value = {
            id = persistence_decay
            base = 0.8
            add = 0
            mult = 1
            floor_at = 0
            ceil_at = 100
            add = 0
            mult = 1
            floor_at = 0
            ceil_at = 100
        }
    "#;
    let document = parse_raw_document(source.as_bytes()).expect("ClauseScript parse");
    let RawValue::Block(root) = &document.root else {
        panic!("ClauseScript root block");
    };
    let (id, program) = compile_persistence_deformation_script_value(&root.properties[0], 100)
        .expect("long modifier chain lowers to admitted persistence EML");
    assert_eq!(id, "persistence_decay");
    assert!(program.value_program().nodes().len() > 16);
    program
}

#[test]
fn cpu_once_mint_identity_decay_and_atomic_refusal() {
    let identity_authority = RuntimeRfDemandGenerationAuthority::new(ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(7),
        generation: GenerationStamp::new(10),
    });
    let empty_bound_authority = RuntimeRfDemandGenerationAuthority::with_persistence_deformations(
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: GenerationStamp::new(10),
        },
        PersistenceDeformationBindings::default(),
    );
    let run_identity = |authority: &RuntimeRfDemandGenerationAuthority| {
        let current = demand(10);
        let claim = ConstrainedClaim::from_runtime_demand(&current, 1.0).unwrap();
        produce_runtime_rf_next_generation_demands_for_tick(
            authority,
            &[ConstrainedSupply {
                scope: scope(),
                available: 4,
            }],
            &[claim],
            &AuthoredClearingProgram::new(TransformOp::set(1.0)),
            vec![demand(2)],
        )
        .unwrap()
    };
    let absent = run_identity(&identity_authority);
    let explicit_empty = run_identity(&empty_bound_authority);
    assert_eq!(absent, explicit_empty);
    assert_eq!(absent.1[0].product().requested, 8);

    let program = clause_long_chain();
    assert_eq!(cpu_step(20, 100, 0, program.clone()), (100, 80));
    assert_eq!(cpu_step(21, 80, 0, program), (80, 64));
    let expiry = PersistenceDeformationProgram::admit(TransformOp::set(0.0), 100).unwrap();
    assert_eq!(cpu_step(22, 50, 7, expiry), (50, 7));
    let saturation_source = r#"
        script_value = {
            id = persistence_saturation
            base = 2
            ceil_at = 100
        }
    "#;
    let saturation_document =
        parse_raw_document(saturation_source.as_bytes()).expect("ClauseScript saturation parse");
    let RawValue::Block(saturation_root) = &saturation_document.root else {
        panic!("ClauseScript saturation root block");
    };
    let (_, saturation) =
        compile_persistence_deformation_script_value(&saturation_root.properties[0], 100)
            .expect("bounded saturation lowers to the same port");
    assert_eq!(cpu_step(23, 80, 0, saturation), (80, 100));

    assert!(matches!(
        PersistenceDeformationProgram::admit(TransformOp::multiply(2.0), 100),
        Err(PersistenceDeformationAdmissionError::MayExceedCap { .. })
    ));
    assert!(matches!(
        PersistenceDeformationProgram::admit(TransformOp::set(f32::NAN), 100),
        Err(PersistenceDeformationAdmissionError::NonFiniteLiteral { .. })
    ));
    let duplicate = PersistenceDeformationBinding::new(
        scope(),
        SimThingId::from_session_raw(SOURCE),
        PersistenceDeformationProgram::admit(TransformOp::multiply(0.5), 100).unwrap(),
    );
    assert_eq!(
        PersistenceDeformationBindings::admit([duplicate.clone(), duplicate]),
        Err(PersistenceDeformationBindingError::DuplicateClaimantBinding)
    );

    // A cap failure occurs inside the already-consumed one mint. No partial
    // demand vector is returned, and retry proves there is no second carry.
    let authority = RuntimeRfDemandGenerationAuthority::with_persistence_deformations(
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: GenerationStamp::new(30),
        },
        PersistenceDeformationBindings::admit([PersistenceDeformationBinding::new(
            scope(),
            SimThingId::from_session_raw(SOURCE),
            PersistenceDeformationProgram::admit(TransformOp::multiply(1.0), 10).unwrap(),
        )])
        .unwrap(),
    );
    let current = demand(11);
    let claim = ConstrainedClaim::from_runtime_demand(&current, 1.0).unwrap();
    let attempt = || {
        produce_runtime_rf_next_generation_demands_for_tick(
            &authority,
            &[ConstrainedSupply {
                scope: scope(),
                available: 0,
            }],
            &[claim.clone()],
            &AuthoredClearingProgram::new(TransformOp::set(1.0)),
            vec![demand(0)],
        )
    };
    assert_eq!(
        attempt().unwrap_err().kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextRejected
    );
    assert_eq!(
        attempt().unwrap_err().kind,
        RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced
    );
}

#[test]
fn production_resident_port_matches_cpu_decay_without_readback() {
    let gpu = GpuContext::new_blocking().expect("qualified resident adapter");
    let tree = loaded_tree();
    let registry = simthing_core::DimensionRegistry::new();
    let mut residency = SlotAllocator::new();
    residency.install_initial_tree(&tree).unwrap();
    let mut schedule = IntegrationSchedule::new();
    schedule.admit_resident_live_head(4).unwrap();
    let program = clause_long_chain();
    let mut runtime = ResidentClearingRuntime::admit_with_persistence_deformations(
        &gpu,
        TreeRealmId::from_u128(0x15_02).unwrap(),
        &tree,
        &registry,
        &residency,
        &schedule,
        GenerationStamp::new(30),
        1,
        &[ResidentPersistenceDeformationBinding {
            source_simthing_id: SimThingId::from_session_raw(SOURCE),
            program,
        }],
    )
    .expect("resident deformation admission");
    let rows = [ResidentClearingBatchBinding {
        source_simthing_id: SimThingId::from_session_raw(SOURCE),
        requested: 100,
        available: 0,
        precedence: 0,
        continuous_weight: 100.0,
    }];
    let n = runtime
        .dispatch(
            &mut schedule,
            SimThingId::from_session_raw(7),
            GenerationStamp::new(30),
            &rows,
        )
        .unwrap();
    let demand_n1 = runtime
        .prepare_temporal_demands(
            &n,
            GenerationStamp::new(31),
            &[ResidentAuthoredDemand {
                source_simthing_id: SimThingId::from_session_raw(SOURCE),
                quantity: 0,
            }],
        )
        .unwrap();
    let n1 = runtime
        .dispatch_temporal(
            &mut schedule,
            &demand_n1,
            SimThingId::from_session_raw(7),
            GenerationStamp::new(31),
            &[ResidentTemporalExecutionBinding {
                source_simthing_id: SimThingId::from_session_raw(SOURCE),
                available: 0,
                precedence: 0,
                continuous_weight: 80.0,
            }],
        )
        .unwrap();
    let minted_n1 = runtime
        .readback_temporal_demands_for_proof(&demand_n1)
        .unwrap();
    assert_eq!(minted_n1[0].quantity(), 80);
    let demand_n2 = runtime
        .prepare_temporal_demands(
            &n1,
            GenerationStamp::new(32),
            &[ResidentAuthoredDemand {
                source_simthing_id: SimThingId::from_session_raw(SOURCE),
                quantity: 0,
            }],
        )
        .unwrap();
    let n2 = runtime
        .dispatch_temporal(
            &mut schedule,
            &demand_n2,
            SimThingId::from_session_raw(7),
            GenerationStamp::new(32),
            &[ResidentTemporalExecutionBinding {
                source_simthing_id: SimThingId::from_session_raw(SOURCE),
                available: 0,
                precedence: 0,
                continuous_weight: 64.0,
            }],
        )
        .unwrap();
    assert!(schedule.entries().is_empty());

    // All three generations were submitted before the first T_s observer maps
    // a byte. Each advance used the ordinary resident once-mint.
    let products = [n, n1, n2]
        .into_iter()
        .map(|ticket| runtime.materialize(&mut schedule, ticket).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        products
            .iter()
            .map(|rows| (rows[0].granted(), rows[0].unresolved()))
            .collect::<Vec<_>>(),
        vec![(0, 100), (0, 80), (0, 64)]
    );
    assert_eq!(cpu_step(30, 100, 0, clause_long_chain()), (100, 80));
    assert_eq!(cpu_step(31, 80, 0, clause_long_chain()), (80, 64));

    println!(
        "PERSISTENCE-DEFORMATION-PORT identity=PASS decay=100->80->64 saturation=80->100 expiry=50->0 resident=PASS cpu-oracle=PASS bounds=PASS atomic-refusal=PASS zero-red=PASS clausescript-long-chain=PASS"
    );
}

#[test]
fn structural_census_has_no_second_lane_or_consequence_reinjection() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let constrained =
        std::fs::read_to_string(root.join("crates/simthing-spec/src/spec/constrained_clearing.rs"))
            .unwrap();
    let runtime =
        std::fs::read_to_string(root.join("crates/simthing-spec/src/spec/runtime_rf_tick.rs"))
            .unwrap();
    let resident = std::fs::read_to_string(
        root.join("crates/simthing-driver/src/resident_clearing_runtime.rs"),
    )
    .unwrap();
    let production = format!("{constrained}\n{runtime}\n{resident}");
    let single_port = |constrained: &str, runtime: &str, production: &str| {
        production.matches("ShadowPersistence").count() == 0
            && constrained
                .matches("carry_unresolved_demand_to_next_generation(")
                .count()
                == 1
            && runtime
                .matches("carry_unresolved_demand_to_next_generation(")
                .count()
                == 1
    };
    assert!(single_port(&constrained, &runtime, &production));
    let planted_shadow = format!("{production}\nstruct ShadowPersistence;");
    assert!(
        !single_port(&constrained, &runtime, &planted_shadow),
        "planted ShadowPersistence lane must RED the single-port census"
    );
    let planted_second_carry =
        format!("{constrained}\nfn carry_unresolved_demand_to_next_generation() {{}}");
    assert!(
        !single_port(&planted_second_carry, &runtime, &production),
        "planted second deformation/carry path must RED the single-port census"
    );
    let consequence = constrained
        .split("pub fn fund_unresolved_persistence")
        .nth(1)
        .unwrap();
    assert!(!consequence.contains("RuntimeOwnerSiloDemandBucket"));
    assert!(!consequence.contains("PersistenceDeformationBindings"));
}
