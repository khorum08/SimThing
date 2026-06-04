#[path = "../src/dress_rehearsal_atlas_batch_0_loc.rs"]
mod dress_rehearsal_atlas_batch_0_loc;

use dress_rehearsal_atlas_batch_0_loc::{
    cell_index, channel_set_has_kind, ChannelDescriptor, ChannelKind, DressRehearsalMap, FleetKind,
    GridCell, LocationId, LocationMaterialization, LocationRole, Mobility, OccupantKind,
    OccupantPlacement, Owner, DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID,
    DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS, EXPECTED_LOCATION_COUNT,
    EXPECTED_OCCUPANT_COUNT, EXPECTED_TOTAL_CELL_SLOTS,
};

#[test]
fn docs_status_matches_gate() {
    assert_eq!(DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_ID, "ATLAS-BATCH-0-LOC");
    assert!(
        DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS.contains("IMPLEMENTED / PASS"),
        "{DRESS_REHEARSAL_ATLAS_BATCH_0_LOC_STATUS_PASS}"
    );
}

#[test]
fn loc_materialization_is_deterministic() {
    let map = DressRehearsalMap::canonical();
    let left = LocationMaterialization::from_map(&map);
    let right = LocationMaterialization::from_map(&map);
    assert_eq!(left, right);
}

#[test]
fn every_location_cell_in_bounds() {
    let materialization = LocationMaterialization::canonical();
    for location in &materialization.locations {
        for y in 0..location.height {
            for x in 0..location.width {
                let index = cell_index(location.map_base, location.width, x, y);
                assert!(
                    index < materialization.total_cell_slots,
                    "cell index {index} must be within total_cell_slots for {:?}",
                    location.id
                );
            }
        }
    }
}

#[test]
fn cell_index_matches_row_major_formula() {
    let materialization = LocationMaterialization::canonical();
    for location in &materialization.locations {
        let samples = [
            (0, 0),
            (location.width - 1, 0),
            (0, location.height - 1),
            (location.width - 1, location.height - 1),
            (location.width / 2, location.height / 2),
        ];
        for (x, y) in samples {
            let expected = location.map_base + y * location.width + x;
            assert_eq!(
                cell_index(location.map_base, location.width, x, y),
                expected,
                "row-major indexing for location {:?} at ({x},{y})",
                location.id
            );
        }
    }
}

#[test]
fn location_slot_ranges_contiguous_and_non_overlapping() {
    let materialization = LocationMaterialization::canonical();
    assert_eq!(materialization.locations.len(), EXPECTED_LOCATION_COUNT);
    assert_eq!(materialization.total_cell_slots, EXPECTED_TOTAL_CELL_SLOTS);

    let mut ordered: Vec<_> = materialization.locations.iter().collect();
    ordered.sort_by_key(|location| location.map_base);

    let mut cursor = 0u32;
    for location in ordered {
        let span = location.width * location.height;
        assert_eq!(
            location.map_base, cursor,
            "expected contiguous base at {cursor}, got {} for {:?}",
            location.map_base, location.id
        );
        cursor += span;
    }
    assert_eq!(cursor, EXPECTED_TOTAL_CELL_SLOTS);
}

#[test]
fn co_located_occupants_remain_distinct() {
    let materialization = LocationMaterialization::canonical();
    assert_eq!(materialization.occupants.len(), EXPECTED_OCCUPANT_COUNT);

    let pirate_fleets: Vec<_> = materialization
        .occupants
        .iter()
        .filter(|occupant| occupant.kind == OccupantKind::PirateFleet)
        .collect();
    assert_eq!(pirate_fleets.len(), 10);

    let shared_cell = pirate_fleets[0].cell;
    let shared_location = pirate_fleets[0].location_id;
    assert!(
        pirate_fleets
            .iter()
            .all(|fleet| fleet.cell == shared_cell && fleet.location_id == shared_location),
        "canonical map places all pirate fleets on one galactic cell"
    );

    let at_cell = materialization.occupants_at(shared_location, shared_cell);
    let pirate_records: Vec<_> = at_cell
        .iter()
        .filter(|occupant| occupant.kind == OccupantKind::PirateFleet)
        .collect();
    assert_eq!(
        pirate_records.len(),
        10,
        "ten pirate fleets must remain ten distinct occupant records"
    );

    let distinct_ids: std::collections::HashSet<_> = pirate_records
        .iter()
        .map(|occupant| occupant.source_id.as_str())
        .collect();
    assert_eq!(distinct_ids.len(), 10);

    let constructed = vec![
        OccupantPlacement {
            kind: OccupantKind::Planet,
            owner: Owner::Terran,
            location_id: LocationId(1),
            cell: GridCell::new(3, 3),
            mobility: Mobility::Fixed,
            channels: Vec::new(),
            surface_location: Some(LocationId(14)),
            source_id: "constructed-planet".to_string(),
        },
        OccupantPlacement {
            kind: OccupantKind::PatrolFleet,
            owner: Owner::Terran,
            location_id: LocationId(1),
            cell: GridCell::new(3, 3),
            mobility: Mobility::Mover,
            channels: vec![ChannelDescriptor {
                kind: ChannelKind::PatrolPresence,
            }],
            surface_location: None,
            source_id: "constructed-patrol".to_string(),
        },
        OccupantPlacement {
            kind: OccupantKind::PirateFleet,
            owner: Owner::Pirate,
            location_id: LocationId(1),
            cell: GridCell::new(3, 3),
            mobility: Mobility::Mover,
            channels: vec![ChannelDescriptor {
                kind: ChannelKind::PiratePresence,
            }],
            surface_location: None,
            source_id: "constructed-pirate".to_string(),
        },
    ];
    assert_eq!(constructed.len(), 3);
    let unique_kinds: std::collections::HashSet<_> =
        constructed.iter().map(|occupant| occupant.kind).collect();
    assert_eq!(unique_kinds.len(), 3);
}

