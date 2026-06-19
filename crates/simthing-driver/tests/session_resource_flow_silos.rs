//! SESSION-RESOURCE-FLOW-SILOS-0 — driver resource-flow materialization proof.

use std::fs;
use std::path::PathBuf;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, PropertyLayout,
    SimProperty, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_owner_silo_resource_flow_spec, compile_and_materialize_owner_silo_flow,
    compile_and_materialize_owner_silo_flow_via_resource_flow, compile_owner_silo_flow_admission,
};
use simthing_spec::{
    compile_property, compile_resource_flow_admission, deserialize_scenario_authority,
    ingest_scenario_from_str, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec, ScenarioIngestionClassification, ScenarioIngestionProfile,
    SpecError,
};

fn corpus_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scenarios/corpus")
        .join(name)
}

fn load_balanced_flow() -> simthing_spec::SimThingScenarioSpec {
    let json = fs::read_to_string(corpus_path(
        "owner_silo_balanced_flow.simthing-scenario.json",
    ))
    .expect("corpus");
    deserialize_scenario_authority(&json).expect("parse")
}

fn flow_subfield(role_name: &str, accumulator: AccumulatorSpec) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(role_name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: role_name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(accumulator),
    }
}

fn setup_owner_silo_registry() -> DimensionRegistry {
    let mut reg = DimensionRegistry::new();
    let spec = PropertySpec {
        id: "owner_silo_flow".into(),
        namespace: "session".into(),
        name: "owner_silo_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![flow_subfield(
            "flow",
            AccumulatorSpec {
                role: AccumulatorRole::IntrinsicFlow,
                log_tier: LogTier::Summary,
            },
        )],
    };
    compile_property(&spec, &mut reg).unwrap();
    reg
}

#[test]
fn owner_silo_flow_compiles_resource_flow_admission() {
    let scenario = load_balanced_flow();
    let reg = setup_owner_silo_registry();
    let (admission, report) =
        compile_owner_silo_flow_admission(&scenario, &reg).expect("compile admission");
    assert_eq!(admission.arenas.len(), 1);
    assert_eq!(admission.arenas[0].name, "owner_silo");
    assert_eq!(report.explicit_participant_count, 2);
    assert!(report.gpu_execution_deferred);
}

#[test]
fn owner_silo_flow_materializes_arena_registry() {
    let scenario = load_balanced_flow();
    let reg = setup_owner_silo_registry();
    let (arena_registry, report) =
        compile_and_materialize_owner_silo_flow(&scenario, &reg).expect("materialize");
    assert_eq!(arena_registry.arenas.len(), 1);
    assert_eq!(arena_registry.participants.len(), 2);
    assert_eq!(report.silo_admission.participant_count, 2);

    let via_rf = compile_and_materialize_owner_silo_flow_via_resource_flow(&scenario, &reg)
        .expect("via resource flow");
    assert_eq!(via_rf.0.participants.len(), 2);
}

#[test]
fn owner_silo_flow_explicit_participants_only() {
    let scenario = load_balanced_flow();
    let reg = setup_owner_silo_registry();
    let flow_spec = build_owner_silo_resource_flow_spec(&scenario).expect("flow spec");
    assert_eq!(flow_spec.arenas.len(), 1);
    assert_eq!(flow_spec.arenas[0].explicit_participants.len(), 2);
    assert!(flow_spec.arenas[0].wildcard_admission.is_none());

    let mut arena = flow_spec.arenas[0].clone();
    arena.explicit_participants.clear();
    let rejected = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
        ..Default::default()
    };
    let err = compile_resource_flow_admission(&rejected, &reg).unwrap_err();
    assert!(matches!(err, SpecError::ImplicitParticipation { .. }));

    let profile = ScenarioIngestionProfile {
        require_canonical_tree: true,
        admit_legacy_world_root: true,
    };
    let json = fs::read_to_string(corpus_path(
        "owner_silo_balanced_flow.simthing-scenario.json",
    ))
    .expect("corpus");
    let (ingestion, _) = ingest_scenario_from_str("balanced", &json, profile);
    assert_ne!(
        ingestion.classification,
        ScenarioIngestionClassification::Rejected
    );
}

#[test]
fn owner_silo_flow_gpu_execution_deferred_without_new_primitive() {
    let scenario = load_balanced_flow();
    let reg = setup_owner_silo_registry();
    let (_, report) =
        compile_and_materialize_owner_silo_flow(&scenario, &reg).expect("materialize");
    assert!(report.gpu_execution_deferred);
    assert!(report.gpu_execution_note.contains("deferred"));

    let flow_spec = build_owner_silo_resource_flow_spec(&scenario).expect("spec");
    assert_eq!(
        flow_spec.arenas[0].flow_property,
        PropertyKey::new("session", "owner_silo_flow")
    );
    assert!(matches!(
        flow_spec.arenas[0].explicit_participants[0],
        ExplicitParticipantSpec { .. }
    ));
    assert_eq!(
        flow_spec.arenas[0].fission_policy,
        FissionPolicySpec::Reevaluate
    );
    assert!(matches!(
        flow_spec.arenas[0],
        ArenaSpec {
            wildcard_admission: None,
            ..
        }
    ));
}
