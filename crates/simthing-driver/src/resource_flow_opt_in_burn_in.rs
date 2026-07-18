//! RF-T2 — controlled opt-in Resource Flow burn-in fixtures and runners (driver/test-only).
//!
//! Opens sessions via authored `ResourceFlowOptInMode::FlatStarOptIn` on `ResourceFlowSpec`
//! through `SimSession::open_from_spec`. Does not flip global default-on.

use std::collections::HashMap;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::{BoundaryOutcome, FissionOutcome};
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, GameModeSpec,
    PropertyKey, PropertySpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode,
    ResourceFlowSpec, SpecVersion, WildcardAdmissionSpec,
};

use crate::arena_hierarchy::{
    build_execution_plan, resolve_node_columns, ArenaTreeLayout, NodeColumnRefs,
};
use crate::resource_flow_burn_in::{ResourceFlowBurnInReport, ResourceFlowSoakSummaryReport};
use crate::resource_flow_dynamic_enrollment_soak::{
    run_dynamic_enrollment_gpu_burn_in, run_dynamic_enrollment_resync_cycles,
    DynamicEnrollmentBoundaryMetrics,
};
use crate::scenario::Scenario;
use crate::session::{SessionError, SimSession};

type CellKey = (crate::arena_registry::SlotId, u32);

pub const RF_T2_STATIC_FLAT_STAR_10: &str = "rf_t2_static_flat_star_10_participants";
pub const RF_T2_STATIC_FLAT_STAR_64: &str = "rf_t2_static_flat_star_64_participants";
pub const RF_T2_STATIC_FLAT_STAR_SKEWED: &str = "rf_t2_static_flat_star_skewed_weights";
pub const RF_T2_DYNAMIC_SINGLE_FISSION: &str = "rf_t2_dynamic_single_fission_flat_star";
pub const RF_T2_DYNAMIC_MULTI_FISSION: &str = "rf_t2_dynamic_multi_fission_flat_star";
pub const RF_T2_TWO_ARENA_NO_COUPLING: &str = "rf_t2_two_arena_flat_star_no_coupling";
pub const RF_T2_DISABLED_POPULATED: &str = "rf_t2_disabled_populated_spec_no_gpu_execution";
pub const RF_T2_WILDCARD_REJECTED: &str = "rf_t2_wildcard_or_nested_claim_rejected";

pub const RF_T3_PRODUCT_STATIC_128: &str = "rf_t3_product_static_128_participants";
pub const RF_T3_PRODUCT_STATIC_256: &str = "rf_t3_product_static_256_participants";
pub const RF_T3_PRODUCT_DYNAMIC_FISSION: &str = "rf_t3_product_dynamic_fission_cadence";
pub const RF_T3_PRODUCT_MULTI_ARENA: &str = "rf_t3_product_multi_arena_no_coupling";
pub const RF_T3_PRODUCT_MULTI_SESSION: &str = "rf_t3_product_multi_session_replay";
pub const RF_T3_PRODUCT_DISABLED: &str = "rf_t3_product_disabled_spec_diagnostics";
pub const RF_T3_PRODUCT_REJECTION: &str = "rf_t3_product_rejection_telemetry";
pub const RF_T3_PRODUCT_RESYNC: &str = "rf_t3_product_repeated_resync_stable";

pub const RF_T5_PROFILE_STATIC_128: &str = "rf_t5_profile_static_128_participants";
pub const RF_T5_PROFILE_STATIC_256: &str = "rf_t5_profile_static_256_participants";
pub const RF_T5_PROFILE_DYNAMIC_FISSION: &str = "rf_t5_profile_dynamic_fission_cadence";
pub const RF_T5_PROFILE_MULTI_ARENA: &str = "rf_t5_profile_multi_arena_no_coupling";
pub const RF_T5_PROFILE_MULTI_SESSION: &str = "rf_t5_profile_multi_session_replay";
pub const RF_T5_PROFILE_DISABLED: &str = "rf_t5_profile_disabled_or_default_no_gpu_execution";
pub const RF_T5_PROFILE_REJECTION: &str = "rf_t5_profile_rejection_telemetry";
pub const RF_T5_PROFILE_RESYNC: &str = "rf_t5_profile_repeated_resync_stable";

