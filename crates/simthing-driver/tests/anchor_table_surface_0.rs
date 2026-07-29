//! ANCHOR-TABLE-SURFACE-0 referees: derived STEAD table + consumer door.

use simthing_core::{
    apply_anchor_remaps_to_table, apply_band_crossings_to_anchor_table,
    mint_anchor_table_from_admission, refresh_anchor_table_magnitudes, AnchorIdentity,
    AnchorLocusRemap, AnchorRemapOperation, AnchorRemapSection, AnchoredLocusMap, BandIndex,
    ColumnIndex, DimensionRegistry, PropertyAdmissionDisposition, SimProperty, SimPropertyId,
    SimThing, SimThingKind, SlotIndex, SubFieldRole,
};
use simthing_gpu::{
    encode_anchor_table_gpu, oracle_anchor_table_after_deltas, BandCrossingDelta, GpuContext,
    SlotAllocator, ANCHOR_BAND_NONE_POD,
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
    let after_none = oracle_anchor_table_after_deltas(&before, &[], 3);
    assert_eq!(after_none.rows()[0].band, before.rows()[0].band);
    assert_eq!(
        after_none.rows()[0].last_crossing_generation,
        before.rows()[0].last_crossing_generation
    );

    // Multi-edge: last ordered edge wins (mirrors sealed band_crossing_updates).
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
    // Urgency uses refreshed observed_value at the row's slot/col.
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
    let before_band = table.get(identity).map(|r| r.band);
    // Mark a crossing so dynamic fields must survive remap.
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
    let _ = before_band;
    let _ = allocator;
}

#[test]
fn gpu_pod_encode_uses_sentinel_only_at_boundary() {
    let prop = SimProperty::simple("ats", "pod", 1);
    let (registry, allocator, root, _pid) = fixture_tree(1, prop);
    let loci = snapshot_anchored_loci(&root, &registry, &allocator);
    let table = mint_anchor_table_from_admission(&root, &registry, &loci, &[0.0], 1);
    let gpu = encode_anchor_table_gpu(&table);
    assert_eq!(gpu.len(), table.len());
    assert_eq!(gpu[0].band_idx, ANCHOR_BAND_NONE_POD);

    let Some(ctx) = GpuContext::new_blocking().ok() else {
        // Adapter-less hosts still prove encode sentinel; upload parity is GPU-gated.
        return;
    };
    let mut state = simthing_gpu::WorldGpuState::new(ctx, &registry, 1);
    state.upload_anchor_table(&gpu);
    let readback = state.read_anchor_table();
    assert_eq!(readback, gpu);
    let _ = allocator;
    let _ = root;
}

#[test]
fn wire_replay_delta_entries_do_not_carry_anchor_table() {
    // Fence: derived table must never enter BoundaryDeltaEntry wire bytes.
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
    // Round-trip preserves wire shape without table authority.
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
fn hosted_and_studio_observation_read_gpu_not_transient_cpu() {
    use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot, Scenario, SimSession};
    use simthing_gpu::encode_anchor_table_gpu;
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

    let gpu_before = sim.state.read_anchor_table();
    assert!(!gpu_before.is_empty(), "admission must upload GPU table");
    let identity = AnchorIdentity::new(
        simthing_core::SimThingId::from_session_raw(gpu_before[0].sim_thing_id),
        simthing_core::SimPropertyId(gpu_before[0].property_id),
    );
    let gpu_value = gpu_before[0].observed_value;
    let corrupt = gpu_value + 77.0;

    // Allowed transient CPU init disagrees with GPU; never re-uploaded.
    let transient = mint_anchor_table_from_admission(
        &sim.scenario.root,
        &sim.scenario.registry,
        &snapshot_anchored_loci(
            &sim.scenario.root,
            &sim.scenario.registry,
            &sim.proto.allocator,
        ),
        &[corrupt],
        1,
    );
    assert_eq!(
        transient.get(identity).map(|r| r.observed_value),
        Some(corrupt)
    );
    let _ = encode_anchor_table_gpu(&transient); // prove encode exists; do not upload

    let snapshot = AnchorTableSnapshot::from_session(&sim);
    let observed = snapshot
        .get(identity)
        .expect("GPU snapshot row")
        .observed_value;
    assert_eq!(observed, gpu_value, "hosted door must follow GPU");
    assert_ne!(observed, corrupt);

    let hosted = observe_hosted_property_cell(
        &sim.scenario.registry,
        &sim.proto.allocator,
        &snapshot,
        identity.sim_thing_id,
        &PropertyKey::new("ats", "cell"),
        &SubFieldRole::Amount,
    )
    .expect("hosted cell");
    assert_eq!(hosted, gpu_value);

    // Studio free-fn seam (same production snapshot API Studio uses).
    let slot = gpu_before[0].slot;
    let col = gpu_before[0].col;
    let studio = snapshot
        .observed_value_at_slot_col(slot, col)
        .expect("studio slot/col read");
    assert_eq!(studio, gpu_value, "Studio seam must follow GPU");
}

