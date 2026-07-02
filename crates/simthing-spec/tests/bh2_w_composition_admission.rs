//! BH-2B — W impedance composition admission tests.

use simthing_spec::{
    compile_w_impedance_compose_preview, WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
    W_IMPEDANCE_COMPOSE_MAX_PROFILES,
};

fn base_spec() -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: 8,
        height: 8,
        n_dims: 6,
        base_w_col: 0,
        choke_a_col: 1,
        choke_b_col: 2,
        profiles: vec![
            WImpedanceComposeProfileSpec {
                weight_a: 1.0,
                weight_b: 0.5,
                output_w_col: 3,
            },
            WImpedanceComposeProfileSpec {
                weight_a: 2.0,
                weight_b: -1.0,
                output_w_col: 4,
            },
        ],
    }
}
