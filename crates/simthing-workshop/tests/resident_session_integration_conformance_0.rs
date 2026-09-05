//! 15.8 exit referee: ordinary SimSession, persistent authored flow claimant.
use simthing_core::{
    bind_owner, AccumulatorRole, DimensionRegistry, IntegrationScheduleRowKind, OwnerRef,
    PersistenceDeformationProgram, SimThing, SimThingId, SimThingKind, SpecializationProfile,
    TransformOp,
};
use simthing_driver::resident_clearing_runtime::{
    install_default_resident_rf_property, ResidentClearingRuntimeError,
};
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession, SpecSessionState};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    PersistenceDeformationBinding, PersistenceDeformationBindings, ResourceKey, ScopeId,
    SpecializationFlowMarketSpec, OWNER_FLOW_DEMAND_PROPERTY_ID, OWNER_SILO_CURRENT_PROPERTY_ID,
};
use std::collections::BTreeSet;

const RESOURCE: &str = "simthing::residency-row-capacity";

fn scope(root: SimThingId, resource: &str) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/15.8"),
        resource_key: ResourceKey::new(resource),
        scope_id: ScopeId::from_boundary(root),
    }
}

fn market(root: SimThingId, resource: &str, max_quantity: u32) -> GrowthEntitlementMarketBinding {
    let triggers = BTreeSet::from(["current-boundary".into()]);
    let admitted = admit_specialization_flow_market(
        &[SpecializationProfile {
            id: "ordinary-flow".into(),
            description: "persistent ordinary owner-flow claimant".into(),
            requirements: Vec::new(),
        }],
        &triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "ordinary-flow".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "offering".into(),
                resource_key: ResourceKey::new(resource),
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
                max_quantity,
            }],
        },
    )
    .unwrap();
    GrowthEntitlementMarketBinding::from_admitted_market(
        admitted,
        root,
        "offering",
        "draw",
        scope(root, resource),
        triggers,
        AuthoredClearingProgram::new(TransformOp::set(1.0)),
        1.0,
        100,
    )
}

fn scenario() -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    bind_owner(&mut root, &OwnerRef::new("owner/15.8"));
    root.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(4),
    );
    let mut claimant = SimThing::new(SimThingKind::Cohort, 0);
    claimant.add_property(
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(10),
    );
    let id = claimant.id;
    root.add_child(claimant);
    let property = install_default_resident_rf_property(&mut registry, &mut root);
    // A second real, independently named arena makes list position observable.
    let mut other = registry.property(property).clone();
    other.namespace = "other".into();
    other.name = "flow".into();
    for field in &mut other.layout.sub_fields {
        if let Some(spec) = &mut field.accumulator_spec {
            match &mut spec.role {
                AccumulatorRole::AllocatedFlow { arena }
                | AccumulatorRole::AllocatorWeight { arena } => *arena = "other-flow".into(),
                _ => {}
            }
        }
    }
    let other_id = registry.register(other);
    let value = registry.property(other_id).default_value();
    root.add_property(other_id, value.clone());
    root.children[0].add_property(other_id, value);
    (
        Scenario {
            name: "15.8 ordinary flow".into(),
            ticks_per_day: 1,
            max_days: 4,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        id,
    )
}

fn clause_policy(base: f32) -> PersistenceDeformationProgram {
    use simthing_clausething::{
        compile_persistence_deformation_script_value, parse_raw_document, raw::RawValue,
    };
    let source = format!("script_value = {{ id = flow_persistence base = {base} }}");
    let doc = parse_raw_document(source.as_bytes()).unwrap();
    let RawValue::Block(root) = &doc.root else {
        panic!("ClauseScript block");
    };
    compile_persistence_deformation_script_value(&root.properties[0], 100)
        .unwrap()
        .1
}

fn install(session: &mut SimSession, claimant: SimThingId, policy: Option<f32>, reverse: bool) {
    let mut arenas = session.spec_state.arena_registry.clone();
    let mut other = arenas.arenas[0].clone();
    other.name = "other-flow".into();
    other.flow_property_id = session.proto.registry.id_of("other", "flow").unwrap();
    other.participant_range.0 = arenas.participants.len() as u32;
    arenas.arenas.push(other);
    let mut participants = arenas.participants.clone();
    for member in &mut participants {
        member.arena_idx = 1;
    }
    arenas.participants.extend(participants);
    if reverse {
        arenas.arenas.reverse();
        for member in &mut arenas.participants {
            member.arena_idx = 1 - member.arena_idx;
        }
    }
    let mut spec = SpecSessionState::new();
    spec.arena_registry = arenas;
    spec.persistence_deformations = PersistenceDeformationBindings::admit(policy.map(|base| {
        PersistenceDeformationBinding::new(
            scope(session.scenario.root.id, RESOURCE),
            claimant,
            clause_policy(base),
        )
    }))
    .unwrap();
    session.install_spec_state(spec).unwrap();
    session
        .install_growth_entitlement_market(market(session.scenario.root.id, RESOURCE, 100))
        .unwrap();
    let qualification = session
        .growth_entitlement_market()
        .resident_qualification()
        .unwrap();
    assert_eq!(
        qualification.flow_property_id(),
        session
            .proto
            .registry
            .id_of("simthing", "residency-row-capacity")
            .unwrap()
    );
    assert_eq!(qualification.arena_idx(), u32::from(reverse));
    println!(
        "15.8 semantic arena=residency-row-capacity property={:?} physical_index={}",
        qualification.flow_property_id(),
        qualification.arena_idx()
    );
}

