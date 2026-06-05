use std::sync::OnceLock;

use simthing_driver::{
    replay_runtime_0080_0_r1a, run_runtime_0080_0_r1a, Runtime0080R1aInput, Runtime0080R1aReport,
    R6C_CANONICAL_TICK_COUNT, RUNTIME_0080_0_R1A_ID, RUNTIME_0080_0_R1A_PRIMITIVE,
    RUNTIME_0080_0_R1A_STATUS_BLOCKED, RUNTIME_0080_0_R1A_STATUS_PASS,
    RUNTIME_R0_EXPECTED_R6C_CHECKSUM, RUNTIME_R0_FOREGROUND_CAPTURE, RUNTIME_R0_R4_F32_BOUND,
    RUNTIME_R1A_EXPECTED_REPORT_CHECKSUM, RUNTIME_R1A_SCOPE,
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
fn runtime_0080_r1a_opt_in_default_off() {
    let default = run_runtime_0080_0_r1a(&Runtime0080R1aInput::default_simsession());
    assert!(!default.explicit_opt_in);
    assert!(default.default_off);
    assert!(default.disabled_no_op);
    assert_eq!(default.ticks_or_zero(), 0);
    assert_eq!(default.id, RUNTIME_0080_0_R1A_ID);
}

#[test]
fn runtime_0080_r1a_selects_discrete_gpu_or_blocks_honestly() {
    let admitted = report();
    if blocked(admitted) {
        assert_eq!(admitted.status, RUNTIME_0080_0_R1A_STATUS_BLOCKED);
        assert!(!admitted.diagnostics.is_empty());
        assert!(admitted.adapter.is_none());
        return;
    }
    assert_eq!(admitted.status, RUNTIME_0080_0_R1A_STATUS_PASS);
    let adapter = admitted.adapter.as_ref().expect("R1a adapter");
    assert!(adapter.selected_discrete_gpu);
    assert!(adapter.adapter_name.to_ascii_lowercase().contains("nvidia"));
}

#[test]
fn runtime_0080_r1a_seeds_tier_a_once() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.initial_seed_upload_count, 1);
    assert_eq!(admitted.inter_tick_tier_a_upload_count, 0);
    assert!(admitted.tier_a_current_next_buffers_exist);
}

#[test]
fn runtime_0080_r1a_uses_double_buffered_resident_state() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.buffer_swap_count, R6C_CANONICAL_TICK_COUNT);
    assert!(admitted.resident_slot_count > 0);
    assert!(admitted.trace.iter().all(
        |row| row.boundary_swap && row.current_hash_after_swap == row.next_hash_after_gpu_write
    ));
}

#[test]
fn runtime_0080_r1a_gpu_writes_state_n_plus_1() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_writes_state_n_plus_1);
    assert!(admitted.gpu_dispatch_count >= R6C_CANONICAL_TICK_COUNT * 3);
    assert!(admitted
        .trace
        .iter()
        .all(|row| row.gpu_wrote_state_n_plus_1));
}

#[test]
fn runtime_0080_r1a_next_tick_reads_gpu_written_state() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.next_tick_reads_gpu_written_state);
    for pair in admitted.trace.windows(2) {
        assert_eq!(
            pair[1].current_hash_before_tick,
            pair[0].current_hash_after_swap
        );
    }
}

#[test]
fn runtime_0080_r1a_gpu_state_feeds_next_tick_true() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.gpu_state_feeds_next_tick);
    assert!(admitted
        .trace
        .iter()
        .all(|row| row.previous_output_read_by_next_tick));
}

#[test]
fn runtime_0080_r1a_no_mirror_dispatch_after_cpu_tick_for_tier_a() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.mirror_dispatch_after_cpu_tick);
}

#[test]
fn runtime_0080_r1a_no_inter_tick_tier_a_cpu_uploads() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert_eq!(admitted.inter_tick_tier_a_upload_count, 0);
    assert!(admitted
        .trace
        .iter()
        .all(|row| row.cpu_tier_a_uploads_this_tick == 0));
}

#[test]
fn runtime_0080_r1a_cpu_shadow_is_boundary_parity_witness_only() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.cpu_shadow_boundary_witness_only);
    assert_eq!(admitted.inter_tick_readback_count, 0);
    assert_eq!(
        admitted.boundary_parity_readback_count,
        R6C_CANONICAL_TICK_COUNT
    );
}

#[test]
fn runtime_0080_r1a_disruption_matches_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "disruption");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_location_status_matches_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "location_status");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_stockpiles_match_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "stockpiles");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_construction_progress_matches_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "construction_progress");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_existing_slot_num_ships_matches_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "existing_slot_num_ships");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_blockade_divert_code_matches_cpu_oracle() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "blockade_divert_code");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(column.integer_bit_exact);
}

#[test]
fn runtime_0080_r1a_r4_f32_within_accepted_bound() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    let column = column(admitted, "r4_magnitude_scratch");
    assert!(column.gpu_authoritative);
    assert!(column.cpu_oracle_parity);
    assert!(!column.integer_bit_exact);
    assert!(admitted.r4_within_bound);
    assert!(admitted.r4_max_abs_delta <= RUNTIME_R0_R4_F32_BOUND);
}

#[test]
fn runtime_0080_r1a_tier_b_structural_ops_boundary_maintained_not_planned() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.boundary_summary.gpu_written_event_journal_rows > 0);
    assert_eq!(
        admitted.boundary_summary.gpu_written_event_journal_rows,
        admitted.boundary_summary.cpu_boundary_maintenance_rows
    );
    assert!(admitted.boundary_summary.cpu_boundary_pass_bounded);
    assert!(!admitted.boundary_summary.cpu_boundary_pass_is_planner);
}

#[test]
fn runtime_0080_r1a_does_not_create_remove_or_compact_cohort_slots() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(
        !admitted
            .boundary_summary
            .created_removed_or_compacted_by_r1a
    );
    assert!(admitted.boundary_summary.resident_reenroll_r1c_remaining);
}

#[test]
fn runtime_0080_r1a_no_new_semantic_wgsl_or_new_op() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(admitted.no_new_semantic_wgsl);
    assert!(admitted.no_new_accumulator_op);
}

#[test]
fn runtime_0080_r1a_no_atlas_batching_or_m4a() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.request_atlas_batching);
    assert!(!admitted.m4a_masking_at_scale);
    assert_eq!(admitted.scope, RUNTIME_R1A_SCOPE);
}

#[test]
fn runtime_0080_r1a_no_scenario_reopen_or_pinned_number_change() {
    let admitted = report();
    if blocked(admitted) {
        return;
    }
    assert!(!admitted.scenario_reopened);
    assert!(!admitted.invariant_edited);
    assert!(!admitted.pinned_number_changed);
    assert!(!admitted.default_simsession_wiring);
    assert_eq!(
        admitted.r6c_checksum_expected,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
    assert_eq!(
        admitted.r6c_checksum_observed,
        RUNTIME_R0_EXPECTED_R6C_CHECKSUM
    );
}

#[test]
fn runtime_0080_r1a_report_checksum_stable() {
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

trait Runtime0080R1aTestExt {
    fn ticks_or_zero(&self) -> u32;
}

impl Runtime0080R1aTestExt for Runtime0080R1aReport {
    fn ticks_or_zero(&self) -> u32 {
        self.trace.len() as u32
    }
}
