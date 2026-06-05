use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1a, run_runtime_0080_0_r1a, run_runtime_0080_0_r1a_negative_control,
    Runtime0080R1aInput, Runtime0080R1aInputSource, Runtime0080R1aReport, RUNTIME_0080_0_R1A_ID,
    RUNTIME_0080_0_R1A_PRIMITIVE, RUNTIME_0080_0_R1A_STATUS_BLOCKED,
    RUNTIME_0080_0_R1A_STATUS_PASS, RUNTIME_R0_EXPECTED_R6C_CHECKSUM,
    RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND, RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM,
    RUNTIME_R1A_REGISTERS_WORLD_GPU_STATE_PIPELINES, RUNTIME_R1A_SCOPE,
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
        "TierAInputTables",
        "TierAInputTables::from_report",
    ] {
        assert!(
            !source.contains(forbidden),
            "R1a source still contains old fake producer token {forbidden}"
        );
    }
    let production_source = source
        .split("fn compute_comparison_oracle_trajectory")
        .next()
        .unwrap_or(source);
    for forbidden in [
        "disruption_source_rows",
        "stockpile_ledger_rows",
        "construction_rows",
        "combat_rows",
        "reinforcement_rows",
        "fusion_rows",
        "field_read_rows",
        "economy_rows",
    ] {
        assert!(
            !production_source.contains(forbidden),
            "R1a production path still contains oracle-fed replay token {forbidden}"
        );
    }
    assert!(
        source.contains("R1aBoundaryWitness"),
        "R1a must derive tick inputs via boundary witness"
    );
    assert!(
        source.contains("compute_comparison_oracle_trajectory"),
        "CPU oracle comparison path must remain for parity only"
    );
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

#[test]
fn r1a_no_cpu_r6c_per_tick_answer_tables_for_covered_columns() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let source = include_str!("../src/runtime_0080_0_r1a.rs");
    assert!(!source.contains("TierAInputTables"));
    assert!(!source.contains("from_report"));
}

#[test]
fn r1a_disruption_inputs_are_gpu_derived_or_declared_partial() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "disruption");
    assert_ne!(col.input_source, Runtime0080R1aInputSource::OracleFed);
    assert_eq!(
        col.input_source,
        Runtime0080R1aInputSource::BoundaryMaintained
    );
}

#[test]
fn r1a_stockpile_updates_are_gpu_derived_not_ledger_replayed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "stockpiles");
    assert_eq!(col.input_source, Runtime0080R1aInputSource::GpuDerived);
    assert_ne!(col.input_source, Runtime0080R1aInputSource::OracleFed);
}

#[test]
fn r1a_construction_progress_is_gpu_derived_not_report_replayed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "construction_progress");
    assert_eq!(col.input_source, Runtime0080R1aInputSource::GpuDerived);
}

#[test]
fn r1a_num_ships_existing_slot_updates_are_gpu_derived_not_report_replayed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "existing_slot_num_ships");
    assert_eq!(
        col.input_source,
        Runtime0080R1aInputSource::BoundaryMaintained
    );
    assert_ne!(col.input_source, Runtime0080R1aInputSource::OracleFed);
}

#[test]
fn r1a_blockade_code_is_gpu_derived_not_report_replayed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "blockade_divert_code");
    assert_eq!(col.input_source, Runtime0080R1aInputSource::GpuDerived);
}

#[test]
fn r1a_r4_magnitude_is_gpu_computed_not_report_replayed() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let col = column(admitted, "r4_magnitude_scratch");
    assert_eq!(col.input_source, Runtime0080R1aInputSource::GpuDerived);
}

#[test]
fn r1a_exact_columns_include_cpu_bits_and_gpu_bits() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.exact_bit_proofs.is_empty());
    for proof in &admitted.exact_bit_proofs {
        assert!(proof.cpu_oracle_bits > 0 || proof.gpu_readback_bits > 0 || proof.slot > 0);
    }
    for col in [
        "stockpiles",
        "construction_progress",
        "existing_slot_num_ships",
        "blockade_divert_code",
    ] {
        assert!(
            admitted
                .covered_columns
                .iter()
                .any(|c| c.column == col && c.sample_cpu_oracle_bits.is_some()),
            "missing sample cpu bits for {col}"
        );
        assert!(
            admitted
                .covered_columns
                .iter()
                .any(|c| c.column == col && c.sample_gpu_readback_bits.is_some()),
            "missing sample gpu bits for {col}"
        );
    }
}

#[test]
fn r1a_stockpile_bits_match_exactly() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted
        .exact_bit_proofs
        .iter()
        .filter(|p| p.column == "stockpiles")
        .all(|p| p.bit_exact));
}

#[test]
fn r1a_construction_progress_bits_match_exactly() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted
        .exact_bit_proofs
        .iter()
        .filter(|p| p.column == "construction_progress")
        .all(|p| p.bit_exact));
}

#[test]
fn r1a_existing_slot_num_ships_bits_match_exactly() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted
        .exact_bit_proofs
        .iter()
        .filter(|p| p.column == "existing_slot_num_ships")
        .all(|p| p.bit_exact));
}

#[test]
fn r1a_blockade_divert_bits_match_exactly() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted
        .exact_bit_proofs
        .iter()
        .filter(|p| p.column == "blockade_divert_code")
        .all(|p| p.bit_exact));
}

#[test]
fn r1a_disabled_transform_for_exact_column_fails_bit_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let disabled = admitted
        .disabled_transform_checks
        .iter()
        .find(|row| row.column == "stockpiles" && !row.transform_enabled)
        .expect("disabled stockpile row");
    assert!(!disabled.bit_exact);
}

#[test]
fn r1a_reenabled_transform_for_exact_column_restores_bit_parity() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let enabled = admitted
        .disabled_transform_checks
        .iter()
        .find(|row| row.column == "stockpiles" && row.transform_enabled)
        .expect("enabled stockpile row");
    assert!(enabled.bit_exact);
}

#[test]
fn r1a_report_classifies_any_oracle_fed_column_as_partial() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    if !admitted.oracle_fed_covered_columns.is_empty() {
        assert_ne!(admitted.verdict, "PASS");
    }
}

#[test]
fn r1a_pass_requires_no_oracle_fed_covered_columns() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.oracle_fed_covered_columns.is_empty());
    assert_eq!(admitted.verdict, "PASS");
}

#[test]
fn r1a_uses_current_domain_neutral_terms() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let artifact = &admitted.artifact_markdown;
    assert!(artifact.contains("GPU-STATE-AUTH-0") || artifact.contains("GPU-side"));
    assert!(source_contains_field_policy_module());
}

fn source_contains_field_policy_module() -> bool {
    let lib = include_str!("../src/lib.rs");
    lib.contains("dress_rehearsal_r4_field_policy_consumption")
}

#[test]
fn r1a_no_legacy_normalized_terms_reintroduced() {
    let source = include_str!("../src/runtime_0080_0_r1a.rs");
    let report_doc =
        include_str!("../../../docs/tests/runtime_0080_0_r1a_next_tick_authority_results.md");
    for legacy in [
        "SEAD",
        "self_ai",
        "self-AI",
        "weaponize",
        "weaponise",
        "exploitation",
    ] {
        assert!(
            !source.contains(legacy),
            "legacy term {legacy} in r1a source"
        );
        assert!(
            !report_doc.contains(legacy),
            "legacy term {legacy} in r1a report"
        );
    }
}
