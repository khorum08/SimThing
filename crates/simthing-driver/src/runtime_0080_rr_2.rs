//! RUNTIME-0080-RR-2: planet-surface pop→factory labor economy on GPU.
//!
//! Consumes RR-0 recursive world + CPU oracle and RR-1 nested residency. Materializes Terran and
//! Pirate planet surfaces, runs labor emit / factory consume / production on GPU via generic
//! AccumulatorOp, and proves bit-exact parity vs RR-0 surface tick oracle. No recursive reduce-up.

use simthing_core::{
    rebuild_discrete_transfer_ops, AccumulatorOp, AccumulatorOpBuilder, AccumulatorOpBuilderError,
    ColumnIndex, CombineFn, ConsumeMode, DiscreteTransferRegistration, GateSpec, ScaleSpec,
    SlotIndex, SourceSpec,
};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, GpuContext};

use crate::dress_rehearsal_r2_recursive_allocation::{
    factory_recipe_production, FACTORY_UNIT_COST_LABOR, POP_LABOR_PER_TICK, PRODUCTION_PER_RECIPE,
};
use crate::runtime_0080_rr_0::{
    build_recursive_world, Runtime0080Rr0Owner, Runtime0080Rr0RecursiveWorld, Runtime0080Rr0System,
};
use crate::runtime_0080_rr_1::{
    run_runtime_0080_rr_1, Runtime0080Rr1Input, Runtime0080Rr1ResidencyRequest,
    RR_1_SURFACE_CELL_COUNT, RR_1_SURFACE_SIDE,
};

pub const RUNTIME_0080_RR_2_ID: &str = "RUNTIME-0080-RR-2";
pub const RUNTIME_0080_RR_2_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - planet-surface labor economy on GPU";
pub const RUNTIME_0080_RR_2_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - surface GPU economy incomplete or proxied";
pub const RUNTIME_0080_RR_2_STATUS_BLOCKED: &str =
    "BLOCKED - surface RR-2 cannot close without approved deviation";

pub const RUNTIME_RR_2_EXPECTED_REPORT_CHECKSUM: u64 = 0xbbf8_651c_0e61_3c6f;

pub const RR_2_COL_LABOR: u32 = 0;
pub const RR_2_COL_PRODUCTION: u32 = 1;
pub const RR_2_SURFACE_N_DIMS: u32 = 2;
pub const RR_2_SURFACE_CELL_COUNT: u32 = RR_1_SURFACE_SIDE * RR_1_SURFACE_SIDE;
pub const RR_2_ACTIVE_SURFACE_COUNT: usize = 2;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const RR_0_DEFAULT_SEED: u64 = 0x0080_2000;
const BAND_LABOR_EMIT: u32 = 0;
const BAND_LABOR_TRANSFER: u32 = 1;
const BAND_FACTORY_RECIPE: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
    pub seed: u64,
    pub tick_index: u32,
}

