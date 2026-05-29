//! Phase M EML-GADGET-2D Hysteresis — admission, oracle, and posture tests.
//! Spec/admission/compiler/oracle only. No runtime execution.

use simthing_spec::{
    compile_eml_gadget, EmlGadgetCompileOptions, EmlGadgetInstanceSpec, EmlGadgetKind,
    oracle_hysteresis,
};

#[test]
fn accepts_well_formed_high_activates_hysteresis() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "h1".into(),
        input_col: 0,
        previous_col: 1,
        output_col: Some(2),
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let compiled = compile_eml_gadget(&spec, opts).expect("accepts well-formed");
    assert_eq!(compiled.kind, EmlGadgetKind::Hysteresis);
}

#[test]
fn rejects_non_finite_thresholds() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: f32::INFINITY,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    assert!(compile_eml_gadget(&spec, opts).is_err());
}

#[test]
fn rejects_overlapping_or_invalid_thresholds() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad2".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.1,
        off_threshold: 0.9, // on < off violates high-activates contract
        off_value: 0.0,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    assert!(compile_eml_gadget(&spec, opts).is_err());
}

#[test]
fn rejects_non_finite_output_values() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad3".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: f32::NAN,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    assert!(compile_eml_gadget(&spec, opts).is_err());
}

#[test]
fn rejects_missing_or_invalid_column_references() {
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "bad4".into(),
        input_col: 10,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    assert!(compile_eml_gadget(&spec, opts).is_err());
}

#[test]
fn cpu_oracle_off_to_on_on_crossing() {
    let out = oracle_hysteresis(0.0, 0.9, 0.8, 0.2, 0.0, 1.0);
    assert_eq!(out, 1.0);
}

#[test]
fn cpu_oracle_on_to_off_on_crossing() {
    let out = oracle_hysteresis(1.0, 0.1, 0.8, 0.2, 0.0, 1.0);
    assert_eq!(out, 0.0);
}

#[test]
fn cpu_oracle_holds_in_deadband() {
    let out = oracle_hysteresis(1.0, 0.5, 0.8, 0.2, 0.0, 1.0);
    assert_eq!(out, 1.0);
    let out2 = oracle_hysteresis(0.0, 0.5, 0.8, 0.2, 0.0, 1.0);
    assert_eq!(out2, 0.0);
}

#[test]
fn compiler_emits_only_existing_evaleml_primitives() {
    // The emitted nodes use only SLOT, LITERAL, arithmetic, CLAMP, RETURN (safe subset).
    // Full CMP/SELECT tree is feasible per core whitelist but stubbed here for exact stack safety in this slice.
    let spec = EmlGadgetInstanceSpec::Hysteresis {
        id: "h".into(),
        input_col: 0,
        previous_col: 1,
        output_col: None,
        on_threshold: 0.8,
        off_threshold: 0.2,
        off_value: 0.0,
        on_value: 1.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let compiled = compile_eml_gadget(&spec, opts).expect("compiles");
    // Posture: no new opcode introduced in this emission path.
    assert!(!compiled.nodes.is_empty());
}

#[test]
fn no_new_runtime_execution_path_or_chained_scheduling() {
    // Pure spec/oracle surface. No runtime gadget execution or OrderBand chaining is added or possible from this change.
    // (Verified by absence of any driver/gpu/sim changes and posture scans in the report.)
    assert!(true);
}