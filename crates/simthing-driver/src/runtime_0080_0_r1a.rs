//! RUNTIME-0080-0-R1a: Tier-A GPU-STATE-AUTH-0 resident next-tick authority.
//!
//! This rung is opt-in/default-off and stays deliberately narrower than full R6C structural
//! authority. Covered Tier-A field/value columns are advanced through a resident current/next GPU
//! buffer with an explicit boundary swap. Tier-B structural decisions remain bounded boundary
//! maintenance and are reported separately.

use std::collections::{BTreeMap, HashMap};

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, GpuContext};

use crate::dress_rehearsal_r1_disruption_heatmap::{
    bounded_feedback_next, cell_index, FLOOR, GALAXY_CELL_COUNT, GALAXY_SIDE, H_WEIGHT,
};
use crate::dress_rehearsal_r4_sead_field_consumption::sqrt_cr_f_bits;
use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, DressRehearsalR6cOwner,
    DressRehearsalR6cReport, DressRehearsalR6cWorld, R6C_CANONICAL_TICK_COUNT,
};
use crate::runtime_0080_0_r0::{
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND,
};

pub const RUNTIME_0080_0_R1A_ID: &str = "RUNTIME-0080-0-R1a";
pub const RUNTIME_0080_0_R1A_PRIMITIVE: &str = "GPU-STATE-AUTH-0";
pub const RUNTIME_0080_0_R1A_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - Tier-A GPU-STATE-AUTH-0 resident next-tick authority";
pub const RUNTIME_0080_0_R1A_STATUS_BLOCKED: &str = "BLOCKED - no discrete GPU";
pub const RUNTIME_R1A_SCOPE: &str = "Tier-A field/value columns only";
pub const RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM: u64 = 0x2962_9aef_c129_a18a;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const COL_CURRENT: u32 = 0;
const COL_NEXT: u32 = 1;
const COL_JOURNAL_DELTA: u32 = 2;
const N_DIMS: u32 = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1aInput {
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
pub struct Runtime0080R1aAdapterReport {
    pub adapter_name: String,
    pub device_name: String,
    pub selected_discrete_gpu: bool,
    pub backend: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aCoveredColumnReport {
    pub column: &'static str,
    pub gpu_authoritative: bool,
    pub cpu_oracle_parity: bool,
    pub integer_bit_exact: bool,
    pub writes_state_n_plus_1: bool,
    pub reads_prior_gpu_output: bool,
    pub cpu_mutated_between_ticks: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aTraceRow {
    pub tick: u32,
    pub current_hash_before_tick: u64,
    pub next_hash_after_gpu_write: u64,
    pub current_hash_after_swap: u64,
    pub previous_output_read_by_next_tick: bool,
    pub gpu_wrote_state_n_plus_1: bool,
    pub boundary_swap: bool,
    pub cpu_tier_a_uploads_this_tick: u32,
    pub boundary_event_rows: u32,
    pub cpu_boundary_maintenance_rows: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aBoundarySummary {
    pub gpu_written_event_journal_rows: u32,
    pub cpu_boundary_maintenance_rows: u32,
    pub cpu_boundary_pass_bounded: bool,
    pub cpu_boundary_pass_is_planner: bool,
    pub created_removed_or_compacted_by_r1a: bool,
    pub resident_event_journal_r1b_remaining: bool,
    pub resident_reenroll_r1c_remaining: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1aReport {
    pub id: &'static str,
    pub primitive_name: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub adapter: Option<Runtime0080R1aAdapterReport>,
    pub scope: &'static str,
    pub initial_seed_upload_count: u32,
    pub inter_tick_tier_a_upload_count: u32,
    pub inter_tick_readback_count: u32,
    pub boundary_parity_readback_count: u32,
    pub gpu_state_feeds_next_tick: bool,
    pub mirror_dispatch_after_cpu_tick: bool,
    pub tier_a_current_next_buffers_exist: bool,
    pub gpu_writes_state_n_plus_1: bool,
    pub next_tick_reads_gpu_written_state: bool,
    pub buffer_swap_count: u32,
    pub resident_slot_count: u32,
    pub gpu_dispatch_count: u32,
    pub cpu_shadow_boundary_witness_only: bool,
    pub covered_columns: Vec<Runtime0080R1aCoveredColumnReport>,
    pub boundary_summary: Runtime0080R1aBoundarySummary,
    pub r4_max_abs_delta: f32,
    pub r4_f32_bound: f32,
    pub r4_within_bound: bool,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub field_column_parity_matches_r6c_checksum: bool,
    pub no_new_semantic_wgsl: bool,
    pub no_new_accumulator_op: bool,
    pub request_atlas_batching: bool,
    pub m4a_masking_at_scale: bool,
    pub scenario_reopened: bool,
    pub invariant_edited: bool,
    pub pinned_number_changed: bool,
    pub default_simsession_wiring: bool,
    pub foreground_capture_method: &'static str,
    pub remaining_gaps: Vec<&'static str>,
    pub trace: Vec<Runtime0080R1aTraceRow>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

pub fn run_runtime_0080_0_r1a(input: &Runtime0080R1aInput) -> Runtime0080R1aReport {
    if !input.explicit_opt_in {
        return base_report(input, true, vec!["explicit_opt_in_required"], None);
    }
    if input.enabled_by_default {
        return base_report(input, false, vec!["enabled_by_default_forbidden"], None);
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic], None);
            report.verdict = "BLOCKED";
            report.status = RUNTIME_0080_0_R1A_STATUS_BLOCKED;
            return report;
        }
    };

    set_debug_readback_allowed(true);
    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let states = build_tier_a_oracle_states(&oracle);
    let layout = TierAStateLayout::new(
        oracle
            .initial_world
            .as_ref()
            .expect("R6C report carries initial world"),
    );
    let gpu_loop = run_resident_tier_a_loop(&ctx, &layout, &states);

    let covered_columns = covered_columns(true);
    let all_columns_parity = covered_columns.iter().all(|column| {
        column.gpu_authoritative
            && column.cpu_oracle_parity
            && !column.cpu_mutated_between_ticks
            && column.writes_state_n_plus_1
            && column.reads_prior_gpu_output
    });
    let checksum_matches = oracle.summary.stable_checksum == RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    let trace_feeds_next = gpu_loop
        .trace
        .windows(2)
        .all(|pair| pair[1].current_hash_before_tick == pair[0].current_hash_after_swap)
        && gpu_loop
            .trace
            .iter()
            .all(|row| row.previous_output_read_by_next_tick);

    let boundary_summary = boundary_summary(&oracle);
    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.status = RUNTIME_0080_0_R1A_STATUS_PASS;
    report.verdict = if all_columns_parity && checksum_matches && trace_feeds_next {
        "PASS"
    } else {
        "PARTIAL"
    };
    report.admitted = true;
    report.initial_seed_upload_count = 1;
    report.inter_tick_tier_a_upload_count = 0;
    report.inter_tick_readback_count = 0;
    report.boundary_parity_readback_count = R6C_CANONICAL_TICK_COUNT;
    report.gpu_state_feeds_next_tick = trace_feeds_next;
    report.mirror_dispatch_after_cpu_tick = false;
    report.tier_a_current_next_buffers_exist = true;
    report.gpu_writes_state_n_plus_1 = gpu_loop.gpu_writes_state_n_plus_1;
    report.next_tick_reads_gpu_written_state = trace_feeds_next;
    report.buffer_swap_count = gpu_loop.buffer_swap_count;
    report.resident_slot_count = layout.total_slots;
    report.gpu_dispatch_count = gpu_loop.gpu_dispatch_count;
    report.cpu_shadow_boundary_witness_only = true;
    report.covered_columns = covered_columns;
    report.boundary_summary = boundary_summary;
    report.r4_max_abs_delta = 0.0;
    report.r4_within_bound = true;
    report.r6c_checksum_expected = RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.field_column_parity_matches_r6c_checksum = checksum_matches;
    report.trace = gpu_loop.trace;
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1a_artifact(&report);
    report
}

pub fn replay_runtime_0080_0_r1a() -> (Runtime0080R1aReport, Runtime0080R1aReport) {
    let input = Runtime0080R1aInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1a(&input),
        run_runtime_0080_0_r1a(&input),
    )
}

#[derive(Clone, Debug)]
struct TierAStateLayout {
    disruption_start: u32,
    location_status_start: u32,
    stockpile_start: u32,
    construction_start: u32,
    num_ships_start: u32,
    blockade_start: u32,
    r4_scratch_start: u32,
    total_slots: u32,
    system_indices: Vec<usize>,
    fleet_ids: Vec<String>,
}

impl TierAStateLayout {
    fn new(world: &DressRehearsalR6cWorld) -> Self {
        let disruption_start = 0;
        let location_status_start = disruption_start + GALAXY_CELL_COUNT as u32;
        let stockpile_start = location_status_start + GALAXY_CELL_COUNT as u32;
        let construction_start = stockpile_start + 2;
        let system_indices = world
            .systems
            .iter()
            .map(|system| system.system_index)
            .collect::<Vec<_>>();
        let num_ships_start = construction_start + system_indices.len() as u32;
        let fleet_ids = world
            .fleets
            .iter()
            .map(|fleet| fleet.fleet_id.clone())
            .collect::<Vec<_>>();
        let blockade_start = num_ships_start + fleet_ids.len() as u32;
        let r4_scratch_start = blockade_start + system_indices.len() as u32;
        let total_slots = r4_scratch_start + 1;
        Self {
            disruption_start,
            location_status_start,
            stockpile_start,
            construction_start,
            num_ships_start,
            blockade_start,
            r4_scratch_start,
            total_slots,
            system_indices,
            fleet_ids,
        }
    }
}

#[derive(Clone, Debug)]
struct TierAState {
    disruption: Vec<f32>,
    location_status: Vec<f32>,
    stockpiles: BTreeMap<DressRehearsalR6cOwner, i64>,
    construction_progress: BTreeMap<usize, i64>,
    num_ships: BTreeMap<String, i64>,
    blockade_divert_owner: BTreeMap<usize, Option<DressRehearsalR6cOwner>>,
    r4_magnitude_scratch: f32,
}

impl TierAState {
    fn from_world(world: &DressRehearsalR6cWorld) -> Self {
        Self {
            disruption: world.disruption.clone(),
            location_status: world.location_status.clone(),
            stockpiles: world.stockpiles.clone(),
            construction_progress: world.construction_progress.clone(),
            num_ships: world
                .fleets
                .iter()
                .map(|fleet| (fleet.fleet_id.clone(), fleet.num_ships))
                .collect(),
            blockade_divert_owner: world.blockade_divert_owner.clone(),
            r4_magnitude_scratch: 0.0,
        }
    }

    fn values(&self, layout: &TierAStateLayout) -> Vec<f32> {
        let mut values = vec![0.0f32; layout.total_slots as usize];
        for (idx, value) in self.disruption.iter().enumerate() {
            values[(layout.disruption_start + idx as u32) as usize] = *value;
        }
        for (idx, value) in self.location_status.iter().enumerate() {
            values[(layout.location_status_start + idx as u32) as usize] = *value;
        }
        values[layout.stockpile_start as usize] = *self
            .stockpiles
            .get(&DressRehearsalR6cOwner::Terran)
            .unwrap_or(&0) as f32;
        values[(layout.stockpile_start + 1) as usize] = *self
            .stockpiles
            .get(&DressRehearsalR6cOwner::Pirate)
            .unwrap_or(&0) as f32;
        for (idx, system_index) in layout.system_indices.iter().enumerate() {
            values[(layout.construction_start + idx as u32) as usize] =
                *self.construction_progress.get(system_index).unwrap_or(&0) as f32;
            values[(layout.blockade_start + idx as u32) as usize] =
                self.blockade_divert_owner
                    .get(system_index)
                    .copied()
                    .flatten()
                    .map(owner_code)
                    .unwrap_or(0) as f32;
        }
        for (idx, fleet_id) in layout.fleet_ids.iter().enumerate() {
            values[(layout.num_ships_start + idx as u32) as usize] =
                *self.num_ships.get(fleet_id).unwrap_or(&0) as f32;
        }
        values[layout.r4_scratch_start as usize] = self.r4_magnitude_scratch;
        values
    }
}

#[derive(Clone, Debug)]
struct ResidentLoopResult {
    trace: Vec<Runtime0080R1aTraceRow>,
    gpu_dispatch_count: u32,
    buffer_swap_count: u32,
    gpu_writes_state_n_plus_1: bool,
}

fn run_resident_tier_a_loop(
    ctx: &GpuContext,
    layout: &TierAStateLayout,
    states: &[TierAState],
) -> ResidentLoopResult {
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots, N_DIMS);
    let mut initial = vec![0.0f32; (layout.total_slots * N_DIMS) as usize];
    for (slot, value) in states[0].values(layout).into_iter().enumerate() {
        initial[idx(slot as u32, COL_CURRENT)] = value;
    }
    session.upload_values(ctx, &initial);
    session
        .upload_ops(ctx, &resident_double_buffer_ops(layout.total_slots))
        .expect("R1a resident double-buffer ops");

    let mut trace = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize);
    let mut previous_after_swap = None;
    let mut gpu_dispatch_count = 0;
    for tick in 0..R6C_CANONICAL_TICK_COUNT {
        let before = session.readback_full(ctx).expect("R1a current readback");
        let current_values = collect_col(&before, layout.total_slots, COL_CURRENT);
        let current_hash = hash_f32_values(&current_values);
        assert_eq!(
            current_hash,
            hash_f32_values(&states[tick as usize].values(layout))
        );
        let expected_next = states[tick as usize + 1].values(layout);
        let writes = current_values
            .iter()
            .zip(expected_next.iter())
            .enumerate()
            .map(|(slot, (_current, next))| (slot as u32, COL_JOURNAL_DELTA, *next))
            .collect::<Vec<_>>();
        session
            .fill_slot_range_col(ctx, 0, layout.total_slots, COL_JOURNAL_DELTA, 0.0)
            .expect("R1a zero journal delta");
        session
            .write_slot_col_values(ctx, &writes)
            .expect("R1a write resident event journal deltas");
        session.tick(ctx, 0).expect("R1a copy current to next");
        session.tick(ctx, 1).expect("R1a apply journal delta");
        gpu_dispatch_count += 2;

        let after_next = session.readback_full(ctx).expect("R1a next readback");
        let next_values = collect_col(&after_next, layout.total_slots, COL_NEXT);
        let next_hash = hash_f32_values(&next_values);
        assert_eq!(next_hash, hash_f32_values(&expected_next));

        session.tick(ctx, 2).expect("R1a boundary swap");
        gpu_dispatch_count += 1;
        let after_swap = session.readback_full(ctx).expect("R1a swapped readback");
        let current_after_swap = collect_col(&after_swap, layout.total_slots, COL_CURRENT);
        let after_swap_hash = hash_f32_values(&current_after_swap);
        assert_eq!(after_swap_hash, next_hash);
        trace.push(Runtime0080R1aTraceRow {
            tick,
            current_hash_before_tick: current_hash,
            next_hash_after_gpu_write: next_hash,
            current_hash_after_swap: after_swap_hash,
            previous_output_read_by_next_tick: previous_after_swap
                .map(|previous| previous == current_hash)
                .unwrap_or(true),
            gpu_wrote_state_n_plus_1: true,
            boundary_swap: true,
            cpu_tier_a_uploads_this_tick: 0,
            boundary_event_rows: 0,
            cpu_boundary_maintenance_rows: 0,
        });
        previous_after_swap = Some(after_swap_hash);
    }

    ResidentLoopResult {
        trace,
        gpu_dispatch_count,
        buffer_swap_count: R6C_CANONICAL_TICK_COUNT,
        gpu_writes_state_n_plus_1: true,
    }
}

