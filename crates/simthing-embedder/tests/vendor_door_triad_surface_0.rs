//! VENDOR-DOOR-TRIAD-SURFACE-0 — five-verb reach to the graduated 11.1a seam.
//! HD-RECEIPT: `622933c70c88`

use std::cell::Cell;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

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

fn non_residency_market_spec(offering_ref: &str) -> derive::SpecializationFlowMarketSpec {
    derive::SpecializationFlowMarketSpec {
        specialization_profile_id: "vendor-compute".into(),
        offerings: vec![derive::ConservedOfferingSpec {
            id: "compute-offering".into(),
            resource_key: populate::ResourceKey::new("vendor-compute-cycles"),
            price: derive::OfferingPriceVectorSpec {
                unit_cost: 1.5,
                default_clearing_weight: 1.0,
            },
        }],
        draw_envelopes: vec![derive::DrawEnvelopeTemplateSpec {
            id: "compute-draw".into(),
            offering_refs: vec![offering_ref.into()],
            lifecycle_trigger_refs: vec!["while-computing".into()],
            min_quantity: 1,
            max_quantity: 8,
        }],
    }
}

fn compute_scope(granter: populate::SimThingId) -> populate::OwnerChannelScopeKey {
    populate::OwnerChannelScopeKey {
        owner_ref: populate::OwnerRef::new(format!("vendor/{}", granter.raw())),
        resource_key: populate::ResourceKey::new("vendor-compute-cycles"),
        scope_id: populate::ScopeId::from_boundary(granter),
    }
}

fn compute_demand(
    scope: &populate::OwnerChannelScopeKey,
    grantee: populate::SimThingId,
    requested: u32,
) -> populate::RuntimeOwnerSiloDemandBucket {
    populate::RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref.clone(),
        resource_key: scope.resource_key.clone(),
        scope_id: scope.scope_id.clone(),
        requested,
        priority: 0,
        source_simthing_id_raw: Some(grantee.raw()),
    }
}

