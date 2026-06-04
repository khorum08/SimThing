use simthing_driver::{
    cpu_mag2_sum, cpu_oracle_dress_rehearsal_r4_sead_field_consumption,
    exact_mag2_bits_from_fixed, render_dress_rehearsal_r4_artifact,
    replay_dress_rehearsal_r4_sead_field_consumption, run_dress_rehearsal_r4_sead_field_consumption,
    sqrt_cr_f_bits, DressRehearsalR4Decision,
    DressRehearsalR4Input, DressRehearsalR4Owner, DressRehearsalR4Report,
    DRESS_REHEARSAL_R4_SEAD_FIELD_CONSUMPTION_ID, DRESS_REHEARSAL_R4_SEAD_FIELD_CONSUMPTION_STATUS_PASS,
    DISRUPTION_DECAY_MODIFIER, PIRATE_EMISSION_MODIFIER, RAIDING_LOGISTICS_MODIFIER,
};
use simthing_spec::SQRT_F_ARTIFACT_HASH;

fn report() -> DressRehearsalR4Report {
    run_dress_rehearsal_r4_sead_field_consumption(&DressRehearsalR4Input::explicit_opt_in())
}

#[test]
fn r4_canonical_checksum_pin() {
    let admitted = report();
    assert_eq!(admitted.summary.stable_checksum, 0xf0ac_be2c_cb98_badb);
    assert!(admitted.summary.step_opportunity_count > 0);
    assert!(admitted.summary.sit_still_count + admitted.summary.step_opportunity_count == 2);
}

#[test]
fn r4_opening_status_matches_track() {
    let admitted = report();
    assert!(admitted.gradientxy_consumed);
    let track = include_str!("../../../docs/design_0_0_8_0_consumer_pulled_production_track.md");
    assert!(track.contains("R4 — SEAD field-consumption + exact sqrt (EC2)"));
    let _artifact = render_dress_rehearsal_r4_artifact(&admitted);
}

#[test]
fn r4_consumes_r1_r2_r3_contracts() {
    let admitted = report();
    assert_eq!(admitted.id, DRESS_REHEARSAL_R4_SEAD_FIELD_CONSUMPTION_ID);
    assert!(DRESS_REHEARSAL_R4_SEAD_FIELD_CONSUMPTION_STATUS_PASS.contains("IMPLEMENTED / PASS"));
    assert!(admitted.admitted, "{:?}", admitted.diagnostics);
    assert!(admitted.r1_contract_consumed);
    assert_eq!(admitted.r1_contract_checksum, 0x17de_0080_304b_3da7);
    assert!(admitted.r1_cpu_oracle_parity);
    assert!(admitted.r2_contract_consumed);
    assert_eq!(admitted.r2_contract_checksum, 0x4fe0_5905_89dd_d975);
    assert!(admitted.r2_cpu_oracle_parity);
    assert!(admitted.r3_contract_consumed);
    assert_eq!(admitted.r3_contract_checksum, 0x28af_b4a2_04d1_01d2);
    assert!(admitted.r3_cpu_oracle_parity);
    assert!(admitted.store_owner_layout_consumed);
    assert!(admitted.single_galactic_tier);
    assert_eq!(admitted.exact_sqrt_artifact_hash, SQRT_F_ARTIFACT_HASH);
}

#[test]
fn r4_mover_reads_parent_grid_field_at_own_cell() {
    let admitted = report();
    assert_eq!(admitted.mover_rows.len(), 2);
    for row in &admitted.mover_rows {
        assert!(!row.mover_id.is_empty());
        assert!(row.cell_index < 400);
        assert_eq!(row.x, row.cell_index % 20);
        assert_eq!(row.y, row.cell_index / 20);
        assert!(row.disruption_at_cell >= 0.0);
    }
}

#[test]
fn r4_pirate_disposition_weights_low_patrol_and_opportunity() {
    let admitted = report();
    let pirate = admitted
        .mover_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR4Owner::Pirate)
        .expect("pirate mover");
    assert!(pirate
        .disposition_weights
        .iter()
        .any(|(id, _)| *id == PIRATE_EMISSION_MODIFIER));
    assert!(pirate
        .disposition_weights
        .iter()
        .any(|(id, _)| *id == PIRATE_EMISSION_MODIFIER));
    assert!(pirate
        .disposition_weights
        .iter()
        .any(|(id, _)| *id == RAIDING_LOGISTICS_MODIFIER));
    let hotspot = pirate.disruption_at_cell;
    assert!(hotspot <= 100.0);
}

#[test]
fn r4_patrol_disposition_weights_high_disruption() {
    let admitted = report();
    let patrol = admitted
        .mover_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR4Owner::Terran)
        .expect("patrol mover");
    assert!(patrol
        .disposition_weights
        .iter()
        .any(|(id, bps)| *id == DISRUPTION_DECAY_MODIFIER && *bps >= 10_000));
    assert!(patrol.disruption_at_cell >= 0.0);
}

#[test]
fn r4_composite_field_uses_disruption_economy_and_masked_disposition() {
    let admitted = report();
    assert!(!admitted.composite_field_rows.is_empty());
    let sample = admitted
        .composite_field_rows
        .iter()
        .find(|row| row.economy_signal > 0.0 || row.disruption > 0.0)
        .expect("composite sample");
    assert!(sample.disposition_weight_bps >= 5_000);
    assert!(sample.composite_opportunity.is_finite());
}

