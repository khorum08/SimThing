struct HygieneTheaterCase {
    original_binary: &'static str,
    original_test: &'static str,
    classifier_input: &'static str,
    scale_label: &'static str,
}

const HYGIENE_THEATER_CASES: &[HygieneTheaterCase] = &[
    HygieneTheaterCase {
        original_binary: "mobility_alloc0_substrate",
        original_test: "alloc_scale_soak_34k",
        classifier_input: "allocation substrate scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_econ0_substrate",
        original_test: "econ_scale_soak_34k",
        classifier_input: "economy substrate scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_idroute0_substrate",
        original_test: "idroute_scale_soak_34k",
        classifier_input: "identity routing substrate scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_owner0_substrate",
        original_test: "owner_scale_soak_34k",
        classifier_input: "owner substrate scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_reenroll0_substrate",
        original_test: "reenroll_scale_soak_34k_movement_churn",
        classifier_input: "reenrollment movement churn scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_runtime0_composition",
        original_test: "runtime0_34k_integrated_scenario_soak",
        classifier_input: "runtime0 integrated scenario scale soak",
        scale_label: "34k",
    },
    HygieneTheaterCase {
        original_binary: "mobility_runtime1_production_fixture",
        original_test: "runtime1_34k_production_fixture_soak",
        classifier_input: "runtime1 production fixture scale soak",
        scale_label: "34k",
    },
];

#[test]
fn hygiene_theater_cases_table_preserves_inputs() {
    let mut seen = std::collections::BTreeSet::new();

    for case in HYGIENE_THEATER_CASES {
        assert!(seen.insert((case.original_binary, case.original_test)));
        assert!(!case.classifier_input.is_empty());
        assert_eq!(case.scale_label, "34k");
    }

    assert_eq!(HYGIENE_THEATER_CASES.len(), 7);
}
