use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1a, run_runtime_0080_0_r1a, run_runtime_0080_0_r1a_negative_control,
    Runtime0080R1aInput, Runtime0080R1aReport, RUNTIME_0080_0_R1A_ID, RUNTIME_0080_0_R1A_PRIMITIVE,
    RUNTIME_0080_0_R1A_STATUS_BLOCKED, RUNTIME_0080_0_R1A_STATUS_PASS,
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND,
    RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES,
    RUNTIME_R1A_SCOPE,
};

static REPORT: OnceLock<Runtime0080R1aReport> = OnceLock::new();

fn report() -> &'static Runtime0080R1aReport {
    REPORT.get_or_init(|| run_runtime_0080_0_r1a(&Runtime0080R1aInput::explicit_opt_in()))
}

fn blocked(report: &Runtime0080R1aReport) -> bool {
    report.verdict == "BLOCKED"
}

fn column<'a>(
    report: &'a Runtime0080R1aReport,
    name: &str,
) -> &'a simthing_driver::Runtime0080R1aCoveredColumnReport {
    report
        .covered_columns
        .iter()
        .find(|column| column.column == name)
        .unwrap_or_else(|| panic!("missing covered column {name}"))
}

#[test]
fn r1a_opt_in_default_off() {
    let default = run_runtime_0080_0_r1a(&Runtime0080R1aInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.trace.len(), 0);
    assert_eq!(default.id, RUNTIME_0080_0_R1A_ID);
    assert!(!default.default_simsession_wiring);
}

#[test]
fn r1a_selects_discrete_gpu_or_blocks_honestly() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1A_STATUS_BLOCKED);
        assert!(!admitted.diagnostics.is_empty());
        assert!(admitted.adapter.is_none());
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1A_STATUS_PASS);
    assert_eq!(admitted.verdict, "PASS");
    let adapter = admitted.adapter.as_ref().expect("R1a adapter");
    assert!(adapter.selected_discrete_gpu);
}

#[test]
fn r1a_registers_tier_a_transforms_on_world_gpu_state_pipelines() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.registers_tier_a_transforms_on_world_gpu_state_pipelines);
    assert!(RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES);
}

#[test]
fn r1a_gpu_transform_is_sole_producer_of_state_n_plus_1() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.anti_fake_evidence.cpu_injected_next_state_removed);
    assert!(admitted.anti_fake_evidence.identity_copy_producer_removed);
    assert!(admitted.gpu_writes_state_n_plus_1);
    assert!(admitted.next_tick_reads_gpu_written_state);
    assert!(admitted.gpu_state_feeds_next_tick);
    assert!(admitted.registers_tier_a_transforms_on_world_gpu_state_pipelines);
}

#[test]
fn r1a_negative_control_disabling_gpu_transform_fails_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(run_runtime_0080_0_r1a_negative_control());
    assert!(admitted.anti_fake_evidence.negative_control_fails_parity);
}

#[test]
fn r1a_inter_tick_tier_a_uploads_zero_by_measurement() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.inter_tick_tier_a_upload_count, 0);
    assert_eq!(
        admitted.measured_counters.inter_tick_tier_a_upload_count,
        admitted.inter_tick_tier_a_upload_count
    );
    assert_eq!(admitted.inter_tick_readback_count, 0);
    assert_eq!(admitted.boundary_parity_readback_count, 100);
    assert!(
        admitted
            .anti_fake_evidence
            .measured_counters_from_call_sites
    );
}

#[test]
fn r1a_no_oracle_value_written_to_resident_buffer_after_seed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.measured_counters.oracle_values_written_after_seed,
        0
    );
    assert_eq!(
        admitted
            .measured_counters
            .tier_a_next_state_cpu_write_call_sites,
        0
    );
    assert!(admitted.anti_fake_evidence.oracle_comparison_only);
}

#[test]
fn r1a_covered_column_parity_is_measured_not_constructed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    for column in &admitted.covered_columns {
        assert!(column.gpu_authoritative);
        assert!(column.cpu_oracle_parity);
        assert!(column.parity_measured_from_gpu_value);
        assert!(column.writes_state_n_plus_1);
        assert!(column.reads_prior_gpu_output);
    }
}

