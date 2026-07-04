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
fn multitarget_gpu_compact_matches_cpu_small() {
    let scenario = make_multitarget_scenario("multitarget_small", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_replay_rich_with_harness(&harness, &scenario).unwrap();
    assert_report_ok(&report);
}

#[test]
fn multitarget_gpu_resident_summary_matches_cpu_small() {
    let scenario = make_multitarget_scenario("multitarget_resident_small", 1_024, 0.7, false);
    let harness = MultiTargetReplayHarness::new().unwrap();
    let report = compare_multitarget_resident_rich_with_harness(&harness, &scenario, 16).unwrap();
    assert_resident_report_ok(&report);
}
