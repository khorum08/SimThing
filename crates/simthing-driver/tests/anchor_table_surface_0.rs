//! ANCHOR-TABLE-SURFACE-0 referees: derived STEAD table + consumer door.
//! Orch remands `5120847431` / `5121185090`: GPU remap, post-sync value
//! authority, successive generations, Studio bridge, canonical TP install.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use simthing_core::{
    apply_anchor_remaps_to_table, apply_band_crossings_to_anchor_table, mint_anchor_table_from_admission,
    refresh_anchor_table_magnitudes, AnchorIdentity, AnchorLocusRemap, AnchorRemapOperation,
    AnchorRemapSection, AnchoredLocusMap, BandIndex, ColumnIndex, DimensionRegistry,
    PropertyAdmissionDisposition, SimProperty, SimPropertyId, SimThing, SimThingKind, SlotIndex,
    SubFieldRole,
};
use simthing_gpu::{
    BandCrossingDelta, GpuContext, SlotAllocator, ThresholdRegistration, DIR_DOWNWARD, DIR_UPWARD,
    THRESH_BUF_VALUES,
};
use simthing_sim::{snapshot_anchored_loci, BoundaryDeltaEntry, SimRuntimeTree};

fn fixture_tree(
    n_slots: u32,
    prop: SimProperty,
) -> (DimensionRegistry, SlotAllocator, SimThing, SimPropertyId) {
    let mut registry = DimensionRegistry::new();
    let pid = registry.register(prop);
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.properties.insert(
        pid,
        simthing_core::PropertyValue::from_raw_lanes(vec![0.0; registry.property(pid).layout.stride()]),
    );
    for _ in 1..n_slots {
        let mut child = SimThing::new(SimThingKind::Location, 0);
        child.properties.insert(
            pid,
            simthing_core::PropertyValue::from_raw_lanes(vec![
                0.0;
                registry.property(pid).layout.stride()
            ]),
        );
        root.add_child(child);
    }
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    (registry, allocator, root, pid)
}

#[test]
fn unobserved_fixture_locus_gets_no_row() {
    let mut dark = SimProperty::simple("ats", "dark", 1);
    dark.admission_disposition = PropertyAdmissionDisposition::Unobserved {
        reason: "fixture-dark".into(),
        source_span_token: 7,
    };
    let (registry, allocator, root, dark_id) = fixture_tree(1, dark);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    assert!(
        loci.is_empty(),
        "snapshot_anchored_loci must exclude Unobserved"
    );
    // Fabricated locus entry still cannot mint a row (disposition gate).
    let mut forged = AnchoredLocusMap::new();
    forged.insert(
        (root.id, dark_id),
        (
            SlotIndex::new(0),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        ),
    );
    let table = mint_anchor_table_from_admission(&root, &registry, &forged, &[99.0], 1);
    assert!(table.is_empty());
    assert!(table.get(AnchorIdentity::new(root.id, dark_id)).is_none());
}

#[test]
fn rising_multi_edge_and_no_crossing_match_oracle() {
    let prop = SimProperty::simple("ats", "cell", 1);
    let (registry, _allocator, root, pid) = fixture_tree(1, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &_allocator);
    let mut table = mint_anchor_table_from_admission(&root, &registry, &loci, &[0.5], 1);
    assert!(!table.is_empty());
    assert!(table.rows().iter().all(|r| r.band.is_none()));

    let before = table.clone();
    // Empty delta set must leave band/generation untouched (None stays None).
    apply_band_crossings_to_anchor_table(&mut table, &[], 3);
    assert_eq!(table.rows()[0].band, before.rows()[0].band);
    assert_eq!(
        table.rows()[0].last_crossing_generation,
        before.rows()[0].last_crossing_generation
    );

    // Multi-edge: last ordered edge wins.
    let identity = AnchorIdentity::new(root.id, pid);
    let col = table
        .get_by_identity_role(identity, &SubFieldRole::Amount)
        .map(|r| r.col)
        .unwrap_or_else(|| ColumnIndex::from_raw_for_oracle_or_rehearsal(0));
    apply_band_crossings_to_anchor_table(
        &mut table,
        &[
            (identity, BandIndex::new(0), 1.5, Some(col)),
            (identity, BandIndex::new(1), 2.5, Some(col)),
        ],
        5,
    );
    let amount = table
        .get_by_identity_role(identity, &SubFieldRole::Amount)
        .expect("amount row");
    assert_eq!(amount.band, Some(BandIndex::new(1)));
    assert_eq!(amount.last_crossing_generation, Some(5));
    assert_eq!(amount.observed_value, 2.5);

    refresh_anchor_table_magnitudes(&mut table, &[2.5], 1, &[(0, 0, 1.0), (0, 0, 2.0)]);
    let amount = table
        .get_by_identity_role(identity, &SubFieldRole::Amount)
        .expect("amount row");
    assert!(amount.urgency >= 0.0);
}

