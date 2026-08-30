//! CLEARING-WEIGHT-DEFORMATION-LIFECYCLE-0 affected-only lifecycle witness.

use std::collections::BTreeMap;

use simthing_core::owner_channel::OwnerRef;
use simthing_core::{
    GenerationStamp, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_gpu::OverlaySpanProjection;
use simthing_spec::{
    clear_constrained_claims_at_generation, resolve_effective_clearing_weights,
    AuthoredClearingProgram, ChangedLocus, ClearingRemainderAuthority, ClearingWeightOverrideSpec,
    ConstrainedClaim, ConstrainedClearingResult, ConstrainedSupply, OwnerChannelScopeKey,
    ResourceKey, RuntimeOwnerSiloDemandBucket, ScopeId, OWNER_POLICY_WEIGHT_AUTHORITY_PROPERTY_ID,
};

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
    requested: u32,
    weight: f32,
) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(
        &RuntimeOwnerSiloDemandBucket {
            owner_ref: scope.owner_ref.clone(),
            resource_key: scope.resource_key.clone(),
            scope_id: scope.scope_id.clone(),
            requested,
            priority: 0,
            source_simthing_id_raw: Some(source.raw()),
        },
        weight,
    )
    .unwrap()
}

fn grants(result: &ConstrainedClearingResult) -> BTreeMap<SimThingId, u32> {
    result
        .grants
        .iter()
        .map(|grant| (grant.source_simthing_id, grant.granted))
        .collect()
}

#[test]
fn authored_operand_change_rebuilds_only_bound_spans_and_changes_next_generation_clear() {
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

    let participants = OverlaySpanProjection::compile(&root).unwrap();
    let default_locus = weight_locus(root.id);
    let ship_locus = weight_locus(ship_id);
    let initial_overrides = [ClearingWeightOverrideSpec {
        source_locus: ship_locus.clone(),
        simthing_id: ship_id,
        value_program: TransformOp::multiply(0.5),
    }];
    let mut weights = resolve_effective_clearing_weights(
        &participants,
        1.0,
        default_locus.clone(),
        &initial_overrides,
    )
    .unwrap();

    assert_eq!(weights.dependency_binding_count(), 2);
    assert_eq!(weights.effective_weight(ship_id), Some(0.5));
    assert_eq!(weights.effective_weight(freighter_id), Some(0.5));
    assert_eq!(weights.effective_weight(commodity_id), Some(1.0));
    let mut default_change_probe = weights.clone();
    let default_refresh = default_change_probe
        .refresh(
            2.0,
            &initial_overrides,
            std::slice::from_ref(&default_locus),
            GenerationStamp::new(8),
        )
        .unwrap();
    assert_eq!(default_refresh.affected_ranges, 1);
    assert_eq!(default_refresh.affected_logical_rows, 4);
    assert_eq!(default_refresh.dirty_spans, 3);
    assert_eq!(default_refresh.semantic_spans_rebuilt, 3);
    assert_eq!(default_refresh.spans_examined, 3);
    assert_eq!(default_refresh.logical_member_rows_scanned, 0);

    let scope = OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("deformation-lifecycle"),
        resource_key: ResourceKey::new("compute-quanta"),
        scope_id: ScopeId::from_boundary(root.id),
    };
    let supply = [ConstrainedSupply {
        scope: scope.clone(),
        available: 3,
    }];
    let program = AuthoredClearingProgram::new(TransformOp::multiply(1.0));
    let before_claims = [
        claim(
            &scope,
            ship_id,
            3,
            weights.effective_weight(ship_id).unwrap(),
        ),
        claim(
            &scope,
            commodity_id,
            3,
            weights.effective_weight(commodity_id).unwrap(),
        ),
    ];
    let before = clear_constrained_claims_at_generation(
        &supply,
        &before_claims,
        &program,
        ClearingRemainderAuthority {
            granter: root.id,
            generation: GenerationStamp::new(7),
        },
    )
    .unwrap();

    let updated_overrides = [ClearingWeightOverrideSpec {
        source_locus: ship_locus.clone(),
        simthing_id: ship_id,
        value_program: TransformOp::multiply(2.0),
    }];
    let refresh = weights
        .refresh(
            1.0,
            &updated_overrides,
            std::slice::from_ref(&ship_locus),
            GenerationStamp::new(8),
        )
        .unwrap();

    assert_eq!(refresh.affected_ranges, 1);
    assert_eq!(refresh.affected_logical_rows, 2);
    assert_eq!(refresh.dirty_spans, 1);
    assert_eq!(refresh.semantic_spans_rebuilt, 1);
    assert_eq!(refresh.spans_examined, 1);
    assert_eq!(refresh.logical_member_rows_scanned, 0);
    assert_eq!(refresh.unaffected_profile_identities_checked, 2);
    assert_eq!(refresh.unaffected_profile_identity_changes, 0);
    assert_eq!(weights.effective_weight(ship_id), Some(2.0));
    assert_eq!(weights.effective_weight(freighter_id), Some(2.0));
    assert_eq!(weights.effective_weight(commodity_id), Some(1.0));

    let after_claims = [
        claim(
            &scope,
            ship_id,
            3,
            weights.effective_weight(ship_id).unwrap(),
        ),
        claim(
            &scope,
            commodity_id,
            3,
            weights.effective_weight(commodity_id).unwrap(),
        ),
    ];
    let authority = ClearingRemainderAuthority {
        granter: root.id,
        generation: GenerationStamp::new(8),
    };
    let after =
        clear_constrained_claims_at_generation(&supply, &after_claims, &program, authority.clone())
            .unwrap();
    let replay =
        clear_constrained_claims_at_generation(&supply, &after_claims, &program, authority)
            .unwrap();

    assert_eq!(replay, after, "same N+1 inputs replay identically");
    assert_eq!(grants(&before[0])[&commodity_id], 3);
    assert_eq!(grants(&before[0])[&ship_id], 0);
    assert_eq!(grants(&after[0])[&commodity_id], 0);
    assert_eq!(grants(&after[0])[&ship_id], 3);
}