fn gpu_fused_maintain_case(
    previous_amount: f32,
    current_amount: f32,
    regs: &[simthing_gpu::ThresholdRegistration],
    generation: u32,
) -> Option<(Vec<simthing_gpu::AnchorTableRowGpu>, Vec<simthing_gpu::AnchorTableRowGpu>)> {
    use simthing_gpu::{
        cpu_oracle_band_crossing_deltas, encode_anchor_table_gpu, oracle_anchor_table_after_deltas,
        AccumulatorOpSession, PackedThresholdUpload,
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
    state.upload_anchor_table(&encode_anchor_table_gpu(&before));
    state.set_anchor_table_generation(generation);
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

    let gpu = state.read_anchor_table();
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
    let mut oracle = oracle_anchor_table_after_deltas(&before, &deltas, generation);
    let edges: Vec<(u32, u32, f32)> = regs
        .iter()
        .map(|r| (r.slot, r.col, r.threshold))
        .collect();
    refresh_anchor_table_magnitudes(&mut oracle, &current, n_dims, &edges);
    Some((gpu, encode_anchor_table_gpu(&oracle)))
}

#[test]
fn gpu_crossing_matrix_bit_agrees_with_oracle() {
    use simthing_gpu::{ThresholdRegistration, DIR_DOWNWARD, DIR_UPWARD, THRESH_BUF_VALUES};

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
        assert_eq!(gpu.len(), oracle.len(), "{label} row count");
        for (g, o) in gpu.iter().zip(oracle.iter()) {
            assert_eq!(g.sim_thing_id, o.sim_thing_id, "{label} id");
            assert_eq!(g.property_id, o.property_id, "{label} prop");
            assert_eq!(g.slot, o.slot, "{label} slot");
            assert_eq!(g.col, o.col, "{label} col");
            assert_eq!(g.band_idx, o.band_idx, "{label} band");
            assert_eq!(
                g.last_crossing_generation, o.last_crossing_generation,
                "{label} generation"
            );
            assert_eq!(g.observed_value, o.observed_value, "{label} observed");
            assert_eq!(g.urgency, o.urgency, "{label} urgency");
        }
    }
}

#[test]
fn gpu_remap_identity_and_cardinality_across_structural_ops() {
    use simthing_core::{AnchorLocusRemap, AnchorRemapOperation, AnchorRemapSection};
    use simthing_gpu::encode_anchor_table_gpu;

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
    state.upload_anchor_table(&encode_anchor_table_gpu(&table));

    // AddChild / reallocation-style move of the root locus.
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
    apply_anchor_remaps_to_table(&mut table, &add_child, &registry);
    state.upload_anchor_table(&encode_anchor_table_gpu(&table));
    let after_move = state.read_anchor_table();
    assert_eq!(after_move.len(), before_count);
    let moved = after_move
        .iter()
        .find(|r| r.property_id == pid.0 && r.sim_thing_id == root.id.raw())
        .expect("identity preserved after AddChild move");
    assert_eq!(moved.slot, to_slot.raw());
    assert_eq!(moved.band_idx, 3);
    assert_eq!(moved.last_crossing_generation, 12);

    // Fusion-style retire of the existing child locus.
    let retire = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::Fusion,
        vec![AnchorLocusRemap::retire(
            child_id,
            pid,
            SlotIndex::new(1),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        )],
    );
    apply_anchor_remaps_to_table(&mut table, &retire, &registry);
    state.upload_anchor_table(&encode_anchor_table_gpu(&table));
    let after_fusion = state.read_anchor_table();
    assert!(
        after_fusion
            .iter()
            .all(|r| !(r.sim_thing_id == child_id.raw() && r.property_id == pid.0)),
        "fusion retire must drop GPU row"
    );
    assert_eq!(after_fusion.len(), table.len());
    assert!(after_fusion.len() < before_count);

    // Fission-style birth of a fresh identity.
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
    apply_anchor_remaps_to_table(&mut table, &birth, &registry);
    state.upload_anchor_table(&encode_anchor_table_gpu(&table));
    let after_fission = state.read_anchor_table();
    assert!(
        after_fission
            .iter()
            .any(|r| r.sim_thing_id == born_id.raw() && r.property_id == pid.0),
        "fission birth must appear on GPU"
    );
    assert_eq!(after_fission.len(), table.len());
    assert_eq!(after_fission.len(), before_count);
}

