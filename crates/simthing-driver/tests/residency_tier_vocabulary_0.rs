//! RESIDENCY-TIER-VOCABULARY-0 — witnesses.
//!
//! StemThing §§2/5/6: tiers are price vectors, never categories; authoring is
//! open, the admitted session set is frozen, the engine vocabulary beneath is
//! closed, and engine code never branches on authored tier identity. Capacity
//! is exact hard currency (`free + in_flight + occupied = capacity`); census
//! is perception materialized only on granting-active nodes.
//!
//! Each planted defect for this rung mutates the REAL production path it
//! guards (DA `5197291879` amendment): the session admission/freeze door
//! (`SpecSessionState::admit_session_residency_tiers`), the capacity
//! partition transitions (`ResidencyCapacityPartition`), the tier-consumption
//! resolver (`resolve_residency_draw`), and the census materializer
//! (`materialize_granting_census`). The source-level mutation runs are
//! captured in the results doc; these witnesses are the batteries that RED.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    materialize_granting_census, resolve_residency_draw, AdjacencyParticipation, LaneSet,
    ResidencyCapacityPartition, ResidencyChurnClass, ResidencyShapeClass, ResidencyTierRow,
    SimThingId, TierAdmissionError, TierId,
};
use simthing_driver::SpecSessionState;

/// The canonical four generic tiers (StemThing §5 health ratio: a full
/// domain needs on the order of four tiers against dozens of entity names).
/// Names are generic SHAPE labels — domain nouns never enter tier rows.
fn four_generic_tiers() -> Vec<ResidencyTierRow> {
    vec![
        ResidencyTierRow {
            name: "spatial-container".into(),
            lanes: LaneSet::all(),
            shape: ResidencyShapeClass::SpatialBlock,
            adjacency: AdjacencyParticipation::GridN4,
            churn: ResidencyChurnClass::Static,
            unit_cost_rows: 100,
        },
        ResidencyTierRow {
            name: "compact-participant".into(),
            lanes: LaneSet {
                participate: true,
                act: true,
                ..LaneSet::default()
            },
            shape: ResidencyShapeClass::CompactRow,
            adjacency: AdjacencyParticipation::Absent,
            churn: ResidencyChurnClass::Recyclable,
            unit_cost_rows: 1,
        },
        ResidencyTierRow {
            name: "compact-policy-holder".into(),
            lanes: LaneSet {
                originate: true,
                receive: true,
                ..LaneSet::default()
            },
            shape: ResidencyShapeClass::CompactRow,
            adjacency: AdjacencyParticipation::Absent,
            churn: ResidencyChurnClass::Static,
            unit_cost_rows: 1,
        },
        ResidencyTierRow {
            name: "granting-root".into(),
            lanes: LaneSet::all(),
            shape: ResidencyShapeClass::CompactRow,
            adjacency: AdjacencyParticipation::Absent,
            churn: ResidencyChurnClass::Elastic,
            unit_cost_rows: 4,
        },
    ]
}

#[test]
fn residency_tier_vocabulary_0_session_admits_and_freezes_mid_session_mint_reds() {
    let mut session = SpecSessionState::default();

    // Spanned admission failures on invalid authored rows — the door
    // validates before it freezes.
    let mut zero_cost = four_generic_tiers();
    zero_cost[2].unit_cost_rows = 0;
    assert!(matches!(
        session.admit_session_residency_tiers(zero_cost),
        Err(TierAdmissionError::ZeroUnitCost { row_index: 2, .. })
    ));
    let mut dup = four_generic_tiers();
    dup[3].name = "spatial-container".into();
    assert!(matches!(
        session.admit_session_residency_tiers(dup),
        Err(TierAdmissionError::DuplicateName {
            first_row_index: 0,
            row_index: 3,
            ..
        })
    ));
    // Failed admissions froze nothing.
    assert!(session.session_residency_tiers().is_none());

    // Lawful admission freezes the session set.
    let width = session
        .admit_session_residency_tiers(four_generic_tiers())
        .expect("valid authored rows admit")
        .census_width();
    assert_eq!(width, 4);

    // THE PLANTED DEFECT'S TARGET — the mid-session tier mint. Any second
    // admission through the ONE production door is a mint attempt and REDs
    // with the spanned admission failure; the Owner-gated epoch-boundary
    // dynamic-tier door does not exist.
    let mint = vec![ResidencyTierRow {
        name: "late-tier".into(),
        lanes: LaneSet::all(),
        shape: ResidencyShapeClass::CompactRow,
        adjacency: AdjacencyParticipation::Absent,
        churn: ResidencyChurnClass::Recyclable,
        unit_cost_rows: 2,
    }];
    assert!(matches!(
        session.admit_session_residency_tiers(mint),
        Err(TierAdmissionError::MidSessionTierMintRefused {
            admitted: 4,
            attempted: 1,
        })
    ));
    // The frozen set is untouched by the refused mint.
    assert_eq!(
        session.session_residency_tiers().map(|s| s.len()),
        Some(4)
    );
}

