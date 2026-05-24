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
    let scenario = make_weighted_mean_perf_scenario(
        "weighted_mean_perf_small",
        128,
        8,
        16,
        2,
        0.1,
    );
    let harness = WeightedMeanPerfHarness::new().unwrap();
    let report = compare_weighted_mean_perf_with_harness(&harness, &scenario).unwrap();
    assert_perf_correctness(&report);
}

#[test]
fn weighted_mean_perf_sparse_100k_x32_dims64_wm1() {
    let scenario = make_weighted_mean_perf_scenario(
        "weighted_mean_perf_sparse_100k",
        100_000,
        32,
        64,
        1,
        0.1,
    );
    let harness = WeightedMeanPerfHarness::new().unwrap();
    let report = compare_weighted_mean_perf_with_harness(&harness, &scenario).unwrap();
    assert_perf_correctness(&report);
    write_perf_reports(&report).expect("write perf reports");
}

#[test]
fn weighted_mean_perf_dense_10k_x32_dims16_wm16() {
    let scenario = make_weighted_mean_perf_scenario(
        "weighted_mean_perf_dense_10k",
        10_000,
        32,
        16,
        8,
        0.1,
    );
    let harness = WeightedMeanPerfHarness::new().unwrap();
    let report = compare_weighted_mean_perf_with_harness(&harness, &scenario).unwrap();
    eprintln!("{}", format_perf_report(&report));
    assert_perf_correctness(&report);
}

#[test]
fn weighted_mean_perf_overlay_density_sweep() {
    let harness = WeightedMeanPerfHarness::new().unwrap();
    for density in [0.0, 0.1, 1.0] {
        let scenario = make_weighted_mean_perf_scenario(
            &format!("weighted_mean_perf_overlay_{density}"),
            10_000,
            32,
            64,
            1,
            density,
        );
        let report = compare_weighted_mean_perf_with_harness(&harness, &scenario).unwrap();
        eprintln!(
            "overlay_density={density}: speedup={:.3}x current_us={} pivot_us={} interpretation={}",
            report.speedup_pivot_vs_current,
            report.current_warm_mean_us,
            report.pivot_warm_mean_us,
            report.interpretation,
        );
        assert_perf_correctness(&report);
    }
}
