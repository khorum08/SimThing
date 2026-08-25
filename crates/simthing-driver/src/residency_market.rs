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
    let offering_id = &grant.key().offering_id;
    let resource = market
        .offering_resource(offering_id)
        .ok_or_else(|| ResidencyMarketBridgeError::UnknownOffering(offering_id.clone()))?;
    if resource != &grant.scope().resource_key {
        return Err(ResidencyMarketBridgeError::OfferingResourceMismatch(
            offering_id.clone(),
        ));
    }
    ProvisionalResidencyEntitlement::try_new(
        grant.key().granter,
        grant.key().grantee,
        market_grant_stable_key(market, grant),
        grant.quantity(),
        grant.granted_generation(),
    )
    .map_err(Into::into)
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

fn market_grant_stable_key(
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for component in [
        grant.key().granter.raw().to_le_bytes().as_slice(),
        grant.key().grantee.raw().to_le_bytes().as_slice(),
        market.specialization_profile_id().as_bytes(),
        grant.key().offering_id.as_bytes(),
        grant.scope().owner_ref.as_str().as_bytes(),
        grant.scope().resource_key.as_str().as_bytes(),
        grant.scope().scope_id.as_str().as_bytes(),
    ] {
        hash ^= component.len() as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in component {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}
