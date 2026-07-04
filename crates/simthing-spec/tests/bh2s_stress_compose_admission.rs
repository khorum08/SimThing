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