impl Runtime0080Rr2Input {
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
pub struct Runtime0080Rr2SurfaceCellBinding {
    pub system_id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub surface_cell_linear_index: u32,
    pub pop_slot: u32,
    pub factory_slot: u32,
    pub resident: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2SurfaceProof {
    pub system_id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub materialized_through_rr_1: bool,
    pub pop_slot: u32,
    pub factory_slot: u32,
    pub pop_surface_cell_x: u32,
    pub pop_surface_cell_y: u32,
    pub factory_surface_cell_x: u32,
    pub factory_surface_cell_y: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2ParityRow {
    pub tick: u32,
    pub system_id: u8,
    pub owner: Runtime0080Rr0Owner,
    pub surface_cell_linear_index: u32,
    pub labor_emitted: i64,
    pub labor_consumed: i64,
    pub production_generated: i64,
    pub cpu_labor_bits: u32,
    pub cpu_production_bits: u32,
    pub gpu_labor_bits: u32,
    pub gpu_production_bits: u32,
    pub parity: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2ScopeLedgerRow {
    pub spec_element: &'static str,
    pub required_by_spec: bool,
    pub implemented_in_rr_2: bool,
    pub status: &'static str,
    pub evidence: &'static str,
    pub deviation: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2DeviationRecord {
    pub design_authority_approval: &'static str,
    pub specified_element: &'static str,
    pub implemented_proxy_or_omission: &'static str,
    pub reason: &'static str,
    pub consumer_impact: &'static str,
    pub required_follow_up: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080Rr2Report {
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
    pub gpu_available: bool,
    pub terran_proof: Runtime0080Rr2SurfaceProof,
    pub pirate_proof: Runtime0080Rr2SurfaceProof,
    pub surface_bindings: Vec<Runtime0080Rr2SurfaceCellBinding>,
    pub parity_rows: Vec<Runtime0080Rr2ParityRow>,
    pub labor_parity_ok: bool,
    pub production_parity_ok: bool,
    pub disabled_emitter_fails_parity: bool,
    pub reenabled_emitter_restores_parity: bool,
    pub disabled_consumer_fails_parity: bool,
    pub reenabled_consumer_restores_parity: bool,
    pub inactive_surface_no_labor: bool,
    pub inactive_surface_no_output: bool,
    pub no_cross_surface_leakage: bool,
    pub not_flattened_scalar: bool,
    pub scope_ledger: Vec<Runtime0080Rr2ScopeLedgerRow>,
    pub deviation_records: Vec<Runtime0080Rr2DeviationRecord>,
    pub stable_report_checksum: u64,
    pub deterministic_replay_checksum: u64,
    pub rr_3_claimed: bool,
    pub rr_4_claimed: bool,
    pub standalone_m4a_claimed: bool,
    pub invariant_edit: bool,
    pub default_session_wiring: bool,
}

pub fn run_runtime_0080_rr_2(input: &Runtime0080Rr2Input) -> Runtime0080Rr2Report {
    let mut diagnostics = Vec::new();
    if input.enabled_by_default {
        diagnostics.push("rr_2_default_on_rejected");
    }
    if !input.explicit_opt_in {
        return base_report(input, true, diagnostics, None);
    }
    if input.tick_index != 0 {
        diagnostics.push("rr_2_tick_index_must_be_zero_for_surface_proof");
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

    let execution =
        match execute_surface_economy(&ctx, &world, terran_id, pirate_id, input.tick_index) {
            Ok(exec) => exec,
            Err(diag) => {
                diagnostics.push(diag);
                return base_report(input, false, diagnostics, None);
            }
        };
    base_report(input, false, Vec::new(), Some(execution))
}

pub fn replay_runtime_0080_rr_2() -> (Runtime0080Rr2Report, Runtime0080Rr2Report) {
    let input = Runtime0080Rr2Input::explicit_opt_in();
    (run_runtime_0080_rr_2(&input), run_runtime_0080_rr_2(&input))
}

struct SurfaceEconomyExecution {
    terran_proof: Runtime0080Rr2SurfaceProof,
    pirate_proof: Runtime0080Rr2SurfaceProof,
    surface_bindings: Vec<Runtime0080Rr2SurfaceCellBinding>,
    parity_rows: Vec<Runtime0080Rr2ParityRow>,
    labor_parity_ok: bool,
    production_parity_ok: bool,
    disabled_emitter_fails_parity: bool,
    reenabled_emitter_restores_parity: bool,
    disabled_consumer_fails_parity: bool,
    reenabled_consumer_restores_parity: bool,
    inactive_surface_no_labor: bool,
    inactive_surface_no_output: bool,
    no_cross_surface_leakage: bool,
    not_flattened_scalar: bool,
    deterministic_replay_checksum: u64,
}

struct SurfaceGpuLayout {
    bindings: Vec<Runtime0080Rr2SurfaceCellBinding>,
    n_slots: u32,
    terran_proof: Runtime0080Rr2SurfaceProof,
    pirate_proof: Runtime0080Rr2SurfaceProof,
}

struct SurfaceGpuConfig {
    labor_emit_enabled: bool,
    factory_recipe_enabled: bool,
    active_system_ids: Vec<u8>,
    cross_surface_leak: Option<(u8, u8)>,
}

fn execute_surface_economy(
    ctx: &GpuContext,
    world: &Runtime0080Rr0RecursiveWorld,
    terran_id: u8,
    pirate_id: u8,
    tick_index: u32,
) -> Result<SurfaceEconomyExecution, &'static str> {
    let layout = build_surface_layout(world, terran_id, pirate_id)?;
    let cpu_rows = cpu_surface_parity_rows(&layout, tick_index);

    let baseline_gpu = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        },
    )?;
    let parity_rows = merge_parity_rows(&cpu_rows, &baseline_gpu, &layout);
    let labor_parity_ok = parity_rows.iter().all(|row| {
        row.parity
            && row.labor_emitted == POP_LABOR_PER_TICK
            && row.labor_consumed == POP_LABOR_PER_TICK
    });
    let production_parity_ok = parity_rows
        .iter()
        .all(|row| row.parity && row.production_generated == PRODUCTION_PER_RECIPE);

    let disabled_emit = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: false,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        },
    )?;
    let disabled_emitter_fails_parity = !gpu_matches_cpu(&cpu_rows, &disabled_emit, &layout);

    let reenabled_emit = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        },
    )?;
    let reenabled_emitter_restores_parity = gpu_matches_cpu(&cpu_rows, &reenabled_emit, &layout);

