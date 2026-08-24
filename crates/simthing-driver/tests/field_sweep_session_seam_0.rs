//! FIELD-SWEEP-SESSION-SEAM-0 — ordinary-session consumption of admitted
//! PALMA / Gu-Yang products through the existing field-sweep executor.
//! HD-RECEIPT: `d08d00d27308`
//! DIMENSION-FINALIZATION-SEAM-0 HD-RECEIPT: `4ed070f7ace4`

use std::cell::Cell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, ColumnIndex, DimensionRegistry, LogTier,
    SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep,
    compile_stead_overlay_parameterized_n4_field_sweep, preview_install,
    ComparativeProjectionBands, FirstSliceSeed, GuYangN4FieldSweepSpec, PalmaN4FieldSweepSpec,
    Scenario, SimSession, SteadOverlayParameterizedN4Spec,
};
use simthing_gpu::{FieldSweepAdmissionError, FieldSweepRegistration, SlotAllocator};
use simthing_spec::{
    compile_property, ArenaPressureBindingSpec, ArenaSpec, ExplicitParticipantSpec,
    FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec, FissionPolicySpec, GameModeSpec,
    MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec, PropertyKey, PropertySpec,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec, RegionFieldSummaryPolicySpec, ResourceFlowOptInMode, ResourceFlowSpec,
    SpecVersion,
};

fn col(raw: u32) -> ColumnIndex {
    ColumnIndex::from_gpu_round_trip(raw)
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

fn region_field(name: &str, source_col: u32, target_col: u32, n_dims: u32) -> RegionFieldSpec {
    RegionFieldSpec {
        name: name.into(),
        grid_size: 2,
        n_dims,
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

fn ordinary_fixture() -> (Scenario, GameModeSpec) {
    let mut registry = DimensionRegistry::new();
    compile_property(
        &PropertySpec {
            id: "flow".into(),
            namespace: "seam".into(),
            name: "flow".into(),
            display_name: String::new(),
            description: String::new(),
            admission_disposition: Default::default(),
            sub_fields: vec![
                flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
                flow_subfield(
                    "allocated",
                    AccumulatorRole::AllocatedFlow {
                        arena: "seam_arena".into(),
                    },
                ),
                flow_subfield(
                    "weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: "seam_arena".into(),
                    },
                ),
            ],
        },
        &mut registry,
    )
    .expect("flow property");
    while registry.total_columns < 24 {
        let i = registry.total_columns;
        registry.register(SimProperty::simple("seam_pad", &format!("c{i}"), 1));
    }

    let mut root = SimThing::new(SimThingKind::World, 0);
    let participant = SimThing::new(SimThingKind::Cohort, 0);
    let participant_id = participant.id;
    root.add_child(participant);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let participant_slot = allocator
        .slot_of(participant_id)
        .expect("participant slot")
        .raw();

    let scenario = Scenario {
        name: "field_sweep_session_seam_0".into(),
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

    let mut first = region_field("emitter_a", 0, 1, 128);
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
        event_kind: 0x4653_5300,
        effect: None,
    });
    first.pressure_binding = Some(ArenaPressureBindingSpec {
        arena: "seam_arena".into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements: vec![PressurePlacementSpec {
            target_id: "pressure_source".into(),
            row: 0,
            col: 0,
        }],
    });
    let second = region_field("emitter_b", 2, 3, 128);

    let game_mode = GameModeSpec {
        id: "field_sweep_session_seam_0".into(),
        display_name: String::new(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: Vec::new(),
        overlays: Vec::new(),
        order_weight_classes: Vec::new(),
        capability_trees: Vec::new(),
        events: Vec::new(),
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "seam_arena".into(),
                flow_property: PropertyKey::new("seam", "flow"),
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
            opt_in_mode: ResourceFlowOptInMode::FlatStarOptIn,
            ..Default::default()
        }),
        resource_economy: None,
        resource_flow_execution_profile: Default::default(),
        region_fields: vec![first, second],
        mapping_execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
    };

    (scenario, game_mode)
}

