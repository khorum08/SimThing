//! E-10R — Resource Flow explicit participant preflight (driver/session layer).
//!
//! Validates live session identity for authored explicit participants without
//! making `simthing-spec` depend on `simthing-driver`.

use simthing_core::SimThingId;
use simthing_core::SlotIndex;
use simthing_gpu::SlotAllocator;
use simthing_spec::{ResourceFlowSpec, SpecError};

/// Validate explicit participant identity against live session state.
pub fn validate_resource_flow_preflight(
    spec: &ResourceFlowSpec,
    allocator: &SlotAllocator,
) -> Result<(), SpecError> {
    for arena in &spec.arenas {
        for participant in &arena.explicit_participants {
            let id = SimThingId::from_session_raw(participant.subtree_root_id);
            match allocator.slot_of(id) {
                Some(actual_slot) => {
                    if actual_slot.raw() != participant.slot {
                        return Err(SpecError::ExplicitParticipantSlotMismatch {
                            arena: arena.name.clone(),
                            subtree_root_id: participant.subtree_root_id,
                            declared_slot: participant.slot,
                            actual_slot: actual_slot.raw(),
                        });
                    }
                    if !allocator.is_live(actual_slot) {
                        return Err(SpecError::ExplicitParticipantTombstoned {
                            arena: arena.name.clone(),
                            subtree_root_id: participant.subtree_root_id,
                            slot: participant.slot,
                        });
                    }
                }
                None => {
                    if participant.slot < allocator.capacity() as u32
                        && !allocator.is_live(SlotIndex::new(participant.slot))
                    {
                        return Err(SpecError::ExplicitParticipantTombstoned {
                            arena: arena.name.clone(),
                            subtree_root_id: participant.subtree_root_id,
                            slot: participant.slot,
                        });
                    }
                    return Err(SpecError::UnknownExplicitParticipantSimThing {
                        arena: arena.name.clone(),
                        subtree_root_id: participant.subtree_root_id,
                    });
                }
            }
        }
    }
    Ok(())
}
