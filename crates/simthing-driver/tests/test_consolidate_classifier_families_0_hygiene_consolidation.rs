// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "bh2d_ct4b_fixture",
        original_test: "bh2d_overlap_stress_available_as_field_policy_feedstock",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "bh2d_ct4b_fixture",
        original_test: "bh2d_stress_operators_are_overlap_and_mismatch_only",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e11_resource_flow_soak",
        original_test: "e11_soak_flag_default_false",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e11_resource_flow_soak",
        original_test: "e11_soak_flat_star_only_no_nested_claims",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e11_resource_flow_soak",
        original_test: "e11_soak_repeated_resync_100_cycles_stable",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e11_resource_flow_soak",
        original_test: "e11_soak_zero_weights_1000_ticks_no_nan",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_contiguity_blocked_no_compaction",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_flag_off_updates_registry_but_no_gpu_sync",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_multiple_fissions_100_ticks_stable",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_no_new_wgsl",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_no_simthing_sim_arena_imports",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_replay_same_seed_same_dynamic_enrollment_frames",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_resource_flow_flag_default_false",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "e2b5_dynamic_enrollment_soak",
        original_test: "e2b5_soak_two_arenas_dynamic_enrollment_100_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "mobility_runtime1b_gpu_passgraph_fixture",
        original_test: "runtime1b_34k_gpu_fixture_soak_or_precise_blocker",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "palma_path_4_benchmark",
        original_test: "palma_path_4_benchmark_full_matrix",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "palma_path_4_benchmark",
        original_test: "palma_path_4_benchmark_smoke_matrix",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "palma_path_4_benchmark",
        original_test: "palma_path_4_dijkstra_baseline_matches_min_plus_when_relaxed_enough",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "palma_path_4_benchmark",
        original_test: "palma_path_4s_scenario_has_100_stars_and_150_fleets",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "palma_path_4_benchmark",
        original_test: "palma_path_4s_stellaris_scale_benchmark",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_benchmark_report_smoke",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d3_nested_resource_flow",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d4_nested_resource_flow",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_generic_kernel_matches_existing_ao_for_b0_transfer_orderband_if_supported",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_generic_kernel_matches_existing_ao_for_flat_star",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_no_default_on_resource_flow_or_hard_currency_reroute",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_no_designer_authored_wgsl",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_no_l3_frontierv2_5_act_event_obs_pipe",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_no_simthing_sim_awareness",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_ao_wgsl0_accumulator_op_performance",
        original_test: "ao_wgsl0_replay_reproducibility",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act0_numeric_proposals",
        original_test: "field_policy_act0_perf_34k_numeric_proposals",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act0_numeric_proposals",
        original_test: "field_policy_act0_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act1_phase_e_proposal_consumer",
        original_test: "field_policy_act1_perf_34k_phase_e_consumer",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act1_phase_e_proposal_consumer",
        original_test: "field_policy_act1_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act3_economic_fixture_records",
        original_test: "field_policy_act3_perf_34k_fixture_records",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_act3_economic_fixture_records",
        original_test: "field_policy_act3_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event0_compaction",
        original_test: "field_policy_event0_perf_34k_compaction",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event0_compaction",
        original_test: "field_policy_event0_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event1_code_bucketing",
        original_test: "field_policy_event1_perf_34k_bucketing",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event1_code_bucketing",
        original_test: "field_policy_event1_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event2_bucket_reductions",
        original_test: "field_policy_event2_perf_34k_bucket_reductions",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_event2_bucket_reductions",
        original_test: "field_policy_event2_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs0_mobile_overlay_score",
        original_test: "field_policy_obs0_perf_34k_mobile_overlay_score",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs0_mobile_overlay_score",
        original_test: "field_policy_obs1_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs2_multilayer_overlay_score",
        original_test: "field_policy_obs2_perf_34k_multilayer_overlay_score",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs2_multilayer_overlay_score",
        original_test: "field_policy_obs2_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs3_fixed_point_score",
        original_test: "field_policy_obs3_perf_34k_fixed_score",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs3_fixed_point_score",
        original_test: "field_policy_obs3_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs4_threshold_event",
        original_test: "field_policy_obs4_perf_34k_threshold_event",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_obs4_threshold_event",
        original_test: "field_policy_obs4_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_pipe0_observer_event_pipeline",
        original_test: "field_policy_pipe0_perf_34k_capacity_variants",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_pipe0_observer_event_pipeline",
        original_test: "field_policy_pipe0_perf_34k_integrated_pipeline",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_field_policy_pipe0_observer_event_pipeline",
        original_test: "field_policy_pipe0_perf_34k_warm_repeated_dispatch",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_exact_candidate_battery",
        original_test: "sqrt_exact4f_perf_e3_vs_f_smoke",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag0_f_exact_magnitude",
        original_test: "sqrt_mag0_perf_34k_mobile_simthing_hot_path",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag0_f_exact_magnitude",
        original_test: "sqrt_mag0_perf_scaled_smoke",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_0_fixed_exact",
        original_test: "sqrt_mag2_0_perf_34k_fixed_mag2_plus_f_sqrt",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_candidate_a_q12_dense_corpus",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_candidate_c_split_kernels_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_correctness_edge_and_dense_q16",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_f_sqrt_only_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_fixed_mag2_only_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_fixed_mag2_plus_f_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_no_readback_dispatch_proxy_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_readback_baseline_34k",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "phase_m_jit_sqrt_mag2_perf0_fixed_hotpath",
        original_test: "sqrt_mag2_perf0_scaled_smoke",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_does_not_enable_transfer_or_emission",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_dynamic_policy_a_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_flat_star_only_no_nested_claims",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_global_flag_default_false",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_multi_arena_no_coupling_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_no_new_wgsl",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_no_simthing_sim_arena_imports",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_populated_spec_without_opt_in_inactive",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_replay_same_seed_same_summary",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_static_512_participants_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_static_skewed_weights_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_flat_star_continued_soak",
        original_test: "rf_flat_star_continued_telemetry_has_flag_source_and_profile",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_disabled_populated_spec_inactive_but_reported",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_does_not_enable_transfer_or_emission",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_flat_star_only_no_nested_claims",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_global_resource_flow_flag_default_false",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_no_new_wgsl",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_no_simthing_sim_arena_imports",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_dynamic_fission_cadence_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_multi_arena_no_coupling_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_multi_session_replay_same_seed",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_repeated_resync_stable",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_static_128_participants_1000_ticks",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "resource_flow_opt_in_product_soak",
        original_test: "rf_t3_product_static_256_participants_if_runtime_reasonable",
        classifier_input: "hygiene-theater soak classifier input",
    },
    HygieneTheaterCase {
        original_binary: "session_integration",
        original_test: "bench_stress_scenarios_within_ceiling",
        classifier_input: "hygiene-theater stress classifier input",
    },
];

#[test]
fn hygiene_theater_cases_table_preserves_inputs() {
    let mut seen = std::collections::BTreeSet::new();

    for case in HYGIENE_THEATER_CASES {
        assert!(seen.insert((case.original_binary, case.original_test)),
            "duplicate classifier input case: {}::{}", case.original_binary, case.original_test);
        assert!(!case.classifier_input.is_empty(),
            "classifier input must be non-empty for {}::{}", case.original_binary, case.original_test);
    }

    assert_eq!(HYGIENE_THEATER_CASES.len(), 91,
        "table row count must match consolidated classifier-input inventory");
}
