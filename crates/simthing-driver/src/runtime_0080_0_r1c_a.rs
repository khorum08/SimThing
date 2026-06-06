//! RUNTIME-0080-0-R1c-a: resident free-list mark-only rung.
//!
//! This is the smaller rung named by R1c after resident scatter/compact hit the
//! free-list stop-line. It marks freed cohort slots in a resident GPU bitmap from
//! R1b's already GPU-read journal rows. It deliberately does not allocate into
//! free slots, scatter membership, compact cohorts, or claim structural decision
//! authority.

use std::collections::BTreeSet;

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession};

use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput,
};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    create_discrete_gpu_context, Runtime0080R1aAdapterReport, R1A_COL_CURRENT, R1A_N_DIMS,
};
use crate::runtime_0080_0_r1b::{
    run_runtime_0080_0_r1b, Runtime0080R1bFreeSlotMarkSource, Runtime0080R1bInput,
    RUNTIME_0080_0_R1B_STATUS_BLOCKED,
};
use crate::runtime_0080_0_r1c::{
    run_runtime_0080_0_r1c, Runtime0080R1cInput, RUNTIME_0080_0_R1C_STATUS_BLOCKED,
};

pub const RUNTIME_0080_0_R1C_A_ID: &str = "RUNTIME-0080-0-R1c-a";
pub const RUNTIME_0080_0_R1C_A_PRIMITIVE: &str = "RESIDENT-FREELIST-MARK-ONLY-0";
pub const RUNTIME_0080_0_R1C_A_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - resident free-list mark-only; no allocation or compaction";
pub const RUNTIME_0080_0_R1C_A_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_A_SCOPE: &str =
    "resident free-list mark-only from R1b journal rows; no scatter/compact";
pub const RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM: u64 = 0x2f4c_d7b8_2b07_ca7d;

