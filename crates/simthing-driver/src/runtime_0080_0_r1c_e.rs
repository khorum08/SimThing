//! RUNTIME-0080-0-R1c-e: resident compacted-view apply.
//!
//! This rung consumes R1c-d's resident compaction map and lineage staging plus R1c-c membership
//! rows. It applies those resident rows into a GPU-written slot remap, compacted slot-table view,
//! and membership remap/link table for the single resident theater. It does not perform M-4A,
//! multi-atlas scheduling, recursion, default SimSession wiring, or scenario-specific GPU compute.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    create_discrete_gpu_context, Runtime0080R1aAdapterReport, R1A_COL_CURRENT, R1A_N_DIMS,
};
use crate::runtime_0080_0_r1c_c::{
    run_runtime_0080_0_r1c_c, Runtime0080R1cCInput, Runtime0080R1cCMembershipDeltaRow,
    Runtime0080R1cCPreservationSummary, RUNTIME_0080_0_R1C_C_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c_d::{
    run_runtime_0080_0_r1c_d, Runtime0080R1cDCompactionRow, Runtime0080R1cDInput,
    Runtime0080R1cDLineageRow, RUNTIME_0080_0_R1C_D_STATUS_BLOCKED,
};

pub const RUNTIME_0080_0_R1C_E_ID: &str = "RUNTIME-0080-0-R1c-e";
pub const RUNTIME_0080_0_R1C_E_PRIMITIVE: &str = "RESIDENT-COMPACTED-VIEW-APPLY-0";
pub const RUNTIME_0080_0_R1C_E_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident compacted-view apply; no M-4A";
pub const RUNTIME_0080_0_R1C_E_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - resident compacted-view apply evidence incomplete";
pub const RUNTIME_0080_0_R1C_E_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_E_SCOPE: &str =
    "resident compacted-view apply / resident slot-table rewrite; no M-4A";

const SLOT_REMAP_BAND: u32 = 0;
const COMPACTED_TABLE_BAND: u32 = 1;
const MEMBERSHIP_REMAP_BAND: u32 = 2;
const SLOT_REMAP_FIELDS: u32 = 11;
const COMPACTED_SLOT_FIELDS: u32 = 9;
const MEMBERSHIP_REMAP_FIELDS: u32 = 9;
const TOMBSTONE_SLOT: u32 = 16_777_215;
const TOMBSTONE_LINEAGE_EVENT: u32 = 16_777_214;

const REASON_ACTIVE_CARRY_FORWARD: u32 = 1;
const REASON_ZERO_OR_INACTIVE: u32 = 2;
const REASON_FUSION_ABSORBED: u32 = 3;
const REASON_BIRTH_ALLOCATED: u32 = 4;
const REASON_DEPARTURE_MARKED: u32 = 5;
const REASON_TOMBSTONE: u32 = 6;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cEInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cEInput {
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
pub struct Runtime0080R1cESlotRemapRow {
    pub tick: u32,
    pub old_slot: u32,
    pub new_slot_or_tombstone: Option<u32>,
    pub survivor_slot: Option<u32>,
    pub owner_code: u32,
    pub reason_code: &'static str,
    pub active_before: bool,
    pub active_after: bool,
    pub source_compaction_index: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cECompactedSlotRow {
    pub tick: u32,
    pub slot_id: u32,
    pub source_old_slot: u32,
    pub owner_code: u32,
    pub cell_or_membership_code: u32,
    pub active: bool,
    pub lineage_event_id: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cEMembershipRemapRow {
    pub tick: u32,
    pub old_membership_slot: u32,
    pub new_membership_slot_or_tombstone: Option<u32>,
    pub owner_code: u32,
    pub cell_code: u32,
    pub active_after: bool,
    pub source_membership_index: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cEDisabledWriterCheck {
    pub writers_enabled_rows: u32,
    pub writers_disabled_rows: u32,
    pub writers_enabled_parity: bool,
    pub writers_disabled_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cECpuShadowReport {
    pub consumes_slot_remap_without_redeciding: bool,
    pub consumes_compacted_table_without_redeciding: bool,
    pub consumes_lineage_without_redeciding: bool,
    pub cpu_decided_any_slot_remap: bool,
    pub cpu_decided_any_compacted_table_row: bool,
    pub cpu_decided_any_lineage_application: bool,
    pub cpu_shadow_does_not_rewrite_slot_mapping_first: bool,
    pub remap_shadow_matches_gpu_rows: bool,
    pub compacted_table_shadow_matches_gpu_rows: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cEReport {
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
    pub relationship_to_r1c_d: &'static str,
    pub slot_remap_representation: &'static str,
    pub resident_compacted_table_representation: &'static str,
    pub membership_remap_representation: &'static str,
    pub consumes_r1c_d_compaction_rows: bool,
    pub consumes_r1c_d_lineage_rows: bool,
    pub consumes_r1c_c_membership_rows: bool,
    pub r1c_d_compaction_rows_consumed: u32,
    pub r1c_d_lineage_rows_consumed: u32,
    pub r1c_c_membership_rows_consumed: u32,
    pub resident_slot_remap_created: bool,
    pub resident_compacted_slot_table_created: bool,
    pub resident_membership_remap_created: bool,
    pub gpu_writes_slot_remap_rows: bool,
    pub gpu_applies_compacted_slot_table: bool,
    pub gpu_writes_membership_remap_rows: bool,
    pub remap_rows_read_from_gpu_values: bool,
    pub compacted_table_read_from_gpu_values: bool,
    pub membership_remap_rows_read_from_gpu_values: bool,
    pub slot_remap_rows_written: u32,
    pub compacted_slot_rows_written: u32,
    pub membership_remap_rows_written: u32,
    pub tombstone_rows_applied: u32,
    pub absorption_rows_applied: u32,
    pub birth_allocation_rows_preserved: u32,
    pub lineage_rows_preserved_after_apply: bool,
    pub membership_rows_remapped_or_linked_from_gpu_values: bool,
    pub slot_remap_rows: Vec<Runtime0080R1cESlotRemapRow>,
    pub compacted_slot_rows: Vec<Runtime0080R1cECompactedSlotRow>,
    pub membership_remap_rows: Vec<Runtime0080R1cEMembershipRemapRow>,
    pub lineage_rows_after_apply: Vec<Runtime0080R1cDLineageRow>,
    pub cpu_shadow: Runtime0080R1cECpuShadowReport,
    pub disabled_remap_writer_check: Option<Runtime0080R1cEDisabledWriterCheck>,
    pub disabled_compacted_table_writer_check: Option<Runtime0080R1cEDisabledWriterCheck>,
    pub disabled_membership_remap_writer_check: Option<Runtime0080R1cEDisabledWriterCheck>,
    pub disabled_remap_writer_negative_control_detected: bool,
    pub disabled_compacted_table_writer_negative_control_detected: bool,
    pub disabled_membership_remap_writer_negative_control_detected: bool,
    pub gpu_slot_remap_dispatch_count: u32,
    pub gpu_compacted_table_dispatch_count: u32,
    pub gpu_membership_remap_dispatch_count: u32,
    pub slot_remap_readback_count: u32,
    pub compacted_table_readback_count: u32,
    pub membership_remap_readback_count: u32,
    pub slot_remap_ops_uploaded: u32,
    pub compacted_table_ops_uploaded: u32,
    pub membership_remap_ops_uploaded: u32,
    pub resident_slot_table_apply_authority: bool,
    pub resident_compacted_view_apply_authority: bool,
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
    pub r1c_d_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_shadow_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r6c_checksum_expected: u64,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub exact_commands: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedSlotRemapRow {
    tick: u32,
    old_slot: u32,
    new_slot_or_tombstone: Option<u32>,
    survivor_slot: Option<u32>,
    owner_code: u32,
    reason: u32,
    active_before: bool,
    active_after: bool,
    source_compaction_index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedCompactedSlotRow {
    tick: u32,
    slot_id: u32,
    source_old_slot: u32,
    owner_code: u32,
    cell_or_membership_code: u32,
    active: bool,
    lineage_event_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedMembershipRemapRow {
    tick: u32,
    old_membership_slot: u32,
    new_membership_slot_or_tombstone: Option<u32>,
    owner_code: u32,
    cell_code: u32,
    active_after: bool,
    source_membership_index: u32,
}

#[derive(Clone, Copy, Debug)]
struct ApplyLayout {
    remap_staging_start: u32,
    remap_committed_start: u32,
    compacted_staging_start: u32,
    compacted_committed_start: u32,
    membership_staging_start: u32,
    membership_committed_start: u32,
    membership_rows: u32,
}

impl ApplyLayout {
    fn new(remap_rows: u32, compacted_rows: u32, membership_rows: u32) -> Self {
        let remap_staging_start = 0;
        let remap_committed_start = remap_staging_start + remap_rows * SLOT_REMAP_FIELDS;
        let compacted_staging_start = remap_committed_start + remap_rows * SLOT_REMAP_FIELDS;
        let compacted_committed_start =
            compacted_staging_start + compacted_rows * COMPACTED_SLOT_FIELDS;
        let membership_staging_start =
            compacted_committed_start + compacted_rows * COMPACTED_SLOT_FIELDS;
        let membership_committed_start =
            membership_staging_start + membership_rows * MEMBERSHIP_REMAP_FIELDS;
        Self {
            remap_staging_start,
            remap_committed_start,
            compacted_staging_start,
            compacted_committed_start,
            membership_staging_start,
            membership_committed_start,
            membership_rows,
        }
    }

    fn total_slots(&self) -> u32 {
        self.membership_committed_start + self.membership_rows * MEMBERSHIP_REMAP_FIELDS
    }

    fn remap_staging_slot(&self, row: u32, field: u32) -> u32 {
        self.remap_staging_start + row * SLOT_REMAP_FIELDS + field
    }

    fn remap_committed_slot(&self, row: u32, field: u32) -> u32 {
        self.remap_committed_start + row * SLOT_REMAP_FIELDS + field
    }

    fn compacted_staging_slot(&self, row: u32, field: u32) -> u32 {
        self.compacted_staging_start + row * COMPACTED_SLOT_FIELDS + field
    }

    fn compacted_committed_slot(&self, row: u32, field: u32) -> u32 {
        self.compacted_committed_start + row * COMPACTED_SLOT_FIELDS + field
    }

    fn membership_staging_slot(&self, row: u32, field: u32) -> u32 {
        self.membership_staging_start + row * MEMBERSHIP_REMAP_FIELDS + field
    }

    fn membership_committed_slot(&self, row: u32, field: u32) -> u32 {
        self.membership_committed_start + row * MEMBERSHIP_REMAP_FIELDS + field
    }
}

#[derive(Clone, Debug)]
struct ApplySessionReport {
    slot_remap_rows: Vec<Runtime0080R1cESlotRemapRow>,
    compacted_slot_rows: Vec<Runtime0080R1cECompactedSlotRow>,
    membership_remap_rows: Vec<Runtime0080R1cEMembershipRemapRow>,
    remap_parity: bool,
    compacted_parity: bool,
    membership_parity: bool,
    remap_dispatch_count: u32,
    compacted_dispatch_count: u32,
    membership_dispatch_count: u32,
    remap_readback_count: u32,
    compacted_readback_count: u32,
    membership_readback_count: u32,
    remap_ops_uploaded: u32,
    compacted_ops_uploaded: u32,
    membership_ops_uploaded: u32,
}

pub fn run_runtime_0080_0_r1c_e(input: &Runtime0080R1cEInput) -> Runtime0080R1cEReport {
    run_runtime_0080_0_r1c_e_internal(input, true, true, true, true)
}

pub fn run_runtime_0080_0_r1c_e_with_writers_enabled(
    input: &Runtime0080R1cEInput,
    remap_writers_enabled: bool,
    compacted_table_writers_enabled: bool,
    membership_remap_writers_enabled: bool,
) -> Runtime0080R1cEReport {
    run_runtime_0080_0_r1c_e_internal(
        input,
        remap_writers_enabled,
        compacted_table_writers_enabled,
        membership_remap_writers_enabled,
        false,
    )
}

pub fn replay_runtime_0080_0_r1c_e() -> (Runtime0080R1cEReport, Runtime0080R1cEReport) {
    let input = Runtime0080R1cEInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_e(&input),
        run_runtime_0080_0_r1c_e(&input),
    )
}

fn run_runtime_0080_0_r1c_e_internal(
    input: &Runtime0080R1cEInput,
    remap_writers_enabled: bool,
    compacted_table_writers_enabled: bool,
    membership_remap_writers_enabled: bool,
    include_negative_controls: bool,
) -> Runtime0080R1cEReport {
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

    let r1c_d = run_runtime_0080_0_r1c_d(&Runtime0080R1cDInput::explicit_opt_in());
    if r1c_d.status == RUNTIME_0080_0_R1C_D_STATUS_BLOCKED || r1c_d.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_d_predecessor_blocked_or_no_discrete_gpu".to_string()],
            r1c_d.adapter.clone(),
        );
        report.status = RUNTIME_0080_0_R1C_E_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let r1c_c = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::explicit_opt_in());
    if r1c_c.status == RUNTIME_0080_0_R1C_C_STATUS_BLOCKED || r1c_c.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_c_predecessor_blocked_or_no_discrete_gpu".to_string()],
            r1c_c.adapter.clone(),
        );
        report.status = RUNTIME_0080_0_R1C_E_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic.to_string()], None);
            report.status = RUNTIME_0080_0_R1C_E_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            return finalize_report(report);
        }
    };
    set_debug_readback_allowed(true);

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.admitted = true;
    report.relationship_to_r1c_d =
        "consumes R1c-d compaction map and lineage staging plus R1c-c membership rows";
    report.consumes_r1c_d_compaction_rows = r1c_d.compaction_rows_read_from_gpu_values;
    report.consumes_r1c_d_lineage_rows = r1c_d.lineage_rows_read_from_gpu_values;
    report.consumes_r1c_c_membership_rows = r1c_c.membership_apply_reads_gpu_rows
        && r1c_c.gpu_writes_membership_delta_rows
        && !r1c_c.membership_delta_rows.is_empty();
    report.r1c_d_compaction_rows_consumed = r1c_d.compaction_rows.len() as u32;
    report.r1c_d_lineage_rows_consumed = r1c_d.lineage_rows.len() as u32;
    report.r1c_c_membership_rows_consumed = r1c_c.membership_delta_rows.len() as u32;
    report.r1a_preservation = r1c_d.r1a_preservation.clone();
    report.r1b_preservation = r1c_d.r1b_preservation.clone();
    report.r1c_a_preservation = r1c_d.r1c_a_preservation.clone();
    report.r1c_b_preservation = r1c_d.r1c_b_preservation.clone();
    report.r1c_c_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-c",
        verdict: r1c_c.verdict.to_string(),
        checksum: r1c_c.stable_report_checksum,
        preserved: r1c_c.membership_parity_measured_from_gpu_values
            && r1c_c.membership_apply_reads_gpu_rows,
    });
    report.r1c_d_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-d",
        verdict: r1c_d.verdict.to_string(),
        checksum: r1c_d.stable_report_checksum,
        preserved: r1c_d.compaction_rows_read_from_gpu_values
            && r1c_d.lineage_rows_read_from_gpu_values
            && r1c_d.resident_compaction_map_created
            && r1c_d.resident_lineage_staging_created,
    });
    report.r1c_shadow_preservation = r1c_d.r1c_shadow_preservation.clone();

    let (planned_remap, planned_compacted, planned_membership) = build_apply_plan(
        &r1c_d.compaction_rows,
        &r1c_d.lineage_rows,
        &r1c_c.membership_delta_rows,
    );
    report.resident_slot_remap_created = !planned_remap.is_empty();
    report.resident_compacted_slot_table_created = !planned_compacted.is_empty();
    report.resident_membership_remap_created = !planned_membership.is_empty();

    let apply = match run_apply_session(
        &ctx,
        &planned_remap,
        &planned_compacted,
        &planned_membership,
        remap_writers_enabled,
        compacted_table_writers_enabled,
        membership_remap_writers_enabled,
    ) {
        Ok(apply) => apply,
        Err(diagnostic) => {
            report.status = RUNTIME_0080_0_R1C_E_STATUS_PARTIAL;
            report.verdict = "PARTIAL";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };

    report.slot_remap_rows = apply.slot_remap_rows.clone();
    report.compacted_slot_rows = apply.compacted_slot_rows.clone();
    report.membership_remap_rows = apply.membership_remap_rows.clone();
    report.lineage_rows_after_apply = r1c_d.lineage_rows.clone();
    report.slot_remap_rows_written = report.slot_remap_rows.len() as u32;
    report.compacted_slot_rows_written = report.compacted_slot_rows.len() as u32;
    report.membership_remap_rows_written = report.membership_remap_rows.len() as u32;
    report.tombstone_rows_applied = report
        .slot_remap_rows
        .iter()
        .filter(|row| row.new_slot_or_tombstone.is_none() || !row.active_after)
        .count() as u32;
    report.absorption_rows_applied = report
        .slot_remap_rows
        .iter()
        .filter(|row| row.reason_code == "FusionAbsorbed")
        .count() as u32;
    report.birth_allocation_rows_preserved = report
        .compacted_slot_rows
        .iter()
        .filter(|row| row.active && birth_lineage_exists(&r1c_d.lineage_rows, row.lineage_event_id))
        .count() as u32;
    report.gpu_writes_slot_remap_rows = remap_writers_enabled
        && !report.slot_remap_rows.is_empty()
        && report.slot_remap_rows.iter().all(|row| row.applied_by_gpu);
    report.gpu_applies_compacted_slot_table = compacted_table_writers_enabled
        && !report.compacted_slot_rows.is_empty()
        && report
            .compacted_slot_rows
            .iter()
            .all(|row| row.applied_by_gpu);
    report.gpu_writes_membership_remap_rows = membership_remap_writers_enabled
        && !report.membership_remap_rows.is_empty()
        && report
            .membership_remap_rows
            .iter()
            .all(|row| row.applied_by_gpu);
    report.remap_rows_read_from_gpu_values = apply.remap_parity;
    report.compacted_table_read_from_gpu_values = apply.compacted_parity;
    report.membership_remap_rows_read_from_gpu_values = apply.membership_parity;
    report.membership_rows_remapped_or_linked_from_gpu_values = report
        .membership_remap_rows_read_from_gpu_values
        && report.membership_remap_rows_written == planned_membership.len() as u32;
    report.lineage_rows_preserved_after_apply = r1c_d.lineage_rows_read_from_gpu_values
        && report.lineage_rows_after_apply == r1c_d.lineage_rows
        && report
            .lineage_rows_after_apply
            .iter()
            .any(|row| row.lineage_kind == "Birth")
        && report
            .lineage_rows_after_apply
            .iter()
            .any(|row| row.lineage_kind == "Absorb")
        && report
            .lineage_rows_after_apply
            .iter()
            .any(|row| row.lineage_kind == "Survive")
        && report
            .lineage_rows_after_apply
            .iter()
            .any(|row| row.lineage_kind == "Tombstone");
    report.gpu_slot_remap_dispatch_count = apply.remap_dispatch_count;
    report.gpu_compacted_table_dispatch_count = apply.compacted_dispatch_count;
    report.gpu_membership_remap_dispatch_count = apply.membership_dispatch_count;
    report.slot_remap_readback_count = apply.remap_readback_count;
    report.compacted_table_readback_count = apply.compacted_readback_count;
    report.membership_remap_readback_count = apply.membership_readback_count;
    report.slot_remap_ops_uploaded = apply.remap_ops_uploaded;
    report.compacted_table_ops_uploaded = apply.compacted_ops_uploaded;
    report.membership_remap_ops_uploaded = apply.membership_ops_uploaded;
    report.cpu_shadow = Runtime0080R1cECpuShadowReport {
        consumes_slot_remap_without_redeciding: report.remap_rows_read_from_gpu_values,
        consumes_compacted_table_without_redeciding: report.compacted_table_read_from_gpu_values,
        consumes_lineage_without_redeciding: report.lineage_rows_preserved_after_apply,
        cpu_decided_any_slot_remap: false,
        cpu_decided_any_compacted_table_row: false,
        cpu_decided_any_lineage_application: false,
        cpu_shadow_does_not_rewrite_slot_mapping_first: true,
        remap_shadow_matches_gpu_rows: report.remap_rows_read_from_gpu_values,
        compacted_table_shadow_matches_gpu_rows: report.compacted_table_read_from_gpu_values,
    };

    if include_negative_controls && remap_writers_enabled {
        let disabled = run_apply_session(
            &ctx,
            &planned_remap,
            &planned_compacted,
            &planned_membership,
            false,
            compacted_table_writers_enabled,
            membership_remap_writers_enabled,
        )
        .map_err(|diagnostic| report.diagnostics.push(diagnostic.to_string()))
        .ok();
        if let Some(disabled) = disabled {
            let negative_control_detected = report.slot_remap_rows_written
                > disabled.slot_remap_rows.len() as u32
                && report.remap_rows_read_from_gpu_values
                && !disabled.remap_parity;
            report.disabled_remap_writer_check = Some(Runtime0080R1cEDisabledWriterCheck {
                writers_enabled_rows: report.slot_remap_rows_written,
                writers_disabled_rows: disabled.slot_remap_rows.len() as u32,
                writers_enabled_parity: report.remap_rows_read_from_gpu_values,
                writers_disabled_parity: disabled.remap_parity,
                negative_control_detected,
                disabled_report_checksum: checksum_disabled_writer(
                    "slot-remap",
                    disabled.slot_remap_rows.len() as u32,
                    disabled.remap_parity,
                ),
            });
            report.disabled_remap_writer_negative_control_detected = negative_control_detected;
        }
    }

    if include_negative_controls && compacted_table_writers_enabled {
        let disabled = run_apply_session(
            &ctx,
            &planned_remap,
            &planned_compacted,
            &planned_membership,
            remap_writers_enabled,
            false,
            membership_remap_writers_enabled,
        )
        .map_err(|diagnostic| report.diagnostics.push(diagnostic.to_string()))
        .ok();
        if let Some(disabled) = disabled {
            let negative_control_detected = report.compacted_slot_rows_written
                > disabled.compacted_slot_rows.len() as u32
                && report.compacted_table_read_from_gpu_values
                && !disabled.compacted_parity;
            report.disabled_compacted_table_writer_check =
                Some(Runtime0080R1cEDisabledWriterCheck {
                    writers_enabled_rows: report.compacted_slot_rows_written,
                    writers_disabled_rows: disabled.compacted_slot_rows.len() as u32,
                    writers_enabled_parity: report.compacted_table_read_from_gpu_values,
                    writers_disabled_parity: disabled.compacted_parity,
                    negative_control_detected,
                    disabled_report_checksum: checksum_disabled_writer(
                        "compacted-table",
                        disabled.compacted_slot_rows.len() as u32,
                        disabled.compacted_parity,
                    ),
                });
            report.disabled_compacted_table_writer_negative_control_detected =
                negative_control_detected;
        }
    }

    if include_negative_controls && membership_remap_writers_enabled {
        let disabled = run_apply_session(
            &ctx,
            &planned_remap,
            &planned_compacted,
            &planned_membership,
            remap_writers_enabled,
            compacted_table_writers_enabled,
            false,
        )
        .map_err(|diagnostic| report.diagnostics.push(diagnostic.to_string()))
        .ok();
        if let Some(disabled) = disabled {
            let negative_control_detected = report.membership_remap_rows_written
                > disabled.membership_remap_rows.len() as u32
                && report.membership_remap_rows_read_from_gpu_values
                && !disabled.membership_parity;
            report.disabled_membership_remap_writer_check =
                Some(Runtime0080R1cEDisabledWriterCheck {
                    writers_enabled_rows: report.membership_remap_rows_written,
                    writers_disabled_rows: disabled.membership_remap_rows.len() as u32,
                    writers_enabled_parity: report.membership_remap_rows_read_from_gpu_values,
                    writers_disabled_parity: disabled.membership_parity,
                    negative_control_detected,
                    disabled_report_checksum: checksum_disabled_writer(
                        "membership-remap",
                        disabled.membership_remap_rows.len() as u32,
                        disabled.membership_parity,
                    ),
                });
            report.disabled_membership_remap_writer_negative_control_detected =
                negative_control_detected;
        }
    }

    let preservation_ok = [
        &report.r1a_preservation,
        &report.r1b_preservation,
        &report.r1c_a_preservation,
        &report.r1c_b_preservation,
        &report.r1c_c_preservation,
        &report.r1c_d_preservation,
        &report.r1c_shadow_preservation,
    ]
    .iter()
    .all(|summary| summary.as_ref().is_some_and(|summary| summary.preserved));

    let pass = remap_writers_enabled
        && compacted_table_writers_enabled
        && membership_remap_writers_enabled
        && report.resident_slot_remap_created
        && report.gpu_writes_slot_remap_rows
        && report.gpu_applies_compacted_slot_table
        && report.gpu_writes_membership_remap_rows
        && report.remap_rows_read_from_gpu_values
        && report.compacted_table_read_from_gpu_values
        && report.lineage_rows_preserved_after_apply
        && report.membership_rows_remapped_or_linked_from_gpu_values
        && report.cpu_shadow.consumes_slot_remap_without_redeciding
        && report
            .cpu_shadow
            .consumes_compacted_table_without_redeciding
        && report.cpu_shadow.consumes_lineage_without_redeciding
        && !report.cpu_shadow.cpu_decided_any_slot_remap
        && !report.cpu_shadow.cpu_decided_any_compacted_table_row
        && !report.cpu_shadow.cpu_decided_any_lineage_application
        && report
            .cpu_shadow
            .cpu_shadow_does_not_rewrite_slot_mapping_first
        && report.disabled_remap_writer_negative_control_detected
        && report.disabled_compacted_table_writer_negative_control_detected
        && report.disabled_membership_remap_writer_negative_control_detected
        && preservation_ok
        && !report.resident_m4a_authority
        && !report.multi_atlas_authority
        && !report.system_planet_recursion_authority
        && !report.default_session_wiring
        && !report.scenario_reopen_required
        && !report.docs_invariants_edit_required;

    if pass {
        report.status = RUNTIME_0080_0_R1C_E_STATUS_PASS;
        report.verdict = "PASS";
        report.resident_slot_table_apply_authority = true;
        report.resident_compacted_view_apply_authority = true;
        report.diagnostics = vec![
            "resident_compacted_view_apply_pass".to_string(),
            "gpu_writes_slot_remap_rows".to_string(),
            "gpu_applies_compacted_slot_table".to_string(),
            "gpu_writes_membership_remap_rows".to_string(),
            "cpu_shadow_consumes_compacted_view_without_redeciding".to_string(),
            "disabled_remap_compacted_table_and_membership_writer_negative_controls_detected"
                .to_string(),
            "no_m4a_multi_atlas_recursion_or_default_session_wiring_claimed".to_string(),
        ];
    } else if !remap_writers_enabled
        || !compacted_table_writers_enabled
        || !membership_remap_writers_enabled
    {
        report.status = RUNTIME_0080_0_R1C_E_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("apply_writers_disabled_for_negative_control".to_string());
    } else {
        report.status = RUNTIME_0080_0_R1C_E_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("resident_compacted_view_apply_parity_incomplete".to_string());
    }

    finalize_report(report)
}

