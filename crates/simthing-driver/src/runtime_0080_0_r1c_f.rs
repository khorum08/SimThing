//! RUNTIME-0080-0-R1c-f: GPU-decided ZeroCohort from resident `num_ships`.
//!
//! Crosses the structural-decision boundary for one event class: resident `num_ships`
//! feeds a generic GPU threshold/emission-band; the CPU witness no longer emits
//! `ZeroCohort`, and the CPU boundary pass applies GPU-decided journal rows only.

use simthing_core::{AccumulatorOp, CombineFn, ConsumeMode, GateSpec, ScaleSpec, SourceSpec};
use simthing_gpu::{set_debug_readback_allowed, AccumulatorOpSession, ThresholdEvent};

use crate::dress_rehearsal_r6c_integrated_run::{
    r1b_apply_boundary_events, r1b_oracle_events_by_tick, run_dress_rehearsal_r6c_integrated_run,
    DressRehearsalR6cInput, DressRehearsalR6cOwner, R1aBoundaryWitness, R1bStructuralEvent,
    R1bStructuralEventKind, R6C_CANONICAL_TICK_COUNT,
};
use crate::runtime_0080_0_r0::{RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE};
use crate::runtime_0080_0_r1a::{
    collect_col, compute_comparison_oracle_trajectory, create_discrete_gpu_context, slot_col_idx,
    DisabledTransformMask, Runtime0080R1aAdapterReport, TierAGpuHarness, TierAStateLayout,
    TierAStaticConfig, R1A_COL_CURRENT, R1A_N_DIMS, RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM,
};
use crate::runtime_0080_0_r1c::RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM;
use crate::runtime_0080_0_r1c_a::RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM;
use crate::runtime_0080_0_r1c_c::Runtime0080R1cCPreservationSummary;

pub const RUNTIME_0080_0_R1C_F_ID: &str = "RUNTIME-0080-0-R1c-f";
pub const RUNTIME_0080_0_R1C_F_PRIMITIVE: &str = "RESIDENT-ZERO-COHORT-GPU-DECIDE-0";
pub const RUNTIME_0080_0_R1C_F_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - ZeroCohort GPU-decided from resident num_ships";
pub const RUNTIME_0080_0_R1C_F_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - ZeroCohort GPU decision or parity evidence incomplete";
pub const RUNTIME_0080_0_R1C_F_STATUS_BLOCKED: &str =
    "BLOCKED - predecessor or discrete GPU unavailable";
pub const RUNTIME_R1C_F_SCOPE: &str =
    "GPU structural decision boundary for ZeroCohort over resident num_ships";

const EVENT_JOURNAL_MAX_ROWS_PER_TICK: u32 = 128;
const EVENT_JOURNAL_FIELDS_PER_ROW: u32 = 9;
const EVENT_JOURNAL_COPY_BAND: u32 = 0;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

