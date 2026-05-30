//! FrontierV1-0 — default-off scenario skeleton and admission contract (Tier-2, test-only).
//!
//! Defines and validates the bounded FrontierV1 scenario envelope for M/E closure preparation.
//! No new WGSL, descriptor, AccumulatorRole, runtime wiring, or simthing-sim semantic awareness.

use simthing_spec::{
    landed_jit_kernel_descriptors, MappingExecutionProfile, RegionFieldCadenceSpec,
    RegionFieldOperatorSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

pub const FRONTIER_V1_PROFILE_NAME: &str = "FrontierV1";
pub const FRONTIER_V1_SKELETON_ID: &str = "frontier_v1_0_scenario_skeleton_v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeadPipelineVersion {
    ProposalPipelineV1,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierTheaterSpec {
    pub theater_count: u32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub operator: RegionFieldOperatorSpec,
    pub horizon: u32,
    pub cadence: RegionFieldCadenceSpec,
    pub request_atlas: bool,
    pub request_active_mask: bool,
    pub request_perception: bool,
    pub request_source_identity: bool,
    pub dirty_skip_allowed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierFactionSpec {
    pub name: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierFlatStarResourceFlowSpec {
    pub depth: u32,
    pub max_children_per_allocator: u32,
    pub nested_e11b: bool,
    pub e11b_5_dynamic_enrollment: bool,
    pub d2a_hard_currency_ordering: bool,
    pub shared_pool_tick_writes: bool,
    pub parallel_fixture_economy: bool,
    pub orderband_sweeps_only: bool,
    pub resource_flow_allocator_only: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierSeadSelfAiSpec {
    pub pipeline_version: SeadPipelineVersion,
    pub exact_f_magnitude_only: bool,
    pub resource_dispatch_via_allocator: bool,
    pub structural_via_threshold_emit: bool,
    pub movement_own_columns_only: bool,
    pub cpu_planner: bool,
    pub cpu_urgency: bool,
    pub cpu_commitment_emission: bool,
    pub semantic_wgsl: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierEconomyFieldCouplingSpec {
    pub district_seeds_supply_field: bool,
    pub field_proposals_dispatch_via_allocator: bool,
    pub coupling_requested: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1ScenarioSkeleton {
    pub profile_name: &'static str,
    pub enabled_by_default: bool,
    pub mapping_execution_profile: MappingExecutionProfile,
    pub resource_flow_opt_in: ResourceFlowOptInMode,
    pub resource_flow_execution_profile: ResourceFlowExecutionProfile,
    pub theater: FrontierTheaterSpec,
    pub factions: [FrontierFactionSpec; 2],
    pub resource_flow: FrontierFlatStarResourceFlowSpec,
    pub sead: FrontierSeadSelfAiSpec,
    pub coupling: FrontierEconomyFieldCouplingSpec,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrontierV1AdmissionReport {
    pub accepted: bool,
    pub mapping_ok: bool,
    pub flat_star_ok: bool,
    pub sead_v1_ok: bool,
    pub coupling_ok: bool,
    pub default_off_ok: bool,
    pub rejected_reasons: Vec<&'static str>,
}

pub fn frontier_v1_happy_path_skeleton() -> FrontierV1ScenarioSkeleton {
    FrontierV1ScenarioSkeleton {
        profile_name: FRONTIER_V1_PROFILE_NAME,
        enabled_by_default: false,
        mapping_execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
        resource_flow_opt_in: ResourceFlowOptInMode::FlatStarOptIn,
        resource_flow_execution_profile: ResourceFlowExecutionProfile::FlatStarResourceFlow,
        theater: FrontierTheaterSpec {
            theater_count: 1,
            grid_width: 32,
            grid_height: 32,
            operator: RegionFieldOperatorSpec::SourceCappedNormalized,
            horizon: 8,
            cadence: RegionFieldCadenceSpec::EveryTick,
            request_atlas: false,
            request_active_mask: false,
            request_perception: false,
            request_source_identity: false,
            dirty_skip_allowed: true,
        },
        factions: [
            FrontierFactionSpec { name: "faction_a" },
            FrontierFactionSpec { name: "faction_b" },
        ],
        resource_flow: FrontierFlatStarResourceFlowSpec {
            depth: 2,
            max_children_per_allocator: 100,
            nested_e11b: false,
            e11b_5_dynamic_enrollment: false,
            d2a_hard_currency_ordering: false,
            shared_pool_tick_writes: false,
            parallel_fixture_economy: false,
            orderband_sweeps_only: true,
            resource_flow_allocator_only: true,
        },
        sead: FrontierSeadSelfAiSpec {
            pipeline_version: SeadPipelineVersion::ProposalPipelineV1,
            exact_f_magnitude_only: true,
            resource_dispatch_via_allocator: true,
            structural_via_threshold_emit: true,
            movement_own_columns_only: true,
            cpu_planner: false,
            cpu_urgency: false,
            cpu_commitment_emission: false,
            semantic_wgsl: false,
        },
        coupling: FrontierEconomyFieldCouplingSpec {
            district_seeds_supply_field: true,
            field_proposals_dispatch_via_allocator: true,
            coupling_requested: true,
        },
    }
}

pub fn validate_frontier_v1_admission(
    skeleton: &FrontierV1ScenarioSkeleton,
) -> FrontierV1AdmissionReport {
    let mut rejected = Vec::new();

    let default_off_ok = validate_default_off(skeleton, &mut rejected);
    let mapping_ok = validate_mapping(skeleton, &mut rejected);
    let flat_star_ok = validate_flat_star(skeleton, &mut rejected);
    let sead_v1_ok = validate_sead_routing(skeleton, &mut rejected);
    let coupling_ok = validate_coupling(skeleton, &mut rejected);

    let accepted = default_off_ok && mapping_ok && flat_star_ok && sead_v1_ok && coupling_ok;

    FrontierV1AdmissionReport {
        accepted,
        mapping_ok,
        flat_star_ok,
        sead_v1_ok,
        coupling_ok,
        default_off_ok,
        rejected_reasons: rejected,
    }
}

fn validate_default_off(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    let mut ok = true;
    if skeleton.enabled_by_default {
        rejected.push("profile must not be enabled by default");
        ok = false;
    }
    if skeleton.profile_name != FRONTIER_V1_PROFILE_NAME {
        rejected.push("profile_name must be FrontierV1");
        ok = false;
    }
    if skeleton.enabled_by_default
        && skeleton.mapping_execution_profile != MappingExecutionProfile::Disabled
    {
        rejected.push("mapping execution profile must not default-on");
        ok = false;
    }
    if skeleton.enabled_by_default && skeleton.resource_flow_opt_in != ResourceFlowOptInMode::Disabled {
        rejected.push("resource flow must not default-on");
        ok = false;
    }
    if skeleton.enabled_by_default
        && skeleton
            .resource_flow_execution_profile
            .enables_flat_star_resource_flow()
    {
        rejected.push("resource flow execution profile must not default-on");
        ok = false;
    }
    ok
}

fn validate_mapping(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    let t = skeleton.theater;
    let mut ok = true;
    if t.theater_count != 1 {
        rejected.push("exactly one theater required");
        ok = false;
    }
    if t.grid_width == 0 || t.grid_height == 0 {
        rejected.push("grid dimensions must be positive");
        ok = false;
    }
    if t.grid_width > 32 {
        rejected.push("grid width exceeds 32");
        ok = false;
    }
    if t.grid_height > 32 {
        rejected.push("grid height exceeds 32");
        ok = false;
    }
    if t.operator != RegionFieldOperatorSpec::SourceCappedNormalized {
        rejected.push("operator must be source_capped_normalized");
        ok = false;
    }
    if t.horizon == 0 || t.horizon > 8 {
        rejected.push("horizon must be 1..=8");
        ok = false;
    }
    if !matches!(
        t.cadence,
        RegionFieldCadenceSpec::EveryTick | RegionFieldCadenceSpec::EveryN(_)
    ) {
        rejected.push("cadence must be EveryTick or explicit bounded EveryN");
        ok = false;
    }
    if t.request_atlas {
        rejected.push("atlas not allowed");
        ok = false;
    }
    if t.request_active_mask {
        rejected.push("active mask not allowed");
        ok = false;
    }
    if t.request_perception {
        rejected.push("perception/fog not allowed");
        ok = false;
    }
    if t.request_source_identity {
        rejected.push("source identity/source_mask not allowed");
        ok = false;
    }
    ok
}

fn validate_flat_star(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    let rf = skeleton.resource_flow;
    let mut ok = true;
    if skeleton.factions.len() != 2 {
        rejected.push("exactly two factions required");
        ok = false;
    }
    if rf.depth == 0 || rf.depth > 2 {
        rejected.push("flat-star depth must be 1..=2");
        ok = false;
    }
    if rf.max_children_per_allocator == 0 || rf.max_children_per_allocator > 100 {
        rejected.push("children per allocator must be 1..=100");
        ok = false;
    }
    if rf.nested_e11b {
        rejected.push("nested E-11B not allowed");
        ok = false;
    }
    if rf.e11b_5_dynamic_enrollment {
        rejected.push("E-11B-5 dynamic enrollment not allowed");
        ok = false;
    }
    if rf.d2a_hard_currency_ordering {
        rejected.push("D-2a hard-currency ordering not allowed");
        ok = false;
    }
    if rf.shared_pool_tick_writes {
        rejected.push("shared-pool tick-time writes not allowed");
        ok = false;
    }
    if rf.parallel_fixture_economy {
        rejected.push("parallel fixture economy not allowed");
        ok = false;
    }
    if !rf.orderband_sweeps_only {
        rejected.push("OrderBand sweeps only");
        ok = false;
    }
    if !rf.resource_flow_allocator_only {
        rejected.push("Resource Flow allocator routing required");
        ok = false;
    }
    if skeleton.resource_flow_opt_in != ResourceFlowOptInMode::FlatStarOptIn {
        rejected.push("resource flow requires explicit FlatStarOptIn when selected");
        ok = false;
    }
    ok
}

fn validate_sead_routing(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    let s = skeleton.sead;
    let mut ok = true;
    if s.pipeline_version != SeadPipelineVersion::ProposalPipelineV1 {
        rejected.push("SEAD Self-AI Proposal Pipeline V1 only");
        ok = false;
    }
    if !s.exact_f_magnitude_only {
        rejected.push("exact F-backed magnitude path only");
        ok = false;
    }
    if !s.resource_dispatch_via_allocator {
        rejected.push("resource dispatch must route through Resource Flow allocator");
        ok = false;
    }
    if !s.structural_via_threshold_emit {
        rejected.push("structural commitments must route through Threshold+EmitEvent");
        ok = false;
    }
    if !s.movement_own_columns_only {
        rejected.push("movement must write unit own columns only");
        ok = false;
    }
    if s.cpu_planner {
        rejected.push("CPU planner not allowed");
        ok = false;
    }
    if s.cpu_urgency {
        rejected.push("CPU urgency computation not allowed");
        ok = false;
    }
    if s.cpu_commitment_emission {
        rejected.push("CPU commitment emission not allowed");
        ok = false;
    }
    if s.semantic_wgsl {
        rejected.push("semantic WGSL not allowed");
        ok = false;
    }
    ok
}

fn validate_coupling(skeleton: &FrontierV1ScenarioSkeleton, rejected: &mut Vec<&'static str>) -> bool {
    let c = skeleton.coupling;
    let mut ok = true;
    if skeleton.profile_name != FRONTIER_V1_PROFILE_NAME {
        if c.coupling_requested {
            rejected.push("economy↔field coupling allowed only for FrontierV1 profile");
            ok = false;
        }
        return ok;
    }
    if c.coupling_requested {
        if !c.district_seeds_supply_field {
            rejected.push("district output must seed supply field when coupling requested");
            ok = false;
        }
        if !c.field_proposals_dispatch_via_allocator {
            rejected.push("field proposals must dispatch via Resource Flow allocator");
            ok = false;
        }
        if skeleton.enabled_by_default {
            rejected.push("coupling must remain default-off");
            ok = false;
        }
    }
    ok
}

#[test]
fn frontier_v1_0_happy_path_skeleton_admits() {
    let skeleton = frontier_v1_happy_path_skeleton();
    let report = validate_frontier_v1_admission(&skeleton);
    assert!(report.accepted, "rejected: {:?}", report.rejected_reasons);
    assert!(report.default_off_ok);
    assert!(report.mapping_ok);
    assert!(report.flat_star_ok);
    assert!(report.sead_v1_ok);
    assert!(report.coupling_ok);
    assert_eq!(skeleton.profile_name, FRONTIER_V1_PROFILE_NAME);
    assert!(!skeleton.enabled_by_default);
    assert_eq!(skeleton.theater.grid_width, 32);
    assert_eq!(skeleton.theater.grid_height, 32);
    assert_eq!(
        skeleton.theater.operator,
        RegionFieldOperatorSpec::SourceCappedNormalized
    );
    assert_eq!(skeleton.theater.horizon, 8);
    assert_eq!(skeleton.resource_flow.depth, 2);
    assert!(skeleton.resource_flow.resource_flow_allocator_only);
    assert_eq!(skeleton.sead.pipeline_version, SeadPipelineVersion::ProposalPipelineV1);
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
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
    rf_default_on.resource_flow_opt_in = ResourceFlowOptInMode::FlatStarOptIn;
    let report3 = validate_frontier_v1_admission(&rf_default_on);
    assert!(!report3.accepted);

    println!("frontier_v1_0_default_on: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_rejects_out_of_bounds_mapping() {
    let cases: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 6] = [
        (
            "width_gt_32",
            Box::new(|s| s.theater.grid_width = 33),
        ),
        (
            "height_gt_32",
            Box::new(|s| s.theater.grid_height = 40),
        ),
        (
            "horizon_gt_8",
            Box::new(|s| s.theater.horizon = 9),
        ),
        (
            "atlas",
            Box::new(|s| s.theater.request_atlas = true),
        ),
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
fn frontier_v1_0_rejects_sead_routing_bypass() {
    let cases: [(&str, Box<dyn Fn(&mut FrontierV1ScenarioSkeleton)>); 7] = [
        (
            "no_allocator",
            Box::new(|s| s.sead.resource_dispatch_via_allocator = false),
        ),
        (
            "no_threshold_emit",
            Box::new(|s| s.sead.structural_via_threshold_emit = false),
        ),
        (
            "foreign_movement",
            Box::new(|s| s.sead.movement_own_columns_only = false),
        ),
        ("cpu_planner", Box::new(|s| s.sead.cpu_planner = true)),
        ("cpu_urgency", Box::new(|s| s.sead.cpu_urgency = true)),
        (
            "cpu_commitment",
            Box::new(|s| s.sead.cpu_commitment_emission = true),
        ),
        ("semantic_wgsl", Box::new(|s| s.sead.semantic_wgsl = true)),
    ];
    for (label, mutate) in cases {
        let mut skeleton = frontier_v1_happy_path_skeleton();
        mutate(&mut skeleton);
        let report = validate_frontier_v1_admission(&skeleton);
        assert!(!report.accepted, "{label} should reject");
        assert!(!report.sead_v1_ok, "{label} sead should fail");
    }
    println!("frontier_v1_0_sead_routing: rejects=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
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

    println!("frontier_v1_0_coupling_scope: frontier_only=true skeleton_id={FRONTIER_V1_SKELETON_ID}");
}

#[test]
fn frontier_v1_0_no_simthing_sim_semantic_awareness() {
    let sim_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../simthing-sim/src/lib.rs"
    ));
    let forbidden = [
        "FrontierV1",
        "SEAD",
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
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
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
    assert_eq!(MappingExecutionProfile::default(), MappingExecutionProfile::Disabled);
    println!(
        "frontier_v1_0_gpu: new_wgsl=false new_descriptor=false skeleton_id={FRONTIER_V1_SKELETON_ID}"
    );
}