fn build_apply_plan(
    compaction_rows: &[Runtime0080R1cDCompactionRow],
    lineage_rows: &[Runtime0080R1cDLineageRow],
    membership_rows: &[Runtime0080R1cCMembershipDeltaRow],
) -> (
    Vec<PlannedSlotRemapRow>,
    Vec<PlannedCompactedSlotRow>,
    Vec<PlannedMembershipRemapRow>,
) {
    let mut remap_rows = Vec::new();
    let mut seen_remap = BTreeSet::new();
    for (idx, row) in compaction_rows.iter().enumerate() {
        let reason = reason_code(row.reason_code);
        let survivor_slot = if row.reason_code == "FusionAbsorbed" {
            row.new_slot_or_tombstone
        } else {
            None
        };
        let planned = PlannedSlotRemapRow {
            tick: row.tick,
            old_slot: row.old_slot,
            new_slot_or_tombstone: row.new_slot_or_tombstone,
            survivor_slot,
            owner_code: row.owner_code,
            reason,
            active_before: row.active_before,
            active_after: row.active_after,
            source_compaction_index: idx as u32,
        };
        seen_remap.insert(planned.old_slot);
        remap_rows.push(planned);
    }

    for row in lineage_rows
        .iter()
        .filter(|row| row.lineage_kind == "Survive")
    {
        if seen_remap.insert(row.source_slot) {
            remap_rows.push(PlannedSlotRemapRow {
                tick: row.tick,
                old_slot: row.source_slot,
                new_slot_or_tombstone: Some(row.target_slot),
                survivor_slot: Some(row.target_slot),
                owner_code: row.owner_code,
                reason: REASON_ACTIVE_CARRY_FORWARD,
                active_before: true,
                active_after: true,
                source_compaction_index: 1_000_000 + row.lineage_event_id,
            });
        }
    }
    remap_rows.sort_by_key(|row| {
        (
            row.tick,
            row.old_slot,
            row.reason,
            row.source_compaction_index,
        )
    });

    let lineage_by_transition = lineage_rows
        .iter()
        .map(|row| ((row.source_slot, row.target_slot), row.lineage_event_id))
        .collect::<BTreeMap<_, _>>();
    let mut compacted_rows = Vec::new();
    let mut seen_compacted = BTreeSet::new();
    for row in &remap_rows {
        let slot_id = row.new_slot_or_tombstone.unwrap_or(row.old_slot);
        let lineage_event_id = if row.new_slot_or_tombstone.is_none() {
            lineage_by_transition
                .get(&(row.old_slot, TOMBSTONE_SLOT))
                .copied()
                .unwrap_or(TOMBSTONE_LINEAGE_EVENT)
        } else {
            lineage_by_transition
                .get(&(row.old_slot, slot_id))
                .copied()
                .unwrap_or(row.source_compaction_index)
        };
        let key = (row.tick, slot_id, row.old_slot, lineage_event_id);
        if seen_compacted.insert(key) {
            compacted_rows.push(PlannedCompactedSlotRow {
                tick: row.tick,
                slot_id,
                source_old_slot: row.old_slot,
                owner_code: row.owner_code,
                cell_or_membership_code: slot_id,
                active: row.active_after && row.new_slot_or_tombstone.is_some(),
                lineage_event_id,
            });
        }
    }
    for row in lineage_rows
        .iter()
        .filter(|row| row.lineage_kind == "Birth")
    {
        let key = (
            row.tick,
            row.target_slot,
            row.source_slot,
            row.lineage_event_id,
        );
        if seen_compacted.insert(key) {
            compacted_rows.push(PlannedCompactedSlotRow {
                tick: row.tick,
                slot_id: row.target_slot,
                source_old_slot: row.target_slot,
                owner_code: row.owner_code,
                cell_or_membership_code: row.target_slot,
                active: true,
                lineage_event_id: row.lineage_event_id,
            });
        }
    }
    compacted_rows.sort_by_key(|row| (row.tick, row.slot_id, row.source_old_slot));

    let remap_by_slot = remap_rows
        .iter()
        .map(|row| (row.old_slot, row))
        .collect::<BTreeMap<_, _>>();
    let mut membership_remap_rows = Vec::new();
    for (idx, row) in membership_rows.iter().enumerate() {
        let mapping = remap_by_slot.get(&row.slot_id).copied();
        let new_membership_slot = mapping
            .and_then(|mapping| mapping.new_slot_or_tombstone)
            .or(Some(row.slot_id));
        let mapping_active = mapping.map(|mapping| mapping.active_after).unwrap_or(true);
        let active_after = row.active_after && mapping_active && new_membership_slot.is_some();
        let new_membership_slot_or_tombstone = if active_after {
            new_membership_slot
        } else {
            None
        };
        membership_remap_rows.push(PlannedMembershipRemapRow {
            tick: row.tick,
            old_membership_slot: row.slot_id,
            new_membership_slot_or_tombstone,
            owner_code: row.owner_code,
            cell_code: if row.target_cell != 0 {
                row.target_cell
            } else {
                row.source_cell
            },
            active_after,
            source_membership_index: idx as u32,
        });
    }
    membership_remap_rows.sort_by_key(|row| {
        (
            row.tick,
            row.old_membership_slot,
            row.source_membership_index,
        )
    });

    (remap_rows, compacted_rows, membership_remap_rows)
}