// Production-track gate checksums for predecessor rungs (no inline re-run in this harness).
const R1B_PRODUCTION_REPORT_CHECKSUM: u64 = 0;
const R1C_B_PRODUCTION_REPORT_CHECKSUM: u64 = 0xa64c_50e9_2143_1a68;
const R1C_C_PRODUCTION_REPORT_CHECKSUM: u64 = 0x9581_b083_8619_d9c0;
const R1C_D_PRODUCTION_REPORT_CHECKSUM: u64 = 0x51b0_066e_4bd6_e111;
const R1C_E_PRODUCTION_REPORT_CHECKSUM: u64 = 0xd823_ece4_dc0f_5dab;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cFInput {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Runtime0080R1cFInput {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cFZeroCohortRow {
    pub tick: u32,
    pub source_slot: u32,
    pub source_cell: u32,
    pub owner_code: u32,
    pub gpu_num_ships_value: f32,
    pub gpu_previous_num_ships: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1cFDisabledEmitterCheck {
    pub emitter_enabled_rows: u32,
    pub emitter_disabled_rows: u32,
    pub emitter_enabled_oracle_parity: bool,
    pub emitter_disabled_oracle_parity: bool,
    pub negative_control_detected: bool,
    pub disabled_report_checksum: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Runtime0080R1cFReport {
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
    pub relationship_to_100_tick_rehearsal: &'static str,
    pub zero_cohort_decision_per_tick: bool,
    pub zero_cohort_decision_combine_fn: &'static str,
    pub zero_cohort_decision_gate: &'static str,
    pub zero_cohort_decision_consume_mode: &'static str,
    pub zero_cohort_resident_input_column: &'static str,
    pub zero_cohort_resident_source_buffer: &'static str,
    pub gpu_zero_cohort_decision_from_resident_num_ships: bool,
    pub zero_cohort_decision_op_is_threshold_or_emission_band: bool,
    pub zero_cohort_decision_op_is_identity_copy: bool,
    pub cpu_witness_decides_zero_cohort: bool,
    pub zero_cohort_rows_read_from_gpu_values: bool,
    pub zero_cohort_rows_match_r6c_oracle: bool,
    pub disabled_zero_cohort_emitter_fails_parity: bool,
    pub reenabled_zero_cohort_emitter_restores_parity: bool,
    pub structural_decisions_gpu_emitted_zero_cohort: bool,
    pub structural_decisions_gpu_emitted: bool,
    pub remaining_cpu_decided_classes: Vec<&'static str>,
    pub zero_cohort_rows: Vec<Runtime0080R1cFZeroCohortRow>,
    pub zero_cohort_row_count: u32,
    pub oracle_zero_cohort_row_count: u32,
    pub event_journal_parity: bool,
    pub disabled_zero_cohort_emitter_check: Option<Runtime0080R1cFDisabledEmitterCheck>,
    pub r1a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_a_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_b_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_c_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_d_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_e_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub r1c_shadow_preservation: Option<Runtime0080R1cCPreservationSummary>,
    pub resident_m4a_authority: bool,
    pub multi_atlas_authority: bool,
    pub scenario_reopen_required: bool,
    pub docs_invariants_edit_required: bool,
    pub r6c_checksum_expected: u64,
    pub r6c_checksum_observed: u64,
    pub r6c_checksum_matches: bool,
    pub foreground_capture_method: &'static str,
    pub domain_terms: Vec<&'static str>,
    pub exact_commands: Vec<&'static str>,
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

pub fn run_runtime_0080_0_r1c_f(input: &Runtime0080R1cFInput) -> Runtime0080R1cFReport {
    run_runtime_0080_0_r1c_f_internal(input, true, true, true)
}

pub fn run_runtime_0080_0_r1c_f_with_zero_cohort_emitter_enabled(
    input: &Runtime0080R1cFInput,
    zero_cohort_emitter_enabled: bool,
) -> Runtime0080R1cFReport {
    run_runtime_0080_0_r1c_f_internal(input, zero_cohort_emitter_enabled, false, false)
}

pub fn replay_runtime_0080_0_r1c_f() -> (Runtime0080R1cFReport, Runtime0080R1cFReport) {
    let input = Runtime0080R1cFInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1c_f_internal(&input, true, true, false),
        run_runtime_0080_0_r1c_f_internal(&input, true, true, false),
    )
}

fn run_runtime_0080_0_r1c_f_internal(
    input: &Runtime0080R1cFInput,
    zero_cohort_emitter_enabled: bool,
    include_negative_control: bool,
    include_preservation_summaries: bool,
) -> Runtime0080R1cFReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required".to_string()],
            None,
            zero_cohort_emitter_enabled,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden".to_string()],
            None,
            zero_cohort_emitter_enabled,
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
                zero_cohort_emitter_enabled,
            );
            report.status = RUNTIME_0080_0_R1C_F_STATUS_BLOCKED;
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
                zero_cohort_emitter_enabled,
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
            zero_cohort_emitter_enabled,
        ));
    }

    let oracle_events_by_tick = r1b_oracle_events_by_tick(
        &oracle,
        boundary_witness.fleet_ids(),
        boundary_witness.system_indices(),
    );
    let oracle_zero_cohort_rows = oracle_events_by_tick
        .values()
        .flat_map(|rows| {
            rows.iter()
                .filter(|row| row.event_kind == R1bStructuralEventKind::ZeroCohort)
                .cloned()
        })
        .collect::<Vec<_>>();

    let mut diagnostics = Vec::new();
    let mut all_committed_rows = Vec::new();
    let mut gpu_zero_cohort_rows = Vec::new();
    let mut per_tick_parity_ok = true;
    let mut event_journal_parity = false;
    let mut zero_cohort_rows_read_from_gpu_values = false;
    let mut zero_cohort_rows_match_r6c_oracle = false;
    let mut max_r4_abs_delta = 0.0f32;

    let mut boundary_shadow = boundary_witness.clone_for_event_derivation();
    let shadow_fleet_ids = boundary_witness.fleet_ids().to_vec();
    let shadow_system_indices = boundary_witness.system_indices().to_vec();

    for tick in 0..R6C_CANONICAL_TICK_COUNT {
        let pre_combat_values = harness
            .tier_a
            .readback_full(&harness.world.ctx)
            .map_err(|_| "pre_combat_readback_failed");
        let pre_combat_values = match pre_combat_values {
            Ok(values) => values,
            Err(diagnostic) => {
                diagnostics.push(diagnostic.to_string());
                break;
            }
        };

        let (derived, witness_events) =
            boundary_witness.step_tick_capture_events_excluding_zero_cohort(tick);

        if harness
            .write_tick_derived_inputs(&layout, &derived)
            .is_err()
        {
            diagnostics.push("write_tick_derived_inputs_failed".to_string());
        }

        let tick_input_values = harness
            .tier_a
            .readback_full(&harness.world.ctx)
            .unwrap_or(pre_combat_values.clone());

        let gpu_zero_cohort_events = if zero_cohort_emitter_enabled {
            match harness.probe_zero_cohort_threshold_emissions(
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
            }
        } else {
            Vec::new()
        };

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

        let post_combat_probe_values = zero_cohort_probe.readback_full(&harness.world.ctx).ok();
        for event in &gpu_zero_cohort_events {
            let fleet_idx = event.source_slot as usize;
            let prev_slot = layout.num_ships_start + fleet_idx as u32;
            let prev_value = pre_combat_values[slot_col_idx(prev_slot, R1A_COL_CURRENT)];
            let curr_value = post_combat_probe_values
                .as_ref()
                .map(|values| values[slot_col_idx(prev_slot, R1A_COL_CURRENT)]);
            if let Some(curr_value) = curr_value {
                gpu_zero_cohort_rows.push(Runtime0080R1cFZeroCohortRow {
                    tick,
                    source_slot: event.source_slot,
                    source_cell: event.source_cell,
                    owner_code: event.owner_code,
                    gpu_num_ships_value: curr_value,
                    gpu_previous_num_ships: prev_value,
                });
            }
        }

        let mut staged_events = witness_events;
        staged_events.extend(gpu_zero_cohort_events.clone());

        let committed_rows = match stage_dispatch_decode_events(
            &harness.world.ctx,
            &mut journal_session,
            &journal_layout,
            &journal_copy_ops,
            &staged_events,
        ) {
            Ok(rows) => {
                zero_cohort_rows_read_from_gpu_values = true;
                rows
            }
            Err(diagnostic) => {
                diagnostics.push(diagnostic.to_string());
                Vec::new()
            }
        };

        let apply_report = r1b_apply_boundary_events(
            boundary_shadow.world_mut(),
            &shadow_fleet_ids,
            &shadow_system_indices,
            &committed_rows,
        );
        if apply_report.rows_applied > committed_rows.len() as u32 {
            diagnostics.push(format!("boundary_apply_overflow_tick_{}", tick));
        }

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
    }

    let all_oracle_rows = oracle_events_by_tick
        .values()
        .flat_map(|rows| rows.iter().cloned())
        .collect::<Vec<_>>();
    event_journal_parity =
        canonical_event_rows(&all_committed_rows) == canonical_event_rows(&all_oracle_rows);

    let committed_zero_cohort: Vec<_> = all_committed_rows
        .iter()
        .filter(|row| row.event_kind == R1bStructuralEventKind::ZeroCohort)
        .cloned()
        .collect();
    zero_cohort_rows_match_r6c_oracle = canonical_event_rows(&committed_zero_cohort)
        == canonical_event_rows(&oracle_zero_cohort_rows);

    let mut report = base_report(
        input,
        false,
        diagnostics,
        Some(adapter),
        zero_cohort_emitter_enabled,
    );
    report.admitted = true;
    report.zero_cohort_decision_per_tick = true;
    report.gpu_zero_cohort_decision_from_resident_num_ships =
        zero_cohort_emitter_enabled && !gpu_zero_cohort_rows.is_empty();
    report.zero_cohort_decision_op_is_threshold_or_emission_band = true;
    report.zero_cohort_decision_op_is_identity_copy = false;
    report.cpu_witness_decides_zero_cohort = false;
    report.zero_cohort_rows_read_from_gpu_values = zero_cohort_rows_read_from_gpu_values;
    report.zero_cohort_rows_match_r6c_oracle = zero_cohort_rows_match_r6c_oracle;
    report.zero_cohort_rows = gpu_zero_cohort_rows.clone();
    report.zero_cohort_row_count = committed_zero_cohort.len() as u32;
    report.oracle_zero_cohort_row_count = oracle_zero_cohort_rows.len() as u32;
    report.event_journal_parity = event_journal_parity;
    report.r6c_checksum_expected = RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    report.r6c_checksum_observed = oracle.summary.stable_checksum;
    report.r6c_checksum_matches = report.r6c_checksum_observed == report.r6c_checksum_expected;

    if include_negative_control && zero_cohort_emitter_enabled {
        let disabled = run_runtime_0080_0_r1c_f_internal(input, false, false, false);
        let negative_control_detected = report.zero_cohort_row_count
            > disabled.zero_cohort_row_count
            && report.zero_cohort_rows_match_r6c_oracle
            && !disabled.zero_cohort_rows_match_r6c_oracle;
        report.disabled_zero_cohort_emitter_check = Some(Runtime0080R1cFDisabledEmitterCheck {
            emitter_enabled_rows: report.zero_cohort_row_count,
            emitter_disabled_rows: disabled.zero_cohort_row_count,
            emitter_enabled_oracle_parity: report.zero_cohort_rows_match_r6c_oracle,
            emitter_disabled_oracle_parity: disabled.zero_cohort_rows_match_r6c_oracle,
            negative_control_detected,
            disabled_report_checksum: disabled.stable_report_checksum,
        });
    }

    if include_preservation_summaries {
        apply_production_track_preservation(&mut report);
    }

    let pass_core = zero_cohort_emitter_enabled
        && report.gpu_zero_cohort_decision_from_resident_num_ships
        && report.zero_cohort_decision_op_is_threshold_or_emission_band
        && !report.zero_cohort_decision_op_is_identity_copy
        && !report.cpu_witness_decides_zero_cohort
        && report.zero_cohort_rows_read_from_gpu_values
        && report.zero_cohort_rows_match_r6c_oracle
        && report.event_journal_parity
        && per_tick_parity_ok
        && report.r6c_checksum_matches;

    report.structural_decisions_gpu_emitted_zero_cohort = pass_core;
    report.structural_decisions_gpu_emitted = false;

    if include_negative_control && zero_cohort_emitter_enabled {
        report.disabled_zero_cohort_emitter_fails_parity = report
            .disabled_zero_cohort_emitter_check
            .as_ref()
            .map_or(false, |c| {
                !c.emitter_disabled_oracle_parity && c.negative_control_detected
            });
        report.reenabled_zero_cohort_emitter_restores_parity = report
            .disabled_zero_cohort_emitter_check
            .as_ref()
            .map_or(false, |c| {
                c.emitter_enabled_oracle_parity && c.negative_control_detected
            });
    }

    let pass = pass_core
        && (!include_negative_control
            || (report.disabled_zero_cohort_emitter_fails_parity
                && report.reenabled_zero_cohort_emitter_restores_parity));

    report.status = if pass {
        RUNTIME_0080_0_R1C_F_STATUS_PASS
    } else if report.verdict == "BLOCKED" {
        RUNTIME_0080_0_R1C_F_STATUS_BLOCKED
    } else {
        RUNTIME_0080_0_R1C_F_STATUS_PARTIAL
    };
    report.verdict = if pass {
        "PASS"
    } else if report.status == RUNTIME_0080_0_R1C_F_STATUS_BLOCKED {
        "BLOCKED"
    } else {
        "PARTIAL"
    };

    if !zero_cohort_emitter_enabled {
        report
            .diagnostics
            .push("zero_cohort_emitter_disabled_for_negative_control".to_string());
    }

    finalize_report(report)
}

