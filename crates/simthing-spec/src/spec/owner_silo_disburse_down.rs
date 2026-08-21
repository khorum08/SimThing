//! OWNER-SILO-DISBURSE-DOWN-0 — runtime owner-silo disburse-down allocation oracle.
//!
//! Pure oracle functions only; mutable runtime state and Scenario authority remain unchanged.

use std::collections::BTreeMap;

use simthing_core::SimThing;

use super::channel_key::{OwnerChannelScopeKey, OwnerRef, ResourceKey, ScopeId};
use super::legacy_owner_channel_rf::planet_child_rf_default_resource_key;
use super::owner_channel_admission::{admit_intrinsic_owner_channels, IntrinsicOwnerChannelView};
use super::owner_silo_runtime_writeback::RuntimeOwnerSiloWritebackResult;
use super::planet_child_location::{
    is_admitted_planet_non_grid_child, planet_id, planet_non_grid_child_kind_label,
    star_system_gridcells,
};
use super::scenario::{
    game_session_galaxy_map, property_u32, SimThingScenarioSpec, OWNER_FLOW_DEFAULT_PRIORITY,
    OWNER_FLOW_DEMAND_PROPERTY_ID, OWNER_FLOW_PRIORITY_PROPERTY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeOwnerSiloDisburseDownErrorKind {
    RejectedWriteback,
    MissingOwnerChannelForActiveDemand,
    InvalidDemandAmount,
    InvalidPriorityAmount,
    UnknownOwnerReference,
    InvalidOwnerAuthority,
    ArithmeticOverflow,
    EmptyWriteback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloDisburseDownError {
    pub kind: RuntimeOwnerSiloDisburseDownErrorKind,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub scope_id: Option<ScopeId>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloDemandBucket {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
    pub requested: u32,
    pub priority: u32,
    pub source_simthing_id_raw: Option<u32>,
}

impl RuntimeOwnerSiloDemandBucket {
    /// Return the already-admitted RF scope without reconstructing ownership.
    pub fn scope_key(&self) -> OwnerChannelScopeKey {
        OwnerChannelScopeKey {
            owner_ref: self.owner_ref.clone(),
            resource_key: self.resource_key.clone(),
            scope_id: self.scope_id.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloDisburseDownInput {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub available: u32,
    pub demands: Vec<RuntimeOwnerSiloDemandBucket>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloDisburseDownAllocation {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
    pub requested: u32,
    pub allocated: u32,
    pub unmet: u32,
    pub priority: u32,
    pub source_simthing_id_raw: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloDisburseDownResult {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub available_before: u32,
    pub allocated_total: u32,
    pub remaining_after: u32,
    pub unmet_total: u32,
    pub allocations: Vec<RuntimeOwnerSiloDisburseDownAllocation>,
}

/// Derive local demand buckets from admitted planet gridcells and non-grid child SimThings.
pub fn owner_silo_demand_buckets_from_planet_child_rf(
    scenario: &SimThingScenarioSpec,
) -> Result<Vec<RuntimeOwnerSiloDemandBucket>, RuntimeOwnerSiloDisburseDownError> {
    let owner_view = admit_intrinsic_owner_channels(scenario).map_err(|error| {
        RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidOwnerAuthority,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            message: error.to_string(),
        }
    })?;
    owner_silo_demand_buckets_from_owner_view(&owner_view)
}

pub fn owner_silo_demand_buckets_from_owner_view(
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<Vec<RuntimeOwnerSiloDemandBucket>, RuntimeOwnerSiloDisburseDownError> {
    let scenario = owner_view.scenario();

    let _galaxy_map =
        game_session_galaxy_map(scenario).map_err(|_| RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::MissingOwnerChannelForActiveDemand,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            message: "galaxy map unavailable for demand derivation".to_string(),
        })?;

    let mut buckets = Vec::new();
    for star_system in
        star_system_gridcells(scenario).map_err(|_| RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::MissingOwnerChannelForActiveDemand,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            message: "star-system gridcells unavailable".to_string(),
        })?
    {
        let star_raw = star_system.id.raw();
        let star_path = format!("galaxymap/star_system/{star_raw}");
        for planet in super::planet_child_location::planet_gridcells(scenario, star_system) {
            let planet_scope = planet_id(planet).unwrap_or_else(|| planet.id.raw().to_string());
            let planet_path = format!("{star_path}/planet/{planet_scope}");
            collect_demand_from_node(
                planet,
                &ScopeId::from_boundary(planet.id),
                &planet_path,
                owner_view,
                &mut buckets,
            )?;

            for child in super::planet_child_location::planet_gameplay_children(planet) {
                if !is_admitted_planet_non_grid_child(&child.kind) {
                    if has_active_demand_metadata(child) {
                        return Err(RuntimeOwnerSiloDisburseDownError {
                            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidDemandAmount,
                            owner_ref: owner_view.owner_for(child.id).ok().cloned(),
                            resource_key: Some(planet_child_rf_default_resource_key()),
                            scope_id: Some(ScopeId::from_boundary(planet.id)),
                            message: format!(
                                "unsupported non-grid child kind {:?} cannot express disburse-down demand",
                                child.kind
                            ),
                        });
                    }
                    continue;
                }
                let child_path = format!(
                    "{}/child/{}/{}",
                    planet_path,
                    planet_non_grid_child_kind_label(&child.kind),
                    child.id.raw()
                );
                collect_demand_from_node(
                    child,
                    &ScopeId::from_boundary(planet.id),
                    &child_path,
                    owner_view,
                    &mut buckets,
                )?;
            }
        }
    }

    buckets.sort_by(demand_bucket_sort_key);
    Ok(buckets)
}

fn collect_demand_from_node(
    node: &SimThing,
    scope_id: &ScopeId,
    path: &str,
    owner_view: &IntrinsicOwnerChannelView,
    buckets: &mut Vec<RuntimeOwnerSiloDemandBucket>,
) -> Result<(), RuntimeOwnerSiloDisburseDownError> {
    let requested = read_demand_amount(node, path, owner_view)?;
    if requested == 0 {
        return Ok(());
    }

    let owner_ref = owner_view
        .owner_for(node.id)
        .map_err(|error| RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidOwnerAuthority,
            owner_ref: None,
            resource_key: Some(planet_child_rf_default_resource_key()),
            scope_id: Some(scope_id.clone()),
            message: format!("invalid owner authority at {path}: {error}"),
        })?
        .clone();

    let priority = read_priority_amount(node, path, owner_view)?;

    buckets.push(RuntimeOwnerSiloDemandBucket {
        owner_ref,
        resource_key: planet_child_rf_default_resource_key(),
        scope_id: scope_id.clone(),
        requested,
        priority,
        source_simthing_id_raw: Some(node.id.raw()),
    });
    Ok(())
}

fn has_active_demand_metadata(node: &SimThing) -> bool {
    node.properties.contains_key(&OWNER_FLOW_DEMAND_PROPERTY_ID)
}

fn read_demand_amount(
    node: &SimThing,
    path: &str,
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<u32, RuntimeOwnerSiloDisburseDownError> {
    let Some(value) = node.properties.get(&OWNER_FLOW_DEMAND_PROPERTY_ID) else {
        return Ok(0);
    };
    match property_u32(value) {
        Some(amount) => Ok(amount),
        None => Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidDemandAmount,
            owner_ref: owner_view.owner_for(node.id).ok().cloned(),
            resource_key: Some(planet_child_rf_default_resource_key()),
            scope_id: None,
            message: format!(
                "owner_flow_demand at {path} must be a non-negative exact integer f32 mirror"
            ),
        }),
    }
}

fn read_priority_amount(
    node: &SimThing,
    path: &str,
    owner_view: &IntrinsicOwnerChannelView,
) -> Result<u32, RuntimeOwnerSiloDisburseDownError> {
    match node.properties.get(&OWNER_FLOW_PRIORITY_PROPERTY_ID) {
        None => Ok(OWNER_FLOW_DEFAULT_PRIORITY),
        Some(value) => match property_u32(value) {
            Some(amount) => Ok(amount),
            None => Err(RuntimeOwnerSiloDisburseDownError {
                kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidPriorityAmount,
                owner_ref: owner_view.owner_for(node.id).ok().cloned(),
                resource_key: Some(planet_child_rf_default_resource_key()),
                scope_id: None,
                message: format!(
                    "owner_flow_priority at {path} must be a non-negative exact integer f32 mirror"
                ),
            }),
        },
    }
}

pub(crate) fn demand_bucket_sort_key(
    a: &RuntimeOwnerSiloDemandBucket,
    b: &RuntimeOwnerSiloDemandBucket,
) -> std::cmp::Ordering {
    (
        &a.owner_ref,
        &a.resource_key,
        a.priority,
        &a.scope_id,
        a.source_simthing_id_raw,
    )
        .cmp(&(
            &b.owner_ref,
            &b.resource_key,
            b.priority,
            &b.scope_id,
            b.source_simthing_id_raw,
        ))
}

/// Allocate runtime owner-silo availability to local demand buckets without mutating Scenario authority.
pub fn apply_owner_silo_runtime_disburse_down_cpu(
    writeback_results: &[RuntimeOwnerSiloWritebackResult],
    demand_buckets: &[RuntimeOwnerSiloDemandBucket],
) -> Result<Vec<RuntimeOwnerSiloDisburseDownResult>, RuntimeOwnerSiloDisburseDownError> {
    if writeback_results.is_empty() {
        return Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::EmptyWriteback,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            message: "no writeback results to disburse from".to_string(),
        });
    }

    let mut availability: BTreeMap<(OwnerRef, ResourceKey), u32> = BTreeMap::new();
    for result in writeback_results {
        availability.insert(
            (result.owner_ref.clone(), result.resource_key.clone()),
            result.next_current,
        );
    }

    let mut grouped: BTreeMap<(OwnerRef, ResourceKey), Vec<RuntimeOwnerSiloDemandBucket>> =
        BTreeMap::new();
    for bucket in demand_buckets {
        grouped
            .entry((bucket.owner_ref.clone(), bucket.resource_key.clone()))
            .or_default()
            .push(bucket.clone());
    }

    let mut results = Vec::new();
    for ((owner_ref, resource_key), mut demands) in grouped {
        let Some(&available_before) = availability.get(&(owner_ref.clone(), resource_key.clone()))
        else {
            return Err(RuntimeOwnerSiloDisburseDownError {
                kind: RuntimeOwnerSiloDisburseDownErrorKind::RejectedWriteback,
                owner_ref: Some(owner_ref),
                resource_key: Some(resource_key),
                scope_id: None,
                message: "demand references owner/resource without writeback availability"
                    .to_string(),
            });
        };

        demands.sort_by(demand_bucket_sort_key);

        let mut remaining = available_before;
        let mut allocations = Vec::with_capacity(demands.len());
        let mut allocated_total: u32 = 0;
        let mut unmet_total: u32 = 0;

        for demand in demands {
            let allocated = remaining.min(demand.requested);
            let unmet = demand.requested - allocated;
            remaining = remaining.saturating_sub(allocated);
            allocated_total = allocated_total.checked_add(allocated).ok_or_else(|| {
                RuntimeOwnerSiloDisburseDownError {
                    kind: RuntimeOwnerSiloDisburseDownErrorKind::ArithmeticOverflow,
                    owner_ref: Some(demand.owner_ref.clone()),
                    resource_key: Some(demand.resource_key.clone()),
                    scope_id: Some(demand.scope_id.clone()),
                    message: "allocated_total overflow".to_string(),
                }
            })?;
            unmet_total = unmet_total.checked_add(unmet).ok_or_else(|| {
                RuntimeOwnerSiloDisburseDownError {
                    kind: RuntimeOwnerSiloDisburseDownErrorKind::ArithmeticOverflow,
                    owner_ref: Some(demand.owner_ref.clone()),
                    resource_key: Some(demand.resource_key.clone()),
                    scope_id: Some(demand.scope_id.clone()),
                    message: "unmet_total overflow".to_string(),
                }
            })?;

            allocations.push(RuntimeOwnerSiloDisburseDownAllocation {
                owner_ref: demand.owner_ref,
                resource_key: demand.resource_key,
                scope_id: demand.scope_id.clone(),
                requested: demand.requested,
                allocated,
                unmet,
                priority: demand.priority,
                source_simthing_id_raw: demand.source_simthing_id_raw,
            });
        }

        results.push(RuntimeOwnerSiloDisburseDownResult {
            owner_ref,
            resource_key,
            available_before,
            allocated_total,
            remaining_after: remaining,
            unmet_total,
            allocations,
        });
    }

    results.sort_by(|a, b| (&a.owner_ref, &a.resource_key).cmp(&(&b.owner_ref, &b.resource_key)));
    Ok(results)
}

/// Aggregate requested demand per owner/resource for GPU proof comparison.
pub fn owner_silo_demand_aggregate_totals(
    demand_buckets: &[RuntimeOwnerSiloDemandBucket],
) -> BTreeMap<(OwnerRef, ResourceKey), u32> {
    let mut totals: BTreeMap<(OwnerRef, ResourceKey), u32> = BTreeMap::new();
    for bucket in demand_buckets {
        let entry = totals
            .entry((bucket.owner_ref.clone(), bucket.resource_key.clone()))
            .or_insert(0);
        *entry = entry.saturating_add(bucket.requested);
    }
    totals
}