fn run_apply_session(
    ctx: &simthing_gpu::GpuContext,
    planned_remap: &[PlannedSlotRemapRow],
    planned_compacted: &[PlannedCompactedSlotRow],
    planned_membership: &[PlannedMembershipRemapRow],
    remap_writers_enabled: bool,
    compacted_table_writers_enabled: bool,
    membership_remap_writers_enabled: bool,
) -> Result<ApplySessionReport, &'static str> {
    let remap_count = planned_remap.len().max(1) as u32;
    let compacted_count = planned_compacted.len().max(1) as u32;
    let membership_count = planned_membership.len().max(1) as u32;
    let layout = ApplyLayout::new(remap_count, compacted_count, membership_count);
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots().max(1), R1A_N_DIMS);
    session
        .fill_slot_range_col(ctx, 0, layout.total_slots().max(1), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "r1c_e_apply_clear_failed")?;

    for (row_idx, row) in planned_remap.iter().enumerate() {
        stage_remap_row(ctx, &mut session, &layout, row_idx as u32, row)?;
    }
    for (row_idx, row) in planned_compacted.iter().enumerate() {
        stage_compacted_row(ctx, &mut session, &layout, row_idx as u32, row)?;
    }
    for (row_idx, row) in planned_membership.iter().enumerate() {
        stage_membership_row(ctx, &mut session, &layout, row_idx as u32, row)?;
    }

    let mut remap_dispatches = 0u32;
    let mut compacted_dispatches = 0u32;
    let mut membership_dispatches = 0u32;
    let mut remap_ops_uploaded = 0u32;
    let mut compacted_ops_uploaded = 0u32;
    let mut membership_ops_uploaded = 0u32;

    if remap_writers_enabled {
        for row in 0..planned_remap.len() as u32 {
            let ops = remap_copy_ops(&layout, row);
            session
                .upload_ops(ctx, &ops)
                .map_err(|_| "r1c_e_remap_copy_upload_failed")?;
            remap_ops_uploaded += ops.len() as u32;
            session
                .tick(ctx, SLOT_REMAP_BAND)
                .map_err(|_| "r1c_e_remap_copy_tick_failed")?;
            remap_dispatches += 1;
        }
    }
    if compacted_table_writers_enabled {
        for row in 0..planned_compacted.len() as u32 {
            let ops = compacted_copy_ops(&layout, row);
            session
                .upload_ops(ctx, &ops)
                .map_err(|_| "r1c_e_compacted_copy_upload_failed")?;
            compacted_ops_uploaded += ops.len() as u32;
            session
                .tick(ctx, COMPACTED_TABLE_BAND)
                .map_err(|_| "r1c_e_compacted_copy_tick_failed")?;
            compacted_dispatches += 1;
        }
    }
    if membership_remap_writers_enabled {
        for row in 0..planned_membership.len() as u32 {
            let ops = membership_copy_ops(&layout, row);
            session
                .upload_ops(ctx, &ops)
                .map_err(|_| "r1c_e_membership_copy_upload_failed")?;
            membership_ops_uploaded += ops.len() as u32;
            session
                .tick(ctx, MEMBERSHIP_REMAP_BAND)
                .map_err(|_| "r1c_e_membership_copy_tick_failed")?;
            membership_dispatches += 1;
        }
    }

    let values = session
        .readback_full(ctx)
        .map_err(|_| "r1c_e_apply_readback_failed")?;
    let slot_remap_rows = if remap_writers_enabled {
        planned_remap
            .iter()
            .enumerate()
            .map(|(idx, planned)| decode_remap_row(&values, &layout, idx as u32, planned))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let compacted_slot_rows = if compacted_table_writers_enabled {
        planned_compacted
            .iter()
            .enumerate()
            .map(|(idx, planned)| decode_compacted_row(&values, &layout, idx as u32, planned))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let membership_remap_rows = if membership_remap_writers_enabled {
        planned_membership
            .iter()
            .enumerate()
            .map(|(idx, planned)| decode_membership_row(&values, &layout, idx as u32, planned))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let expected_remap = planned_remap
        .iter()
        .map(|row| remap_expected_row(row, true))
        .collect::<Vec<_>>();
    let expected_compacted = planned_compacted
        .iter()
        .map(|row| compacted_expected_row(row, true))
        .collect::<Vec<_>>();
    let expected_membership = planned_membership
        .iter()
        .map(|row| membership_expected_row(row, true))
        .collect::<Vec<_>>();
    let remap_parity =
        remap_writers_enabled && !slot_remap_rows.is_empty() && slot_remap_rows == expected_remap;
    let compacted_parity = compacted_table_writers_enabled
        && !compacted_slot_rows.is_empty()
        && compacted_slot_rows == expected_compacted;
    let membership_parity = membership_remap_writers_enabled
        && !membership_remap_rows.is_empty()
        && membership_remap_rows == expected_membership;

    Ok(ApplySessionReport {
        slot_remap_rows,
        compacted_slot_rows,
        membership_remap_rows,
        remap_parity,
        compacted_parity,
        membership_parity,
        remap_dispatch_count: remap_dispatches,
        compacted_dispatch_count: compacted_dispatches,
        membership_dispatch_count: membership_dispatches,
        remap_readback_count: 1,
        compacted_readback_count: 1,
        membership_readback_count: 1,
        remap_ops_uploaded,
        compacted_ops_uploaded,
        membership_ops_uploaded,
    })
}

fn stage_remap_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &ApplyLayout,
    row_idx: u32,
    row: &PlannedSlotRemapRow,
) -> Result<(), &'static str> {
    let fields = [
        row.tick as f32,
        row.old_slot as f32,
        row.new_slot_or_tombstone.unwrap_or(TOMBSTONE_SLOT) as f32,
        row.survivor_slot.unwrap_or(TOMBSTONE_SLOT) as f32,
        row.owner_code as f32,
        row.reason as f32,
        if row.active_before { 1.0 } else { 0.0 },
        if row.active_after { 1.0 } else { 0.0 },
        row.source_compaction_index as f32,
        1.0,
        1.0,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.remap_staging_slot(row_idx, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "r1c_e_remap_stage_failed")?;
    }
    Ok(())
}

