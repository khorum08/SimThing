//! PLAN-STRUCT-TYPING-0 referees: typed plan columns + WGSL wire parity.

use simthing_core::{
    AccumulatorOp, ClampBehavior, ColumnIndex, CombineFn, ConsumeMode, GateSpec,
    PropertyColumnRange, PropertyLayout, ScaleSpec, SlotIndex, SourceSpec, SubFieldRole,
    SubFieldSpec,
};
use simthing_kernel::{
    column_from_wire, encode_column, governed_pairs_for_property, AccumulatorOpGpu, GovernedPair,
    CLAMP_UNBOUNDED,
};

fn sample_layout() -> PropertyLayout {
    PropertyLayout {
        sub_fields: vec![
            SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: "amount".into(),
                display_range: None,
                governed_by: Some(SubFieldRole::Velocity),
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Velocity,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: Some(1.0),
                default: 0.0,
                display_name: "velocity".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
        ],
    }
}

#[test]
fn encode_column_and_column_from_wire_round_trip_bits() {
    let col = ColumnIndex::from_raw_for_oracle_or_rehearsal(17);
    assert_eq!(encode_column(col), 17);
    assert_eq!(column_from_wire(17), col);
    assert_eq!(column_from_wire(u32::MAX).raw_u32(), u32::MAX);
}

#[test]
fn governed_pair_wire_bytes_drop_only_through_encode_column() {
    let layout = sample_layout();
    let range = PropertyColumnRange {
        start: 4,
        stride: layout.stride(),
    };
    let pairs = governed_pairs_for_property(&range, &layout);
    assert_eq!(pairs.len(), 1);
    let expected = GovernedPair {
        governed_col: encode_column(range.col_for_role(&SubFieldRole::Amount, &layout).unwrap()),
        governing_col: encode_column(
            range
                .col_for_role(&SubFieldRole::Velocity, &layout)
                .unwrap(),
        ),
        clamp_min: f32::NEG_INFINITY,
        clamp_max: f32::INFINITY,
        vel_max: f32::INFINITY,
        clamp_kind: CLAMP_UNBOUNDED,
    };
    assert_eq!(pairs[0], expected);
    // POD layout unchanged: two u32 cols then three f32 then u32.
    assert_eq!(std::mem::size_of::<GovernedPair>(), 24);
}

#[test]
fn accumulator_op_encode_preserves_typed_column_wire_bits() {
    let src = ColumnIndex::from_raw_for_oracle_or_rehearsal(3);
    let dst = ColumnIndex::from_raw_for_oracle_or_rehearsal(9);
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(2),
            col: src,
        },
        combine: CombineFn::Identity,
        gate: GateSpec::OrderBand(4),
        scale: ScaleSpec::Identity,
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(2), dst)],
    };
    let gpu = AccumulatorOpGpu::from_op(&op).expect("encode");
    assert_eq!(gpu.source_col, encode_column(src));
    assert_eq!(gpu.target0_col, encode_column(dst));
    assert_eq!(column_from_wire(gpu.source_col), src);
    assert_eq!(column_from_wire(gpu.target0_col), dst);
}