const FREELIST_MARK_COPY_BAND: u32 = 0;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cAInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cAInput {
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
pub struct Runtime0080R1cAPredecessorReport {
    pub r1c_verdict: String,
    pub r1c_status: String,
    pub r1c_checksum: u64,
    pub r1b_verdict: String,
    pub r1b_status: String,
    pub r1b_event_journal_parity: bool,
    pub r1b_event_rows_read_from_gpu_values: bool,
    pub r1b_free_slot_mark_source_rows: u32,
    pub r1b_checksum: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cAMarkTraceRow {
    pub tick: u32,
    pub slot: u32,
    pub reason: &'static str,
    pub source_event_kind: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cAMarkerReport {
    pub resident_marker_session_created: bool,
    pub mark_sources_from_r1b_gpu_journal: bool,
    pub mark_source_rows: u32,
    pub unique_slots_marked_expected: u32,
    pub unique_slots_marked_gpu: u32,
    pub gpu_marked_slots: Vec<u32>,
    pub oracle_marked_slots: Vec<u32>,
    pub marker_ops_uploaded: u32,
    pub marker_dispatch_count: u32,
    pub marker_readback_count: u32,
    pub mark_parity_measured_from_gpu_values: bool,
    pub disabled_marker_gpu_marked_slots: u32,
    pub disabled_marker_parity: bool,
    pub disabled_marker_negative_control_detected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cAReport {
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
    pub predecessor: Option<Runtime0080R1cAPredecessorReport>,
    pub marker: Option<Runtime0080R1cAMarkerReport>,
    pub mark_trace: Vec<Runtime0080R1cAMarkTraceRow>,
    pub resident_free_list_mark_authority: bool,
    pub resident_free_list_allocation_authority: bool,
    pub resident_reenroll_scatter_authority: bool,
    pub resident_birth_removal_authority: bool,
    pub resident_fusion_compaction_authority: bool,
    pub structural_decisions_gpu_emitted: bool,
    pub requires_compaction_for_next_rung: bool,
    pub requires_allocation_for_birth_rung: bool,
    pub semantic_gpu_code_required: bool,
    pub cpu_planner_required: bool,
    pub docs_invariants_edit_required: bool,
    pub pinned_number_change_required: bool,
    pub scenario_reopen_required: bool,
    pub next_horizon: &'static str,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub r6c_checksum_matches: bool,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub stable_report_checksum: u64,
    pub artifact_markdown: String,
}

#[derive(Clone, Copy, Debug)]
struct MarkLayout {
    staging_start: u32,
    committed_start: u32,
    fleet_slots: u32,
}

impl MarkLayout {
    fn new(fleet_slots: u32) -> Self {
        Self {
            staging_start: 0,
            committed_start: fleet_slots,
            fleet_slots,
        }
    }

    fn total_slots(&self) -> u32 {
        self.fleet_slots * 2
    }
}

pub fn run_runtime_0080_0_r1c_a(input: &Runtime0080R1cAInput) -> Runtime0080R1cAReport {
    run_runtime_0080_0_r1c_a_internal(input, true, true)
}

pub fn run_runtime_0080_0_r1c_a_with_mark_writers_enabled(
    input: &Runtime0080R1cAInput,
    mark_writers_enabled: bool,
) -> Runtime0080R1cAReport {
    run_runtime_0080_0_r1c_a_internal(input, mark_writers_enabled, false)
}

pub fn replay_runtime_0080_0_r1c_a() -> (Runtime0080R1cAReport, Runtime0080R1cAReport) {
    let input = Runtime0080R1cAInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_a(&input),
        run_runtime_0080_0_r1c_a(&input),
    )
}

fn run_runtime_0080_0_r1c_a_internal(
    input: &Runtime0080R1cAInput,
    mark_writers_enabled: bool,
    include_negative_control: bool,
) -> Runtime0080R1cAReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required".to_string()],
            None,
            mark_writers_enabled,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden".to_string()],
            None,
            mark_writers_enabled,
        ));
    }

    let r1c = run_runtime_0080_0_r1c(&Runtime0080R1cInput::explicit_opt_in());
    if r1c.status == RUNTIME_0080_0_R1C_STATUS_BLOCKED || r1c.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1c_predecessor_blocked_or_no_discrete_gpu".to_string()],
            None,
            mark_writers_enabled,
        );
        report.status = RUNTIME_0080_0_R1C_A_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let r1b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::explicit_opt_in());
    if r1b.status == RUNTIME_0080_0_R1B_STATUS_BLOCKED || r1b.verdict == "BLOCKED" {
        let mut report = base_report(
            input,
            false,
            vec!["r1b_predecessor_blocked_or_no_discrete_gpu".to_string()],
            None,
            mark_writers_enabled,
        );
        report.status = RUNTIME_0080_0_R1C_A_STATUS_BLOCKED;
        report.verdict = "BLOCKED";
        return finalize_report(report);
    }

    let mut report = base_report(
        input,
        false,
        Vec::new(),
        r1b.adapter.clone(),
        mark_writers_enabled,
    );
    report.predecessor = Some(Runtime0080R1cAPredecessorReport {
        r1c_verdict: r1c.verdict.to_string(),
        r1c_status: r1c.status.to_string(),
        r1c_checksum: r1c.stable_report_checksum,
        r1b_verdict: r1b.verdict.to_string(),
        r1b_status: r1b.status.to_string(),
        r1b_event_journal_parity: r1b.event_journal_parity_measured_from_gpu_values,
        r1b_event_rows_read_from_gpu_values: r1b.event_rows_read_from_gpu_values,
        r1b_free_slot_mark_source_rows: r1b.free_slot_mark_sources_from_gpu_journal.len() as u32,
        r1b_checksum: r1b.stable_report_checksum,
    });

    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let world = oracle
        .initial_world
        .as_ref()
        .expect("R6C report carries initial world");
    let fleet_slots = world.fleets.len() as u32;
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.r6c_checksum_matches = report.r6c_checksum_observed == report.r6c_checksum_expected;

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            report.status = RUNTIME_0080_0_R1C_A_STATUS_BLOCKED;
            report.verdict = "BLOCKED";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };
    report.adapter = Some(adapter);
    set_debug_readback_allowed(true);

    let marker = match run_mark_session(
        &ctx,
        fleet_slots,
        &r1b.free_slot_mark_sources_from_gpu_journal,
        mark_writers_enabled,
    ) {
        Ok(marker) => marker,
        Err(diagnostic) => {
            report.status = "IMPLEMENTED / PARTIAL - resident mark session failed";
            report.verdict = "PARTIAL";
            report.diagnostics.push(diagnostic.to_string());
            return finalize_report(report);
        }
    };
    report.mark_trace = r1b
        .free_slot_mark_sources_from_gpu_journal
        .iter()
        .map(|row| Runtime0080R1cAMarkTraceRow {
            tick: row.tick,
            slot: row.slot,
            reason: row.reason,
            source_event_kind: row.source_event_kind,
        })
        .collect();
    report.marker = Some(marker);

    if include_negative_control && mark_writers_enabled {
        let disabled = run_runtime_0080_0_r1c_a_internal(input, false, false);
        if let (Some(marker), Some(disabled_marker)) = (&mut report.marker, disabled.marker) {
            marker.disabled_marker_gpu_marked_slots = disabled_marker.unique_slots_marked_gpu;
            marker.disabled_marker_parity = disabled_marker.mark_parity_measured_from_gpu_values;
            marker.disabled_marker_negative_control_detected = marker.unique_slots_marked_gpu
                > disabled_marker.unique_slots_marked_gpu
                && marker.mark_parity_measured_from_gpu_values
                && !disabled_marker.mark_parity_measured_from_gpu_values;
        }
    }

    let marker_pass = report.marker.as_ref().is_some_and(|marker| {
        marker.resident_marker_session_created
            && marker.mark_sources_from_r1b_gpu_journal
            && marker.mark_source_rows > 0
            && marker.mark_parity_measured_from_gpu_values
            && marker.disabled_marker_negative_control_detected
    });
    let predecessor_pass = report.predecessor.as_ref().is_some_and(|predecessor| {
        predecessor.r1c_verdict == "PARTIAL"
            && predecessor.r1b_event_journal_parity
            && predecessor.r1b_event_rows_read_from_gpu_values
            && predecessor.r1b_free_slot_mark_source_rows > 0
    });
    if marker_pass && predecessor_pass && report.r6c_checksum_matches && mark_writers_enabled {
        report.status = RUNTIME_0080_0_R1C_A_STATUS_PASS;
        report.verdict = "PASS";
        report.admitted = true;
        report.resident_free_list_mark_authority = true;
        report.diagnostics = vec![
            "resident_free_list_mark_only_pass".to_string(),
            "no_allocation_no_scatter_no_compaction_claimed".to_string(),
            "mark_negative_control_detected".to_string(),
        ];
    } else if !mark_writers_enabled {
        report.status = "IMPLEMENTED / PARTIAL - mark writers disabled for negative control";
        report.verdict = "PARTIAL";
        report
            .diagnostics
            .push("mark_writers_disabled_for_negative_control".to_string());
    } else {
        report.status = "IMPLEMENTED / PARTIAL - resident mark parity incomplete";
        report.verdict = "PARTIAL";
    }

    finalize_report(report)
}