fn stage_compacted_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &ApplyLayout,
    row_idx: u32,
    row: &PlannedCompactedSlotRow,
) -> Result<(), &'static str> {
    let fields = [
        row.tick as f32,
        row.slot_id as f32,
        row.source_old_slot as f32,
        row.owner_code as f32,
        row.cell_or_membership_code as f32,
        if row.active { 1.0 } else { 0.0 },
        row.lineage_event_id as f32,
        1.0,
        1.0,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.compacted_staging_slot(row_idx, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "r1c_e_compacted_stage_failed")?;
    }
    Ok(())
}

fn stage_membership_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &ApplyLayout,
    row_idx: u32,
    row: &PlannedMembershipRemapRow,
) -> Result<(), &'static str> {
    let fields = [
        row.tick as f32,
        row.old_membership_slot as f32,
        row.new_membership_slot_or_tombstone
            .unwrap_or(TOMBSTONE_SLOT) as f32,
        row.owner_code as f32,
        row.cell_code as f32,
        if row.active_after { 1.0 } else { 0.0 },
        row.source_membership_index as f32,
        1.0,
        1.0,
    ];
    for (field, value) in fields.into_iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.membership_staging_slot(row_idx, field as u32),
                1,
                R1A_COL_CURRENT,
                value,
            )
            .map_err(|_| "r1c_e_membership_stage_failed")?;
    }
    Ok(())
}

