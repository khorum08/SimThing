//! Read-only per-system disruption snapshot over ScenarioSpec authority.

use std::collections::BTreeMap;

use thiserror::Error;

use super::planet_child_location::star_system_gridcells;
use super::scenario::{gridcell_generated_system_id, ScenarioRootError, SimThingScenarioSpec};

#[derive(Debug, Clone, PartialEq)]
pub struct DisruptionReadoutRecord {
    system_id: u32,
    max_disruption_accreted: f32,
}

impl DisruptionReadoutRecord {
    pub fn system_id(&self) -> u32 {
        self.system_id
    }

    pub fn max_disruption_accreted(&self) -> f32 {
        self.max_disruption_accreted
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DisruptionReadoutSnapshot {
    records: Vec<DisruptionReadoutRecord>,
}

impl DisruptionReadoutSnapshot {
    pub fn records(&self) -> &[DisruptionReadoutRecord] {
        &self.records
    }

    pub fn by_system_id(&self) -> BTreeMap<u32, DisruptionReadoutRecord> {
        self.records
            .iter()
            .map(|record| (record.system_id, record.clone()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("disruption authority readback failed: {message}")]
pub struct DisruptionAuthorityReadbackError {
    message: String,
}

impl DisruptionAuthorityReadbackError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub trait DisruptionAuthorityReadback {
    /// HORIZON-ENTRY(2026-07-16): 12.8 STUDIO-FIELD-SESSION-ELEVATE-0 wires live values
    fn max_disruption_accreted_by_system_id(
        &self,
    ) -> Result<Option<BTreeMap<u32, f32>>, DisruptionAuthorityReadbackError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AbsentDisruptionAuthorityReadback;

impl DisruptionAuthorityReadback for AbsentDisruptionAuthorityReadback {
    fn max_disruption_accreted_by_system_id(
        &self,
    ) -> Result<Option<BTreeMap<u32, f32>>, DisruptionAuthorityReadbackError> {
        Ok(None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DisruptionReadoutSnapshotError {
    #[error("disruption readout snapshot could not read scenario authority: {0}")]
    ScenarioRoot(#[from] ScenarioRootError),
    #[error(transparent)]
    AuthorityReadback(#[from] DisruptionAuthorityReadbackError),
}

pub fn disruption_readout_snapshot(
    spec: &SimThingScenarioSpec,
) -> Result<DisruptionReadoutSnapshot, DisruptionReadoutSnapshotError> {
    disruption_readout_snapshot_with_readback(spec, &AbsentDisruptionAuthorityReadback)
}

pub fn disruption_readout_snapshot_with_readback(
    spec: &SimThingScenarioSpec,
    readback: &dyn DisruptionAuthorityReadback,
) -> Result<DisruptionReadoutSnapshot, DisruptionReadoutSnapshotError> {
    let systems = match star_system_gridcells(spec) {
        Ok(systems) => systems,
        Err(
            ScenarioRootError::MissingGameSessionChild
            | ScenarioRootError::MissingGalaxyMap
            | ScenarioRootError::LegacyWorldRootHasNoGameSessionRequirement
            | ScenarioRootError::LegacyWorldRootHasNoGalaxyMapRequirement,
        ) => {
            return Ok(DisruptionReadoutSnapshot::default());
        }
        Err(err) => return Err(err.into()),
    };
    let system_id_by_raw: BTreeMap<u32, u32> = spec
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.simthing_id_raw, placement.system_id))
        .collect();

    let values_by_system_id = readback.max_disruption_accreted_by_system_id()?;

    let mut records = Vec::new();
    for system in systems {
        let Some(system_id) = gridcell_generated_system_id(system)
            .or_else(|| system_id_by_raw.get(&system.id.raw()).copied())
        else {
            continue;
        };
        let max_disruption_accreted = values_by_system_id
            .as_ref()
            .and_then(|values| values.get(&system_id).copied())
            .unwrap_or(0.0);
        records.push(DisruptionReadoutRecord {
            system_id,
            max_disruption_accreted,
        });
    }
    records.sort_by_key(|record| record.system_id);

    Ok(DisruptionReadoutSnapshot { records })
}
