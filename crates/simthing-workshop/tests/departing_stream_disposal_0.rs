//! 15.12 actual authoring/session stream-membership lifecycle referee.
use simthing_clausething::{parse_raw_document, raw::RawProperty, raw::RawValue};
use simthing_core::{
    bind_owner, ClearingExecutionPosture, DimensionRegistry, GenerationStamp,
    NeutralStreamTerminationFact, OwnerRef, SimProperty, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SpecializationProfile, TransformOp,
};
use simthing_driver::resident_clearing_runtime::install_default_resident_rf_property;
use simthing_driver::{GrowthEntitlementMarketBinding, Scenario, SimSession};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    ResourceKey, ScopeId, SpecializationFlowMarketSpec, OWNER_FLOW_DEMAND_PROPERTY_ID,
    OWNER_SILO_CURRENT_PROPERTY_ID,
};
use std::collections::BTreeSet;

const RESOURCE: &str = "simthing::residency-row-capacity";

fn scope(root: SimThingId) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/15.12"),
        resource_key: ResourceKey::new(RESOURCE),
        scope_id: ScopeId::from_boundary(root),
    }
}

fn scenario(requests: &[u32], supply: u32) -> (Scenario, Vec<SimThingId>, SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let scar = SimProperty::simple("departure-proof", "scar", 0);
    let scar_id = registry.register(scar.clone());
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    bind_owner(&mut root, &OwnerRef::new("owner/15.12"));
    root.add_property(scar_id, scar.default_value());
    root.add_property(
        OWNER_SILO_CURRENT_PROPERTY_ID,
        scenario_metadata_u32_value(supply),
    );
    let mut ids = Vec::new();
    for &requested in requests {
        let mut claimant = SimThing::new(SimThingKind::Cohort, 0);
        claimant.add_property(
            OWNER_FLOW_DEMAND_PROPERTY_ID,
            scenario_metadata_u32_value(requested),
        );
        ids.push(claimant.id);
        root.add_child(claimant);
    }
    let property = install_default_resident_rf_property(&mut registry, &mut root);
    let mut other = registry.property(property).clone();
    other.namespace = "other".into();
    other.name = "flow".into();
    for field in &mut other.layout.sub_fields {
        if let Some(spec) = &mut field.accumulator_spec {
            match &mut spec.role {
                simthing_core::AccumulatorRole::AllocatedFlow { arena }
                | simthing_core::AccumulatorRole::AllocatorWeight { arena } => {
                    *arena = "other-flow".into()
                }
                _ => {}
            }
        }
    }
    let other_id = registry.register(other);
    let other_value = registry.property(other_id).default_value();
    root.add_property(other_id, other_value.clone());
    for child in &mut root.children {
        child.add_property(other_id, other_value.clone());
    }

    (
        Scenario {
            name: "15.12 ordinary departure".into(),
            ticks_per_day: 1,
            max_days: 12,
            dt: 1.0,
            n_slots: 16,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: Default::default(),
        },
        ids,
        scar_id,
    )
}

fn open_bound(
    scenario: Scenario,
    posture: ClearingExecutionPosture,
    bindings: simthing_spec::PersistenceDeformationBindings,
) -> Result<SimSession, simthing_driver::SessionError> {
    let mut session = SimSession::open_with_clearing_posture(scenario, posture)?;
    let mut spec = simthing_driver::SpecSessionState::new();
    spec.arena_registry = session.spec_state.arena_registry.clone();
    spec.persistence_deformations = bindings;
    session.install_spec_state(spec)?;
    Ok(session)
}

fn install(session: &mut SimSession, posture: ClearingExecutionPosture) {
    install_order(session, posture, false);
}

