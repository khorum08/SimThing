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

fn session() -> (SimSession, SimThingId) {
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
    let mut session = SimSession::open(Scenario {
        name: "15.10 ordinary post-dispatch abort".into(),
        ticks_per_day: 1,
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
    })
    .unwrap();
    assert!(session.clearing_execution_posture().is_resident_required());
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
    let (mut session, claimant) = session();
    malformed_late_refresh(&mut session, claimant);
    let first = session.step_once().unwrap_err();
    let first_facts = resident_facts(&session);
    let first_tick = session.coord.tick_index();
    let first_day = session.coord.day_index();
    let first_values = session.state.read_values();
    println!("15.10 first attempt: error={first:?}; tick={first_tick}; day={first_day}; resident_facts={first_facts:?}");
    assert!(matches!(
        first,
        SessionError::ResourceEconomy(ResourceEconomySyncError::RecipeExecutionMetadataMismatch {
            recipes: 0,
            coefficients: 1,
            order_bands: 0
        })
    ));
    assert_eq!(
        first_facts,
        [(1, claimant.raw(), 4, 6)],
        "confirmed actual resident economic effect before failure"
    );
    let retry = session.step_once();
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
    assert_eq!(first_values, retry_values);
    let error = retry.expect_err("touched generation is fail-stop");
    assert!(
        error.to_string().contains("fault"),
        "typed generation fault required: {error:?}"
    );
}
