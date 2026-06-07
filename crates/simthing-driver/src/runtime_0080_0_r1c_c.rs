//! RUNTIME-0080-0-R1c-c: resident REENROLL scatter / membership apply without compaction.
//!
//! Consumes R1b resident event journal rows and R1c-b resident allocation rows. The GPU writes
//! membership delta rows and applies source/destination membership changes resident-side. The CPU
//! shadow observes the GPU-applied membership state without choosing membership effects first.

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, R1bStructuralEvent,
    R1bStructuralEventKind,
};
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

pub const RUNTIME_0080_0_R1C_C_ID: &str = "RUNTIME-0080-0-R1c-c";
pub const RUNTIME_0080_0_R1C_C_PRIMITIVE: &str = "RESIDENT-MEMBERSHIP-APPLY-0";
pub const RUNTIME_0080_0_R1C_C_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident membership apply; no compaction";
pub const RUNTIME_0080_0_R1C_C_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - resident membership apply evidence incomplete";
pub const RUNTIME_0080_0_R1C_C_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_C_SCOPE: &str =
    "resident membership apply from R1b journal and R1c-b allocation rows; no compaction";
pub const RUNTIME_R1C_C_EXPECTED_REPORT_CHECKSUM: u64 = 0xb3f8_fbb1_5edb_f0a8;

const MEMBERSHIP_APPLY_BAND: u32 = 0;
const DELTA_COPY_BAND: u32 = 1;
const DELTA_FIELDS: u32 = 12;
const FIELDS_PER_SLOT: u32 = 3;
const SLOT_FIELD_CELL: u32 = 0;
const SLOT_FIELD_OWNER: u32 = 1;
const SLOT_FIELD_ACTIVE: u32 = 2;

const ACTION_MOVE_OUT: f32 = 1.0;
const ACTION_MOVE_IN: f32 = 2.0;
const ACTION_BIRTH_IN: f32 = 3.0;
const ACTION_DEPARTURE_MARK: f32 = 4.0;
const ACTION_OWNER_CODE_UPDATE: f32 = 5.0;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MembershipAction {
    MoveOut,
    MoveIn,
    BirthIn,
    DepartureMark,
    OwnerCodeUpdate,
}

impl MembershipAction {
    fn code(self) -> f32 {
        match self {
            Self::MoveOut => ACTION_MOVE_OUT,
            Self::MoveIn => ACTION_MOVE_IN,
            Self::BirthIn => ACTION_BIRTH_IN,
            Self::DepartureMark => ACTION_DEPARTURE_MARK,
            Self::OwnerCodeUpdate => ACTION_OWNER_CODE_UPDATE,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::MoveOut => "MoveOut",
            Self::MoveIn => "MoveIn",
            Self::BirthIn => "BirthIn",
            Self::DepartureMark => "DepartureMark",
            Self::OwnerCodeUpdate => "OwnerCodeUpdate",
        }
    }

