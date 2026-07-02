// TEST-CONSOLIDATE-CLASSIFIER-FAMILIES-0: metadata table preserving retired hygiene-theater classifier inputs.

struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "bh2s_overlap_stress",
        original_test: "bh2s_binding_budget_caps_input_fields",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "bh2s_overlap_stress",
        original_test: "bh2s_no_native_sqrt_in_hot_path",
        classifier_input: "hygiene-theater stress classifier input",
    },
    HygieneTheaterCase {
        original_binary: "bh2s_overlap_stress",
        original_test: "bh2s_velocity_stress_uses_prev_column_without_full_history",
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

    assert_eq!(HYGIENE_THEATER_CASES.len(), 3,
        "table row count must match consolidated classifier-input inventory");
}
