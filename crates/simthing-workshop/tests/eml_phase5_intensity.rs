use simthing_workshop::eml_phase5::{
    compare_cpu_gpu, compare_cpu_gpu_rich, eval_cpu_node, format_rich_report, intensity_update_direct_cpu,
    intensity_update_nodes, make_inputs_with_params, EmlGpuHarness, IntensityFormulaParams, IntensityInput,
    MAX_NODES,
};

fn run_match_test(n: usize) {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    let inputs = make_inputs_with_params(n, threshold);
    let formula_params =
        IntensityFormulaParams::new(n as u32, threshold, build, decay, dt);

    let report = compare_cpu_gpu_rich(&inputs, &nodes, root, formula_params).unwrap();

    eprintln!("{}", format_rich_report(&report));

    if n == 100_000 {
        let report_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/workshop");
        std::fs::create_dir_all(&report_dir).expect("create target/workshop");
        std::fs::write(
            report_dir.join("eml_phase5_rich_report_100k.md"),
            format_rich_report(&report),
        )
        .expect("write 100k rich report");
    }

    assert!(
        report.eml_vs_cpu_max_abs_error <= 1e-4,
        "eml_vs_cpu_max_abs_error too high: {:?}",
        report
    );
    assert!(
        report.eml_vs_cpu_mean_abs_error <= 1e-5,
        "eml_vs_cpu_mean_abs_error too high: {:?}",
        report
    );
    assert!(
        report.hardcoded_vs_cpu_max_abs_error <= 1e-4,
        "hardcoded_vs_cpu_max_abs_error too high: {:?}",
        report
    );
    assert!(
        report.hardcoded_vs_cpu_mean_abs_error <= 1e-5,
        "hardcoded_vs_cpu_mean_abs_error too high: {:?}",
        report
    );
    assert!(
        report.eml_vs_hardcoded_max_abs_error <= 1e-4,
        "eml_vs_hardcoded_max_abs_error too high: {:?}",
        report
    );
    assert!(
        report.eml_repeated_runs_identical,
        "EML warm repeated runs were not identical: {:?}",
        report
    );
    assert!(
        report.hardcoded_repeated_runs_identical,
        "hardcoded warm repeated runs were not identical: {:?}",
        report
    );
}

#[test]
fn cpu_node_evaluator_matches_direct_formula() {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    let inputs = make_inputs_with_params(10_000, threshold);

    for input in inputs {
        let via_nodes = eval_cpu_node(&nodes, root, input);
        let direct = intensity_update_direct_cpu(input, threshold, build, decay, dt);
        assert!(
            (via_nodes - direct).abs() <= 1e-6,
            "node eval {} != direct {} for {:?}",
            via_nodes,
            direct,
            input
        );
    }
}

#[test]
fn gpu_eml_intensity_matches_cpu_1k() {
    run_match_test(1_000);
}

#[test]
fn gpu_eml_intensity_matches_cpu_10k() {
    run_match_test(10_000);
}

#[test]
fn gpu_eml_intensity_matches_cpu_100k() {
    run_match_test(100_000);
}

#[test]
fn rejects_nan_or_infinite_inputs() {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    let formula_params =
        IntensityFormulaParams::new(3, threshold, build, decay, dt);

    let bad = vec![
        IntensityInput {
            velocity: f32::NAN,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: f32::INFINITY,
            intensity: 0.5,
        },
        IntensityInput {
            velocity: 1.0,
            intensity: f32::NAN,
        },
    ];

    assert!(compare_cpu_gpu(&bad, &nodes, root).is_err());
    assert!(compare_cpu_gpu_rich(&bad, &nodes, root, formula_params).is_err());
}

#[test]
fn zero_length_inputs_return_empty_outputs() {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    let harness = EmlGpuHarness::new().unwrap();

    let outputs = harness.eval_eml(&[], &nodes, root).unwrap();
    assert!(outputs.is_empty());

    let hardcoded = harness
        .eval_hardcoded(
            &[],
            IntensityFormulaParams::new(0, threshold, build, decay, dt),
        )
        .unwrap();
    assert!(hardcoded.is_empty());
}

#[test]
fn rejects_too_many_nodes() {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (mut nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    while nodes.len() <= MAX_NODES {
        nodes.push(nodes[0]);
    }

    let inputs = make_inputs_with_params(1, threshold);
    let harness = EmlGpuHarness::new().unwrap();
    assert!(harness.eval_eml(&inputs, &nodes, root).is_err());
}
