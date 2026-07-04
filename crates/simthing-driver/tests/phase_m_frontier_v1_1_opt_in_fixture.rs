//! FrontierV1-1 — end-to-end opt-in FrontierV1 fixture wiring (Tier-2, test-only).
//!
//! Wires accepted first-slice Mapping, flat-star Resource Flow, and FIELD_POLICY Field agent Proposal
//! Pipeline V1 substrates through a bounded CPU-oracle fixture. No default SimSession wiring,
//! no new WGSL/descriptor, no simthing-sim semantic awareness.

#[path = "support/frontier_v1.rs"]
mod frontier_v1;

use frontier_v1::*;
use simthing_spec::{
    landed_jit_kernel_descriptors, MappingExecutionProfile, ResourceFlowExecutionProfile,
    ResourceFlowOptInMode,
};

pub const FRONTIER_V1_FIXTURE_FINGERPRINT: &str = "49d4c94ce1f52be5";

fn smoke_fixture() -> (FrontierV1ScenarioSkeleton, FrontierV1FixtureConfig) {
    (
        frontier_v1_1_smoke_skeleton(),
        frontier_v1_1_fixture_config(),
    )
}

fn cpu_oracle_expected(
    config: &FrontierV1FixtureConfig,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> FrontierV1FixtureOutput {
    let mapping = cpu_mapping_oracle(config, skeleton);
    let resource_flow = cpu_resource_flow_oracle(config, mapping, skeleton);
    let routes = cpu_route_oracle(config, skeleton);
    let proposal_count = config.proposals.len() as u32;
    let fingerprint = fingerprint_from_parts(mapping, resource_flow, proposal_count, routes);
    FrontierV1FixtureOutput {
        admission_accepted: validate_frontier_v1_admission(skeleton).accepted,
        mapping,
        resource_flow,
        proposal_count,
        event_count: routes.structural_route_count,
        routes,
        fingerprint,
    }
}

#[test]
fn frontier_v1_1_cpu_oracle_parity() {
    let (skeleton, config) = smoke_fixture();
    let output = run_frontier_v1_fixture(&skeleton, &config);
    let expected = cpu_oracle_expected(&config, &skeleton);

    assert_eq!(output.admission_accepted, expected.admission_accepted);
    assert_eq!(output.mapping, expected.mapping);
    assert_eq!(output.resource_flow, expected.resource_flow);
    assert_eq!(output.routes, expected.routes);
    assert_eq!(output.proposal_count, expected.proposal_count);
    assert_eq!(output.event_count, expected.event_count);
    assert_eq!(output.fingerprint, expected.fingerprint);
    assert!(!output.mapping.overflow);
    assert!(!output.resource_flow.overflow);

    println!(
        "frontier_v1_1_oracle: fp={} fixture_id={FRONTIER_V1_FIXTURE_ID}",
        output.fingerprint.hex()
    );
}