fn install_order(session: &mut SimSession, posture: ClearingExecutionPosture, reverse: bool) {
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
    let mut spec = simthing_driver::SpecSessionState::new();
    spec.arena_registry = arenas;
    spec.persistence_deformations = session.spec_state.persistence_deformations.clone();
    session.install_spec_state(spec).unwrap();

    let triggers = BTreeSet::from(["current-boundary".to_owned()]);
    let admitted = admit_specialization_flow_market(
        &[SpecializationProfile {
            id: "ordinary-flow".into(),
            description: "15.12 stream".into(),
            requirements: Vec::new(),
        }],
        &triggers,
        SpecializationFlowMarketSpec {
            specialization_profile_id: "ordinary-flow".into(),
            offerings: vec![ConservedOfferingSpec {
                id: "offering".into(),
                resource_key: ResourceKey::new(RESOURCE),
                price: OfferingPriceVectorSpec {
                    unit_cost: 1.0,
                    default_clearing_weight: 1.0,
                },
            }],
            draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                id: "draw".into(),
                offering_refs: vec!["offering".into()],
                lifecycle_trigger_refs: vec!["current-boundary".into()],
                min_quantity: 0,
                max_quantity: 100,
            }],
        },
    )
    .unwrap();
    session
        .install_growth_entitlement_market(GrowthEntitlementMarketBinding::from_admitted_market(
            admitted,
            session.scenario.root.id,
            "offering",
            "draw",
            scope(session.scenario.root.id),
            triggers,
            AuthoredClearingProgram::new(TransformOp::set(1.0)),
            1.0,
            100,
        ))
        .unwrap();
    session.set_clearing_execution_posture(posture).unwrap();
}

fn remove_demand(session: &mut SimSession, claimant: SimThingId) {
    let mut tree: SimThing =
        serde_json::from_value(serde_json::to_value(&session.proto.root).unwrap()).unwrap();
    let node = tree
        .children
        .iter_mut()
        .find(|node| node.id == claimant)
        .unwrap();
    assert!(node
        .remove_property(&OWNER_FLOW_DEMAND_PROPERTY_ID)
        .is_some());
    session
        .proto
        .root
        .swap(simthing_sim::SimRuntimeTree::admit(tree));
}

fn terminations(session: &SimSession) -> Vec<NeutralStreamTerminationFact> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.neutral_stream_termination_fact.clone())
        .collect()
}

fn script(source: &str) -> RawProperty {
    let parsed = parse_raw_document(source.as_bytes()).expect("ordinary ClauseScript parse");
    let RawValue::Block(root) = parsed.root else {
        panic!("document block")
    };
    root.properties[0].clone()
}

fn assert_authored_departure(posture: ClearingExecutionPosture) {
    let (scenario, sources, _) = scenario(&[10], 4);
    let root = scenario.root.id;
    let source = sources[0];
    let authored = script(&format!(
        r#"
        script_value = {{ id = departure_value base = 1 add = 0 floor_at = 0 ceil_at = 64
            departure = {{ owner = "owner/15.12" resource = "{RESOURCE}" scope = "{}"
                claimant = {} origin = {} target = {} property = "departure-proof::scar"
                add = 2 unit_cost = 2 after_ticks = 3 }} }}
    "#,
        ScopeId::from_boundary(root).as_str(),
        source.raw(),
        root.raw(),
        root.raw()
    ));
    let (_, binding) = simthing_clausething::compile_departure_disposition_script_value(
        &authored,
        &scenario.registry,
    )
    .unwrap();
    let admitted = simthing_spec::PersistenceDeformationBindings::default()
        .with_departure_dispositions([binding])
        .unwrap();
    let mut session = open_bound(scenario, posture, admitted).unwrap();
    install(&mut session, posture);
    assert!(
        snapshot(&session).overlays.is_empty(),
        "lifecycle admission is not an Overlay or cost"
    );
    session.step_once().unwrap();
    remove_demand(&mut session, source);
    session.step_once().unwrap();
    let recorded = terminations(&session);
    assert_eq!(recorded.len(), 1);
    let final_product = &recorded[0].final_products[0];
    assert_eq!((final_product.granted, final_product.unresolved), (4, 6));
    let consequences: Vec<_> = session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.departure_consequence_fact.as_ref())
        .cloned()
        .collect();
    assert_eq!(consequences.len(), 1);
    let consequence = &consequences[0];
    assert_eq!(consequence.termination, recorded[0]);
    assert_eq!(consequence.units, 3);
    assert_eq!(consequence.value_bits, 6.0f32.to_bits());
    assert_eq!(consequence.unit_cost_bits, 2.0f32.to_bits());
    assert_eq!(consequence.remainder_bits, 0.0f32.to_bits());
    assert_eq!(consequence.consequence_generation, GenerationStamp::new(2));
    let overlay_id = consequence.overlay_id.unwrap();
    session.step_once().unwrap();
    assert!(snapshot(&session)
        .overlays
        .iter()
        .any(|overlay| overlay.id == overlay_id));
    let history = session.integration_schedule().entries().to_vec();
    session.step_once().unwrap();
    assert_eq!(
        session.integration_schedule().entries(),
        history.as_slice(),
        "no duplicate termination/consequence or departed demand"
    );
    println!("15.12 {posture:?}: recorded G4/U6 -> authored existing ingress -> CostBand n3 -> actual N3 Overlay; N4 no duplicate effects");
}