pub const RF_CONTINUED_STATIC_512: &str = "rf_continued_static_512_participants";
pub const RF_CONTINUED_STATIC_SKEWED: &str = "rf_continued_static_skewed_weights";
pub const RF_CONTINUED_DYNAMIC_POLICY_A: &str = "rf_continued_dynamic_policy_a_fission";
pub const RF_CONTINUED_MULTI_ARENA: &str = "rf_continued_multi_arena_no_coupling";
pub const RF_CONTINUED_REPLAY: &str = "rf_continued_replay_same_seed";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfT2EnrollmentKind {
    StaticExplicit,
    DynamicFissionSingle,
    DynamicFissionMulti,
    DynamicFissionReject,
    TwoArenaStatic,
    DisabledPopulated,
    WildcardRejected,
}

#[derive(Clone, Debug)]
pub struct RfT2BurnInFixture {
    pub name: &'static str,
    pub opt_in_mode: ResourceFlowOptInMode,
    pub enrollment: RfT2EnrollmentKind,
    pub participant_count: u32,
    pub ticks: u32,
    pub sync_cycles: u32,
    pub root_intrinsic_flow: f32,
    pub leaf_weights: Vec<f32>,
    pub expected_admissions: u32,
    pub expected_rejections: u32,
    pub expect_generation_bump: bool,
    pub expect_gpu_active: bool,
    pub require_bit_exact: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RfT2BurnInReport {
    pub scenario_name: String,
    pub ticks_checked: u32,
    pub sync_cycles_checked: u32,
    pub admissions_observed: u32,
    pub rejections_observed: u32,
    pub generation_start: u64,
    pub generation_end: u64,
    pub resource_flow_syncs_observed: u32,
    pub total_ops: u32,
    pub n_bands: u32,
    pub max_abs_error: f32,
    pub replay_bit_exact: bool,
}

pub struct RfT2OptInSession {
    pub session: SimSession,
    pub layout: ArenaTreeLayout,
    pub cols: NodeColumnRefs,
    pub leaf_slots: Vec<u32>,
    pub inputs: HashMap<CellKey, f32>,
    pub boundary_metrics: DynamicEnrollmentBoundaryMetrics,
    pub fission: Option<FissionOutcome>,
}

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

pub fn register_food_flow(reg: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "food".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "food".into(),
                },
            ),
        ],
    };
    compile_property(&spec, reg).unwrap();
}

fn register_research_flow(reg: &mut DimensionRegistry) {
    let spec = PropertySpec {
        id: "research_flow".into(),
        namespace: "core".into(),
        name: "research_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "research".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "research".into(),
                },
            ),
        ],
    };
    compile_property(&spec, reg).unwrap();
}

fn n_slots_for(participant_count: u32) -> u32 {
    participant_count
        .saturating_mul(2)
        .saturating_add(32)
        .max(128)
}

