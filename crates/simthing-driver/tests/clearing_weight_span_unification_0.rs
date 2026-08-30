//! CLEARING-WEIGHT-SPAN-UNIFICATION-0 cross-domain parity proofs.

use std::collections::BTreeMap;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    GenerationStamp, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_gpu::OverlaySpanProjection;
use simthing_spec::{
    clear_constrained_claims_at_generation, resolve_effective_clearing_weights,
    AuthoredClearingProgram, ChangedLocus, ClearingRemainderAuthority, ClearingWeightOverrideSpec,
    ClearingWeightResolutionError, ConstrainedClaim, ConstrainedClearingResult, ConstrainedSupply,
    OwnerChannelScopeKey, ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId,
    OWNER_POLICY_WEIGHT_AUTHORITY_PROPERTY_ID,
};

struct Participants {
    root: SimThing,
    ship: SimThingId,
    commodity: SimThingId,
    freighter: SimThingId,
}

fn participants() -> Participants {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut ship = SimThing::new(SimThingKind::Custom("ship".into()), 0);
    let freighter = SimThing::new(SimThingKind::Custom("freighter".into()), 0);
    let commodity = SimThing::new(SimThingKind::Custom("commodity".into()), 0);
    let ship_id = ship.id;
    let freighter_id = freighter.id;
    let commodity_id = commodity.id;
    ship.add_child(freighter);
    root.add_child(ship);
    root.add_child(commodity);
    Participants {
        root,
        ship: ship_id,
        commodity: commodity_id,
        freighter: freighter_id,
    }
}

fn scope(root: SimThingId) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("cross-domain"),
        resource_key: ResourceKey::new("decimal-commodity-quanta"),
        scope_id: ScopeId::from_boundary(root),
    }
}

fn weight_locus(id: SimThingId) -> ChangedLocus {
    ChangedLocus::new(
        id,
        OWNER_POLICY_WEIGHT_AUTHORITY_PROPERTY_ID,
        SubFieldRole::Amount,
    )
}

fn claim(
    scope: &OwnerChannelScopeKey,
    source: SimThingId,
    requested_decimal_quanta: u32,
    weight: f32,
) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(
        &RuntimeOwnerSiloDemandBucket {
            owner_ref: scope.owner_ref.clone(),
            resource_key: scope.resource_key.clone(),
            scope_id: scope.scope_id.clone(),
            requested: requested_decimal_quanta,
            priority: 0,
            source_simthing_id_raw: Some(source.raw()),
        },
        weight,
    )
    .unwrap()
}

fn grant_map(result: &ConstrainedClearingResult) -> BTreeMap<SimThingId, u32> {
    result
        .grants
        .iter()
        .map(|grant| (grant.source_simthing_id, grant.granted))
        .collect()
}

