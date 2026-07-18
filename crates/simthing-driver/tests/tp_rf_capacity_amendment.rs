//! TP-RF-CAPACITY-AMENDMENT-0 capacity admission proof.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, DimensionRegistry, LogTier, SimThing,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{Scenario, SimSession};
use simthing_gpu::GpuContext;
use simthing_spec::{
    compile_property, ArenaSpec, CouplingDelaySpec, CouplingSpec, EnrollmentSelectorSpec,
    FissionPolicySpec, GameModeSpec, InstallTargetSpec, PropertyKey, PropertySpec,
    ResourceFlowCapacityBudgetSpec, ResourceFlowOptInMode, ResourceFlowSpec, SpecVersion,
};

const OWNED_SYSTEMS: usize = 250;
const FLEETS: usize = 20;
const SHIPS_PER_FLEET: usize = 30;
const BUDGET_GPU_SLOTS: u32 = 2_048;
const BUDGET_PARTICIPANTS_PER_ARENA: u32 = 704;
const BUDGET_FANOUT: u32 = 8;
const BUDGET_ORDERBAND: u32 = 16;

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

fn register_flow(registry: &mut DimensionRegistry, property: &str, arena: &str) {
    let spec = PropertySpec {
        id: property.into(),
        namespace: "tp".into(),
        name: property.into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: arena.into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: arena.into(),
                },
            ),
        ],
    };
    compile_property(&spec, registry).expect("compile flow property");
}

fn galaxy_scale_scenario() -> Scenario {
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..OWNED_SYSTEMS {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    for _ in 0..FLEETS {
        let mut fleet = SimThing::new(SimThingKind::Fleet, 0);
        for _ in 0..SHIPS_PER_FLEET {
            fleet.add_child(SimThing::new(SimThingKind::Cohort, 0));
        }
        root.add_child(fleet);
    }

    let mut registry = DimensionRegistry::new();
    register_flow(&mut registry, "system_flow", "systems");
    register_flow(&mut registry, "fleet_flow", "fleets");

    Scenario {
        name: "tp_rf_capacity_amendment".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 64,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    }
}

fn arena(name: &str, property: &str, kind: &str, reserved_depth: u32) -> ArenaSpec {
    ArenaSpec {
        name: name.into(),
        flow_property: PropertyKey::new("tp", property),
        balance_property: None,
        max_participants: 16,
        max_coupling_fanout: 1,
        max_orderband_depth: 1,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: reserved_depth,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: Vec::new(),
        enrollment: Some(EnrollmentSelectorSpec::InstallTarget(
            InstallTargetSpec::AllOfKind { kind: kind.into() },
        )),
        wildcard_admission: None,
    }
}

fn galaxy_scale_game_mode() -> GameModeSpec {
    GameModeSpec {
        id: "tp_rf_capacity_amendment".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: vec![],
        properties: vec![],
        overlays: vec![],
        capability_trees: vec![],
        events: vec![],
        resource_flow: Some(ResourceFlowSpec {
            opt_in_mode: ResourceFlowOptInMode::Disabled,
            arenas: vec![
                arena("systems", "system_flow", "Location", 8),
                arena("fleets", "fleet_flow", "Fleet", 8),
            ],
            couplings: vec![CouplingSpec {
                from_arena: "systems".into(),
                to_arena: "fleets".into(),
                delay: CouplingDelaySpec::OneTickDelay,
            }],
            base_obligations: vec![],
            capacity_budget: Some(ResourceFlowCapacityBudgetSpec {
                simthing_count: (1 + OWNED_SYSTEMS + FLEETS + (FLEETS * SHIPS_PER_FLEET)) as u32,
                property_columns: 6,
                rf_arena_count: 2,
                participants_per_arena: BUDGET_PARTICIPANTS_PER_ARENA,
                coupling_fanout_per_arena: BUDGET_FANOUT,
                orderband_depth: BUDGET_ORDERBAND,
                emission_capacity: BUDGET_PARTICIPANTS_PER_ARENA,
                threshold_emission_capacity: BUDGET_PARTICIPANTS_PER_ARENA,
                gpu_slots: BUDGET_GPU_SLOTS,
                field_buffer_cells: BUDGET_GPU_SLOTS * 6,
                readback_records: BUDGET_PARTICIPANTS_PER_ARENA,
            }),
            gated_rates: vec![],
                need_weight_profiles: vec![],
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    }
}

#[test]
fn tp_rf_capacity_budget_installs_250_owned_systems_plus_fleet_load() {
    let Some(_gpu) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping: no GPU");
        return;
    };

    let session =
        SimSession::open_from_spec(galaxy_scale_scenario(), &galaxy_scale_game_mode()).unwrap();
    let registry = &session.spec_state.arena_registry;

    assert_eq!(registry.arenas.len(), 2);
    assert_eq!(registry.arenas[0].participant_range.1, OWNED_SYSTEMS as u32);
    assert_eq!(registry.arenas[1].participant_range.1, FLEETS as u32);
    for arena in &registry.arenas {
        assert_eq!(arena.max_participants, BUDGET_PARTICIPANTS_PER_ARENA);
        assert_eq!(arena.max_coupling_fanout, BUDGET_FANOUT);
        assert_eq!(arena.max_orderband_depth, BUDGET_ORDERBAND);
    }
    assert!(session.state.n_slots >= BUDGET_GPU_SLOTS);
    assert_eq!(
        session
            .spec_state
            .resource_flow_capacity_budget
            .as_ref()
            .expect("resolved budget")
            .gpu_slots,
        BUDGET_GPU_SLOTS
    );
}
