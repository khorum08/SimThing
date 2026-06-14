//! MapGen PR4 — bounded Resource Flow enrollment tests.

use simthing_clausething::{
    MAPGEN_RF_DEPOSIT_ARENA, MAPGEN_RF_SUPPRESSION_ARENA, MapGenLatticeOptions,
    MapGenResourceFlowOptions, generate_default_mapgen_resource_flow_enrollment,
    generate_mapgen_lattice_hierarchy, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, validate_arena_caps, validate_explicit_enrollment,
    validate_resource_flow_enrollment,
};
use simthing_core::{DimensionRegistry, SimThingKind};
use simthing_spec::{
    ArenaSpec, EnrollmentSelectorSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    SpecError, compile_property, compile_resource_flow_admission,
};

const RAW_FIXTURE: &str = include_str!("fixtures/mapgen/tiny_pentad_hub_slice_raw.clause");

fn default_hierarchy() -> simthing_clausething::MapGenLatticeHierarchy {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse raw fixture");
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("generate lattice hierarchy")
}

fn default_enrollment() -> simthing_clausething::MapGenResourceFlowEnrollment {
    generate_mapgen_resource_flow_enrollment(
        &default_hierarchy(),
        MapGenResourceFlowOptions::default(),
    )
    .expect("generate RF enrollment")
}

fn registry_for_rf_admission(
    pack: &simthing_clausething::HydratedScenarioPack,
) -> DimensionRegistry {
    let mut registry = DimensionRegistry::new();
    for property in &pack.game_mode.properties {
        if property.name == "deposit_minerals_flow" || property.name == "suppression_flow" {
            compile_property(property, &mut registry).expect("compile RF flow property");
        }
    }
    registry
}

fn deposit_arena(enrollment: &simthing_clausething::MapGenResourceFlowEnrollment) -> &ArenaSpec {
    enrollment
        .pack
        .game_mode
        .resource_flow
        .as_ref()
        .expect("resource_flow")
        .arenas
        .iter()
        .find(|arena| arena.name == MAPGEN_RF_DEPOSIT_ARENA)
        .expect("deposit arena")
}

fn suppression_arena(
    enrollment: &simthing_clausething::MapGenResourceFlowEnrollment,
) -> &ArenaSpec {
    enrollment
        .pack
        .game_mode
        .resource_flow
        .as_ref()
        .expect("resource_flow")
        .arenas
        .iter()
        .find(|arena| arena.name == MAPGEN_RF_SUPPRESSION_ARENA)
        .expect("suppression arena")
}

#[test]
fn pr2_adapter_and_pr3_hierarchy_still_succeed_before_pr4() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    assert!(neutral.source_byte_len > 0);
    generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 hierarchy");
}

#[test]
fn tiny_fixture_generates_bounded_rf_enrollment() {
    let enrollment = default_enrollment();
    assert_eq!(enrollment.expansion_report.arenas.len(), 2);
    assert!(enrollment.pack.game_mode.resource_flow.is_some());
}

#[test]
fn generated_rf_participants_are_explicit_not_property_possession_implicit() {
    let enrollment = default_enrollment();
    let deposit = deposit_arena(&enrollment);
    let suppression = suppression_arena(&enrollment);

    assert_eq!(deposit.explicit_participants.len(), 1);
    // DA repair (PR4): both arenas enroll via ExplicitOnly over their authoritative
    // explicit_participants list — the deposit arena must NOT single out one deposit via an
    // InstallTarget selector (multi-deposit-safe; see mapgen_resource_flow.rs deposit-arena note).
    assert_eq!(deposit.enrollment, Some(EnrollmentSelectorSpec::ExplicitOnly));
    assert_eq!(suppression.explicit_participants.len(), 5);
    assert_eq!(
        suppression.enrollment,
        Some(EnrollmentSelectorSpec::ExplicitOnly)
    );

    let registry = registry_for_rf_admission(&enrollment.pack);
    compile_resource_flow_admission(
        enrollment.pack.game_mode.resource_flow.as_ref().unwrap(),
        &registry,
    )
    .expect("admission compile");
}

#[test]
fn generated_arenas_declare_all_caps() {
    let enrollment = default_enrollment();
    for arena in &enrollment
        .pack
        .game_mode
        .resource_flow
        .as_ref()
        .unwrap()
        .arenas
    {
        validate_arena_caps(arena).expect("arena caps");
        assert!(arena.max_participants > 0);
        assert!(arena.max_coupling_fanout > 0);
        assert!(arena.max_orderband_depth > 0);
    }
}

#[test]
fn generated_expansion_report_exists_and_is_bounded() {
    let enrollment = default_enrollment();
    let deposit_report = enrollment
        .expansion_report
        .arenas
        .iter()
        .find(|arena| arena.arena_id == MAPGEN_RF_DEPOSIT_ARENA)
        .expect("deposit report");
    assert_eq!(deposit_report.participant_count, 1);
    assert_eq!(deposit_report.max_participants, 4);
    assert_eq!(deposit_report.coupling_fanout, 1);
    assert_eq!(deposit_report.max_coupling_fanout, 4);
    assert_eq!(deposit_report.max_orderband_depth, 8);
    assert_eq!(
        deposit_report.source_properties_enrolled,
        vec!["mapgen::deposit_minerals_flow".to_string()]
    );
    assert_eq!(deposit_report.rejected_implicit_participants_count, 0);
    assert!(deposit_report.unsafe_expansion_flags.is_empty());

    let suppression_report = enrollment
        .expansion_report
        .arenas
        .iter()
        .find(|arena| arena.arena_id == MAPGEN_RF_SUPPRESSION_ARENA)
        .expect("suppression report");
    assert_eq!(suppression_report.participant_count, 5);
    assert_eq!(suppression_report.max_participants, 8);
}

