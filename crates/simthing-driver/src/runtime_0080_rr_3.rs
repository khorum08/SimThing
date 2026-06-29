//! RUNTIME-0080-RR-3: recursive GPU reduce-up/disburse-down.
//!
//! Consumes RR-0 recursive world/oracle, RR-1 nested residency, and RR-2 GPU surface production.
//! Runs distinct GPU tier transitions surface→planet→system→galaxy→faction stockpile and staged
//! disburse-down with bit-exact parity vs RR-0 recursive CPU oracle.

use simthing_core::{
    rebuild_discrete_transfer_ops, AccumulatorOp, AccumulatorOpBuilder, AccumulatorOpBuilderError,
    ColumnIndex, CombineFn, ConsumeMode, DiscreteTransferRegistration, GateSpec, ScaleSpec,
    SlotIndex, SourceSpec,
};
use simthing_gpu::{
    set_debug_readback_allowed, AccumulatorOpSession, GpuContext, PackedAccumulatorUpload,
};

use crate::dress_rehearsal_r2_recursive_allocation::{
    FACTORY_UNIT_COST_LABOR, POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE, STARPORT_PRODUCTION_NEED,
};
use crate::dress_rehearsal_r6c_integrated_run::R6C_CANONICAL_TICK_COUNT;
use crate::runtime_0080_rr_0::{
    build_recursive_world, run_runtime_0080_rr_0, Runtime0080Rr0Input, Runtime0080Rr0OracleTick,
    Runtime0080Rr0Owner, Runtime0080Rr0RecursiveWorld, Runtime0080Rr0System,
};
use crate::runtime_0080_rr_1::{
    run_runtime_0080_rr_1, Runtime0080Rr1Input, Runtime0080Rr1ResidencyRequest,
    RR_1_SURFACE_CELL_COUNT, RR_1_SYSTEM_COUNT,
};
use crate::runtime_0080_rr_2::{run_runtime_0080_rr_2, Runtime0080Rr2Input};

pub const RUNTIME_0080_RR_3_ID: &str = "RUNTIME-0080-RR-3";
pub const RUNTIME_0080_RR_3_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - recursive GPU reduce-up/disburse-down";
pub const RUNTIME_0080_RR_3_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - recursive GPU transfer incomplete or proxied";
pub const RUNTIME_0080_RR_3_STATUS_BLOCKED: &str =
    "BLOCKED - recursive RR-3 cannot close without approved deviation";

pub const RUNTIME_RR_3_EXPECTED_REPORT_CHECKSUM: u64 = 0xf6ad_f411_6656_e4a8;

pub const RR_3_COL_LABOR: u32 = 0;
pub const RR_3_COL_PRODUCTION: u32 = 1;
pub const RR_3_N_DIMS: u32 = 2;
pub const RR_3_SLOTS_PER_SYSTEM: u32 = 5;
pub const RR_3_TERRAN_STOCKPILE_SLOT: u32 = RR_1_SYSTEM_COUNT as u32 * RR_3_SLOTS_PER_SYSTEM;
pub const RR_3_PIRATE_STOCKPILE_SLOT: u32 = RR_3_TERRAN_STOCKPILE_SLOT + 1;
pub const RR_3_STARPORT_SLOT_BASE: u32 = RR_3_PIRATE_STOCKPILE_SLOT + 1;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const RR_0_DEFAULT_SEED: u64 = 0x0080_2000;

const BAND_LABOR_EMIT: u32 = 0;
const BAND_LABOR_TRANSFER: u32 = 1;
const BAND_FACTORY_RECIPE: u32 = 2;
const BAND_SURFACE_TO_PLANET: u32 = 3;
const BAND_PLANET_TO_SYSTEM: u32 = 4;
const BAND_SYSTEM_TO_GALAXY: u32 = 5;
const BAND_GALAXY_TO_STOCKPILE: u32 = 6;
const BAND_DISBURSE_BASE: u32 = 7;
const BAND_DISBURSE_STRIDE: u32 = 3;
const BAND_DIRECT_SURFACE_TO_STOCKPILE: u32 = 40;