fn build_hosted_flat_scenario(participant_count: u32, registry: DimensionRegistry) -> Scenario {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..participant_count {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    Scenario {
        name: format!("rf_t2_hosted_{participant_count}"),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: n_slots_for(participant_count),
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

fn build_skewed_scenario(registry: DimensionRegistry) -> Scenario {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for i in 0..3 {
        let mut hosted = SimThing::new(SimThingKind::Cohort, 0);
        if i == 0 {
            hosted.add_child(SimThing::new(SimThingKind::Cohort, 0));
            hosted.add_child(SimThing::new(SimThingKind::Cohort, 0));
        }
        root.add_child(hosted);
    }
    Scenario {
        name: "rf_t2_skewed".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

fn build_dynamic_scenario(
    parent_count: usize,
    multi_fission: bool,
    registry: DimensionRegistry,
) -> (Scenario, FissionOutcome) {
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut pairs = Vec::new();
    for i in 0..parent_count {
        let mut parent = SimThing::new(SimThingKind::Cohort, 0);
        let parent_id = parent.id;
        if !multi_fission && i > 0 {
            root.add_child(parent);
            continue;
        }
        let child = SimThing::new(SimThingKind::Cohort, 0);
        let child_id = child.id;
        parent.add_child(child);
        pairs.push((parent_id, child_id));
        root.add_child(parent);
    }
    let fission = FissionOutcome {
        fissions_executed: pairs.len() as u32,
        fission_pairs: pairs,
        ..Default::default()
    };
    let scenario = Scenario {
        name: format!("rf_t2_dynamic_{parent_count}"),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 128,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    (scenario, fission)
}

fn build_two_arena_scenario(registry: DimensionRegistry) -> Scenario {
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    Scenario {
        name: "rf_t2_two_arena".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 128,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    }
}

fn fill_explicit_roots(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap().raw(), c.id.raw()))
        .collect();
    game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = participants;
}

fn base_game_mode(id: &str) -> GameModeSpec {
    GameModeSpec {
        id: id.into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: None,
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

fn food_arena_spec(max_participants: u32, fission: FissionPolicySpec) -> ArenaSpec {
    ArenaSpec {
        name: "food".into(),
        flow_property: PropertyKey::new("core", "food_flow"),
        balance_property: None,
        max_participants,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: fission,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: vec![],
        enrollment: None,
        wildcard_admission: None,
    }
}

fn build_game_mode(fixture: &RfT2BurnInFixture, scenario: &Scenario) -> GameModeSpec {
    let mut mode = base_game_mode(fixture.name);
    let max_participants = match fixture.enrollment {
        RfT2EnrollmentKind::DynamicFissionReject => 1,
        _ => fixture.participant_count.max(16),
    };
    match fixture.enrollment {
        RfT2EnrollmentKind::TwoArenaStatic => {
            mode.resource_flow = Some(ResourceFlowSpec {
                opt_in_mode: fixture.opt_in_mode,
                arenas: vec![
                    food_arena_spec(max_participants, FissionPolicySpec::Inherit),
                    ArenaSpec {
                        name: "research".into(),
                        flow_property: PropertyKey::new("core", "research_flow"),
                        balance_property: None,
                        max_participants,
                        max_coupling_fanout: 4,
                        max_orderband_depth: 16,
                        fission_policy: FissionPolicySpec::Inherit,
                        reserved_orderband_depth: 0,
                        reserved_gap_per_intermediate: 0,
                        expected_max_children_per_intermediate: 0,
                        explicit_participants: vec![],
                        enrollment: None,
                        wildcard_admission: None,
                    },
                ],
                couplings: vec![],
                base_obligations: vec![],
                capacity_budget: None,
                gated_rates: vec![],
            });
            let mut alloc = SlotAllocator::new();
            alloc.populate_from_tree(&scenario.root);
            let explicit: Vec<_> = scenario
                .root
                .children
                .iter()
                .map(|hosted| {
                    ExplicitParticipantSpec::flat(
                        alloc.slot_of(hosted.id).unwrap().raw(),
                        hosted.id.raw(),
                    )
                })
                .collect();
            let flow = mode.resource_flow.as_mut().unwrap();
            flow.arenas[0].explicit_participants = explicit.clone();
            flow.arenas[1].explicit_participants = explicit;
        }
        RfT2EnrollmentKind::WildcardRejected => {
            let mut arena = food_arena_spec(max_participants, FissionPolicySpec::Reject);
            arena.wildcard_admission = Some(WildcardAdmissionSpec {
                max_expansion: Some(4),
                expanded_count: 0,
            });
            mode.resource_flow = Some(ResourceFlowSpec {
                opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
                arenas: vec![arena],
                couplings: vec![],
                base_obligations: vec![],
                capacity_budget: None,
                gated_rates: vec![],
            });
            fill_explicit_roots(&mut mode, scenario);
        }
        _ => {
            let fission = match fixture.enrollment {
                RfT2EnrollmentKind::DynamicFissionSingle
                | RfT2EnrollmentKind::DynamicFissionMulti
                | RfT2EnrollmentKind::DynamicFissionReject => FissionPolicySpec::Inherit,
                _ => FissionPolicySpec::Reject,
            };
            mode.resource_flow = Some(ResourceFlowSpec {
                opt_in_mode: fixture.opt_in_mode,
                arenas: vec![food_arena_spec(max_participants, fission)],
                couplings: vec![],
                base_obligations: vec![],
                capacity_budget: None,
                gated_rates: vec![],
            });
            if matches!(
                fixture.enrollment,
                RfT2EnrollmentKind::DynamicFissionSingle
                    | RfT2EnrollmentKind::DynamicFissionMulti
                    | RfT2EnrollmentKind::DynamicFissionReject
            ) {
                let mut alloc = SlotAllocator::new();
                alloc.populate_from_tree(&scenario.root);
                let explicit: Vec<_> = scenario
                    .root
                    .children
                    .iter()
                    .map(|hosted| {
                        ExplicitParticipantSpec::flat(
                            alloc.slot_of(hosted.id).unwrap().raw(),
                            hosted.id.raw(),
                        )
                    })
                    .collect();
                mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = explicit;
            } else {
                fill_explicit_roots(&mut mode, scenario);
            }
        }
    }
    mode
}

pub fn fixture_static_flat_star_10_participants() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_STATIC_FLAT_STAR_10,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 10,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_static_flat_star_64_participants() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_STATIC_FLAT_STAR_64,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 64,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: false,
    }
}

pub fn fixture_static_flat_star_skewed_weights() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_STATIC_FLAT_STAR_SKEWED,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 3,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 3.0],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_dynamic_single_fission() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_DYNAMIC_SINGLE_FISSION,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::DynamicFissionSingle,
        participant_count: 16,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 3.0],
        expected_admissions: 1,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_dynamic_multi_fission() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_DYNAMIC_MULTI_FISSION,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::DynamicFissionMulti,
        participant_count: 16,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 1.0, 2.0, 3.0],
        expected_admissions: 2,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_two_arena_no_coupling() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_TWO_ARENA_NO_COUPLING,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::TwoArenaStatic,
        participant_count: 16,
        ticks: 100,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_disabled_populated_spec() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_DISABLED_POPULATED,
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        enrollment: RfT2EnrollmentKind::DisabledPopulated,
        participant_count: 10,
        ticks: 0,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: false,
        require_bit_exact: true,
    }
}