fn admitted_products(n_dims: u32) -> Result<Vec<FieldSweepRegistration>, FieldSweepAdmissionError> {
    let palma = compile_palma_n4_field_sweep(PalmaN4FieldSweepSpec {
        width: 2,
        height: 2,
        n_dims,
        d_col: col(10),
        w_col: col(13),
        destination_slot: SlotIndex::new(0),
        inf_sentinel: f32::MAX,
    })?;
    let guyang = compile_gu_yang_n4_field_sweeps(GuYangN4FieldSweepSpec {
        width: 2,
        height: 2,
        n_dims,
        value_col: col(11),
        conductance_col: col(12),
        saturation: 1.0,
        chi: 0.1,
        dt: 1.0,
    })?;
    // Keep the ordinary one-shot source alive for the configured propagation
    // hop using another admitted registration through the same executor.
    let resident_copy =
        compile_stead_overlay_parameterized_n4_field_sweep(SteadOverlayParameterizedN4Spec {
            width: 2,
            height: 2,
            n_dims,
            source_col: col(1),
            falloff_col: col(14),
            output_col: col(0),
            dt: 1.0,
        })?;
    Ok(vec![
        palma,
        guyang[0].clone(),
        guyang[1].clone(),
        resident_copy,
    ])
}

#[test]
fn ordinary_session_executes_admitted_palma_guyang_and_observes_comparative() {
    let (scenario, game_mode) = ordinary_fixture();
    let product_count = 4usize;
    let authored_n_dims: Vec<_> = game_mode
        .region_fields
        .iter()
        .map(|field| field.n_dims)
        .collect();
    let compiled_n_dims = Cell::new(None);
    let mut session = match SimSession::open_from_spec_with_admitted_field_sweeps(
        scenario,
        &game_mode,
        |n_dims| {
            compiled_n_dims.set(Some(n_dims));
            admitted_products(n_dims)
        },
        (col(10), col(11), col(12)),
        ComparativeProjectionBands::default(),
        None,
    ) {
        Ok(session) => session,
        Err(simthing_driver::SessionError::Gpu(_))
            if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_none() =>
        {
            return;
        }
        Err(error) => panic!("ordinary production session must open: {error}"),
    };
    assert_eq!(
        game_mode
            .region_fields
            .iter()
            .map(|field| field.n_dims)
            .collect::<Vec<_>>(),
        authored_n_dims,
        "authored RegionFieldSpec dimensions must remain untouched"
    );
    assert_eq!(
        compiled_n_dims.get(),
        Some(session.proto.registry.total_columns as u32),
        "caller registrations must compile at the live post-admission registry width"
    );
    let adapter = session.state.ctx.adapter.get_info();
    eprintln!(
        "DIMENSION-FINALIZATION-SEAM-0 adapter={} backend={:?}",
        adapter.name, adapter.backend
    );

    let comparative = session
        .spec_state
        .comparative_projection
        .as_ref()
        .expect("FIELD-SWEEP-SESSION-SEAM-ASSIGNMENT-REMOVAL: production path must assign comparative_projection");
    let comparative_registration_count = comparative.bundle.registrations.len();
    let margin_col = comparative.outputs.margin_col;
    let base_registration_count = 1usize;
    let n_dims = session
        .mapping
        .as_ref()
        .expect("ordinary mapping state")
        .hot
        .mapping
        .preview()
        .stencil
        .n_dims;
    assert_eq!(
        n_dims,
        compiled_n_dims.get().expect("caller compiler width"),
        "ordinary mapping and caller registrations must share the one finalized width"
    );

    let mapping = session.mapping.as_mut().expect("ordinary mapping state");
    assert_eq!(
        mapping.hot.mapping.field_registration_count(),
        base_registration_count + product_count + comparative_registration_count,
        "all caller-admitted and comparative registrations must attach to the existing chain"
    );
    let dispatches_before = mapping.hot.mapping.field_registration_dispatches();
    mapping
        .hot
        .mapping
        .queue_seeds(&[FirstSliceSeed {
            row: 0,
            col: 0,
            value: 8.0,
        }])
        .expect("queue ordinary field seed");

    let step = session.step_once().expect("ordinary production tick");
    assert_eq!(step.ticks_run, 1);
    let mapping = session.mapping.as_ref().expect("mapping survives tick");
    let dispatches_after = mapping.hot.mapping.field_registration_dispatches();
    assert!(
        dispatches_after > dispatches_before,
        "PALMA and Gu-Yang products must execute through FieldSweepSession telemetry"
    );
    let values = mapping
        .hot
        .mapping
        .readback_canonical_field(&session.state.ctx);
    assert!(
        values
            .chunks_exact(n_dims as usize)
            .any(|row| row[margin_col.raw()] > 0.0),
        "comparative margin must be observable after the ordinary session tick"
    );
}

#[test]
fn session_seam_mutant_and_shape_seals_remain_closed() {
    comparative_assignment_removal_mutant_guard();
    second_field_execution_path_mutant_guard();
    ordinary_install_never_defaults_triad_columns();
    comparative_observable_names_have_no_public_driver_functions();
}

#[test]
fn dimension_finalization_single_authority_and_no_prediction_seals() {
    dimension_finalization_authority_mutant_guard();
    caller_width_workaround_removal_guard();
}

