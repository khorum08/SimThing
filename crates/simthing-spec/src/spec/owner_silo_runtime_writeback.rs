//! OWNER-SILO-RUNTIME-WRITEBACK-0 — runtime owner-silo writeback from scoped planet RF reduce-up.
//!
//! Pure oracle functions only; mutable runtime state lives outside Scenario authority.

use std::collections::BTreeMap;

use simthing_core::SimThing;

use super::channel_key::{OwnerRef, ResourceKey};
use super::planet_child_rf::{
    planet_child_rf_default_resource_key, PlanetChildRfAdmissionClassification,
    PlanetChildRfReduceUpReport, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};
use super::scenario::{
    game_session_owners, owner_entity_id, owner_has_silo_metadata, owner_silo_capacity,
    owner_silo_current, property_u32, SimThingScenarioSpec, OWNER_SILO_CAPACITY_PROPERTY_ID,
    OWNER_SILO_CURRENT_PROPERTY_ID,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeOwnerSiloWritebackErrorKind {
    RejectedReduceUp,
    MissingOwnerSiloMetadata,
    InvalidOwnerSiloAmount,
    UnknownOwnerReference,
    ArithmeticOverflow,
    EmptyReduceUp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloWritebackError {
    pub kind: RuntimeOwnerSiloWritebackErrorKind,
    pub owner_ref: Option<OwnerRef>,
    pub resource_key: Option<ResourceKey>,
    pub message: String,
}

/// Runtime-resident owner-silo channel state (not Scenario authority).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloState {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub current: u32,
    pub capacity: Option<u32>,
}

/// Aggregated owner/resource writeback input derived from planet-local reduce-up buckets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloWritebackInput {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub net_surplus: u32,
    pub net_deficit: u32,
    pub source_bucket_count: u32,
}

/// Deterministic runtime writeback outcome for one owner/resource channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOwnerSiloWritebackResult {
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub previous_current: u32,
    pub next_current: u32,
    pub capacity: Option<u32>,
    pub applied_surplus: u32,
    pub applied_deficit: u32,
    pub clamped_surplus: u32,
    pub unmet_deficit: u32,
}

/// Derive runtime initial owner-silo states from Scenario Owner metadata (read-only).
pub fn runtime_owner_silo_states_from_scenario(
    spec: &SimThingScenarioSpec,
) -> Result<Vec<RuntimeOwnerSiloState>, RuntimeOwnerSiloWritebackError> {
    let owners = game_session_owners(spec).map_err(|_| RuntimeOwnerSiloWritebackError {
        kind: RuntimeOwnerSiloWritebackErrorKind::MissingOwnerSiloMetadata,
        owner_ref: None,
        resource_key: None,
        message: "GameSession owners unavailable".to_string(),
    })?;

    let mut states = Vec::new();
    for owner in owners {
        let Some(owner_ref_str) = owner_entity_id(owner) else {
            continue;
        };
        let owner_ref = OwnerRef::new(&owner_ref_str);
        if !owner_has_silo_metadata(owner) {
            continue;
        }

        let current = read_required_silo_amount(
            owner,
            OWNER_SILO_CURRENT_PROPERTY_ID,
            &owner_ref_str,
            "owner_silo_current",
        )?;
        let capacity = match owner.properties.get(&OWNER_SILO_CAPACITY_PROPERTY_ID) {
            None => None,
            Some(value) => Some(read_required_silo_amount(
                owner,
                OWNER_SILO_CAPACITY_PROPERTY_ID,
                &owner_ref_str,
                "owner_silo_capacity",
            )?),
        };

        if let Some(capacity) = capacity {
            if current > capacity {
                return Err(RuntimeOwnerSiloWritebackError {
                    kind: RuntimeOwnerSiloWritebackErrorKind::InvalidOwnerSiloAmount,
                    owner_ref: Some(owner_ref.clone()),
                    resource_key: Some(planet_child_rf_default_resource_key()),
                    message: format!(
                        "owner_silo_current ({current}) exceeds owner_silo_capacity ({capacity})"
                    ),
                });
            }
        }

        states.push(RuntimeOwnerSiloState {
            owner_ref,
            resource_key: planet_child_rf_default_resource_key(),
            current,
            capacity,
        });
    }

    states.sort_by(|a, b| (&a.owner_ref, &a.resource_key).cmp(&(&b.owner_ref, &b.resource_key)));
    Ok(states)
}

