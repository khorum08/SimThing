//! CONTINUOUS-POSTURE-SOAK-0 — scheduling posture over ONE kernel.
//!
//! Two postures, never two kernels:
//! - **Paced** (default): the front end schedules each generation barrier.
//! - **Continuous**: free-running batched generations; the CPU side is a
//!   submission pump + observer over the same tick/boundary machinery.
//!
//! Posture is a scheduling policy only. It does not fork semantics, mint a
//! second resolution model, or change what a generation is.
//!
//! Continuous `batch_generations` must be ≥ 1 at admission. Zero fails closed —
//! never a silent no-op success.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Scheduling policy for advancing generations over the single kernel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPosture {
    /// Front-end-scheduled generation barriers (game model; default).
    #[default]
    Paced,
    /// Free-running batched generations over the same kernel.
    ///
    /// Construct only through [`ExecutionPosture::admit_continuous`]. A raw
    /// `Continuous { batch_generations: 0 }` value is admission-invalid and is
    /// rejected by [`ExecutionPosture::ensure_admitted`] / session doors.
    Continuous {
        /// How many generation barriers one continuous batch advances.
        /// Must be ≥ 1 at admission; zero is rejected, never silently paced.
        batch_generations: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum ExecutionPostureError {
    #[error(
        "continuous batch_generations must be >= 1; zero is rejected (never a silent no-op success)"
    )]
    ZeroContinuousBatch,
}

impl ExecutionPosture {
    /// Admit a continuous batch size. Zero fails closed.
    pub const fn admit_continuous(batch_generations: u32) -> Result<Self, ExecutionPostureError> {
        if batch_generations == 0 {
            return Err(ExecutionPostureError::ZeroContinuousBatch);
        }
        Ok(Self::Continuous { batch_generations })
    }

    /// Convenience alias for [`Self::admit_continuous`].
    pub const fn continuous(batch_generations: u32) -> Result<Self, ExecutionPostureError> {
        Self::admit_continuous(batch_generations)
    }

    /// Fail closed on an admission-invalid continuous zero batch.
    pub const fn ensure_admitted(self) -> Result<(), ExecutionPostureError> {
        match self {
            Self::Continuous {
                batch_generations: 0,
            } => Err(ExecutionPostureError::ZeroContinuousBatch),
            _ => Ok(()),
        }
    }

    pub const fn is_paced(self) -> bool {
        matches!(self, Self::Paced)
    }

    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::Continuous { .. })
    }

    /// Generations one scheduling call advances. Paced advances exactly one.
    ///
    /// Panics are not used for zero continuous — callers must
    /// [`Self::ensure_admitted`] first; this returns the stored value for
    /// admitted postures only.
    pub const fn generations_per_schedule(self) -> u32 {
        match self {
            Self::Paced => 1,
            Self::Continuous { batch_generations } => batch_generations,
        }
    }
}

#[cfg(test)]
mod admit_proof {
    use super::*;

    #[test]
    fn continuous_zero_batch_fails_closed_at_admit() {
        assert_eq!(
            ExecutionPosture::continuous(0),
            Err(ExecutionPostureError::ZeroContinuousBatch)
        );
        assert_eq!(
            ExecutionPosture::Continuous {
                batch_generations: 0
            }
            .ensure_admitted(),
            Err(ExecutionPostureError::ZeroContinuousBatch)
        );
        let ok = ExecutionPosture::continuous(3).expect("nonzero admits");
        assert!(ok.ensure_admitted().is_ok());
        assert_eq!(ok.generations_per_schedule(), 3);
    }
}
