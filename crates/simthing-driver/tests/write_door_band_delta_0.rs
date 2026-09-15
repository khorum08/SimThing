//! WRITE-DOOR-BAND-DELTA-0 referees: fused band-crossing deltas + structural remap gates.

use simthing_core::{
    derive_exact_anchor_remaps, validate_anchor_remap_for_encode,
    validate_exact_anchor_remap_endpoints, AnchorLocusRemap, AnchorRemapOperation,
    AnchorRemapSection, AnchoredLocusMap, ColumnIndex, DimensionRegistry,
    PropertyAdmissionDisposition, RemapKey, SimProperty, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SlotIndex,
};
use simthing_gpu::{
    apply_band_crossing_deltas_from_fused_emissions, cpu_oracle_band_crossing_deltas,
    set_debug_readback_allowed, AccumulatorOpSession, BandCrossingDirection, GpuContext,
    PackedThresholdUpload, SlotAllocator, ThresholdRegistration, DIR_DOWNWARD, DIR_UPWARD,
    THRESH_BUF_VALUES,
};
use simthing_sim::{
    gate_structural_gpu_encode, BoundaryDeltaEntry, ReplayDriver, ReplayFrame, ReplaySnapshot,
    SimRuntimeTree,
};

fn anchored_fixture(n_slots: u32, n_cols: usize) -> (DimensionRegistry, SlotAllocator) {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("wd", "anchored", n_cols));
    let mut root = SimThing::new(SimThingKind::GameSession, 0);
    for _ in 1..n_slots {
        root.add_child(SimThing::new(SimThingKind::Location, 0));
    }
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);
    (registry, allocator)
}

#[test]
fn unobserved_exclusion_without_caller_column_filter() {
    let mut registry = DimensionRegistry::new();
    let mut dark = SimProperty::simple("wd", "dark", 1);
    dark.admission_disposition = PropertyAdmissionDisposition::Unobserved {
        reason: "referee".into(),
        source_span_token: 0,
    };
    let _ = registry.register(dark);
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root);

    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 1,
        buffer: THRESH_BUF_VALUES,
    }];
    let prev = [0.0f32];
    let curr = [2.0f32];
    let deltas =
        cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 1, &regs, &registry, &allocator);
    assert!(deltas.is_empty());
}

#[test]
fn stable_slot_reparent_empty_witness_admits() {
    let section = AnchorRemapSection::empty_not_required(AnchorRemapOperation::Reparent);
    assert!(validate_anchor_remap_for_encode(&section, &[]).is_ok());
    let err = validate_anchor_remap_for_encode(
        &section,
        &[(SimThingId::from_session_raw(1), SimPropertyId(1))],
    )
    .unwrap_err();
    assert_eq!(err.operation, AnchorRemapOperation::Reparent);
}

#[test]
fn replay_bit_exact_remaps_and_band_deltas() {
    let id = SimThingId::from_session_raw(5);
    let prop = SimPropertyId(8);
    let section = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::BoundaryFlush,
        vec![AnchorLocusRemap::birth(
            id,
            prop,
            SlotIndex::new(0),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        )],
    );

    let (registry, allocator) = anchored_fixture(1, 1);
    let regs = [ThresholdRegistration {
        slot: 0,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 42,
        buffer: THRESH_BUF_VALUES,
    }];
    let deltas = cpu_oracle_band_crossing_deltas(
        &[0.0f32],
        &[1.5f32],
        &[],
        &[],
        1,
        &regs,
        &registry,
        &allocator,
    );
    assert_eq!(deltas.len(), 1);

    // Serde bit-exact for remap section.
    let encoded = serde_json::to_string(&section).expect("serialize remap section");
    let decoded: AnchorRemapSection =
        serde_json::from_str(&encoded).expect("deserialize remap section");
    assert_eq!(decoded, section);

    // End-to-end replay retains both remaps and band deltas bit-exact.
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let snapshot = ReplaySnapshot {
        day: 0,
        root: SimRuntimeTree::admit(root),
        registry: DimensionRegistry::new(),
        fission_lineage: Vec::new(),
    };
    let mut driver = ReplayDriver::from_snapshot(snapshot).expect("replay snapshot install");
    let entries = vec![
        BoundaryDeltaEntry::AnchorRemapApplied {
            section: section.clone(),
        },
        BoundaryDeltaEntry::BandCrossingDeltasApplied {
            deltas: deltas.clone(),
        },
    ];
    // Round-trip entries through JSON like the replay file format.
    let encoded_entries = serde_json::to_string(&entries).expect("serialize entries");
    let decoded_entries: Vec<BoundaryDeltaEntry> =
        serde_json::from_str(&encoded_entries).expect("deserialize entries");
    driver.apply_frame(ReplayFrame {
        day: 1,
        entries: decoded_entries,
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert_eq!(driver.last_anchor_remap.as_ref(), Some(&section));
    assert_eq!(driver.last_band_crossing_deltas, deltas);
}

