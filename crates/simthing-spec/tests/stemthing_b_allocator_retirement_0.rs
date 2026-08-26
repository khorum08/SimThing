use std::collections::BTreeMap;

use simthing_spec::{
    plan_mobility_reenroll0, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move, MobilityReenroll0PlanInput,
    MobilityReenroll0RegistryState,
};

fn parent(key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey {
        parent_id: 19,
        key_id,
    }
}

fn input(moves: Vec<MobilityReenroll0Move>) -> MobilityReenroll0PlanInput {
    MobilityReenroll0PlanInput {
        registry: MobilityReenroll0RegistryState {
            live_slices: vec![
                MobilityAlloc0LiveSlice {
                    entity_id: 301,
                    parent_key: parent(1),
                    slot: 41,
                },
                MobilityAlloc0LiveSlice {
                    entity_id: 102,
                    parent_key: parent(1),
                    slot: 7,
                },
                MobilityAlloc0LiveSlice {
                    entity_id: 990,
                    parent_key: parent(1),
                    slot: 88,
                },
            ],
            origin_generations: BTreeMap::from([(parent(1), 5)]),
            destination_generations: BTreeMap::from([(parent(2), 9)]),
        },
        moves,
        forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
    }
}

fn moves() -> Vec<MobilityReenroll0Move> {
    [301, 102, 990]
        .into_iter()
        .map(|entity_id| MobilityReenroll0Move {
            entity_id,
            origin: parent(1),
            destination: parent(2),
        })
        .collect()
}

#[test]
fn reparenting_preserves_every_resident_slot_without_recipient_or_free_list_order() {
    let forward = plan_mobility_reenroll0(&input(moves()));
    let mut reverse_moves = moves();
    reverse_moves.reverse();
    let reverse = plan_mobility_reenroll0(&input(reverse_moves));

    assert!(forward.admitted, "{:?}", forward.diagnostics);
    assert_eq!(forward, reverse, "caller order must carry no authority");
    assert_eq!(forward.committed_moves.len(), 3);
    assert_eq!(
        forward
            .committed_moves
            .iter()
            .map(|movement| (movement.entity_id, movement.destination_slot))
            .collect::<Vec<_>>(),
        vec![(102, 7), (301, 41), (990, 88)],
        "re-enrollment must carry each pre-existing stable slot exactly"
    );
    assert!(forward
        .final_live_slices
        .iter()
        .all(|slice| slice.parent_key == parent(2)));
}

#[test]
fn duplicate_live_logical_slot_is_a_typed_admission_red() {
    let mut malformed = input(Vec::new());
    malformed.registry.live_slices[1].slot = 41;

    let report = plan_mobility_reenroll0(&malformed);

    assert!(!report.admitted);
    assert_eq!(report.diagnostics, vec!["duplicate live logical slot"]);
    assert!(report.committed_moves.is_empty());
    assert_eq!(report.final_live_slices, malformed.registry.live_slices);
}
