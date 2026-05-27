//! E-10R — Resource Flow explicit participant preflight tests.

use simthing_core::{SimThing, SimThingKind};
use simthing_driver::validate_resource_flow_preflight;
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey, ResourceFlowSpec,
    SpecError,
};

fn food_arena_with_participant(slot: u32, subtree_root_id: u32) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants: 4,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicySpec::Reevaluate,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 4,
        expected_max_children_per_intermediate: 2,
        explicit_participants: vec![ExplicitParticipantSpec {
            slot,
            subtree_root_id,
        }],
        enrollment: None,
        wildcard_admission: None,
    }
}

#[test]
fn e10r_rejects_unknown_subtree_root_id() {
    let mut alloc = SlotAllocator::new();
    let world = SimThing::new(SimThingKind::World, 0);
    alloc.populate_from_tree(&world);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_with_participant(0, 9999)],
        couplings: vec![],
    };
    let err = validate_resource_flow_preflight(&spec, &alloc).unwrap_err();
    assert!(matches!(
        err,
        SpecError::UnknownExplicitParticipantSimThing { .. }
    ));
}

#[test]
fn e10r_rejects_slot_mismatch() {
    let mut alloc = SlotAllocator::new();
    let world = SimThing::new(SimThingKind::World, 0);
    alloc.populate_from_tree(&world);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_with_participant(99, world.id.raw())],
        couplings: vec![],
    };
    let err = validate_resource_flow_preflight(&spec, &alloc).unwrap_err();
    assert!(matches!(
        err,
        SpecError::ExplicitParticipantSlotMismatch { .. }
    ));
}

#[test]
fn e10r_rejects_tombstoned_participant() {
    let mut alloc = SlotAllocator::new();
    let world = SimThing::new(SimThingKind::World, 0);
    let cohort = SimThing::new(SimThingKind::Cohort, 0);
    let mut root = world;
    root.add_child(cohort);
    alloc.populate_from_tree(&root);
    let cohort_id = root.children[0].id;
    let slot = alloc.slot_of(cohort_id).unwrap();
    alloc.tombstone(cohort_id);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_with_participant(slot, cohort_id.raw())],
        couplings: vec![],
    };
    let err = validate_resource_flow_preflight(&spec, &alloc).unwrap_err();
    assert!(matches!(
        err,
        SpecError::ExplicitParticipantTombstoned { .. }
    ));
}

#[test]
fn e10r_accepts_valid_explicit_participant() {
    let mut alloc = SlotAllocator::new();
    let world = SimThing::new(SimThingKind::World, 0);
    alloc.populate_from_tree(&world);
    let spec = ResourceFlowSpec {
        arenas: vec![food_arena_with_participant(0, world.id.raw())],
        couplings: vec![],
    };
    assert!(validate_resource_flow_preflight(&spec, &alloc).is_ok());
}

#[test]
fn e10r_rejects_reserved_gap_smaller_than_expected_fanout() {
    let mut alloc = SlotAllocator::new();
    let world = SimThing::new(SimThingKind::World, 0);
    alloc.populate_from_tree(&world);
    let mut arena = food_arena_with_participant(0, world.id.raw());
    arena.reserved_gap_per_intermediate = 1;
    arena.expected_max_children_per_intermediate = 4;
    let spec = ResourceFlowSpec {
        arenas: vec![arena],
        couplings: vec![],
    };
    let err = validate_resource_flow_preflight(&spec, &alloc).unwrap_err();
    assert!(matches!(err, SpecError::ReservedGapTooSmall { .. }));
}
