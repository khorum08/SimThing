use std::collections::BTreeMap;

use simthing_core::{
    deliver_routed_overlay, DimensionRegistry, GenerationStamp, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing,
    SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_kernel::{
    build_overlay_deltas, plan_overlay_orderband, ChangedLocus, DerivedDependencyBinding,
    DerivedDependencyIndex, DerivedDependencyTarget, DerivedSpanAdmissionError,
    DerivedSpanProjection, DerivedWorkId, EffectiveProfileId, EffectiveSpanSeed,
    FieldRegistrationAuthority, FieldRegistrationRef, LogicalRowRange, LogicalSubtreeDirectory,
    OverlayCompileCache, OverlayDenseMaterialization, OverlayProjectionHostChange,
    OverlaySpanProjection, SlotAllocator,
};

fn logical(raw: u32) -> SimThingId {
    SimThingId::from_session_raw(raw)
}

fn compact_directory(
    total: u64,
    rows: Vec<(SimThingId, LogicalRowRange)>,
) -> LogicalSubtreeDirectory {
    LogicalSubtreeDirectory::admit(total, rows).expect("compact logical directory admits")
}

#[test]
fn homogeneous_million_row_projection_rejects_descendant_scale_profile_explosion() {
    let root = logical(10_001);
    let directory = compact_directory(
        1_000_000,
        vec![(root, LogicalRowRange::new(0, 1_000_000).unwrap())],
    );
    let profile = EffectiveProfileId::from_semantic_digest(7);
    let projection = DerivedSpanProjection::admit(
        directory.clone(),
        vec![EffectiveSpanSeed::new(
            LogicalRowRange::new(0, 1_000_000).unwrap(),
            profile,
            "shared-profile",
        )],
        DerivedDependencyIndex::admit(Vec::new()).unwrap(),
    )
    .expect("one homogeneous span admits");
    assert_eq!(projection.profile_count(), 1);
    assert_eq!(projection.span_count(), 1);

    let mutant = DerivedSpanProjection::admit(
        directory,
        vec![
            EffectiveSpanSeed::new(
                LogicalRowRange::new(0, 1).unwrap(),
                profile,
                "shared-profile",
            ),
            EffectiveSpanSeed::new(
                LogicalRowRange::new(1, 999_999).unwrap(),
                profile,
                "shared-profile",
            ),
        ],
        DerivedDependencyIndex::admit(Vec::new()).unwrap(),
    );
    assert!(matches!(
        mutant,
        Err(DerivedSpanAdmissionError::DescendantScaleProfileExplosion { at_row: 1 })
    ));
}

#[test]
fn invalidation_visits_spans_not_depth_times_descendants() {
    let root = logical(10_010);
    let divergent = logical(10_011);
    let root_locus = ChangedLocus::new(root, SimPropertyId(0), SubFieldRole::Amount);
    let mut homogeneous = DerivedSpanProjection::admit(
        compact_directory(
            1_000_000,
            vec![(root, LogicalRowRange::new(0, 1_000_000).unwrap())],
        ),
        vec![EffectiveSpanSeed::new(
            LogicalRowRange::new(0, 1_000_000).unwrap(),
            EffectiveProfileId::from_semantic_digest(1),
            1u8,
        )],
        DerivedDependencyIndex::admit(vec![DerivedDependencyBinding::new(
            root_locus.clone(),
            DerivedDependencyTarget::SpanRoot(root),
        )])
        .unwrap(),
    )
    .unwrap();
    let all = homogeneous
        .invalidate(&[root_locus], GenerationStamp::new(8))
        .unwrap();
    assert_eq!(all.affected_ranges.len(), 1);
    assert_eq!(all.spans_examined, 1);
    assert_eq!(homogeneous.span_count(), 1);
    assert_eq!(all.logical_member_rows_scanned, 0);

    let directory = compact_directory(
        1_000_000,
        vec![
            (root, LogicalRowRange::new(0, 1_000_000).unwrap()),
            (divergent, LogicalRowRange::new(500_000, 1).unwrap()),
        ],
    );
    let seeds = (0..10_000u64)
        .map(|span| {
            EffectiveSpanSeed::new(
                LogicalRowRange::new(span * 100, 100).unwrap(),
                EffectiveProfileId::from_semantic_digest(span % 2 + 1),
                (span % 2) as u8,
            )
        })
        .collect();
    let divergent_locus = ChangedLocus::new(divergent, SimPropertyId(0), SubFieldRole::Amount);
    let mut projection = DerivedSpanProjection::admit(
        directory,
        seeds,
        DerivedDependencyIndex::admit(vec![DerivedDependencyBinding::new(
            divergent_locus.clone(),
            DerivedDependencyTarget::LogicalMember(divergent),
        )])
        .unwrap(),
    )
    .unwrap();
    assert_eq!(projection.span_count(), 10_000);

    let rebuilt = projection
        .remap_range(
            LogicalRowRange::new(500_000, 1).unwrap(),
            GenerationStamp::new(9),
            |_, _, _| (2u8, EffectiveProfileId::from_semantic_digest(3)),
        )
        .unwrap();
    assert_eq!(rebuilt, 1);
    assert_eq!(projection.profile_count(), 3);
    assert_eq!(projection.span_count(), 10_001);
    let local = projection
        .invalidate(&[divergent_locus], GenerationStamp::new(10))
        .unwrap();
    assert_eq!(local.dirty_span_ranges.len(), 1);
    assert_eq!(local.spans_examined, 1);
    assert_eq!(projection.span_count(), 10_001);
    assert_eq!(local.logical_member_rows_scanned, 0);
}

#[test]
fn changed_locus_rejects_writer_subsystem_discriminants() {
    let locus = ChangedLocus::new(logical(10_020), SimPropertyId(3), SubFieldRole::Velocity);
    let mut encoded = serde_json::to_value(&locus).unwrap();
    encoded
        .as_object_mut()
        .unwrap()
        .insert("change_source".into(), serde_json::json!("overlay-manager"));
    let error = serde_json::from_value::<ChangedLocus>(encoded).unwrap_err();
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn dependency_index_is_frozen_and_routes_exact_span_field_and_work_targets() {
    let changed = logical(10_030);
    let dependent = logical(10_031);
    let locus = ChangedLocus::new(changed, SimPropertyId(4), SubFieldRole::Amount);
    let stead = FieldRegistrationRef::new(FieldRegistrationAuthority::Stead, 11);
    let palma = FieldRegistrationRef::new(FieldRegistrationAuthority::Palma, 12);
    let guyang = FieldRegistrationRef::new(FieldRegistrationAuthority::GuYang, 13);
    let work = DerivedWorkId::new(14);
    let index = DerivedDependencyIndex::admit(vec![
        DerivedDependencyBinding::new(
            locus.clone(),
            DerivedDependencyTarget::LogicalMember(changed),
        ),
        DerivedDependencyBinding::new(locus.clone(), DerivedDependencyTarget::SpanRoot(dependent)),
        DerivedDependencyBinding::new(
            locus.clone(),
            DerivedDependencyTarget::FieldRegistration(stead),
        ),
        DerivedDependencyBinding::new(
            locus.clone(),
            DerivedDependencyTarget::FieldRegistration(palma),
        ),
        DerivedDependencyBinding::new(
            locus.clone(),
            DerivedDependencyTarget::FieldRegistration(guyang),
        ),
        DerivedDependencyBinding::new(locus.clone(), DerivedDependencyTarget::Work(work)),
    ])
    .unwrap();
    assert_eq!(index.binding_count(), 6);

    let directory = compact_directory(
        8,
        vec![
            (changed, LogicalRowRange::new(0, 1).unwrap()),
            (dependent, LogicalRowRange::new(4, 2).unwrap()),
        ],
    );
    let mut projection = DerivedSpanProjection::admit(
        directory,
        vec![
            EffectiveSpanSeed::new(
                LogicalRowRange::new(0, 1).unwrap(),
                EffectiveProfileId::from_semantic_digest(1),
                1u8,
            ),
            EffectiveSpanSeed::new(
                LogicalRowRange::new(1, 3).unwrap(),
                EffectiveProfileId::from_semantic_digest(2),
                2u8,
            ),
            EffectiveSpanSeed::new(
                LogicalRowRange::new(4, 2).unwrap(),
                EffectiveProfileId::from_semantic_digest(3),
                3u8,
            ),
            EffectiveSpanSeed::new(
                LogicalRowRange::new(6, 2).unwrap(),
                EffectiveProfileId::from_semantic_digest(2),
                2u8,
            ),
        ],
        index,
    )
    .unwrap();
    let invalidation = projection
        .invalidate(&[locus], GenerationStamp::new(4))
        .unwrap();
    assert_eq!(invalidation.dirty_span_ranges.len(), 2);
    assert_eq!(invalidation.field_registrations, vec![stead, palma, guyang]);
    assert_eq!(invalidation.work, vec![work]);
    assert_eq!(projection.dependency_index().binding_count(), 6);
}

fn registry() -> (DimensionRegistry, SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("span-proof", "signal", 0));
    (registry, property)
}

fn node_with_property(registry: &DimensionRegistry, property: SimPropertyId) -> SimThing {
    let mut node = SimThing::new(SimThingKind::Cohort, 0);
    node.add_property(property, registry.property(property).default_value());
    node
}

fn overlay(
    host: SimThingId,
    kind: OverlayKind,
    property: SimPropertyId,
    op: TransformOp,
) -> Overlay {
    let lifecycle = if matches!(kind, OverlayKind::Instruction | OverlayKind::Custom(_)) {
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![simthing_core::DissolveCondition::AtSessionEnd],
        }
    } else {
        OverlayLifecycle::UntilDissolved
    };
    Overlay {
        id: OverlayId::new(),
        kind,
        source: OverlaySource::System,
        origin: host,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: property,
            sub_field_deltas: vec![(SubFieldRole::Amount, op)],
        },
        lifecycle,
    }
}

