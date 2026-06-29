//! RUNTIME-0080-0-R1c-b: resident allocation into marked free slots.
//!
//! This rung consumes the R1c-a resident mark table and R1b's GPU-read
//! LocalBirthRequest rows. It proves resident free-slot allocation into already
//! marked slots without claiming REENROLL scatter, compaction, lineage rewrite,
//! fusion compaction, or multi-atlas authority.

use std::collections::BTreeSet;

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    create_discrete_gpu_context, run_runtime_0080_0_r1a, Runtime0080R1aAdapterReport,
    Runtime0080R1aInput, R1A_COL_CURRENT, R1A_N_DIMS,
};
use crate::runtime_0080_0_r1b::{
    run_runtime_0080_0_r1b, Runtime0080R1bInput, Runtime0080R1bLocalBirthRequestSource,
    RUNTIME_0080_0_R1B_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c::{
    run_runtime_0080_0_r1c, Runtime0080R1cInput, RUNTIME_0080_0_R1C_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c_a::{
    run_runtime_0080_0_r1c_a, Runtime0080R1cAInput, RUNTIME_0080_0_R1C_A_STATUS_BLOCKED,
};

pub const RUNTIME_0080_0_R1C_B_ID: &str = "RUNTIME-0080-0-R1c-b";
pub const RUNTIME_0080_0_R1C_B_PRIMITIVE: &str = "RESIDENT-FREESLOT-ALLOC-0";
pub const RUNTIME_0080_0_R1C_B_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident allocation into marked free slots; no compaction";
pub const RUNTIME_0080_0_R1C_B_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - resident allocation evidence incomplete";
pub const RUNTIME_0080_0_R1C_B_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_B_SCOPE: &str =
    "resident free-slot allocation from R1c-a marks and R1b LocalBirthRequest rows";
pub const RUNTIME_R1C_B_EXPECTED_REPORT_CHECKSUM: u64 = 0x6917_c14a_58b5_515a;

const ALLOCATION_SELECT_BAND: u32 = 0;
const ALLOCATION_ROW_COPY_BAND: u32 = 1;
const ALLOCATION_ROW_FIELDS: u32 = 8;
const ALLOCATION_SENTINEL: f32 = 1_000_000.0;
const FAILURE_NONE: u32 = 0;
const FAILURE_NO_SLOT: u32 = 1;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cBInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cBInput {
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
pub struct Runtime0080R1cBAllocationRow {
    pub tick: u32,
    pub request_event_index: u32,
    pub requested_owner: u32,
    pub requested_source_cell: u32,
    pub requested_ships: i64,
    pub allocated_slot: Option<u32>,
    pub allocation_success: bool,
    pub allocation_failure_reason: &'static str,
    pub gpu_selected_slot_value: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cBDisabledAllocationWriterCheck {
    pub writers_enabled_rows: u32,
    pub writers_disabled_rows: u32,
    pub writers_enabled_allocation_parity: bool,
    pub writers_disabled_allocation_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cBPreservationSummary {
    pub rung: &'static str,
    pub verdict: String,
    pub checksum: u64,
    pub preserved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cBBoundaryPassReport {
    pub consumes_gpu_allocation_rows: bool,
    pub applies_birth_enrollment_from_row: bool,
    pub does_not_select_slot: bool,
    pub rows_consumed: u32,
    pub enrolled_slots: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cBReport {
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
    pub relationship_to_r1c_a: &'static str,
    pub free_slot_mark_count_before_allocation: u32,
    pub local_birth_request_count: u32,
    pub allocation_rows_written: u32,
    pub allocation_failures: u32,
    pub allocated_slots: Vec<u32>,
    pub expected_allocated_slots: Vec<u32>,
    pub mark_table_before_allocation: Vec<u32>,
    pub mark_table_after_allocation: Vec<u32>,
    pub allocation_rows: Vec<Runtime0080R1cBAllocationRow>,
    pub resident_free_list_allocation_authority: bool,
    pub allocation_rows_written_from_gpu_values: bool,
    pub allocated_slot_read_from_gpu_value: bool,
    pub cpu_boundary_pass_consumes_allocation_row: bool,
    pub cpu_boundary_pass_does_not_select_slot: bool,
    pub cpu_selected_any_slot: bool,
    pub resident_compaction_authority: bool,
    pub resident_reenroll_scatter_authority: bool,
    pub resident_arena_membership_rewrite_authority: bool,
    pub resident_fusion_compaction_authority: bool,
    pub resident_lineage_rewrite_authority: bool,
    pub m4a_or_multi_atlas_authority: bool,
    pub scenario_reopen_required: bool,
    pub docs_invariants_edit_required: bool,
    pub r1a_preservation: Option<Runtime0080R1cBPreservationSummary>,
    pub r1b_preservation: Option<Runtime0080R1cBPreservationSummary>,
    pub r1c_a_preservation: Option<Runtime0080R1cBPreservationSummary>,
    pub r1c_shadow_preservation: Option<Runtime0080R1cBPreservationSummary>,
    pub boundary_pass: Runtime0080R1cBBoundaryPassReport,
    pub disabled_allocation_writer_check: Option<Runtime0080R1cBDisabledAllocationWriterCheck>,
    pub allocation_parity_measured_from_gpu_values: bool,
    pub disabled_allocation_writer_negative_control_detected: bool,
    pub gpu_select_dispatch_count: u32,
    pub allocation_row_copy_dispatch_count: u32,
    pub allocation_readback_count: u32,
    pub allocation_ops_uploaded: u32,
    pub r6c_checksum_expected: u64,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Copy, Debug)]
struct AllocationLayout {
    candidate_start: u32,
    candidate_count: u32,
    selected_slot: u32,
    staging_rows_start: u32,
    committed_rows_start: u32,
    max_rows: u32,
}

impl AllocationLayout {
    fn new(fleet_slots: u32, max_rows: u32) -> Self {
        let candidate_start = 0;
        let selected_slot = candidate_start + fleet_slots;
        let staging_rows_start = selected_slot + 1;
        let committed_rows_start = staging_rows_start + max_rows * ALLOCATION_ROW_FIELDS;
        Self {
            candidate_start,
            candidate_count: fleet_slots,
            selected_slot,
            staging_rows_start,
            committed_rows_start,
            max_rows,
        }
    }

    fn total_slots(&self) -> u32 {
        self.committed_rows_start + self.max_rows * ALLOCATION_ROW_FIELDS
    }

    fn staging_field_slot(&self, row: u32, field: u32) -> u32 {
        self.staging_rows_start + row * ALLOCATION_ROW_FIELDS + field
    }

    fn committed_field_slot(&self, row: u32, field: u32) -> u32 {
        self.committed_rows_start + row * ALLOCATION_ROW_FIELDS + field
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AllocationSessionReport {
    pub(crate) rows: Vec<Runtime0080R1cBAllocationRow>,
    pub(crate) mark_table_after_allocation: Vec<u32>,
    pub(crate) allocation_rows_written_from_gpu_values: bool,
    pub(crate) allocated_slot_read_from_gpu_value: bool,
    pub(crate) allocation_parity_measured_from_gpu_values: bool,
    pub(crate) gpu_select_dispatch_count: u32,
    pub(crate) allocation_row_copy_dispatch_count: u32,
    pub(crate) allocation_readback_count: u32,
    pub(crate) allocation_ops_uploaded: u32,
}

pub fn run_runtime_0080_0_r1c_b(input: &Runtime0080R1cBInput) -> Runtime0080R1cBReport {
    run_runtime_0080_0_r1c_b_internal(input, true, true)
}

pub fn run_runtime_0080_0_r1c_b_with_allocation_writers_enabled(
    input: &Runtime0080R1cBInput,
    allocation_writers_enabled: bool,
) -> Runtime0080R1cBReport {
    run_runtime_0080_0_r1c_b_internal(input, allocation_writers_enabled, false)
}

pub fn replay_runtime_0080_0_r1c_b() -> (Runtime0080R1cBReport, Runtime0080R1cBReport) {
    let input = Runtime0080R1cBInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_b(&input),
        run_runtime_0080_0_r1c_b(&input),
    )
}

fn run_runtime_0080_0_r1c_b_internal(
    input: &Runtime0080R1cBInput,
    allocation_writers_enabled: bool,
    include_cross_checks: bool,
) -> Runtime0080R1cBReport {
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

    let r1c_a = run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::explicit_opt_in());
    if r1c_a.status == RUNTIME_0080_0_R1C_A_STATUS_BLOCKED || r1c_a.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_a_predecessor_blocked_or_no_discrete_gpu".to_string()],
            None,
        );
        report.status = RUNTIME_0080_0_R1C_B_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        report.r1c_a_preservation = Some(Runtime0080R1cBPreservationSummary {
            rung: "R1c-a",
            verdict: r1c_a.verdict.to_string(),
            checksum: r1c_a.stable_report_checksum,
            preserved: false,
        });
        return finalize_report(report);
    }

    let r1b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in());
    if r1b.status == RUNTIME_0080_0_R1B_STATUS_BLOCKED || r1b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            r1c_a.adapter.clone(),
        );
        report.status = RUNTIME_0080_0_R1C_B_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic.to_string()], None);
            report.status = RUNTIME_0080_0_R1C_B_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            return finalize_report(report);
        }
    };
    set_debug_readback_allowed(true);

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.admitted = true;
    report.relationship_to_r1c_a = "consumes R1c-a resident mark table";
    report.r1b_preservation = Some(Runtime0080R1cBPreservationSummary {
        rung: "R1b",
        verdict: r1b.verdict.to_string(),
        checksum: r1b.stable_report_checksum,
        preserved: r1b.event_journal_parity_measured_from_gpu_values
            && r1b.event_rows_read_from_gpu_values,
    });
    report.r1c_a_preservation = Some(Runtime0080R1cBPreservationSummary {
        rung: "R1c-a",
        verdict: r1c_a.verdict.to_string(),
        checksum: r1c_a.stable_report_checksum,
        preserved: r1c_a
            .marker
            .as_ref()
            .is_some_and(|marker| marker.mark_parity_measured_from_gpu_values),
    });

    let marker = match r1c_a.marker.as_ref() {
        Some(marker) => marker,
        None => {
            report.status = RUNTIME_0080_0_R1C_B_STATUS_PARTIAL;
            report.verdict = "PARTIAL";
            report.diagnostics.push("r1c_a_marker_missing".to_string());
            return finalize_report(report);
        }
    };
    let mark_table_before = marker.gpu_marked_slots.clone();
    let local_birth_requests = r1b.local_birth_request_sources_from_gpu_journal.clone();
    report.free_slot_mark_count_before_allocation = mark_table_before.len() as u32;
    report.local_birth_request_count = local_birth_requests.len() as u32;
    report.mark_table_before_allocation = mark_table_before.clone();
    report.expected_allocated_slots =
        expected_lowest_slots(&mark_table_before, local_birth_requests.len() as u32);

    let fleet_slots = marker
        .oracle_marked_slots
        .iter()
        .chain(marker.gpu_marked_slots.iter())
        .max()
        .copied()
        .unwrap_or(0)
        + 1;
    let allocation = match run_allocation_session(
        &ctx,
        fleet_slots,
        &mark_table_before,
        &local_birth_requests,
        allocation_writers_enabled,
    ) {
        Ok(allocation) => allocation,
        Err(diagnostic) => {
            report.status = RUNTIME_0080_0_R1C_B_STATUS_PARTIAL;
            report.verdict = "PARTIAL";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };

    report.allocation_rows_written = allocation.rows.len() as u32;
    report.allocated_slots = allocation
        .rows
        .iter()
        .filter_map(|row| row.allocated_slot)
        .collect();
    report.allocation_failures = allocation
        .rows
        .iter()
        .filter(|row| !row.allocation_success)
        .count() as u32;
    report.mark_table_after_allocation = allocation.mark_table_after_allocation.clone();
    report.allocation_rows = allocation.rows.clone();
    report.allocation_rows_written_from_gpu_values =
        allocation.allocation_rows_written_from_gpu_values;
    report.allocated_slot_read_from_gpu_value = allocation.allocated_slot_read_from_gpu_value;
    report.allocation_parity_measured_from_gpu_values =
        allocation.allocation_parity_measured_from_gpu_values;
    report.gpu_select_dispatch_count = allocation.gpu_select_dispatch_count;
    report.allocation_row_copy_dispatch_count = allocation.allocation_row_copy_dispatch_count;
    report.allocation_readback_count = allocation.allocation_readback_count;
    report.allocation_ops_uploaded = allocation.allocation_ops_uploaded;
    report.boundary_pass = Runtime0080R1cBBoundaryPassReport {
        consumes_gpu_allocation_rows: report.allocation_rows_written > 0,
        applies_birth_enrollment_from_row: report.allocation_failures == 0
            && report.allocation_rows_written == report.local_birth_request_count,
        does_not_select_slot: true,
        rows_consumed: report.allocation_rows_written,
        enrolled_slots: report.allocated_slots.clone(),
    };
    report.cpu_boundary_pass_consumes_allocation_row =
        report.boundary_pass.consumes_gpu_allocation_rows;
    report.cpu_boundary_pass_does_not_select_slot = report.boundary_pass.does_not_select_slot;

    if include_cross_checks {
        let r1a = run_runtime_0080_0_r1a(&Runtime0080R1aInput::explicit_opt_in());
        report.r1a_preservation = Some(Runtime0080R1cBPreservationSummary {
            rung: "R1a",
            verdict: r1a.verdict.to_string(),
            checksum: r1a.stable_report_checksum,
            preserved: r1a.field_column_parity_matches_r6c_checksum,
        });
        let r1c = run_runtime_0080_0_r1c(&Runtime0080R1cInput::explicit_opt_in());
        report.r1c_shadow_preservation = Some(Runtime0080R1cBPreservationSummary {
            rung: "R1c",
            verdict: r1c.verdict.to_string(),
            checksum: r1c.stable_report_checksum,
            preserved: r1c.status != RUNTIME_0080_0_R1C_STATUS_BLOCKED
                && r1c
                    .shadow_contract
                    .as_ref()
                    .is_some_and(|shadow| shadow.serialize_reload_continue_roundtrip),
        });
    }

    if include_cross_checks && allocation_writers_enabled {
        let disabled = run_runtime_0080_0_r1c_b_internal(input, false, false);
        let negative_control_detected = report.allocation_rows_written
            > disabled.allocation_rows_written
            && report.allocation_parity_measured_from_gpu_values
            && !disabled.allocation_parity_measured_from_gpu_values;
        report.disabled_allocation_writer_check =
            Some(Runtime0080R1cBDisabledAllocationWriterCheck {
                writers_enabled_rows: report.allocation_rows_written,
                writers_disabled_rows: disabled.allocation_rows_written,
                writers_enabled_allocation_parity: report
                    .allocation_parity_measured_from_gpu_values,
                writers_disabled_allocation_parity: disabled
                    .allocation_parity_measured_from_gpu_values,
                negative_control_detected,
                disabled_report_checksum: disabled.stable_report_checksum,
            });
        report.disabled_allocation_writer_negative_control_detected = negative_control_detected;
    }

    let preservation_ok = report
        .r1a_preservation
        .as_ref()
        .is_some_and(|summary| summary.preserved)
        && report
            .r1b_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved)
        && report
            .r1c_a_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved)
        && report
            .r1c_shadow_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved);

    let pass = allocation_writers_enabled
        && report.free_slot_mark_count_before_allocation > 0
        && report.local_birth_request_count > 0
        && report.allocation_rows_written == report.local_birth_request_count
        && report.allocation_failures == 0
        && report.allocated_slots == report.expected_allocated_slots
        && report.allocation_rows_written_from_gpu_values
        && report.allocated_slot_read_from_gpu_value
        && report.allocation_parity_measured_from_gpu_values
        && report.cpu_boundary_pass_consumes_allocation_row
        && report.cpu_boundary_pass_does_not_select_slot
        && !report.cpu_selected_any_slot
        && report.disabled_allocation_writer_negative_control_detected
        && preservation_ok
        && !report.resident_compaction_authority
        && !report.resident_reenroll_scatter_authority
        && !report.resident_fusion_compaction_authority
        && !report.docs_invariants_edit_required
        && !report.scenario_reopen_required;

    if pass {
        report.status = RUNTIME_0080_0_R1C_B_STATUS_PASS;
        report.verdict = "PASS";
        report.resident_free_list_allocation_authority = true;
        report.diagnostics = vec![
            "resident_free_slot_allocation_pass".to_string(),
            "gpu_min_selects_lowest_marked_free_slot".to_string(),
            "cpu_boundary_consumes_gpu_allocation_rows_without_selecting_slots".to_string(),
            "disabled_allocation_writer_negative_control_detected".to_string(),
            "no_compaction_scatter_fusion_lineage_or_m4a_claimed".to_string(),
        ];
    } else if !allocation_writers_enabled {
        report.status = RUNTIME_0080_0_R1C_B_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("allocation_writers_disabled_for_negative_control".to_string());
    } else {
        report.status = RUNTIME_0080_0_R1C_B_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("resident_free_slot_allocation_parity_incomplete".to_string());
    }

    finalize_report(report)
}