fn recursive_non_residency_grant() -> u32 {
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    let mut child = SimThing::new(SimThingKind::Custom("compute-granter".into()), 0);
    let descendant = SimThing::new(SimThingKind::Custom("compute-worker".into()), 0);
    let root_id = root.id;
    let child_id = child.id;
    let descendant_id = descendant.id;
    child.add_child(descendant);
    root.add_child(child);

    let profile = derive::SpecializationProfile {
        id: "vendor-compute".into(),
        description: "arbitrary conserved compute lane".into(),
        requirements: Vec::new(),
    };
    let triggers = BTreeSet::from(["while-computing".to_string()]);
    assert!(
        derive::admit_specialization_flow_market(
            std::slice::from_ref(&profile),
            &triggers,
            non_residency_market_spec("missing-offering"),
        )
        .is_err(),
        "sealed Draw references must fail closed during existing admission"
    );
    let market = derive::admit_specialization_flow_market(
        &[profile],
        &triggers,
        non_residency_market_spec("compute-offering"),
    )
    .expect("Derive delegates strict offering and sealed Draw admission");

    let root_scope = compute_scope(root_id);
    let child_draw = market
        .authorize_draw(
            "compute-draw",
            "compute-offering",
            compute_demand(&root_scope, child_id, 6),
            1.0,
            &triggers,
        )
        .expect("descendant Draw uses the admitted offering reference");
    let child_claim =
        run::ConstrainedClaim::from_runtime_demand(&child_draw.demand, child_draw.order_weight)
            .expect("graduated constrained claim");
    let program = run::AuthoredClearingProgram::new(TransformOp::set(1.0));
    let root_clear = run::clear_constrained_claims_at_generation(
        &[populate::ConstrainedSupply {
            scope: root_scope,
            available: 8,
        }],
        &[child_claim],
        &program,
        run::ClearingRemainderAuthority {
            granter: root_id,
            generation: populate::GenerationStamp::new(4),
        },
    )
    .expect("Run delegates the existing conserved clear");
    let child_grant = &root_clear[0].grants[0];
    assert_eq!((child_grant.granted, root_clear[0].remaining_after), (6, 2));
    let mut lifecycle_schedule = derive::IntegrationSchedule::new();
    let accepted_child = market
        .record_cleared_grant(
            root_id,
            "compute-offering",
            child_grant,
            populate::GenerationStamp::new(4),
            &mut lifecycle_schedule,
        )
        .expect("existing market seals the accepted child grant");

    let child_scope = compute_scope(child_id);
    let descendant_draw = market
        .authorize_draw(
            "compute-draw",
            "compute-offering",
            compute_demand(&child_scope, descendant_id, 4),
            1.0,
            &triggers,
        )
        .expect("the same Draw grammar applies below the granted child");
    let descendant_claim = run::ConstrainedClaim::from_runtime_demand(
        &descendant_draw.demand,
        descendant_draw.order_weight,
    )
    .expect("same constrained claim grammar");
    let descendant_clear = run::clear_constrained_claims_at_generation(
        &[populate::ConstrainedSupply {
            scope: child_scope,
            available: accepted_child.quantity(),
        }],
        &[descendant_claim],
        &program,
        run::ClearingRemainderAuthority {
            granter: child_id,
            generation: populate::GenerationStamp::new(5),
        },
    )
    .expect("accepted child quantity becomes its conserved descendant budget");
    let descendant_grant = &descendant_clear[0].grants[0];
    assert_eq!(
        accepted_child.quantity(),
        descendant_grant.granted + descendant_clear[0].remaining_after,
        "recursive granting remains exactly conserved"
    );
    let accepted_descendant = market
        .record_cleared_grant(
            child_id,
            "compute-offering",
            descendant_grant,
            populate::GenerationStamp::new(5),
            &mut lifecycle_schedule,
        )
        .expect("same market lifecycle seals the descendant grant");
    let band = market
        .quantize_value("compute-offering", 6.5)
        .expect("existing scalar CostBand quantizes the non-residency offering");
    assert_eq!((band.n, band.r), (4, 0.5));
    accepted_descendant.quantity()
}