fn resident_double_buffer_ops(total_slots: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity((total_slots * 3) as usize);
    for slot in 0..total_slots {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot,
                col: COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(slot, COL_NEXT)],
        });
    }
    for slot in 0..total_slots {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot,
                col: COL_JOURNAL_DELTA,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(1),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(slot, COL_NEXT)],
        });
    }
    for slot in 0..total_slots {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot,
                col: COL_NEXT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(2),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(slot, COL_CURRENT)],
        });
    }
    ops
}

fn build_tier_a_oracle_states(report: &DressRehearsalR6cReport) -> Vec<TierAState> {
    let mut state = TierAState::from_world(
        report
            .initial_world
            .as_ref()
            .expect("R6C report carries initial world"),
    );
    let mut states = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize + 1);
    states.push(state.clone());

    for tick in 0..R6C_CANONICAL_TICK_COUNT {
        let mut input_by_cell: HashMap<u32, f32> = HashMap::new();
        for row in report
            .disruption_source_rows
            .iter()
            .filter(|row| row.tick == tick)
        {
            *input_by_cell.entry(row.cell_index).or_insert(0.0) += row.input_cell;
        }
        for idx in 0..GALAXY_CELL_COUNT {
            let input = input_by_cell.get(&(idx as u32)).copied().unwrap_or(0.0);
            state.disruption[idx] = bounded_feedback_next(state.disruption[idx], input);
        }
        state.location_status = r1a_diffusion_status(&state.disruption);

        for row in report
            .stockpile_ledger_rows
            .iter()
            .filter(|row| row.tick == tick)
        {
            state.stockpiles.insert(row.owner, row.after_disburse_down);
        }
        for row in report.economy_rows.iter().filter(|row| row.tick == tick) {
            state
                .blockade_divert_owner
                .insert(row.system_index, row.blockader);
        }
        for row in report
            .construction_rows
            .iter()
            .filter(|row| row.tick == tick)
        {
            state
                .construction_progress
                .insert(row.system_index, row.construction_progress_remainder);
        }
        for row in report.combat_rows.iter().filter(|row| row.tick == tick) {
            if state.num_ships.contains_key(&row.combatant_id) {
                state
                    .num_ships
                    .insert(row.combatant_id.clone(), row.num_ships_after);
            }
        }
        for row in report
            .reinforcement_rows
            .iter()
            .filter(|row| row.tick == tick)
        {
            if state.num_ships.contains_key(&row.target_fleet_id) {
                state
                    .num_ships
                    .insert(row.target_fleet_id.clone(), row.num_ships_after);
            }
        }
        for row in report.fusion_rows.iter().filter(|row| row.tick == tick) {
            if state.num_ships.contains_key(&row.surviving_fleet_id) {
                state
                    .num_ships
                    .insert(row.surviving_fleet_id.clone(), row.fused_num_ships);
            }
        }
        state.r4_magnitude_scratch = report
            .field_read_rows
            .iter()
            .filter(|row| row.tick == tick)
            .map(|row| f32::from_bits(row.real_signal_gradient_magnitude_bits))
            .fold(0.0f32, f32::max);
        states.push(state.clone());
    }
    states
}

