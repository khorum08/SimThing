pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID: &str = "ATLAS-BATCH-0-LOC";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - fixture-only Location gridcell layout; no runtime wiring";

pub const EXPECTED_LOCATION_COUNT: usize = 27;
pub const EXPECTED_OCCUPANT_COUNT: usize = 56;
pub const EXPECTED_TOTAL_CELL_SLOTS: u32 = 3000;

#[path = "dress_rehearsal_atlas_batch_0_gen.rs"]
mod gen;

pub use gen::{DressRehearsalMap, FleetKind, GridCell, Owner};

use gen::{GALAXY_SIDE, PLANET_SURFACE_SIDE, SYSTEM_COUNT, SYSTEM_SIDE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocationId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocationRole {
    Galactic,
    StarSystem,
    PlanetSurface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OccupantKind {
    Planet,
    Starport,
    FactoryDistrict,
    PopCohort,
    PatrolFleet,
    PirateFleet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mobility {
    Fixed,
    Mover,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelKind {
    Labor,
    Production,
    ProductionPassThrough,
    Disruption,
    PatrolPresence,
    PiratePresence,
    FleetStrength(Owner),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChannelDescriptor {
    pub kind: ChannelKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelSet {
    pub channels: Vec<ChannelDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationGridDescriptor {
    pub id: LocationId,
    pub role: LocationRole,
    pub parent: Option<LocationId>,
    pub map_base: u32,
    pub width: u32,
    pub height: u32,
    pub channels: ChannelSet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OccupantPlacement {
    pub kind: OccupantKind,
    pub owner: Owner,
    pub location_id: LocationId,
    pub cell: GridCell,
    pub mobility: Mobility,
    pub channels: Vec<ChannelDescriptor>,
    pub surface_location: Option<LocationId>,
    pub source_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocationMaterialization {
    pub locations: Vec<LocationGridDescriptor>,
    pub occupants: Vec<OccupantPlacement>,
    pub total_cell_slots: u32,
}

impl LocationMaterialization {
    pub fn canonical() -> Self {
        Self::from_map(&DressRehearsalMap::canonical())
    }

    pub fn from_map(map: &DressRehearsalMap) -> Self {
        let galactic_id = LocationId(0);
        let mut locations = Vec::with_capacity(EXPECTED_LOCATION_COUNT);
        let mut occupants = Vec::with_capacity(EXPECTED_OCCUPANT_COUNT);
        let mut next_base = 0u32;

        let galactic_base = next_base;
        next_base += GALAXY_SIDE * GALAXY_SIDE;
        locations.push(LocationGridDescriptor {
            id: galactic_id,
            role: LocationRole::Galactic,
            parent: None,
            map_base: galactic_base,
            width: GALAXY_SIDE,
            height: GALAXY_SIDE,
            channels: galactic_channel_set(),
        });

        for system in &map.systems {
            let system_id = system_location_id(system.index);
            let system_base = next_base;
            next_base += SYSTEM_SIDE * SYSTEM_SIDE;
            locations.push(LocationGridDescriptor {
                id: system_id,
                role: LocationRole::StarSystem,
                parent: Some(galactic_id),
                map_base: system_base,
                width: SYSTEM_SIDE,
                height: SYSTEM_SIDE,
                channels: star_system_channel_set(),
            });
        }

        for system in &map.systems {
            let system_id = system_location_id(system.index);
            let surface_id = surface_location_id(system.index);
            let surface_base = next_base;
            next_base += PLANET_SURFACE_SIDE * PLANET_SURFACE_SIDE;
            locations.push(LocationGridDescriptor {
                id: surface_id,
                role: LocationRole::PlanetSurface,
                parent: Some(system_id),
                map_base: surface_base,
                width: PLANET_SURFACE_SIDE,
                height: PLANET_SURFACE_SIDE,
                channels: planet_surface_channel_set(),
            });
        }

        for system in &map.systems {
            let system_id = system_location_id(system.index);
            let surface_id = surface_location_id(system.index);

            occupants.push(OccupantPlacement {
                kind: OccupantKind::Planet,
                owner: system.planet.owner,
                location_id: system_id,
                cell: system.planet.system_cell,
                mobility: Mobility::Fixed,
                channels: Vec::new(),
                surface_location: Some(surface_id),
                source_id: format!("planet-{}", system.index),
            });

            if let Some(starport) = &system.starport {
                occupants.push(OccupantPlacement {
                    kind: OccupantKind::Starport,
                    owner: starport.owner,
                    location_id: system_id,
                    cell: starport.cell,
                    mobility: Mobility::Fixed,
                    channels: vec![ChannelDescriptor {
                        kind: ChannelKind::ProductionPassThrough,
                    }],
                    surface_location: None,
                    source_id: format!("starport-{}", system.index),
                });
            }

            occupants.push(OccupantPlacement {
                kind: OccupantKind::FactoryDistrict,
                owner: system.planet.surface.factory.owner,
                location_id: surface_id,
                cell: system.planet.surface.factory.cell,
                mobility: Mobility::Fixed,
                channels: vec![ChannelDescriptor {
                    kind: ChannelKind::Production,
                }],
                surface_location: None,
                source_id: format!("factory-{}", system.index),
            });

            occupants.push(OccupantPlacement {
                kind: OccupantKind::PopCohort,
                owner: system.planet.surface.pop_cohort.owner,
                location_id: surface_id,
                cell: system.planet.surface.pop_cohort.cell,
                mobility: Mobility::Fixed,
                channels: vec![ChannelDescriptor {
                    kind: ChannelKind::Labor,
                }],
                surface_location: None,
                source_id: format!("pop-{}", system.index),
            });
        }

        for fleet in &map.fleets {
            let (kind, channel) = match fleet.kind {
                FleetKind::Patrol => (
                    OccupantKind::PatrolFleet,
                    ChannelDescriptor {
                        kind: ChannelKind::PatrolPresence,
                    },
                ),
                FleetKind::PirateShip => (
                    OccupantKind::PirateFleet,
                    ChannelDescriptor {
                        kind: ChannelKind::PiratePresence,
                    },
                ),
            };
            occupants.push(OccupantPlacement {
                kind,
                owner: fleet.owner,
                location_id: galactic_id,
                cell: fleet.galactic_cell,
                mobility: Mobility::Mover,
                channels: vec![channel],
                surface_location: None,
                source_id: fleet.id.clone(),
            });
        }

        Self {
            locations,
            occupants,
            total_cell_slots: next_base,
        }
    }

    pub fn location(&self, id: LocationId) -> Option<&LocationGridDescriptor> {
        self.locations.iter().find(|location| location.id == id)
    }

    pub fn occupants_at(&self, location_id: LocationId, cell: GridCell) -> Vec<&OccupantPlacement> {
        self.occupants
            .iter()
            .filter(|occupant| occupant.location_id == location_id && occupant.cell == cell)
            .collect()
    }

    pub fn locations_by_role(&self, role: LocationRole) -> Vec<&LocationGridDescriptor> {
        self.locations
            .iter()
            .filter(|location| location.role == role)
            .collect()
    }
}

pub fn cell_index(map_base: u32, width: u32, x: u32, y: u32) -> u32 {
    map_base + y * width + x
}

fn system_location_id(system_index: usize) -> LocationId {
    LocationId(1 + system_index as u32)
}

fn surface_location_id(system_index: usize) -> LocationId {
    LocationId(1 + SYSTEM_COUNT as u32 + system_index as u32)
}

fn planet_surface_channel_set() -> ChannelSet {
    ChannelSet {
        channels: vec![
            ChannelDescriptor {
                kind: ChannelKind::Labor,
            },
            ChannelDescriptor {
                kind: ChannelKind::Production,
            },
        ],
    }
}

fn star_system_channel_set() -> ChannelSet {
    ChannelSet {
        channels: vec![
            ChannelDescriptor {
                kind: ChannelKind::Disruption,
            },
            ChannelDescriptor {
                kind: ChannelKind::ProductionPassThrough,
            },
        ],
    }
}

fn galactic_channel_set() -> ChannelSet {
    ChannelSet {
        channels: vec![
            ChannelDescriptor {
                kind: ChannelKind::Disruption,
            },
            ChannelDescriptor {
                kind: ChannelKind::FleetStrength(Owner::Terran),
            },
            ChannelDescriptor {
                kind: ChannelKind::FleetStrength(Owner::Pirate),
            },
            ChannelDescriptor {
                kind: ChannelKind::PatrolPresence,
            },
            ChannelDescriptor {
                kind: ChannelKind::PiratePresence,
            },
        ],
    }
}

pub fn channel_set_has_kind(set: &ChannelSet, expected: ChannelKind) -> bool {
    set.channels.iter().any(|channel| channel.kind == expected)
}
