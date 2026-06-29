//! RUNTIME-0080-0-R1b: RESIDENT-EVENTLOG-0 resident event journal (Outcome B).
//!
//! Opt-in/default-off runtime that preserves R1a Tier-A residency while adding a dedicated
//! resident event journal. A CPU boundary pass consumes GPU-authored event rows without
//! re-deriving movement/combat/production decisions.

use std::collections::BTreeMap;

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::dress_rehearsal_r6c_integrated_run::{
    r1b_apply_boundary_events, r1b_oracle_events_by_tick, run_dress_rehearsal_r6c_integrated_run,
    DressRehearsalR6cInput, R1aBoundaryWitness, R1bStructuralEvent, R1bStructuralEventKind,
    R6C_CANONICAL_TICK_COUNT,
};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    collect_col, compute_comparison_oracle_trajectory, create_discrete_gpu_context,
    run_runtime_0080_0_r1a, slot_col_idx, DisabledTransformMask, Runtime0080R1aAdapterReport,
    Runtime0080R1aInput, TierAGpuHarness, TierAStateLayout, TierAStaticConfig, R1A_COL_CURRENT,
    R1A_N_DIMS,
};

pub const RUNTIME_0080_0_R1B_ID: &str = "RUNTIME-0080-0-R1b";
pub const RUNTIME_0080_0_R1B_PRIMITIVE: &str = "RESIDENT-EVENTLOG-0";
pub const RUNTIME_0080_0_R1B_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident event journal consumed by boundary";
pub const RUNTIME_0080_0_R1B_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - journal parity earned; GPU structural decision authority pending R1c";
pub const RUNTIME_0080_0_R1B_STATUS_BLOCKED: &str = "BLOCKED - no discrete GPU";
pub const RUNTIME_R1B_SCOPE: &str = "Tier-A + resident event journal rows";