#[test]
fn inherited_weight_profiles_are_bit_exact_maximal_and_fail_closed() {
    let fixture = participants();
    let projection = OverlaySpanProjection::compile(&fixture.root).unwrap();
    let one_ulp_above_one = f32::from_bits(1.0f32.to_bits() + 1);
    let overrides = [
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(fixture.ship),
            simthing_id: fixture.ship,
            value_program: TransformOp::multiply(4.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(fixture.freighter),
            simthing_id: fixture.freighter,
            value_program: TransformOp::multiply(1.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(fixture.commodity),
            simthing_id: fixture.commodity,
            value_program: TransformOp::set(one_ulp_above_one),
        },
    ];
    let weights = resolve_effective_clearing_weights(
        &projection,
        0.5,
        weight_locus(fixture.root.id),
        &overrides,
    )
    .unwrap();

    assert_eq!(
        weights.effective_weight(fixture.root.id).unwrap().to_bits(),
        0.5f32.to_bits()
    );
    assert_eq!(
        weights.effective_weight(fixture.ship).unwrap().to_bits(),
        2.0f32.to_bits()
    );
    assert_eq!(
        weights
            .effective_weight(fixture.freighter)
            .unwrap()
            .to_bits(),
        2.0f32.to_bits()
    );
    assert_eq!(
        weights
            .effective_weight(fixture.commodity)
            .unwrap()
            .to_bits(),
        one_ulp_above_one.to_bits(),
        "one-ULP authored distinction is exact rather than epsilon-folded"
    );
    assert_eq!(
        weights.profile_and_span_counts(),
        (3, 3),
        "the no-op descendant override stays in its inherited maximal span"
    );

    assert!(matches!(
        resolve_effective_clearing_weights(
            &projection,
            f32::NAN,
            weight_locus(fixture.root.id),
            &[]
        ),
        Err(ClearingWeightResolutionError::InvalidDefault)
    ));
    assert!(matches!(
        resolve_effective_clearing_weights(
            &projection,
            1.0,
            weight_locus(fixture.root.id),
            &[overrides[0].clone(), overrides[0].clone()]
        ),
        Err(ClearingWeightResolutionError::DuplicateOverride(id)) if id == fixture.ship
    ));
    let absent = SimThingId::from_session_raw(u32::MAX - 10);
    assert!(matches!(
        resolve_effective_clearing_weights(
            &projection,
            1.0,
            weight_locus(fixture.root.id),
            &[ClearingWeightOverrideSpec {
                source_locus: weight_locus(fixture.root.id),
                simthing_id: absent,
                value_program: TransformOp::set(1.0),
            }],
        ),
        Err(ClearingWeightResolutionError::UnknownSimThing(id)) if id == absent
    ));
    assert!(matches!(
        resolve_effective_clearing_weights(
            &projection,
            1.0,
            weight_locus(fixture.root.id),
            &[ClearingWeightOverrideSpec {
                source_locus: weight_locus(fixture.ship),
                simthing_id: fixture.ship,
                value_program: TransformOp::multiply(-1.0),
            }],
        ),
        Err(ClearingWeightResolutionError::InvalidResolvedWeight(id)) if id == fixture.ship
    ));
}

#[test]
fn ship_commodity_freighter_matrix_preserves_order_ties_decimal_apportionment_and_replay() {
    let fixture = participants();
    let projection = OverlaySpanProjection::compile(&fixture.root).unwrap();
    let weights = resolve_effective_clearing_weights(
        &projection,
        1.0,
        weight_locus(fixture.root.id),
        &[
            ClearingWeightOverrideSpec {
                source_locus: weight_locus(fixture.ship),
                simthing_id: fixture.ship,
                value_program: TransformOp::multiply(2.0),
            },
            ClearingWeightOverrideSpec {
                source_locus: weight_locus(fixture.commodity),
                simthing_id: fixture.commodity,
                value_program: TransformOp::set(f32::from_bits(1.0f32.to_bits() + 1)),
            },
        ],
    )
    .unwrap();
    let scope = scope(fixture.root.id);
    let weighted_claims = vec![
        claim(
            &scope,
            fixture.ship,
            3,
            weights.effective_weight(fixture.ship).unwrap(),
        ),
        claim(
            &scope,
            fixture.commodity,
            3,
            weights.effective_weight(fixture.commodity).unwrap(),
        ),
        claim(
            &scope,
            fixture.freighter,
            3,
            weights.effective_weight(fixture.freighter).unwrap(),
        ),
    ];
    let supply = [ConstrainedSupply {
        scope: scope.clone(),
        available: 5,
    }];
    let weighted_program = AuthoredClearingProgram::new(TransformOp::multiply(1.0));
    let authority = |generation| ClearingRemainderAuthority {
        granter: fixture.root.id,
        generation: GenerationStamp::new(generation),
    };

    let first = clear_constrained_claims_at_generation(
        &supply,
        &weighted_claims,
        &weighted_program,
        authority(7),
    )
    .unwrap();
    let replay = clear_constrained_claims_at_generation(
        &supply,
        &weighted_claims,
        &weighted_program,
        authority(7),
    )
    .unwrap();
    assert_eq!(replay, first, "same stamped decision replays identically");

    let mut reversed = weighted_claims.clone();
    reversed.reverse();
    let reordered =
        clear_constrained_claims_at_generation(&supply, &reversed, &weighted_program, authority(7))
            .unwrap();
    assert_eq!(reordered, first, "physical claim order is not semantic");
    let next_generation = clear_constrained_claims_at_generation(
        &supply,
        &weighted_claims,
        &weighted_program,
        authority(8),
    )
    .unwrap();
    assert_ne!(
        grant_map(&first[0]),
        grant_map(&next_generation[0]),
        "the equal ship/freighter weighted tie rotates under real generation authority"
    );
    assert_eq!(grant_map(&first[0])[&fixture.commodity], 0);

    // DecimalField quantities are canonical integer hundredths. The exact
    // 100:200:300 apportionment of 100 hundredths is 17:33:50, with no float
    // conversion or tolerance in the clearing arithmetic.
    let decimal_claims = vec![
        claim(
            &scope,
            fixture.ship,
            100,
            weights.effective_weight(fixture.ship).unwrap(),
        ),
        claim(
            &scope,
            fixture.commodity,
            200,
            weights.effective_weight(fixture.commodity).unwrap(),
        ),
        claim(
            &scope,
            fixture.freighter,
            300,
            weights.effective_weight(fixture.freighter).unwrap(),
        ),
    ];
    let decimal = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope,
            available: 100,
        }],
        &decimal_claims,
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        authority(7),
    )
    .unwrap();
    let exact = grant_map(&decimal[0]);
    assert_eq!(exact[&fixture.ship], 17);
    assert_eq!(exact[&fixture.commodity], 33);
    assert_eq!(exact[&fixture.freighter], 50);
    assert_eq!(decimal[0].granted_total, 100);
    assert_eq!(decimal[0].remaining_after, 0);
}
