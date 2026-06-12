//! BH-1 — SaturatingFlux choke readout admission bounds.

use simthing_spec::{
    compile_region_field_preview, validate_region_field_frame_gradient_sinks,
    RegionFieldCadenceSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, RegionFieldSummaryPolicySpec,
};

fn base_spec() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "bh1_choke".into(),
        grid_size: 4,
        n_dims: 4,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 2.0,
            chi: 0.25,
            choke_output_col: Some(1),
        },
        horizon: 4,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::default(),
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: RegionFieldSummaryPolicySpec::default(),
        pressure_binding: None,
    }
}

#[test]
fn bh1_admission_accepts_valid_choke_output_col() {
    let spec = base_spec();
    let preview = compile_region_field_preview(&spec).expect("valid choke readout");
    match preview.stencil.operator {
        simthing_spec::CompiledRegionFieldOperator::SaturatingFlux {
            u_sat,
            chi,
            choke_output_col,
        } => {
            assert_eq!(u_sat, 2.0);
            assert_eq!(chi, 0.25);
            assert_eq!(choke_output_col, Some(1));
        }
        other => panic!("expected SaturatingFlux, got {other:?}"),
    }
}

#[test]
fn bh1_admission_rejects_choke_col_out_of_range() {
    let mut spec = base_spec();
    spec.operator = RegionFieldOperatorSpec::SaturatingFlux {
        u_sat: 2.0,
        chi: 0.25,
        choke_output_col: Some(4),
    };
    assert!(compile_region_field_preview(&spec).is_err());
}

#[test]
fn bh1_admission_rejects_choke_col_same_as_source_col() {
    let mut spec = base_spec();
    spec.operator = RegionFieldOperatorSpec::SaturatingFlux {
        u_sat: 2.0,
        chi: 0.25,
        choke_output_col: Some(0),
    };
    assert!(compile_region_field_preview(&spec).is_err());
}

#[test]
fn bh1_admission_rejects_same_frame_choke_sink_as_source_col() {
    let choke = base_spec();
    let mut consumer = base_spec();
    consumer.name = "consumer".into();
    consumer.source_col = 1;
    consumer.operator = RegionFieldOperatorSpec::Normalized;
    let fields = [&choke, &consumer];
    let err = validate_region_field_frame_gradient_sinks(&fields).expect_err("expected rejection");
    assert!(
        err.to_string().contains("strict sink column 1"),
        "unexpected error: {err}"
    );
}
