//! FrontierV1-1 — end-to-end opt-in FrontierV1 fixture wiring (Tier-2, test-only).
//!
//! Wires accepted first-slice Mapping, flat-star Resource Flow, and SEAD Self-AI Proposal
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

fn cpu_oracle_expected(config: &FrontierV1FixtureConfig, skeleton: &FrontierV1ScenarioSkeleton) -> FrontierV1FixtureOutput {
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
fn frontier_v1_1_happy_path_opt_in_fixture_runs() {
    let (skeleton, config) = smoke_fixture();
    let output = run_frontier_v1_fixture(&skeleton, &config);

    assert!(output.admission_accepted);
    assert_eq!(skeleton.profile_name, FRONTIER_V1_PROFILE_NAME);
    assert!(!skeleton.enabled_by_default);
    assert_eq!(
        skeleton.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1
    );
    assert_eq!(skeleton.resource_flow_opt_in, ResourceFlowOptInMode::FlatStarOptIn);
    assert_eq!(
        skeleton.resource_flow_execution_profile,
        ResourceFlowExecutionProfile::FlatStarResourceFlow
    );
    assert_eq!(skeleton.sead.pipeline_version, SeadPipelineVersion::ProposalPipelineV1);
    assert_eq!(skeleton.theater.grid_width, 8);
    assert_eq!(skeleton.theater.grid_height, 8);
    assert!(output.mapping.cell_sum > 0);
    assert!(output.resource_flow.allocated_a > 0);
    assert!(output.resource_flow.allocated_b > 0);
    assert_eq!(output.proposal_count, 3);
    assert_eq!(output.routes.resource_route_count, 1);
    assert_eq!(output.routes.structural_route_count, 1);
    assert_eq!(output.routes.movement_route_count, 1);
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);

    println!(
        "frontier_v1_1_happy: fixture_id={FRONTIER_V1_FIXTURE_ID} fp={} mapping_sum={} alloc_a={} alloc_b={}",
        output.fingerprint.hex(),
        output.mapping.cell_sum,
        output.resource_flow.allocated_a,
        output.resource_flow.allocated_b,
    );
}

#[test]
fn frontier_v1_1_defaults_remain_disabled() {
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
    assert_eq!(ResourceFlowOptInMode::default(), ResourceFlowOptInMode::Disabled);
    assert_eq!(
        ResourceFlowExecutionProfile::default(),
        ResourceFlowExecutionProfile::DefaultDisabled
    );
    assert!(
        !ResourceFlowExecutionProfile::default().enables_flat_star_resource_flow()
    );

    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    assert!(
        !sim_lib.contains("FrontierV1"),
        "FrontierV1 must not be wired into default simthing-sim"
    );

    let (skeleton, config) = smoke_fixture();
    assert!(!skeleton.enabled_by_default);
    let output = run_frontier_v1_fixture(&skeleton, &config);
    assert!(output.admission_accepted);

    println!("frontier_v1_1_defaults: disabled=true fixture_id={FRONTIER_V1_FIXTURE_ID}");
}

#[test]
fn frontier_v1_1_resource_dispatch_routes_through_allocator() {
    let (skeleton, config) = smoke_fixture();
    let output = run_frontier_v1_fixture(&skeleton, &config);

    assert_eq!(
        classify_proposal_route(ProposalKind::ResourceDispatch, &skeleton),
        ProposalRoute::ResourceFlowAllocator
    );
    assert_eq!(output.routes.resource_route_count, 1);
    assert!(!skeleton.resource_flow.parallel_fixture_economy);
    assert!(!skeleton.resource_flow.shared_pool_tick_writes);
    assert!(!skeleton.sead.cpu_planner);
    assert!(skeleton.resource_flow.resource_flow_allocator_only);

    println!(
        "frontier_v1_1_resource_route: ResourceFlowAllocator fixture_id={FRONTIER_V1_FIXTURE_ID}"
    );
}

