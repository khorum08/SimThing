use simthing_workshop::weighted_mean_perf::{
    compare_weighted_mean_perf_with_harness, format_perf_report, make_weighted_mean_perf_scenario,
    write_perf_reports, WeightedMeanPerfHarness,
};

fn assert_perf_correctness(report: &simthing_workshop::weighted_mean_perf::WeightedMeanPerfReport) {
    eprintln!("{}", format_perf_report(report));

    assert_ne!(report.current_parity_classification, "FAIL", "{:?}", report);
    assert_ne!(report.pivot_parity_classification, "FAIL", "{:?}", report);
    assert!(report.current_deterministic, "{:?}", report);
    assert!(report.pivot_deterministic, "{:?}", report);
}

#[test]
fn weighted_mean_perf_current_and_pivot_match_cpu_small() {
    let scenario = make_weighted_mean_perf_scenario("weighted_mean_perf_small", 128, 8, 16, 2, 0.1);
    let harness = WeightedMeanPerfHarness::new().unwrap();
    let report = compare_weighted_mean_perf_with_harness(&harness, &scenario).unwrap();
    assert_perf_correctness(&report);
}

