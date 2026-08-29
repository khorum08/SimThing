//! 12.1 portability witness: one unrelated network domain crosses the complete
//! five-verb Vendor Door, then reopens and replays its own Run artifact.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use simthing_embedder::populate::{
    AccumulatorRole, AccumulatorSpec, ArenaPressureBindingSpec, ArenaSpec, ClampBehavior,
    DimensionRegistry, ExplicitParticipantSpec, FirstSliceCommitmentDirectionSpec,
    FirstSliceCommitmentSpec, FissionPolicySpec, LogTier, MappingExecutionProfile,
    PressurePlacementSpec, PressureSourceSpec, PropertyKey, PropertySpec, PropertyValue,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec, RegionFieldSummaryPolicySpec, ResourceFlowSpec, SimProperty, SimThing,
    SimThingKind, SlotAllocator, SubFieldRole, SubFieldSpec, TransformOp,
};
use simthing_embedder::{bind, derive, overlay, populate, run};

const AUTHORED_BASE_DIMS: u32 = 25;

fn col(raw: u32, bound: u32) -> bind::ColumnIndex {
    bind::authored_column(raw, bound).expect("bounded authored column")
}

fn flow_subfield(name: &str, role: AccumulatorRole, default: f32) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default,
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

fn region_field(name: &str, source_col: u32, target_col: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: name.into(),
        grid_size: 2,
        n_dims: AUTHORED_BASE_DIMS,
        source_col,
        target_col,
        operator: RegionFieldOperatorSpec::Normalized,
        horizon: 1,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 1.0,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: None,
        parent_formula: None,
        commitment: None,
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: RegionFieldSummaryPolicySpec::default(),
        pressure_binding: None,
    }
}

fn saturation_fixture() -> (run::Scenario, run::GameModeSpec, populate::SimThingId) {
    let mut registry = DimensionRegistry::new();
    let (flow_property_id, _diagnostics) = populate::compile_property(
        &PropertySpec {
            id: "flow".into(),
            namespace: "vendor".into(),
            name: "flow".into(),
            display_name: String::new(),
            description: String::new(),
            admission_disposition: Default::default(),
            sub_fields: vec![
                flow_subfield("flow", AccumulatorRole::IntrinsicFlow, 8.0),
                flow_subfield(
                    "allocated",
                    AccumulatorRole::AllocatedFlow {
                        arena: "load_arena".into(),
                    },
                    0.0,
                ),
                flow_subfield(
                    "weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: "load_arena".into(),
                    },
                    1.0,
                ),
            ],
        },
        &mut registry,
    )
    .expect("flow property");
    while registry.total_columns < 24 {
        let i = registry.total_columns;
        registry.register(SimProperty::simple("vendor_pad", &format!("c{i}"), 1));
    }

    let flow_layout = registry.property(flow_property_id).layout.clone();
    let mut flow_value = PropertyValue::from_layout(&flow_layout);
    flow_value.set_role(&SubFieldRole::Named("flow".into()), &flow_layout, 8.0);
    flow_value.set_role(&SubFieldRole::Named("weight".into()), &flow_layout, 1.0);

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut participant = SimThing::new(SimThingKind::Cohort, 0);
    participant.add_property(flow_property_id, flow_value);
    let participant_id = participant.id;
    root.add_child(participant);
    let mut allocator = SlotAllocator::new();
    allocator
        .install_initial_tree(&root)
        .expect("fixture tree slots");
    let participant_slot = allocator
        .slot_of(participant_id)
        .expect("participant slot")
        .raw();

    let scenario = run::Scenario {
        name: "network-saturation-portability".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 16,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::from([("pressure_source".into(), vec![participant_id])]),
    };

    let mut first = region_field("load_class_a", 0, 1);
    first.reduction = Some(RegionFieldReductionSpec {
        child_slot_start: 0,
        child_slot_count: 4,
        child_col: 1,
        parent_slot: 4,
        parent_col: 1,
        order_band: 0,
    });
    first.parent_formula = Some(RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: Some(1),
        weight_pressure: Some(1.0),
        weight_resource: Some(0.0),
    });
    first.commitment = Some(FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: 4,
        urgency_col: 4,
        threshold: 10_000.0,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: 0x5050_1201,
        effect: None,
    });
    first.pressure_binding = Some(ArenaPressureBindingSpec {
        arena: "load_arena".into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements: vec![PressurePlacementSpec {
            target_id: "pressure_source".into(),
            row: 0,
            col: 0,
        }],
    });

    let game_mode = run::GameModeSpec {
        id: "network-saturation-portability".into(),
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "load_arena".into(),
                flow_property: PropertyKey::new("vendor", "flow"),
                balance_property: None,
                max_participants: 4,
                max_coupling_fanout: 4,
                max_orderband_depth: 8,
                fission_policy: FissionPolicySpec::Reject,
                reserved_orderband_depth: 0,
                explicit_participants: vec![ExplicitParticipantSpec::flat(
                    participant_slot,
                    participant_id.raw(),
                )],
                enrollment: None,
                wildcard_admission: None,
            }],
            couplings: Vec::new(),
            ..Default::default()
        }),
        region_fields: vec![first, region_field("load_class_b", 2, 3)],
        mapping_execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
        ..Default::default()
    };

    let _ = flow_property_id;
    (scenario, game_mode, participant_id)
}

