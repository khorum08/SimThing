use super::*;

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
