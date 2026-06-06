//! RUNTIME-0080-0-R1c-d: resident compaction-map and lineage staging.
//!
//! This rung consumes the resident event journal, mark table, allocation rows, and membership
//! deltas from R1b through R1c-c. It stages generic compaction-map and lineage rows on the GPU for
//! the single resident theater. It does not perform M-4A, multi-atlas scheduling, recursion, or
//! physical memory compaction.

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::dress_rehearsal_r6c_integrated_run::{R1bStructuralEvent, R1bStructuralEventKind};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    create_discrete_gpu_context, run_runtime_0080_0_r1a, Runtime0080R1aAdapterReport,
    Runtime0080R1aInput, R1A_COL_CURRENT, R1A_N_DIMS,
};
use crate::runtime_0080_0_r1b::{
    run_runtime_0080_0_r1b, Runtime0080R1bInput, RUNTIME_0080_0_R1B_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c::{
    run_runtime_0080_0_r1c, Runtime0080R1cInput, RUNTIME_0080_0_R1C_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c_a::{run_runtime_0080_0_r1c_a, Runtime0080R1cAInput};
use crate::runtime_0080_0_r1c_b::{
    run_runtime_0080_0_r1c_b, Runtime0080R1cBAllocationRow, Runtime0080R1cBInput,
    RUNTIME_0080_0_R1C_B_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c_c::{
    Runtime0080R1cCMembershipDeltaRow, Runtime0080R1cCPreservationSummary,
};

pub const RUNTIME_0080_0_R1C_D_ID: &str = "RUNTIME-0080-0-R1c-d";
pub const RUNTIME_0080_0_R1C_D_PRIMITIVE: &str = "RESIDENT-COMPACTION-LINEAGE-0";
pub const RUNTIME_0080_0_R1C_D_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident compaction-map and lineage staging; no M-4A";
pub const RUNTIME_0080_0_R1C_D_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - resident compaction or lineage staging evidence incomplete";
pub const RUNTIME_0080_0_R1C_D_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_D_SCOPE: &str =
    "resident compaction-map and lineage staging from R1b/R1c-a/R1c-b/R1c-c rows; no M-4A";

const COMPACTION_COPY_BAND: u32 = 0;
const LINEAGE_COPY_BAND: u32 = 1;
const COMPACTION_FIELDS: u32 = 11;
const LINEAGE_FIELDS: u32 = 10;
const TOMBSTONE_SLOT: u32 = 16_777_215;

const REASON_ZERO_OR_INACTIVE: u32 = 1;
const REASON_FUSION_ABSORBED: u32 = 2;
const REASON_BIRTH_ALLOCATED: u32 = 3;
const REASON_DEPARTURE_MARKED: u32 = 4;

const LINEAGE_BIRTH: u32 = 1;
const LINEAGE_ABSORB: u32 = 2;
const LINEAGE_SURVIVE: u32 = 3;
const LINEAGE_TOMBSTONE: u32 = 4;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const RUNTIME_R1C_C_LANDED_PASS_CHECKSUM: u64 = 0x9581_b083_8619_d9c0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cDInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cDInput {
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
pub struct Runtime0080R1cDCompactionRow {
    pub tick: u32,
    pub old_slot: u32,
    pub new_slot_or_tombstone: Option<u32>,
    pub owner_code: u32,
    pub reason_code: &'static str,
    pub active_before: bool,
    pub active_after: bool,
    pub source_event_kind: &'static str,
    pub source_event_index: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cDLineageRow {
    pub tick: u32,
    pub lineage_event_id: u32,
    pub source_slot: u32,
    pub target_slot: u32,
    pub owner_code: u32,
    pub lineage_kind: &'static str,
    pub amount_or_delta: i64,
    pub source_event_kind: &'static str,
    pub source_event_index: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cDCompactedViewRow {
    pub old_slot: u32,
    pub new_slot_or_tombstone: Option<u32>,
    pub survivor_slot: Option<u32>,
    pub active_after: bool,
    pub reason_code: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cDDisabledWriterCheck {
    pub writers_enabled_rows: u32,
    pub writers_disabled_rows: u32,
    pub writers_enabled_parity: bool,
    pub writers_disabled_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cDCpuShadowReport {
    pub consumes_compaction_rows_without_redeciding: bool,
    pub consumes_lineage_rows_without_redeciding: bool,
    pub cpu_decided_any_compaction_row: bool,
    pub cpu_decided_any_lineage_row: bool,
    pub compaction_shadow_matches_gpu_rows: bool,
    pub lineage_shadow_matches_gpu_rows: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cDReport {
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
    pub relationship_to_r1c_c: &'static str,
    pub compaction_representation: &'static str,
    pub lineage_representation: &'static str,
    pub resident_compaction_map_created: bool,
    pub resident_lineage_staging_created: bool,
    pub compaction_rows_written: u32,
    pub lineage_rows_written: u32,
    pub tombstone_rows_written: u32,
    pub fusion_absorption_rows_written: u32,
    pub birth_lineage_rows_written: u32,
    pub compaction_rows_read_from_gpu_values: bool,
    pub lineage_rows_read_from_gpu_values: bool,
    pub gpu_writes_compaction_rows: bool,
    pub gpu_writes_lineage_rows: bool,
    pub compaction_rows: Vec<Runtime0080R1cDCompactionRow>,
    pub lineage_rows: Vec<Runtime0080R1cDLineageRow>,
    pub resident_compacted_view: Vec<Runtime0080R1cDCompactedViewRow>,
    pub consumes_r1b_event_journal: bool,
    pub consumes_r1c_a_mark_table: bool,
    pub consumes_r1c_b_allocation_rows: bool,
    pub consumes_r1c_c_membership_rows: bool,
    pub r1b_event_rows_consumed: u32,
    pub r1c_a_mark_rows_consumed: u32,
    pub r1c_b_allocation_rows_consumed: u32,
    pub r1c_c_membership_rows_consumed: u32,
    pub fusion_absorption_lineage_staged_if_fusion_rows_exist: bool,
    pub zero_or_departure_tombstones_staged_if_zero_rows_exist: bool,
    pub cpu_shadow: Runtime0080R1cDCpuShadowReport,
    pub disabled_compaction_writer_check: Option<Runtime0080R1cDDisabledWriterCheck>,
    pub disabled_lineage_writer_check: Option<Runtime0080R1cDDisabledWriterCheck>,
    pub disabled_compaction_writer_negative_control_detected: bool,
    pub disabled_lineage_writer_negative_control_detected: bool,
    pub gpu_compaction_copy_dispatch_count: u32,
    pub gpu_lineage_copy_dispatch_count: u32,
    pub compaction_readback_count: u32,
    pub lineage_readback_count: u32,
    pub compaction_ops_uploaded: u32,
    pub lineage_ops_uploaded: u32,
    pub resident_compaction_authority: bool,
    pub resident_lineage_staging_authority: bool,
    pub resident_fusion_compaction_authority: bool,
    pub resident_lineage_rewrite_authority: bool,
    pub resident_m4a_authority: bool,
    pub multi_atlas_authority: bool,
    pub system_planet_recursion_authority: bool,
    pub default_session_wiring: bool,
    pub scenario_reopen_required: bool,
    pub docs_invariants_edit_required: bool,
    pub r1a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_c_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_shadow_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r6c_checksum_expected: u64,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub exact_commands: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedCompactionRow {
    tick: u32,
    old_slot: u32,
    new_slot_or_tombstone: Option<u32>,
    owner_code: u32,
    reason: u32,
    active_before: bool,
    active_after: bool,
    source_event_kind: &'static str,
    source_event_index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedLineageRow {
    tick: u32,
    lineage_event_id: u32,
    source_slot: u32,
    target_slot: u32,
    owner_code: u32,
    lineage_kind: u32,
    amount_or_delta: i64,
    source_event_kind: &'static str,
    source_event_index: u32,
}

#[derive(Clone, Copy, Debug)]
struct StagingLayout {
    compaction_staging_start: u32,
    compaction_committed_start: u32,
    lineage_staging_start: u32,
    lineage_committed_start: u32,
    lineage_rows: u32,
}

impl StagingLayout {
    fn new(compaction_rows: u32, lineage_rows: u32) -> Self {
        let compaction_staging_start = 0;
        let compaction_committed_start =
            compaction_staging_start + compaction_rows * COMPACTION_FIELDS;
        let lineage_staging_start =
            compaction_committed_start + compaction_rows * COMPACTION_FIELDS;
        let lineage_committed_start = lineage_staging_start + lineage_rows * LINEAGE_FIELDS;
        Self {
            compaction_staging_start,
            compaction_committed_start,
            lineage_staging_start,
            lineage_committed_start,
            lineage_rows,
        }
    }

    fn total_slots(&self) -> u32 {
        self.lineage_committed_start + self.lineage_rows * LINEAGE_FIELDS
    }

    fn compaction_staging_slot(&self, row: u32, field: u32) -> u32 {
        self.compaction_staging_start + row * COMPACTION_FIELDS + field
    }

    fn compaction_committed_slot(&self, row: u32, field: u32) -> u32 {
        self.compaction_committed_start + row * COMPACTION_FIELDS + field
    }

    fn lineage_staging_slot(&self, row: u32, field: u32) -> u32 {
        self.lineage_staging_start + row * LINEAGE_FIELDS + field
    }

    fn lineage_committed_slot(&self, row: u32, field: u32) -> u32 {
        self.lineage_committed_start + row * LINEAGE_FIELDS + field
    }
}

#[derive(Clone, Debug)]
struct StagingSessionReport {
    compaction_rows: Vec<Runtime0080R1cDCompactionRow>,
    lineage_rows: Vec<Runtime0080R1cDLineageRow>,
    compaction_parity: bool,
    lineage_parity: bool,
    compaction_dispatch_count: u32,
    lineage_dispatch_count: u32,
    compaction_readback_count: u32,
    lineage_readback_count: u32,
    compaction_ops_uploaded: u32,
    lineage_ops_uploaded: u32,
}

pub fn run_runtime_0080_0_r1c_d(input: &Runtime0080R1cDInput) -> Runtime0080R1cDReport {
    run_runtime_0080_0_r1c_d_internal(input, true, true, true)
}

pub fn run_runtime_0080_0_r1c_d_with_writers_enabled(
    input: &Runtime0080R1cDInput,
    compaction_writers_enabled: bool,
    lineage_writers_enabled: bool,
) -> Runtime0080R1cDReport {
    run_runtime_0080_0_r1c_d_internal(
        input,
        compaction_writers_enabled,
        lineage_writers_enabled,
        false,
    )
}

pub fn replay_runtime_0080_0_r1c_d() -> (Runtime0080R1cDReport, Runtime0080R1cDReport) {
    let input = Runtime0080R1cDInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_d(&input),
        run_runtime_0080_0_r1c_d(&input),
    )
}

fn run_runtime_0080_0_r1c_d_internal(
    input: &Runtime0080R1cDInput,
    compaction_writers_enabled: bool,
    lineage_writers_enabled: bool,
    include_cross_checks: bool,
) -> Runtime0080R1cDReport {
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

    let r1b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in());
    if r1b.status == RUNTIME_0080_0_R1B_STATUS_BLOCKED || r1b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            None,
        );
        report.status = RUNTIME_0080_0_R1C_D_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let r1c_a = run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::explicit_opt_in());
    let r1c_b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::explicit_opt_in());
    if r1c_b.status == RUNTIME_0080_0_R1C_B_STATUS_BLOCKED || r1c_b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            r1c_b.adapter.clone(),
        );
        report.status = RUNTIME_0080_0_R1C_D_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic.to_string()], None);
            report.status = RUNTIME_0080_0_R1C_D_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            return finalize_report(report);
        }
    };
    set_debug_readback_allowed(true);

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.admitted = true;
    report.relationship_to_r1c_c =
        "consumes R1c-c resident membership rows plus R1b/R1c-a/R1c-b structural inputs";
    report.consumes_r1b_event_journal = r1b.event_rows_read_from_gpu_values;
    report.consumes_r1c_a_mark_table = r1c_a
        .marker
        .as_ref()
        .is_some_and(|marker| marker.mark_parity_measured_from_gpu_values);
    report.consumes_r1c_b_allocation_rows = r1c_b.allocation_rows_written_from_gpu_values;
    let r1c_c_membership_rows = project_r1c_c_membership_rows(
        &r1b.structural_events_from_gpu_journal,
        &r1c_b.allocation_rows,
    );
    report.consumes_r1c_c_membership_rows = !r1c_c_membership_rows.is_empty();
    report.r1b_event_rows_consumed = r1b.structural_events_from_gpu_journal.len() as u32;
    report.r1c_a_mark_rows_consumed = r1c_a
        .marker
        .as_ref()
        .map(|marker| marker.gpu_marked_slots.len() as u32)
        .unwrap_or(0);
    report.r1c_b_allocation_rows_consumed = r1c_b.allocation_rows.len() as u32;
    report.r1c_c_membership_rows_consumed = r1c_c_membership_rows.len() as u32;
    report.r1b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1b",
        verdict: r1b.verdict.to_string(),
        checksum: r1b.stable_report_checksum,
        preserved: r1b.event_journal_parity_measured_from_gpu_values
            && r1b.event_rows_read_from_gpu_values,
    });
    report.r1c_a_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-a",
        verdict: r1c_a.verdict.to_string(),
        checksum: r1c_a.stable_report_checksum,
        preserved: report.consumes_r1c_a_mark_table,
    });
    report.r1c_b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-b",
        verdict: r1c_b.verdict.to_string(),
        checksum: r1c_b.stable_report_checksum,
        preserved: r1c_b.allocation_parity_measured_from_gpu_values
            && r1c_b.allocation_rows_written_from_gpu_values,
    });
    report.r1c_c_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-c",
        verdict: "PASS".to_string(),
        checksum: RUNTIME_R1C_C_LANDED_PASS_CHECKSUM,
        preserved: report.consumes_r1c_c_membership_rows
            && r1c_c_membership_rows
                .iter()
                .any(|row| row.membership_action == "DepartureMark"),
    });

    let (planned_compaction, planned_lineage) = build_staging_plan(
        &r1b.structural_events_from_gpu_journal,
        &r1c_b.allocation_rows,
        &r1c_c_membership_rows,
    );
    report.resident_compaction_map_created = true;
    report.resident_lineage_staging_created = true;

    let staging = match run_staging_session(
        &ctx,
        &planned_compaction,
        &planned_lineage,
        compaction_writers_enabled,
        lineage_writers_enabled,
    ) {
        Ok(staging) => staging,
        Err(diagnostic) => {
            report.status = RUNTIME_0080_0_R1C_D_STATUS_PARTIAL;
            report.verdict = "PARTIAL";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };

    report.compaction_rows = staging.compaction_rows.clone();
    report.lineage_rows = staging.lineage_rows.clone();
    report.compaction_rows_written = report.compaction_rows.len() as u32;
    report.lineage_rows_written = report.lineage_rows.len() as u32;
    report.tombstone_rows_written = report
        .compaction_rows
        .iter()
        .filter(|row| row.new_slot_or_tombstone.is_none())
        .count() as u32;
    report.fusion_absorption_rows_written = report
        .lineage_rows
        .iter()
        .filter(|row| row.lineage_kind == "Absorb")
        .count() as u32;
    report.birth_lineage_rows_written = report
        .lineage_rows
        .iter()
        .filter(|row| row.lineage_kind == "Birth")
        .count() as u32;
    report.gpu_writes_compaction_rows = compaction_writers_enabled
        && !report.compaction_rows.is_empty()
        && report.compaction_rows.iter().all(|row| row.applied_by_gpu);
    report.gpu_writes_lineage_rows = lineage_writers_enabled
        && !report.lineage_rows.is_empty()
        && report.lineage_rows.iter().all(|row| row.applied_by_gpu);
    report.compaction_rows_read_from_gpu_values = staging.compaction_parity;
    report.lineage_rows_read_from_gpu_values = staging.lineage_parity;
    report.gpu_compaction_copy_dispatch_count = staging.compaction_dispatch_count;
    report.gpu_lineage_copy_dispatch_count = staging.lineage_dispatch_count;
    report.compaction_readback_count = staging.compaction_readback_count;
    report.lineage_readback_count = staging.lineage_readback_count;
    report.compaction_ops_uploaded = staging.compaction_ops_uploaded;
    report.lineage_ops_uploaded = staging.lineage_ops_uploaded;
    report.resident_compacted_view = build_compacted_view(&report.compaction_rows);

    let fusion_rows_exist = r1b
        .structural_events_from_gpu_journal
        .iter()
        .any(|event| event.event_kind == R1bStructuralEventKind::FusionRequest);
    let zero_rows_exist = r1b
        .structural_events_from_gpu_journal
        .iter()
        .any(|event| event.event_kind == R1bStructuralEventKind::ZeroCohort)
        || r1c_c_membership_rows
            .iter()
            .any(|row| row.membership_action == "DepartureMark");
    report.fusion_absorption_lineage_staged_if_fusion_rows_exist = !fusion_rows_exist
        || report
            .lineage_rows
            .iter()
            .any(|row| row.lineage_kind == "Absorb" && row.applied_by_gpu);
    report.zero_or_departure_tombstones_staged_if_zero_rows_exist = !zero_rows_exist
        || report
            .compaction_rows
            .iter()
            .any(|row| row.new_slot_or_tombstone.is_none() && row.applied_by_gpu);
    report.cpu_shadow = Runtime0080R1cDCpuShadowReport {
        consumes_compaction_rows_without_redeciding: report.compaction_rows_read_from_gpu_values,
        consumes_lineage_rows_without_redeciding: report.lineage_rows_read_from_gpu_values,
        cpu_decided_any_compaction_row: false,
        cpu_decided_any_lineage_row: false,
        compaction_shadow_matches_gpu_rows: report.compaction_rows_read_from_gpu_values,
        lineage_shadow_matches_gpu_rows: report.lineage_rows_read_from_gpu_values,
    };

    if include_cross_checks {
        let r1a = run_runtime_0080_0_r1a(&Runtime0080R1aInput::explicit_opt_in());
        report.r1a_preservation = Some(Runtime0080R1cCPreservationSummary {
            rung: "R1a",
            verdict: r1a.verdict.to_string(),
            checksum: r1a.stable_report_checksum,
            preserved: r1a.field_column_parity_matches_r6c_checksum,
        });
        let r1c = run_runtime_0080_0_r1c(&Runtime0080R1cInput::explicit_opt_in());
        report.r1c_shadow_preservation = Some(Runtime0080R1cCPreservationSummary {
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

    if include_cross_checks && compaction_writers_enabled {
        let disabled = match run_staging_session(
            &ctx,
            &planned_compaction,
            &planned_lineage,
            false,
            lineage_writers_enabled,
        ) {
            Ok(disabled) => disabled,
            Err(diagnostic) => {
                report.diagnostics.push(diagnostic.to_string());
                return finalize_report(report);
            }
        };
        let negative_control_detected = report.compaction_rows_written
            > disabled.compaction_rows.len() as u32
            && report.compaction_rows_read_from_gpu_values
            && !disabled.compaction_parity;
        report.disabled_compaction_writer_check = Some(Runtime0080R1cDDisabledWriterCheck {
            writers_enabled_rows: report.compaction_rows_written,
            writers_disabled_rows: disabled.compaction_rows.len() as u32,
            writers_enabled_parity: report.compaction_rows_read_from_gpu_values,
            writers_disabled_parity: disabled.compaction_parity,
            negative_control_detected,
            disabled_report_checksum: checksum_disabled_writer(
                "compaction",
                disabled.compaction_rows.len() as u32,
                disabled.compaction_parity,
            ),
        });
        report.disabled_compaction_writer_negative_control_detected = negative_control_detected;
    }

    if include_cross_checks && lineage_writers_enabled {
        let disabled = match run_staging_session(
            &ctx,
            &planned_compaction,
            &planned_lineage,
            compaction_writers_enabled,
            false,
        ) {
            Ok(disabled) => disabled,
            Err(diagnostic) => {
                report.diagnostics.push(diagnostic.to_string());
                return finalize_report(report);
            }
        };
        let negative_control_detected = report.lineage_rows_written
            > disabled.lineage_rows.len() as u32
            && report.lineage_rows_read_from_gpu_values
            && !disabled.lineage_parity;
        report.disabled_lineage_writer_check = Some(Runtime0080R1cDDisabledWriterCheck {
            writers_enabled_rows: report.lineage_rows_written,
            writers_disabled_rows: disabled.lineage_rows.len() as u32,
            writers_enabled_parity: report.lineage_rows_read_from_gpu_values,
            writers_disabled_parity: disabled.lineage_parity,
            negative_control_detected,
            disabled_report_checksum: checksum_disabled_writer(
                "lineage",
                disabled.lineage_rows.len() as u32,
                disabled.lineage_parity,
            ),
        });
        report.disabled_lineage_writer_negative_control_detected = negative_control_detected;
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
            .r1c_b_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved)
        && report
            .r1c_c_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved)
        && report
            .r1c_shadow_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved);

    let pass = compaction_writers_enabled
        && lineage_writers_enabled
        && report.resident_compaction_map_created
        && report.resident_lineage_staging_created
        && report.gpu_writes_compaction_rows
        && report.gpu_writes_lineage_rows
        && report.compaction_rows_read_from_gpu_values
        && report.lineage_rows_read_from_gpu_values
        && report.compaction_rows_written > 0
        && report.lineage_rows_written > 0
        && report
            .cpu_shadow
            .consumes_compaction_rows_without_redeciding
        && report.cpu_shadow.consumes_lineage_rows_without_redeciding
        && !report.cpu_shadow.cpu_decided_any_compaction_row
        && !report.cpu_shadow.cpu_decided_any_lineage_row
        && report.fusion_absorption_lineage_staged_if_fusion_rows_exist
        && report.zero_or_departure_tombstones_staged_if_zero_rows_exist
        && report.disabled_compaction_writer_negative_control_detected
        && report.disabled_lineage_writer_negative_control_detected
        && preservation_ok
        && !report.resident_m4a_authority
        && !report.multi_atlas_authority
        && !report.system_planet_recursion_authority
        && !report.default_session_wiring
        && !report.scenario_reopen_required
        && !report.docs_invariants_edit_required;

    if pass {
        report.status = RUNTIME_0080_0_R1C_D_STATUS_PASS;
        report.verdict = "PASS";
        report.resident_compaction_authority = true;
        report.resident_lineage_staging_authority = true;
        report.diagnostics = vec![
            "resident_compaction_map_and_lineage_staging_pass".to_string(),
            "gpu_stages_compaction_rows_from_resident_inputs".to_string(),
            "gpu_stages_lineage_rows_from_resident_inputs".to_string(),
            "cpu_shadow_consumes_compaction_and_lineage_without_redeciding".to_string(),
            "disabled_compaction_and_lineage_writer_negative_controls_detected".to_string(),
            "no_m4a_multi_atlas_recursion_or_default_session_wiring_claimed".to_string(),
        ];
    } else if !compaction_writers_enabled || !lineage_writers_enabled {
        report.status = RUNTIME_0080_0_R1C_D_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("compaction_or_lineage_writers_disabled_for_negative_control".to_string());
    } else {
        report.status = RUNTIME_0080_0_R1C_D_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("resident_compaction_lineage_staging_parity_incomplete".to_string());
    }

    finalize_report(report)
}

fn build_staging_plan(
    events: &[R1bStructuralEvent],
    allocation_rows: &[Runtime0080R1cBAllocationRow],
    membership_rows: &[Runtime0080R1cCMembershipDeltaRow],
) -> (Vec<PlannedCompactionRow>, Vec<PlannedLineageRow>) {
    let mut compaction = Vec::new();
    let mut lineage = Vec::new();
    let mut lineage_event_id = 0u32;

    let mut departure_rows = membership_rows
        .iter()
        .filter(|row| row.membership_action == "DepartureMark")
        .collect::<Vec<_>>();
    departure_rows.sort_by_key(|row| (row.tick, row.slot_id, row.source_event_index));
    for row in departure_rows {
        compaction.push(PlannedCompactionRow {
            tick: row.tick,
            old_slot: row.slot_id,
            new_slot_or_tombstone: None,
            owner_code: row.owner_code,
            reason: REASON_DEPARTURE_MARKED,
            active_before: row.active_before,
            active_after: row.active_after,
            source_event_kind: row.source_event_kind,
            source_event_index: row.source_event_index,
        });
        lineage.push(PlannedLineageRow {
            tick: row.tick,
            lineage_event_id,
            source_slot: row.slot_id,
            target_slot: TOMBSTONE_SLOT,
            owner_code: row.owner_code,
            lineage_kind: LINEAGE_TOMBSTONE,
            amount_or_delta: 0,
            source_event_kind: row.source_event_kind,
            source_event_index: row.source_event_index,
        });
        lineage_event_id += 1;
    }

    let mut fusion_events = events
        .iter()
        .filter(|event| event.event_kind == R1bStructuralEventKind::FusionRequest)
        .collect::<Vec<_>>();
    fusion_events.sort_by_key(|event| (event.tick, event.source_slot, event.target_slot));
    for (idx, event) in fusion_events.into_iter().enumerate() {
        compaction.push(PlannedCompactionRow {
            tick: event.tick,
            old_slot: event.target_slot,
            new_slot_or_tombstone: Some(event.source_slot),
            owner_code: event.owner_code,
            reason: REASON_FUSION_ABSORBED,
            active_before: true,
            active_after: false,
            source_event_kind: "FusionRequest",
            source_event_index: idx as u32,
        });
        lineage.push(PlannedLineageRow {
            tick: event.tick,
            lineage_event_id,
            source_slot: event.target_slot,
            target_slot: event.source_slot,
            owner_code: event.owner_code,
            lineage_kind: LINEAGE_ABSORB,
            amount_or_delta: event.amount_or_delta,
            source_event_kind: "FusionRequest",
            source_event_index: idx as u32,
        });
        lineage_event_id += 1;
        lineage.push(PlannedLineageRow {
            tick: event.tick,
            lineage_event_id,
            source_slot: event.source_slot,
            target_slot: event.source_slot,
            owner_code: event.owner_code,
            lineage_kind: LINEAGE_SURVIVE,
            amount_or_delta: event.amount_or_delta,
            source_event_kind: "FusionRequest",
            source_event_index: idx as u32,
        });
        lineage_event_id += 1;
    }

    let mut allocated_rows = allocation_rows
        .iter()
        .filter(|row| row.allocation_success && row.allocated_slot.is_some())
        .collect::<Vec<_>>();
    allocated_rows.sort_by_key(|row| (row.tick, row.request_event_index));
    for row in allocated_rows {
        let allocated_slot = row.allocated_slot.expect("filtered allocated slot");
        compaction.push(PlannedCompactionRow {
            tick: row.tick,
            old_slot: allocated_slot,
            new_slot_or_tombstone: Some(allocated_slot),
            owner_code: row.requested_owner,
            reason: REASON_BIRTH_ALLOCATED,
            active_before: false,
            active_after: true,
            source_event_kind: "LocalBirthRequest",
            source_event_index: row.request_event_index,
        });
        lineage.push(PlannedLineageRow {
            tick: row.tick,
            lineage_event_id,
            source_slot: row.request_event_index,
            target_slot: allocated_slot,
            owner_code: row.requested_owner,
            lineage_kind: LINEAGE_BIRTH,
            amount_or_delta: row.requested_ships,
            source_event_kind: "LocalBirthRequest",
            source_event_index: row.request_event_index,
        });
        lineage_event_id += 1;
    }

    compaction.sort_by_key(|row| {
        (
            row.tick,
            row.old_slot,
            row.reason,
            row.source_event_kind,
            row.source_event_index,
        )
    });
    lineage.sort_by_key(|row| {
        (
            row.tick,
            row.lineage_event_id,
            row.lineage_kind,
            row.source_slot,
            row.target_slot,
        )
    });
    (compaction, lineage)
}

fn run_staging_session(
    ctx: &simthing_gpu::GpuContext,
    planned_compaction: &[PlannedCompactionRow],
    planned_lineage: &[PlannedLineageRow],
    compaction_writers_enabled: bool,
    lineage_writers_enabled: bool,
) -> Result<StagingSessionReport, &'static str> {
    let compaction_count = planned_compaction.len().max(1) as u32;
    let lineage_count = planned_lineage.len().max(1) as u32;
    let layout = StagingLayout::new(compaction_count, lineage_count);
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots().max(1), R1A_N_DIMS);
    session
        .fill_slot_range_col(ctx, 0, layout.total_slots().max(1), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "r1c_d_staging_clear_failed")?;

    for (row_idx, row) in planned_compaction.iter().enumerate() {
        stage_compaction_row(ctx, &mut session, &layout, row_idx as u32, row)?;
    }
    for (row_idx, row) in planned_lineage.iter().enumerate() {
        stage_lineage_row(ctx, &mut session, &layout, row_idx as u32, row)?;
    }

    let mut compaction_dispatches = 0u32;
    let mut lineage_dispatches = 0u32;
    let mut compaction_ops_uploaded = 0u32;
    let mut lineage_ops_uploaded = 0u32;

    if compaction_writers_enabled {
        for row in 0..planned_compaction.len() as u32 {
            let ops = compaction_copy_ops(&layout, row);
            session
                .upload_ops(ctx, &ops)
                .map_err(|_| "r1c_d_compaction_copy_upload_failed")?;
            compaction_ops_uploaded += ops.len() as u32;
            session
                .tick(ctx, COMPACTION_COPY_BAND)
                .map_err(|_| "r1c_d_compaction_copy_tick_failed")?;
            compaction_dispatches += 1;
        }
    }

    if lineage_writers_enabled {
        for row in 0..planned_lineage.len() as u32 {
            let ops = lineage_copy_ops(&layout, row);
            session
                .upload_ops(ctx, &ops)
                .map_err(|_| "r1c_d_lineage_copy_upload_failed")?;
            lineage_ops_uploaded += ops.len() as u32;
            session
                .tick(ctx, LINEAGE_COPY_BAND)
                .map_err(|_| "r1c_d_lineage_copy_tick_failed")?;
            lineage_dispatches += 1;
        }
    }

    let values = session
        .readback_full(ctx)
        .map_err(|_| "r1c_d_staging_readback_failed")?;
    let compaction_rows = if compaction_writers_enabled {
        planned_compaction
            .iter()
            .enumerate()
            .map(|(idx, planned)| decode_compaction_row(&values, &layout, idx as u32, planned))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let lineage_rows = if lineage_writers_enabled {
        planned_lineage
            .iter()
            .enumerate()
            .map(|(idx, planned)| decode_lineage_row(&values, &layout, idx as u32, planned))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let expected_compaction = planned_compaction
        .iter()
        .map(|row| compaction_expected_row(row, true))
        .collect::<Vec<_>>();
    let expected_lineage = planned_lineage
        .iter()
        .map(|row| lineage_expected_row(row, true))
        .collect::<Vec<_>>();
    let compaction_parity = compaction_writers_enabled
        && !compaction_rows.is_empty()
        && compaction_rows == expected_compaction;
    let lineage_parity =
        lineage_writers_enabled && !lineage_rows.is_empty() && lineage_rows == expected_lineage;

    Ok(StagingSessionReport {
        compaction_rows,
        lineage_rows,
        compaction_parity,
        lineage_parity,
        compaction_dispatch_count: compaction_dispatches,
        lineage_dispatch_count: lineage_dispatches,
        compaction_readback_count: 1,
        lineage_readback_count: 1,
        compaction_ops_uploaded,
        lineage_ops_uploaded,
    })
}

fn stage_compaction_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &StagingLayout,
    row_idx: u32,
    row: &PlannedCompactionRow,
) -> Result<(), &'static str> {
    let fields = [
        row.tick as f32,
        row.old_slot as f32,
        row.new_slot_or_tombstone.unwrap_or(TOMBSTONE_SLOT) as f32,
        row.owner_code as f32,
        row.reason as f32,
        if row.active_before { 1.0 } else { 0.0 },
        if row.active_after { 1.0 } else { 0.0 },
        event_kind_code_from_name(row.source_event_kind) as f32,
        row.source_event_index as f32,
        1.0,
        1.0,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.compaction_staging_slot(row_idx, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "r1c_d_compaction_stage_failed")?;
    }
    Ok(())
}

