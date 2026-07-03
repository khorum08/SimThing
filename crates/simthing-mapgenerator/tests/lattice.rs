use simthing_mapgenerator::{CoreMask, LatticeCoord, MapGenRng, MapGenSeed, SquareLattice};

#[test]
fn same_seed_rng_sequence_is_stable() {
    let mut a = MapGenRng::from_seed(MapGenSeed::new(4242));
    let mut b = MapGenRng::from_seed(MapGenSeed::new(4242));
    for _ in 0..16 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn different_seed_rng_sequence_differs() {
    let mut a = MapGenRng::from_seed(MapGenSeed::new(1));
    let mut b = MapGenRng::from_seed(MapGenSeed::new(2));
    assert_ne!(a.next_u64(), b.next_u64());
}

#[test]
fn square_lattice_bounds_checks() {
    let lattice = SquareLattice::new(8).expect("8x8");
    assert!(lattice.contains(LatticeCoord { col: 0, row: 0 }));
    assert!(lattice.contains(LatticeCoord { col: 7, row: 7 }));
    assert!(!lattice.contains(LatticeCoord { col: 8, row: 0 }));
    assert!(!lattice.contains(LatticeCoord { col: 0, row: 8 }));
}

#[test]
fn square_lattice_index_round_trips() {
    let lattice = SquareLattice::new(5).expect("5x5");
    for index in 0..lattice.cell_count() {
        let coord = lattice.from_index(index).expect("valid index");
        assert_eq!(lattice.index(coord), Some(index));
    }
}

#[test]
fn core_mask_excludes_center_cells() {
    let lattice = SquareLattice::new(9).expect("9x9");
    let mask = lattice.core_mask_from_scale(120.0, 450.0);
    let center = lattice.center();
    assert!(mask.is_masked(center));
}

#[test]
fn core_mask_allows_outer_cells() {
    let mask = CoreMask::new(4, 4, 1);
    assert!(mask.is_placeable(LatticeCoord { col: 0, row: 0 }));
    assert!(!mask.is_placeable(LatticeCoord { col: 4, row: 4 }));
}

#[test]
fn square_lattice_iter_coords_are_row_major_stable() {
    let lattice = SquareLattice::new(3).expect("3x3");
    let coords: Vec<_> = lattice.iter_coords().collect();
    assert_eq!(
        coords,
        vec![
            LatticeCoord { col: 0, row: 0 },
            LatticeCoord { col: 1, row: 0 },
            LatticeCoord { col: 2, row: 0 },
            LatticeCoord { col: 0, row: 1 },
            LatticeCoord { col: 1, row: 1 },
            LatticeCoord { col: 2, row: 1 },
            LatticeCoord { col: 0, row: 2 },
            LatticeCoord { col: 1, row: 2 },
            LatticeCoord { col: 2, row: 2 },
        ]
    );
}