fn base_report(
    input: &Runtime0080R1cAInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
    mark_writers_enabled: bool,
) -> Runtime0080R1cAReport {
    Runtime0080R1cAReport {
        id: RUNTIME_0080_0_R1C_A_ID,
        primitive_name: RUNTIME_0080_0_R1C_A_PRIMITIVE,
        status: "NOT RUN",
        verdict: "NOT RUN",
        admitted: false,
        diagnostics,
        explicit_opt_in: input.explicit_opt_in,
        default_off: !input.enabled_by_default,
        disabled_no_op: disabled_no_op || !mark_writers_enabled,
        scope: RUNTIME_R1C_A_SCOPE,
        adapter,
        predecessor: None,
        marker: None,
        mark_trace: Vec::new(),
        resident_free_list_mark_authority: false,
        resident_free_list_allocation_authority: false,
        resident_reenroll_scatter_authority: false,
        resident_birth_removal_authority: false,
        resident_fusion_compaction_authority: false,
        structural_decisions_gpu_emitted: false,
        requires_compaction_for_next_rung: true,
        requires_allocation_for_birth_rung: true,
        semantic_gpu_code_required: false,
        cpu_planner_required: false,
        docs_invariants_edit_required: false,
        pinned_number_change_required: false,
        scenario_reopen_required: false,
        next_horizon: "R1c-b resident allocation into marked free slots / no compaction",
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        r6c_checksum_matches: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "resident event journal",
            "free-list mark",
            "slot bitmap",
            "GPU-side structural event rows",
            "disabled-marker negative control",
        ],
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn run_mark_session(
    ctx: &simthing_gpu::GpuContext,
    fleet_slots: u32,
    mark_sources: &[Runtime0080R1bFreeSlotMarkSource],
    mark_writers_enabled: bool,
) -> Result<Runtime0080R1cAMarkerReport, &'static str> {
    let layout = MarkLayout::new(fleet_slots);
    let mut session = AccumulatorOpSession::new(ctx, layout.total_slots(), R1A_N_DIMS);
    session
        .fill_slot_range_col(ctx, 0, layout.total_slots(), R1A_COL_CURRENT, 0.0)
        .map_err(|_| "mark_bitmap_clear_failed")?;
    if mark_writers_enabled {
        for source in mark_sources {
            if source.slot >= layout.fleet_slots {
                return Err("mark_source_slot_out_of_bounds");
            }
            session
                .fill_slot_range_col(
                    ctx,
                    layout.staging_start + source.slot,
                    1,
                    R1A_COL_CURRENT,
                    1.0,
                )
                .map_err(|_| "mark_bitmap_stage_failed")?;
        }
    }
    let ops = build_mark_copy_ops(&layout);
    session
        .upload_ops(ctx, &ops)
        .map_err(|_| "mark_bitmap_upload_ops_failed")?;
    session
        .tick(ctx, FREELIST_MARK_COPY_BAND)
        .map_err(|_| "mark_bitmap_copy_tick_failed")?;
    let values = session
        .readback_full(ctx)
        .map_err(|_| "mark_bitmap_readback_failed")?;
    let gpu_marked_slots = marked_slots_from_values(&values, &layout);
    let oracle_marked_slots = expected_marked_slots(mark_sources);
    let parity = gpu_marked_slots == oracle_marked_slots;
    Ok(Runtime0080R1cAMarkerReport {
        resident_marker_session_created: true,
        mark_sources_from_r1b_gpu_journal: true,
        mark_source_rows: mark_sources.len() as u32,
        unique_slots_marked_expected: oracle_marked_slots.len() as u32,
        unique_slots_marked_gpu: gpu_marked_slots.len() as u32,
        gpu_marked_slots,
        oracle_marked_slots,
        marker_ops_uploaded: ops.len() as u32,
        marker_dispatch_count: 1,
        marker_readback_count: 1,
        mark_parity_measured_from_gpu_values: parity,
        disabled_marker_gpu_marked_slots: 0,
        disabled_marker_parity: false,
        disabled_marker_negative_control_detected: false,
    })
}