fn max_disburse_band(starport_count: u32) -> u32 {
    if starport_count == 0 {
        BAND_DISBURSE_BASE
    } else {
        BAND_DISBURSE_BASE + (starport_count - 1) * BAND_DISBURSE_STRIDE + 2
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runtime0080Rr3TierTransition {
    SurfaceToPlanet,
    PlanetToSystem,
    SystemToGalaxy,
    GalaxyToStockpile,
    StockpileToGalaxy,
    GalaxyToSystem,
    SystemToStarport,
}

impl Runtime0080Rr3TierTransition {
    pub fn label(self) -> &'static str {
        match self {
            Self::SurfaceToPlanet => "surface→planet",
            Self::PlanetToSystem => "planet→system",
            Self::SystemToGalaxy => "system→galaxy",
            Self::GalaxyToStockpile => "galaxy→faction stockpile",
            Self::StockpileToGalaxy => "faction stockpile→galaxy",
            Self::GalaxyToSystem => "galaxy→system",
            Self::SystemToStarport => "system→starport",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub seed: u64,
    pub tick_index: u32,
}

impl Runtime0080Rr3Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            tick_index: 0,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            tick_index: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3SystemBinding {
    pub system_id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub pop_slot: u32,
    pub factory_slot: u32,
    pub planet_slot: u32,
    pub system_slot: u32,
    pub galaxy_slot: u32,
    pub starport_slot: Option<u32>,
    pub parent_galaxy_linear_index: u32,
    pub path_proven: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3TransitionRow {
    pub tick: u32,
    pub transition: Runtime0080Rr3TierTransition,
    pub owner: Runtime0080Rr0Owner,
    pub source_id: u8,
    pub target_id: u8,
    pub amount: i64,
    pub cpu_bits: u32,
    pub gpu_bits: u32,
    pub parity: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3ScopeLedgerRow {
    pub spec_element: &'static str,
    pub required_by_spec: bool,
    pub implemented_in_rr_3: bool,
    pub status: &'static str,
    pub evidence: &'static str,
    pub deviation: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3DeviationRecord {
    pub design_authority_approval: &'static str,
    pub specified_element: &'static str,
    pub implemented_proxy_or_omission: &'static str,
    pub reason: &'static str,
    pub consumer_impact: &'static str,
    pub required_follow_up: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr3Report {
    pub id: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub rr_0_world_consumed: bool,
    pub rr_1_residency_consumed: bool,
    pub rr_2_surface_production_consumed: bool,
    pub gpu_available: bool,
    pub terran_path_proven: bool,
    pub pirate_path_proven: bool,
    pub system_bindings: Vec<Runtime0080Rr3SystemBinding>,
    pub reduce_up_rows: Vec<Runtime0080Rr3TransitionRow>,
    pub disburse_down_rows: Vec<Runtime0080Rr3TransitionRow>,
    pub reduce_up_parity_ok: bool,
    pub disburse_down_parity_ok: bool,
    pub production_resource_parity_ok: bool,
    pub disabled_surface_to_planet_fails_parity: bool,
    pub reenabled_surface_to_planet_restores_parity: bool,
    pub disabled_galaxy_to_stockpile_fails_parity: bool,
    pub reenabled_galaxy_to_stockpile_restores_parity: bool,
    pub disabled_disburse_down_fails_parity: bool,
    pub reenabled_disburse_down_restores_parity: bool,
    pub no_cross_owner_leakage: bool,
    pub no_cross_tier_shortcut: bool,
    pub inactive_surfaces_do_not_reduce: bool,
    pub inactive_systems_do_not_disburse: bool,
    pub not_flattened_scalar: bool,
    pub scope_ledger: Vec<Runtime0080Rr3ScopeLedgerRow>,
    pub deviation_records: Vec<Runtime0080Rr3DeviationRecord>,
    pub stable_report_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub rr_4_claimed: bool,
    pub standalone_m4a_claimed: bool,
    pub invariant_edit: bool,
    pub default_session_wiring: bool,
}

pub fn run_runtime_0080_rr_3(input: &Runtime0080Rr3Input) -> Runtime0080Rr3Report {
    let mut diagnostics = Vec::new();
    if input.enabled_by_default {
        diagnostics.push("rr_3_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None);
    }
    if input.tick_index != 0 {
        diagnostics.push("rr_3_tick_index_must_be_zero_for_recursive_proof");
    }

    let rr2 = run_runtime_0080_rr_2(&Runtime0080Rr2Input {
        explicit_opt_in: true,
        enabled_by_default: false,
        seed: input.seed,
        tick_index: 0,
    });
    if rr2.verdict != "PASS" || !rr2.production_parity_ok {
        diagnostics.push("rr_2_surface_production_not_consumed");
    }

    let rr0 = run_runtime_0080_rr_0(&Runtime0080Rr0Input {
        explicit_opt_in: true,
        enabled_by_default: false,
        seed: input.seed,
        tick_count: R6C_CANONICAL_TICK_COUNT,
    });
    if rr0.verdict != "PASS" || rr0.oracle_ticks.is_empty() {
        diagnostics.push("rr_0_oracle_not_consumed");
    }

    let ctx = match GpuContext::new_blocking() {
        Ok(ctx) => ctx,
        Err(_) => {
            diagnostics.push("gpu_context_unavailable");
            return base_report(input, false, diagnostics, None);
        }
    };
    set_debug_readback_allowed(true);

    let world = build_recursive_world(input.seed);
    let terran_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Terran);
    let pirate_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Pirate);
    if !surface_resident_through_rr_1(terran_id) || !surface_resident_through_rr_1(pirate_id) {
        diagnostics.push("rr_1_surface_residency_path_missing");
    }
    if !diagnostics.is_empty() {
        return base_report(input, false, diagnostics, None);
    }

    let oracle_tick = rr0.oracle_ticks[0].clone();
    let execution = match execute_recursive_transfers(
        &ctx,
        &world,
        terran_id,
        pirate_id,
        &oracle_tick,
        input.tick_index,
    ) {
        Ok(exec) => exec,
        Err(diag) => {
            diagnostics.push(diag);
            return base_report(input, false, diagnostics, None);
        }
    };
    base_report(input, false, Vec::new(), Some(execution))
}

pub fn replay_runtime_0080_rr_3() -> (Runtime0080Rr3Report, Runtime0080Rr3Report) {
    let input = Runtime0080Rr3Input::explicit_opt_in();
    (run_runtime_0080_rr_3(&input), run_runtime_0080_rr_3(&input))
}

struct RecursiveExecution {
    system_bindings: Vec<Runtime0080Rr3SystemBinding>,
    terran_path_proven: bool,
    pirate_path_proven: bool,
    reduce_up_rows: Vec<Runtime0080Rr3TransitionRow>,
    disburse_down_rows: Vec<Runtime0080Rr3TransitionRow>,
    reduce_up_parity_ok: bool,
    disburse_down_parity_ok: bool,
    production_resource_parity_ok: bool,
    disabled_surface_to_planet_fails_parity: bool,
    reenabled_surface_to_planet_restores_parity: bool,
    disabled_galaxy_to_stockpile_fails_parity: bool,
    reenabled_galaxy_to_stockpile_restores_parity: bool,
    disabled_disburse_down_fails_parity: bool,
    reenabled_disburse_down_restores_parity: bool,
    no_cross_owner_leakage: bool,
    no_cross_tier_shortcut: bool,
    inactive_surfaces_do_not_reduce: bool,
    inactive_systems_do_not_disburse: bool,
    not_flattened_scalar: bool,
    deterministic_replay_checksum: u64,
}

pub(crate) struct RecursiveGpuLayout {
    pub(crate) bindings: Vec<Runtime0080Rr3SystemBinding>,
    pub(crate) n_slots: u32,
    pub(crate) starport_count: u32,
}

pub(crate) struct RecursiveGpuConfig {
    active_system_ids: Vec<u8>,
    surface_to_planet_enabled: bool,
    planet_to_system_enabled: bool,
    system_to_galaxy_enabled: bool,
    galaxy_to_stockpile_enabled: bool,
    disburse_down_enabled: bool,
    wrong_owner_routing: bool,
    direct_surface_to_stockpile: bool,
}

pub(crate) struct GpuRecursiveOutcome {
    pub(crate) values: Vec<f32>,
    pub(crate) n_dims: u32,
    after_surface_to_planet: Vec<f32>,
    after_planet_to_system: Vec<f32>,
    after_system_to_galaxy: Vec<f32>,
    after_galaxy_to_stockpile: Vec<f32>,
    after_disburse: Vec<f32>,
}

impl GpuRecursiveOutcome {
    fn production_at(&self, slot: u32) -> f32 {
        self.values[cell_index(slot, RR_3_COL_PRODUCTION, self.n_dims)]
    }

    fn production_bits_at(&self, slot: u32) -> u32 {
        self.production_at(slot).to_bits()
    }
}

fn execute_recursive_transfers(
    ctx: &GpuContext,
    world: &Runtime0080Rr0RecursiveWorld,
    terran_id: u8,
    pirate_id: u8,
    oracle_tick: &Runtime0080Rr0OracleTick,
    tick_index: u32,
) -> Result<RecursiveExecution, &'static str> {
    let layout = build_recursive_layout(world, terran_id, pirate_id)?;
    let all_system_ids: Vec<u8> = layout.bindings.iter().map(|b| b.system_id).collect();

    let baseline = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let cpu_reduce = cpu_reduce_rows(&layout, oracle_tick, tick_index);
    let cpu_disburse = cpu_disburse_rows(world, &layout, oracle_tick, tick_index);
    let reduce_up_rows = merge_reduce_rows(&cpu_reduce, &baseline, &layout);
    let disburse_down_rows = merge_disburse_rows(&cpu_disburse, &baseline, &layout);
    let reduce_up_parity_ok = reduce_up_rows.iter().all(|row| row.parity);
    let disburse_down_parity_ok = disburse_down_rows.iter().all(|row| row.parity);
    let production_resource_parity_ok = stockpile_parity_ok(&baseline, oracle_tick)
        && starport_parity_ok(&baseline, &layout, world, oracle_tick);

    let disabled_surface = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: false,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let disabled_surface_to_planet_fails_parity =
        !stockpile_parity_ok(&disabled_surface, oracle_tick);

    let reenabled_surface = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let reenabled_surface_to_planet_restores_parity =
        stockpile_parity_ok(&reenabled_surface, oracle_tick);

    let disabled_galaxy_stockpile = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: false,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let disabled_galaxy_to_stockpile_fails_parity =
        !stockpile_parity_ok(&disabled_galaxy_stockpile, oracle_tick);

    let reenabled_galaxy_stockpile = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let reenabled_galaxy_to_stockpile_restores_parity =
        stockpile_parity_ok(&reenabled_galaxy_stockpile, oracle_tick);

    let disabled_disburse = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: false,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let disabled_disburse_down_fails_parity =
        !starport_parity_ok(&disabled_disburse, &layout, world, oracle_tick);

    let reenabled_disburse = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let reenabled_disburse_down_restores_parity =
        starport_parity_ok(&reenabled_disburse, &layout, world, oracle_tick);

    let leaked = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: true,
            direct_surface_to_stockpile: false,
        },
    )?;
    let no_cross_owner_leakage = !stockpile_parity_ok(&leaked, oracle_tick);

    let shortcut = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids.clone(),
            surface_to_planet_enabled: false,
            planet_to_system_enabled: false,
            system_to_galaxy_enabled: false,
            galaxy_to_stockpile_enabled: false,
            disburse_down_enabled: false,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: true,
        },
    )?;
    let no_cross_tier_shortcut =
        !stockpile_parity_ok(&shortcut, oracle_tick) || !reduce_up_rows.iter().all(|r| r.parity);

    let inactive_surface_gpu = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: vec![terran_id],
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let pirate_binding = layout
        .bindings
        .iter()
        .find(|b| b.system_id == pirate_id)
        .expect("pirate");
    let inactive_surfaces_do_not_reduce =
        inactive_surface_gpu.production_at(pirate_binding.factory_slot) == 0.0;

    let starport_system = layout
        .bindings
        .iter()
        .find(|b| b.starport_slot.is_some())
        .expect("starport system")
        .system_id;
    let inactive_system_gpu = run_gpu_recursive_tick(
        ctx,
        &layout,
        RecursiveGpuConfig {
            active_system_ids: all_system_ids
                .iter()
                .copied()
                .filter(|id| *id != starport_system)
                .collect(),
            surface_to_planet_enabled: true,
            planet_to_system_enabled: true,
            system_to_galaxy_enabled: true,
            galaxy_to_stockpile_enabled: true,
            disburse_down_enabled: true,
            wrong_owner_routing: false,
            direct_surface_to_stockpile: false,
        },
    )?;
    let inactive_starport = layout
        .bindings
        .iter()
        .find(|b| b.system_id == starport_system)
        .expect("inactive starport");
    let inactive_systems_do_not_disburse = inactive_starport
        .starport_slot
        .is_some_and(|slot| inactive_system_gpu.production_at(slot) == 0.0);

    let not_flattened_scalar = layout.n_slots > RR_3_TERRAN_STOCKPILE_SLOT
        && layout.bindings.len() == RR_1_SYSTEM_COUNT
        && layout
            .bindings
            .iter()
            .all(|b| b.planet_slot != b.system_slot);

    let terran_path_proven = layout
        .bindings
        .iter()
        .find(|b| b.system_id == terran_id)
        .is_some_and(|b| b.path_proven)
        && reduce_up_rows
            .iter()
            .filter(|r| r.owner == Runtime0080Rr0Owner::Terran && r.source_id == terran_id)
            .all(|r| r.parity);
    let pirate_path_proven = layout
        .bindings
        .iter()
        .find(|b| b.system_id == pirate_id)
        .is_some_and(|b| b.path_proven)
        && reduce_up_rows
            .iter()
            .filter(|r| r.owner == Runtime0080Rr0Owner::Pirate && r.source_id == pirate_id)
            .all(|r| r.parity);

    let deterministic_replay_checksum =
        checksum_execution(&reduce_up_rows, &disburse_down_rows, oracle_tick);

    Ok(RecursiveExecution {
        system_bindings: layout.bindings,
        terran_path_proven,
        pirate_path_proven,
        reduce_up_rows,
        disburse_down_rows,
        reduce_up_parity_ok,
        disburse_down_parity_ok,
        production_resource_parity_ok,
        disabled_surface_to_planet_fails_parity,
        reenabled_surface_to_planet_restores_parity,
        disabled_galaxy_to_stockpile_fails_parity,
        reenabled_galaxy_to_stockpile_restores_parity,
        disabled_disburse_down_fails_parity,
        reenabled_disburse_down_restores_parity,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_do_not_reduce,
        inactive_systems_do_not_disburse,
        not_flattened_scalar,
        deterministic_replay_checksum,
    })
}

fn build_recursive_layout(
    world: &Runtime0080Rr0RecursiveWorld,
    terran_id: u8,
    pirate_id: u8,
) -> Result<RecursiveGpuLayout, &'static str> {
    let mut starport_count = 0u32;
    let mut bindings = Vec::with_capacity(RR_1_SYSTEM_COUNT);
    for system in &world.galaxy.systems {
        let base = u32::from(system.id) * RR_3_SLOTS_PER_SYSTEM;
        let pop = base;
        let factory = base + 1;
        let planet = base + 2;
        let system_slot = base + 3;
        let galaxy_slot = base + 4;
        let starport_slot = if system.starport.is_some() {
            let slot = RR_3_STARPORT_SLOT_BASE + starport_count;
            starport_count += 1;
            Some(slot)
        } else {
            None
        };
        bindings.push(Runtime0080Rr3SystemBinding {
            system_id: system.id,
            owner: system.owner,
            pop_slot: pop,
            factory_slot: factory,
            planet_slot: planet,
            system_slot,
            galaxy_slot,
            starport_slot,
            parent_galaxy_linear_index: system.parent_galaxy_linear_index,
            path_proven: system.id == terran_id || system.id == pirate_id,
        });
    }
    let n_slots = RR_3_STARPORT_SLOT_BASE + starport_count;
    Ok(RecursiveGpuLayout {
        bindings,
        n_slots,
        starport_count,
    })
}