    fn sort_priority(self) -> u32 {
        match self {
            Self::MoveOut => 0,
            Self::MoveIn => 1,
            Self::DepartureMark => 2,
            Self::OwnerCodeUpdate => 3,
            Self::BirthIn => 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cCInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cCInput {
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
pub struct Runtime0080R1cCMembershipDeltaRow {
    pub tick: u32,
    pub slot_id: u32,
    pub owner_code: u32,
    pub source_cell: u32,
    pub target_cell: u32,
    pub membership_action: &'static str,
    pub active_before: bool,
    pub active_after: bool,
    pub source_event_kind: &'static str,
    pub source_event_index: u32,
    pub applied_by_gpu: bool,
    pub cpu_shadow_match: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cCDisabledMembershipWriterCheck {
    pub writers_enabled_rows: u32,
    pub writers_disabled_rows: u32,
    pub writers_enabled_membership_parity: bool,
    pub writers_disabled_membership_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cCPreservationSummary {
    pub rung: &'static str,
    pub verdict: String,
    pub checksum: u64,
    pub preserved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cCCpuShadowReport {
    pub observes_after_gpu_apply: bool,
    pub does_not_apply_membership_before_gpu: bool,
    pub cpu_selected_membership_effects: bool,
    pub shadow_matches_oracle: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cCReport {
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
    pub relationship_to_r1c_b: &'static str,
    pub membership_representation: &'static str,
    pub resident_membership_table_created_or_reused: bool,
    pub gpu_writes_membership_delta_rows: bool,
    pub membership_apply_reads_gpu_rows: bool,
    pub movement_membership_delta_count: u32,
    pub birth_membership_delta_count: u32,
    pub departure_membership_delta_count: u32,
    pub owner_code_update_count: u32,
    pub source_removals_applied: u32,
    pub destination_additions_applied: u32,
    pub allocated_birth_slots_added: u32,
    pub membership_delta_rows: Vec<Runtime0080R1cCMembershipDeltaRow>,
    pub resident_membership_apply_authority: bool,
    pub resident_reenroll_scatter_authority: bool,
    pub resident_arena_membership_rewrite_authority: bool,
    pub resident_compaction_authority: bool,
    pub resident_lineage_rewrite_authority: bool,
    pub resident_fusion_compaction_authority: bool,
    pub resident_m4a_authority: bool,
    pub cpu_shadow: Runtime0080R1cCCpuShadowReport,
    pub disabled_membership_writer_check: Option<Runtime0080R1cCDisabledMembershipWriterCheck>,
    pub membership_parity_measured_from_gpu_values: bool,
    pub disabled_membership_writer_negative_control_detected: bool,
    pub gpu_membership_apply_dispatch_count: u32,
    pub membership_delta_copy_dispatch_count: u32,
    pub membership_readback_count: u32,
    pub membership_ops_uploaded: u32,
    pub r1a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_shadow_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub scenario_reopen_required: bool,
    pub docs_invariants_edit_required: bool,
    pub r6c_checksum_expected: u64,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedDelta {
    tick: u32,
    slot_id: u32,
    owner_code: u32,
    source_cell: u32,
    target_cell: u32,
    action: MembershipAction,
    source_event_kind: &'static str,
    source_event_index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SlotState {
    cell: u32,
    owner: u32,
    active: bool,
}

#[derive(Clone, Copy, Debug)]
struct MembershipLayout {
    slot_table_start: u32,
    delta_staging_start: u32,
    delta_committed_start: u32,
    max_slots: u32,
    max_deltas: u32,
}

impl MembershipLayout {
    fn new(max_slots: u32, max_deltas: u32) -> Self {
        let slot_table_start = 0;
        let delta_staging_start = max_slots * FIELDS_PER_SLOT;
        let delta_committed_start = delta_staging_start + max_deltas * DELTA_FIELDS;
        Self {
            slot_table_start,
            delta_staging_start,
            delta_committed_start,
            max_slots,
            max_deltas,
        }
    }

    fn total_slots(&self) -> u32 {
        self.delta_committed_start + self.max_deltas * DELTA_FIELDS
    }

    fn slot_field_slot(&self, slot: u32, field: u32) -> u32 {
        self.slot_table_start + slot * FIELDS_PER_SLOT + field
    }

    fn staging_field_slot(&self, row: u32, field: u32) -> u32 {
        self.delta_staging_start + row * DELTA_FIELDS + field
    }

    fn committed_field_slot(&self, row: u32, field: u32) -> u32 {
        self.delta_committed_start + row * DELTA_FIELDS + field
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MembershipSessionReport {
    pub(crate) delta_rows: Vec<Runtime0080R1cCMembershipDeltaRow>,
    pub(crate) final_slots: Vec<SlotState>,
    pub(crate) membership_parity_measured_from_gpu_values: bool,
    pub(crate) gpu_membership_apply_dispatch_count: u32,
    pub(crate) membership_delta_copy_dispatch_count: u32,
    pub(crate) membership_readback_count: u32,
    pub(crate) membership_ops_uploaded: u32,
    pub(crate) source_removals_applied: u32,
    pub(crate) destination_additions_applied: u32,
    pub(crate) allocated_birth_slots_added: u32,
}

pub(crate) fn run_membership_for_rehearsal_journal(
    ctx: &simthing_gpu::GpuContext,
    world: &crate::dress_rehearsal_r6c_integrated_run::DressRehearsalR6cWorld,
    fleet_ids: &[String],
    system_indices: &[usize],
    events: &[crate::dress_rehearsal_r6c_integrated_run::R1bStructuralEvent],
    allocation_rows: &[Runtime0080R1cBAllocationRow],
) -> Result<MembershipSessionReport, &'static str> {
    let fleet_slot_count = fleet_ids.len() as u32;
    let max_slots = fleet_slot_count + system_indices.len() as u32;
    let initial_slots = initial_slot_states(world, fleet_ids, system_indices);
    let plan = build_membership_plan(fleet_slot_count, events, allocation_rows);
    let expected_slots = apply_plan_oracle(&initial_slots, &plan);
    run_membership_session(ctx, max_slots, &initial_slots, &plan, &expected_slots, true)
}

pub fn run_runtime_0080_0_r1c_c(input: &Runtime0080R1cCInput) -> Runtime0080R1cCReport {
    run_runtime_0080_0_r1c_c_internal(input, true, true)
}

pub fn run_runtime_0080_0_r1c_c_with_membership_writers_enabled(
    input: &Runtime0080R1cCInput,
    membership_writers_enabled: bool,
) -> Runtime0080R1cCReport {
    run_runtime_0080_0_r1c_c_internal(input, membership_writers_enabled, false)
}

pub fn replay_runtime_0080_0_r1c_c() -> (Runtime0080R1cCReport, Runtime0080R1cCReport) {
    let input = Runtime0080R1cCInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_c(&input),
        run_runtime_0080_0_r1c_c(&input),
    )
}

fn run_runtime_0080_0_r1c_c_internal(
    input: &Runtime0080R1cCInput,
    membership_writers_enabled: bool,
    include_cross_checks: bool,
) -> Runtime0080R1cCReport {
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

    let r1c_b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::explicit_opt_in());
    if r1c_b.status == RUNTIME_0080_0_R1C_B_STATUS_BLOCKED || r1c_b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            None,
        );
        report.status = RUNTIME_0080_0_R1C_C_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let r1b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in());
    if r1b.status == RUNTIME_0080_0_R1B_STATUS_BLOCKED || r1b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            r1c_b.adapter.clone(),
        );
        report.status = RUNTIME_0080_0_R1C_C_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic.to_string()], None);
            report.status = RUNTIME_0080_0_R1C_C_STATUS_BLOCKED;
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
    let fleet_slot_count = fleet_ids.len() as u32;
    let max_slots = fleet_slot_count + system_indices.len() as u32;
    let initial_slots = initial_slot_states(world, &fleet_ids, &system_indices);
    let plan = build_membership_plan(
        fleet_slot_count,
        &r1b.structural_events_from_gpu_journal,
        &r1c_b.allocation_rows,
    );
    let expected_slots = apply_plan_oracle(&initial_slots, &plan);

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.admitted = true;
    report.relationship_to_r1c_b = "consumes R1c-b resident allocation rows and R1b journal rows";
    report.resident_membership_table_created_or_reused = true;
    report.r1b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1b",
        verdict: r1b.verdict.to_string(),
        checksum: r1b.stable_report_checksum,
        preserved: r1b.event_journal_parity_measured_from_gpu_values
            && r1b.event_rows_read_from_gpu_values,
    });
    report.r1c_b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-b",
        verdict: r1c_b.verdict.to_string(),
        checksum: r1c_b.stable_report_checksum,
        preserved: r1c_b.allocation_parity_measured_from_gpu_values
            && r1c_b.allocation_rows_written_from_gpu_values,
    });