fn snapshot(session: &SimSession) -> SimThing {
    serde_json::from_value(serde_json::to_value(&session.proto.root).unwrap()).unwrap()
}

#[test]
fn authored_departure_binding_reaches_the_existing_consequence_ingress() {
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        assert_authored_departure(posture);
    }
}

#[test]
fn established_partial_departure_terminates_before_survivor_carry() {
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        assert_partial_departure(posture);
    }
}

fn assert_partial_departure(posture: ClearingExecutionPosture) {
    let mut refused = Vec::new();
    {
        for supply in [4, 40] {
            let (scenario, sources, _) = scenario(&[10, 10], supply);
            let mut session = SimSession::open_with_clearing_posture(scenario, posture).unwrap();
            install(&mut session, posture);
            assert!(session.step_once().unwrap().boundary_reached);
            assert!(terminations(&session).is_empty());
            let before = session.integration_schedule().entries().to_vec();
            remove_demand(&mut session, sources[0]);
            match session.step_once() {
                Err(error) => {
                    let text = error.to_string();
                    assert!(text.contains(&simthing_driver::resident_clearing_runtime::ResidentClearingRuntimeError::TemporalSourceMismatch.to_string()), "unexpected refusal: {error:?}");
                    assert_eq!(session.integration_schedule().entries(), before.as_slice());
                    assert!(terminations(&session).is_empty());
                    println!("15.12 RED-B {posture:?} supply={supply}: actual established two-claimant stream loses one property; {error}; no departing fact or survivor carry");
                    refused.push(format!("{posture:?} supply={supply}: {text}"));
                }
                Ok(step) => {
                    assert!(step.boundary_reached);
                    let facts = terminations(&session);
                    assert_eq!(facts.len(), 1);
                    assert_eq!(facts[0].termination_generation, GenerationStamp::new(2));
                    assert_eq!(facts[0].final_products.len(), 1);
                    let departed = &facts[0].final_products[0];
                    assert_eq!(departed.source_simthing_id, sources[0]);
                    assert_eq!(departed.generation, GenerationStamp::new(1));
                    assert_eq!(
                        (departed.granted, departed.unresolved),
                        if supply == 4 { (2, 8) } else { (10, 0) }
                    );
                    remove_demand(&mut session, sources[1]);
                    assert!(session.step_once().unwrap().boundary_reached);
                    let facts = terminations(&session);
                    assert_eq!(facts.len(), 2);
                    let survivor = &facts[1].final_products[0];
                    assert_eq!(survivor.source_simthing_id, sources[1]);
                    assert_eq!(survivor.generation, GenerationStamp::new(2));
                    assert_eq!(
                        (survivor.granted, survivor.unresolved),
                        if supply == 4 { (4, 14) } else { (10, 0) }
                    );
                }
            }
        }
    }
    assert!(refused.is_empty(), "lawful fact-authorized partial departure refuses instead of carrying the survivor: {refused:#?}");
}