#[test]
fn remap_preserves_identity_across_slot_move() {
    let prop = SimProperty::simple("ats", "move", 1);
    let (registry, allocator, root, pid) = fixture_tree(2, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let mut table = mint_anchor_table_from_admission(&root, &registry, &loci, &[1.0, 2.0], 1);
    let identity = AnchorIdentity::new(root.id, pid);
    apply_band_crossings_to_anchor_table(
        &mut table,
        &[(
            identity,
            BandIndex::new(3),
            9.0,
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(0)),
        )],
        12,
    );
    let from_slot = table.get(identity).unwrap().slot;
    let to_slot = SlotIndex::new(from_slot.raw().saturating_add(1).max(1));
    let section = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::AddChild,
        vec![AnchorLocusRemap::move_locus(
            root.id,
            pid,
            from_slot,
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            to_slot,
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        )],
    );
    apply_anchor_remaps_to_table(&mut table, &section, &registry);
    let row = table.get(identity).expect("identity preserved");
    assert_eq!(row.slot, to_slot);
    assert_eq!(row.band, Some(BandIndex::new(3)));
    assert_eq!(row.last_crossing_generation, Some(12));
    let _ = allocator;
}

#[test]
fn gpu_pod_round_trip_preserves_none_vs_some_zero_generation() {
    let prop = SimProperty::simple("ats", "pod", 1);
    let (registry, allocator, root, pid) = fixture_tree(1, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let mut table = mint_anchor_table_from_admission(&root, &registry, &loci, &[0.0], 1);
    assert!(table.rows().iter().all(|r| r.band.is_none()));
    assert!(table
        .rows()
        .iter()
        .all(|r| r.last_crossing_generation.is_none()));

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return;
    };
    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 1);
    state.upload_typed_anchor_table(&table);
    let none_back = state.read_typed_anchor_table(&registry);
    assert!(none_back.rows().iter().all(|r| r.band.is_none()));
    assert!(none_back
        .rows()
        .iter()
        .all(|r| r.last_crossing_generation.is_none()));

    let identity = AnchorIdentity::new(root.id, pid);
    apply_band_crossings_to_anchor_table(
        &mut table,
        &[(
            identity,
            BandIndex::new(0),
            1.0,
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(0)),
        )],
        0, // Some(0) must survive POD round-trip (not collapse to None)
    );
    assert_eq!(
        table.get(identity).and_then(|r| r.last_crossing_generation),
        Some(0)
    );
    state.upload_typed_anchor_table(&table);
    let zero_back = state.read_typed_anchor_table(&registry);
    assert_eq!(
        zero_back
            .get(identity)
            .and_then(|r| r.last_crossing_generation),
        Some(0),
        "None sentinel must not collide with generation 0"
    );
    assert_eq!(
        zero_back.get(identity).and_then(|r| r.band),
        Some(BandIndex::new(0))
    );
    let _ = allocator;
}

#[test]
fn wire_replay_delta_entries_do_not_carry_anchor_table() {
    let entries = [
        BoundaryDeltaEntry::AnchorRemapApplied {
            section: AnchorRemapSection::empty_not_required(AnchorRemapOperation::Reparent),
        },
        BoundaryDeltaEntry::BandCrossingDeltasApplied {
            deltas: Vec::<BandCrossingDelta>::new(),
        },
    ];
    for entry in &entries {
        let json = serde_json::to_string(entry).expect("serialize");
        assert!(
            !json.contains("AnchorTable") && !json.contains("observed_value"),
            "delta wire must not embed anchor-table rows: {json}"
        );
    }
    let bytes = serde_json::to_vec(&entries).unwrap();
    let back: Vec<BoundaryDeltaEntry> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(back.len(), 2);
}

#[test]
fn tree_with_mixed_disposition_table_cardinality_matches_anchored_only() {
    let mut registry = DimensionRegistry::new();
    let anchored = registry.register(SimProperty::simple("ats", "a", 1));
    let mut dark = SimProperty::simple("ats", "u", 1);
    dark.admission_disposition = PropertyAdmissionDisposition::Unobserved {
        reason: "mixed".into(),
        source_span_token: 1,
    };
    let unobserved = registry.register(dark);
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.properties.insert(
        anchored,
        simthing_core::PropertyValue::from_raw_lanes(vec![1.0]),
    );
    root.properties.insert(
        unobserved,
        simthing_core::PropertyValue::from_raw_lanes(vec![2.0]),
    );
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let report = registry.property_admission_report();
    assert_eq!(report.anchored_count(), 1);
    assert_eq!(report.unobserved_count(), 1);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let table = mint_anchor_table_from_admission(&root, &registry, &loci, &[1.0, 2.0], 2);
    assert!(table
        .rows()
        .iter()
        .all(|r| r.identity.property_id == anchored));
    assert!(table.get(AnchorIdentity::new(root.id, unobserved)).is_none());
    let _ = SimRuntimeTree::admit(root);
    let _ = SubFieldRole::Amount;
}