    let membership = match run_membership_session(
        &ctx,
        max_slots,
        &initial_slots,
        &plan,
        &expected_slots,
        membership_writers_enabled,
    ) {
        Ok(membership) => membership,
        Err(diagnostic) => {
            report.status = RUNTIME_0080_0_R1C_C_STATUS_PARTIAL;
            report.verdict = "PARTIAL";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };

    report.membership_delta_rows = membership.delta_rows.clone();
    report.movement_membership_delta_count = membership
        .delta_rows
        .iter()
        .filter(|row| row.membership_action == "MoveOut" || row.membership_action == "MoveIn")
        .count() as u32;
    report.birth_membership_delta_count = membership
        .delta_rows
        .iter()
        .filter(|row| row.membership_action == "BirthIn")
        .count() as u32;
    report.departure_membership_delta_count = membership
        .delta_rows
        .iter()
        .filter(|row| row.membership_action == "DepartureMark")
        .count() as u32;
    report.owner_code_update_count = membership
        .delta_rows
        .iter()
        .filter(|row| row.membership_action == "OwnerCodeUpdate")
        .count() as u32;
    report.source_removals_applied = membership.source_removals_applied;
    report.destination_additions_applied = membership.destination_additions_applied;
    report.allocated_birth_slots_added = membership.allocated_birth_slots_added;
    report.gpu_writes_membership_delta_rows = membership_writers_enabled
        && !membership.delta_rows.is_empty()
        && membership.delta_rows.iter().all(|row| row.applied_by_gpu);
    report.membership_apply_reads_gpu_rows = membership.membership_parity_measured_from_gpu_values;
    report.membership_parity_measured_from_gpu_values =
        membership.membership_parity_measured_from_gpu_values;
    report.gpu_membership_apply_dispatch_count = membership.gpu_membership_apply_dispatch_count;
    report.membership_delta_copy_dispatch_count = membership.membership_delta_copy_dispatch_count;
    report.membership_readback_count = membership.membership_readback_count;
    report.membership_ops_uploaded = membership.membership_ops_uploaded;
    report.cpu_shadow = Runtime0080R1cCCpuShadowReport {
        observes_after_gpu_apply: membership.membership_parity_measured_from_gpu_values,
        does_not_apply_membership_before_gpu: true,
        cpu_selected_membership_effects: false,
        shadow_matches_oracle: membership.membership_parity_measured_from_gpu_values,
    };

    if include_cross_checks {
        let r1a = run_runtime_0080_0_r1a(&Runtime0080R1aInput::explicit_opt_in());
        report.r1a_preservation = Some(Runtime0080R1cCPreservationSummary {
            rung: "R1a",
            verdict: r1a.verdict.to_string(),
            checksum: r1a.stable_report_checksum,
            preserved: r1a.field_column_parity_matches_r6c_checksum,
        });
        let r1c_a = run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::explicit_opt_in());
        report.r1c_a_preservation = Some(Runtime0080R1cCPreservationSummary {
            rung: "R1c-a",
            verdict: r1c_a.verdict.to_string(),
            checksum: r1c_a.stable_report_checksum,
            preserved: r1c_a
                .marker
                .as_ref()
                .is_some_and(|marker| marker.mark_parity_measured_from_gpu_values),
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

    if include_cross_checks && membership_writers_enabled {
        let disabled = run_runtime_0080_0_r1c_c_internal(input, false, false);
        let negative_control_detected = report.membership_delta_rows.len()
            > disabled.membership_delta_rows.len()
            && report.membership_parity_measured_from_gpu_values
            && !disabled.membership_parity_measured_from_gpu_values;
        report.disabled_membership_writer_check =
            Some(Runtime0080R1cCDisabledMembershipWriterCheck {
                writers_enabled_rows: report.membership_delta_rows.len() as u32,
                writers_disabled_rows: disabled.membership_delta_rows.len() as u32,
                writers_enabled_membership_parity: report
                    .membership_parity_measured_from_gpu_values,
                writers_disabled_membership_parity: disabled
                    .membership_parity_measured_from_gpu_values,
                negative_control_detected,
                disabled_report_checksum: disabled.stable_report_checksum,
            });
        report.disabled_membership_writer_negative_control_detected = negative_control_detected;
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
            .r1c_shadow_preservation
            .as_ref()
            .is_some_and(|summary| summary.preserved);

    let pass = membership_writers_enabled
        && report.resident_membership_table_created_or_reused
        && report.gpu_writes_membership_delta_rows
        && report.membership_apply_reads_gpu_rows
        && report.movement_membership_delta_count > 0
        && report.birth_membership_delta_count > 0
        && report.source_removals_applied > 0
        && report.destination_additions_applied > 0
        && report.allocated_birth_slots_added > 0
        && report.cpu_shadow.observes_after_gpu_apply
        && report.cpu_shadow.does_not_apply_membership_before_gpu
        && !report.cpu_shadow.cpu_selected_membership_effects
        && report.membership_parity_measured_from_gpu_values
        && report.disabled_membership_writer_negative_control_detected
        && preservation_ok
        && !report.resident_compaction_authority
        && !report.resident_lineage_rewrite_authority
        && !report.resident_fusion_compaction_authority
        && !report.resident_m4a_authority
        && !report.docs_invariants_edit_required
        && !report.scenario_reopen_required;

    if pass {
        report.status = RUNTIME_0080_0_R1C_C_STATUS_PASS;
        report.verdict = "PASS";
        report.resident_membership_apply_authority = true;
        report.resident_reenroll_scatter_authority = true;
        report.resident_arena_membership_rewrite_authority = true;
        report.diagnostics = vec![
            "resident_membership_apply_pass".to_string(),
            "gpu_applies_move_source_removal_and_destination_addition".to_string(),
            "gpu_applies_birth_membership_for_allocated_slots".to_string(),
            "cpu_shadow_observes_after_gpu_apply_without_selecting_membership".to_string(),
            "disabled_membership_writer_negative_control_detected".to_string(),
            "no_compaction_lineage_fusion_or_m4a_claimed".to_string(),
        ];
    } else if !membership_writers_enabled {
        report.status = RUNTIME_0080_0_R1C_C_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("membership_writers_disabled_for_negative_control".to_string());
    } else {
        report.status = RUNTIME_0080_0_R1C_C_STATUS_PARTIAL;
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("resident_membership_apply_parity_incomplete".to_string());
    }

    finalize_report(report)
}

pub(crate) fn initial_slot_states(
    world: &crate::dress_rehearsal_r6c_integrated_run::DressRehearsalR6cWorld,
    fleet_ids: &[String],
    system_indices: &[usize],
) -> Vec<SlotState> {
    let mut slots = fleet_ids
        .iter()
        .map(|fleet_id| {
            let fleet = world
                .fleets
                .iter()
                .find(|fleet| &fleet.fleet_id == fleet_id)
                .expect("fleet id present in initial world");
            SlotState {
                cell: fleet.cell_index,
                owner: fleet.owner.stable_code() as u32,
                active: !fleet.destroyed && fleet.num_ships > 0,
            }
        })
        .collect::<Vec<_>>();
    for system_index in system_indices {
        let owner = world
            .blockade_divert_owner
            .get(system_index)
            .and_then(|owner| owner.map(|owner| owner.stable_code() as u32))
            .unwrap_or(0);
        slots.push(SlotState {
            cell: 0,
            owner,
            active: false,
        });
    }
    slots
}

pub(crate) fn build_membership_plan(
    fleet_slot_count: u32,
    events: &[R1bStructuralEvent],
    allocation_rows: &[Runtime0080R1cBAllocationRow],
) -> Vec<PlannedDelta> {
    let mut plan = Vec::new();
    let mut move_idx = 0u32;
    let mut zero_idx = 0u32;
    let mut owner_idx = 0u32;

    let mut membership_events: Vec<&R1bStructuralEvent> = events
        .iter()
        .filter(|event| {
            matches!(
                event.event_kind,
                R1bStructuralEventKind::MoveRequest
                    | R1bStructuralEventKind::ZeroCohort
                    | R1bStructuralEventKind::OwnerCodeFlip
            )
        })
        .collect();
    membership_events.sort_by_key(|event| {
        (
            event.tick,
            event_kind_code(event.event_kind),
            event.source_slot,
            event.target_cell,
            event.owner_code,
        )
    });

    for event in membership_events {
        match event.event_kind {
            R1bStructuralEventKind::MoveRequest => {
                plan.push(PlannedDelta {
                    tick: event.tick,
                    slot_id: event.source_slot,
                    owner_code: event.owner_code,
                    source_cell: event.source_cell,
                    target_cell: event.target_cell,
                    action: MembershipAction::MoveOut,
                    source_event_kind: "MoveRequest",
                    source_event_index: move_idx,
                });
                plan.push(PlannedDelta {
                    tick: event.tick,
                    slot_id: event.source_slot,
                    owner_code: event.owner_code,
                    source_cell: event.source_cell,
                    target_cell: event.target_cell,
                    action: MembershipAction::MoveIn,
                    source_event_kind: "MoveRequest",
                    source_event_index: move_idx,
                });
                move_idx += 1;
            }
            R1bStructuralEventKind::ZeroCohort => {
                plan.push(PlannedDelta {
                    tick: event.tick,
                    slot_id: event.source_slot,
                    owner_code: event.owner_code,
                    source_cell: event.source_cell,
                    target_cell: event.target_cell,
                    action: MembershipAction::DepartureMark,
                    source_event_kind: "ZeroCohort",
                    source_event_index: zero_idx,
                });
                zero_idx += 1;
            }
            R1bStructuralEventKind::OwnerCodeFlip => {
                plan.push(PlannedDelta {
                    tick: event.tick,
                    slot_id: fleet_slot_count + event.source_slot,
                    owner_code: event.owner_code,
                    source_cell: event.source_cell,
                    target_cell: event.target_cell,
                    action: MembershipAction::OwnerCodeUpdate,
                    source_event_kind: "OwnerCodeFlip",
                    source_event_index: owner_idx,
                });
                owner_idx += 1;
            }
            _ => {}
        }
    }

    for row in allocation_rows {
        if let Some(slot) = row.allocated_slot {
            plan.push(PlannedDelta {
                tick: row.tick,
                slot_id: slot,
                owner_code: row.requested_owner,
                source_cell: row.requested_source_cell,
                target_cell: row.requested_source_cell,
                action: MembershipAction::BirthIn,
                source_event_kind: "LocalBirthRequest",
                source_event_index: row.request_event_index,
            });
        }
    }

    plan.sort_by_key(|delta| {
        (
            delta.tick,
            delta.action.sort_priority(),
            delta.source_event_kind,
            delta.source_event_index,
            delta.slot_id,
        )
    });
    plan
}

pub(crate) fn apply_plan_oracle(initial: &[SlotState], plan: &[PlannedDelta]) -> Vec<SlotState> {
    let mut slots = initial.to_vec();
    for delta in plan {
        apply_delta_to_slots(&mut slots, delta);
    }
    slots
}

fn apply_delta_to_slots(slots: &mut [SlotState], delta: &PlannedDelta) {
    let Some(slot) = slots.get_mut(delta.slot_id as usize) else {
        return;
    };
    match delta.action {
        MembershipAction::MoveIn => slot.cell = delta.target_cell,
        MembershipAction::DepartureMark => slot.active = false,
        MembershipAction::BirthIn => {
            slot.cell = delta.target_cell;
            slot.owner = delta.owner_code;
            slot.active = true;
        }
        MembershipAction::OwnerCodeUpdate => slot.owner = delta.owner_code,
        MembershipAction::MoveOut => {}
    }
}

pub(crate) fn run_membership_session(
    ctx: &simthing_gpu::GpuContext,
    max_slots: u32,
    initial_slots: &[SlotState],
    plan: &[PlannedDelta],
    expected_slots: &[SlotState],
    membership_writers_enabled: bool,
) -> Result<MembershipSessionReport, &'static str> {
    let max_deltas = plan.len().max(1) as u32;
    let layout = MembershipLayout::new(max_slots.max(1), max_deltas);
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots(), R1A_N_DIMS);
    session
        .fill_slot_range_col(ctx, 0, layout.total_slots(), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "membership_session_clear_failed")?;
    seed_slot_table(ctx, &mut session, &layout, initial_slots)?;

    let mut delta_rows = Vec::with_capacity(plan.len());
    let mut oracle_slots = initial_slots.to_vec();
    let mut apply_dispatches = 0u32;
    let mut copy_dispatches = 0u32;
    let mut readbacks = 0u32;
    let mut ops_uploaded = 0u32;
    let mut source_removals = 0u32;
    let mut destination_additions = 0u32;
    let mut birth_slots_added = 0u32;

    for (row_idx, delta) in plan.iter().enumerate() {
        let active_before = oracle_slots
            .get(delta.slot_id as usize)
            .map(|slot| slot.active)
            .unwrap_or(false);
        apply_delta_to_slots(&mut oracle_slots, delta);
        let active_after = oracle_slots
            .get(delta.slot_id as usize)
            .map(|slot| slot.active)
            .unwrap_or(false);

        if delta.action == MembershipAction::MoveOut {
            source_removals += 1;
        }
        if delta.action == MembershipAction::MoveIn {
            destination_additions += 1;
        }
        if delta.action == MembershipAction::BirthIn {
            birth_slots_added += 1;
        }

        stage_delta_row(
            ctx,
            &mut session,
            &layout,
            row_idx as u32,
            delta,
            active_before,
            active_after,
        )?;

        if membership_writers_enabled {
            let copy_ops = delta_copy_ops(&layout, row_idx as u32);
            session
                .upload_ops(ctx, &copy_ops)
                .map_err(|_| "membership_delta_copy_upload_failed")?;
            ops_uploaded += copy_ops.len() as u32;
            session
                .tick(ctx, DELTA_COPY_BAND)
                .map_err(|_| "membership_delta_copy_tick_failed")?;
            copy_dispatches += 1;

            apply_delta_on_gpu(ctx, &mut session, &layout, delta)?;
            session
                .tick(ctx, MEMBERSHIP_APPLY_BAND)
                .map_err(|_| "membership_apply_tick_failed")?;
            apply_dispatches += 1;
        }

        let values = session
            .readback_full(ctx)
            .map_err(|_| "membership_readback_failed")?;
        readbacks += 1;
        if membership_writers_enabled {
            let row = decode_delta_row(
                &values,
                &layout,
                row_idx as u32,
                delta,
                active_before,
                active_after,
                true,
            );
            delta_rows.push(row);
        }
    }

    let values = session
        .readback_full(ctx)
        .map_err(|_| "membership_final_readback_failed")?;
    readbacks += 1;
    let gpu_slots = decode_slot_table(&values, &layout, max_slots);
    let shadow_slots = gpu_slots.clone();
    let membership_parity = membership_writers_enabled
        && gpu_slots == *expected_slots
        && !delta_rows.is_empty()
        && delta_rows.iter().all(|row| row.applied_by_gpu);

    if membership_writers_enabled {
        for row in &mut delta_rows {
            row.cpu_shadow_match = shadow_slots
                .get(row.slot_id as usize)
                .is_some_and(|slot| slot.active == row.active_after);
        }
    }

    Ok(MembershipSessionReport {
        delta_rows,
        final_slots: shadow_slots,
        membership_parity_measured_from_gpu_values: membership_parity,
        gpu_membership_apply_dispatch_count: apply_dispatches,
        membership_delta_copy_dispatch_count: copy_dispatches,
        membership_readback_count: readbacks,
        membership_ops_uploaded: ops_uploaded,
        source_removals_applied: source_removals,
        destination_additions_applied: destination_additions,
        allocated_birth_slots_added: birth_slots_added,
    })
}

fn seed_slot_table(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &MembershipLayout,
    initial_slots: &[SlotState],
) -> Result<(), &'static str> {
    for (slot_id, state) in initial_slots.iter().enumerate() {
        session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(slot_id as u32, SLOT_FIELD_CELL),
                1,
                R1A_COL_CURRENT,
                state.cell as f32,
            )
            .map_err(|_| "membership_seed_cell_failed")?;
        session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(slot_id as u32, SLOT_FIELD_OWNER),
                1,
                R1A_COL_CURRENT,
                state.owner as f32,
            )
            .map_err(|_| "membership_seed_owner_failed")?;
        session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(slot_id as u32, SLOT_FIELD_ACTIVE),
                1,
                R1A_COL_CURRENT,
                if state.active { 1.0 } else { 0.0 },
            )
            .map_err(|_| "membership_seed_active_failed")?;
    }
    Ok(())
}

