use simthing_mapgenerator::{
    CoreMask, LatticeCoord, MapGenRng, MapGenSeed, OccupancyError, OccupancyGrid, SquareLattice,
};

fn small_grid() -> (SquareLattice, CoreMask) {
    let lattice = SquareLattice::new(5).expect("5x5");
    let mask = CoreMask::new(2, 2, 0);
    (lattice, mask)
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
