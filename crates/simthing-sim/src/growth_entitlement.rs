//! Ordinary-growth entitlement products shared by the boundary and replay.
//!
//! Candidate collection is structural; entitlement remains the existing
//! market's decision, and placement remains kernel physics.  This module owns
//! no market, clearing program, clock, retry loop, or history surface.

use serde::{Deserialize, Serialize};
use simthing_core::{GenerationStamp, SimThingId};
use simthing_gpu::{
    GrowthResidencyCommit, ProvisionalResidencyEntitlement, ResidencyPlacementRefusal,
};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OrdinaryGrowthOrigin {
    Fission,
    AddChild,
}

/// Complete pre-mutation identity and row quantity of one ordinary growth
/// candidate. Ordering is logical and stable; physical iteration order is not
/// represented.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrdinaryGrowthCandidate {
    structural_parent: SimThingId,
    grantee: SimThingId,
    quantity: u32,
    origin: OrdinaryGrowthOrigin,
}

impl OrdinaryGrowthCandidate {
    pub fn new(
        structural_parent: SimThingId,
        grantee: SimThingId,
        quantity: u32,
        origin: OrdinaryGrowthOrigin,
    ) -> Self {
        Self {
            structural_parent,
            grantee,
            quantity,
            origin,
        }
    }

    pub const fn structural_parent(self) -> SimThingId {
        self.structural_parent
    }

    pub const fn grantee(self) -> SimThingId {
        self.grantee
    }

    pub const fn quantity(self) -> u32 {
        self.quantity
    }

    pub const fn origin(self) -> OrdinaryGrowthOrigin {
        self.origin
    }

    pub fn product_key(self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        for component in [
            self.structural_parent.raw(),
            self.grantee.raw(),
            self.quantity,
            match self.origin {
                OrdinaryGrowthOrigin::Fission => 0,
                OrdinaryGrowthOrigin::AddChild => 1,
            },
        ] {
            for byte in component.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        hash
    }
}

/// Result returned by the session-installed 11.2a clearing input. A granted
/// decision must carry the real converted market entitlement; a refusal is U
/// and carries no attach/populate capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrowthEntitlementDecision {
    Granted {
        candidate: OrdinaryGrowthCandidate,
        entitlement: ProvisionalResidencyEntitlement,
    },
    Refused {
        candidate: OrdinaryGrowthCandidate,
        granted: u32,
        market_grant_key: Option<u64>,
    },
}

impl GrowthEntitlementDecision {
    pub fn granted(
        candidate: OrdinaryGrowthCandidate,
        entitlement: ProvisionalResidencyEntitlement,
    ) -> Self {
        Self::Granted {
            candidate,
            entitlement,
        }
    }

    pub fn refused(
        candidate: OrdinaryGrowthCandidate,
        granted: u32,
        market_grant_key: Option<u64>,
    ) -> Self {
        Self::Refused {
            candidate,
            granted,
            market_grant_key,
        }
    }

