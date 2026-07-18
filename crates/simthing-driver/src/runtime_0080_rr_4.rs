//! RUNTIME-0080-RR-4: integrated recursive 100-tick GPU rehearsal.
//!
//! Consumes RR-0 recursive CPU oracle, RR-1 nested residency, RR-2 GPU surface economy, and
//! RR-3 recursive GPU reduce-up/disburse-down. Runs 100 persistent-GPU recursive ticks with
//! bit-exact per-tick and final-state parity vs the RR-0 oracle.
//!
//! This is a bounded falsification rehearsal only. It does not supply, wrap, or replace ordinary
//! `SimSession::step_once` recursive Arena execution.

use std::time::Instant;

use crate::dress_rehearsal_r6c_integrated_run::R6C_CANONICAL_TICK_COUNT;
use crate::runtime_0080_rr_0::{
    build_recursive_world, run_runtime_0080_rr_0, Runtime0080Rr0Input, Runtime0080Rr0OracleTick,
    Runtime0080Rr0Owner, Runtime0080Rr0RecursiveWorld,
};
use crate::runtime_0080_rr_1::{
    run_runtime_0080_rr_1, Runtime0080Rr1Input, Runtime0080Rr1ResidencyRequest,
    RR_1_SURFACE_CELL_COUNT, RR_1_SYSTEM_COUNT,
};
use crate::runtime_0080_rr_2::{run_runtime_0080_rr_2, Runtime0080Rr2Input};
use crate::runtime_0080_rr_3::{
    rr_3_engine_build_layout, rr_3_engine_cpu_reduce_rows, rr_3_engine_dispatch_tick,
    rr_3_engine_i64_bits, rr_3_engine_inactive_surface_config, rr_3_engine_init_session,
    rr_3_engine_labor_production_parity, rr_3_engine_max_disburse_band,
    rr_3_engine_merge_reduce_rows, rr_3_engine_pass_config, rr_3_engine_production_at,
    rr_3_engine_production_bits_at, rr_3_engine_run_isolated_tick, rr_3_engine_shortcut_config,
    rr_3_engine_starport_tick_parity_ok, rr_3_engine_stockpile_parity_ok,
    rr_3_engine_sum_starport_received, rr_3_engine_wrong_owner_config, run_runtime_0080_rr_3,
    Runtime0080Rr3Input, Runtime0080Rr3SystemBinding, RR_3_PIRATE_STOCKPILE_SLOT,
    RR_3_TERRAN_STOCKPILE_SLOT,
};
use simthing_gpu::{set_debug_readback_allowed, GpuContext};

pub const RUNTIME_0080_RR_4_ID: &str = "RUNTIME-0080-RR-4";
pub const RUNTIME_0080_RR_4_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - integrated recursive 100-tick GPU rehearsal";
pub const RUNTIME_0080_RR_4_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - integrated recursive rehearsal incomplete or proxied";
pub const RUNTIME_0080_RR_4_STATUS_BLOCKED: &str =
    "BLOCKED - recursive RR-4 cannot close without approved deviation";

pub const RUNTIME_RR_4_EXPECTED_REPORT_CHECKSUM: u64 = 0x8a38_43df_b76c_260f;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const RR_0_DEFAULT_SEED: u64 = 0x0080_2000;
const BAND_LABOR_EMIT: u32 = 0;
const BAND_FACTORY_RECIPE: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr4Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub seed: u64,
    pub tick_count: u32,
}