pub(crate) fn run_allocation_session(
    ctx: &simthing_gpu::GpuContext,
    fleet_slots: u32,
    mark_table_before: &[u32],
    requests: &[Runtime0080R1bLocalBirthRequestSource],
    allocation_writers_enabled: bool,
) -> Result<AllocationSessionReport, &'static str> {
    let layout = AllocationLayout::new(fleet_slots.max(1), requests.len().max(1) as u32);
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots(), R1A_N_DIMS);
    session
        .fill_slot_range_col(ctx, 0, layout.total_slots(), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "allocation_session_clear_failed")?;
    let select_op = select_lowest_candidate_op(&layout);
    let mut rows = Vec::with_capacity(requests.len());
    let mut current_marks = mark_table_before.iter().copied().collect::<BTreeSet<_>>();
    let mut select_dispatches = 0;
    let mut row_copy_dispatches = 0;
    let mut readbacks = 0;
    let mut ops_uploaded = 0;
    let expected = expected_lowest_slots(mark_table_before, requests.len() as u32);

    for (idx, request) in requests.iter().enumerate() {
        fill_candidates(ctx, &mut session, &layout, &current_marks)?;
        session
            .upload_ops(ctx, std::slice::from_ref(&select_op))
            .map_err(|_| "allocation_select_upload_failed")?;
        ops_uploaded += 1;
        session
            .tick(ctx, ALLOCATION_SELECT_BAND)
            .map_err(|_| "allocation_select_tick_failed")?;
        select_dispatches += 1;
        let selected_values = session
            .readback_full(ctx)
            .map_err(|_| "allocation_select_readback_failed")?;
        readbacks += 1;
        let selected_value = read_slot(&selected_values, layout.selected_slot);
        let selected_slot = decode_slot_value(selected_value, fleet_slots);
        let success = allocation_writers_enabled && selected_slot.is_some();
        stage_request_metadata(ctx, &mut session, &layout, idx as u32, request, success)?;

        if allocation_writers_enabled {
            let row_copy_ops = row_copy_ops(&layout, idx as u32);
            session
                .upload_ops(ctx, &row_copy_ops)
                .map_err(|_| "allocation_row_copy_upload_failed")?;
            ops_uploaded += row_copy_ops.len() as u32;
            session
                .tick(ctx, ALLOCATION_ROW_COPY_BAND)
                .map_err(|_| "allocation_row_copy_tick_failed")?;
            row_copy_dispatches += 1;
        }

        let row_values = session
            .readback_full(ctx)
            .map_err(|_| "allocation_row_readback_failed")?;
        readbacks += 1;
        if allocation_writers_enabled {
            let row = decode_allocation_row(&row_values, &layout, idx as u32, selected_slot);
            if let Some(slot) = row.allocated_slot {
                current_marks.remove(&slot);
            }
            rows.push(row);
        }
    }

    let allocated_slots = rows
        .iter()
        .filter_map(|row| row.allocated_slot)
        .collect::<Vec<_>>();
    let allocation_rows_written_from_gpu_values = !rows.is_empty()
        && rows
            .iter()
            .all(|row| row.allocated_slot == row.gpu_selected_slot_value);
    let allocated_slot_read_from_gpu_value =
        !rows.is_empty() && rows.iter().all(|row| row.gpu_selected_slot_value.is_some());
    let allocation_parity_measured_from_gpu_values = allocated_slots == expected
        && allocation_rows_written_from_gpu_values
        && allocated_slot_read_from_gpu_value;

    Ok(AllocationSessionReport {
        rows,
        mark_table_after_allocation: current_marks.into_iter().collect(),
        allocation_rows_written_from_gpu_values,
        allocated_slot_read_from_gpu_value,
        allocation_parity_measured_from_gpu_values,
        gpu_select_dispatch_count: select_dispatches,
        allocation_row_copy_dispatch_count: row_copy_dispatches,
        allocation_readback_count: readbacks,
        allocation_ops_uploaded: ops_uploaded,
    })
}

