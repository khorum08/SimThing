// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "typeface_lr5",
        original_test: "ci_bench_budget_gates_pass",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr5",
        original_test: "heavy_bench_manual",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "binding_perf_evidence_documented_for_lr9",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "flat_5k_binding_noop_perf_profile",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "flat_5k_noop_perf_profile_records_budget",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "numeric_damage_5k_binding_perf_profile",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "numeric_damage_5k_perf_profile_records_budget",
        classifier_input: "hygiene-theater perf/benchmark classifier input",
    },
    HygieneTheaterCase {
        original_binary: "typeface_lr9",
        original_test: "warped_nameplate_binding_perf_profile",
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

    assert_eq!(HYGIENE_THEATER_CASES.len(), 8,
        "table row count must match consolidated classifier-input inventory");
}
