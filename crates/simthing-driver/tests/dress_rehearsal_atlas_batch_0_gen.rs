#[path = "../src/dress_rehearsal_atlas_batch_0_gen.rs"]
mod dress_rehearsal_atlas_batch_0_gen;

use std::collections::HashSet;

use dress_rehearsal_atlas_batch_0_gen::{
    BuildingKind, DressRehearsalMap, FleetKind, GridCell, Owner, GALAXY_SIDE,
    PIRATE_STARTING_SHIPS, PIRATE_STARPORT_COUNT, PIRATE_SYSTEM_COUNT, PLANET_SURFACE_SIDE,
    SYSTEM_CENTER_CELL, SYSTEM_COUNT, SYSTEM_SIDE, TERRAN_PATROL_STARTING_SHIPS,
    TERRAN_STARPORT_COUNT, TERRAN_SYSTEM_COUNT,
};

#[test]
fn same_seed_produces_identical_descriptor() {
    let left = DressRehearsalMap::canonical();
    let right = DressRehearsalMap::canonical();
    assert_eq!(left, right);
}

#[test]
fn descriptor_shape_and_counts_hold() {
    let map = DressRehearsalMap::canonical();

    assert_eq!(map.galaxy_dims.width, GALAXY_SIDE);
    assert_eq!(map.galaxy_dims.height, GALAXY_SIDE);
    assert_eq!(map.systems.len(), SYSTEM_COUNT);
    assert_eq!(map.terran_systems().count(), TERRAN_SYSTEM_COUNT);
    assert_eq!(map.pirate_systems().count(), PIRATE_SYSTEM_COUNT);

    assert_eq!(
        map.starports()
            .filter(|starport| starport.owner == Owner::Terran)
            .count(),
        TERRAN_STARPORT_COUNT
    );
    assert_eq!(
        map.starports()
            .filter(|starport| starport.owner == Owner::Pirate)
            .count(),
        PIRATE_STARPORT_COUNT
    );

    assert_eq!(map.systems.len(), 13, "one planet per system");
    assert_eq!(
        map.systems
            .iter()
            .filter(|system| system.planet.surface.factory.kind == BuildingKind::FactoryDistrict)
            .count(),
        13
    );
    assert_eq!(
        map.systems
            .iter()
            .filter(|system| system.planet.surface.pop_cohort.kind == BuildingKind::PopCohort)
            .count(),
        13
    );

    assert_eq!(
        map.fleets_by_owner(Owner::Pirate).count(),
        PIRATE_STARTING_SHIPS
    );
    assert_eq!(
        map.fleets_by_owner(Owner::Terran).count(),
        TERRAN_PATROL_STARTING_SHIPS
    );
    assert_eq!(
        map.fleets
            .iter()
            .filter(|fleet| fleet.kind == FleetKind::PirateShip)
            .count(),
        PIRATE_STARTING_SHIPS
    );
    assert_eq!(
        map.fleets
            .iter()
            .filter(|fleet| fleet.kind == FleetKind::Patrol)
            .count(),
        TERRAN_PATROL_STARTING_SHIPS
    );
}

#[test]
fn system_and_surface_cells_are_bounded_and_unique_at_galactic_tier() {
    let map = DressRehearsalMap::canonical();
    let mut occupied = HashSet::new();

    for system in &map.systems {
        assert!(system.galactic_cell.in_bounds(GALAXY_SIDE));
        assert!(occupied.insert(system.galactic_cell));
        assert_eq!(system.system_dims.width, SYSTEM_SIDE);
        assert_eq!(system.system_dims.height, SYSTEM_SIDE);
        assert!(system.planet.system_cell.in_bounds(SYSTEM_SIDE));
        assert_eq!(system.planet.surface.dims.width, PLANET_SURFACE_SIDE);
        assert_eq!(system.planet.surface.dims.height, PLANET_SURFACE_SIDE);
        assert!(system
            .planet
            .surface
            .factory
            .cell
            .in_bounds(PLANET_SURFACE_SIDE));
        assert!(system
            .planet
            .surface
            .pop_cohort
            .cell
            .in_bounds(PLANET_SURFACE_SIDE));
        assert_ne!(
            system.planet.surface.factory.cell, system.planet.surface.pop_cohort.cell,
            "factory/pop occupants stay distinct in the descriptor"
        );
    }
}

#[test]
fn terran_spacing_and_pirate_adjacency_hold() {
    let map = DressRehearsalMap::canonical();
    let terran: Vec<_> = map.terran_systems().collect();

    for left in 0..terran.len() {
        for right in (left + 1)..terran.len() {
            let empty_cells_between = terran[left]
                .galactic_cell
                .empty_cells_between(terran[right].galactic_cell);
            assert!(
                (2..=4).contains(&empty_cells_between),
                "Terran systems {:?} and {:?} should have 2-4 empty cells between them, got {empty_cells_between}",
                terran[left].galactic_cell,
                terran[right].galactic_cell
            );
        }
    }

    for pirate in map.pirate_systems() {
        assert!(
            map.pirate_within_one_empty_cell_of_terran(pirate),
            "pirate system {:?} must be within 1 empty cell of a Terran system",
            pirate.galactic_cell
        );
    }
}

#[test]
fn starports_and_starting_fleets_use_owner_system_cells() {
    let map = DressRehearsalMap::canonical();

    for system in &map.systems {
        if let Some(starport) = &system.starport {
            assert_eq!(starport.kind, BuildingKind::Starport);
            assert_eq!(starport.cell, SYSTEM_CENTER_CELL);
            assert_eq!(starport.owner, system.owner);
        }
    }

    for fleet in &map.fleets {
        let system = &map.systems[fleet.system_index];
        assert_eq!(fleet.owner, system.owner);
        assert_eq!(fleet.galactic_cell, system.galactic_cell);
        assert_eq!(fleet.system_cell, GridCell::new(5, 5));
        assert!(system.starport.is_some(), "fleets start at owner starport systems");
    }
}
