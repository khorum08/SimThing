//! 15.10 falsifier first: actual ordinary session, real resident dispatch.
use std::collections::BTreeSet;

use simthing_core::{
    bind_owner, ColumnIndex, DimensionRegistry, DiscreteTransferRegistration, OwnerRef,
    PropertyTransformDelta, SimProperty, SimThing, SimThingId, SimThingKind, SpecializationProfile,
    SubFieldRole, TransformOp,
};
use simthing_driver::{
    resident_clearing_runtime::install_default_resident_rf_property,
    resource_economy_compile::{
        ResourceEconomyMaterializationReport, ResourceEconomyRegistrations, ResourceEconomyRegistry,
    },
    resource_economy_sync::ResourceEconomySyncError,
    GrowthEntitlementMarketBinding, Scenario, SessionError, SimSession,
};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, ScopeId, SpecializationFlowMarketSpec, OWNER_FLOW_DEMAND_PROPERTY_ID,
    OWNER_SILO_CURRENT_PROPERTY_ID,
};

fn session_with(
    posture: simthing_core::ClearingExecutionPosture,
    ticks_per_day: u32,
) -> (SimSession, SimThingId) {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let owner = OwnerRef::new("owner/15.10");
    bind_owner(&mut root, &owner);
    root.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(4),
    );
    let mut claimant = SimThing::new(SimThingKind::Cohort, 0);
    claimant.add_property(
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(10),
    );
    let claimant_id = claimant.id;
    let counter = registry.register(SimProperty::simple("abort-proof", "counter", 0));
    claimant.add_property(counter, registry.property(counter).default_value());
    root.add_child(claimant);
    install_default_resident_rf_property(&mut registry, &mut root);
    let root_id = root.id;
    let mut session = SimSession::open_with_clearing_posture(
        Scenario {
            name: "15.10 ordinary post-dispatch abort".into(),
            ticks_per_day,
            max_days: 4,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: vec![simthing_feeder::PatchTransform {
                target: claimant_id,
                delta: PropertyTransformDelta {
                    property_id: counter,
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.25))],
                },
            }],
            install_targets: Default::default(),
        },
        posture,
    )
    .unwrap();
    assert_eq!(session.clearing_execution_posture(), posture);
    let resource = ResourceKey::new("simthing::residency-row-capacity");
    let triggers = BTreeSet::from(["current-boundary".into()]);
    let market = admit_specialization_flow_market(
        &[SpecializationProfile {
            id: "ordinary-flow".into(),
            description: "abort witness".into(),
            requirements: Vec::new(),
        }],
        &triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "ordinary-flow".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "offering".into(),
                resource_key: resource.clone(),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "draw".into(),
                offering_refs: vec!["offering".into()],
                lifecycle_trigger_refs: vec!["current-boundary".into()],
                min_quantity: 1,
                max_quantity: 100,
            }],
        },
    )
    .unwrap();
    session
        .install_growth_entitlement_market(GrowthEntitlementMarketBinding::from_admitted_market(
            market,
            root_id,
            "offering",
            "draw",
            OwnerChannelScopeKey {
                owner_ref: owner,
                resource_key: resource,
                scope_id: ScopeId::from_boundary(root_id),
            },
            triggers,
            AuthoredClearingProgram::new(TransformOp::set(1.0)),
            1.0,
            100,
        ))
        .unwrap();
    (session, claimant_id)
}

fn resident_facts(session: &SimSession) -> Vec<(u32, u32, u32, u32)> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.resident_clearing_fact)
        .map(|fact| {
            (
                fact.generation.get(),
                fact.source_simthing_id_raw,
                fact.granted,
                fact.unresolved,
            )
        })
        .collect()
}