#[test]
fn r4_gradientxy_computed_from_composite_field() {
    let admitted = report();
    assert!(admitted.gradientxy_consumed);
    let pirate = admitted
        .mover_rows
        .iter()
        .find(|row| row.owner == DressRehearsalR4Owner::Pirate)
        .expect("pirate mover");
    assert!(pirate.gradient_dx_f32.is_finite());
    assert!(pirate.gradient_dy_f32.is_finite());
}

#[test]
fn r4_exact_mag2_computed_from_fixed_dxdy() {
    let admitted = report();
    let row = admitted.exact_magnitude_rows.first().expect("exact row");
    let expected_mag2 = cpu_mag2_sum(row.dx_fixed, row.dy_fixed);
    assert_eq!(row.exact_mag2_u64, expected_mag2);
    assert_eq!(
        row.exact_mag2_bits,
        exact_mag2_bits_from_fixed(row.dx_fixed, row.dy_fixed)
    );
}

#[test]
fn r4_candidate_f_exact_sqrt_gates_commitment() {
    let admitted = report();
    for row in &admitted.exact_magnitude_rows {
        assert_eq!(
            row.candidate_f_exact_mag_bits,
            sqrt_cr_f_bits(row.exact_mag2_bits)
        );
        assert!(row.commitment_uses_candidate_f);
        let mover = admitted
            .mover_rows
            .iter()
            .find(|m| m.mover_id == row.mover_id)
            .expect("mover for exact row");
        assert_eq!(
            mover.candidate_f_exact_mag_bits,
            row.candidate_f_exact_mag_bits
        );
        let expected_pass = if mover.movement_threshold_mag_bits == 0 {
            mover.candidate_f_exact_mag_bits > 0
        } else {
            mover.candidate_f_exact_mag_bits >= mover.movement_threshold_mag_bits
        };
        assert_eq!(mover.threshold_passed, expected_pass);
    }
}

#[test]
fn r4_raw_f32_magnitude_is_diagnostic_not_authority() {
    let admitted = report();
    for mover in &admitted.mover_rows {
        let exact_pass = if mover.movement_threshold_mag_bits == 0 {
            mover.candidate_f_exact_mag_bits > 0
        } else {
            mover.candidate_f_exact_mag_bits >= mover.movement_threshold_mag_bits
        };
        let diag_pass = if mover.movement_threshold_mag_bits == 0 {
            mover.approximate_diagnostic_mag_bits > 0
        } else {
            mover.approximate_diagnostic_mag_bits >= mover.movement_threshold_mag_bits
        };
        assert_eq!(mover.threshold_passed, exact_pass);
        let decision_is_step = mover.decision == DressRehearsalR4Decision::StepOpportunity;
        assert_eq!(decision_is_step, exact_pass);
        if diag_pass != exact_pass {
            assert_ne!(decision_is_step, diag_pass);
        }
    }
}

#[test]
fn r4_threshold_false_yields_sit_still() {
    let mut input = DressRehearsalR4Input::explicit_opt_in();
    input.movement_threshold_mag_bits = 0x7f7f_ffff; // very high threshold
    let admitted = run_dress_rehearsal_r4_sead_field_consumption(&input);
    assert!(admitted.admitted);
    assert!(admitted
        .mover_rows
        .iter()
        .all(|row| row.decision == DressRehearsalR4Decision::SitStill));
    assert_eq!(admitted.summary.sit_still_count, admitted.summary.mover_count);
}

#[test]
fn r4_threshold_true_yields_step_opportunity() {
    let mut input = DressRehearsalR4Input::explicit_opt_in();
    input.movement_threshold_mag_bits = 0; // any positive exact mag passes
    let admitted = run_dress_rehearsal_r4_sead_field_consumption(&input);
    assert!(admitted.admitted);
    assert!(
        admitted
            .mover_rows
            .iter()
            .any(|row| row.decision == DressRehearsalR4Decision::StepOpportunity),
        "expected at least one step-opportunity mover with zero threshold"
    );
}

#[test]
fn r4_step_opportunity_does_not_move_or_emit_boundary_request() {
    let admitted = report();
    assert!(!admitted.boundary_request_emitted);
    assert!(!admitted.movement_applied);
    assert!(!admitted.reenroll_emitted);
    assert_eq!(admitted.occupant_positions_before, admitted.occupant_positions_after);
    for row in &admitted.mover_rows {
        assert!(!row.movement_applied);
        if row.decision == DressRehearsalR4Decision::StepOpportunity {
            assert!(row.threshold_passed);
        }
    }
}

#[test]
fn r4_deterministic_replay_and_cpu_oracle_parity() {
    let (left, right) = replay_dress_rehearsal_r4_sead_field_consumption();
    assert!(left.admitted && right.admitted);
    assert_eq!(
        left.summary.stable_checksum,
        right.summary.stable_checksum
    );
    assert_eq!(left.mover_rows, right.mover_rows);
    let input = DressRehearsalR4Input::explicit_opt_in();
    let admitted = run_dress_rehearsal_r4_sead_field_consumption(&input);
    let oracle = cpu_oracle_dress_rehearsal_r4_sead_field_consumption(&input);
    assert!(admitted.cpu_oracle_parity);
    assert_eq!(admitted.mover_rows, oracle.mover_rows);
    assert_eq!(admitted.summary, oracle.summary);
}

#[test]
fn r4_opt_in_default_off() {
    let disabled = run_dress_rehearsal_r4_sead_field_consumption(&DressRehearsalR4Input::default_simsession());
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert_eq!(disabled.summary.mover_count, 0);

    let mut default_on = DressRehearsalR4Input::explicit_opt_in();
    default_on.enabled_by_default = true;
    let rejected = run_dress_rehearsal_r4_sead_field_consumption(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"r4_default_on_rejected"));
}
