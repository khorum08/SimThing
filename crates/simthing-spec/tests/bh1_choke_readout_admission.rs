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