fn build_mark_copy_ops(layout: &MarkLayout) -> Vec<AccumulatorOp> {
    (0..layout.fleet_slots)
        .map(|slot| AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: layout.staging_start + slot,
                col: R1A_COL_CURRENT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(FREELIST_MARK_COPY_BAND),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(layout.committed_start + slot, R1A_COL_CURRENT)],
        })
        .collect()
}

fn expected_marked_slots(mark_sources: &[Runtime0080R1bFreeSlotMarkSource]) -> Vec<u32> {
    mark_sources
        .iter()
        .map(|source| source.slot)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn marked_slots_from_values(values: &[f32], layout: &MarkLayout) -> Vec<u32> {
    (0..layout.fleet_slots)
        .filter(|slot| {
            let idx = ((layout.committed_start + *slot) * R1A_N_DIMS + R1A_COL_CURRENT) as usize;
            values.get(idx).copied().unwrap_or(0.0) > 0.5
        })
        .collect()
}

fn finalize_report(mut report: Runtime0080R1cAReport) -> Runtime0080R1cAReport {
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1c_a_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1cAReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.resident_free_list_mark_authority as u64);
    mix_u64(
        &mut hash,
        report.resident_free_list_allocation_authority as u64,
    );
    mix_u64(&mut hash, report.resident_reenroll_scatter_authority as u64);
    mix_u64(
        &mut hash,
        report.resident_fusion_compaction_authority as u64,
    );
    if let Some(predecessor) = &report.predecessor {
        mix_u64(&mut hash, predecessor.r1c_checksum);
        mix_u64(&mut hash, predecessor.r1b_checksum);
        mix_u64(&mut hash, predecessor.r1b_free_slot_mark_source_rows as u64);
    }
    if let Some(marker) = &report.marker {
        mix_u64(&mut hash, marker.mark_source_rows as u64);
        mix_u64(&mut hash, marker.unique_slots_marked_expected as u64);
        mix_u64(&mut hash, marker.unique_slots_marked_gpu as u64);
        mix_u64(&mut hash, marker.marker_ops_uploaded as u64);
        mix_u64(&mut hash, marker.marker_dispatch_count as u64);
        mix_u64(&mut hash, marker.marker_readback_count as u64);
        mix_u64(
            &mut hash,
            marker.mark_parity_measured_from_gpu_values as u64,
        );
        mix_u64(
            &mut hash,
            marker.disabled_marker_negative_control_detected as u64,
        );
        for slot in &marker.gpu_marked_slots {
            mix_u64(&mut hash, *slot as u64);
        }
    }
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(&mut hash, report.r6c_checksum_matches as u64);
    hash
}