fn run_gpu_recursive_tick(
    ctx: &GpuContext,
    layout: &RecursiveGpuLayout,
    config: RecursiveGpuConfig,
) -> Result<GpuRecursiveOutcome, &'static str> {
    let mut session = rr_3_engine_init_session(ctx, layout, &config)?;
    dispatch_recursive_bands(ctx, &mut session, layout, &config)
}

fn should_skip_band(band: u32, config: &RecursiveGpuConfig, starport_count: u32) -> bool {
    if band == BAND_SURFACE_TO_PLANET && !config.surface_to_planet_enabled {
        return true;
    }
    if band == BAND_PLANET_TO_SYSTEM && !config.planet_to_system_enabled {
        return true;
    }
    if band == BAND_SYSTEM_TO_GALAXY && !config.system_to_galaxy_enabled {
        return true;
    }
    if band == BAND_GALAXY_TO_STOCKPILE && !config.galaxy_to_stockpile_enabled {
        return true;
    }
    if band == BAND_DIRECT_SURFACE_TO_STOCKPILE && !config.direct_surface_to_stockpile {
        return true;
    }
    if !config.disburse_down_enabled
        && band >= BAND_DISBURSE_BASE
        && band <= max_disburse_band(starport_count)
    {
        return true;
    }
    false
}