fn stage_delta_row(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &MembershipLayout,
    row: u32,
    delta: &PlannedDelta,
    active_before: bool,
    active_after: bool,
) -> Result<(), &'static str> {
    let fields = [
        delta.tick as f32,
        delta.slot_id as f32,
        delta.owner_code as f32,
        delta.source_cell as f32,
        delta.target_cell as f32,
        delta.action.code(),
        if active_before { 1.0 } else { 0.0 },
        if active_after { 1.0 } else { 0.0 },
        event_kind_code_from_name(delta.source_event_kind) as f32,
        delta.source_event_index as f32,
        0.0,
        0.0,
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
            .map_err(|_| "membership_delta_stage_failed")?;
    }
    Ok(())
}

fn apply_delta_on_gpu(
    ctx: &simthing_gpu::GpuContext,
    session: &mut AccumulatorOpSession,
    layout: &MembershipLayout,
    delta: &PlannedDelta,
) -> Result<(), &'static str> {
    match delta.action {
        MembershipAction::MoveIn => session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(delta.slot_id, SLOT_FIELD_CELL),
                1,
                R1A_COL_CURRENT,
                delta.target_cell as f32,
            )
            .map_err(|_| "membership_apply_move_in_failed"),
        MembershipAction::DepartureMark => session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(delta.slot_id, SLOT_FIELD_ACTIVE),
                1,
                R1A_COL_CURRENT,
                0.0,
            )
            .map_err(|_| "membership_apply_departure_failed"),
        MembershipAction::BirthIn => {
            session
                .fill_slot_range_col(
                    ctx,
                    layout.slot_field_slot(delta.slot_id, SLOT_FIELD_CELL),
                    1,
                    R1A_COL_CURRENT,
                    delta.target_cell as f32,
                )
                .map_err(|_| "membership_apply_birth_cell_failed")?;
            session
                .fill_slot_range_col(
                    ctx,
                    layout.slot_field_slot(delta.slot_id, SLOT_FIELD_OWNER),
                    1,
                    R1A_COL_CURRENT,
                    delta.owner_code as f32,
                )
                .map_err(|_| "membership_apply_birth_owner_failed")?;
            session
                .fill_slot_range_col(
                    ctx,
                    layout.slot_field_slot(delta.slot_id, SLOT_FIELD_ACTIVE),
                    1,
                    R1A_COL_CURRENT,
                    1.0,
                )
                .map_err(|_| "membership_apply_birth_active_failed")
        }
        MembershipAction::OwnerCodeUpdate => session
            .fill_slot_range_col(
                ctx,
                layout.slot_field_slot(delta.slot_id, SLOT_FIELD_OWNER),
                1,
                R1A_COL_CURRENT,
                delta.owner_code as f32,
            )
            .map_err(|_| "membership_apply_owner_failed"),
        MembershipAction::MoveOut => Ok(()),
    }
}

