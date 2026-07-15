//! Read-only fleet presence snapshot over ScenarioSpec authority.

use std::collections::BTreeMap;

use simthing_core::{SimPropertyId, SimThing, SimThingKind};
use thiserror::Error;

use super::channel_key::OwnerRef;
use super::planet_child_location::star_system_gridcells;
use super::scenario::{
    gridcell_generated_system_id, owner_flow_owner_ref, scenario_metadata_string,
    ScenarioRootError, SimThingScenarioSpec,
};

/// TP fleet posture metadata emitted by ClauseThing fleet payload hydration.
pub const TP_FLEET_POSTURE_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_500);
/// TP authored home-system metadata emitted by ClauseThing fleet payload hydration.
pub const TP_FLEET_HOME_SYSTEM_PROPERTY_ID: SimPropertyId = SimPropertyId(8_301_501);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FleetPresenceLocation {
    Anchored(u32),
    InTransit {
        source_system_id: u32,
        dest_system_id: u32,
    },
}

impl FleetPresenceLocation {
    pub fn system_key(&self) -> u32 {
        match self {
            Self::Anchored(system_id) => *system_id,
            Self::InTransit {
                source_system_id, ..
            } => *source_system_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetPresenceRecord {
    pub fleet_simthing_id_raw: u32,
    pub owner_ref: Option<OwnerRef>,
    pub posture: Option<String>,
    pub location: FleetPresenceLocation,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FleetPresenceSnapshot {
    records: Vec<FleetPresenceRecord>,
}

impl FleetPresenceSnapshot {
    pub fn records(&self) -> &[FleetPresenceRecord] {
        &self.records
    }

    pub fn by_system_id(&self) -> BTreeMap<u32, Vec<FleetPresenceRecord>> {
        let mut by_system = BTreeMap::new();
        for record in &self.records {
            by_system
                .entry(record.location.system_key())
                .or_insert_with(Vec::new)
                .push(record.clone());
        }
        by_system
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FleetPresenceSnapshotError {
    #[error("fleet presence snapshot could not read scenario authority: {0}")]
    ScenarioRoot(#[from] ScenarioRootError),
    #[error("fleet {fleet_simthing_id_raw} is under a star-system without generated system id")]
    MissingAnchorSystemId { fleet_simthing_id_raw: u32 },
}

pub fn fleet_presence_snapshot(
    spec: &SimThingScenarioSpec,
) -> Result<FleetPresenceSnapshot, FleetPresenceSnapshotError> {
    let systems = match star_system_gridcells(spec) {
        Ok(systems) => systems,
        Err(
            ScenarioRootError::MissingGameSessionChild
            | ScenarioRootError::MissingGalaxyMap
            | ScenarioRootError::LegacyWorldRootHasNoGameSessionRequirement
            | ScenarioRootError::LegacyWorldRootHasNoGalaxyMapRequirement,
        ) => {
            return Ok(FleetPresenceSnapshot::default());
        }
        Err(err) => return Err(err.into()),
    };
    let system_id_by_raw: BTreeMap<u32, u32> = spec
        .structural_grid
        .placements
        .iter()
        .map(|placement| (placement.simthing_id_raw, placement.system_id))
        .collect();

    let mut records = Vec::new();
    for system in systems {
        let Some(system_id) = gridcell_generated_system_id(system)
            .or_else(|| system_id_by_raw.get(&system.id.raw()).copied())
        else {
            if let Some(fleet) = first_fleet_under(system) {
                return Err(FleetPresenceSnapshotError::MissingAnchorSystemId {
                    fleet_simthing_id_raw: fleet.id.raw(),
                });
            }
            continue;
        };
        collect_fleet_records(system, system_id, &mut records)?;
    }
    records.sort_by_key(|record| record.fleet_simthing_id_raw);

    Ok(FleetPresenceSnapshot { records })
}

fn first_fleet_under(node: &SimThing) -> Option<&SimThing> {
    if node.kind == SimThingKind::Fleet {
        return Some(node);
    }
    node.children.iter().find_map(first_fleet_under)
}

fn collect_fleet_records(
    node: &SimThing,
    system_id: u32,
    records: &mut Vec<FleetPresenceRecord>,
) -> Result<(), FleetPresenceSnapshotError> {
    if node.kind == SimThingKind::Fleet {
        let raw = node.id.raw();
        records.push(FleetPresenceRecord {
            fleet_simthing_id_raw: raw,
            owner_ref: owner_flow_owner_ref(node).map(OwnerRef::new),
            posture: scenario_metadata_string(node, TP_FLEET_POSTURE_PROPERTY_ID)
                .filter(|value| !value.trim().is_empty()),
            location: FleetPresenceLocation::Anchored(system_id),
        });
        return Ok(());
    }

    for child in &node.children {
        collect_fleet_records(child, system_id, records)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transit_contract_is_test_private_until_authoritative_state_exists() {
        let snapshot = FleetPresenceSnapshot {
            records: vec![FleetPresenceRecord {
                fleet_simthing_id_raw: 42,
                owner_ref: None,
                posture: None,
                location: FleetPresenceLocation::InTransit {
                    source_system_id: 7,
                    dest_system_id: 8,
                },
            }],
        };

        let by_system = snapshot.by_system_id();
        assert_eq!(by_system.keys().copied().collect::<Vec<_>>(), vec![7]);
        assert!(matches!(
            by_system[&7][0].location,
            FleetPresenceLocation::InTransit {
                source_system_id: 7,
                dest_system_id: 8,
            }
        ));
    }
}