#[derive(Clone, Copy, Debug)]
enum Loop {
    Step,
    Run,
    Record,
}
fn advance_result(
    session: &mut SimSession,
    path: Loop,
) -> Result<(), simthing_driver::SessionError> {
    match path {
        Loop::Step => session
            .step_once()
            .map(|step| assert!(step.boundary_reached)),
        Loop::Run => session
            .run(1)
            .map(|summary| assert_eq!(summary.boundaries_run, 1)),
        Loop::Record => {
            let directory = tempfile::tempdir().unwrap();
            session
                .record_to_path(&directory.path().join("departure.ldjson"), 1)
                .map(|summary| assert_eq!(summary.boundaries_run, 1))
        }
    }
}
fn advance(session: &mut SimSession, path: Loop) {
    advance_result(session, path).unwrap();
}
fn set_demand(session: &mut SimSession, source: SimThingId, quantity: u32) {
    assert!(session.proto.root.add_property_to_node(
        source,
        OWNER_FLOW_DEMAND_PROPERTY_ID,
        scenario_metadata_u32_value(quantity)
    ));
}
fn disposition(
    scenario: &Scenario,
    source: SimThingId,
    delta: u32,
) -> simthing_spec::DepartureDispositionBinding {
    let root = scenario.root.id;
    let authored = script(&format!(
        r#"script_value = {{ id = departure_value base = 1
        departure = {{ owner = "owner/15.12" resource = "{RESOURCE}" scope = "{}"
            claimant = {} origin = {} target = {} property = "departure-proof::scar"
            add = {delta} unit_cost = 2 after_ticks = 3 }} }}"#,
        ScopeId::from_boundary(root).as_str(),
        source.raw(),
        root.raw(),
        root.raw()
    ));
    simthing_clausething::compile_departure_disposition_script_value(&authored, &scenario.registry)
        .unwrap()
        .1
}
fn consequence_facts(session: &SimSession) -> Vec<simthing_core::DepartureConsequenceFact> {
    session
        .integration_schedule()
        .entries()
        .iter()
        .filter_map(|entry| entry.departure_consequence_fact.clone())
        .collect()
}

