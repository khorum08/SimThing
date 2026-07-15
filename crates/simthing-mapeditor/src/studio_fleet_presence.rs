//! STUDIO-FLEET-PRESENCE-READOUT-0 mapeditor projection.
//!
//! Consumes the typed spec snapshot only. Property-id translation stays in
//! simthing-spec / clausething authority helpers.

use std::collections::BTreeMap;

use simthing_spec::{
    fleet_presence_snapshot, FleetPresenceLocation, FleetPresenceRecord, FleetPresenceSnapshot,
    FleetPresenceSnapshotError,
};

use crate::session::StudioSession;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StudioFleetPresenceMap {
    pub by_system_id: BTreeMap<u32, Vec<FleetPresenceRecord>>,
    pub total_fleets: usize,
    pub transit_fleets: usize,
}

pub fn studio_fleet_presence_map_from_session(
    session: &StudioSession,
) -> Result<StudioFleetPresenceMap, FleetPresenceSnapshotError> {
    let snapshot = fleet_presence_snapshot(&session.scenario_authority)?;
    Ok(studio_fleet_presence_map_from_snapshot(&snapshot))
}

pub fn studio_fleet_presence_map_from_snapshot(
    snapshot: &FleetPresenceSnapshot,
) -> StudioFleetPresenceMap {
    let mut by_system_id = BTreeMap::new();
    let mut transit_fleets = 0usize;
    for record in snapshot.records() {
        if matches!(record.location, FleetPresenceLocation::InTransit { .. }) {
            transit_fleets = transit_fleets.saturating_add(1);
        }
        by_system_id
            .entry(record.location.system_key())
            .or_insert_with(Vec::new)
            .push(record.clone());
    }
    StudioFleetPresenceMap {
        by_system_id,
        total_fleets: snapshot.records().len(),
        transit_fleets,
    }
}

pub fn studio_fleet_presence_source_forbids_raw_property_ids(source: &str) -> Result<(), String> {
    let posture = format!("{}{}", "TP_FLEET_", "POSTURE_PROPERTY_ID");
    let home = format!("{}{}", "TP_FLEET_", "HOME_SYSTEM_PROPERTY_ID");
    let posture_raw = format!("{}_{}", "8_301", "500");
    let home_raw = format!("{}_{}", "8_301", "501");

    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("//!") || trimmed.starts_with("///") {
            continue;
        }
        if trimmed.contains("studio_fleet_presence_source_forbids_raw_property_ids") {
            continue;
        }
        for token in [&posture, &home, &posture_raw, &home_raw] {
            if trimmed.contains(token) {
                return Err(format!(
                    "studio_fleet_presence forbids raw fleet property id token in: {trimmed}"
                ));
            }
        }
    }
    Ok(())
}
