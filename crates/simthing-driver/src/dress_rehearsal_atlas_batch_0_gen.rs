pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_ID: &str = "ATLAS-BATCH-0-GEN";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_GEN_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - deterministic static descriptor only; no runtime wiring";

pub const DRESS_REHEARSAL_DEFAULT_SEED: u64 = 0x0080_2000;
pub const GALAXY_SIDE: u32 = 20;
pub const SYSTEM_SIDE: u32 = 10;
pub const PLANET_SURFACE_SIDE: u32 = 10;
pub const TERRAN_SYSTEM_COUNT: usize = 10;
pub const PIRATE_SYSTEM_COUNT: usize = 3;
pub const SYSTEM_COUNT: usize = TERRAN_SYSTEM_COUNT + PIRATE_SYSTEM_COUNT;
pub const TERRAN_STARPORT_COUNT: usize = 3;
pub const PIRATE_STARPORT_COUNT: usize = 1;
pub const PIRATE_STARTING_SHIPS: usize = 10;
pub const TERRAN_PATROL_STARTING_SHIPS: usize = 3;
pub const SYSTEM_CENTER_CELL: GridCell = GridCell { x: 5, y: 5 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Owner {
    Terran,
    Pirate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemKind {
    Terran,
    Pirate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FleetKind {
    Patrol,
    PirateShip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildingKind {
    Starport,
    FactoryDistrict,
    PopCohort,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: u32,
    pub y: u32,
}

impl GridCell {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn in_bounds(self, side: u32) -> bool {
        self.x < side && self.y < side
    }

    pub fn chebyshev_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    pub fn empty_cells_between(self, other: Self) -> u32 {
        self.chebyshev_distance(other).saturating_sub(1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridDims {
    pub width: u32,
    pub height: u32,
}

impl GridDims {
    pub const fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }

    pub const fn cell_count(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildingPlacement {
    pub kind: BuildingKind,
    pub owner: Owner,
    pub cell: GridCell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanetSurfaceDescriptor {
    pub owner: Owner,
    pub dims: GridDims,
    pub factory: BuildingPlacement,
    pub pop_cohort: BuildingPlacement,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanetDescriptor {
    pub owner: Owner,
    pub system_cell: GridCell,
    pub surface: PlanetSurfaceDescriptor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemDescriptor {
    pub index: usize,
    pub kind: SystemKind,
    pub owner: Owner,
    pub galactic_cell: GridCell,
    pub system_dims: GridDims,
    pub planet: PlanetDescriptor,
    pub starport: Option<BuildingPlacement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FleetPlacement {
    pub id: String,
    pub kind: FleetKind,
    pub owner: Owner,
    pub system_index: usize,
    pub galactic_cell: GridCell,
    pub system_cell: GridCell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DressRehearsalMap {
    pub seed: u64,
    pub galaxy_dims: GridDims,
    pub systems: Vec<SystemDescriptor>,
    pub fleets: Vec<FleetPlacement>,
}

impl DressRehearsalMap {
    pub fn canonical() -> Self {
        Self::from_seed(DRESS_REHEARSAL_DEFAULT_SEED)
    }

    pub fn from_seed(seed: u64) -> Self {
        let transform = Symmetry::from_seed(seed);
        let mut systems = Vec::with_capacity(SYSTEM_COUNT);

        for (index, base_cell) in TERRAN_BASE_CELLS.iter().copied().enumerate() {
            let owner = Owner::Terran;
            systems.push(build_system(
                index,
                SystemKind::Terran,
                owner,
                transform.apply(base_cell, GALAXY_SIDE),
                transform,
                seed,
                terran_starport_indices().contains(&index),
            ));
        }

        for (pirate_offset, base_cell) in PIRATE_BASE_CELLS.iter().copied().enumerate() {
            let index = TERRAN_SYSTEM_COUNT + pirate_offset;
            let owner = Owner::Pirate;
            systems.push(build_system(
                index,
                SystemKind::Pirate,
                owner,
                transform.apply(base_cell, GALAXY_SIDE),
                transform,
                seed,
                pirate_offset == 0,
            ));
        }

        let fleets = build_fleets(&systems);
        Self {
            seed,
            galaxy_dims: GridDims::square(GALAXY_SIDE),
            systems,
            fleets,
        }
    }

    pub fn terran_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
        self.systems
            .iter()
            .filter(|system| system.owner == Owner::Terran)
    }

    pub fn pirate_systems(&self) -> impl Iterator<Item = &SystemDescriptor> {
        self.systems
            .iter()
            .filter(|system| system.owner == Owner::Pirate)
    }

    pub fn starports(&self) -> impl Iterator<Item = &BuildingPlacement> {
        self.systems
            .iter()
            .filter_map(|system| system.starport.as_ref())
    }

    pub fn fleets_by_owner(&self, owner: Owner) -> impl Iterator<Item = &FleetPlacement> {
        self.fleets.iter().filter(move |fleet| fleet.owner == owner)
    }

    pub fn minimum_terran_empty_spacing(&self) -> Option<u32> {
        let terran: Vec<_> = self.terran_systems().collect();
        let mut min_spacing = None;
        for left in 0..terran.len() {
            for right in (left + 1)..terran.len() {
                let spacing = terran[left]
                    .galactic_cell
                    .empty_cells_between(terran[right].galactic_cell);
                min_spacing = Some(min_spacing.map_or(spacing, |current: u32| current.min(spacing)));
            }
        }
        min_spacing
    }

    pub fn pirate_within_one_empty_cell_of_terran(&self, pirate: &SystemDescriptor) -> bool {
        self.terran_systems().any(|terran| {
            let distance = pirate.galactic_cell.chebyshev_distance(terran.galactic_cell);
            distance > 0 && pirate.galactic_cell.empty_cells_between(terran.galactic_cell) <= 1
        })
    }
}

const TERRAN_BASE_CELLS: [GridCell; TERRAN_SYSTEM_COUNT] = [
    GridCell::new(2, 3),
    GridCell::new(6, 3),
    GridCell::new(10, 3),
    GridCell::new(14, 3),
    GridCell::new(18, 3),
    GridCell::new(2, 12),
    GridCell::new(6, 12),
    GridCell::new(10, 12),
    GridCell::new(14, 12),
    GridCell::new(18, 12),
];

const PIRATE_BASE_CELLS: [GridCell; PIRATE_SYSTEM_COUNT] = [
    GridCell::new(4, 5),
    GridCell::new(12, 10),
    GridCell::new(16, 10),
];

fn terran_starport_indices() -> [usize; TERRAN_STARPORT_COUNT] {
    [0, 4, 8]
}

fn build_system(
    index: usize,
    kind: SystemKind,
    owner: Owner,
    galactic_cell: GridCell,
    transform: Symmetry,
    seed: u64,
    has_starport: bool,
) -> SystemDescriptor {
    let planet_cell = deterministic_subgrid_cell(seed, index as u64, 0x51A7_E11A, transform);
    let factory_cell = deterministic_subgrid_cell(seed, index as u64, 0xFACA_70A1, transform);
    let mut pop_cell = deterministic_subgrid_cell(seed, index as u64, 0xC0B0_0001, transform);
    if pop_cell == factory_cell {
        pop_cell = GridCell::new((pop_cell.x + 1) % PLANET_SURFACE_SIDE, pop_cell.y);
    }

    let starport = has_starport.then_some(BuildingPlacement {
        kind: BuildingKind::Starport,
        owner,
        cell: SYSTEM_CENTER_CELL,
    });

    SystemDescriptor {
        index,
        kind,
        owner,
        galactic_cell,
        system_dims: GridDims::square(SYSTEM_SIDE),
        planet: PlanetDescriptor {
            owner,
            system_cell: planet_cell,
            surface: PlanetSurfaceDescriptor {
                owner,
                dims: GridDims::square(PLANET_SURFACE_SIDE),
                factory: BuildingPlacement {
                    kind: BuildingKind::FactoryDistrict,
                    owner,
                    cell: factory_cell,
                },
                pop_cohort: BuildingPlacement {
                    kind: BuildingKind::PopCohort,
                    owner,
                    cell: pop_cell,
                },
            },
        },
        starport,
    }
}

fn build_fleets(systems: &[SystemDescriptor]) -> Vec<FleetPlacement> {
    let mut fleets = Vec::with_capacity(PIRATE_STARTING_SHIPS + TERRAN_PATROL_STARTING_SHIPS);

    let terran_starport_systems: Vec<_> = systems
        .iter()
        .filter(|system| system.owner == Owner::Terran && system.starport.is_some())
        .collect();
    for (patrol_index, system) in terran_starport_systems.iter().enumerate() {
        fleets.push(FleetPlacement {
            id: format!("terran-patrol-{patrol_index:02}"),
            kind: FleetKind::Patrol,
            owner: Owner::Terran,
            system_index: system.index,
            galactic_cell: system.galactic_cell,
            system_cell: SYSTEM_CENTER_CELL,
        });
    }

    let pirate_starport_system = systems
        .iter()
        .find(|system| system.owner == Owner::Pirate && system.starport.is_some())
        .expect("canonical dress-rehearsal map must include one pirate starport");
    for pirate_index in 0..PIRATE_STARTING_SHIPS {
        fleets.push(FleetPlacement {
            id: format!("pirate-ship-{pirate_index:02}"),
            kind: FleetKind::PirateShip,
            owner: Owner::Pirate,
            system_index: pirate_starport_system.index,
            galactic_cell: pirate_starport_system.galactic_cell,
            system_cell: SYSTEM_CENTER_CELL,
        });
    }

    fleets
}

fn deterministic_subgrid_cell(seed: u64, index: u64, salt: u64, transform: Symmetry) -> GridCell {
    let mixed = stable_mix(seed ^ index.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt);
    let cell = GridCell::new(
        (mixed % u64::from(SYSTEM_SIDE)) as u32,
        ((mixed / 17) % u64::from(SYSTEM_SIDE)) as u32,
    );
    transform.apply(cell, SYSTEM_SIDE)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Symmetry {
    Identity,
    MirrorX,
    MirrorY,
    Rotate180,
}

impl Symmetry {
    fn from_seed(seed: u64) -> Self {
        match stable_mix(seed) & 0b11 {
            0 => Self::Identity,
            1 => Self::MirrorX,
            2 => Self::MirrorY,
            _ => Self::Rotate180,
        }
    }

    fn apply(self, cell: GridCell, side: u32) -> GridCell {
        let max = side - 1;
        match self {
            Self::Identity => cell,
            Self::MirrorX => GridCell::new(max - cell.x, cell.y),
            Self::MirrorY => GridCell::new(cell.x, max - cell.y),
            Self::Rotate180 => GridCell::new(max - cell.x, max - cell.y),
        }
    }
}

fn stable_mix(mut value: u64) -> u64 {
    value ^= value >> 33;
    value = value.wrapping_mul(0xff51_afd7_ed55_8ccd);
    value ^= value >> 33;
    value = value.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    value ^ (value >> 33)
}