fn membership_matrix(posture: ClearingExecutionPosture) {
    for path in [Loop::Step, Loop::Run, Loop::Record] {
        for reverse in [false, true] {
            for (supply, departing, surviving, factor) in [
                (4, 10, 10, 1.0),
                (4, 10, 10, 0.5),
                (40, 10, 10, 1.0),
                (4, 0, 10, 1.0),
                (4, 10, 0, 1.0),
                (4, 0, 0, 1.0),
            ] {
                let (mut scenario, sources, _) = scenario(&[departing, surviving, 5], supply);
                scenario.root.children[2]
                    .remove_property(&OWNER_FLOW_DEMAND_PROPERTY_ID)
                    .unwrap();
                if reverse {
                    scenario.root.children.reverse();
                }
                let program = simthing_clausething::compile_persistence_deformation_script_value(
                    &script(&format!(
                        "script_value = {{ id = survivor_policy base = {factor} }}"
                    )),
                    100,
                )
                .unwrap()
                .1;
                let bindings = simthing_spec::PersistenceDeformationBindings::admit([
                    simthing_spec::PersistenceDeformationBinding::new(
                        scope(scenario.root.id),
                        sources[1],
                        program,
                    ),
                ])
                .unwrap();
                let mut session = open_bound(scenario, posture, bindings).unwrap();
                install_order(&mut session, posture, reverse);
                advance(&mut session, path);
                remove_demand(&mut session, sources[0]);
                set_demand(&mut session, sources[2], 5);
                advance(&mut session, path);
                let facts = terminations(&session);
                assert_eq!(facts.len(), 1);
                assert_eq!(facts[0].termination_generation, GenerationStamp::new(2));
                assert_eq!(facts[0].final_products.len(), 1);
                assert_eq!(facts[0].final_products[0].source_simthing_id, sources[0]);
                let depart_g = if departing + surviving <= supply {
                    departing
                } else if surviving == 0 {
                    supply
                } else if departing == 0 {
                    0
                } else {
                    supply / 2
                };
                assert_eq!(
                    (
                        facts[0].final_products[0].granted,
                        facts[0].final_products[0].unresolved
                    ),
                    (depart_g, departing - depart_g)
                );
                let initial_survivor_g = if departing + surviving <= supply {
                    surviving
                } else if departing == 0 {
                    supply
                } else if surviving == 0 {
                    0
                } else {
                    supply / 2
                };
                let expected_demand =
                    surviving + ((surviving - initial_survivor_g) as f32 * factor).floor() as u32;
                remove_demand(&mut session, sources[1]);
                remove_demand(&mut session, sources[2]);
                advance(&mut session, path);
                let facts = terminations(&session);
                assert_eq!(
                    facts.len(),
                    2,
                    "unbound all-depart preserves the single aggregate neutral row"
                );
                let terminal = &facts[1].final_products;
                assert_eq!(terminal.len(), 2);
                let survivor = terminal
                    .iter()
                    .find(|product| product.source_simthing_id == sources[1])
                    .unwrap();
                let entrant = terminal
                    .iter()
                    .find(|product| product.source_simthing_id == sources[2])
                    .unwrap();
                assert_eq!(survivor.generation, GenerationStamp::new(2));
                assert_eq!(
                    survivor.granted + survivor.unresolved,
                    expected_demand,
                    "survivor consumes only own U through authored f exactly once"
                );
                assert_eq!(
                    entrant.granted + entrant.unresolved,
                    5,
                    "entrant is fresh and never mints from prior U"
                );
                let survivor_g = if expected_demand + 5 <= supply {
                    expected_demand
                } else if expected_demand == 0 {
                    0
                } else {
                    3
                };
                assert_eq!(survivor.granted, survivor_g);
                assert_eq!(entrant.granted, (supply - survivor_g).min(5));
                assert!(consequence_facts(&session).is_empty());
                let history = session.integration_schedule().clone();
                advance(&mut session, path);
                advance(&mut session, path);
                assert_eq!(session.integration_schedule(), &history);
                set_demand(&mut session, sources[0], 3);
                advance(&mut session, path);
                remove_demand(&mut session, sources[0]);
                advance(&mut session, path);
                let facts = terminations(&session);
                let reentry = &facts.last().unwrap().final_products[0];
                assert_eq!(
                    (
                        reentry.source_simthing_id,
                        reentry.granted,
                        reentry.unresolved,
                        reentry.generation
                    ),
                    (sources[0], 3, 0, GenerationStamp::new(6))
                );
                let serialized = serde_json::to_vec(session.integration_schedule()).unwrap();
                let replay: simthing_core::IntegrationSchedule =
                    serde_json::from_slice(&serialized).unwrap();
                assert_eq!(serde_json::to_vec(&replay).unwrap(), serialized);
                println!("15.12 mixed {posture:?}/{path:?}/reverse={reverse}: supply={supply} departed={departing} survivor={surviving} f={factor}; survivor d={expected_demand}; entrant5; late history unchanged; same-ID reentry fresh3");
            }
        }
    }
}

#[test]
fn mixed_membership_order_zero_and_reentry_matrix() {
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        membership_matrix(posture);
    }
}