fn malformed_late_refresh(session: &mut SimSession, claimant: SimThingId) {
    // Existing public malformed registration input. The session refresh checks
    // this metadata after resident settlement; no new executor or failpoint.
    session.spec_state.resource_economy_registry = Some(ResourceEconomyRegistry {
        generation: 1,
        registrations: ResourceEconomyRegistrations {
            transfers: vec![DiscreteTransferRegistration {
                source_slot: session
                    .proto
                    .allocator
                    .slot_of(session.scenario.root.id)
                    .unwrap(),
                source_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                target_slot: session.proto.allocator.slot_of(claimant).unwrap(),
                target_col: ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                amount: 1.0,
                order_band: 0,
            }],
            recipes: Vec::new(),
            emissions: Vec::new(),
            emit_on_threshold: Vec::new(),
            report: ResourceEconomyMaterializationReport {
                recipe_output_coefficients: vec![1.0],
                ..Default::default()
            },
        },
    });
}

#[test]
fn ordinary_session_post_dispatch_abort_cannot_reenter_economics() {
    use simthing_core::ClearingExecutionPosture::{CpuVendorizedOracle, ResidentRequired};
    for posture in [ResidentRequired, CpuVendorizedOracle] {
        for ticks_per_day in [1, 3] {
            for recording in [false, true] {
                let (mut session, claimant) = session_with(posture, ticks_per_day);
                malformed_late_refresh(&mut session, claimant);
                let path = std::env::temp_dir().join(format!(
                    "simthing-1510-{}-{posture:?}-{ticks_per_day}-{recording}.ldjson",
                    std::process::id()
                ));
                let first = if recording {
                    session.record_to_path(&path, 1).map(|_| ()).unwrap_err()
                } else {
                    session.run(1).map(|_| ()).unwrap_err()
                };
                let first_facts = resident_facts(&session);
                let first_tick = session.coord.tick_index();
                let first_day = session.coord.day_index();
                let first_values = session.state.read_values();
                let first_schedule = session.integration_schedule().clone();
                println!("15.10 first attempt: posture={posture:?}; ticks_per_day={ticks_per_day}; recording={recording}; error={first:?}; tick={first_tick}; day={first_day}; resident_facts={first_facts:?}");
                assert!(matches!(
                    first,
                    SessionError::ResourceEconomy(
                        ResourceEconomySyncError::RecipeExecutionMetadataMismatch {
                            recipes: 0,
                            coefficients: 1,
                            order_bands: 0
                        }
                    )
                ));
                if posture.is_resident_required() {
                    assert_eq!(
                        first_facts,
                        [(1, claimant.raw(), 4, 6)],
                        "confirmed actual resident economic effect before failure"
                    );
                } else {
                    assert!(first_facts.is_empty());
                    assert!(
                        first_schedule
                            .entries()
                            .iter()
                            .any(|entry| entry.grant_lifecycle_fact.is_some()),
                        "real CPU grant was published"
                    );
                }
                // Cross both entry points repeatedly on the same failed session. No clock
                // changes or reconstructed host/session state between attempts.
                for retry_recording in [false, true, false] {
                    let retry = if retry_recording {
                        session.record_to_path(&path, 1).map(|_| ())
                    } else {
                        session.step_once().map(|_| ())
                    };
                    let retry_facts = resident_facts(&session);
                    let retry_values = session.state.read_values();
                    let changed_cells = first_values
                        .iter()
                        .zip(&retry_values)
                        .filter(|(a, b)| a.to_bits() != b.to_bits())
                        .count();
                    println!("15.10 same-session retry: result={retry:?}; tick={}; day={}; resident_facts={retry_facts:?}; changed_gpu_cells={changed_cells}",session.coord.tick_index(),session.coord.day_index());
                    assert_eq!(
                        session.coord.tick_index(),
                        first_tick,
                        "touched abort must refuse before another hot cycle"
                    );
                    assert_eq!(session.coord.day_index(), first_day);
                    assert_eq!(retry_facts, first_facts);
                    assert_eq!(
                        session.integration_schedule(),
                        &first_schedule,
                        "no extra reservation, record, or stranded placeholder on retry"
                    );
                    assert_eq!(first_values, retry_values);
                    let error = retry.expect_err("touched generation is fail-stop");
                    assert!(
                        error.to_string().contains("fault"),
                        "typed generation fault required: {error:?}"
                    );
                }
                if path.exists() {
                    std::fs::remove_file(&path).unwrap();
                }
            }
        }
    }
}

