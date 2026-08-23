//! **Derive** — specialist and owner-seat data plus the sanctioned query.

pub use simthing_core::{
    AuthoredOwnerRefError, DeclaredSpecialization, KindIdentity, OwnerRef, OwnerResolutionError,
    OwnerSpecializationRow, SpecializationError, SpecializationObservations, SpecializationProfile,
    SpecializationReport, SpecializationRequirement, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT,
    PROFILE_SPATIAL,
};
pub use simthing_driver::ComparativeEmitterClass;
pub use simthing_spec::{
    compile_eml_gadget_stack, CompiledEmlGadgetStack, EmlGadgetCompileOptions,
    EmlGadgetInstanceSpec, EmlGadgetStackSpec,
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
