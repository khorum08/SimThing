use simthing_workshop::weighted_mean::{
    compare_weighted_mean_rich, compare_weighted_mean_rich_with_harness, format_report,
    make_weighted_mean_scenario, weighted_mean_cpu, WeightedChild, WeightedMeanGpuHarness,
    ParentRange, DEFAULT_TOLERANCE,
};

fn assert_report_passes(report: &simthing_workshop::weighted_mean::WeightedMeanReport) {
    eprintln!("{}", format_report(report));

    assert_eq!(report.correctness_gate, "PASS", "{:?}", report);
    assert_eq!(report.determinism_gate, "PASS", "{:?}", report);
    assert!(report.within_tolerance, "{:?}", report);
    assert!(report.repeated_runs_identical, "{:?}", report);
    assert!(report.max_abs_error <= DEFAULT_TOLERANCE, "{:?}", report);
    assert!(
        report.parity_classification == "BIT_EXACT" || report.parity_classification == "TOLERANCE_EXACT",
        "unexpected parity classification {:?}",
        report.parity_classification
    );
}

fn run_parity_test(harness: &WeightedMeanGpuHarness, name: &str, n_parents: usize, children_per_parent: usize) {
    let scenario = make_weighted_mean_scenario(name, n_parents, children_per_parent);
    let report = compare_weighted_mean_rich_with_harness(harness, &scenario).unwrap();
    assert_report_passes(&report);

    if n_parents == 100_000 {
        let report_dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/workshop");
        std::fs::create_dir_all(&report_dir).expect("create target/workshop");
        std::fs::write(
            report_dir.join("weighted_mean_parity_report_100k.md"),
            format_report(&report),
        )
        .expect("write 100k weighted mean report");
    }
}

#[test]
fn weighted_mean_cpu_oracle_handles_edge_cases() {
    let children = vec![
        WeightedChild { value: 42.0, weight: 2.0 },
        WeightedChild { value: 10.0, weight: 1.0 },
        WeightedChild { value: 20.0, weight: 3.0 },
        WeightedChild { value: 99.0, weight: 0.0 },
        WeightedChild { value: 99.0, weight: 0.0 },
    ];
    let ranges = vec![
        ParentRange { offset: 0, len: 0 },
        ParentRange { offset: 3, len: 2 },
        ParentRange { offset: 0, len: 1 },
        ParentRange { offset: 1, len: 2 },
    ];

    let outputs = weighted_mean_cpu(&children, &ranges);
    assert_eq!(outputs.len(), 4);
    assert_eq!(outputs[0].value, 0.0);
    assert_eq!(outputs[1].value, 0.0);
    assert_eq!(outputs[2].value, 42.0);
    assert!((outputs[3].value - 17.5).abs() <= 1e-6);
}

#[test]
fn weighted_mean_gpu_matches_cpu_small() {
    let harness = WeightedMeanGpuHarness::new().unwrap();
    run_parity_test(&harness, "weighted_mean_small_x8", 128, 8);
}

#[test]
fn weighted_mean_gpu_matches_cpu_10k_x32() {
    let harness = WeightedMeanGpuHarness::new().unwrap();
    run_parity_test(&harness, "weighted_mean_10k_x32", 10_000, 32);
}

#[test]
fn weighted_mean_gpu_matches_cpu_100k_x32() {
    let harness = WeightedMeanGpuHarness::new().unwrap();
    run_parity_test(&harness, "weighted_mean_100k_x32", 100_000, 32);
}

#[test]
fn weighted_mean_rejects_invalid_inputs() {
    let harness = WeightedMeanGpuHarness::new().unwrap();

    let valid = make_weighted_mean_scenario("valid", 4, 4);

    let mut nan_value = valid.clone();
    nan_value.children[0].value = f32::NAN;
    assert!(compare_weighted_mean_rich(&nan_value).is_err());
    assert!(harness.eval(&nan_value.children, &nan_value.ranges).is_err());

    let mut inf_value = valid.clone();
    inf_value.children[0].value = f32::INFINITY;
    assert!(compare_weighted_mean_rich(&inf_value).is_err());

    let mut nan_weight = valid.clone();
    nan_weight.children[0].weight = f32::NAN;
    assert!(compare_weighted_mean_rich(&nan_weight).is_err());

    let mut negative_weight = valid.clone();
    negative_weight.children[0].weight = -1.0;
    assert!(compare_weighted_mean_rich(&negative_weight).is_err());

    let mut bad_range = valid.clone();
    bad_range.ranges[0].len += 1000;
    assert!(compare_weighted_mean_rich(&bad_range).is_err());
    assert!(harness.eval(&bad_range.children, &bad_range.ranges).is_err());
}

#[test]
fn weighted_mean_report_is_deterministic_across_runs() {
    let scenario = make_weighted_mean_scenario("weighted_mean_determinism", 256, 8);
    let mut harness = WeightedMeanGpuHarness::new().unwrap();

    let r1 = compare_weighted_mean_rich_with_harness(&mut harness, &scenario).unwrap();
    let r2 = compare_weighted_mean_rich_with_harness(&mut harness, &scenario).unwrap();
    let r3 = compare_weighted_mean_rich_with_harness(&mut harness, &scenario).unwrap();

    for (a, b) in [(&r1, &r2), (&r2, &r3)] {
        assert_eq!(a.max_abs_error, b.max_abs_error);
        assert_eq!(a.mean_abs_error, b.mean_abs_error);
        assert_eq!(a.parity_classification, b.parity_classification);
        assert!(a.repeated_runs_identical);
        assert!(b.repeated_runs_identical);
    }
}
