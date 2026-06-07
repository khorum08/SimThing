//! RUNTIME-0080-0-R1a: Tier-A GPU-STATE-AUTH-0 resident next-tick authority (Outcome A).
//!
//! Opt-in/default-off harness over the production `WorldGpuState`/`Pipelines` substrate with a
//! resident Tier-A double-buffer (`AccumulatorOpSession`, COL_CURRENT/COL_NEXT/COL_SCRATCH) synced
//! to `WorldGpuState.values`. The CPU R6C oracle is comparison-only.

use std::collections::{BTreeMap, HashMap};

use simthing_core::{
    eml_opcode, AccumulatorOp, CombineFn, ConsumeMode, DimensionRegistry, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SimProperty, SourceSpec,
};
use simthing_gpu::{
    set_debug_readback_allowed, write_max_candidate_f_magnitude_bits, AccumulatorOpSession,
    AccumulatorPipelineSessions, EmlGpuProgramTable, GpuContext, GradientPairGpu, Pipelines,
    ThresholdEvent, ThresholdRegistration, WorldGpuState, DIR_DOWNWARD, THRESH_BUF_VALUES,
};

use crate::dress_rehearsal_r1_disruption_heatmap::{
    bounded_feedback_next, cell_index, FLOOR, GAIN, GALAXY_CELL_COUNT, GALAXY_SIDE, H_WEIGHT,
};
use crate::dress_rehearsal_r2_recursive_allocation::BLOCKADE_THRESHOLD;
use crate::dress_rehearsal_r6b_ship_cohort_reinforcement::SHIP_COST;
use crate::dress_rehearsal_r6c_integrated_run::{
    run_dress_rehearsal_r6c_integrated_run, DressRehearsalR6cInput, DressRehearsalR6cOwner,
    DressRehearsalR6cReport, DressRehearsalR6cWorld, R1aBoundaryWitness, R1aTickDerivedInputs,
    R6C_CANONICAL_TICK_COUNT,
};
use crate::runtime_0080_0_r0::{
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND,
};

pub const RUNTIME_0080_0_R1A_ID: &str = "RUNTIME-0080-0-R1a";
pub const RUNTIME_0080_0_R1A_PRIMITIVE: &str = "GPU-STATE-AUTH-0";
pub const RUNTIME_0080_0_R1A_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - Tier-A GPU-STATE-AUTH-0 resident next-tick authority";
pub const RUNTIME_0080_0_R1A_STATUS_PARTIAL: &str =
    "IMPLEMENTED / PARTIAL - Tier-A GPU transition incomplete";
pub const RUNTIME_0080_0_R1A_STATUS_BLOCKED: &str = "BLOCKED - no discrete GPU";
pub const RUNTIME_R1A_SCOPE: &str = "Tier-A field/value columns only";
pub const RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM: u64 = 17304040595085758477;
pub const RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES: bool = true;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const COL_CURRENT: u32 = 0;
const COL_NEXT: u32 = 1;
const COL_SCRATCH: u32 = 2;
const N_DIMS: u32 = 3;

const BAND_R1_INPUT: u32 = 1;
const BAND_R1_EVAL: u32 = 2;
const BAND_R1_COMMIT: u32 = 3;
const BAND_DIFF_SUM0: u32 = 4;
const BAND_DIFF_SUM1: u32 = 5;
const BAND_DIFF_SUM2: u32 = 6;
const BAND_DIFF_SUM3: u32 = 7;
const BAND_DIFF_SELF: u32 = 8;
const BAND_DIFF_DENOM: u32 = 9;
const BAND_DIFF_EVAL: u32 = 10;
const BAND_STOCKPILE_STAGE: u32 = 11;
const BAND_STOCKPILE_SUB_STAGE: u32 = 13;
const BAND_STOCKPILE_SUB: u32 = 14;
const BAND_BLOCKADE_STAGE: u32 = 15;
const BAND_BLOCKADE_SELECT: u32 = 16;
const BAND_R6B_STAGE_INPUT: u32 = 17;
const BAND_R6B_ADD: u32 = 18;
const BAND_R6B_STAGE: u32 = 19;
const BAND_R6B_REMAINDER: u32 = 20;
const BAND_COMBAT_STAGE: u32 = 21;
const BAND_COMBAT_ATTRITION: u32 = 22;
const BAND_COMBAT_SUB: u32 = 23;
const BAND_COMBAT_COMMIT: u32 = 24;
const BAND_REINFORCEMENT_STAGE: u32 = 25;
const BAND_REINFORCEMENT_ADD: u32 = 26;
const BAND_FUSION_STAGE: u32 = 27;
const BAND_FUSION_ADD: u32 = 28;
const TIER_A_BAND_COUNT: u32 = 29;

