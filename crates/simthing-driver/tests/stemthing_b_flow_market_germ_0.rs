//! STEMTHING-B-FLOW-MARKET-GERM-0 standing integration witness.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Cursor;

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    deliver_routed_overlay, AccumulatorOp, AncestorStandingPolicyView, AuthoredSeamStaleness,
    CombineFn, CompiledAccumulatorOpPlan, ConsumeMode, DimensionRegistry, EmitOnThresholdBuffer,
    EmitOnThresholdRegistration, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlPerProgramCap, EmlTreeId, GateSpec, GenerationStamp, GenerationStamped,
    IntegrationSchedule, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, ScaleSpec, SimProperty, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SlotIndex, SourceSpec, StructuralScalarChannel, SubFieldRole, ThresholdDirection,
    TransformOp,
};
use simthing_driver::{
    compile_action_band_gpu_execution_with_native_lanes, compile_gu_yang_n4_field_sweeps,
    compile_palma_n4_field_sweep, ActionBandActiveInstance, ActionBandNativeLaneAdmission,
    GuYangN4FieldSweepSpec, PalmaN4FieldSweepSpec,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, emit_on_threshold_registrations_to_gpu, wgpu,
    AccumulatorOpSession, ActionBandEmissionBindingGpu, ActionBandGpuExecution, FieldSweepSession,
    GpuContext, PackedAccumulatorUpload, PackedThresholdUpload, SlotAllocator,
};
use simthing_sim::{
    BoundaryDeltaEntry, ReplayDriver, ReplayFrame, ReplayReader, ReplaySnapshot, ReplayWriter,
    SimRuntimeTree, ThresholdRegistry,
};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    clear_stamped_owner_channels, reduce_owner_channel_rf, replay_async_owner_channel_rf_seam,
    resolve_effective_clearing_weights, ActionBandAdmissionBudgetSpec, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandConservedProgressBindingSpec,
    ActionBandConservedProgressBoundSourceSpec, ActionBandRequirementSemantics,
    ActionBandSessionBuildDoor, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec, AdmittedSpecializationFlowMarket, AsyncOwnerChannelRfSeam,
    AuthoredClearingProgram, ClearingRemainderAuthority, ClearingWeightOverrideSpec,
    ConservedOfferingSpec, ConstrainedClaim, ConstrainedGrant, ConstrainedSupply,
    DrawAuthorizationError, DrawEnvelopeTemplateSpec, GrantReleaseCause, MarketGrantRecord,
    OfferingPriceVectorSpec, OwnerChannelRfOwnAggregate, ParentRfIntegrationState, ResourceKey,
    RuntimeOwnerSiloDemandBucket, ScalarBoundDirection, ScopeId, SpecializationFlowMarketSpec,
};

static GPU_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

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

fn demand(
    scope: &simthing_spec::OwnerChannelScopeKey,
    source: SimThingId,
    requested: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested,
        priority: 0,
        source_simthing_id_raw: Some(source.raw()),
    }
}

fn price_program() -> AuthoredClearingProgram {
    use simthing_core::eml_nodes::{opcode, EmlNode};

    AuthoredClearingProgram::new(
        TransformOp::admit_eml(
            vec![EmlNode {
                opcode: opcode::PARAM,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            }],
            EmlPerProgramCap::DEFAULT,
        )
        .expect("existing EML PARAM program"),
    )
}

fn market_spec() -> SpecializationFlowMarketSpec {
    SpecializationFlowMarketSpec {
        specialization_profile_id: "session-root".into(),
        offerings: vec![
            ConservedOfferingSpec {
                id: "residency-claim".into(),
                resource_key: ResourceKey::new("residency-slots"),
                price: OfferingPriceVectorSpec {
                    unit_cost: 2.0,
                    default_clearing_weight: 1.0,
                },
            },
            ConservedOfferingSpec {
                id: "compute-claim".into(),
                resource_key: ResourceKey::new("compute-quanta"),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.5,
                    default_clearing_weight: 1.0,
                },
            },
        ],
        draw_envelopes: vec![
            DrawEnvelopeTemplateSpec {
                id: "residency-draw".into(),
                offering_refs: vec!["residency-claim".into()],
                lifecycle_trigger_refs: vec!["while-resident".into()],
                min_quantity: 1,
                max_quantity: 4,
            },
            DrawEnvelopeTemplateSpec {
                id: "compute-draw".into(),
                offering_refs: vec!["compute-claim".into()],
                lifecycle_trigger_refs: vec!["while-executing".into()],
                min_quantity: 1,
                max_quantity: 8,
            },
        ],
    }
}

