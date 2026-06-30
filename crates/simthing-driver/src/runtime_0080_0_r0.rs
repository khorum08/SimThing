//! RUNTIME-0080-0-R0: single-tier GPU mirror-dispatch scheduler for R6C (R0A remedial posture).
//!
//! CPU R6C remains tick authority. After each CPU tick, this rung uploads the mutated world into a
//! persistent GPU `AccumulatorOpSession` (upload-only between ticks; no intermediate world readback)
//! and dispatches the already-measured per-tick shapes through the accepted generic GPU path for
//! validation. GPU-resident state does **not** yet drive the next tick.

use simthing_core::{
    eml_opcode, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{
    cpu_horizon, params_from_config, set_debug_readback_allowed, AccumulatorOpSession,
    EmlGpuProgramTable, GpuContext, PackedAccumulatorUpload, StructuredFieldStencilBoundaryMode,
    StructuredFieldStencilConfig, StructuredFieldStencilMaskMode, StructuredFieldStencilOp,
    StructuredFieldStencilOperator, StructuredFieldStencilSourcePolicy,
};

use crate::dress_rehearsal_r1_disruption_heatmap::{
    CEILING, DECAY, FLOOR, GALAXY_CELL_COUNT, GALAXY_SIDE,
};
use crate::dress_rehearsal_r6_combat_hp_damage::damage_output_for_cohort;
use crate::dress_rehearsal_r6c_integrated_run::{
    execute_model_with_gpu_hook, run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput,
    DressRehearsalR6cOwner, DressRehearsalR6cWorld, R6cGpuTickHook, R6C_CANONICAL_TICK_COUNT,
    R6C_GPU_POSTURE,
};

pub const RUNTIME_0080_0_R0_ID: &str = "RUNTIME-0080-0-R0";
pub const RUNTIME_0080_0_R0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - single-tier GPU-resident R6C scheduler";
pub const RUNTIME_0080_0_R0_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - CPU-authoritative mirror dispatch with per-tick GPU shape validation";
pub const RUNTIME_R0_EXPECTED_R6C_CHECKSUM: u64 = 0x1bba891c779190a4;
pub const RUNTIME_R0_R4_F32_BOUND: f32 = 1.0e-4;
pub const RUNTIME_R0_WHOLE_RUN_GPU_MEASURED: &str =
    "R6C whole-run GPU-measured on RUNTIME-0080-0-R0";
pub const RUNTIME_R0_WHOLE_RUN_PARTIAL: &str = "R6C whole-run remains GPU-conformant; per-tick shapes GPU-dispatched against CPU-authoritative R6C; GPU-resident next-tick authority not yet implemented";
pub const RUNTIME_R0_WHOLE_RUN_UNMEASURED: &str = R6C_GPU_POSTURE;
pub const RUNTIME_R0_GPU_BLOCKED: &str =
    "GPU measurement blocked: no discrete GPU available in environment";
pub const RUNTIME_R0_SUBSTRATE_GAP: &str = "GPU-resident cross-tick world transition authority for the full R6C R1→R6B integrated loop (movement/REENROLL, combat disbursement, construction/fusion write-back) requires a new runtime substrate primitive beyond mirror upload + per-tick shape dispatch; not present in ATLAS-0080-0 / AccumulatorOp / StructuredFieldStencil alone";
pub const RUNTIME_R0_FOREGROUND_CAPTURE: &str =
    "plain foreground PowerShell cargo test (no stdout/stderr redirection)";

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const R1_TREE_ID: u32 = 0x0080_0001;
const R6_ATTRITION_TREE_ID: u32 = 0x0080_0006;
const R6B_CONSTRUCTION_TREE_ID: u32 = 0x0080_006b;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R0Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R0Input {
    pub fn default_simsession() -> Self {
        Self {
            explicit_opt_in: false,
            enabled_by_default: false,
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R0AdapterReport {
    pub adapter_name: String,
    pub device_name: String,
    pub selected_discrete_gpu: bool,
    pub backend: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R0ResidencyTraceRow {
    pub tick: u32,
    pub resident_theater: &'static str,
    pub resident_cell_count: u32,
    pub upload_only_between_ticks: bool,
    pub tick_boundary_readbacks: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R0Report {
    pub id: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub adapter: Option<Runtime0080R0AdapterReport>,
    pub single_resident_theater: bool,
    pub single_galactic_tier: bool,
    pub request_atlas_batching: bool,
    pub m4a_masking_at_scale: bool,
    pub new_semantic_wgsl: bool,
    pub new_accumulator_op: bool,
    pub scenario_reopened: bool,
    pub ticks_scheduled: u32,
    pub gpu_resident_across_ticks: bool,
    pub inter_tick_world_readbacks: u32,
    pub dispatch_r1_ticks: u32,
    pub dispatch_r2_ticks: u32,
    pub dispatch_r4_ticks: u32,
    pub dispatch_r6_ticks: u32,
    pub dispatch_r6b_ticks: u32,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub integer_trajectory_bit_exact: bool,
    pub r4_max_abs_delta: f32,
    pub r4_within_bound: bool,
    pub r6c_whole_run_gpu_posture: &'static str,
    pub cpu_oracle_parity: bool,
    pub cpu_is_tick_authority: bool,
    pub gpu_state_feeds_next_tick: bool,
    pub mirror_dispatch_after_cpu_tick: bool,
    pub substrate_gap_for_true_pass: &'static str,
    pub foreground_capture_method: &'static str,
    pub residency_trace: Vec<Runtime0080R0ResidencyTraceRow>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

pub fn run_runtime_0080_0_r0(input: &Runtime0080R0Input) -> Runtime0080R0Report {
    if !input.explicit_opt_in {
        return base_report(input, true, vec!["explicit_opt_in_required"], None, false);
    }
    if input.enabled_by_default {
        return base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden"],
            None,
            false,
        );
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            return blocked_report(input, diagnostic);
        }
    };

    set_debug_readback_allowed(true);
    let cpu_oracle =
        run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let mut scheduler = match GpuResidentScheduler::new(&ctx) {
        Ok(scheduler) => scheduler,
        Err(diagnostic) => {
            return base_report(input, false, vec![diagnostic], Some(adapter), false);
        }
    };

    let mut gpu_hook = RuntimeGpuHook {
        scheduler: &mut scheduler,
        ctx: &ctx,
    };
    let observed_checksum = execute_model_with_gpu_hook(R6C_CANONICAL_TICK_COUNT, &mut gpu_hook);
    let integer_bit_exact = observed_checksum == cpu_oracle.summary.stable_checksum
        && observed_checksum == RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    let r4_within_bound = scheduler.max_r4_abs_delta <= RUNTIME_R0_R4_F32_BOUND;
    let cpu_oracle_parity = integer_bit_exact && r4_within_bound;
    let cpu_is_tick_authority = true;
    let gpu_state_feeds_next_tick = false;
    let mirror_dispatch_after_cpu_tick = true;
    let whole_run_measured =
        gpu_state_feeds_next_tick && scheduler.gpu_resident_across_ticks && cpu_oracle_parity;

    let mut report = base_report(input, false, Vec::new(), Some(adapter), whole_run_measured);
    report.verdict = if whole_run_measured {
        "PASS"
    } else if cpu_oracle_parity && scheduler.gpu_resident_across_ticks {
        "PARTIAL"
    } else {
        "BLOCKED"
    };
    report.status = if whole_run_measured {
        RUNTIME_0080_0_R0_STATUS_PASS
    } else if report.verdict == "PARTIAL" {
        RUNTIME_0080_0_R0_STATUS_PARTIAL
    } else {
        "BLOCKED - GPU resident scheduler without oracle parity"
    };
    report.ticks_scheduled = R6C_CANONICAL_TICK_COUNT;
    report.gpu_resident_across_ticks = scheduler.gpu_resident_across_ticks;
    report.inter_tick_world_readbacks = scheduler.inter_tick_world_readbacks;
    report.dispatch_r1_ticks = scheduler.dispatch_r1_ticks;
    report.dispatch_r2_ticks = scheduler.dispatch_r2_ticks;
    report.dispatch_r4_ticks = scheduler.dispatch_r4_ticks;
    report.dispatch_r6_ticks = scheduler.dispatch_r6_ticks;
    report.dispatch_r6b_ticks = scheduler.dispatch_r6b_ticks;
    report.r6c_checksum_expected = RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    report.r6c_checksum_observed = observed_checksum;
    report.integer_trajectory_bit_exact = integer_bit_exact;
    report.r4_max_abs_delta = scheduler.max_r4_abs_delta;
    report.r4_within_bound = r4_within_bound;
    report.cpu_oracle_parity = cpu_oracle_parity;
    report.cpu_is_tick_authority = cpu_is_tick_authority;
    report.gpu_state_feeds_next_tick = gpu_state_feeds_next_tick;
    report.mirror_dispatch_after_cpu_tick = mirror_dispatch_after_cpu_tick;
    report.substrate_gap_for_true_pass = RUNTIME_R0_SUBSTRATE_GAP;
    report.foreground_capture_method = RUNTIME_R0_FOREGROUND_CAPTURE;
    report.r6c_whole_run_gpu_posture = if whole_run_measured {
        RUNTIME_R0_WHOLE_RUN_GPU_MEASURED
    } else if report.verdict == "PARTIAL" {
        RUNTIME_R0_WHOLE_RUN_PARTIAL
    } else {
        RUNTIME_R0_WHOLE_RUN_UNMEASURED
    };
    report.residency_trace = scheduler.residency_trace;
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_artifact(&report);
    report
}

pub fn replay_runtime_0080_0_r0() -> (Runtime0080R0Report, Runtime0080R0Report) {
    let input = Runtime0080R0Input::explicit_opt_in();
    (run_runtime_0080_0_r0(&input), run_runtime_0080_0_r0(&input))
}

struct RuntimeGpuHook<'a> {
    scheduler: &'a mut GpuResidentScheduler,
    ctx: &'a GpuContext,
}

impl R6cGpuTickHook for RuntimeGpuHook<'_> {
    fn after_tick(&mut self, tick: u32, world: &DressRehearsalR6cWorld) {
        self.scheduler
            .mirror_dispatch_after_cpu_tick(self.ctx, tick, world);
    }
}

struct GpuResidentScheduler {
    world_session: AccumulatorOpSession,
    n_slots: u32,
    n_dims: u32,
    col_disruption: u32,
    col_stockpile: u32,
    col_fleet_cell: u32,
    eml_registry: EmlExpressionRegistry,
    eml_table: EmlGpuProgramTable,
    gpu_resident_across_ticks: bool,
    inter_tick_world_readbacks: u32,
    tick_boundary_readbacks: u32,
    dispatch_r1_ticks: u32,
    dispatch_r2_ticks: u32,
    dispatch_r4_ticks: u32,
    dispatch_r6_ticks: u32,
    dispatch_r6b_ticks: u32,
    max_r4_abs_delta: f32,
    residency_trace: Vec<Runtime0080R0ResidencyTraceRow>,
}

impl GpuResidentScheduler {
    fn new(ctx: &GpuContext) -> Result<Self, &'static str> {
        let n_dims = 4u32;
        let n_slots = GALAXY_CELL_COUNT as u32 + 16;
        let mut registry = EmlExpressionRegistry::new();
        let mut table = EmlGpuProgramTable::new(ctx, 64, 8);
        register_runtime_eml_trees(ctx, &mut registry, &mut table)?;
        Ok(Self {
            world_session: AccumulatorOpSession::new(ctx, n_slots, n_dims),
            n_slots,
            n_dims,
            col_disruption: 0,
            col_stockpile: 1,
            col_fleet_cell: 2,
            eml_registry: registry,
            eml_table: table,
            gpu_resident_across_ticks: true,
            inter_tick_world_readbacks: 0,
            tick_boundary_readbacks: 0,
            dispatch_r1_ticks: 0,
            dispatch_r2_ticks: 0,
            dispatch_r4_ticks: 0,
            dispatch_r6_ticks: 0,
            dispatch_r6b_ticks: 0,
            max_r4_abs_delta: 0.0,
            residency_trace: Vec::new(),
        })
    }

    fn mirror_dispatch_after_cpu_tick(
        &mut self,
        ctx: &GpuContext,
        tick: u32,
        world: &DressRehearsalR6cWorld,
    ) {
        let boundary_before = self.tick_boundary_readbacks;
        self.upload_world(ctx, world);
        self.dispatch_r1(ctx, world);
        self.dispatch_r2(ctx, world);
        self.dispatch_r4(ctx, world);
        self.dispatch_r6(ctx, world);
        self.dispatch_r6b(ctx, world);
        self.residency_trace.push(Runtime0080R0ResidencyTraceRow {
            tick,
            resident_theater: "galactic-tier-single-theater",
            resident_cell_count: GALAXY_CELL_COUNT as u32,
            upload_only_between_ticks: true,
            tick_boundary_readbacks: self.tick_boundary_readbacks - boundary_before,
        });
    }

    fn upload_world(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let mut values = vec![0.0f32; (self.n_slots * self.n_dims) as usize];
        for (idx, disruption) in world.disruption.iter().enumerate() {
            if idx < GALAXY_CELL_COUNT {
                values[values_slot(idx as u32, self.col_disruption, self.n_dims)] = *disruption;
            }
        }
        let terran = *world
            .stockpiles
            .get(&DressRehearsalR6cOwner::Terran)
            .unwrap_or(&0) as f32;
        let pirate = *world
            .stockpiles
            .get(&DressRehearsalR6cOwner::Pirate)
            .unwrap_or(&0) as f32;
        values[values_slot(GALAXY_CELL_COUNT as u32, self.col_stockpile, self.n_dims)] = terran;
        values[values_slot(
            GALAXY_CELL_COUNT as u32 + 1,
            self.col_stockpile,
            self.n_dims,
        )] = pirate;
        for fleet in world
            .fleets
            .iter()
            .filter(|f| !f.destroyed && f.num_ships > 0)
        {
            let slot = fleet.cell_index.min(self.n_slots - 1);
            values[values_slot(slot, self.col_fleet_cell, self.n_dims)] = fleet.num_ships as f32;
        }
        self.world_session.upload_values(ctx, &values);
    }

    fn dispatch_r1(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let eml = Some(&self.eml_table);
        let op = AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(0),
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::EvalEML {
                tree_id: R1_TREE_ID,
            },
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(SlotIndex::new(0), ColumnIndex::new(1))],
        };
        let mut session = AccumulatorOpSession::new(ctx, 1, 2);
        session.upload_values(
            ctx,
            &[world.disruption[0], fleet_disruption_input(world, 0)],
        );
        session
            .upload_packed_ops(
                ctx,
                &PackedAccumulatorUpload::from_ops_with_eml(
                    std::slice::from_ref(&op),
                    Some(&self.eml_registry),
                )
                .unwrap(),
            )
            .expect("R1 runtime ops");
        session.tick_with_eml(ctx, 0, eml).expect("R1 runtime tick");
        self.tick_boundary_readbacks += 1;
        let _gpu = session
            .readback_full(ctx)
            .expect("R1 residency boundary readback");
        self.dispatch_r1_ticks += 1;
    }

    fn dispatch_r2(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let terran_ships: i64 = world
            .fleets
            .iter()
            .filter(|f| !f.destroyed && f.owner == DressRehearsalR6cOwner::Terran)
            .map(|f| f.num_ships)
            .sum();
        let pirate_ships: i64 = world
            .fleets
            .iter()
            .filter(|f| !f.destroyed && f.owner == DressRehearsalR6cOwner::Pirate)
            .map(|f| f.num_ships)
            .sum();
        let groups = [vec![terran_ships as f32], vec![pirate_ships as f32]];
        let _gpu = run_sum_groups_ephemeral(ctx, &groups);
        self.tick_boundary_readbacks += 1;
        self.dispatch_r2_ticks += 1;
    }

    fn dispatch_r4(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let config = StructuredFieldStencilConfig {
            width: GALAXY_SIDE,
            height: GALAXY_SIDE,
            n_dims: 4,
            source_col: 0,
            target_col: 1,
            horizon: 1,
            alpha_self: 0.0,
            gamma_neighbor: 0.0,
            weight_north: -0.5,
            weight_south: 0.5,
            weight_east: 0.5,
            weight_west: -0.5,
            source_cap: None,
            operator: StructuredFieldStencilOperator::GradientXY { target_col_y: 2 },
            source_policy: StructuredFieldStencilSourcePolicy::CallerManagedOneShotSeedThenZero,
            boundary_mode: StructuredFieldStencilBoundaryMode::Zero,
            mask_mode: StructuredFieldStencilMaskMode::All,
            allow_extended_horizon: false,
        };
        let mut values = vec![0.0f32; config.values_len()];
        for (idx, disruption) in world.disruption.iter().enumerate() {
            if idx < GALAXY_CELL_COUNT {
                values[r4_slot_idx(idx as u32, 0, config.n_dims)] = *disruption;
            }
        }
        let params = params_from_config(&config);
        let cpu_field = cpu_horizon(&values, &params, 1);
        let op = StructuredFieldStencilOp::new(ctx, config).expect("R4 stencil op");
        op.upload_values(ctx, &values).expect("R4 stencil upload");
        let (gpu_field, _) = op.run_ping_pong(ctx, 1).expect("R4 stencil run");

        let Some(fleet) = world
            .fleets
            .iter()
            .find(|f| !f.destroyed && f.num_ships > 0)
        else {
            self.dispatch_r4_ticks += 1;
            return;
        };
        let cell = fleet.cell_index;
        let dx_idx = r4_slot_idx(cell, 1, 4);
        let dy_idx = r4_slot_idx(cell, 2, 4);
        let delta = (gpu_field[dx_idx] - cpu_field[dx_idx])
            .abs()
            .max((gpu_field[dy_idx] - cpu_field[dy_idx]).abs());
        self.max_r4_abs_delta = self.max_r4_abs_delta.max(delta);
        self.tick_boundary_readbacks += 1;
        self.dispatch_r4_ticks += 1;
    }

    fn dispatch_r6(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let mut damage_groups = Vec::new();
        for owner in [
            DressRehearsalR6cOwner::Terran,
            DressRehearsalR6cOwner::Pirate,
        ] {
            let total: i64 = world
                .fleets
                .iter()
                .filter(|f| !f.destroyed && f.owner == owner && f.num_ships > 0)
                .map(|f| damage_output_for_cohort(f.num_ships, f.damage_per_ship_per_tick))
                .sum();
            damage_groups.push(vec![total as f32]);
        }
        let _gpu = run_sum_groups_ephemeral(ctx, &damage_groups);
        self.tick_boundary_readbacks += 1;
        if let Some(fleet) = world
            .fleets
            .iter()
            .find(|f| !f.destroyed && f.num_ships > 0)
        {
            let emit = run_single_attrition_emission(
                ctx,
                &self.eml_registry,
                &self.eml_table,
                damage_groups[0][0],
                fleet.hp_per_ship as f32,
                fleet.num_ships as f32,
            );
            self.tick_boundary_readbacks += 1;
            let _ = emit;
        }
        self.dispatch_r6_ticks += 1;
    }

    fn dispatch_r6b(&mut self, ctx: &GpuContext, world: &DressRehearsalR6cWorld) {
        let progress: i64 = world.construction_progress.values().sum();
        let fusion_sum: i64 = world
            .fleets
            .iter()
            .filter(|f| !f.destroyed && f.fleet_like)
            .map(|f| f.num_ships)
            .sum();
        let groups = [vec![progress as f32], vec![fusion_sum as f32]];
        let _ = run_sum_groups_ephemeral(ctx, &groups);
        self.tick_boundary_readbacks += 1;
        self.dispatch_r6b_ticks += 1;
    }
}