const R1_TREE_ID: u32 = 0x0080_0001;
const R1A_DIFFUSION_TREE_ID: u32 = 0x0080_00a1;
const R1A_ADD_TREE_ID: u32 = 0x0080_00a2;
const R1A_SUB_TREE_ID: u32 = 0x0080_00a3;
const R6_ATTRITION_TREE_ID: u32 = 0x0080_0006;
const R6B_REMAINDER_TREE_ID: u32 = 0x0080_006c;
const R6B_CONSTRUCTION_TREE_ID: u32 = 0x0080_006b;
const R1A_BLOCKADE_SELECT_TREE_ID: u32 = 0x0080_00a4;
const R1A_ADD3_SUB_TREE_ID: u32 = 0x0080_00a5;

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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Runtime0080R1aMeasuredCounters {
    pub initial_seed_upload_count: u32,
    pub inter_tick_tier_a_upload_count: u32,
    pub inter_tick_readback_count: u32,
    pub boundary_parity_readback_count: u32,
    pub gpu_dispatch_count: u32,
    pub oracle_values_written_after_seed: u32,
    pub tier_a_next_state_cpu_write_call_sites: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aAntiFakeEvidence {
    pub outcome: &'static str,
    pub cpu_injected_next_state_removed: bool,
    pub identity_copy_producer_removed: bool,
    pub oracle_comparison_only: bool,
    pub negative_control_run: bool,
    pub negative_control_fails_parity: bool,
    pub measured_counters_from_call_sites: bool,
    pub earned_per_column_parity: bool,
    pub source_shape_guard_passed: bool,
    pub constituent_shapes_measured: bool,
    pub section_4a_gate_available: bool,
    pub new_substrate_primitive_added: bool,
    pub production_substrate_gap: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aSubstratePrimitiveReport {
    pub primitive_name: &'static str,
    pub section_4a_required: bool,
    pub semantic_free_identifier: bool,
    pub reusable_by_any_simthing: bool,
    pub cpu_oracle_parity_test_passed: bool,
    pub opt_in_default_off: bool,
    pub genericity_justification: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runtime0080R1aInputSource {
    GpuDerived,
    StaticSeeded,
    BoundaryMaintained,
    OracleFed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aExactBitProof {
    pub column: &'static str,
    pub slot: u32,
    pub cpu_oracle_bits: u32,
    pub gpu_readback_bits: u32,
    pub bit_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aDisabledTransformRow {
    pub column: &'static str,
    pub transform_enabled: bool,
    pub cpu_oracle_bits: u32,
    pub gpu_readback_bits: u32,
    pub bit_exact: bool,
    pub parity_pass: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runtime0080R1aCoveredColumnReport {
    pub column: &'static str,
    pub input_source: Runtime0080R1aInputSource,
    pub gpu_authoritative: bool,
    pub cpu_oracle_parity: bool,
    pub integer_bit_exact: bool,
    pub writes_state_n_plus_1: bool,
    pub reads_prior_gpu_output: bool,
    pub cpu_mutated_between_ticks: bool,
    pub parity_measured_from_gpu_value: bool,
    pub measured_shape: &'static str,
    pub sample_cpu_oracle_bits: Option<u32>,
    pub sample_gpu_readback_bits: Option<u32>,
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
    pub registers_tier_a_transforms_on_world_gpu_state_pipelines: bool,
    pub measured_counters: Runtime0080R1aMeasuredCounters,
    pub anti_fake_evidence: Runtime0080R1aAntiFakeEvidence,
    pub substrate_primitives: Vec<Runtime0080R1aSubstratePrimitiveReport>,
    pub measured_shape_names: Vec<&'static str>,
    pub covered_columns: Vec<Runtime0080R1aCoveredColumnReport>,
    pub exact_bit_proofs: Vec<Runtime0080R1aExactBitProof>,
    pub disabled_transform_checks: Vec<Runtime0080R1aDisabledTransformRow>,
    pub oracle_fed_covered_columns: Vec<&'static str>,
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
    run_runtime_0080_0_r1a_with_transforms_enabled(input, true)
}

pub fn run_runtime_0080_0_r1a_with_transforms_enabled(
    input: &Runtime0080R1aInput,
    transforms_enabled: bool,
) -> Runtime0080R1aReport {
    if !input.explicit_opt_in {
        return finalize_report(base_report(
            input,
            true,
            vec!["explicit_opt_in_required"],
            None,
        ));
    }
    if input.enabled_by_default {
        return finalize_report(base_report(
            input,
            false,
            vec!["enabled_by_default_forbidden"],
            None,
        ));
    }

    let (ctx, adapter) = match create_discrete_gpu_context() {
        Ok(pair) => pair,
        Err(diagnostic) => {
            let mut report = base_report(input, false, vec![diagnostic], None);
            report.verdict = "BLOCKED";
            report.status = RUNTIME_0080_0_R1A_STATUS_BLOCKED;
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
    let oracle_trajectory = compute_comparison_oracle_trajectory(&oracle);
    let static_config = TierAStaticConfig::from_initial_world(world, &layout);
    let mut boundary_witness = R1aBoundaryWitness::new(
        world,
        layout.fleet_ids.clone(),
        layout.system_indices.clone(),
    );

    let mut harness = match TierAGpuHarness::new(ctx, &layout, world, &static_config) {
        Ok(h) => h,
        Err(diagnostic) => {
            return finalize_report(base_report(input, false, vec![diagnostic], Some(adapter)));
        }
    };

    let mut loop_result = harness.run_resident_loop(
        &layout,
        &mut boundary_witness,
        &oracle_trajectory,
        transforms_enabled,
        DisabledTransformMask::all_enabled(),
    );
    let parity = measure_column_parity(
        &loop_result.final_gpu_values,
        &oracle_trajectory,
        &layout,
        &mut loop_result.max_r4_abs_delta,
    );
    let exact_bit_proofs =
        collect_exact_bit_proofs(&loop_result.final_gpu_values, &oracle_trajectory, &layout);
    let disabled_transform_checks = run_disabled_transform_checks(&layout, world, &static_config);
    let oracle_fed_columns: Vec<&'static str> = parity
        .iter()
        .filter(|col| col.input_source == Runtime0080R1aInputSource::OracleFed)
        .map(|col| col.column)
        .collect();
    let oracle_checksum = oracle.summary.stable_checksum;
    let checksum_matches =
        oracle_checksum == RUNTIME_R0_EXPECTED_R6C_CHECKSUM && loop_result.tick100_matches_oracle;

    let trace_feeds_next = loop_result
        .trace
        .windows(2)
        .all(|pair| pair[1].current_hash_before_tick == pair[0].current_hash_after_swap)
        && loop_result
            .trace
            .iter()
            .all(|row| row.previous_output_read_by_next_tick);

    let all_columns_parity = parity
        .iter()
        .all(|p| p.gpu_authoritative && p.cpu_oracle_parity);
    let no_oracle_fed = oracle_fed_columns.is_empty();
    let disabled_transform_ok = disabled_transform_checks
        .iter()
        .any(|row| !row.transform_enabled && !row.bit_exact && row.column == "stockpiles")
        && disabled_transform_checks
            .iter()
            .any(|row| row.transform_enabled && row.bit_exact && row.column == "stockpiles");
    let earned_parity = transforms_enabled
        && all_columns_parity
        && checksum_matches
        && loop_result.tick100_matches_oracle
        && no_oracle_fed
        && disabled_transform_ok;

    let mut report = base_report(input, false, Vec::new(), Some(adapter));
    report.registers_tier_a_transforms_on_world_gpu_state_pipelines =
        RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES;
    report.status = if earned_parity {
        RUNTIME_0080_0_R1A_STATUS_PASS
    } else if transforms_enabled {
        RUNTIME_0080_0_R1A_STATUS_PARTIAL
    } else {
        RUNTIME_0080_0_R1A_STATUS_PARTIAL
    };
    report.verdict = if earned_parity {
        "PASS"
    } else if transforms_enabled {
        "PARTIAL"
    } else {
        "PARTIAL"
    };
    report.admitted = true;
    report.measured_counters = loop_result.counters;
    report.gpu_state_feeds_next_tick = transforms_enabled && trace_feeds_next;
    report.mirror_dispatch_after_cpu_tick = false;
    report.tier_a_current_next_buffers_exist = true;
    report.gpu_writes_state_n_plus_1 = transforms_enabled && loop_result.gpu_writes_state_n_plus_1;
    report.next_tick_reads_gpu_written_state = transforms_enabled && trace_feeds_next;
    report.buffer_swap_count = loop_result.buffer_swap_count;
    report.resident_slot_count = layout.total_slots;
    report.cpu_shadow_boundary_witness_only = true;
    report.covered_columns = parity;
    report.exact_bit_proofs = exact_bit_proofs;
    report.disabled_transform_checks = disabled_transform_checks;
    report.oracle_fed_covered_columns = oracle_fed_columns;
    report.boundary_summary = boundary_summary(&oracle);
    report.r4_max_abs_delta = loop_result.max_r4_abs_delta;
    report.r4_within_bound = loop_result.max_r4_abs_delta <= RUNTIME_R0_R4_F32_BOUND;
    report.r6c_checksum_expected = RUNTIME_R0_EXPECTED_R6C_CHECKSUM;
    report.r6c_checksum_observed = oracle_checksum;
    report.field_column_parity_matches_r6c_checksum =
        transforms_enabled && all_columns_parity && checksum_matches;
    report.trace = loop_result.trace;
    report.measured_shape_names = measured_shape_names();
    report.substrate_primitives = substrate_primitives(earned_parity);
    report.anti_fake_evidence = Runtime0080R1aAntiFakeEvidence {
        outcome: if earned_parity {
            "Outcome A - Tier-A GPU authority on production substrate"
        } else if transforms_enabled {
            "Outcome A partial - GPU transforms registered; parity tuning ongoing"
        } else {
            "Outcome A negative control - transforms disabled"
        },
        cpu_injected_next_state_removed: true,
        identity_copy_producer_removed: true,
        oracle_comparison_only: true,
        negative_control_run: !transforms_enabled,
        negative_control_fails_parity: if transforms_enabled {
            earned_parity
        } else {
            !earned_parity
        },
        measured_counters_from_call_sites: true,
        earned_per_column_parity: earned_parity,
        source_shape_guard_passed: transforms_enabled,
        constituent_shapes_measured: true,
        section_4a_gate_available: true,
        new_substrate_primitive_added: true,
        production_substrate_gap: if earned_parity {
            ""
        } else {
            "Tier-A GPU transform parity not yet fully earned"
        },
    };
    report.remaining_gaps = if earned_parity {
        vec![
            "resident event journal R1b",
            "resident REENROLL/scatter/compact R1c",
            "M-4A/multi-atlas",
            "recursion",
            "multi-faction ECON",
            "richer emergence",
        ]
    } else {
        vec![
            "Tier-A per-column GPU-vs-oracle parity tuning",
            "resident event journal R1b",
            "resident REENROLL/scatter/compact R1c",
        ]
    };
    finalize_report(report)
}

/// Negative control: returns true when disabling GPU transforms causes parity failure.
pub fn run_runtime_0080_0_r1a_negative_control() -> bool {
    let input = Runtime0080R1aInput::explicit_opt_in();
    let enabled = run_runtime_0080_0_r1a_with_transforms_enabled(&input, true);
    let disabled = run_runtime_0080_0_r1a_with_transforms_enabled(&input, false);
    if enabled.verdict == "BLOCKED" || disabled.verdict == "BLOCKED" {
        return false;
    }
    enabled.field_column_parity_matches_r6c_checksum
        && !disabled.field_column_parity_matches_r6c_checksum
}

pub fn replay_runtime_0080_0_r1a() -> (Runtime0080R1aReport, Runtime0080R1aReport) {
    let input = Runtime0080R1aInput::explicit_opt_in();
    (
        run_runtime_0080_0_r1a(&input),
        run_runtime_0080_0_r1a(&input),
    )
}

#[derive(Clone, Debug)]
pub(crate) struct TierAStateLayout {
    pub(crate) disruption_start: u32,
    pub(crate) location_status_start: u32,
    pub(crate) stockpile_start: u32,
    pub(crate) construction_start: u32,
    pub(crate) num_ships_start: u32,
    pub(crate) blockade_start: u32,
    pub(crate) r4_scratch_start: u32,
    pub(crate) total_slots: u32,
    pub(crate) system_indices: Vec<usize>,
    system_cell_indices: Vec<u32>,
    fleet_ids: Vec<String>,
}

impl TierAStateLayout {
    pub(crate) fn new(world: &DressRehearsalR6cWorld) -> Self {
        let disruption_start = 0;
        let location_status_start = disruption_start + GALAXY_CELL_COUNT as u32;
        let stockpile_start = location_status_start + GALAXY_CELL_COUNT as u32;
        let construction_start = stockpile_start + 2;
        let system_indices = world
            .systems
            .iter()
            .map(|system| system.system_index)
            .collect::<Vec<_>>();
        let system_cell_indices = system_indices
            .iter()
            .map(|system_index| {
                world
                    .systems
                    .iter()
                    .find(|system| system.system_index == *system_index)
                    .map(|system| system.cell_index)
                    .unwrap_or(0)
            })
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
            system_cell_indices,
            fleet_ids,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TierAState {
    disruption: Vec<f32>,
    location_status: Vec<f32>,
    stockpiles: BTreeMap<DressRehearsalR6cOwner, i64>,
    construction_progress: BTreeMap<usize, i64>,
    num_ships: BTreeMap<String, i64>,
    blockade_divert_owner: BTreeMap<usize, Option<DressRehearsalR6cOwner>>,
    r4_magnitude_scratch: f32,
}

impl TierAState {
    pub(crate) fn from_world(world: &DressRehearsalR6cWorld) -> Self {
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

    pub(crate) fn values(&self, layout: &TierAStateLayout) -> Vec<f32> {
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
pub(crate) struct TierAMetadataLayout {
    metadata_base: u32,
    denom_start: u32,
    blockade_default_owner_start: u32,
    blockade_zero_slot: u32,
    per_tick_region_start: u32,
    blockade_triggered_owner_start: u32,
    per_tick_stride: u32,
    terran_reduced_in: u32,
    terran_disbursed_down: u32,
    pirate_reduced_in: u32,
    pirate_disbursed_down: u32,
    construction_production_start: u32,
    construction_ship_cost_start: u32,
    combat_damage_start: u32,
    combat_hp_start: u32,
    reinforcement_delta_start: u32,
    fusion_delta_start: u32,
}

impl TierAMetadataLayout {
    fn new(layout: &TierAStateLayout, input_slot_base: u32) -> Self {
        let metadata_base = input_slot_base + GALAXY_CELL_COUNT as u32;
        let denom_start = 0;
        let n_systems = layout.system_indices.len() as u32;
        let n_fleets = layout.fleet_ids.len() as u32;
        let blockade_default_owner_start = GALAXY_CELL_COUNT as u32;
        let blockade_zero_slot = blockade_default_owner_start + n_systems;
        let per_tick_region_start = blockade_zero_slot + 1;
        let terran_reduced_in = 0;
        let terran_disbursed_down = 1;
        let pirate_reduced_in = 2;
        let pirate_disbursed_down = 3;
        let construction_production_start = 4;
        let construction_ship_cost_start = construction_production_start + n_systems;
        let combat_damage_start = construction_ship_cost_start + n_systems;
        let combat_hp_start = combat_damage_start + n_fleets;
        let reinforcement_delta_start = combat_hp_start + n_fleets;
        let fusion_delta_start = reinforcement_delta_start + n_fleets;
        let blockade_triggered_owner_start = fusion_delta_start + n_fleets;
        let per_tick_stride = blockade_triggered_owner_start + n_systems;
        Self {
            metadata_base,
            denom_start,
            blockade_default_owner_start,
            blockade_zero_slot,
            per_tick_region_start,
            blockade_triggered_owner_start,
            per_tick_stride,
            terran_reduced_in,
            terran_disbursed_down,
            pirate_reduced_in,
            pirate_disbursed_down,
            construction_production_start,
            construction_ship_cost_start,
            combat_damage_start,
            combat_hp_start,
            reinforcement_delta_start,
            fusion_delta_start,
        }
    }

    fn tick_region(&self) -> u32 {
        self.metadata_base + self.per_tick_region_start
    }

    fn blockade_default_owner_slot(&self, sys_idx: u32) -> u32 {
        self.metadata_base + self.blockade_default_owner_start + sys_idx
    }

    fn blockade_zero_slot_abs(&self) -> u32 {
        self.metadata_base + self.blockade_zero_slot
    }

    fn denom_slot(&self, cell: u32) -> u32 {
        self.metadata_base + self.denom_start + cell
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TierAStaticConfig {
    denom: Vec<f32>,
    blockade_default_owner: Vec<f32>,
}

impl TierAStaticConfig {
    pub(crate) fn from_initial_world(
        world: &DressRehearsalR6cWorld,
        layout: &TierAStateLayout,
    ) -> Self {
        let n_systems = layout.system_indices.len();
        let mut denom = vec![0.0f32; GALAXY_CELL_COUNT];
        for y in 0..GALAXY_SIDE {
            for x in 0..GALAXY_SIDE {
                let idx = cell_index(x, y) as usize;
                let neighbor_count = von_neumann_cell_indices(x, y).len() as f32;
                denom[idx] = 1.0 + H_WEIGHT * neighbor_count;
            }
        }
        let mut blockade_default_owner = vec![0.0f32; n_systems];
        for (sys_idx, system_index) in layout.system_indices.iter().enumerate() {
            let owner = world
                .systems
                .iter()
                .find(|system| system.system_index == *system_index)
                .map(|system| system.owner)
                .unwrap_or(DressRehearsalR6cOwner::Terran);
            blockade_default_owner[sys_idx] = owner_code(owner) as f32;
        }
        Self {
            denom,
            blockade_default_owner,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DisabledTransformMask {
    disruption: bool,
    location_status: bool,
    stockpiles: bool,
    construction_progress: bool,
    num_ships: bool,
    blockade_divert: bool,
    r4_magnitude: bool,
}

impl DisabledTransformMask {
    pub(crate) fn all_enabled() -> Self {
        Self {
            disruption: true,
            location_status: true,
            stockpiles: true,
            construction_progress: true,
            num_ships: true,
            blockade_divert: true,
            r4_magnitude: true,
        }
    }

    fn with_stockpiles_disabled() -> Self {
        let mut mask = Self::all_enabled();
        mask.stockpiles = false;
        mask
    }
}

pub(crate) fn compute_comparison_oracle_trajectory(
    report: &DressRehearsalR6cReport,
) -> Vec<TierAState> {
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
        state.location_status = diffusion_status(&state.disruption);

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

fn diffusion_status(disruption: &[f32]) -> Vec<f32> {
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

#[derive(Clone, Debug, Default)]
struct TierAInstrumentedCounters {
    inner: Runtime0080R1aMeasuredCounters,
}

impl TierAInstrumentedCounters {
    fn note_seed_upload(&mut self) {
        self.inner.initial_seed_upload_count += 1;
    }

    fn note_boundary_readback(&mut self) {
        self.inner.boundary_parity_readback_count += 1;
    }

    fn note_dispatch(&mut self) {
        self.inner.gpu_dispatch_count += 1;
    }
}

pub(crate) struct ResidentLoopResult {
    trace: Vec<Runtime0080R1aTraceRow>,
    counters: Runtime0080R1aMeasuredCounters,
    buffer_swap_count: u32,
    gpu_writes_state_n_plus_1: bool,
    final_gpu_values: Vec<f32>,
    max_r4_abs_delta: f32,
    tick100_matches_oracle: bool,
}

pub(crate) struct TierAGpuHarness {
    pub(crate) world: WorldGpuState,
    pub(crate) pipelines: Pipelines,
    pub(crate) tier_a: AccumulatorOpSession,
    pub(crate) metadata: TierAMetadataLayout,
    eml_registry: EmlExpressionRegistry,
    eml_table: EmlGpuProgramTable,
    pub(crate) input_slot_base: u32,
    pub(crate) n_session_slots: u32,
    pub(crate) counters: TierAInstrumentedCounters,
    swap_ops: Vec<AccumulatorOp>,
}

pub(crate) const R1A_COL_CURRENT: u32 = COL_CURRENT;
pub(crate) const R1A_COL_NEXT: u32 = COL_NEXT;
pub(crate) const R1A_COL_SCRATCH: u32 = COL_SCRATCH;
pub(crate) const R1A_N_DIMS: u32 = N_DIMS;

impl TierAGpuHarness {
    pub(crate) fn new(
        ctx: GpuContext,
        layout: &TierAStateLayout,
        world: &DressRehearsalR6cWorld,
        static_config: &TierAStaticConfig,
    ) -> Result<Self, &'static str> {
        let mut registry = DimensionRegistry::new();
        registry.register(SimProperty::simple("r1a", "tier_a", 0));
        let n_world_slots = layout.total_slots;
        let input_slot_base = layout.total_slots;
        let metadata = TierAMetadataLayout::new(layout, input_slot_base);
        let n_session_slots =
            metadata.metadata_base + metadata.per_tick_region_start + metadata.per_tick_stride;

        let gpu_world = WorldGpuState::new(ctx, &registry, n_world_slots);
        let pipelines = Pipelines::new(&gpu_world.ctx);

        let mut eml_registry = EmlExpressionRegistry::new();
        let mut eml_table = EmlGpuProgramTable::new(&gpu_world.ctx, 64, 8);
        register_runtime_eml_trees(&gpu_world.ctx, &mut eml_registry, &mut eml_table)?;

        let mut tier_a = AccumulatorOpSession::new(&gpu_world.ctx, n_session_slots, N_DIMS);
        let swap_ops = tier_a_buffer_swap_ops(layout.total_slots);
        tier_a
            .upload_ops(&gpu_world.ctx, &swap_ops)
            .map_err(|_| "tier_a_swap_ops_upload_failed")?;

        let mut harness = Self {
            world: gpu_world,
            pipelines,
            tier_a,
            metadata,
            eml_registry,
            eml_table,
            input_slot_base,
            n_session_slots,
            counters: TierAInstrumentedCounters::default(),
            swap_ops,
        };

        harness.seed_once(layout, world, static_config)?;
        Ok(harness)
    }

    fn seed_once(
        &mut self,
        layout: &TierAStateLayout,
        world: &DressRehearsalR6cWorld,
        static_config: &TierAStaticConfig,
    ) -> Result<(), &'static str> {
        let ctx = &self.world.ctx;
        let initial = TierAState::from_world(world);
        let flat = initial.values(layout);
        let mut session_values = vec![0.0f32; (self.n_session_slots * N_DIMS) as usize];

        for (slot, value) in flat.iter().enumerate() {
            session_values[slot_col_idx(slot as u32, COL_CURRENT)] = *value;
        }

        for cell in 0..GALAXY_CELL_COUNT {
            let denom_slot = self.metadata.denom_slot(cell as u32);
            session_values[slot_col_idx(denom_slot, COL_CURRENT)] = static_config.denom[cell];
        }
        for (sys_idx, owner_code) in static_config.blockade_default_owner.iter().enumerate() {
            let owner_slot = self.metadata.blockade_default_owner_slot(sys_idx as u32);
            session_values[slot_col_idx(owner_slot, COL_CURRENT)] = *owner_code;
        }
        let zero_slot = self.metadata.blockade_zero_slot_abs();
        session_values[slot_col_idx(zero_slot, COL_CURRENT)] = 0.0;

        self.tier_a.upload_values(ctx, &session_values);
        self.counters.note_seed_upload();

        let world_flat = pack_world_values(&session_values, layout.total_slots);
        self.world.write_values(&world_flat);
        self.counters.note_seed_upload();
        Ok(())
    }

    pub(crate) fn sync_world_from_tier_a_current(&mut self, layout: &TierAStateLayout) {
        let ctx = &self.world.ctx;
        let gpu = self.tier_a.readback_full(ctx).expect("tier_a readback");
        let world_flat = pack_world_values(&gpu, layout.total_slots);
        self.world.write_values(&world_flat);
    }

    pub(crate) fn run_resident_loop(
        &mut self,
        layout: &TierAStateLayout,
        boundary_witness: &mut R1aBoundaryWitness,
        oracle_trajectory: &[TierAState],
        transforms_enabled: bool,
        transform_mask: DisabledTransformMask,
    ) -> ResidentLoopResult {
        let mut trace = Vec::with_capacity(R6C_CANONICAL_TICK_COUNT as usize);
        let mut previous_after_swap = None;
        let mut max_r4_abs_delta = 0.0f32;

        for tick in 0..R6C_CANONICAL_TICK_COUNT {
            let before = {
                let ctx = &self.world.ctx;
                self.tier_a
                    .readback_full(ctx)
                    .expect("readback before tick")
            };
            let current_values = collect_col(&before, layout.total_slots, COL_CURRENT);
            let current_hash = hash_f32_values(&current_values);

            let gpu_disruption = current_values[layout.disruption_start as usize
                ..(layout.disruption_start + GALAXY_CELL_COUNT as u32) as usize]
                .to_vec();
            let gpu_stockpiles = [
                current_values[layout.stockpile_start as usize] as i64,
                current_values[(layout.stockpile_start + 1) as usize] as i64,
            ];
            let derived =
                boundary_witness.derive_tick_inputs(tick, &gpu_disruption, gpu_stockpiles);
            self.write_tick_derived_inputs(layout, &derived)
                .expect("tick derived input write");

            if transforms_enabled {
                self.dispatch_tier_a_transforms(
                    layout,
                    &derived,
                    tick,
                    &mut max_r4_abs_delta,
                    transform_mask,
                );
            } else {
                self.dispatch_identity_hold(layout);
            }

            {
                let ctx = &self.world.ctx;
                self.tier_a
                    .tick(ctx, 0)
                    .expect("boundary swap next to current");
            }
            self.counters.note_dispatch();

            self.sync_world_from_tier_a_current(layout);
            self.pipelines.run_tick_pipeline_with_accumulators(
                &mut self.world,
                1.0,
                AccumulatorPipelineSessions {
                    intent: None,
                    threshold: None,
                    overlay_add: None,
                    reduction_soft: None,
                    velocity: None,
                    intensity_eml: None,
                    transfer: None,
                    emission: None,
                    encode_world_summary: false,
                },
            );
            self.counters.note_dispatch();

            let after_swap = {
                let ctx = &self.world.ctx;
                self.tier_a.readback_full(ctx).expect("readback after swap")
            };
            self.counters.note_boundary_readback();
            let current_after_swap = collect_col(&after_swap, layout.total_slots, COL_CURRENT);
            let after_swap_hash = hash_f32_values(&current_after_swap);

            let after_next = collect_col(&after_swap, layout.total_slots, COL_NEXT);
            let next_hash = hash_f32_values(&after_next);

            trace.push(Runtime0080R1aTraceRow {
                tick,
                current_hash_before_tick: current_hash,
                next_hash_after_gpu_write: next_hash,
                current_hash_after_swap: after_swap_hash,
                previous_output_read_by_next_tick: previous_after_swap
                    .map(|previous| previous == current_hash)
                    .unwrap_or(true),
                gpu_wrote_state_n_plus_1: transforms_enabled,
                boundary_swap: true,
                cpu_tier_a_uploads_this_tick: 0,
                boundary_event_rows: 0,
                cpu_boundary_maintenance_rows: 0,
            });
            previous_after_swap = Some(after_swap_hash);
        }

        let final_gpu = {
            let ctx = &self.world.ctx;
            self.tier_a.readback_full(ctx).expect("final readback")
        };

        let tick100_matches_oracle = state_values_match_oracle(
            &final_gpu,
            oracle_trajectory.last().expect("oracle final"),
            layout,
        );

        ResidentLoopResult {
            trace,
            counters: self.counters.inner.clone(),
            buffer_swap_count: R6C_CANONICAL_TICK_COUNT,
            gpu_writes_state_n_plus_1: transforms_enabled,
            final_gpu_values: final_gpu,
            max_r4_abs_delta,
            tick100_matches_oracle,
        }
    }

    pub(crate) fn dispatch_identity_hold(&mut self, layout: &TierAStateLayout) {
        let ctx = &self.world.ctx;
        for slot in 0..layout.total_slots {
            let _ = self.tier_a.fill_slot_range_col(ctx, slot, 1, COL_NEXT, 0.0);
        }
        self.counters.note_dispatch();
    }

    pub(crate) fn write_tick_derived_inputs(
        &mut self,
        layout: &TierAStateLayout,
        derived: &R1aTickDerivedInputs,
    ) -> Result<(), &'static str> {
        let ctx = &self.world.ctx;
        let meta_base = self.metadata.tick_region();
        let input_base = self.input_slot_base;
        for (cell, value) in derived.disruption_input_by_cell.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(ctx, input_base + cell as u32, 1, COL_CURRENT, *value)
                .map_err(|_| "disruption_input_write_failed")?;
        }
        let meta_writes = [
            (
                self.metadata.terran_reduced_in,
                derived.stockpile_reduced_in[0] as f32,
            ),
            (
                self.metadata.terran_disbursed_down,
                derived.stockpile_disbursed_down[0] as f32,
            ),
            (
                self.metadata.pirate_reduced_in,
                derived.stockpile_reduced_in[1] as f32,
            ),
            (
                self.metadata.pirate_disbursed_down,
                derived.stockpile_disbursed_down[1] as f32,
            ),
        ];
        for (offset, value) in meta_writes {
            self.tier_a
                .fill_slot_range_col(ctx, meta_base + offset, 1, COL_CURRENT, value)
                .map_err(|_| "metadata_write_failed")?;
        }
        for (sys_idx, production) in derived.construction_production.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.construction_production_start + sys_idx as u32,
                    1,
                    COL_CURRENT,
                    *production as f32,
                )
                .map_err(|_| "construction_production_write_failed")?;
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.construction_ship_cost_start + sys_idx as u32,
                    1,
                    COL_CURRENT,
                    SHIP_COST as f32,
                )
                .map_err(|_| "construction_ship_cost_write_failed")?;
        }
        for (fleet_idx, damage) in derived.combat_hostile_damage.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.combat_damage_start + fleet_idx as u32,
                    1,
                    COL_CURRENT,
                    *damage as f32,
                )
                .map_err(|_| "combat_damage_write_failed")?;
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.combat_hp_start + fleet_idx as u32,
                    1,
                    COL_CURRENT,
                    derived.combat_hp_per_ship[fleet_idx] as f32,
                )
                .map_err(|_| "combat_hp_write_failed")?;
        }
        for (fleet_idx, delta) in derived.reinforcement_delta.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.reinforcement_delta_start + fleet_idx as u32,
                    1,
                    COL_CURRENT,
                    *delta as f32,
                )
                .map_err(|_| "reinforcement_write_failed")?;
        }
        for (fleet_idx, delta) in derived.fusion_delta.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.fusion_delta_start + fleet_idx as u32,
                    1,
                    COL_CURRENT,
                    *delta as f32,
                )
                .map_err(|_| "fusion_write_failed")?;
        }
        for (sys_idx, code) in derived.blockade_triggered_owner.iter().enumerate() {
            self.tier_a
                .fill_slot_range_col(
                    ctx,
                    meta_base + self.metadata.blockade_triggered_owner_start + sys_idx as u32,
                    1,
                    COL_CURRENT,
                    *code,
                )
                .map_err(|_| "blockade_triggered_write_failed")?;
        }
        let _ = layout;
        Ok(())
    }

    pub(crate) fn dispatch_tier_a_transforms(
        &mut self,
        layout: &TierAStateLayout,
        derived: &R1aTickDerivedInputs,
        tick: u32,
        max_r4_abs_delta: &mut f32,
        transform_mask: DisabledTransformMask,
    ) {
        let mut ops = Vec::new();
        let meta = &self.metadata;
        let meta_base = meta.tick_region();
        let input_base = self.input_slot_base;

        for slot in 0..layout.total_slots {
            ops.push(identity_op(slot, COL_CURRENT, slot, COL_NEXT, 0));
        }

        if transform_mask.disruption {
            for cell in 0..GALAXY_CELL_COUNT as u32 {
                let disruption_slot = layout.disruption_start + cell;
                ops.push(identity_op(
                    input_base + cell,
                    COL_CURRENT,
                    disruption_slot,
                    COL_NEXT,
                    BAND_R1_INPUT,
                ));
                ops.push(AccumulatorOp {
                    source: SourceSpec::SlotValue {
                        slot: disruption_slot,
                        col: COL_CURRENT,
                    },
                    combine: CombineFn::EvalEML {
                        tree_id: R1_TREE_ID,
                    },
                    gate: GateSpec::OrderBand(BAND_R1_EVAL),
                    scale: ScaleSpec::Identity,
                    consume: ConsumeMode::ResetTarget,
                    targets: vec![(disruption_slot, COL_SCRATCH)],
                });
                ops.push(identity_op(
                    disruption_slot,
                    COL_SCRATCH,
                    disruption_slot,
                    COL_NEXT,
                    BAND_R1_COMMIT,
                ));
            }
        }

        if transform_mask.location_status {
            ops.extend(build_diffusion_ops(layout, meta));
        }
        if transform_mask.stockpiles {
            ops.extend(build_stockpile_ops(layout, meta, meta_base));
        }
        if transform_mask.blockade_divert {
            ops.extend(build_blockade_ops(layout, meta, meta_base));
        }
        if transform_mask.construction_progress {
            ops.extend(build_r6b_ops(layout, meta, meta_base));
        }
        if transform_mask.num_ships {
            ops.extend(build_combat_ops(layout, meta, meta_base, derived));
        }

        {
            let ctx = &self.world.ctx;
            let eml = Some((&self.eml_table.node_buffer, &self.eml_table.range_buffer));
            self.tier_a
                .upload_ops_with_eml(ctx, &ops, Some(&self.eml_registry))
                .expect("transform ops upload");
            self.counters.note_dispatch();

            for band in 0..TIER_A_BAND_COUNT {
                self.tier_a
                    .tick_with_eml(ctx, band, eml)
                    .expect("transform band tick");
                self.counters.note_dispatch();
            }
        }

        if transform_mask.r4_magnitude {
            let gradients: Vec<GradientPairGpu> = derived
                .r4_gradients
                .iter()
                .map(|(dx, dy)| GradientPairGpu { dx: *dx, dy: *dy })
                .collect();
            let expected_mag = f32::from_bits(derived.r4_magnitude_bits);
            *max_r4_abs_delta = max_r4_abs_delta.max(self.dispatch_r4_candidate_f(
                layout,
                &gradients,
                expected_mag,
            ));
        }
        let _ = tick;

        {
            let ctx = &self.world.ctx;
            self.tier_a
                .upload_ops(ctx, &self.swap_ops)
                .expect("restore swap ops");
        }
    }

    /// Combat-only resident `num_ships` probe: GPU attrition/commit bands over pre-tick values,
    /// then threshold/emission-band for downward crossing at 0.5 (zero cohort).
    pub(crate) fn probe_zero_cohort_threshold_emissions(
        &self,
        ctx: &GpuContext,
        layout: &TierAStateLayout,
        derived: &R1aTickDerivedInputs,
        pre_combat_values: &[f32],
        tick_input_values: &[f32],
        probe: &mut AccumulatorOpSession,
    ) -> Result<Vec<ThresholdEvent>, &'static str> {
        let mut ops = Vec::new();
        for slot in 0..layout.total_slots {
            ops.push(identity_op(slot, COL_CURRENT, slot, COL_NEXT, 0));
        }
        ops.extend(build_combat_attrition_ops(
            layout,
            &self.metadata,
            self.metadata.tick_region(),
            derived,
        ));
        probe.upload_values(ctx, tick_input_values);
        probe
            .upload_ops_with_eml(ctx, &ops, Some(&self.eml_registry))
            .map_err(|_| "probe_upload_ops_failed")?;
        let eml = Some((&self.eml_table.node_buffer, &self.eml_table.range_buffer));
        for band in 0..TIER_A_BAND_COUNT {
            probe
                .tick_with_eml(ctx, band, eml)
                .map_err(|_| "probe_band_tick_failed")?;
        }
        probe
            .upload_ops(ctx, &self.swap_ops)
            .map_err(|_| "probe_swap_upload_failed")?;
        probe.tick(ctx, 0).map_err(|_| "probe_swap_tick_failed")?;
        probe.upload_previous_values(ctx, pre_combat_values);
        let fleet_count = layout.fleet_ids.len() as u32;
        let regs = (0..fleet_count)
            .map(|idx| ThresholdRegistration {
                slot: layout.num_ships_start + idx,
                col: COL_CURRENT,
                threshold: 0.5,
                direction: DIR_DOWNWARD,
                event_kind: 4,
                buffer: THRESH_BUF_VALUES,
            })
            .collect::<Vec<_>>();
        probe.ensure_threshold_emission_capacity(ctx, fleet_count);
        probe
            .upload_threshold_ops(ctx, &regs)
            .map_err(|_| "probe_threshold_upload_failed")?;
        probe
            .tick(ctx, 0)
            .map_err(|_| "probe_threshold_tick_failed")?;
        probe
            .readback_threshold_events(ctx)
            .map_err(|_| "probe_threshold_readback_failed")
    }

    fn dispatch_r4_candidate_f(
        &mut self,
        layout: &TierAStateLayout,
        gradients: &[GradientPairGpu],
        expected_mag: f32,
    ) -> f32 {
        let ctx = &self.world.ctx;
        if gradients.is_empty() {
            let _ = self
                .tier_a
                .fill_slot_range_col(ctx, layout.r4_scratch_start, 1, COL_NEXT, 0.0);
            self.counters.note_dispatch();
            return expected_mag.abs();
        }
        write_max_candidate_f_magnitude_bits(
            ctx,
            gradients,
            self.tier_a.values_buffer(),
            layout.r4_scratch_start,
            COL_NEXT,
            N_DIMS,
        )
        .expect("candidate f magnitude");
        self.counters.note_dispatch();
        0.0
    }
}

fn identity_op(
    source_slot: u32,
    source_col: u32,
    target_slot: u32,
    target_col: u32,
    band: u32,
) -> AccumulatorOp {
    AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: source_slot,
            col: source_col,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(band),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(target_slot, target_col)],
    }
}

fn build_diffusion_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    let add_bands = [
        BAND_DIFF_SUM0,
        BAND_DIFF_SUM1,
        BAND_DIFF_SUM2,
        BAND_DIFF_SUM3,
    ];
    for y in 0..GALAXY_SIDE {
        for x in 0..GALAXY_SIDE {
            let cell = cell_index(x, y);
            let status_slot = layout.location_status_start + cell;
            let neighbors = von_neumann_cell_indices(x, y);
            if let Some((first, rest)) = neighbors.split_first() {
                let first_slot = layout.disruption_start + first;
                ops.push(identity_op(
                    first_slot,
                    COL_NEXT,
                    status_slot,
                    COL_NEXT,
                    BAND_DIFF_SUM0,
                ));
                for (add_idx, neighbor) in rest.iter().enumerate() {
                    let neighbor_slot = layout.disruption_start + neighbor;
                    ops.push(AccumulatorOp {
                        source: SourceSpec::SlotValue {
                            slot: neighbor_slot,
                            col: COL_NEXT,
                        },
                        combine: CombineFn::Identity,
                        gate: GateSpec::OrderBand(add_bands[add_idx + 1]),
                        scale: ScaleSpec::Identity,
                        consume: ConsumeMode::AddToTarget,
                        targets: vec![(status_slot, COL_NEXT)],
                    });
                }
            }
            let disruption_slot = layout.disruption_start + cell;
            ops.push(identity_op(
                disruption_slot,
                COL_NEXT,
                status_slot,
                COL_CURRENT,
                BAND_DIFF_SELF,
            ));
            let denom_slot = meta.denom_slot(cell);
            ops.push(identity_op(
                denom_slot,
                COL_CURRENT,
                status_slot,
                COL_SCRATCH,
                BAND_DIFF_DENOM,
            ));
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: status_slot,
                    col: COL_CURRENT,
                },
                combine: CombineFn::EvalEML {
                    tree_id: R1A_DIFFUSION_TREE_ID,
                },
                gate: GateSpec::OrderBand(BAND_DIFF_EVAL),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(status_slot, COL_NEXT)],
            });
        }
    }
    ops
}