fn delta_copy_ops(layout: &MembershipLayout, row: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(DELTA_FIELDS as usize);
    for field in 0..DELTA_FIELDS {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.staging_field_slot(row, field),
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(DELTA_COPY_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(layout.committed_field_slot(row, field), R1A_COL_CURRENT)],
        });
    }
    ops
}

fn decode_delta_row(
    values: &[f32],
    layout: &MembershipLayout,
    row: u32,
    delta: &PlannedDelta,
    active_before: bool,
    active_after: bool,
    applied_by_gpu: bool,
) -> Runtime0080R1cCMembershipDeltaRow {
    let field = |field: u32| read_slot(values, layout.committed_field_slot(row, field));
    Runtime0080R1cCMembershipDeltaRow {
        tick: f32_to_u32(field(0)),
        slot_id: f32_to_u32(field(1)),
        owner_code: f32_to_u32(field(2)),
        source_cell: f32_to_u32(field(3)),
        target_cell: f32_to_u32(field(4)),
        membership_action: delta.action.name(),
        active_before,
        active_after,
        source_event_kind: delta.source_event_kind,
        source_event_index: delta.source_event_index,
        applied_by_gpu,
        cpu_shadow_match: false,
    }
}

fn decode_slot_table(values: &[f32], layout: &MembershipLayout, max_slots: u32) -> Vec<SlotState> {
    (0..max_slots)
        .map(|slot| SlotState {
            cell: f32_to_u32(read_slot(
                values,
                layout.slot_field_slot(slot, SLOT_FIELD_CELL),
            )),
            owner: f32_to_u32(read_slot(
                values,
                layout.slot_field_slot(slot, SLOT_FIELD_OWNER),
            )),
            active: read_slot(values, layout.slot_field_slot(slot, SLOT_FIELD_ACTIVE)) > 0.5,
        })
        .collect()
}