// Auxiliary door coverage supplements the actual-session falsifier above.
// The arena, GPU executor, tickets, permits and schedule are all production.
#[test]
fn untouched_retry_executes_and_resident_axes_poison_on_abort() {
    use simthing_core::{
        GenerationStamp, IntegrationSchedule, TreeExecutionContextError, TreeRealmId,
    };
    use simthing_driver::resident_clearing_runtime::{
        build_default_resident_arena_registry, ResidentAuthoredDemand,
        ResidentClearingBatchBinding, ResidentClearingRuntime, ResidentClearingRuntimeError,
        ResidentSpatialClaimBinding, ResidentTemporalExecutionBinding,
    };
    use simthing_driver::{resolve_node_columns_for_property, sync_resource_flow_accumulator};
    use simthing_gpu::{GpuContext, SlotAllocator, WorldGpuState};
    let gpu = GpuContext::new_blocking().unwrap();
    // Each axis gets its own aborted generation. Temporal mint and execution
    // are separately the first economic door in their current generation.
    for axis in ["immediate", "spatial", "temporal-mint", "temporal-execute"] {
        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        let mut child = SimThing::new(SimThingKind::Owner, 0);
        let descendant = SimThing::new(SimThingKind::Cohort, 0);
        let (root_id, child_id, descendant_id) = (root.id, child.id, descendant.id);
        child.add_child(descendant);
        root.add_child(child);
        let mut registry = DimensionRegistry::new();
        let property = install_default_resident_rf_property(&mut registry, &mut root);
        let mut residency = SlotAllocator::new();
        residency.install_initial_tree(&root).unwrap();
        let mut schedule = IntegrationSchedule::new();
        schedule.admit_resident_live_head(16).unwrap();
        let arena = build_default_resident_arena_registry(property, &root, &residency, 3).unwrap();
        let columns = resolve_node_columns_for_property(
            &registry,
            property,
            simthing_driver::resident_clearing_runtime::RESIDENT_MARKET_RF_ARENA,
        )
        .unwrap();
        let mut state = WorldGpuState::new(gpu.clone(), &registry, residency.capacity() as u32);
        let mut values = vec![0.0; state.values_len()];
        simthing_gpu::project_tree_to_values(
            &root,
            &registry,
            &residency,
            state.n_dims as usize,
            &mut values,
        );
        state.install_resolved_values_at_boundary(&values);
        let flow = sync_resource_flow_accumulator(&mut state, &registry, &arena, &[], &[]).unwrap();
        state.run_resource_flow_bands(flow.n_bands, 1.0);
        // Same auxiliary arena setup used by the frozen axis referee. This is
        // never used to reconstruct or retry the ordinary SimSession witness.
        let mut values = state.read_values();
        for participant in [child_id, descendant_id] {
            let slot = arena.participant_slot(participant, 0).unwrap();
            values
                [slot.raw() as usize * state.n_dims as usize + columns.allocated_flow_col.raw()] =
                10.0;
        }
        state.install_resolved_values_at_boundary(&values);
        let mut runtime = ResidentClearingRuntime::admit_with_persistence_deformations(
            &gpu,
            TreeRealmId::from_u128(0x1510).unwrap(),
            &root,
            &registry,
            &arena,
            &residency,
            &schedule,
            GenerationStamp::new(7),
            3,
            &[],
        )
        .unwrap();
        let qualification = runtime.market_qualification();
        let rows = [ResidentClearingBatchBinding {
            source_simthing_id: child_id,
            rf_participant: child_id,
            requested: 10,
            available: 4,
            precedence: 0,
        }];
        let untouched = runtime.begin_generation(GenerationStamp::new(7)).unwrap();
        assert!(matches!(
            runtime.dispatch(
                &state,
                &qualification,
                &untouched,
                &mut schedule,
                root_id,
                GenerationStamp::new(8),
                &rows
            ),
            Err(ResidentClearingRuntimeError::ExecutionAuthority(
                TreeExecutionContextError::PermitGenerationMismatch { .. }
            ))
        ));
        drop(untouched);
        let mut permit = runtime.begin_generation(GenerationStamp::new(7)).unwrap();
        let parent = runtime
            .dispatch(
                &state,
                &qualification,
                &permit,
                &mut schedule,
                root_id,
                GenerationStamp::new(7),
                &rows,
            )
            .unwrap();
        let products = runtime
            .materialize(&state, &qualification, &mut schedule, &parent)
            .unwrap();
        assert_eq!((products[0].granted(), products[0].unresolved()), (4, 6));
        let authored = [ResidentAuthoredDemand {
            source_simthing_id: child_id,
            quantity: 2,
        }];
        // Mint at N while healthy so temporal execution itself must touch N+1.
        let early_demand = if axis == "temporal-execute" {
            Some(
                runtime
                    .prepare_temporal_demands(
                        &state,
                        &qualification,
                        &permit,
                        &parent,
                        GenerationStamp::new(8),
                        &authored,
                    )
                    .unwrap(),
            )
        } else {
            None
        };
        runtime
            .finish_generation(&mut permit, GenerationStamp::new(8))
            .unwrap();
        assert!(permit.is_consumed());
        assert!(matches!(
            permit.authorize_economics(),
            Err(TreeExecutionContextError::GenerationPermitAlreadyConsumed { .. })
        ));
        drop(permit);
        let permit = runtime.begin_generation(GenerationStamp::new(8)).unwrap();
        // A second untouched drop after successful finish proves the next seal
        // is clean, before any current-generation effect is authorized.
        drop(permit);
        let permit = runtime.begin_generation(GenerationStamp::new(8)).unwrap();
        match axis {
            "immediate" => {
                let ticket = runtime
                    .dispatch(
                        &state,
                        &qualification,
                        &permit,
                        &mut schedule,
                        root_id,
                        GenerationStamp::new(8),
                        &rows,
                    )
                    .unwrap();
                assert_eq!(
                    runtime
                        .materialize(&state, &qualification, &mut schedule, &ticket)
                        .unwrap()[0]
                        .granted(),
                    4
                );
            }
            "spatial" => {
                // Spatial descent remains in N: establish its parent under the
                // same permit, then prove the child product survives the abort.
                let parent = runtime
                    .dispatch(
                        &state,
                        &qualification,
                        &permit,
                        &mut schedule,
                        root_id,
                        GenerationStamp::new(8),
                        &rows,
                    )
                    .unwrap();
                runtime
                    .materialize(&state, &qualification, &mut schedule, &parent)
                    .unwrap();
                let child = runtime
                    .dispatch_spatial(
                        &state,
                        &qualification,
                        &permit,
                        &mut schedule,
                        &parent,
                        child_id,
                        GenerationStamp::new(8),
                        &[ResidentSpatialClaimBinding {
                            source_simthing_id: descendant_id,
                            rf_participant: descendant_id,
                            requested: 4,
                            precedence: 0,
                        }],
                    )
                    .unwrap();
                assert_eq!(
                    runtime
                        .materialize(&state, &qualification, &mut schedule, &child)
                        .unwrap()[0]
                        .granted(),
                    4
                );
            }
            "temporal-mint" => {
                let demand = runtime
                    .prepare_temporal_demands(
                        &state,
                        &qualification,
                        &permit,
                        &parent,
                        GenerationStamp::new(8),
                        &authored,
                    )
                    .unwrap();
                let minted = runtime
                    .readback_temporal_demands_for_proof(&state, &qualification, &demand)
                    .unwrap();
                assert_eq!(minted[0].quantity(), 8);
                assert!(minted[0].is_successful());
            }
            "temporal-execute" => {
                let ticket = runtime
                    .dispatch_temporal(
                        &state,
                        &qualification,
                        &permit,
                        &mut schedule,
                        early_demand.as_ref().unwrap(),
                        root_id,
                        GenerationStamp::new(8),
                        &[ResidentTemporalExecutionBinding {
                            source_simthing_id: child_id,
                            rf_participant: child_id,
                            available: 5,
                            precedence: 0,
                        }],
                    )
                    .unwrap();
                let products = runtime
                    .materialize(&state, &qualification, &mut schedule, &ticket)
                    .unwrap();
                assert_eq!((products[0].granted(), products[0].unresolved()), (5, 3));
            }
            _ => unreachable!(),
        }
        let before = schedule.clone();
        drop(permit);
        for requested in [8, 9, 8] {
            assert!(
                matches!(runtime.begin_generation(GenerationStamp::new(requested)), Err(ResidentClearingRuntimeError::ExecutionAuthority(TreeExecutionContextError::GenerationFaulted { generation })) if generation == GenerationStamp::new(8))
            );
        }
        assert_eq!(schedule, before);
        println!("15.10 axis={axis}: untouched drop/retry executes G4/U6; finish7->8 clean; touched abort faults8; no mint or advance after abort");
    }
}