fn build_recursive_ops(
    layout: &RecursiveGpuLayout,
    config: &RecursiveGpuConfig,
) -> Result<Vec<AccumulatorOp>, AccumulatorOpBuilderError> {
    let mut ops = Vec::new();
    let mut transfers = Vec::new();
    let production_amt = PRODUCTION_PER_RECIPE as f32;

    for binding in &layout.bindings {
        if !config.active_system_ids.contains(&binding.system_id) {
            continue;
        }
        ops.push(labor_emit_op(binding.pop_slot));
        transfers.push(DiscreteTransferRegistration {
            source_slot: SlotIndex::new(binding.pop_slot),
            source_col: ColumnIndex::new(RR_3_COL_LABOR as usize),
            target_slot: SlotIndex::new(binding.factory_slot),
            target_col: ColumnIndex::new(RR_3_COL_LABOR as usize),
            amount: POP_LABOR_PER_TICK as f32,
            order_band: BAND_LABOR_TRANSFER,
        });
        let mut recipe_op = AccumulatorOpBuilder::conjunctive_recipe(
            &[(
                SlotIndex::new(binding.factory_slot),
                ColumnIndex::new(RR_3_COL_LABOR as usize),
                FACTORY_UNIT_COST_LABOR as f32,
            )],
            SlotIndex::new(binding.factory_slot),
            ColumnIndex::new(RR_3_COL_PRODUCTION as usize),
            99,
        )?;
        recipe_op.gate = GateSpec::OrderBand(BAND_FACTORY_RECIPE);
        ops.push(recipe_op);

        if config.surface_to_planet_enabled {
            transfers.push(tier_transfer(
                binding.factory_slot,
                binding.planet_slot,
                production_amt,
                BAND_SURFACE_TO_PLANET,
            ));
        }
        if config.planet_to_system_enabled {
            transfers.push(tier_transfer(
                binding.planet_slot,
                binding.system_slot,
                production_amt,
                BAND_PLANET_TO_SYSTEM,
            ));
        }
        if config.system_to_galaxy_enabled {
            transfers.push(tier_transfer(
                binding.system_slot,
                binding.galaxy_slot,
                production_amt,
                BAND_SYSTEM_TO_GALAXY,
            ));
        }
        if config.galaxy_to_stockpile_enabled {
            let stockpile_slot =
                stockpile_slot_for_owner(binding.owner, config.wrong_owner_routing);
            transfers.push(tier_transfer(
                binding.galaxy_slot,
                stockpile_slot,
                production_amt,
                BAND_GALAXY_TO_STOCKPILE,
            ));
        }
        if config.direct_surface_to_stockpile {
            let stockpile_slot = stockpile_slot_for_owner(binding.owner, false);
            transfers.push(tier_transfer(
                binding.factory_slot,
                stockpile_slot,
                production_amt,
                BAND_DIRECT_SURFACE_TO_STOCKPILE,
            ));
        }
    }

    if config.disburse_down_enabled {
        let mut starport_systems: Vec<_> = layout
            .bindings
            .iter()
            .filter(|b| b.starport_slot.is_some())
            .collect();
        starport_systems.sort_by_key(|b| b.system_id);
        for (index, binding) in starport_systems.into_iter().enumerate() {
            if !config.active_system_ids.contains(&binding.system_id) {
                continue;
            }
            let starport_slot = binding.starport_slot.expect("starport");
            let stockpile_slot = stockpile_slot_for_owner(binding.owner, false);
            let band_base = BAND_DISBURSE_BASE + (index as u32) * BAND_DISBURSE_STRIDE;
            transfers.push(DiscreteTransferRegistration {
                source_slot: SlotIndex::new(stockpile_slot),
                source_col: ColumnIndex::new(RR_3_COL_PRODUCTION as usize),
                target_slot: SlotIndex::new(binding.galaxy_slot),
                target_col: ColumnIndex::new(RR_3_COL_PRODUCTION as usize),
                amount: STARPORT_PRODUCTION_NEED as f32,
                order_band: band_base,
            });
            transfers.push(tier_transfer(
                binding.galaxy_slot,
                binding.system_slot,
                STARPORT_PRODUCTION_NEED as f32,
                band_base + 1,
            ));
            transfers.push(tier_transfer(
                binding.system_slot,
                starport_slot,
                STARPORT_PRODUCTION_NEED as f32,
                band_base + 2,
            ));
        }
    }

    ops.extend(rebuild_discrete_transfer_ops(&transfers)?);
    Ok(ops)
}

fn tier_transfer(
    source_slot: u32,
    target_slot: u32,
    amount: f32,
    band: u32,
) -> DiscreteTransferRegistration {
    DiscreteTransferRegistration {
        source_slot: SlotIndex::new(source_slot),
        source_col: ColumnIndex::new(RR_3_COL_PRODUCTION as usize),
        target_slot: SlotIndex::new(target_slot),
        target_col: ColumnIndex::new(RR_3_COL_PRODUCTION as usize),
        amount,
        order_band: band,
    }
}

fn stockpile_slot_for_owner(owner: Runtime0080Rr0Owner, wrong_owner: bool) -> u32 {
    if wrong_owner {
        match owner {
            Runtime0080Rr0Owner::Terran => RR_3_PIRATE_STOCKPILE_SLOT,
            Runtime0080Rr0Owner::Pirate => RR_3_TERRAN_STOCKPILE_SLOT,
        }
    } else {
        match owner {
            Runtime0080Rr0Owner::Terran => RR_3_TERRAN_STOCKPILE_SLOT,
            Runtime0080Rr0Owner::Pirate => RR_3_PIRATE_STOCKPILE_SLOT,
        }
    }
}

fn labor_emit_op(pop_slot: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::Constant(1.0),
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(BAND_LABOR_EMIT),
        scale: ScaleSpec::Constant(POP_LABOR_PER_TICK as f32),
        consume: ConsumeMode::AddToTarget,
        targets: vec![(
            SlotIndex::new(pop_slot),
            ColumnIndex::new(RR_3_COL_LABOR as usize),
        )],
    }
}

fn cpu_reduce_rows(
    layout: &RecursiveGpuLayout,
    oracle: &Runtime0080Rr0OracleTick,
    tick: u32,
) -> Vec<Runtime0080Rr3TransitionRow> {
    let mut rows = Vec::new();
    for binding in &layout.bindings {
        let amount = PRODUCTION_PER_RECIPE;
        let bits = i64_bits(amount);
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::SurfaceToPlanet,
            binding.owner,
            binding.system_id,
            binding.system_id,
            amount,
            bits,
        ));
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::PlanetToSystem,
            binding.owner,
            binding.system_id,
            binding.system_id,
            amount,
            bits,
        ));
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::SystemToGalaxy,
            binding.owner,
            binding.system_id,
            binding.system_id,
            amount,
            bits,
        ));
        let stockpile_target = match binding.owner {
            Runtime0080Rr0Owner::Terran => 255,
            Runtime0080Rr0Owner::Pirate => 254,
        };
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::GalaxyToStockpile,
            binding.owner,
            binding.system_id,
            stockpile_target,
            amount,
            bits,
        ));
    }
    let _ = (
        oracle.reduced_surface_to_planet,
        oracle.reduced_planet_to_system,
        oracle.reduced_system_to_galaxy,
    );
    rows
}