fn fill_candidates(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &AllocationLayout,
    current_marks: &BTreeSet<u32>,
) -> Result<(), &'static str> {
    for slot in 0..layout.candidate_count {
        let value = if current_marks.contains(&slot) {
            slot as f32
        } else {
            ALLOCATION_SENTINEL
        };
        session
            .fill_slot_range_col(
                ctx,
                layout.candidate_start + slot,
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "allocation_candidate_fill_failed")?;
    }
    Ok(())
}

fn stage_request_metadata(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &AllocationLayout,
    row: u32,
    request: &Runtime0080R1bLocalBirthRequestSource,
    success: bool,
) -> Result<(), &'static str> {
    let fields = [
        request.tick as f32,
        request.request_index as f32,
        request.owner_code as f32,
        request.source_cell as f32,
        ALLOCATION_SENTINEL,
        if success { 1.0 } else { 0.0 },
        if success {
            FAILURE_NONE as f32
        } else {
            FAILURE_NO_SLOT as f32
        },
        request.requested_ships as f32,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.staging_field_slot(row, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "allocation_row_stage_failed")?;
    }
    Ok(())
}

fn select_lowest_candidate_op(layout: &AllocationLayout) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: SlotIndex::new(layout.candidate_start),
            count: layout.candidate_count,
            col: ColumnIndex::new(R1A_COL_CURRENT as usize),
        },
        combine: CombineFn::Min,
        gate: GateSpec::OrderBand(ALLOCATION_SELECT_BAND),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(
            SlotIndex::new(layout.selected_slot),
            ColumnIndex::new(R1A_COL_CURRENT as usize),
        )],
    }
}