pub fn fixture_wildcard_rejected() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T2_WILDCARD_REJECTED,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::WildcardRejected,
        participant_count: 10,
        ticks: 0,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: false,
        require_bit_exact: true,
    }
}

pub fn fixture_repeated_resync() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: "rf_t2_repeated_resync_stable",
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 10,
        ticks: 10,
        sync_cycles: 100,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_product_static_128_participants() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_STATIC_128,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 128,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: false,
    }
}

pub fn fixture_product_static_512_participants() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_CONTINUED_STATIC_512,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 512,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: false,
    }
}

pub fn fixture_product_static_256_participants() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_STATIC_256,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 256,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: false,
    }
}

pub fn fixture_product_dynamic_fission_cadence() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_DYNAMIC_FISSION,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::DynamicFissionMulti,
        participant_count: 16,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0, 1.0, 2.0, 3.0],
        expected_admissions: 2,
        expected_rejections: 0,
        expect_generation_bump: true,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_product_multi_arena_no_coupling() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_MULTI_ARENA,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::TwoArenaStatic,
        participant_count: 16,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_product_multi_session_replay() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_MULTI_SESSION,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 10,
        ticks: 1000,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_product_disabled_spec_diagnostics() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_DISABLED,
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        enrollment: RfT2EnrollmentKind::DisabledPopulated,
        participant_count: 10,
        ticks: 0,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: false,
        require_bit_exact: true,
    }
}

pub fn fixture_product_rejection_telemetry() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_REJECTION,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::DynamicFissionReject,
        participant_count: 1,
        ticks: 0,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![1.0],
        expected_admissions: 0,
        expected_rejections: 1,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

pub fn fixture_product_repeated_resync() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T3_PRODUCT_RESYNC,
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 10,
        ticks: 10,
        sync_cycles: 100,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

fn profile_fixture_from_product(
    mut base: RfT2BurnInFixture,
    name: &'static str,
) -> RfT2BurnInFixture {
    base.name = name;
    base.opt_in_mode = ResourceFlowOptInMode::Disabled;
    base
}

pub fn fixture_profile_static_128_participants() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_static_128_participants(),
        RF_T5_PROFILE_STATIC_128,
    )
}

pub fn fixture_profile_static_512_participants() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_static_512_participants(),
        RF_CONTINUED_STATIC_512,
    )
}

pub fn fixture_profile_static_256_participants() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_static_256_participants(),
        RF_T5_PROFILE_STATIC_256,
    )
}

pub fn fixture_profile_dynamic_fission_cadence() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_dynamic_fission_cadence(),
        RF_T5_PROFILE_DYNAMIC_FISSION,
    )
}