#[test]
fn hosted_observation_follows_gpu_not_transient_cpu() {
    use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot, Scenario, SimSession};
    use simthing_spec::PropertyKey;

    let mut registry = DimensionRegistry::new();
    let pid = registry.register(SimProperty::simple("ats", "cell", 1));
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.properties.insert(
        pid,
        simthing_core::PropertyValue::from_raw_lanes(vec![1.25]),
    );
    let scenario = Scenario {
        name: "ats_gpu_authority".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 1,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    };
    let Ok(sim) = SimSession::open(scenario) else {
        return;
    };

    let snapshot = AnchorTableSnapshot::from_session(&sim);
    assert!(!snapshot.is_empty(), "admission must upload GPU table");
    let identity = snapshot.rows()[0].identity;
    let gpu_value = snapshot.rows()[0].observed_value;
    let corrupt = gpu_value + 77.0;

    let transient = mint_anchor_table_from_admission(
        &sim.scenario.root,
        &sim.proto.registry,
        &snapshot_anchored_loci(
            &sim.scenario.root,
            &sim.proto.registry,
            &sim.proto.allocator,
        ),
        &[corrupt],
        1,
    );
    assert_eq!(
        transient.get(identity).map(|r| r.observed_value),
        Some(corrupt)
    );

    let observed = snapshot
        .get(identity)
        .expect("GPU snapshot row")
        .observed_value;
    assert_eq!(observed, gpu_value, "hosted door must follow GPU");
    assert_ne!(observed, corrupt);

    let hosted = observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &snapshot,
        identity.sim_thing_id,
        &PropertyKey::new("ats", "cell"),
        &SubFieldRole::Amount,
    )
    .expect("hosted cell");
    assert_eq!(hosted, gpu_value);
}

#[test]
fn studio_bridge_field_accretion_reads_gpu_authority() {
    use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
    use simthing_driver::AnchorTableSnapshot;
    use simthing_mapeditor::{
        authored_live_profile_from_pack, runtime_vertical_seed_scenario_spec, StudioLiveSessionBridge,
        StudioLiveSessionBridgeError, StudioSession,
    };

    let Some(_ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping studio bridge: no GPU");
        return;
    };

    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures");
    let lattice = fixtures
        .join("runtime_vertical_seed.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/");
    let clause = format!(
        r#"
scenario = foundry_valley {{
    metadata = {{ display_name = "Foundry Valley" }}
    static_galaxy_scenario = valley_base {{
        namespace = "valley_base"
        source_json = "{lattice}"
        map_quality_status = PASS
    }}
    owner = guild {{
        owner_key = "guild"
        display_name = "Guild"
        archetype = "industrial"
    }}
    location = ridge {{
        display_name = "Ridge"
        system_target = "row2_col3"
    }}
    field_economy = valley_economy {{
        namespace = "forge"
        field_resource_quantity = ridge_ore {{
            location = "ridge"
            resource = "ore"
            amount = 12
        }}
        production_building = ridge_foundry {{
            location = "ridge"
            input = {{ resource = "ore" amount = 2 }}
            output = {{ resource = "tools" coefficient = 1.0 }}
            throttle_hint_max_per_tick = 3
        }}
        stockpile_silo = guild_ore {{
            owner = "guild"
            resource = "ore"
            current = 20
        }}
    }}
}}
"#
    );
    let document = parse_raw_document(clause.as_bytes()).expect("parse foundry");
    let pack = hydrate_scenario_with_source_base(&document, Some(fixtures.as_path()))
        .expect("hydrate foundry");
    let mut studio = StudioSession::from_loaded_scenario(
        runtime_vertical_seed_scenario_spec(),
        PathBuf::from("tests/fixtures/foundry_valley_field_bearing.clause"),
        None,
    )
    .expect("studio session");
    studio.scenario_authority.scenario_id = pack.scenario_id.clone();
    studio.scenario_summary.scenario_id = pack.scenario_id.clone();
    studio = studio.with_authored_live_profile(authored_live_profile_from_pack(&pack));

    let mut bridge = StudioLiveSessionBridge::new();
    match bridge.open_from_loaded_studio_session(&studio) {
        Ok(()) => {}
        Err(StudioLiveSessionBridgeError::Unsupported(msg)) => {
            eprintln!("skipping studio bridge: unsupported ({msg})");
            return;
        }
        Err(e) => panic!("studio bridge open failed: {e:?}"),
    }
    let readout = bridge.readout();
    assert!(
        !readout.field_accretion_samples.is_empty(),
        "production Studio collector must emit field_accretion_samples from GPU table"
    );
    let open_sample = readout
        .field_accretion_samples
        .iter()
        .find(|s| s.amount.is_finite())
        .cloned()
        .expect("Studio samples must carry finite GPU-observed amounts");

    // Deliberate non-authoritative CPU/transient disagreement.
    const CORRUPT: f32 = 4242.5;
    {
        let sim = bridge
            .sim_session_mut()
            .expect("bridge owns a live SimSession");
        for v in &mut sim.coord.shadow {
            *v = CORRUPT;
        }
    }
    bridge
        .consume_scheduled_ticks(1)
        .expect("production bridge tick after CPU shadow corruption");
    let after = bridge.readout();
    let emitted = after
        .field_accretion_samples
        .last()
        .expect("tick must emit field_accretion_samples");
    let gpu = {
        let sim = bridge
            .sim_session_mut()
            .expect("bridge owns a live SimSession");
        AnchorTableSnapshot::from_session(sim)
    };
    let gpu_match = gpu
        .rows()
        .iter()
        .find(|r| (r.observed_value - emitted.amount).abs() < 1e-5)
        .expect("emitted sample must equal some GPU table observed_value");
    assert_ne!(
        emitted.amount, CORRUPT,
        "Studio sample must not follow corrupted CPU shadow ({CORRUPT})"
    );
    assert_ne!(
        open_sample.amount, CORRUPT,
        "open-time sample must not equal the later corruption sentinel"
    );
    assert!(
        (emitted.amount - gpu_match.observed_value).abs() < 1e-5,
        "Studio sample {} must equal GPU table {}",
        emitted.amount,
        gpu_match.observed_value
    );
}