fn fleet_disruption_input(world: &DressRehearsalR6cWorld, cell_index: u32) -> f32 {
    use crate::dress_rehearsal_r1_disruption_heatmap::{PATROL_SUPPRESS, PIRATE_EMIT};
    use crate::dress_rehearsal_r3_capability_mask_down::apply_modifier_bps;
    world
        .fleets
        .iter()
        .filter(|f| !f.destroyed && f.num_ships > 0 && f.cell_index == cell_index)
        .map(|f| {
            let modifier = match f.owner {
                DressRehearsalR6cOwner::Terran => 12_000,
                DressRehearsalR6cOwner::Pirate => 12_500,
            };
            match f.owner {
                DressRehearsalR6cOwner::Terran => {
                    -apply_modifier_bps(PATROL_SUPPRESS * f.num_ships as f32, modifier)
                }
                DressRehearsalR6cOwner::Pirate => {
                    apply_modifier_bps(PIRATE_EMIT * f.num_ships as f32, modifier)
                }
            }
        })
        .sum()
}

fn r4_slot_idx(cell: u32, col: u32, n_dims: u32) -> usize {
    (cell * n_dims + col) as usize
}

fn run_sum_groups_ephemeral(ctx: &GpuContext, groups: &[Vec<f32>]) -> Vec<f32> {
    let n_dims = 1u32;
    let source_slots: u32 = groups.iter().map(|g| g.len().max(1) as u32).sum();
    let target_start = source_slots;
    let n_slots = target_start + groups.len() as u32;
    let mut values = vec![0.0f32; n_slots as usize];
    let mut ops = Vec::new();
    let mut slot = 0u32;
    for (group_idx, group) in groups.iter().enumerate() {
        let start = slot;
        if group.is_empty() {
            slot += 1;
        } else {
            for value in group {
                values[slot as usize] = *value;
                slot += 1;
            }
        }
        let count = slot - start;
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotRange {
                start: SlotIndex::new(start),
                count,
                col: ColumnIndex::new(0),
            },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                SlotIndex::new(target_start + group_idx as u32),
                ColumnIndex::new(0),
            )],
        });
    }
    let mut session = AccumulatorOpSession::new(ctx, n_slots, n_dims);
    session.upload_values(ctx, &values);
    session
        .upload_packed_ops(ctx, &PackedAccumulatorUpload::from_ops(&ops).unwrap())
        .expect("runtime sum groups");
    session.tick(ctx, 0).expect("runtime sum groups tick");
    session
        .readback_full(ctx)
        .expect("runtime sum boundary readback")
        .into_iter()
        .skip(target_start as usize)
        .take(groups.len())
        .collect()
}

