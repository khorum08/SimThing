use simthing_workshop::weighted_mean::{
    compare_weighted_mean_rich, compare_weighted_mean_rich_with_harness, format_report,
    make_weighted_mean_scenario, production_shape_fixture, validate_scenario, weighted_mean_cpu,
    ParentRange, WeightedChild, WeightedMeanGpuHarness, WeightedMeanScenario, LOOSE_TOLERANCE,
};

fn assert_generated_scenario_report(report: &simthing_workshop::weighted_mean::WeightedMeanReport) {
    eprintln!("{}", format_report(report));

    assert!(report.within_loose_tolerance, "{:?}", report);
    assert_ne!(report.parity_classification, "FAIL", "{:?}", report);
    assert!(report.repeated_runs_identical, "{:?}", report);
    assert!(report.max_abs_error <= LOOSE_TOLERANCE, "{:?}", report);

    assert!(report.non_empty_zero_weight_ranges > 0, "{:?}", report);
    assert!(report.empty_ranges > 0, "{:?}", report);
    assert!(report.single_child_ranges > 0, "{:?}", report);
    assert!(report.negative_value_ranges > 0, "{:?}", report);
    assert!(report.mixed_magnitude_ranges > 0, "{:?}", report);
}

fn run_parity_test(
    harness: &WeightedMeanGpuHarness,
    name: &str,
    n_parents: usize,
    children_per_parent: usize,
) {
    let scenario = make_weighted_mean_scenario(name, n_parents, children_per_parent);
    let report = compare_weighted_mean_rich_with_harness(harness, &scenario).unwrap();
    assert_generated_scenario_report(&report);

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
        WeightedChild {
            value: 42.0,
            weight: 2.0,
        },
        WeightedChild {
            value: 10.0,
            weight: 1.0,
        },
        WeightedChild {
            value: 20.0,
            weight: 3.0,
        },
        WeightedChild {
            value: 99.0,
            weight: 0.0,
        },
        WeightedChild {
            value: 99.0,
            weight: 0.0,
        },
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
    for output in &outputs {
        assert!(output.value.is_finite());
    }
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
fn weighted_mean_workshop_matches_production_shape_fixture() {
    let scenario = production_shape_fixture();
    let cpu_expected = weighted_mean_cpu(&scenario.children, &scenario.ranges);

    assert_eq!(cpu_expected[0].value, 0.0);
    assert_eq!(cpu_expected[1].value, 10.0);
    assert_eq!(cpu_expected[2].value, 0.0);
    assert_eq!(cpu_expected[3].value, 0.0);
    assert!((cpu_expected[4].value - (-0.8325)).abs() <= 1e-4);
    assert!((cpu_expected[5].value - 15.0).abs() <= 1e-6);
    assert_eq!(cpu_expected[6].value, 0.0);

    let harness = WeightedMeanGpuHarness::new().unwrap();
    let report = compare_weighted_mean_rich_with_harness(&harness, &scenario).unwrap();
    eprintln!("{}", format_report(&report));

    assert!(report.within_loose_tolerance, "{:?}", report);
    assert_ne!(report.parity_classification, "FAIL", "{:?}", report);
    assert!(report.repeated_runs_identical, "{:?}", report);
    assert!(report.non_empty_zero_weight_ranges > 0, "{:?}", report);
    assert!(report.empty_ranges > 0, "{:?}", report);
    assert!(report.single_child_ranges > 0, "{:?}", report);
    assert!(report.negative_value_ranges > 0, "{:?}", report);
    assert!(report.mixed_magnitude_ranges > 0, "{:?}", report);
}

#[test]
fn weighted_mean_parity_by_child_count_sweep() {
    let harness = WeightedMeanGpuHarness::new().unwrap();
    let child_counts = [0, 1, 2, 3, 4, 8, 16, 32, 64];

    for n in child_counts {
        let scenario = make_weighted_mean_scenario(&format!("weighted_mean_sweep_x{n}"), 2048, n);
        let report = compare_weighted_mean_rich_with_harness(&harness, &scenario).unwrap();
        eprintln!(
            "sweep n={n}: classification={} max_abs_error={} max_ulp_diff={}",
            report.parity_classification, report.max_abs_error, report.max_ulp_diff
        );

        assert!(report.repeated_runs_identical, "n={n}: {:?}", report);
        assert!(report.within_loose_tolerance, "n={n}: {:?}", report);
    }
}

#[test]
fn weighted_mean_rejects_invalid_inputs() {
    let harness = WeightedMeanGpuHarness::new().unwrap();

    let valid = make_weighted_mean_scenario("valid", 4, 4);

    let mut nan_value = valid.clone();
    nan_value.children[0].value = f32::NAN;
    assert!(compare_weighted_mean_rich(&nan_value).is_err());
    assert!(harness
        .eval(&nan_value.children, &nan_value.ranges)
        .is_err());

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
    assert!(harness
        .eval(&bad_range.children, &bad_range.ranges)
        .is_err());

    let mut past_end_range = valid.clone();
    past_end_range.ranges[0].offset = valid.children.len() as u32;
    past_end_range.ranges[0].len = 1;
    assert!(compare_weighted_mean_rich(&past_end_range).is_err());
    assert!(harness
        .eval(&past_end_range.children, &past_end_range.ranges)
        .is_err());
}

#[test]
fn weighted_mean_accepts_valid_edge_inputs() {
    let harness = WeightedMeanGpuHarness::new().unwrap();

    let children = vec![
        WeightedChild {
            value: 1.0,
            weight: 1.0,
        },
        WeightedChild {
            value: 2.0,
            weight: -0.0,
        },
    ];
    let ranges = vec![
        ParentRange {
            offset: children.len() as u32,
            len: 0,
        },
        ParentRange { offset: 0, len: 2 },
    ];
    validate_scenario(&children, &ranges).expect("valid scenario");
    let gpu = harness.eval(&children, &ranges).expect("gpu eval");
    let cpu = weighted_mean_cpu(&children, &ranges);
    assert_eq!(gpu.len(), 2);
    assert_eq!(cpu.len(), 2);
    for (g, c) in gpu.iter().zip(cpu.iter()) {
        assert!(g.value.is_finite());
        assert_eq!(g.value.to_bits(), c.value.to_bits());
    }

    let empty = WeightedMeanScenario {
        name: "empty".to_string(),
        children: Vec::new(),
        ranges: Vec::new(),
    };
    let gpu_empty = harness
        .eval(&empty.children, &empty.ranges)
        .expect("empty gpu");
    assert!(gpu_empty.is_empty());

    let trailing_empty = WeightedMeanScenario {
        name: "trailing_empty".to_string(),
        children: children.clone(),
        ranges: vec![ParentRange {
            offset: children.len() as u32,
            len: 0,
        }],
    };
    validate_scenario(&trailing_empty.children, &trailing_empty.ranges).expect("trailing empty");
    let gpu_trailing = harness
        .eval(&trailing_empty.children, &trailing_empty.ranges)
        .expect("trailing empty gpu");
    assert_eq!(gpu_trailing.len(), 1);
    assert_eq!(gpu_trailing[0].value, 0.0);
}

#[test]
fn weighted_mean_report_is_deterministic_across_runs() {
    let scenario = make_weighted_mean_scenario("weighted_mean_determinism", 256, 8);
    let harness = WeightedMeanGpuHarness::new().unwrap();

    let r1 = compare_weighted_mean_rich_with_harness(&harness, &scenario).unwrap();
    let r2 = compare_weighted_mean_rich_with_harness(&harness, &scenario).unwrap();
    let r3 = compare_weighted_mean_rich_with_harness(&harness, &scenario).unwrap();

    for (a, b) in [(&r1, &r2), (&r2, &r3)] {
        assert_eq!(a.max_abs_error, b.max_abs_error);
        assert_eq!(a.mean_abs_error, b.mean_abs_error);
        assert_eq!(a.parity_classification, b.parity_classification);
        assert!(a.repeated_runs_identical);
        assert!(b.repeated_runs_identical);
    }
}
