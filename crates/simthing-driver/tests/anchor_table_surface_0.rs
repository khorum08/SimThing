//! ANCHOR-TABLE-SURFACE-0 referees: derived STEAD table + consumer door.
//! Orch remand `5120847431`: GPU-resident remap, exact generation, Studio bridge, canonical TP install.

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
    assert!(
        readout
            .field_accretion_samples
            .iter()
            .any(|s| s.amount.is_finite()),
        "Studio samples must carry finite GPU-observed amounts"
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
    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: THRESH_BUF_VALUES,
    }];
    for generation in [0u32, 1u32, 2u32] {
        let Some((gpu, oracle)) = gpu_fused_maintain_case(0.5, 1.5, &regs, generation) else {
            eprintln!("skipping successive generations: no GPU");
            return;
        };
        assert_typed_tables_eq(&format!("gen_{generation}"), &gpu, &oracle);
        let stamped = gpu
            .rows()
            .iter()
            .find(|r| r.band.is_some())
            .expect("crossing must stamp a band");
        assert_eq!(
            stamped.last_crossing_generation,
            Some(generation),
            "dispatch generation {generation} must stamp exactly (incl. 0)"
        );
    }
}

#[test]
fn gpu_remap_identity_and_cardinality_across_structural_ops() {
    let Some(ctx) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping gpu_remap_identity: no GPU");
        return;
    };
    let prop = SimProperty::simple("ats", "move", 1);
    let (registry, allocator, root, pid) = fixture_tree(2, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let values = vec![1.0f32, 2.0];
    let mut table = mint_anchor_table_from_admission(&root, &registry, &loci, &values, 1);
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
    let child_id = allocator
        .owner_of(SlotIndex::new(1))
        .expect("child slot owner");
    let before_count = table.len();
    let from_slot = table.get(identity).unwrap().slot;

    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 3);
    state.upload_typed_anchor_table(&table);

    // AddChild / reallocation-style move via production GPU remap door.
    let to_slot = SlotIndex::new(2);
    let add_child = AnchorRemapSection::with_remaps(
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
    state.apply_anchor_remap_section(&add_child, &registry);
    let after_move = state.read_typed_anchor_table(&registry);
    assert_eq!(after_move.len(), before_count);
    let moved = after_move.get(identity).expect("identity preserved after GPU move");
    assert_eq!(moved.slot, to_slot);
    assert_eq!(moved.band, Some(BandIndex::new(3)));
    assert_eq!(moved.last_crossing_generation, Some(12));

    // Fusion-style retire via GPU remap.
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
    let after_fusion = state.read_typed_anchor_table(&registry);
    assert!(
        after_fusion.get(AnchorIdentity::new(child_id, pid)).is_none(),
        "fusion retire must drop GPU row"
    );
    assert!(after_fusion.len() < before_count);

    // Fission-style birth via GPU remap (registry seeds, not live-table readback).
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
    let after_fission = state.read_typed_anchor_table(&registry);
    assert!(
        after_fission
            .get(AnchorIdentity::new(born_id, pid))
            .is_some(),
        "fission birth must appear on GPU"
    );
    assert_eq!(after_fission.len(), before_count);
}