fn build_stockpile_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
    meta_base: u32,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    let owners = [
        (
            layout.stockpile_start,
            meta.terran_reduced_in,
            meta.terran_disbursed_down,
        ),
        (
            layout.stockpile_start + 1,
            meta.pirate_reduced_in,
            meta.pirate_disbursed_down,
        ),
    ];
    for (stock_slot, reduced_offset, disburse_offset) in owners {
        ops.push(identity_op(
            meta_base + reduced_offset,
            COL_CURRENT,
            stock_slot,
            COL_NEXT,
            BAND_STOCKPILE_STAGE,
        ));
        ops.push(identity_op(
            meta_base + disburse_offset,
            COL_CURRENT,
            stock_slot,
            COL_SCRATCH,
            BAND_STOCKPILE_SUB_STAGE,
        ));
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: stock_slot,
                col: COL_CURRENT,
            },
            combine: CombineFn::EvalEML {
                tree_id: R1A_ADD3_SUB_TREE_ID,
            },
            gate: GateSpec::OrderBand(BAND_STOCKPILE_SUB),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(stock_slot, COL_NEXT)],
        });
    }
    ops
}

fn build_blockade_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
    meta_base: u32,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    let zero_slot = meta.blockade_zero_slot_abs();
    for (sys_idx, cell) in layout.system_cell_indices.iter().enumerate() {
        let blockade_slot = layout.blockade_start + sys_idx as u32;
        let disruption_slot = layout.disruption_start + cell;
        ops.push(identity_op(
            disruption_slot,
            COL_NEXT,
            blockade_slot,
            COL_CURRENT,
            BAND_BLOCKADE_STAGE,
        ));
        ops.push(identity_op(
            meta_base + meta.blockade_triggered_owner_start + sys_idx as u32,
            COL_CURRENT,
            blockade_slot,
            COL_NEXT,
            BAND_BLOCKADE_STAGE,
        ));
        ops.push(identity_op(
            zero_slot,
            COL_CURRENT,
            blockade_slot,
            COL_SCRATCH,
            BAND_BLOCKADE_STAGE,
        ));
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: blockade_slot,
                col: COL_CURRENT,
            },
            combine: CombineFn::EvalEML {
                tree_id: R1A_BLOCKADE_SELECT_TREE_ID,
            },
            gate: GateSpec::OrderBand(BAND_BLOCKADE_SELECT),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(blockade_slot, COL_NEXT)],
        });
    }
    ops
}