pub fn fixture_profile_multi_arena_no_coupling() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_multi_arena_no_coupling(),
        RF_T5_PROFILE_MULTI_ARENA,
    )
}

pub fn fixture_profile_multi_session_replay() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_multi_session_replay(),
        RF_T5_PROFILE_MULTI_SESSION,
    )
}

pub fn fixture_profile_disabled_or_default() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: RF_T5_PROFILE_DISABLED,
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        enrollment: RfT2EnrollmentKind::DisabledPopulated,
        participant_count: 10,
        ticks: 0,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: false,
        require_bit_exact: true,
    }
}

pub fn fixture_profile_rejection_telemetry() -> RfT2BurnInFixture {
    profile_fixture_from_product(
        fixture_product_rejection_telemetry(),
        RF_T5_PROFILE_REJECTION,
    )
}

pub fn fixture_profile_repeated_resync() -> RfT2BurnInFixture {
    profile_fixture_from_product(fixture_product_repeated_resync(), RF_T5_PROFILE_RESYNC)
}

pub fn open_fixture_session_with_default_profile(
    fixture: &RfT2BurnInFixture,
) -> Result<RfT2OptInSession, SessionError> {
    open_fixture_session_with_execution_profile(
        fixture,
        ResourceFlowExecutionProfile::DefaultDisabled,
    )
}

pub fn fixture_replay_static() -> RfT2BurnInFixture {
    RfT2BurnInFixture {
        name: "rf_t2_replay_same_seed",
        opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
        enrollment: RfT2EnrollmentKind::StaticExplicit,
        participant_count: 10,
        ticks: 10,
        sync_cycles: 0,
        root_intrinsic_flow: 10.0,
        leaf_weights: vec![],
        expected_admissions: 0,
        expected_rejections: 0,
        expect_generation_bump: false,
        expect_gpu_active: true,
        require_bit_exact: true,
    }
}

fn scenario_for_fixture(fixture: &RfT2BurnInFixture) -> (Scenario, Option<FissionOutcome>) {
    let mut reg = DimensionRegistry::new();
    register_food_flow(&mut reg);
    match fixture.enrollment {
        RfT2EnrollmentKind::TwoArenaStatic => {
            register_research_flow(&mut reg);
            (build_two_arena_scenario(reg), None)
        }
        RfT2EnrollmentKind::DynamicFissionSingle => {
            let (scenario, fission) = build_dynamic_scenario(2, false, reg);
            (scenario, Some(fission))
        }
        RfT2EnrollmentKind::DynamicFissionMulti => {
            let (scenario, fission) = build_dynamic_scenario(2, true, reg);
            (scenario, Some(fission))
        }
        RfT2EnrollmentKind::DynamicFissionReject => {
            let (scenario, fission) = build_dynamic_scenario(1, false, reg);
            (scenario, Some(fission))
        }
        _ => {
            if matches!(
                fixture.name,
                RF_T2_STATIC_FLAT_STAR_SKEWED | RF_CONTINUED_STATIC_SKEWED
            ) {
                (build_skewed_scenario(reg), None)
            } else {
                (
                    build_hosted_flat_scenario(fixture.participant_count, reg),
                    None,
                )
            }
        }
    }
}

fn leaf_slots_for_layout(layout: &ArenaTreeLayout) -> Vec<crate::arena_registry::SlotId> {
    layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect()
}

fn cell_inputs(
    layout: &ArenaTreeLayout,
    cols: NodeColumnRefs,
    root_intrinsic_flow: f32,
    leaf_weights: &[f32],
) -> HashMap<CellKey, f32> {
    let root_slot = layout.participant_roots[0].participant_slot;
    let leaves = leaf_slots_for_layout(layout);
    let weights: Vec<f32> = if leaf_weights.is_empty() {
        vec![1.0; leaves.len()]
    } else {
        leaf_weights.to_vec()
    };
    let mut inputs = HashMap::from([((root_slot, cols.intrinsic_flow_col), root_intrinsic_flow)]);
    for (slot, &weight) in leaves.iter().zip(weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }
    inputs
}