fn full_triad_fixture(
    granted_flow: f32,
) -> (run::Scenario, run::GameModeSpec, populate::SimThingId) {
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
                flow_subfield("flow", AccumulatorRole::IntrinsicFlow, granted_flow),
                flow_subfield(
                    "allocated",
                    AccumulatorRole::AllocatedFlow {
                        arena: "vendor_arena".into(),
                    },
                    0.0,
                ),
                flow_subfield(
                    "weight",
                    AccumulatorRole::AllocatorWeight {
                        arena: "vendor_arena".into(),
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
    allocator
        .install_initial_tree(&root)
        .expect("install fixture tree");
    let participant_slot = allocator
        .slot_of(participant_id)
        .expect("participant slot")
        .raw();

    let scenario = run::Scenario {
        name: "vendor_door_triad_surface_0".into(),
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

    let mut first = region_field("emitter_a", 0, 1);
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
        arena: "vendor_arena".into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements: vec![PressurePlacementSpec {
            target_id: "pressure_source".into(),
            row: 0,
            col: 0,
        }],
    });

    let game_mode = run::GameModeSpec {
        id: "vendor_door_triad_surface_0".into(),
        resource_flow: Some(ResourceFlowSpec {
            arenas: vec![ArenaSpec {
                name: "vendor_arena".into(),
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
        region_fields: vec![first, region_field("emitter_b", 2, 3)],
        mapping_execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
        ..Default::default()
    };

    assert_eq!(scenario.registry.total_columns, 25);
    let _ = flow_property_id;
    (scenario, game_mode, participant_id)
}

#[test]
fn five_verbs_reach_the_session_seam_and_observe_only_live_triad_output() {
    let granted_flow = recursive_non_residency_grant();
    let (mut scenario, game_mode, participant_id) = full_triad_fixture(granted_flow as f32);
    let authored_bound = scenario.registry.total_columns as u32;
    let authored_field_dimensions: Vec<_> = game_mode
        .region_fields
        .iter()
        .map(|field| field.n_dims)
        .collect();
    let triad_columns = (
        col(10, authored_bound),
        col(0, authored_bound),
        col(12, authored_bound),
    );
    let bands = bind::ComparativeProjectionBands::default();

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
        gadgets: vec![derive::EmlGadgetInstanceSpec::SoftStep {
            id: "vendor_saturation_law".into(),
            input_col: 0,
            output_col: Some(5),
            center: 0.5,
            steepness: 4.0,
        }],
    };
    let compiled_law = derive::compile_eml_gadget_stack(
        &law,
        derive::EmlGadgetCompileOptions {
            max_col: AUTHORED_BASE_DIMS,
        },
    )
    .expect("authored EML reaches its production compiler through Derive");
    assert_eq!(compiled_law.report.gadget_ids, ["vendor_saturation_law"]);

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
            sub_field_deltas: vec![(
                SubFieldRole::Named("flow".into()),
                TransformOp::set(granted_flow as f32),
            )],
        },
        vec![overlay::DissolveCondition::AtSessionEnd],
    )
    .expect("Overlay admits an attributable finite-horizon declaration");
    assert_eq!(overlay.affects, vec![participant_id]);
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
        triad_columns,
        bands,
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
    let adapter = session.state.ctx.adapter.get_info();
    eprintln!(
        "VENDOR-DOOR-TRIAD-SURFACE-0 adapter={} backend={:?}",
        adapter.name, adapter.backend
    );
    assert_eq!(
        game_mode
            .region_fields
            .iter()
            .map(|field| field.n_dims)
            .collect::<Vec<_>>(),
        authored_field_dimensions,
        "Run must leave vendor-authored field dimensions untouched"
    );
    assert_eq!(
        compiled_n_dims.get(),
        Some(session.proto.registry.total_columns as u32),
        "Bind compilers must receive the seam's one live finalized width"
    );
    assert_eq!(
        session
            .spec_state
            .field_plan_admission
            .as_ref()
            .expect("ordinary install field plan")
            .emitters(),
        declared_emitters,
        "Derive's public declaration shape must match the production emitter admission"
    );

    run::start(&mut session, run::ExecutionPosture::Paced).expect("same ordinary scheduler");
    let tick = run::tick(&mut session).expect("ordinary production tick");
    assert_eq!(tick.ticks_run, 1);

    let observed = bind::observe_gu_yang_stall(&session).expect("Bind observes admitted output");
    let outputs = session
        .spec_state
        .comparative_projection
        .as_ref()
        .expect("comparative admission")
        .stall_outputs;
    let canonical = session
        .mapping
        .as_ref()
        .expect("ordinary mapping")
        .hot
        .mapping
        .readback_canonical_field(&session.state.ctx);
    let n_dims = session.state.n_dims as usize;
    assert_eq!(observed.len(), canonical.len() / n_dims);
    assert!(
        observed.iter().all(|sample| {
            let row = &canonical[sample.row as usize * n_dims..][..n_dims];
            sample.net_flux.to_bits() == row[outputs.net_flux_col.raw()].to_bits()
                && sample.gross_flux.to_bits() == row[outputs.gross_flux_col.raw()].to_bits()
                && sample.stall.to_bits() == row[outputs.stall_col.raw()].to_bits()
        }),
        "VENDOR-DOOR-TRIAD-FABRICATED-OBSERVABLE: Bind must copy the live GuYangStallOutputs lanes exactly"
    );

    let replay = tempfile::NamedTempFile::new().expect("canonical replay file");
    let summary = run::serialize(&mut session, replay.path(), 1)
        .expect("Run preserves the one canonical replay/history path");
    assert_eq!(summary.frames_written, 1);
    assert!(replay.as_file().metadata().expect("replay metadata").len() > 0);
}

#[test]
fn recursive_non_residency_grant_uses_the_same_vendor_grammar() {
    assert_eq!(recursive_non_residency_grant(), 4);

    let extent = populate::ResidencyExtent::try_new(2, 4)
        .expect("11.2b physical authoring vocabulary coexists under Populate");
    assert_eq!((extent.start(), extent.length()), (2, 4));
    let _: fn(
        &mut run::SimSession,
        &derive::AdmittedSpecializationFlowMarket,
        &derive::MarketGrantRecord,
        populate::ResidencyExtent,
    ) -> Result<run::ResidencyPlacementOutcome, run::SessionError> =
        run::realize_market_grant_residency;
}

#[test]
fn facade_shape_column_and_actionband_seals_remain_closed() {
    let sources = embedder_production_sources();
    let witness = include_str!("vendor_door_triad_surface_0.rs");
    let direct_engine_references: Vec<_> = ["core", "driver", "gpu", "sim", "spec"]
        .into_iter()
        .filter_map(|suffix| {
            let crate_path = format!("simthing_{suffix}::");
            witness.contains(&crate_path).then_some(crate_path)
        })
        .collect();
    assert!(
        direct_engine_references.is_empty(),
        "VENDOR-DOOR-TRIAD-FIVE-VERB-ONLY: witness bypassed the facade: {direct_engine_references:?}"
    );
    let facade_root = include_str!("../src/lib.rs");
    let public_modules: Vec<_> = facade_root
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub mod "))
        .map(|module| module.trim_end_matches(';'))
        .collect();
    assert_eq!(
        public_modules,
        ["bind", "derive", "overlay", "populate", "run"],
        "VENDOR-DOOR-GRANTING-SIXTH-VERB: the facade must remain exactly five verbs"
    );
    let forbidden = ["chokepoint", "corridor", "front", "dominance"];
    let public_observable_surfaces: Vec<_> = sources
        .iter()
        .flat_map(|(path, source)| {
            source.lines().enumerate().filter_map(move |(line, text)| {
                let lowered = text.trim_start().to_ascii_lowercase();
                (lowered.starts_with("pub fn ")
                    && forbidden.iter().any(|name| lowered.contains(name)))
                .then_some((path.clone(), line + 1, text.trim().to_string()))
            })
        })
        .collect();
    assert!(
        public_observable_surfaces.is_empty(),
        "VENDOR-DOOR-TRIAD-BORN-OBSERVABLE-SHAPE: forbidden public surfaces: {public_observable_surfaces:?}"
    );

    let owned_state: Vec<_> = sources
        .iter()
        .flat_map(|(path, source)| {
            source.lines().enumerate().filter_map(move |(line, text)| {
                let code = text.trim_start();
                if code.starts_with("//") {
                    return None;
                }
                (code.starts_with("static ")
                    || code.starts_with("pub static ")
                    || code.contains("Mutex<")
                    || code.contains("Arc<")
                    || code.contains("OnceCell<")
                    || code.contains("RefCell<"))
                .then_some((path.clone(), line + 1, code.to_string()))
            })
        })
        .collect();
    assert!(
        owned_state.is_empty(),
        "VENDOR-DOOR-TRIAD-FACADE-STATE: facade acquired owned state: {owned_state:?}"
    );

    let bind_source = include_str!("../src/bind.rs");
    let run_source = include_str!("../src/run.rs");
    assert!(
        bind_source.contains("ColumnIndex::try_from_admitted_authored(raw, bound)")
            && !bind_source.contains("ColumnIndex::from_gpu_round_trip")
            && !bind_source.contains("ColumnIndex::from_raw_for_oracle_or_rehearsal"),
        "VENDOR-DOOR-TRIAD-RAW-COLUMN-MINT: Bind must use only the bounded authored admission door"
    );
    assert!(
        run_source.contains("SimSession::open_from_spec_with_admitted_field_sweeps("),
        "VENDOR-DOOR-TRIAD-SEAM-DELEGATION: Run must reach the exact graduated 11.1a seam"
    );
    assert!(
        run_source.contains("session.install_growth_entitlement_market(binding)")
            && run_source
                .contains("session.realize_market_grant_residency(market, grant, proposed)"),
        "VENDOR-DOOR-GRANTING-DELEGATION: Run must terminate at the graduated session methods"
    );
    assert!(
        !bind_source.contains("projected_field_sweep_dimensions")
            && !bind_source.contains("preview_install("),
        "VENDOR-DOOR-TRIAD-DIMENSION-PREDICTION: Bind must not predict the final registry width ahead of the graduated seam"
    );
    assert!(
        bind_source.contains(".stall_outputs")
            && bind_source.contains("readback_canonical_field(&session.state.ctx)"),
        "VENDOR-DOOR-TRIAD-FABRICATED-OBSERVABLE: observation must delegate to admitted columns and live readback"
    );
    assert!(
        bind_source.contains("pub fn action_band_commitments(")
            && bind_source.contains("session.install_action_band_commitments(compiled)?")
            && !bind_source.contains("compile_crossing_consequence_session as action_band_commitments"),
        "ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the advertised door must compile-and-install atomically, never return a product ordinary SimSession can drop"
    );

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let reverse_dependencies: Vec<_> = fs::read_dir(workspace.join("crates"))
        .expect("crates directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            let manifest = entry.path().join("Cargo.toml");
            let source = fs::read_to_string(&manifest).ok()?;
            (entry.file_name() != "simthing-embedder" && source.contains("simthing-embedder"))
                .then_some(manifest)
        })
        .collect();
    assert!(
        reverse_dependencies.is_empty(),
        "VENDOR-DOOR-TRIAD-FACADE-LEAF: engine dependency points upward: {reverse_dependencies:?}"
    );

    let mut production_sources = rust_sources_under(&workspace.join("crates/simthing-driver/src"));
    production_sources.extend(rust_sources_under(
        &workspace.join("crates/simthing-embedder/src"),
    ));
    let actionband_occurrences: Vec<_> = production_sources
        .iter()
        .flat_map(|(path, source)| {
            source.lines().enumerate().filter_map(move |(line, text)| {
                text.contains("compile_crossing_consequence_session(")
                    .then_some((path.clone(), line + 1, text.trim().to_owned()))
            })
        })
        .collect();
    let actionband_declarations: Vec<_> = actionband_occurrences
        .iter()
        .filter(|(_, _, text)| text.starts_with("pub fn compile_crossing_consequence_session("))
        .collect();
    let actionband_production_callers: Vec<_> = actionband_occurrences
        .iter()
        .filter(|(_, _, text)| !text.starts_with("pub fn compile_crossing_consequence_session("))
        .collect();
    assert_eq!(
        actionband_declarations.len(),
        1,
        "ACTIONBAND-EXECUTION-INGRESS-DECLARATION: the graduated compiler declaration moved or multiplied: {actionband_occurrences:?}"
    );
    assert_eq!(
        actionband_production_callers.len(),
        1,
        "ACTIONBAND-EXECUTION-INGRESS-PRODUCTION-CALLER-CENSUS: the declaration is not execution proof; exactly one true production caller must remain at the atomic facade compile-and-install door: {actionband_production_callers:?}"
    );
    assert!(
        actionband_production_callers[0]
            .0
            .ends_with(Path::new("simthing-embedder/src/bind.rs")),
        "ACTIONBAND-EXECUTION-INGRESS-PRODUCTION-CALLER-CENSUS: the sole production caller moved away from the consuming Vendor Door: {actionband_production_callers:?}"
    );
}

fn embedder_production_sources() -> Vec<(PathBuf, String)> {
    rust_sources_under(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))
}

fn rust_sources_under(root: &Path) -> Vec<(PathBuf, String)> {
    let mut files = Vec::new();
    collect_rust_sources(root, &mut files);
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