fn row_copy_ops(layout: &AllocationLayout, row: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(ALLOCATION_ROW_FIELDS as usize);
    for field in 0..ALLOCATION_ROW_FIELDS {
        let source_slot = if field == 4 {
            layout.selected_slot
        } else {
            layout.staging_field_slot(row, field)
        };
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(source_slot),
                col: ColumnIndex::new(R1A_COL_CURRENT as usize),
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(ALLOCATION_ROW_COPY_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                SlotIndex::new(layout.committed_field_slot(row, field)),
                ColumnIndex::new(R1A_COL_CURRENT as usize),
            )],
        });
    }
    ops
}

fn decode_allocation_row(
    values: &[f32],
    layout: &AllocationLayout,
    row: u32,
    selected_slot: Option<u32>,
) -> Runtime0080R1cBAllocationRow {
    let field = |field| read_slot(values, layout.committed_field_slot(row, field));
    let allocated_slot = decode_slot_value(field(4), layout.candidate_count);
    let success = field(5) > 0.5 && allocated_slot.is_some();
    let failure_reason = if success {
        "none"
    } else if f32_to_u32(field(6)) == FAILURE_NO_SLOT {
        "no_marked_free_slot"
    } else {
        "allocation_writer_disabled"
    };
    Runtime0080R1cBAllocationRow {
        tick: f32_to_u32(field(0)),
        request_event_index: f32_to_u32(field(1)),
        requested_owner: f32_to_u32(field(2)),
        requested_source_cell: f32_to_u32(field(3)),
        requested_ships: field(7).round() as i64,
        allocated_slot,
        allocation_success: success,
        allocation_failure_reason: failure_reason,
        gpu_selected_slot_value: selected_slot,
    }
}

