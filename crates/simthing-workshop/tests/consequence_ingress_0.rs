//! CONSEQUENCE-INGRESS-0 terminal end-to-end referee.

use simthing_clausething::{parse_raw_document, raw::RawValue};
use simthing_core::evaluate::Evaluator;
use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    DimensionRegistry, DissolveCondition, GenerationStamp, PropertyTransformDelta, SimProperty,
    SimThing, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::PersistenceConsequenceIngressError;
use simthing_feeder::{feeder_channel, BoundaryRequest, FeederWork};
use simthing_gpu::SlotAllocator;
use simthing_mapeditor::{
    submit_clause_persistence_consequence_script_value, ClausePersistenceConsequenceError,
};
use simthing_sim::overlay_lifecycle::OverlayLifecycleAdmissionState;
use simthing_sim::{apply_structural_mutations, SimRuntimeTree};
use simthing_spec::{
    clear_constrained_claims_at_generation, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConstrainedClaim, ConstrainedSupply, OwnerChannelScopeKey, PersistenceConsequenceError,
    PersistenceOverlayBinding, ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId,
    UnresolvedDemandObservation,
};

const AUTHORED_SCENARIO: &str = r#"
    scenario = {
        script_value = {
            id = unresolved_consequence_value
            base = 1
            add = 1
            floor_at = 0
            ceil_at = 64
        }
    }
"#;

fn authored_script_value() -> simthing_clausething::raw::RawProperty {
    let document = parse_raw_document(AUTHORED_SCENARIO.as_bytes()).expect("ClauseScript parse");
    let RawValue::Block(root) = document.root else {
        panic!("authored document root must be a block")
    };
    let RawValue::Block(scenario) = &root.properties[0].value else {
        panic!("authored scenario must be a block")
    };
    scenario.properties[0].clone()
}

fn unresolved_from_filter(
    source: simthing_core::SimThingId,
    granter: simthing_core::SimThingId,
) -> UnresolvedDemandObservation {
    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("consequence-owner"),
        resource_key: ResourceKey::new("ore"),
        scope_id: ScopeId::new("consequence-scope"),
    };
    let demand = RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested: 5,
        priority: 0,
        source_simthing_id_raw: Some(source.raw()),
    };
    let claim = ConstrainedClaim::from_runtime_demand(&demand, 1.0).unwrap();
    let cleared = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope,
            available: 2,
        }],
        &[claim],
        &AuthoredClearingProgram::new(TransformOp::set(1.0)),
        ClearingRemainderAuthority {
            granter,
            generation: GenerationStamp::new(10),
        },
    )
    .expect("ordinary recursive filter clears");
    let grant = &cleared[0].grants[0];
    assert_eq!(
        (grant.requested, grant.granted, grant.unresolved),
        (5, 2, 3)
    );
    UnresolvedDemandObservation::from_grant(grant, GenerationStamp::new(10))
        .expect("positive U becomes an observation")
}

#[test]
fn authored_scenario_u_costband_binding_and_overlay_are_one_live_chain() {
    let mut registry = DimensionRegistry::new();
    let property_spec = SimProperty::simple("consequence", "scar", 0);
    let property = registry.register(property_spec.clone());
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut target = SimThing::new(SimThingKind::Custom("consequence-target".into()), 0);
    target.add_property(property, property_spec.default_value());
    let target_id = target.id;
    let origin_id = root.id;
    root.add_child(target);

    let observation = unresolved_from_filter(target_id, origin_id);
    let binding = PersistenceOverlayBinding {
        origin: origin_id,
        target: target_id,
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(2.0))],
        },
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
    };
    let script_value = authored_script_value();
    let (tx, rx) = feeder_channel();

    let same_generation = submit_clause_persistence_consequence_script_value(
        &script_value,
        2.0,
        &observation,
        GenerationStamp::new(10),
        &binding,
        &tx,
    );
    assert!(matches!(
        same_generation,
        Err(ClausePersistenceConsequenceError::Ingress(
            PersistenceConsequenceIngressError::Consequence(
                PersistenceConsequenceError::SameGenerationConsequence
            )
        ))
    ));
    assert!(rx.drain_now().is_empty());

    let (id, consequence) = submit_clause_persistence_consequence_script_value(
        &script_value,
        2.0,
        &observation,
        GenerationStamp::new(11),
        &binding,
        &tx,
    )
    .expect("authored later consequence enters the existing route");
    assert_eq!(id, "unresolved_consequence_value");
    assert_eq!(consequence.observed_generation, GenerationStamp::new(10));
    assert_eq!(consequence.consequence_generation, GenerationStamp::new(11));
    assert_eq!(consequence.cost_band.v.to_bits(), 4.0f32.to_bits());
    assert_eq!((consequence.cost_band.n, consequence.cost_band.r), (2, 0.0));
    let overlay_id = consequence.overlay.as_ref().expect("funded overlay").id;

    let requests = rx
        .drain_now()
        .into_iter()
        .map(|work| match work {
            FeederWork::Boundary(request) => request,
            _ => panic!("consequence ingress emitted non-boundary work"),
        })
        .collect::<Vec<_>>();
    assert_eq!(requests.len(), 1);
    let BoundaryRequest::AttachOverlay {
        target,
        source_generation,
        ..
    } = &requests[0]
    else {
        panic!("consequence must use the existing routed-overlay boundary")
    };
    assert_eq!(
        (*target, *source_generation),
        (target_id, GenerationStamp::new(11))
    );

    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root).unwrap();
    let mut runtime = SimRuntimeTree::admit(root);
    let mut shadow = vec![0.0; allocator.capacity() * registry.total_columns];
    let mut lifecycle = OverlayLifecycleAdmissionState::default();
    let applied = apply_structural_mutations(
        requests,
        &mut runtime,
        &mut allocator,
        &mut registry,
        &mut shadow,
        property_spec.layout.stride(),
        None,
        GenerationStamp::new(12),
        &mut lifecycle,
        &Default::default(),
    );
    assert_eq!(applied.overlays_attached, vec![(target_id, overlay_id)]);
    assert_eq!(
        lifecycle.routed_provenance(target_id, overlay_id),
        Some(GenerationStamp::new(11))
    );
    assert_eq!(
        lifecycle.activation_generation(target_id, overlay_id),
        Some(GenerationStamp::new(12))
    );
    let received: SimThing =
        serde_json::from_value(serde_json::to_value(&runtime).unwrap()).unwrap();
    let evaluated = Evaluator::new(&registry, 0.0).evaluate(&received, 0);
    let amount = evaluated
        .get(target_id)
        .and_then(|entity| entity.properties.get(&property))
        .expect("received overlay activates on the target")
        .get_role(&SubFieldRole::Amount, &property_spec.layout);
    assert_eq!(amount.to_bits(), 2.0f32.to_bits());
}