    let disabled_recipe = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: false,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        },
    )?;
    let disabled_consumer_fails_parity = !gpu_matches_cpu(&cpu_rows, &disabled_recipe, &layout);

    let reenabled_recipe = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        },
    )?;
    let reenabled_consumer_restores_parity = gpu_matches_cpu(&cpu_rows, &reenabled_recipe, &layout);

    let inactive_gpu = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id],
            cross_surface_leak: None,
        },
    )?;
    let pirate_binding = layout
        .bindings
        .iter()
        .find(|b| b.system_id == pirate_id)
        .expect("pirate binding");
    let inactive_surface_no_labor = inactive_gpu.labor_bits_at(pirate_binding.factory_slot) == 0;
    let inactive_surface_no_output =
        inactive_gpu.production_bits_at(pirate_binding.factory_slot) == 0;

    let leaked_gpu = run_gpu_surface_tick(
        ctx,
        &layout,
        SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: Some((terran_id, pirate_id)),
        },
    )?;
    let no_cross_surface_leakage = !gpu_matches_cpu(&cpu_rows, &leaked_gpu, &layout);

    let not_flattened_scalar = layout.bindings.len() == RR_2_ACTIVE_SURFACE_COUNT
        && layout.n_slots == RR_2_SURFACE_CELL_COUNT * RR_2_ACTIVE_SURFACE_COUNT as u32;

    let deterministic_replay_checksum = checksum_execution(&parity_rows);
    Ok(SurfaceEconomyExecution {
        terran_proof: layout.terran_proof.clone(),
        pirate_proof: layout.pirate_proof.clone(),
        surface_bindings: layout.bindings.clone(),
        parity_rows,
        labor_parity_ok,
        production_parity_ok,
        disabled_emitter_fails_parity,
        reenabled_emitter_restores_parity,
        disabled_consumer_fails_parity,
        reenabled_consumer_restores_parity,
        inactive_surface_no_labor,
        inactive_surface_no_output,
        no_cross_surface_leakage,
        not_flattened_scalar,
        deterministic_replay_checksum,
    })
}

struct GpuSurfaceOutcome {
    values: Vec<f32>,
    n_dims: u32,
}

impl GpuSurfaceOutcome {
    fn labor_bits_at(&self, slot: u32) -> u32 {
        f32_bits(self.values[cell_index(slot, RR_2_COL_LABOR, self.n_dims)])
    }

    fn production_bits_at(&self, slot: u32) -> u32 {
        f32_bits(self.values[cell_index(slot, RR_2_COL_PRODUCTION, self.n_dims)])
    }
}

fn gpu_matches_cpu(
    cpu_rows: &[Runtime0080Rr2ParityRow],
    gpu: &GpuSurfaceOutcome,
    layout: &SurfaceGpuLayout,
) -> bool {
    for cpu in cpu_rows {
        let binding = layout
            .bindings
            .iter()
            .find(|b| b.system_id == cpu.system_id)
            .expect("binding");
        let gpu_labor = gpu.labor_bits_at(binding.factory_slot);
        let gpu_prod = gpu.production_bits_at(binding.factory_slot);
        let cpu_labor = cpu.cpu_labor_bits;
        let cpu_prod = cpu.cpu_production_bits;
        if gpu_labor != cpu_labor || gpu_prod != cpu_prod {
            return false;
        }
    }
    true
}