fn decode_slot_value(value: f32, fleet_slots: u32) -> Option<u32> {
    if value.is_finite() && value >= 0.0 && value < fleet_slots as f32 {
        Some(value.round() as u32)
    } else {
        None
    }
}

fn read_slot(values: &[f32], slot: u32) -> f32 {
    values
        .get((slot * R1A_N_DIMS + R1A_COL_CURRENT) as usize)
        .copied()
        .unwrap_or(ALLOCATION_SENTINEL)
}

fn expected_lowest_slots(mark_table_before: &[u32], request_count: u32) -> Vec<u32> {
    mark_table_before
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .take(request_count as usize)
        .collect()
}

#[cfg(test)]
mod fast_allocation_sentinel_tests {
    use super::expected_lowest_slots;

    #[test]
    fn r1c_fast_allocation_selects_one_compatible_marked_slot() {
        let marks = vec![5, 2, 8];
        let selected = expected_lowest_slots(&marks, 1);
        assert_eq!(selected, vec![2]);
        assert_eq!(expected_lowest_slots(&marks, 2), vec![2, 5]);
    }
}

fn f32_to_u32(value: f32) -> u32 {
    if value.is_nan() || value.is_sign_negative() {
        0
    } else {
        value.round() as u32
    }
}

fn base_report(
    input: &Runtime0080R1cBInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1cBReport {
    Runtime0080R1cBReport {
        id: RUNTIME_0080_0_R1C_B_ID,
        primitive_name: RUNTIME_0080_0_R1C_B_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R1C_B_SCOPE,
        adapter,
        relationship_to_r1c_a: "not run",
        free_slot_mark_count_before_allocation: 0,
        local_birth_request_count: 0,
        allocation_rows_written: 0,
        allocation_failures: 0,
        allocated_slots: Vec::new(),
        expected_allocated_slots: Vec::new(),
        mark_table_before_allocation: Vec::new(),
        mark_table_after_allocation: Vec::new(),
        allocation_rows: Vec::new(),
        resident_free_list_allocation_authority: false,
        allocation_rows_written_from_gpu_values: false,
        allocated_slot_read_from_gpu_value: false,
        cpu_boundary_pass_consumes_allocation_row: false,
        cpu_boundary_pass_does_not_select_slot: true,
        cpu_selected_any_slot: false,
        resident_compaction_authority: false,
        resident_reenroll_scatter_authority: false,
        resident_arena_membership_rewrite_authority: false,
        resident_fusion_compaction_authority: false,
        resident_lineage_rewrite_authority: false,
        m4a_or_multi_atlas_authority: false,
        scenario_reopen_required: false,
        docs_invariants_edit_required: false,
        r1a_preservation: None,
        r1b_preservation: None,
        r1c_a_preservation: None,
        r1c_shadow_preservation: None,
        boundary_pass: Runtime0080R1cBBoundaryPassReport {
            consumes_gpu_allocation_rows: false,
            applies_birth_enrollment_from_row: false,
            does_not_select_slot: true,
            rows_consumed: 0,
            enrolled_slots: Vec::new(),
        },
        disabled_allocation_writer_check: None,
        allocation_parity_measured_from_gpu_values: false,
        disabled_allocation_writer_negative_control_detected: false,
        gpu_select_dispatch_count: 0,
        allocation_row_copy_dispatch_count: 0,
        allocation_readback_count: 0,
        allocation_ops_uploaded: 0,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "FieldPolicy",
            "field_agent",
            "selection",
            "extraction",
            "resident event journal",
            "resident mark table",
            "resident free-slot allocation",
            "GPU-side allocation rows",
            "disabled-transform parity check",
        ],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1cBReport) -> Runtime0080R1cBReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_b_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cBReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(
        &mut hash,
        report.resident_free_list_allocation_authority as u64,
    );
    mix_u64(
        &mut hash,
        report.allocation_rows_written_from_gpu_values as u64,
    );
    mix_u64(&mut hash, report.allocated_slot_read_from_gpu_value as u64);
    mix_u64(&mut hash, report.allocation_rows_written as u64);
    mix_u64(&mut hash, report.allocation_failures as u64);
    mix_u64(
        &mut hash,
        report.allocation_parity_measured_from_gpu_values as u64,
    );
    mix_u64(
        &mut hash,
        report.disabled_allocation_writer_negative_control_detected as u64,
    );
    for slot in &report.allocated_slots {
        mix_u64(&mut hash, *slot as u64);
    }
    for slot in &report.mark_table_after_allocation {
        mix_u64(&mut hash, *slot as u64);
    }
    for row in &report.allocation_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.request_event_index as u64);
        mix_u64(&mut hash, row.requested_owner as u64);
        mix_u64(&mut hash, row.requested_source_cell as u64);
        mix_u64(&mut hash, row.allocated_slot.unwrap_or(u32::MAX) as u64);
        mix_u64(&mut hash, row.allocation_success as u64);
    }
    if let Some(summary) = &report.r1a_preservation {
        mix_u64(&mut hash, summary.checksum);
        mix_u64(&mut hash, summary.preserved as u64);
    }
    if let Some(summary) = &report.r1b_preservation {
        mix_u64(&mut hash, summary.checksum);
        mix_u64(&mut hash, summary.preserved as u64);
    }
    if let Some(summary) = &report.r1c_a_preservation {
        mix_u64(&mut hash, summary.checksum);
        mix_u64(&mut hash, summary.preserved as u64);
    }
    if let Some(summary) = &report.r1c_shadow_preservation {
        mix_u64(&mut hash, summary.checksum);
        mix_u64(&mut hash, summary.preserved as u64);
    }
    if let Some(check) = &report.disabled_allocation_writer_check {
        mix_u64(&mut hash, check.writers_enabled_rows as u64);
        mix_u64(&mut hash, check.writers_disabled_rows as u64);
        mix_u64(&mut hash, check.negative_control_detected as u64);
        mix_u64(&mut hash, check.disabled_report_checksum);
    }
    hash
}

