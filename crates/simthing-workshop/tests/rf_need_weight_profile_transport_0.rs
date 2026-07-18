//! RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) — need / weight_profile transport proofs.
//!
//! §12 homing: synthetic foundry vocabulary only. Production path is ordinary
//! `SimSession::open_from_spec` + `step_once`. GPU/adapter Unsupported is FAIL.

use std::collections::HashMap;

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{binding_from_hydrated_stack, Scenario, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_property, ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec,
    EmlGadgetInstanceSpec, EmlGadgetStackSpec, ExplicitParticipantSpec, FissionPolicySpec,
    GameModeSpec, InstallTargetSpec, NeedWeightProfileInputSpec, NeedWeightProfileThresholdSpec,
    PropertyKey, PropertySpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec,
    SpecVersion,
};

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

fn register_flow(registry: &mut DimensionRegistry) {
    let property = PropertySpec {
        id: "foundry_flow".into(),
        namespace: "workshop".into(),
        name: "foundry_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "foundry".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "foundry".into(),
                },
            ),
            SubFieldSpec {
                role: SubFieldRole::Named("balance_rate".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "balance_rate".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Named("balance".into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "balance".into(),
                display_range: None,
                governed_by: Some(SubFieldRole::Named("balance_rate".into())),
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: Some(AccumulatorSpec {
                    role: AccumulatorRole::Balance(BalanceSpec::default()),
                    log_tier: LogTier::Summary,
                }),
            },
        ],
    };
    compile_property(&property, registry).expect("register workshop flow property");
}

fn expansion_stack() -> EmlGadgetStackSpec {
    EmlGadgetStackSpec {
        gadgets: vec![EmlGadgetInstanceSpec::WeightedAccumulator {
            id: "expansion_need_weighted_accumulator".into(),
            input_cols: vec![0],
            weight_cols: vec![10],
            output_col: Some(12),
        }],
    }
}

fn open_session(
    weight_seeds: Vec<f32>,
    input_literal: f32,
    threshold: f32,
    misbind: bool,
) -> Result<SimSession, String> {
    let mut registry = DimensionRegistry::new();
    register_flow(&mut registry);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let session_root = SimThing::new(SimThingKind::Cohort, 0);
    let owner = SimThing::new(SimThingKind::Cohort, 0);
    let child = SimThing::new(SimThingKind::Cohort, 0);
    let ids = [session_root.id, owner.id, child.id];
    root.add_child(session_root);
    root.add_child(owner);
    root.add_child(child);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot = |id: SimThingId| allocator.slot_of(id).expect("hosted").raw();

    let install = if misbind {
        InstallTargetSpec::ScenarioListed {
            target_id: "missing_owner".into(),
        }
    } else {
        InstallTargetSpec::ScenarioListed {
            target_id: "owner".into(),
        }
    };

    let binding = binding_from_hydrated_stack(
        "expansion_need",
        "expansion-need",
        expansion_stack(),
        "foundry",
        install,
        weight_seeds,
        vec![NeedWeightProfileInputSpec::Literal(input_literal)],
        Some(NeedWeightProfileThresholdSpec {
            threshold,
            event_kind: 91,
        }),
    );

    let mut install_targets = HashMap::new();
    install_targets.insert("owner".into(), vec![ids[1]]);
    install_targets.insert("root".into(), vec![ids[0]]);

    let game_mode = GameModeSpec {
        id: "rf5_need_transport".into(),
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
            arenas: vec![ArenaSpec {
                name: "foundry".into(),
                flow_property: PropertyKey::new("workshop", "foundry_flow"),
                balance_property: Some(PropertyKey::new("workshop", "foundry_flow")),
                max_participants: 8,
                max_coupling_fanout: 4,
                max_orderband_depth: 16,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                reserved_gap_per_intermediate: 0,
                expected_max_children_per_intermediate: 0,
                explicit_participants: vec![
                    ExplicitParticipantSpec::flat(slot(ids[0]), ids[0].raw()),
                    ExplicitParticipantSpec::nested(slot(ids[1]), ids[1].raw(), ids[0].raw() as u64),
                    ExplicitParticipantSpec::nested(slot(ids[2]), ids[2].raw(), ids[1].raw() as u64),
                ],
                enrollment: None,
                wildcard_admission: None,
            }],
            base_obligations: vec![
                BaseFlowObligationSpec {
                    id: "root_budget".into(),
                    arena: "foundry".into(),
                    install: InstallTargetSpec::ScenarioListed {
                        target_id: "root".into(),
                    },
                    direction: BaseFlowDirectionSpec::Produce,
                    rate: 10.0,
                },
                BaseFlowObligationSpec {
                    id: "child_intrinsic".into(),
                    arena: "foundry".into(),
                    install: InstallTargetSpec::ScenarioListed {
                        target_id: "owner".into(),
                    },
                    direction: BaseFlowDirectionSpec::Produce,
                    rate: 3.0,
                },
            ],
            need_weight_profiles: vec![binding],
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: ResourceFlowExecutionProfile::RecursiveArenaResourceFlow,
        region_fields: vec![],
        mapping_execution_profile: Default::default(),
    };

    let scenario = Scenario {
        name: "rf5_need_transport".into(),
        ticks_per_day: 8,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets,
    };

    SimSession::open_from_spec(scenario, &game_mode).map_err(|e| format!("{e}"))
}

