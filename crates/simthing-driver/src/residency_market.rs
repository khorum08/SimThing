//! Driver bridge from a graduated flow-market grant to kernel-owned physical placement.
//!
//! The bridge validates provenance and converts quantity into a provisional entitlement.
//! It does not clear, rank, retry, choose geometry, or own committed placement state.

use simthing_core::{AnchoredLocusMap, BindingTableSnapshot, GenerationStamp, IntegrationSchedule};
use simthing_gpu::{
    ProvisionalResidencyEntitlement, ResidencyEntitlementError, ResidencyExtent,
    ResidencyPlacementError, ResidencyPlacementOutcome, ResidencyRelocationOutcome, SlotAllocator,
};
use simthing_spec::{AdmittedSpecializationFlowMarket, MarketGrantRecord};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidencyMarketBridgeError {
    #[error("market grant names unknown admitted offering `{0}`")]
    UnknownOffering(String),
    #[error("market grant resource does not match admitted offering `{0}`")]
    OfferingResourceMismatch(String),
    #[error("market grant provenance projection failed: {0}")]
    Provenance(String),
    #[error(transparent)]
    Entitlement(#[from] ResidencyEntitlementError),
    #[error(transparent)]
    Placement(#[from] ResidencyPlacementError),
}

/// Convert one graduated market grant into the provisional physical product.
/// The stable key intentionally excludes generation and quantity so renewal and
/// relocation preserve the same market relationship identity.
pub fn provisional_residency_from_market_grant(
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
) -> Result<ProvisionalResidencyEntitlement, ResidencyMarketBridgeError> {
    provisional_residency_and_provenance_from_market_grant(market, grant)
        .map(|(entitlement, _)| entitlement)
}

pub(crate) fn provisional_residency_and_provenance_from_market_grant(
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
) -> Result<
    (
        ProvisionalResidencyEntitlement,
        simthing_spec::MarketGrantResidencyProvenance,
    ),
    ResidencyMarketBridgeError,
> {
    let offering_id = &grant.key().offering_id;
    let resource = market
        .offering_resource(offering_id)
        .ok_or_else(|| ResidencyMarketBridgeError::UnknownOffering(offering_id.clone()))?;
    if resource != &grant.scope().resource_key {
        return Err(ResidencyMarketBridgeError::OfferingResourceMismatch(
            offering_id.clone(),
        ));
    }
    let provenance = market
        .residency_provenance(grant)
        .map_err(|error| ResidencyMarketBridgeError::Provenance(error.to_string()))?;
    let entitlement = ProvisionalResidencyEntitlement::try_new(
        provenance.granter(),
        provenance.grantee(),
        provenance.stable_key(),
        provenance.quantity(),
        provenance.granted_generation(),
    )?;
    Ok((entitlement, provenance))
}

pub fn realize_market_grant_residency(
    allocator: &mut SlotAllocator,
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
    proposed: ResidencyExtent,
    generation: GenerationStamp,
    schedule: &mut IntegrationSchedule,
) -> Result<ResidencyPlacementOutcome, ResidencyMarketBridgeError> {
    let entitlement = provisional_residency_from_market_grant(market, grant)?;
    allocator
        .realize_provisional_residency(entitlement, proposed, generation, schedule)
        .map_err(Into::into)
}

#[allow(clippy::too_many_arguments)]
pub fn relocate_market_grant_residency(
    allocator: &mut SlotAllocator,
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
    proposed: ResidencyExtent,
    generation: GenerationStamp,
    assignment: &BindingTableSnapshot,
    pre_loci: &AnchoredLocusMap,
    post_loci: &AnchoredLocusMap,
    schedule: &mut IntegrationSchedule,
) -> Result<ResidencyRelocationOutcome, ResidencyMarketBridgeError> {
    let entitlement = provisional_residency_from_market_grant(market, grant)?;
    allocator
        .relocate_provisional_residency(
            entitlement,
            proposed,
            generation,
            assignment,
            pre_loci,
            post_loci,
            schedule,
        )
        .map_err(Into::into)
}