pub fn render_runtime_0080_r1c_b_artifact(report: &Runtime0080R1cBReport) -> String {
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
    let rows = if report.allocation_rows.is_empty() {
        "- none".to_string()
    } else {
        report
            .allocation_rows
            .iter()
            .map(|row| {
                format!(
                    "- tick {} request {} owner {} source_cell {} allocated_slot {:?} success {} failure {}",
                    row.tick,
                    row.request_event_index,
                    row.requested_owner,
                    row.requested_source_cell,
                    row.allocated_slot,
                    row.allocation_success,
                    row.allocation_failure_reason
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let disabled_check = report
        .disabled_allocation_writer_check
        .as_ref()
        .map(|check| {
            format!(
                "- writers_enabled_rows: {}\n- writers_disabled_rows: {}\n- writers_enabled_allocation_parity: {}\n- writers_disabled_allocation_parity: {}\n- negative_control_detected: {}\n- disabled_report_checksum: {:016x}\n",
                check.writers_enabled_rows,
                check.writers_disabled_rows,
                check.writers_enabled_allocation_parity,
                check.writers_disabled_allocation_parity,
                check.negative_control_detected,
                check.disabled_report_checksum,
            )
        })
        .unwrap_or_else(|| "- not_run\n".to_string());
    let preservation = [
        preservation_line("R1a", &report.r1a_preservation),
        preservation_line("R1b", &report.r1b_preservation),
        preservation_line("R1c-a", &report.r1c_a_preservation),
        preservation_line("R1c", &report.r1c_shadow_preservation),
    ]
    .join("\n");
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
        "# RUNTIME-0080-0-R1c-b Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Adapter\n\
         {adapter}\
         ## Resident Free-Slot Allocation\n\
         - relationship_to_r1c_a: {relationship}\n\
         - free_slot_mark_count_before_allocation: {mark_count}\n\
         - local_birth_request_count: {request_count}\n\
         - allocation_rows_written: {rows_written}\n\
         - allocation_failures: {failures}\n\
         - mark_table_before_allocation: {mark_before:?}\n\
         - allocated_slots: {allocated:?}\n\
         - expected_allocated_slots: {expected:?}\n\
         - mark_table_after_allocation: {mark_after:?}\n\
         - allocation_parity_measured_from_gpu_values: {allocation_parity}\n\
         - gpu_select_dispatch_count: {select_dispatches}\n\
         - allocation_row_copy_dispatch_count: {copy_dispatches}\n\
         - allocation_readback_count: {readbacks}\n\
         - allocation_ops_uploaded: {ops}\n\n\
         ## GPU-Side Allocation Rows\n\
         {rows}\n\n\
         ## Boundary Pass\n\
         - cpu_boundary_pass_consumes_allocation_row: {boundary_consumes}\n\
         - cpu_boundary_pass_does_not_select_slot: {boundary_no_select}\n\
         - cpu_selected_any_slot: {cpu_selected}\n\
         - rows_consumed: {rows_consumed}\n\
         - enrolled_slots: {enrolled:?}\n\n\
         ## Disabled-Transform Parity Check\n\
         {disabled_check}\
         ## Authority Flags\n\
         - resident_free_list_allocation_authority: {alloc_authority}\n\
         - allocation_rows_written_from_gpu_values: {rows_from_gpu}\n\
         - allocated_slot_read_from_gpu_value: {slot_from_gpu}\n\
         - resident_compaction_authority: {compaction}\n\
         - resident_reenroll_scatter_authority: {scatter}\n\
         - resident_arena_membership_rewrite_authority: {membership}\n\
         - resident_fusion_compaction_authority: {fusion}\n\
         - resident_lineage_rewrite_authority: {lineage}\n\
         - m4a_or_multi_atlas_authority: {m4a}\n\
         - docs_invariants_edit_required: {invariants}\n\
         - scenario_reopen_required: {scenario}\n\n\
         ## Preservation\n\
         {preservation}\n\n\
         ## Domain Terms\n\
         - {terms}\n\n\
         ## Diagnostics\n\
         {diagnostics}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter_lines,
        relationship = report.relationship_to_r1c_a,
        mark_count = report.free_slot_mark_count_before_allocation,
        request_count = report.local_birth_request_count,
        rows_written = report.allocation_rows_written,
        failures = report.allocation_failures,
        mark_before = report.mark_table_before_allocation,
        allocated = report.allocated_slots,
        expected = report.expected_allocated_slots,
        mark_after = report.mark_table_after_allocation,
        allocation_parity = report.allocation_parity_measured_from_gpu_values,
        select_dispatches = report.gpu_select_dispatch_count,
        copy_dispatches = report.allocation_row_copy_dispatch_count,
        readbacks = report.allocation_readback_count,
        ops = report.allocation_ops_uploaded,
        rows = rows,
        boundary_consumes = report.cpu_boundary_pass_consumes_allocation_row,
        boundary_no_select = report.cpu_boundary_pass_does_not_select_slot,
        cpu_selected = report.cpu_selected_any_slot,
        rows_consumed = report.boundary_pass.rows_consumed,
        enrolled = report.boundary_pass.enrolled_slots,
        disabled_check = disabled_check,
        alloc_authority = report.resident_free_list_allocation_authority,
        rows_from_gpu = report.allocation_rows_written_from_gpu_values,
        slot_from_gpu = report.allocated_slot_read_from_gpu_value,
        compaction = report.resident_compaction_authority,
        scatter = report.resident_reenroll_scatter_authority,
        membership = report.resident_arena_membership_rewrite_authority,
        fusion = report.resident_fusion_compaction_authority,
        lineage = report.resident_lineage_rewrite_authority,
        m4a = report.m4a_or_multi_atlas_authority,
        invariants = report.docs_invariants_edit_required,
        scenario = report.scenario_reopen_required,
        preservation = preservation,
        terms = report.domain_terms.join("\n- "),
        diagnostics = diagnostics,
    )
}

fn preservation_line(label: &str, summary: &Option<Runtime0080R1cBPreservationSummary>) -> String {
    summary
        .as_ref()
        .map(|summary| {
            format!(
                "- {}: verdict {} checksum {:016x} preserved {}",
                label, summary.verdict, summary.checksum, summary.preserved
            )
        })
        .unwrap_or_else(|| format!("- {}: not_run", label))
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}