fn authored_all_depart_matrix(posture: ClearingExecutionPosture) {
    for path in [Loop::Step, Loop::Run, Loop::Record] {
        for (request, supply) in [(10, 4), (10, 40), (0, 4)] {
            for count in 0..=2 {
                let (scenario, sources, _) = scenario(&[request, request], supply);
                let bindings = simthing_spec::PersistenceDeformationBindings::default()
                    .with_departure_dispositions(
                        (0..count)
                            .map(|index| disposition(&scenario, sources[index], index as u32 + 2)),
                    )
                    .unwrap();
                let mut session = open_bound(scenario, posture, bindings).unwrap();
                install(&mut session, posture);
                assert!(snapshot(&session).overlays.is_empty());
                advance(&mut session, path);
                remove_demand(&mut session, sources[0]);
                remove_demand(&mut session, sources[1]);
                advance(&mut session, path);
                let facts = terminations(&session);
                assert_eq!(facts.len(), if count == 0 { 1 } else { 2 });
                let products: Vec<_> = facts.iter().flat_map(|fact| &fact.final_products).collect();
                assert_eq!(products.len(), 2);
                let granted = request.min(supply / 2);
                for product in products {
                    assert_eq!(
                        (product.granted, product.unresolved, product.generation),
                        (granted, request - granted, GenerationStamp::new(1))
                    );
                }
                let consequences = consequence_facts(&session);
                assert_eq!(consequences.len(), count);
                assert_consequence_history_only(&session);
                for consequence in &consequences {
                    assert_eq!(consequence.units, (request - granted) / 2);
                    assert_eq!(consequence.termination.final_products.len(), 1);
                    assert!(sources[..count]
                        .contains(&consequence.termination.final_products[0].source_simthing_id));
                    let entries = session.integration_schedule().entries();
                    let neutral_index = entries
                        .iter()
                        .position(|entry| {
                            entry.product_key == consequence.termination_product_key
                                && entry.neutral_stream_termination_fact.is_some()
                        })
                        .unwrap();
                    let consequence_index = entries
                        .iter()
                        .position(|entry| {
                            entry.departure_consequence_fact.as_ref() == Some(consequence)
                        })
                        .unwrap();
                    assert!(
                        neutral_index < consequence_index,
                        "immutable termination precedes existing ingress proof"
                    );
                }
                advance(&mut session, path);
                let tree = snapshot(&session);
                assert_eq!(
                    tree.overlays.len(),
                    if request > granted { count } else { 0 }
                );
                for consequence in &consequences {
                    if let Some(overlay_id) = consequence.overlay_id {
                        assert!(tree.overlays.iter().any(|overlay| overlay.id == overlay_id));
                    }
                }
                let history = session.integration_schedule().clone();
                advance(&mut session, path);
                advance(&mut session, path);
                advance(&mut session, path);
                assert_eq!(session.integration_schedule(), &history);
                // The ordinary facility publishes the deadline result at the
                // following boundary; the N6 condition is observed at N7.
                advance(&mut session, path);
                assert!(
                    snapshot(&session).overlays.is_empty(),
                    "authored AfterTicks lifecycle dissolves through the existing facility"
                );
                assert_eq!(session.integration_schedule(), &history);
                println!("15.12 all-depart {posture:?}/{path:?}: {count} per-claimant authored consequences; unbound claimants neutral; actual Overlay attachment/lifecycle; no later duplicates");
            }
        }
    }
}

#[test]
fn per_claimant_authored_all_depart_and_neutral_default_matrix() {
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        authored_all_depart_matrix(posture);
    }
}

fn fault_matrix(posture: ClearingExecutionPosture) {
    use simthing_driver::resource_economy_compile::{
        ResourceEconomyMaterializationReport, ResourceEconomyRegistrations, ResourceEconomyRegistry,
    };
    for path in [Loop::Step, Loop::Run, Loop::Record] {
        let (scenario, sources, _) = scenario(&[10, 10], 4);
        let binding = disposition(&scenario, sources[0], 2);
        let bindings = simthing_spec::PersistenceDeformationBindings::default()
            .with_departure_dispositions([binding])
            .unwrap();
        let mut session = open_bound(scenario, posture, bindings).unwrap();
        install(&mut session, posture);
        advance(&mut session, path);
        remove_demand(&mut session, sources[0]);
        session.spec_state.resource_economy_registry = Some(ResourceEconomyRegistry {
            generation: 1,
            registrations: ResourceEconomyRegistrations {
                transfers: vec![simthing_core::DiscreteTransferRegistration {
                    source_slot: session
                        .proto
                        .allocator
                        .slot_of(session.scenario.root.id)
                        .unwrap(),
                    source_col: simthing_core::ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    target_slot: session.proto.allocator.slot_of(sources[1]).unwrap(),
                    target_col: simthing_core::ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                    amount: 1.0,
                    order_band: 0,
                }],
                recipes: vec![],
                emissions: vec![],
                emit_on_threshold: vec![],
                report: ResourceEconomyMaterializationReport {
                    recipe_output_coefficients: vec![1.0],
                    ..Default::default()
                },
            },
        });
        let error = advance_result(&mut session, path).unwrap_err();
        assert!(error.to_string().contains("coefficient"), "{error:?}");
        assert_eq!(terminations(&session).len(), 1);
        assert_eq!(consequence_facts(&session).len(), 1);
        let history = session.integration_schedule().clone();
        let cells = session.state.read_values();
        let tick = session.coord.tick_index();
        let day = session.coord.day_index();
        session.spec_state.resource_economy_registry = None;
        remove_demand(&mut session, sources[1]);
        let retry = advance_result(&mut session, path).unwrap_err();
        let expected = simthing_core::TreeExecutionContextError::GenerationFaulted {
            generation: GenerationStamp::new(2),
        }
        .to_string();
        assert!(
            matches!(retry, simthing_driver::SessionError::ExecutionIdentity(ref message) if message == &expected)
        );
        assert_eq!(session.integration_schedule(), &history);
        assert_eq!(session.state.read_values(), cells);
        assert_eq!(session.coord.tick_index(), tick);
        assert_eq!(session.coord.day_index(), day);
        assert!(
            snapshot(&session).overlays.is_empty(),
            "queued consequence cannot execute another hot/boundary cycle after fault"
        );
        println!("15.12 {posture:?}/{path:?}: actual late refresh failure after departing fact/consequence and survivor settlement; retry GenerationFaulted before all duplicate effects");
    }
}

