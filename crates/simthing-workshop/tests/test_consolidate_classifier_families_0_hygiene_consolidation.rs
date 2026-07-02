// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_distributed_100k_summary_only",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_distributed_1m",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_hotspot_100k_summary_only",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_records_mode_replays_small",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_report_bundle",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "persistent_bench",
        original_test: "persistent_bench_sparse_100k_summary_only",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "weighted_mean_perf",
        original_test: "weighted_mean_perf_dense_10k_x32_dims16_wm16",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "weighted_mean_perf",
        original_test: "weighted_mean_perf_overlay_density_sweep",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "weighted_mean_perf",
        original_test: "weighted_mean_perf_sparse_100k_x32_dims64_wm1",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
];

#[test]
fn hygiene_theater_cases_table_preserves_inputs() {
    let mut seen = std::collections::BTreeSet::new();

    for case in HYGIENE_THEATER_CASES {
        assert!(seen.insert((case.original_binary, case.original_test)),
            "duplicate classifier input case: {}::{}", case.original_binary, case.original_test);
        assert!(!case.classifier_input.is_empty(),
            "classifier input must be non-empty for {}::{}", case.original_binary, case.original_test);
    }

    assert_eq!(HYGIENE_THEATER_CASES.len(), 9,
        "table row count must match consolidated classifier-input inventory");
}
