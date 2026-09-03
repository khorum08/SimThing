//! RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick boundary report.
//!
//! Composes participant admission → reduce-up → writeback → disburse-down → local allocation.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};

use simthing_core::{GenerationStamp, GenerationStamped, SimThingId};

use super::constrained_clearing::{
    carry_unresolved_demand_to_next_generation, clear_constrained_claims_at_generation,
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim,
    ConstrainedClearingResult, ConstrainedSupply, PersistenceDeformationBindings,
    UnresolvedDemandObservation,
};
use super::legacy_owner_channel_rf::{
    evaluate_planet_child_rf_admission_from_owner_view,
    evaluate_planet_child_rf_reduce_up_from_owner_view, PlanetChildRfAdmissionClassification,
    PlanetChildRfAdmissionReport, PlanetChildRfReduceUpReport,
};
use super::owner_channel_admission::{admit_intrinsic_owner_channels, IntrinsicOwnerChannelView};
use super::owner_silo_disburse_down::{
    apply_owner_silo_runtime_disburse_down_cpu, demand_bucket_sort_key,
    owner_silo_demand_buckets_from_owner_view, RuntimeOwnerSiloDemandBucket,
    RuntimeOwnerSiloDisburseDownResult,
};
use super::owner_silo_runtime_writeback::{
    apply_owner_silo_runtime_writeback_cpu,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, RuntimeOwnerSiloWritebackResult,
};
use super::runtime_local_allocation::{
    apply_runtime_local_allocations_from_disburse_down, RuntimeLocalAllocationApplicationReport,
};
use super::scenario::SimThingScenarioSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRfTickErrorKind {
    ParticipantAdmissionRejected,
    ReduceUpRejected,
    OwnerSiloWritebackRejected,
    DisburseDownRejected,
    LocalAllocationRejected,
    ArithmeticOverflow,
    DemandCurrentToNextAlreadyProduced,
    DemandCurrentToNextRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRfTickError {
    pub kind: RuntimeRfTickErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRfTickDeferralKind {
    EconomyExecutionDeferred,
    ScenarioAuthorityMutationDeferred,
    LocalEffectApplicationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRfTickDeferral {
    pub kind: RuntimeRfTickDeferralKind,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeRfTickReport {
    pub participant_count: u32,
    pub reduce_up_bucket_count: u32,
    pub owner_silo_writeback_count: u32,
    pub disburse_down_result_count: u32,
    pub local_allocation_count: u32,

    pub surplus_total: u32,
    pub deficit_total: u32,
    pub writeback_allocated_total: u32,
    pub disburse_allocated_total: u32,
    pub local_allocated_total: u32,
    pub local_unmet_total: u32,

    pub participant_admission_ready: bool,
    pub reduce_up_ready: bool,
    pub owner_silo_writeback_ready: bool,
    pub owner_silo_disburse_down_ready: bool,
    pub runtime_local_allocation_ready: bool,

    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub local_effect_application_deferred: bool,

    pub participant_report: PlanetChildRfAdmissionReport,
    pub reduce_up_report: PlanetChildRfReduceUpReport,
    pub writeback_results: Vec<RuntimeOwnerSiloWritebackResult>,
    pub disburse_down_results: Vec<RuntimeOwnerSiloDisburseDownResult>,
    pub local_allocation_report: RuntimeLocalAllocationApplicationReport,

    pub errors: Vec<RuntimeRfTickError>,
    pub deferrals: Vec<RuntimeRfTickDeferral>,
}

/// The one generation authority allowed to mint ordinary RF demand's N→N+1
/// production door.
///
/// This is transient execution authority, not economic vocabulary. It is
/// intentionally non-Clone and has no serialization surface. One authority
/// instance admits one Current→Next production attempt; a second attempt is a
/// typed refusal even if the caller retained the clearing results.
#[derive(Debug)]
pub struct RuntimeRfDemandGenerationAuthority {
    clearing_authority: ClearingRemainderAuthority,
    persistence_deformations: PersistenceDeformationBindings,
    current_to_next_produced: AtomicBool,
}

impl RuntimeRfDemandGenerationAuthority {
    pub fn new(clearing_authority: ClearingRemainderAuthority) -> Self {
        Self::with_persistence_deformations(
            clearing_authority,
            PersistenceDeformationBindings::default(),
        )
    }

    pub fn with_persistence_deformations(
        clearing_authority: ClearingRemainderAuthority,
        persistence_deformations: PersistenceDeformationBindings,
    ) -> Self {
        Self {
            clearing_authority,
            persistence_deformations,
            current_to_next_produced: AtomicBool::new(false),
        }
    }

    pub const fn current_generation(&self) -> GenerationStamp {
        self.clearing_authority.generation
    }

    fn mint_current_to_next(&self) -> Result<(), RuntimeRfTickError> {
        self.current_to_next_produced
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| demand_current_to_next_already_produced(self.current_generation()))?;
        Ok(())
    }
}

/// Vendorized CPU-oracle proof of the ordinary demand product for N+1.
///
/// This is the frozen host-reference Current→Next door for
/// `RuntimeOwnerSiloDemandBucket`; resident production carries the identical
/// `T_s` bytes directly into its N+1 intake.
/// It performs the generation-N clear inside the door, derives unresolved
/// observations from those sealed results, consumes the authority's one mint
/// before evaluating caller data, and matches every observation to the same
/// claimant/full-scope next demand automatically. A caller cannot supply a
/// filtered clearing-result slice or pass `None` to omit recurrence. Any
/// unconsumed observation, duplicate next product, malformed clearing result,
/// or second door attempt is refused without returning partial products. The
/// returned tuple exposes the current clear only after recurrence was consumed.
/// `u = 0` traverses this same door and stamps the independently produced demand
/// byte-for-byte at N+1.
pub fn produce_runtime_rf_next_generation_demands(
    authority: &RuntimeRfDemandGenerationAuthority,
    current_supplies: &[ConstrainedSupply],
    current_claims: &[ConstrainedClaim],
    current_program: &AuthoredClearingProgram,
    mut next_demands: Vec<RuntimeOwnerSiloDemandBucket>,
) -> Result<
    (
        Vec<ConstrainedClearingResult>,
        Vec<GenerationStamped<RuntimeOwnerSiloDemandBucket>>,
    ),
    RuntimeRfTickError,
> {
    authority.mint_current_to_next()?;
    let current_generation = authority.current_generation();
    current_generation
        .get()
        .checked_add(1)
        .ok_or_else(|| demand_current_to_next_rejected("generation overflow"))?;

    let current_clearing_results = clear_constrained_claims_at_generation(
        current_supplies,
        current_claims,
        current_program,
        authority.clearing_authority,
    )
    .map_err(|error| demand_current_to_next_rejected(&error.to_string()))?;

    let mut unresolved_by_claimant = BTreeMap::new();
    for result in &current_clearing_results {
        let unresolved_total = result
            .grants
            .iter()
            .try_fold(0u32, |total, grant| total.checked_add(grant.unresolved));
        if unresolved_total != Some(result.unresolved_total) {
            return Err(demand_current_to_next_rejected(
                "clearing result unresolved total is not intact",
            ));
        }
        for grant in &result.grants {
            if grant.scope != result.scope || !grant.has_intact_clearance_seal() {
                return Err(demand_current_to_next_rejected(
                    "clearing grant seal is not intact",
                ));
            }
            if grant.clearing_generation() != current_generation {
                return Err(demand_current_to_next_rejected(
                    "clearing grant generation does not match Current-to-Next authority",
                ));
            }
            let Some(observation) = UnresolvedDemandObservation::from_sealed_grant(grant) else {
                continue;
            };
            let key = (observation.scope.clone(), observation.source_simthing_id);
            if unresolved_by_claimant.insert(key, observation).is_some() {
                return Err(demand_current_to_next_rejected(
                    "duplicate unresolved claimant/full-scope observation",
                ));
            }
        }
    }

    next_demands.sort_by(demand_bucket_sort_key);
    let mut seen_next = BTreeSet::new();
    let mut stamped = Vec::with_capacity(next_demands.len());
    for next_demand in next_demands {
        let source = next_demand
            .source_simthing_id_raw
            .map(SimThingId::from_session_raw)
            .ok_or_else(|| demand_current_to_next_rejected("next demand has no source SimThing"))?;
        let key = (next_demand.scope_key(), source);
        if !seen_next.insert(key.clone()) {
            return Err(demand_current_to_next_rejected(
                "duplicate next demand for claimant/full scope",
            ));
        }
        let unresolved = unresolved_by_claimant.remove(&key);
        let deformation = authority
            .persistence_deformations
            .program_for(&key.0, key.1);
        stamped.push(
            carry_unresolved_demand_to_next_generation(
                current_generation,
                next_demand,
                unresolved,
                deformation,
            )
            .map_err(|error| demand_current_to_next_rejected(&error.to_string()))?,
        );
    }

    if !unresolved_by_claimant.is_empty() {
        return Err(demand_current_to_next_rejected(
            "next demand production omitted an unresolved claimant/full scope",
        ));
    }

    Ok((current_clearing_results, stamped))
}

fn demand_current_to_next_already_produced(generation: GenerationStamp) -> RuntimeRfTickError {
    RuntimeRfTickError {
        kind: RuntimeRfTickErrorKind::DemandCurrentToNextAlreadyProduced,
        message: format!(
            "ordinary RF demand Current-to-Next was already produced from generation {}",
            generation.get()
        ),
    }
}

fn demand_current_to_next_rejected(message: &str) -> RuntimeRfTickError {
    RuntimeRfTickError {
        kind: RuntimeRfTickErrorKind::DemandCurrentToNextRejected,
        message: message.to_owned(),
    }
}

/// Evaluate the full runtime RF tick boundary from Scenario authority input (read-only).
pub fn evaluate_runtime_rf_tick(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeRfTickReport, RuntimeRfTickError> {
    let owner_view =
        admit_intrinsic_owner_channels(scenario).map_err(|error| RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ParticipantAdmissionRejected,
            message: error.to_string(),
        })?;
    evaluate_runtime_rf_tick_from_owner_view(&owner_view)
}

pub fn evaluate_runtime_rf_tick_from_owner_view(
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<RuntimeRfTickReport, RuntimeRfTickError> {
    let scenario = owner_view.scenario();
    let deferrals = default_deferrals();
    let errors = Vec::new();

    let participant_report = evaluate_planet_child_rf_admission_from_owner_view(owner_view);
    if participant_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ParticipantAdmissionRejected,
            message: "planet child RF participant admission rejected".to_string(),
        });
    }
    let participant_admission_ready = participant_report.classification
        != PlanetChildRfAdmissionClassification::Unsupported
        && participant_report.total_participant_count > 0;

    let reduce_up_report = evaluate_planet_child_rf_reduce_up_from_owner_view(owner_view);
    if reduce_up_report.classification == PlanetChildRfAdmissionClassification::Rejected
        || !reduce_up_report.errors.is_empty()
    {
        return Err(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ReduceUpRejected,
            message: "planet child RF reduce-up rejected".to_string(),
        });
    }
    let reduce_up_ready = reduce_up_report.bucket_count > 0;

    let initial_owner_silos =
        runtime_owner_silo_states_from_scenario(scenario).map_err(|e| RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::OwnerSiloWritebackRejected,
            message: e.message,
        })?;
    if initial_owner_silos.is_empty() {
        return Err(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::OwnerSiloWritebackRejected,
            message: "no owner-silo metadata for writeback".to_string(),
        });
    }

    let writeback_inputs = owner_silo_writeback_inputs_from_planet_child_reduce_up(
        &reduce_up_report,
    )
    .map_err(|e| RuntimeRfTickError {
        kind: RuntimeRfTickErrorKind::OwnerSiloWritebackRejected,
        message: e.message,
    })?;

    let writeback_results =
        apply_owner_silo_runtime_writeback_cpu(&initial_owner_silos, &writeback_inputs).map_err(
            |e| RuntimeRfTickError {
                kind: RuntimeRfTickErrorKind::OwnerSiloWritebackRejected,
                message: e.message,
            },
        )?;
    let owner_silo_writeback_ready = !writeback_results.is_empty();

    let demand_buckets =
        owner_silo_demand_buckets_from_owner_view(owner_view).map_err(|e| RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::DisburseDownRejected,
            message: e.message,
        })?;

    let disburse_down_results = if demand_buckets.is_empty() {
        Vec::new()
    } else {
        apply_owner_silo_runtime_disburse_down_cpu(&writeback_results, &demand_buckets).map_err(
            |e| RuntimeRfTickError {
                kind: RuntimeRfTickErrorKind::DisburseDownRejected,
                message: e.message,
            },
        )?
    };
    let owner_silo_disburse_down_ready = true;

    let local_allocation_report = apply_runtime_local_allocations_from_disburse_down(
        &disburse_down_results,
    )
    .map_err(|e| RuntimeRfTickError {
        kind: RuntimeRfTickErrorKind::LocalAllocationRejected,
        message: e.message,
    })?;
    let runtime_local_allocation_ready = true;

    let surplus_total = participant_report.surplus_total;
    let deficit_total = participant_report.deficit_total;

    let writeback_allocated_total = writeback_results
        .iter()
        .try_fold(0u32, |acc, r| {
            r.applied_surplus
                .checked_add(acc)
                .and_then(|v| v.checked_add(r.applied_deficit))
        })
        .ok_or(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ArithmeticOverflow,
            message: "writeback_allocated_total overflow".to_string(),
        })?;

    let disburse_allocated_total = disburse_down_results
        .iter()
        .try_fold(0u32, |acc, r| acc.checked_add(r.allocated_total))
        .ok_or(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ArithmeticOverflow,
            message: "disburse_allocated_total overflow".to_string(),
        })?;

    Ok(RuntimeRfTickReport {
        participant_count: participant_report.total_participant_count,
        reduce_up_bucket_count: reduce_up_report.bucket_count,
        owner_silo_writeback_count: writeback_results.len() as u32,
        disburse_down_result_count: disburse_down_results.len() as u32,
        local_allocation_count: local_allocation_report.allocation_count,

        surplus_total,
        deficit_total,
        writeback_allocated_total,
        disburse_allocated_total,
        local_allocated_total: local_allocation_report.allocated_total,
        local_unmet_total: local_allocation_report.unmet_total,

        participant_admission_ready,
        reduce_up_ready,
        owner_silo_writeback_ready,
        owner_silo_disburse_down_ready,
        runtime_local_allocation_ready,

        economy_execution_deferred: false,
        scenario_authority_mutation_deferred: true,
        local_effect_application_deferred: true,

        participant_report,
        reduce_up_report,
        writeback_results,
        disburse_down_results,
        local_allocation_report,

        errors,
        deferrals,
    })
}

fn default_deferrals() -> Vec<RuntimeRfTickDeferral> {
    vec![
        RuntimeRfTickDeferral {
            kind: RuntimeRfTickDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated by runtime RF tick report".to_string(),
        },
        RuntimeRfTickDeferral {
            kind: RuntimeRfTickDeferralKind::LocalEffectApplicationDeferred,
            reason: "local participant consumption/supply effects remain deferred".to_string(),
        },
        RuntimeRfTickDeferral {
            kind: RuntimeRfTickDeferralKind::StudioPresentationDeferred,
            reason: "Studio RF tick presentation remains deferred".to_string(),
        },
    ]
}