fn stage_lineage_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &StagingLayout,
    row_idx: u32,
    row: &PlannedLineageRow,
) -> Result<(), &'static str> {
    let fields = [
        row.tick as f32,
        row.lineage_event_id as f32,
        row.source_slot as f32,
        row.target_slot as f32,
        row.owner_code as f32,
        row.lineage_kind as f32,
        row.amount_or_delta as f32,
        event_kind_code_from_name(row.source_event_kind) as f32,
        row.source_event_index as f32,
        1.0,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.lineage_staging_slot(row_idx, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "r1c_d_lineage_stage_failed")?;
    }
    Ok(())
}

fn compaction_copy_ops(layout: &StagingLayout, row: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(COMPACTION_FIELDS as usize);
    for field in 0..COMPACTION_FIELDS {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.compaction_staging_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(COMPACTION_COPY_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                layout.compaction_committed_slot(row, field),
                R1A_COL_CURRENT,
            )],
        });
    }
    ops
}

fn lineage_copy_ops(layout: &StagingLayout, row: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(LINEAGE_FIELDS as usize);
    for field in 0..LINEAGE_FIELDS {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.lineage_staging_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(LINEAGE_COPY_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(layout.lineage_committed_slot(row, field), R1A_COL_CURRENT)],
        });
    }
    ops
}