fn read_need(sim: &SimSession) -> f32 {
    let binding = sim
        .spec_state
        .resolved_need_weight_profiles
        .first()
        .expect("resolved need binding");
    let values = sim.state.read_values();
    let n_dims = sim.state.n_dims as usize;
    values[binding.participant_slot as usize * n_dims + binding.need_col as usize]
}

#[test]
fn paired_weight_seeds_change_live_need_and_threshold() {
    let mut low = match open_session(vec![0.2], 2.0, 1.5, false) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") || e.to_lowercase().contains("gpu") => {
            panic!("RF-5: GPU/adapter Unsupported is FAIL (not skip): {e}");
        }
        Err(e) => panic!("open low: {e}"),
    };
    let mut high = match open_session(vec![1.0], 2.0, 1.5, false) {
        Ok(s) => s,
        Err(e) if e.to_lowercase().contains("adapter") || e.to_lowercase().contains("gpu") => {
            panic!("RF-5: GPU/adapter Unsupported is FAIL (not skip): {e}");
        }
        Err(e) => panic!("open high: {e}"),
    };

    low.step_once().expect("step low");
    high.step_once().expect("step high");

    let need_low = read_need(&low);
    let need_high = read_need(&high);
    assert!(
        (need_low - 0.4).abs() < 1e-3,
        "low need expected ~0.4 got {need_low}"
    );
    assert!(
        (need_high - 2.0).abs() < 1e-3,
        "high need expected ~2.0 got {need_high}"
    );
    assert!(need_high > need_low);

    let thr = 1.5;
    assert!(need_low < thr, "below-threshold control");
    assert!(need_high >= thr, "crossing control");

    let b = high.spec_state.resolved_need_weight_profiles.first().unwrap();
    assert_eq!(b.profile, "expansion-need");
    assert_eq!(b.weight_seeds, vec![1.0]);
}

#[test]
fn empty_weight_seeds_fail_closed() {
    match open_session(vec![], 2.0, 1.5, false) {
        Ok(_) => panic!("empty weight_seeds must fail closed"),
        Err(err) => {
            assert!(
                err.contains("weight_seeds empty") || err.to_lowercase().contains("need weight"),
                "unexpected error: {err}"
            );
        }
    }
}

#[test]
fn misbound_install_target_fails_closed() {
    match open_session(vec![1.0], 2.0, 1.5, true) {
        Ok(_) => panic!("misbound install target must fail closed"),
        Err(err) => {
            assert!(
                err.contains("not admitted")
                    || err.contains("NoMatchingOwners")
                    || err.contains("not defined")
                    || err.to_lowercase().contains("need weight")
                    || err.to_lowercase().contains("install"),
                "unexpected error: {err}"
            );
        }
    }
}