struct ClearedFixture {
    market: AdmittedSpecializationFlowMarket,
    granter: SimThingId,
    compute_a: SimThingId,
    compute_grant: ConstrainedGrant,
}

fn clear_two_markets() -> ClearedFixture {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let mut granter = SimThing::new(SimThingKind::Custom("granter".into()), 0);
    let compute_a = SimThing::new(SimThingKind::Custom("worker".into()), 0);
    let compute_b = SimThing::new(SimThingKind::Custom("worker".into()), 0);
    let resident = SimThing::new(SimThingKind::Custom("resident".into()), 0);
    let granter_id = granter.id;
    let compute_a_id = compute_a.id;
    let compute_b_id = compute_b.id;
    let resident_id = resident.id;
    granter.add_child(compute_a);
    granter.add_child(compute_b);
    granter.add_child(resident);
    root.add_child(granter);

    let authored_market = market_spec();
    let encoded = serde_json::to_value(&authored_market).unwrap();
    assert_eq!(
        serde_json::from_value::<SpecializationFlowMarketSpec>(encoded.clone()).unwrap(),
        authored_market
    );
    let mut per_type_delta_mutant = encoded;
    per_type_delta_mutant["offerings"][0]["price"]["per_type_delta"] = serde_json::json!(1.0);
    assert!(
        serde_json::from_value::<SpecializationFlowMarketSpec>(per_type_delta_mutant).is_err(),
        "sealed price vector has no per-type delta authoring slot"
    );
    let active_triggers = BTreeSet::from(["while-resident".into(), "while-executing".into()]);
    let admitted = admit_specialization_flow_market(
        &simthing_core::seed_profiles(),
        &active_triggers,
        authored_market,
    )
    .expect("profile-attached offering/Draw data admits once");
    assert_eq!(admitted.specialization_profile_id(), "session-root");

    let compute_weights = resolve_effective_clearing_weights(
        &root,
        admitted
            .offering("compute-claim")
            .unwrap()
            .price
            .default_clearing_weight,
        &[ClearingWeightOverrideSpec {
            simthing_id: compute_a_id,
            value_program: TransformOp::multiply(2.0),
        }],
    )
    .expect("6.0-shaped inherited EML weight resolution");
    assert_eq!(compute_weights[&compute_a_id], 2.0);
    assert_eq!(compute_weights[&compute_b_id], 1.0);

    let rows = vec![
        own(granter_id, "compute-quanta", 5, 0),
        own(granter_id, "residency-slots", 2, 0),
        own(compute_a_id, "compute-quanta", 0, 4),
        own(compute_b_id, "compute-quanta", 0, 4),
        own(resident_id, "residency-slots", 0, 3),
    ];
    let reduced = reduce_owner_channel_rf(&root, &rows, GenerationStamp::new(12))
        .expect("two resources reduce through the same RF germ");
    let scopes: BTreeMap<_, _> = reduced
        .product()
        .buckets
        .iter()
        .map(|bucket| (bucket.scope.resource_key.as_str(), &bucket.scope))
        .collect();
    let authored = vec![
        admitted
            .authorize_draw(
                "compute-draw",
                "compute-claim",
                demand(scopes["compute-quanta"], compute_a_id, 4),
                compute_weights[&compute_a_id],
                &active_triggers,
            )
            .unwrap(),
        admitted
            .authorize_draw(
                "compute-draw",
                "compute-claim",
                demand(scopes["compute-quanta"], compute_b_id, 4),
                compute_weights[&compute_b_id],
                &active_triggers,
            )
            .unwrap(),
        admitted
            .authorize_draw(
                "residency-draw",
                "residency-claim",
                demand(scopes["residency-slots"], resident_id, 3),
                1.0,
                &active_triggers,
            )
            .unwrap(),
    ];
    assert!(matches!(
        admitted.authorize_draw(
            "residency-draw",
            "compute-claim",
            demand(scopes["compute-quanta"], compute_a_id, 4),
            1.0,
            &active_triggers,
        ),
        Err(DrawAuthorizationError::OfferingNotAuthorized { .. })
    ));
    assert!(matches!(
        admitted.authorize_draw(
            "compute-draw",
            "compute-claim",
            demand(scopes["compute-quanta"], compute_a_id, 4),
            1.0,
            &BTreeSet::new(),
        ),
        Err(DrawAuthorizationError::InactiveLifecycleTrigger { .. })
    ));

    let results = clear_stamped_owner_channels(&reduced, &authored, &price_program(), granter_id)
        .expect("both markets use the existing constrained clear");
    assert_eq!(results.len(), 2);
    assert!(results
        .iter()
        .all(|result| result.available_before == result.granted_total + result.remaining_after));
    let compute = results
        .iter()
        .find(|result| result.scope.resource_key.as_str() == "compute-quanta")
        .unwrap();
    let residency = results
        .iter()
        .find(|result| result.scope.resource_key.as_str() == "residency-slots")
        .unwrap();
    assert_eq!(compute.granted_total, 5);
    assert_eq!(residency.granted_total, 2);
    let compute_grant = compute
        .grants
        .iter()
        .find(|grant| grant.source_simthing_id == compute_a_id)
        .unwrap()
        .clone();
    assert_eq!(compute_grant.granted, 4);
    let band = admitted.quantize_value("compute-claim", 5.0).unwrap();
    assert_eq!((band.n, band.r), (3, 0.5));
    assert!(admitted.quantize_value("absent-offering", 5.0).is_err());

    // Equal fractional ties rotate by canonical id under granter generation.
    let tie_scope = simthing_spec::OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("alpha"),
        resource_key: ResourceKey::new("tie"),
        scope_id: ScopeId::from_boundary(granter_id),
    };
    let tie_claims: Vec<_> = [compute_a_id, compute_b_id]
        .into_iter()
        .map(|id| ConstrainedClaim::from_runtime_demand(&demand(&tie_scope, id, 1), 1.0).unwrap())
        .collect();
    let clear_at = |generation| {
        clear_constrained_claims_at_generation(
            &[ConstrainedSupply {
                scope: tie_scope.clone(),
                available: 1,
            }],
            &tie_claims,
            &AuthoredClearingProgram::new(TransformOp::set(1.0)),
            ClearingRemainderAuthority {
                granter: granter_id,
                generation: GenerationStamp::new(generation),
            },
        )
        .unwrap()[0]
            .grants
            .iter()
            .find(|grant| grant.granted == 1)
            .unwrap()
            .source_simthing_id
    };
    assert_ne!(clear_at(12), clear_at(13));

    let third = SimThingId::from_session_raw(90_003);
    let unequal_claims: Vec<_> = [(compute_a_id, 1), (compute_b_id, 2), (third, 3)]
        .into_iter()
        .map(|(id, requested)| {
            ConstrainedClaim::from_runtime_demand(&demand(&tie_scope, id, requested), 1.0).unwrap()
        })
        .collect();
    let largest_remainder = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: tie_scope,
            available: 2,
        }],
        &unequal_claims,
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        ClearingRemainderAuthority {
            granter: granter_id,
            generation: GenerationStamp::new(12),
        },
    )
    .unwrap();
    let grants: BTreeMap<_, _> = largest_remainder[0]
        .grants
        .iter()
        .map(|grant| (grant.source_simthing_id, grant.granted))
        .collect();
    assert_eq!(grants[&compute_a_id], 0);
    assert_eq!(grants[&compute_b_id], 1, "largest fractional residual wins");
    assert_eq!(grants[&third], 1);

    ClearedFixture {
        market: admitted,
        granter: granter_id,
        compute_a: compute_a_id,
        compute_grant,
    }
}