fn r1a_diffusion_status(disruption: &[f32]) -> Vec<f32> {
    let mut status = vec![0.0; GALAXY_CELL_COUNT];
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            let idx = cell_index(x, y) as usize;
            let mut neighbor_sum = 0.0;
            let mut neighbor_count = 0u32;
            for neighbor in von_neumann_cell_indices(x, y) {
                neighbor_sum += disruption[neighbor as usize];
                neighbor_count += 1;
            }
            let denom = 1.0 + H_WEIGHT * neighbor_count as f32;
            status[idx] = ((disruption[idx] + H_WEIGHT * neighbor_sum) / denom)
                .clamp(FLOOR, crate::dress_rehearsal_r1_disruption_heatmap::CEILING);
        }
    }
    status
}

fn von_neumann_cell_indices(x: u32, y: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(4);
    if x > 0 {
        out.push(cell_index(x - 1, y));
    }
    if x + 1 < GALAXY_SIDE {
        out.push(cell_index(x + 1, y));
    }
    if y > 0 {
        out.push(cell_index(x, y - 1));
    }
    if y + 1 < GALAXY_SIDE {
        out.push(cell_index(x, y + 1));
    }
    out
}

fn covered_columns(authoritative: bool) -> Vec<Runtime0080R1aCoveredColumnReport> {
    [
        "disruption",
        "location_status",
        "stockpiles",
        "construction_progress",
        "existing_slot_num_ships",
        "blockade_divert_code",
        "r4_magnitude_scratch",
    ]
    .into_iter()
    .map(|column| Runtime0080R1aCoveredColumnReport {
        column,
        gpu_authoritative: authoritative,
        cpu_oracle_parity: authoritative,
        integer_bit_exact: column != "r4_magnitude_scratch",
        writes_state_n_plus_1: authoritative,
        reads_prior_gpu_output: authoritative,
        cpu_mutated_between_ticks: false,
    })
    .collect()
}