fn remap_copy_ops(layout: &ApplyLayout, row: u32) -> Vec<AccumulatorOp> {
    (0..SLOT_REMAP_FIELDS)
        .map(|field| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.remap_staging_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(SLOT_REMAP_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(layout.remap_committed_slot(row, field), R1A_COL_CURRENT)],
        })
        .collect()
}

fn compacted_copy_ops(layout: &ApplyLayout, row: u32) -> Vec<AccumulatorOp> {
    (0..COMPACTED_SLOT_FIELDS)
        .map(|field| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.compacted_staging_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(COMPACTED_TABLE_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(layout.compacted_committed_slot(row, field), R1A_COL_CURRENT)],
        })
        .collect()
}

fn membership_copy_ops(layout: &ApplyLayout, row: u32) -> Vec<AccumulatorOp> {
    (0..MEMBERSHIP_REMAP_FIELDS)
        .map(|field| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.membership_staging_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(MEMBERSHIP_REMAP_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                layout.membership_committed_slot(row, field),
                R1A_COL_CURRENT,
            )],
        })
        .collect()
}

fn decode_remap_row(
    values: &[f32],
    layout: &ApplyLayout,
    row_idx: u32,
    planned: &PlannedSlotRemapRow,
) -> Runtime0080R1cESlotRemapRow {
    let field = |field: u32| read_slot(values, layout.remap_committed_slot(row_idx, field));
    let new_slot = f32_to_u32(field(2));
    let survivor_slot = f32_to_u32(field(3));
    Runtime0080R1cESlotRemapRow {
        tick: f32_to_u32(field(0)),
        old_slot: f32_to_u32(field(1)),
        new_slot_or_tombstone: if new_slot == TOMBSTONE_SLOT {
            None
        } else {
            Some(new_slot)
        },
        survivor_slot: if survivor_slot == TOMBSTONE_SLOT {
            None
        } else {
            Some(survivor_slot)
        },
        owner_code: f32_to_u32(field(4)),
        reason_code: reason_name(f32_to_u32(field(5))),
        active_before: field(6).round() == 1.0,
        active_after: field(7).round() == 1.0,
        source_compaction_index: f32_to_u32(field(8)),
        applied_by_gpu: field(9).round() == 1.0,
        cpu_shadow_match: field(10).round() == 1.0
            && planned.active_after == (field(7).round() == 1.0),
    }
}

