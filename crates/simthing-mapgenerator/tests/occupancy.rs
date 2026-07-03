use simthing_mapgenerator::{
    CoreMask, LatticeCoord, MapGenRng, MapGenSeed, OccupancyError, OccupancyGrid, SquareLattice,
};

fn small_grid() -> (SquareLattice, CoreMask) {
    let lattice = SquareLattice::new(5).expect("5x5");
    let mask = CoreMask::new(2, 2, 0);
    (lattice, mask)
}

#[test]
fn occupancy_insert_free_cell_succeeds() {
    let (lattice, mask) = small_grid();
    let mut grid = OccupancyGrid::new(lattice, mask);
    let coord = LatticeCoord { col: 0, row: 0 };
    grid.try_insert(coord).expect("free cell");
    assert!(grid.contains(coord));
}

#[test]
fn occupancy_relocation_is_deterministic_for_same_seed() {
    let run = |seed: u64| {
        let lattice = SquareLattice::new(4).expect("4x4");
        let mask = CoreMask::new(1, 1, 0);
        let mut grid = OccupancyGrid::new(lattice, mask);
        let mut rng = MapGenRng::from_seed(MapGenSeed::new(seed));
        grid.try_insert(LatticeCoord { col: 0, row: 0 })
            .expect("anchor");
        grid.insert_or_relocate(LatticeCoord { col: 0, row: 0 }, &mut rng)
            .expect("relocate");
        grid.insert_or_relocate(LatticeCoord { col: 0, row: 0 }, &mut rng)
            .expect("relocate again");
        grid.occupied_coords().to_vec()
    };
    assert_eq!(run(99), run(99));
}

#[test]
fn occupancy_relocation_changes_with_different_seed_when_possible() {
    let run = |seed: u64| {
        let lattice = SquareLattice::new(6).expect("6x6");
        let mask = CoreMask::new(2, 2, 0);
        let mut grid = OccupancyGrid::new(lattice, mask);
        let mut rng = MapGenRng::from_seed(MapGenSeed::new(seed));
        for _ in 0..4 {
            grid.insert_or_relocate(LatticeCoord { col: 0, row: 0 }, &mut rng)
                .expect("place");
        }
        grid.occupied_coords().to_vec()
    };
    assert_ne!(run(1), run(2));
}

#[test]
fn occupancy_never_places_inside_core_mask() {
    let lattice = SquareLattice::new(7).expect("7x7");
    let mask = CoreMask::new(3, 3, 2);
    let mut grid = OccupancyGrid::new(lattice, mask);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(5));
    for _ in 0..10 {
        let coord = grid.insert_next(&mut rng).expect("place");
        assert!(grid.core_mask().is_placeable(coord));
    }
}

#[test]
fn occupancy_errors_when_exhausted() {
    let lattice = SquareLattice::new(2).expect("2x2");
    let mask = CoreMask::new(0, 0, 0);
    let mut grid = OccupancyGrid::new(lattice, mask);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(1));
    for _ in 0..4 {
        grid.insert_next(&mut rng).expect("four cells");
    }
    assert_eq!(
        grid.insert_next(&mut rng),
        Err(OccupancyError::LatticeExhausted)
    );
}

#[test]
fn occupied_cells_are_stably_ordered() {
    let lattice = SquareLattice::new(3).expect("3x3");
    let mask = CoreMask::new(1, 1, 0);
    let mut grid = OccupancyGrid::new(lattice, mask);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(11));
    for _ in 0..5 {
        grid.insert_next(&mut rng).expect("place");
    }
    let first = grid.occupied_coords().to_vec();
    let second = grid.occupied_coords().to_vec();
    assert_eq!(first, second);
}
