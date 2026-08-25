//! StemThing-B VRAM residency placement physics.
//!
//! Market clearing decides entitlement. This module owns only the distinct
//! physical question: whether one provisional entitlement can be realized as
//! a disjoint, in-bounds extent under the owning granter. The oracle is
//! deliberately level-local; it never walks a global extent registry and it
//! never retries or re-clears an entitlement.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use simthing_core::{
    AnchorRemapSection, GenerationStamp, IntegrationSchedule, IntegrationScheduleRowKind,
    SimThingId,
};
use thiserror::Error;

/// Half-open physical residency range `[start, end_exclusive)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResidencyExtent {
    start: u32,
    length: u32,
}

impl ResidencyExtent {
    pub fn try_new(start: u32, length: u32) -> Result<Self, ResidencyExtentError> {
        if length == 0 {
            return Err(ResidencyExtentError::ZeroLength);
        }
        start
            .checked_add(length)
            .ok_or(ResidencyExtentError::EndOverflow { start, length })?;
        Ok(Self { start, length })
    }

    pub const fn start(self) -> u32 {
        self.start
    }

    pub const fn length(self) -> u32 {
        self.length
    }

    pub fn end_exclusive(self) -> u32 {
        self.start + self.length
    }

    pub fn contains(self, other: Self) -> bool {
        self.start <= other.start && other.end_exclusive() <= self.end_exclusive()
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end_exclusive() && other.start < self.end_exclusive()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum ResidencyExtentError {
    #[error("residency extent length must be nonzero")]
    ZeroLength,
    #[error("residency extent end overflows: start={start}, length={length}")]
    EndOverflow { start: u32, length: u32 },
}

/// Stable bridge identity for one already-cleared market relationship.
///
/// This product does not clear or rank anything. Its quantity and generation
/// arrive from the admitted market record at the driver boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProvisionalResidencyEntitlement {
    granter: SimThingId,
    grantee: SimThingId,
    market_grant_key: u64,
    quantity: u32,
    granted_generation: GenerationStamp,
}

impl ProvisionalResidencyEntitlement {
    pub fn try_new(
        granter: SimThingId,
        grantee: SimThingId,
        market_grant_key: u64,
        quantity: u32,
        granted_generation: GenerationStamp,
    ) -> Result<Self, ResidencyEntitlementError> {
        if quantity == 0 {
            return Err(ResidencyEntitlementError::ZeroQuantity);
        }
        Ok(Self {
            granter,
            grantee,
            market_grant_key,
            quantity,
            granted_generation,
        })
    }

    pub const fn granter(self) -> SimThingId {
        self.granter
    }

    pub const fn grantee(self) -> SimThingId {
        self.grantee
    }

    pub const fn market_grant_key(self) -> u64 {
        self.market_grant_key
    }

    pub const fn quantity(self) -> u32 {
        self.quantity
    }

    pub const fn granted_generation(self) -> GenerationStamp {
        self.granted_generation
    }