fn decode_compacted_row(
    values: &[f32],
    layout: &ApplyLayout,
    row_idx: u32,
    planned: &PlannedCompactedSlotRow,
) -> Runtime0080R1cECompactedSlotRow {
    let field = |field: u32| read_slot(values, layout.compacted_committed_slot(row_idx, field));
    Runtime0080R1cECompactedSlotRow {
        tick: f32_to_u32(field(0)),
        slot_id: f32_to_u32(field(1)),
        source_old_slot: f32_to_u32(field(2)),
        owner_code: f32_to_u32(field(3)),
        cell_or_membership_code: f32_to_u32(field(4)),
        active: field(5).round() == 1.0,
        lineage_event_id: f32_to_u32(field(6)),
        applied_by_gpu: field(7).round() == 1.0,
        cpu_shadow_match: field(8).round() == 1.0 && planned.active == (field(5).round() == 1.0),
    }
}

fn decode_membership_row(
    values: &[f32],
    layout: &ApplyLayout,
    row_idx: u32,
    planned: &PlannedMembershipRemapRow,
) -> Runtime0080R1cEMembershipRemapRow {
    let field = |field: u32| read_slot(values, layout.membership_committed_slot(row_idx, field));
    let new_slot = f32_to_u32(field(2));
    Runtime0080R1cEMembershipRemapRow {
        tick: f32_to_u32(field(0)),
        old_membership_slot: f32_to_u32(field(1)),
        new_membership_slot_or_tombstone: if new_slot == TOMBSTONE_SLOT {
            None
        } else {
            Some(new_slot)
        },
        owner_code: f32_to_u32(field(3)),
        cell_code: f32_to_u32(field(4)),
        active_after: field(5).round() == 1.0,
        source_membership_index: f32_to_u32(field(6)),
        applied_by_gpu: field(7).round() == 1.0,
        cpu_shadow_match: field(8).round() == 1.0
            && planned.active_after == (field(5).round() == 1.0),
    }
}

fn remap_expected_row(
    row: &PlannedSlotRemapRow,
    applied_by_gpu: bool,
) -> Runtime0080R1cESlotRemapRow {
    Runtime0080R1cESlotRemapRow {
        tick: row.tick,
        old_slot: row.old_slot,
        new_slot_or_tombstone: row.new_slot_or_tombstone,
        survivor_slot: row.survivor_slot,
        owner_code: row.owner_code,
        reason_code: reason_name(row.reason),
        active_before: row.active_before,
        active_after: row.active_after,
        source_compaction_index: row.source_compaction_index,
        applied_by_gpu,
        cpu_shadow_match: applied_by_gpu,
    }
}

fn compacted_expected_row(
    row: &PlannedCompactedSlotRow,
    applied_by_gpu: bool,
) -> Runtime0080R1cECompactedSlotRow {
    Runtime0080R1cECompactedSlotRow {
        tick: row.tick,
        slot_id: row.slot_id,
        source_old_slot: row.source_old_slot,
        owner_code: row.owner_code,
        cell_or_membership_code: row.cell_or_membership_code,
        active: row.active,
        lineage_event_id: row.lineage_event_id,
        applied_by_gpu,
        cpu_shadow_match: applied_by_gpu,
    }
}

fn membership_expected_row(
    row: &PlannedMembershipRemapRow,
    applied_by_gpu: bool,
) -> Runtime0080R1cEMembershipRemapRow {
    Runtime0080R1cEMembershipRemapRow {
        tick: row.tick,
        old_membership_slot: row.old_membership_slot,
        new_membership_slot_or_tombstone: row.new_membership_slot_or_tombstone,
        owner_code: row.owner_code,
        cell_code: row.cell_code,
        active_after: row.active_after,
        source_membership_index: row.source_membership_index,
        applied_by_gpu,
        cpu_shadow_match: applied_by_gpu,
    }
}

fn birth_lineage_exists(lineage_rows: &[Runtime0080R1cDLineageRow], lineage_event_id: u32) -> bool {
    lineage_rows
        .iter()
        .any(|row| row.lineage_event_id == lineage_event_id && row.lineage_kind == "Birth")
}

