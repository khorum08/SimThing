//! E-2B — Resource Flow enrollment compile / resolution tests.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::{resolve_resource_flow_enrollment, validate_resource_flow_preflight, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    ArenaSpec, EnrollmentSelectorSpec, ExplicitParticipantSpec, FissionPolicySpec,
    InstallTargetSpec, PropertyKey, ResourceFlowSpec, SpecError,
};
use std::collections::HashMap;

fn food_arena(max_participants: u32) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![],
        enrollment: Some(EnrollmentSelectorSpec::InstallTarget(
            InstallTargetSpec::AllOfKind {
                kind: "Cohort".into(),
            },
        )),
        wildcard_admission: None,
    }
}

fn cohort_scenario(n: usize) -> (Scenario, SimThing, SlotAllocator) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..n {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);
    let scenario = Scenario {
        name: "e2b".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 64,
        registry: simthing_core::DimensionRegistry::new(),
        root: root.clone(),
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: HashMap::new(),
    };
    (scenario, root, alloc)
}

#[test]
fn resource_flow_enrollment_install_target_resolves_all_of_kind() {
    let (scenario, root, alloc) = cohort_scenario(3);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena(16)],
        couplings: vec![],
    };
    let resolved =
        resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).expect("resolve");
    assert_eq!(resolved.arenas[0].explicit_participants.len(), 3);
    for child in &root.children {
        let raw = child.id.raw();
        assert!(
            resolved
                .arenas[0]
                .explicit_participants
                .iter()
                .any(|p| p.subtree_root_id == raw)
        );
    }
}

#[test]
fn resource_flow_enrollment_rejects_empty_resolution() {
    let (scenario, root, alloc) = cohort_scenario(0);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena(16)],
        couplings: vec![],
    };
    let err = resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).unwrap_err();
    assert!(matches!(
        err,
        simthing_driver::EnrollmentError::Spec(SpecError::ImplicitParticipation { .. })
    ));
}

#[test]
fn resource_flow_enrollment_rejects_over_max_participants() {
    let (scenario, root, alloc) = cohort_scenario(3);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena(2)],
        couplings: vec![],
    };
    let err = resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).unwrap_err();
    assert!(matches!(
        err,
        simthing_driver::EnrollmentError::Spec(SpecError::MaxParticipantsExceeded { .. })
    ));
}

#[test]
fn resource_flow_enrollment_rejects_duplicate_hosted_simthing() {
    let (scenario, root, alloc) = cohort_scenario(1);
    let mut spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
            explicit_participants: vec![
                ExplicitParticipantSpec {
                    slot: alloc.slot_of(root.children[0].id).unwrap(),
                    subtree_root_id: root.children[0].id.raw(),
                },
                ExplicitParticipantSpec {
                    slot: alloc.slot_of(root.children[0].id).unwrap(),
                    subtree_root_id: root.children[0].id.raw(),
                },
            ],
            ..food_arena(16)
        }],
        couplings: vec![],
    };
    spec.arenas[0].enrollment = Some(EnrollmentSelectorSpec::ExplicitOnly);
    let err = resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).unwrap_err();
    assert!(matches!(
        err,
        simthing_driver::EnrollmentError::Spec(SpecError::DuplicateEnrollmentHostedSimThing { .. })
    ));
}

#[test]
fn resource_flow_enrollment_resolved_participants_pass_e10r_preflight() {
    let (scenario, root, alloc) = cohort_scenario(2);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena(16)],
        couplings: vec![],
    };
    let resolved =
        resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).expect("resolve");
    assert!(validate_resource_flow_preflight(&resolved, &alloc).is_ok());
}

#[test]
fn resource_flow_enrollment_materializes_arena_participant_scaffold() {
    use simthing_core::DimensionRegistry;
    use simthing_driver::materialize_arena_participants;
    use simthing_spec::compile_property;

    let (scenario, mut root, mut alloc) = cohort_scenario(2);
    let mut reg = DimensionRegistry::new();
    let prop = simthing_spec::PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![],
    };
    let _ = compile_property(&prop, &mut reg);

    let spec = ResourceFlowSpec {
        arenas: vec![food_arena(16)],
        couplings: vec![],
    };
    let resolved =
        resolve_resource_flow_enrollment(&spec, &scenario, &root, &alloc).expect("resolve");
    validate_resource_flow_preflight(&resolved, &alloc).unwrap();
    let scaffold =
        materialize_arena_participants(&resolved, &reg, &mut root, &mut alloc).expect("scaffold");
    assert_eq!(scaffold.reports.len(), 1);
    assert_eq!(scaffold.reports[0].participant_count, 2);
    assert_eq!(scaffold.index.by_host_and_arena.len(), 2);
}

#[test]
fn resource_flow_enrollment_preserves_sibling_contiguity() {
    use simthing_core::DimensionRegistry;
    use simthing_driver::{arena_participant_sibling_slots, materialize_arena_participants, slots_are_contiguous};
    use simthing_spec::compile_property;

    let (scenario, mut root, mut alloc) = cohort_scenario(3);
    let mut reg = DimensionRegistry::new();
    let prop = simthing_spec::PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![],
    };
    let _ = compile_property(&prop, &mut reg);

    let resolved = resolve_resource_flow_enrollment(
        &ResourceFlowSpec {
            arenas: vec![food_arena(16)],
            couplings: vec![],
        },
        &scenario,
        &root,
        &alloc,
    )
    .unwrap();
    let scaffold = materialize_arena_participants(&resolved, &reg, &mut root, &mut alloc).unwrap();
    let arena_root_id = *scaffold.arena_root_ids.get(&0).expect("arena root");
    let siblings = arena_participant_sibling_slots(&root, arena_root_id, &alloc);
    assert_eq!(siblings.len(), 3);
    assert!(slots_are_contiguous(&siblings));
}

#[test]
fn resource_flow_enrollment_no_simthing_sim_arena_imports() {
    let sim_lib = include_str!("../../simthing-sim/src/lib.rs");
    let boundary = include_str!("../../simthing-sim/src/boundary.rs");
    for src in [sim_lib, boundary] {
        assert!(!src.contains("EnrollmentSelectorSpec"));
        assert!(!src.contains("resolve_resource_flow_enrollment"));
        assert!(!src.contains("resource_flow_enrollment"));
    }
}

#[test]
fn resource_flow_enrollment_does_not_require_e11b_nested_gpu() {
    let (scenario, root, alloc) = cohort_scenario(2);
    let resolved = resolve_resource_flow_enrollment(
        &ResourceFlowSpec {
            arenas: vec![food_arena(16)],
            couplings: vec![],
        },
        &scenario,
        &root,
        &alloc,
    )
    .unwrap();
    assert_eq!(resolved.arenas[0].explicit_participants.len(), 2);
    assert!(resolved.arenas[0].enrollment.is_some());
}