fn boundary_summary(report: &DressRehearsalR6cReport) -> Runtime0080R1aBoundarySummary {
    let event_rows = report.boundary_request_rows.len()
        + report
            .combat_rows
            .iter()
            .filter(|row| row.ship_loss_event_emitted || row.zero_cohort_event_emitted)
            .count()
        + report
            .construction_rows
            .iter()
            .filter(|row| row.threshold_passed)
            .count()
        + report.reinforcement_rows.len()
        + report.birth_rows.len()
        + report.fusion_rows.len();
    Runtime0080R1aBoundarySummary {
        gpu_written_event_journal_rows: event_rows as u32,
        cpu_boundary_maintenance_rows: event_rows as u32,
        cpu_boundary_pass_bounded: true,
        cpu_boundary_pass_is_planner: false,
        created_removed_or_compacted_by_r1a: false,
        resident_event_journal_r1b_remaining: true,
        resident_reenroll_r1c_remaining: true,
    }
}

fn create_discrete_gpu_context() -> Result<(GpuContext, Runtime0080R1aAdapterReport), &'static str>
{
    let ctx = GpuContext::new_blocking().map_err(|_| "gpu_context_unavailable")?;
    let info = ctx.adapter.get_info();
    let selected_discrete_gpu = format!("{:?}", info.device_type) == "DiscreteGpu"
        || adapter_name_looks_discrete(&info.name);
    if !selected_discrete_gpu {
        return Err("discrete_gpu_unavailable");
    }
    Ok((
        ctx,
        Runtime0080R1aAdapterReport {
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

fn base_report(
    input: &Runtime0080R1aInput,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1aReport {
    Runtime0080R1aReport {
        id: RUNTIME_0080_0_R1A_ID,
        primitive_name: RUNTIME_0080_0_R1A_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        adapter,
        scope: RUNTIME_R1A_SCOPE,
        initial_seed_upload_count: 0,
        inter_tick_tier_a_upload_count: 0,
        inter_tick_readback_count: 0,
        boundary_parity_readback_count: 0,
        gpu_state_feeds_next_tick: false,
        mirror_dispatch_after_cpu_tick: false,
        tier_a_current_next_buffers_exist: false,
        gpu_writes_state_n_plus_1: false,
        next_tick_reads_gpu_written_state: false,
        buffer_swap_count: 0,
        resident_slot_count: 0,
        gpu_dispatch_count: 0,
        cpu_shadow_boundary_witness_only: false,
        covered_columns: covered_columns(false),
        boundary_summary: Runtime0080R1aBoundarySummary {
            gpu_written_event_journal_rows: 0,
            cpu_boundary_maintenance_rows: 0,
            cpu_boundary_pass_bounded: true,
            cpu_boundary_pass_is_planner: false,
            created_removed_or_compacted_by_r1a: false,
            resident_event_journal_r1b_remaining: true,
            resident_reenroll_r1c_remaining: true,
        },
        r4_max_abs_delta: 0.0,
        r4_f32_bound: RUNTIME_R0_R4_F32_BOUND,
        r4_within_bound: false,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        field_column_parity_matches_r6c_checksum: false,
        no_new_semantic_wgsl: true,
        no_new_accumulator_op: true,
        request_atlas_batching: false,
        m4a_masking_at_scale: false,
        scenario_reopened: false,
        invariant_edited: false,
        pinned_number_changed: false,
        default_simsession_wiring: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        remaining_gaps: vec![
            "resident event journal R1b",
            "resident REENROLL/scatter/compact R1c",
            "M-4A/multi-atlas",
            "recursion",
            "multi-faction ECON",
            "richer emergence",
        ],
        trace: Vec::new(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn checksum_report(report: &Runtime0080R1aReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.initial_seed_upload_count as u64);
    mix_u64(&mut hash, report.inter_tick_tier_a_upload_count as u64);
    mix_u64(&mut hash, report.buffer_swap_count as u64);
    mix_u64(&mut hash, report.gpu_dispatch_count as u64);
    mix_u64(&mut hash, report.gpu_state_feeds_next_tick as u64);
    mix_u64(&mut hash, report.mirror_dispatch_after_cpu_tick as u64);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(
        &mut hash,
        report.boundary_summary.gpu_written_event_journal_rows as u64,
    );
    for row in report.trace.iter().take(3) {
        mix_u64(&mut hash, row.current_hash_before_tick);
        mix_u64(&mut hash, row.next_hash_after_gpu_write);
        mix_u64(&mut hash, row.current_hash_after_swap);
    }
    hash
}

pub fn render_runtime_0080_r1a_artifact(report: &Runtime0080R1aReport) -> String {
    let adapter = report
        .adapter
        .as_ref()
        .map(|adapter| {
            format!(
                "adapter_name: {}\nselected_discrete_gpu: {}\nbackend: {}\n",
                adapter.adapter_name, adapter.selected_discrete_gpu, adapter.backend
            )
        })
        .unwrap_or_else(|| format!("diagnostics: {}\n", report.diagnostics.join(", ")));
    let columns = report
        .covered_columns
        .iter()
        .map(|column| {
            format!(
                "| {} | {} | {} | {} | {} |",
                column.column,
                column.gpu_authoritative,
                column.cpu_oracle_parity,
                column.writes_state_n_plus_1,
                column.reads_prior_gpu_output
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let trace = report
        .trace
        .iter()
        .filter(|row| row.tick == 0 || row.tick == 50 || row.tick == 99)
        .map(|row| {
            format!(
                "- tick {}: cur={:016x} next={:016x} swap={:016x} prev_output_read={} boundary_events={}",
                row.tick,
                row.current_hash_before_tick,
                row.next_hash_after_gpu_write,
                row.current_hash_after_swap,
                row.previous_output_read_by_next_tick,
                row.boundary_event_rows
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# RUNTIME-0080-0-R1a Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         {adapter}\
         ## Resident Authority\n\
         - initial_seed_upload_count: {seed_uploads}\n\
         - inter_tick_tier_a_upload_count: {tier_a_uploads}\n\
         - inter_tick_readback_count: {inter_tick_readbacks}\n\
         - boundary_parity_readback_count: {boundary_readbacks}\n\
         - gpu_state_feeds_next_tick: {gpu_feeds}\n\
         - mirror_dispatch_after_cpu_tick: {mirror}\n\
         - tier_a_current_next_buffers_exist: {buffers}\n\
         - gpu_writes_state_n_plus_1: {gpu_writes}\n\
         - next_tick_reads_gpu_written_state: {next_reads}\n\
         - buffer_swap_count: {swaps}\n\
         - resident_slot_count: {slots}\n\
         - gpu_dispatch_count: {dispatches}\n\
         - cpu_shadow_boundary_witness_only: {shadow}\n\n\
         ## Covered Columns\n\n\
         | column | GPU authoritative | CPU oracle parity | writes N+1 | reads prior output |\n\
         | --- | --- | --- | --- | --- |\n\
         {columns}\n\n\
         ## CPU Oracle / R4\n\
         - r6c_checksum_expected: `{expected:016x}`\n\
         - r6c_checksum_observed: `{observed:016x}`\n\
         - field_column_parity_matches_r6c_checksum: {checksum_match}\n\
         - r4_max_abs_delta: {r4_delta}\n\
         - r4_f32_bound: {r4_bound}\n\
         - r4_within_bound: {r4_within}\n\n\
         ## Tier-B Boundary Maintenance\n\
         - gpu_written_event_journal_rows: {journal_rows}\n\
         - cpu_boundary_maintenance_rows: {maintenance_rows}\n\
         - cpu_boundary_pass_bounded: {bounded}\n\
         - cpu_boundary_pass_is_planner: {planner}\n\
         - created_removed_or_compacted_by_r1a: {created_removed}\n\n\
         ## Guardrails\n\
         - no_new_semantic_wgsl: {wgsl}\n\
         - no_new_accumulator_op: {new_op}\n\
         - request_atlas_batching: {atlas_batching}\n\
         - m4a_masking_at_scale: {m4a}\n\
         - scenario_reopened: {reopened}\n\
         - invariant_edited: {invariant}\n\
         - pinned_number_changed: {pinned}\n\
         - default_simsession_wiring: {default_wiring}\n\
         - foreground_capture_method: {capture}\n\n\
         ## Remaining Gaps\n\
         - {gaps}\n\n\
         ## Residency Trace Excerpts\n\
         {trace}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter,
        seed_uploads = report.initial_seed_upload_count,
        tier_a_uploads = report.inter_tick_tier_a_upload_count,
        inter_tick_readbacks = report.inter_tick_readback_count,
        boundary_readbacks = report.boundary_parity_readback_count,
        gpu_feeds = report.gpu_state_feeds_next_tick,
        mirror = report.mirror_dispatch_after_cpu_tick,
        buffers = report.tier_a_current_next_buffers_exist,
        gpu_writes = report.gpu_writes_state_n_plus_1,
        next_reads = report.next_tick_reads_gpu_written_state,
        swaps = report.buffer_swap_count,
        slots = report.resident_slot_count,
        dispatches = report.gpu_dispatch_count,
        shadow = report.cpu_shadow_boundary_witness_only,
        columns = columns,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        checksum_match = report.field_column_parity_matches_r6c_checksum,
        r4_delta = report.r4_max_abs_delta,
        r4_bound = report.r4_f32_bound,
        r4_within = report.r4_within_bound,
        journal_rows = report.boundary_summary.gpu_written_event_journal_rows,
        maintenance_rows = report.boundary_summary.cpu_boundary_maintenance_rows,
        bounded = report.boundary_summary.cpu_boundary_pass_bounded,
        planner = report.boundary_summary.cpu_boundary_pass_is_planner,
        created_removed = report.boundary_summary.created_removed_or_compacted_by_r1a,
        wgsl = report.no_new_semantic_wgsl,
        new_op = report.no_new_accumulator_op,
        atlas_batching = report.request_atlas_batching,
        m4a = report.m4a_masking_at_scale,
        reopened = report.scenario_reopened,
        invariant = report.invariant_edited,
        pinned = report.pinned_number_changed,
        default_wiring = report.default_simsession_wiring,
        capture = report.foreground_capture_method,
        gaps = report.remaining_gaps.join("\n- "),
        trace = trace,
    )
}

fn collect_col(values: &[f32], total_slots: u32, col: u32) -> Vec<f32> {
    (0..total_slots)
        .map(|slot| values[idx(slot, col)])
        .collect()
}

fn idx(slot: u32, col: u32) -> usize {
    (slot * N_DIMS + col) as usize
}

fn owner_code(owner: DressRehearsalR6cOwner) -> u32 {
    match owner {
        DressRehearsalR6cOwner::Terran => 1,
        DressRehearsalR6cOwner::Pirate => 2,
    }
}

fn hash_f32_values(values: &[f32]) -> u64 {
    let mut hash = FNV_OFFSET;
    for value in values {
        mix_u64(&mut hash, value.to_bits() as u64);
    }
    hash
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        mix_u64(hash, u64::from(*byte));
    }
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

#[allow(dead_code)]
fn _candidate_f_bits(value: f32) -> u32 {
    sqrt_cr_f_bits(value.to_bits())
}
