//! SCENARIO-0080-2 — `ATLAS-BATCH-0-STORE` (EC-A3 CPU storage shape).
//!
//! CPU-only fixture: child contributions aggregated into dense `(location_id, cell_index,
//! channel, owner)` slots. Live OWNER masked reduction deferred to STORE-GPU. Not in `lib.rs`.

pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_ID: &str = "ATLAS-BATCH-0-STORE";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_STORE_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - EC-A3 CPU storage shape; STORE-GPU deferred; CPU-only, not GPU";

#[path = "dress_rehearsal_atlas_batch_0_pack.rs"]
mod pack;

pub use pack::{
    AtlasBatchPlan, ChannelKind, ChannelSet, LocationId, LocationMaterialization, LocationRole,
    Owner, CLASS_GALACTIC_20X20, CLASS_PLANET_SURFACE_10X10, CLASS_STAR_SYSTEM_10X10,
};

use pack::{pack_coord, unpack_coord};

/// LOC single indexing home (`cell_index(map_base, width, x, y)`).
pub fn cell_index(map_base: u32, width: u32, x: u32, y: u32) -> u32 {
    map_base + y * width + x
}

pub type SourceOccupantId = String;

#[derive(Clone, Debug, PartialEq)]
pub struct ChildContribution {
    pub source_occupant_id: SourceOccupantId,
    pub location_id: LocationId,
    pub cell_x: u32,
    pub cell_y: u32,
    pub cell_index: u32,
    pub owner: Owner,
    pub channel: ChannelKind,
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StoreKey {
    pub location_id: LocationId,
    pub cell_index: u32,
    pub channel: ChannelKind,
    pub owner: Owner,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoreEntry {
    pub key: StoreKey,
    pub value: f32,
    pub source_occupant_ids: Vec<SourceOccupantId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoreOracle {
    pub contributions: Vec<ChildContribution>,
    pub entries: Vec<StoreEntry>,
}

pub fn canonical_materialization() -> LocationMaterialization {
    LocationMaterialization::canonical()
}

pub fn canonical_pack_plan() -> AtlasBatchPlan {
    AtlasBatchPlan::canonical()
}

/// Deterministic generic fixture seed — not gameplay computation.
pub fn fixture_contribution_value(source_id: &str, channel: ChannelKind) -> f32 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in source_id.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^= channel_seed_mix(channel);
    hash = hash.wrapping_mul(0x100000001b3);
    1.0 + (hash % 10_000) as f32 * 0.001
}

fn channel_seed_mix(channel: ChannelKind) -> u64 {
    match channel {
        ChannelKind::Labor => 1,
        ChannelKind::Production => 2,
        ChannelKind::ProductionPassThrough => 3,
        ChannelKind::Disruption => 4,
        ChannelKind::PatrolPresence => 5,
        ChannelKind::PiratePresence => 6,
        ChannelKind::FleetStrength(owner) => 7 + owner as u64,
    }
}

fn occupant_kind_name(source_id: &str) -> Option<&'static str> {
    if source_id.starts_with("planet-") {
        Some("planet")
    } else if source_id.starts_with("starport-") {
        Some("starport")
    } else if source_id.starts_with("factory-") {
        Some("factory")
    } else if source_id.starts_with("pop-") {
        Some("pop")
    } else if source_id.contains("patrol") {
        Some("patrol")
    } else if source_id.contains("pirate") {
        Some("pirate")
    } else {
        None
    }
}

/// LOC-declared occupant → channel mapping (Opus contract; no invented gameplay columns).
fn channels_for_source(
    materialization: &LocationMaterialization,
    source_id: &str,
) -> Vec<ChannelKind> {
    let occupant = materialization
        .occupants
        .iter()
        .find(|o| o.source_id == source_id)
        .expect("occupant must exist");
    let mut channels: Vec<ChannelKind> = occupant.channels.iter().map(|d| d.kind).collect();
    match occupant_kind_name(source_id).as_deref() {
        Some("planet") => {
            if channels.is_empty() {
                return Vec::new();
            }
        }
        Some("patrol") => {
            if !channels.contains(&ChannelKind::FleetStrength(Owner::Terran)) {
                channels.push(ChannelKind::FleetStrength(Owner::Terran));
            }
        }
        Some("pirate") => {
            if !channels.contains(&ChannelKind::FleetStrength(Owner::Pirate)) {
                channels.push(ChannelKind::FleetStrength(Owner::Pirate));
            }
        }
        _ => {}
    }
    channels
}

pub fn child_contributions_for_source(
    materialization: &LocationMaterialization,
    source_id: &str,
) -> Vec<ChildContribution> {
    let occupant = materialization
        .occupants
        .iter()
        .find(|o| o.source_id == source_id)
        .expect("occupant must exist");
    let location = materialization
        .location(occupant.location_id)
        .expect("occupant location must exist");
    let cell_index = cell_index(
        location.map_base,
        location.width,
        occupant.cell.x,
        occupant.cell.y,
    );
    channels_for_source(materialization, source_id)
        .into_iter()
        .map(|channel| ChildContribution {
            source_occupant_id: source_id.to_string(),
            location_id: occupant.location_id,
            cell_x: occupant.cell.x,
            cell_y: occupant.cell.y,
            cell_index,
            owner: occupant.owner,
            channel,
            value: fixture_contribution_value(source_id, channel),
        })
        .collect()
}

pub fn child_contributions_from_materialization(
    materialization: &LocationMaterialization,
) -> Vec<ChildContribution> {
    materialization
        .occupants
        .iter()
        .flat_map(|occupant| {
            child_contributions_for_source(materialization, occupant.source_id.as_str())
        })
        .collect()
}

pub fn aggregate_contributions(contributions: &[ChildContribution]) -> Vec<StoreEntry> {
    use std::collections::HashMap;

    let mut grouped: HashMap<StoreKey, StoreEntry> = HashMap::new();
    for contribution in contributions {
        let key = StoreKey {
            location_id: contribution.location_id,
            cell_index: contribution.cell_index,
            channel: contribution.channel,
            owner: contribution.owner,
        };
        grouped
            .entry(key)
            .and_modify(|entry| {
                entry.value += contribution.value;
                entry
                    .source_occupant_ids
                    .push(contribution.source_occupant_id.clone());
            })
            .or_insert(StoreEntry {
                key,
                value: contribution.value,
                source_occupant_ids: vec![contribution.source_occupant_id.clone()],
            });
    }
    let mut entries: Vec<_> = grouped.into_values().collect();
    entries.sort_by(|left, right| {
        left.key
            .location_id
            .0
            .cmp(&right.key.location_id.0)
            .then(left.key.cell_index.cmp(&right.key.cell_index))
            .then(
                store_key_channel_rank(left.key.channel)
                    .cmp(&store_key_channel_rank(right.key.channel)),
            )
            .then(store_key_owner_rank(left.key.owner).cmp(&store_key_owner_rank(right.key.owner)))
    });
    entries
}

fn store_key_channel_rank(channel: ChannelKind) -> u8 {
    match channel {
        ChannelKind::Labor => 0,
        ChannelKind::Production => 1,
        ChannelKind::ProductionPassThrough => 2,
        ChannelKind::Disruption => 3,
        ChannelKind::PatrolPresence => 4,
        ChannelKind::PiratePresence => 5,
        ChannelKind::FleetStrength(Owner::Terran) => 6,
        ChannelKind::FleetStrength(Owner::Pirate) => 7,
    }
}

fn store_key_owner_rank(owner: Owner) -> u8 {
    match owner {
        Owner::Terran => 0,
        Owner::Pirate => 1,
    }
}

pub fn store_oracle_from_materialization(materialization: &LocationMaterialization) -> StoreOracle {
    let contributions = child_contributions_from_materialization(materialization);
    let entries = aggregate_contributions(&contributions);
    StoreOracle {
        contributions,
        entries,
    }
}

pub fn store_oracle_with_additional_sources(
    materialization: &LocationMaterialization,
    additional_source_ids: &[&str],
) -> StoreOracle {
    let mut contributions = child_contributions_from_materialization(materialization);
    for source_id in additional_source_ids {
        contributions.extend(child_contributions_for_source(materialization, source_id));
    }
    let entries = aggregate_contributions(&contributions);
    StoreOracle {
        contributions,
        entries,
    }
}

pub fn store_oracle_constructed_planet_patrol_pirate(
    materialization: &LocationMaterialization,
) -> StoreOracle {
    let extended = register_constructed_co_location_occupants(materialization);
    store_oracle_from_materialization(&extended)
}

pub fn register_constructed_co_location_occupants(
    materialization: &LocationMaterialization,
) -> LocationMaterialization {
    let mut extended = materialization.clone();
    let patrol = extended
        .occupants
        .iter()
        .find(|o| o.source_id.contains("patrol"))
        .expect("canonical patrol fleet")
        .clone();
    let pirate = extended
        .occupants
        .iter()
        .find(|o| o.source_id.starts_with("pirate-ship"))
        .expect("canonical pirate fleet")
        .clone();
    let mut planet = extended
        .occupants
        .iter()
        .find(|o| o.source_id == "planet-0")
        .expect("canonical planet")
        .clone();

    let target_location = LocationId(1);
    let mut target_cell = patrol.cell;
    target_cell.x = 3;
    target_cell.y = 3;

    planet.source_id = "constructed-planet".to_string();
    planet.location_id = target_location;
    planet.cell = target_cell;
    planet.channels = extended
        .occupants
        .iter()
        .find(|o| o.source_id.starts_with("starport-"))
        .expect("starport channel template")
        .channels
        .clone();

    let mut patrol_c = patrol;
    patrol_c.source_id = "constructed-patrol".to_string();
    patrol_c.location_id = target_location;
    patrol_c.cell = target_cell;

    let mut pirate_c = pirate;
    pirate_c.source_id = "constructed-pirate".to_string();
    pirate_c.location_id = target_location;
    pirate_c.cell = target_cell;

    extended.occupants.push(planet);
    extended.occupants.push(patrol_c);
    extended.occupants.push(pirate_c);
    extended
}

pub fn entries_at_cell_index(
    oracle: &StoreOracle,
    location_id: LocationId,
    cell_index: u32,
) -> Vec<&StoreEntry> {
    oracle
        .entries
        .iter()
        .filter(|entry| entry.key.location_id == location_id && entry.key.cell_index == cell_index)
        .collect()
}

pub fn class_id_for_location_role(role: LocationRole) -> &'static str {
    match role {
        LocationRole::Galactic => CLASS_GALACTIC_20X20,
        LocationRole::StarSystem => CLASS_STAR_SYSTEM_10X10,
        LocationRole::PlanetSurface => CLASS_PLANET_SURFACE_10X10,
    }
}