fn function_body<'a>(source: &'a str, name: &str) -> &'a str {
    let marker = format!("pub fn {name}");
    let start = source.find(&marker).expect("named public function");
    let open = source[start..].find('{').unwrap() + start;
    let mut depth = 0usize;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[start..=open + offset];
                }
            }
            _ => {}
        }
    }
    panic!("unclosed function")
}

#[test]
fn consequence_only_types_and_symbol_census_reject_every_feedback_route() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let clause = std::fs::read_to_string(
        root.join("crates/simthing-clausething/src/hydrate_shipsize_decoder.rs"),
    )
    .unwrap();
    let driver = std::fs::read_to_string(
        root.join("crates/simthing-driver/src/persistence_consequence_ingress.rs"),
    )
    .unwrap();
    let application = std::fs::read_to_string(
        root.join("crates/simthing-mapeditor/src/clause_persistence_consequence.rs"),
    )
    .unwrap();
    let lowerer = function_body(&clause, "compile_persistence_consequence_script_value");
    let ingress = function_body(&driver, "submit_authored_persistence_consequence");
    let authoring = function_body(
        &application,
        "submit_clause_persistence_consequence_script_value",
    );

    assert_eq!(application.matches("pub fn ").count(), 1);
    assert_eq!(driver.matches("pub fn ").count(), 1);
    assert_eq!(
        lowerer
            .matches("AuthoredPersistenceValuation::new(")
            .count(),
        1
    );
    assert_eq!(
        authoring
            .matches("compile_persistence_consequence_script_value(")
            .count(),
        1
    );
    assert_eq!(
        authoring
            .matches("submit_authored_persistence_consequence(")
            .count(),
        1
    );
    assert_eq!(ingress.matches("fund_unresolved_persistence(").count(), 1);
    assert_eq!(ingress.matches("RoutedOverlayDelivery::admit(").count(), 1);

    let consequence_only = |sources: &str| {
        [
            "produce_runtime_rf_next_generation_demands",
            "RuntimeOwnerSiloDemandBucket",
            "carry_unresolved_demand_to_next_generation",
            "PersistenceDeformationProgram",
            "PersistenceDeformationBinding",
            "ConstrainedClaim",
            "ConstrainedSupply",
        ]
        .iter()
        .all(|forbidden| !sources.contains(forbidden))
    };
    let production_chain = format!("{lowerer}\n{ingress}\n{authoring}");
    assert!(consequence_only(&production_chain));
    for planted in [
        "produce_runtime_rf_next_generation_demands()",
        "let _: RuntimeOwnerSiloDemandBucket = consequence;",
        "let _: PersistenceDeformationProgram = valuation;",
        "carry_unresolved_demand_to_next_generation()",
    ] {
        assert!(
            !consequence_only(&format!("{production_chain}\n{planted}")),
            "planted feedback shape must RED: {planted}"
        );
    }

    for manifest in ["core", "spec", "kernel", "sim", "gpu", "feeder", "driver"] {
        let cargo =
            std::fs::read_to_string(root.join(format!("crates/simthing-{manifest}/Cargo.toml")))
                .unwrap();
        assert!(
            !cargo.contains("simthing-clausething"),
            "engine crate {manifest} acquired a ClauseThing dependency"
        );
    }

    let property_operand = parse_raw_document(
        br#"script_value = {
            id = forbidden_field_cache
            base = 1
            add = { property = state::scar }
        }"#,
    )
    .unwrap();
    let RawValue::Block(root) = property_operand.root else {
        panic!("property-operand document root must be a block")
    };
    let error = simthing_clausething::compile_persistence_consequence_script_value(
        &root.properties[0],
        1.0,
    )
    .expect_err("consequence authoring cannot introduce a property/field read");
    assert!(error
        .to_string()
        .contains("persistence script_value admits literal modifiers only"));
}
