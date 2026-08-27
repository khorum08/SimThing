//! **Derive** — specialist, offering, and sealed Draw declarations.
//!
//! The graduated offering vector is closed: vendors cannot add per-type
//! deltas or shadow tiers through this facade.
//!
//! ```compile_fail,E0560
//! use simthing_embedder::derive::OfferingPriceVectorSpec;
//!
//! let _ = OfferingPriceVectorSpec {
//!     unit_cost: 1.0,
//!     default_clearing_weight: 1.0,
//!     per_type_delta: 0.5,
//! };
//! ```
//!
//! ```compile_fail,E0560
//! use simthing_embedder::derive::OfferingPriceVectorSpec;
//!
//! let _ = OfferingPriceVectorSpec {
//!     unit_cost: 1.0,
//!     default_clearing_weight: 1.0,
//!     shadow_tier: 2,
//! };
//! ```

pub use simthing_core::{
    AuthoredOwnerRefError, DeclaredSpecialization, IntegrationSchedule, KindIdentity, OwnerRef,
    OwnerResolutionError, OwnerSpecializationRow, SpecializationError, SpecializationObservations,
    SpecializationProfile, SpecializationReport, SpecializationRequirement, PROFILE_OWNER_SEAT,
    PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
pub use simthing_driver::ComparativeEmitterClass;
pub use simthing_spec::{
    admit_specialization_flow_market, compile_eml_gadget_stack, resolve_effective_clearing_weights,
    AdmittedSpecializationFlowMarket, ClearingWeightOverrideSpec, ClearingWeightResolutionError,
    CompiledEmlGadgetStack, ConservedOfferingSpec, DrawAuthorizationError,
    DrawEnvelopeTemplateSpec, EmlGadgetCompileOptions, EmlGadgetInstanceSpec, EmlGadgetStackSpec,
    FlowMarketAdmissionError, GrantLifecycleError, MarketGrantKey, MarketGrantRecord,
    MarketGrantResidencyProvenance, OfferingPriceVectorSpec, OfferingQuantizationError,
    SpecializationFlowMarketSpec,
};

/// Author one validated Owner seat using the canonical scenario metadata shape.
pub fn owner_seat(
    owner_id: impl Into<String>,
    display_name: &str,
    archetype: &str,
) -> Result<simthing_core::SimThing, AuthoredOwnerRefError> {
    let owner_id = owner_id.into();
    let admitted = OwnerRef::try_new_authored(owner_id)?;
    Ok(simthing_spec::make_owner_entity(
        admitted.as_str(),
        display_name,
        archetype,
    ))
}

/// Derive/validate authored specializations through the core protocol.
pub fn specializations(
    root: &simthing_core::SimThing,
    profiles: &[SpecializationProfile],
    observations: &SpecializationObservations,
) -> Result<SpecializationReport, SpecializationError> {
    simthing_core::derive_specializations(root, profiles, observations)
}

/// The kind-free owner × specialization query.
pub fn owner_specializations(
    root: &simthing_core::SimThing,
    report: &SpecializationReport,
) -> Result<Vec<OwnerSpecializationRow>, OwnerResolutionError> {
    simthing_core::query_owner_specializations(root, report)
}

/// Query owner × specialization from the running session's installed report.
pub fn installed_owner_specializations(
    session: &simthing_driver::SimSession,
) -> Result<Vec<OwnerSpecializationRow>, OwnerResolutionError> {
    session.owner_specializations()
}

/// Reserved neutral ownership, surfaced as a real owner rather than absence.
pub fn reserved_unowned() -> OwnerRef {
    simthing_core::unowned()
}