fn standing(origin: SimThingId, amount: f32) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(77),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::set(amount))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

#[test]
fn two_markets_draw_clear_costband_and_rotate_exact_ties() {
    let fixture = clear_two_markets();
    assert_eq!(
        fixture.compute_grant.scope.resource_key.as_str(),
        "compute-quanta"
    );
}

#[test]
fn detached_grant_lifecycle_conserves_and_replays_on_the_existing_stamped_seam() {
    let fixture = clear_two_markets();
    let initial = fixture
        .market
        .record_cleared_grant(
            fixture.granter,
            "compute-claim",
            &fixture.compute_grant,
            GenerationStamp::new(12),
        )
        .unwrap();
    assert_eq!(initial.retained_after_detachment(), initial);

    let mut renewed = initial.clone();
    renewed
        .renew_from_clearance(&fixture.compute_grant, GenerationStamp::new(13))
        .unwrap();
    let total_cleared = renewed.quantity();
    let revoked = renewed.revoke(1).unwrap();
    assert_eq!(revoked.scope, fixture.compute_grant.scope);
    let left = SimThingId::from_session_raw(70_001);
    let right = SimThingId::from_session_raw(70_002);
    let partition = renewed
        .partition_for_fission(
            &[(left, 3), (right, total_cleared - 4)],
            GenerationStamp::new(14),
        )
        .unwrap();
    assert_eq!(
        partition
            .iter()
            .map(MarketGrantRecord::quantity)
            .sum::<u32>(),
        total_cleared - 1
    );
    let fused = MarketGrantRecord::transfer_for_fusion(
        partition,
        SimThingId::from_session_raw(70_003),
        GenerationStamp::new(15),
    )
    .unwrap();
    let terminated = fused
        .clone()
        .terminate(GrantReleaseCause::ExplicitTermination);
    assert_eq!(terminated.scope, fixture.compute_grant.scope);
    assert_eq!(revoked.quantity + terminated.quantity, total_cleared);
    for cause in [GrantReleaseCause::Death, GrantReleaseCause::Dissolution] {
        let release = fused.clone().terminate(cause);
        assert_eq!(release.quantity, fused.quantity());
        assert_eq!(release.cause, cause);
    }

    // The provisioned child is now a detached, independently reduced tree.
    let mut detached = SimThing::new(SimThingKind::Custom("detached-executor".into()), 13);
    detached.id = fixture.compute_a;
    bind_owner(&mut detached, &OwnerRef::new("alpha"));
    let product = reduce_owner_channel_rf(
        &detached,
        &[own(detached.id, "compute-quanta", 1, 2)],
        GenerationStamp::new(13),
    )
    .expect("detached tree executes its own RF generation");
    let mut seam = AsyncOwnerChannelRfSeam::admit(AuthoredSeamStaleness::new(4));
    let mut schedule = IntegrationSchedule::new();
    let mut parent = ParentRfIntegrationState::default();
    seam.enqueue_reduce_up(&product).unwrap();
    seam.apply_parent_generation_barrier(GenerationStamp::new(15), &mut parent, &mut schedule)
        .unwrap();

    let renewal_view = GenerationStamped::stamp(
        GenerationStamp::new(14),
        AncestorStandingPolicyView::new(
            OwnerRef::new("alpha"),
            vec![standing(fixture.granter, total_cleared as f32)],
        ),
    );
    let revocation_view = GenerationStamped::stamp(
        GenerationStamp::new(15),
        AncestorStandingPolicyView::new(
            OwnerRef::new("alpha"),
            vec![standing(fixture.granter, fused.quantity() as f32)],
        ),
    );
    seam.stage_ancestor_standing_view(renewal_view.clone());
    seam.apply_child_generation_barrier(GenerationStamp::new(15), &mut schedule)
        .unwrap();
    seam.stage_ancestor_standing_view(revocation_view.clone());
    seam.apply_child_generation_barrier(GenerationStamp::new(16), &mut schedule)
        .unwrap();
    assert_eq!(
        seam.standing_view(GenerationStamp::new(16))
            .unwrap()
            .product(),
        revocation_view.product()
    );
    seam.check_conservation().unwrap();

    let replay = replay_async_owner_channel_rf_seam(
        &schedule,
        &[product],
        &[revocation_view.clone(), renewal_view.clone()],
    )
    .expect("one 6.1 schedule replays both directions");
    assert_eq!(replay.parent_state, parent);
    assert_eq!(replay.standing_reads, vec![renewal_view, revocation_view]);
}