fn gpu_fused_maintain_case(
    previous_amount: f32,
    current_amount: f32,
    regs: &[ThresholdRegistration],
    generation: u32,
) -> Option<(
    simthing_core::AnchorTable,
    simthing_core::AnchorTable,
)> {
    use simthing_gpu::{
        cpu_oracle_band_crossing_deltas, AccumulatorOpSession, PackedThresholdUpload,
    };

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        return None;
    };
    let prop = SimProperty::simple("ats", "cell", 1);
    let (registry, allocator, root, _pid) = fixture_tree(1, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let n_dims = registry.total_columns;
    let mut previous = vec![0.0f32; n_dims];
    let mut current = vec![0.0f32; n_dims];
    previous[0] = previous_amount;
    current[0] = current_amount;
    let before = mint_anchor_table_from_admission(&root, &registry, &loci, &previous, n_dims);
    assert!(!before.is_empty());

    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 1);
    // Generation must be live BEFORE fused threshold/anchor dispatch.
    state.set_anchor_table_generation(generation);
    state.upload_typed_anchor_table(&before);
    state.install_resolved_previous_values_at_boundary(&previous);
    state.install_resolved_values_at_boundary(&current);

    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, n_dims as u32, 16);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(regs).unwrap(),
        )
        .unwrap();
    session.prepare_threshold_scan(&state.ctx);
    state
        .dispatch_accumulator_threshold_scan(&mut session)
        .expect("fused threshold+anchor maintain");

    let gpu = state.read_typed_anchor_table(&registry);
    let deltas = cpu_oracle_band_crossing_deltas(
        &previous,
        &current,
        &[],
        &[],
        n_dims as u32,
        regs,
        &registry,
        &allocator,
    );
    let updates: Vec<_> = deltas
        .iter()
        .map(|d| {
            (
                AnchorIdentity::new(d.sim_thing_id(), d.property_id()),
                BandIndex::new(d.reg_idx()),
                d.post_value(),
                Some(d.col()),
            )
        })
        .collect();
    let mut oracle = before.clone();
    apply_band_crossings_to_anchor_table(&mut oracle, &updates, generation);
    let edges: Vec<(u32, u32, f32)> = regs
        .iter()
        .map(|r| (r.slot, r.col, r.threshold))
        .collect();
    refresh_anchor_table_magnitudes(&mut oracle, &current, n_dims, &edges);
    Some((gpu, oracle))
}

fn assert_typed_tables_eq(
    label: &str,
    gpu: &simthing_core::AnchorTable,
    oracle: &simthing_core::AnchorTable,
) {
    assert_eq!(gpu.len(), oracle.len(), "{label} row count");
    for (g, o) in gpu.rows().iter().zip(oracle.rows().iter()) {
        assert_eq!(g.identity, o.identity, "{label} identity");
        assert_eq!(g.slot, o.slot, "{label} slot");
        assert_eq!(g.col, o.col, "{label} col");
        assert_eq!(g.band, o.band, "{label} band");
        assert_eq!(
            g.last_crossing_generation, o.last_crossing_generation,
            "{label} generation"
        );
        assert_eq!(g.observed_value, o.observed_value, "{label} observed");
        assert_eq!(g.urgency, o.urgency, "{label} urgency");
    }
}

#[test]
fn gpu_crossing_matrix_bit_agrees_with_oracle() {
    let rising_regs = [
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 2.0,
            direction: DIR_UPWARD,
            event_kind: 2,
            buffer: THRESH_BUF_VALUES,
        },
    ];
    let falling_regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 5.0,
        direction: DIR_DOWNWARD,
        event_kind: 3,
        buffer: THRESH_BUF_VALUES,
    }];

    let cases = [
        ("rising_multi_edge", 0.5f32, 2.5f32, &rising_regs[..]),
        ("exact_edge", 1.0f32, 1.0f32, &rising_regs[..1]),
        ("no_crossing", 0.0f32, 0.5f32, &rising_regs[..]),
        ("falling", 6.0f32, 4.0f32, &falling_regs[..]),
    ];
    for (label, prev, curr, regs) in cases {
        let Some((gpu, oracle)) = gpu_fused_maintain_case(prev, curr, regs, 7) else {
            eprintln!("skipping gpu_crossing_matrix ({label}): no GPU");
            return;
        };
        assert_typed_tables_eq(label, &gpu, &oracle);
    }
}