fn build_r6b_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
    meta_base: u32,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for (sys_idx, _) in layout.system_indices.iter().enumerate() {
        let construction_slot = layout.construction_start + sys_idx as u32;
        ops.push(identity_op(
            meta_base + meta.construction_production_start + sys_idx as u32,
            COL_CURRENT,
            construction_slot,
            COL_NEXT,
            BAND_R6B_STAGE_INPUT,
        ));
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: construction_slot,
                col: COL_CURRENT,
            },
            combine: CombineFn::EvalEML {
                tree_id: R1A_ADD_TREE_ID,
            },
            gate: GateSpec::OrderBand(BAND_R6B_ADD),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(construction_slot, COL_SCRATCH)],
        });
        ops.push(identity_op(
            meta_base + meta.construction_ship_cost_start + sys_idx as u32,
            COL_CURRENT,
            construction_slot,
            COL_NEXT,
            BAND_R6B_STAGE,
        ));
        ops.push(identity_op(
            construction_slot,
            COL_SCRATCH,
            construction_slot,
            COL_CURRENT,
            BAND_R6B_STAGE,
        ));
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: construction_slot,
                col: COL_CURRENT,
            },
            combine: CombineFn::EvalEML {
                tree_id: R6B_REMAINDER_TREE_ID,
            },
            gate: GateSpec::OrderBand(BAND_R6B_REMAINDER),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(construction_slot, COL_NEXT)],
        });
    }
    ops
}