#[test]
fn r1a_tier_a_transform_uses_measured_shapes_not_identity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.anti_fake_evidence.constituent_shapes_measured);
    assert!(admitted.anti_fake_evidence.source_shape_guard_passed);
    for shape in [
        "R1 disruption input + bounded recurrence",
        "R2 owner reduce-up + disburse-down",
        "R4 GradientXY + Candidate-F magnitude",
        "R6 combat damage reduce + attrition emission",
        "R6B construction threshold + fusion sum",
    ] {
        assert!(
            admitted.measured_shape_names.contains(&shape),
            "missing measured constituent shape {shape}"
        );
    }
    assert!(admitted
        .covered_columns
        .iter()
        .all(|column| column.measured_shape != "Identity"));
}

#[test]
fn r1a_gpu_state_feeds_next_tick_true() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_state_feeds_next_tick);
    assert_eq!(admitted.verdict, "PASS");
    assert_eq!(admitted.trace.len(), 100);
}

#[test]
fn r1a_field_column_parity_matches_r6c_checksum() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(
        admitted.r6c_checksum_observed,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert!(admitted.field_column_parity_matches_r6c_checksum);
    assert!(admitted.anti_fake_evidence.earned_per_column_parity);
}

#[test]
fn r1a_r4_f32_within_accepted_bound() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let r4 = column(admitted, "r4_magnitude_scratch");
    assert!(r4.gpu_authoritative);
    assert!(!r4.integer_bit_exact);
    assert!(admitted.r4_within_bound);
    assert!(admitted.r4_max_abs_delta <= RUNTIME_R0_R4_F32_BOUND);
}

#[test]
fn r1a_tier_b_structural_ops_boundary_maintained_via_threshold_event_not_planner() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.boundary_summary.gpu_written_event_journal_rows, 0);
    assert!(admitted.boundary_summary.cpu_boundary_maintenance_rows > 0);
    assert!(admitted.boundary_summary.cpu_boundary_pass_bounded);
    assert!(!admitted.boundary_summary.cpu_boundary_pass_is_planner);
    assert!(
        admitted
            .boundary_summary
            .resident_event_journal_r1b_remaining
    );
    assert!(admitted.boundary_summary.resident_reenroll_r1c_remaining);
}

#[test]
fn r1a_no_semantic_wgsl_or_opcode_no_atlas_batching_no_m4a() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.no_new_semantic_wgsl);
    assert!(admitted.no_new_accumulator_op);
    assert!(!admitted.request_atlas_batching);
    assert!(!admitted.m4a_masking_at_scale);
    assert!(!admitted.scenario_reopened);
    assert!(!admitted.invariant_edited);
    assert!(!admitted.pinned_number_changed);
    assert!(!admitted.default_simsession_wiring);
    assert_eq!(admitted.scope, RUNTIME_R1A_SCOPE);
}

#[test]
fn r1a_any_new_substrate_primitive_passes_4a_gate() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.anti_fake_evidence.section_4a_gate_available);
    assert!(admitted.anti_fake_evidence.new_substrate_primitive_added);
    assert_eq!(admitted.substrate_primitives.len(), 2);
    for primitive in &admitted.substrate_primitives {
        assert!(primitive.section_4a_required);
        assert!(primitive.semantic_free_identifier);
        assert!(primitive.reusable_by_any_simthing);
        assert!(primitive.cpu_oracle_parity_test_passed);
        assert!(primitive.opt_in_default_off);
    }
}

#[test]
fn r1a_source_shape_guard_blocks_old_inject_and_copy_pattern() {
    let source = include_str!("../src/runtime_0080_0_r1a.rs");
    for forbidden in [
        "COL_JOURNAL_DELTA",
        "write_slot_col_values",
        "build_tier_a_oracle_states",
        "resident_double_buffer_ops",
        "copy current to next",
        "apply journal delta",
    ] {
        assert!(
            !source.contains(forbidden),
            "R1a source still contains old fake producer token {forbidden}"
        );
    }
}

#[test]
fn r1a_report_checksum_stable() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let (left, right) = replay_runtime_0080_0_r1a();
    assert_eq!(left.verdict, "PASS");
    assert_eq!(right.verdict, "PASS");
    assert_eq!(left.stable_report_checksum, right.stable_report_checksum);
    assert_eq!(
        left.stable_report_checksum,
        RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM
    );
    assert_eq!(admitted.primitive_name, RUNTIME_0080_0_R1A_PRIMITIVE);
    assert_eq!(
        admitted.foreground_capture_method,
        RUNTIME_R0_FOREGROUND_CAPTURE
    );
}
