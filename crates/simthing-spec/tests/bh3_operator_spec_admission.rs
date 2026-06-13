//! BH-3-AUTHORING-0 — admission guardrails for lowered field-operator profiles.

use simthing_spec::{
    compile_region_field_preview, compile_stress_compose_preview,
    compile_w_impedance_compose_preview, FirstSliceCommitmentDirectionSpec,
    FirstSliceCommitmentSpec, MappingExecutionProfile, RegionFieldCadenceSpec,
    RegionFieldFormulaBindingSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSourcePolicySpec, RegionFieldSpec, StressComposeSpec,
    StressOperatorSpec, WImpedanceComposeProfileSpec, WImpedanceComposeSpec,
};

fn lowered_region_field() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "bh3_field".into(),
        grid_size: 10,
        n_dims: 6,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 1.0,
            chi: 0.25,
            choke_output_col: Some(2),
        },
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 0.5,
        gamma_neighbor: 0.125,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 0,
            child_slot_count: 100,
            child_col: 0,
            parent_slot: 100,
            parent_col: 0,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: None,
            weight_pressure: Some(1.0),
            weight_resource: Some(0.5),
        }),
        commitment: Some(FirstSliceCommitmentSpec {
            source_formula_class: "field_urgency".into(),
            parent_slot: 100,
            urgency_col: 4,
            threshold: 5490.8657,
            direction: FirstSliceCommitmentDirectionSpec::Upward,
            event_kind: 1_398_039_876,
            effect: None,
        }),
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: None,
    }
}

fn lowered_w_compose() -> WImpedanceComposeSpec {
    WImpedanceComposeSpec {
        width: 10,
        height: 10,
        n_dims: 6,
        base_w_col: 1,
        choke_a_col: 2,
        choke_b_col: 3,
        profiles: vec![WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 0.5,
            output_w_col: 4,
        }],
    }
}

fn lowered_stress_compose() -> StressComposeSpec {
    StressComposeSpec {
        width: 10,
        height: 10,
        n_dims: 6,
        choke_a_col: 2,
        choke_b_col: 3,
        profiles: vec![simthing_spec::StressComposeProfileSpec {
            operator: StressOperatorSpec::Overlap,
            output_col: 5,
        }],
    }
}

#[test]
fn bh3_admission_accepts_lowered_operator_profile() {
    let field = lowered_region_field();
    compile_region_field_preview(&field).expect("region field admits");
    compile_w_impedance_compose_preview(&lowered_w_compose()).expect("w compose admits");
    compile_stress_compose_preview(&lowered_stress_compose()).expect("stress compose admits");
    assert_eq!(
        MappingExecutionProfile::Disabled,
        MappingExecutionProfile::Disabled
    );
    assert!(!MappingExecutionProfile::Disabled.enables_execution());
}

#[test]
fn bh3_admission_rejects_missing_u_sat() {
    let mut field = lowered_region_field();
    if let RegionFieldOperatorSpec::SaturatingFlux { u_sat, .. } = &mut field.operator {
        *u_sat = 0.0;
    }
    assert!(compile_region_field_preview(&field).is_err());
}

#[test]
fn bh3_admission_rejects_invalid_chi_and_non_finite_values() {
    let mut field = lowered_region_field();
    if let RegionFieldOperatorSpec::SaturatingFlux { chi, .. } = &mut field.operator {
        *chi = 0.5;
    }
    assert!(compile_region_field_preview(&field).is_err());

    let mut field = lowered_region_field();
    if let RegionFieldOperatorSpec::SaturatingFlux { u_sat, .. } = &mut field.operator {
        *u_sat = f32::NAN;
    }
    assert!(compile_region_field_preview(&field).is_err());
}

#[test]
fn bh3_admission_rejects_unknown_output_binding_and_unbounded_fanout() {
    let mut w = lowered_w_compose();
    w.profiles[0].output_w_col = 99;
    assert!(compile_w_impedance_compose_preview(&w).is_err());

    let mut w = lowered_w_compose();
    w.profiles = (0..9)
        .map(|i| WImpedanceComposeProfileSpec {
            weight_a: 1.0,
            weight_b: 0.0,
            output_w_col: 4 + i,
        })
        .collect();
    assert!(compile_w_impedance_compose_preview(&w).is_err());
}