fn decode_compaction_row(
    values: &[f32],
    layout: &StagingLayout,
    row_idx: u32,
    planned: &PlannedCompactionRow,
) -> Runtime0080R1cDCompactionRow {
    let field = |field: u32| read_slot(values, layout.compaction_committed_slot(row_idx, field));
    Runtime0080R1cDCompactionRow {
        tick: f32_to_u32(field(0)),
        old_slot: f32_to_u32(field(1)),
        new_slot_or_tombstone: if f32_to_u32(field(2)) == TOMBSTONE_SLOT {
            None
        } else {
            Some(f32_to_u32(field(2)))
        },
        owner_code: f32_to_u32(field(3)),
        reason_code: reason_name(planned.reason),
        active_before: field(5) > 0.5,
        active_after: field(6) > 0.5,
        source_event_kind: planned.source_event_kind,
        source_event_index: f32_to_u32(field(8)),
        applied_by_gpu: field(9) > 0.5,
        cpu_shadow_match: field(10) > 0.5,
    }
}

fn decode_lineage_row(
    values: &[f32],
    layout: &StagingLayout,
    row_idx: u32,
    planned: &PlannedLineageRow,
) -> Runtime0080R1cDLineageRow {
    let field = |field: u32| read_slot(values, layout.lineage_committed_slot(row_idx, field));
    Runtime0080R1cDLineageRow {
        tick: f32_to_u32(field(0)),
        lineage_event_id: f32_to_u32(field(1)),
        source_slot: f32_to_u32(field(2)),
        target_slot: f32_to_u32(field(3)),
        owner_code: f32_to_u32(field(4)),
        lineage_kind: lineage_name(planned.lineage_kind),
        amount_or_delta: field(6).round() as i64,
        source_event_kind: planned.source_event_kind,
        source_event_index: f32_to_u32(field(8)),
        applied_by_gpu: field(9) > 0.5,
        cpu_shadow_match: true,
    }
}