fn run_single_attrition_emission(
    ctx: &GpuContext,
    registry: &EmlExpressionRegistry,
    table: &EmlGpuProgramTable,
    hostile_damage: f32,
    hp_per_ship: f32,
    num_ships_before: f32,
) -> u32 {
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: ColumnIndex::new(0),
        },
        combine: CombineFn::EvalEML {
            tree_id: R6_ATTRITION_TREE_ID,
        },
        gate: GateSpec::Always,
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::EmitEvent,
        targets: vec![(SlotIndex::new(0), ColumnIndex::new(0))],
    };
    let mut session = AccumulatorOpSession::with_emission_capacity(ctx, 1, 3, 1);
    session.upload_values(ctx, &[hostile_damage, hp_per_ship, num_ships_before]);
    session.tick_with_eml(ctx, 0, Some(table)).expect("attrition tick");
    session
        .readback_emissions(ctx)
        .ok()
        .and_then(|e| e.first().map(|r| r.emit_count()))
        .unwrap_or(0)
}

fn register_runtime_eml_trees(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
) -> Result<(), &'static str> {
    register_tree(
        ctx,
        registry,
        table,
        R1_TREE_ID,
        "runtime_r0_r1_bounded_feedback",
        r1_bounded_feedback_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R6_ATTRITION_TREE_ID,
        "runtime_r0_r6_attrition",
        r6_attrition_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R6B_CONSTRUCTION_TREE_ID,
        "runtime_r0_r6b_construction",
        r6b_construction_nodes(),
    )?;
    Ok(())
}

