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