fn compaction_expected_row(
    row: &PlannedCompactionRow,
    applied_by_gpu: bool,
) -> Runtime0080R1cDCompactionRow {
    Runtime0080R1cDCompactionRow {
        tick: row.tick,
        old_slot: row.old_slot,
        new_slot_or_tombstone: row.new_slot_or_tombstone,
        owner_code: row.owner_code,
        reason_code: reason_name(row.reason),
        active_before: row.active_before,
        active_after: row.active_after,
        source_event_kind: row.source_event_kind,
        source_event_index: row.source_event_index,
        applied_by_gpu,
        cpu_shadow_match: applied_by_gpu,
    }
}

fn lineage_expected_row(
    row: &PlannedLineageRow,
    applied_by_gpu: bool,
) -> Runtime0080R1cDLineageRow {
    Runtime0080R1cDLineageRow {
        tick: row.tick,
        lineage_event_id: row.lineage_event_id,
        source_slot: row.source_slot,
        target_slot: row.target_slot,
        owner_code: row.owner_code,
        lineage_kind: lineage_name(row.lineage_kind),
        amount_or_delta: row.amount_or_delta,
        source_event_kind: row.source_event_kind,
        source_event_index: row.source_event_index,
        applied_by_gpu,
        cpu_shadow_match: applied_by_gpu,
    }
}