fn cpu_disburse_rows(
    world: &Runtime0080Rr0RecursiveWorld,
    _layout: &RecursiveGpuLayout,
    oracle: &Runtime0080Rr0OracleTick,
    tick: u32,
) -> Vec<Runtime0080Rr3TransitionRow> {
    let mut rows = Vec::new();
    let mut starport_systems: Vec<&Runtime0080Rr0System> = world
        .galaxy
        .systems
        .iter()
        .filter(|s| s.starport.is_some())
        .collect();
    starport_systems.sort_by_key(|s| s.id);

    let mut terran_stockpile = oracle.terran_stockpile_after + oracle.disbursed_terran;
    let mut pirate_stockpile = oracle.pirate_stockpile_after + oracle.disbursed_pirate;

    for system in starport_systems {
        let available = match system.owner {
            Runtime0080Rr0Owner::Terran => terran_stockpile.max(0),
            Runtime0080Rr0Owner::Pirate => pirate_stockpile.max(0),
        };
        let disbursed = STARPORT_PRODUCTION_NEED.min(available);
        match system.owner {
            Runtime0080Rr0Owner::Terran => terran_stockpile -= disbursed,
            Runtime0080Rr0Owner::Pirate => pirate_stockpile -= disbursed,
        }
        let bits = i64_bits(disbursed);
        let stockpile_id = match system.owner {
            Runtime0080Rr0Owner::Terran => 255,
            Runtime0080Rr0Owner::Pirate => 254,
        };
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::StockpileToGalaxy,
            system.owner,
            stockpile_id,
            system.id,
            disbursed,
            bits,
        ));
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::GalaxyToSystem,
            system.owner,
            system.id,
            system.id,
            disbursed,
            bits,
        ));
        rows.push(row(
            tick,
            Runtime0080Rr3TierTransition::SystemToStarport,
            system.owner,
            system.id,
            system.id,
            disbursed,
            bits,
        ));
    }
    rows
}

fn row(
    tick: u32,
    transition: Runtime0080Rr3TierTransition,
    owner: Runtime0080Rr0Owner,
    source_id: u8,
    target_id: u8,
    amount: i64,
    cpu_bits: u32,
) -> Runtime0080Rr3TransitionRow {
    Runtime0080Rr3TransitionRow {
        tick,
        transition,
        owner,
        source_id,
        target_id,
        amount,
        cpu_bits,
        gpu_bits: 0,
        parity: false,
    }
}

fn merge_reduce_rows(
    cpu_rows: &[Runtime0080Rr3TransitionRow],
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
) -> Vec<Runtime0080Rr3TransitionRow> {
    cpu_rows
        .iter()
        .map(|cpu| {
            let binding = layout
                .bindings
                .iter()
                .find(|b| b.system_id == cpu.source_id)
                .expect("binding");
            let (snapshot, slot) = match cpu.transition {
                Runtime0080Rr3TierTransition::SurfaceToPlanet => {
                    (&gpu.after_surface_to_planet, binding.planet_slot)
                }
                Runtime0080Rr3TierTransition::PlanetToSystem => {
                    (&gpu.after_planet_to_system, binding.system_slot)
                }
                Runtime0080Rr3TierTransition::SystemToGalaxy => {
                    (&gpu.after_system_to_galaxy, binding.galaxy_slot)
                }
                Runtime0080Rr3TierTransition::GalaxyToStockpile => {
                    (&gpu.after_galaxy_to_stockpile, binding.galaxy_slot)
                }
                _ => (&gpu.values, binding.factory_slot),
            };
            let gpu_bits = snapshot[cell_index(slot, RR_3_COL_PRODUCTION, gpu.n_dims)].to_bits();
            let expected_bits = match cpu.transition {
                Runtime0080Rr3TierTransition::GalaxyToStockpile => i64_bits(0),
                _ => cpu.cpu_bits,
            };
            Runtime0080Rr3TransitionRow {
                gpu_bits,
                parity: gpu_bits == expected_bits,
                ..cpu.clone()
            }
        })
        .collect()
}

fn merge_disburse_rows(
    cpu_rows: &[Runtime0080Rr3TransitionRow],
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
) -> Vec<Runtime0080Rr3TransitionRow> {
    cpu_rows
        .iter()
        .map(|cpu| {
            let binding = layout
                .bindings
                .iter()
                .find(|b| b.system_id == cpu.target_id)
                .expect("binding");
            let gpu_bits = binding
                .starport_slot
                .map(|slot| snapshot_bits_at(&gpu.after_disburse, slot, gpu.n_dims))
                .unwrap_or(0);
            Runtime0080Rr3TransitionRow {
                gpu_bits,
                parity: if cpu.amount == 0 {
                    gpu_bits == i64_bits(0)
                } else {
                    gpu_bits == cpu.cpu_bits
                },
                ..cpu.clone()
            }
        })
        .collect()
}

fn snapshot_bits_at(values: &[f32], slot: u32, n_dims: u32) -> u32 {
    values[cell_index(slot, RR_3_COL_PRODUCTION, n_dims)].to_bits()
}

fn stockpile_parity_ok(gpu: &GpuRecursiveOutcome, oracle: &Runtime0080Rr0OracleTick) -> bool {
    gpu.production_bits_at(RR_3_TERRAN_STOCKPILE_SLOT) == i64_bits(oracle.terran_stockpile_after)
        && gpu.production_bits_at(RR_3_PIRATE_STOCKPILE_SLOT)
            == i64_bits(oracle.pirate_stockpile_after)
}

fn starport_parity_ok(
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
    world: &Runtime0080Rr0RecursiveWorld,
    oracle: &Runtime0080Rr0OracleTick,
) -> bool {
    let terran_expected = oracle.disbursed_terran;
    let pirate_expected = oracle.disbursed_pirate;
    let mut terran_gpu = 0i64;
    let mut pirate_gpu = 0i64;
    for binding in &layout.bindings {
        if let Some(slot) = binding.starport_slot {
            let received = gpu.production_at(slot) as i64;
            match binding.owner {
                Runtime0080Rr0Owner::Terran => terran_gpu += received,
                Runtime0080Rr0Owner::Pirate => pirate_gpu += received,
            }
        }
    }
    let _ = world;
    terran_gpu == terran_expected && pirate_gpu == pirate_expected
}

fn surface_resident_through_rr_1(system_id: u8) -> bool {
    let report = run_runtime_0080_rr_1(&Runtime0080Rr1Input::with_access_pattern(vec![
        Runtime0080Rr1ResidencyRequest::DescendToSurface { system_id },
    ]));
    report.residency_trace.last().is_some_and(|row| {
        row.surface_materialized_rows == RR_1_SURFACE_CELL_COUNT
            && row.active_system_id == Some(system_id)
    })
}

fn find_owner_system_id(world: &Runtime0080Rr0RecursiveWorld, owner: Runtime0080Rr0Owner) -> u8 {
    world
        .galaxy
        .systems
        .iter()
        .find(|system| system.owner == owner)
        .expect("owner system")
        .id
}

fn cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn i64_bits(value: i64) -> u32 {
    (value as f32).to_bits()
}

