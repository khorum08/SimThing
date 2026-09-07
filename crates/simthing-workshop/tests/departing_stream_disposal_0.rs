//! 15.12 actual authoring/session stream-membership lifecycle referee.
use simthing_clausething::{
    compile_persistence_consequence_script_value, parse_raw_document, raw::RawProperty,
    raw::RawValue,
};
use simthing_core::{
    bind_owner, ClearingExecutionPosture, DimensionRegistry, DissolveCondition, GenerationStamp,
    NeutralStreamTerminationFact, OwnerRef, PropertyTransformDelta, SimProperty, SimPropertyId,
    SimThing, SimThingId, SimThingKind, SpecializationProfile, SubFieldRole, TransformOp,
};
use simthing_driver::resident_clearing_runtime::install_default_resident_rf_property;
use simthing_driver::{
    submit_authored_persistence_consequence, GrowthEntitlementMarketBinding, Scenario, SimSession,
};
use simthing_spec::{
    admit_specialization_flow_market, scenario_metadata_u32_value, AuthoredClearingProgram,
    ConservedOfferingSpec, DrawEnvelopeTemplateSpec, OfferingPriceVectorSpec, OwnerChannelScopeKey,
    PersistenceOverlayBinding, ResourceKey, ScopeId, SpecializationFlowMarketSpec,
    UnresolvedDemandObservation, OWNER_FLOW_DEMAND_PROPERTY_ID, OWNER_SILO_CURRENT_PROPERTY_ID,
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
    install_default_resident_rf_property(&mut registry, &mut root);
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

fn install(session: &mut SimSession, posture: ClearingExecutionPosture) {
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

#[test]
fn authored_departure_binding_reaches_the_existing_consequence_ingress() {
    let mut authoring_failures = Vec::new();
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        let (scenario, sources, scar) = scenario(&[10], 4);
        let root = scenario.root.id;
        let source = sources[0];
        let authored = script(&format!(
            r#"
            script_value = {{
                id = departure_value base = 1 add = 0 floor_at = 0 ceil_at = 64
                departure = {{
                    owner = "owner/15.12" resource = "{RESOURCE}" scope = "{}"
                    claimant = {} origin = {} target = {}
                    property = "departure-proof::scar" add = 2 unit_cost = 2 after_ticks = 3
                }}
            }}
        "#,
            ScopeId::from_boundary(root).as_str(),
            source.raw(),
            root.raw(),
            root.raw()
        ));
        // Actual existing authoring door: the plain formula compiles, whereas
        // its authored departure destination/lifecycle cannot enter admission.
        let compiled = compile_persistence_consequence_script_value(&authored, 2.0);
        let plain = script(
            "script_value = { id = departure_value base = 1 add = 0 floor_at = 0 ceil_at = 64 }",
        );
        let (_, valuation) = compile_persistence_consequence_script_value(&plain, 2.0).unwrap();
        let mut session = SimSession::open(scenario).unwrap();
        install(&mut session, posture);
        assert!(session.step_once().unwrap().boundary_reached);
        remove_demand(&mut session, source);
        assert!(session.step_once().unwrap().boundary_reached);
        let recorded = terminations(&session);
        assert_eq!(recorded.len(), 1);
        let final_product = &recorded[0].final_products[0];
        assert_eq!((final_product.granted, final_product.unresolved), (4, 6));
        let neutral_tree: SimThing =
            serde_json::from_value(serde_json::to_value(&session.proto.root).unwrap()).unwrap();
        assert!(
            neutral_tree.overlays.is_empty(),
            "neutral termination alone has no authored consequence"
        );
        // A control proves the existing 15.3 ingress itself is operational.
        // This explicit application call is not automatic departure consumption.
        let observation = UnresolvedDemandObservation {
            scope: scope(root),
            source_simthing_id: source,
            unresolved: final_product.unresolved,
            observed_generation: final_product.generation,
        };
        let binding = PersistenceOverlayBinding {
            origin: root,
            target: root,
            transform: PropertyTransformDelta {
                property_id: scar,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(2.0))],
            },
            dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
        };
        let consequence = submit_authored_persistence_consequence(
            &observation,
            GenerationStamp::new(2),
            &valuation,
            &binding,
            &session.tx,
        )
        .unwrap();
        assert_eq!(consequence.cost_band.n, 3);
        let overlay_id = consequence.overlay.unwrap().id;
        let queued = session.rx.drain_now();
        assert_eq!(queued.len(), 1);
        assert!(matches!(&queued[0], simthing_feeder::FeederWork::Boundary(
            simthing_feeder::BoundaryRequest::AttachOverlay { target, overlay, source_generation }
        ) if *target == root && overlay.id == overlay_id && *source_generation == GenerationStamp::new(2)));
        println!("15.12 RED-A {posture:?}: real neutral fact G4/U6; no automatic Overlay; explicit existing consequence ingress funds n=3 and queues the routed Overlay; authored binding={compiled:?}");
        if let Err(error) = compiled {
            authoring_failures.push(format!("{posture:?}: {error}"));
        }
    }
    assert!(
        authoring_failures.is_empty(),
        "authored departure binding cannot reach ordinary admission: {authoring_failures:#?}"
    );
}

#[test]
fn established_partial_departure_terminates_before_survivor_carry() {
    let mut refused = Vec::new();
    for posture in [
        ClearingExecutionPosture::ResidentRequired,
        ClearingExecutionPosture::CpuVendorizedOracle,
    ] {
        for supply in [4, 40] {
            let (scenario, sources, _) = scenario(&[10, 10], supply);
            let mut session = SimSession::open(scenario).unwrap();
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
