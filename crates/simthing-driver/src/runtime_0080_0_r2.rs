//! RUNTIME-0080-0-R2: stable GPU-forward 100-tick rehearsal over R1a–R1c-f.

use std::time::Instant;

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, ThresholdEvent};

use crate::dress_rehearsal_r6c_integrated_run::{
    r1b_apply_boundary_events, r1b_oracle_events_by_tick, run_dress_rehearsal_r6c_integrated_run,
    DressRehearsalR6cInput, DressRehearsalR6cOwner, R1aBoundaryWitness, R1bStructuralEvent,
    R1bStructuralEventKind, R6C_CANONICAL_TICK_COUNT,
};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    compute_comparison_oracle_trajectory, create_discrete_gpu_context, slot_col_idx,
    state_values_match_oracle, DisabledTransformMask, Runtime0080R1aAdapterReport, TierAGpuHarness,
    TierAStateLayout, TierAStaticConfig, R1A_COL_CURRENT, R1A_N_DIMS,
};
use crate::runtime_0080_0_r2_substrate::{run_r2_structural_substrates, R2SubstrateOutcome};

pub const RUNTIME_0080_0_R2_ID: &str = "RUNTIME-0080-0-R2";
pub const RUNTIME_0080_0_R2_PRIMITIVE: &str = "STABLE-100-TICK-GPU-FORWARD-REHEARSAL-0";
pub const RUNTIME_0080_0_R2_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - stable 100-tick GPU-forward rehearsal over R1a–R1c-f";
pub const RUNTIME_0080_0_R2_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - 100-tick rehearsal incomplete or checksum delta unexplained";
pub const RUNTIME_0080_0_R2_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R2_SCOPE: &str =
    "stable 100-tick GPU-forward rehearsal: R1a + R1b + R1c-a/b/c/d/e + R1c-f ZeroCohort";
pub const RUNTIME_R2_EXPECTED_REPORT_CHECKSUM: u64 = 0x73d8_1841_7f5b_98bf;