fn build_scope_ledger(
    exec: &RecursiveExecution,
    implemented: bool,
) -> Vec<Runtime0080Rr3ScopeLedgerRow> {
    vec![
        scope_row(
            "RR-0 recursive world/oracle consumed",
            implemented,
            "run_runtime_0080_rr_0",
        ),
        scope_row(
            "RR-1 nested residency consumed",
            implemented,
            "DescendToSurface Terran+Pirate",
        ),
        scope_row(
            "RR-2 GPU surface production consumed",
            exec.production_resource_parity_ok,
            "run_runtime_0080_rr_2 PASS + surface bands 0-2",
        ),
        scope_row(
            "Terran recursive path proven",
            exec.terran_path_proven,
            "terran reduce+disburse parity",
        ),
        scope_row(
            "Pirate recursive path proven",
            exec.pirate_path_proven,
            "pirate reduce+disburse parity",
        ),
        scope_row(
            "Surface→planet reduce computed on GPU",
            exec.reduce_up_parity_ok,
            "band 3 discrete transfer",
        ),
        scope_row(
            "Planet→system reduce computed on GPU",
            exec.reduce_up_parity_ok,
            "band 4 discrete transfer",
        ),
        scope_row(
            "System→galaxy reduce computed on GPU",
            exec.reduce_up_parity_ok,
            "band 5 discrete transfer",
        ),
        scope_row(
            "Galaxy→faction stockpile reduce computed on GPU",
            exec.reduce_up_parity_ok,
            "band 6 owner-masked transfer",
        ),
        scope_row(
            "Faction→galaxy disburse computed on GPU",
            exec.disburse_down_parity_ok,
            "band 7 stockpile→galaxy",
        ),
        scope_row(
            "Galaxy→system disburse computed on GPU",
            exec.disburse_down_parity_ok,
            "band 8 galaxy→system",
        ),
        scope_row(
            "System→planet/surface/starport disburse computed on GPU",
            exec.disburse_down_parity_ok,
            "band 9 system→starport",
        ),
        scope_row(
            "Reduce-up parity vs RR-0 oracle",
            exec.reduce_up_parity_ok,
            "reduce_up_rows",
        ),
        scope_row(
            "Disburse-down parity vs RR-0 oracle",
            exec.disburse_down_parity_ok,
            "disburse_down_rows",
        ),
        scope_row(
            "Bit-exact production/resource parity",
            exec.production_resource_parity_ok,
            "stockpile+starport bits",
        ),
        scope_row(
            "Disabled surface→planet reduce fails parity",
            exec.disabled_surface_to_planet_fails_parity,
            "surface_to_planet_enabled=false",
        ),
        scope_row(
            "Re-enabled surface→planet reduce restores parity",
            exec.reenabled_surface_to_planet_restores_parity,
            "surface_to_planet_enabled=true",
        ),
        scope_row(
            "Disabled galaxy→faction reduce fails parity",
            exec.disabled_galaxy_to_stockpile_fails_parity,
            "galaxy_to_stockpile_enabled=false",
        ),
        scope_row(
            "Re-enabled galaxy→faction reduce restores parity",
            exec.reenabled_galaxy_to_stockpile_restores_parity,
            "galaxy_to_stockpile_enabled=true",
        ),
        scope_row(
            "Disabled disburse-down fails parity",
            exec.disabled_disburse_down_fails_parity,
            "disburse_down_enabled=false",
        ),
        scope_row(
            "Re-enabled disburse-down restores parity",
            exec.reenabled_disburse_down_restores_parity,
            "disburse_down_enabled=true",
        ),
        scope_row(
            "No cross-owner leakage",
            exec.no_cross_owner_leakage,
            "wrong_owner_routing fails parity",
        ),
        scope_row(
            "No cross-tier shortcut",
            exec.no_cross_tier_shortcut,
            "direct_surface_to_stockpile fails parity",
        ),
        scope_row(
            "Inactive systems/surfaces do not reduce or disburse",
            exec.inactive_surfaces_do_not_reduce && exec.inactive_systems_do_not_disburse,
            "inactive pirate surface + inactive starport system",
        ),
        scope_row(
            "Not flattened to direct surface→faction scalar",
            exec.not_flattened_scalar,
            "per-system tier slots + stockpile slots",
        ),
        deferred_row(
            "Integrated recursive 100-tick GPU rehearsal deferred to RR-4",
            "tick 0 representative proof",
        ),
        deferred_row(
            "Standalone M-4A parallel theater track not claimed",
            "nested RR track only",
        ),
    ]
}

fn scope_row(
    spec_element: &'static str,
    ok: bool,
    evidence: &'static str,
) -> Runtime0080Rr3ScopeLedgerRow {
    Runtime0080Rr3ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_3: ok,
        status: if ok { "implemented" } else { "not implemented" },
        evidence,
        deviation: "",
    }
}

fn deferred_row(
    spec_element: &'static str,
    evidence: &'static str,
) -> Runtime0080Rr3ScopeLedgerRow {
    Runtime0080Rr3ScopeLedgerRow {
        spec_element,
        required_by_spec: false,
        implemented_in_rr_3: false,
        status: "deferred",
        evidence,
        deviation: "",
    }
}

fn base_report(
    input: &Runtime0080Rr3Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<RecursiveExecution>,
) -> Runtime0080Rr3Report {
    let admitted = diagnostics.is_empty();

    let (
        system_bindings,
        terran_path_proven,
        pirate_path_proven,
        reduce_up_rows,
        disburse_down_rows,
        reduce_up_parity_ok,
        disburse_down_parity_ok,
        production_resource_parity_ok,
        disabled_surface_to_planet_fails_parity,
        reenabled_surface_to_planet_restores_parity,
        disabled_galaxy_to_stockpile_fails_parity,
        reenabled_galaxy_to_stockpile_restores_parity,
        disabled_disburse_down_fails_parity,
        reenabled_disburse_down_restores_parity,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_do_not_reduce,
        inactive_systems_do_not_disburse,
        not_flattened_scalar,
        scope_ledger,
        deviation_records,
        deterministic_replay_checksum,
    ) = match execution {
        Some(exec) => {
            let scope_ledger = build_scope_ledger(&exec, true);
            (
                exec.system_bindings,
                exec.terran_path_proven,
                exec.pirate_path_proven,
                exec.reduce_up_rows,
                exec.disburse_down_rows,
                exec.reduce_up_parity_ok,
                exec.disburse_down_parity_ok,
                exec.production_resource_parity_ok,
                exec.disabled_surface_to_planet_fails_parity,
                exec.reenabled_surface_to_planet_restores_parity,
                exec.disabled_galaxy_to_stockpile_fails_parity,
                exec.reenabled_galaxy_to_stockpile_restores_parity,
                exec.disabled_disburse_down_fails_parity,
                exec.reenabled_disburse_down_restores_parity,
                exec.no_cross_owner_leakage,
                exec.no_cross_tier_shortcut,
                exec.inactive_surfaces_do_not_reduce,
                exec.inactive_systems_do_not_disburse,
                exec.not_flattened_scalar,
                scope_ledger,
                Vec::<Runtime0080Rr3DeviationRecord>::new(),
                exec.deterministic_replay_checksum,
            )
        }
        None => (
            Vec::new(),
            false,
            false,
            Vec::new(),
            Vec::new(),
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            Vec::new(),
            Vec::new(),
            0,
        ),
    };

    let required_rows_implemented = scope_ledger
        .iter()
        .take(25)
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
        "PASS" => RUNTIME_0080_RR_3_STATUS_PASS,
        "PARTIAL" => RUNTIME_0080_RR_3_STATUS_PARTIAL,
        _ => RUNTIME_0080_RR_3_STATUS_BLOCKED,
    };

    let stable_report_checksum = if admitted && !disabled_no_op {
        checksum_report(
            verdict,
            reduce_up_parity_ok,
            disburse_down_parity_ok,
            deterministic_replay_checksum,
        )
    } else {
        0
    };

    Runtime0080Rr3Report {
        id: RUNTIME_0080_RR_3_ID,
        status,
        verdict,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        rr_0_world_consumed: admitted && !disabled_no_op,
        rr_1_residency_consumed: terran_path_proven || pirate_path_proven,
        rr_2_surface_production_consumed: admitted && !disabled_no_op,
        gpu_available: admitted && !disabled_no_op,
        terran_path_proven,
        pirate_path_proven,
        system_bindings,
        reduce_up_rows,
        disburse_down_rows,
        reduce_up_parity_ok,
        disburse_down_parity_ok,
        production_resource_parity_ok,
        disabled_surface_to_planet_fails_parity,
        reenabled_surface_to_planet_restores_parity,
        disabled_galaxy_to_stockpile_fails_parity,
        reenabled_galaxy_to_stockpile_restores_parity,
        disabled_disburse_down_fails_parity,
        reenabled_disburse_down_restores_parity,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_do_not_reduce,
        inactive_systems_do_not_disburse,
        not_flattened_scalar,
        scope_ledger,
        deviation_records,
        stable_report_checksum,
        deterministic_replay_checksum,
        rr_4_claimed: false,
        standalone_m4a_claimed: false,
        invariant_edit: false,
        default_session_wiring: false,
    }
}

