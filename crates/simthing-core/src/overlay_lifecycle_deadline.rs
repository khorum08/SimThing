//! Generation-denominated overlay lifecycle admission (7.7).

use thiserror::Error;

use crate::{DissolveCondition, GenerationStamp, OverlayLifecycle, RoutedGenerationDuration};

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum OverlayLifecycleAdmitError {
    #[error("DissolveCondition::OverrideReceived is not an admitted lifecycle arm")]
    OverrideReceivedForbidden,
    #[error("automatic overlay lifecycle requires at least one condition")]
    EmptyConditions,
    #[error("overlay lifecycle supports at most 31 conjunctive conditions (got {0})")]
    ConditionBudgetExceeded(usize),
    #[error("deadline_generation overflow: activation {activation} + duration {duration}")]
    DeadlineOverflow { activation: u32, duration: u32 },
    #[error("deadline_generation {0} is not exactly representable by the Phase-5 f32 operand")]
    DeadlineNotExactlyRepresentable(u32),
}

pub fn establish_overlay_deadline(
    activation: GenerationStamp,
    duration: u32,
) -> Result<GenerationStamp, OverlayLifecycleAdmitError> {
    let deadline = activation.get().checked_add(duration).ok_or(
        OverlayLifecycleAdmitError::DeadlineOverflow {
            activation: activation.get(),
            duration,
        },
    )?;
    let comparator_threshold = deadline.saturating_sub(1);
    if deadline as f32 as u32 != deadline
        || comparator_threshold as f32 as u32 != comparator_threshold
    {
        return Err(OverlayLifecycleAdmitError::DeadlineNotExactlyRepresentable(
            deadline,
        ));
    }
    Ok(GenerationStamp::new(deadline))
}

/// Rebase a routed authored duration against the destination's own generation.
pub fn rebase_routed_overlay_duration(
    routed: RoutedGenerationDuration,
    destination_generation: GenerationStamp,
) -> Result<GenerationStamp, OverlayLifecycleAdmitError> {
    let _source_provenance = routed.provenance();
    establish_overlay_deadline(destination_generation, routed.authored_duration())
}

pub fn admit_dissolve_conditions(
    conditions: &[DissolveCondition],
) -> Result<(), OverlayLifecycleAdmitError> {
    if conditions.is_empty() {
        return Err(OverlayLifecycleAdmitError::EmptyConditions);
    }
    if conditions.len() > 31 {
        return Err(OverlayLifecycleAdmitError::ConditionBudgetExceeded(
            conditions.len(),
        ));
    }
    if conditions
        .iter()
        .any(|condition| matches!(condition, DissolveCondition::OverrideReceived))
    {
        return Err(OverlayLifecycleAdmitError::OverrideReceivedForbidden);
    }
    Ok(())
}

pub fn admit_overlay_lifecycle(
    lifecycle: &OverlayLifecycle,
) -> Result<(), OverlayLifecycleAdmitError> {
    match lifecycle {
        OverlayLifecycle::UntilDissolved => Ok(()),
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        }
        | OverlayLifecycle::Transient {
            dissolution_conditions,
        } => admit_dissolve_conditions(dissolution_conditions),
        OverlayLifecycle::Suspended { when_activated } => admit_overlay_lifecycle(when_activated),
    }
}