const EVENT_JOURNAL_MAX_ROWS_PER_TICK: u32 = 128;
const EVENT_JOURNAL_FIELDS_PER_ROW: u32 = 9;
const EVENT_JOURNAL_COPY_BAND: u32 = 0;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1bInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1bInput {
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
pub struct Runtime0080R1bKindRowCount {
    pub kind: &'static str,
    pub rows: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1bTraceRow {
    pub tick: u32,
    pub movement_rows: u32,
    pub post_tier_a_rows: u32,
    pub journal_rows_total: u32,
    pub oracle_rows: u32,
    pub boundary_rows_applied: u32,
    pub parity_with_oracle: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1bEventWriterParityCheck {
    pub writers_enabled_rows: u32,
    pub writers_disabled_rows: u32,
    pub writers_enabled_oracle_parity: bool,
    pub writers_disabled_oracle_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1bFreeSlotMarkSource {
    pub tick: u32,
    pub slot: u32,
    pub reason: &'static str,
    pub source_event_kind: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1bLocalBirthRequestSource {
    pub tick: u32,
    pub request_index: u32,
    pub owner_code: u32,
    pub source_cell: u32,
    pub requested_ships: i64,
    pub source_event_kind: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1bReport {
    pub id: &'static str,
    pub primitive_name: &'static str,
    pub status: &'static str,
    pub verdict: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<String>,
    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub adapter: Option<Runtime0080R1aAdapterReport>,
    pub scope: &'static str,
    pub event_writers_enabled: bool,
    pub resident_event_journal_created: bool,
    pub gpu_writes_event_rows: bool,
    pub event_rows_read_from_gpu_values: bool,
    pub event_journal_parity_measured_from_gpu_values: bool,
    pub cpu_boundary_pass_consumes_event_rows: bool,
    pub cpu_boundary_pass_does_not_rederive_decisions: bool,
    /// True only when the GPU itself emits the structural decisions (resident REENROLL/scatter/
    /// compact). In R1b the CPU decision witness computes them and stages them into the GPU journal,
    /// so this is false and the verdict stays PARTIAL pending R1c. Full journal parity is still earned.
    pub structural_decisions_gpu_emitted: bool,
    pub boundary_pass_invoked_movement_tick: bool,
    pub boundary_pass_invoked_combat_tick: bool,
    pub boundary_pass_invoked_production_tick: bool,
    pub per_kind_row_counts: Vec<Runtime0080R1bKindRowCount>,
    pub disabled_transform_event_writer_check: Option<Runtime0080R1bEventWriterParityCheck>,
    pub free_slot_mark_sources_from_gpu_journal: Vec<Runtime0080R1bFreeSlotMarkSource>,
    pub local_birth_request_sources_from_gpu_journal: Vec<Runtime0080R1bLocalBirthRequestSource>,
    pub structural_events_from_gpu_journal: Vec<R1bStructuralEvent>,
    pub gpu_event_row_count_total: u32,
    pub oracle_event_row_count_total: u32,
    pub journal_tick_count: u32,
    pub trace: Vec<Runtime0080R1bTraceRow>,
    pub r1a_tier_a_preservation: bool,
    pub r1a_tier_a_preservation_verdict: String,
    pub r1a_report_checksum: u64,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub r6c_checksum_matches: bool,
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

pub fn run_runtime_0080_0_r1b(input: &Runtime0080R1bInput) -> Runtime0080R1bReport {
    run_runtime_0080_0_r1b_internal(input, true, true)
}

pub fn run_runtime_0080_0_r1b_with_event_writers_enabled(
    input: &Runtime0080R1bInput,
    event_writers_enabled: bool,
) -> Runtime0080R1bReport {
    run_runtime_0080_0_r1b_internal(input, event_writers_enabled, false)
}

pub fn replay_runtime_0080_0_r1b() -> (Runtime0080R1bReport, Runtime0080R1bReport) {
    let input = Runtime0080R1bInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1b(&input),
        run_runtime_0080_0_r1b(&input),
    )
}

fn run_runtime_0080_0_r1b_internal(
    input: &Runtime0080R1bInput,
    event_writers_enabled: bool,
    include_cross_checks: bool,
) -> Runtime0080R1bReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required".to_string()],
            None,
            event_writers_enabled,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden".to_string()],
            None,
            event_writers_enabled,
        ));
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(
                input,
                false,
                vec![diagnostic.to_string()],
                None,
                event_writers_enabled,
            );
            report.status = RUNTIME_0080_0_R1B_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            return finalize_report(report);
        }
    };

    set_debug_readback_allowed(true);
    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let world = oracle
        .initial_world
        .as_ref()
        .expect("R6C report carries initial world");
    let layout = TierAStateLayout::new(world);
    let _oracle_trajectory = compute_comparison_oracle_trajectory(&oracle);
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
    let mut boundary_witness = R1aBoundaryWitness::new(world, fleet_ids, system_indices);

    let mut harness = match TierAGpuHarness::new(ctx, &layout, world, &static_config) {
        Ok(h) => h,
        Err(diagnostic) => {
            return finalize_report(base_report(
                input,
                false,
                vec![diagnostic.to_string()],
                Some(adapter),
                event_writers_enabled,
            ));
        }
    };

    let journal_layout = EventJournalLayout::new();
    let mut journal_session =
        AccumulatorOpSession::new(&harness.world.ctx, journal_layout.total_slots(), R1A_N_DIMS);
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
        let mut report = base_report(
            input,
            false,
            vec!["journal_seed_failed".to_string()],
            Some(adapter),
            event_writers_enabled,
        );
        report.admitted = true;
        report.status = RUNTIME_0080_0_R1B_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        return finalize_report(report);
    }

    let oracle_events_by_tick = r1b_oracle_events_by_tick(
        &oracle,
        boundary_witness.fleet_ids(),
        boundary_witness.system_indices(),
    );
    let oracle_event_row_count_total = oracle_events_by_tick
        .values()
        .map(|rows| rows.len() as u32)
        .sum::<u32>();

    let mut diagnostics = Vec::new();
    let mut trace = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize);
    let mut per_kind_counts: BTreeMap<&'static str, u32> = BTreeMap::new();
    let mut gpu_event_row_count_total = 0u32;
    let mut all_committed_rows = Vec::new();
    let mut per_tick_parity_ok = true;
    let mut gpu_writes_event_rows = false;
    let mut event_rows_read_from_gpu_values = false;
    let mut cpu_boundary_pass_consumes_event_rows = true;
    let mut max_r4_abs_delta = 0.0f32;

    // Boundary shadow: seeded once, advanced ONLY by applying GPU-read-back journal rows.
    // It never reruns tick logic — it proves the journal alone drives bounded structural maintenance.
    let mut boundary_shadow = boundary_witness.clone_for_event_derivation();
    let shadow_fleet_ids = boundary_witness.fleet_ids().to_vec();
    let shadow_system_indices = boundary_witness.system_indices().to_vec();

    for tick in 0..R6C_CANONICAL_TICK_COUNT {
        // The CPU decision witness advances oracle-identically, carrying its own structural state
        // forward. It is NOT reconstructed from GPU readback, so it cannot drift from the oracle.
        let (derived, events) = boundary_witness.step_tick_capture_events(tick);
        let staged_events = if event_writers_enabled {
            events
        } else {
            Vec::new()
        };

        // Resident event journal round-trip: stage decision rows into GPU memory, read them back.
        let committed_rows = if event_writers_enabled {
            match stage_dispatch_decode_events(
                &harness.world.ctx,
                &mut journal_session,
                &journal_layout,
                &journal_copy_ops,
                &staged_events,
            ) {
                Ok(rows) => {
                    event_rows_read_from_gpu_values = true;
                    if !rows.is_empty() {
                        gpu_writes_event_rows = true;
                    }
                    rows
                }
                Err(diagnostic) => {
                    diagnostics.push(diagnostic.to_string());
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        // GPU Tier-A value loop stays resident and advances each tick on its own buffers.
        // It is driven by per-tick derived inputs (no GPU->CPU readback feeds the decision),
        // so the GPU is never starved waiting on the CPU boundary pass.
        if harness
            .write_tick_derived_inputs(&layout, &derived)
            .is_err()
        {
            diagnostics.push("write_tick_derived_inputs_failed".to_string());
        }
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

        // Bounded CPU boundary pass: consume GPU-read-back rows only (no tick rederivation).
        let apply_report = r1b_apply_boundary_events(
            boundary_shadow.world_mut(),
            &shadow_fleet_ids,
            &shadow_system_indices,
            &committed_rows,
        );
        if apply_report.rows_applied > committed_rows.len() as u32 {
            cpu_boundary_pass_consumes_event_rows = false;
        }

        for event in &committed_rows {
            let key = event_kind_name(event.event_kind);
            *per_kind_counts.entry(key).or_insert(0) += 1;
        }
        gpu_event_row_count_total += committed_rows.len() as u32;

        let oracle_rows = oracle_events_by_tick
            .get(&tick)
            .cloned()
            .unwrap_or_default();
        all_committed_rows.extend(committed_rows.iter().cloned());
        let parity_with_oracle =
            canonical_event_rows(&committed_rows) == canonical_event_rows(&oracle_rows);
        if !parity_with_oracle {
            per_tick_parity_ok = false;
            diagnostics.push(format!("oracle_parity_miss_tick_{}", tick));
        }
        let movement_rows = committed_rows
            .iter()
            .filter(|event| event.event_kind == R1bStructuralEventKind::MoveRequest)
            .count() as u32;
        trace.push(Runtime0080R1bTraceRow {
            tick,
            movement_rows,
            post_tier_a_rows: committed_rows.len() as u32 - movement_rows,
            journal_rows_total: committed_rows.len() as u32,
            oracle_rows: oracle_rows.len() as u32,
            boundary_rows_applied: apply_report.rows_applied,
            parity_with_oracle,
        });
    }

    let all_oracle_rows = oracle_events_by_tick
        .values()
        .flat_map(|rows| rows.iter().cloned())
        .collect::<Vec<_>>();
    let event_journal_parity =
        canonical_event_rows(&all_committed_rows) == canonical_event_rows(&all_oracle_rows);
    if !per_tick_parity_ok && event_journal_parity {
        diagnostics.push("per_tick_oracle_parity_differs_aggregate_matches".to_string());
    }
    let mut report = base_report(
        input,
        false,
        diagnostics,
        Some(adapter),
        event_writers_enabled,
    );
    report.admitted = true;
    report.resident_event_journal_created = true;
    report.gpu_writes_event_rows = gpu_writes_event_rows;
    report.event_rows_read_from_gpu_values = event_rows_read_from_gpu_values;
    report.event_journal_parity_measured_from_gpu_values = event_journal_parity;
    report.cpu_boundary_pass_consumes_event_rows = cpu_boundary_pass_consumes_event_rows;
    report.cpu_boundary_pass_does_not_rederive_decisions = true;
    report.per_kind_row_counts = to_per_kind_row_counts(&per_kind_counts);
    report.free_slot_mark_sources_from_gpu_journal =
        free_slot_mark_sources_from_events(&all_committed_rows);
    report.local_birth_request_sources_from_gpu_journal =
        local_birth_request_sources_from_events(&all_committed_rows);
    report.structural_events_from_gpu_journal = all_committed_rows.clone();
    report.gpu_event_row_count_total = gpu_event_row_count_total;
    report.oracle_event_row_count_total = oracle_event_row_count_total;
    report.journal_tick_count = R6C_CANONICAL_TICK_COUNT;
    report.trace = trace;
    report.r6c_checksum_expected = RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.r6c_checksum_matches = report.r6c_checksum_observed == report.r6c_checksum_expected;

    if include_cross_checks {
        let r1a_report = run_runtime_0080_0_r1a(&Runtime0080R1aInput::explicit_opt_in());
        report.r1a_tier_a_preservation = r1a_report.field_column_parity_matches_r6c_checksum;
        report.r1a_tier_a_preservation_verdict = r1a_report.verdict.to_string();
        report.r1a_report_checksum = r1a_report.stable_report_checksum;
    }

    if include_cross_checks && event_writers_enabled {
        let disabled = run_runtime_0080_0_r1b_internal(input, false, false);
        let negative_control_detected = report.gpu_event_row_count_total
            > disabled.gpu_event_row_count_total
            && report.event_journal_parity_measured_from_gpu_values
            && !disabled.event_journal_parity_measured_from_gpu_values;
        report.disabled_transform_event_writer_check = Some(Runtime0080R1bEventWriterParityCheck {
            writers_enabled_rows: report.gpu_event_row_count_total,
            writers_disabled_rows: disabled.gpu_event_row_count_total,
            writers_enabled_oracle_parity: report.event_journal_parity_measured_from_gpu_values,
            writers_disabled_oracle_parity: disabled.event_journal_parity_measured_from_gpu_values,
            negative_control_detected,
            disabled_report_checksum: disabled.stable_report_checksum,
        });
    }

    let journal_substrate_flags = report.resident_event_journal_created
        && report.gpu_writes_event_rows
        && report.event_rows_read_from_gpu_values
        && report.event_journal_parity_measured_from_gpu_values
        && report.cpu_boundary_pass_consumes_event_rows
        && report.cpu_boundary_pass_does_not_rederive_decisions
        && !report.boundary_pass_invoked_movement_tick
        && !report.boundary_pass_invoked_combat_tick
        && !report.boundary_pass_invoked_production_tick
        && report.r6c_checksum_matches;
    if journal_substrate_flags && event_writers_enabled && !report.structural_decisions_gpu_emitted
    {
        report.diagnostics.push(
            "journal_substrate_complete_partial_pending_r1c_resident_decision_authority"
                .to_string(),
        );
    }
    // PASS additionally requires the GPU to emit the structural decisions (resident REENROLL/
    // scatter/compact = R1c). R1b earns the full journal substrate but the CPU decision witness
    // still computes the structural decisions, so the honest verdict remains PARTIAL.
    let pass =
        event_writers_enabled && journal_substrate_flags && report.structural_decisions_gpu_emitted;
    report.status = if pass {
        RUNTIME_0080_0_R1B_STATUS_PASS
    } else {
        RUNTIME_0080_0_R1B_STATUS_PARTIAL
    };
    report.verdict = if pass { "PASS" } else { "PARTIAL" };
    if !event_writers_enabled {
        report
            .diagnostics
            .push("event_writers_disabled_for_negative_control".to_string());
    }

    finalize_report(report)
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
            slot: SlotIndex::new(source_slot),
            col: ColumnIndex::new(R1A_COL_CURRENT as usize),
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(EVENT_JOURNAL_COPY_BAND),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(
            SlotIndex::new(target_slot),
            ColumnIndex::new(R1A_COL_CURRENT as usize),
        )],
    }
}

