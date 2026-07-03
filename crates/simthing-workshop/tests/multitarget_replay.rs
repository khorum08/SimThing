use simthing_workshop::multitarget_replay::{
    compare_multitarget_replay_rich_with_harness, compare_multitarget_resident_rich_with_harness,
    conservation_check, format_multitarget_replay_report, format_multitarget_resident_report,
    make_depletion_n_tick_scenario, make_manual_edge_case_scenario, make_multitarget_scenario,
    replay_from_compact_records, replay_from_compact_records_n_ticks, resolve_cpu_current,
    resolve_cpu_current_n_ticks, write_multitarget_replay_reports,
    write_multitarget_replay_reports_bundle, write_multitarget_resident_reports,
    write_multitarget_resident_reports_bundle, MultiTargetReplayHarness, REPLAY_MODE_COMPACT,
    REPLAY_MODE_FULL, RESIDENT_TICKS,
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

fn assert_resident_report_ok(
    report: &simthing_workshop::multitarget_replay::MultiTargetResidentReport,
) {
    eprintln!("{}", format_multitarget_resident_report(report));
    assert_eq!(report.conservation_gate, "PASS", "{:?}", report);
    assert_eq!(report.replay_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.final_state_matches_cpu_summary_mode, "{:?}", report);
    assert!(report.final_state_matches_cpu_records_mode, "{:?}", report);
    assert!(report.summary_matches_cpu, "{:?}", report);
    assert!(report.replay_from_records_matches_gpu, "{:?}", report);
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
    let (gpu_compact_final, gpu_compact, _) =
        harness.run_gpu(&scenario, REPLAY_MODE_COMPACT).unwrap();
    let (_, _, gpu_full) = harness.run_gpu(&scenario, REPLAY_MODE_FULL).unwrap();

    assert_eq!(gpu_compact_final, cpu_final);
    assert_eq!(gpu_compact, cpu_compact);
    assert_eq!(gpu_full.len(), scenario.states.len());
    for (full, compact) in gpu_full.iter().zip(gpu_compact.iter()) {
        assert_eq!(full.transfer_amount, compact.transfer_amount);
        assert_eq!(full.emit_count, compact.emit_count);
        assert_eq!(full.is_active, compact.is_active);
        assert_eq!(
            full.source_after,
            gpu_compact_final[full.item as usize].source_pool
        );
        assert_eq!(
            full.queue_after,
            gpu_compact_final[full.item as usize].queue_accum
        );
        assert_eq!(
            full.units_after,
            gpu_compact_final[full.item as usize].units
        );
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

#[test]
fn multitarget_gpu_resident_summary_matches_cpu_small() {
    let scenario = make_multitarget_scenario("multitarget_resident_small", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_resident_rich_with_harness(&harness, &scenario, 16).unwrap();
    assert_resident_report_ok(&report);
}

#[test]
fn multitarget_gpu_resident_records_replay_small() {
    let scenario = make_multitarget_scenario("multitarget_resident_records", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let (cpu_final, cpu_summaries, cpu_records) = resolve_cpu_current_n_ticks(&scenario, 16);
    let gpu = harness.run_gpu_resident(&scenario, 16, true).unwrap();
    assert_eq!(gpu.final_states, cpu_final);
    assert_eq!(gpu.summaries, cpu_summaries);
    let replayed = replay_from_compact_records_n_ticks(
        &scenario.states,
        &scenario.params,
        &gpu.compact_records,
        16,
        scenario.states.len(),
    )
    .unwrap();
    assert_eq!(replayed, cpu_final);
    assert_eq!(replayed, gpu.final_states);
    let _ = cpu_records;
}

#[test]
fn multitarget_gpu_resident_sparse_100k() {
    let scenario =
        make_multitarget_scenario("multitarget_resident_sparse_100k", 100_000, 0.01, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report =
        compare_multitarget_resident_rich_with_harness(&harness, &scenario, RESIDENT_TICKS)
            .unwrap();
    assert_resident_report_ok(&report);
}

#[test]
fn multitarget_gpu_resident_bursty_100k() {
    let scenario =
        make_multitarget_scenario("multitarget_resident_bursty_100k", 100_000, 1.0, true);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report =
        compare_multitarget_resident_rich_with_harness(&harness, &scenario, RESIDENT_TICKS)
            .unwrap();
    assert_resident_report_ok(&report);
    write_multitarget_resident_reports(&report).expect("write resident reports");
}

#[test]
#[ignore = "large GPU resident benchmark"]
fn multitarget_gpu_resident_bursty_1m() {
    let scenario =
        make_multitarget_scenario("multitarget_resident_bursty_1m", 1_000_000, 1.0, true);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report =
        compare_multitarget_resident_rich_with_harness(&harness, &scenario, RESIDENT_TICKS)
            .unwrap();
    assert_resident_report_ok(&report);
}

#[test]
fn multitarget_resident_report_bundle() {
    let harness = MultiTargetReplayHarness::new().unwrap();
    let scenarios = [
        make_multitarget_scenario("multitarget_resident_small", 1_024, 0.7, false),
        make_multitarget_scenario("multitarget_resident_sparse_100k", 100_000, 0.01, false),
        make_multitarget_scenario("multitarget_resident_bursty_100k", 100_000, 1.0, true),
    ];
    let mut reports = Vec::with_capacity(scenarios.len());
    for scenario in scenarios {
        let ticks = if scenario.states.len() >= 100_000 {
            RESIDENT_TICKS
        } else {
            16
        };
        let report =
            compare_multitarget_resident_rich_with_harness(&harness, &scenario, ticks).unwrap();
        assert_resident_report_ok(&report);
        reports.push(report);
    }
    write_multitarget_resident_reports_bundle(&reports)
        .expect("write multitarget resident reports bundle");
}