    pub const fn candidate(self) -> OrdinaryGrowthCandidate {
        match self {
            Self::Granted { candidate, .. } | Self::Refused { candidate, .. } => candidate,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrdinaryGrowthRefusalReason {
    MarketUnresolved { granted: u32 },
    Placement(ResidencyPlacementRefusal),
    LifecycleAdmission,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrdinaryGrowthRefusal {
    candidate: OrdinaryGrowthCandidate,
    attempted_generation: GenerationStamp,
    revalue_generation: GenerationStamp,
    market_grant_key: Option<u64>,
    reason: OrdinaryGrowthRefusalReason,
}

impl OrdinaryGrowthRefusal {
    pub fn market(
        candidate: OrdinaryGrowthCandidate,
        attempted_generation: GenerationStamp,
        granted: u32,
        market_grant_key: Option<u64>,
    ) -> Self {
        Self {
            candidate,
            attempted_generation,
            revalue_generation: GenerationStamp::new(attempted_generation.get().saturating_add(1)),
            market_grant_key,
            reason: OrdinaryGrowthRefusalReason::MarketUnresolved { granted },
        }
    }

    pub fn placement(
        candidate: OrdinaryGrowthCandidate,
        refusal: ResidencyPlacementRefusal,
    ) -> Self {
        Self {
            candidate,
            attempted_generation: refusal.attempted_generation(),
            revalue_generation: refusal.revalue_generation(),
            market_grant_key: Some(refusal.identity().market_grant_key()),
            reason: OrdinaryGrowthRefusalReason::Placement(refusal),
        }
    }

    pub fn lifecycle(
        candidate: OrdinaryGrowthCandidate,
        attempted_generation: GenerationStamp,
        market_grant_key: u64,
    ) -> Self {
        Self {
            candidate,
            attempted_generation,
            revalue_generation: GenerationStamp::new(attempted_generation.get().saturating_add(1)),
            market_grant_key: Some(market_grant_key),
            reason: OrdinaryGrowthRefusalReason::LifecycleAdmission,
        }
    }

    pub const fn candidate(&self) -> OrdinaryGrowthCandidate {
        self.candidate
    }

    pub const fn attempted_generation(&self) -> GenerationStamp {
        self.attempted_generation
    }

    pub const fn revalue_generation(&self) -> GenerationStamp {
        self.revalue_generation
    }

    pub const fn market_grant_key(&self) -> Option<u64> {
        self.market_grant_key
    }

    pub fn reason(&self) -> &OrdinaryGrowthRefusalReason {
        &self.reason
    }

    pub fn schedule_product_key(&self) -> u64 {
        self.market_grant_key
            .unwrap_or_else(|| self.candidate.product_key())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordedGrowthResidencyFact {
    Accepted(GrowthResidencyCommit),
    Refused(OrdinaryGrowthRefusal),
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum GrowthEntitlementBatchError {
    #[error("ordinary growth entitlement resolver returned no decision for {0:?}")]
    MissingDecision(SimThingId),
    #[error("ordinary growth entitlement resolver returned duplicate decision for {0:?}")]
    DuplicateDecision(SimThingId),
    #[error("ordinary growth entitlement resolver returned an unknown candidate {0:?}")]
    UnknownCandidate(SimThingId),
    #[error("ordinary growth entitlement product does not match candidate {0:?}")]
    ProductMismatch(SimThingId),
    #[error("ordinary growth candidate batch repeats grantee {0:?}")]
    DuplicateCandidate(SimThingId),
}

pub(crate) fn validate_decisions(
    candidates: &[OrdinaryGrowthCandidate],
    decisions: Vec<GrowthEntitlementDecision>,
) -> Result<BTreeMap<SimThingId, GrowthEntitlementDecision>, GrowthEntitlementBatchError> {
    let mut expected = BTreeMap::new();
    for candidate in candidates {
        if expected.insert(candidate.grantee(), *candidate).is_some() {
            return Err(GrowthEntitlementBatchError::DuplicateCandidate(
                candidate.grantee(),
            ));
        }
    }
    let mut by_grantee = BTreeMap::new();
    for decision in decisions {
        let candidate = decision.candidate();
        let Some(expected_candidate) = expected.get(&candidate.grantee()).copied() else {
            return Err(GrowthEntitlementBatchError::UnknownCandidate(
                candidate.grantee(),
            ));
        };
        if expected_candidate != candidate {
            return Err(GrowthEntitlementBatchError::ProductMismatch(
                candidate.grantee(),
            ));
        }
        if let GrowthEntitlementDecision::Granted { entitlement, .. } = decision {
            if entitlement.grantee() != candidate.grantee()
                || entitlement.quantity() != candidate.quantity()
            {
                return Err(GrowthEntitlementBatchError::ProductMismatch(
                    candidate.grantee(),
                ));
            }
        }
        if by_grantee.insert(candidate.grantee(), decision).is_some() {
            return Err(GrowthEntitlementBatchError::DuplicateDecision(
                candidate.grantee(),
            ));
        }
    }
    let decided: BTreeSet<_> = by_grantee.keys().copied().collect();
    if let Some(missing) = expected.keys().find(|id| !decided.contains(id)) {
        return Err(GrowthEntitlementBatchError::MissingDecision(*missing));
    }
    Ok(by_grantee)
}