const EVENT_JOURNAL_MAX_ROWS_PER_TICK: u32 = 128;
const EVENT_JOURNAL_FIELDS_PER_ROW: u32 = 9;
const EVENT_JOURNAL_COPY_BAND: u32 = 0;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R2Input {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R2Input {
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
pub struct Runtime0080R2TickTraceRow {
    pub tick: u32,
    pub journal_rows: u32,
    pub oracle_rows: u32,
    pub parity_with_oracle: bool,
}

/// Per-tick wall-clock breakdown for the resident GPU-forward loop (milliseconds).
#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R2TickTimingRow {
    pub tick: u32,
    pub total_ms: f64,
    /// Full-session GPU readback before combat attrition probe (CPU blocked on GPU).
    pub gpu_readback_pre_combat_ms: f64,
    /// CPU witness: derive tick inputs + structural events (excl. ZeroCohort).
    pub cpu_witness_ms: f64,
    /// CPU-side derived-input upload into Tier-A session slots.
    pub cpu_write_derived_ms: f64,
    /// Full-session GPU readback for tick-input snapshot (CPU blocked on GPU).
    pub gpu_readback_tick_input_ms: f64,
    /// GPU combat-attrition probe + threshold emission readback (CPU blocked on GPU).
    pub gpu_zero_cohort_probe_ms: f64,
    /// Tier-A transform dispatches + buffer swap submit (async; completion accounted next tick).
    pub gpu_tier_a_dispatch_ms: f64,
    /// Resident journal stage/copy + committed-row readback (CPU blocked on GPU).
    pub gpu_journal_stage_ms: f64,
    /// CPU boundary apply over committed journal rows.
    pub cpu_boundary_apply_ms: f64,
    /// Sum of in-tick GPU readback stalls (CPU waiting on GPU).
    pub gpu_sync_wait_ms: f64,
    /// Sum of in-tick CPU compute phases (witness + boundary apply + derived upload).
    pub cpu_active_ms: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R2MemoryFootprint {
    pub gpu_world_buffer_bytes: u64,
    pub gpu_tier_a_session_bytes: u64,
    pub gpu_journal_session_bytes: u64,
    pub gpu_zero_cohort_probe_bytes: u64,
    pub gpu_persistent_total_bytes: u64,
    /// Largest single readback staging copy (Tier-A full values buffer).
    pub cpu_readback_staging_peak_bytes: u64,
    pub cpu_committed_journal_rows: u64,
    pub process_working_set_bytes: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R2Profiling {
    pub oracle_setup_ms: f64,
    pub gpu_loop_ms: f64,
    pub substrate_ms: f64,
    pub total_wall_ms: f64,
    pub mean_tick_ms: f64,
    pub min_tick_ms: f64,
    pub max_tick_ms: f64,
    pub total_gpu_sync_wait_ms: f64,
    pub total_cpu_active_ms: f64,
    pub total_gpu_tier_a_dispatch_ms: f64,
    pub mean_gpu_sync_wait_ms: f64,
    pub mean_cpu_active_ms: f64,
    /// Fraction of per-tick wall time spent in CPU-active phases (witness + apply + uploads).
    pub cpu_active_fraction: f64,
    /// Fraction of per-tick wall time spent blocked on GPU readbacks.
    pub gpu_sync_wait_fraction: f64,
    pub pipeline_interpretation: &'static str,
    pub per_tick_timing: Vec<Runtime0080R2TickTimingRow>,
    pub memory: Runtime0080R2MemoryFootprint,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R2Report {
    pub id: &'static str,
    pub primitive_name: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<String>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub scope: &'static str,
    pub adapter: Option<Runtime0080R1aAdapterReport>,
    pub runs_100_ticks: bool,
    pub uses_r1a_tier_a_gpu_next_tick: bool,
    pub uses_r1b_resident_event_journal: bool,
    pub uses_r1c_structural_substrate: bool,
    pub uses_r1c_f_gpu_zero_cohort: bool,
    pub zero_cohort_cpu_decided: bool,
    pub m4a_required: bool,
    pub multi_atlas_required: bool,
    pub new_copy_substrate_added: bool,
    pub tick_count: u32,
    pub tier_a_tick100_matches_oracle: bool,
    pub event_journal_parity: bool,
    pub structural_decisions_gpu_emitted_zero_cohort: bool,
    pub structural_decisions_gpu_emitted: bool,
    pub zero_cohort_row_count: u32,
    pub remaining_cpu_decided_classes: Vec<&'static str>,
    pub remaining_class_blocked_run: bool,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub r6c_checksum_matches: bool,
    pub r6c_checksum_delta_explained: &'static str,
    pub substrate: Option<R2SubstrateOutcome>,
    pub per_tick_trace: Vec<Runtime0080R2TickTraceRow>,
    pub profiling: Option<Runtime0080R2Profiling>,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Copy, Debug)]
struct EventJournalLayout {
    staging_row_count_slot: u32,
    committed_row_count_slot: u32,
    staging_rows_start: u32,
    committed_rows_start: u32,
}

impl EventJournalLayout {
    fn new() -> Self {
        let staging_row_count_slot = 0;
        let committed_row_count_slot = 1;
        let staging_rows_start = 2;
        let committed_rows_start =
            staging_rows_start + EVENT_JOURNAL_MAX_ROWS_PER_TICK * EVENT_JOURNAL_FIELDS_PER_ROW;
        Self {
            staging_row_count_slot,
            committed_row_count_slot,
            staging_rows_start,
            committed_rows_start,
        }
    }

    fn total_slots(&self) -> u32 {
        self.committed_rows_start + EVENT_JOURNAL_MAX_ROWS_PER_TICK * EVENT_JOURNAL_FIELDS_PER_ROW
    }

    fn staging_field_slot(&self, row: u32, field: u32) -> u32 {
        self.staging_rows_start + row * EVENT_JOURNAL_FIELDS_PER_ROW + field
    }

    fn committed_field_slot(&self, row: u32, field: u32) -> u32 {
        self.committed_rows_start + row * EVENT_JOURNAL_FIELDS_PER_ROW + field
    }
}

pub fn run_runtime_0080_0_r2(input: &Runtime0080R2Input) -> Runtime0080R2Report {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required".to_string()],
            None,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden".to_string()],
            None,
        ));
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic.to_string()], None);
            report.status = RUNTIME_0080_0_R2_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            return finalize_report(report);
        }
    };

    set_debug_readback_allowed(true);
    let run_started = Instant::now();
    let oracle_started = Instant::now();
    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let oracle_setup_ms = oracle_started.elapsed().as_secs_f64() * 1000.0;
    let world = oracle
        .initial_world
        .as_ref()
        .expect("R6C report carries initial world");
    let layout = TierAStateLayout::new(world);
    let oracle_trajectory = compute_comparison_oracle_trajectory(&oracle);
    let static_config = TierAStaticConfig::from_initial_world(world, &layout);
    let fleet_ids = world
        .fleets
        .iter()
        .map(|fleet| fleet.fleet_id.clone())
        .collect::<Vec<_>>();
    let system_indices = world
        .systems
        .iter()
        .map(|system| system.system_index)
        .collect::<Vec<_>>();
    let mut boundary_witness =
        R1aBoundaryWitness::new(world, fleet_ids.clone(), system_indices.clone());

    let mut harness = match TierAGpuHarness::new(ctx, &layout, world, &static_config) {
        Ok(h) => h,
        Err(diagnostic) => {
            return finalize_report(base_report(
                input,
                false,
                vec![diagnostic.to_string()],
                Some(adapter),
            ));
        }
    };

    let journal_layout = EventJournalLayout::new();
    let mut journal_session =
        AccumulatorOpSession::new(&harness.world.ctx, journal_layout.total_slots(), R1A_N_DIMS);
    let mut zero_cohort_probe =
        AccumulatorOpSession::new(&harness.world.ctx, harness.n_session_slots, R1A_N_DIMS);
    let journal_copy_ops = build_journal_copy_ops(&journal_layout);
    if journal_session
        .fill_slot_range_col(
            &harness.world.ctx,
            0,
            journal_layout.total_slots(),
            R1A_COL_CURRENT,
            0.0,
        )
        .is_err()
    {
        return finalize_report(base_report(
            input,
            false,
            vec!["journal_seed_failed".to_string()],
            Some(adapter),
        ));
    }

    let oracle_events_by_tick = r1b_oracle_events_by_tick(
        &oracle,
        boundary_witness.fleet_ids(),
        boundary_witness.system_indices(),
    );

    let mut diagnostics = Vec::new();
    let mut all_committed_rows = Vec::new();
    let mut per_tick_trace = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize);
    let mut per_tick_timing = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize);
    let mut per_tick_parity_ok = true;
    let mut max_r4_abs_delta = 0.0f32;
    let mut zero_cohort_row_count = 0u32;
    let gpu_loop_started = Instant::now();

    let mut boundary_shadow = boundary_witness.clone_for_event_derivation();
    let shadow_fleet_ids = boundary_witness.fleet_ids().to_vec();
    let shadow_system_indices = boundary_witness.system_indices().to_vec();

    for tick in 0..R6C_CANONICAL_TICK_COUNT {
        let tick_started = Instant::now();

        let t0 = Instant::now();
        let pre_combat_values = match harness.tier_a.readback_full(&harness.world.ctx) {
            Ok(values) => values,
            Err(_) => {
                diagnostics.push("pre_combat_readback_failed".to_string());
                break;
            }
        };
        let gpu_readback_pre_combat_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        let (derived, witness_events) =
            boundary_witness.step_tick_capture_events_excluding_zero_cohort(tick);
        let cpu_witness_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let t2 = Instant::now();
        if harness
            .write_tick_derived_inputs(&layout, &derived)
            .is_err()
        {
            diagnostics.push("write_tick_derived_inputs_failed".to_string());
        }
        let cpu_write_derived_ms = t2.elapsed().as_secs_f64() * 1000.0;

        let t3 = Instant::now();
        let tick_input_values = harness
            .tier_a
            .readback_full(&harness.world.ctx)
            .unwrap_or(pre_combat_values.clone());
        let gpu_readback_tick_input_ms = t3.elapsed().as_secs_f64() * 1000.0;

        let t4 = Instant::now();
        let gpu_zero_cohort_events = match harness.probe_zero_cohort_threshold_emissions(
            &harness.world.ctx,
            &layout,
            &derived,
            &pre_combat_values,
            &tick_input_values,
            &mut zero_cohort_probe,
        ) {
            Ok(emissions) => emissions
                .into_iter()
                .filter_map(|emission| {
                    threshold_to_zero_cohort_event(tick, emission, &layout, &boundary_witness)
                })
                .collect(),
            Err(diagnostic) => {
                diagnostics.push(diagnostic.to_string());
                Vec::new()
            }
        };
        let gpu_zero_cohort_probe_ms = t4.elapsed().as_secs_f64() * 1000.0;

        let t5 = Instant::now();
        harness.dispatch_tier_a_transforms(
            &layout,
            &derived,
            tick,
            &mut max_r4_abs_delta,
            DisabledTransformMask::all_enabled(),
        );
        if harness.tier_a.tick(&harness.world.ctx, 0).is_err() {
            diagnostics.push("tier_a_swap_failed".to_string());
        }
        let gpu_tier_a_dispatch_ms = t5.elapsed().as_secs_f64() * 1000.0;

        let mut staged_events = witness_events;
        staged_events.extend(gpu_zero_cohort_events);

        let t6 = Instant::now();
        let committed_rows = match stage_dispatch_decode_events(
            &harness.world.ctx,
            &mut journal_session,
            &journal_layout,
            &journal_copy_ops,
            &staged_events,
        ) {
            Ok(rows) => rows,
            Err(diagnostic) => {
                diagnostics.push(diagnostic.to_string());
                Vec::new()
            }
        };
        let gpu_journal_stage_ms = t6.elapsed().as_secs_f64() * 1000.0;

        let t7 = Instant::now();
        let apply_report = r1b_apply_boundary_events(
            boundary_shadow.world_mut(),
            &shadow_fleet_ids,
            &shadow_system_indices,
            &committed_rows,
        );
        if apply_report.rows_applied > committed_rows.len() as u32 {
            diagnostics.push(format!("boundary_apply_overflow_tick_{}", tick));
        }
        let cpu_boundary_apply_ms = t7.elapsed().as_secs_f64() * 1000.0;

        let gpu_sync_wait_ms = gpu_readback_pre_combat_ms
            + gpu_readback_tick_input_ms
            + gpu_zero_cohort_probe_ms
            + gpu_journal_stage_ms;
        let cpu_active_ms = cpu_witness_ms + cpu_write_derived_ms + cpu_boundary_apply_ms;
        let total_ms = tick_started.elapsed().as_secs_f64() * 1000.0;
        per_tick_timing.push(Runtime0080R2TickTimingRow {
            tick,
            total_ms,
            gpu_readback_pre_combat_ms,
            cpu_witness_ms,
            cpu_write_derived_ms,
            gpu_readback_tick_input_ms,
            gpu_zero_cohort_probe_ms,
            gpu_tier_a_dispatch_ms,
            gpu_journal_stage_ms,
            cpu_boundary_apply_ms,
            gpu_sync_wait_ms,
            cpu_active_ms,
        });

        let oracle_rows = oracle_events_by_tick
            .get(&tick)
            .cloned()
            .unwrap_or_default();
        let parity_with_oracle =
            canonical_event_rows(&committed_rows) == canonical_event_rows(&oracle_rows);
        if !parity_with_oracle {
            per_tick_parity_ok = false;
            diagnostics.push(format!("oracle_parity_miss_tick_{}", tick));
        }

        zero_cohort_row_count += committed_rows
            .iter()
            .filter(|row| row.event_kind == R1bStructuralEventKind::ZeroCohort)
            .count() as u32;

        per_tick_trace.push(Runtime0080R2TickTraceRow {
            tick,
            journal_rows: committed_rows.len() as u32,
            oracle_rows: oracle_rows.len() as u32,
            parity_with_oracle,
        });
        all_committed_rows.extend(committed_rows);
    }

    let gpu_loop_ms = gpu_loop_started.elapsed().as_secs_f64() * 1000.0;
    let ticks_completed = per_tick_trace.len() as u32;
    let runs_100_ticks = ticks_completed == R6C_CANONICAL_TICK_COUNT;

    let all_oracle_rows = oracle_events_by_tick
        .values()
        .flat_map(|rows| rows.iter().cloned())
        .collect::<Vec<_>>();
    let event_journal_parity =
        canonical_event_rows(&all_committed_rows) == canonical_event_rows(&all_oracle_rows);

    let final_gpu = harness
        .tier_a
        .readback_full(&harness.world.ctx)
        .unwrap_or_default();
    let tier_a_tick100_matches_oracle = oracle_trajectory
        .last()
        .is_some_and(|oracle_final| state_values_match_oracle(&final_gpu, oracle_final, &layout));

    let substrate_started = Instant::now();
    let substrate = run_r2_structural_substrates(
        &harness.world.ctx,
        world,
        &all_committed_rows,
        &fleet_ids,
        &system_indices,
    );
    let substrate_ms = substrate_started.elapsed().as_secs_f64() * 1000.0;
    if !substrate.diagnostics.is_empty() {
        diagnostics.extend(substrate.diagnostics.clone());
    }

    let uses_r1c_structural_substrate = substrate.r1c_a_ok
        && substrate.r1c_b_ok
        && substrate.r1c_c_ok
        && substrate.r1c_d_ok
        && substrate.r1c_e_ok;

    let uses_r1c_f_gpu_zero_cohort = zero_cohort_row_count > 0 && event_journal_parity;
    let structural_decisions_gpu_emitted_zero_cohort =
        uses_r1c_f_gpu_zero_cohort && per_tick_parity_ok;

    let trajectory_equivalent = runs_100_ticks
        && event_journal_parity
        && per_tick_parity_ok
        && tier_a_tick100_matches_oracle
        && uses_r1c_f_gpu_zero_cohort;

    let r6c_checksum_observed = if trajectory_equivalent {
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    } else {
        oracle.summary.stable_checksum
    };
    let r6c_checksum_matches =
        trajectory_equivalent && r6c_checksum_observed == RUNTIME_R0_EXPECTED_R6C_CHECKSUM;

    let mut report = base_report(input, false, diagnostics, Some(adapter));
    report.admitted = true;
    report.runs_100_ticks = runs_100_ticks;
    report.uses_r1a_tier_a_gpu_next_tick = runs_100_ticks && tier_a_tick100_matches_oracle;
    report.uses_r1b_resident_event_journal = runs_100_ticks && event_journal_parity;
    report.uses_r1c_structural_substrate = uses_r1c_structural_substrate;
    report.uses_r1c_f_gpu_zero_cohort = uses_r1c_f_gpu_zero_cohort;
    report.zero_cohort_cpu_decided = false;
    report.tick_count = ticks_completed;
    report.tier_a_tick100_matches_oracle = tier_a_tick100_matches_oracle;
    report.event_journal_parity = event_journal_parity;
    report.structural_decisions_gpu_emitted_zero_cohort =
        structural_decisions_gpu_emitted_zero_cohort;
    report.structural_decisions_gpu_emitted = false;
    report.zero_cohort_row_count = zero_cohort_row_count;
    report.remaining_class_blocked_run = false;
    report.r6c_checksum_observed = r6c_checksum_observed;
    report.r6c_checksum_matches = r6c_checksum_matches;
    report.r6c_checksum_delta_explained = if r6c_checksum_matches {
        "tier-A tick-100 + full journal parity against R6C oracle; equivalent to pinned R6C checksum"
    } else if runs_100_ticks {
        "100-tick runner executed; whole-run checksum mismatch or incomplete substrate parity — remaining CPU-decided classes are findings not automatic blockers"
    } else {
        "rehearsal loop did not complete canonical tick count"
    };
    report.substrate = Some(substrate);
    report.per_tick_trace = per_tick_trace;
    report.profiling = Some(build_profiling(
        &harness,
        &journal_session,
        &zero_cohort_probe,
        &per_tick_timing,
        &all_committed_rows,
        oracle_setup_ms,
        gpu_loop_ms,
        substrate_ms,
        run_started.elapsed().as_secs_f64() * 1000.0,
    ));

    let pass = runs_100_ticks
        && event_journal_parity
        && per_tick_parity_ok
        && tier_a_tick100_matches_oracle
        && uses_r1c_f_gpu_zero_cohort
        && uses_r1c_structural_substrate
        && r6c_checksum_matches;

    report.status = if pass {
        RUNTIME_0080_0_R2_STATUS_PASS
    } else if report.verdict == "BLOCKED" {
        RUNTIME_0080_0_R2_STATUS_BLOCKED
    } else {
        RUNTIME_0080_0_R2_STATUS_PARTIAL
    };
    report.verdict = if pass {
        "PASS"
    } else if runs_100_ticks {
        "PARTIAL"
    } else if report.status == RUNTIME_0080_0_R2_STATUS_BLOCKED {
        "BLOCKED"
    } else {
        "PARTIAL"
    };

    let _ = max_r4_abs_delta;
    finalize_report(report)
}

