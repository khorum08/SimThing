//! Session-side ordinary-growth entitlement input.
//!
//! This is a binding over the graduated 11.2a market lifecycle. It authors no
//! second clearing engine: Draw authorization, generic constrained clearing,
//! `MarketGrantRecord`, and the 11.2b conversion bridge are consumed directly.

use std::collections::BTreeSet;

use simthing_core::{GenerationStamp, SimThingId, SpecializationProfile, TransformOp};
use simthing_gpu::SlotAllocator;
use simthing_sim::{GrowthEntitlementDecision, OrdinaryGrowthCandidate};
use simthing_spec::{
    admit_specialization_flow_market, clear_constrained_claims_at_generation,
    AdmittedSpecializationFlowMarket, AuthoredClearingProgram, ClearingRemainderAuthority,
    ConservedOfferingSpec, ConstrainedClaim, ConstrainedSupply, DrawEnvelopeTemplateSpec,
    OfferingPriceVectorSpec, OwnerChannelScopeKey, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, ScopeId, SpecializationFlowMarketSpec,
};
use thiserror::Error;

const IMPLICIT_PROFILE: &str = "simthing::implicit-root-standing-growth";
const IMPLICIT_OFFERING: &str = "simthing::ordinary-growth-residency";
const IMPLICIT_DRAW: &str = "simthing::ordinary-growth-draw";
const IMPLICIT_TRIGGER: &str = "simthing::ordinary-growth-boundary";
const IMPLICIT_RESOURCE: &str = "simthing::residency-row-capacity";

#[derive(Debug, Error)]
pub enum GrowthEntitlementError {
    #[error("implicit standing entitlement admission failed: {0}")]
    ImplicitAdmission(String),
    #[error("ordinary growth Draw authorization failed: {0}")]
    Draw(String),
    #[error("ordinary growth clearing failed: {0}")]
    Clearing(String),
    #[error("ordinary growth market grant failed: {0}")]
    Grant(String),
    #[error("ordinary growth residency bridge failed: {0}")]
    Bridge(String),
    #[error("ordinary growth clearing omitted candidate {0:?}")]
    MissingCandidate(SimThingId),
    #[error("ordinary growth market binding must install at tick zero, not tick {tick} / generation {generation}")]
    LateInstall { tick: u64, generation: u64 },
    #[error("ordinary growth granter {0:?} has no resident row")]
    UnresidentGranter(SimThingId),
    #[error("ordinary growth market is not qualified for resident clearing")]
    ResidentProfileUnqualified,
    #[error("ordinary growth resident clearing failed: {0}")]
    Resident(String),
}

/// Frozen session binding for one standing granter. Authored sessions may
/// replace the implicit root binding before the first tick; both shapes run
/// the identical Draw -> clear -> MarketGrantRecord path.
#[derive(Clone, Debug)]
pub struct GrowthEntitlementMarketBinding {
    market: AdmittedSpecializationFlowMarket,
    granter: SimThingId,
    offering_id: String,
    draw_id: String,
    scope: OwnerChannelScopeKey,
    active_lifecycle_triggers: BTreeSet<String>,
    clearing_program: AuthoredClearingProgram,
    effective_weight: f32,
    priority: u32,
    implicit_root_standing: bool,
    resident_qualification: Option<crate::resident_clearing_runtime::ResidentMarketQualification>,
}

impl GrowthEntitlementMarketBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn from_admitted_market(
        market: AdmittedSpecializationFlowMarket,
        granter: SimThingId,
        offering_id: impl Into<String>,
        draw_id: impl Into<String>,
        scope: OwnerChannelScopeKey,
        active_lifecycle_triggers: BTreeSet<String>,
        clearing_program: AuthoredClearingProgram,
        effective_weight: f32,
        priority: u32,
    ) -> Self {
        Self {
            market,
            granter,
            offering_id: offering_id.into(),
            draw_id: draw_id.into(),
            scope,
            active_lifecycle_triggers,
            clearing_program,
            effective_weight,
            priority,
            implicit_root_standing: false,
            resident_qualification: None,
        }
    }

    /// Admit the compatibility standing-root input through the ordinary 11.2a
    /// market germ. This is a one-granter authored market shape, not a direct
    /// allocation or placement bypass.
    pub fn implicit_root_standing(granter: SimThingId) -> Result<Self, GrowthEntitlementError> {
        let profiles = vec![SpecializationProfile {
            id: IMPLICIT_PROFILE.into(),
            description: "implicit root standing entitlement through the ordinary market germ"
                .into(),
            requirements: Vec::new(),
        }];
        let active_lifecycle_triggers = BTreeSet::from([IMPLICIT_TRIGGER.to_string()]);
        let resource = ResourceKey::new(IMPLICIT_RESOURCE);
        let market = admit_specialization_flow_market(
            &profiles,
            &active_lifecycle_triggers,
            SpecializationFlowMarketSpec {
                specialization_profile_id: IMPLICIT_PROFILE.into(),
                offerings: vec![ConservedOfferingSpec {
                    id: IMPLICIT_OFFERING.into(),
                    resource_key: resource.clone(),
                    price: OfferingPriceVectorSpec {
                        unit_cost: 1.0,
                        default_clearing_weight: 1.0,
                    },
                }],
                draw_envelopes: vec![DrawEnvelopeTemplateSpec {
                    id: IMPLICIT_DRAW.into(),
                    offering_refs: vec![IMPLICIT_OFFERING.into()],
                    lifecycle_trigger_refs: vec![IMPLICIT_TRIGGER.into()],
                    min_quantity: 1,
                    max_quantity: u32::MAX,
                }],
            },
        )
        .map_err(|error| GrowthEntitlementError::ImplicitAdmission(error.to_string()))?;
        Ok(Self {
            market,
            granter,
            offering_id: IMPLICIT_OFFERING.into(),
            draw_id: IMPLICIT_DRAW.into(),
            scope: OwnerChannelScopeKey {
                owner_ref: OwnerRef::new(format!("standing-root/{}", granter.raw())),
                resource_key: resource,
                scope_id: ScopeId::from_boundary(granter),
            },
            active_lifecycle_triggers,
            clearing_program: AuthoredClearingProgram::new(TransformOp::set(1.0)),
            effective_weight: 1.0,
            priority: 0,
            implicit_root_standing: true,
            resident_qualification: None,
        })
    }

    pub fn granter(&self) -> SimThingId {
        self.granter
    }

    pub fn is_implicit_root_standing(&self) -> bool {
        self.implicit_root_standing
    }

    pub fn resident_qualification(
        &self,
    ) -> Option<&crate::resident_clearing_runtime::ResidentMarketQualification> {
        self.resident_qualification.as_ref()
    }

    pub(crate) fn resident_market_admission(
        &self,
    ) -> crate::resident_clearing_runtime::ResidentMarketAdmission {
        let offering = self
            .market
            .offering(&self.offering_id)
            .expect("admitted binding retains its offering");
        let draw = self
            .market
            .draw_envelope(&self.draw_id)
            .expect("admitted binding retains its Draw");
        crate::resident_clearing_runtime::ResidentMarketAdmission::new(
            format!(
                "{}|{}|{:?}|{:?}",
                self.market.specialization_profile_id(),
                self.offering_id,
                offering,
                draw
            ),
            self.scope.resource_key.as_str(),
            format!(
                "{}|{}|{}",
                self.scope.owner_ref.as_str(),
                self.scope.resource_key.as_str(),
                self.scope.scope_id.as_str()
            ),
            &self.draw_id,
            None,
            format!("hard-precedence/{}", self.priority),
            format!(
                "{:?}|effective-weight={:08x}",
                self.clearing_program.score_program().nodes(),
                self.effective_weight.to_bits()
            ),
            simthing_gpu::ResidentExactBasisIdentity::LiveAllocatedFlow,
        )
    }

    pub(crate) fn install_resident_qualification(
        &mut self,
        qualification: crate::resident_clearing_runtime::ResidentMarketQualification,
    ) {
        self.resident_qualification = Some(qualification);
    }

    /// Explicit vendorized CPU oracle. Ordinary production selects this door
    /// only under `ClearingExecutionPosture::CpuVendorizedOracle`; adapter or
    /// resident dispatch failure never reaches it.
    pub fn resolve_batch_cpu_vendorized_oracle(
        &self,
        allocator: &SlotAllocator,
        generation: GenerationStamp,
        candidates: &[OrdinaryGrowthCandidate],
        integration_schedule: &mut simthing_core::IntegrationSchedule,
    ) -> Result<Vec<GrowthEntitlementDecision>, GrowthEntitlementError> {
        self.resolve_batch(allocator, generation, candidates, integration_schedule)
    }

    /// Compatibility name retained for the frozen CPU-oracle witnesses. There
    /// is no production caller after RESIDENT-CLEARING-CUTOVER-0.
    pub fn resolve_batch(
        &self,
        allocator: &SlotAllocator,
        generation: GenerationStamp,
        candidates: &[OrdinaryGrowthCandidate],
        integration_schedule: &mut simthing_core::IntegrationSchedule,
    ) -> Result<Vec<GrowthEntitlementDecision>, GrowthEntitlementError> {
        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        let mut claims = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            let demand = RuntimeOwnerSiloDemandBucket {
                owner_ref: self.scope.owner_ref.clone(),
                resource_key: self.scope.resource_key.clone(),
                scope_id: self.scope.scope_id.clone(),
                requested: candidate.quantity(),
                priority: self.priority,
                source_simthing_id_raw: Some(candidate.grantee().raw()),
            };
            let authored = self
                .market
                .authorize_draw(
                    &self.draw_id,
                    &self.offering_id,
                    demand,
                    self.effective_weight,
                    &self.active_lifecycle_triggers,
                )
                .map_err(|error| GrowthEntitlementError::Draw(error.to_string()))?;
            claims.push(
                ConstrainedClaim::from_runtime_demand(&authored.demand, authored.order_weight)
                    .map_err(|error| GrowthEntitlementError::Clearing(error.to_string()))?,
            );
        }

        let results = clear_constrained_claims_at_generation(
            &[ConstrainedSupply {
                scope: self.scope.clone(),
                available: allocator.growth_capacity_available(self.granter),
            }],
            &claims,
            &self.clearing_program,
            ClearingRemainderAuthority {
                granter: self.granter,
                generation,
            },
        )
        .map_err(|error| GrowthEntitlementError::Clearing(error.to_string()))?;
        let grants = &results
            .first()
            .ok_or_else(|| GrowthEntitlementError::Clearing("missing scope result".into()))?
            .grants;

        let mut decisions = Vec::with_capacity(candidates.len());
        for candidate in candidates {
            let grant = grants
                .iter()
                .find(|grant| grant.source_simthing_id == candidate.grantee())
                .ok_or(GrowthEntitlementError::MissingCandidate(
                    candidate.grantee(),
                ))?;
            if grant.granted == candidate.quantity() {
                let record = self
                    .market
                    .record_cleared_grant(
                        self.granter,
                        &self.offering_id,
                        grant,
                        generation,
                        integration_schedule,
                    )
                    .map_err(|error| GrowthEntitlementError::Grant(error.to_string()))?;
                let (entitlement, provenance) =
                    crate::residency_market::provisional_residency_and_provenance_from_market_grant(
                        &self.market,
                        &record,
                    )
                    .map_err(|error| GrowthEntitlementError::Bridge(error.to_string()))?;
                decisions.push(GrowthEntitlementDecision::granted(
                    *candidate,
                    entitlement,
                    provenance,
                ));
            } else {
                let key = if grant.granted == 0 {
                    None
                } else {
                    let record = self
                        .market
                        .record_cleared_grant(
                            self.granter,
                            &self.offering_id,
                            grant,
                            generation,
                            integration_schedule,
                        )
                        .map_err(|error| GrowthEntitlementError::Grant(error.to_string()))?;
                    Some(
                        crate::residency_market::provisional_residency_from_market_grant(
                            &self.market,
                            &record,
                        )
                        .map_err(|error| GrowthEntitlementError::Bridge(error.to_string()))?
                        .market_grant_key(),
                    )
                };
                decisions.push(GrowthEntitlementDecision::refused(
                    *candidate,
                    grant.granted,
                    key,
                ));
            }
        }
        Ok(decisions)
    }

    /// Production resident authority for the already-qualified standing-root
    /// growth profile. This boundary executes only its current generation;
    /// temporal demand may be prepared separately once N+1 inputs exist.
    pub fn resolve_batch_resident(
        &self,
        runtime: &mut crate::resident_clearing_runtime::RecursiveResourceFilterRuntime,
        state: &simthing_gpu::WorldGpuState,
        allocator: &SlotAllocator,
        generation: GenerationStamp,
        candidates: &[OrdinaryGrowthCandidate],
        integration_schedule: &mut simthing_core::IntegrationSchedule,
    ) -> Result<Vec<GrowthEntitlementDecision>, GrowthEntitlementError> {
        let qualification = self
            .resident_qualification
            .as_ref()
            .ok_or(GrowthEntitlementError::ResidentProfileUnqualified)?;
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let available = allocator.growth_capacity_available(self.granter);
        let mut ordered = candidates.to_vec();
        ordered.sort_by_key(|candidate| candidate.grantee());
        let rows: Vec<_> = ordered
            .iter()
            .map(
                |candidate| crate::resident_clearing_runtime::ResidentClearingBatchBinding {
                    source_simthing_id: candidate.grantee(),
                    rf_participant: candidate.structural_parent(),
                    requested: candidate.quantity(),
                    available,
                    precedence: 0,
                },
            )
            .collect();
        let root_ticket = runtime
            .dispatch(
                state,
                qualification,
                integration_schedule,
                self.granter,
                generation,
                &rows,
            )
            .map_err(|error| GrowthEntitlementError::Resident(error.to_string()))?;
        let products = runtime
            .materialize(state, qualification, integration_schedule, root_ticket)
            .map_err(|error| GrowthEntitlementError::Resident(error.to_string()))?;
        let mut decisions = Vec::with_capacity(ordered.len());
        for candidate in ordered {
            let product = products
                .iter()
                .copied()
                .find(|product| product.source_simthing_id() == candidate.grantee())
                .ok_or(GrowthEntitlementError::MissingCandidate(
                    candidate.grantee(),
                ))?;
            if product.granted() == candidate.quantity() {
                let record = self
                    .market
                    .record_resident_structural_grant(
                        self.granter,
                        &self.offering_id,
                        &self.scope,
                        candidate.quantity(),
                        product,
                        generation,
                        integration_schedule,
                    )
                    .map_err(|error| GrowthEntitlementError::Grant(error.to_string()))?;
                let (entitlement, provenance) =
                    crate::residency_market::provisional_residency_and_provenance_from_market_grant(
                        &self.market,
                        &record,
                    )
                    .map_err(|error| GrowthEntitlementError::Bridge(error.to_string()))?;
                decisions.push(GrowthEntitlementDecision::granted(
                    candidate,
                    entitlement,
                    provenance,
                ));
            } else {
                let key = if product.granted() == 0 {
                    None
                } else {
                    let record = self
                        .market
                        .record_resident_structural_grant(
                            self.granter,
                            &self.offering_id,
                            &self.scope,
                            candidate.quantity(),
                            product,
                            generation,
                            integration_schedule,
                        )
                        .map_err(|error| GrowthEntitlementError::Grant(error.to_string()))?;
                    Some(
                        crate::residency_market::provisional_residency_from_market_grant(
                            &self.market,
                            &record,
                        )
                        .map_err(|error| GrowthEntitlementError::Bridge(error.to_string()))?
                        .market_grant_key(),
                    )
                };
                decisions.push(GrowthEntitlementDecision::refused(
                    candidate,
                    product.granted(),
                    key,
                ));
            }
        }
        Ok(decisions)
    }
}