#[test]
fn generated_output_preserves_ordinary_simthing_hierarchy() {
    let enrollment = default_enrollment();
    assert_eq!(enrollment.pack.root.kind, SimThingKind::World);
    assert!(enrollment.pack.w_impedance_compose.is_none());
    assert!(enrollment.pack.stress_compose.is_none());
    assert!(enrollment.pack.palma_feedstock.is_none());
    assert!(enrollment.pack.commitment.is_none());
    assert!(enrollment.pack.grid_metadata.links.is_empty());
}

#[test]
fn deposit_intrinsic_flow_obligation_is_authored() {
    let enrollment = default_enrollment();
    let obligations = &enrollment
        .pack
        .game_mode
        .resource_flow
        .as_ref()
        .unwrap()
        .base_obligations;
    assert_eq!(obligations.len(), 1);
    assert_eq!(obligations[0].arena, MAPGEN_RF_DEPOSIT_ARENA);
    assert_eq!(obligations[0].rate, 4.0);
}

#[test]
fn convenience_default_pipeline_succeeds() {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("parse");
    generate_default_mapgen_resource_flow_enrollment(&neutral).expect("default pipeline");
}

#[test]
fn missing_explicit_enrollment_is_rejected() {
    let mut arena = sample_arena();
    arena.enrollment = None;
    let err = validate_explicit_enrollment(&arena).unwrap_err();
    assert!(err.message.contains("missing explicit enrollment"));
}

#[test]
fn arena_missing_max_participants_is_rejected() {
    let mut arena = sample_arena();
    arena.max_participants = 0;
    let err = validate_arena_caps(&arena).unwrap_err();
    assert!(err.message.contains("missing max_participants"));
}

#[test]
fn arena_missing_max_coupling_fanout_is_rejected() {
    let mut arena = sample_arena();
    arena.max_coupling_fanout = 0;
    let err = validate_arena_caps(&arena).unwrap_err();
    assert!(err.message.contains("missing max_coupling_fanout"));
}

#[test]
fn arena_missing_max_orderband_depth_is_rejected() {
    let mut arena = sample_arena();
    arena.max_orderband_depth = 0;
    let err = validate_arena_caps(&arena).unwrap_err();
    assert!(err.message.contains("missing max_orderband_depth"));
}

#[test]
fn participant_count_beyond_cap_is_rejected() {
    let hierarchy = default_hierarchy();
    let err = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            suppression_max_participants: 2,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .unwrap_err();
    assert!(err.message.contains("exceeds suppression max_participants"));
}

#[test]
fn coupling_fanout_beyond_cap_is_rejected() {
    let mut spec = default_enrollment()
        .pack
        .game_mode
        .resource_flow
        .clone()
        .unwrap();
    spec.couplings.push(simthing_spec::CouplingSpec {
        from_arena: MAPGEN_RF_DEPOSIT_ARENA.into(),
        to_arena: MAPGEN_RF_SUPPRESSION_ARENA.into(),
        delay: simthing_spec::CouplingDelaySpec::Algebraic,
    });
    spec.couplings.push(simthing_spec::CouplingSpec {
        from_arena: MAPGEN_RF_DEPOSIT_ARENA.into(),
        to_arena: MAPGEN_RF_SUPPRESSION_ARENA.into(),
        delay: simthing_spec::CouplingDelaySpec::OneTickDelay,
    });
    for arena in &mut spec.arenas {
        arena.max_coupling_fanout = 1;
    }
    let err = validate_resource_flow_enrollment(&spec).unwrap_err();
    assert!(err.message.contains("coupling fanout"));
}

fn implicit_participation_spec(
    enrollment: &simthing_clausething::MapGenResourceFlowEnrollment,
) -> simthing_spec::ResourceFlowSpec {
    let mut spec = enrollment.pack.game_mode.resource_flow.clone().unwrap();
    for arena in &mut spec.arenas {
        arena.explicit_participants.clear();
        arena.enrollment = Some(EnrollmentSelectorSpec::ExplicitOnly);
        arena.wildcard_admission = None;
    }
    spec
}

#[test]
fn implicit_property_possession_admission_is_rejected_by_admission_compiler() {
    let enrollment = default_enrollment();
    let spec = implicit_participation_spec(&enrollment);
    let registry = registry_for_rf_admission(&enrollment.pack);
    let err = compile_resource_flow_admission(&spec, &registry).unwrap_err();
    assert!(
        matches!(err, SpecError::ImplicitParticipation { .. })
            || matches!(err, SpecError::PropertyPossessionNotArenaAdmission { .. }),
        "{err:?}"
    );
}

#[test]
fn generated_properties_reject_forbidden_movement_vocabulary() {
    let enrollment = default_enrollment();
    for property in &enrollment.pack.game_mode.properties {
        let haystack = format!(
            "{} {} {} {}",
            property.id, property.namespace, property.name, property.description
        );
        for forbidden in ["route", "pathfinding", "predecessor", "border", "frontline"] {
            assert!(
                !haystack.contains(forbidden),
                "generated property must not reference `{forbidden}`"
            );
        }
    }
}

fn sample_arena() -> ArenaSpec {
    ArenaSpec {
        name: "sample".into(),
        flow_property: PropertyKey::new("mapgen", "sample_flow"),
        balance_property: None,
        max_participants: 4,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![ExplicitParticipantSpec::flat(0, 1)],
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    }
}