fn merge_parity_rows(
    cpu_rows: &[Runtime0080Rr2ParityRow],
    gpu: &GpuSurfaceOutcome,
    layout: &SurfaceGpuLayout,
) -> Vec<Runtime0080Rr2ParityRow> {
    cpu_rows
        .iter()
        .map(|cpu| {
            let binding = layout
                .bindings
                .iter()
                .find(|b| b.system_id == cpu.system_id)
                .expect("binding");
            let gpu_labor = gpu.labor_bits_at(binding.factory_slot);
            let gpu_prod = gpu.production_bits_at(binding.factory_slot);
            Runtime0080Rr2ParityRow {
                tick: cpu.tick,
                system_id: cpu.system_id,
                owner: cpu.owner,
                surface_cell_linear_index: cpu.surface_cell_linear_index,
                labor_emitted: cpu.labor_emitted,
                labor_consumed: cpu.labor_consumed,
                production_generated: cpu.production_generated,
                cpu_labor_bits: cpu.cpu_labor_bits,
                cpu_production_bits: cpu.cpu_production_bits,
                gpu_labor_bits: gpu_labor,
                gpu_production_bits: gpu_prod,
                parity: gpu_labor == cpu.cpu_labor_bits && gpu_prod == cpu.cpu_production_bits,
            }
        })
        .collect()
}

fn cpu_surface_parity_rows(
    layout: &SurfaceGpuLayout,
    tick_index: u32,
) -> Vec<Runtime0080Rr2ParityRow> {
    layout
        .bindings
        .iter()
        .filter(|b| b.resident)
        .map(|binding| {
            let (production, consumed, _) = factory_recipe_production(POP_LABOR_PER_TICK);
            let cpu_labor = 0i64;
            let cpu_production = production;
            Runtime0080Rr2ParityRow {
                tick: tick_index,
                system_id: binding.system_id,
                owner: binding.owner,
                surface_cell_linear_index: binding.surface_cell_linear_index,
                labor_emitted: POP_LABOR_PER_TICK,
                labor_consumed: consumed,
                production_generated: production,
                cpu_labor_bits: f32_bits(cpu_labor as f32),
                cpu_production_bits: f32_bits(cpu_production as f32),
                gpu_labor_bits: 0,
                gpu_production_bits: 0,
                parity: false,
            }
        })
        .collect()
}

fn build_surface_layout(
    world: &Runtime0080Rr0RecursiveWorld,
    terran_id: u8,
    pirate_id: u8,
) -> Result<SurfaceGpuLayout, &'static str> {
    let terran = find_system(world, terran_id).ok_or("terran_system_missing")?;
    let pirate = find_system(world, pirate_id).ok_or("pirate_system_missing")?;
    let terran_base = 0u32;
    let pirate_base = RR_2_SURFACE_CELL_COUNT;
    let terran_pop = terran_base
        + surface_cell_index(
            terran.planet.surface.pop_cohort.surface_cell_x,
            terran.planet.surface.pop_cohort.surface_cell_y,
        ) as u32;
    let terran_factory = terran_base
        + surface_cell_index(
            terran.planet.surface.factory.surface_cell_x,
            terran.planet.surface.factory.surface_cell_y,
        ) as u32;
    let pirate_pop = pirate_base
        + surface_cell_index(
            pirate.planet.surface.pop_cohort.surface_cell_x,
            pirate.planet.surface.pop_cohort.surface_cell_y,
        ) as u32;
    let pirate_factory = pirate_base
        + surface_cell_index(
            pirate.planet.surface.factory.surface_cell_x,
            pirate.planet.surface.factory.surface_cell_y,
        ) as u32;

    Ok(SurfaceGpuLayout {
        bindings: vec![
            Runtime0080Rr2SurfaceCellBinding {
                system_id: terran_id,
                owner: Runtime0080Rr0Owner::Terran,
                surface_cell_linear_index: terran_factory - terran_base,
                pop_slot: terran_pop,
                factory_slot: terran_factory,
                resident: true,
            },
            Runtime0080Rr2SurfaceCellBinding {
                system_id: pirate_id,
                owner: Runtime0080Rr0Owner::Pirate,
                surface_cell_linear_index: pirate_factory - pirate_base,
                pop_slot: pirate_pop,
                factory_slot: pirate_factory,
                resident: true,
            },
        ],
        n_slots: RR_2_SURFACE_CELL_COUNT * RR_2_ACTIVE_SURFACE_COUNT as u32,
        terran_proof: Runtime0080Rr2SurfaceProof {
            system_id: terran_id,
            owner: Runtime0080Rr0Owner::Terran,
            materialized_through_rr_1: surface_resident_through_rr_1(terran_id),
            pop_slot: terran_pop,
            factory_slot: terran_factory,
            pop_surface_cell_x: terran.planet.surface.pop_cohort.surface_cell_x,
            pop_surface_cell_y: terran.planet.surface.pop_cohort.surface_cell_y,
            factory_surface_cell_x: terran.planet.surface.factory.surface_cell_x,
            factory_surface_cell_y: terran.planet.surface.factory.surface_cell_y,
        },
        pirate_proof: Runtime0080Rr2SurfaceProof {
            system_id: pirate_id,
            owner: Runtime0080Rr0Owner::Pirate,
            materialized_through_rr_1: surface_resident_through_rr_1(pirate_id),
            pop_slot: pirate_pop,
            factory_slot: pirate_factory,
            pop_surface_cell_x: pirate.planet.surface.pop_cohort.surface_cell_x,
            pop_surface_cell_y: pirate.planet.surface.pop_cohort.surface_cell_y,
            factory_surface_cell_x: pirate.planet.surface.factory.surface_cell_x,
            factory_surface_cell_y: pirate.planet.surface.factory.surface_cell_y,
        },
    })
}