    pub const fn identity(self) -> ResidencyPlacementIdentity {
        ResidencyPlacementIdentity {
            granter: self.granter,
            grantee: self.grantee,
            market_grant_key: self.market_grant_key,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum ResidencyEntitlementError {
    #[error("zero market entitlement cannot request physical residency")]
    ZeroQuantity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResidencyPlacementIdentity {
    granter: SimThingId,
    grantee: SimThingId,
    market_grant_key: u64,
}

impl ResidencyPlacementIdentity {
    pub const fn granter(self) -> SimThingId {
        self.granter
    }

    pub const fn grantee(self) -> SimThingId {
        self.grantee
    }

    pub const fn market_grant_key(self) -> u64 {
        self.market_grant_key
    }
}

/// Kernel-owned authoritative placement product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedResidencyPlacement {
    identity: ResidencyPlacementIdentity,
    extent: ResidencyExtent,
    quantity: u32,
    committed_generation: GenerationStamp,
}

impl CommittedResidencyPlacement {
    pub const fn identity(self) -> ResidencyPlacementIdentity {
        self.identity
    }

    pub const fn extent(self) -> ResidencyExtent {
        self.extent
    }

    pub const fn quantity(self) -> u32 {
        self.quantity
    }

    pub const fn committed_generation(self) -> GenerationStamp {
        self.committed_generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResidencyPlacementDisposition {
    Committed,
    Relocated,
    /// Exact same grant, quantity, and extent. No schedule row and no global
    /// per-generation re-proof are owed.
    Unchanged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyPlacementOutcome {
    placement: CommittedResidencyPlacement,
    disposition: ResidencyPlacementDisposition,
}

/// A relocation's physical result and the one existing epoch-rebind history section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidencyRelocationOutcome {
    placement: ResidencyPlacementOutcome,
    remap: AnchorRemapSection,
}

impl ResidencyRelocationOutcome {
    pub(crate) fn new(placement: ResidencyPlacementOutcome, remap: AnchorRemapSection) -> Self {
        Self { placement, remap }
    }

    pub const fn placement(&self) -> ResidencyPlacementOutcome {
        self.placement
    }

    pub fn remap(&self) -> &AnchorRemapSection {
        &self.remap
    }
}

impl ResidencyPlacementOutcome {
    pub const fn placement(self) -> CommittedResidencyPlacement {
        self.placement
    }

    pub const fn disposition(self) -> ResidencyPlacementDisposition {
        self.disposition
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResidencyPlacementRefusalReason {
    MissingOwningExtent {
        granter: SimThingId,
    },
    NotDirectChild {
        granter: SimThingId,
        grantee: SimThingId,
        actual_parent: Option<SimThingId>,
    },
    QuantityExtentMismatch {
        entitlement_quantity: u32,
        extent_length: u32,
    },
    OutOfBounds {
        containing: ResidencyExtent,
        proposed: ResidencyExtent,
    },
    Overlap {
        proposed: ResidencyExtent,
        occupied: CommittedResidencyPlacement,
    },
    GrantIdentityConflict {
        existing: ResidencyPlacementIdentity,
        requested: ResidencyPlacementIdentity,
    },
    MissingCommittedPlacementForRelocation {
        requested: ResidencyPlacementIdentity,
    },
}

/// Ordinary physical infeasibility: no authoritative geometry was committed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyPlacementRefusal {
    identity: ResidencyPlacementIdentity,
    proposed: ResidencyExtent,
    reason: ResidencyPlacementRefusalReason,
    retained_unmet_quantity: u32,
    attempted_generation: GenerationStamp,
    revalue_generation: GenerationStamp,
}

impl ResidencyPlacementRefusal {
    pub const fn identity(&self) -> ResidencyPlacementIdentity {
        self.identity
    }

    pub const fn proposed(&self) -> ResidencyExtent {
        self.proposed
    }

    pub fn reason(&self) -> &ResidencyPlacementRefusalReason {
        &self.reason
    }

    pub const fn retained_unmet_quantity(&self) -> u32 {
        self.retained_unmet_quantity
    }

    pub const fn attempted_generation(&self) -> GenerationStamp {
        self.attempted_generation
    }

    pub const fn revalue_generation(&self) -> GenerationStamp {
        self.revalue_generation
    }
}

/// Diagnosable already-committed invariant breach.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommittedResidencyCorruption {
    OutOfBounds {
        boundary_granter: SimThingId,
        placement_granter: SimThingId,
        grantee: SimThingId,
        containing: ResidencyExtent,
        committed: ResidencyExtent,
    },
    Overlap {
        boundary_granter: SimThingId,
        first_granter: SimThingId,
        first_grantee: SimThingId,
        first_extent: ResidencyExtent,
        second_granter: SimThingId,
        second_grantee: SimThingId,
        second_extent: ResidencyExtent,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error(
    "residency session terminated at generation {observed_at_generation:?}: committed placement corruption {corruption:?}"
)]
pub struct ResidencySessionTermination {
    corruption: CommittedResidencyCorruption,
    observed_at_generation: GenerationStamp,
}

impl ResidencySessionTermination {
    pub fn corruption(&self) -> &CommittedResidencyCorruption {
        &self.corruption
    }

    pub const fn observed_at_generation(&self) -> GenerationStamp {
        self.observed_at_generation
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidencyPlacementError {
    #[error("ordinary residency placement refusal: {0:?}")]
    Refused(ResidencyPlacementRefusal),
    #[error(transparent)]
    SessionTerminated(ResidencySessionTermination),
    #[error("residency placement configuration refused: {detail}")]
    Configuration { detail: String },
    #[error("existing epoch-rebind authority refused relocation: {detail}")]
    RemapRefused { detail: String },
}

impl ResidencyPlacementError {
    pub fn refusal(&self) -> Option<&ResidencyPlacementRefusal> {
        match self {
            Self::Refused(refusal) => Some(refusal),
            _ => None,
        }
    }

    pub fn termination(&self) -> Option<&ResidencySessionTermination> {
        match self {
            Self::SessionTerminated(termination) => Some(termination),
            _ => None,
        }
    }
}

/// Stateless level-local physical judge. Only the owning kernel boundary may
/// invoke it; the type carries no allocator, clearing, history, or clock.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ResidencyPlacementOracle;

impl ResidencyPlacementOracle {
    fn judge(
        containing: ResidencyExtent,
        proposed: ResidencyExtent,
        siblings: &[CommittedResidencyPlacement],
    ) -> Result<(), ResidencyPlacementRefusalReason> {
        if !containing.contains(proposed) {
            return Err(ResidencyPlacementRefusalReason::OutOfBounds {
                containing,
                proposed,
            });
        }
        if let Some(occupied) = siblings
            .iter()
            .copied()
            .find(|placement| placement.extent.overlaps(proposed))
        {
            return Err(ResidencyPlacementRefusalReason::Overlap { proposed, occupied });
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PreparedResidencyPlacement {
    placement: CommittedResidencyPlacement,
    disposition: ResidencyPlacementDisposition,
}

impl PreparedResidencyPlacement {
    pub(crate) fn disposition(&self) -> ResidencyPlacementDisposition {
        self.disposition
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ResidencyPlacementBook {
    root_extents: BTreeMap<SimThingId, ResidencyExtent>,
    /// Direct-child placements indexed first by their owning granter. Every
    /// adjudication reads exactly one inner map; there is no global extent scan.
    committed: BTreeMap<SimThingId, BTreeMap<SimThingId, CommittedResidencyPlacement>>,
    terminal: Option<ResidencySessionTermination>,
}

impl ResidencyPlacementBook {
    pub(crate) fn declare_root_extent(
        &mut self,
        granter: SimThingId,
        extent: ResidencyExtent,
    ) -> Result<(), ResidencyPlacementError> {
        if let Some(existing) = self.root_extents.get(&granter).copied() {
            if existing != extent {
                return Err(ResidencyPlacementError::Configuration {
                    detail: format!(
                        "root granter {} already owns extent {:?}, not {:?}",
                        granter.raw(),
                        existing,
                        extent
                    ),
                });
            }
            return Ok(());
        }
        self.root_extents.insert(granter, extent);
        Ok(())
    }

    pub(crate) fn root_extent(&self, granter: SimThingId) -> Option<ResidencyExtent> {
        self.root_extents.get(&granter).copied()
    }

    pub(crate) fn placement(
        &self,
        granter: SimThingId,
        grantee: SimThingId,
    ) -> Option<CommittedResidencyPlacement> {
        self.committed.get(&granter)?.get(&grantee).copied()
    }

    pub(crate) fn ensure_active(&self) -> Result<(), ResidencyPlacementError> {
        match &self.terminal {
            Some(termination) => Err(ResidencyPlacementError::SessionTerminated(
                termination.clone(),
            )),
            None => Ok(()),
        }
    }

    pub(crate) fn refuse(
        &mut self,
        entitlement: ProvisionalResidencyEntitlement,
        proposed: ResidencyExtent,
        reason: ResidencyPlacementRefusalReason,
        attempted_generation: GenerationStamp,
        schedule: &mut IntegrationSchedule,
    ) -> ResidencyPlacementError {
        let refusal = ResidencyPlacementRefusal {
            identity: entitlement.identity(),
            proposed,
            reason,
            retained_unmet_quantity: entitlement.quantity,
            attempted_generation,
            revalue_generation: GenerationStamp::new(attempted_generation.get().saturating_add(1)),
        };
        schedule.record_kind(
            IntegrationScheduleRowKind::ResidencyPlacementRefusal,
            attempted_generation,
            entitlement.granted_generation,
            entitlement.market_grant_key,
        );
        ResidencyPlacementError::Refused(refusal)
    }

    pub(crate) fn prepare(
        &mut self,
        entitlement: ProvisionalResidencyEntitlement,
        containing: ResidencyExtent,
        proposed: ResidencyExtent,
        attempted_generation: GenerationStamp,
        require_existing: bool,
        schedule: &mut IntegrationSchedule,
    ) -> Result<PreparedResidencyPlacement, ResidencyPlacementError> {
        self.ensure_active()?;
        self.audit_level(
            entitlement.granter,
            containing,
            attempted_generation,
            schedule,
        )?;

        let identity = entitlement.identity();
        let existing = self
            .committed
            .get(&identity.granter)
            .and_then(|level| level.get(&identity.grantee))
            .copied();
        if let Some(existing) = existing {
            if existing.identity != identity {
                return Err(self.refuse(
                    entitlement,
                    proposed,
                    ResidencyPlacementRefusalReason::GrantIdentityConflict {
                        existing: existing.identity,
                        requested: identity,
                    },
                    attempted_generation,
                    schedule,
                ));
            }
            if existing.extent == proposed && existing.quantity == entitlement.quantity {
                return Ok(PreparedResidencyPlacement {
                    placement: existing,
                    disposition: ResidencyPlacementDisposition::Unchanged,
                });
            }
        } else if require_existing {
            return Err(self.refuse(
                entitlement,
                proposed,
                ResidencyPlacementRefusalReason::MissingCommittedPlacementForRelocation {
                    requested: identity,
                },
                attempted_generation,
                schedule,
            ));
        }

        if entitlement.quantity != proposed.length {
            return Err(self.refuse(
                entitlement,
                proposed,
                ResidencyPlacementRefusalReason::QuantityExtentMismatch {
                    entitlement_quantity: entitlement.quantity,
                    extent_length: proposed.length,
                },
                attempted_generation,
                schedule,
            ));
        }

        let siblings: Vec<_> = self
            .committed
            .get(&entitlement.granter)
            .into_iter()
            .flat_map(|level| level.values())
            .filter(|placement| placement.identity.grantee != entitlement.grantee)
            .copied()
            .collect();
        if let Err(reason) = ResidencyPlacementOracle::judge(containing, proposed, &siblings) {
            return Err(self.refuse(
                entitlement,
                proposed,
                reason,
                attempted_generation,
                schedule,
            ));
        }

        Ok(PreparedResidencyPlacement {
            placement: CommittedResidencyPlacement {
                identity,
                extent: proposed,
                quantity: entitlement.quantity,
                committed_generation: attempted_generation,
            },
            disposition: if existing.is_some() {
                ResidencyPlacementDisposition::Relocated
            } else {
                ResidencyPlacementDisposition::Committed
            },
        })
    }

    pub(crate) fn commit(
        &mut self,
        prepared: PreparedResidencyPlacement,
        source_generation: GenerationStamp,
        schedule: &mut IntegrationSchedule,
    ) -> ResidencyPlacementOutcome {
        let placement = prepared.placement;
        match prepared.disposition {
            ResidencyPlacementDisposition::Unchanged => {}
            ResidencyPlacementDisposition::Committed => {
                self.committed
                    .entry(placement.identity.granter)
                    .or_default()
                    .insert(placement.identity.grantee, placement);
                schedule.record_kind(
                    IntegrationScheduleRowKind::ResidencyPlacementCommit,
                    placement.committed_generation,
                    source_generation,
                    placement.identity.market_grant_key,
                );
            }
            ResidencyPlacementDisposition::Relocated => {
                self.committed
                    .entry(placement.identity.granter)
                    .or_default()
                    .insert(placement.identity.grantee, placement);
                schedule.record_kind(
                    IntegrationScheduleRowKind::ResidencyRelocation,
                    placement.committed_generation,
                    source_generation,
                    placement.identity.market_grant_key,
                );
            }
        }
        ResidencyPlacementOutcome {
            placement,
            disposition: prepared.disposition,
        }
    }

    pub(crate) fn audit_level(
        &mut self,
        granter: SimThingId,
        containing: ResidencyExtent,
        observed_generation: GenerationStamp,
        schedule: &mut IntegrationSchedule,
    ) -> Result<(), ResidencyPlacementError> {
        self.ensure_active()?;
        let level: Vec<_> = self
            .committed
            .get(&granter)
            .into_iter()
            .flat_map(|level| level.values())
            .copied()
            .collect();

        for placement in &level {
            if !containing.contains(placement.extent) {
                return self.terminate_for_corruption(
                    CommittedResidencyCorruption::OutOfBounds {
                        boundary_granter: granter,
                        placement_granter: placement.identity.granter,
                        grantee: placement.identity.grantee,
                        containing,
                        committed: placement.extent,
                    },
                    observed_generation,
                    placement.identity.market_grant_key,
                    schedule,
                );
            }
        }
        for (index, first) in level.iter().enumerate() {
            for second in level.iter().skip(index + 1) {
                if first.extent.overlaps(second.extent) {
                    return self.terminate_for_corruption(
                        CommittedResidencyCorruption::Overlap {
                            boundary_granter: granter,
                            first_granter: first.identity.granter,
                            first_grantee: first.identity.grantee,
                            first_extent: first.extent,
                            second_granter: second.identity.granter,
                            second_grantee: second.identity.grantee,
                            second_extent: second.extent,
                        },
                        observed_generation,
                        first.identity.market_grant_key
                            ^ second.identity.market_grant_key.rotate_left(1),
                        schedule,
                    );
                }
            }
        }
        Ok(())
    }

    fn terminate_for_corruption(
        &mut self,
        corruption: CommittedResidencyCorruption,
        observed_generation: GenerationStamp,
        product_key: u64,
        schedule: &mut IntegrationSchedule,
    ) -> Result<(), ResidencyPlacementError> {
        let termination = ResidencySessionTermination {
            corruption,
            observed_at_generation: observed_generation,
        };
        // Record first. Only after the canonical schedule owns the fault point
        // does this placement boundary become unusable.
        schedule.record_kind(
            IntegrationScheduleRowKind::ResidencyCommittedCorruption,
            observed_generation,
            observed_generation,
            product_key,
        );
        self.terminal = Some(termination.clone());
        Err(ResidencyPlacementError::SessionTerminated(termination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn placement(
        granter: u32,
        grantee: u32,
        key: u64,
        start: u32,
        length: u32,
    ) -> CommittedResidencyPlacement {
        CommittedResidencyPlacement {
            identity: ResidencyPlacementIdentity {
                granter: SimThingId::from_session_raw(granter),
                grantee: SimThingId::from_session_raw(grantee),
                market_grant_key: key,
            },
            extent: ResidencyExtent::try_new(start, length).unwrap(),
            quantity: length,
            committed_generation: GenerationStamp::new(4),
        }
    }

    #[test]
    fn committed_residency_corruption_records_then_hard_faults_for_exact_reason() {
        let granter = SimThingId::from_session_raw(10);
        let containing = ResidencyExtent::try_new(0, 8).unwrap();

        let mut overlap = ResidencyPlacementBook::default();
        let level = overlap.committed.entry(granter).or_default();
        level.insert(SimThingId::from_session_raw(11), placement(10, 11, 1, 0, 5));
        level.insert(SimThingId::from_session_raw(12), placement(10, 12, 2, 4, 2));
        let mut schedule = IntegrationSchedule::new();
        let error = overlap
            .audit_level(granter, containing, GenerationStamp::new(9), &mut schedule)
            .expect_err("committed overlap must terminate");
        let termination = error.termination().expect("typed session termination");
        assert!(matches!(
            termination.corruption(),
            CommittedResidencyCorruption::Overlap {
                boundary_granter,
                first_granter,
                first_extent,
                second_granter,
                second_extent,
                ..
            } if *boundary_granter == granter
                && *first_granter == granter
                && *second_granter == granter
                && *first_extent == ResidencyExtent::try_new(0, 5).unwrap()
                && *second_extent == ResidencyExtent::try_new(4, 2).unwrap()
        ));
        assert_eq!(
            schedule
                .entries_of_kind(IntegrationScheduleRowKind::ResidencyCommittedCorruption)
                .count(),
            1,
            "the canonical schedule owns the fault observation before shutdown"
        );
        assert!(
            overlap.ensure_active().is_err(),
            "the boundary is unusable after the recorded fault"
        );

        let mut out_of_bounds = ResidencyPlacementBook::default();
        out_of_bounds
            .committed
            .entry(granter)
            .or_default()
            .insert(SimThingId::from_session_raw(13), placement(10, 13, 3, 7, 2));
        let mut schedule = IntegrationSchedule::new();
        let error = out_of_bounds
            .audit_level(granter, containing, GenerationStamp::new(10), &mut schedule)
            .expect_err("committed out-of-bounds must terminate");
        assert!(matches!(
            error.termination().unwrap().corruption(),
            CommittedResidencyCorruption::OutOfBounds {
                boundary_granter,
                placement_granter,
                containing: recorded_container,
                committed,
                ..
            } if *boundary_granter == granter
                && *placement_granter == granter
                && *recorded_container == containing
                && *committed == ResidencyExtent::try_new(7, 2).unwrap()
        ));
        assert_eq!(
            schedule.entries()[0].row_kind(),
            IntegrationScheduleRowKind::ResidencyCommittedCorruption
        );
    }
}