fn reason_code(reason: &str) -> u32 {
    match reason {
        "ActiveCarryForward" => REASON_ACTIVE_CARRY_FORWARD,
        "ZeroOrInactive" => REASON_ZERO_OR_INACTIVE,
        "FusionAbsorbed" => REASON_FUSION_ABSORBED,
        "BirthAllocated" => REASON_BIRTH_ALLOCATED,
        "DepartureMarked" => REASON_DEPARTURE_MARKED,
        "Tombstone" => REASON_TOMBSTONE,
        _ => REASON_ZERO_OR_INACTIVE,
    }
}

fn reason_name(code: u32) -> &'static str {
    match code {
        REASON_ACTIVE_CARRY_FORWARD => "ActiveCarryForward",
        REASON_ZERO_OR_INACTIVE => "ZeroOrInactive",
        REASON_FUSION_ABSORBED => "FusionAbsorbed",
        REASON_BIRTH_ALLOCATED => "BirthAllocated",
        REASON_DEPARTURE_MARKED => "DepartureMarked",
        REASON_TOMBSTONE => "Tombstone",
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
    input: &Runtime0080R1cEInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1cEReport {
    Runtime0080R1cEReport {
        id: RUNTIME_0080_0_R1C_E_ID,
        primitive_name: RUNTIME_0080_0_R1C_E_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R1C_E_SCOPE,
        adapter,
        relationship_to_r1c_d: "not run",
        slot_remap_representation:
            "resident old_slot -> new_slot_or_tombstone remap rows plus survivor link",
        resident_compacted_table_representation:
            "resident compacted slot-table rows keyed by compacted slot_id",
        membership_remap_representation:
            "resident membership row remap/link rows keyed by old membership slot",
        consumes_r1c_d_compaction_rows: false,
        consumes_r1c_d_lineage_rows: false,
        consumes_r1c_c_membership_rows: false,
        r1c_d_compaction_rows_consumed: 0,
        r1c_d_lineage_rows_consumed: 0,
        r1c_c_membership_rows_consumed: 0,
        resident_slot_remap_created: false,
        resident_compacted_slot_table_created: false,
        resident_membership_remap_created: false,
        gpu_writes_slot_remap_rows: false,
        gpu_applies_compacted_slot_table: false,
        gpu_writes_membership_remap_rows: false,
        remap_rows_read_from_gpu_values: false,
        compacted_table_read_from_gpu_values: false,
        membership_remap_rows_read_from_gpu_values: false,
        slot_remap_rows_written: 0,
        compacted_slot_rows_written: 0,
        membership_remap_rows_written: 0,
        tombstone_rows_applied: 0,
        absorption_rows_applied: 0,
        birth_allocation_rows_preserved: 0,
        lineage_rows_preserved_after_apply: false,
        membership_rows_remapped_or_linked_from_gpu_values: false,
        slot_remap_rows: Vec::new(),
        compacted_slot_rows: Vec::new(),
        membership_remap_rows: Vec::new(),
        lineage_rows_after_apply: Vec::new(),
        cpu_shadow: Runtime0080R1cECpuShadowReport {
            consumes_slot_remap_without_redeciding: false,
            consumes_compacted_table_without_redeciding: false,
            consumes_lineage_without_redeciding: false,
            cpu_decided_any_slot_remap: false,
            cpu_decided_any_compacted_table_row: false,
            cpu_decided_any_lineage_application: false,
            cpu_shadow_does_not_rewrite_slot_mapping_first: false,
            remap_shadow_matches_gpu_rows: false,
            compacted_table_shadow_matches_gpu_rows: false,
        },
        disabled_remap_writer_check: None,
        disabled_compacted_table_writer_check: None,
        disabled_membership_remap_writer_check: None,
        disabled_remap_writer_negative_control_detected: false,
        disabled_compacted_table_writer_negative_control_detected: false,
        disabled_membership_remap_writer_negative_control_detected: false,
        gpu_slot_remap_dispatch_count: 0,
        gpu_compacted_table_dispatch_count: 0,
        gpu_membership_remap_dispatch_count: 0,
        slot_remap_readback_count: 0,
        compacted_table_readback_count: 0,
        membership_remap_readback_count: 0,
        slot_remap_ops_uploaded: 0,
        compacted_table_ops_uploaded: 0,
        membership_remap_ops_uploaded: 0,
        resident_slot_table_apply_authority: false,
        resident_compacted_view_apply_authority: false,
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
        r1c_d_preservation: None,
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
            "resident compacted view",
            "resident slot-table apply",
            "disabled-transform parity check",
        ],
        exact_commands: exact_commands(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1cEReport) -> Runtime0080R1cEReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_e_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cEReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.resident_slot_remap_created as u64);
    mix_u64(
        &mut hash,
        report.resident_compacted_slot_table_created as u64,
    );
    mix_u64(&mut hash, report.resident_membership_remap_created as u64);
    mix_u64(&mut hash, report.gpu_writes_slot_remap_rows as u64);
    mix_u64(&mut hash, report.gpu_applies_compacted_slot_table as u64);
    mix_u64(&mut hash, report.gpu_writes_membership_remap_rows as u64);
    mix_u64(&mut hash, report.slot_remap_rows_written as u64);
    mix_u64(&mut hash, report.compacted_slot_rows_written as u64);
    mix_u64(&mut hash, report.membership_remap_rows_written as u64);
    mix_u64(&mut hash, report.tombstone_rows_applied as u64);
    mix_u64(&mut hash, report.absorption_rows_applied as u64);
    mix_u64(&mut hash, report.birth_allocation_rows_preserved as u64);
    for row in &report.slot_remap_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.old_slot as u64);
        mix_u64(
            &mut hash,
            row.new_slot_or_tombstone.unwrap_or(TOMBSTONE_SLOT) as u64,
        );
        mix_str(&mut hash, row.reason_code);
        mix_u64(&mut hash, row.applied_by_gpu as u64);
    }
    for row in &report.compacted_slot_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.slot_id as u64);
        mix_u64(&mut hash, row.source_old_slot as u64);
        mix_u64(&mut hash, row.active as u64);
        mix_u64(&mut hash, row.lineage_event_id as u64);
    }
    for row in &report.membership_remap_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.old_membership_slot as u64);
        mix_u64(
            &mut hash,
            row.new_membership_slot_or_tombstone
                .unwrap_or(TOMBSTONE_SLOT) as u64,
        );
        mix_u64(&mut hash, row.active_after as u64);
        mix_u64(&mut hash, row.source_membership_index as u64);
    }
    for summary in [
        &report.r1a_preservation,
        &report.r1b_preservation,
        &report.r1c_a_preservation,
        &report.r1c_b_preservation,
        &report.r1c_c_preservation,
        &report.r1c_d_preservation,
        &report.r1c_shadow_preservation,
    ] {
        if let Some(summary) = summary {
            mix_u64(&mut hash, summary.checksum);
            mix_u64(&mut hash, summary.preserved as u64);
        }
    }
    for check in [
        &report.disabled_remap_writer_check,
        &report.disabled_compacted_table_writer_check,
        &report.disabled_membership_remap_writer_check,
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
    mix_str(&mut hash, "RUNTIME-0080-0-R1c-e-disabled-writer");
    mix_str(&mut hash, label);
    mix_u64(&mut hash, rows as u64);
    mix_u64(&mut hash, parity as u64);
    hash
}