fn initialize_saturation(
    scenario: run::Scenario,
    game_mode: &run::GameModeSpec,
    authored_bound: u32,
) -> Result<run::SimSession, run::InitializeError> {
    run::initialize_with_admitted_field_sweeps(
        scenario,
        game_mode,
        |n_dims| {
            let palma = bind::compile_palma_n4_field_sweep(bind::PalmaN4FieldSweepSpec {
                width: 2,
                height: 2,
                n_dims,
                d_col: col(10, authored_bound),
                w_col: col(13, authored_bound),
                destination_slot: bind::SlotIndex::new(0),
                inf_sentinel: f32::MAX,
            })?;
            let guyang = bind::compile_gu_yang_n4_field_sweeps(bind::GuYangN4FieldSweepSpec {
                width: 2,
                height: 2,
                n_dims,
                value_col: col(0, authored_bound),
                conductance_col: col(12, authored_bound),
                saturation: 1.0,
                chi: 0.1,
                dt: 1.0,
            })?;
            Ok(vec![palma, guyang[0].clone(), guyang[1].clone()])
        },
        (
            col(10, authored_bound),
            col(0, authored_bound),
            col(12, authored_bound),
        ),
        bind::ComparativeProjectionBands::default(),
        None,
    )
}

fn bind_network_checkpoint(session: &mut run::SimSession, participant_id: populate::SimThingId) {
    let flow_property = session
        .proto
        .registry
        .id_of("vendor", "flow")
        .expect("flow property");
    let (_, cost_band) =
        populate::queued_cost_band(1.0, Some(1), None).expect("bounded checkpoint event");
    bind::velocity_threshold(
        session,
        bind::VelocityAlertRegistration {
            sim_thing_id: participant_id,
            property_id: flow_property,
            sub_field: SubFieldRole::Named("flow".into()),
            threshold: 0.5,
            direction: bind::Direction::Rising,
            cost_band,
        },
    );
}

struct ReplayArtifact(PathBuf);