#[test]
fn frontier_v1_1_coupling_rejects_non_frontier_profile() {
    let (mut skeleton, config) = smoke_fixture();
    skeleton.coupling.coupling_requested = true;
    let frontier = validate_frontier_v1_admission(&skeleton);
    assert!(frontier.coupling_ok);
    assert!(frontier.accepted);

    skeleton.profile_name = "OtherProfile";
    let other = validate_frontier_v1_admission(&skeleton);
    assert!(!other.coupling_ok);
    assert!(!other.accepted);

    skeleton.profile_name = FRONTIER_V1_PROFILE_NAME;
    skeleton.enabled_by_default = true;
    let default_on = validate_frontier_v1_admission(&skeleton);
    assert!(!default_on.accepted);

    let output = run_frontier_v1_fixture(&frontier_v1_1_smoke_skeleton(), &config);
    assert!(output.admission_accepted);
    assert!(!frontier_v1_1_smoke_skeleton().enabled_by_default);

    println!("frontier_v1_1_coupling: frontier_only=true fixture_id={FRONTIER_V1_FIXTURE_ID}");
}

#[test]
fn frontier_v1_1_deferred_features_reject() {
    let deferred: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 10] = [
        ("atlas", Box::new(|s| s.theater.request_atlas = true)),
        ("active_mask", Box::new(|s| s.theater.request_active_mask = true)),
        ("perception", Box::new(|s| s.theater.request_perception = true)),
        (
            "source_identity",
            Box::new(|s| s.theater.request_source_identity = true),
        ),
        ("nested_e11b", Box::new(|s| s.resource_flow.nested_e11b = true)),
        (
            "e11b_5",
            Box::new(|s| s.resource_flow.e11b_5_dynamic_enrollment = true),
        ),
        (
            "d2a",
            Box::new(|s| s.resource_flow.d2a_hard_currency_ordering = true),
        ),
        (
            "act5_ladder",
            Box::new(|s| s.sead.pipeline_version = SeadPipelineVersion::Other),
        ),
        (
            "parallel_fixture",
            Box::new(|s| s.resource_flow.parallel_fixture_economy = true),
        ),
        ("cpu_planner", Box::new(|s| s.sead.cpu_planner = true)),
    ];
    for (label, mutate) in deferred {
        let mut skeleton = frontier_v1_1_smoke_skeleton();
        mutate(&mut skeleton);
        let report = validate_frontier_v1_admission(&skeleton);
        assert!(!report.accepted, "{label} should reject");
    }
    println!("frontier_v1_1_deferred: rejects=true fixture_id={FRONTIER_V1_FIXTURE_ID}");
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

#[test]
fn frontier_v1_1_replay_reproducibility() {
    let (skeleton, config) = smoke_fixture();
    let run_a = run_frontier_v1_fixture(&skeleton, &config);
    let run_b = run_frontier_v1_fixture(&skeleton, &config);

    assert_eq!(run_a.fingerprint, run_b.fingerprint);
    assert_eq!(run_a.mapping, run_b.mapping);
    assert_eq!(run_a.resource_flow, run_b.resource_flow);
    assert_eq!(run_a.routes, run_b.routes);
    assert_eq!(run_a.fingerprint.hex(), FRONTIER_V1_FIXTURE_FINGERPRINT);

    println!(
        "frontier_v1_1_replay: fp={} fixture_id={FRONTIER_V1_FIXTURE_ID}",
        run_a.fingerprint.hex()
    );
}

#[test]
fn frontier_v1_1_no_simthing_sim_semantic_awareness() {
    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    for needle in [
        "FrontierV1",
        "SEAD",
        "RegionCell",
        "ArenaRegistry",
        "proposal",
        "ResourceFlow",
    ] {
        assert!(
            !sim_lib.contains(needle),
            "simthing-sim must not contain `{needle}`"
        );
    }
    println!("frontier_v1_1_sim: semantic_free=true fixture_id={FRONTIER_V1_FIXTURE_ID}");
}

#[test]
fn frontier_v1_1_no_new_gpu_primitive() {
    let frontier_descriptor = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| {
            d.id.contains("frontier")
                || d.id.contains("FrontierV1")
                || d.id.contains("frontier_v1_1")
        });
    assert!(
        frontier_descriptor.is_none(),
        "FrontierV1-1 must not add a landed kernel descriptor"
    );
    let wgsl_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wgsl");
    if wgsl_dir.is_dir() {
        for entry in std::fs::read_dir(&wgsl_dir).expect("read wgsl dir") {
            let path = entry.expect("entry").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            assert!(
                !name.contains("frontier"),
                "no FrontierV1 WGSL: {}",
                path.display()
            );
        }
    }
    println!(
        "frontier_v1_1_gpu: new_wgsl=false new_descriptor=false fixture_id={FRONTIER_V1_FIXTURE_ID}"
    );
}