#[test]
fn residency_tier_vocabulary_0_capacity_partition_is_exact_over_synthetic_grants() {
    // Inline scenario-neutral synthetic grants: interleaved issue / deliver /
    // cancel / release cycles with live in-flight throughout. Every
    // transition of the PRODUCTION partition re-verifies
    // free + in_flight + occupied = capacity exactly.
    let mut p = ResidencyCapacityPartition::new(10_000);
    let script: &[(&str, u64)] = &[
        ("issue", 2_500),
        ("issue", 400),
        ("deliver", 2_000),
        ("issue", 128),
        ("cancel", 300),
        ("deliver", 500),
        ("release", 750),
        ("issue", 4_096),
        ("deliver", 4_000),
        ("cancel", 128),
        ("release", 5_000),
        ("deliver", 96),
        ("release", 846),
        ("cancel", 100),
    ];
    for &(op, rows) in script {
        match op {
            "issue" => p.issue(rows).expect("free covers the issue"),
            "deliver" => p.deliver(rows).expect("in_flight covers the delivery"),
            "cancel" => p.cancel_in_flight(rows).expect("in_flight covers the cancel"),
            "release" => p.release(rows).expect("occupied covers the release"),
            _ => unreachable!(),
        }
        p.verify_exact().expect("partition holds after every transition");
    }
    assert_eq!(p.capacity(), 10_000);
    assert_eq!(p.free() + p.in_flight() + p.occupied(), 10_000);
    assert_eq!(p.in_flight(), 0);
    assert_eq!(p.occupied(), 0);
    assert_eq!(p.free(), 10_000);

    // Over-draws refuse exactly — no approximate conservation anywhere.
    let mut q = ResidencyCapacityPartition::new(10);
    q.issue(10).unwrap();
    assert!(q.issue(1).is_err());
    q.deliver(10).unwrap();
    assert!(q.deliver(1).is_err());
    assert!(q.cancel_in_flight(1).is_err());
    q.release(10).unwrap();
    assert!(q.release(1).is_err());
    q.verify_exact().unwrap();
}

#[test]
fn residency_tier_vocabulary_0_consumption_is_identity_blind_many_names_few_tiers() {
    let mut session = SpecSessionState::default();
    // Two rows with IDENTICAL price vectors and different authored names —
    // one of them named like the row an identity branch would single out.
    let mut tiers = four_generic_tiers();
    let mut mirror = tiers[3].clone();
    mirror.name = "granting-root-mirror".into();
    tiers.push(mirror);
    let set = session
        .admit_session_residency_tiers(tiers)
        .expect("admit")
        .clone();

    // Identity blindness: equal vectors resolve to byte-equal shapes at any
    // draw quantity, whatever the rows are called. A `match tier`/domain
    // branch in the production consumption path REDs here.
    let root = set
        .tier(set.tier_id_by_name("granting-root").unwrap())
        .unwrap();
    let mirror = set
        .tier(set.tier_id_by_name("granting-root-mirror").unwrap())
        .unwrap();
    for n in [0_u32, 1, 7, 25, 10_000] {
        assert_eq!(
            resolve_residency_draw(root, n),
            resolve_residency_draw(mirror, n),
            "authored tier identity leaked into engine behavior at n={n}"
        );
    }
    assert_eq!(resolve_residency_draw(root, 25).rows, 100);

    // Health ratio: dozens of authored entity names share the four tiers as
    // PURE DATA — a name→tier binding table, zero engine change. Entities
    // bound to the same tier are indistinguishable downstream.
    let binding: BTreeMap<String, TierId> = (0..40)
        .map(|i| {
            let tier_name = match i % 4 {
                0 => "spatial-container",
                1 => "compact-participant",
                2 => "compact-policy-holder",
                _ => "granting-root",
            };
            (
                format!("entity-kind-{i:02}"),
                set.tier_id_by_name(tier_name).unwrap(),
            )
        })
        .collect();
    assert_eq!(binding.len(), 40);
    let shapes: BTreeSet<_> = binding
        .values()
        .map(|&tid| resolve_residency_draw(set.tier(tid).unwrap(), 3))
        .collect();
    assert!(
        shapes.len() <= 5,
        "forty entity names resolve through at most the five admitted price vectors"
    );
}

#[test]
fn residency_tier_vocabulary_0_census_is_sparse_bytes_absent_on_non_granting_nodes() {
    let mut session = SpecSessionState::default();
    let set = session
        .admit_session_residency_tiers(four_generic_tiers())
        .expect("admit")
        .clone();

    let nodes: BTreeSet<SimThingId> = (1..=200).map(SimThingId::from_session_raw).collect();
    let granting: BTreeSet<SimThingId> =
        [1, 40, 155].map(SimThingId::from_session_raw).into_iter().collect();

    let census = materialize_granting_census(&set, &nodes, &granting);
    assert_eq!(census.width(), 4);
    assert_eq!(census.granting_node_count(), 3);

    // Granting-active nodes carry the fixed-width lanes.
    let per_node_bytes = census
        .lanes(SimThingId::from_session_raw(40))
        .expect("granting node has lanes")
        .lane_bytes();
    assert_eq!(per_node_bytes, 4 * (4 + 4 + 4));

    // Non-granting nodes are ABSENT — no zero-filled rows, zero bytes.
    for raw in [2_u32, 39, 41, 199, 200] {
        assert!(
            census.lanes(SimThingId::from_session_raw(raw)).is_none(),
            "non-granting node {raw} must allocate no census lanes"
        );
    }
    assert_eq!(census.total_lane_bytes(), 3 * per_node_bytes);

    // The memory profile scales with granting activity, never with node
    // count: a 10× larger universe with the same granting set costs the
    // same bytes. A dense/non-sparse materialization REDs here.
    let big_nodes: BTreeSet<SimThingId> = (1..=2_000).map(SimThingId::from_session_raw).collect();
    let big = materialize_granting_census(&set, &big_nodes, &granting);
    assert_eq!(big.granting_node_count(), 3);
    assert_eq!(big.total_lane_bytes(), census.total_lane_bytes());
}
