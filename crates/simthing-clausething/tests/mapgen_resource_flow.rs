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
fn tiny_fixture_generates_bounded_rf_enrollment() {
    let enrollment = default_enrollment();
    assert_eq!(enrollment.expansion_report.arenas.len(), 2);
    assert!(enrollment.pack.game_mode.resource_flow.is_some());
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
        explicit_participants: vec![ExplicitParticipantSpec::flat(0, 1)],
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    }
}
