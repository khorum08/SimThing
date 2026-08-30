//! CLEARING-WEIGHT-SEMANTIC-PARTITION-0 derivation-boundary witnesses.

use simthing_core::{
    GenerationStamp, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_gpu::OverlaySpanProjection;
use simthing_spec::{
    resolve_effective_clearing_weights, ChangedLocus, ClearingWeightOverrideSpec,
    OWNER_POLICY_WEIGHT_AUTHORITY_PROPERTY_ID,
};

fn weight_locus(id: SimThingId) -> ChangedLocus {
    ChangedLocus::new(
        id,
        OWNER_POLICY_WEIGHT_AUTHORITY_PROPERTY_ID,
        SubFieldRole::Amount,
    )
}

#[test]
fn exact_s1_reconstitutes_equal_valued_child_boundary_after_default_change() {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut child = SimThing::new(SimThingKind::Custom("child".into()), 0);
    let grandchild = SimThing::new(SimThingKind::Custom("grandchild".into()), 0);
    let sibling = SimThing::new(SimThingKind::Custom("sibling".into()), 0);
    let root_id = root.id;
    let child_id = child.id;
    let grandchild_id = grandchild.id;
    let sibling_id = sibling.id;
    child.add_child(grandchild);
    root.add_child(child);
    root.add_child(sibling);

    let participants = OverlaySpanProjection::compile(&root).unwrap();
    let default_locus = weight_locus(root_id);
    let overrides = [ClearingWeightOverrideSpec {
        source_locus: weight_locus(child_id),
        simthing_id: child_id,
        value_program: TransformOp::set(1.0),
    }];
    let mut weights =
        resolve_effective_clearing_weights(&participants, 1.0, default_locus.clone(), &overrides)
            .unwrap();

    assert_eq!(weights.profile_and_span_counts(), (1, 1));
    assert_eq!(weights.effective_weight(root_id), Some(1.0));
    assert_eq!(weights.effective_weight(child_id), Some(1.0));
    assert_eq!(weights.effective_weight(grandchild_id), Some(1.0));
    assert_eq!(weights.effective_weight(sibling_id), Some(1.0));

    let refresh = weights
        .refresh(
            2.0,
            &overrides,
            std::slice::from_ref(&default_locus),
            GenerationStamp::new(1),
        )
        .unwrap();

    assert_eq!(refresh.affected_ranges, 1);
    assert_eq!(refresh.affected_logical_rows, 4);
    assert_eq!(refresh.dirty_spans, 1);
    assert_eq!(refresh.semantic_spans_rebuilt, 3);
    assert_eq!(refresh.spans_examined, 1);
    assert_eq!(refresh.logical_member_rows_scanned, 0);
    assert_eq!(refresh.unaffected_profile_identities_checked, 0);
    assert_eq!(refresh.unaffected_profile_identity_changes, 0);
    assert_eq!(weights.profile_and_span_counts(), (2, 3));
    assert_eq!(weights.effective_weight(root_id), Some(2.0));
    assert_eq!(weights.effective_weight(child_id), Some(1.0));
    assert_eq!(weights.effective_weight(grandchild_id), Some(1.0));
    assert_eq!(weights.effective_weight(sibling_id), Some(2.0));

    let recoalesced = weights
        .refresh(
            1.0,
            &overrides,
            std::slice::from_ref(&default_locus),
            GenerationStamp::new(2),
        )
        .unwrap();
    assert_eq!(recoalesced.semantic_spans_rebuilt, 3);
    assert_eq!(recoalesced.logical_member_rows_scanned, 0);
    assert_eq!(weights.profile_and_span_counts(), (1, 1));
    assert_eq!(weights.effective_weight(root_id), Some(1.0));
    assert_eq!(weights.effective_weight(child_id), Some(1.0));
    assert_eq!(weights.effective_weight(grandchild_id), Some(1.0));
    assert_eq!(weights.effective_weight(sibling_id), Some(1.0));
}

#[test]
fn nested_boundary_survives_equal_compression_and_refresh_stays_range_local() {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let before_a = SimThing::new(SimThingKind::Custom("before-a".into()), 0);
    let before_b = SimThing::new(SimThingKind::Custom("before-b".into()), 0);
    let mut target = SimThing::new(SimThingKind::Custom("target".into()), 0);
    let target_leaf = SimThing::new(SimThingKind::Custom("target-leaf".into()), 0);
    let after_a = SimThing::new(SimThingKind::Custom("after-a".into()), 0);
    let after_b = SimThing::new(SimThingKind::Custom("after-b".into()), 0);
    let root_id = root.id;
    let before_a_id = before_a.id;
    let before_b_id = before_b.id;
    let target_id = target.id;
    let target_leaf_id = target_leaf.id;
    let after_a_id = after_a.id;
    let after_b_id = after_b.id;
    target.add_child(target_leaf);
    root.add_child(before_a);
    root.add_child(before_b);
    root.add_child(target);
    root.add_child(after_a);
    root.add_child(after_b);

    let participants = OverlaySpanProjection::compile(&root).unwrap();
    let target_locus = weight_locus(target_id);
    let initial_overrides = vec![
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(before_a_id),
            simthing_id: before_a_id,
            value_program: TransformOp::set(10.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(before_b_id),
            simthing_id: before_b_id,
            value_program: TransformOp::set(20.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: target_locus.clone(),
            simthing_id: target_id,
            value_program: TransformOp::multiply(2.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(target_leaf_id),
            simthing_id: target_leaf_id,
            value_program: TransformOp::set(2.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(after_a_id),
            simthing_id: after_a_id,
            value_program: TransformOp::set(30.0),
        },
        ClearingWeightOverrideSpec {
            source_locus: weight_locus(after_b_id),
            simthing_id: after_b_id,
            value_program: TransformOp::set(40.0),
        },
    ];
    let mut weights = resolve_effective_clearing_weights(
        &participants,
        1.0,
        weight_locus(root_id),
        &initial_overrides,
    )
    .unwrap();

    assert_eq!(weights.profile_and_span_counts(), (6, 6));
    assert_eq!(weights.effective_weight(target_id), Some(2.0));
    assert_eq!(weights.effective_weight(target_leaf_id), Some(2.0));

    let mut changed_overrides = initial_overrides.clone();
    changed_overrides[2].value_program = TransformOp::multiply(3.0);
    let refresh = weights
        .refresh(
            1.0,
            &changed_overrides,
            std::slice::from_ref(&target_locus),
            GenerationStamp::new(1),
        )
        .unwrap();

    assert_eq!(refresh.affected_ranges, 1);
    assert_eq!(refresh.affected_logical_rows, 2);
    assert_eq!(refresh.dirty_spans, 1);
    assert_eq!(refresh.semantic_spans_rebuilt, 2);
    assert_eq!(refresh.spans_examined, 1);
    assert_eq!(refresh.logical_member_rows_scanned, 0);
    assert_eq!(refresh.unaffected_profile_identities_checked, 2);
    assert_eq!(refresh.unaffected_profile_identity_changes, 0);
    assert_eq!(weights.profile_and_span_counts(), (7, 7));
    assert_eq!(weights.effective_weight(target_id), Some(3.0));
    assert_eq!(weights.effective_weight(target_leaf_id), Some(2.0));
    assert_eq!(weights.effective_weight(before_a_id), Some(10.0));
    assert_eq!(weights.effective_weight(before_b_id), Some(20.0));
    assert_eq!(weights.effective_weight(after_a_id), Some(30.0));
    assert_eq!(weights.effective_weight(after_b_id), Some(40.0));

    let mut equal_again = changed_overrides;
    equal_again[2].value_program = TransformOp::multiply(2.0);
    let recoalesced = weights
        .refresh(
            1.0,
            &equal_again,
            std::slice::from_ref(&target_locus),
            GenerationStamp::new(2),
        )
        .unwrap();

    assert_eq!(recoalesced.affected_ranges, 1);
    assert_eq!(recoalesced.affected_logical_rows, 2);
    assert_eq!(recoalesced.semantic_spans_rebuilt, 2);
    assert_eq!(recoalesced.logical_member_rows_scanned, 0);
    assert_eq!(recoalesced.unaffected_profile_identities_checked, 2);
    assert_eq!(recoalesced.unaffected_profile_identity_changes, 0);
    assert_eq!(weights.profile_and_span_counts(), (6, 6));
    assert_eq!(weights.effective_weight(target_id), Some(2.0));
    assert_eq!(weights.effective_weight(target_leaf_id), Some(2.0));
}