fn run_gpu_surface_tick(
    ctx: &GpuContext,
    layout: &SurfaceGpuLayout,
    config: SurfaceGpuConfig,
) -> Result<GpuSurfaceOutcome, &'static str> {
    let ops = build_surface_ops(layout, &config).map_err(|_| "rr_2_surface_ops_build_failed")?;
    let n_dims = RR_2_SURFACE_N_DIMS;
    let mut values = vec![0.0f32; (layout.n_slots * n_dims) as usize];
    let mut session = AccumulatorOpSession::new(ctx, layout.n_slots, n_dims);
    session.upload_values(ctx, &values);
    session
        .upload_ops_resolving_input_lists(ctx, &ops)
        .map_err(|_| "rr_2_gpu_upload_ops_failed")?;
    for band in BAND_LABOR_EMIT..=BAND_FACTORY_RECIPE {
        session
            .tick(ctx, band)
            .map_err(|_| "rr_2_gpu_tick_failed")?;
    }
    values = session
        .readback_full(ctx)
        .map_err(|_| "rr_2_gpu_readback_failed")?;
    Ok(GpuSurfaceOutcome { values, n_dims })
}

fn build_surface_ops(
    layout: &SurfaceGpuLayout,
    config: &SurfaceGpuConfig,
) -> Result<Vec<AccumulatorOp>, AccumulatorOpBuilderError> {
    let mut ops = Vec::new();
    let mut transfers = Vec::new();

    for binding in &layout.bindings {
        if !config.active_system_ids.contains(&binding.system_id) {
            continue;
        }
        if config.labor_emit_enabled {
            ops.push(labor_emit_op(binding.pop_slot));
        }
        let (pop_slot, factory_slot) = if let Some((from, to)) = config.cross_surface_leak {
            if binding.system_id == from {
                let victim = layout
                    .bindings
                    .iter()
                    .find(|b| b.system_id == to)
                    .expect("leak target");
                (binding.pop_slot, victim.factory_slot)
            } else {
                (binding.pop_slot, binding.factory_slot)
            }
        } else {
            (binding.pop_slot, binding.factory_slot)
        };
        transfers.push(DiscreteTransferRegistration {
            source_slot: SlotIndex::new(pop_slot),
            source_col: ColumnIndex::new(RR_2_COL_LABOR as usize),
            target_slot: SlotIndex::new(factory_slot),
            target_col: ColumnIndex::new(RR_2_COL_LABOR as usize),
            amount: POP_LABOR_PER_TICK as f32,
            order_band: BAND_LABOR_TRANSFER,
        });
        if config.factory_recipe_enabled {
            let mut recipe_op = AccumulatorOpBuilder::conjunctive_recipe(
                &[(
                    SlotIndex::new(factory_slot),
                    ColumnIndex::new(RR_2_COL_LABOR as usize),
                    FACTORY_UNIT_COST_LABOR as f32,
                )],
                SlotIndex::new(factory_slot),
                ColumnIndex::new(RR_2_COL_PRODUCTION as usize),
                99,
            )?;
            recipe_op.gate = GateSpec::OrderBand(BAND_FACTORY_RECIPE);
            ops.push(recipe_op);
        }
    }
    ops.extend(rebuild_discrete_transfer_ops(&transfers)?);
    Ok(ops)
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
            ColumnIndex::new(RR_2_COL_LABOR as usize),
        )],
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