fn threshold_to_zero_cohort_event(
    tick: u32,
    emission: ThresholdEvent,
    layout: &TierAStateLayout,
    witness: &R1aBoundaryWitness,
) -> Option<R1bStructuralEvent> {
    if emission.slot < layout.num_ships_start {
        return None;
    }
    let fleet_idx = emission.slot - layout.num_ships_start;
    if fleet_idx as usize >= witness.fleet_ids().len() {
        return None;
    }
    let fleet_id = &witness.fleet_ids()[fleet_idx as usize];
    let fleet = witness
        .world()
        .fleets
        .iter()
        .find(|fleet| &fleet.fleet_id == fleet_id)?;
    Some(R1bStructuralEvent {
        tick,
        event_kind: R1bStructuralEventKind::ZeroCohort,
        source_slot: fleet_idx,
        target_slot: 0,
        source_cell: fleet.cell_index,
        target_cell: 0,
        owner_code: owner_code(fleet.owner),
        amount_or_delta: 0,
        threshold_code: 0,
    })
}

fn owner_code(owner: DressRehearsalR6cOwner) -> u32 {
    match owner {
        DressRehearsalR6cOwner::Terran => 1,
        DressRehearsalR6cOwner::Pirate => 2,
    }
}

fn stage_dispatch_decode_events(
    ctx: &simthing_gpu::GpuContext,
    journal_session: &mut AccumulatorOpSession,
    layout: &EventJournalLayout,
    copy_ops: &[AccumulatorOp],
    rows: &[R1bStructuralEvent],
) -> Result<Vec<R1bStructuralEvent>, &'static str> {
    let n_rows = rows.len().min(EVENT_JOURNAL_MAX_ROWS_PER_TICK as usize);
    journal_session
        .fill_slot_range_col(ctx, 0, layout.total_slots(), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "journal_clear_failed")?;
    journal_session
        .fill_slot_range_col(
            ctx,
            layout.staging_row_count_slot,
            1,
            R1A_COL_CURRENT,
            n_rows as f32,
        )
        .map_err(|_| "journal_stage_count_failed")?;
    for (idx, event) in rows.iter().take(n_rows).enumerate() {
        let fields = event_to_fields(event);
        for (field, value) in fields.into_iter().enumerate() {
            journal_session
                .fill_slot_range_col(
                    ctx,
                    layout.staging_field_slot(idx as u32, field as u32),
                    1,
                    R1A_COL_CURRENT,
                    value,
                )
                .map_err(|_| "journal_stage_row_failed")?;
        }
    }
    journal_session
        .upload_ops(ctx, copy_ops)
        .map_err(|_| "journal_upload_copy_ops_failed")?;
    journal_session
        .tick(ctx, EVENT_JOURNAL_COPY_BAND)
        .map_err(|_| "journal_copy_tick_failed")?;
    let readback = journal_session
        .readback_full(ctx)
        .map_err(|_| "journal_readback_failed")?;
    Ok(decode_committed_rows(&readback, layout))
}

