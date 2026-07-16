//! STUDIO-DISRUPTION-READOUT-0 mapeditor projection.
//!
//! Consumes the typed spec snapshot only. Live field authority and id
//! translation stay in simthing-spec.

use std::collections::BTreeMap;

use simthing_spec::{
    disruption_readout_snapshot, DisruptionReadoutRecord, DisruptionReadoutSnapshot,
    DisruptionReadoutSnapshotError,
};

use crate::session::StudioSession;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StudioDisruptionReadoutMap {
    pub by_system_id: BTreeMap<u32, DisruptionReadoutRecord>,
    pub system_count: usize,
}

pub fn studio_disruption_readout_map_from_session(
    session: &StudioSession,
) -> Result<StudioDisruptionReadoutMap, DisruptionReadoutSnapshotError> {
    let snapshot = disruption_readout_snapshot(&session.scenario_authority)?;
    Ok(studio_disruption_readout_map_from_snapshot(&snapshot))
}

pub fn studio_disruption_readout_map_from_snapshot(
    snapshot: &DisruptionReadoutSnapshot,
) -> StudioDisruptionReadoutMap {
    let by_system_id = snapshot.by_system_id();
    StudioDisruptionReadoutMap {
        system_count: by_system_id.len(),
        by_system_id,
    }
}