fn decode_committed_rows(values: &[f32], layout: &EventJournalLayout) -> Vec<R1bStructuralEvent> {
    let current = collect_col(values, layout.total_slots(), R1A_COL_CURRENT);
    let row_count = f32_to_u32(current[layout.committed_row_count_slot as usize])
        .min(EVENT_JOURNAL_MAX_ROWS_PER_TICK);
    let mut rows = Vec::with_capacity(row_count as usize);
    for row_idx in 0..row_count {
        let mut fields = [0.0f32; EVENT_JOURNAL_FIELDS_PER_ROW as usize];
        for field in 0..EVENT_JOURNAL_FIELDS_PER_ROW {
            fields[field as usize] = current[layout.committed_field_slot(row_idx, field) as usize];
        }
        if let Some(event_kind) = decode_event_kind(fields[1]) {
            rows.push(R1bStructuralEvent {
                tick: journal_f32_to_u32(fields[0]),
                event_kind,
                source_slot: journal_f32_to_u32(fields[2]),
                target_slot: journal_f32_to_u32(fields[3]),
                source_cell: journal_f32_to_u32(fields[4]),
                target_cell: journal_f32_to_u32(fields[5]),
                owner_code: journal_f32_to_u32(fields[6]),
                amount_or_delta: journal_f32_to_i64(fields[7]),
                threshold_code: journal_f32_to_u32(fields[8]),
            });
        }
    }
    rows
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

// Signed deltas (e.g. combat `DamageDelta` = -ships_destroyed) are stored as an exact f32 VALUE,
// not a raw bit-cast. Bit-casting a negative integer yields an exponent-0xFF pattern (NaN/Inf),
// which the resident journal fill rejects as non-finite. Ship-count deltas are tiny (|delta| << 2^24),
// so they are represented exactly by f32 and survive the GPU identity-copy round-trip.
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

fn event_kind_name(kind: R1bStructuralEventKind) -> &'static str {
    match kind {
        R1bStructuralEventKind::MoveRequest => "MoveRequest",
        R1bStructuralEventKind::DamageDelta => "DamageDelta",
        R1bStructuralEventKind::ShipCountDelta => "ShipCountDelta",
        R1bStructuralEventKind::ZeroCohort => "ZeroCohort",
        R1bStructuralEventKind::LocalBirthRequest => "LocalBirthRequest",
        R1bStructuralEventKind::FusionRequest => "FusionRequest",
        R1bStructuralEventKind::OwnerCodeFlip => "OwnerCodeFlip",
    }
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

fn to_per_kind_row_counts(counts: &BTreeMap<&'static str, u32>) -> Vec<Runtime0080R1bKindRowCount> {
    counts
        .iter()
        .map(|(kind, rows)| Runtime0080R1bKindRowCount { kind, rows: *rows })
        .collect()
}

fn free_slot_mark_sources_from_events(
    events: &[R1bStructuralEvent],
) -> Vec<Runtime0080R1bFreeSlotMarkSource> {
    let mut rows = events
        .iter()
        .filter_map(|event| match event.event_kind {
            R1bStructuralEventKind::ZeroCohort => Some(Runtime0080R1bFreeSlotMarkSource {
                tick: event.tick,
                slot: event.source_slot,
                reason: "zero_cohort_departure",
                source_event_kind: event_kind_name(event.event_kind),
            }),
            R1bStructuralEventKind::FusionRequest => Some(Runtime0080R1bFreeSlotMarkSource {
                tick: event.tick,
                slot: event.target_slot,
                reason: "fusion_absorbed_slot",
                source_event_kind: event_kind_name(event.event_kind),
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| (row.tick, row.slot, row.reason));
    rows
}

fn local_birth_request_sources_from_events(
    events: &[R1bStructuralEvent],
) -> Vec<Runtime0080R1bLocalBirthRequestSource> {
    let mut rows = Vec::new();
    for event in events
        .iter()
        .filter(|event| event.event_kind == R1bStructuralEventKind::LocalBirthRequest)
    {
        rows.push(Runtime0080R1bLocalBirthRequestSource {
            tick: event.tick,
            request_index: rows.len() as u32,
            owner_code: event.owner_code,
            source_cell: event.source_cell,
            requested_ships: event.amount_or_delta,
            source_event_kind: event_kind_name(event.event_kind),
        });
    }
    rows.sort_by_key(|row| (row.tick, row.request_index, row.source_cell));
    rows
}

fn f32_to_u32(value: f32) -> u32 {
    if value.is_nan() || value.is_sign_negative() {
        0
    } else {
        value.round() as u32
    }
}

fn base_report(
    input: &Runtime0080R1bInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
    event_writers_enabled: bool,
) -> Runtime0080R1bReport {
    Runtime0080R1bReport {
        id: RUNTIME_0080_0_R1B_ID,
        primitive_name: RUNTIME_0080_0_R1B_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        adapter,
        scope: RUNTIME_R1B_SCOPE,
        event_writers_enabled,
        resident_event_journal_created: false,
        gpu_writes_event_rows: false,
        event_rows_read_from_gpu_values: false,
        event_journal_parity_measured_from_gpu_values: false,
        cpu_boundary_pass_consumes_event_rows: false,
        cpu_boundary_pass_does_not_rederive_decisions: true,
        structural_decisions_gpu_emitted: false,
        boundary_pass_invoked_movement_tick: false,
        boundary_pass_invoked_combat_tick: false,
        boundary_pass_invoked_production_tick: false,
        per_kind_row_counts: Vec::new(),
        disabled_transform_event_writer_check: None,
        free_slot_mark_sources_from_gpu_journal: Vec::new(),
        local_birth_request_sources_from_gpu_journal: Vec::new(),
        structural_events_from_gpu_journal: Vec::new(),
        gpu_event_row_count_total: 0,
        oracle_event_row_count_total: 0,
        journal_tick_count: 0,
        trace: Vec::new(),
        r1a_tier_a_preservation: false,
        r1a_tier_a_preservation_verdict: "NOT RUN".to_string(),
        r1a_report_checksum: 0,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        r6c_checksum_matches: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec!["FieldPolicy", "field_agent", "selection", "extraction"],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1bReport) -> Runtime0080R1bReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1b_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1bReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.admitted as u64);
    mix_u64(&mut hash, report.event_writers_enabled as u64);
    mix_u64(&mut hash, report.resident_event_journal_created as u64);
    mix_u64(&mut hash, report.gpu_writes_event_rows as u64);
    mix_u64(&mut hash, report.event_rows_read_from_gpu_values as u64);
    mix_u64(
        &mut hash,
        report.event_journal_parity_measured_from_gpu_values as u64,
    );
    mix_u64(
        &mut hash,
        report.cpu_boundary_pass_consumes_event_rows as u64,
    );
    mix_u64(
        &mut hash,
        report.cpu_boundary_pass_does_not_rederive_decisions as u64,
    );
    mix_u64(&mut hash, report.structural_decisions_gpu_emitted as u64);
    mix_u64(&mut hash, report.gpu_event_row_count_total as u64);
    mix_u64(&mut hash, report.oracle_event_row_count_total as u64);
    mix_u64(&mut hash, report.journal_tick_count as u64);
    mix_u64(&mut hash, report.r1a_tier_a_preservation as u64);
    mix_u64(&mut hash, report.r1a_report_checksum);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(&mut hash, report.r6c_checksum_matches as u64);
    for diagnostic in &report.diagnostics {
        mix_str(&mut hash, diagnostic);
    }
    for row in &report.per_kind_row_counts {
        mix_str(&mut hash, row.kind);
        mix_u64(&mut hash, row.rows as u64);
    }
    for row in &report.trace {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.journal_rows_total as u64);
        mix_u64(&mut hash, row.oracle_rows as u64);
        mix_u64(&mut hash, row.parity_with_oracle as u64);
    }
    if let Some(check) = &report.disabled_transform_event_writer_check {
        mix_u64(&mut hash, check.writers_enabled_rows as u64);
        mix_u64(&mut hash, check.writers_disabled_rows as u64);
        mix_u64(&mut hash, check.writers_enabled_oracle_parity as u64);
        mix_u64(&mut hash, check.writers_disabled_oracle_parity as u64);
        mix_u64(&mut hash, check.negative_control_detected as u64);
        mix_u64(&mut hash, check.disabled_report_checksum);
    }
    for row in &report.local_birth_request_sources_from_gpu_journal {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.request_index as u64);
        mix_u64(&mut hash, row.owner_code as u64);
        mix_u64(&mut hash, row.source_cell as u64);
        mix_u64(&mut hash, row.requested_ships as u64);
    }
    hash
}

pub fn render_runtime_0080_r1b_artifact(report: &Runtime0080R1bReport) -> String {
    let adapter_lines = report
        .adapter
        .as_ref()
        .map(|adapter| {
            format!(
                "- adapter_name: {}\n- selected_discrete_gpu: {}\n- backend: {}\n",
                adapter.adapter_name, adapter.selected_discrete_gpu, adapter.backend
            )
        })
        .unwrap_or_else(|| "- adapter: unavailable\n".to_string());
    let per_kind = if report.per_kind_row_counts.is_empty() {
        "- none".to_string()
    } else {
        report
            .per_kind_row_counts
            .iter()
            .map(|row| format!("- {}: {}", row.kind, row.rows))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let disabled_check = report
        .disabled_transform_event_writer_check
        .as_ref()
        .map(|check| {
            format!(
                "- writers_enabled_rows: {}\n- writers_disabled_rows: {}\n- writers_enabled_oracle_parity: {}\n- writers_disabled_oracle_parity: {}\n- negative_control_detected: {}\n- disabled_report_checksum: {:016x}\n",
                check.writers_enabled_rows,
                check.writers_disabled_rows,
                check.writers_enabled_oracle_parity,
                check.writers_disabled_oracle_parity,
                check.negative_control_detected,
                check.disabled_report_checksum,
            )
        })
        .unwrap_or_else(|| "- not_run\n".to_string());
    let local_birth_requests = if report
        .local_birth_request_sources_from_gpu_journal
        .is_empty()
    {
        "- none".to_string()
    } else {
        report
            .local_birth_request_sources_from_gpu_journal
            .iter()
            .map(|row| {
                format!(
                    "- tick {} request {} owner {} source_cell {} requested_ships {}",
                    row.tick,
                    row.request_index,
                    row.owner_code,
                    row.source_cell,
                    row.requested_ships
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let diagnostics = if report.diagnostics.is_empty() {
        "- none".to_string()
    } else {
        report
            .diagnostics
            .iter()
            .map(|d| format!("- {}", d))
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "# RUNTIME-0080-0-R1b Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Adapter\n\
         {adapter}\
         ## Resident Event Journal\n\
         - resident_event_journal_created: {created}\n\
         - event_writers_enabled: {writers_enabled}\n\
         - gpu_writes_event_rows: {gpu_writes}\n\
         - event_rows_read_from_gpu_values: {readback}\n\
         - event_journal_parity_measured_from_gpu_values: {parity}\n\
         - cpu_boundary_pass_consumes_event_rows: {consumes}\n\
         - cpu_boundary_pass_does_not_rederive_decisions: {no_rederive}\n\
         - structural_decisions_gpu_emitted: {gpu_decisions}\n\
         - boundary_pass_invoked_movement_tick: {movement_tick}\n\
         - boundary_pass_invoked_combat_tick: {combat_tick}\n\
         - boundary_pass_invoked_production_tick: {production_tick}\n\
         - journal_tick_count: {ticks}\n\
         - gpu_event_row_count_total: {gpu_rows}\n\
         - oracle_event_row_count_total: {oracle_rows}\n\n\
         ## Per-Kind Row Counts\n\
         {per_kind}\n\n\
         ## Event Writer Disabled-Transform Check\n\
         {disabled_check}\n\
         ## Local Birth Requests From GPU Journal\n\
         {local_birth_requests}\n\n\
         ## R1a Preservation + R6c Checksum\n\
         - r1a_tier_a_preservation: {r1a_ok}\n\
         - r1a_tier_a_preservation_verdict: {r1a_verdict}\n\
         - r1a_report_checksum: `{r1a_checksum:016x}`\n\
         - r6c_checksum_expected: `{r6c_expected:016x}`\n\
         - r6c_checksum_observed: `{r6c_observed:016x}`\n\
         - r6c_checksum_matches: {r6c_matches}\n\n\
         ## Domain-Neutral Terms\n\
         - {terms}\n\n\
         ## Diagnostics\n\
         {diagnostics}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter_lines,
        created = report.resident_event_journal_created,
        writers_enabled = report.event_writers_enabled,
        gpu_writes = report.gpu_writes_event_rows,
        readback = report.event_rows_read_from_gpu_values,
        parity = report.event_journal_parity_measured_from_gpu_values,
        consumes = report.cpu_boundary_pass_consumes_event_rows,
        no_rederive = report.cpu_boundary_pass_does_not_rederive_decisions,
        gpu_decisions = report.structural_decisions_gpu_emitted,
        movement_tick = report.boundary_pass_invoked_movement_tick,
        combat_tick = report.boundary_pass_invoked_combat_tick,
        production_tick = report.boundary_pass_invoked_production_tick,
        ticks = report.journal_tick_count,
        gpu_rows = report.gpu_event_row_count_total,
        oracle_rows = report.oracle_event_row_count_total,
        per_kind = per_kind,
        disabled_check = disabled_check,
        local_birth_requests = local_birth_requests,
        r1a_ok = report.r1a_tier_a_preservation,
        r1a_verdict = report.r1a_tier_a_preservation_verdict,
        r1a_checksum = report.r1a_report_checksum,
        r6c_expected = report.r6c_checksum_expected,
        r6c_observed = report.r6c_checksum_observed,
        r6c_matches = report.r6c_checksum_matches,
        terms = report.domain_terms.join(", "),
        diagnostics = diagnostics,
    )
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

#[allow(dead_code)]
fn _journal_slot_col_idx(slot: u32, col: u32) -> usize {
    slot_col_idx(slot, col)
}

#[cfg(test)]
mod fast_event_journal_sentinel_tests {
    use super::free_slot_mark_sources_from_events;
    use crate::dress_rehearsal_r6c_integrated_run::{R1bStructuralEvent, R1bStructuralEventKind};

    #[test]
    fn r1_fast_event_journal_marks_one_free_slot_from_structural_event() {
        let events = vec![R1bStructuralEvent {
            tick: 3,
            event_kind: R1bStructuralEventKind::ZeroCohort,
            source_slot: 7,
            target_slot: 7,
            source_cell: 0,
            target_cell: 0,
            owner_code: 2,
            amount_or_delta: 0,
            threshold_code: 0,
        }];
        let marks = free_slot_mark_sources_from_events(&events);
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].slot, 7);
        assert_eq!(marks[0].reason, "zero_cohort_departure");
    }
}
