//! RF-T3 — product-like FlatStarOptIn soak scenarios.

mod support;

use simthing_driver::{
    assert_telemetry_contract, fixture_product_disabled_spec_diagnostics,
    fixture_product_dynamic_fission_cadence, fixture_product_multi_arena_no_coupling,
    fixture_product_multi_session_replay, fixture_product_rejection_telemetry,
    fixture_product_repeated_resync, fixture_product_static_128_participants,
    fixture_product_static_256_participants, open_product_session, run_multi_session_replay,
    run_product_soak_with_telemetry, RF_T3_PRODUCT_DISABLED,
};
use simthing_sim::PipelineFlags;
use simthing_spec::ResourceFlowOptInMode;

use support::e11_burn_in_scenarios::assert_flat_star_only_no_nested_claims;
use support::e11_flat_star::{try_gpu, FlatStarSession};

#[test]
fn rf_t3_product_rejection_telemetry_fixture() {
    let fixture = fixture_product_rejection_telemetry();
    let fx = open_product_session(&fixture).expect("open");
    let telemetry = simthing_driver::telemetry_for_open_session(&fx, &fixture, None);
    assert_eq!(telemetry.dynamic_rejections, 1);
    assert_eq!(telemetry.dynamic_admissions, 0);
}

