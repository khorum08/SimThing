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

#[test]
fn bh2_admission_accepts_valid_compose_shape() {
    let compiled = compile_w_impedance_compose_preview(&base_spec()).expect("valid compose");
    assert_eq!(compiled.width, 8);
    assert_eq!(compiled.height, 8);
    assert_eq!(compiled.n_dims, 6);
    assert_eq!(compiled.profiles.len(), 2);
    assert_eq!(compiled.profiles[0].output_w_col, 3);
    assert_eq!(compiled.profiles[1].output_w_col, 4);
}

#[test]
fn bh2_admission_rejects_invalid_columns_weights_or_aliasing() {
    let mut bad_col = base_spec();
    bad_col.choke_b_col = 6;
    assert!(compile_w_impedance_compose_preview(&bad_col).is_err());

    let mut alias = base_spec();
    alias.profiles[0].output_w_col = 1;
    assert!(compile_w_impedance_compose_preview(&alias).is_err());

    let mut nan_weight = base_spec();
    nan_weight.profiles[0].weight_a = f32::INFINITY;
    assert!(compile_w_impedance_compose_preview(&nan_weight).is_err());

    let mut empty = base_spec();
    empty.profiles.clear();
    assert!(compile_w_impedance_compose_preview(&empty).is_err());

    let mut too_many = base_spec();
    too_many.n_dims = 20;
    too_many.profiles = (0..=W_IMPEDANCE_COMPOSE_MAX_PROFILES)
        .map(|i| WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 0.0,
            output_w_col: 5 + i as u32,
        })
        .collect();
    assert!(compile_w_impedance_compose_preview(&too_many).is_err());
}

#[test]
fn bh2_admission_rejects_zero_dimensions() {
    let mut spec = base_spec();
    spec.width = 0;
    assert!(compile_w_impedance_compose_preview(&spec).is_err());
}

#[test]
fn bh2_admission_rejects_duplicate_output_cols() {
    let mut spec = base_spec();
    spec.profiles[1].output_w_col = 3;
    assert!(compile_w_impedance_compose_preview(&spec).is_err());
}