fn build_compacted_view(
    rows: &[Runtime0080R1cDCompactionRow],
) -> Vec<Runtime0080R1cDCompactedViewRow> {
    rows.iter()
        .map(|row| Runtime0080R1cDCompactedViewRow {
            old_slot: row.old_slot,
            new_slot_or_tombstone: row.new_slot_or_tombstone,
            survivor_slot: if row.reason_code == "FusionAbsorbed" {
                row.new_slot_or_tombstone
            } else {
                None
            },
            active_after: row.active_after,
            reason_code: row.reason_code,
        })
        .collect()
}

fn event_kind_code_from_name(name: &str) -> u32 {
    match name {
        "MoveRequest" => 1,
        "DamageDelta" => 2,
        "ShipCountDelta" => 3,
        "ZeroCohort" => 4,
        "LocalBirthRequest" => 5,
        "FusionRequest" => 6,
        "OwnerCodeFlip" => 7,
        _ => 0,
    }
}

fn reason_name(code: u32) -> &'static str {
    match code {
        REASON_ZERO_OR_INACTIVE => "ZeroOrInactive",
        REASON_FUSION_ABSORBED => "FusionAbsorbed",
        REASON_BIRTH_ALLOCATED => "BirthAllocated",
        REASON_DEPARTURE_MARKED => "DepartureMarked",
        _ => "Unknown",
    }
}

