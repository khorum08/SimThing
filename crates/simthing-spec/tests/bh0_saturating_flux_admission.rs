//! BH-0 — RegionField SaturatingFlux admission bounds.

use simthing_spec::{
    compile_region_field_preview, RegionFieldCadenceSpec, RegionFieldOperatorSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, SATURATING_FLUX_CHI_CFL_MAX,
};

fn base_spec() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "bh0_test".into(),
        grid_size: 4,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: None,
        },
        horizon: 4,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: Default::default(),
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

#[test]
fn bh0_invalid_cfl_rejected() {
    let mut spec = base_spec();
    spec.operator = RegionFieldOperatorSpec::SaturatingFlux {
        u_sat: 2.0,
        chi: SATURATING_FLUX_CHI_CFL_MAX + 0.01,
        choke_output_col: None,
    };
    let err = compile_region_field_preview(&spec).unwrap_err();
    assert!(err.to_string().contains("CFL"));
}

#[test]
fn saturating_flux_admission_rejects_invalid_shapes() {
    let mut spec = base_spec();
    spec.operator = RegionFieldOperatorSpec::SaturatingFlux {
        u_sat: 0.0,
        chi: 0.25,
        choke_output_col: None,
    };
    assert!(compile_region_field_preview(&spec).is_err());

    spec = base_spec();
    spec.operator = RegionFieldOperatorSpec::SaturatingFlux {
        u_sat: 2.0,
        chi: 0.0,
        choke_output_col: None,
    };
    assert!(compile_region_field_preview(&spec).is_err());

    spec = base_spec();
    spec.source_cap = Some(1.0);
    assert!(compile_region_field_preview(&spec).is_err());

    spec = base_spec();
    spec.source_cap = None;
    spec.target_col = 1;
    assert!(compile_region_field_preview(&spec).is_err());
}

#[test]
fn saturating_flux_admission_accepts_valid_shape() {
    let spec = base_spec();
    let preview = compile_region_field_preview(&spec).expect("valid spec");
    match preview.stencil.operator {
        simthing_spec::CompiledRegionFieldOperator::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col,
        } => {
            assert_eq!(u_sat, 2.0);
            assert_eq!(chi, 0.25);
            assert_eq!(choke_output_col, None);
        }
        other => panic!("expected SaturatingFlux, got {other:?}"),
    }
}