fn find_mut(root: &mut SimThing, target: SimThingId) -> &mut SimThing {
    if root.id == target {
        return root;
    }
    root.children
        .iter_mut()
        .find_map(|child| {
            if child.id == target {
                Some(child)
            } else {
                find_mut_optional(child, target)
            }
        })
        .expect("target in test tree")
}

fn find_mut_optional(root: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if root.id == target {
        return Some(root);
    }
    root.children
        .iter_mut()
        .find_map(|child| find_mut_optional(child, target))
}

fn ops_for(
    id: SimThingId,
    allocator: &SlotAllocator,
    dense: &OverlayDenseMaterialization,
) -> Vec<(u32, u32, u32)> {
    let range = dense.ranges[allocator.slot_of(id).unwrap().as_usize()];
    dense.deltas[range.offset as usize..(range.offset + range.length) as usize]
        .iter()
        .map(|delta| (delta.col, delta.op_kind, delta.value.to_bits()))
        .collect()
}

#[test]
fn dense_materialization_is_deletable_cache_and_remaps_by_logical_identity() {
    let (registry, property) = registry();
    let mut root = node_with_property(&registry, property);
    root.add_overlay(overlay(
        root.id,
        OverlayKind::Policy,
        property,
        TransformOp::add(0.25),
    ));
    let child_a = node_with_property(&registry, property);
    let child_a_id = child_a.id;
    let child_b = node_with_property(&registry, property);
    let child_b_id = child_b.id;
    root.add_child(child_a);
    root.add_child(child_b);

    let projection = OverlaySpanProjection::compile(&root).unwrap();
    assert_eq!(projection.metrics().profiles, 1);
    assert_eq!(projection.metrics().spans, 1);
    let semantic_before = projection.profile_digest_by_logical_identity();

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let dense_before = projection.materialize_dense(&registry, &allocator);
    let by_id_before = [
        (root.id, ops_for(root.id, &allocator, &dense_before)),
        (child_a_id, ops_for(child_a_id, &allocator, &dense_before)),
        (child_b_id, ops_for(child_b_id, &allocator, &dense_before)),
    ];
    let plan = plan_overlay_orderband(
        &dense_before.deltas,
        &dense_before.ranges,
        allocator.capacity() as u32,
    );
    let mut cache = OverlayCompileCache {
        compiled_at_revision: 1,
        projection,
        cached_deltas: dense_before.deltas.clone(),
        cached_ranges: dense_before.ranges.clone(),
        cached_n_bands: plan.n_bands,
        cached_op_buffer_uploaded_n_ops: plan.ops.len() as u32,
        compile_count: 1,
        upload_count: 1,
    };
    cache.drop_dense_materialization();
    assert!(cache.cached_deltas.is_empty());
    assert!(cache.cached_ranges.is_empty());
    assert_eq!(
        cache.projection.profile_digest_by_logical_identity(),
        semantic_before
    );
    let rebuilt = cache.rebuild_dense_materialization(&registry, &allocator);
    assert_eq!(rebuilt, dense_before);
    let rebuilt_again = cache.rebuild_dense_materialization(&registry, &allocator);
    assert_eq!(rebuilt, rebuilt_again);

    let pre = allocator.binding_table_snapshot();
    let mut ids = pre.keys().copied().collect::<Vec<_>>();
    let mut slots = pre.values().copied().collect::<Vec<_>>();
    ids.sort_unstable();
    slots.sort_unstable();
    slots.reverse();
    let assignment = ids.into_iter().zip(slots).collect::<BTreeMap<_, _>>();
    allocator
        .epoch_rebind(&assignment, &BTreeMap::new(), &BTreeMap::new())
        .unwrap();
    cache.drop_dense_materialization();
    let after_remap = cache.rebuild_dense_materialization(&registry, &allocator);
    for (id, expected) in by_id_before {
        assert_eq!(ops_for(id, &allocator, &after_remap), expected);
    }
    assert_eq!(
        cache.projection.profile_digest_by_logical_identity(),
        semantic_before
    );
}