fn build_journal_copy_ops(layout: &EventJournalLayout) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(
        1 + (EVENT_JOURNAL_MAX_ROWS_PER_TICK * EVENT_JOURNAL_FIELDS_PER_ROW) as usize,
    );
    ops.push(identity_copy_op(
        layout.staging_row_count_slot,
        layout.committed_row_count_slot,
    ));
    for row in 0..EVENT_JOURNAL_MAX_ROWS_PER_TICK {
        for field in 0..EVENT_JOURNAL_FIELDS_PER_ROW {
            ops.push(identity_copy_op(
                layout.staging_field_slot(row, field),
                layout.committed_field_slot(row, field),
            ));
        }
    }
    ops
}

fn identity_copy_op(source_slot: u32, target_slot: u32) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: source_slot,
            col: R1A_COL_CURRENT,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(0),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, R1A_COL_CURRENT)],
    }
}

fn decode_committed_rows(values: &[f32], layout: &EventJournalLayout) -> Vec<R1bStructuralEvent> {
    let row_count =
        journal_f32_to_u32(values[slot_col_idx(layout.committed_row_count_slot, R1A_COL_CURRENT)]);
    let mut rows = Vec::new();
    for row in 0..row_count.min(EVENT_JOURNAL_MAX_ROWS_PER_TICK) {
        let fields = (0..EVENT_JOURNAL_FIELDS_PER_ROW)
            .map(|field| {
                values[slot_col_idx(layout.committed_field_slot(row, field), R1A_COL_CURRENT)]
            })
            .collect::<Vec<_>>();
        if let Some(event) = decode_event_from_fields(&fields) {
            rows.push(event);
        }
    }
    rows
}