fn execution_layout(session: &SimSession) -> (ArenaTreeLayout, NodeColumnRefs) {
    let flow_id = session
        .proto
        .registry
        .id_of("core", "food_flow")
        .expect("food_flow");
    let cols = resolve_node_columns(&session.proto.registry.property(flow_id).layout, "food")
        .expect("cols");
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("plan")
    .arenas
    .into_iter()
    .next()
    .expect("food arena");
    (layout, cols)
}

pub fn open_fixture_session(fixture: &RfT2BurnInFixture) -> Result<RfT2OptInSession, SessionError> {
    open_fixture_session_with_execution_profile(
        fixture,
        ResourceFlowExecutionProfile::DefaultDisabled,
    )
}

pub fn open_fixture_session_with_execution_profile(
    fixture: &RfT2BurnInFixture,
    profile: ResourceFlowExecutionProfile,
) -> Result<RfT2OptInSession, SessionError> {
    let (scenario, fission) = scenario_for_fixture(fixture);
    let mut game_mode = build_game_mode(fixture, &scenario);
    game_mode.resource_flow_execution_profile = profile;
    let mut session = SimSession::open_from_spec(scenario, &game_mode)?;

    let expect_flag = fixture.opt_in_mode == ResourceFlowOptInMode::FlatStarOptIn
        || profile.enables_arena_resource_flow();
    assert_eq!(
        session.proto.flags.use_accumulator_resource_flow,
        expect_flag,
        "fixture {name} flag must match opt-in/profile",
        name = fixture.name
    );

    let mut boundary_metrics = DynamicEnrollmentBoundaryMetrics::default();
    boundary_metrics.generation_start = session.spec_state.arena_registry.generation;

    if let Some(fission) = fission {
        let outcome = BoundaryOutcome {
            fission,
            ..Default::default()
        };
        session.react_to_fission_resource_flow_enrollment(&outcome)?;
        if let Some(report) = session
            .last_resource_flow_dynamic_enrollment_report
            .as_ref()
        {
            boundary_metrics = DynamicEnrollmentBoundaryMetrics::from_enrollment_report(
                report,
                1,
                report.admissions.len() as u32,
                if report.any_admissions() && session.proto.flags.use_accumulator_resource_flow {
                    1
                } else {
                    0
                },
            );
        }
    } else {
        boundary_metrics.generation_end = session.spec_state.arena_registry.generation;
    }

    let (layout, cols) = execution_layout(&session);
    let leaf_slots: Vec<u32> = leaf_slots_for_layout(&layout)
        .into_iter()
        .map(|s| s.raw())
        .collect();
    let inputs = cell_inputs(
        &layout,
        cols,
        fixture.root_intrinsic_flow,
        &fixture.leaf_weights,
    );

    Ok(RfT2OptInSession {
        session,
        layout,
        cols,
        leaf_slots,
        inputs,
        boundary_metrics,
        fission: None,
    })
}

pub fn assert_fixture_contract(fx: &RfT2OptInSession, fixture: &RfT2BurnInFixture) {
    assert_eq!(
        fx.layout.max_depth,
        2,
        "fixture {name} must stay flat-star D=2",
        name = fixture.name
    );
    assert_eq!(
        fx.session.state.accumulator_resource_flow_active,
        fixture.expect_gpu_active,
        "fixture {name} gpu active",
        name = fixture.name
    );
    if fixture.expected_admissions > 0 || fixture.expected_rejections > 0 {
        let report = fx
            .session
            .last_resource_flow_dynamic_enrollment_report
            .as_ref()
            .expect("dynamic enrollment report");
        assert_eq!(
            report.admissions.len() as u32,
            fixture.expected_admissions,
            "fixture {name} admissions",
            name = fixture.name
        );
        assert_eq!(
            report.rejections.len() as u32,
            fixture.expected_rejections,
            "fixture {name} rejections",
            name = fixture.name
        );
    }
    if fixture.expect_generation_bump {
        assert!(
            fx.boundary_metrics.generation_end > fx.boundary_metrics.generation_start,
            "fixture {name} generation bump",
            name = fixture.name
        );
    }
}

