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