#[test]
fn germ_absence_census_and_lifecycle_mutants_red() {
    let market_source = include_str!("../../simthing-spec/src/spec/flow_market.rs");
    for forbidden in [
        "struct FlowMarketManager",
        "struct MarketAllocator",
        "struct MarketLedger",
        "struct MarketHistory",
        "struct MarketTelemetry",
        "FieldSweepRegistration",
        "pub fn clear_",
        "ReplayWriter",
        "IntegrationSchedule",
    ] {
        assert!(
            !market_source.contains(forbidden),
            "new mechanism/authority reach must RED: {forbidden}"
        );
    }
    let clearing_source = include_str!("../../simthing-spec/src/spec/constrained_clearing.rs");
    assert_eq!(
        clearing_source
            .matches("pub fn clear_constrained_claims_at_generation(")
            .count(),
        1,
        "generation rotation must extend the one landed clearing engine"
    );

    let fixture = clear_two_markets();
    let record = fixture
        .market
        .record_cleared_grant(
            fixture.granter,
            "compute-claim",
            &fixture.compute_grant,
            GenerationStamp::new(12),
        )
        .unwrap();
    assert!(fixture
        .market
        .record_cleared_grant(
            fixture.granter,
            "residency-claim",
            &fixture.compute_grant,
            GenerationStamp::new(12),
        )
        .is_err());
    let mut over_revoke = record.clone();
    assert!(over_revoke.revoke(record.quantity() + 1).is_err());
    assert!(record
        .clone()
        .partition_for_fission(
            &[(SimThingId::from_session_raw(80_001), record.quantity() - 1)],
            GenerationStamp::new(13),
        )
        .is_err());
    let duplicate = SimThingId::from_session_raw(80_002);
    assert!(record
        .partition_for_fission(
            &[
                (duplicate, 1),
                (duplicate, fixture.compute_grant.granted - 1)
            ],
            GenerationStamp::new(13),
        )
        .is_err());
}

