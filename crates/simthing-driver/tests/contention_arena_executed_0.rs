//! CONTENTION-ARENA-EXECUTED-0 focused execution proof.
//!
//! The rival allocators below are test-side mutants. Production has one
//! scenario-neutral claim -> clear -> disburse path.

use std::collections::BTreeMap;

use simthing_core::eml_nodes::{opcode, EmlNode};
use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    deliver_routed_overlay, DissolveCondition, EmlPerProgramCap, GenerationStamp,
    PropertyTransformDelta, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_spec::{
    clear_constrained_claims_at_generation, clear_reduced_owner_channels,
    fund_unresolved_persistence, is_authored_until_dissolved, judge_conservation,
    AuthoredClaimClearingData, AuthoredClearingProgram, AuthoredPersistenceValuation, ChannelBound,
    ClearingRemainderAuthority, ConservationJudgeReason, ConservationSnapshot, ConservationVerdict,
    ConstrainedClaim, ConstrainedClearingResult, ConstrainedSupply, OwnerChannelRfOwnAggregate,
    OwnerChannelScopeKey, PersistenceConsequenceError, PersistenceOverlayBinding, ResourceKey,
    RuntimeOwnerSiloDemandBucket, ScopeId, UnresolvedDemandObservation,
};

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("contention-fixture".into()), 0)
}

fn own(simthing_id: SimThingId, surplus: u32, deficit: u32) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new("ore"),
        surplus,
        deficit,
    }
}