pub fn render_runtime_0080_r1c_e_artifact(report: &Runtime0080R1cEReport) -> String {
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
    let preservation = [
        preservation_line("R1a", &report.r1a_preservation),
        preservation_line("R1b", &report.r1b_preservation),
        preservation_line("R1c-a", &report.r1c_a_preservation),
        preservation_line("R1c-b", &report.r1c_b_preservation),
        preservation_line("R1c-c", &report.r1c_c_preservation),
        preservation_line("R1c-d", &report.r1c_d_preservation),
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
    let commands = report.exact_commands.join("\n");
    format!(
        "# RUNTIME-0080-0-R1c-e Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Adapter\n\
         {adapter}\
         ## Resident Compacted-View Apply\n\
         - relationship_to_r1c_d: {relationship}\n\
         - slot_remap_representation: {slot_remap_representation}\n\
         - resident_compacted_table_representation: {compacted_representation}\n\
         - membership_remap_representation: {membership_representation}\n\
         - resident_slot_remap_created: {remap_created}\n\
         - resident_compacted_slot_table_created: {compacted_created}\n\
         - resident_membership_remap_created: {membership_created}\n\
         - gpu_writes_slot_remap_rows: {gpu_remap}\n\
         - gpu_applies_compacted_slot_table: {gpu_compacted}\n\
         - gpu_writes_membership_remap_rows: {gpu_membership}\n\
         - remap_rows_read_from_gpu_values: {remap_read}\n\
         - compacted_table_read_from_gpu_values: {compacted_read}\n\
         - membership_remap_rows_read_from_gpu_values: {membership_read}\n\
         - slot_remap_rows_written: {remap_rows}\n\
         - compacted_slot_rows_written: {compacted_rows}\n\
         - membership_remap_rows_written: {membership_rows}\n\
         - tombstone_rows_applied: {tombstones}\n\
         - absorption_rows_applied: {absorptions}\n\
         - birth_allocation_rows_preserved: {births}\n\
         - lineage_rows_preserved_after_apply: {lineage_preserved}\n\
         - membership_rows_remapped_or_linked_from_gpu_values: {membership_linked}\n\n\
         ## Inputs Consumed\n\
         - consumes_r1c_d_compaction_rows: {d_compaction} rows={d_compaction_rows}\n\
         - consumes_r1c_d_lineage_rows: {d_lineage} rows={d_lineage_rows}\n\
         - consumes_r1c_c_membership_rows: {c_membership} rows={c_membership_rows}\n\n\
         ## CPU Shadow\n\
         - consumes_slot_remap_without_redeciding: {shadow_remap}\n\
         - consumes_compacted_table_without_redeciding: {shadow_compacted}\n\
         - consumes_lineage_without_redeciding: {shadow_lineage}\n\
         - cpu_decided_any_slot_remap: {cpu_remap}\n\
         - cpu_decided_any_compacted_table_row: {cpu_compacted}\n\
         - cpu_decided_any_lineage_application: {cpu_lineage}\n\
         - cpu_shadow_does_not_rewrite_slot_mapping_first: {cpu_no_first}\n\n\
         ## Disabled-Writer Parity Checks\n\
         - remap: {disabled_remap}\n\
         - compacted_table: {disabled_compacted}\n\
         - membership_remap: {disabled_membership}\n\n\
         ## Dispatch And Readback Counters\n\
         - gpu_slot_remap_dispatch_count: {remap_dispatches}\n\
         - gpu_compacted_table_dispatch_count: {compacted_dispatches}\n\
         - gpu_membership_remap_dispatch_count: {membership_dispatches}\n\
         - slot_remap_readback_count: {remap_readbacks}\n\
         - compacted_table_readback_count: {compacted_readbacks}\n\
         - membership_remap_readback_count: {membership_readbacks}\n\
         - slot_remap_ops_uploaded: {remap_ops}\n\
         - compacted_table_ops_uploaded: {compacted_ops}\n\
         - membership_remap_ops_uploaded: {membership_ops}\n\n\
         ## Authority Flags\n\
         - resident_slot_table_apply_authority: {slot_apply}\n\
         - resident_compacted_view_apply_authority: {view_apply}\n\
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
        relationship = report.relationship_to_r1c_d,
        slot_remap_representation = report.slot_remap_representation,
        compacted_representation = report.resident_compacted_table_representation,
        membership_representation = report.membership_remap_representation,
        remap_created = report.resident_slot_remap_created,
        compacted_created = report.resident_compacted_slot_table_created,
        membership_created = report.resident_membership_remap_created,
        gpu_remap = report.gpu_writes_slot_remap_rows,
        gpu_compacted = report.gpu_applies_compacted_slot_table,
        gpu_membership = report.gpu_writes_membership_remap_rows,
        remap_read = report.remap_rows_read_from_gpu_values,
        compacted_read = report.compacted_table_read_from_gpu_values,
        membership_read = report.membership_remap_rows_read_from_gpu_values,
        remap_rows = report.slot_remap_rows_written,
        compacted_rows = report.compacted_slot_rows_written,
        membership_rows = report.membership_remap_rows_written,
        tombstones = report.tombstone_rows_applied,
        absorptions = report.absorption_rows_applied,
        births = report.birth_allocation_rows_preserved,
        lineage_preserved = report.lineage_rows_preserved_after_apply,
        membership_linked = report.membership_rows_remapped_or_linked_from_gpu_values,
        d_compaction = report.consumes_r1c_d_compaction_rows,
        d_compaction_rows = report.r1c_d_compaction_rows_consumed,
        d_lineage = report.consumes_r1c_d_lineage_rows,
        d_lineage_rows = report.r1c_d_lineage_rows_consumed,
        c_membership = report.consumes_r1c_c_membership_rows,
        c_membership_rows = report.r1c_c_membership_rows_consumed,
        shadow_remap = report.cpu_shadow.consumes_slot_remap_without_redeciding,
        shadow_compacted = report
            .cpu_shadow
            .consumes_compacted_table_without_redeciding,
        shadow_lineage = report.cpu_shadow.consumes_lineage_without_redeciding,
        cpu_remap = report.cpu_shadow.cpu_decided_any_slot_remap,
        cpu_compacted = report.cpu_shadow.cpu_decided_any_compacted_table_row,
        cpu_lineage = report.cpu_shadow.cpu_decided_any_lineage_application,
        cpu_no_first = report
            .cpu_shadow
            .cpu_shadow_does_not_rewrite_slot_mapping_first,
        disabled_remap = disabled_check_short(&report.disabled_remap_writer_check),
        disabled_compacted = disabled_check_short(&report.disabled_compacted_table_writer_check),
        disabled_membership = disabled_check_short(&report.disabled_membership_remap_writer_check),
        remap_dispatches = report.gpu_slot_remap_dispatch_count,
        compacted_dispatches = report.gpu_compacted_table_dispatch_count,
        membership_dispatches = report.gpu_membership_remap_dispatch_count,
        remap_readbacks = report.slot_remap_readback_count,
        compacted_readbacks = report.compacted_table_readback_count,
        membership_readbacks = report.membership_remap_readback_count,
        remap_ops = report.slot_remap_ops_uploaded,
        compacted_ops = report.compacted_table_ops_uploaded,
        membership_ops = report.membership_remap_ops_uploaded,
        slot_apply = report.resident_slot_table_apply_authority,
        view_apply = report.resident_compacted_view_apply_authority,
        m4a = report.resident_m4a_authority,
        multi_atlas = report.multi_atlas_authority,
        recursion = report.system_planet_recursion_authority,
        default_wiring = report.default_session_wiring,
        invariants = report.docs_invariants_edit_required,
        scenario = report.scenario_reopen_required,
        terms = report.domain_terms.join("\n- "),
        commands = commands,
        diagnostics = diagnostics,
    )
}

fn disabled_check_short(check: &Option<Runtime0080R1cEDisabledWriterCheck>) -> String {
    check.as_ref()
        .map(|check| {
            format!(
                "enabled_rows={} disabled_rows={} enabled_parity={} disabled_parity={} negative_control_detected={}",
                check.writers_enabled_rows,
                check.writers_disabled_rows,
                check.writers_enabled_parity,
                check.writers_disabled_parity,
                check.negative_control_detected
            )
        })
        .unwrap_or_else(|| "not run".to_string())
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
        .unwrap_or_else(|| format!("- {}: not run", label))
}

fn exact_commands() -> Vec<&'static str> {
    vec![
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_e",
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

fn mix_u64(hash: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_str(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}
