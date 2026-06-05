//! FrontierV1 scenario skeleton, admission validator, and opt-in fixture CPU oracle (test-only).

use simthing_spec::{
    MappingExecutionProfile, RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec,
    RegionFieldGridProfile, RegionFieldOperatorSpec, RegionFieldReductionSpec,
    RegionFieldSourcePolicySpec, RegionFieldSpec, RegionFieldSummaryPolicySpec,
    ResourceFlowExecutionProfile, ResourceFlowOptInMode,
};

pub const FRONTIER_V1_PROFILE_NAME: &str = "FrontierV1";
pub const FRONTIER_V1_SKELETON_ID: &str = "frontier_v1_0_scenario_skeleton_v1";
pub const FRONTIER_V1_FIXTURE_ID: &str = "frontier_v1_1_opt_in_fixture_v1";
pub const FRONTIER_V1_GPU_FIXTURE_ID: &str = "frontier_v1_2_gpu_replay_acceptance_v1";
pub const FRONTIER_V1_GPU_RF_FIXTURE_ID: &str = "frontier_v1_3_gpu_resource_flow_v1";
pub const FRONTIER_V1_FIELD_POLICY_ROUTE_FIXTURE_ID: &str =
    "frontier_v1_4_field_policy_route_replay_v1";
pub const FRONTIER_V1_LIVE_FIELD_AGENT_FIXTURE_ID: &str = "frontier_v1_5_live_field_agent_route_v1";

/// Named multi-tick closed-loop consumer profile (FrontierV2-0 fixture).
pub const FRONTIER_V2_PROFILE_NAME: &str = "FrontierV2";

/// FIELD_POLICY ACT-2 `proposal_code` for resource-dispatch (event_code 1 max-rule emission).
pub const FRONTIER_V1_RESOURCE_PROPOSAL_CODE: u32 = 1001;
/// FIELD_POLICY event bucket code for resource-side threshold events.
pub const FRONTIER_V1_RESOURCE_EVENT_CODE: u32 = 1;