#[test]
fn post_disposition_failure_faults_before_duplicate_effects() {
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        fault_matrix(posture);
    }
}

#[test]
fn no_fact_no_subset_and_untouched_oracle_retry() {
    use simthing_spec::{
        produce_runtime_rf_survivor_demands, ClearingRemainderAuthority, ConstrainedClaim,
        ConstrainedSupply, RuntimeOwnerSiloDemandBucket, RuntimeRfDemandGenerationAuthority,
    };
    let root = SimThingId::from_session_raw(700);
    let sources = [
        SimThingId::from_session_raw(701),
        SimThingId::from_session_raw(702),
    ];
    let scope = scope(root);
    let bucket = |source: SimThingId| RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested: 10,
        priority: 100,
        source_simthing_id_raw: Some(source.raw()),
    };
    let claims: Vec<_> = sources
        .iter()
        .map(|source| ConstrainedClaim::from_runtime_demand(&bucket(*source), 1.0).unwrap())
        .collect();
    let supply = [ConstrainedSupply {
        scope: scope.clone(),
        available: 4,
    }];
    let authority = RuntimeRfDemandGenerationAuthority::new(ClearingRemainderAuthority {
        granter: root,
        generation: GenerationStamp::new(1),
    });
    let program = AuthoredClearingProgram::new(TransformOp::set(1.0));
    let mut schedule = simthing_core::IntegrationSchedule::new();
    let permit = |schedule: &simthing_core::IntegrationSchedule| {
        simthing_core::SurvivorSubsetPermission::from_recorded_terminations(
            schedule,
            &sources,
            &sources[1..],
            root,
            &scope.owner_ref,
            scope.resource_key.as_str(),
            scope.scope_id.as_str(),
            GenerationStamp::new(1),
            GenerationStamp::new(2),
        )
    };
    assert!(permit(&schedule).is_none());
    let error = produce_runtime_rf_survivor_demands(
        &authority,
        &supply,
        &claims,
        &program,
        None,
        vec![bucket(sources[1])],
    )
    .unwrap_err();
    assert_eq!(error.message, "TemporalSourceMismatch");
    let fact = simthing_core::NeutralStreamTerminationFact {
        granter: root,
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.as_str().to_owned(),
        scope_id: scope.scope_id.as_str().to_owned(),
        termination_generation: GenerationStamp::new(2),
        final_products: vec![simthing_core::NeutralStreamFinalProduct {
            source_simthing_id: sources[0],
            granted: 2,
            unresolved: 8,
            generation: GenerationStamp::new(1),
        }],
    };
    for mutation in ["stale", "scope", "source", "duplicate"] {
        let mut wrong = fact.clone();
        match mutation {
            "stale" => wrong.termination_generation = GenerationStamp::new(1),
            "scope" => wrong.scope_id = "another-scope".into(),
            "source" => wrong.final_products[0].source_simthing_id = sources[1],
            _ => {}
        }
        let mut wrong_schedule = simthing_core::IntegrationSchedule::new();
        wrong_schedule.record_neutral_stream_termination(wrong.clone());
        if mutation == "duplicate" {
            wrong_schedule.record_neutral_stream_termination(wrong);
        }
        assert!(
            permit(&wrong_schedule).is_none(),
            "{mutation} is not exact termination permission"
        );
    }
    schedule.record_neutral_stream_termination(fact);
    let permission = permit(&schedule).unwrap();
    let demands = produce_runtime_rf_survivor_demands(
        &authority,
        &supply,
        &claims,
        &program,
        Some(&permission),
        vec![bucket(sources[1])],
    )
    .unwrap();
    assert_eq!(demands.len(), 1);
    assert_eq!(demands[0].product().requested, 18);
    assert_eq!(demands[0].generation(), GenerationStamp::new(2));
    let repeated = produce_runtime_rf_survivor_demands(
        &authority,
        &supply,
        &claims,
        &program,
        Some(&permission),
        vec![bucket(sources[1])],
    )
    .unwrap_err();
    assert_eq!(
        repeated.kind,
        simthing_spec::RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced
    );
}