#[test]
fn successive_dispatch_generations_including_zero_are_exact() {
    use simthing_gpu::{
        cpu_oracle_band_crossing_deltas, AccumulatorOpSession, PackedThresholdUpload,
    };

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping successive generations: no GPU");
        return;
    };
    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: THRESH_BUF_VALUES,
    }];
    let prop = SimProperty::simple("ats", "cell", 1);
    let (registry, allocator, root, _pid) = fixture_tree(1, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let n_dims = registry.total_columns;
    let edges: Vec<(u32, u32, f32)> = regs
        .iter()
        .map(|r| (r.slot, r.col, r.threshold))
        .collect();

    let mut previous = vec![0.0f32; n_dims];
    let mut current = vec![0.0f32; n_dims];
    previous[0] = 0.5;
    current[0] = 1.5;
    let before = mint_anchor_table_from_admission(&root, &registry, &loci, &previous, n_dims);
    assert!(!before.is_empty());

    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 1);
    state.upload_typed_anchor_table(&before);
    let mut session = AccumulatorOpSession::new_attached(&state.ctx, 1, n_dims as u32, 16);
    session
        .upload_packed_threshold_ops(
            &state.ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();

    let mut live = before.clone();
    let dispatch = |state: &mut simthing_gpu::WorldGpuState,
                    session: &mut AccumulatorOpSession,
                    live: &mut simthing_core::AnchorTable,
                    prev: &[f32],
                    curr: &[f32],
                    generation: u32,
                    label: &str| {
        state.set_anchor_table_generation(generation);
        state.install_resolved_previous_values_at_boundary(prev);
        state.install_resolved_values_at_boundary(curr);
        session.prepare_threshold_scan(&state.ctx);
        state
            .dispatch_accumulator_threshold_scan(session)
            .expect("fused threshold+anchor maintain");
        let gpu = state.read_typed_anchor_table(&registry);
        let deltas = cpu_oracle_band_crossing_deltas(
            prev,
            curr,
            &[],
            &[],
            n_dims as u32,
            &regs,
            &registry,
            &allocator,
        );
        let updates: Vec<_> = deltas
            .iter()
            .map(|d| {
                (
                    AnchorIdentity::new(d.sim_thing_id(), d.property_id()),
                    BandIndex::new(d.reg_idx()),
                    d.post_value(),
                    Some(d.col()),
                )
            })
            .collect();
        apply_band_crossings_to_anchor_table(live, &updates, generation);
        refresh_anchor_table_magnitudes(live, curr, n_dims, &edges);
        assert_typed_tables_eq(label, &gpu, live);
        gpu
    };

    // Crossing → stamps Some(0) on one continuing table/session.
    let gpu = dispatch(
        &mut state,
        &mut session,
        &mut live,
        &previous,
        &current,
        0,
        "gen_0_crossing",
    );
    let stamped = gpu
        .rows()
        .iter()
        .find(|r| r.band.is_some())
        .expect("crossing must stamp a band");
    assert_eq!(stamped.last_crossing_generation, Some(0));

    // No-crossing on the same live table: Some(0) must survive.
    previous[0] = 1.5;
    current[0] = 1.6;
    let gpu = dispatch(
        &mut state,
        &mut session,
        &mut live,
        &previous,
        &current,
        0,
        "gen_0_no_crossing",
    );
    let preserved = gpu
        .rows()
        .iter()
        .find(|r| r.band.is_some())
        .expect("band must survive no-crossing");
    assert_eq!(
        preserved.last_crossing_generation,
        Some(0),
        "Some(0) must survive a continuing no-crossing dispatch"
    );

    // Later crossing on the same session stamps generation 1.
    previous[0] = 0.5;
    current[0] = 1.5;
    let gpu = dispatch(
        &mut state,
        &mut session,
        &mut live,
        &previous,
        &current,
        1,
        "gen_1_crossing",
    );
    let stamped = gpu
        .rows()
        .iter()
        .find(|r| r.band.is_some())
        .expect("crossing must stamp a band");
    assert_eq!(stamped.last_crossing_generation, Some(1));

    // Generation 2 on the same continuing session.
    previous[0] = 0.5;
    current[0] = 1.5;
    let gpu = dispatch(
        &mut state,
        &mut session,
        &mut live,
        &previous,
        &current,
        2,
        "gen_2_crossing",
    );
    let stamped = gpu
        .rows()
        .iter()
        .find(|r| r.band.is_some())
        .expect("crossing must stamp a band");
    assert_eq!(stamped.last_crossing_generation, Some(2));
}