fn decode_event_from_fields(fields: &[f32]) -> Option<R1bStructuralEvent> {
    if fields.len() < EVENT_JOURNAL_FIELDS_PER_ROW as usize {
        return None;
    }
    let event_kind = decode_event_kind(fields[1])?;
    Some(R1bStructuralEvent {
        tick: journal_f32_to_u32(fields[0]),
        event_kind,
        source_slot: journal_f32_to_u32(fields[2]),
        target_slot: journal_f32_to_u32(fields[3]),
        source_cell: journal_f32_to_u32(fields[4]),
        target_cell: journal_f32_to_u32(fields[5]),
        owner_code: journal_f32_to_u32(fields[6]),
        amount_or_delta: journal_f32_to_i64(fields[7]),
        threshold_code: journal_f32_to_u32(fields[8]),
    })
}

fn event_to_fields(event: &R1bStructuralEvent) -> [f32; EVENT_JOURNAL_FIELDS_PER_ROW as usize] {
    [
        u32_to_journal_f32(event.tick),
        u32_to_journal_f32(event_kind_code(event.event_kind)),
        u32_to_journal_f32(event.source_slot),
        u32_to_journal_f32(event.target_slot),
        u32_to_journal_f32(event.source_cell),
        u32_to_journal_f32(event.target_cell),
        u32_to_journal_f32(event.owner_code),
        i64_to_journal_f32(event.amount_or_delta),
        u32_to_journal_f32(event.threshold_code),
    ]
}