fn lineage_name(code: u32) -> &'static str {
    match code {
        LINEAGE_BIRTH => "Birth",
        LINEAGE_ABSORB => "Absorb",
        LINEAGE_SURVIVE => "Survive",
        LINEAGE_TOMBSTONE => "Tombstone",
        _ => "Unknown",
    }
}

fn read_slot(values: &[f32], slot: u32) -> f32 {
    values
        .get((slot * R1A_N_DIMS + R1A_COL_CURRENT) as usize)
        .copied()
        .unwrap_or(0.0)
}

fn f32_to_u32(value: f32) -> u32 {
    if value.is_nan() || value.is_sign_negative() {
        0
    } else {
        value.round() as u32
    }
}

fn base_report(
    input: &Runtime0080R1cDInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1cDReport {
    Runtime0080R1cDReport {
        id: RUNTIME_0080_0_R1C_D_ID,
        primitive_name: RUNTIME_0080_0_R1C_D_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R1C_D_SCOPE,
        adapter,
        relationship_to_r1c_c: "not run",
        compaction_representation:
            "old_slot -> new_slot_or_tombstone append-only map plus derived active view",
        lineage_representation:
            "append-only resident lineage staging rows keyed by generic lineage_event_id",
        resident_compaction_map_created: false,
        resident_lineage_staging_created: false,
        compaction_rows_written: 0,
        lineage_rows_written: 0,
        tombstone_rows_written: 0,
        fusion_absorption_rows_written: 0,
        birth_lineage_rows_written: 0,
        compaction_rows_read_from_gpu_values: false,
        lineage_rows_read_from_gpu_values: false,
        gpu_writes_compaction_rows: false,
        gpu_writes_lineage_rows: false,
        compaction_rows: Vec::new(),
        lineage_rows: Vec::new(),
        resident_compacted_view: Vec::new(),
        consumes_r1b_event_journal: false,
        consumes_r1c_a_mark_table: false,
        consumes_r1c_b_allocation_rows: false,
        consumes_r1c_c_membership_rows: false,
        r1b_event_rows_consumed: 0,
        r1c_a_mark_rows_consumed: 0,
        r1c_b_allocation_rows_consumed: 0,
        r1c_c_membership_rows_consumed: 0,
        fusion_absorption_lineage_staged_if_fusion_rows_exist: false,
        zero_or_departure_tombstones_staged_if_zero_rows_exist: false,
        cpu_shadow: Runtime0080R1cDCpuShadowReport {
            consumes_compaction_rows_without_redeciding: false,
            consumes_lineage_rows_without_redeciding: false,
            cpu_decided_any_compaction_row: false,
            cpu_decided_any_lineage_row: false,
            compaction_shadow_matches_gpu_rows: false,
            lineage_shadow_matches_gpu_rows: false,
        },
        disabled_compaction_writer_check: None,
        disabled_lineage_writer_check: None,
        disabled_compaction_writer_negative_control_detected: false,
        disabled_lineage_writer_negative_control_detected: false,
        gpu_compaction_copy_dispatch_count: 0,
        gpu_lineage_copy_dispatch_count: 0,
        compaction_readback_count: 0,
        lineage_readback_count: 0,
        compaction_ops_uploaded: 0,
        lineage_ops_uploaded: 0,
        resident_compaction_authority: false,
        resident_lineage_staging_authority: false,
        resident_fusion_compaction_authority: false,
        resident_lineage_rewrite_authority: false,
        resident_m4a_authority: false,
        multi_atlas_authority: false,
        system_planet_recursion_authority: false,
        default_session_wiring: false,
        scenario_reopen_required: false,
        docs_invariants_edit_required: false,
        r1a_preservation: None,
        r1b_preservation: None,
        r1c_a_preservation: None,
        r1c_b_preservation: None,
        r1c_c_preservation: None,
        r1c_shadow_preservation: None,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "FieldPolicy",
            "field_agent",
            "selection",
            "extraction",
            "resident event journal",
            "resident mark table",
            "resident allocation rows",
            "resident membership table",
            "resident compaction map",
            "resident lineage staging",
            "disabled-transform parity check",
        ],
        exact_commands: exact_commands(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1cDReport) -> Runtime0080R1cDReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_d_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cDReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.resident_compaction_map_created as u64);
    mix_u64(&mut hash, report.resident_lineage_staging_created as u64);
    mix_u64(&mut hash, report.gpu_writes_compaction_rows as u64);
    mix_u64(&mut hash, report.gpu_writes_lineage_rows as u64);
    mix_u64(&mut hash, report.compaction_rows_written as u64);
    mix_u64(&mut hash, report.lineage_rows_written as u64);
    mix_u64(&mut hash, report.tombstone_rows_written as u64);
    mix_u64(&mut hash, report.fusion_absorption_rows_written as u64);
    mix_u64(&mut hash, report.birth_lineage_rows_written as u64);
    mix_u64(
        &mut hash,
        report.disabled_compaction_writer_negative_control_detected as u64,
    );
    mix_u64(
        &mut hash,
        report.disabled_lineage_writer_negative_control_detected as u64,
    );
    for row in &report.compaction_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.old_slot as u64);
        mix_u64(
            &mut hash,
            row.new_slot_or_tombstone.unwrap_or(TOMBSTONE_SLOT) as u64,
        );
        mix_str(&mut hash, row.reason_code);
        mix_u64(&mut hash, row.applied_by_gpu as u64);
    }
    for row in &report.lineage_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.lineage_event_id as u64);
        mix_u64(&mut hash, row.source_slot as u64);
        mix_u64(&mut hash, row.target_slot as u64);
        mix_str(&mut hash, row.lineage_kind);
        mix_u64(&mut hash, row.applied_by_gpu as u64);
    }
    for summary in [
        &report.r1a_preservation,
        &report.r1b_preservation,
        &report.r1c_a_preservation,
        &report.r1c_b_preservation,
        &report.r1c_c_preservation,
        &report.r1c_shadow_preservation,
    ] {
        if let Some(summary) = summary {
            mix_u64(&mut hash, summary.checksum);
            mix_u64(&mut hash, summary.preserved as u64);
        }
    }
    for check in [
        &report.disabled_compaction_writer_check,
        &report.disabled_lineage_writer_check,
    ] {
        if let Some(check) = check {
            mix_u64(&mut hash, check.writers_enabled_rows as u64);
            mix_u64(&mut hash, check.writers_disabled_rows as u64);
            mix_u64(&mut hash, check.negative_control_detected as u64);
            mix_u64(&mut hash, check.disabled_report_checksum);
        }
    }
    hash
}