impl ReplayArtifact {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        Self(std::env::temp_dir().join(format!(
            "simthing-portability-{}-{nonce}.ldjson",
            std::process::id()
        )))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ReplayArtifact {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[test]
fn unrelated_network_domain_round_trips_through_the_five_verb_vendor_door() {
    let seat = derive::owner_seat("network-operator", "Network Operator", "carrier")
        .expect("Derive authors a non-game owner seat");
    assert_eq!(seat.kind, SimThingKind::Owner);

    let law = derive::EmlGadgetStackSpec {
        gadgets: vec![derive::EmlGadgetInstanceSpec::PowerLaw {
            id: "volume-delay".into(),
            input_col: 0,
            output_col: Some(5),
            exponent: 4.0,
            input_floor: 0.25,
        }],
    };
    let compiled_law = derive::compile_eml_gadget_stack(
        &law,
        derive::EmlGadgetCompileOptions {
            max_col: AUTHORED_BASE_DIMS,
        },
    )
    .expect("Derive admits the network volume-delay law");
    assert_eq!(compiled_law.report.gadget_ids, ["volume-delay"]);

    let (mut scenario, game_mode, participant_id) = saturation_fixture();
    populate::ownership(&scenario.root).expect("Populate admits the authored tree");
    let authored_bound = scenario.registry.total_columns as u32;

    let directive = overlay::authored(
        &scenario.root,
        &scenario.root,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![participant_id],
        overlay::PropertyTransformDelta {
            property_id: scenario
                .registry
                .id_of("vendor", "flow")
                .expect("flow property"),
            sub_field_deltas: vec![(SubFieldRole::Named("flow".into()), TransformOp::set(8.0))],
        },
        vec![overlay::DissolveCondition::AfterTicks { remaining: 2 }],
    )
    .expect("Overlay admits an attributable finite-horizon modifier");
    assert_eq!(directive.affects, vec![participant_id]);
    scenario.root.children[0].add_overlay(directive);

    let replay_scenario = scenario.clone();
    let replay_game_mode = game_mode.clone();
    let mut session = match initialize_saturation(scenario, &game_mode, authored_bound) {
        Ok(session) => session,
        Err(run::InitializeError::Session(run::SessionError::Gpu(_)))
            if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_none() =>
        {
            return;
        }
        Err(error) => panic!("Run must initialize through the ordinary admitted seam: {error}"),
    };
    assert!(
        session.spec_state.field_plan_admission.is_some(),
        "Bind's admitted PALMA/Gu-Yang plan must be installed on Run"
    );
    bind_network_checkpoint(&mut session, participant_id);

    run::start(&mut session, run::ExecutionPosture::Paced).expect("Run starts the session");
    let artifact = ReplayArtifact::new();
    let summary = run::serialize(&mut session, artifact.path(), 1)
        .expect("Run serializes the ordinary session history");
    assert_eq!(summary.frames_written, 1);

    let live_observed = bind::observe_gu_yang_stall(&session)
        .expect("Bind observes the live STEAD/Triad result read-only");
    assert!(
        live_observed.iter().any(|sample| {
            sample.net_flux.to_bits() != 0
                || sample.gross_flux.to_bits() != 0
                || sample.stall.to_bits() != 0
        }),
        "the ordinary GPU/session execution must produce a nontrivial authoritative observable"
    );
    let live_canonical = session
        .mapping
        .as_ref()
        .expect("ordinary mapping")
        .hot
        .mapping
        .readback_canonical_field(&session.state.ctx);
    let live_outputs = session
        .spec_state
        .comparative_projection
        .as_ref()
        .expect("live comparative admission")
        .stall_outputs;
    let live_n_dims = session.state.n_dims as usize;
    assert!(live_observed.iter().all(|sample| {
        let row = &live_canonical[sample.row as usize * live_n_dims..][..live_n_dims];
        sample.net_flux.to_bits() == row[live_outputs.net_flux_col.raw()].to_bits()
            && sample.gross_flux.to_bits() == row[live_outputs.gross_flux_col.raw()].to_bits()
            && sample.stall.to_bits() == row[live_outputs.stall_col.raw()].to_bits()
    }));
    let live_shadow_row: Vec<_> = bind::shadow(&session)
        .row(participant_id)
        .expect("authoritative participant shadow row")
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert!(
        live_shadow_row.iter().any(|bits| *bits != 0),
        "the domain state promised by the replay law must be nontrivial"
    );

    let loaded = run::read_spec_replay_file(artifact.path())
        .expect("Run reads the artifact through the same verb surface");
    assert_eq!(loaded.frames.len(), summary.frames_written as usize);
    let run::LoadedReplay {
        structural_snapshot,
        spec_snapshot,
        frames,
    } = loaded;
    let mut replay_session =
        initialize_saturation(replay_scenario, &replay_game_mode, authored_bound)
            .expect("Run reopens the same admitted-Triad domain spec");
    bind_network_checkpoint(&mut replay_session, participant_id);
    if let Some(snapshot) = spec_snapshot {
        run::apply_spec_snapshot(&mut replay_session.spec_state, &snapshot)
            .expect("Run restores the recorded spec snapshot");
    }
    let mut replay_driver = run::ReplayDriver::from_snapshot(structural_snapshot)
        .expect("Run reopens the structural replay snapshot");
    assert_eq!(frames.len(), summary.frames_written as usize);

    for (frame, deltas) in frames {
        for delta in deltas {
            run::apply_spec_delta(&mut replay_session.spec_state, &delta, &[])
                .expect("Run replays the recorded spec delta");
        }
        replay_driver
            .try_apply_frame(frame)
            .expect("Run replays the canonical structural frame");
    }

    let replayed = replay_driver
        .shadow_values
        .as_ref()
        .expect("replay carries the authoritative post-boundary shadow");
    let replay_slot = replay_driver
        .allocator
        .slot_of(participant_id)
        .expect("replayed participant slot")
        .raw() as usize;
    let replay_n_dims = replay_session.state.n_dims as usize;
    let replayed_row = &replayed[replay_slot * replay_n_dims..][..replay_n_dims];
    assert_eq!(
        replayed_row
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        live_shadow_row,
        "serialized/replayed authoritative domain state must be bit-exact under the existing replay law"
    );
}