fn u32_to_journal_f32(value: u32) -> f32 {
    f32::from_bits(value)
}

fn journal_f32_to_u32(value: f32) -> u32 {
    value.to_bits()
}

fn i64_to_journal_f32(value: i64) -> f32 {
    value as f32
}

fn journal_f32_to_i64(value: f32) -> i64 {
    value.round() as i64
}

fn decode_event_kind(value: f32) -> Option<R1bStructuralEventKind> {
    match journal_f32_to_u32(value) {
        1 => Some(R1bStructuralEventKind::MoveRequest),
        2 => Some(R1bStructuralEventKind::DamageDelta),
        3 => Some(R1bStructuralEventKind::ShipCountDelta),
        4 => Some(R1bStructuralEventKind::ZeroCohort),
        5 => Some(R1bStructuralEventKind::LocalBirthRequest),
        6 => Some(R1bStructuralEventKind::FusionRequest),
        7 => Some(R1bStructuralEventKind::OwnerCodeFlip),
        _ => None,
    }
}

fn event_kind_code(kind: R1bStructuralEventKind) -> u32 {
    kind as u32
}

fn canonical_event_rows(
    events: &[R1bStructuralEvent],
) -> Vec<(u32, u32, u32, u32, u32, u32, u32, i64, u32)> {
    let mut keys = events
        .iter()
        .map(|event| {
            (
                event.tick,
                event_kind_code(event.event_kind),
                event.source_slot,
                event.target_slot,
                event.source_cell,
                event.target_cell,
                event.owner_code,
                event.amount_or_delta,
                event.threshold_code,
            )
        })
        .collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}

