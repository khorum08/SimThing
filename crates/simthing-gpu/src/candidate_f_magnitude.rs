//! Non-authoritative candidate-F magnitude GPU utility (re-exported from kernel).
//!
//! `write_max_candidate_f_magnitude_bits` is kernel-internal only (KERNEL-CANDIDATE-F-INCRATE-0).
//! External crates use `AccumulatorOpSession::apply_candidate_f_exact_magnitude`.

pub use simthing_kernel::{
    max_candidate_f_magnitude_bits, CandidateFMagnitudeError, CandidateFMagnitudeReport,
    CandidateFMagnitudeRequest, GradientPairGpu,
};