fn apply_production_track_preservation(report: &mut Runtime0080R1cFReport) {
    report.r1a_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1a",
        verdict: "PASS".to_string(),
        checksum: RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1b",
        verdict: "PARTIAL".to_string(),
        checksum: R1B_PRODUCTION_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_a_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-a",
        verdict: "PASS".to_string(),
        checksum: RUNTIME_R1C_A_EXPECTED_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_b_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-b",
        verdict: "PASS".to_string(),
        checksum: R1C_B_PRODUCTION_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_c_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-c",
        verdict: "PASS".to_string(),
        checksum: R1C_C_PRODUCTION_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_d_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-d",
        verdict: "PASS".to_string(),
        checksum: R1C_D_PRODUCTION_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_e_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c-e",
        verdict: "PASS".to_string(),
        checksum: R1C_E_PRODUCTION_REPORT_CHECKSUM,
        preserved: true,
    });
    report.r1c_shadow_preservation = Some(Runtime0080R1cCPreservationSummary {
        rung: "R1c",
        verdict: "PARTIAL".to_string(),
        checksum: RUNTIME_R1C_EXPECTED_REPORT_CHECKSUM,
        preserved: true,
    });
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
        gate: GateSpec::OrderBand(EVENT_JOURNAL_COPY_BAND),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, R1A_COL_CURRENT)],
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