fn storage_buffer(ctx: &GpuContext, label: &str, byte_len: u64) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: byte_len,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

#[test]
fn non_residency_market_executes_rf_costband_full_triad_action_and_existing_replay() {
    let _gpu = GPU_MUTEX
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("STEMTHING-B-FLOW-MARKET-GERM-0 GPU leg skipped (no adapter)");
        return;
    };
    let cleared = clear_two_markets();
    let granted = cleared.compute_grant.granted as f32;
    assert!(
        granted > 0.0,
        "field seed comes from actual compute clearing"
    );

    let mut registry = DimensionRegistry::new();
    let mut register = |name: &str| {
        let property = registry.register(SimProperty::simple("stemthing-b-market", name, 1));
        registry
            .column_range(property)
            .col_for_role(&SubFieldRole::Amount, &registry.property(property).layout)
            .unwrap()
    };
    let palma_d = register("palma-potential");
    let palma_w = register("palma-impedance");
    let value = register("gu-yang-value");
    let conductance = register("gu-yang-conductance");
    let rf_claim = register("actionband-rf-claim");
    let rf_result = register("rf-disbursement");
    let response = register("ordinary-overlay-response");
    let n_dims = registry.total_columns as u32;

    let palma = compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims,
        d_col: palma_d,
        w_col: palma_w,
        destination_slot: SlotIndex::new(0),
        inf_sentinel: f32::MAX,
    })
    .unwrap();
    let gu_yang = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 1,
        n_dims,
        value_col: value,
        conductance_col: conductance,
        saturation: 1.0,
        chi: 0.25,
        dt: 1.0,
    })
    .unwrap();
    let registrations = vec![palma, gu_yang[0].clone(), gu_yang[1].clone()];

    let mut initial = vec![0.0; 2 * n_dims as usize];
    initial[palma_d.raw()] = 0.0;
    initial[n_dims as usize + palma_d.raw()] = f32::MAX;
    initial[palma_w.raw()] = 1.0;
    initial[n_dims as usize + palma_w.raw()] = 1.0;
    initial[value.raw()] = granted * 0.025;
    initial[n_dims as usize + value.raw()] = granted * 0.2;

    let mut palma_field = FieldSweepSession::new(&ctx, &registrations[0]).unwrap();
    palma_field.upload_values(&ctx, &initial).unwrap();
    palma_field
        .dispatch_chain(&ctx, &registrations[..1], 1)
        .unwrap();
    let palma_resident = storage_buffer(
        &ctx,
        "stemthing_b_market_palma_resident",
        std::mem::size_of_val(initial.as_slice()) as u64,
    );
    palma_field.copy_values_to_buffer(&ctx, &palma_resident);
    let mut field = FieldSweepSession::new(&ctx, &registrations[1]).unwrap();
    field.upload_values_from_buffer(&ctx, &palma_resident);
    field.dispatch_chain(&ctx, &registrations[1..], 1).unwrap();
    let resident = storage_buffer(
        &ctx,
        "stemthing_b_market_resident_field",
        std::mem::size_of_val(initial.as_slice()) as u64,
    );
    field.copy_values_to_buffer(&ctx, &resident);

    // STEAD consumes the Gu-Yang field through the existing Phase-5 band door.
    let threshold = EmitOnThresholdRegistration {
        slot: SlotIndex::new(0),
        col: value,
        threshold: 0.15,
        direction: ThresholdDirection::Upward,
        event_kind: 11_200,
        buffer: EmitOnThresholdBuffer::Values,
    };
    let mut phase5 = AccumulatorOpSession::new_attached(&ctx, 2, n_dims, 1);
    phase5.upload_previous_values(&ctx, &initial);
    phase5
        .copy_values_prefix_from_buffer(&ctx, &resident, 0, 0, resident.size())
        .unwrap();
    phase5
        .upload_packed_threshold_ops(
            &ctx,
            &PackedThresholdUpload::from_registrations(&emit_on_threshold_registrations_to_gpu(
                std::slice::from_ref(&threshold),
            ))
            .unwrap(),
        )
        .unwrap();
    phase5.tick(&ctx, 0).unwrap();
    let emissions = phase5.readback_threshold_emissions(&ctx).unwrap();

    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let root_id = root.id;
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        phase5.threshold_registrations(),
        &registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 1, "real Gu-Yang -> STEAD crossing");

    let mut eml = EmlExpressionRegistry::new();
    use simthing_core::eml_nodes::{opcode, EmlNode};
    let node = |opcode| EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    eml.register_formula(
        EmlTreeId(17),
        EmlFormulaMeta {
            tree_id: EmlTreeId(17),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: 4,
            max_stack_depth: 2,
            has_loops: false,
            has_recursion: false,
            display_name: "compute-market-response-2x".into(),
        },
        vec![
            EmlNode {
                opcode: opcode::SLOT_VALUE,
                a: value.raw_u32(),
                ..node(opcode::SLOT_VALUE)
            },
            EmlNode {
                opcode: opcode::LITERAL_F32,
                a: 2.0f32.to_bits(),
                ..node(opcode::LITERAL_F32)
            },
            node(opcode::MUL),
            node(opcode::RETURN_TOP),
        ],
    )
    .unwrap();
    let session_spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: 1,
            dependency_binding_count: 0,
            storage_rows: 1,
            eml_program_count: 1,
            emission_binding_count: 2,
        },
        templates: vec![ActionBandTemplateSpec {
            id: "compute-market-response".into(),
            label: None,
            axis_channels: vec![ActionBandChannelBindingSpec {
                column: value.raw_u32(),
                kind: ActionBandChannelKind::Primitive,
            }],
            target: ActionBandTargetSpec::ScalarBound {
                channel: value.raw_u32(),
                bound: threshold.threshold,
                direction: ScalarBoundDirection::AtLeast,
            },
            velocity: None,
            bands: vec![ActionBandBandSpec {
                threshold_registration_index: 0,
                eml_program: Some(17),
                emission_binding_indices: vec![0, 1],
            }],
            subordinate_template_ids: Vec::new(),
            max_active_subordinates: 0,
            reserved_instance_rows: 1,
            requirement_semantics: ActionBandRequirementSemantics::Ordinary,
        }],
    };
    let mut door = ActionBandSessionBuildDoor::new();
    let frozen = door
        .admit_once_with_conserved_progress_at_session_build(
            &session_spec,
            &[ActionBandConservedProgressBindingSpec {
                template_id: "compute-market-response".into(),
                band_index: 0,
                emission_binding_index: 0,
                bound_source: ActionBandConservedProgressBoundSourceSpec::GuYangRealized,
            }],
            &registry,
            &eml,
            std::slice::from_ref(&threshold),
        )
        .unwrap();
    let rf_plan = CompiledAccumulatorOpPlan {
        slot_count: 2,
        n_dims,
        input_channel: StructuralScalarChannel::new(rf_claim.raw_u32()),
        output_channel: StructuralScalarChannel::new(rf_result.raw_u32()),
        ops: vec![AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: rf_claim,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(0), rf_result)],
        }],
    };
    let native = ActionBandNativeLaneAdmission::from_existing_surfaces(
        &registry,
        &[response],
        std::slice::from_ref(&rf_plan),
        &[],
        &ThresholdRegistry::new(),
    );
    let compiled = compile_action_band_gpu_execution_with_native_lanes(
        frozen,
        &eml,
        &[
            ActionBandEmissionBindingGpu::rf_claim(rf_claim.raw_u32()),
            ActionBandEmissionBindingGpu::property_next(
                response.raw_u32(),
                simthing_gpu::ActionBandPropertyWrite::Set,
            ),
        ],
        &[ActionBandActiveInstance::new(
            frozen.templates()[0].index(),
            SlotIndex::new(0),
            [0.0; 4],
        )],
        &native,
    )
    .unwrap();
    let plan = compiled.into_execution_plan();
    let crossings = plan.crossings_from_sealed(&deltas).unwrap();
    let next = storage_buffer(
        &ctx,
        "stemthing_b_market_action_next",
        std::mem::size_of_val(initial.as_slice()) as u64,
    );
    let mut action = match ActionBandGpuExecution::new(&ctx, plan).unwrap() {
        ActionBandGpuExecution::Active(session) => session,
        ActionBandGpuExecution::Inactive => panic!("admitted market ActionBand must be active"),
    };
    action
        .dispatch_with_native_next(&ctx, &resident, &next, n_dims, &crossings)
        .unwrap();
    let mut rf = AccumulatorOpSession::new(&ctx, rf_plan.slot_count, rf_plan.n_dims);
    rf.copy_values_prefix_from_buffer(&ctx, &next, 0, 0, next.size())
        .unwrap();
    rf.upload_packed_ops(
        &ctx,
        &PackedAccumulatorUpload::from_ops(&rf_plan.ops).unwrap(),
    )
    .unwrap();
    rf.tick(&ctx, 0).unwrap();
    let action_values = rf.readback_full(&ctx).unwrap();
    assert!(action_values[rf_result.raw()] != 0.0);
    assert!(action_values[response.raw()] != 0.0);

    let field_values = field.readback(&ctx).unwrap();
    assert_eq!(field_values[palma_d.raw()].to_bits(), 0.0f32.to_bits());
    assert_eq!(
        field_values[n_dims as usize + palma_d.raw()].to_bits(),
        1.0f32.to_bits(),
        "PALMA potential executes over the admitted topology"
    );

    let snapshot = ReplaySnapshot {
        day: 12,
        root: SimRuntimeTree::admit(root.clone()),
        registry: registry.clone(),
        fission_lineage: Vec::new(),
    };
    let response_overlay = standing(root_id, action_values[response.raw()]);
    let response_overlay_id = response_overlay.id;
    deliver_routed_overlay(&mut root, root_id, response_overlay.clone())
        .expect("ordinary OverlayThing response route");
    let frame = ReplayFrame {
        day: 13,
        entries: vec![
            BoundaryDeltaEntry::BandCrossingDeltasApplied {
                deltas: deltas.clone(),
            },
            BoundaryDeltaEntry::OverlayAttached {
                target: root_id,
                overlay: response_overlay,
            },
        ],
        shadow_values: Some(field_values.clone()),
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    };
    let mut writer = ReplayWriter::new(Vec::new());
    writer.write_snapshot(&snapshot).unwrap();
    writer.write_frame(&frame).unwrap();
    let mut reader = ReplayReader::new(Cursor::new(writer.into_inner()));
    let decoded_snapshot = reader.read_snapshot().unwrap();
    let decoded_frame = reader.next_frame().unwrap().unwrap();
    let mut replay = ReplayDriver::from_snapshot(decoded_snapshot);
    replay.apply_frame(decoded_frame);
    assert_eq!(replay.last_band_crossing_deltas, deltas);
    assert_eq!(replay.shadow_values.as_ref(), Some(&field_values));
    assert!(replay.root.has_overlay(root_id, response_overlay_id));
    eprintln!(
        "STEMTHING-B-FLOW-MARKET-SIGNAL grant={} palma={:08x} guyang={:08x} action={:08x} replay_rows=2",
        cleared.compute_grant.granted,
        field_values[n_dims as usize + palma_d.raw()].to_bits(),
        deltas[0].post_value().to_bits(),
        action_values[rf_result.raw()].to_bits(),
    );
}