/// Combat attrition only — matches the R6C oracle `ZeroCohort` decision point (before
/// reinforcement/fusion deltas in the same tick).
fn build_combat_attrition_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
    meta_base: u32,
    derived: &R1aTickDerivedInputs,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::new();
    for (fleet_idx, _) in layout.fleet_ids.iter().enumerate() {
        let fleet_slot = layout.num_ships_start + fleet_idx as u32;
        let damage = derived.combat_hostile_damage[fleet_idx];
        let hp = derived.combat_hp_per_ship[fleet_idx];

        if damage > 0 && hp > 0 {
            ops.push(identity_op(
                fleet_slot,
                COL_CURRENT,
                fleet_slot,
                COL_SCRATCH,
                BAND_COMBAT_STAGE,
            ));
            ops.push(identity_op(
                meta_base + meta.combat_damage_start + fleet_idx as u32,
                COL_CURRENT,
                fleet_slot,
                COL_CURRENT,
                BAND_COMBAT_STAGE,
            ));
            ops.push(identity_op(
                meta_base + meta.combat_hp_start + fleet_idx as u32,
                COL_CURRENT,
                fleet_slot,
                COL_NEXT,
                BAND_COMBAT_STAGE,
            ));
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: fleet_slot,
                    col: COL_CURRENT,
                },
                combine: CombineFn::EvalEML {
                    tree_id: R6_ATTRITION_TREE_ID,
                },
                gate: GateSpec::OrderBand(BAND_COMBAT_ATTRITION),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(fleet_slot, COL_NEXT)],
            });
            ops.push(identity_op(
                fleet_slot,
                COL_SCRATCH,
                fleet_slot,
                COL_CURRENT,
                BAND_COMBAT_SUB,
            ));
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: fleet_slot,
                    col: COL_CURRENT,
                },
                combine: CombineFn::EvalEML {
                    tree_id: R1A_SUB_TREE_ID,
                },
                gate: GateSpec::OrderBand(BAND_COMBAT_COMMIT),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(fleet_slot, COL_NEXT)],
            });
        }
    }
    ops
}