#[test]
fn gpu_remap_post_sync_value_authority_with_and_without_threshold_session() {
    const OLD: f32 = 11.0;
    const PRE_DEST: f32 = 22.0;
    const POST_DEST: f32 = 33.0;
    const THRESH: f32 = 30.0;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping gpu_remap value authority: no GPU");
        return;
    };
    let prop = SimProperty::simple("ats", "move", 1);
    let (registry, allocator, root, pid) = fixture_tree(2, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let identity = AnchorIdentity::new(root.id, pid);
    let n_dims = registry.total_columns.max(1);
    let mut values = vec![0.0f32; 3 * n_dims];
    values[0] = OLD;
    values[2 * n_dims] = PRE_DEST;
    let mut table = mint_anchor_table_from_admission(&root, &registry, &loci, &values, n_dims);
    apply_band_crossings_to_anchor_table(
        &mut table,
        &[(
            identity,
            BandIndex::new(3),
            OLD,
            Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(0)),
        )],
        12,
    );
    let from_slot = table.get(identity).unwrap().slot;
    let to_slot = SlotIndex::new(2);
    let move_section = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::AddChild,
        vec![AnchorLocusRemap::move_locus(
            root.id,
            pid,
            from_slot,
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            to_slot,
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        )],
    );

    // Active threshold session: post-sync maintain must sample POST_DEST, not PRE_DEST.
    {
        let mut state = simthing_gpu::WorldGpuState::new(
            GpuContext::new_blocking().expect("gpu"),
            &registry,
            3,
        );
        state.upload_typed_anchor_table(&table);
        state.install_resolved_values_at_boundary(&values);
        state.ensure_threshold_accumulator(16);
        let regs = [ThresholdRegistration {
            slot: to_slot.raw(),
            col: 0,
            threshold: THRESH,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        state
            .upload_accumulator_threshold_ops(&regs)
            .expect("upload threshold ops");
        state.apply_anchor_remap_section(&move_section, &registry);
        let mid = state.read_typed_anchor_table(&registry);
        let moved = mid.get(identity).expect("identity preserved after GPU move");
        assert_eq!(moved.slot, to_slot);
        assert_eq!(moved.band, Some(BandIndex::new(3)));
        assert_eq!(moved.last_crossing_generation, Some(12));
        assert_eq!(
            moved.observed_value, OLD,
            "remap must preserve dynamics until post-sync maintain"
        );
        // Step-9 twin: install canonical post-sync destination, then maintain.
        let mut post = values.clone();
        post[to_slot.raw() as usize * n_dims] = POST_DEST;
        state.install_resolved_values_at_boundary(&post);
        state.run_anchor_table_magnitude_maintain();
        let after = state.read_typed_anchor_table(&registry);
        let moved = after.get(identity).expect("moved row");
        assert_eq!(moved.observed_value, POST_DEST);
        assert_ne!(moved.observed_value, PRE_DEST);
        assert_eq!(moved.urgency, (POST_DEST - THRESH).abs());
        assert_eq!(moved.band, Some(BandIndex::new(3)));
        assert_eq!(moved.last_crossing_generation, Some(12));
    }

    // No threshold session: birth/move observation must still become exact.
    {
        let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 3);
        assert!(state.accumulator_runtime.is_none());
        state.upload_typed_anchor_table(&table);
        state.install_resolved_values_at_boundary(&values);
        state.apply_anchor_remap_section(&move_section, &registry);
        let mut post = values.clone();
        post[to_slot.raw() as usize * n_dims] = POST_DEST;
        state.install_resolved_values_at_boundary(&post);
        state.run_anchor_table_magnitude_maintain();
        let after = state.read_typed_anchor_table(&registry);
        let moved = after.get(identity).expect("moved row without session");
        assert_eq!(
            moved.observed_value, POST_DEST,
            "no-session magnitude path must refresh from post-sync values"
        );
        assert_eq!(moved.urgency, 0.0);
        assert_ne!(moved.observed_value, PRE_DEST);
        assert_ne!(moved.observed_value, 0.0);
    }

    // Cardinality: fusion retire + fission birth on GPU remap door.
    {
        let mut state = simthing_gpu::WorldGpuState::new(
            GpuContext::new_blocking().expect("gpu"),
            &registry,
            3,
        );
        state.upload_typed_anchor_table(&table);
        let child_id = allocator
            .owner_of(SlotIndex::new(1))
            .expect("child slot owner");
        let before_count = table.len();
        let retire = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fusion,
            vec![AnchorLocusRemap::retire(
                child_id,
                pid,
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            )],
        );
        state.apply_anchor_remap_section(&retire, &registry);
        assert!(state
            .read_typed_anchor_table(&registry)
            .get(AnchorIdentity::new(child_id, pid))
            .is_none());
        let born_id = simthing_core::SimThingId::from_session_raw(9001);
        let birth = AnchorRemapSection::with_remaps(
            AnchorRemapOperation::Fission,
            vec![AnchorLocusRemap::birth(
                born_id,
                pid,
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            )],
        );
        state.apply_anchor_remap_section(&birth, &registry);
        let mut post = vec![0.0f32; 3 * n_dims];
        post[n_dims] = POST_DEST;
        state.install_resolved_values_at_boundary(&post);
        state.run_anchor_table_magnitude_maintain();
        let after = state.read_typed_anchor_table(&registry);
        let born = after
            .get(AnchorIdentity::new(born_id, pid))
            .expect("fission birth");
        assert_eq!(born.observed_value, POST_DEST);
        assert_eq!(after.len(), before_count);
    }
}