fn facts(session: &SimSession, claimant: SimThingId) -> Vec<(u32, u32, u32)> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.resident_clearing_fact)
        .filter(|fact| fact.source_simthing_id_raw == claimant.raw())
        .map(|fact| (fact.generation.get(), fact.granted, fact.unresolved))
        .collect()
}

fn cross_product(
    input: (Scenario, SimThingId),
    policy: Option<f32>,
    reverse: bool,
    record: bool,
) -> Vec<(u32, u32, u32)> {
    let (scenario, claimant) = input;
    let mut session = SimSession::open(scenario).unwrap();
    install(&mut session, claimant, policy, reverse);
    let identity = session.persisted_execution_identity();
    assert!(session.step_once().unwrap().boundary_reached);
    assert_eq!(facts(&session, claimant), [(1, 4, 6)]);
    // Authorship happens after N has finished; no future datum exists at N.
    assert!(session.proto.root.add_property_to_node(
        claimant,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(2)
    ));
    if record {
        let dir = tempfile::tempdir().unwrap();
        session
            .record_to_path(&dir.path().join("ordinary-flow.ldjson"), 1)
            .unwrap();
    } else {
        assert!(session.step_once().unwrap().boundary_reached);
    }
    assert_eq!(session.persisted_execution_identity(), identity);
    assert_eq!(session.coord.day_index(), 2);
    let expected = if policy == Some(0.5) { 5 } else { 8 };
    let result = facts(&session, claimant);
    assert_eq!(result, [(1, 4, 6), (2, 4, expected - 4)]);
    assert!(!session
        .integration_schedule()
        .entries()
        .iter()
        .any(|entry| matches!(
            entry.row_kind(),
            IntegrationScheduleRowKind::ResidencyPlacementCommit
                | IntegrationScheduleRowKind::GrowthEntitlementRefusal
        )));
    let property = session
        .proto
        .registry
        .id_of("simthing", "residency-row-capacity")
        .unwrap();
    let columns = simthing_driver::resolve_node_columns_for_property(
        &session.proto.registry,
        property,
        "residency-row-capacity",
    )
    .unwrap();
    let slot = session.proto.allocator.slot_of(claimant).unwrap().raw() as usize;
    let flow = session.state.read_values()
        [slot * session.state.n_dims as usize + columns.allocated_flow_col.raw()];
    assert!(
        flow.is_finite() && flow > 0.0,
        "actual N+1 ordinary RF cell: {flow}"
    );
    println!("15.8 claimant={} scope={:?} N: authored10 -> G4/U6; N+1: authored2 -> effective{expected} -> G4/U{}; RF={flow}; reverse={reverse}; record={record}", claimant.raw(), scope(session.scenario.root.id, RESOURCE), expected - 4);
    result
}

#[test]
fn ordinary_session_identity_half_and_registry_permutation_cross_real_generations() {
    for policy in [None, Some(1.0), Some(0.5)] {
        let fixture = scenario();
        assert_eq!(
            cross_product(fixture.clone(), policy, false, false),
            cross_product(fixture, policy, true, false)
        );
    }
}

#[test]
fn recording_session_uses_the_same_late_mint_and_authored_policy() {
    cross_product(scenario(), Some(0.5), true, true);
}

#[test]
fn unmatched_market_refuses_symmetrically_in_both_arena_orders() {
    let fixture = scenario();
    for reverse in [false, true] {
        let (scenario, claimant) = fixture.clone();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, reverse);
        let error = session
            .install_growth_entitlement_market(market(
                session.scenario.root.id,
                "unbound::resource",
                100,
            ))
            .unwrap_err();
        assert!(
            matches!(
                error,
                simthing_driver::SessionError::ResidentClearing(
                    ResidentClearingRuntimeError::MarketCannotLower { .. }
                )
            ),
            "{error:?}"
        );
        println!("15.8 reverse={reverse}: {error}");
    }
}

#[test]
fn structural_batches_cannot_overwrite_continuing_flow_products() {
    let (scenario, claimant) = scenario();
    let mut session = SimSession::open(scenario).unwrap();
    install(&mut session, claimant, None, false);
    session.step_once().unwrap();
    session.proto.root.add_property_to_node(
        claimant,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(2),
    );
    // Same continuous flow claimant; the new structural claimant has no demand datum.
    session
        .tx
        .submit_boundary(simthing_feeder::BoundaryRequest::AddChild {
            parent: session.scenario.root.id,
            child: SimThing::new(SimThingKind::Cohort, 2),
        })
        .unwrap();
    session.step_once().unwrap();
    session.step_once().unwrap();
    assert_eq!(facts(&session, claimant), [(1, 4, 6), (2, 4, 4), (3, 4, 2)]);
    println!("15.8 interleaved structural placement: continuing flow 10/8/6, U6/U4/U2 across three permits and topology rebind");
}

#[test]
fn both_postures_exercise_the_same_draw_envelope_at_the_current_boundary() {
    for posture in [
        simthing_core::ClearingExecutionPosture::ResidentRequired,
        simthing_core::ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        let (scenario, claimant) = scenario();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, claimant, None, false);
        session.set_clearing_execution_posture(posture).unwrap();
        session
            .install_growth_entitlement_market(market(session.scenario.root.id, RESOURCE, 9))
            .unwrap();
        let error = session.step_once().unwrap_err();
        assert!(error.to_string().contains("Draw"), "{error:?}");
        assert!(facts(&session, claimant).is_empty());
        println!("15.8 posture={posture:?} authored10/max9: {error}");
    }
}
