//! WRITE-DOOR-BAND-DELTA-0 referees: fused band-crossing deltas + structural remap gates.

use simthing_core::{
    validate_anchor_remap_for_encode, AnchorLocusRemap, AnchorRemapOperation, AnchorRemapSection,
    ColumnIndex, SimPropertyId, SimThingId, SlotIndex,
};
use simthing_gpu::{
    cpu_oracle_band_crossing_deltas, BandCrossingDirection, ThresholdRegistration, DIR_DOWNWARD,
    DIR_UPWARD, THRESH_BUF_VALUES,
};
use simthing_sim::gate_structural_gpu_encode;

#[test]
fn rising_falling_exact_edge_no_crossing_and_multi_edge_oracle() {
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
        cpu_oracle_band_crossing_deltas(&prev, &curr, &[], &[], 2, &regs, Some(&[0, 1]));
    assert_eq!(rising.len(), 2);
    assert_eq!(rising[0].direction(), BandCrossingDirection::Rising);
    assert_eq!(rising[1].direction(), BandCrossingDirection::Rising);

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
        Some(&[0]),
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
        Some(&[1]),
    );
    assert_eq!(falling.len(), 1);
    assert_eq!(falling[0].direction(), BandCrossingDirection::Falling);

    // No crossing when values stay on the same side.
    let prev_nc = [0.0f32, 6.0];
    let curr_nc = [0.5f32, 5.5];
    let none =
        cpu_oracle_band_crossing_deltas(&prev_nc, &curr_nc, &[], &[], 2, &regs, Some(&[0, 1]));
    assert!(none.is_empty());
}

#[test]
fn remap_less_structural_encode_is_rejected_with_operation_context() {
    let id = SimThingId::from_session_raw(9);
    let prop = SimPropertyId(3);
    let section = AnchorRemapSection::with_remaps(AnchorRemapOperation::AddChild, vec![]);
    let err = gate_structural_gpu_encode(&section, &[(id, prop)]).unwrap_err();
    assert_eq!(err.operation, AnchorRemapOperation::AddChild);
    assert_eq!(err.missing, vec![(id, prop)]);
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
fn replay_delta_log_carries_anchor_remap_section() {
    // Evidence-only: BoundaryDeltaEntry::AnchorRemapApplied is part of the
    // sealed transport; bit-exact clone of the section must round-trip.
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
    let encoded = serde_json::to_string(&section).expect("serialize remap section");
    let decoded: AnchorRemapSection =
        serde_json::from_str(&encoded).expect("deserialize remap section");
    assert_eq!(decoded, section);
}