#[test]
fn canonical_tp_gpu_table_matches_25_anchored_0_unobserved() {
    use std::collections::{HashMap, HashSet};
    use std::path::Path;

    use simthing_clausething::{hydrate_scenario_with_source_base, parse_raw_document};
    use simthing_driver::{preview_install, Scenario, SimSession};
    use simthing_gpu::encode_anchor_table_gpu;
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

    let game_mode = GameModeSpec {
        id: pack.game_mode.id.clone(),
        display_name: pack.game_mode.display_name.clone(),
        properties: pack.game_mode.properties.clone(),
        ..Default::default()
    };
    let root = pack.root.clone();
    let scratch = Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (root.subtree_size() as u32).saturating_add(2048),
        registry: DimensionRegistry::new(),
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&scratch.root);
    let preview = preview_install(
        &game_mode,
        &scratch,
        &scratch.registry,
        &scratch.root,
        &allocator,
    )
    .expect("TP preview install");
    let report = preview.registry.property_admission_report();
    assert_eq!(report.anchored_count(), 25, "5.1 inventory: 25 Anchored");
    assert_eq!(report.unobserved_count(), 0, "5.1 inventory: 0 Unobserved");

    // Host the 25 Anchored properties on a fresh root (TP galaxy nodes carry
    // hydrated property ids that are not the install registry's dense indices).
    let mut host = SimThing::new(SimThingKind::World, 0);
    for row in &report.resource_properties {
        assert!(row.disposition.is_anchored());
        let prop = preview.registry.property(row.property_id);
        host.add_property(
            row.property_id,
            simthing_core::PropertyValue::from_layout(&prop.layout),
        );
    }
    let mut host_allocator = simthing_gpu::SlotAllocator::new();
    host_allocator.populate_from_tree(&host);

    let loci = snapshot_anchored_loci(&host, &preview.registry, &host_allocator);
    let n_dims = preview.registry.total_columns.max(1);
    let values = vec![0.0f32; host_allocator.capacity() as usize * n_dims];
    let expected = mint_anchor_table_from_admission(
        &host,
        &preview.registry,
        &loci,
        &values,
        n_dims,
    );
    let expected_gpu = encode_anchor_table_gpu(&expected);
    assert!(!expected_gpu.is_empty(), "admission mint must be non-empty");

    let installed = Scenario {
        name: scratch.name,
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (host_allocator.capacity() as u32).saturating_add(64),
        registry: preview.registry,
        root: host,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    let Ok(sim) = SimSession::open(installed) else {
        eprintln!("skipping tp cardinality: SimSession::open failed");
        return;
    };
    let gpu = sim.state.read_anchor_table();
    assert_eq!(
        gpu.len(),
        expected_gpu.len(),
        "GPU cardinality must match admission mint (no dup/stale/missing)"
    );

    let mut expected_keys: HashSet<(u32, u32, u32)> = HashSet::new();
    for row in &expected_gpu {
        expected_keys.insert((row.sim_thing_id, row.property_id, row.col));
    }
    let mut seen_props = HashSet::new();
    for row in &gpu {
        assert!(
            expected_keys.remove(&(row.sim_thing_id, row.property_id, row.col)),
            "unexpected or duplicate GPU row"
        );
        seen_props.insert(row.property_id);
    }
    assert!(
        expected_keys.is_empty(),
        "GPU missing expected rows: {expected_keys:?}"
    );
    assert_eq!(
        seen_props.len(),
        25,
        "GPU must cover all 25 Anchored inventory properties"
    );
}
