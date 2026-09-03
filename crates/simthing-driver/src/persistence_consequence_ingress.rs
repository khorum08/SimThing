//! Application ingress for authored consequences of an already-cleared U.
//!
//! This composes the existing valuation, CostBand, and routed-overlay doors.
//! It has no demand output and owns no Current-to-Next or deformation authority.

use simthing_core::GenerationStamp;
use simthing_feeder::FeederSender;
use simthing_spec::{
    fund_unresolved_persistence, AuthoredPersistenceValuation, PersistenceConsequence,
    PersistenceConsequenceError, PersistenceOverlayBinding, UnresolvedDemandObservation,
};
use thiserror::Error;

use crate::{
    CrossingConsequenceAdmissionError, CrossingConsequenceDispatchError, RoutedOverlayDelivery,
};

#[derive(Debug, Error)]
pub enum PersistenceConsequenceIngressError {
    #[error(transparent)]
    Consequence(#[from] PersistenceConsequenceError),
    #[error(transparent)]
    OverlayAdmission(#[from] CrossingConsequenceAdmissionError),
    #[error(transparent)]
    Dispatch(#[from] CrossingConsequenceDispatchError),
}

/// Fund one later-generation consequence from an ordinary unresolved-U
/// observation, then submit its admitted overlay through the existing feeder
/// boundary. A zero CostBand draw is a successful no-overlay consequence.
pub fn submit_authored_persistence_consequence(
    observation: &UnresolvedDemandObservation,
    consequence_generation: GenerationStamp,
    valuation: &AuthoredPersistenceValuation,
    binding: &PersistenceOverlayBinding,
    boundary: &FeederSender,
) -> Result<PersistenceConsequence, PersistenceConsequenceIngressError> {
    let consequence =
        fund_unresolved_persistence(observation, consequence_generation, valuation, binding)?;
    if let Some(overlay) = consequence.overlay.as_ref() {
        RoutedOverlayDelivery::admit(binding.target, overlay.clone())?
            .submit_boundary(consequence_generation, boundary)?;
    }
    Ok(consequence)
}