/// Aggregate planet-local reduce-up buckets into owner/resource writeback inputs.
pub fn owner_silo_writeback_inputs_from_planet_child_reduce_up(
    reduce_up: &PlanetChildRfReduceUpReport,
) -> Result<Vec<RuntimeOwnerSiloWritebackInput>, RuntimeOwnerSiloWritebackError> {
    if reduce_up.classification == PlanetChildRfAdmissionClassification::Rejected {
        return Err(RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::RejectedReduceUp,
            owner_ref: None,
            resource_key: None,
            message: "planet child RF reduce-up rejected".to_string(),
        });
    }
    if !reduce_up.errors.is_empty() {
        return Err(RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::RejectedReduceUp,
            owner_ref: None,
            resource_key: None,
            message: "planet child RF reduce-up contains errors".to_string(),
        });
    }
    if reduce_up.buckets.is_empty() {
        return Err(RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::EmptyReduceUp,
            owner_ref: None,
            resource_key: None,
            message: "no reduce-up buckets to write back".to_string(),
        });
    }

    let mut grouped: BTreeMap<(OwnerRef, ResourceKey), (u64, u64, u32)> = BTreeMap::new();
    for bucket in &reduce_up.buckets {
        if bucket.net_surplus < 0 || bucket.net_deficit < 0 {
            return Err(RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
                owner_ref: Some(bucket.scope.owner_ref.clone()),
                resource_key: Some(bucket.scope.resource_key.clone()),
                message: "bucket net values must be non-negative".to_string(),
            });
        }
        let key = (
            bucket.scope.owner_ref.clone(),
            bucket.scope.resource_key.clone(),
        );
        let entry = grouped.entry(key).or_insert((0, 0, 0));
        entry.0 = entry
            .0
            .checked_add(bucket.net_surplus as u64)
            .ok_or_else(|| RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
                owner_ref: Some(bucket.scope.owner_ref.clone()),
                resource_key: Some(bucket.scope.resource_key.clone()),
                message: "net_surplus aggregate overflow".to_string(),
            })?;
        entry.1 = entry
            .1
            .checked_add(bucket.net_deficit as u64)
            .ok_or_else(|| RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
                owner_ref: Some(bucket.scope.owner_ref.clone()),
                resource_key: Some(bucket.scope.resource_key.clone()),
                message: "net_deficit aggregate overflow".to_string(),
            })?;
        entry.2 += 1;
    }

    let mut inputs = Vec::with_capacity(grouped.len());
    for ((owner_ref, resource_key), (net_surplus, net_deficit, source_bucket_count)) in grouped {
        let net_surplus =
            u32::try_from(net_surplus).map_err(|_| RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
                owner_ref: Some(owner_ref.clone()),
                resource_key: Some(resource_key.clone()),
                message: "aggregated net_surplus exceeds u32".to_string(),
            })?;
        let net_deficit =
            u32::try_from(net_deficit).map_err(|_| RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
                owner_ref: Some(owner_ref.clone()),
                resource_key: Some(resource_key.clone()),
                message: "aggregated net_deficit exceeds u32".to_string(),
            })?;
        inputs.push(RuntimeOwnerSiloWritebackInput {
            owner_ref,
            resource_key,
            net_surplus,
            net_deficit,
            source_bucket_count,
        });
    }
    Ok(inputs)
}

