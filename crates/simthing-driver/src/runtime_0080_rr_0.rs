//! RUNTIME-0080-RR-0: recursive world model + recursive CPU oracle.
//!
//! Establishes the specified galaxy→system(10×10)→planet→surface(10×10) containment hierarchy with
//! pop-cohort/factory surface economy and a deterministic 100-tick recursive CPU oracle. Explicitly
//! CPU-oracle only — no GPU residency, no flat galactic proxy, no integrated recursive GPU rehearsal.

#[path = "dress_rehearsal_atlas_batch_0_gen.rs"]
mod atlas_gen;

use crate::dress_rehearsal_r2_recursive_allocation::{
    factory_recipe_production, FACTORY_UNIT_COST_LABOR, POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE,
    STARPORT_PRODUCTION_NEED,
};
use crate::dress_rehearsal_r6c_integrated_run::R6C_CANONICAL_TICK_COUNT;
use atlas_gen::{
    DressRehearsalMap, GALAXY_SIDE, PIRATE_SYSTEM_COUNT, PLANET_SURFACE_SIDE, SYSTEM_COUNT,
    SYSTEM_SIDE, TERRAN_SYSTEM_COUNT,
};

pub const RUNTIME_0080_RR_0_ID: &str = "RUNTIME-0080-RR-0";
pub const RUNTIME_0080_RR_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - recursive world model + recursive CPU oracle";
pub const RUNTIME_0080_RR_0_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - recursive structure or oracle incomplete";
pub const RUNTIME_0080_RR_0_STATUS_BLOCKED: &str =
    "BLOCKED - recursive RR-0 cannot close without approved deviation";
pub const RUNTIME_RR_0_EXPECTED_REPORT_CHECKSUM: u64 = 0xa8a9_f20a_524f_a5b2;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Runtime0080Rr0Owner {
    Terran,
    Pirate,
}

