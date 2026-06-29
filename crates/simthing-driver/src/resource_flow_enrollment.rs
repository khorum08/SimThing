//! E-2B — Resource Flow enrollment selector resolution at session install.

use simthing_core::SimThing;
use simthing_gpu::SlotAllocator;
use simthing_spec::{EnrollmentSelectorSpec, ExplicitParticipantSpec, ResourceFlowSpec, SpecError};
use std::collections::HashSet;
use thiserror::Error;

use crate::install::{resolve_install_target, InstallError};
use crate::scenario::Scenario;

#[derive(Debug, Error)]
pub enum EnrollmentError {
    #[error(transparent)]
    Install(#[from] InstallError),
    #[error(transparent)]
    Spec(#[from] SpecError),
}

impl From<EnrollmentError> for InstallError {
    fn from(err: EnrollmentError) -> Self {
        match err {
            EnrollmentError::Install(e) => e,
            EnrollmentError::Spec(e) => InstallError::Spec(e),
        }
    }
}

/// Resolve authored enrollment selectors into live `explicit_participants`.
///
/// `ExplicitOnly` (or omitted `enrollment`) preserves authored explicit rows.
/// `InstallTarget` replaces explicit rows with live session resolution.
pub fn resolve_resource_flow_enrollment(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<ResourceFlowSpec, EnrollmentError> {
    let mut resolved = spec.clone();
    for arena in &mut resolved.arenas {
        match arena
            .enrollment
            .as_ref()
            .unwrap_or(&EnrollmentSelectorSpec::ExplicitOnly)
        {
            EnrollmentSelectorSpec::ExplicitOnly => {}
            EnrollmentSelectorSpec::InstallTarget(target) => {
                arena.explicit_participants =
                    resolve_install_target_to_explicit(arena, target, scenario, root, allocator)?;
            }
        }
        validate_resolved_arena_admission(arena)?;
    }
    Ok(resolved)
}

fn resolve_install_target_to_explicit(
    arena: &simthing_spec::ArenaSpec,
    target: &simthing_spec::InstallTargetSpec,
    scenario: &Scenario,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<Vec<ExplicitParticipantSpec>, EnrollmentError> {
    let hosted_ids = resolve_install_target(target, scenario, root)?;
    let mut explicit = Vec::with_capacity(hosted_ids.len());
    for id in hosted_ids {
        let raw = id.raw();
        let slot = allocator
            .slot_of(id)
            .ok_or(SpecError::UnknownExplicitParticipantSimThing {
                arena: arena.name.clone(),
                subtree_root_id: raw,
            })?;
        explicit.push(ExplicitParticipantSpec::flat(slot.raw(), raw));
    }
    Ok(explicit)
}

fn validate_resolved_arena_admission(
    arena: &simthing_spec::ArenaSpec,
) -> Result<(), EnrollmentError> {
    let mut seen = HashSet::new();
    for participant in &arena.explicit_participants {
        if !seen.insert(participant.subtree_root_id) {
            return Err(SpecError::DuplicateEnrollmentHostedSimThing {
                arena: arena.name.clone(),
                subtree_root_id: participant.subtree_root_id,
            }
            .into());
        }
    }

    if arena.explicit_participants.is_empty() && arena.wildcard_admission.is_none() {
        return Err(SpecError::ImplicitParticipation {
            arena: arena.name.clone(),
        }
        .into());
    }

    let computed = arena.explicit_participants.len() as u32;
    if computed > arena.max_participants {
        return Err(SpecError::MaxParticipantsExceeded {
            arena: arena.name.clone(),
            declared: arena.max_participants,
            computed,
        }
        .into());
    }

    Ok(())
}
