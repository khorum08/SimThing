//! OWNER-SILO-DISBURSE-DOWN-0 — runtime owner-silo disburse-down allocation oracle.
//!
//! Pure oracle functions only; mutable runtime state and Scenario authority remain unchanged.

use std::collections::BTreeMap;

use simthing_core::SimThing;

use super::channel_key::{OwnerRef, ResourceKey, ScopeId};
use super::owner_silo_runtime_writeback::RuntimeOwnerSiloWritebackResult;
use super::planet_child_location::{
    is_admitted_planet_non_grid_child, planet_id, planet_non_grid_child_kind_label,
    planet_non_grid_child_owner_ref, planet_owner_ref, star_system_gridcells,
};
use super::planet_child_rf::{
    planet_child_rf_default_resource_key, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
use super::scenario::{
    game_session_galaxy_map, game_session_owners, owner_entity_id, owner_flow_owner_ref,
    property_u32, SimThingScenarioSpec, OWNER_FLOW_DEFAULT_PRIORITY, OWNER_FLOW_DEMAND_PROPERTY_ID,
    OWNER_FLOW_PRIORITY_PROPERTY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeOwnerSiloDisburseDownErrorKind {
    RejectedWriteback,
    MissingOwnerChannelForActiveDemand,
    InvalidDemandAmount,
    InvalidPriorityAmount,
    UnknownOwnerReference,
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
    pub planet_id: Option<String>,
    pub star_system_gridcell_id_raw: Option<u32>,
    pub requested: u32,
    pub priority: u32,
    pub source_simthing_id_raw: Option<u32>,
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
    pub planet_id: Option<String>,
    pub star_system_gridcell_id_raw: Option<u32>,
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
    let owner_refs: std::collections::BTreeSet<OwnerRef> = game_session_owners(scenario)
        .map_err(|_| RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::MissingOwnerChannelForActiveDemand,
            owner_ref: None,
            resource_key: None,
            scope_id: None,
            message: "GameSession owners unavailable".to_string(),
        })?
        .into_iter()
        .filter_map(owner_entity_id)
        .map(OwnerRef::new)
        .collect();

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
                &planet_scope,
                Some(planet_scope.clone()),
                Some(star_raw),
                &planet_path,
                &owner_refs,
                true,
                &mut buckets,
            )?;

            for child in super::planet_child_location::planet_gameplay_children(planet) {
                if !is_admitted_planet_non_grid_child(&child.kind) {
                    if has_active_demand_metadata(child) {
                        return Err(RuntimeOwnerSiloDisburseDownError {
                            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidDemandAmount,
                            owner_ref: owner_flow_owner_ref(child).map(OwnerRef::new),
                            resource_key: Some(planet_child_rf_default_resource_key()),
                            scope_id: Some(ScopeId::new(&planet_scope)),
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
                    &planet_scope,
                    Some(planet_scope.clone()),
                    Some(star_raw),
                    &child_path,
                    &owner_refs,
                    false,
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
    scope_id: &str,
    planet_id: Option<String>,
    star_system_gridcell_id_raw: Option<u32>,
    path: &str,
    owner_refs: &std::collections::BTreeSet<OwnerRef>,
    is_planet_gridcell: bool,
    buckets: &mut Vec<RuntimeOwnerSiloDemandBucket>,
) -> Result<(), RuntimeOwnerSiloDisburseDownError> {
    let requested = read_demand_amount(node, path)?;
    if requested == 0 {
        return Ok(());
    }

    let owner_ref = resolve_demand_owner_ref(node, is_planet_gridcell);
    let Some(owner_ref) = owner_ref else {
        return Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::MissingOwnerChannelForActiveDemand,
            owner_ref: None,
            resource_key: Some(planet_child_rf_default_resource_key()),
            scope_id: Some(ScopeId::new(scope_id)),
            message: format!("active demand at {path} requires owner/channel reference"),
        });
    };
    if owner_ref.trim().is_empty() {
        return Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::MissingOwnerChannelForActiveDemand,
            owner_ref: None,
            resource_key: Some(planet_child_rf_default_resource_key()),
            scope_id: Some(ScopeId::new(scope_id)),
            message: format!("active demand at {path} has empty owner/channel reference"),
        });
    }
    let owner_ref = OwnerRef::new(&owner_ref);
    if !owner_refs.contains(&owner_ref) {
        return Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::UnknownOwnerReference,
            owner_ref: Some(owner_ref),
            resource_key: Some(planet_child_rf_default_resource_key()),
            scope_id: Some(ScopeId::new(scope_id)),
            message: format!("unknown owner/channel reference at {path}"),
        });
    }

    let priority = read_priority_amount(node, path)?;

    buckets.push(RuntimeOwnerSiloDemandBucket {
        owner_ref,
        resource_key: planet_child_rf_default_resource_key(),
        scope_id: ScopeId::new(scope_id),
        planet_id,
        star_system_gridcell_id_raw,
        requested,
        priority,
        source_simthing_id_raw: Some(node.id.raw()),
    });
    Ok(())
}

fn resolve_demand_owner_ref(node: &SimThing, is_planet_gridcell: bool) -> Option<String> {
    if is_planet_gridcell {
        planet_owner_ref(node).or_else(|| owner_flow_owner_ref(node))
    } else {
        planet_non_grid_child_owner_ref(node)
    }
}

fn has_active_demand_metadata(node: &SimThing) -> bool {
    node.properties.contains_key(&OWNER_FLOW_DEMAND_PROPERTY_ID)
}

fn read_demand_amount(
    node: &SimThing,
    path: &str,
) -> Result<u32, RuntimeOwnerSiloDisburseDownError> {
    let Some(value) = node.properties.get(&OWNER_FLOW_DEMAND_PROPERTY_ID) else {
        return Ok(0);
    };
    match property_u32(value) {
        Some(amount) => Ok(amount),
        None => Err(RuntimeOwnerSiloDisburseDownError {
            kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidDemandAmount,
            owner_ref: owner_flow_owner_ref(node).map(OwnerRef::new),
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
) -> Result<u32, RuntimeOwnerSiloDisburseDownError> {
    match node.properties.get(&OWNER_FLOW_PRIORITY_PROPERTY_ID) {
        None => Ok(OWNER_FLOW_DEFAULT_PRIORITY),
        Some(value) => match property_u32(value) {
            Some(amount) => Ok(amount),
            None => Err(RuntimeOwnerSiloDisburseDownError {
                kind: RuntimeOwnerSiloDisburseDownErrorKind::InvalidPriorityAmount,
                owner_ref: owner_flow_owner_ref(node).map(OwnerRef::new),
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
                planet_id: demand.planet_id,
                star_system_gridcell_id_raw: demand.star_system_gridcell_id_raw,
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
