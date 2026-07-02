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
