//! FrontierV1-0 — default-off scenario skeleton and admission contract (Tier-2, test-only).

#[path = "support/frontier_v1.rs"]
mod frontier_v1;

use frontier_v1::*;
use simthing_spec::{landed_jit_kernel_descriptors, MappingExecutionProfile};

#[test]
fn frontier_v1_0_happy_path_skeleton_admits() {
    let skeleton = frontier_v1_happy_path_skeleton();
    let report = validate_frontier_v1_admission(&skeleton);
    assert!(report.accepted, "rejected: {:?}", report.rejected_reasons);
    assert!(report.default_off_ok);
    assert!(report.mapping_ok);
    assert!(report.flat_star_ok);
    assert!(report.field_policy_v1_ok);
    assert!(report.coupling_ok);
    assert_eq!(skeleton.profile_name, FRONTIER_V1_PROFILE_NAME);
    assert!(!skeleton.enabled_by_default);
    assert_eq!(skeleton.theater.grid_width, 32);
    assert_eq!(skeleton.theater.grid_height, 32);
    assert_eq!(
        skeleton.theater.operator,
        simthing_spec::RegionFieldOperatorSpec::SourceCappedNormalized
    );
    assert_eq!(skeleton.theater.horizon, 8);
    assert_eq!(skeleton.resource_flow.depth, 2);
    assert!(skeleton.resource_flow.resource_flow_allocator_only);
    assert_eq!(
        skeleton.field_policy.pipeline_version,
        FieldPolicyPipelineVersion::ProposalPipelineV1
    );
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    println!(
        "frontier_v1_0_happy: profile={} skeleton_id={FRONTIER_V1_SKELETON_ID} accepted=true",
        skeleton.profile_name
    );
}

#[test]
fn frontier_v1_0_rejects_default_on() {
    let mut skeleton = frontier_v1_happy_path_skeleton();
    skeleton.enabled_by_default = true;
    let report = validate_frontier_v1_admission(&skeleton);
    assert!(!report.accepted);
    assert!(!report.default_off_ok);

    let mut mapping_default_on = frontier_v1_happy_path_skeleton();
    mapping_default_on.enabled_by_default = true;
    mapping_default_on.mapping_execution_profile = MappingExecutionProfile::SparseRegionFieldV1;
    let report2 = validate_frontier_v1_admission(&mapping_default_on);
    assert!(!report2.accepted);

    let mut rf_default_on = frontier_v1_happy_path_skeleton();
    rf_default_on.enabled_by_default = true;
    rf_default_on.resource_flow_opt_in = simthing_spec::ResourceFlowOptInMode::FlatStarOptIn;
    let report3 = validate_frontier_v1_admission(&rf_default_on);
    assert!(!report3.accepted);

    println!("frontier_v1_0_default_on: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_rejects_out_of_bounds_mapping() {
    let cases: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 6] = [
        ("width_gt_32", Box::new(|s| s.theater.grid_width = 33)),
        ("height_gt_32", Box::new(|s| s.theater.grid_height = 40)),
        ("horizon_gt_8", Box::new(|s| s.theater.horizon = 9)),
        ("atlas", Box::new(|s| s.theater.request_atlas = true)),
        (
            "active_mask",
            Box::new(|s| s.theater.request_active_mask = true),
        ),
        (
            "perception",
            Box::new(|s| s.theater.request_perception = true),
        ),
    ];
    for (label, mutate) in cases {
        let mut skeleton = frontier_v1_happy_path_skeleton();
        mutate(&mut skeleton);
        let report = validate_frontier_v1_admission(&skeleton);
        assert!(!report.accepted, "{label} should reject");
        assert!(!report.mapping_ok, "{label} mapping should fail");
    }

    let mut source_id = frontier_v1_happy_path_skeleton();
    source_id.theater.request_source_identity = true;
    let report = validate_frontier_v1_admission(&source_id);
    assert!(!report.accepted);
    assert!(!report.mapping_ok);

    println!("frontier_v1_0_mapping_bounds: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_rejects_non_flat_star_resource_flow() {
    let cases: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 7] = [
        ("depth_gt_2", Box::new(|s| s.resource_flow.depth = 3)),
        (
            "children_gt_100",
            Box::new(|s| s.resource_flow.max_children_per_allocator = 101),
        ),
        (
            "nested_e11b",
            Box::new(|s| s.resource_flow.nested_e11b = true),
        ),
        (
            "e11b_5",
            Box::new(|s| s.resource_flow.e11b_5_dynamic_enrollment = true),
        ),
        (
            "d2a",
            Box::new(|s| s.resource_flow.d2a_hard_currency_ordering = true),
        ),
        (
            "shared_pool",
            Box::new(|s| s.resource_flow.shared_pool_tick_writes = true),
        ),
        (
            "parallel_fixture",
            Box::new(|s| s.resource_flow.parallel_fixture_economy = true),
        ),
    ];
    for (label, mutate) in cases {
        let mut skeleton = frontier_v1_happy_path_skeleton();
        mutate(&mut skeleton);
        let report = validate_frontier_v1_admission(&skeleton);
        assert!(!report.accepted, "{label} should reject");
        assert!(!report.flat_star_ok, "{label} flat_star should fail");
    }
    println!("frontier_v1_0_rf_scope: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_rejects_field_policy_routing_bypass() {
    let cases: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 7] = [
        (
            "no_allocator",
            Box::new(|s| s.field_policy.resource_dispatch_via_allocator = false),
        ),
        (
            "no_threshold_emit",
            Box::new(|s| s.field_policy.structural_via_threshold_emit = false),
        ),
        (
            "foreign_movement",
            Box::new(|s| s.field_policy.movement_own_columns_only = false),
        ),
        (
            "cpu_planner",
            Box::new(|s| s.field_policy.cpu_planner = true),
        ),
        (
            "cpu_urgency",
            Box::new(|s| s.field_policy.cpu_urgency = true),
        ),
        (
            "cpu_commitment",
            Box::new(|s| s.field_policy.cpu_commitment_emission = true),
        ),
        (
            "semantic_wgsl",
            Box::new(|s| s.field_policy.semantic_wgsl = true),
        ),
    ];
    for (label, mutate) in cases {
        let mut skeleton = frontier_v1_happy_path_skeleton();
        mutate(&mut skeleton);
        let report = validate_frontier_v1_admission(&skeleton);
        assert!(!report.accepted, "{label} should reject");
        assert!(
            !report.field_policy_v1_ok,
            "{label} field_policy should fail"
        );
    }
    println!(
        "frontier_v1_0_field_policy_routing: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}"
    );
}

