//! BH-2S — stress compose admission tests.

use simthing_spec::{
    compile_stress_compose_preview, StressComposeProfileSpec, StressComposeSpec,
    StressOperatorSpec, STRESS_COMPOSE_MAX_INPUT_FIELDS, STRESS_COMPOSE_MAX_PROFILES,
};

fn base_spec() -> StressComposeSpec {
    StressComposeSpec {
        width: 8,
        height: 8,
        n_dims: 8,
        choke_a_col: 0,
        choke_b_col: 1,
        profiles: vec![
            StressComposeProfileSpec {
                operator: StressOperatorSpec::Overlap,
                output_col: 4,
            },
            StressComposeProfileSpec {
                operator: StressOperatorSpec::Mismatch,
                output_col: 5,
            },
        ],
    }
}

#[test]
fn bh2s_admission_accepts_valid_stress_shape() {
    let compiled = compile_stress_compose_preview(&base_spec()).expect("valid");
    assert_eq!(compiled.profiles.len(), 2);
}

#[test]
fn bh2s_admission_rejects_input_field_budget_exceeded() {
    let spec = StressComposeSpec {
        width: 4,
        height: 4,
        n_dims: 10,
        choke_a_col: 0,
        choke_b_col: 1,
        profiles: vec![
            StressComposeProfileSpec {
                operator: StressOperatorSpec::Velocity {
                    choke_now_col: 2,
                    choke_prev_col: 3,
                },
                output_col: 4,
            },
            StressComposeProfileSpec {
                operator: StressOperatorSpec::Velocity {
                    choke_now_col: 6,
                    choke_prev_col: 7,
                },
                output_col: 5,
            },
        ],
    };
    assert!(compile_stress_compose_preview(&spec).is_err());
    assert!(spec.profiles.len() <= STRESS_COMPOSE_MAX_PROFILES);
    let _ = STRESS_COMPOSE_MAX_INPUT_FIELDS;
}

#[test]
fn bh2s_admission_rejects_duplicate_output_cols() {
    let mut spec = base_spec();
    spec.profiles[1].output_col = 4;
    assert!(compile_stress_compose_preview(&spec).is_err());
}

#[test]
fn bh2s_admission_rejects_column_aliasing() {
    let mut spec = base_spec();
    spec.profiles[0].output_col = 0;
    assert!(compile_stress_compose_preview(&spec).is_err());
}
