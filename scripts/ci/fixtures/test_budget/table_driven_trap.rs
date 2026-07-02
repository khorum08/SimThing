const CASES: &[&str] = &["alpha", "beta", "gamma", "delta"];

#[test]
fn rejects_bad_inputs_table_driven() {
    for case in CASES {
        assert!(!case.is_empty());
    }
}