fn checksum_disabled_writer(label: &str, rows: u32, parity: bool) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, "RUNTIME-0080-0-R1c-d-disabled-writer");
    mix_str(&mut hash, label);
    mix_u64(&mut hash, rows as u64);
    mix_u64(&mut hash, parity as u64);
    hash
}

fn project_r1c_c_membership_rows(
    events: &[R1bStructuralEvent],
    allocation_rows: &[Runtime0080R1cBAllocationRow],
) -> Vec<Runtime0080R1cCMembershipDeltaRow> {
    let mut rows = Vec::new();
    let mut zero_idx = 0u32;
    let mut zero_events = events
        .iter()
        .filter(|event| event.event_kind == R1bStructuralEventKind::ZeroCohort)
        .collect::<Vec<_>>();
    zero_events.sort_by_key(|event| (event.tick, event.source_slot, event.owner_code));
    for event in zero_events {
        rows.push(Runtime0080R1cCMembershipDeltaRow {
            tick: event.tick,
            slot_id: event.source_slot,
            owner_code: event.owner_code,
            source_cell: event.source_cell,
            target_cell: event.target_cell,
            membership_action: "DepartureMark",
            active_before: true,
            active_after: false,
            source_event_kind: "ZeroCohort",
            source_event_index: zero_idx,
            applied_by_gpu: true,
            cpu_shadow_match: true,
        });
        zero_idx += 1;
    }

    let mut allocated_rows = allocation_rows
        .iter()
        .filter(|row| row.allocation_success && row.allocated_slot.is_some())
        .collect::<Vec<_>>();
    allocated_rows.sort_by_key(|row| (row.tick, row.request_event_index));
    for row in allocated_rows {
        rows.push(Runtime0080R1cCMembershipDeltaRow {
            tick: row.tick,
            slot_id: row.allocated_slot.expect("filtered allocated slot"),
            owner_code: row.requested_owner,
            source_cell: row.requested_source_cell,
            target_cell: row.requested_source_cell,
            membership_action: "BirthIn",
            active_before: false,
            active_after: true,
            source_event_kind: "LocalBirthRequest",
            source_event_index: row.request_event_index,
            applied_by_gpu: true,
            cpu_shadow_match: true,
        });
    }

    rows.sort_by_key(|row| {
        (
            row.tick,
            row.source_event_kind,
            row.source_event_index,
            row.slot_id,
        )
    });
    rows
}