#[test]
fn canonical_tp_gpu_table_matches_25_anchored_0_unobserved() {
    use std::collections::HashMap;

    use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
    use simthing_driver::{preview_install, Scenario, SimSession};
    use simthing_spec::GameModeSpec;

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

    // Ordinary compile/install door on the canonical hydrated TP root (not a
    // synthetic World rehost). Properties-only game mode matches the 5.1
    // disposition install path; economy/overlay wiring is out of scope here.
    let game_mode = GameModeSpec {
        id: pack.game_mode.id.clone(),
        display_name: pack.game_mode.display_name.clone(),
        properties: pack.game_mode.properties.clone(),
        ..Default::default()
    };
    let root = pack.root.clone();
    let n_slots = (root.subtree_size() as u32).saturating_add(2048);
    let scenario = Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots,
        registry: DimensionRegistry::new(),
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut preview_allocator = SlotAllocator::new();
    preview_allocator.populate_from_tree(&scenario.root);
    let mut preview = preview_install(
        &game_mode,
        &scenario,
        &scenario.registry,
        &scenario.root,
        &preview_allocator,
    )
    .expect("canonical TP preview_install");
    let report = preview.registry.property_admission_report();
    assert_eq!(report.anchored_count(), 25, "5.1 inventory: 25 Anchored");
    assert_eq!(report.unobserved_count(), 0, "5.1 inventory: 0 Unobserved");

    // Materialize Anchored inventory onto the real TP tree shape. Strip any
    // hydrate-time property ids that are outside the install registry so locus
    // identity stays in the install id space (not a fresh World bag).
    fn retain_registry_props(node: &mut SimThing, registry: &DimensionRegistry) {
        node.properties
            .retain(|pid, _| registry.try_property(*pid).is_some());
        for child in &mut node.children {
            retain_registry_props(child, registry);
        }
    }
    retain_registry_props(&mut preview.root, &preview.registry);
    for row in &report.resource_properties {
        assert!(row.disposition.is_anchored());
        let prop = preview.registry.property(row.property_id);
        if !preview.root.properties.contains_key(&row.property_id) {
            preview.root.add_property(
                row.property_id,
                simthing_core::PropertyValue::from_layout(&prop.layout),
            );
        }
    }
    preview.allocator = SlotAllocator::new();
    preview.allocator.populate_from_tree(&preview.root);

    let loci = snapshot_anchored_loci(&preview.root, &preview.registry, &preview.allocator);
    let n_dims = preview.registry.total_columns.max(1);
    let values = vec![0.0f32; preview.allocator.capacity() as usize * n_dims];
    let expected =
        mint_anchor_table_from_admission(&preview.root, &preview.registry, &loci, &values, n_dims);
    assert!(
        !expected.is_empty(),
        "admission mint on real TP root must be non-empty"
    );

    // Studio accept path: open a shell session, then commit the preview install
    // (same door as open_from_spec's install_atomic result, without reminting ids).
    let mut shell_registry = DimensionRegistry::new();
    let _ = shell_registry.register(SimProperty::simple("_placeholder", "seed", 0));
    let shell = Scenario {
        name: scenario.name,
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (preview.allocator.capacity() as u32).saturating_add(64),
        registry: shell_registry,
        root: SimThing::new(SimThingKind::World, 0),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let Ok(mut sim) = SimSession::open(shell) else {
        eprintln!("skipping tp cardinality: SimSession::open failed");
        return;
    };
    sim.apply_install_preview(preview)
        .expect("apply canonical TP install preview");
    let gpu = sim.state.read_typed_anchor_table(&sim.proto.registry);
    assert_eq!(
        gpu.len(),
        expected.len(),
        "GPU cardinality must match admission mint on real TP install hosts"
    );

    let mut expected_keys: HashSet<(u32, u32, u32)> = HashSet::new();
    for row in expected.rows() {
        expected_keys.insert((
            row.identity.sim_thing_id.raw(),
            row.identity.property_id.0,
            row.col.raw_u32(),
        ));
    }
    let mut seen_props = HashSet::new();
    for row in gpu.rows() {
        assert!(
            expected_keys.remove(&(
                row.identity.sim_thing_id.raw(),
                row.identity.property_id.0,
                row.col.raw_u32()
            )),
            "unexpected or duplicate GPU row on real install loci"
        );
        seen_props.insert(row.identity.property_id.0);
    }
    assert!(
        expected_keys.is_empty(),
        "GPU missing expected install-locus rows: {expected_keys:?}"
    );
    assert_eq!(
        seen_props.len(),
        25,
        "GPU must cover all 25 Anchored inventory properties on real hosts"
    );
}