fn eml_node(opcode: u32, a: u32) -> EmlNode {
    EmlNode {
        opcode,
        flags: 0,
        a,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn priority_program() -> AuthoredClearingProgram {
    let nodes = vec![
        eml_node(opcode::LITERAL_F32, 100.0f32.to_bits()),
        eml_node(opcode::PARAM, 1),
        eml_node(opcode::SUB, 0),
    ];
    AuthoredClearingProgram::new(
        TransformOp::admit_eml(nodes, EmlPerProgramCap::DEFAULT).expect("priority EML admits"),
    )
}

fn price_program() -> AuthoredClearingProgram {
    AuthoredClearingProgram::new(
        TransformOp::admit_eml(vec![eml_node(opcode::PARAM, 0)], EmlPerProgramCap::DEFAULT)
            .expect("price EML admits"),
    )
}

fn demand(
    scope: &OwnerChannelScopeKey,
    source: SimThingId,
    requested: u32,
    priority: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested,
        priority,
        source_simthing_id_raw: Some(source.raw()),
    }
}

fn grants(result: &ConstrainedClearingResult) -> BTreeMap<SimThingId, u32> {
    result
        .grants
        .iter()
        .map(|grant| (grant.source_simthing_id, grant.granted))
        .collect()
}

/// Test-side forbidden policy: physical claim order gets the supply.
fn row_order_mutant(
    demands: &[RuntimeOwnerSiloDemandBucket],
    available: u32,
) -> BTreeMap<SimThingId, u32> {
    let mut remaining = available;
    demands
        .iter()
        .map(|demand| {
            let granted = demand.requested.min(remaining);
            remaining -= granted;
            (
                SimThingId::from_session_raw(demand.source_simthing_id_raw.expect("source")),
                granted,
            )
        })
        .collect()
}

#[test]
fn generic_constrained_clearing_is_authored_generation_paced_and_conserved() {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let mut supply_node = node();
    let claim_a = node();
    let actionband_claim = node();
    let claim_b = node();
    let supply_id = supply_node.id;
    let claim_a_id = claim_a.id;
    let actionband_id = actionband_claim.id;
    let claim_b_id = claim_b.id;
    supply_node.add_child(claim_a);
    supply_node.add_child(actionband_claim);
    supply_node.add_child(claim_b);
    root.add_child(supply_node);

    let rf_rows = vec![
        own(supply_id, 6, 0),
        own(claim_a_id, 0, 4),
        // This is ActionBand-originated, but it is deliberately an ordinary RF row.
        own(actionband_id, 0, 1),
        own(claim_b_id, 0, 4),
    ];
    let stamped = simthing_spec::reduce_owner_channel_rf(&root, &rf_rows, GenerationStamp::new(10))
        .expect("ordinary reduce-up");
    let report = stamped.product();
    assert_eq!(report.buckets.len(), 1);
    let scope = report.buckets[0].scope.clone();
    let clearing_authority = ClearingRemainderAuthority {
        granter: supply_id,
        generation: GenerationStamp::new(10),
    };

    // The priorities here are the existing CommandDeficit landing shape. No
    // clearing-local priority field or constructor exists.
    let demand_a = demand(&scope, claim_a_id, 4, 0);
    let demand_actionband = demand(&scope, actionband_id, 1, 1);
    let demand_b = demand(&scope, claim_b_id, 4, 2);
    let authored = vec![
        AuthoredClaimClearingData {
            demand: demand_a.clone(),
            order_weight: 1.0,
        },
        AuthoredClaimClearingData {
            demand: demand_actionband.clone(),
            order_weight: 2.0,
        },
        AuthoredClaimClearingData {
            demand: demand_b.clone(),
            order_weight: 9.0,
        },
    ];

    let priority = clear_reduced_owner_channels(report, &authored, &priority_program())
        .expect("priority-authored clearing");
    let priority_grants = grants(&priority[0]);
    assert_eq!(priority_grants[&claim_a_id], 4);
    assert_eq!(priority_grants[&actionband_id], 1);
    assert_eq!(priority_grants[&claim_b_id], 1);
    assert!(priority[0].is_oversubscribed());
    assert_eq!(priority[0].granted_total, 6);
    assert_eq!(priority[0].remaining_after, 0);

    // Claims and executor are unchanged. Authored numerical data alone changes
    // the emergent allocation from priority order to price order.
    let authored_before = authored.clone();
    let price = clear_reduced_owner_channels(report, &authored, &price_program())
        .expect("price-authored clearing");
    assert_eq!(authored, authored_before);
    let price_grants = grants(&price[0]);
    assert_eq!(price_grants[&claim_b_id], 4);
    assert_eq!(price_grants[&actionband_id], 1);
    assert_eq!(price_grants[&claim_a_id], 1);

    // The fitting posture goes through the identical generic function.
    let claims: Vec<ConstrainedClaim> = authored
        .iter()
        .map(|row| ConstrainedClaim::from_runtime_demand(&row.demand, row.order_weight).unwrap())
        .collect();
    let fit = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: scope.clone(),
            available: 9,
        }],
        &claims,
        &priority_program(),
        clearing_authority,
    )
    .expect("fitting clearing");
    assert!(!fit[0].is_oversubscribed());
    assert!(fit[0]
        .grants
        .iter()
        .all(|grant| grant.granted == grant.requested));

    // Stable logical ids defeat a physical-row policy. The planted mutant is
    // order-sensitive while production is identical under the same shuffle.
    let proportional = AuthoredClearingProgram::new(TransformOp::set(0.0));
    let lawful_a = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: scope.clone(),
            available: 6,
        }],
        &claims,
        &proportional,
        clearing_authority,
    )
    .unwrap();
    let mut shuffled_claims = claims.clone();
    shuffled_claims.reverse();
    let lawful_b = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: scope.clone(),
            available: 6,
        }],
        &shuffled_claims,
        &proportional,
        clearing_authority,
    )
    .unwrap();
    assert_eq!(lawful_a, lawful_b);
    let demand_rows = vec![demand_a, demand_actionband, demand_b];
    let mut reversed_demands = demand_rows.clone();
    reversed_demands.reverse();
    assert_ne!(
        row_order_mutant(&demand_rows, 6),
        row_order_mutant(&reversed_demands, 6)
    );

    // Full owner-channel keys segregate claims without owner equality or a
    // reconstructed identity plane.
    let foreign_scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("beta"),
        resource_key: ResourceKey::new("ore"),
        scope_id: ScopeId::from_boundary(SimThingId::from_session_raw(900)),
    };
    let foreign_id = SimThingId::from_session_raw(901);
    let foreign_demand = demand(&foreign_scope, foreign_id, 5, 0);
    let segregated = clear_constrained_claims_at_generation(
        &[
            ConstrainedSupply {
                scope: scope.clone(),
                available: 1,
            },
            ConstrainedSupply {
                scope: foreign_scope.clone(),
                available: 5,
            },
        ],
        &[
            ConstrainedClaim::from_runtime_demand(&authored[0].demand, 1.0).unwrap(),
            ConstrainedClaim::from_runtime_demand(&foreign_demand, 1.0).unwrap(),
        ],
        &proportional,
        clearing_authority,
    )
    .unwrap();
    assert_eq!(segregated.len(), 2);
    assert_eq!(segregated[0].scope, scope);
    assert_eq!(segregated[0].granted_total, 1);
    assert_eq!(segregated[1].scope, foreign_scope);
    assert_eq!(segregated[1].granted_total, 5);

    // U is requested-not-granted. It is not the CostBand remainder R.
    let unresolved_grant = priority[0]
        .grants
        .iter()
        .find(|grant| grant.source_simthing_id == claim_b_id)
        .expect("unresolved claim");
    let observation =
        UnresolvedDemandObservation::from_grant(unresolved_grant, GenerationStamp::new(10))
            .expect("U > 0");
    assert_eq!(observation.unresolved, 3);
    let valuation = AuthoredPersistenceValuation::new(TransformOp::multiply(1.0), 2.0)
        .expect("CostBand admission");
    let binding = PersistenceOverlayBinding {
        origin: root.id,
        target: claim_b_id,
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(1.0))],
        },
        dissolution_conditions: vec![DissolveCondition::AfterTicks { remaining: 3 }],
    };
    assert!(matches!(
        fund_unresolved_persistence(&observation, GenerationStamp::new(10), &valuation, &binding,),
        Err(PersistenceConsequenceError::SameGenerationConsequence)
    ));
    let consequence =
        fund_unresolved_persistence(&observation, GenerationStamp::new(11), &valuation, &binding)
            .expect("later-generation persistence");
    assert_eq!(consequence.cost_band.v, 3.0);
    assert_eq!(consequence.cost_band.n, 1);
    assert_eq!(consequence.cost_band.r, 1.0);
    assert_ne!(observation.unresolved as f32, consequence.cost_band.r);
    let overlay = consequence.overlay.expect("funded ordinary outcome");
    assert!(is_authored_until_dissolved(&overlay.lifecycle));
    deliver_routed_overlay(&mut root, claim_b_id, overlay).expect("ordinary overlay route");

    // The unchanged 8.1 judge sees executed grants, including the ActionBand-
    // originated ordinary row, and rejects a local ActionBand shortcut mutant.
    let granted_rows: Vec<_> = priority[0]
        .grants
        .iter()
        .map(|grant| own(grant.source_simthing_id, grant.granted, 0))
        .collect();
    let actionband_granted = granted_rows
        .iter()
        .find(|row| row.simthing_id == actionband_id)
        .expect("ActionBand grant")
        .clone();
    let channels = [ChannelBound {
        resource: ResourceKey::new("ore"),
        supply: 6,
        remainder: 0,
    }];
    let verdict = judge_conservation(&ConservationSnapshot {
        root: &root,
        own_aggregates: &granted_rows,
        channels: &channels,
        quantized: None,
        seam: None,
        stemthing: None,
        actionband_originated: std::slice::from_ref(&actionband_granted),
    })
    .expect("judge");
    assert_eq!(verdict, ConservationVerdict::Green);
    let mut local_shortcut_mutant = granted_rows.clone();
    local_shortcut_mutant
        .iter_mut()
        .find(|row| row.simthing_id == actionband_id)
        .expect("ActionBand grant")
        .surplus += 1;
    let mutant_actionband = local_shortcut_mutant
        .iter()
        .find(|row| row.simthing_id == actionband_id)
        .expect("ActionBand mutant")
        .clone();
    let mutant_verdict = judge_conservation(&ConservationSnapshot {
        root: &root,
        own_aggregates: &local_shortcut_mutant,
        channels: &channels,
        quantized: None,
        seam: None,
        stemthing: None,
        actionband_originated: std::slice::from_ref(&mutant_actionband),
    })
    .expect("judge mutant");
    assert_eq!(
        mutant_verdict,
        ConservationVerdict::Red(ConservationJudgeReason::SeededOverAccounting)
    );
}