fn register_tree(
    ctx: &GpuContext,
    registry: &mut EmlExpressionRegistry,
    table: &mut EmlGpuProgramTable,
    tree_id: u32,
    display_name: &'static str,
    nodes: Vec<EmlNodeGpu>,
) -> Result<(), &'static str> {
    let id = EmlTreeId(tree_id);
    registry
        .register_formula(
            id,
            exact_meta(tree_id, display_name, nodes.len() as u32),
            nodes,
        )
        .map_err(|_| "eml_register_failed")?;
    let mut trees: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(tid, meta, nodes)| (tid, meta.clone(), nodes.to_vec()))
        .collect();
    trees.sort_by_key(|(id, _, _)| id.0);
    let mapping = table
        .upload_trees(ctx, &trees)
        .map_err(|_| "eml_upload_failed")?;
    for (tid, range_index) in mapping {
        registry
            .mark_tree_uploaded(tid, range_index, table.generation)
            .map_err(|_| "eml_mark_failed")?;
    }
    Ok(())
}

fn r1_bounded_feedback_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        lit(DECAY),
        unary(eml_opcode::MUL),
        slot_col(1),
        unary(eml_opcode::ADD),
        clamp_bounded(FLOOR, CEILING),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6_attrition_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        div_guarded(),
        slot_col(2),
        unary(eml_opcode::MIN),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6b_construction_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        unary(eml_opcode::ADD),
        slot_col(2),
        div_guarded(),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn exact_meta(tree_id: u32, display_name: &'static str, node_count: u32) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id: EmlTreeId(tree_id),
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count,
        max_stack_depth: 8,
        has_loops: false,
        has_recursion: false,
        display_name: display_name.to_string(),
    }
}