/// Accepted Resource Flow allocator route code for FrontierV1 resource dispatch.
pub const FRONTIER_V1_ALLOCATOR_ROUTE_CODE: u32 = 1;
/// Accepted structural route code (Threshold+EmitEvent → BoundaryRequest).
pub const FRONTIER_V1_STRUCTURAL_ROUTE_CODE: u32 = 2;
/// Accepted movement route code (own-column-only writes).
pub const FRONTIER_V1_MOVEMENT_ROUTE_CODE: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPolicyPipelineVersion {
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
pub struct FrontierFieldPolicyFieldAgentSpec {
    pub pipeline_version: FieldPolicyPipelineVersion,
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
    pub field_policy: FrontierFieldPolicyFieldAgentSpec,
    pub coupling: FrontierEconomyFieldCouplingSpec,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrontierV1AdmissionReport {
    pub accepted: bool,
    pub mapping_ok: bool,
    pub flat_star_ok: bool,
    pub field_policy_v1_ok: bool,
    pub coupling_ok: bool,
    pub default_off_ok: bool,
    pub rejected_reasons: Vec<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProposalKind {
    ResourceDispatch,
    StructuralCommit,
    Movement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProposalRoute {
    ResourceFlowAllocator,
    ThresholdEmitBoundaryRequest,
    OwnColumnsOnly,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1FixtureConfig {
    pub seed: u64,
    pub grid_size: u32,
    pub source_cap: u32,
    pub horizon: u32,
    pub district_output_a: u32,
    pub district_output_b: u32,
    pub proposals: [ProposalKind; 3],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MappingSummary {
    pub cell_sum: u32,
    pub cell_count: u32,
    pub overflow: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceFlowSummary {
    pub allocated_a: u32,
    pub allocated_b: u32,
    pub overflow: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteSummary {
    pub resource_route_count: u32,
    pub structural_route_count: u32,
    pub movement_route_count: u32,
    pub rejected_route_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1FixtureFingerprint {
    pub mapping_summary_hash: u64,
    pub resource_flow_summary_hash: u64,
    pub proposal_summary_hash: u64,
    pub route_summary_hash: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV1FieldStatus {
    GpuVerified,
    CpuOracleOnly,
    ReplayAccepted,
    PendingGpu,
}

/// Named consumer scenario status (FrontierV2 not implemented in V1-5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV2Status {
    NotImplemented,
}

/// Honest per-field status for FrontierV1-5 live field agent route reporting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontierV1LiveFieldAgentFieldStatus {
    GpuVerified,
    PartialGpuVerified,
    ReplayAccepted,
    CpuOracleOnly,
    FixtureOnly,
    OutOfScopeForV1_5,
    Pending,
}

/// Fixture-only next-tick feedback payload shape for the named FrontierV2 consumer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1LiveFieldAgentFeedbackCandidate {
    pub tick_index: u32,
    pub source_unit_id: u32,
    pub route_code: u32,
    pub proposal_code: u32,
    pub dispatch_count: u32,
    pub allocator_total: u32,
    pub faction_a_allocation: u32,
    pub faction_b_allocation: u32,
    pub field_feedback_code: u32,
    pub overflow_flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1LiveFieldAgentSummary {
    pub mapping_summary_hash: u64,
    pub resource_flow_summary_hash: u64,
    pub field_agent_summary_hash: u64,
    pub feedback_candidate_hash: u64,
    pub route_summary_hash: u64,
    pub overflow_flags: u32,
    pub mapping_status: FrontierV1LiveFieldAgentFieldStatus,
    pub resource_flow_status: FrontierV1LiveFieldAgentFieldStatus,
    pub field_agent_resource_route_status: FrontierV1LiveFieldAgentFieldStatus,
    pub feedback_candidate_status: FrontierV1LiveFieldAgentFieldStatus,
    pub full_field_policy_pipe_status: FrontierV1LiveFieldAgentFieldStatus,
    pub structural_route_status: FrontierV1LiveFieldAgentFieldStatus,
    pub movement_route_status: FrontierV1LiveFieldAgentFieldStatus,
    pub frontier_v2_status: FrontierV2Status,
}

impl FrontierV1LiveFieldAgentSummary {
    pub fn combined_hex(&self) -> String {
        let combined = fnv_mix(self.mapping_summary_hash)
            ^ fnv_mix(self.resource_flow_summary_hash)
            ^ fnv_mix(self.field_agent_summary_hash)
            ^ fnv_mix(self.feedback_candidate_hash)
            ^ fnv_mix(self.route_summary_hash)
            ^ fnv_mix(u64::from(self.overflow_flags));
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1LiveFieldAgentOracleOutput {
    pub resource_route_code: u32,
    pub resource_route_count: u32,
    pub invalid_route_count: u32,
    pub allocator_total: u32,
    pub faction_a_allocation: u32,
    pub faction_b_allocation: u32,
    pub feedback: FrontierV1LiveFieldAgentFeedbackCandidate,
    pub overflow_flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1GpuReplaySummary {
    pub mapping_summary_hash: u64,
    pub resource_flow_summary_hash: u64,
    pub field_policy_summary_hash: u64,
    pub proposal_summary_hash: u64,
    pub route_summary_hash: u64,
    pub overflow_flags: u32,
    pub accepted: bool,
    pub mapping_status: FrontierV1FieldStatus,
    pub resource_flow_status: FrontierV1FieldStatus,
    pub field_policy_routing_status: FrontierV1FieldStatus,
    pub field_policy_pipe_status: FrontierV1FieldStatus,
    pub route_status: FrontierV1FieldStatus,
    pub gpu_reduction_eml_executed: bool,
}

impl FrontierV1GpuReplaySummary {
    pub fn combined_hex(&self) -> String {
        let combined = if self.field_policy_summary_hash == 0 {
            fnv_mix(self.mapping_summary_hash)
                ^ fnv_mix(self.resource_flow_summary_hash)
                ^ fnv_mix(self.proposal_summary_hash)
                ^ fnv_mix(self.route_summary_hash)
                ^ fnv_mix(u64::from(self.overflow_flags))
        } else {
            fnv_mix(self.mapping_summary_hash)
                ^ fnv_mix(self.resource_flow_summary_hash)
                ^ fnv_mix(self.field_policy_summary_hash)
                ^ fnv_mix(self.proposal_summary_hash)
                ^ fnv_mix(self.route_summary_hash)
                ^ fnv_mix(u64::from(self.overflow_flags))
        };
        format!("{:016x}", combined & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1RouteReplaySummary {
    pub resource_route_code: u32,
    pub structural_route_code: u32,
    pub movement_route_code: u32,
    pub route_overflow_flags: u32,
    pub invalid_route_count: u32,
    pub resource_route_count: u32,
    pub structural_route_count: u32,
    pub movement_route_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1FieldPolicyReplaySummary {
    pub event_count: u32,
    pub proposal_count: u32,
    pub admission_count: u32,
    pub field_policy_overflow_flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuResourceFlowAllocationSummary {
    pub faction_a_allocation: u32,
    pub faction_b_allocation: u32,
    pub allocator_total: u32,
    pub resource_overflow_flags: u32,
    pub allocator_route_code: u32,
}

impl FrontierV1FixtureFingerprint {
    pub fn combined(&self) -> u64 {
        fnv_mix(self.mapping_summary_hash)
            ^ fnv_mix(self.resource_flow_summary_hash)
            ^ fnv_mix(self.proposal_summary_hash)
            ^ fnv_mix(self.route_summary_hash)
    }

    pub fn hex(&self) -> String {
        format!("{:016x}", self.combined() & 0xFFFF_FFFF_FFFF_FFFF)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrontierV1FixtureOutput {
    pub admission_accepted: bool,
    pub mapping: MappingSummary,
    pub resource_flow: ResourceFlowSummary,
    pub proposal_count: u32,
    pub event_count: u32,
    pub routes: RouteSummary,
    pub fingerprint: FrontierV1FixtureFingerprint,
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
        field_policy: FrontierFieldPolicyFieldAgentSpec {
            pipeline_version: FieldPolicyPipelineVersion::ProposalPipelineV1,
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

pub fn frontier_v1_1_smoke_skeleton() -> FrontierV1ScenarioSkeleton {
    let mut skeleton = frontier_v1_happy_path_skeleton();
    skeleton.theater.grid_width = 8;
    skeleton.theater.grid_height = 8;
    skeleton
}

pub fn frontier_v1_1_fixture_config() -> FrontierV1FixtureConfig {
    FrontierV1FixtureConfig {
        seed: 0x6672_6F6E_7469_6572,
        grid_size: 8,
        source_cap: 500,
        horizon: 8,
        district_output_a: 120,
        district_output_b: 80,
        proposals: [
            ProposalKind::ResourceDispatch,
            ProposalKind::StructuralCommit,
            ProposalKind::Movement,
        ],
    }
}

/// Admitted 8×8 first-slice RegionCell field for FrontierV1 GPU mapping execution.
pub fn frontier_v1_mapping_field_spec() -> RegionFieldSpec {
    RegionFieldSpec {
        name: "frontier_v1_theater".into(),
        grid_size: 8,
        n_dims: 8,
        source_col: 0,
        target_col: 0,
        operator: RegionFieldOperatorSpec::SourceCappedNormalized,
        horizon: 8,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.8,
        source_cap: Some(500.0),
        source_policy: RegionFieldSourcePolicySpec::default(),
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(RegionFieldReductionSpec {
            child_slot_start: 0,
            child_slot_count: 64,
            child_col: 0,
            parent_slot: 100,
            parent_col: 0,
            order_band: 0,
        }),
        parent_formula: Some(RegionFieldFormulaBindingSpec {
            formula_class: "field_urgency".into(),
            tree_id: Some(7),
        }),
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: Some(65536),
        summary_policy: RegionFieldSummaryPolicySpec::default(),
    }
}

pub fn hash_gpu_field_values(values: &[f32]) -> u64 {
    let mut h = fnv64(b"gpu_field");
    for v in values {
        h = fnv_append_u32(h, v.to_bits());
    }
    h
}

pub fn hash_gpu_resource_flow(summary: GpuResourceFlowAllocationSummary) -> u64 {
    let mut h = fnv64(b"gpu_resource_flow");
    h = fnv_append_u32(h, summary.faction_a_allocation);
    h = fnv_append_u32(h, summary.faction_b_allocation);
    h = fnv_append_u32(h, summary.allocator_total);
    h = fnv_append_u32(h, summary.resource_overflow_flags);
    h = fnv_append_u32(h, summary.allocator_route_code);
    h
}

pub fn frontier_v1_flat_star_weights() -> (f32, f32) {
    (3.0, 2.0)
}

pub fn proposal_route_to_code(route: ProposalRoute) -> Option<u32> {
    match route {
        ProposalRoute::ResourceFlowAllocator => Some(FRONTIER_V1_ALLOCATOR_ROUTE_CODE),
        ProposalRoute::ThresholdEmitBoundaryRequest => Some(FRONTIER_V1_STRUCTURAL_ROUTE_CODE),
        ProposalRoute::OwnColumnsOnly => Some(FRONTIER_V1_MOVEMENT_ROUTE_CODE),
        ProposalRoute::Rejected => None,
    }
}

pub fn build_route_replay_summary(
    config: &FrontierV1FixtureConfig,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> FrontierV1RouteReplaySummary {
    let mut resource_route_count = 0u32;
    let mut structural_route_count = 0u32;
    let mut movement_route_count = 0u32;
    let mut invalid_route_count = 0u32;
    for kind in config.proposals {
        match classify_proposal_route(kind, skeleton) {
            ProposalRoute::ResourceFlowAllocator => resource_route_count += 1,
            ProposalRoute::ThresholdEmitBoundaryRequest => structural_route_count += 1,
            ProposalRoute::OwnColumnsOnly => movement_route_count += 1,
            ProposalRoute::Rejected => invalid_route_count += 1,
        }
    }
    FrontierV1RouteReplaySummary {
        resource_route_code: FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
        structural_route_code: FRONTIER_V1_STRUCTURAL_ROUTE_CODE,
        movement_route_code: FRONTIER_V1_MOVEMENT_ROUTE_CODE,
        route_overflow_flags: 0,
        invalid_route_count,
        resource_route_count,
        structural_route_count,
        movement_route_count,
    }
}

pub fn build_field_policy_replay_summary(
    cpu_output: &FrontierV1FixtureOutput,
) -> FrontierV1FieldPolicyReplaySummary {
    FrontierV1FieldPolicyReplaySummary {
        event_count: cpu_output.event_count,
        proposal_count: cpu_output.proposal_count,
        admission_count: cpu_output.routes.structural_route_count,
        field_policy_overflow_flags: 0,
    }
}

pub fn hash_route_replay_detail(summary: FrontierV1RouteReplaySummary) -> u64 {
    let mut h = fnv64(b"route_replay");
    h = fnv_append_u32(h, summary.resource_route_code);
    h = fnv_append_u32(h, summary.structural_route_code);
    h = fnv_append_u32(h, summary.movement_route_code);
    h = fnv_append_u32(h, summary.route_overflow_flags);
    h = fnv_append_u32(h, summary.invalid_route_count);
    h = fnv_append_u32(h, summary.resource_route_count);
    h = fnv_append_u32(h, summary.structural_route_count);
    h = fnv_append_u32(h, summary.movement_route_count);
    h
}

pub fn hash_live_field_agent_gpu_execution(
    event_count: u32,
    pipe_overflow: u32,
    proposal_count: u32,
    proposal_overflow: u32,
    admission_code: u32,
    admission_flags: u32,
) -> u64 {
    let mut h = fnv64(b"frontier_v1_5_field_agent_gpu");
    h = fnv_append_u32(h, event_count);
    h = fnv_append_u32(h, pipe_overflow);
    h = fnv_append_u32(h, proposal_count);
    h = fnv_append_u32(h, proposal_overflow);
    h = fnv_append_u32(h, admission_code);
    h = fnv_append_u32(h, admission_flags);
    h
}

pub fn hash_live_field_agent_feedback_candidate(
    c: FrontierV1LiveFieldAgentFeedbackCandidate,
) -> u64 {
    let mut h = fnv64(b"frontier_v1_5_feedback");
    h = fnv_append_u32(h, c.tick_index);
    h = fnv_append_u32(h, c.source_unit_id);
    h = fnv_append_u32(h, c.route_code);
    h = fnv_append_u32(h, c.proposal_code);
    h = fnv_append_u32(h, c.dispatch_count);
    h = fnv_append_u32(h, c.allocator_total);
    h = fnv_append_u32(h, c.faction_a_allocation);
    h = fnv_append_u32(h, c.faction_b_allocation);
    h = fnv_append_u32(h, c.field_feedback_code);
    h = fnv_append_u32(h, c.overflow_flags);
    h
}

pub fn hash_live_field_agent_summary(summary: FrontierV1LiveFieldAgentSummary) -> u64 {
    let mut h = fnv64(b"frontier_v1_5_live_field_agent");
    h = fnv_append_u32(h, summary.mapping_summary_hash as u32);
    h = fnv_append_u32(h, (summary.mapping_summary_hash >> 32) as u32);
    h = fnv_append_u32(h, summary.resource_flow_summary_hash as u32);
    h = fnv_append_u32(h, (summary.resource_flow_summary_hash >> 32) as u32);
    h = fnv_append_u32(h, summary.field_agent_summary_hash as u32);
    h = fnv_append_u32(h, (summary.field_agent_summary_hash >> 32) as u32);
    h = fnv_append_u32(h, summary.feedback_candidate_hash as u32);
    h = fnv_append_u32(h, (summary.feedback_candidate_hash >> 32) as u32);
    h = fnv_append_u32(h, summary.route_summary_hash as u32);
    h = fnv_append_u32(h, (summary.route_summary_hash >> 32) as u32);
    h = fnv_append_u32(h, summary.overflow_flags);
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.mapping_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.resource_flow_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.field_agent_resource_route_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.feedback_candidate_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.full_field_policy_pipe_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.structural_route_status),
    );
    h = fnv_append_u32(
        h,
        live_field_agent_field_status_code(summary.movement_route_status),
    );
    h = fnv_append_u32(
        h,
        u32::from(summary.frontier_v2_status == FrontierV2Status::NotImplemented),
    );
    h
}

pub fn build_feedback_candidate(
    tick_index: u32,
    source_unit_id: u32,
    route_code: u32,
    proposal_code: u32,
    dispatch_count: u32,
    gpu_rf: GpuResourceFlowAllocationSummary,
    field_feedback_code: u32,
    overflow_flags: u32,
) -> FrontierV1LiveFieldAgentFeedbackCandidate {
    FrontierV1LiveFieldAgentFeedbackCandidate {
        tick_index,
        source_unit_id,
        route_code,
        proposal_code,
        dispatch_count,
        allocator_total: gpu_rf.allocator_total,
        faction_a_allocation: gpu_rf.faction_a_allocation,
        faction_b_allocation: gpu_rf.faction_b_allocation,
        field_feedback_code,
        overflow_flags,
    }
}

pub fn cpu_live_field_agent_oracle(
    skeleton: &FrontierV1ScenarioSkeleton,
    config: &FrontierV1FixtureConfig,
    tick_index: u32,
    source_unit_id: u32,
    dispatch_count: u32,
    field_feedback_code: u32,
) -> FrontierV1LiveFieldAgentOracleOutput {
    let cpu = run_frontier_v1_fixture(skeleton, config);
    let gpu_rf =
        gpu_resource_flow_from_cpu_oracle(cpu.resource_flow, FRONTIER_V1_ALLOCATOR_ROUTE_CODE);
    let mut overflow_flags = 0u32;
    if cpu.mapping.overflow {
        overflow_flags |= 1;
    }
    if cpu.resource_flow.overflow {
        overflow_flags |= 2;
    }
    if gpu_rf.resource_overflow_flags != 0 {
        overflow_flags |= 4;
    }
    let feedback = build_feedback_candidate(
        tick_index,
        source_unit_id,
        FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
        FRONTIER_V1_RESOURCE_PROPOSAL_CODE,
        dispatch_count,
        gpu_rf,
        field_feedback_code,
        overflow_flags,
    );
    FrontierV1LiveFieldAgentOracleOutput {
        resource_route_code: FRONTIER_V1_ALLOCATOR_ROUTE_CODE,
        resource_route_count: cpu.routes.resource_route_count,
        invalid_route_count: cpu.routes.rejected_route_count,
        allocator_total: gpu_rf.allocator_total,
        faction_a_allocation: gpu_rf.faction_a_allocation,
        faction_b_allocation: gpu_rf.faction_b_allocation,
        feedback,
        overflow_flags,
    }
}

pub fn hash_field_policy_replay_summary(summary: FrontierV1FieldPolicyReplaySummary) -> u64 {
    let mut h = fnv64(b"field_policy_replay");
    h = fnv_append_u32(h, summary.event_count);
    h = fnv_append_u32(h, summary.proposal_count);
    h = fnv_append_u32(h, summary.admission_count);
    h = fnv_append_u32(h, summary.field_policy_overflow_flags);
    h
}

pub fn build_gpu_replay_summary(
    mapping_summary_hash: u64,
    cpu_output: &FrontierV1FixtureOutput,
    gpu_reduction_eml_executed: bool,
) -> FrontierV1GpuReplaySummary {
    let mut summary = build_frontier_v1_4_replay_summary(
        mapping_summary_hash,
        cpu_output.fingerprint.resource_flow_summary_hash,
        0,
        cpu_output,
        FrontierV1FieldStatus::CpuOracleOnly,
        if gpu_reduction_eml_executed {
            FrontierV1FieldStatus::GpuVerified
        } else {
            FrontierV1FieldStatus::CpuOracleOnly
        },
        FrontierV1FieldStatus::CpuOracleOnly,
        FrontierV1FieldStatus::CpuOracleOnly,
        gpu_reduction_eml_executed,
    );
    summary.field_policy_summary_hash = 0;
    summary
}

pub fn build_gpu_replay_summary_with_rf(
    mapping_summary_hash: u64,
    resource_flow_summary_hash: u64,
    cpu_output: &FrontierV1FixtureOutput,
    resource_flow_status: FrontierV1FieldStatus,
    gpu_reduction_eml_executed: bool,
) -> FrontierV1GpuReplaySummary {
    let mut summary = build_frontier_v1_4_replay_summary(
        mapping_summary_hash,
        resource_flow_summary_hash,
        0,
        cpu_output,
        resource_flow_status,
        if gpu_reduction_eml_executed {
            FrontierV1FieldStatus::GpuVerified
        } else {
            FrontierV1FieldStatus::CpuOracleOnly
        },
        FrontierV1FieldStatus::CpuOracleOnly,
        FrontierV1FieldStatus::CpuOracleOnly,
        gpu_reduction_eml_executed,
    );
    summary.field_policy_summary_hash = 0;
    summary
}

pub fn build_frontier_v1_4_replay_summary(
    mapping_summary_hash: u64,
    resource_flow_summary_hash: u64,
    field_policy_summary_hash: u64,
    cpu_output: &FrontierV1FixtureOutput,
    resource_flow_status: FrontierV1FieldStatus,
    field_policy_routing_status: FrontierV1FieldStatus,
    field_policy_pipe_status: FrontierV1FieldStatus,
    route_status: FrontierV1FieldStatus,
    gpu_reduction_eml_executed: bool,
) -> FrontierV1GpuReplaySummary {
    let mut overflow_flags = 0u32;
    if cpu_output.mapping.overflow {
        overflow_flags |= 1;
    }
    if cpu_output.resource_flow.overflow {
        overflow_flags |= 2;
    }
    FrontierV1GpuReplaySummary {
        mapping_summary_hash,
        resource_flow_summary_hash,
        field_policy_summary_hash,
        proposal_summary_hash: cpu_output.fingerprint.proposal_summary_hash,
        route_summary_hash: cpu_output.fingerprint.route_summary_hash,
        overflow_flags,
        accepted: cpu_output.admission_accepted,
        mapping_status: FrontierV1FieldStatus::GpuVerified,
        resource_flow_status,
        field_policy_routing_status,
        field_policy_pipe_status,
        route_status,
        gpu_reduction_eml_executed,
    }
}

pub fn gpu_resource_flow_from_cpu_oracle(
    cpu_rf: ResourceFlowSummary,
    route_code: u32,
) -> GpuResourceFlowAllocationSummary {
    GpuResourceFlowAllocationSummary {
        faction_a_allocation: cpu_rf.allocated_a,
        faction_b_allocation: cpu_rf.allocated_b,
        allocator_total: cpu_rf.allocated_a.saturating_add(cpu_rf.allocated_b),
        resource_overflow_flags: u32::from(cpu_rf.overflow),
        allocator_route_code: route_code,
    }
}

pub fn validate_frontier_v1_admission(
    skeleton: &FrontierV1ScenarioSkeleton,
) -> FrontierV1AdmissionReport {
    let mut rejected = Vec::new();

    let default_off_ok = validate_default_off(skeleton, &mut rejected);
    let mapping_ok = validate_mapping(skeleton, &mut rejected);
    let flat_star_ok = validate_flat_star(skeleton, &mut rejected);
    let field_policy_v1_ok = validate_field_policy_routing(skeleton, &mut rejected);
    let coupling_ok = validate_coupling(skeleton, &mut rejected);

    let accepted =
        default_off_ok && mapping_ok && flat_star_ok && field_policy_v1_ok && coupling_ok;

    FrontierV1AdmissionReport {
        accepted,
        mapping_ok,
        flat_star_ok,
        field_policy_v1_ok,
        coupling_ok,
        default_off_ok,
        rejected_reasons: rejected,
    }
}

pub fn classify_proposal_route(
    kind: ProposalKind,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> ProposalRoute {
    let field_policy = skeleton.field_policy;
    match kind {
        ProposalKind::ResourceDispatch => {
            if field_policy.resource_dispatch_via_allocator
                && skeleton.resource_flow.resource_flow_allocator_only
                && !skeleton.resource_flow.parallel_fixture_economy
            {
                ProposalRoute::ResourceFlowAllocator
            } else {
                ProposalRoute::Rejected
            }
        }
        ProposalKind::StructuralCommit => {
            if field_policy.structural_via_threshold_emit && !field_policy.cpu_commitment_emission {
                ProposalRoute::ThresholdEmitBoundaryRequest
            } else {
                ProposalRoute::Rejected
            }
        }
        ProposalKind::Movement => {
            if field_policy.movement_own_columns_only && !field_policy.cpu_planner {
                ProposalRoute::OwnColumnsOnly
            } else {
                ProposalRoute::Rejected
            }
        }
    }
}

pub fn cpu_mapping_oracle(
    config: &FrontierV1FixtureConfig,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> MappingSummary {
    let n = config.grid_size * config.grid_size;
    let mut cells = vec![0u32; n as usize];
    for i in 0..n {
        cells[i as usize] =
            ((config.seed as u32).wrapping_mul(i + 1).wrapping_add(17)) % config.source_cap;
    }
    if skeleton.coupling.district_seeds_supply_field {
        cells[0] = cells[0]
            .saturating_add(config.district_output_a)
            .min(config.source_cap);
        cells[n as usize - 1] = cells[n as usize - 1]
            .saturating_add(config.district_output_b)
            .min(config.source_cap);
    }
    let mut overflow = false;
    for _ in 0..config.horizon.min(skeleton.theater.horizon) {
        for i in 0..n {
            let capped = cells[i as usize].min(config.source_cap);
            if cells[i as usize] > config.source_cap {
                overflow = true;
            }
            cells[i as usize] = capped;
        }
    }
    MappingSummary {
        cell_sum: cells.iter().copied().sum(),
        cell_count: n,
        overflow,
    }
}

pub fn cpu_resource_flow_oracle(
    config: &FrontierV1FixtureConfig,
    mapping: MappingSummary,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> ResourceFlowSummary {
    let coupling_bonus = if skeleton.coupling.field_proposals_dispatch_via_allocator {
        mapping.cell_sum % 1000
    } else {
        0
    };
    let total = config
        .district_output_a
        .saturating_add(config.district_output_b)
        .saturating_add(coupling_bonus);
    let allocated_a = total.saturating_mul(3) / 5;
    let allocated_b = total.saturating_sub(allocated_a);
    ResourceFlowSummary {
        allocated_a,
        allocated_b,
        overflow: total > u32::MAX / 2,
    }
}

pub fn cpu_route_oracle(
    config: &FrontierV1FixtureConfig,
    skeleton: &FrontierV1ScenarioSkeleton,
) -> RouteSummary {
    let mut summary = RouteSummary {
        resource_route_count: 0,
        structural_route_count: 0,
        movement_route_count: 0,
        rejected_route_count: 0,
    };
    for kind in config.proposals {
        match classify_proposal_route(kind, skeleton) {
            ProposalRoute::ResourceFlowAllocator => summary.resource_route_count += 1,
            ProposalRoute::ThresholdEmitBoundaryRequest => summary.structural_route_count += 1,
            ProposalRoute::OwnColumnsOnly => summary.movement_route_count += 1,
            ProposalRoute::Rejected => summary.rejected_route_count += 1,
        }
    }
    summary
}

pub fn fingerprint_from_parts(
    mapping: MappingSummary,
    resource_flow: ResourceFlowSummary,
    proposal_count: u32,
    routes: RouteSummary,
) -> FrontierV1FixtureFingerprint {
    FrontierV1FixtureFingerprint {
        mapping_summary_hash: hash_mapping(mapping),
        resource_flow_summary_hash: hash_resource_flow(resource_flow),
        proposal_summary_hash: hash_u32(proposal_count),
        route_summary_hash: hash_routes(routes),
    }
}

pub fn run_frontier_v1_fixture(
    skeleton: &FrontierV1ScenarioSkeleton,
    config: &FrontierV1FixtureConfig,
) -> FrontierV1FixtureOutput {
    let admission = validate_frontier_v1_admission(skeleton);
    let mapping = cpu_mapping_oracle(config, skeleton);
    let resource_flow = cpu_resource_flow_oracle(config, mapping, skeleton);
    let routes = cpu_route_oracle(config, skeleton);
    let proposal_count = config.proposals.len() as u32;
    let event_count = routes.structural_route_count;
    let fingerprint = fingerprint_from_parts(mapping, resource_flow, proposal_count, routes);
    FrontierV1FixtureOutput {
        admission_accepted: admission.accepted,
        mapping,
        resource_flow,
        proposal_count,
        event_count,
        routes,
        fingerprint,
    }
}

fn validate_default_off(
    skeleton: &FrontierV1ScenarioSkeleton,
    rejected: &mut Vec<&'static str>,
) -> bool {
    let mut ok = true;
    if skeleton.enabled_by_default {
        rejected.push("profile must not be enabled by default");
        ok = false;
    }
    if skeleton.profile_name != FRONTIER_V1_PROFILE_NAME
        && skeleton.profile_name != FRONTIER_V2_PROFILE_NAME
    {
        rejected.push("profile_name must be FrontierV1 or FrontierV2");
        ok = false;
    }
    if skeleton.enabled_by_default
        && skeleton.mapping_execution_profile != MappingExecutionProfile::Disabled
    {
        rejected.push("mapping execution profile must not default-on");
        ok = false;
    }
    if skeleton.enabled_by_default
        && skeleton.resource_flow_opt_in != ResourceFlowOptInMode::Disabled
    {
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

fn validate_mapping(
    skeleton: &FrontierV1ScenarioSkeleton,
    rejected: &mut Vec<&'static str>,
) -> bool {
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

fn validate_flat_star(
    skeleton: &FrontierV1ScenarioSkeleton,
    rejected: &mut Vec<&'static str>,
) -> bool {
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

fn validate_field_policy_routing(
    skeleton: &FrontierV1ScenarioSkeleton,
    rejected: &mut Vec<&'static str>,
) -> bool {
    let s = skeleton.field_policy;
    let mut ok = true;
    if s.pipeline_version != FieldPolicyPipelineVersion::ProposalPipelineV1 {
        rejected.push("FIELD_POLICY Field agent Proposal Pipeline V1 only");
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

fn validate_coupling(
    skeleton: &FrontierV1ScenarioSkeleton,
    rejected: &mut Vec<&'static str>,
) -> bool {
    let c = skeleton.coupling;
    let mut ok = true;
    let coupling_profile_ok = skeleton.profile_name == FRONTIER_V1_PROFILE_NAME
        || skeleton.profile_name == FRONTIER_V2_PROFILE_NAME;
    if !coupling_profile_ok {
        if c.coupling_requested {
            rejected.push("economy↔field coupling allowed only for FrontierV1/FrontierV2 profiles");
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

fn hash_mapping(m: MappingSummary) -> u64 {
    let mut h = fnv64(b"mapping");
    h = fnv_append_u32(h, m.cell_sum);
    h = fnv_append_u32(h, m.cell_count);
    h = fnv_append_u32(h, u32::from(m.overflow));
    h
}

fn hash_resource_flow(r: ResourceFlowSummary) -> u64 {
    let mut h = fnv64(b"resource_flow");
    h = fnv_append_u32(h, r.allocated_a);
    h = fnv_append_u32(h, r.allocated_b);
    h = fnv_append_u32(h, u32::from(r.overflow));
    h
}

fn hash_routes(r: RouteSummary) -> u64 {
    let mut h = fnv64(b"routes");
    h = fnv_append_u32(h, r.resource_route_count);
    h = fnv_append_u32(h, r.structural_route_count);
    h = fnv_append_u32(h, r.movement_route_count);
    h = fnv_append_u32(h, r.rejected_route_count);
    h
}

fn hash_u32(v: u32) -> u64 {
    fnv_append_u32(fnv64(b"proposal"), v)
}

fn fnv64(seed: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in seed {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn fnv_append_u32(mut hash: u64, v: u32) -> u64 {
    for b in v.to_le_bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn fnv_mix(v: u64) -> u64 {
    fnv_append_u32(fnv64(b"mix"), v as u32) ^ (v >> 32)
}

fn live_field_agent_field_status_code(s: FrontierV1LiveFieldAgentFieldStatus) -> u32 {
    match s {
        FrontierV1LiveFieldAgentFieldStatus::GpuVerified => 0,
        FrontierV1LiveFieldAgentFieldStatus::PartialGpuVerified => 1,
        FrontierV1LiveFieldAgentFieldStatus::ReplayAccepted => 2,
        FrontierV1LiveFieldAgentFieldStatus::CpuOracleOnly => 3,
        FrontierV1LiveFieldAgentFieldStatus::FixtureOnly => 4,
        FrontierV1LiveFieldAgentFieldStatus::OutOfScopeForV1_5 => 5,
        FrontierV1LiveFieldAgentFieldStatus::Pending => 6,
    }
}