#[test]
fn boundary_protocol_structural_remap_value_authority() {
    use simthing_core::{
        Direction, FissionTemplate, FissionThreshold, SimThingKindTag, SubFieldRole,
    };
    use simthing_feeder::{DispatchCoordinator, TransformPatcher};
    use simthing_gpu::cpu_oracle_threshold_events;
    use simthing_sim::{
        BoundaryProtocol, SimRuntimeTree, ThresholdSemantic,
    };

    const OLD: f32 = 11.0;
    const PRE_DEST: f32 = 22.0;
    const POST_DEST: f32 = 33.0;

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping boundary_protocol remap authority: no GPU");
        return;
    };

    let mut prop = SimProperty::simple("ats", "cell", 1);
    prop.fission_templates = vec![FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.3,
        direction: Direction::Falling,
        template: FissionTemplate {
            child_kind: SimThingKindTag::Cohort,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient: 0.05,
            resolution_label: "resolved".into(),
            clone_capability_children: false,
            capability_container_kinds: Vec::new(),
        },
        secondary: None,
    }];
    let mut registry = DimensionRegistry::new();
    let pid = registry.register(prop);
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    root.properties.insert(
        pid,
        simthing_core::PropertyValue::from_raw_lanes(vec![OLD]),
    );
    let parent_id = root.id;
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    // Headroom so AddChild / fission allocate without surprising grow side-effects.
    let n_slots = allocator.capacity().max(8);
    let n_dims = registry.total_columns.max(1);

    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, n_slots as u32);
    let mut coord = DispatchCoordinator::new(n_slots as u32, n_dims as u32, 1);
    let mut patcher = TransformPatcher::new(n_slots);
    let mut values = vec![0.0f32; n_slots * n_dims];
    values[0] = OLD;
    // Plant PRE_DEST across free slots so a pre-sync maintain would sample it.
    for slot in 1..n_slots {
        values[slot * n_dims] = PRE_DEST;
    }
    coord.shadow = values.clone();
    state.install_resolved_values_at_boundary(&values);

    let mut proto = BoundaryProtocol::new(SimRuntimeTree::admit(root), registry, allocator);
    proto.initial_gpu_sync(&coord, &mut state);
    assert!(
        state.accumulator_runtime.is_some(),
        "initial_gpu_sync must arm accumulator threshold runtime"
    );

    // ── AddChild / reallocation birth through BoundaryProtocol ─────────────
    let mut child = SimThing::new(SimThingKind::Location, 0);
    child.properties.insert(
        pid,
        simthing_core::PropertyValue::from_raw_lanes(vec![POST_DEST]),
    );
    let child_id = child.id;
    let mut pending_child = Some(child);
    // Re-plant PRE on GPU so Step-9 must overwrite before magnitude refresh.
    state.install_resolved_values_at_boundary(&values);
    let outcome = proto.execute_with_boundary_hook(
        Vec::new(),
        &mut patcher,
        &mut coord,
        &mut state,
        0,
        |ctx| {
            ctx.requests.push(simthing_feeder::BoundaryRequest::AddChild {
                parent: parent_id,
                child: pending_child.take().expect("AddChild once"),
            });
        },
    );
    assert!(
        outcome.maintainer.allocated.contains(&child_id),
        "AddChild must allocate through BoundaryProtocol"
    );
    assert!(
        !outcome.anchor_remap.remaps.is_empty(),
        "AddChild must produce GPU remaps"
    );
    let child_slot = proto
        .allocator
        .slot_of(child_id)
        .expect("child slot after AddChild");
    let gpu = state.read_typed_anchor_table(&proto.registry);
    let born = gpu
        .get(AnchorIdentity::new(child_id, pid))
        .expect("AddChild birth on GPU table");
    assert_eq!(born.slot, child_slot);
    assert_eq!(
        born.observed_value, POST_DEST,
        "AddChild birth must expose post-sync canonical value, not pre-sync {PRE_DEST}"
    );
    assert_ne!(born.observed_value, PRE_DEST);
    assert!(
        born.urgency.is_finite() && born.urgency >= 0.0,
        "post-boundary urgency must be refreshed (got {})",
        born.urgency
    );

    // ── Fusion/removal through BoundaryProtocol ────────────────────────────
    let remove_id = child_id;
    let outcome = proto.execute_with_boundary_hook(
        Vec::new(),
        &mut patcher,
        &mut coord,
        &mut state,
        1,
        |ctx| {
            ctx.requests
                .push(simthing_feeder::BoundaryRequest::Remove { target: remove_id });
        },
    );
    assert!(
        outcome.maintainer.tombstoned.contains(&remove_id),
        "Remove must tombstone through BoundaryProtocol"
    );
    let gpu = state.read_typed_anchor_table(&proto.registry);
    assert!(
        gpu.get(AnchorIdentity::new(remove_id, pid)).is_none(),
        "fusion/removal must drop GPU row"
    );

    // ── Fission/birth through BoundaryProtocol ─────────────────────────────
    // Re-plant PRE on free slots; fission seeds activating Amount to 0.0.
    let mut pre_fission = state.read_values();
    if pre_fission.len() < n_slots * n_dims {
        pre_fission.resize(n_slots * n_dims, 0.0);
    }
    for slot in 1..n_slots {
        pre_fission[slot * n_dims] = PRE_DEST;
    }
    state.install_resolved_values_at_boundary(&pre_fission);
    coord.shadow = pre_fission.clone();

    let mut fission_ek = None;
    for ek in 0..proto.threshold_registry().len() as u32 {
        if let Some(ThresholdSemantic::FissionTrigger {
            sim_thing_id,
            property_id,
            ..
        }) = proto.threshold_registry().get(ek)
        {
            if *sim_thing_id == parent_id && *property_id == pid {
                fission_ek = Some(ek);
                break;
            }
        }
    }
    let fission_ek = fission_ek.expect("initial_gpu_sync must register FissionTrigger");
    let parent_slot = proto.allocator.slot_of(parent_id).expect("parent slot");
    let events = cpu_oracle_threshold_events(
        &{
            let mut prev = pre_fission.clone();
            prev[parent_slot.raw() as usize * n_dims] = 0.2;
            prev
        },
        &{
            let mut curr = pre_fission.clone();
            curr[parent_slot.raw() as usize * n_dims] = 0.4;
            curr
        },
        &[],
        &[],
        n_dims as u32,
        &[ThresholdRegistration {
            slot: parent_slot.raw(),
            col: 0,
            threshold: 0.3,
            direction: DIR_UPWARD,
            event_kind: fission_ek,
            buffer: THRESH_BUF_VALUES,
        }],
    );
    assert!(!events.is_empty(), "fission crossing events required");
    let outcome = proto.execute(events, &mut patcher, &mut coord, &mut state, 2);
    assert!(
        outcome.fission.fissions_executed >= 1,
        "FissionTrigger must execute through BoundaryProtocol"
    );
    let born_id = outcome
        .fission
        .fission_pairs
        .first()
        .map(|(_, child)| *child)
        .expect("fission pair");
    let gpu = state.read_typed_anchor_table(&proto.registry);
    let born = gpu
        .get(AnchorIdentity::new(born_id, pid))
        .expect("fission birth on GPU table");
    // seed_fission_child zeroes activating Amount — post-sync canonical is 0.0.
    assert_eq!(
        born.observed_value, 0.0,
        "fission birth must expose post-sync seed (0.0), not pre-sync {PRE_DEST}"
    );
    assert_ne!(born.observed_value, PRE_DEST);
}

