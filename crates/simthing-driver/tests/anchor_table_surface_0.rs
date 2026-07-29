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
fn hosted_observation_reads_gpu_when_cpu_staging_disagrees() {
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
    let Ok(mut sim) = SimSession::open(scenario) else {
        // Adapter-less hosts still keep encode/wire referees; this arm needs GPU.
        return;
    };

    let staging = sim.proto.writer_staging_anchor_table_for_oracle_or_test();
    assert!(
        !staging.is_empty(),
        "writer staging must be minted at initial_gpu_sync"
    );
    let identity = staging.rows()[0].identity;
    let gpu_value = staging.rows()[0].observed_value;
    let corrupt = gpu_value + 77.0;
    {
        let staging_mut = sim
            .proto
            .writer_staging_anchor_table_mut_for_oracle_or_test();
        let row = staging_mut.get_mut(identity).expect("staging row");
        row.observed_value = corrupt;
    }
    assert_eq!(
        sim.proto
            .writer_staging_anchor_table_for_oracle_or_test()
            .get(identity)
            .map(|r| r.observed_value),
        Some(corrupt)
    );

    let snapshot = AnchorTableSnapshot::from_session(&sim);
    let observed = snapshot
        .get(identity)
        .expect("GPU snapshot row")
        .observed_value;
    assert_eq!(
        observed, gpu_value,
        "hosted observation must read GPU table, not corrupted CPU staging"
    );
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
}