#[test]
fn standing_and_routed_projection_match_inheritance_oracle_after_local_split() {
    let (registry, property) = registry();
    let mut root = node_with_property(&registry, property);
    let mut policy_host = node_with_property(&registry, property);
    let origin = node_with_property(&registry, property);
    let origin_id = origin.id;
    policy_host.add_child(origin);
    let policy_host_id = policy_host.id;
    let mut deferred_policy = overlay(
        policy_host_id,
        OverlayKind::Policy,
        property,
        TransformOp::multiply(0.5),
    );
    let deferred_policy_id = deferred_policy.id;
    deferred_policy.lifecycle = OverlayLifecycle::Suspended {
        when_activated: Box::new(OverlayLifecycle::UntilDissolved),
    };
    policy_host.add_overlay(deferred_policy);
    let receiver = node_with_property(&registry, property);
    let receiver_id = receiver.id;
    root.add_child(policy_host);
    root.add_child(receiver);
    let instruction = overlay(
        origin_id,
        OverlayKind::Instruction,
        property,
        TransformOp::add(0.4),
    );
    deliver_routed_overlay(&mut root, receiver_id, instruction).unwrap();

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let mut projection = OverlaySpanProjection::compile(&root).unwrap();
    assert!(projection.dependency_index().binding_count() > 0);
    let initial = projection.materialize_dense(&registry, &allocator);
    let (oracle_deltas, oracle_ranges) = build_overlay_deltas(&root, &registry, &allocator);
    assert_eq!(initial.deltas, oracle_deltas);
    assert_eq!(initial.ranges, oracle_ranges);

    find_mut(&mut root, policy_host_id)
        .overlays
        .iter_mut()
        .find(|overlay| overlay.id == deferred_policy_id)
        .unwrap()
        .lifecycle = OverlayLifecycle::UntilDissolved;
    let refresh = projection
        .refresh(
            &root,
            &[OverlayProjectionHostChange::OverlayState(policy_host_id)],
            GenerationStamp::new(11),
        )
        .unwrap();
    assert_eq!(refresh.invalidation.logical_member_rows_scanned, 0);
    assert!(refresh.semantic_spans_rebuilt > 0);
    let incrementally_rebuilt = projection.materialize_dense(&registry, &allocator);
    let (oracle_deltas, oracle_ranges) = build_overlay_deltas(&root, &registry, &allocator);
    assert_eq!(incrementally_rebuilt.deltas, oracle_deltas);
    assert_eq!(incrementally_rebuilt.ranges, oracle_ranges);

    find_mut(&mut root, policy_host_id).add_overlay(overlay(
        policy_host_id,
        OverlayKind::Policy,
        property,
        TransformOp::add(0.1),
    ));
    assert!(matches!(
        projection.refresh(
            &root,
            &[OverlayProjectionHostChange::OverlayState(policy_host_id)],
            GenerationStamp::new(12),
        ),
        Err(DerivedSpanAdmissionError::FrozenDependencyShapeChanged(id)) if id == policy_host_id
    ));
}
