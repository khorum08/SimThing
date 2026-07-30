//! Canonical observation / telemetry / metrics read seam.
//!
//! Consumers resolve a hosted cell by [`SimThingId`] + typed [`PropertyKey`] +
//! [`SubFieldRole`] through the property layout/registry and the derived STEAD
//! [`AnchorTableSnapshot`]. The snapshot is built from the governed GPU compact
//! readback (orch remand `5120259758`) — never by cloning boundary CPU staging.
//! Raw [`ColumnIndex`] values are never minted or exposed on this door.
//!
//! Internal boundary mutation / RF / replay may still call `WorldGpuState::read_values`;
//! that path is not a production observation authority (see observation-bypass census).

use std::collections::BTreeMap;

use simthing_core::{
    AnchorIdentity, AnchorTable, AnchorTableRow, DimensionRegistry, SimThingId,
    SlotIndex, SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{DisruptionAuthorityReadback, DisruptionAuthorityReadbackError, PropertyKey};
use thiserror::Error;

use crate::session::SimSession;

/// Typed compact readback of the derived STEAD anchor table (sole consumer door).
#[derive(Debug, Clone)]
pub struct AnchorTableSnapshot {
    table: AnchorTable,
}

impl AnchorTableSnapshot {
    /// Sole production observation door: decode the GPU-resident compact table.
    pub fn from_session(sim: &SimSession) -> Self {
        Self {
            table: sim
                .state
                .read_typed_anchor_table(&sim.proto.registry),
        }
    }

    pub fn from_table(table: AnchorTable) -> Self {
        Self { table }
    }

    pub fn table(&self) -> &AnchorTable {
        &self.table
    }

    pub fn rows(&self) -> &[AnchorTableRow] {
        self.table.rows()
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn get(&self, identity: AnchorIdentity) -> Option<&AnchorTableRow> {
        self.table.get(identity)
    }

    pub fn observed_value_at_slot_col(&self, slot_raw: u32, col_raw: u32) -> Option<f32> {
        self.table
            .get_by_slot_col(
                SlotIndex::new(slot_raw),
                simthing_gpu::column_from_wire(col_raw),
            )
            .map(|r| r.observed_value)
    }
}

/// Legacy name retained as a type alias during migration; production consumers
/// must use [`AnchorTableSnapshot`]. Creating this via raw values is test-only.
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct GpuValuesSnapshot {
    values: Vec<f32>,
    n_dims: usize,
}

impl GpuValuesSnapshot {
    /// Internal/test helper only — not a production observation authority.
    #[doc(hidden)]
    pub fn from_values_for_test(values: Vec<f32>, n_dims: usize) -> Self {
        Self { values, n_dims }
    }

    #[doc(hidden)]
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    #[doc(hidden)]
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
    #[error("no anchor-table row for host {host:?} property {namespace}::{name}")]
    MissingAnchorRow {
        host: SimThingId,
        namespace: String,
        name: String,
    },
}

/// Canonical read of one hosted property cell from the STEAD anchor table.
pub fn observe_hosted_property_cell(
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    snapshot: &AnchorTableSnapshot,
    host: SimThingId,
    property: &PropertyKey,
    role: &SubFieldRole,
) -> Result<f32, HostedPropertyObservationError> {
    let property_id = registry
        .id_of(&property.namespace, &property.name)
        .ok_or_else(|| HostedPropertyObservationError::UnknownProperty {
            namespace: property.namespace.clone(),
            name: property.name.clone(),
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
    let identity = AnchorIdentity::new(host, property_id);
    if let Some(row) = snapshot.table().get_by_identity_role(identity, role) {
        return Ok(row.observed_value);
    }
    let slot = allocator
        .slot_of(host)
        .ok_or(HostedPropertyObservationError::HostHasNoSlot { host })?;
    snapshot
        .table()
        .get_by_slot_col(slot, col)
        .filter(|r| r.identity == identity)
        .map(|r| r.observed_value)
        .ok_or(HostedPropertyObservationError::MissingAnchorRow {
            host,
            namespace: property.namespace.clone(),
            name: property.name.clone(),
        })
}

/// Live disruption authority readback over one anchor-table snapshot + typed loci.
///
/// `system_id_by_host_raw` must be pre-resolved through authored structural
/// authority (no ownership / substring / positional inference here).
pub struct LiveDisruptionAuthorityReadback<'a> {
    pub snapshot: &'a AnchorTableSnapshot,
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
        let kind = if out.is_empty() { "total" } else { "partial" };
        return Err(DisruptionAuthorityReadbackError::new(format!(
            "{kind} structural mapping failure for hosted disruption loci: {detail}"
        )));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        AnchorIdentity, AnchorTable, AnchorTableRow, ClampBehavior, ColumnIndex, SimPropertyId,
        SimThing, SimThingKind, SlotIndex, SubFieldSpec,
    };
    use simthing_spec::{compile_property, PropertySpec};

    fn snapshot_from_row(
        host: SimThingId,
        property_id: SimPropertyId,
        value: f32,
        col: usize,
    ) -> AnchorTableSnapshot {
        let mut table = AnchorTable::new();
        table.replace_rows(vec![AnchorTableRow {
            identity: AnchorIdentity::new(host, property_id),
            slot: SlotIndex::new(0),
            col: ColumnIndex::from_raw_for_oracle_or_rehearsal(col),
            role: SubFieldRole::Amount,
            band: None,
            last_crossing_generation: None,
            urgency: 0.0,
            observed_value: value,
        }]);
        AnchorTableSnapshot::from_table(table)
    }
}
