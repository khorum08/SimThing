//! RUNTIME-TICK-EXECUTION-SHELL-0 — deterministic runtime tick shell over composed RF reports.

use super::runtime_rf_tick::evaluate_runtime_rf_tick;
use super::scenario::SimThingScenarioSpec;

/// Deterministic runtime tick identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeTickId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTickStage {
    RuntimeRfTickComposition,
    ParticipantAdmission,
    ReduceUp,
    OwnerSiloWriteback,
    OwnerSiloDisburseDown,
    RuntimeLocalAllocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTickShellErrorKind {
    RuntimeRfTickRejected,
    InvalidTickId,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickShellError {
    pub kind: RuntimeTickShellErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeTickShellDeferralKind {
    EconomyExecutionDeferred,
    ScenarioAuthorityMutationDeferred,
    LocalEffectApplicationDeferred,
    StudioPresentationDeferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTickShellDeferral {
    pub kind: RuntimeTickShellDeferralKind,
    pub reason: String,
}

/// Proof/report-only runtime tick execution boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTickExecutionReport {
    pub tick_id: RuntimeTickId,
    pub stage_count: u32,
    pub stage_order: Vec<RuntimeTickStage>,
    pub runtime_rf_tick_ready: bool,
    pub participant_count: u32,
    pub reduce_up_bucket_count: u32,
    pub owner_silo_writeback_count: u32,
    pub disburse_down_result_count: u32,
    pub local_allocation_count: u32,
    pub local_allocated_total: u32,
    pub local_unmet_total: u32,
    pub gpu_stage_proof_available: bool,
    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
    pub local_effect_application_deferred: bool,
    pub errors: Vec<RuntimeTickShellError>,
    pub deferrals: Vec<RuntimeTickShellDeferral>,
}

/// Deterministic stage order for the composed RF tick boundary.
pub fn runtime_tick_shell_stage_order() -> Vec<RuntimeTickStage> {
    vec![
        RuntimeTickStage::RuntimeRfTickComposition,
        RuntimeTickStage::ParticipantAdmission,
        RuntimeTickStage::ReduceUp,
        RuntimeTickStage::OwnerSiloWriteback,
        RuntimeTickStage::OwnerSiloDisburseDown,
        RuntimeTickStage::RuntimeLocalAllocation,
    ]
}

/// Evaluate the runtime tick execution shell from Scenario authority input (read-only).
pub fn evaluate_runtime_tick_shell(
    scenario: &SimThingScenarioSpec,
    tick_id: RuntimeTickId,
) -> Result<RuntimeTickExecutionReport, RuntimeTickShellError> {
    if tick_id.0 == 0 {
        return Err(RuntimeTickShellError {
            kind: RuntimeTickShellErrorKind::InvalidTickId,
            message: "tick id must be non-zero".to_string(),
        });
    }

    let tick_report = evaluate_runtime_rf_tick(scenario).map_err(|e| RuntimeTickShellError {
        kind: RuntimeTickShellErrorKind::RuntimeRfTickRejected,
        message: format!("{:?}: {}", e.kind, e.message),
    })?;

    let stage_order = runtime_tick_shell_stage_order();
    let runtime_rf_tick_ready = tick_report.participant_admission_ready
        && tick_report.reduce_up_ready
        && tick_report.owner_silo_writeback_ready
        && tick_report.owner_silo_disburse_down_ready
        && tick_report.runtime_local_allocation_ready;

    if !runtime_rf_tick_ready {
        return Err(RuntimeTickShellError {
            kind: RuntimeTickShellErrorKind::RuntimeRfTickRejected,
            message: "composed runtime RF tick report is not ready".to_string(),
        });
    }

    Ok(RuntimeTickExecutionReport {
        tick_id,
        stage_count: stage_order.len() as u32,
        stage_order,
        runtime_rf_tick_ready,
        participant_count: tick_report.participant_count,
        reduce_up_bucket_count: tick_report.reduce_up_bucket_count,
        owner_silo_writeback_count: tick_report.owner_silo_writeback_count,
        disburse_down_result_count: tick_report.disburse_down_result_count,
        local_allocation_count: tick_report.local_allocation_count,
        local_allocated_total: tick_report.local_allocated_total,
        local_unmet_total: tick_report.local_unmet_total,
        gpu_stage_proof_available: false,
        economy_execution_deferred: true,
        scenario_authority_mutation_deferred: true,
        local_effect_application_deferred: true,
        errors: Vec::new(),
        deferrals: default_deferrals(),
    })
}

fn default_deferrals() -> Vec<RuntimeTickShellDeferral> {
    vec![
        RuntimeTickShellDeferral {
            kind: RuntimeTickShellDeferralKind::EconomyExecutionDeferred,
            reason: "full economy execution remains deferred".to_string(),
        },
        RuntimeTickShellDeferral {
            kind: RuntimeTickShellDeferralKind::ScenarioAuthorityMutationDeferred,
            reason: "Scenario authority is not mutated by runtime tick shell".to_string(),
        },
        RuntimeTickShellDeferral {
            kind: RuntimeTickShellDeferralKind::LocalEffectApplicationDeferred,
            reason: "local participant consumption/supply effects remain deferred".to_string(),
        },
        RuntimeTickShellDeferral {
            kind: RuntimeTickShellDeferralKind::StudioPresentationDeferred,
            reason: "Studio runtime tick shell presentation remains deferred".to_string(),
        },
    ]
}