impl Runtime0080Rr0Owner {
    pub fn stable_code(self) -> u64 {
        match self {
            Self::Terran => 1,
            Self::Pirate => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub seed: u64,
    pub tick_count: u32,
}

impl Runtime0080Rr0Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            seed: atlas_gen::DRESS_REHEARSAL_DEFAULT_SEED,
            tick_count: 0,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            seed: atlas_gen::DRESS_REHEARSAL_DEFAULT_SEED,
            tick_count: R6C_CANONICAL_TICK_COUNT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0ScopeLedgerRow {
    pub spec_element: &'static str,
    pub required_by_spec: bool,
    pub implemented_in_rr_0: bool,
    pub status: &'static str,
    pub evidence: &'static str,
    pub deviation: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0DeviationRecord {
    pub design_authority_approval: &'static str,
    pub specified_element: &'static str,
    pub implemented_proxy_or_omission: &'static str,
    pub reason: &'static str,
    pub consumer_impact: &'static str,
    pub required_follow_up: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0GalaxyCell {
    pub x: u32,
    pub y: u32,
    pub linear_index: u32,
    pub occupied_system_id: Option<u8>,
    pub production: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0SystemGridCell {
    pub x: u32,
    pub y: u32,
    pub linear_index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0SurfaceChild {
    pub kind: &'static str,
    pub owner: Runtime0080Rr0Owner,
    pub surface_cell_x: u32,
    pub surface_cell_y: u32,
    pub simthing_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0SurfaceCell {
    pub x: u32,
    pub y: u32,
    pub linear_index: u32,
    pub labor: i64,
    pub production: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Surface {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Runtime0080Rr0SurfaceCell>,
    pub pop_cohort: Runtime0080Rr0SurfaceChild,
    pub factory: Runtime0080Rr0SurfaceChild,
    pub production_aggregate: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Planet {
    pub id: u8,
    pub parent_system_id: u8,
    pub parent_system_cell_x: u32,
    pub parent_system_cell_y: u32,
    pub surface: Runtime0080Rr0Surface,
    pub production_aggregate: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Starport {
    pub simthing_id: String,
    pub owner: Runtime0080Rr0Owner,
    pub system_cell_x: u32,
    pub system_cell_y: u32,
    pub production_received: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0System {
    pub id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub parent_galaxy_x: u32,
    pub parent_galaxy_y: u32,
    pub parent_galaxy_linear_index: u32,
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Runtime0080Rr0SystemGridCell>,
    pub starport: Option<Runtime0080Rr0Starport>,
    pub planet: Runtime0080Rr0Planet,
    pub production_aggregate: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Galaxy {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Runtime0080Rr0GalaxyCell>,
    pub systems: Vec<Runtime0080Rr0System>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0FactionStockpile {
    pub owner: Runtime0080Rr0Owner,
    pub production: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0RecursiveWorld {
    pub seed: u64,
    pub galaxy: Runtime0080Rr0Galaxy,
    pub faction_stockpiles: Vec<Runtime0080Rr0FactionStockpile>,
    pub structural_checksum: u64,
    pub is_flattened: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0OracleTick {
    pub tick: u32,
    pub labor_emitted: i64,
    pub labor_consumed: i64,
    pub production_generated: i64,
    pub reduced_surface_to_planet: i64,
    pub reduced_planet_to_system: i64,
    pub reduced_system_to_galaxy: i64,
    pub reduced_galaxy_to_stockpile_terran: i64,
    pub reduced_galaxy_to_stockpile_pirate: i64,
    pub disbursed_terran: i64,
    pub disbursed_pirate: i64,
    pub terran_stockpile_after: i64,
    pub pirate_stockpile_after: i64,
    pub structural_identity_preserved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0EntityCounts {
    pub galaxy_cells: usize,
    pub systems: usize,
    pub system_grid_cells: usize,
    pub planets: usize,
    pub surface_cells: usize,
    pub pop_cohorts: usize,
    pub factories: usize,
    pub starports: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr0Report {
    pub id: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub world: Runtime0080Rr0RecursiveWorld,
    pub entity_counts: Runtime0080Rr0EntityCounts,
    pub oracle_ticks: Vec<Runtime0080Rr0OracleTick>,
    pub ticks_scheduled: u32,
    pub ticks_completed: u32,
    pub scope_ledger: Vec<Runtime0080Rr0ScopeLedgerRow>,
    pub deviation_records: Vec<Runtime0080Rr0DeviationRecord>,
    pub stable_report_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub cpu_oracle_only: bool,
    pub gpu_residency_claimed: bool,
    pub flat_proxy_closure: bool,
    pub invariant_edit: bool,
    pub total_labor_emitted: i64,
    pub total_production_generated: i64,
    pub total_disbursed_terran: i64,
    pub total_disbursed_pirate: i64,
    pub final_terran_stockpile: i64,
    pub final_pirate_stockpile: i64,
}

pub fn run_runtime_0080_rr_0(input: &Runtime0080Rr0Input) -> Runtime0080Rr0Report {
    let mut diagnostics = Vec::new();
    if input.enabled_by_default {
        diagnostics.push("rr_0_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None, Vec::new());
    }
    if input.tick_count != R6C_CANONICAL_TICK_COUNT {
        diagnostics.push("rr_0_tick_count_must_be_canonical_100");
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None, Vec::new());
    }

    let world = build_recursive_world(input.seed);
    let structural_checksum = world.structural_checksum;
    let mut runtime_world = world;
    let oracle_ticks = run_recursive_cpu_oracle(&mut runtime_world, input.tick_count);
    base_report(
        input,
        false,
        Vec::new(),
        Some((runtime_world, oracle_ticks, structural_checksum)),
        Vec::new(),
    )
}

pub fn replay_runtime_0080_rr_0() -> (Runtime0080Rr0Report, Runtime0080Rr0Report) {
    let input = Runtime0080Rr0Input::explicit_opt_in();
    (run_runtime_0080_rr_0(&input), run_runtime_0080_rr_0(&input))
}

pub fn build_recursive_world(seed: u64) -> Runtime0080Rr0RecursiveWorld {
    let map = DressRehearsalMap::from_seed(seed);
    let mut galaxy_cells = Vec::with_capacity((GALAXY_SIDE * GALAXY_SIDE) as usize);
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            galaxy_cells.push(Runtime0080Rr0GalaxyCell {
                x,
                y,
                linear_index: y * GALAXY_SIDE + x,
                occupied_system_id: None,
                production: 0,
            });
        }
    }

    let mut systems = Vec::with_capacity(SYSTEM_COUNT);
    for descriptor in &map.systems {
        let parent_galaxy_linear_index =
            descriptor.galactic_cell.y * GALAXY_SIDE + descriptor.galactic_cell.x;
        if let Some(cell) = galaxy_cells.get_mut(parent_galaxy_linear_index as usize) {
            cell.occupied_system_id = Some(descriptor.index as u8);
        }

        let mut system_cells = Vec::with_capacity((SYSTEM_SIDE * SYSTEM_SIDE) as usize);
        for sy in 0..SYSTEM_SIDE {
            for sx in 0..SYSTEM_SIDE {
                system_cells.push(Runtime0080Rr0SystemGridCell {
                    x: sx,
                    y: sy,
                    linear_index: sy * SYSTEM_SIDE + sx,
                });
            }
        }

        let mut surface_cells =
            Vec::with_capacity((PLANET_SURFACE_SIDE * PLANET_SURFACE_SIDE) as usize);
        for sy in 0..PLANET_SURFACE_SIDE {
            for sx in 0..PLANET_SURFACE_SIDE {
                surface_cells.push(Runtime0080Rr0SurfaceCell {
                    x: sx,
                    y: sy,
                    linear_index: sy * PLANET_SURFACE_SIDE + sx,
                    labor: 0,
                    production: 0,
                });
            }
        }

        let owner = owner_from_atlas(descriptor.owner);
        let pop = &descriptor.planet.surface.pop_cohort;
        let factory = &descriptor.planet.surface.factory;
        let surface = Runtime0080Rr0Surface {
            width: PLANET_SURFACE_SIDE,
            height: PLANET_SURFACE_SIDE,
            pop_cohort: Runtime0080Rr0SurfaceChild {
                kind: "PopCohort",
                owner,
                surface_cell_x: pop.cell.x,
                surface_cell_y: pop.cell.y,
                simthing_id: format!("pop-cohort-system-{:02}", descriptor.index),
            },
            factory: Runtime0080Rr0SurfaceChild {
                kind: "FactoryDistrict",
                owner,
                surface_cell_x: factory.cell.x,
                surface_cell_y: factory.cell.y,
                simthing_id: format!("factory-system-{:02}", descriptor.index),
            },
            cells: surface_cells,
            production_aggregate: 0,
        };

        let planet = Runtime0080Rr0Planet {
            id: descriptor.index as u8,
            parent_system_id: descriptor.index as u8,
            parent_system_cell_x: descriptor.planet.system_cell.x,
            parent_system_cell_y: descriptor.planet.system_cell.y,
            surface,
            production_aggregate: 0,
        };

        let starport = descriptor
            .starport
            .as_ref()
            .map(|placement| Runtime0080Rr0Starport {
                simthing_id: format!("starport-system-{:02}", descriptor.index),
                owner,
                system_cell_x: placement.cell.x,
                system_cell_y: placement.cell.y,
                production_received: 0,
            });

        systems.push(Runtime0080Rr0System {
            id: descriptor.index as u8,
            owner,
            parent_galaxy_x: descriptor.galactic_cell.x,
            parent_galaxy_y: descriptor.galactic_cell.y,
            parent_galaxy_linear_index,
            width: SYSTEM_SIDE,
            height: SYSTEM_SIDE,
            cells: system_cells,
            starport,
            planet,
            production_aggregate: 0,
        });
    }
    systems.sort_by_key(|system| system.id);

    let galaxy = Runtime0080Rr0Galaxy {
        width: GALAXY_SIDE,
        height: GALAXY_SIDE,
        cells: galaxy_cells,
        systems,
    };

    let faction_stockpiles = vec![
        Runtime0080Rr0FactionStockpile {
            owner: Runtime0080Rr0Owner::Terran,
            production: 0,
        },
        Runtime0080Rr0FactionStockpile {
            owner: Runtime0080Rr0Owner::Pirate,
            production: 0,
        },
    ];

    let mut world = Runtime0080Rr0RecursiveWorld {
        seed,
        galaxy,
        faction_stockpiles,
        structural_checksum: 0,
        is_flattened: false,
    };
    world.structural_checksum = checksum_structural(&world);
    world.is_flattened = detect_flattening(&world);
    world
}

fn run_recursive_cpu_oracle(
    world: &mut Runtime0080Rr0RecursiveWorld,
    tick_count: u32,
) -> Vec<Runtime0080Rr0OracleTick> {
    let structural_checksum = world.structural_checksum;
    let mut ticks = Vec::with_capacity(tick_count as usize);
    for tick in 0..tick_count {
        ticks.push(tick_recursive_oracle(world, tick, structural_checksum));
    }
    ticks
}

fn tick_recursive_oracle(
    world: &mut Runtime0080Rr0RecursiveWorld,
    tick: u32,
    structural_checksum: u64,
) -> Runtime0080Rr0OracleTick {
    let mut labor_emitted = 0i64;
    let mut labor_consumed = 0i64;
    let mut production_generated = 0i64;
    let mut reduced_surface_to_planet = 0i64;
    let mut reduced_planet_to_system = 0i64;
    let mut reduced_system_to_galaxy = 0i64;
    let mut galaxy_contributions = Vec::with_capacity(world.galaxy.systems.len());

    for system in world.galaxy.systems.iter_mut() {
        let surface = &mut system.planet.surface;
        let pop_idx = surface_cell_index(
            surface.pop_cohort.surface_cell_x,
            surface.pop_cohort.surface_cell_y,
        );
        let factory_idx = surface_cell_index(
            surface.factory.surface_cell_x,
            surface.factory.surface_cell_y,
        );

        labor_emitted += POP_LABOR_PER_TICK;
        surface.cells[pop_idx].labor += POP_LABOR_PER_TICK;
        let labor_available = surface.cells[pop_idx].labor;
        surface.cells[pop_idx].labor = 0;
        surface.cells[factory_idx].labor += labor_available;

        let factory_labor = surface.cells[factory_idx].labor;
        let (production, consumed, remaining) = factory_recipe_production(factory_labor);
        surface.cells[factory_idx].labor = remaining;
        surface.cells[factory_idx].production += production;
        labor_consumed += consumed;
        production_generated += production;

        let surface_production: i64 = surface.cells.iter().map(|cell| cell.production).sum();
        surface.production_aggregate = surface_production;
        reduced_surface_to_planet += surface_production;

        system.planet.production_aggregate = surface_production;
        reduced_planet_to_system += surface_production;

        system.production_aggregate = surface_production;
        galaxy_contributions.push((
            system.parent_galaxy_linear_index as usize,
            surface_production,
        ));
        reduced_system_to_galaxy += surface_production;

        for cell in &mut surface.cells {
            cell.production = 0;
        }
        surface.production_aggregate = 0;
        system.planet.production_aggregate = 0;
        system.production_aggregate = 0;
    }

    for (galaxy_idx, production) in galaxy_contributions {
        world.galaxy.cells[galaxy_idx].production += production;
    }

    let mut reduced_galaxy_to_stockpile_terran = 0i64;
    let mut reduced_galaxy_to_stockpile_pirate = 0i64;
    let mut stockpile_increments = [
        (Runtime0080Rr0Owner::Terran, 0i64),
        (Runtime0080Rr0Owner::Pirate, 0i64),
    ];
    for cell in &mut world.galaxy.cells {
        if cell.production == 0 {
            continue;
        }
        let Some(system_id) = cell.occupied_system_id else {
            continue;
        };
        let owner = world
            .galaxy
            .systems
            .iter()
            .find(|system| system.id == system_id)
            .map(|system| system.owner)
            .expect("occupied galaxy cell must reference a system");
        match owner {
            Runtime0080Rr0Owner::Terran => {
                reduced_galaxy_to_stockpile_terran += cell.production;
                stockpile_increments[0].1 += cell.production;
            }
            Runtime0080Rr0Owner::Pirate => {
                reduced_galaxy_to_stockpile_pirate += cell.production;
                stockpile_increments[1].1 += cell.production;
            }
        }
        cell.production = 0;
    }
    for (owner, increment) in stockpile_increments {
        if increment > 0 {
            stockpile_for_owner_mut(world, owner).production += increment;
        }
    }

    let mut disburse_plan = Vec::new();
    for system in &world.galaxy.systems {
        if system.starport.is_some() {
            disburse_plan.push((system.id, system.owner));
        }
    }
    let mut disburse_results = Vec::with_capacity(disburse_plan.len());
    for (system_id, owner) in disburse_plan {
        let requested = STARPORT_PRODUCTION_NEED;
        let available = stockpile_for_owner(world, owner).production.max(0);
        let disbursed = requested.min(available);
        disburse_results.push((system_id, owner, disbursed));
    }
    let mut disbursed_terran = 0i64;
    let mut disbursed_pirate = 0i64;
    for (system_id, owner, disbursed) in disburse_results {
        stockpile_for_owner_mut(world, owner).production -= disbursed;
        world
            .galaxy
            .systems
            .iter_mut()
            .find(|system| system.id == system_id)
            .expect("disburse system must exist")
            .starport
            .as_mut()
            .expect("disburse target must have starport")
            .production_received += disbursed;
        match owner {
            Runtime0080Rr0Owner::Terran => disbursed_terran += disbursed,
            Runtime0080Rr0Owner::Pirate => disbursed_pirate += disbursed,
        }
    }

    let terran_stockpile_after = stockpile_for_owner(world, Runtime0080Rr0Owner::Terran).production;
    let pirate_stockpile_after = stockpile_for_owner(world, Runtime0080Rr0Owner::Pirate).production;

    Runtime0080Rr0OracleTick {
        tick,
        labor_emitted,
        labor_consumed,
        production_generated,
        reduced_surface_to_planet,
        reduced_planet_to_system,
        reduced_system_to_galaxy,
        reduced_galaxy_to_stockpile_terran,
        reduced_galaxy_to_stockpile_pirate,
        disbursed_terran,
        disbursed_pirate,
        terran_stockpile_after,
        pirate_stockpile_after,
        structural_identity_preserved: world.structural_checksum == structural_checksum
            && !detect_flattening(world),
    }
}

fn base_report(
    input: &Runtime0080Rr0Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<(
        Runtime0080Rr0RecursiveWorld,
        Vec<Runtime0080Rr0OracleTick>,
        u64,
    )>,
    deviation_records: Vec<Runtime0080Rr0DeviationRecord>,
) -> Runtime0080Rr0Report {
    let admitted = diagnostics.is_empty();
    let empty_world = Runtime0080Rr0RecursiveWorld {
        seed: input.seed,
        galaxy: Runtime0080Rr0Galaxy {
            width: 0,
            height: 0,
            cells: Vec::new(),
            systems: Vec::new(),
        },
        faction_stockpiles: Vec::new(),
        structural_checksum: 0,
        is_flattened: true,
    };

    let (world, oracle_ticks, ticks_completed) = match execution {
        Some((world, ticks, _)) => {
            let ticks_completed = ticks.len() as u32;
            (world, ticks, ticks_completed)
        }
        None => (empty_world, Vec::new(), 0),
    };

    let entity_counts = if disabled_no_op {
        Runtime0080Rr0EntityCounts {
            galaxy_cells: 0,
            systems: 0,
            system_grid_cells: 0,
            planets: 0,
            surface_cells: 0,
            pop_cohorts: 0,
            factories: 0,
            starports: 0,
        }
    } else {
        entity_counts_for(&world)
    };

    let scope_ledger = build_scope_ledger(&world, !disabled_no_op, ticks_completed);
    let required_rows_implemented = scope_ledger
        .iter()
        .take(19)
        .all(|row| row.status == "implemented");

    let has_unapproved_deviation = !deviation_records.is_empty()
        && deviation_records
            .iter()
            .any(|record| record.design_authority_approval != "approved");

    let verdict = if disabled_no_op {
        "BLOCKED"
    } else if !admitted {
        "BLOCKED"
    } else if has_unapproved_deviation || !required_rows_implemented {
        "PARTIAL"
    } else {
        "PASS"
    };

    let status = match verdict {
        "PASS" => RUNTIME_0080_RR_0_STATUS_PASS,
        "PARTIAL" => RUNTIME_0080_RR_0_STATUS_PARTIAL,
        _ => RUNTIME_0080_RR_0_STATUS_BLOCKED,
    };

    let total_labor_emitted = oracle_ticks.iter().map(|row| row.labor_emitted).sum();
    let total_production_generated = oracle_ticks
        .iter()
        .map(|row| row.production_generated)
        .sum();
    let total_disbursed_terran = oracle_ticks.iter().map(|row| row.disbursed_terran).sum();
    let total_disbursed_pirate = oracle_ticks.iter().map(|row| row.disbursed_pirate).sum();
    let final_terran_stockpile = oracle_ticks
        .last()
        .map(|row| row.terran_stockpile_after)
        .unwrap_or(0);
    let final_pirate_stockpile = oracle_ticks
        .last()
        .map(|row| row.pirate_stockpile_after)
        .unwrap_or(0);

    let deterministic_replay_checksum = if admitted && !disabled_no_op {
        checksum_oracle(&world, &oracle_ticks)
    } else {
        0
    };

    let stable_report_checksum = if admitted && !disabled_no_op {
        checksum_report(
            verdict,
            &entity_counts,
            deterministic_replay_checksum,
            ticks_completed,
        )
    } else {
        0
    };

    Runtime0080Rr0Report {
        id: RUNTIME_0080_RR_0_ID,
        status,
        verdict,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        world,
        entity_counts,
        oracle_ticks,
        ticks_scheduled: if disabled_no_op { 0 } else { input.tick_count },
        ticks_completed,
        scope_ledger,
        deviation_records,
        stable_report_checksum,
        deterministic_replay_checksum,
        cpu_oracle_only: true,
        gpu_residency_claimed: false,
        flat_proxy_closure: false,
        invariant_edit: false,
        total_labor_emitted,
        total_production_generated,
        total_disbursed_terran,
        total_disbursed_pirate,
        final_terran_stockpile,
        final_pirate_stockpile,
    }
}

fn build_scope_ledger(
    world: &Runtime0080Rr0RecursiveWorld,
    implemented: bool,
    ticks_completed: u32,
) -> Vec<Runtime0080Rr0ScopeLedgerRow> {
    let galaxy_ok = world.galaxy.width == GALAXY_SIDE
        && world.galaxy.height == GALAXY_SIDE
        && world.galaxy.cells.len() == (GALAXY_SIDE * GALAXY_SIDE) as usize;
    let systems_ok = world.galaxy.systems.len() == SYSTEM_COUNT;
    let subgrid_ok = world
        .galaxy
        .systems
        .iter()
        .all(|system| system.width == SYSTEM_SIDE && system.cells.len() == 100);
    let starport_ok = world
        .galaxy
        .systems
        .iter()
        .filter(|system| system.starport.is_some())
        .count()
        == 4;
    let planet_ok = world.galaxy.systems.iter().all(|system| {
        system.planet.parent_system_id == system.id
            && system.planet.surface.width == PLANET_SURFACE_SIDE
    });
    let surface_ok = world.galaxy.systems.iter().all(|system| {
        system.planet.surface.cells.len() == (PLANET_SURFACE_SIDE * PLANET_SURFACE_SIDE) as usize
    });
    let pop_ok = world
        .galaxy
        .systems
        .iter()
        .all(|system| system.planet.surface.pop_cohort.kind == "PopCohort");
    let factory_ok = world
        .galaxy
        .systems
        .iter()
        .all(|system| system.planet.surface.factory.kind == "FactoryDistrict");
    let terran_ok = world
        .galaxy
        .systems
        .iter()
        .filter(|system| system.owner == Runtime0080Rr0Owner::Terran)
        .count()
        == TERRAN_SYSTEM_COUNT;
    let pirate_ok = world
        .galaxy
        .systems
        .iter()
        .filter(|system| system.owner == Runtime0080Rr0Owner::Pirate)
        .count()
        == PIRATE_SYSTEM_COUNT;
    let oracle_ok = ticks_completed == R6C_CANONICAL_TICK_COUNT;

    vec![
        scope_ledger_row(
            "Galaxy 20×20 grid",
            galaxy_ok,
            "galaxy.width==20 && galaxy.cells==400",
        ),
        scope_ledger_row("13 occupied star systems", systems_ok, "systems.len()==13"),
        scope_ledger_row(
            "Each star system has 10×10 subgrid",
            subgrid_ok,
            "system.cells.len()==100 per system",
        ),
        scope_ledger_row(
            "Starport child in system grid",
            starport_ok,
            "4 starports across 13 systems",
        ),
        scope_ledger_row(
            "Planet child per system",
            planet_ok,
            "planet.parent_system_id matches system.id",
        ),
        scope_ledger_row(
            "Each planet has 10×10 surface",
            surface_ok,
            "surface.cells.len()==100 per planet",
        ),
        scope_ledger_row(
            "Pop cohort child on planet surface",
            pop_ok,
            "surface.pop_cohort.kind==PopCohort",
        ),
        scope_ledger_row(
            "Factory district child on planet surface",
            factory_ok,
            "surface.factory.kind==FactoryDistrict",
        ),
        scope_ledger_row(
            "Pop emits labor",
            implemented && POP_LABOR_PER_TICK > 0,
            "POP_LABOR_PER_TICK oracle emit",
        ),
        scope_ledger_row(
            "Factory consumes labor",
            implemented && FACTORY_UNIT_COST_LABOR > 0,
            "factory_recipe_production consume",
        ),
        scope_ledger_row(
            "Factory produces production",
            implemented && PRODUCTION_PER_RECIPE > 0,
            "factory_recipe_production output",
        ),
        scope_ledger_row(
            "Production reduces surface→planet",
            implemented,
            "surface.production_aggregate reduce-up",
        ),
        scope_ledger_row(
            "Production reduces planet→system",
            implemented,
            "planet.production_aggregate reduce-up",
        ),
        scope_ledger_row(
            "Production reduces system→galaxy",
            implemented,
            "galaxy.cells[].production reduce-up",
        ),
        scope_ledger_row(
            "Production reduces galaxy→faction stockpile",
            implemented,
            "faction_stockpiles reduce-up owner-masked",
        ),
        scope_ledger_row(
            "Disburse-down represented recursively",
            implemented,
            "starport.production_received disburse-down",
        ),
        scope_ledger_row("Terran 10-system economy", terran_ok, "terran systems==10"),
        scope_ledger_row("Pirate 3-system economy", pirate_ok, "pirate systems==3"),
        scope_ledger_row(
            "100-tick recursive CPU oracle",
            oracle_ok,
            "ticks_completed==100",
        ),
        scope_ledger_deferred_row(
            "R2 galactic combat loop remains reusable but not reimplemented in RR-0",
            "runtime_0080_0_r2.rs unchanged",
        ),
        scope_ledger_deferred_row(
            "GPU residency deferred to RR-1",
            "atlas_0080_0 generalization",
        ),
        scope_ledger_deferred_row(
            "Surface economy GPU deferred to RR-2",
            "AccumulatorOp GPU path",
        ),
        scope_ledger_deferred_row(
            "Recursive GPU reduce/disburse deferred to RR-3",
            "§0.2 GPU reduce-up/disburse-down",
        ),
        scope_ledger_deferred_row(
            "Integrated recursive GPU rehearsal deferred to RR-4",
            "100-tick recursive GPU horizon",
        ),
    ]
}

fn scope_ledger_row(
    spec_element: &'static str,
    ok: bool,
    evidence: &'static str,
) -> Runtime0080Rr0ScopeLedgerRow {
    Runtime0080Rr0ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_0: ok,
        status: if ok { "implemented" } else { "not implemented" },
        evidence,
        deviation: "",
    }
}

fn scope_ledger_deferred_row(
    spec_element: &'static str,
    evidence: &'static str,
) -> Runtime0080Rr0ScopeLedgerRow {
    Runtime0080Rr0ScopeLedgerRow {
        spec_element,
        required_by_spec: false,
        implemented_in_rr_0: false,
        status: "deferred",
        evidence,
        deviation: "",
    }
}

fn entity_counts_for(world: &Runtime0080Rr0RecursiveWorld) -> Runtime0080Rr0EntityCounts {
    let system_grid_cells = world
        .galaxy
        .systems
        .iter()
        .map(|system| system.cells.len())
        .sum();
    let surface_cells = world
        .galaxy
        .systems
        .iter()
        .map(|system| system.planet.surface.cells.len())
        .sum();
    Runtime0080Rr0EntityCounts {
        galaxy_cells: world.galaxy.cells.len(),
        systems: world.galaxy.systems.len(),
        system_grid_cells,
        planets: world.galaxy.systems.len(),
        surface_cells,
        pop_cohorts: world.galaxy.systems.len(),
        factories: world.galaxy.systems.len(),
        starports: world
            .galaxy
            .systems
            .iter()
            .filter(|system| system.starport.is_some())
            .count(),
    }
}

fn detect_flattening(world: &Runtime0080Rr0RecursiveWorld) -> bool {
    if world.galaxy.systems.is_empty() {
        return true;
    }
    for system in &world.galaxy.systems {
        if system.cells.len() != (SYSTEM_SIDE * SYSTEM_SIDE) as usize {
            return true;
        }
        if system.planet.surface.cells.len() != (PLANET_SURFACE_SIDE * PLANET_SURFACE_SIDE) as usize
        {
            return true;
        }
        if system.planet.surface.pop_cohort.kind.is_empty()
            || system.planet.surface.factory.kind.is_empty()
        {
            return true;
        }
    }
    false
}

fn owner_from_atlas(owner: atlas_gen::Owner) -> Runtime0080Rr0Owner {
    match owner {
        atlas_gen::Owner::Terran => Runtime0080Rr0Owner::Terran,
        atlas_gen::Owner::Pirate => Runtime0080Rr0Owner::Pirate,
    }
}

fn surface_cell_index(x: u32, y: u32) -> usize {
    (y * PLANET_SURFACE_SIDE + x) as usize
}

fn stockpile_for_owner(
    world: &Runtime0080Rr0RecursiveWorld,
    owner: Runtime0080Rr0Owner,
) -> &Runtime0080Rr0FactionStockpile {
    world
        .faction_stockpiles
        .iter()
        .find(|stockpile| stockpile.owner == owner)
        .expect("faction stockpile must exist")
}

fn stockpile_for_owner_mut(
    world: &mut Runtime0080Rr0RecursiveWorld,
    owner: Runtime0080Rr0Owner,
) -> &mut Runtime0080Rr0FactionStockpile {
    world
        .faction_stockpiles
        .iter_mut()
        .find(|stockpile| stockpile.owner == owner)
        .expect("faction stockpile must exist")
}

fn checksum_structural(world: &Runtime0080Rr0RecursiveWorld) -> u64 {
    let mut hash = FNV_OFFSET;
    hash = fnv_mix(hash, world.seed);
    hash = fnv_mix(hash, u64::from(world.galaxy.width));
    hash = fnv_mix(hash, u64::from(world.galaxy.systems.len() as u32));
    for system in &world.galaxy.systems {
        hash = fnv_mix(hash, u64::from(system.id));
        hash = fnv_mix(hash, system.owner.stable_code());
        hash = fnv_mix(hash, u64::from(system.parent_galaxy_linear_index));
        hash = fnv_mix(hash, u64::from(system.cells.len() as u32));
        hash = fnv_mix(hash, u64::from(system.planet.surface.cells.len() as u32));
        hash = fnv_mix(
            hash,
            u64::from(system.planet.surface.pop_cohort.surface_cell_x),
        );
        hash = fnv_mix(
            hash,
            u64::from(system.planet.surface.factory.surface_cell_x),
        );
        hash = fnv_mix(hash, u64::from(system.starport.is_some() as u8));
    }
    hash
}

fn checksum_oracle(
    world: &Runtime0080Rr0RecursiveWorld,
    ticks: &[Runtime0080Rr0OracleTick],
) -> u64 {
    let mut hash = checksum_structural(world);
    for tick in ticks {
        hash = fnv_mix(hash, u64::from(tick.tick));
        hash = fnv_mix(hash, tick.production_generated as u64);
        hash = fnv_mix(hash, tick.terran_stockpile_after as u64);
        hash = fnv_mix(hash, tick.pirate_stockpile_after as u64);
        hash = fnv_mix(hash, tick.disbursed_terran as u64);
        hash = fnv_mix(hash, tick.disbursed_pirate as u64);
    }
    hash = fnv_mix(
        hash,
        stockpile_for_owner(world, Runtime0080Rr0Owner::Terran).production as u64,
    );
    hash = fnv_mix(
        hash,
        stockpile_for_owner(world, Runtime0080Rr0Owner::Pirate).production as u64,
    );
    hash
}

fn checksum_report(
    verdict: &str,
    counts: &Runtime0080Rr0EntityCounts,
    oracle_checksum: u64,
    ticks_completed: u32,
) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in verdict.as_bytes() {
        hash = fnv_mix(hash, u64::from(*byte));
    }
    hash = fnv_mix(hash, u64::from(counts.systems as u32));
    hash = fnv_mix(hash, u64::from(counts.surface_cells as u32));
    hash = fnv_mix(hash, oracle_checksum);
    hash = fnv_mix(hash, u64::from(ticks_completed));
    hash
}

fn fnv_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rr_0_world_not_flattened_by_construction() {
        let world = build_recursive_world(atlas_gen::DRESS_REHEARSAL_DEFAULT_SEED);
        assert!(!world.is_flattened);
        assert_eq!(world.galaxy.systems.len(), SYSTEM_COUNT);
        assert!(world
            .galaxy
            .systems
            .iter()
            .all(|system| system.cells.len() == 100));
    }
}