#[test]
fn healthy_session_generations_and_migrated_fault_keep_one_seal() {
    use simthing_core::{
        ClearingExecutionPosture, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
        TreeExecutionAuthority, TreeExecutionContextError, TreeGenerationAuthority, TreeRealmId,
    };
    // Stop between hot cycles and alternate the two public execution loops.
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        let (mut session, _) = session_with(posture, 3);
        session.step_once().unwrap();
        assert_eq!(session.coord.tick_index(), 1);
        session.run(1).unwrap();
        assert_eq!(session.coord.day_index(), 1);
        session.step_once().unwrap();
        let path = std::env::temp_dir().join(format!(
            "simthing-1510-healthy-{}-{posture:?}.ldjson",
            std::process::id()
        ));
        session.record_to_path(&path, 1).unwrap();
        session.run(1).unwrap();
        assert_eq!(
            (session.coord.tick_index(), session.coord.day_index()),
            (9, 3)
        );
        if posture.is_resident_required() {
            assert_eq!(
                resident_facts(&session)
                    .iter()
                    .map(|f| f.0)
                    .collect::<Vec<_>>(),
                [1, 2, 3]
            );
        }
        std::fs::remove_file(path).unwrap();
    }
    let tree = SimThing::new(SimThingKind::GameSession, 0);
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(7));
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(1510).unwrap(),
        ExecutionIncarnation::new(1).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &(),
    )
    .unwrap();
    let lease = authority.seal_lease().unwrap();
    let untouched = lease.begin_generation(GenerationStamp::new(7)).unwrap();
    lease
        .verifier()
        .validate_generation(&untouched, GenerationStamp::new(7))
        .unwrap();
    drop(untouched); // successful validation did not touch the generation
    let mut touched = lease.begin_generation(GenerationStamp::new(7)).unwrap();
    touched.authorize_economics().unwrap();
    assert!(matches!(
        lease.finish_generation(&mut touched, GenerationStamp::new(9)),
        Err(TreeExecutionContextError::GenerationAdvanceOutOfSequence { .. })
    ));
    let migrated = lease
        .migrate(ExecutionIncarnation::new(2).unwrap())
        .unwrap();
    assert!(matches!(
        touched.authorize_economics(),
        Err(TreeExecutionContextError::StaleIncarnation { .. })
    ));
    drop(touched);
    assert_eq!(migrated.current_generation(), GenerationStamp::new(7));
    assert!(matches!(
        migrated.begin_generation(GenerationStamp::new(7)),
        Err(TreeExecutionContextError::GenerationFaulted { .. })
    ));
    println!("15.10 healthy multi-tick step/run/record commits1->2->3; validation untouched; migrated touched abort remains faulted at7");
}
