//! TREE-EXECUTION-AUTHORITY-LIFETIME-0 focused lifetime/identity proof.

use simthing_core::{
    DimensionRegistry, ExecutionIncarnation, GenerationStamp, IntegrationSchedule,
    PersistedTreeExecutionIdentity, RecordedTreeForkIdentity, SimThing, SimThingKind,
    TreeExecutionAuthority, TreeExecutionContextError, TreeGenerationAuthority, TreeRealmId,
};
use simthing_driver::{Scenario, SimSession};

fn tiny_scenario() -> Scenario {
    Scenario::rebellion_demo("same-semantic-input".into(), 1, 1, 1.0, 8)
}

#[test]
fn identical_scenario_double_open_restore_and_semantic_fork_have_explicit_identity_laws() {
    let first = SimSession::open(tiny_scenario()).expect("first fresh execution");
    let second = SimSession::open(tiny_scenario()).expect("second fresh execution");
    assert_ne!(first.tree_realm(), second.tree_realm());

    let durable = first.persisted_execution_identity();
    let restored = SimSession::open_restored(tiny_scenario(), durable).expect("restore execution");
    assert_eq!(restored.tree_realm(), first.tree_realm());
    assert_eq!(
        restored.execution_incarnation(),
        first.execution_incarnation().next().unwrap()
    );

    let fork_identity = RecordedTreeForkIdentity::new(0x15_07).unwrap();
    let forked = SimSession::open_semantic_fork(tiny_scenario(), durable, fork_identity)
        .expect("semantic fork execution");
    assert_ne!(forked.tree_realm(), first.tree_realm());
    assert_eq!(
        forked.execution_incarnation(),
        ExecutionIncarnation::new(1).unwrap()
    );
}

#[test]
fn persisted_execution_identity_is_validated_data_not_scenario_fingerprint() {
    let realm = TreeRealmId::from_u128(0x15_0700).unwrap();
    let identity =
        PersistedTreeExecutionIdentity::new(realm, ExecutionIncarnation::new(4).unwrap());
    assert_eq!(identity.realm().unwrap(), realm);
    assert_eq!(identity.incarnation().unwrap().get(), 4);
    let wire = serde_json::to_string(&identity).unwrap();
    let round_trip: PersistedTreeExecutionIdentity = serde_json::from_str(&wire).unwrap();
    assert_eq!(round_trip, identity);
    assert!(wire.contains("realm_bytes"));
    assert!(wire.contains("incarnation"));
    assert_eq!(GenerationStamp::new(9).get(), 9);
}

#[test]
fn whole_generation_permit_rejects_wrong_reused_foreign_and_stale_authority() {
    let tree = SimThing::new(SimThingKind::GameSession, 9);
    let generation = TreeGenerationAuthority::new(GenerationStamp::new(9));
    let schedule = IntegrationSchedule::new();
    let registry = DimensionRegistry::new();
    let residency = ();
    let authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(0x15_0701).unwrap(),
        ExecutionIncarnation::new(1).unwrap(),
        &tree,
        &generation,
        &schedule,
        &registry,
        &residency,
    )
    .unwrap();
    let lease = authority.seal_lease().unwrap();
    let verifier = lease.verifier();
    let mut permit = lease.begin_generation(GenerationStamp::new(9)).unwrap();
    verifier
        .validate_generation(&permit, GenerationStamp::new(9))
        .unwrap();
    verifier
        .validate_generation(&permit, GenerationStamp::new(9))
        .unwrap();
    assert!(matches!(
        verifier.validate_generation(&permit, GenerationStamp::new(10)),
        Err(TreeExecutionContextError::PermitGenerationMismatch { .. })
    ));
    lease
        .finish_generation(&mut permit, GenerationStamp::new(10))
        .unwrap();
    assert!(matches!(
        verifier.validate_generation(&permit, GenerationStamp::new(9)),
        Err(TreeExecutionContextError::GenerationPermitAlreadyConsumed { .. })
    ));

    let stale_permit = lease.begin_generation(GenerationStamp::new(10)).unwrap();
    let migrated = lease
        .migrate(ExecutionIncarnation::new(2).unwrap())
        .unwrap();
    assert!(matches!(
        migrated
            .verifier()
            .validate_generation(&stale_permit, GenerationStamp::new(10)),
        Err(TreeExecutionContextError::StaleIncarnation { .. })
    ));

    let other_tree = SimThing::new(SimThingKind::GameSession, 10);
    let other_generation = TreeGenerationAuthority::new(GenerationStamp::new(10));
    let other_schedule = IntegrationSchedule::new();
    let other_registry = DimensionRegistry::new();
    let other_residency = ();
    let other_authority = TreeExecutionAuthority::seal(
        TreeRealmId::from_u128(0x15_0701).unwrap(),
        ExecutionIncarnation::new(2).unwrap(),
        &other_tree,
        &other_generation,
        &other_schedule,
        &other_registry,
        &other_residency,
    )
    .unwrap();
    let other_lease = other_authority.seal_lease().unwrap();
    assert_eq!(
        other_lease
            .verifier()
            .validate_generation(&stale_permit, GenerationStamp::new(10)),
        Err(TreeExecutionContextError::AuthorityCapsuleMismatch)
    );
}

#[test]
fn production_provenance_has_no_runtime_compiler_or_filesystem_discovery() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let runtime =
        std::fs::read_to_string(root.join("crates/simthing-gpu/src/resident_clearing_runtime.rs"))
            .unwrap();
    for forbidden in ["Command::new", "rustc -Vv", "std::fs::", "read_to_string("] {
        assert!(!runtime.contains(forbidden), "runtime contains {forbidden}");
    }
    let build = std::fs::read_to_string(root.join("crates/simthing-gpu/build.rs")).unwrap();
    assert!(build.contains("Command::new"));
    assert!(build.contains("resident_clearing_build_provenance.rs"));
    assert!(build.contains("child_share_eml.rs"));
    assert!(build.contains("resident_recursive_intake_transform.wgsl"));
    assert!(build.contains("arena_allocation_plan.rs"));
    assert!(build.contains("resident_clearing_plan.rs"));
}
