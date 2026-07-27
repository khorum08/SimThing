//! E-2B-5 — Policy A inherit-only dynamic fission enrollment for Resource Flow.

use simthing_core::SimThingId;
use simthing_gpu::SlotAllocator;
use simthing_sim::FissionOutcome;

use crate::arena_registry::{ArenaIdx, ArenaRegistry, FissionPolicy};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicFissionEnrollmentAdmission {
    pub parent_id: SimThingId,
    pub child_id: SimThingId,
    pub arena_idx: ArenaIdx,
    pub participant_slot: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicFissionEnrollmentRejection {
    pub parent_id: SimThingId,
    pub child_id: SimThingId,
    pub arena_idx: ArenaIdx,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DynamicFissionEnrollmentReport {
    pub admissions: Vec<DynamicFissionEnrollmentAdmission>,
    pub rejections: Vec<DynamicFissionEnrollmentRejection>,
    pub generation_before: u64,
    pub generation_after: u64,
}

impl DynamicFissionEnrollmentReport {
    pub fn any_admissions(&self) -> bool {
        !self.admissions.is_empty()
    }
}

/// Policy A: fission children inherit membership on the row they already own.
pub fn react_to_fission_resource_flow_enrollment(
    fission: &FissionOutcome,
    arena_registry: &mut ArenaRegistry,
    allocator: &SlotAllocator,
) -> DynamicFissionEnrollmentReport {
    let generation_before = arena_registry.generation;
    let mut report = DynamicFissionEnrollmentReport {
        generation_before,
        ..Default::default()
    };

    if fission.fission_pairs.is_empty() || arena_registry.arenas.is_empty() {
        report.generation_after = generation_before;
        return report;
    }

    let mut admitted_this_batch = false;

    for &(parent_id, child_id) in &fission.fission_pairs {
        let mut parent_arenas: Vec<ArenaIdx> = arena_registry
            .participants
            .iter()
            .filter(|p| p.subtree_root == parent_id)
            .map(|p| p.arena_idx)
            .collect();
        parent_arenas.sort_unstable();
        parent_arenas.dedup();

        for arena_idx in parent_arenas {
            let Some(arena) = arena_registry.arenas.get(arena_idx as usize) else {
                continue;
            };

            if !fission_enrollment_allowed(arena.fission_policy) {
                report.rejections.push(DynamicFissionEnrollmentRejection {
                    parent_id,
                    child_id,
                    arena_idx,
                    reason: format!(
                        "arena `{}` fission policy {:?} rejects dynamic enrollment",
                        arena.name, arena.fission_policy
                    ),
                });
                continue;
            }

            if arena_registry
                .participant_slot(child_id, arena_idx)
                .is_some()
            {
                continue;
            }

            let Some(slot) = allocator.slot_of(child_id) else {
                report.rejections.push(DynamicFissionEnrollmentRejection {
                    parent_id,
                    child_id,
                    arena_idx,
                    reason: "fission child has no owned slot".into(),
                });
                continue;
            };
            match arena_registry.admit_participant_runtime(
                arena_idx,
                slot,
                child_id,
                Some(parent_id),
            ) {
                Ok(()) => {
                    admitted_this_batch = true;
                    report.admissions.push(DynamicFissionEnrollmentAdmission {
                        parent_id,
                        child_id,
                        arena_idx,
                        participant_slot: slot.raw(),
                    });
                }
                Err(err) => {
                    report.rejections.push(DynamicFissionEnrollmentRejection {
                        parent_id,
                        child_id,
                        arena_idx,
                        reason: enrollment_error_reason(&err),
                    });
                }
            }
        }
    }

    if admitted_this_batch {
        arena_registry.bump_generation_after_runtime_admit();
    }
    report.generation_after = arena_registry.generation;
    report
}

/// Authoring/test path for Policy A dynamic fission enrollment.
pub fn react_to_fission_resource_flow_enrollment_on_authoring(
    fission: &FissionOutcome,
    arena_registry: &mut ArenaRegistry,
    allocator: &SlotAllocator,
) -> DynamicFissionEnrollmentReport {
    react_to_fission_resource_flow_enrollment(fission, arena_registry, allocator)
}

fn fission_enrollment_allowed(policy: FissionPolicy) -> bool {
    match policy {
        FissionPolicy::Reject => false,
        // Policy B (`Reevaluate`) deferred — v1 maps to inherit-only admission.
        FissionPolicy::Inherit | FissionPolicy::Reevaluate => true,
    }
}

fn enrollment_error_reason(err: &crate::arena_registry::ArenaRegistryError) -> String {
    err.to_string()
}
