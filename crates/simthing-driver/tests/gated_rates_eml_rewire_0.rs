//! GATED-RATES-EML-REWIRE-0 proofs.
//!
//! Ordinary production path uses role-pathway columns and the one existing EML
//! library/cap. Wrong implementations are test-side only.

use simthing_core::{ColumnIndex, MAX_EML_TREE_NODES};

const GATED_RATES_SRC: &str = include_str!("../src/gated_rates.rs");
const MAPPING_RUNTIME_SRC: &str = include_str!("../src/mapping_runtime.rs");

#[test]
fn gated_rates_eml_rewire_table() {
    for needle in [
        "ColumnIndex::new",
        "flow_start +",
        "fold_output_into_input",
        "EmlExpressionRegistry::new",
        "ExactBearingEvidence",
        "derive_consumer_arms",
        "ExactConsumerArm",
        "ExactConsumerDigestEvidence",
    ] {
        assert!(
            !GATED_RATES_SRC.contains(needle),
            "gated_rates production must not contain {needle}"
        );
    }
    assert!(
        GATED_RATES_SRC.contains("col_for_role"),
        "gated_rates must reach columns through the role pathway"
    );
    assert!(
        GATED_RATES_SRC.contains("MAX_EML_TREE_NODES"),
        "gated_rates must inherit the one existing per-program cap"
    );

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("first_slice_mapping_runtime.rs");
    assert!(
        !path.exists(),
        "first_slice_mapping_runtime.rs must be deleted, found {}",
        path.display()
    );

    for needle in [
        "let eml_resource_col = 1",
        "let eml_output_col = 4",
        "let eml_weight_a_col = 2",
        "let eml_weight_b_col = 3",
    ] {
        assert!(
            !MAPPING_RUNTIME_SRC.contains(needle),
            "mapping_runtime must not hardcode {needle}"
        );
    }
    assert!(
        MAPPING_RUNTIME_SRC.contains("try_from_admitted_authored")
            || MAPPING_RUNTIME_SRC.contains("urgency_col"),
        "mapping columns must come from admitted/compiled identity"
    );

    let mutant = ColumnIndex::from_raw_for_oracle_or_rehearsal(1);
    let admitted = ColumnIndex::try_from_admitted_authored(4, 8).expect("admitted");
    assert_ne!(
        mutant, admitted,
        "test-side raw col-1 must disagree with an admitted urgency/role column"
    );
    assert!(MAX_EML_TREE_NODES > 0);
}