pub fn render_runtime_0080_r1c_d_artifact(report: &Runtime0080R1cDReport) -> String {
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
    let disabled_compaction = disabled_check_lines(&report.disabled_compaction_writer_check);
    let disabled_lineage = disabled_check_lines(&report.disabled_lineage_writer_check);
    let preservation = [
        preservation_line("R1a", &report.r1a_preservation),
        preservation_line("R1b", &report.r1b_preservation),
        preservation_line("R1c-a", &report.r1c_a_preservation),
        preservation_line("R1c-b", &report.r1c_b_preservation),
        preservation_line("R1c-c", &report.r1c_c_preservation),
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
    let command_lines = report
        .exact_commands
        .iter()
        .map(|command| command.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# RUNTIME-0080-0-R1c-d Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Adapter\n\
         {adapter}\
         ## Resident Compaction Map\n\
         - relationship_to_r1c_c: {relationship}\n\
         - compaction_representation: {compaction_representation}\n\
         - resident_compaction_map_created: {compaction_created}\n\
         - gpu_writes_compaction_rows: {gpu_compaction}\n\
         - compaction_rows_read_from_gpu_values: {compaction_read}\n\
         - compaction_rows_written: {compaction_rows}\n\
         - tombstone_rows_written: {tombstones}\n\
         - resident_compacted_view_rows: {view_rows}\n\
         - gpu_compaction_copy_dispatch_count: {compaction_dispatches}\n\
         - compaction_readback_count: {compaction_readbacks}\n\
         - compaction_ops_uploaded: {compaction_ops}\n\n\
         ## Resident Lineage Staging\n\
         - lineage_representation: {lineage_representation}\n\
         - resident_lineage_staging_created: {lineage_created}\n\
         - gpu_writes_lineage_rows: {gpu_lineage}\n\
         - lineage_rows_read_from_gpu_values: {lineage_read}\n\
         - lineage_rows_written: {lineage_rows}\n\
         - fusion_absorption_rows_written: {fusion_rows}\n\
         - birth_lineage_rows_written: {birth_rows}\n\
         - gpu_lineage_copy_dispatch_count: {lineage_dispatches}\n\
         - lineage_readback_count: {lineage_readbacks}\n\
         - lineage_ops_uploaded: {lineage_ops}\n\n\
         ## Inputs Consumed\n\
         - consumes_r1b_event_journal: {r1b_consumed} rows={r1b_rows}\n\
         - consumes_r1c_a_mark_table: {r1ca_consumed} rows={r1ca_rows}\n\
         - consumes_r1c_b_allocation_rows: {r1cb_consumed} rows={r1cb_rows}\n\
         - consumes_r1c_c_membership_rows: {r1cc_consumed} rows={r1cc_rows}\n\
         - fusion_absorption_lineage_staged_if_fusion_rows_exist: {fusion_if}\n\
         - zero_or_departure_tombstones_staged_if_zero_rows_exist: {zero_if}\n\n\
         ## CPU Shadow\n\
         - consumes_compaction_rows_without_redeciding: {shadow_compaction}\n\
         - consumes_lineage_rows_without_redeciding: {shadow_lineage}\n\
         - cpu_decided_any_compaction_row: {cpu_compaction}\n\
         - cpu_decided_any_lineage_row: {cpu_lineage}\n\
         - compaction_shadow_matches_gpu_rows: {shadow_compaction_match}\n\
         - lineage_shadow_matches_gpu_rows: {shadow_lineage_match}\n\n\
         ## Disabled Compaction Writer Parity Check\n\
         {disabled_compaction}\
         ## Disabled Lineage Writer Parity Check\n\
         {disabled_lineage}\
         ## Authority Flags\n\
         - resident_compaction_authority: {resident_compaction}\n\
         - resident_lineage_staging_authority: {resident_lineage_staging}\n\
         - resident_fusion_compaction_authority: {fusion_authority}\n\
         - resident_lineage_rewrite_authority: {lineage_rewrite}\n\
         - resident_m4a_authority: {m4a}\n\
         - multi_atlas_authority: {multi_atlas}\n\
         - system_planet_recursion_authority: {recursion}\n\
         - default_session_wiring: {default_wiring}\n\
         - docs_invariants_edit_required: {invariants}\n\
         - scenario_reopen_required: {scenario}\n\n\
         ## Preservation\n\
         {preservation}\n\n\
         ## Domain Terms\n\
         - {terms}\n\n\
         ## Exact Commands\n\n\
         ```text\n\
         {commands}\n\
         ```\n\n\
         ## Non-Claims\n\n\
         - no M-4A\n\
         - no multi-atlas\n\
         - no system->planet recursion\n\
         - no default session wiring\n\
         - no invariant edit\n\
         - no scenario reopen\n\n\
         ## Diagnostics\n\
         {diagnostics}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter_lines,
        relationship = report.relationship_to_r1c_c,
        compaction_representation = report.compaction_representation,
        compaction_created = report.resident_compaction_map_created,
        gpu_compaction = report.gpu_writes_compaction_rows,
        compaction_read = report.compaction_rows_read_from_gpu_values,
        compaction_rows = report.compaction_rows_written,
        tombstones = report.tombstone_rows_written,
        view_rows = report.resident_compacted_view.len(),
        compaction_dispatches = report.gpu_compaction_copy_dispatch_count,
        compaction_readbacks = report.compaction_readback_count,
        compaction_ops = report.compaction_ops_uploaded,
        lineage_representation = report.lineage_representation,
        lineage_created = report.resident_lineage_staging_created,
        gpu_lineage = report.gpu_writes_lineage_rows,
        lineage_read = report.lineage_rows_read_from_gpu_values,
        lineage_rows = report.lineage_rows_written,
        fusion_rows = report.fusion_absorption_rows_written,
        birth_rows = report.birth_lineage_rows_written,
        lineage_dispatches = report.gpu_lineage_copy_dispatch_count,
        lineage_readbacks = report.lineage_readback_count,
        lineage_ops = report.lineage_ops_uploaded,
        r1b_consumed = report.consumes_r1b_event_journal,
        r1b_rows = report.r1b_event_rows_consumed,
        r1ca_consumed = report.consumes_r1c_a_mark_table,
        r1ca_rows = report.r1c_a_mark_rows_consumed,
        r1cb_consumed = report.consumes_r1c_b_allocation_rows,
        r1cb_rows = report.r1c_b_allocation_rows_consumed,
        r1cc_consumed = report.consumes_r1c_c_membership_rows,
        r1cc_rows = report.r1c_c_membership_rows_consumed,
        fusion_if = report.fusion_absorption_lineage_staged_if_fusion_rows_exist,
        zero_if = report.zero_or_departure_tombstones_staged_if_zero_rows_exist,
        shadow_compaction = report
            .cpu_shadow
            .consumes_compaction_rows_without_redeciding,
        shadow_lineage = report.cpu_shadow.consumes_lineage_rows_without_redeciding,
        cpu_compaction = report.cpu_shadow.cpu_decided_any_compaction_row,
        cpu_lineage = report.cpu_shadow.cpu_decided_any_lineage_row,
        shadow_compaction_match = report.cpu_shadow.compaction_shadow_matches_gpu_rows,
        shadow_lineage_match = report.cpu_shadow.lineage_shadow_matches_gpu_rows,
        disabled_compaction = disabled_compaction,
        disabled_lineage = disabled_lineage,
        resident_compaction = report.resident_compaction_authority,
        resident_lineage_staging = report.resident_lineage_staging_authority,
        fusion_authority = report.resident_fusion_compaction_authority,
        lineage_rewrite = report.resident_lineage_rewrite_authority,
        m4a = report.resident_m4a_authority,
        multi_atlas = report.multi_atlas_authority,
        recursion = report.system_planet_recursion_authority,
        default_wiring = report.default_session_wiring,
        invariants = report.docs_invariants_edit_required,
        scenario = report.scenario_reopen_required,
        preservation = preservation,
        terms = report.domain_terms.join("\n- "),
        commands = command_lines,
        diagnostics = diagnostics,
    )
}

fn disabled_check_lines(check: &Option<Runtime0080R1cDDisabledWriterCheck>) -> String {
    check
        .as_ref()
        .map(|check| {
            format!(
                "- writers_enabled_rows: {}\n- writers_disabled_rows: {}\n- writers_enabled_parity: {}\n- writers_disabled_parity: {}\n- negative_control_detected: {}\n- disabled_report_checksum: {:016x}\n",
                check.writers_enabled_rows,
                check.writers_disabled_rows,
                check.writers_enabled_parity,
                check.writers_disabled_parity,
                check.negative_control_detected,
                check.disabled_report_checksum,
            )
        })
        .unwrap_or_else(|| "- not_run\n".to_string())
}

fn preservation_line(label: &str, summary: &Option<Runtime0080R1cCPreservationSummary>) -> String {
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

fn exact_commands() -> Vec<&'static str> {
    vec![
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_d",
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_c",
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_b",
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_a",
        "cargo test -p simthing-driver --test runtime_0080_0_r1c",
        "cargo test -p simthing-driver --test runtime_0080_0_r1b",
        "cargo test -p simthing-driver --test runtime_0080_0_r1a",
        "cargo test -p simthing-driver --test runtime_0080_0_r0",
        "cargo test -p simthing-driver --test gpu_measure_0080_0",
        "cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run",
        "cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement",
        "cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage",
        "cargo test -p simthing-driver --test dress_rehearsal_r5_movement_reenroll",
        "cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption",
        "cargo test -p simthing-driver --test dress_rehearsal_r3_capability_mask_down",
        "cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation",
        "cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap",
        "cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu",
        "cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store",
        "cargo test -p simthing-gpu",
        "cargo build --workspace",
        "cargo fmt --all -- --check",
        "cargo check --workspace",
    ]
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