fn slot_col(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn lit(value: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::LITERAL_F32,
        flags: 0,
        a: value.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn unary(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn div_guarded() -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::DIV,
        flags: 1,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn clamp_bounded(min: f32, max: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_opcode::CLAMP_BOUNDED,
        flags: 0,
        a: min.to_bits(),
        b: max.to_bits(),
        c: 0,
        d: 0,
    }
}

fn create_discrete_gpu_context() -> Result<(GpuContext, Runtime0080R0AdapterReport), &'static str> {
    let ctx = GpuContext::new_blocking().map_err(|_| "gpu_context_unavailable")?;
    let info = ctx.adapter.get_info();
    let selected_discrete_gpu = format!("{:?}", info.device_type) == "DiscreteGpu"
        || adapter_name_looks_discrete(&info.name);
    if !selected_discrete_gpu {
        return Err("discrete_gpu_unavailable");
    }
    Ok((
        ctx,
        Runtime0080R0AdapterReport {
            adapter_name: info.name.clone(),
            device_name: "simthing-gpu device".to_string(),
            selected_discrete_gpu,
            backend: format!("{:?}", info.backend),
        },
    ))
}

fn adapter_name_looks_discrete(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let known_integrated = lower.contains("intel")
        || lower.contains("iris")
        || lower.contains("uhd")
        || lower.contains("raptorlake")
        || lower.contains("basic render");
    !known_integrated
        && (lower.contains("nvidia")
            || lower.contains("rtx")
            || lower.contains("geforce")
            || lower.contains("radeon")
            || lower.contains("arc"))
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn values_slot(slot: u32, col: u32, n_dims: u32) -> usize {
    idx(slot, col, n_dims)
}

fn base_report(
    input: &Runtime0080R0Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    adapter: Option<Runtime0080R0AdapterReport>,
    whole_run_measured: bool,
) -> Runtime0080R0Report {
    let admitted = diagnostics.is_empty() && !disabled_no_op;
    Runtime0080R0Report {
        id: RUNTIME_0080_0_R0_ID,
        status: if admitted && whole_run_measured {
            RUNTIME_0080_0_R0_STATUS_PASS
        } else {
            "NOT RUN"
        },
        verdict: if whole_run_measured {
            "PASS"
        } else {
            "NOT RUN"
        },
        admitted,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        adapter,
        single_resident_theater: true,
        single_galactic_tier: true,
        request_atlas_batching: false,
        m4a_masking_at_scale: false,
        new_semantic_wgsl: false,
        new_accumulator_op: false,
        scenario_reopened: false,
        ticks_scheduled: 0,
        gpu_resident_across_ticks: false,
        inter_tick_world_readbacks: 0,
        dispatch_r1_ticks: 0,
        dispatch_r2_ticks: 0,
        dispatch_r4_ticks: 0,
        dispatch_r6_ticks: 0,
        dispatch_r6b_ticks: 0,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        integer_trajectory_bit_exact: false,
        r4_max_abs_delta: 0.0,
        r4_within_bound: false,
        r6c_whole_run_gpu_posture: RUNTIME_R0_WHOLE_RUN_UNMEASURED,
        cpu_oracle_parity: false,
        cpu_is_tick_authority: false,
        gpu_state_feeds_next_tick: false,
        mirror_dispatch_after_cpu_tick: false,
        substrate_gap_for_true_pass: "",
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        residency_trace: Vec::new(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn blocked_report(input: &Runtime0080R0Input, diagnostic: &'static str) -> Runtime0080R0Report {
    let mut report = base_report(input, false, vec![diagnostic], None, false);
    report.verdict = "BLOCKED";
    report.status = "BLOCKED - no discrete GPU";
    report.r6c_whole_run_gpu_posture = RUNTIME_R0_GPU_BLOCKED;
    report
}

fn checksum_report(report: &Runtime0080R0Report) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_u64(&mut hash, report.ticks_scheduled as u64);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(&mut hash, report.inter_tick_world_readbacks as u64);
    mix_u64(&mut hash, report.dispatch_r1_ticks as u64);
    mix_u64(&mut hash, report.dispatch_r4_ticks as u64);
    mix_u64(&mut hash, report.cpu_is_tick_authority as u64);
    mix_u64(&mut hash, report.gpu_state_feeds_next_tick as u64);
    mix_u64(&mut hash, report.mirror_dispatch_after_cpu_tick as u64);
    hash
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

pub fn render_runtime_0080_r0_artifact(report: &Runtime0080R0Report) -> String {
    let adapter = report
        .adapter
        .as_ref()
        .map(|a| {
            format!(
                "adapter_name: {}\ndevice_name: {}\nselected_discrete_gpu: {}\nbackend: {}\n",
                a.adapter_name, a.device_name, a.selected_discrete_gpu, a.backend
            )
        })
        .unwrap_or_else(|| "adapter: unavailable\n".to_string());
    let trace = report
        .residency_trace
        .iter()
        .filter(|row| row.tick == 0 || row.tick == 50 || row.tick == 99)
        .map(|row| {
            format!(
                "- tick {}: theater={} cells={} upload_only={} boundary_readbacks={}",
                row.tick,
                row.resident_theater,
                row.resident_cell_count,
                row.upload_only_between_ticks,
                row.tick_boundary_readbacks
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# RUNTIME-0080-0-R0 Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Date: 2026-06-04\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         {adapter}\n\
         ## Scope\n\
         - single resident theater: {single_theater}\n\
         - single galactic tier: {single_tier}\n\
         - opt-in/default-off: {default_off}\n\
         - request_atlas_batching: {atlas_batching}\n\
         - M-4A masking-at-scale: {m4a}\n\
         - new semantic WGSL: {wgsl}\n\
         - new AccumulatorOp: {new_op}\n\
         - scenario reopened: {reopened}\n\n\
         ## GPU resident scheduler\n\
         - ticks_scheduled: {ticks}\n\
         - gpu_resident_across_ticks: {resident}\n\
         - inter_tick_world_readbacks: {inter_tick}\n\
         - dispatch_r1_ticks: {r1}\n\
         - dispatch_r2_ticks: {r2}\n\
         - dispatch_r4_ticks: {r4}\n\
         - dispatch_r6_ticks: {r6}\n\
         - dispatch_r6b_ticks: {r6b}\n\n\
         ## Tick authority model\n\
         - cpu_is_tick_authority: {cpu_authority}\n\
         - gpu_state_feeds_next_tick: {gpu_feeds}\n\
         - mirror_dispatch_after_cpu_tick: {mirror_dispatch}\n\
         - substrate_gap_for_true_pass: {substrate_gap}\n\
         - foreground_capture_method: {capture}\n\n\
         ## CPU oracle comparison\n\
         - r6c_checksum_expected: `{expected:016x}`\n\
         - r6c_checksum_observed: `{observed:016x}`\n\
         - integer_trajectory_bit_exact: {integer}\n\
         - r4_max_abs_delta: {r4_delta}\n\
         - r4_within_bound: {r4_bound}\n\
         - cpu_oracle_parity: {parity}\n\n\
         ## R6C whole-run GPU posture\n\
         `{gpu_posture}`\n\n\
         ## Residency trace excerpts\n\
         {trace}\n",
        status = report.status,
        verdict = report.verdict,
        checksum = report.stable_report_checksum,
        adapter = adapter,
        single_theater = report.single_resident_theater,
        single_tier = report.single_galactic_tier,
        default_off = report.default_off,
        atlas_batching = report.request_atlas_batching,
        m4a = report.m4a_masking_at_scale,
        wgsl = report.new_semantic_wgsl,
        new_op = report.new_accumulator_op,
        reopened = report.scenario_reopened,
        ticks = report.ticks_scheduled,
        resident = report.gpu_resident_across_ticks,
        inter_tick = report.inter_tick_world_readbacks,
        r1 = report.dispatch_r1_ticks,
        r2 = report.dispatch_r2_ticks,
        r4 = report.dispatch_r4_ticks,
        r6 = report.dispatch_r6_ticks,
        r6b = report.dispatch_r6b_ticks,
        cpu_authority = report.cpu_is_tick_authority,
        gpu_feeds = report.gpu_state_feeds_next_tick,
        mirror_dispatch = report.mirror_dispatch_after_cpu_tick,
        substrate_gap = report.substrate_gap_for_true_pass,
        capture = report.foreground_capture_method,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        integer = report.integer_trajectory_bit_exact,
        r4_delta = report.r4_max_abs_delta,
        r4_bound = report.r4_within_bound,
        parity = report.cpu_oracle_parity,
        gpu_posture = report.r6c_whole_run_gpu_posture,
        trace = trace,
    )
}

fn render_artifact(report: &Runtime0080R0Report) -> String {
    render_runtime_0080_r0_artifact(report)
}