/// Apply checked runtime writeback to owner/resource channels without mutating Scenario authority.
pub fn apply_owner_silo_runtime_writeback_cpu(
    initial: &[RuntimeOwnerSiloState],
    inputs: &[RuntimeOwnerSiloWritebackInput],
) -> Result<Vec<RuntimeOwnerSiloWritebackResult>, RuntimeOwnerSiloWritebackError> {
    let mut state_map: BTreeMap<(OwnerRef, ResourceKey), RuntimeOwnerSiloState> = BTreeMap::new();
    for state in initial {
        state_map.insert(
            (state.owner_ref.clone(), state.resource_key.clone()),
            state.clone(),
        );
    }

    let mut results = Vec::with_capacity(inputs.len());
    for input in inputs {
        let key = (input.owner_ref.clone(), input.resource_key.clone());
        let Some(state) = state_map.get(&key) else {
            return Err(RuntimeOwnerSiloWritebackError {
                kind: RuntimeOwnerSiloWritebackErrorKind::UnknownOwnerReference,
                owner_ref: Some(input.owner_ref.clone()),
                resource_key: Some(input.resource_key.clone()),
                message: "writeback input references unknown owner/resource channel".to_string(),
            });
        };

        let previous_current = state.current;
        let capacity = state.capacity;
        let result = apply_single_writeback(
            &input.owner_ref,
            &input.resource_key,
            previous_current,
            capacity,
            input.net_surplus,
            input.net_deficit,
        )?;
        results.push(result);

        if let Some(runtime) = state_map.get_mut(&key) {
            runtime.current = results.last().expect("result").next_current;
        }
    }

    results.sort_by(|a, b| (&a.owner_ref, &a.resource_key).cmp(&(&b.owner_ref, &b.resource_key)));
    Ok(results)
}

fn apply_single_writeback(
    owner_ref: &OwnerRef,
    resource_key: &ResourceKey,
    previous_current: u32,
    capacity: Option<u32>,
    net_surplus: u32,
    net_deficit: u32,
) -> Result<RuntimeOwnerSiloWritebackResult, RuntimeOwnerSiloWritebackError> {
    let after_surplus = previous_current.checked_add(net_surplus).ok_or_else(|| {
        RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::ArithmeticOverflow,
            owner_ref: Some(owner_ref.clone()),
            resource_key: Some(resource_key.clone()),
            message: "previous_current + net_surplus overflow".to_string(),
        }
    })?;

    let (intermediate, clamped_surplus) = match capacity {
        Some(cap) if after_surplus > cap => (cap, after_surplus - cap),
        _ => (after_surplus, 0),
    };
    let applied_surplus = net_surplus.saturating_sub(clamped_surplus);

    let (next_current, applied_deficit, unmet_deficit) = if intermediate >= net_deficit {
        (intermediate - net_deficit, net_deficit, 0)
    } else {
        (0, intermediate, net_deficit - intermediate)
    };

    Ok(RuntimeOwnerSiloWritebackResult {
        owner_ref: owner_ref.clone(),
        resource_key: resource_key.clone(),
        previous_current,
        next_current,
        capacity,
        applied_surplus,
        applied_deficit,
        clamped_surplus,
        unmet_deficit,
    })
}

fn read_required_silo_amount(
    owner: &SimThing,
    property_id: simthing_core::SimPropertyId,
    owner_ref: &str,
    label: &str,
) -> Result<u32, RuntimeOwnerSiloWritebackError> {
    let Some(value) = owner.properties.get(&property_id) else {
        return Err(RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::MissingOwnerSiloMetadata,
            owner_ref: Some(OwnerRef::new(owner_ref)),
            resource_key: Some(planet_child_rf_default_resource_key()),
            message: format!("{label} is required for runtime owner-silo writeback"),
        });
    };
    match property_u32(value) {
        Some(amount) => Ok(amount),
        None => Err(RuntimeOwnerSiloWritebackError {
            kind: RuntimeOwnerSiloWritebackErrorKind::InvalidOwnerSiloAmount,
            owner_ref: Some(OwnerRef::new(owner_ref)),
            resource_key: Some(planet_child_rf_default_resource_key()),
            message: format!("{label} must be a non-negative exact integer f32 mirror"),
        }),
    }
}

/// Convenience reader for tests and driver compile without duplicating property access.
pub fn read_owner_silo_current_from_owner(owner: &SimThing) -> Option<u32> {
    owner_silo_current(owner)
}

pub fn read_owner_silo_capacity_from_owner(owner: &SimThing) -> Option<u32> {
    owner_silo_capacity(owner)
}
