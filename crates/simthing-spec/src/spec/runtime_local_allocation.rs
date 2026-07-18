//! RUNTIME-LOCAL-ALLOCATION-APPLICATION-0 — runtime-local participant allocation state from disburse-down.

use std::collections::BTreeSet;

use super::channel_key::{OwnerRef, ResourceKey, ScopeId};
use super::owner_silo_disburse_down::RuntimeOwnerSiloDisburseDownResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeLocalAllocationApplicationErrorKind {
    MissingSourceSimThingId,
    DuplicateSourceAllocation,
    ArithmeticOverflow,
    EmptyDisburseDown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLocalAllocationApplicationError {
    pub kind: RuntimeLocalAllocationApplicationErrorKind,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub scope_id: Option<ScopeId>,
    pub source_simthing_id_raw: Option<u32>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLocalAllocationState {
    pub source_simthing_id_raw: u32,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
    pub planet_id: Option<String>,
    pub star_system_gridcell_id_raw: Option<u32>,
    pub requested: u32,
    pub allocated: u32,
    pub unmet: u32,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeLocalAllocationApplicationReport {
    pub allocation_count: u32,
    pub owner_channel_count: u32,
    pub allocated_total: u32,
    pub unmet_total: u32,
    pub states: Vec<RuntimeLocalAllocationState>,
    pub economy_execution_deferred: bool,
    pub scenario_authority_mutation_deferred: bool,
}

/// Map disburse-down allocation results into runtime-local participant allocation state.
pub fn apply_runtime_local_allocations_from_disburse_down(
    disburse_results: &[RuntimeOwnerSiloDisburseDownResult],
) -> Result<RuntimeLocalAllocationApplicationReport, RuntimeLocalAllocationApplicationError> {
    if disburse_results.is_empty() {
        return Ok(RuntimeLocalAllocationApplicationReport {
            allocation_count: 0,
            owner_channel_count: 0,
            allocated_total: 0,
            unmet_total: 0,
            states: Vec::new(),
            economy_execution_deferred: false,
            scenario_authority_mutation_deferred: true,
        });
    }

    let mut states = Vec::new();
    let mut seen_sources = BTreeSet::new();
    let mut owner_channels = BTreeSet::new();
    let mut allocated_total: u32 = 0;
    let mut unmet_total: u32 = 0;

    for result in disburse_results {
        for allocation in &result.allocations {
            if allocation.requested == 0 {
                continue;
            }

            let Some(source_simthing_id_raw) = allocation.source_simthing_id_raw else {
                return Err(RuntimeLocalAllocationApplicationError {
                    kind: RuntimeLocalAllocationApplicationErrorKind::MissingSourceSimThingId,
                    owner_ref: Some(allocation.owner_ref.clone()),
                    resource_key: Some(allocation.resource_key.clone()),
                    scope_id: Some(allocation.scope_id.clone()),
                    source_simthing_id_raw: None,
                    message: "disburse-down allocation requires source_simthing_id_raw".to_string(),
                });
            };

            let dedupe_key = (
                allocation.owner_ref.clone(),
                allocation.resource_key.clone(),
                allocation.scope_id.clone(),
                source_simthing_id_raw,
            );
            if !seen_sources.insert(dedupe_key) {
                return Err(RuntimeLocalAllocationApplicationError {
                    kind: RuntimeLocalAllocationApplicationErrorKind::DuplicateSourceAllocation,
                    owner_ref: Some(allocation.owner_ref.clone()),
                    resource_key: Some(allocation.resource_key.clone()),
                    scope_id: Some(allocation.scope_id.clone()),
                    source_simthing_id_raw: Some(source_simthing_id_raw),
                    message: format!(
                        "duplicate allocation for source SimThing id {source_simthing_id_raw}"
                    ),
                });
            }

            allocated_total = allocated_total
                .checked_add(allocation.allocated)
                .ok_or_else(|| RuntimeLocalAllocationApplicationError {
                    kind: RuntimeLocalAllocationApplicationErrorKind::ArithmeticOverflow,
                    owner_ref: Some(allocation.owner_ref.clone()),
                    resource_key: Some(allocation.resource_key.clone()),
                    scope_id: Some(allocation.scope_id.clone()),
                    source_simthing_id_raw: Some(source_simthing_id_raw),
                    message: "allocated_total overflow".to_string(),
                })?;
            unmet_total = unmet_total.checked_add(allocation.unmet).ok_or_else(|| {
                RuntimeLocalAllocationApplicationError {
                    kind: RuntimeLocalAllocationApplicationErrorKind::ArithmeticOverflow,
                    owner_ref: Some(allocation.owner_ref.clone()),
                    resource_key: Some(allocation.resource_key.clone()),
                    scope_id: Some(allocation.scope_id.clone()),
                    source_simthing_id_raw: Some(source_simthing_id_raw),
                    message: "unmet_total overflow".to_string(),
                }
            })?;

            owner_channels.insert((
                allocation.owner_ref.clone(),
                allocation.resource_key.clone(),
            ));

            states.push(RuntimeLocalAllocationState {
                source_simthing_id_raw,
                owner_ref: allocation.owner_ref.clone(),
                resource_key: allocation.resource_key.clone(),
                scope_id: allocation.scope_id.clone(),
                planet_id: allocation.planet_id.clone(),
                star_system_gridcell_id_raw: allocation.star_system_gridcell_id_raw,
                requested: allocation.requested,
                allocated: allocation.allocated,
                unmet: allocation.unmet,
                priority: allocation.priority,
            });
        }
    }

    states.sort_by(|a, b| {
        (
            &a.owner_ref,
            &a.resource_key,
            &a.scope_id,
            a.source_simthing_id_raw,
        )
            .cmp(&(
                &b.owner_ref,
                &b.resource_key,
                &b.scope_id,
                b.source_simthing_id_raw,
            ))
    });

    let allocation_count =
        u32::try_from(states.len()).map_err(|_| RuntimeLocalAllocationApplicationError {
            kind: RuntimeLocalAllocationApplicationErrorKind::ArithmeticOverflow,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            source_simthing_id_raw: None,
            message: "allocation_count exceeds u32".to_string(),
        })?;

    Ok(RuntimeLocalAllocationApplicationReport {
        allocation_count,
        owner_channel_count: owner_channels.len() as u32,
        allocated_total,
        unmet_total,
        states,
        economy_execution_deferred: false,
        scenario_authority_mutation_deferred: true,
    })
}

/// Aggregate allocated totals per owner/resource for GPU proof comparison.
pub fn runtime_local_allocation_aggregate_totals(
    report: &RuntimeLocalAllocationApplicationReport,
) -> std::collections::BTreeMap<(OwnerRef, ResourceKey), u32> {
    use std::collections::BTreeMap;
    let mut totals: BTreeMap<(OwnerRef, ResourceKey), u32> = BTreeMap::new();
    for state in &report.states {
        let entry = totals
            .entry((state.owner_ref.clone(), state.resource_key.clone()))
            .or_insert(0);
        *entry = entry.saturating_add(state.allocated);
    }
    totals
}
