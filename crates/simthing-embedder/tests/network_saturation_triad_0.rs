//! Network-saturation full-Triad exemplar for the Embedder Guide.
//!
//! Competing load classes and admitted PALMA / Gu-Yang topology birth
//! comparative outputs through the five-verb door. Volume-delay is the
//! admitted `PowerLaw` gadget. No POW, no hand-rolled EXP/LN, no generic
//! threshold substitute for the Triad.

use std::cell::Cell;
use std::collections::HashMap;

use simthing_embedder::populate::{
    AccumulatorRole, AccumulatorSpec, ArenaPressureBindingSpec, ArenaSpec, ClampBehavior,
    DimensionRegistry, ExplicitParticipantSpec, FirstSliceCommitmentDirectionSpec,
    FirstSliceCommitmentSpec, FissionPolicySpec, LogTier, MappingExecutionProfile,
    PressurePlacementSpec, PressureSourceSpec, PropertyKey, PropertySpec, RegionFieldCadenceSpec,
    RegionFieldFormulaBindingSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
    RegionFieldSummaryPolicySpec, ResourceFlowSpec, SimProperty, SimThing, SimThingKind,
    SlotAllocator, SubFieldRole, SubFieldSpec, TransformOp,
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
    let flow_property_id = populate::compile_property(
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

    let mut root = SimThing::new(SimThingKind::World, 0);
    let participant = SimThing::new(SimThingKind::Cohort, 0);
    let participant_id = participant.id;
    root.add_child(participant);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    let participant_slot = allocator
        .slot_of(participant_id)
        .expect("participant slot")
        .raw();

    let scenario = run::Scenario {
        name: "network-saturation".into(),
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
        event_kind: 0x5644_5400,
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
        id: "network-saturation".into(),
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

#[test]
fn volume_delay_power_law_reds_a_staircase_rival() {
    let law = derive::EmlGadgetStackSpec {
        gadgets: vec![derive::EmlGadgetInstanceSpec::PowerLaw {
            id: "volume-delay".into(),
            input_col: 0,
            output_col: Some(1),
            exponent: 4.0,
            input_floor: 0.25,
        }],
    };
    let compiled =
        derive::compile_eml_gadget_stack(&law, derive::EmlGadgetCompileOptions { max_col: 2 })
            .expect("admitted PowerLaw");
    assert_eq!(compiled.report.gadget_kinds, ["PowerLaw"]);

    let unsafe_floor = derive::EmlGadgetStackSpec {
        gadgets: vec![derive::EmlGadgetInstanceSpec::PowerLaw {
            id: "unsafe-floor".into(),
            input_col: 0,
            output_col: None,
            exponent: 4.0,
            input_floor: 0.0,
        }],
    };
    assert!(
        derive::compile_eml_gadget_stack(
            &unsafe_floor,
            derive::EmlGadgetCompileOptions { max_col: 1 },
        )
        .is_err(),
        "positive-floor semantics must RED an LN-unsafe authored law"
    );
}

#[test]
fn network_saturation_triad_bands_are_born_from_the_tree() {
    let _seat = derive::owner_seat("alpha", "Alpha", "carrier").expect("owner seat");
    let (mut scenario, game_mode, participant_id) = saturation_fixture();
    let authored_bound = scenario.registry.total_columns as u32;
    let declared_emitters = [
        derive::ComparativeEmitterClass {
            authored_order: 0,
            class_id: 0.0,
            value_col: col(1, authored_bound),
        },
        derive::ComparativeEmitterClass {
            authored_order: 1,
            class_id: 1.0,
            value_col: col(3, authored_bound),
        },
    ];
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
    .expect("PowerLaw reaches its production compiler through Derive");
    assert_eq!(compiled_law.report.gadget_kinds, ["PowerLaw"]);

    populate::ownership(&scenario.root).expect("Populate validates the authored tree");
    let overlay = overlay::authored(
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
        vec![overlay::DissolveCondition::AtSessionEnd],
    )
    .expect("Overlay admits an attributable finite-horizon declaration");
    scenario.root.children[0].add_overlay(overlay);

    let compiled_n_dims = Cell::new(None);
    let mut session = match run::initialize_with_admitted_field_sweeps(
        scenario,
        &game_mode,
        |n_dims| {
            compiled_n_dims.set(Some(n_dims));
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
    ) {
        Ok(session) => session,
        Err(run::InitializeError::Session(run::SessionError::Gpu(_)))
            if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_none() =>
        {
            return;
        }
        Err(error) => panic!("Run must delegate to the ordinary admitted-sweep seam: {error}"),
    };

    assert_eq!(
        session
            .spec_state
            .field_plan_admission
            .as_ref()
            .expect("ordinary install field plan")
            .emitters(),
        declared_emitters,
        "competing load classes must match production emitter admission"
    );

    run::start(&mut session, run::ExecutionPosture::Paced).expect("start");
    run::tick(&mut session).expect("tick");
    let observed = bind::observe_gu_yang_stall(&session).expect("born Gu-Yang stall");
    let admission = session
        .spec_state
        .comparative_projection
        .as_ref()
        .expect("born comparative projection");
    let _contest = admission.outputs.contest_col;
    let _border = admission.band_readouts.border_col;
    let _chokepoint = admission.band_readouts.chokepoint_col;
    let canonical = session
        .mapping
        .as_ref()
        .expect("ordinary mapping")
        .hot
        .mapping
        .readback_canonical_field(&session.state.ctx);
    let n_dims = session.state.n_dims as usize;
    let stall = admission.stall_outputs;
    assert_eq!(observed.len(), canonical.len() / n_dims);
    assert!(
        observed.iter().all(|sample| {
            let row = &canonical[sample.row as usize * n_dims..][..n_dims];
            sample.net_flux.to_bits() == row[stall.net_flux_col.raw()].to_bits()
                && sample.gross_flux.to_bits() == row[stall.gross_flux_col.raw()].to_bits()
                && sample.stall.to_bits() == row[stall.stall_col.raw()].to_bits()
        }),
        "Bind must copy live GuYangStallOutputs; contest/border/chokepoint are born columns"
    );
}
