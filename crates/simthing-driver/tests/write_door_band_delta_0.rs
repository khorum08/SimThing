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
fn rising_falling_exact_edge_no_crossing_and_multi_edge_oracle() {
    let (registry, allocator) = anchored_fixture(1, 2);
    let regs = [
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 100,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 2.0,
            direction: DIR_UPWARD,
            event_kind: 101,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 0,
            col: 1,
            threshold: 5.0,
            direction: DIR_DOWNWARD,
            event_kind: 102,
            buffer: THRESH_BUF_VALUES,
        },
    ];

    // Rising multi-edge jump 0.5 → 2.5 crosses both upward edges.
    let prev = [0.5f32, 6.0];
    let curr = [2.5f32, 6.0];
    let rising =
        cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 2, &regs, &registry, &allocator);
    assert_eq!(rising.len(), 2);
    assert_eq!(rising[0].direction(), BandCrossingDirection::Rising);
    assert_eq!(rising[1].direction(), BandCrossingDirection::Rising);
    assert_eq!(rising[0].slot(), SlotIndex::new(0));
    assert_eq!(rising[0].col().raw(), 0);

    // Exact-edge landing is not a rising cross.
    let prev_exact = [1.0f32, 6.0];
    let curr_exact = [1.0f32, 6.0];
    let exact = cpu_oracle_band_crossing_deltas(
        &prev_exact,
        &curr_exact,
        &[],
        &[],
        2,
        &regs[..1],
        &registry,
        &allocator,
    );
    assert!(exact.is_empty());

    // Falling cross.
    let prev_fall = [0.0f32, 6.0];
    let curr_fall = [0.0f32, 4.0];
    let falling = cpu_oracle_band_crossing_deltas(
        &prev_fall,
        &curr_fall,
        &[],
        &[],
        2,
        &regs[2..],
        &registry,
        &allocator,
    );
    assert_eq!(falling.len(), 1);
    assert_eq!(falling[0].direction(), BandCrossingDirection::Falling);

    // No crossing when values stay on the same side.
    let prev_nc = [0.0f32, 6.0];
    let curr_nc = [0.5f32, 5.5];
    let none = cpu_oracle_band_crossing_deltas(
        &prev_nc,
        &curr_nc,
        &[],
        &[],
        2,
        &regs,
        &registry,
        &allocator,
    );
    assert!(none.is_empty());
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
fn remap_less_structural_encode_is_rejected_with_operation_context() {
    let id = SimThingId::from_session_raw(9);
    let prop = SimPropertyId(3);
    let section = AnchorRemapSection::with_remaps(AnchorRemapOperation::AddChild, vec![]);
    let err = gate_structural_gpu_encode(&section, &[(id, prop)]).unwrap_err();
    assert_eq!(err.operation, AnchorRemapOperation::AddChild);
    assert_eq!(err.missing, vec![RemapKey::Locus(id, prop)]);
}

#[test]
fn retire_from_nonzero_slot_is_exact() {
    let id = SimThingId::from_session_raw(11);
    let prop = SimPropertyId(4);
    let mut pre = AnchoredLocusMap::new();
    pre.insert(
        (id, prop),
        (
            SlotIndex::new(3),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
        ),
    );
    let post = AnchoredLocusMap::new();
    let section =
        derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::Fusion, false).unwrap();
    assert_eq!(section.remaps.len(), 1);
    assert_eq!(section.remaps[0].from_slot, Some(SlotIndex::new(3)));
    assert_ne!(section.remaps[0].from_slot, Some(SlotIndex::new(0)));
    assert!(validate_exact_anchor_remap_endpoints(&section, &pre, &post, false).is_ok());
}

#[test]
fn column_shift_records_pre_to_post_layout() {
    let id = SimThingId::from_session_raw(5);
    let prop = SimPropertyId(8);
    let mut pre = AnchoredLocusMap::new();
    let mut post = AnchoredLocusMap::new();
    pre.insert(
        (id, prop),
        (
            SlotIndex::new(1),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
        ),
    );
    post.insert(
        (id, prop),
        (
            SlotIndex::new(1),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(5),
        ),
    );
    let section =
        derive_exact_anchor_remaps(&pre, &post, AnchorRemapOperation::AddDimension, true).unwrap();
    assert_eq!(
        section.remaps[0].from_col(),
        Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(2))
    );
    assert_eq!(
        section.remaps[0].to_col(),
        Some(ColumnIndex::from_raw_for_oracle_or_rehearsal(5))
    );
}

#[test]
fn wrong_endpoint_and_duplicate_remap_negatives() {
    let id = SimThingId::from_session_raw(9);
    let prop = SimPropertyId(1);
    let mut pre = AnchoredLocusMap::new();
    let mut post = AnchoredLocusMap::new();
    pre.insert(
        (id, prop),
        (
            SlotIndex::new(1),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        ),
    );
    post.insert(
        (id, prop),
        (
            SlotIndex::new(2),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        ),
    );
    let wrong = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::Fission,
        vec![AnchorLocusRemap::move_locus(
            id,
            prop,
            SlotIndex::new(1),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            SlotIndex::new(9),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        )],
    );
    assert!(validate_exact_anchor_remap_endpoints(&wrong, &pre, &post, false).is_err());

    let dup = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::Fission,
        vec![
            AnchorLocusRemap::move_locus(
                id,
                prop,
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                SlotIndex::new(2),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
            AnchorLocusRemap::move_locus(
                id,
                prop,
                SlotIndex::new(1),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
                SlotIndex::new(2),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
        ],
    );
    assert!(validate_anchor_remap_for_encode(&dup, &[(id, prop)]).is_err());
}

