use simthing_workshop::multitarget_replay::{
    compare_multitarget_replay_rich_with_harness, conservation_check,
    format_multitarget_replay_report, make_manual_edge_case_scenario, make_multitarget_scenario,
    replay_from_compact_records, resolve_cpu_current, write_multitarget_replay_reports,
    write_multitarget_replay_reports_bundle, MultiTargetReplayHarness, REPLAY_MODE_COMPACT,
    REPLAY_MODE_FULL,
};

fn assert_report_ok(report: &simthing_workshop::multitarget_replay::MultiTargetReplayReport) {
    eprintln!("{}", format_multitarget_replay_report(report));
    assert_eq!(report.conservation_gate, "PASS", "{:?}", report);
    assert_eq!(report.replay_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.final_state_matches_cpu, "{:?}", report);
    assert!(report.replay_from_compact_matches_gpu, "{:?}", report);
    assert!(report.repeated_gpu_outputs_identical, "{:?}", report);
}

#[test]
fn multitarget_cpu_resolver_handles_edge_cases() {
    let scenario = make_manual_edge_case_scenario();
    let (outputs, compact, _) = resolve_cpu_current(&scenario, false);

    assert_eq!(outputs[0].source_pool, 0);
    assert_eq!(outputs[0].queue_accum, 0);
    assert_eq!(outputs[0].units, 1);
    assert_eq!(compact[0].transfer_amount, 0);

    assert_eq!(outputs[1].source_pool, 500);
    assert_eq!(compact[1].transfer_amount, 0);

    assert_eq!(outputs[2].queue_accum, 9);
    assert_eq!(outputs[2].units, 1);

    assert_eq!(outputs[3].units, 1);
    assert_eq!(outputs[3].queue_accum, 10);

    assert_eq!(outputs[4].units, 4);
    assert_eq!(outputs[4].queue_accum, 0);

    assert_eq!(outputs[5].units, 4);
    assert_eq!(outputs[6].units, 5);
    assert_eq!(outputs[7].units, 500);
    assert_eq!(outputs[8].source_pool, 0);
    assert_eq!(outputs[9], scenario.states[9]);
    assert_eq!(compact[9].transfer_amount, 0);
}

#[test]
fn multitarget_compact_replay_reconstructs_cpu() {
    let scenario = make_multitarget_scenario("multitarget_replay_small", 256, 0.5, false);
    let (cpu_final, compact, _) = resolve_cpu_current(&scenario, false);
    let replayed =
        replay_from_compact_records(&scenario.states, &scenario.params, &compact).unwrap();
    assert_eq!(replayed, cpu_final);
    assert!(conservation_check(
        &scenario.states,
        &cpu_final,
        &scenario.params,
        &compact
    ));
}

#[test]
fn multitarget_gpu_compact_matches_cpu_small() {
    let scenario = make_multitarget_scenario("multitarget_small", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn multitarget_gpu_full_records_match_compact() {
    let scenario = make_multitarget_scenario("multitarget_full_mode", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();

    let (cpu_final, cpu_compact, _) = resolve_cpu_current(&scenario, true);
    let (gpu_compact_final, gpu_compact, _) = harness.run_gpu(&scenario, REPLAY_MODE_COMPACT).unwrap();
    let (_, _, gpu_full) = harness.run_gpu(&scenario, REPLAY_MODE_FULL).unwrap();

    assert_eq!(gpu_compact_final, cpu_final);
    assert_eq!(gpu_compact, cpu_compact);
    assert_eq!(gpu_full.len(), scenario.states.len());
    for (full, compact) in gpu_full.iter().zip(gpu_compact.iter()) {
        assert_eq!(full.transfer_amount, compact.transfer_amount);
        assert_eq!(full.emit_count, compact.emit_count);
        assert_eq!(full.is_active, compact.is_active);
        assert_eq!(full.source_after, gpu_compact_final[full.item as usize].source_pool);
        assert_eq!(full.queue_after, gpu_compact_final[full.item as usize].queue_accum);
        assert_eq!(full.units_after, gpu_compact_final[full.item as usize].units);
    }
}

#[test]
fn multitarget_sparse_100k_load() {
    let scenario = make_multitarget_scenario("multitarget_sparse_100k", 100_000, 0.01, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn multitarget_bursty_100k_load() {
    let scenario = make_multitarget_scenario("multitarget_bursty_100k", 100_000, 1.0, true);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
    write_multitarget_replay_reports(&report).expect("write multitarget replay reports");
}

#[test]
fn multitarget_conservation_rejects_corrupted_record() {
    let scenario = make_multitarget_scenario("multitarget_corrupt", 64, 1.0, false);
    let (cpu_final, mut compact, _) = resolve_cpu_current(&scenario, false);
    assert!(conservation_check(
        &scenario.states,
        &cpu_final,
        &scenario.params,
        &compact
    ));

    compact[0].transfer_amount = compact[0].transfer_amount.saturating_add(1);
    assert!(!conservation_check(
        &scenario.states,
        &cpu_final,
        &scenario.params,
        &compact
    ));
    assert!(replay_from_compact_records(&scenario.states, &scenario.params, &compact).is_err()
        || !conservation_check(
            &scenario.states,
            &cpu_final,
            &scenario.params,
            &compact
        ));
}

#[test]
#[ignore = "large GPU load benchmark"]
fn multitarget_bursty_1m_load() {
    let scenario = make_multitarget_scenario("multitarget_bursty_1m", 1_000_000, 1.0, true);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn multitarget_replay_report_bundle() {
    let harness = MultiTargetReplayHarness::new().unwrap();
    let scenarios = [
        make_multitarget_scenario("multitarget_small", 1_024, 0.7, false),
        make_multitarget_scenario("multitarget_sparse_100k", 100_000, 0.01, false),
        make_multitarget_scenario("multitarget_bursty_100k", 100_000, 1.0, true),
    ];
    let mut reports = Vec::with_capacity(scenarios.len());
    for scenario in scenarios {
        let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
        assert_report_ok(&report);
        reports.push(report);
    }
    write_multitarget_replay_reports_bundle(&reports)
        .expect("write multitarget replay reports bundle");
}