#[test]
fn occupants_retain_gen_owner() {
    let map = DressRehearsalMap::canonical();
    let materialization = LocationMaterialization::from_map(&map);

    for system in &map.systems {
        let planet = materialization
            .occupants
            .iter()
            .find(|occupant| occupant.source_id == format!("planet-{}", system.index))
            .expect("planet occupant");
        assert_eq!(planet.owner, system.planet.owner);

        let factory = materialization
            .occupants
            .iter()
            .find(|occupant| occupant.source_id == format!("factory-{}", system.index))
            .expect("factory occupant");
        assert_eq!(factory.owner, system.planet.surface.factory.owner);

        let pop = materialization
            .occupants
            .iter()
            .find(|occupant| occupant.source_id == format!("pop-{}", system.index))
            .expect("pop occupant");
        assert_eq!(pop.owner, system.planet.surface.pop_cohort.owner);

        if system.starport.is_some() {
            let starport = materialization
                .occupants
                .iter()
                .find(|occupant| occupant.source_id == format!("starport-{}", system.index))
                .expect("starport occupant");
            assert_eq!(starport.owner, system.owner);
        }
    }

    for fleet in &map.fleets {
        let occupant = materialization
            .occupants
            .iter()
            .find(|occupant| occupant.source_id == fleet.id)
            .expect("fleet occupant");
        assert_eq!(occupant.owner, fleet.owner);
        let expected_kind = match fleet.kind {
            FleetKind::Patrol => OccupantKind::PatrolFleet,
            FleetKind::PirateShip => OccupantKind::PirateFleet,
        };
        assert_eq!(occupant.kind, expected_kind);
    }
}

#[test]
fn channel_descriptors_present_per_tier() {
    let materialization = LocationMaterialization::canonical();

    for surface in materialization.locations_by_role(LocationRole::PlanetSurface) {
        assert!(channel_set_has_kind(&surface.channels, ChannelKind::Labor));
        assert!(channel_set_has_kind(
            &surface.channels,
            ChannelKind::Production
        ));
        assert_eq!(surface.channels.channels.len(), 2);
    }

    for system in materialization.locations_by_role(LocationRole::StarSystem) {
        assert!(channel_set_has_kind(
            &system.channels,
            ChannelKind::Disruption
        ));
        assert!(channel_set_has_kind(
            &system.channels,
            ChannelKind::ProductionPassThrough
        ));
        assert_eq!(system.channels.channels.len(), 2);
    }

    let galactic = materialization
        .locations
        .iter()
        .find(|location| location.role == LocationRole::Galactic)
        .expect("galactic location");
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::Disruption
    ));
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::FleetStrength(Owner::Terran)
    ));
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::FleetStrength(Owner::Pirate)
    ));
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::PatrolPresence
    ));
    assert!(channel_set_has_kind(
        &galactic.channels,
        ChannelKind::PiratePresence
    ));
    assert_eq!(galactic.channels.channels.len(), 5);
}

#[test]
fn planet_links_to_its_surface_location() {
    let materialization = LocationMaterialization::canonical();

    for planet in materialization
        .occupants
        .iter()
        .filter(|occupant| occupant.kind == OccupantKind::Planet)
    {
        let surface_id = planet
            .surface_location
            .expect("planet must link to a surface location");
        let surface = materialization
            .location(surface_id)
            .expect("surface location exists");
        assert_eq!(surface.role, LocationRole::PlanetSurface);

        let system = materialization
            .location(planet.location_id)
            .expect("planet's system location exists");
        assert_eq!(system.role, LocationRole::StarSystem);
        assert_eq!(surface.parent, Some(planet.location_id));

        let galactic_id = system.parent.expect("system parent is galactic");
        let galactic = materialization
            .location(galactic_id)
            .expect("galactic location");
        assert_eq!(galactic.role, LocationRole::Galactic);
        assert!(galactic.parent.is_none());
    }

    assert_eq!(
        materialization
            .occupants
            .iter()
            .filter(|occupant| occupant.kind == OccupantKind::Planet)
            .count(),
        13
    );
}