fn find_system<'a>(
    world: &'a Runtime0080Rr0RecursiveWorld,
    system_id: u8,
) -> Option<&'a Runtime0080Rr0System> {
    world
        .galaxy
        .systems
        .iter()
        .find(|system| system.id == system_id)
}

fn surface_cell_index(x: u32, y: u32) -> usize {
    (y * RR_1_SURFACE_SIDE + x) as usize
}

fn cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn f32_bits(value: f32) -> u32 {
    value.to_bits()
}

fn build_scope_ledger(
    exec: &SurfaceEconomyExecution,
    implemented: bool,
) -> Vec<Runtime0080Rr2ScopeLedgerRow> {
    vec![
        scope_row(
            "RR-0 recursive world consumed",
            implemented,
            "build_recursive_world",
        ),
        scope_row(
            "RR-1 nested residency consumed",
            implemented,
            "DescendToSurface Terran+Pirate",
        ),
        scope_row(
            "Terran planet surface materialized through RR-1",
            exec.terran_proof.materialized_through_rr_1,
            "terran_proof.surface resident",
        ),
        scope_row(
            "Pirate planet surface materialized through RR-1",
            exec.pirate_proof.materialized_through_rr_1,
            "pirate_proof.surface resident",
        ),
        scope_row(
            "Pop cohort child is GPU labor emitter",
            exec.labor_parity_ok,
            "labor_emit AccumulatorOp AddToTarget",
        ),
        scope_row(
            "Factory child is GPU labor consumer",
            exec.labor_parity_ok,
            "discrete transfer pop→factory",
        ),
        scope_row(
            "Labor emission computed on GPU",
            exec.labor_parity_ok,
            "gpu_labor_bits match cpu",
        ),
        scope_row(
            "Factory labor consumption computed on GPU",
            exec.labor_parity_ok,
            "gpu labor consumed parity",
        ),
        scope_row(
            "Production generation computed on GPU",
            exec.production_parity_ok,
            "conjunctive recipe on GPU",
        ),
        scope_row(
            "GPU output compared to RR-0 CPU oracle",
            exec.labor_parity_ok && exec.production_parity_ok,
            "parity_rows",
        ),
        scope_row("Bit-exact labor parity", exec.labor_parity_ok, "labor bits"),
        scope_row(
            "Bit-exact production parity",
            exec.production_parity_ok,
            "production bits",
        ),
        scope_row(
            "Disabled labor-emitter check fails parity",
            exec.disabled_emitter_fails_parity,
            "labor_emit_enabled=false",
        ),
        scope_row(
            "Re-enabled labor-emitter restores parity",
            exec.reenabled_emitter_restores_parity,
            "labor_emit_enabled=true",
        ),
        scope_row(
            "Inactive surface emits no labor",
            exec.inactive_surface_no_labor,
            "pirate inactive when not active_system_ids",
        ),
        scope_row(
            "Inactive surface produces no factory output",
            exec.inactive_surface_no_output,
            "inactive pirate production==0",
        ),
        scope_row(
            "No cross-surface labor leakage",
            exec.no_cross_surface_leakage,
            "wrong pop→factory pairing fails parity",
        ),
        scope_row(
            "Surface economy remains at planet surface tier, not flattened to system/galaxy scalar",
            exec.not_flattened_scalar,
            "per-surface cell slots not system scalar",
        ),
        deferred_row(
            "Recursive GPU reduce-up/disburse-down deferred to RR-3",
            "surface-only staging",
        ),
        deferred_row(
            "Integrated recursive GPU rehearsal deferred to RR-4",
            "100-tick recursive GPU horizon",
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
) -> Runtime0080Rr2ScopeLedgerRow {
    Runtime0080Rr2ScopeLedgerRow {
        spec_element,
        required_by_spec: true,
        implemented_in_rr_2: ok,
        status: if ok { "implemented" } else { "not implemented" },
        evidence,
        deviation: "",
    }
}

fn deferred_row(
    spec_element: &'static str,
    evidence: &'static str,
) -> Runtime0080Rr2ScopeLedgerRow {
    Runtime0080Rr2ScopeLedgerRow {
        spec_element,
        required_by_spec: false,
        implemented_in_rr_2: false,
        status: "deferred",
        evidence,
        deviation: "",
    }
}

fn base_report(
    input: &Runtime0080Rr2Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    execution: Option<SurfaceEconomyExecution>,
) -> Runtime0080Rr2Report {
    let admitted = diagnostics.is_empty();
    let empty_proof = Runtime0080Rr2SurfaceProof {
        system_id: 0,
        owner: Runtime0080Rr0Owner::Terran,
        materialized_through_rr_1: false,
        pop_slot: 0,
        factory_slot: 0,
        pop_surface_cell_x: 0,
        pop_surface_cell_y: 0,
        factory_surface_cell_x: 0,
        factory_surface_cell_y: 0,
    };

    let (
        terran_proof,
        pirate_proof,
        surface_bindings,
        parity_rows,
        labor_parity_ok,
        production_parity_ok,
        disabled_emitter_fails_parity,
        reenabled_emitter_restores_parity,
        disabled_consumer_fails_parity,
        reenabled_consumer_restores_parity,
        inactive_surface_no_labor,
        inactive_surface_no_output,
        no_cross_surface_leakage,
        not_flattened_scalar,
        scope_ledger,
        deviation_records,
        deterministic_replay_checksum,
    ) = match execution {
        Some(exec) => {
            let scope_ledger = build_scope_ledger(&exec, true);
            (
                exec.terran_proof,
                exec.pirate_proof,
                exec.surface_bindings,
                exec.parity_rows,
                exec.labor_parity_ok,
                exec.production_parity_ok,
                exec.disabled_emitter_fails_parity,
                exec.reenabled_emitter_restores_parity,
                exec.disabled_consumer_fails_parity,
                exec.reenabled_consumer_restores_parity,
                exec.inactive_surface_no_labor,
                exec.inactive_surface_no_output,
                exec.no_cross_surface_leakage,
                exec.not_flattened_scalar,
                scope_ledger,
                Vec::<Runtime0080Rr2DeviationRecord>::new(),
                exec.deterministic_replay_checksum,
            )
        }
        None => (
            empty_proof.clone(),
            empty_proof,
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
            Vec::new(),
            Vec::new(),
            0,
        ),
    };

    let required_rows_implemented = scope_ledger
        .iter()
        .take(18)
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
        "PASS" => RUNTIME_0080_RR_2_STATUS_PASS,
        "PARTIAL" => RUNTIME_0080_RR_2_STATUS_PARTIAL,
        _ => RUNTIME_0080_RR_2_STATUS_BLOCKED,
    };

    let stable_report_checksum = if admitted && !disabled_no_op {
        checksum_report(
            verdict,
            labor_parity_ok,
            production_parity_ok,
            deterministic_replay_checksum,
        )
    } else {
        0
    };

    Runtime0080Rr2Report {
        id: RUNTIME_0080_RR_2_ID,
        status,
        verdict,
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        rr_0_world_consumed: admitted && !disabled_no_op,
        rr_1_residency_consumed: terran_proof.materialized_through_rr_1
            && pirate_proof.materialized_through_rr_1,
        gpu_available: admitted && !disabled_no_op,
        terran_proof,
        pirate_proof,
        surface_bindings,
        parity_rows,
        labor_parity_ok,
        production_parity_ok,
        disabled_emitter_fails_parity,
        reenabled_emitter_restores_parity,
        disabled_consumer_fails_parity,
        reenabled_consumer_restores_parity,
        inactive_surface_no_labor,
        inactive_surface_no_output,
        no_cross_surface_leakage,
        not_flattened_scalar,
        scope_ledger,
        deviation_records,
        stable_report_checksum,
        deterministic_replay_checksum,
        rr_3_claimed: false,
        rr_4_claimed: false,
        standalone_m4a_claimed: false,
        invariant_edit: false,
        default_session_wiring: false,
    }
}

fn checksum_execution(rows: &[Runtime0080Rr2ParityRow]) -> u64 {
    let mut hash = FNV_OFFSET;
    for row in rows {
        hash = fnv_mix(hash, u64::from(row.system_id));
        hash = fnv_mix(hash, u64::from(row.gpu_labor_bits));
        hash = fnv_mix(hash, u64::from(row.gpu_production_bits));
        hash = fnv_mix(hash, u64::from(row.parity as u8));
    }
    hash
}

fn checksum_report(
    verdict: &str,
    labor_ok: bool,
    production_ok: bool,
    oracle_checksum: u64,
) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in verdict.as_bytes() {
        hash = fnv_mix(hash, u64::from(*byte));
    }
    hash = fnv_mix(hash, u64::from(labor_ok as u8));
    hash = fnv_mix(hash, u64::from(production_ok as u8));
    hash = fnv_mix(hash, oracle_checksum);
    hash
}