fn dimension_finalization_authority_mutant_guard() {
    let sources = production_rust_sources();
    let authorities: Vec<_> = sources
        .iter()
        .flat_map(|(path, source)| {
            source.lines().enumerate().filter_map(move |(line, text)| {
                text.contains("DIMENSION-FINALIZATION-SEAM-0-AUTHORITY")
                    .then_some((path.clone(), line + 1))
            })
        })
        .collect();
    assert_eq!(
        authorities.len(),
        1,
        "DIMENSION-FINALIZATION-SEAM-0-SECOND-AUTHORITY: exactly one production site may own the post-admission registry width; got {authorities:?}"
    );
}

fn caller_width_workaround_removal_guard() {
    let witness = include_str!("field_sweep_session_seam_0.rs");
    let authored_rewrite = ["field.n_", "dims ="].concat();
    let projection_token = ["projected_", "registry"].concat();
    assert!(
        !witness.contains(&authored_rewrite) && !witness.contains(&projection_token),
        "DIMENSION-FINALIZATION-SEAM-0-CALLER-WIDTH-WORKAROUND: the witness must neither rewrite authored field dimensions nor project the final registry width"
    );
}

fn comparative_assignment_removal_mutant_guard() {
    let source = include_str!("../src/session.rs");
    assert_eq!(
        source
            .matches("spec_state.comparative_projection = Some(comparative);")
            .count(),
        1,
        "FIELD-SWEEP-SESSION-SEAM-ASSIGNMENT-REMOVAL: removing the production assignment must RED"
    );
}

fn second_field_execution_path_mutant_guard() {
    let sources = production_rust_sources();
    let constructor_sites: Vec<_> = sources
        .iter()
        .flat_map(|(path, source)| {
            source.lines().enumerate().filter_map(move |(line, text)| {
                text.contains("FieldSweepSession::new(")
                    .then_some((path.clone(), line + 1))
            })
        })
        .collect();
    assert_eq!(
        constructor_sites.len(),
        1,
        "FIELD-SWEEP-SESSION-SEAM-SECOND-EXECUTION-PATH: only the contiguous-binding constructor at the existing mapping attach point may construct FieldSweepSession; got {constructor_sites:?}"
    );
    assert!(
        constructor_sites
            .iter()
            .all(|(path, _)| path.ends_with("mapping_runtime.rs")),
        "FIELD-SWEEP-SESSION-SEAM-SECOND-EXECUTION-PATH: constructor escaped mapping_runtime.rs: {constructor_sites:?}"
    );
    assert!(
        sources
            .iter()
            .all(|(_, source)| !source.contains("execute_field_sweep_cpu_chain(")),
        "FIELD-SWEEP-SESSION-SEAM-SECOND-EXECUTION-PATH: CPU proof executor entered production source"
    );
}

fn ordinary_install_never_defaults_triad_columns() {
    let install = include_str!("../src/install.rs");
    assert!(
        !install.contains("admit_comparative_from_field_plan")
            && !install.contains("comparative_projection =")
            && !install.contains("triad_columns"),
        "FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT: ordinary install must only mint the field plan; Triad columns stay explicit consumer inputs"
    );

    let (scenario, game_mode) = ordinary_fixture();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&scenario.root);
    let preview = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &allocator,
    )
    .expect("ordinary install preview");
    assert!(preview.state.field_plan_admission.is_some());
    assert!(
        preview.state.comparative_projection.is_none(),
        "FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT: install planted a comparative default"
    );
}

fn comparative_observable_names_have_no_public_driver_functions() {
    let forbidden = ["chokepoint", "corridor", "front", "dominance"];
    let mut surfaces = Vec::new();
    for (path, source) in production_rust_sources() {
        for (line, text) in source.lines().enumerate() {
            let trimmed = text.trim_start().to_ascii_lowercase();
            if trimmed.starts_with("pub fn ") && forbidden.iter().any(|name| trimmed.contains(name))
            {
                surfaces.push((path.clone(), line + 1, text.trim().to_string()));
            }
        }
    }
    assert!(
        surfaces.is_empty(),
        "FIELD-SWEEP-SESSION-SEAM-OBSERVABLE-SHAPE: comparative observables must not mint public production functions: {surfaces:?}"
    );
}

fn production_rust_sources() -> Vec<(PathBuf, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rust_sources(&root, &mut files);
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let source = fs::read_to_string(&path).expect("read production Rust source");
            (path, source)
        })
        .collect()
}

fn collect_rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read production source directory") {
        let path = entry.expect("source entry").path();
        if path.is_dir() {
            collect_rust_sources(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}