pub fn run_opt_in_burn_in(
    fx: &mut RfT2OptInSession,
    fixture: &RfT2BurnInFixture,
) -> Result<RfT2BurnInReport, SessionError> {
    assert_fixture_contract(fx, fixture);

    let mut sync_cycles_checked = 0u32;
    if fixture.sync_cycles > 0 && fx.session.proto.flags.use_accumulator_resource_flow {
        let (syncs, _, _) =
            run_dynamic_enrollment_resync_cycles(&mut fx.session, fixture.sync_cycles)?;
        sync_cycles_checked = syncs;
        fx.boundary_metrics.resource_flow_syncs_observed += syncs;
    }

    let total_ops = fx
        .session
        .state
        .accumulator_runtime
        .as_ref()
        .map(|r| r.resource_flow_ops.count)
        .unwrap_or(0);
    let n_bands = fx.session.state.accumulator_resource_flow_bands;

    let burn = if fixture.ticks > 0 && fx.session.proto.flags.use_accumulator_resource_flow {
        run_dynamic_enrollment_gpu_burn_in(
            &mut fx.session.state,
            &fx.layout,
            fx.cols,
            fx.session.proto.registry.total_columns as u32,
            &fx.inputs,
            &fx.leaf_slots,
            n_bands,
            fixture.ticks,
            fx.session.scenario.dt,
        )
    } else {
        ResourceFlowBurnInReport::default()
    };

    let replay_bit_exact = burn.max_abs_error.to_bits() == 0.0_f32.to_bits();
    if fixture.require_bit_exact
        && fixture.ticks > 0
        && fx.session.proto.flags.use_accumulator_resource_flow
    {
        assert_eq!(
            burn.max_abs_error.to_bits(),
            0.0_f32.to_bits(),
            "fixture {name} must be bit-exact",
            name = fixture.name
        );
    }

    Ok(RfT2BurnInReport {
        scenario_name: fixture.name.to_string(),
        ticks_checked: burn.ticks_checked,
        sync_cycles_checked: sync_cycles_checked.max(
            if fx.session.proto.flags.use_accumulator_resource_flow {
                1
            } else {
                0
            },
        ),
        admissions_observed: fx.boundary_metrics.admissions_observed,
        rejections_observed: fx.boundary_metrics.rejections_observed,
        generation_start: fx.boundary_metrics.generation_start,
        generation_end: fx.session.spec_state.arena_registry.generation,
        resource_flow_syncs_observed: fx.boundary_metrics.resource_flow_syncs_observed,
        total_ops,
        n_bands,
        max_abs_error: burn.max_abs_error,
        replay_bit_exact: fixture.require_bit_exact && replay_bit_exact,
    })
}

pub fn clone_for_replay(fx: &RfT2OptInSession, fixture: &RfT2BurnInFixture) -> RfT2OptInSession {
    let (scenario, _) = scenario_for_fixture(fixture);
    let mut game_mode = build_game_mode(fixture, &scenario);
    game_mode.resource_flow_execution_profile = fx.session.resource_flow_execution_profile;
    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("replay open");
    session.proto.root = fx.session.proto.root.clone();
    session.proto.allocator = fx.session.proto.allocator.clone();
    session.proto.registry = fx.session.proto.registry.clone();
    session.spec_state.arena_registry = fx.session.spec_state.arena_registry.clone();
    session.spec_state.arena_participant_scaffold =
        fx.session.spec_state.arena_participant_scaffold.clone();
    session.proto.flags = fx.session.proto.flags.clone();
    session
        .sync_resource_flow_if_enabled()
        .expect("replay sync");

    let (layout, cols) = execution_layout(&session);
    let leaf_slots: Vec<u32> = leaf_slots_for_layout(&layout)
        .into_iter()
        .map(|s| s.raw())
        .collect();
    let inputs = cell_inputs(
        &layout,
        cols,
        fixture.root_intrinsic_flow,
        &fixture.leaf_weights,
    );

    RfT2OptInSession {
        session,
        layout,
        cols,
        leaf_slots,
        inputs,
        boundary_metrics: fx.boundary_metrics.clone(),
        fission: None,
    }
}

impl RfT2BurnInReport {
    pub fn from_soak_summary(summary: &ResourceFlowSoakSummaryReport) -> Self {
        Self {
            scenario_name: summary.scenario_name.clone(),
            ticks_checked: summary.ticks_checked,
            sync_cycles_checked: summary.sync_cycles_checked,
            total_ops: summary.total_ops,
            n_bands: summary.n_bands,
            max_abs_error: summary.max_abs_error,
            replay_bit_exact: summary.replay_bit_exact,
            ..Default::default()
        }
    }
}
