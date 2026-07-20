//! Canonical CPU-side observation / telemetry / metrics read seam.
//!
//! Consumers resolve a hosted cell by [`SimThingId`] + typed [`PropertyKey`] +
//! [`SubFieldRole`] through the property layout/registry. Raw [`ColumnIndex`]
//! values are never minted or exposed on this door. Future Studio / metrics
//! reads must reuse or extend this module rather than adding feature-specific
//! GPU indexing paths.

use std::collections::BTreeMap;

use simthing_core::{DimensionRegistry, SimThingId, SubFieldRole};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    DisruptionAuthorityReadback, DisruptionAuthorityReadbackError, PropertyKey,
};
use thiserror::Error;

use crate::session::SimSession;

/// One coherent GPU values snapshot (single `read_values` capture).
#[derive(Debug, Clone)]
pub struct GpuValuesSnapshot {
    values: Vec<f32>,
    n_dims: usize,
}

impl GpuValuesSnapshot {
    pub fn from_session(sim: &SimSession) -> Self {
        Self {
            values: sim.state.read_values(),
            n_dims: sim.state.n_dims as usize,
        }
    }

    /// Test/oracle helper: build a coherent snapshot without a live session.
    pub fn from_values_for_test(values: Vec<f32>, n_dims: usize) -> Self {
        Self { values, n_dims }
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }

    pub fn n_dims(&self) -> usize {
        self.n_dims
    }
}

/// Authored/materialized hosted property locus retained for observation.
///
/// `host_entity` is the install-target / location id used to join structural
/// authority (`location_id` / `target_id`) when the runtime host id and Spec
/// placement id spaces diverge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostedPropertyLocus {
    pub host_id: SimThingId,
    pub host_entity: Option<String>,
    pub property: PropertyKey,
    pub role: SubFieldRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HostedPropertyObservationError {
    #[error("unknown property {namespace}::{name}")]
    UnknownProperty { namespace: String, name: String },
    #[error("unknown role {role:?} on property {namespace}::{name}")]
    UnknownRole {
        namespace: String,
        name: String,
        role: SubFieldRole,
    },
    #[error("host {host:?} has no allocated GPU slot")]
    HostHasNoSlot { host: SimThingId },
    #[error("cell out of bounds for host {host:?} property {namespace}::{name}")]
    CellOutOfBounds {
        host: SimThingId,
        namespace: String,
        name: String,
    },
}

/// Canonical read of one hosted property cell from a coherent GPU snapshot.
pub fn observe_hosted_property_cell(
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    snapshot: &GpuValuesSnapshot,
    host: SimThingId,
    property: &PropertyKey,
    role: &SubFieldRole,
) -> Result<f32, HostedPropertyObservationError> {
    let property_id = registry.id_of(&property.namespace, &property.name).ok_or_else(|| {
        HostedPropertyObservationError::UnknownProperty {
            namespace: property.namespace.clone(),
            name: property.name.clone(),
        }
    })?;
    let layout = &registry.property(property_id).layout;
    let col = registry
        .column_range(property_id)
        .col_for_role(role, layout)
        .ok_or_else(|| HostedPropertyObservationError::UnknownRole {
            namespace: property.namespace.clone(),
            name: property.name.clone(),
            role: role.clone(),
        })?;
    let slot = allocator
        .slot_of(host)
        .ok_or(HostedPropertyObservationError::HostHasNoSlot { host })?;
    let idx = usize::from(slot) * snapshot.n_dims + col.raw();
    snapshot
        .values
        .get(idx)
        .copied()
        .ok_or(HostedPropertyObservationError::CellOutOfBounds {
            host,
            namespace: property.namespace.clone(),
            name: property.name.clone(),
        })
}

/// Live disruption authority readback over one GPU snapshot + typed loci.
///
/// `system_id_by_host_raw` must be pre-resolved through authored structural
/// authority (no ownership / substring / positional inference here).
pub struct LiveDisruptionAuthorityReadback<'a> {
    pub snapshot: &'a GpuValuesSnapshot,
    pub registry: &'a DimensionRegistry,
    pub allocator: &'a SlotAllocator,
    pub loci: &'a [HostedPropertyLocus],
    pub system_id_by_host_raw: &'a BTreeMap<u32, u32>,
}

impl DisruptionAuthorityReadback for LiveDisruptionAuthorityReadback<'_> {
    fn max_disruption_accreted_by_system_id(
        &self,
    ) -> Result<Option<BTreeMap<u32, f32>>, DisruptionAuthorityReadbackError> {
        if self.loci.is_empty() {
            return Ok(None);
        }
        let mut out: BTreeMap<u32, f32> = BTreeMap::new();
        for locus in self.loci {
            let value = observe_hosted_property_cell(
                self.registry,
                self.allocator,
                self.snapshot,
                locus.host_id,
                &locus.property,
                &locus.role,
            )
            .map_err(|err| DisruptionAuthorityReadbackError::new(err.to_string()))?;
            let Some(&system_id) = self.system_id_by_host_raw.get(&locus.host_id.raw()) else {
                return Err(DisruptionAuthorityReadbackError::new(format!(
                    "hosted disruption locus has no structural system_id mapping for host {:?}",
                    locus.host_id
                )));
            };
            out.entry(system_id)
                .and_modify(|max| *max = max.max(value))
                .or_insert(value);
        }
        Ok(Some(out))
    }
}

/// Resolve host raw id → generated system id from Spec structural placements
/// plus authored host entity keys (exact `location_id` / `target_id` match).
///
/// Any unmapped locus in a nonempty set fails loud (including an all-miss set).
/// Callers may fail-soft only when `loci` is empty.
pub fn system_id_by_host_raw_from_structural_authority(
    placements: &[simthing_spec::SimThingStructuralGridPlacement],
    _install_targets: &std::collections::HashMap<String, Vec<SimThingId>>,
    loci: &[HostedPropertyLocus],
    location_system_ids: &BTreeMap<String, u32>,
) -> Result<BTreeMap<u32, u32>, DisruptionAuthorityReadbackError> {
    if loci.is_empty() {
        return Ok(BTreeMap::new());
    }
    let mut by_raw: BTreeMap<u32, u32> = BTreeMap::new();
    let mut by_location: BTreeMap<&str, u32> = BTreeMap::new();
    for placement in placements {
        by_raw.insert(placement.simthing_id_raw, placement.system_id);
        by_location.insert(placement.location_id.as_str(), placement.system_id);
        by_location.insert(placement.target_id.as_str(), placement.system_id);
    }

    let mut out = BTreeMap::new();
    let mut unmapped = Vec::new();
    for locus in loci {
        if let Some(&system_id) = by_raw.get(&locus.host_id.raw()) {
            out.insert(locus.host_id.raw(), system_id);
            continue;
        }
        if let Some(entity) = locus.host_entity.as_deref() {
            if let Some(&system_id) = by_location.get(entity) {
                out.insert(locus.host_id.raw(), system_id);
                continue;
            }
            if let Some(&system_id) = location_system_ids.get(entity) {
                out.insert(locus.host_id.raw(), system_id);
                continue;
            }
        }
        unmapped.push(locus);
    }
    if !unmapped.is_empty() {
        let detail = unmapped
            .iter()
            .map(|locus| format!("{:?} entity={:?}", locus.host_id, locus.host_entity))
            .collect::<Vec<_>>()
            .join("; ");
        let kind = if out.is_empty() {
            "total"
        } else {
            "partial"
        };
        return Err(DisruptionAuthorityReadbackError::new(format!(
            "{kind} structural mapping failure for hosted disruption loci: {detail}"
        )));
    }
    Ok(out)
}