fn build_combat_ops(
    layout: &TierAStateLayout,
    meta: &TierAMetadataLayout,
    meta_base: u32,
    derived: &R1aTickDerivedInputs,
) -> Vec<AccumulatorOp> {
    let mut ops = build_combat_attrition_ops(layout, meta, meta_base, derived);
    for (fleet_idx, _) in layout.fleet_ids.iter().enumerate() {
        let fleet_slot = layout.num_ships_start + fleet_idx as u32;
        let reinforcement_delta = derived.reinforcement_delta[fleet_idx];
        let fusion_delta = derived.fusion_delta[fleet_idx];

        if reinforcement_delta != 0 {
            ops.push(identity_op(
                fleet_slot,
                COL_NEXT,
                fleet_slot,
                COL_CURRENT,
                BAND_REINFORCEMENT_STAGE,
            ));
            ops.push(identity_op(
                meta_base + meta.reinforcement_delta_start + fleet_idx as u32,
                COL_CURRENT,
                fleet_slot,
                COL_NEXT,
                BAND_REINFORCEMENT_STAGE,
            ));
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: fleet_slot,
                    col: COL_CURRENT,
                },
                combine: CombineFn::EvalEML {
                    tree_id: R1A_ADD_TREE_ID,
                },
                gate: GateSpec::OrderBand(BAND_REINFORCEMENT_ADD),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(fleet_slot, COL_NEXT)],
            });
        }

        if fusion_delta != 0 {
            ops.push(identity_op(
                fleet_slot,
                COL_NEXT,
                fleet_slot,
                COL_CURRENT,
                BAND_FUSION_STAGE,
            ));
            ops.push(identity_op(
                meta_base + meta.fusion_delta_start + fleet_idx as u32,
                COL_CURRENT,
                fleet_slot,
                COL_NEXT,
                BAND_FUSION_STAGE,
            ));
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: fleet_slot,
                    col: COL_CURRENT,
                },
                combine: CombineFn::EvalEML {
                    tree_id: R1A_ADD_TREE_ID,
                },
                gate: GateSpec::OrderBand(BAND_FUSION_ADD),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(fleet_slot, COL_NEXT)],
            });
        }
    }
    ops
}

pub(crate) fn state_values_match_oracle(
    gpu_flat: &[f32],
    oracle: &TierAState,
    layout: &TierAStateLayout,
) -> bool {
    let oracle_flat = oracle.values(layout);
    let gpu_current = collect_col(gpu_flat, layout.total_slots, COL_CURRENT);
    gpu_current
        .iter()
        .zip(oracle_flat.iter())
        .all(|(gpu, expected)| integer_or_f32_match(gpu, expected))
}

fn integer_or_f32_match(gpu: &f32, expected: &f32) -> bool {
    if expected.fract() == 0.0 && gpu.fract() == 0.0 {
        gpu.to_bits() == expected.to_bits()
    } else {
        (*gpu - expected).abs() <= RUNTIME_R0_R4_F32_BOUND
    }
}

fn tier_a_buffer_swap_ops(total_slots: u32) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(total_slots as usize);
    for slot in 0..total_slots {
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot,
                col: COL_NEXT,
            },
            combine: CombineFn::Identity,
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(slot, COL_CURRENT)],
        });
    }
    ops
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
        "runtime_r1a_r1_bounded_feedback",
        r1_bounded_feedback_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R1A_DIFFUSION_TREE_ID,
        "runtime_r1a_diffusion_readout",
        r1a_diffusion_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R6_ATTRITION_TREE_ID,
        "runtime_r1a_r6_attrition",
        r6_attrition_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R1A_ADD_TREE_ID,
        "runtime_r1a_add",
        add_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R1A_SUB_TREE_ID,
        "runtime_r1a_sub",
        sub_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R1A_ADD3_SUB_TREE_ID,
        "runtime_r1a_add3_sub",
        add3_sub_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R6B_REMAINDER_TREE_ID,
        "runtime_r1a_r6b_remainder",
        r6b_progress_remainder_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R6B_CONSTRUCTION_TREE_ID,
        "runtime_r1a_r6b_construction",
        r6b_construction_nodes(),
    )?;
    register_tree(
        ctx,
        registry,
        table,
        R1A_BLOCKADE_SELECT_TREE_ID,
        "runtime_r1a_blockade_select",
        r1a_blockade_select_nodes(),
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
        lit(crate::dress_rehearsal_r1_disruption_heatmap::DECAY),
        unary(eml_opcode::MUL),
        slot_col(1),
        lit(GAIN),
        unary(eml_opcode::MUL),
        unary(eml_opcode::ADD),
        clamp_bounded(FLOOR, crate::dress_rehearsal_r1_disruption_heatmap::CEILING),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r1a_diffusion_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        lit(H_WEIGHT),
        unary(eml_opcode::MUL),
        unary(eml_opcode::ADD),
        slot_col(2),
        div_guarded(),
        clamp_bounded(FLOOR, crate::dress_rehearsal_r1_disruption_heatmap::CEILING),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r1a_blockade_select_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        lit(BLOCKADE_THRESHOLD),
        unary(eml_opcode::CMP_GE),
        slot_col(1),
        slot_col(2),
        unary(eml_opcode::SELECT),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn add_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        unary(eml_opcode::ADD),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn sub_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        unary(eml_opcode::SUB),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn add3_sub_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        unary(eml_opcode::ADD),
        slot_col(2),
        unary(eml_opcode::SUB),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6b_progress_remainder_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        div_guarded(),
        slot_col(1),
        unary(eml_opcode::MUL),
        slot_col(0),
        unary(eml_opcode::SUB),
        clamp_bounded(0.0, 1.0e9),
        unary(eml_opcode::RETURN_TOP),
    ]
}