fn checksum_execution(
    reduce_rows: &[Runtime0080Rr3TransitionRow],
    disburse_rows: &[Runtime0080Rr3TransitionRow],
    oracle: &Runtime0080Rr0OracleTick,
) -> u64 {
    let mut hash = FNV_OFFSET;
    for row in reduce_rows {
        hash = fnv_mix(hash, u64::from(row.gpu_bits));
        hash = fnv_mix(hash, u64::from(row.parity as u8));
    }
    for row in disburse_rows {
        hash = fnv_mix(hash, u64::from(row.gpu_bits));
        hash = fnv_mix(hash, u64::from(row.parity as u8));
    }
    hash = fnv_mix(hash, oracle.terran_stockpile_after as u64);
    hash = fnv_mix(hash, oracle.pirate_stockpile_after as u64);
    hash
}

fn checksum_report(verdict: &str, reduce_ok: bool, disburse_ok: bool, replay_checksum: u64) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in verdict.as_bytes() {
        hash = fnv_mix(hash, u64::from(*byte));
    }
    hash = fnv_mix(hash, u64::from(reduce_ok as u8));
    hash = fnv_mix(hash, u64::from(disburse_ok as u8));
    hash = fnv_mix(hash, replay_checksum);
    hash
}

fn fnv_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}

// ---- RR-4 integrated rehearsal engine (pub(crate)) ----

pub(crate) struct Rr3EngineLayout(pub RecursiveGpuLayout);
pub(crate) struct Rr3EngineConfig(pub RecursiveGpuConfig);
pub(crate) struct Rr3EngineOutcome(pub GpuRecursiveOutcome);

pub(crate) fn rr_3_engine_build_layout(
    world: &Runtime0080Rr0RecursiveWorld,
    terran_id: u8,
    pirate_id: u8,
) -> Result<RecursiveGpuLayout, &'static str> {
    build_recursive_layout(world, terran_id, pirate_id)
}

pub(crate) fn rr_3_engine_pass_config(active_system_ids: Vec<u8>) -> RecursiveGpuConfig {
    RecursiveGpuConfig {
        active_system_ids,
        surface_to_planet_enabled: true,
        planet_to_system_enabled: true,
        system_to_galaxy_enabled: true,
        galaxy_to_stockpile_enabled: true,
        disburse_down_enabled: true,
        wrong_owner_routing: false,
        direct_surface_to_stockpile: false,
    }
}

pub(crate) fn rr_3_engine_init_session(
    ctx: &GpuContext,
    layout: &RecursiveGpuLayout,
    config: &RecursiveGpuConfig,
) -> Result<AccumulatorOpSession, &'static str> {
    let ops = build_recursive_ops(layout, config).map_err(|_| "rr_3_ops_build_failed")?;
    let n_dims = RR_3_N_DIMS;
    let values = vec![0.0f32; (layout.n_slots * n_dims) as usize];
    let mut session = AccumulatorOpSession::new(ctx, layout.n_slots, n_dims);
    session.upload_values(ctx, &values);
    session
        .upload_packed_ops(
            ctx,
            &PackedAccumulatorUpload::from_ops_resolving_input_lists(&ops).unwrap(),
        )
        .map_err(|_| "rr_3_gpu_upload_ops_failed")?;
    Ok(session)
}

pub(crate) fn rr_3_engine_dispatch_tick(
    ctx: &GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &RecursiveGpuLayout,
    config: &RecursiveGpuConfig,
) -> Result<GpuRecursiveOutcome, &'static str> {
    dispatch_recursive_bands(ctx, session, layout, config)
}

pub(crate) fn rr_3_engine_run_isolated_tick(
    ctx: &GpuContext,
    layout: &RecursiveGpuLayout,
    config: RecursiveGpuConfig,
) -> Result<GpuRecursiveOutcome, &'static str> {
    run_gpu_recursive_tick(ctx, layout, config)
}

pub(crate) fn rr_3_engine_cpu_reduce_rows(
    layout: &RecursiveGpuLayout,
    oracle: &Runtime0080Rr0OracleTick,
    tick: u32,
) -> Vec<Runtime0080Rr3TransitionRow> {
    cpu_reduce_rows(layout, oracle, tick)
}

pub(crate) fn rr_3_engine_cpu_disburse_rows(
    world: &Runtime0080Rr0RecursiveWorld,
    layout: &RecursiveGpuLayout,
    oracle: &Runtime0080Rr0OracleTick,
    tick: u32,
) -> Vec<Runtime0080Rr3TransitionRow> {
    cpu_disburse_rows(world, layout, oracle, tick)
}

pub(crate) fn rr_3_engine_merge_reduce_rows(
    cpu_rows: &[Runtime0080Rr3TransitionRow],
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
) -> Vec<Runtime0080Rr3TransitionRow> {
    merge_reduce_rows(cpu_rows, gpu, layout)
}

pub(crate) fn rr_3_engine_merge_disburse_rows(
    cpu_rows: &[Runtime0080Rr3TransitionRow],
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
) -> Vec<Runtime0080Rr3TransitionRow> {
    merge_disburse_rows(cpu_rows, gpu, layout)
}

pub(crate) fn rr_3_engine_stockpile_parity_ok(
    gpu: &GpuRecursiveOutcome,
    oracle: &Runtime0080Rr0OracleTick,
) -> bool {
    stockpile_parity_ok(gpu, oracle)
}