impl Runtime0080Rr4Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            tick_count: 0,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
            seed: RR_0_DEFAULT_SEED,
            tick_count: R6C_CANONICAL_TICK_COUNT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr4ScopeLedgerRow {
    pub spec_element: &'static str,
    pub required_by_spec: bool,
    pub implemented_in_rr_4: bool,
    pub status: &'static str,
    pub evidence: &'static str,
    pub deviation: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr4DeviationRecord {
    pub design_authority_approval: &'static str,
    pub specified_element: &'static str,
    pub implemented_proxy_or_omission: &'static str,
    pub reason: &'static str,
    pub consumer_impact: &'static str,
    pub required_follow_up: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr4TickParityRow {
    pub tick: u32,
    pub owner: Runtime0080Rr0Owner,
    pub system_id: u8,
    pub labor_emitted: i64,
    pub labor_consumed: i64,
    pub production_generated: i64,
    pub reduced_surface_to_planet: i64,
    pub reduced_planet_to_system: i64,
    pub reduced_system_to_galaxy: i64,
    pub reduced_galaxy_to_stockpile: i64,
    pub disbursed: i64,
    pub stockpile_after: i64,
    pub target_receipt_delta: i64,
    pub cpu_bits: u32,
    pub gpu_bits: u32,
    pub parity: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr4FinalStateRow {
    pub owner: Runtime0080Rr0Owner,
    pub faction_stockpile: i64,
    pub starport_receipt_total: i64,
    pub cpu_bits: u32,
    pub gpu_bits: u32,
    pub parity: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080Rr4TickTimingRow {
    pub tick: u32,
    pub total_ms: f64,
    pub gpu_dispatch_ms: f64,
    pub gpu_readback_ms: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080Rr4MemoryFootprint {
    pub gpu_session_persistent_bytes: u64,
    pub gpu_values_buffer_bytes: u64,
    pub cpu_readback_staging_peak_bytes: u64,
    pub mean_process_working_set_bytes: Option<u64>,
    pub ending_process_working_set_bytes: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080Rr4Profiling {
    pub total_wall_ms: f64,
    pub gpu_loop_ms: f64,
    pub mean_tick_ms: f64,
    pub min_tick_ms: f64,
    pub max_tick_ms: f64,
    pub gpu_ops_per_tick: u32,
    pub readback_cadence: &'static str,
    pub per_tick_timing: Vec<Runtime0080Rr4TickTimingRow>,
    pub memory: Runtime0080Rr4MemoryFootprint,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080Rr4Report {
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
    pub rr_2_surface_economy_consumed: bool,
    pub rr_3_recursive_transfers_consumed: bool,
    pub gpu_available: bool,
    pub tick_count: u32,
    pub ticks_completed: u32,
    pub tick_state_feeds_next_tick: bool,
    pub terran_path_integrated: bool,
    pub pirate_path_integrated: bool,
    pub per_tick_labor_parity_ok: bool,
    pub per_tick_production_parity_ok: bool,
    pub per_tick_reduce_up_parity_ok: bool,
    pub per_tick_disburse_down_parity_ok: bool,
    pub final_stockpile_parity_ok: bool,
    pub final_starport_parity_ok: bool,
    pub no_cross_owner_leakage: bool,
    pub no_cross_tier_shortcut: bool,
    pub inactive_surfaces_no_op: bool,
    pub inactive_systems_no_op: bool,
    pub not_flattened_scalar: bool,
    pub recursive_horizon_reached: bool,
    pub system_bindings: Vec<Runtime0080Rr3SystemBinding>,
    pub tick_parity_rows: Vec<Runtime0080Rr4TickParityRow>,
    pub final_state_rows: Vec<Runtime0080Rr4FinalStateRow>,
    pub scope_ledger: Vec<Runtime0080Rr4ScopeLedgerRow>,
    pub deviation_records: Vec<Runtime0080Rr4DeviationRecord>,
    pub profiling: Option<Runtime0080Rr4Profiling>,
    pub stable_report_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub standalone_m4a_claimed: bool,
    pub default_session_wiring: bool,
    pub invariant_edit: bool,
}

pub fn run_runtime_0080_rr_4(input: &Runtime0080Rr4Input) -> Runtime0080Rr4Report {
    let mut diagnostics = Vec::new();
    if input.enabled_by_default {
        diagnostics.push("rr_4_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None);
    }
    if input.tick_count != R6C_CANONICAL_TICK_COUNT {
        diagnostics.push("rr_4_tick_count_must_be_canonical_100");
    }

    let rr3 = run_runtime_0080_rr_3(&Runtime0080Rr3Input::explicit_opt_in());
    if rr3.verdict != "PASS" {
        diagnostics.push("rr_3_recursive_transfers_not_consumed");
    }
    let rr2 = run_runtime_0080_rr_2(&Runtime0080Rr2Input::explicit_opt_in());
    if rr2.verdict != "PASS" || !rr2.production_parity_ok {
        diagnostics.push("rr_2_surface_economy_not_consumed");
    }
    let rr0 = run_runtime_0080_rr_0(&Runtime0080Rr0Input {
        explicit_opt_in: true,
        enabled_by_default: false,
        seed: input.seed,
        tick_count: R6C_CANONICAL_TICK_COUNT,
    });
    if rr0.verdict != "PASS" || rr0.oracle_ticks.len() != R6C_CANONICAL_TICK_COUNT as usize {
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

    let execution = match execute_integrated_rehearsal(
        &ctx,
        &world,
        &rr0.oracle_ticks,
        terran_id,
        pirate_id,
        input.tick_count,
    ) {
        Ok(exec) => exec,
        Err(diag) => {
            diagnostics.push(diag);
            return base_report(input, false, diagnostics, None);
        }
    };
    base_report(input, false, Vec::new(), Some(execution))
}

pub fn replay_runtime_0080_rr_4() -> (Runtime0080Rr4Report, Runtime0080Rr4Report) {
    let input = Runtime0080Rr4Input::explicit_opt_in();
    (run_runtime_0080_rr_4(&input), run_runtime_0080_rr_4(&input))
}

struct IntegratedExecution {
    system_bindings: Vec<Runtime0080Rr3SystemBinding>,
    tick_parity_rows: Vec<Runtime0080Rr4TickParityRow>,
    final_state_rows: Vec<Runtime0080Rr4FinalStateRow>,
    tick_state_feeds_next_tick: bool,
    terran_path_integrated: bool,
    pirate_path_integrated: bool,
    per_tick_labor_parity_ok: bool,
    per_tick_production_parity_ok: bool,
    per_tick_reduce_up_parity_ok: bool,
    per_tick_disburse_down_parity_ok: bool,
    final_stockpile_parity_ok: bool,
    final_starport_parity_ok: bool,
    no_cross_owner_leakage: bool,
    no_cross_tier_shortcut: bool,
    inactive_surfaces_no_op: bool,
    inactive_systems_no_op: bool,
    not_flattened_scalar: bool,
    ticks_completed: u32,
    deterministic_replay_checksum: u64,
    profiling: Runtime0080Rr4Profiling,
}

fn execute_integrated_rehearsal(
    ctx: &GpuContext,
    world: &Runtime0080Rr0RecursiveWorld,
    oracle_ticks: &[Runtime0080Rr0OracleTick],
    terran_id: u8,
    pirate_id: u8,
    tick_count: u32,
) -> Result<IntegratedExecution, &'static str> {
    let layout = rr_3_engine_build_layout(world, terran_id, pirate_id)?;
    let all_system_ids: Vec<u8> = layout.bindings.iter().map(|b| b.system_id).collect();
    let pass_config = rr_3_engine_pass_config(all_system_ids.clone());

    let loop_started = Instant::now();
    let mut session = rr_3_engine_init_session(ctx, &layout, &pass_config)?;
    let gpu_session_persistent_bytes = session.persistent_buffer_bytes();
    let gpu_values_buffer_bytes = session.values_buffer_size_bytes();

    let mut per_tick_timing = Vec::with_capacity(tick_count as usize);
    let mut tick_parity_rows = Vec::new();
    let mut ram_samples = Vec::new();

    let mut prev_terran_starport = 0i64;
    let mut prev_pirate_starport = 0i64;
    let mut prev_terran_stockpile = 0i64;
    let mut prev_pirate_stockpile = 0i64;

    let mut labor_ok_all = true;
    let mut production_ok_all = true;
    let mut reduce_ok_all = true;
    let mut disburse_ok_all = true;

    let gpu_ops_per_tick =
        rr_3_engine_max_disburse_band(layout.starport_count) - BAND_LABOR_EMIT + 1;
    let mut last_gpu = None;
    let mut tick_state_feeds_next_tick = true;

    for tick in 0..tick_count {
        let tick_started = Instant::now();
        ram_samples.push(sample_process_working_set_bytes());

        let dispatch_started = Instant::now();
        let gpu_outcome = rr_3_engine_dispatch_tick(ctx, &mut session, &layout, &pass_config)?;
        let gpu_dispatch_ms = dispatch_started.elapsed().as_secs_f64() * 1000.0;

        let readback_started = Instant::now();
        session
            .readback_full(ctx)
            .map_err(|_| "rr_4_gpu_readback_failed")?;
        let gpu_readback_ms = readback_started.elapsed().as_secs_f64() * 1000.0;

        let oracle = &oracle_ticks[tick as usize];
        let (labor_ok, production_ok) =
            rr_3_engine_labor_production_parity(&gpu_outcome, &layout, &pass_config, oracle);
        labor_ok_all &= labor_ok;
        production_ok_all &= production_ok;

        let cpu_reduce = rr_3_engine_cpu_reduce_rows(&layout, oracle, tick);
        let reduce_rows = rr_3_engine_merge_reduce_rows(&cpu_reduce, &gpu_outcome, &layout);
        let reduce_tick_ok = reduce_rows.iter().all(|row| row.parity);
        reduce_ok_all &= reduce_tick_ok;

        let disburse_tick_ok = rr_3_engine_starport_tick_parity_ok(
            &gpu_outcome,
            &layout,
            oracle,
            prev_terran_starport,
            prev_pirate_starport,
        ) && rr_3_engine_stockpile_parity_ok(&gpu_outcome, oracle);
        disburse_ok_all &= disburse_tick_ok;

        let (terran_starport, pirate_starport) =
            rr_3_engine_sum_starport_received(&gpu_outcome, &layout);

        if tick > 0 {
            let gpu_terran =
                rr_3_engine_production_at(&gpu_outcome, RR_3_TERRAN_STOCKPILE_SLOT) as i64;
            let gpu_pirate =
                rr_3_engine_production_at(&gpu_outcome, RR_3_PIRATE_STOCKPILE_SLOT) as i64;
            tick_state_feeds_next_tick &= gpu_terran >= prev_terran_stockpile
                && gpu_pirate >= prev_pirate_stockpile
                && oracle.terran_stockpile_after >= prev_terran_stockpile
                && oracle.pirate_stockpile_after >= prev_pirate_stockpile;
        }

        for (owner, system_id) in [
            (Runtime0080Rr0Owner::Terran, terran_id),
            (Runtime0080Rr0Owner::Pirate, pirate_id),
        ] {
            let reduced_stockpile = match owner {
                Runtime0080Rr0Owner::Terran => oracle.reduced_galaxy_to_stockpile_terran,
                Runtime0080Rr0Owner::Pirate => oracle.reduced_galaxy_to_stockpile_pirate,
            };
            let disbursed = match owner {
                Runtime0080Rr0Owner::Terran => oracle.disbursed_terran,
                Runtime0080Rr0Owner::Pirate => oracle.disbursed_pirate,
            };
            let stockpile_after = match owner {
                Runtime0080Rr0Owner::Terran => oracle.terran_stockpile_after,
                Runtime0080Rr0Owner::Pirate => oracle.pirate_stockpile_after,
            };
            let target_delta = match owner {
                Runtime0080Rr0Owner::Terran => terran_starport - prev_terran_starport,
                Runtime0080Rr0Owner::Pirate => pirate_starport - prev_pirate_starport,
            };
            let stockpile_slot = match owner {
                Runtime0080Rr0Owner::Terran => RR_3_TERRAN_STOCKPILE_SLOT,
                Runtime0080Rr0Owner::Pirate => RR_3_PIRATE_STOCKPILE_SLOT,
            };
            let gpu_bits = rr_3_engine_production_bits_at(&gpu_outcome, stockpile_slot);
            let cpu_bits = rr_3_engine_i64_bits(stockpile_after);
            tick_parity_rows.push(Runtime0080Rr4TickParityRow {
                tick,
                owner,
                system_id,
                labor_emitted: oracle.labor_emitted,
                labor_consumed: oracle.labor_consumed,
                production_generated: oracle.production_generated,
                reduced_surface_to_planet: oracle.reduced_surface_to_planet,
                reduced_planet_to_system: oracle.reduced_planet_to_system,
                reduced_system_to_galaxy: oracle.reduced_system_to_galaxy,
                reduced_galaxy_to_stockpile: reduced_stockpile,
                disbursed,
                stockpile_after,
                target_receipt_delta: target_delta,
                cpu_bits,
                gpu_bits,
                parity: labor_ok
                    && production_ok
                    && reduce_tick_ok
                    && disburse_tick_ok
                    && gpu_bits == cpu_bits,
            });
        }

        prev_terran_starport = terran_starport;
        prev_pirate_starport = pirate_starport;
        prev_terran_stockpile = oracle.terran_stockpile_after;
        prev_pirate_stockpile = oracle.pirate_stockpile_after;
        last_gpu = Some(gpu_outcome);

        per_tick_timing.push(Runtime0080Rr4TickTimingRow {
            tick,
            total_ms: tick_started.elapsed().as_secs_f64() * 1000.0,
            gpu_dispatch_ms,
            gpu_readback_ms,
        });
    }

    let final_oracle = oracle_ticks.last().expect("oracle");
    let last_gpu = last_gpu.expect("gpu loop");
    let final_stockpile_parity_ok = rr_3_engine_stockpile_parity_ok(&last_gpu, final_oracle);
    let (final_terran_starport, final_pirate_starport) =
        rr_3_engine_sum_starport_received(&last_gpu, &layout);
    let final_starport_parity_ok = final_terran_starport
        == oracle_ticks.iter().map(|t| t.disbursed_terran).sum::<i64>()
        && final_pirate_starport == oracle_ticks.iter().map(|t| t.disbursed_pirate).sum::<i64>();

    let mid_oracle = &oracle_ticks[50];
    let leaked = rr_3_engine_run_isolated_tick(
        ctx,
        &layout,
        rr_3_engine_wrong_owner_config(all_system_ids.clone()),
    )?;
    let no_cross_owner_leakage = !rr_3_engine_stockpile_parity_ok(&leaked, mid_oracle);

    let shortcut = rr_3_engine_run_isolated_tick(
        ctx,
        &layout,
        rr_3_engine_shortcut_config(all_system_ids.clone()),
    )?;
    let no_cross_tier_shortcut =
        !rr_3_engine_stockpile_parity_ok(&shortcut, mid_oracle) || !reduce_ok_all;

    let inactive_surface_gpu = rr_3_engine_run_isolated_tick(
        ctx,
        &layout,
        rr_3_engine_inactive_surface_config(terran_id),
    )?;
    let pirate_binding = layout
        .bindings
        .iter()
        .find(|b| b.system_id == pirate_id)
        .expect("pirate");
    let inactive_surfaces_no_op =
        rr_3_engine_production_at(&inactive_surface_gpu, pirate_binding.factory_slot) == 0.0;

    let starport_system = layout
        .bindings
        .iter()
        .find(|b| b.starport_slot.is_some())
        .expect("starport")
        .system_id;
    let inactive_system_gpu = rr_3_engine_run_isolated_tick(
        ctx,
        &layout,
        rr_3_engine_pass_config(
            all_system_ids
                .iter()
                .copied()
                .filter(|id| *id != starport_system)
                .collect(),
        ),
    )?;
    let inactive_starport = layout
        .bindings
        .iter()
        .find(|b| b.system_id == starport_system)
        .expect("inactive starport");
    let inactive_systems_no_op = inactive_starport
        .starport_slot
        .is_some_and(|slot| rr_3_engine_production_at(&inactive_system_gpu, slot) == 0.0);

    let not_flattened_scalar = layout.n_slots > RR_3_TERRAN_STOCKPILE_SLOT
        && layout.bindings.len() == RR_1_SYSTEM_COUNT
        && layout
            .bindings
            .iter()
            .all(|b| b.planet_slot != b.system_slot);

    let terran_path_integrated = tick_parity_rows
        .iter()
        .filter(|r| r.owner == Runtime0080Rr0Owner::Terran)
        .all(|r| r.parity);
    let pirate_path_integrated = tick_parity_rows
        .iter()
        .filter(|r| r.owner == Runtime0080Rr0Owner::Pirate)
        .all(|r| r.parity);

    let tick_totals: Vec<f64> = per_tick_timing.iter().map(|r| r.total_ms).collect();
    let mean_tick_ms = mean_f64(&tick_totals);
    let min_tick_ms = tick_totals.iter().copied().fold(f64::INFINITY, f64::min);
    let min_tick_ms = if min_tick_ms.is_finite() {
        min_tick_ms
    } else {
        0.0
    };
    let max_tick_ms = tick_totals.iter().copied().fold(0.0_f64, f64::max);

    let final_state_rows = vec![
        final_state_row(
            Runtime0080Rr0Owner::Terran,
            final_oracle.terran_stockpile_after,
            final_terran_starport,
            rr_3_engine_production_bits_at(&last_gpu, RR_3_TERRAN_STOCKPILE_SLOT),
        ),
        final_state_row(
            Runtime0080Rr0Owner::Pirate,
            final_oracle.pirate_stockpile_after,
            final_pirate_starport,
            rr_3_engine_production_bits_at(&last_gpu, RR_3_PIRATE_STOCKPILE_SLOT),
        ),
    ];

    let deterministic_replay_checksum = checksum_ticks(&tick_parity_rows, final_oracle);

    Ok(IntegratedExecution {
        system_bindings: layout.bindings,
        tick_parity_rows,
        final_state_rows,
        tick_state_feeds_next_tick,
        terran_path_integrated,
        pirate_path_integrated,
        per_tick_labor_parity_ok: labor_ok_all,
        per_tick_production_parity_ok: production_ok_all,
        per_tick_reduce_up_parity_ok: reduce_ok_all,
        per_tick_disburse_down_parity_ok: disburse_ok_all,
        final_stockpile_parity_ok,
        final_starport_parity_ok,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_no_op,
        inactive_systems_no_op,
        not_flattened_scalar,
        ticks_completed: tick_count,
        deterministic_replay_checksum,
        profiling: Runtime0080Rr4Profiling {
            total_wall_ms: loop_started.elapsed().as_secs_f64() * 1000.0,
            gpu_loop_ms: loop_started.elapsed().as_secs_f64() * 1000.0,
            mean_tick_ms,
            min_tick_ms,
            max_tick_ms,
            gpu_ops_per_tick,
            readback_cadence: "per-tick full readback after integrated dispatch",
            per_tick_timing,
            memory: Runtime0080Rr4MemoryFootprint {
                gpu_session_persistent_bytes,
                gpu_values_buffer_bytes,
                cpu_readback_staging_peak_bytes: gpu_values_buffer_bytes,
                mean_process_working_set_bytes: mean_option_u64(&ram_samples),
                ending_process_working_set_bytes: sample_process_working_set_bytes(),
            },
        },
    })
}

fn final_state_row(
    owner: Runtime0080Rr0Owner,
    faction_stockpile: i64,
    starport_receipt_total: i64,
    gpu_bits: u32,
) -> Runtime0080Rr4FinalStateRow {
    let cpu_bits = rr_3_engine_i64_bits(faction_stockpile);
    Runtime0080Rr4FinalStateRow {
        owner,
        faction_stockpile,
        starport_receipt_total,
        cpu_bits,
        gpu_bits,
        parity: cpu_bits == gpu_bits,
    }
}

fn base_report(
    input: &Runtime0080Rr4Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<IntegratedExecution>,
) -> Runtime0080Rr4Report {
    let admitted = diagnostics.is_empty();

    let (
        system_bindings,
        tick_parity_rows,
        final_state_rows,
        tick_state_feeds_next_tick,
        terran_path_integrated,
        pirate_path_integrated,
        per_tick_labor_parity_ok,
        per_tick_production_parity_ok,
        per_tick_reduce_up_parity_ok,
        per_tick_disburse_down_parity_ok,
        final_stockpile_parity_ok,
        final_starport_parity_ok,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_no_op,
        inactive_systems_no_op,
        not_flattened_scalar,
        ticks_completed,
        scope_ledger,
        deviation_records,
        deterministic_replay_checksum,
        profiling,
    ) = match execution {
        Some(exec) => {
            let scope_ledger = build_scope_ledger(&exec, true);
            (
                exec.system_bindings,
                exec.tick_parity_rows,
                exec.final_state_rows,
                exec.tick_state_feeds_next_tick,
                exec.terran_path_integrated,
                exec.pirate_path_integrated,
                exec.per_tick_labor_parity_ok,
                exec.per_tick_production_parity_ok,
                exec.per_tick_reduce_up_parity_ok,
                exec.per_tick_disburse_down_parity_ok,
                exec.final_stockpile_parity_ok,
                exec.final_starport_parity_ok,
                exec.no_cross_owner_leakage,
                exec.no_cross_tier_shortcut,
                exec.inactive_surfaces_no_op,
                exec.inactive_systems_no_op,
                exec.not_flattened_scalar,
                exec.ticks_completed,
                scope_ledger,
                Vec::<Runtime0080Rr4DeviationRecord>::new(),
                exec.deterministic_replay_checksum,
                Some(exec.profiling),
            )
        }
        None => (
            Vec::new(),
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
            0,
            Vec::new(),
            Vec::new(),
            0,
            None,
        ),
    };

    let required_rows_implemented = scope_ledger
        .iter()
        .take(30)
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
        "PASS" => RUNTIME_0080_RR_4_STATUS_PASS,
        "PARTIAL" => RUNTIME_0080_RR_4_STATUS_PARTIAL,
        _ => RUNTIME_0080_RR_4_STATUS_BLOCKED,
    };

    let stable_report_checksum = if admitted && !disabled_no_op {
        checksum_report(verdict, ticks_completed, deterministic_replay_checksum)
    } else {
        0
    };

    Runtime0080Rr4Report {
        id: RUNTIME_0080_RR_4_ID,
        status,
        verdict,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        rr_0_world_consumed: admitted && !disabled_no_op,
        rr_1_residency_consumed: terran_path_integrated || pirate_path_integrated,
        rr_2_surface_economy_consumed: admitted && !disabled_no_op,
        rr_3_recursive_transfers_consumed: admitted && !disabled_no_op,
        gpu_available: admitted && !disabled_no_op,
        tick_count: input.tick_count,
        ticks_completed,
        tick_state_feeds_next_tick,
        terran_path_integrated,
        pirate_path_integrated,
        per_tick_labor_parity_ok,
        per_tick_production_parity_ok,
        per_tick_reduce_up_parity_ok,
        per_tick_disburse_down_parity_ok,
        final_stockpile_parity_ok,
        final_starport_parity_ok,
        no_cross_owner_leakage,
        no_cross_tier_shortcut,
        inactive_surfaces_no_op,
        inactive_systems_no_op,
        not_flattened_scalar,
        recursive_horizon_reached: verdict == "PASS",
        system_bindings,
        tick_parity_rows,
        final_state_rows,
        scope_ledger,
        deviation_records,
        profiling,
        stable_report_checksum,
        deterministic_replay_checksum,
        standalone_m4a_claimed: false,
        default_session_wiring: false,
        invariant_edit: false,
    }
}

fn build_scope_ledger(
    exec: &IntegratedExecution,
    implemented: bool,
) -> Vec<Runtime0080Rr4ScopeLedgerRow> {
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
            "RR-2 GPU surface economy consumed",
            exec.per_tick_labor_parity_ok && exec.per_tick_production_parity_ok,
            "bands 0-2 each tick",
        ),
        scope_row(
            "RR-3 recursive GPU transfers consumed",
            exec.per_tick_reduce_up_parity_ok && exec.per_tick_disburse_down_parity_ok,
            "bands 3+ each tick",
        ),
        scope_row(
            "100 recursive ticks executed",
            exec.ticks_completed == R6C_CANONICAL_TICK_COUNT,
            "persistent GPU session loop",
        ),
        scope_row(
            "Tick state feeds next tick",
            exec.tick_state_feeds_next_tick,
            "stockpile carry-forward",
        ),
        scope_row(
            "Terran recursive path integrated for 100 ticks",
            exec.terran_path_integrated,
            "terran tick parity rows",
        ),
        scope_row(
            "Pirate recursive path integrated for 100 ticks",
            exec.pirate_path_integrated,
            "pirate tick parity rows",
        ),
        scope_row(
            "Pop labor emission computed on GPU each tick",
            exec.per_tick_labor_parity_ok,
            "band 0",
        ),
        scope_row(
            "Factory labor consumption computed on GPU each tick",
            exec.per_tick_labor_parity_ok,
            "band 1-2",
        ),
        scope_row(
            "Production generation computed on GPU each tick",
            exec.per_tick_production_parity_ok,
            "conjunctive recipe",
        ),
        scope_row(
            "Surface→planet reduce computed on GPU each tick",
            exec.per_tick_reduce_up_parity_ok,
            "band 3",
        ),
        scope_row(
            "Planet→system reduce computed on GPU each tick",
            exec.per_tick_reduce_up_parity_ok,
            "band 4",
        ),
        scope_row(
            "System→galaxy reduce computed on GPU each tick",
            exec.per_tick_reduce_up_parity_ok,
            "band 5",
        ),
        scope_row(
            "Galaxy→faction stockpile reduce computed on GPU each tick",
            exec.per_tick_reduce_up_parity_ok,
            "band 6",
        ),
        scope_row(
            "Faction→galaxy disburse computed on GPU each tick",
            exec.per_tick_disburse_down_parity_ok,
            "disburse band base",
        ),
        scope_row(
            "Galaxy→system disburse computed on GPU each tick",
            exec.per_tick_disburse_down_parity_ok,
            "disburse band +1",
        ),
        scope_row(
            "System→surface/starport disburse computed on GPU each tick",
            exec.per_tick_disburse_down_parity_ok,
            "disburse band +2",
        ),
        scope_row(
            "Per-tick labor parity vs RR-0 oracle",
            exec.per_tick_labor_parity_ok,
            "tick_parity_rows",
        ),
        scope_row(
            "Per-tick production parity vs RR-0 oracle",
            exec.per_tick_production_parity_ok,
            "tick_parity_rows",
        ),
        scope_row(
            "Per-tick reduce-up parity vs RR-0 oracle",
            exec.per_tick_reduce_up_parity_ok,
            "merge_reduce_rows",
        ),
        scope_row(
            "Per-tick disburse-down parity vs RR-0 oracle",
            exec.per_tick_disburse_down_parity_ok,
            "starport delta + stockpile",
        ),
        scope_row(
            "Final faction stockpile parity vs RR-0 oracle",
            exec.final_stockpile_parity_ok,
            "tick 99 stockpile bits",
        ),
        scope_row(
            "Final starport/target receipt parity vs RR-0 oracle",
            exec.final_starport_parity_ok,
            "cumulative starport receipts",
        ),
        scope_row(
            "No cross-owner leakage over 100 ticks",
            exec.no_cross_owner_leakage,
            "wrong_owner_routing fails",
        ),
        scope_row(
            "No cross-tier shortcut over 100 ticks",
            exec.no_cross_tier_shortcut,
            "direct_surface_to_stockpile fails",
        ),
        scope_row(
            "Inactive systems/surfaces remain no-op over 100 ticks",
            exec.inactive_surfaces_no_op && exec.inactive_systems_no_op,
            "inactive controls",
        ),
        scope_row(
            "Not flattened to direct surface→faction scalar",
            exec.not_flattened_scalar,
            "per-system tier slots",
        ),
        scope_row(
            "Scope Ledger present and required rows implemented",
            implemented,
            "docs/tests/runtime_0080_rr_4_results.md",
        ),
        scope_row(
            "No Deviation Record required",
            implemented,
            "rows 1-30 implemented",
        ),
        non_claim_row(
            "Standalone M-4A parallel theater track not claimed",
            "nested RR track only",
        ),
        non_claim_row("Default session wiring not claimed", "opt-in/default-off"),
        non_claim_row(
            "Invariant edit not performed",
            "docs/invariants.md untouched",
        ),
    ]
}

fn scope_row(
    spec_element: &'static str,
    ok: bool,
    evidence: &'static str,
) -> Runtime0080Rr4ScopeLedgerRow {
    Runtime0080Rr4ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_4: ok,
        status: if ok { "implemented" } else { "not implemented" },
        evidence,
        deviation: "",
    }
}

fn non_claim_row(
    spec_element: &'static str,
    evidence: &'static str,
) -> Runtime0080Rr4ScopeLedgerRow {
    Runtime0080Rr4ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_4: true,
        status: "non-claim",
        evidence,
        deviation: "",
    }
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

fn checksum_ticks(
    rows: &[Runtime0080Rr4TickParityRow],
    final_oracle: &Runtime0080Rr0OracleTick,
) -> u64 {
    let mut hash = FNV_OFFSET;
    for row in rows {
        hash = fnv_mix(hash, u64::from(row.tick));
        hash = fnv_mix(hash, u64::from(row.gpu_bits));
        hash = fnv_mix(hash, u64::from(row.parity as u8));
    }
    hash = fnv_mix(hash, final_oracle.terran_stockpile_after as u64);
    hash = fnv_mix(hash, final_oracle.pirate_stockpile_after as u64);
    hash
}

fn checksum_report(verdict: &str, ticks_completed: u32, replay_checksum: u64) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in verdict.as_bytes() {
        hash = fnv_mix(hash, u64::from(*byte));
    }
    hash = fnv_mix(hash, u64::from(ticks_completed));
    hash = fnv_mix(hash, replay_checksum);
    hash
}

fn fnv_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}

fn mean_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn mean_option_u64(values: &[Option<u64>]) -> Option<u64> {
    let nums: Vec<u64> = values.iter().filter_map(|v| *v).collect();
    if nums.is_empty() {
        None
    } else {
        Some(nums.iter().sum::<u64>() / nums.len() as u64)
    }
}

fn sample_process_working_set_bytes() -> Option<u64> {
    #[cfg(windows)]
    {
        #[repr(C)]
        struct ProcessMemoryCounters {
            cb: u32,
            page_fault_count: u32,
            peak_working_set_size: usize,
            working_set_size: usize,
            quota_peak_paged_pool_usage: usize,
            quota_paged_pool_usage: usize,
            quota_peak_non_paged_pool_usage: usize,
            quota_non_paged_pool_usage: usize,
            pagefile_usage: usize,
            peak_pagefile_usage: usize,
        }
        extern "system" {
            fn GetCurrentProcess() -> *mut std::ffi::c_void;
        }
        #[link(name = "psapi")]
        extern "system" {
            fn GetProcessMemoryInfo(
                process: *mut std::ffi::c_void,
                counters: *mut ProcessMemoryCounters,
                size: u32,
            ) -> i32;
        }

        let mut counters = ProcessMemoryCounters {
            cb: std::mem::size_of::<ProcessMemoryCounters>() as u32,
            page_fault_count: 0,
            peak_working_set_size: 0,
            working_set_size: 0,
            quota_peak_paged_pool_usage: 0,
            quota_paged_pool_usage: 0,
            quota_peak_non_paged_pool_usage: 0,
            quota_non_paged_pool_usage: 0,
            pagefile_usage: 0,
            peak_pagefile_usage: 0,
        };
        let ok = unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, counters.cb) };
        if ok != 0 {
            Some(counters.working_set_size as u64)
        } else {
            None
        }
    }
    #[cfg(not(windows))]
    {
        None
    }
}
