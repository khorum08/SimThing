use simthing_workshop::eml_phase5::{
    compare_cpu_gpu, eval_cpu_node, intensity_update_direct_cpu, intensity_update_nodes,
    make_inputs_with_params, IntensityInput,
};

fn run_match_test(n: usize) {
    let threshold = 0.1;
    let build = 0.2;
    let decay = 0.05;
    let dt = 1.0;

    let (nodes, root) = intensity_update_nodes(threshold, build, decay, dt);
    let inputs = make_inputs_with_params(n, threshold);

    let report = compare_cpu_gpu(&inputs, &nodes, root).unwrap();

    eprintln!(
        "EML Phase 5 intensity spike:\n\
         n_slots={}\n\
         cpu_eval_us={}\n\
         gpu_eval_us={}\n\
         max_abs_error={}\n\
         mean_abs_error={}\n\
         repeated_runs_identical={}",
        report.n_slots,
        report.cpu_eval_us,
        report.gpu_eval_us,
        report.max_abs_error,
        report.mean_abs_error,
        report.repeated_runs_identical,
    );

    assert!(
        report.max_abs_error <= 1e-4,
        "max_abs_error too high: {:?}",
        report
    );

    assert!(
        report.mean_abs_error <= 1e-5,
        "mean_abs_error too high: {:?}",
        report
    );

    assert!(
        report.repeated_runs_identical,
        "GPU repeated runs were not identical: {:?}",
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
}
