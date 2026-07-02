// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "ct_1b_recalc",
        original_test: "ct_1b_recalc_stress_measurement",
        classifier_input: "hygiene-theater stress classifier input",
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

    assert_eq!(HYGIENE_THEATER_CASES.len(), 1,
        "table row count must match consolidated classifier-input inventory");
}