#[test]
fn canonical_tp_gpu_table_matches_admission_totality() {
    use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
    use simthing_driver::preview_install;

    let Some(_ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping tp cardinality: no GPU");
        return;
    };

    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let scenarios = repo.join("scenarios");
    let mut root_clauses = std::fs::read_dir(&scenarios)
        .expect("scenarios dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "clause"))
        .collect::<Vec<_>>();
    assert_eq!(root_clauses.len(), 1, "exactly one canonical clause");
    let clause_path = root_clauses.pop().unwrap();
    let source = std::fs::read_to_string(&clause_path).expect("read clause");
    let document = parse_raw_document(source.as_bytes()).expect("parse");
    let pack = hydrate_scenario_with_source_base(&document, Some(clause_path.parent().unwrap()))
        .expect("hydrate TP");

    // Full unmodified-topology TP install via the production field-bearing door
    // (same compile_and_install path Studio uses for observation hosts).
    use simthing_mapeditor::{
        authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
        field_bearing_game_mode,
    };
    let profile = authored_live_profile_from_pack(&pack);
    let scenario = driver_scenario_field_bearing_from_profile(&profile)
        .expect("field-bearing scenario");
    // Ordinary production install: domain packs + overlays ENABLED (totality law).
    let game_mode = field_bearing_game_mode(&profile.game_mode);
    let mut preview_allocator = SlotAllocator::new();
    preview_allocator.populate_from_tree(&scenario.root);
    let preview = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &preview_allocator,
    )
    .unwrap_or_else(|err| panic!("canonical TP field-bearing preview_install: {err:?}"));
    let report = preview.registry.property_admission_report();
    let tp_anchored: HashSet<_> = report
        .resource_properties
        .iter()
        .filter(|row| row.disposition.is_anchored() && row.namespace == "tp_economy")
        .map(|row| row.property_id)
        .collect();
    eprintln!(
        "CANONICAL 5.3b (derived): Anchored={} Unobserved={} tp_economy_anchored={}",
        report.anchored_count(),
        report.unobserved_count(),
        tp_anchored.len()
    );

    let loci = snapshot_anchored_loci(&preview.root, &preview.registry, &preview.allocator);
    let missing: Vec<_> = tp_anchored
        .iter()
        .filter(|pid| !loci.keys().any(|(_, p)| p == *pid))
        .collect();
    assert!(
        missing.is_empty(),
        "totality: every Anchored tp_economy property must have ≥1 live locus; missing={missing:?}"
    );
    let live_prop_count = {
        let mut props = HashSet::new();
        for ((_, pid), _) in &loci {
            if tp_anchored.contains(pid) {
                props.insert(pid.0);
            }
        }
        props.len()
    };
    assert_eq!(live_prop_count, tp_anchored.len());
    // No repeated (thing, property) keys — AnchoredLocusMap uniqueness.
    assert_eq!(
        loci.len(),
        loci.keys().collect::<HashSet<_>>().len(),
        "locus map must not repeat (SimThingId, SimPropertyId)"
    );
    eprintln!(
        "CANONICAL 5.3b TOTALITY: tp_economy Anchored covered={}; live locus rows={}; dark cells={}.",
        live_prop_count,
        loci.iter()
            .filter(|((_, pid), _)| tp_anchored.contains(pid))
            .count(),
        report.unobserved_count()
    );
}