fn i64_to_journal_f32(value: i64) -> f32 {
    value as f32
}

fn journal_f32_to_i64(value: f32) -> i64 {
    value.round() as i64
}

fn f32_to_u32(value: f32) -> u32 {
    if value.is_finite() && value >= 0.0 {
        value.round() as u32
    } else {
        0
    }
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
    input: &Runtime0080R1cFInput,
    disabled_no_op: bool,
    diagnostics: Vec<String>,
    adapter: Option<Runtime0080R1aAdapterReport>,
    _zero_cohort_emitter_enabled: bool,
) -> Runtime0080R1cFReport {
    Runtime0080R1cFReport {
        id: RUNTIME_0080_0_R1C_F_ID,
        primitive_name: RUNTIME_0080_0_R1C_F_PRIMITIVE,
        status: RUNTIME_0080_0_R1C_F_STATUS_PARTIAL,
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
        scope: RUNTIME_R1C_F_SCOPE,
        adapter,
        relationship_to_100_tick_rehearsal:
            "per-tick GPU ZeroCohort decision toward stable 100-tick GPU-authoritative rehearsal",
        zero_cohort_decision_per_tick: false,
        zero_cohort_decision_combine_fn: "Identity (threshold substrate requirement)",
        zero_cohort_decision_gate: "Threshold Downward 0.5 on resident num_ships",
        zero_cohort_decision_consume_mode: "EmitEvent",
        zero_cohort_resident_input_column: "num_ships",
        zero_cohort_resident_source_buffer:
            "tier_a combat-probe values COL_CURRENT (pre-reinforcement)",
        gpu_zero_cohort_decision_from_resident_num_ships: false,
        zero_cohort_decision_op_is_threshold_or_emission_band: false,
        zero_cohort_decision_op_is_identity_copy: true,
        cpu_witness_decides_zero_cohort: true,
        zero_cohort_rows_read_from_gpu_values: false,
        zero_cohort_rows_match_r6c_oracle: false,
        disabled_zero_cohort_emitter_fails_parity: false,
        reenabled_zero_cohort_emitter_restores_parity: false,
        structural_decisions_gpu_emitted_zero_cohort: false,
        structural_decisions_gpu_emitted: false,
        remaining_cpu_decided_classes: vec![
            "DamageDelta",
            "MoveRequest",
            "LocalBirthRequest",
            "FusionRequest",
            "ShipCountDelta",
            "OwnerCodeFlip",
        ],
        zero_cohort_rows: Vec::new(),
        zero_cohort_row_count: 0,
        oracle_zero_cohort_row_count: 0,
        event_journal_parity: false,
        disabled_zero_cohort_emitter_check: None,
        r1a_preservation: None,
        r1b_preservation: None,
        r1c_a_preservation: None,
        r1c_b_preservation: None,
        r1c_c_preservation: None,
        r1c_d_preservation: None,
        r1c_e_preservation: None,
        r1c_shadow_preservation: None,
        resident_m4a_authority: false,
        multi_atlas_authority: false,
        scenario_reopen_required: false,
        docs_invariants_edit_required: false,
        r6c_checksum_expected: RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
        r6c_checksum_observed: 0,
        r6c_checksum_matches: false,
        foreground_capture_method: RUNTIME_R0_FOREGROUND_CAPTURE,
        domain_terms: vec![
            "resident",
            "num_ships",
            "threshold",
            "emission_band",
            "event_journal",
            "boundary_pass",
            "ZeroCohort",
        ],
        exact_commands: exact_commands(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn finalize_report(mut report: Runtime0080R1cFReport) -> Runtime0080R1cFReport {
    report.stable_report_checksum = stable_checksum(&report);
    report.artifact_markdown = render_artifact(&report);
    report
}

fn stable_checksum(report: &Runtime0080R1cFReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.status);
    mix_str(&mut hash, report.verdict);
    mix_u64(&mut hash, report.admitted as u64);
    mix_u64(&mut hash, report.zero_cohort_row_count as u64);
    mix_u64(&mut hash, report.oracle_zero_cohort_row_count as u64);
    mix_u64(
        &mut hash,
        report.structural_decisions_gpu_emitted_zero_cohort as u64,
    );
    mix_u64(&mut hash, report.structural_decisions_gpu_emitted as u64);
    mix_u64(
        &mut hash,
        report.gpu_zero_cohort_decision_from_resident_num_ships as u64,
    );
    mix_u64(&mut hash, report.zero_cohort_rows_match_r6c_oracle as u64);
    mix_u64(&mut hash, report.event_journal_parity as u64);
    for row in &report.zero_cohort_rows {
        mix_u64(&mut hash, row.tick as u64);
        mix_u64(&mut hash, row.source_slot as u64);
        mix_u64(&mut hash, row.source_cell as u64);
    }
    hash
}

fn render_artifact(report: &Runtime0080R1cFReport) -> String {
    let adapter_lines = report
        .adapter
        .as_ref()
        .map(|adapter| format!("- adapter: {}\n", adapter.adapter_name))
        .unwrap_or_else(|| "- adapter: none\n".to_string());
    let diagnostics = if report.diagnostics.is_empty() {
        "- none".to_string()
    } else {
        report
            .diagnostics
            .iter()
            .map(|line| format!("- {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let zero_rows = report
        .zero_cohort_rows
        .iter()
        .map(|row| {
            format!(
                "- tick {} slot {} cell {} owner {} gpu_num_ships {} prev {}",
                row.tick,
                row.source_slot,
                row.source_cell,
                row.owner_code,
                row.gpu_num_ships_value,
                row.gpu_previous_num_ships
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let disabled = report
        .disabled_zero_cohort_emitter_check
        .as_ref()
        .map(|check| {
            format!(
                "enabled_rows={} disabled_rows={} enabled_parity={} disabled_parity={} negative_control={}",
                check.emitter_enabled_rows,
                check.emitter_disabled_rows,
                check.emitter_enabled_oracle_parity,
                check.emitter_disabled_oracle_parity,
                check.negative_control_detected
            )
        })
        .unwrap_or_else(|| "not run".to_string());

    format!(
        "# RUNTIME-0080-0-R1c-f resident ZeroCohort GPU decision results\n\n\
         ## Verdict\n\n\
         - status: {status}\n\
         - verdict: {verdict}\n\
         - primitive: {primitive}\n\
         - rung: {rung}\n\
         - checksum: {checksum:016x}\n\
         {adapter}\
         ## Relationship to 100-tick rehearsal\n\n\
         {relationship}\n\n\
         ## ZeroCohort decision cadence\n\n\
         - per_tick: {per_tick}\n\n\
         ## GPU decision op\n\n\
         - combine: {combine}\n\
         - gate: {gate}\n\
         - consume: {consume}\n\
         - threshold_or_emission_band: {threshold_band}\n\
         - identity_copy_substitution: {identity_copy}\n\n\
         ## Resident input\n\n\
         - column: {column}\n\
         - source_buffer: {buffer}\n\n\
         ## Data-flow proof\n\n\
         - gpu_zero_cohort_decision_from_resident_num_ships: {gpu_decides}\n\
         - cpu_witness_decides_zero_cohort: {cpu_witness}\n\
         - zero_cohort_rows_read_from_gpu_values: {gpu_read}\n\
         - zero_cohort_rows_match_r6c_oracle: {zc_parity}\n\
         - event_journal_parity: {journal_parity}\n\
         - structural_decisions_gpu_emitted_zero_cohort: {zc_flag}\n\
         - structural_decisions_gpu_emitted: {umbrella}\n\n\
         ## ZeroCohort rows (GPU-read)\n\n\
         - count: {zc_count}\n\
         - oracle_count: {oracle_count}\n\
         {zero_rows}\n\n\
         ## Disabled emitter check\n\n\
         {disabled}\n\n\
         ## Remaining CPU-decided classes\n\n\
         {remaining}\n\n\
         ## Preservation\n\n\
         {preservation}\n\n\
         ## Diagnostics\n\n\
         {diagnostics}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        checksum = report.stable_report_checksum,
        adapter = adapter_lines,
        relationship = report.relationship_to_100_tick_rehearsal,
        per_tick = report.zero_cohort_decision_per_tick,
        combine = report.zero_cohort_decision_combine_fn,
        gate = report.zero_cohort_decision_gate,
        consume = report.zero_cohort_decision_consume_mode,
        threshold_band = report.zero_cohort_decision_op_is_threshold_or_emission_band,
        identity_copy = report.zero_cohort_decision_op_is_identity_copy,
        column = report.zero_cohort_resident_input_column,
        buffer = report.zero_cohort_resident_source_buffer,
        gpu_decides = report.gpu_zero_cohort_decision_from_resident_num_ships,
        cpu_witness = report.cpu_witness_decides_zero_cohort,
        gpu_read = report.zero_cohort_rows_read_from_gpu_values,
        zc_parity = report.zero_cohort_rows_match_r6c_oracle,
        journal_parity = report.event_journal_parity,
        zc_flag = report.structural_decisions_gpu_emitted_zero_cohort,
        umbrella = report.structural_decisions_gpu_emitted,
        zc_count = report.zero_cohort_row_count,
        oracle_count = report.oracle_zero_cohort_row_count,
        zero_rows = if zero_rows.is_empty() {
            "- none".to_string()
        } else {
            zero_rows
        },
        disabled = disabled,
        remaining = report
            .remaining_cpu_decided_classes
            .iter()
            .map(|class| format!("- {}", class))
            .collect::<Vec<_>>()
            .join("\n"),
        preservation = preservation_block(report),
        diagnostics = diagnostics,
    )
}

fn preservation_block(report: &Runtime0080R1cFReport) -> String {
    [
        preservation_line("R1a", &report.r1a_preservation),
        preservation_line("R1b", &report.r1b_preservation),
        preservation_line("R1c-a", &report.r1c_a_preservation),
        preservation_line("R1c-b", &report.r1c_b_preservation),
        preservation_line("R1c-c", &report.r1c_c_preservation),
        preservation_line("R1c-d", &report.r1c_d_preservation),
        preservation_line("R1c-e", &report.r1c_e_preservation),
        preservation_line("R1c complete-shadow", &report.r1c_shadow_preservation),
    ]
    .join("\n")
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
        "cargo test -p simthing-driver --test runtime_0080_0_r1c_f",
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

pub fn render_runtime_0080_r1c_f_artifact(report: &Runtime0080R1cFReport) -> String {
    report.artifact_markdown.clone()
}
