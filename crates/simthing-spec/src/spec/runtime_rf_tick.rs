//! RUNTIME-RF-TICK-INTEGRATION-0 — composed runtime RF tick boundary report.
//!
//! Composes participant admission → reduce-up → writeback → disburse-down → local allocation.

use super::owner_silo_disburse_down::{
    apply_owner_silo_runtime_disburse_down_cpu, owner_silo_demand_buckets_from_planet_child_rf,
    RuntimeOwnerSiloDisburseDownResult,
};
use super::owner_silo_runtime_writeback::{
    apply_owner_silo_runtime_writeback_cpu,
    owner_silo_writeback_inputs_from_planet_child_reduce_up,
    runtime_owner_silo_states_from_scenario, RuntimeOwnerSiloWritebackResult,
};
use super::planet_child_rf::{
    evaluate_planet_child_rf_admission, evaluate_planet_child_rf_reduce_up,
    PlanetChildRfAdmissionClassification, PlanetChildRfAdmissionReport,
    PlanetChildRfReduceUpReport,
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

/// Evaluate the full runtime RF tick boundary from Scenario authority input (read-only).
pub fn evaluate_runtime_rf_tick(
    scenario: &SimThingScenarioSpec,
) -> Result<RuntimeRfTickReport, RuntimeRfTickError> {
    let deferrals = default_deferrals();
    let errors = Vec::new();

    let participant_report = evaluate_planet_child_rf_admission(scenario);
    if participant_report.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::ParticipantAdmissionRejected,
            message: "planet child RF participant admission rejected".to_string(),
        });
    }
    let participant_admission_ready = participant_report.classification
        != PlanetChildRfAdmissionClassification::Unsupported
        && participant_report.total_participant_count > 0;

    let reduce_up_report = evaluate_planet_child_rf_reduce_up(scenario);
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

    let demand_buckets = owner_silo_demand_buckets_from_planet_child_rf(scenario).map_err(|e| {
        RuntimeRfTickError {
            kind: RuntimeRfTickErrorKind::DisburseDownRejected,
            message: e.message,
        }
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

        economy_execution_deferred: true,
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
            kind: RuntimeRfTickDeferralKind::EconomyExecutionDeferred,
            reason: "full economy execution remains deferred".to_string(),
        },
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