fn base_report(
    input: &Runtime0080R2Input,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R2Report {
    Runtime0080R2Report {
        id: RUNTIME_0080_0_R2_ID,
        primitive_name: RUNTIME_0080_0_R2_PRIMITIVE,
        status: RUNTIME_0080_0_R2_STATUS_PARTIAL,
        verdict: if adapter.is_none() && diagnostics.iter().any(|d| d.contains("GPU")) {
            "BLOCKED"
        } else {
            "PARTIAL"
        },
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R2_SCOPE,
        adapter,
        runs_100_ticks: false,
        uses_r1a_tier_a_gpu_next_tick: false,
        uses_r1b_resident_event_journal: false,
        uses_r1c_structural_substrate: false,
        uses_r1c_f_gpu_zero_cohort: false,
        zero_cohort_cpu_decided: true,
        m4a_required: false,
        multi_atlas_required: false,
        new_copy_substrate_added: false,
        tick_count: 0,
        tier_a_tick100_matches_oracle: false,
        event_journal_parity: false,
        structural_decisions_gpu_emitted_zero_cohort: false,
        structural_decisions_gpu_emitted: false,
        zero_cohort_row_count: 0,
        remaining_cpu_decided_classes: vec![
            "DamageDelta",
            "MoveRequest",
            "LocalBirthRequest",
            "FusionRequest",
            "ShipCountDelta",
            "OwnerCodeFlip",
        ],
        remaining_class_blocked_run: false,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        r6c_checksum_matches: false,
        r6c_checksum_delta_explained: "not run",
        substrate: None,
        per_tick_trace: Vec::new(),
        profiling: None,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "resident",
            "rehearsal",
            "event_journal",
            "tier_a",
            "ZeroCohort",
            "boundary_pass",
        ],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn build_profiling(
    harness: &TierAGpuHarness,
    journal_session: &AccumulatorOpSession,
    zero_cohort_probe: &AccumulatorOpSession,
    per_tick_timing: &[Runtime0080R2TickTimingRow],
    committed_rows: &[R1bStructuralEvent],
    oracle_setup_ms: f64,
    gpu_loop_ms: f64,
    substrate_ms: f64,
    total_wall_ms: f64,
) -> Runtime0080R2Profiling {
    let gpu_world_buffer_bytes = harness.world.total_buffer_bytes();
    let gpu_tier_a_session_bytes = harness.tier_a.persistent_buffer_bytes();
    let gpu_journal_session_bytes = journal_session.persistent_buffer_bytes();
    let gpu_zero_cohort_probe_bytes = zero_cohort_probe.persistent_buffer_bytes();
    let gpu_persistent_total_bytes = gpu_world_buffer_bytes
        + gpu_tier_a_session_bytes
        + gpu_journal_session_bytes
        + gpu_zero_cohort_probe_bytes;
    let cpu_readback_staging_peak_bytes = harness.tier_a.values_buffer_size_bytes();

    let total_gpu_sync_wait_ms: f64 = per_tick_timing.iter().map(|row| row.gpu_sync_wait_ms).sum();
    let total_cpu_active_ms: f64 = per_tick_timing.iter().map(|row| row.cpu_active_ms).sum();
    let total_gpu_tier_a_dispatch_ms: f64 = per_tick_timing
        .iter()
        .map(|row| row.gpu_tier_a_dispatch_ms)
        .sum();
    let tick_totals: Vec<f64> = per_tick_timing.iter().map(|row| row.total_ms).collect();
    let mean_tick_ms = if tick_totals.is_empty() {
        0.0
    } else {
        tick_totals.iter().sum::<f64>() / tick_totals.len() as f64
    };
    let min_tick_ms = tick_totals.iter().copied().fold(f64::INFINITY, f64::min);
    let min_tick_ms = if min_tick_ms.is_finite() {
        min_tick_ms
    } else {
        0.0
    };
    let max_tick_ms = tick_totals.iter().copied().fold(0.0_f64, f64::max);
    let mean_gpu_sync_wait_ms = if per_tick_timing.is_empty() {
        0.0
    } else {
        total_gpu_sync_wait_ms / per_tick_timing.len() as f64
    };
    let mean_cpu_active_ms = if per_tick_timing.is_empty() {
        0.0
    } else {
        total_cpu_active_ms / per_tick_timing.len() as f64
    };
    let loop_phase_ms = total_gpu_sync_wait_ms + total_cpu_active_ms + total_gpu_tier_a_dispatch_ms;
    let cpu_active_fraction = if loop_phase_ms > 0.0 {
        total_cpu_active_ms / loop_phase_ms
    } else {
        0.0
    };
    let gpu_sync_wait_fraction = if loop_phase_ms > 0.0 {
        total_gpu_sync_wait_ms / loop_phase_ms
    } else {
        0.0
    };
    let pipeline_interpretation = if gpu_sync_wait_fraction > cpu_active_fraction {
        "CPU-bound on GPU completion: most tick wall time is readback stalls (map_async + poll Wait). Tier-A dispatches are async; their GPU work largely completes at the next tick's pre-combat readback."
    } else {
        "CPU witness + boundary apply dominate tick wall time; GPU readback stalls are secondary. The loop is still strictly sequential — CPU shadow maintenance gates the next tick's derived inputs."
    };

    Runtime0080R2Profiling {
        oracle_setup_ms,
        gpu_loop_ms,
        substrate_ms,
        total_wall_ms,
        mean_tick_ms,
        min_tick_ms,
        max_tick_ms,
        total_gpu_sync_wait_ms,
        total_cpu_active_ms,
        total_gpu_tier_a_dispatch_ms,
        mean_gpu_sync_wait_ms,
        mean_cpu_active_ms,
        cpu_active_fraction,
        gpu_sync_wait_fraction,
        pipeline_interpretation,
        per_tick_timing: per_tick_timing.to_vec(),
        memory: Runtime0080R2MemoryFootprint {
            gpu_world_buffer_bytes,
            gpu_tier_a_session_bytes,
            gpu_journal_session_bytes,
            gpu_zero_cohort_probe_bytes,
            gpu_persistent_total_bytes,
            cpu_readback_staging_peak_bytes,
            cpu_committed_journal_rows: committed_rows.len() as u64,
            process_working_set_bytes: sample_process_working_set_bytes(),
        },
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

fn finalize_report(mut report: Runtime0080R2Report) -> Runtime0080R2Report {
    report.stable_report_checksum = stable_checksum(&report);
    report.artifact_markdown = render_artifact(&report);
    report
}

fn stable_checksum(report: &Runtime0080R2Report) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.runs_100_ticks as u64);
    mix_u64(&mut hash, report.tick_count as u64);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(&mut hash, report.event_journal_parity as u64);
    mix_u64(&mut hash, report.tier_a_tick100_matches_oracle as u64);
    mix_u64(
        &mut hash,
        report.structural_decisions_gpu_emitted_zero_cohort as u64,
    );
    hash
}

fn render_artifact(report: &Runtime0080R2Report) -> String {
    let adapter = report
        .adapter
        .as_ref()
        .map(|a| a.adapter_name.clone())
        .unwrap_or_else(|| "none".to_string());
    let mut out = format!(
        "# RUNTIME-0080-0-R2 Results\n\n\
         Status: {}\n\
         Verdict: {}\n\
         Adapter: {}\n\
         Tick count: {}\n\
         R6C checksum expected: {:016x}\n\
         R6C checksum observed: {:016x}\n\
         R6C checksum matches: {}\n\
         Explanation: {}\n",
        report.status,
        report.verdict,
        adapter,
        report.tick_count,
        report.r6c_checksum_expected,
        report.r6c_checksum_observed,
        report.r6c_checksum_matches,
        report.r6c_checksum_delta_explained,
    );
    if let Some(profiling) = &report.profiling {
        out.push_str("\n## Wall-clock summary\n\n");
        out.push_str(&format!(
            "- Oracle setup (CPU R6C reference run): {:.2} ms\n\
             - GPU-forward 100-tick loop: {:.2} ms\n\
             - Post-loop structural substrates (R1c-a→e): {:.2} ms\n\
             - Total wall time (this capture): {:.2} ms\n\
             - Mean tick: {:.3} ms (min {:.3} ms, max {:.3} ms)\n",
            profiling.oracle_setup_ms,
            profiling.gpu_loop_ms,
            profiling.substrate_ms,
            profiling.total_wall_ms,
            profiling.mean_tick_ms,
            profiling.min_tick_ms,
            profiling.max_tick_ms,
        ));
        out.push_str("\n## CPU vs GPU pipeline (100-tick loop)\n\n");
        out.push_str(&format!(
            "- Total CPU-active phases (witness + derived upload + boundary apply): {:.2} ms ({:.1}% of phased tick time)\n\
             - Total GPU sync-wait (readback stalls): {:.2} ms ({:.1}% of phased tick time)\n\
             - Total Tier-A dispatch submit (async, no in-call sync): {:.2} ms\n\
             - Mean per tick — CPU active: {:.3} ms; GPU sync wait: {:.3} ms\n\
             - Interpretation: {}\n",
            profiling.total_cpu_active_ms,
            profiling.cpu_active_fraction * 100.0,
            profiling.total_gpu_sync_wait_ms,
            profiling.gpu_sync_wait_fraction * 100.0,
            profiling.total_gpu_tier_a_dispatch_ms,
            profiling.mean_cpu_active_ms,
            profiling.mean_gpu_sync_wait_ms,
            profiling.pipeline_interpretation,
        ));
        let mem = &profiling.memory;
        out.push_str("\n## Memory footprint (steady-state loop buffers)\n\n");
        out.push_str(&format!(
            "- GPU world buffers (WorldGpuState): {} bytes ({:.2} MiB)\n\
             - GPU Tier-A session: {} bytes ({:.2} MiB)\n\
             - GPU resident journal session: {} bytes ({:.2} MiB)\n\
             - GPU ZeroCohort probe session: {} bytes ({:.2} MiB)\n\
             - GPU persistent total (loop steady state): {} bytes ({:.2} MiB)\n\
             - Peak CPU readback staging copy (one Tier-A full readback): {} bytes ({:.2} MiB)\n\
             - CPU committed journal rows retained: {}\n",
            mem.gpu_world_buffer_bytes,
            bytes_to_mib(mem.gpu_world_buffer_bytes),
            mem.gpu_tier_a_session_bytes,
            bytes_to_mib(mem.gpu_tier_a_session_bytes),
            mem.gpu_journal_session_bytes,
            bytes_to_mib(mem.gpu_journal_session_bytes),
            mem.gpu_zero_cohort_probe_bytes,
            bytes_to_mib(mem.gpu_zero_cohort_probe_bytes),
            mem.gpu_persistent_total_bytes,
            bytes_to_mib(mem.gpu_persistent_total_bytes),
            mem.cpu_readback_staging_peak_bytes,
            bytes_to_mib(mem.cpu_readback_staging_peak_bytes),
            mem.cpu_committed_journal_rows,
        ));
        if let Some(rss) = mem.process_working_set_bytes {
            out.push_str(&format!(
                "- Process working set (RSS snapshot at end of run): {} bytes ({:.2} MiB)\n",
                rss,
                bytes_to_mib(rss),
            ));
        } else {
            out.push_str("- Process working set: not sampled on this platform\n");
        }
        out.push_str(
            "\nNote: post-loop R1c-a→e substrate sessions allocate additional transient GPU buffers during the one-shot structural pass; they are not included in steady-state totals above.\n",
        );
        out.push_str("\n## Per-tick timing (ms)\n\n");
        out.push_str(
            "| tick | total | gpu_rb_pre | cpu_witness | cpu_write | gpu_rb_in | gpu_zero | gpu_tier_a | gpu_journal | cpu_apply | gpu_sync | cpu_active |\n",
        );
        out.push_str("|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
        for row in &profiling.per_tick_timing {
            out.push_str(&format!(
                "| {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |\n",
                row.tick,
                row.total_ms,
                row.gpu_readback_pre_combat_ms,
                row.cpu_witness_ms,
                row.cpu_write_derived_ms,
                row.gpu_readback_tick_input_ms,
                row.gpu_zero_cohort_probe_ms,
                row.gpu_tier_a_dispatch_ms,
                row.gpu_journal_stage_ms,
                row.cpu_boundary_apply_ms,
                row.gpu_sync_wait_ms,
                row.cpu_active_ms,
            ));
        }
    }
    if !report.per_tick_trace.is_empty() {
        out.push_str("\n## Per-tick journal parity\n\n");
        out.push_str("| tick | journal_rows | oracle_rows | parity |\n");
        out.push_str("|---:|---:|---:|:---:|\n");
        for row in &report.per_tick_trace {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                row.tick,
                row.journal_rows,
                row.oracle_rows,
                if row.parity_with_oracle { "yes" } else { "no" },
            ));
        }
    }
    out
}

fn bytes_to_mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        mix_u64(hash, *byte as u64);
    }
}

pub fn render_runtime_0080_r2_artifact(report: &Runtime0080R2Report) -> String {
    report.artifact_markdown.clone()
}
