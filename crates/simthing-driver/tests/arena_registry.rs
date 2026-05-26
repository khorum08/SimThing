//! E-9 — ArenaRegistry driver/spec integration tests.

use simthing_core::SimPropertyId;
use simthing_driver::{
    ArenaCoupling, ArenaRegistryBuilder, ArenaRegistryError, CouplingDelay, FissionPolicy,
    GpuArenaDescriptor, SpecSessionState,
};

fn food_arena(max_participants: u32) -> GpuArenaDescriptor {
    GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: SimPropertyId(0),
        balance_property_id: Some(SimPropertyId(1)),
        max_participants,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicy::default(),
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    }
}

#[test]
fn spec_session_state_carries_empty_arena_registry_by_default() {
    let state = SpecSessionState::new();
    assert!(state.arena_registry.arenas.is_empty());
    assert_eq!(state.arena_registry.generation, 0);
}

#[test]
fn arena_registry_three_arena_fixture_builds() {
    let mut b = ArenaRegistryBuilder::new();
    let food = b.push_arena(food_arena(8));
    let research = b.push_arena(GpuArenaDescriptor {
        name: "research".into(),
        flow_property_id: SimPropertyId(2),
        balance_property_id: None,
        max_participants: 8,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicy::Reevaluate,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    });
    let suppression = b.push_arena(GpuArenaDescriptor {
        name: "suppression".into(),
        flow_property_id: SimPropertyId(3),
        balance_property_id: None,
        max_participants: 8,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    });
    b.admit_participant(food, 0, simthing_core::SimThingId::new())
        .unwrap();
    b.admit_participant(research, 1, simthing_core::SimThingId::new())
        .unwrap();
    b.declare_wildcard_admission(suppression, Some(4)).unwrap();
    b.push_coupling(ArenaCoupling {
        from_arena: food,
        to_arena: research,
        delay: CouplingDelay::OneTickDelay,
    })
    .unwrap();
    b.push_coupling(ArenaCoupling {
        from_arena: research,
        to_arena: suppression,
        delay: CouplingDelay::BoundaryStage { stage: 1 },
    })
    .unwrap();
    let (reg, report) = b.build().unwrap();
    assert_eq!(report.arena_count, 3);
    assert_eq!(report.coupling_count, 2);
    assert_eq!(reg.arenas[2].fission_policy, FissionPolicy::Reject);
}

#[test]
fn arena_registry_orderband_depth_cap_enforced() {
    let mut b = ArenaRegistryBuilder::new();
    let food = b.push_arena(food_arena(2));
    b.admit_participant(food, 0, simthing_core::SimThingId::new())
        .unwrap();
    b.reserve_orderband_depth(food, 16).unwrap();
    let err = b.build().unwrap_err();
    assert!(matches!(
        err,
        ArenaRegistryError::MaxOrderBandDepthExceeded { .. }
    ));
}