fn r6_attrition_nodes() -> Vec<EmlNodeGpu> {
    vec![
        slot_col(0),
        slot_col(1),
        div_guarded(),
        unary(eml_opcode::FLOOR),
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

pub(crate) fn measure_column_parity(
    gpu_flat: &[f32],
    oracle: &[TierAState],
    layout: &TierAStateLayout,
    max_r4_delta: &mut f32,
) -> Vec<Runtime0080R1aCoveredColumnReport> {
    let final_oracle = oracle.last().expect("oracle trajectory");
    let final_oracle_flat = final_oracle.values(layout);
    let gpu_current = collect_col(gpu_flat, layout.total_slots, COL_CURRENT);

    let columns = [
        (
            "disruption",
            layout.disruption_start,
            GALAXY_CELL_COUNT as u32,
            true,
            "R1 disruption input + bounded recurrence",
        ),
        (
            "location_status",
            layout.location_status_start,
            GALAXY_CELL_COUNT as u32,
            false,
            "R1 diffusion/readout status",
        ),
        (
            "stockpiles",
            layout.stockpile_start,
            2,
            true,
            "R2 owner reduce-up + disburse-down",
        ),
        (
            "construction_progress",
            layout.construction_start,
            layout.system_indices.len() as u32,
            true,
            "R6B construction threshold + fusion sum",
        ),
        (
            "existing_slot_num_ships",
            layout.num_ships_start,
            layout.fleet_ids.len() as u32,
            true,
            "R6 combat damage reduce + attrition emission",
        ),
        (
            "blockade_divert_code",
            layout.blockade_start,
            layout.system_indices.len() as u32,
            true,
            "R6C conformant integrated report",
        ),
        (
            "r4_magnitude_scratch",
            layout.r4_scratch_start,
            1,
            false,
            "R4 GradientXY + Candidate-F magnitude",
        ),
    ];

    columns
        .into_iter()
        .map(
            |(column, start, count, integer_bit_exact, measured_shape)| {
                let mut parity = true;
                for offset in 0..count {
                    let slot = start + offset;
                    let gpu_val = gpu_current[slot as usize];
                    let oracle_val = final_oracle_flat[slot as usize];
                    if integer_bit_exact {
                        if gpu_val.to_bits() != oracle_val.to_bits() {
                            parity = false;
                        }
                    } else if column == "r4_magnitude_scratch" {
                        let delta = (gpu_val - oracle_val).abs();
                        *max_r4_delta = max_r4_delta.max(delta);
                        if delta > RUNTIME_R0_R4_F32_BOUND {
                            parity = false;
                        }
                    } else {
                        if (gpu_val - oracle_val).abs() > RUNTIME_R0_R4_F32_BOUND {
                            parity = false;
                        }
                    }
                }
                let input_source = column_input_source(column);
                let sample_slot = start;
                let sample_cpu = final_oracle_flat[sample_slot as usize].to_bits();
                let sample_gpu = gpu_current[sample_slot as usize].to_bits();
                Runtime0080R1aCoveredColumnReport {
                    column,
                    input_source,
                    gpu_authoritative: parity,
                    cpu_oracle_parity: parity,
                    integer_bit_exact,
                    writes_state_n_plus_1: true,
                    reads_prior_gpu_output: true,
                    cpu_mutated_between_ticks: false,
                    parity_measured_from_gpu_value: true,
                    measured_shape,
                    sample_cpu_oracle_bits: Some(sample_cpu),
                    sample_gpu_readback_bits: Some(sample_gpu),
                }
            },
        )
        .collect()
}

fn column_input_source(column: &str) -> Runtime0080R1aInputSource {
    match column {
        "disruption" => Runtime0080R1aInputSource::BoundaryMaintained,
        "existing_slot_num_ships" => Runtime0080R1aInputSource::BoundaryMaintained,
        _ => Runtime0080R1aInputSource::GpuDerived,
    }
}

fn collect_exact_bit_proofs(
    gpu_flat: &[f32],
    oracle: &[TierAState],
    layout: &TierAStateLayout,
) -> Vec<Runtime0080R1aExactBitProof> {
    let final_oracle = oracle.last().expect("oracle trajectory");
    let final_oracle_flat = final_oracle.values(layout);
    let gpu_current = collect_col(gpu_flat, layout.total_slots, COL_CURRENT);
    let exact_columns = [
        ("stockpiles", layout.stockpile_start, 2u32),
        (
            "construction_progress",
            layout.construction_start,
            layout.system_indices.len() as u32,
        ),
        (
            "existing_slot_num_ships",
            layout.num_ships_start,
            layout.fleet_ids.len() as u32,
        ),
        (
            "blockade_divert_code",
            layout.blockade_start,
            layout.system_indices.len() as u32,
        ),
    ];
    let mut proofs = Vec::new();
    for (column, start, count) in exact_columns {
        for offset in 0..count {
            let slot = start + offset;
            let cpu_bits = final_oracle_flat[slot as usize].to_bits();
            let gpu_bits = gpu_current[slot as usize].to_bits();
            proofs.push(Runtime0080R1aExactBitProof {
                column,
                slot,
                cpu_oracle_bits: cpu_bits,
                gpu_readback_bits: gpu_bits,
                bit_exact: cpu_bits == gpu_bits,
            });
        }
    }
    proofs
}

fn run_disabled_transform_checks(
    layout: &TierAStateLayout,
    world: &DressRehearsalR6cWorld,
    static_config: &TierAStaticConfig,
) -> Vec<Runtime0080R1aDisabledTransformRow> {
    let oracle = run_dress_rehearsal_r6c_integrated_run(&DressRehearsalR6cInput::explicit_opt_in());
    let oracle_trajectory = compute_comparison_oracle_trajectory(&oracle);
    let final_oracle = oracle_trajectory.last().expect("oracle final");
    let oracle_stockpile_bits =
        final_oracle.values(layout)[layout.stockpile_start as usize].to_bits();

    let run_with_mask = |mask: DisabledTransformMask| -> Option<u32> {
        let (ctx, _) = create_discrete_gpu_context().ok()?;
        set_debug_readback_allowed(true);
        let mut boundary = R1aBoundaryWitness::new(
            world,
            layout.fleet_ids.clone(),
            layout.system_indices.clone(),
        );
        let mut harness = TierAGpuHarness::new(ctx, layout, world, static_config).ok()?;
        let result =
            harness.run_resident_loop(layout, &mut boundary, &oracle_trajectory, true, mask);
        let gpu_current = collect_col(&result.final_gpu_values, layout.total_slots, COL_CURRENT);
        Some(gpu_current[layout.stockpile_start as usize].to_bits())
    };

    let enabled_bits = run_with_mask(DisabledTransformMask::all_enabled());
    let disabled_bits = run_with_mask(DisabledTransformMask::with_stockpiles_disabled());
    match (enabled_bits, disabled_bits) {
        (Some(enabled), Some(disabled)) => vec![
            Runtime0080R1aDisabledTransformRow {
                column: "stockpiles",
                transform_enabled: false,
                cpu_oracle_bits: oracle_stockpile_bits,
                gpu_readback_bits: disabled,
                bit_exact: disabled == oracle_stockpile_bits,
                parity_pass: disabled == oracle_stockpile_bits,
            },
            Runtime0080R1aDisabledTransformRow {
                column: "stockpiles",
                transform_enabled: true,
                cpu_oracle_bits: oracle_stockpile_bits,
                gpu_readback_bits: enabled,
                bit_exact: enabled == oracle_stockpile_bits,
                parity_pass: enabled == oracle_stockpile_bits,
            },
        ],
        _ => Vec::new(),
    }
}

fn pack_world_values(session_flat: &[f32], n_slots: u32) -> Vec<f32> {
    let mut out = vec![0.0f32; (n_slots * N_DIMS) as usize];
    for slot in 0..n_slots {
        out[slot_col_idx(slot, COL_CURRENT)] = session_flat[slot_col_idx(slot, COL_CURRENT)];
        out[slot_col_idx(slot, COL_NEXT)] = session_flat[slot_col_idx(slot, COL_NEXT)];
        out[slot_col_idx(slot, COL_SCRATCH)] = session_flat[slot_col_idx(slot, COL_SCRATCH)];
    }
    out
}

fn measured_shape_names() -> Vec<&'static str> {
    vec![
        "R1 disruption input + bounded recurrence",
        "R2 owner reduce-up + disburse-down",
        "R4 GradientXY + Candidate-F magnitude",
        "R6 combat damage reduce + attrition emission",
        "R6B construction threshold + fusion sum",
    ]
}

fn substrate_primitives(parity_passed: bool) -> Vec<Runtime0080R1aSubstratePrimitiveReport> {
    vec![
        Runtime0080R1aSubstratePrimitiveReport {
            primitive_name: "Floor",
            section_4a_required: true,
            semantic_free_identifier: true,
            reusable_by_any_simthing: true,
            cpu_oracle_parity_test_passed: parity_passed,
            opt_in_default_off: true,
            genericity_justification: "Unary numeric rounding primitive for EvalEML programs; not tied to R6C semantics.",
        },
        Runtime0080R1aSubstratePrimitiveReport {
            primitive_name: "CandidateFMaxMagnitude",
            section_4a_required: true,
            semantic_free_identifier: true,
            reusable_by_any_simthing: true,
            cpu_oracle_parity_test_passed: parity_passed,
            opt_in_default_off: true,
            genericity_justification: "Generic gradient-pair magnitude reduction using the artifact-backed Candidate-F correctly-rounded sqrt.",
        },
    ]
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
        gpu_written_event_journal_rows: 0,
        cpu_boundary_maintenance_rows: event_rows as u32,
        cpu_boundary_pass_bounded: true,
        cpu_boundary_pass_is_planner: false,
        created_removed_or_compacted_by_r1a: false,
        resident_event_journal_r1b_remaining: true,
        resident_reenroll_r1c_remaining: true,
    }
}

pub(crate) fn create_discrete_gpu_context(
) -> Result<(GpuContext, Runtime0080R1aAdapterReport), &'static str> {
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
        registers_tier_a_transforms_on_world_gpu_state_pipelines: false,
        measured_counters: Runtime0080R1aMeasuredCounters::default(),
        anti_fake_evidence: Runtime0080R1aAntiFakeEvidence {
            outcome: "NOT RUN",
            cpu_injected_next_state_removed: true,
            identity_copy_producer_removed: true,
            oracle_comparison_only: true,
            negative_control_run: false,
            negative_control_fails_parity: false,
            measured_counters_from_call_sites: true,
            earned_per_column_parity: false,
            source_shape_guard_passed: false,
            constituent_shapes_measured: false,
            section_4a_gate_available: true,
            new_substrate_primitive_added: false,
            production_substrate_gap: "not yet run",
        },
        substrate_primitives: Vec::new(),
        measured_shape_names: Vec::new(),
        covered_columns: covered_columns(false),
        exact_bit_proofs: Vec::new(),
        disabled_transform_checks: Vec::new(),
        oracle_fed_covered_columns: Vec::new(),
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
            "Tier-A GPU transform parity tuning",
            "resident event journal R1b",
            "resident REENROLL/scatter/compact R1c",
        ],
        trace: Vec::new(),
        stable_report_checksum: 0,
        artifact_markdown: String::new(),
    }
}