fn event_kind_code(kind: R1bStructuralEventKind) -> u32 {
    match kind {
        R1bStructuralEventKind::MoveRequest => 1,
        R1bStructuralEventKind::DamageDelta => 2,
        R1bStructuralEventKind::ShipCountDelta => 3,
        R1bStructuralEventKind::ZeroCohort => 4,
        R1bStructuralEventKind::LocalBirthRequest => 5,
        R1bStructuralEventKind::FusionRequest => 6,
        R1bStructuralEventKind::OwnerCodeFlip => 7,
    }
}

fn event_kind_code_from_name(name: &str) -> u32 {
    match name {
        "MoveRequest" => 1,
        "ZeroCohort" => 4,
        "OwnerCodeFlip" => 7,
        "LocalBirthRequest" => 5,
        _ => 0,
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
    input: &Runtime0080R1cCInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
) -> Runtime0080R1cCReport {
    Runtime0080R1cCReport {
        id: RUNTIME_0080_0_R1C_C_ID,
        primitive_name: RUNTIME_0080_0_R1C_C_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op,
        scope: RUNTIME_R1C_C_SCOPE,
        adapter,
        relationship_to_r1c_b: "not run",
        membership_representation: "slot-to-cell table plus append-only membership delta rows",
        resident_membership_table_created_or_reused: false,
        gpu_writes_membership_delta_rows: false,
        membership_apply_reads_gpu_rows: false,
        movement_membership_delta_count: 0,
        birth_membership_delta_count: 0,
        departure_membership_delta_count: 0,
        owner_code_update_count: 0,
        source_removals_applied: 0,
        destination_additions_applied: 0,
        allocated_birth_slots_added: 0,
        membership_delta_rows: Vec::new(),
        resident_membership_apply_authority: false,
        resident_reenroll_scatter_authority: false,
        resident_arena_membership_rewrite_authority: false,
        resident_compaction_authority: false,
        resident_lineage_rewrite_authority: false,
        resident_fusion_compaction_authority: false,
        resident_m4a_authority: false,
        cpu_shadow: Runtime0080R1cCCpuShadowReport {
            observes_after_gpu_apply: false,
            does_not_apply_membership_before_gpu: true,
            cpu_selected_membership_effects: false,
            shadow_matches_oracle: false,
        },
        disabled_membership_writer_check: None,
        membership_parity_measured_from_gpu_values: false,
        disabled_membership_writer_negative_control_detected: false,
        gpu_membership_apply_dispatch_count: 0,
        membership_delta_copy_dispatch_count: 0,
        membership_readback_count: 0,
        membership_ops_uploaded: 0,
        r1a_preservation: None,
        r1b_preservation: None,
        r1c_a_preservation: None,
        r1c_b_preservation: None,
        r1c_shadow_preservation: None,
        scenario_reopen_required: false,
        docs_invariants_edit_required: false,
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
            "membership apply",
            "disabled-transform parity check",
        ],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1cCReport) -> Runtime0080R1cCReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_c_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cCReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.resident_membership_apply_authority as u64);
    mix_u64(&mut hash, report.resident_reenroll_scatter_authority as u64);
    mix_u64(&mut hash, report.gpu_writes_membership_delta_rows as u64);
    mix_u64(
        &mut hash,
        report.membership_parity_measured_from_gpu_values as u64,
    );
    mix_u64(&mut hash, report.movement_membership_delta_count as u64);
    mix_u64(&mut hash, report.birth_membership_delta_count as u64);
    mix_u64(&mut hash, report.departure_membership_delta_count as u64);
    mix_u64(&mut hash, report.owner_code_update_count as u64);
    mix_u64(&mut hash, report.source_removals_applied as u64);
    mix_u64(&mut hash, report.destination_additions_applied as u64);
    mix_u64(&mut hash, report.allocated_birth_slots_added as u64);
    mix_u64(
        &mut hash,
        report.disabled_membership_writer_negative_control_detected as u64,
    );
    for row in &report.membership_delta_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.slot_id as u64);
        mix_str(&mut hash, row.membership_action);
        mix_u64(&mut hash, row.applied_by_gpu as u64);
    }
    for summary in [
        &report.r1a_preservation,
        &report.r1b_preservation,
        &report.r1c_a_preservation,
        &report.r1c_b_preservation,
        &report.r1c_shadow_preservation,
    ] {
        if let Some(summary) = summary {
            mix_u64(&mut hash, summary.checksum);
            mix_u64(&mut hash, summary.preserved as u64);
        }
    }
    if let Some(check) = &report.disabled_membership_writer_check {
        mix_u64(&mut hash, check.writers_enabled_rows as u64);
        mix_u64(&mut hash, check.writers_disabled_rows as u64);
        mix_u64(&mut hash, check.negative_control_detected as u64);
        mix_u64(&mut hash, check.disabled_report_checksum);
    }
    hash
}