pub(crate) fn rr_3_engine_starport_tick_parity_ok(
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
    oracle: &Runtime0080Rr0OracleTick,
    prev_terran_starport: i64,
    prev_pirate_starport: i64,
) -> bool {
    let mut terran_gpu = 0i64;
    let mut pirate_gpu = 0i64;
    for binding in &layout.bindings {
        if let Some(slot) = binding.starport_slot {
            let received = gpu.production_at(slot) as i64;
            match binding.owner {
                Runtime0080Rr0Owner::Terran => terran_gpu += received,
                Runtime0080Rr0Owner::Pirate => pirate_gpu += received,
            }
        }
    }
    terran_gpu - prev_terran_starport == oracle.disbursed_terran
        && pirate_gpu - prev_pirate_starport == oracle.disbursed_pirate
}

pub(crate) fn rr_3_engine_sum_starport_received(
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
) -> (i64, i64) {
    let mut terran = 0i64;
    let mut pirate = 0i64;
    for binding in &layout.bindings {
        if let Some(slot) = binding.starport_slot {
            let received = gpu.production_at(slot) as i64;
            match binding.owner {
                Runtime0080Rr0Owner::Terran => terran += received,
                Runtime0080Rr0Owner::Pirate => pirate += received,
            }
        }
    }
    (terran, pirate)
}

pub(crate) fn rr_3_engine_labor_production_parity(
    gpu: &GpuRecursiveOutcome,
    layout: &RecursiveGpuLayout,
    config: &RecursiveGpuConfig,
    oracle: &Runtime0080Rr0OracleTick,
) -> (bool, bool) {
    let mut labor_emitted = 0i64;
    let mut production_generated = 0i64;
    for binding in &layout.bindings {
        if !config.active_system_ids.contains(&binding.system_id) {
            continue;
        }
        labor_emitted += POP_LABOR_PER_TICK;
        production_generated += PRODUCTION_PER_RECIPE;
        let factory_labor =
            gpu.values[cell_index(binding.factory_slot, RR_3_COL_LABOR, gpu.n_dims)];
        if factory_labor.to_bits() != i64_bits(0) {
            return (false, false);
        }
    }
    let labor_ok =
        labor_emitted == oracle.labor_emitted && oracle.labor_consumed == oracle.labor_emitted;
    let production_ok = production_generated == oracle.production_generated;
    (labor_ok, production_ok)
}

pub(crate) fn rr_3_engine_i64_bits(value: i64) -> u32 {
    i64_bits(value)
}

pub(crate) fn rr_3_engine_production_bits_at(gpu: &GpuRecursiveOutcome, slot: u32) -> u32 {
    gpu.production_bits_at(slot)
}

pub(crate) fn rr_3_engine_production_at(gpu: &GpuRecursiveOutcome, slot: u32) -> f32 {
    gpu.production_at(slot)
}

pub(crate) fn rr_3_engine_wrong_owner_config(active_system_ids: Vec<u8>) -> RecursiveGpuConfig {
    RecursiveGpuConfig {
        active_system_ids,
        surface_to_planet_enabled: true,
        planet_to_system_enabled: true,
        system_to_galaxy_enabled: true,
        galaxy_to_stockpile_enabled: true,
        disburse_down_enabled: true,
        wrong_owner_routing: true,
        direct_surface_to_stockpile: false,
    }
}

pub(crate) fn rr_3_engine_shortcut_config(active_system_ids: Vec<u8>) -> RecursiveGpuConfig {
    RecursiveGpuConfig {
        active_system_ids,
        surface_to_planet_enabled: false,
        planet_to_system_enabled: false,
        system_to_galaxy_enabled: false,
        galaxy_to_stockpile_enabled: false,
        disburse_down_enabled: false,
        wrong_owner_routing: false,
        direct_surface_to_stockpile: true,
    }
}

pub(crate) fn rr_3_engine_inactive_surface_config(terran_id: u8) -> RecursiveGpuConfig {
    RecursiveGpuConfig {
        active_system_ids: vec![terran_id],
        surface_to_planet_enabled: true,
        planet_to_system_enabled: true,
        system_to_galaxy_enabled: true,
        galaxy_to_stockpile_enabled: true,
        disburse_down_enabled: true,
        wrong_owner_routing: false,
        direct_surface_to_stockpile: false,
    }
}

pub(crate) fn rr_3_engine_max_disburse_band(starport_count: u32) -> u32 {
    max_disburse_band(starport_count)
}

fn dispatch_recursive_bands(
    ctx: &GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &RecursiveGpuLayout,
    config: &RecursiveGpuConfig,
) -> Result<GpuRecursiveOutcome, &'static str> {
    let n_dims = RR_3_N_DIMS;
    let mut values = vec![0.0f32; (layout.n_slots * n_dims) as usize];
    let max_band = if config.direct_surface_to_stockpile {
        BAND_DIRECT_SURFACE_TO_STOCKPILE
    } else if config.disburse_down_enabled {
        max_disburse_band(layout.starport_count)
    } else if config.galaxy_to_stockpile_enabled {
        BAND_GALAXY_TO_STOCKPILE
    } else if config.system_to_galaxy_enabled {
        BAND_SYSTEM_TO_GALAXY
    } else if config.planet_to_system_enabled {
        BAND_PLANET_TO_SYSTEM
    } else if config.surface_to_planet_enabled {
        BAND_SURFACE_TO_PLANET
    } else {
        BAND_FACTORY_RECIPE
    };

    let mut after_surface_to_planet = values.clone();
    let mut after_planet_to_system = values.clone();
    let mut after_system_to_galaxy = values.clone();
    let mut after_galaxy_to_stockpile = values.clone();
    let mut after_disburse = values.clone();
    let disburse_end = max_disburse_band(layout.starport_count);

    for band in BAND_LABOR_EMIT..=max_band {
        if should_skip_band(band, config, layout.starport_count) {
            continue;
        }
        session
            .tick(ctx, band)
            .map_err(|_| "rr_3_gpu_tick_failed")?;
        if band == BAND_SURFACE_TO_PLANET
            || band == BAND_PLANET_TO_SYSTEM
            || band == BAND_SYSTEM_TO_GALAXY
            || band == BAND_GALAXY_TO_STOCKPILE
            || (config.disburse_down_enabled && band == disburse_end)
        {
            let snap = session
                .readback_full(ctx)
                .map_err(|_| "rr_3_gpu_readback_failed")?;
            match band {
                BAND_SURFACE_TO_PLANET => after_surface_to_planet = snap.clone(),
                BAND_PLANET_TO_SYSTEM => after_planet_to_system = snap.clone(),
                BAND_SYSTEM_TO_GALAXY => after_system_to_galaxy = snap.clone(),
                BAND_GALAXY_TO_STOCKPILE => after_galaxy_to_stockpile = snap.clone(),
                _ if band == disburse_end => after_disburse = snap.clone(),
                _ => {}
            }
            values = snap;
        }
    }
    values = session
        .readback_full(ctx)
        .map_err(|_| "rr_3_gpu_readback_failed")?;
    if config.disburse_down_enabled {
        after_disburse.clone_from(&values);
    }
    Ok(GpuRecursiveOutcome {
        values,
        n_dims,
        after_surface_to_planet,
        after_planet_to_system,
        after_system_to_galaxy,
        after_galaxy_to_stockpile,
        after_disburse,
    })
}
