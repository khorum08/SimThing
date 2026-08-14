//! Deadline Authority Law + Routed Lifecycle Epoch Law (7.7).
//!
//! Fixed-duration overlay lifecycle is `deadline_generation = g_activation + duration`
//! in the **owning tree's** generation authority. Compare, never decrement.
//! Routed durations transport authored duration + provenance, never a foreign
//! absolute deadline.

use thiserror::Error;

use crate::generation_stamp::GenerationStamp;
use crate::overlay::{DissolveCondition, OverlayLifecycle};

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum OverlayLifecycleAdmitError {
    #[error("DissolveCondition::OverrideReceived is rejected at admission; override-replacement is not an admitted dissolve arm")]
    OverrideReceivedForbidden,
    #[error("deadline_generation overflow: activation {activation} + duration {duration}")]
    DeadlineOverflow { activation: u32, duration: u32 },
}

/// Authored duration plus the origin generation stamp. Never an absolute deadline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoutedDuration {
    pub duration: u32,
    pub provenance: GenerationStamp,
}

/// Fail-closed deadline in the destination/owning tree generation authority.
pub fn establish_deadline(
    activation: GenerationStamp,
    duration: u32,
) -> Result<GenerationStamp, OverlayLifecycleAdmitError> {
    activation
        .get()
        .checked_add(duration)
        .map(GenerationStamp::new)
        .ok_or(OverlayLifecycleAdmitError::DeadlineOverflow {
            activation: activation.get(),
            duration,
        })
}

pub fn deadline_reached(now: GenerationStamp, deadline: GenerationStamp) -> bool {
    now.get() >= deadline.get()
}

/// Destination residency: duration + dest generation, never a foreign absolute.
pub fn rebase_routed_duration_at_destination(
    routed: RoutedDuration,
    destination_generation: GenerationStamp,
) -> Result<GenerationStamp, OverlayLifecycleAdmitError> {
    let _ = routed.provenance;
    establish_deadline(destination_generation, routed.duration)
}

pub fn authored_after_ticks_duration(lifecycle: &OverlayLifecycle) -> Option<u32> {
    let conds = match lifecycle {
        OverlayLifecycle::Transient {
            dissolution_conditions,
        }
        | OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions,
        } => dissolution_conditions,
        OverlayLifecycle::Suspended { when_activated } => {
            return authored_after_ticks_duration(when_activated)
        }
        OverlayLifecycle::UntilDissolved => return None,
    };
    conds.iter().find_map(|c| match c {
        DissolveCondition::AfterTicks { remaining } => Some(*remaining),
        _ => None,
    })
}

pub fn admit_dissolve_conditions(
    conditions: &[DissolveCondition],
) -> Result<(), OverlayLifecycleAdmitError> {
    if conditions
        .iter()
        .any(|c| matches!(c, DissolveCondition::OverrideReceived))
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

/// Logical-identity overlay lifecycle binding. Physical slot exists only at upload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverlayLifecycleBinding {
    pub overlay_id: crate::ids::OverlayId,
    pub host: crate::ids::SimThingId,
    pub property_id: crate::ids::SimPropertyId,
    pub deadline: Option<GenerationStamp>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_received_is_rejected_at_admission() {
        let err = admit_dissolve_conditions(&[DissolveCondition::OverrideReceived]).unwrap_err();
        assert_eq!(err, OverlayLifecycleAdmitError::OverrideReceivedForbidden);
    }

    #[test]
    fn deadline_overflow_fails_closed() {
        let err = establish_deadline(GenerationStamp::new(u32::MAX), 1).unwrap_err();
        assert!(matches!(
            err,
            OverlayLifecycleAdmitError::DeadlineOverflow { .. }
        ));
    }

    #[test]
    fn routed_duration_rebases_on_destination_generation() {
        let routed = RoutedDuration {
            duration: 3,
            provenance: GenerationStamp::new(10),
        };
        let dest = rebase_routed_duration_at_destination(routed, GenerationStamp::new(100)).unwrap();
        assert_eq!(dest, GenerationStamp::new(103));
        assert_ne!(dest, GenerationStamp::new(13));
    }
}