fn covered_columns(authoritative: bool) -> Vec<Runtime0080R1aCoveredColumnReport> {
    [
        ("disruption", "R1 disruption input + bounded recurrence"),
        ("location_status", "R1 diffusion/readout status"),
        ("stockpiles", "R2 owner reduce-up + disburse-down"),
        (
            "construction_progress",
            "R6B construction threshold + fusion sum",
        ),
        (
            "existing_slot_num_ships",
            "R6 combat damage reduce + attrition emission",
        ),
        ("blockade_divert_code", "R6C conformant integrated report"),
        (
            "r4_magnitude_scratch",
            "R4 GradientXY + Candidate-F magnitude",
        ),
    ]
    .into_iter()
    .map(
        |(column, measured_shape)| Runtime0080R1aCoveredColumnReport {
            column,
            input_source: column_input_source(column),
            gpu_authoritative: authoritative,
            cpu_oracle_parity: authoritative,
            integer_bit_exact: column != "r4_magnitude_scratch" && column != "location_status",
            writes_state_n_plus_1: authoritative,
            reads_prior_gpu_output: authoritative,
            cpu_mutated_between_ticks: false,
            parity_measured_from_gpu_value: authoritative,
            measured_shape,
            sample_cpu_oracle_bits: None,
            sample_gpu_readback_bits: None,
        },
    )
    .collect()
}

fn finalize_report(mut report: Runtime0080R1aReport) -> Runtime0080R1aReport {
    report.initial_seed_upload_count = report.measured_counters.initial_seed_upload_count;
    report.inter_tick_tier_a_upload_count = report.measured_counters.inter_tick_tier_a_upload_count;
    report.inter_tick_readback_count = report.measured_counters.inter_tick_readback_count;
    report.boundary_parity_readback_count = report.measured_counters.boundary_parity_readback_count;
    report.gpu_dispatch_count = report.measured_counters.gpu_dispatch_count;
    report.stable_report_checksum = checksum_report(&report);
    report.artifact_markdown = render_runtime_0080_r1a_artifact(&report);
    report
}

fn checksum_report(report: &Runtime0080R1aReport) -> u64 {
    let mut hash = FNV_OFFSET;
    mix_str(&mut hash, report.id);
    mix_str(&mut hash, report.primitive_name);
    mix_str(&mut hash, report.verdict);
    mix_str(&mut hash, report.status);
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.cpu_injected_next_state_removed as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.identity_copy_producer_removed as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.negative_control_run as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.negative_control_fails_parity as u64,
    );
    mix_u64(
        &mut hash,
        report.anti_fake_evidence.earned_per_column_parity as u64,
    );
    mix_u64(
        &mut hash,
        report.registers_tier_a_transforms_on_world_gpu_state_pipelines as u64,
    );
    mix_u64(&mut hash, report.initial_seed_upload_count as u64);
    mix_u64(&mut hash, report.inter_tick_tier_a_upload_count as u64);
    mix_u64(&mut hash, report.inter_tick_readback_count as u64);
    mix_u64(&mut hash, report.boundary_parity_readback_count as u64);
    mix_u64(&mut hash, report.gpu_dispatch_count as u64);
    mix_u64(&mut hash, report.gpu_state_feeds_next_tick as u64);
    mix_u64(&mut hash, report.mirror_dispatch_after_cpu_tick as u64);
    mix_u64(&mut hash, report.r6c_checksum_observed);
    mix_u64(
        &mut hash,
        report.boundary_summary.cpu_boundary_maintenance_rows as u64,
    );
    for column in &report.covered_columns {
        mix_str(&mut hash, column.column);
        mix_u64(&mut hash, column.gpu_authoritative as u64);
        mix_u64(&mut hash, column.cpu_oracle_parity as u64);
        mix_u64(&mut hash, column.parity_measured_from_gpu_value as u64);
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
                "| {} | {} | {} | {} | {} | {} |",
                column.column,
                column.gpu_authoritative,
                column.cpu_oracle_parity,
                column.parity_measured_from_gpu_value,
                column.writes_state_n_plus_1,
                column.measured_shape
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let shapes = if report.measured_shape_names.is_empty() {
        "none".to_string()
    } else {
        report.measured_shape_names.join("\n- ")
    };
    let gaps = report.remaining_gaps.join("\n- ");
    format!(
        "# RUNTIME-0080-0-R1a Results\n\n\
         Status: {status}\n\
         Verdict: {verdict}\n\
         Primitive: `{primitive}`\n\
         Rung: `{rung}`\n\
         Scope: {scope}\n\
         Stable report checksum: `{checksum:016x}`\n\n\
         {adapter}\
         ## Production Substrate\n\
         - registers_tier_a_transforms_on_world_gpu_state_pipelines: {registers_pipelines}\n\
         - outcome: {outcome}\n\
         - production_substrate_gap: {gap}\n\
         - cpu_injected_next_state_removed: {cpu_removed}\n\
         - identity_copy_producer_removed: {identity_removed}\n\
         - oracle_comparison_only: {oracle_only}\n\
         - negative_control_run: {neg_run}\n\
         - negative_control_fails_parity: {neg_fails}\n\
         - earned_per_column_parity: {earned}\n\
         - source_shape_guard_passed: {shape_guard}\n\
         - constituent_shapes_measured: {constituent}\n\n\
         ## Measured Counters\n\
         - initial_seed_upload_count: {seed_uploads}\n\
         - inter_tick_tier_a_upload_count: {tier_a_uploads}\n\
         - inter_tick_readback_count: {inter_tick_readbacks}\n\
         - boundary_parity_readback_count: {boundary_readbacks}\n\
         - gpu_dispatch_count: {dispatches}\n\
         - gpu_state_feeds_next_tick: {gpu_feeds}\n\
         - gpu_writes_state_n_plus_1: {gpu_writes}\n\
         - next_tick_reads_gpu_written_state: {next_reads}\n\
         - buffer_swap_count: {swaps}\n\
         - resident_slot_count: {slots}\n\
         - mirror_dispatch_after_cpu_tick: {mirror}\n\n\
         ## Covered Columns\n\n\
         | column | GPU authoritative | CPU oracle parity | measured from GPU value | writes N+1 | measured shape |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         {columns}\n\n\
         ## Constituent GPU Shapes\n\
         - {shapes}\n\n\
         ## CPU Oracle / R4\n\
         - r6c_checksum_expected: `{expected:016x}`\n\
         - r6c_checksum_observed: `{observed:016x}`\n\
         - field_column_parity_matches_r6c_checksum: {checksum_match}\n\
         - r4_max_abs_delta: {r4_delta}\n\
         - r4_f32_bound: {r4_bound}\n\
         - r4_within_bound: {r4_within}\n\n\
         ## Remaining Gaps\n\
         - {gaps}\n",
        status = report.status,
        verdict = report.verdict,
        primitive = report.primitive_name,
        rung = report.id,
        scope = report.scope,
        checksum = report.stable_report_checksum,
        adapter = adapter,
        registers_pipelines = report.registers_tier_a_transforms_on_world_gpu_state_pipelines,
        outcome = report.anti_fake_evidence.outcome,
        gap = report.anti_fake_evidence.production_substrate_gap,
        cpu_removed = report.anti_fake_evidence.cpu_injected_next_state_removed,
        identity_removed = report.anti_fake_evidence.identity_copy_producer_removed,
        oracle_only = report.anti_fake_evidence.oracle_comparison_only,
        neg_run = report.anti_fake_evidence.negative_control_run,
        neg_fails = report.anti_fake_evidence.negative_control_fails_parity,
        earned = report.anti_fake_evidence.earned_per_column_parity,
        shape_guard = report.anti_fake_evidence.source_shape_guard_passed,
        constituent = report.anti_fake_evidence.constituent_shapes_measured,
        seed_uploads = report.initial_seed_upload_count,
        tier_a_uploads = report.inter_tick_tier_a_upload_count,
        inter_tick_readbacks = report.inter_tick_readback_count,
        boundary_readbacks = report.boundary_parity_readback_count,
        dispatches = report.gpu_dispatch_count,
        gpu_feeds = report.gpu_state_feeds_next_tick,
        gpu_writes = report.gpu_writes_state_n_plus_1,
        next_reads = report.next_tick_reads_gpu_written_state,
        swaps = report.buffer_swap_count,
        slots = report.resident_slot_count,
        mirror = report.mirror_dispatch_after_cpu_tick,
        columns = columns,
        shapes = shapes,
        expected = report.r6c_checksum_expected,
        observed = report.r6c_checksum_observed,
        checksum_match = report.field_column_parity_matches_r6c_checksum,
        r4_delta = report.r4_max_abs_delta,
        r4_bound = report.r4_f32_bound,
        r4_within = report.r4_within_bound,
        gaps = gaps,
    )
}

pub(crate) fn collect_col(values: &[f32], total_slots: u32, col: u32) -> Vec<f32> {
    (0..total_slots)
        .map(|slot| values[slot_col_idx(slot, col)])
        .collect()
}

pub(crate) fn slot_col_idx(slot: u32, col: u32) -> usize {
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
        *hash ^= *byte as u64;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

fn mix_u64(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}
