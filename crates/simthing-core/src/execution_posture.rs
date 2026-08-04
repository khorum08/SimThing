//! CONTINUOUS-POSTURE-SOAK-0 — scheduling posture over ONE kernel.
//!
//! Two postures, never two kernels:
//! - **Paced** (default): the front end schedules each generation barrier.
//! - **Continuous**: free-running batched generations; the CPU side is a
//!   submission pump + observer over the same tick/boundary machinery.
//!
//! Posture is a scheduling policy only. It does not fork semantics, mint a
//! second resolution model, or change what a generation is.

use serde::{Deserialize, Serialize};

/// Scheduling policy for advancing generations over the single kernel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPosture {
    /// Front-end-scheduled generation barriers (game model; default).
    #[default]
    Paced,
    /// Free-running batched generations over the same kernel.
    Continuous {
        /// How many generation barriers one continuous batch advances.
        /// Must be ≥ 1 at admission; zero is rejected, never silently paced.
        batch_generations: u32,
    },
}

impl ExecutionPosture {
    /// Admit a continuous batch size. Zero is an admission error shape (caller REDs).
    pub const fn continuous(batch_generations: u32) -> Self {
        Self::Continuous { batch_generations }
    }

    pub const fn is_paced(self) -> bool {
        matches!(self, Self::Paced)
    }

    pub const fn is_continuous(self) -> bool {
        matches!(self, Self::Continuous { .. })
    }

    /// Generations one scheduling call advances. Paced advances exactly one.
    pub const fn generations_per_schedule(self) -> u32 {
        match self {
            Self::Paced => 1,
            Self::Continuous { batch_generations } => batch_generations,
        }
    }
}