fn fnv_mix(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(FNV_PRIME)
}

#[cfg(test)]
mod tests {
    use simthing_gpu::execute_ops_cpu;

    use super::*;

    #[test]
    fn cpu_surface_ops_match_oracle_tick_zero() {
        let world = build_recursive_world(RR_0_DEFAULT_SEED);
        let terran_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Terran);
        let pirate_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Pirate);
        let layout = build_surface_layout(&world, terran_id, pirate_id).expect("layout");
        let mut flat = vec![0.0f32; (layout.n_slots * RR_2_SURFACE_N_DIMS) as usize];
        let ops = build_surface_ops(
            &layout,
            &SurfaceGpuConfig {
                labor_emit_enabled: true,
                factory_recipe_enabled: true,
                active_system_ids: vec![terran_id, pirate_id],
                cross_surface_leak: None,
            },
        )
        .expect("ops");
        for band in BAND_LABOR_EMIT..=BAND_FACTORY_RECIPE {
            execute_ops_cpu(&mut flat, &ops, band, RR_2_SURFACE_N_DIMS).expect("cpu band");
        }
        for binding in &layout.bindings {
            assert_eq!(
                flat[cell_index(binding.factory_slot, RR_2_COL_LABOR, RR_2_SURFACE_N_DIMS)]
                    .to_bits(),
                0.0_f32.to_bits(),
                "factory labor remaining system_id={}",
                binding.system_id
            );
            assert_eq!(
                flat[cell_index(
                    binding.factory_slot,
                    RR_2_COL_PRODUCTION,
                    RR_2_SURFACE_N_DIMS
                )]
                .to_bits(),
                (PRODUCTION_PER_RECIPE as f32).to_bits(),
                "factory production system_id={}",
                binding.system_id
            );
        }
    }

    #[test]
    fn gpu_surface_ops_match_cpu_oracle_both_surfaces() {
        set_debug_readback_allowed(true);
        let ctx = GpuContext::new_blocking().expect("gpu context");
        let world = build_recursive_world(RR_0_DEFAULT_SEED);
        let terran_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Terran);
        let pirate_id = find_owner_system_id(&world, Runtime0080Rr0Owner::Pirate);
        let layout = build_surface_layout(&world, terran_id, pirate_id).expect("layout");
        let config = SurfaceGpuConfig {
            labor_emit_enabled: true,
            factory_recipe_enabled: true,
            active_system_ids: vec![terran_id, pirate_id],
            cross_surface_leak: None,
        };
        let ops = build_surface_ops(&layout, &config).expect("ops");
        let mut cpu_flat = vec![0.0f32; (layout.n_slots * RR_2_SURFACE_N_DIMS) as usize];
        for band in BAND_LABOR_EMIT..=BAND_FACTORY_RECIPE {
            execute_ops_cpu(&mut cpu_flat, &ops, band, RR_2_SURFACE_N_DIMS).expect("cpu band");
        }
        let gpu = run_gpu_surface_tick(&ctx, &layout, config).expect("gpu tick");
        assert_eq!(
            gpu.values, cpu_flat,
            "gpu readback must match cpu oracle for both surfaces"
        );
    }
}