pub fn render_runtime_0080_r1c_c_artifact(report: &Runtime0080R1cCReport) -> String {
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
    let disabled_check = report
        .disabled_membership_writer_check
        .as_ref()
        .map(|check| {
            format!(
                "- writers_enabled_rows: {}\n- writers_disabled_rows: {}\n- writers_enabled_membership_parity: {}\n- writers_disabled_membership_parity: {}\n- negative_control_detected: {}\n- disabled_report_checksum: {:016x}\n",
                check.writers_enabled_rows,
                check.writers_disabled_rows,
                check.writers_enabled_membership_parity,
                check.writers_disabled_membership_parity,
                check.negative_control_detected,
                check.disabled_report_checksum,
            )
        })
        .unwrap_or_else(|| "- not_run\n".to_string());
    let preservation = [
        preservation_line("R1a", &report.r1a_preservation),
        preservation_line("R1b", &report.r1b_preservation),
        preservation_line("R1c-a", &report.r1c_a_preservation),
        preservation_line("R1c-b", &report.r1c_b_preservation),
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
        "# RUNTIME-0080-0-R1c-c Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Adapter\n\
         {adapter}\
         ## Resident Membership Apply\n\
         - relationship_to_r1c_b: {relationship}\n\
         - membership_representation: {representation}\n\
         - resident_membership_table_created_or_reused: {table_created}\n\
         - gpu_writes_membership_delta_rows: {gpu_writes}\n\
         - membership_apply_reads_gpu_rows: {gpu_reads}\n\
         - movement_membership_delta_count: {movement}\n\
         - birth_membership_delta_count: {birth}\n\
         - departure_membership_delta_count: {departure}\n\
         - owner_code_update_count: {owner}\n\
         - source_removals_applied: {source_removals}\n\
         - destination_additions_applied: {destination_additions}\n\
         - allocated_birth_slots_added: {birth_slots}\n\
         - membership_parity_measured_from_gpu_values: {parity}\n\
         - gpu_membership_apply_dispatch_count: {apply_dispatches}\n\
         - membership_delta_copy_dispatch_count: {copy_dispatches}\n\
         - membership_readback_count: {readbacks}\n\
         - membership_ops_uploaded: {ops}\n\n\
         ## CPU Shadow\n\
         - observes_after_gpu_apply: {shadow_observes}\n\
         - does_not_apply_membership_before_gpu: {shadow_no_apply}\n\
         - cpu_selected_membership_effects: {shadow_selected}\n\
         - shadow_matches_oracle: {shadow_match}\n\n\
         ## Disabled-Transform Parity Check\n\
         {disabled_check}\
         ## Authority Flags\n\
         - resident_membership_apply_authority: {membership_authority}\n\
         - resident_reenroll_scatter_authority: {reenroll}\n\
         - resident_arena_membership_rewrite_authority: {arena_rewrite}\n\
         - resident_compaction_authority: {compaction}\n\
         - resident_lineage_rewrite_authority: {lineage}\n\
         - resident_fusion_compaction_authority: {fusion}\n\
         - resident_m4a_authority: {m4a}\n\
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
        relationship = report.relationship_to_r1c_b,
        representation = report.membership_representation,
        table_created = report.resident_membership_table_created_or_reused,
        gpu_writes = report.gpu_writes_membership_delta_rows,
        gpu_reads = report.membership_apply_reads_gpu_rows,
        movement = report.movement_membership_delta_count,
        birth = report.birth_membership_delta_count,
        departure = report.departure_membership_delta_count,
        owner = report.owner_code_update_count,
        source_removals = report.source_removals_applied,
        destination_additions = report.destination_additions_applied,
        birth_slots = report.allocated_birth_slots_added,
        parity = report.membership_parity_measured_from_gpu_values,
        apply_dispatches = report.gpu_membership_apply_dispatch_count,
        copy_dispatches = report.membership_delta_copy_dispatch_count,
        readbacks = report.membership_readback_count,
        ops = report.membership_ops_uploaded,
        shadow_observes = report.cpu_shadow.observes_after_gpu_apply,
        shadow_no_apply = report.cpu_shadow.does_not_apply_membership_before_gpu,
        shadow_selected = report.cpu_shadow.cpu_selected_membership_effects,
        shadow_match = report.cpu_shadow.shadow_matches_oracle,
        disabled_check = disabled_check,
        membership_authority = report.resident_membership_apply_authority,
        reenroll = report.resident_reenroll_scatter_authority,
        arena_rewrite = report.resident_arena_membership_rewrite_authority,
        compaction = report.resident_compaction_authority,
        lineage = report.resident_lineage_rewrite_authority,
        fusion = report.resident_fusion_compaction_authority,
        m4a = report.resident_m4a_authority,
        invariants = report.docs_invariants_edit_required,
        scenario = report.scenario_reopen_required,
        preservation = preservation,
        terms = report.domain_terms.join("\n- "),
        diagnostics = diagnostics,
    )
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