pub fn render_runtime_0080_r1c_a_artifact(report: &Runtime0080R1cAReport) -> String {
    let predecessor = report
        .predecessor
        .as_ref()
        .map(|predecessor| {
            format!(
                "- r1c_verdict: {}\n- r1b_event_journal_parity: {}\n- r1b_event_rows_read_from_gpu_values: {}\n- r1b_free_slot_mark_source_rows: {}\n",
                predecessor.r1c_verdict,
                predecessor.r1b_event_journal_parity,
                predecessor.r1b_event_rows_read_from_gpu_values,
                predecessor.r1b_free_slot_mark_source_rows
            )
        })
        .unwrap_or_else(|| "- predecessor: not run\n".to_string());
    let marker = report
        .marker
        .as_ref()
        .map(|marker| {
            format!(
                "- resident_marker_session_created: {}\n- mark_sources_from_r1b_gpu_journal: {}\n- mark_source_rows: {}\n- unique_slots_marked_expected: {}\n- unique_slots_marked_gpu: {}\n- marker_ops_uploaded: {}\n- marker_dispatch_count: {}\n- marker_readback_count: {}\n- mark_parity_measured_from_gpu_values: {}\n- disabled_marker_negative_control_detected: {}\n",
                marker.resident_marker_session_created,
                marker.mark_sources_from_r1b_gpu_journal,
                marker.mark_source_rows,
                marker.unique_slots_marked_expected,
                marker.unique_slots_marked_gpu,
                marker.marker_ops_uploaded,
                marker.marker_dispatch_count,
                marker.marker_readback_count,
                marker.mark_parity_measured_from_gpu_values,
                marker.disabled_marker_negative_control_detected
            )
        })
        .unwrap_or_else(|| "- marker: not run\n".to_string());

    format!(
        "# RUNTIME-0080-0-R1c-a Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         ## Predecessors\n\
         {predecessor}\n\
         ## Resident Mark-Only Evidence\n\
         {marker}\n\
         ## Authority Flags\n\
         - resident_free_list_mark_authority: {mark_authority}\n\
         - resident_free_list_allocation_authority: {allocation}\n\
         - resident_reenroll_scatter_authority: {scatter}\n\
         - resident_birth_removal_authority: {birth_removal}\n\
         - resident_fusion_compaction_authority: {fusion}\n\
         - structural_decisions_gpu_emitted: {decisions}\n\n\
         ## Remaining Gates\n\
         - requires_compaction_for_next_rung: {compaction_required}\n\
         - requires_allocation_for_birth_rung: {allocation_required}\n\
         - semantic_gpu_code_required: {semantic_gpu}\n\
         - cpu_planner_required: {cpu_planner}\n\
         - docs_invariants_edit_required: {invariant}\n\
         - pinned_number_change_required: {pinned}\n\
         - scenario_reopen_required: {scenario}\n\
         - next_horizon: {next_horizon}\n\n\
         ## Checksum\n\
         - expected: `{expected:016x}`\n\
         - observed: `{observed:016x}`\n\
         - matches: {matches}\n\n\
         ## Domain Terms\n\
         - {terms}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        predecessor = predecessor,
        marker = marker,
        mark_authority = report.resident_free_list_mark_authority,
        allocation = report.resident_free_list_allocation_authority,
        scatter = report.resident_reenroll_scatter_authority,
        birth_removal = report.resident_birth_removal_authority,
        fusion = report.resident_fusion_compaction_authority,
        decisions = report.structural_decisions_gpu_emitted,
        compaction_required = report.requires_compaction_for_next_rung,
        allocation_required = report.requires_allocation_for_birth_rung,
        semantic_gpu = report.semantic_gpu_code_required,
        cpu_planner = report.cpu_planner_required,
        invariant = report.docs_invariants_edit_required,
        pinned = report.pinned_number_change_required,
        scenario = report.scenario_reopen_required,
        next_horizon = report.next_horizon,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        matches = report.r6c_checksum_matches,
        terms = report.domain_terms.join("\n- "),
    )
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