#[test]
fn frontier_v1_0_coupling_scoped_to_frontier_only() {
    let frontier = frontier_v1_happy_path_skeleton();
    let frontier_report = validate_frontier_v1_admission(&frontier);
    assert!(frontier_report.coupling_ok);
    assert!(frontier.coupling.coupling_requested);
    assert!(!frontier.enabled_by_default);

    let mut other = frontier_v1_happy_path_skeleton();
    other.profile_name = "OtherProfile";
    other.coupling.coupling_requested = true;
    let other_report = validate_frontier_v1_admission(&other);
    assert!(!other_report.coupling_ok);
    assert!(!other_report.accepted);

    let mut frontier_no_coupling = frontier_v1_happy_path_skeleton();
    frontier_no_coupling.coupling.coupling_requested = false;
    let no_coupling_report = validate_frontier_v1_admission(&frontier_no_coupling);
    assert!(no_coupling_report.coupling_ok);
    assert!(no_coupling_report.accepted);

    println!(
        "frontier_v1_0_coupling_scope: frontier_only=true skeleton_id={FRONTIER_V1_SKELETON_ID}"
    );
}

#[test]
fn frontier_v1_0_no_simthing_sim_semantic_awareness() {
    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    let forbidden = [
        "FrontierV1",
        "FIELD_POLICY",
        "RegionCell",
        "ArenaRegistry",
        "proposal",
    ];
    for needle in forbidden {
        assert!(
            !sim_lib.contains(needle),
            "simthing-sim must not contain semantic marker `{needle}`"
        );
    }
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    println!("frontier_v1_0_sim: semantic_free=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_no_new_gpu_primitive() {
    let frontier_descriptor = landed_jit_kernel_descriptors()
        .into_iter()
        .find(|d| d.id.contains("frontier") || d.id.contains("FrontierV1"));
    assert!(
        frontier_descriptor.is_none(),
        "FrontierV1-0 must not add a landed kernel descriptor"
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
                "no new FrontierV1 WGSL file: {}",
                path.display()
            );
        }
    }
    assert_eq!(
        MappingExecutionProfile::default(),
        MappingExecutionProfile::Disabled
    );
    println!(
        "frontier_v1_0_gpu: new_wgsl=false new_descriptor=false skeleton_id={FRONTIER_V1_SKELETON_ID}"
    );
}