pub fn pack_round_trip_cell(
    plan: &AtlasBatchPlan,
    materialization: &LocationMaterialization,
    location_id: LocationId,
    x: u32,
    y: u32,
) -> Option<(u32, u32)> {
    let location = materialization.location(location_id)?;
    let packed = pack_coord(plan, location_id, x, y)?;
    let class_id = class_id_for_location_role(location.role);
    let (round_id, lx, ly) = unpack_coord(plan, class_id, packed.0, packed.1)?;
    if round_id != location_id || (lx, ly) != (x, y) {
        return None;
    }
    Some(packed)
}

pub fn canonical_pirate_shared_galactic_cell(
    materialization: &LocationMaterialization,
) -> (LocationId, u32, u32, u32) {
    let pirate_ids: Vec<_> = materialization
        .occupants
        .iter()
        .filter(|o| o.source_id.starts_with("pirate-ship"))
        .map(|o| o.source_id.as_str())
        .collect();
    assert_eq!(pirate_ids.len(), 10);
    let first = materialization
        .occupants
        .iter()
        .find(|o| o.source_id == pirate_ids[0])
        .expect("pirate fleet");
    let location = materialization
        .location(first.location_id)
        .expect("galactic location");
    let cell_index = cell_index(
        location.map_base,
        location.width,
        first.cell.x,
        first.cell.y,
    );
    (first.location_id, first.cell.x, first.cell.y, cell_index)
}

pub fn pirate_fleet_source_ids(materialization: &LocationMaterialization) -> Vec<String> {
    let mut ids: Vec<_> = materialization
        .occupants
        .iter()
        .filter(|o| o.source_id.starts_with("pirate-ship"))
        .map(|o| o.source_id.clone())
        .collect();
    ids.sort();
    ids
}
