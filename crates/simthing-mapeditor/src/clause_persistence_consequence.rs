//! SimThing Studio's one ClauseScript persistence-consequence authoring door.

use simthing_clausething::{
    compile_persistence_consequence_script_value, raw::RawProperty, HydrateError,
};
use simthing_core::GenerationStamp;
use simthing_driver::{
    submit_authored_persistence_consequence, FeederSender, PersistenceConsequenceIngressError,
};
use simthing_spec::{
    PersistenceConsequence, PersistenceOverlayBinding, UnresolvedDemandObservation,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClausePersistenceConsequenceError {
    #[error(transparent)]
    Hydrate(#[from] HydrateError),
    #[error(transparent)]
    Ingress(#[from] PersistenceConsequenceIngressError),
}

/// Lower one ordinary ClauseScript `script_value` modifier chain and submit
/// its funded consequence for an already-cleared unresolved-U observation.
/// This application door cannot create or return a demand product.
pub fn submit_clause_persistence_consequence_script_value(
    property: &RawProperty,
    unit_cost: f32,
    observation: &UnresolvedDemandObservation,
    consequence_generation: GenerationStamp,
    binding: &PersistenceOverlayBinding,
    boundary: &FeederSender,
) -> Result<(String, PersistenceConsequence), ClausePersistenceConsequenceError> {
    let (id, valuation) = compile_persistence_consequence_script_value(property, unit_cost)?;
    let consequence = submit_authored_persistence_consequence(
        observation,
        consequence_generation,
        &valuation,
        binding,
        boundary,
    )?;
    Ok((id, consequence))
}