fn assert_consequence_history_only(session: &SimSession) {
    use simthing_core::{
        AncestorStandingPolicyView, AuthoredSeamStaleness, GenerationStamped, IntegrationSchedule,
        StandingViewDoubleBuffer,
    };
    use simthing_spec::{
        integrate_stamped_reduce_up, reduce_owner_channel_rf, replay_async_owner_channel_rf_seam,
        OwnerChannelRfOwnAggregate, ParentRfIntegrationState,
    };
    let product = reduce_owner_channel_rf(
        &session.scenario.root,
        &[OwnerChannelRfOwnAggregate {
            simthing_id: session.scenario.root.children[0].id,
            resource_key: ResourceKey::new(RESOURCE),
            surplus: 3,
            deficit: 7,
        }],
        GenerationStamp::new(1),
    )
    .unwrap();
    let mut baseline = IntegrationSchedule::new();
    let mut state = ParentRfIntegrationState::default();
    integrate_stamped_reduce_up(GenerationStamp::new(4), &product, &mut state, &mut baseline)
        .unwrap();
    let standing = GenerationStamped::stamp(
        GenerationStamp::new(4),
        AncestorStandingPolicyView::new(OwnerRef::new("owner/15.12"), vec![]),
    );
    let mut buffer = StandingViewDoubleBuffer::new();
    buffer.stage(standing.clone());
    buffer
        .publish_at_generation_barrier(
            GenerationStamp::new(4),
            AuthoredSeamStaleness::new(0),
            &mut baseline,
        )
        .unwrap();
    let mut with_consequence = baseline.clone();
    with_consequence.entries.extend(
        session
            .integration_schedule()
            .entries()
            .iter()
            .filter(|entry| {
                entry.neutral_stream_termination_fact.is_some()
                    || entry.departure_consequence_fact.is_some()
            })
            .cloned(),
    );
    let before =
        replay_async_owner_channel_rf_seam(&baseline, &[product.clone()], &[standing.clone()])
            .unwrap();
    let after =
        replay_async_owner_channel_rf_seam(&with_consequence, &[product], &[standing]).unwrap();
    assert_eq!(before.parent_state, state);
    assert_eq!(after, before);
    let bytes = serde_json::to_vec(&with_consequence).unwrap();
    let decoded: IntegrationSchedule = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
}

#[test]
fn authored_binding_rejects_feedback_metadata_and_duplicate_admission() {
    let (scenario, sources, _) = scenario(&[10], 4);
    let binding = disposition(&scenario, sources[0], 2);
    assert!(simthing_spec::PersistenceDeformationBindings::default()
        .with_departure_dispositions([binding.clone(), binding])
        .is_err());
    for forbidden in ["demand", "reinject", "deformation", "current_to_next"] {
        let authored = script(&format!(
            "script_value = {{ id = consequence base = 1 departure = {{ {forbidden} = 1 }} }}"
        ));
        let refusal = simthing_clausething::compile_departure_disposition_script_value(
            &authored,
            &scenario.registry,
        )
        .unwrap_err();
        assert!(refusal
            .to_string()
            .contains("unknown or duplicate departure metadata"));
    }
}