#[test]
fn slot_churn_birth_remap_is_complete() {
    let id = SimThingId::from_session_raw(11);
    let prop = SimPropertyId(4);
    let section = AnchorRemapSection::with_remaps(
        AnchorRemapOperation::Fission,
        vec![AnchorLocusRemap::birth(
            id,
            prop,
            SlotIndex::new(2),
            ColumnIndex::from_raw_for_oracle_or_rehearsal(1),
        )],
    );
    assert!(validate_anchor_remap_for_encode(&section, &[(id, prop)]).is_ok());
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
    let mut driver = ReplayDriver::from_snapshot(snapshot);
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

/// Remand-2: one Anchored cell crosses ≥2 ordered edges in a real GPU threshold
/// pass; GPU-minted deltas ride BoundaryDeltaEntry JSON → replay bit-exact.
#[test]
fn gpu_multi_edge_band_delta_boundary_replay_transport() {
    let Some(_) = GpuContext::new_blocking().ok() else {
        eprintln!("skipping: no GPU");
        return;
    };
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("gpu");

    let n_slots = 1u32;
    let n_dims = 1u32;
    let (registry, allocator) = anchored_fixture(n_slots, n_dims as usize);
    let owner = allocator.owner_of(SlotIndex::new(0)).expect("slot 0 owner");
    let prop = SimPropertyId(0);
    let regs = [
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 1.0,
            direction: DIR_UPWARD,
            event_kind: 201,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: 2.0,
            direction: DIR_UPWARD,
            event_kind: 202,
            buffer: THRESH_BUF_VALUES,
        },
    ];
    let previous = vec![0.5f32];
    let current = vec![2.5f32];

    let mut session = AccumulatorOpSession::new_attached(&ctx, n_slots, n_dims, 16);
    session.upload_values(&ctx, &current);
    session.upload_previous_values(&ctx, &previous);
    session
        .upload_packed_threshold_ops(
            &ctx,
            &PackedThresholdUpload::from_registrations(&regs).unwrap(),
        )
        .unwrap();
    session.tick(&ctx, 0).unwrap();

    let mut emissions = session.readback_threshold_emissions(&ctx).unwrap();
    // Canonical write-door ladder order is the registration index ladder.
    emissions.sort_by_key(|e| e.reg_idx());
    assert_eq!(emissions.len(), 2, "GPU must emit both rising edges");
    assert_eq!(emissions[0].reg_idx(), 0);
    assert_eq!(emissions[1].reg_idx(), 1);
    assert_eq!(emissions[0].slot(), 0);
    assert_eq!(emissions[1].slot(), 0);

    let gpu_deltas = apply_band_crossing_deltas_from_fused_emissions(
        &emissions,
        session.threshold_registrations(),
        &registry,
        &allocator,
    );
    assert_eq!(gpu_deltas.len(), 2);
    assert_eq!(gpu_deltas[0].reg_idx(), 0);
    assert_eq!(gpu_deltas[1].reg_idx(), 1);
    assert_eq!(gpu_deltas[0].direction(), BandCrossingDirection::Rising);
    assert_eq!(gpu_deltas[1].direction(), BandCrossingDirection::Rising);
    assert_eq!(gpu_deltas[0].threshold(), 1.0);
    assert_eq!(gpu_deltas[1].threshold(), 2.0);
    assert_eq!(gpu_deltas[0].post_value(), 2.5);
    assert_eq!(gpu_deltas[1].post_value(), 2.5);
    assert_eq!(gpu_deltas[0].sim_thing_id(), owner);
    assert_eq!(gpu_deltas[1].sim_thing_id(), owner);
    assert_eq!(gpu_deltas[0].property_id(), prop);
    assert_eq!(gpu_deltas[1].property_id(), prop);
    assert_eq!(gpu_deltas[0].slot(), SlotIndex::new(0));
    assert_eq!(gpu_deltas[0].col().raw(), 0);

    let cpu_deltas = cpu_oracle_band_crossing_deltas(
        &previous,
        &current,
        &[],
        &[],
        n_dims,
        &regs,
        &registry,
        &allocator,
    );
    assert_eq!(
        gpu_deltas, cpu_deltas,
        "GPU-minted deltas must agree with CPU oracle"
    );

    // GPU-derived deltas through BoundaryDeltaEntry JSON → replay retention.
    let root = SimThing::new(SimThingKind::GameSession, 0);
    let snapshot = ReplaySnapshot {
        day: 0,
        root: SimRuntimeTree::admit(root),
        registry: DimensionRegistry::new(),
        fission_lineage: Vec::new(),
    };
    let mut driver = ReplayDriver::from_snapshot(snapshot);
    let entries = vec![BoundaryDeltaEntry::BandCrossingDeltasApplied {
        deltas: gpu_deltas.clone(),
    }];
    let encoded = serde_json::to_string(&entries).expect("serialize GPU band deltas");
    let decoded: Vec<BoundaryDeltaEntry> =
        serde_json::from_str(&encoded).expect("deserialize GPU band deltas");
    driver.apply_frame(ReplayFrame {
        day: 1,
        entries: decoded,
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    });
    assert_eq!(driver.last_band_crossing_deltas, gpu_deltas);
}
