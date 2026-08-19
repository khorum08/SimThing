//! CONTENTION-CONSERVATION-JUDGE-0 referee.
//!
//! Table-driven ordinary-path GREEN/RED on `judge_conservation`, plus:
//! - owner-uniformity unrepresentability (no production switch)
//! - test-side folding accountant disagreement on a lawful quantized case
//! - test-side child+parent-only accountant disagreement on an in-flight seam
//!
//! Feedstock is inline and synthetic. Test-side mutants are not production.

use simthing_core::owner_channel::{bind_owner, OwnerRef};
use simthing_core::{
    cost_band_quantize, ResidencyCapacityPartition, SimThing, SimThingId, SimThingKind,
};
use simthing_spec::{
    judge_conservation, ChannelBound, ConservationJudgeReason, ConservationSnapshot,
    ConservationVerdict, OwnerChannelRfConservedValue, OwnerChannelRfOwnAggregate,
    OwnerChannelRfSeamBalance, QuantizedChannelObservation, ResourceKey, SeamObservation,
    StemThingPartitionObservation,
};

const PRODUCTION_JUDGE_SRC: &str =
    include_str!("../../simthing-spec/src/spec/contention_conservation_judge.rs");

fn node() -> SimThing {
    SimThing::new(SimThingKind::Custom("synthetic".into()), 0)
}

fn own(
    simthing_id: SimThingId,
    resource: &str,
    surplus: u32,
    deficit: u32,
) -> OwnerChannelRfOwnAggregate {
    OwnerChannelRfOwnAggregate {
        simthing_id,
        resource_key: ResourceKey::new(resource),
        surplus,
        deficit,
    }
}

fn two_owner_tree(alpha: u32, beta: u32) -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = node();
    bind_owner(&mut root, &OwnerRef::new("alpha"));
    let mut crossing = node();
    bind_owner(&mut crossing, &OwnerRef::new("beta"));
    let crossing_id = crossing.id;
    let root_id = root.id;
    root.add_child(crossing);
    let rows = vec![
        own(root_id, "ore", alpha, 0),
        own(crossing_id, "ore", beta, 0),
    ];
    (root, rows)
}

fn conserved(surplus: u64) -> OwnerChannelRfConservedValue {
    OwnerChannelRfConservedValue {
        participant_count: if surplus == 0 { 0 } else { 1 },
        surplus_total: surplus,
        deficit_total: 0,
        net_surplus: surplus,
        net_deficit: 0,
    }
}

fn ore_channels(supply: u32) -> [ChannelBound; 1] {
    [ChannelBound {
        resource: ResourceKey::new("ore"),
        supply,
        remainder: 0,
    }]
}

fn snapshot<'a>(
    root: &'a SimThing,
    rows: &'a [OwnerChannelRfOwnAggregate],
    channels: &'a [ChannelBound],
    quantized: Option<QuantizedChannelObservation>,
    seam: Option<SeamObservation>,
    stemthing: Option<StemThingPartitionObservation>,
    actionband_originated: &'a [OwnerChannelRfOwnAggregate],
) -> ConservationSnapshot<'a> {
    ConservationSnapshot {
        root,
        own_aggregates: rows,
        channels,
        quantized,
        seam,
        stemthing,
        actionband_originated,
    }
}

fn in_flight_seam() -> SeamObservation {
    let in_flight = conserved(4);
    SeamObservation {
        balance: OwnerChannelRfSeamBalance::observe(
            conserved(0),
            in_flight,
            conserved(0),
            in_flight,
        ),
    }
}

fn lawful_quantized() -> QuantizedChannelObservation {
    let input = cost_band_quantize(10.0, 3.0, true, None).expect("quantize");
    QuantizedChannelObservation {
        input,
        output_created: input.n,
    }
}

/// Test-side wrong accountant: fold created output into the input conservation sum.
fn fold_output_into_input_accountant(obs: QuantizedChannelObservation) -> bool {
    let folded = obs.input.v + obs.output_created as f32;
    folded != obs.input.v
}

/// Test-side wrong accountant: child+parent only, omitting the 6.2 seam.
fn child_parent_only_accountant(balance: OwnerChannelRfSeamBalance) -> bool {
    let child_plus_parent = balance
        .child()
        .surplus_total
        .checked_add(balance.parent().surplus_total);
    let in_flight = balance.seam().surplus_total;
    in_flight != 0 && child_plus_parent != Some(balance.admitted().surplus_total)
}

#[derive(Clone, Copy, Debug)]
enum Case {
    LawfulA,
    LawfulB,
    MultiOwner,
    OverAccounting,
    UnderAccounting,
    QuantizedConserves,
    SeamExact,
    StemThingExact,
    StemThingBroken,
    ActionBandIncluded,
    ActionBandOmitted,
}

fn expected(case: Case) -> ConservationVerdict {
    match case {
        Case::LawfulA
        | Case::LawfulB
        | Case::MultiOwner
        | Case::QuantizedConserves
        | Case::SeamExact
        | Case::StemThingExact
        | Case::ActionBandIncluded => ConservationVerdict::Green,
        Case::OverAccounting => {
            ConservationVerdict::Red(ConservationJudgeReason::SeededOverAccounting)
        }
        Case::UnderAccounting => {
            ConservationVerdict::Red(ConservationJudgeReason::SeededUnderAccounting)
        }
        Case::StemThingBroken => {
            ConservationVerdict::Red(ConservationJudgeReason::StemThingPartition)
        }
        Case::ActionBandOmitted => {
            ConservationVerdict::Red(ConservationJudgeReason::ActionBandOmission)
        }
    }
}

fn judge_case(case: Case) -> ConservationVerdict {
    match case {
        Case::LawfulA => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(&root, &rows, &channels, None, None, None, &[]))
                .expect("reduce")
        }
        Case::LawfulB => {
            let (root, rows) = two_owner_tree(3, 7);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(&root, &rows, &channels, None, None, None, &[]))
                .expect("reduce")
        }
        Case::MultiOwner => {
            let (root, rows) = two_owner_tree(5, 5);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(&root, &rows, &channels, None, None, None, &[]))
                .expect("reduce")
        }
        Case::OverAccounting => {
            let (root, rows) = two_owner_tree(6, 5);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(&root, &rows, &channels, None, None, None, &[]))
                .expect("reduce")
        }
        Case::UnderAccounting => {
            let (root, rows) = two_owner_tree(3, 2);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(&root, &rows, &channels, None, None, None, &[]))
                .expect("reduce")
        }
        Case::QuantizedConserves => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                Some(lawful_quantized()),
                None,
                None,
                &[],
            ))
            .expect("reduce")
        }
        Case::SeamExact => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                None,
                Some(in_flight_seam()),
                None,
                &[],
            ))
            .expect("reduce")
        }
        Case::StemThingExact => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = ore_channels(10);
            let mut partition = ResidencyCapacityPartition::new(16);
            partition.issue(6).expect("issue");
            partition.deliver(2).expect("deliver");
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                None,
                None,
                Some(StemThingPartitionObservation {
                    free: partition.free(),
                    in_flight: partition.in_flight(),
                    occupied: partition.occupied(),
                    capacity: partition.capacity(),
                }),
                &[],
            ))
            .expect("reduce")
        }
        Case::StemThingBroken => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                None,
                None,
                Some(StemThingPartitionObservation {
                    free: 10,
                    in_flight: 0,
                    occupied: 0,
                    capacity: 16,
                }),
                &[],
            ))
            .expect("reduce")
        }
        Case::ActionBandIncluded => {
            let (root, mut rows) = two_owner_tree(6, 4);
            let ab = own(root.id, "actuate", 2, 0);
            rows.push(ab.clone());
            let channels = [
                ChannelBound {
                    resource: ResourceKey::new("ore"),
                    supply: 10,
                    remainder: 0,
                },
                ChannelBound {
                    resource: ResourceKey::new("actuate"),
                    supply: 2,
                    remainder: 0,
                },
            ];
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                None,
                None,
                None,
                std::slice::from_ref(&ab),
            ))
            .expect("reduce")
        }
        Case::ActionBandOmitted => {
            let (root, rows) = two_owner_tree(6, 4);
            let ab = own(root.id, "actuate", 2, 0);
            let channels = ore_channels(10);
            judge_conservation(&snapshot(
                &root,
                &rows,
                &channels,
                None,
                None,
                None,
                std::slice::from_ref(&ab),
            ))
            .expect("reduce")
        }
    }
}

fn assert_owner_uniformity_unrepresentable() {
    for needle in [
        "owner_uniformity_required",
        "OwnerUniformityRejection",
        "owner_uniformity",
        "OwnerUniformity",
        "owners_equal",
        "same_owner",
        "uniform_owner",
        "owner_eq",
        "fold_output_into_input",
        "omit_seam",
        "CrossChannelSum",
    ] {
        assert!(
            !PRODUCTION_JUDGE_SRC.contains(needle),
            "production judge must not contain {needle}"
        );
    }
}

#[test]
fn contention_conservation_judge_table() {
    let cases = [
        Case::LawfulA,
        Case::LawfulB,
        Case::MultiOwner,
        Case::OverAccounting,
        Case::UnderAccounting,
        Case::QuantizedConserves,
        Case::SeamExact,
        Case::StemThingExact,
        Case::StemThingBroken,
        Case::ActionBandIncluded,
        Case::ActionBandOmitted,
    ];
    for case in cases {
        assert_eq!(judge_case(case), expected(case), "case {case:?}");
    }

    assert_eq!(judge_case(Case::MultiOwner), ConservationVerdict::Green);
    assert_owner_uniformity_unrepresentable();

    assert_eq!(
        judge_case(Case::QuantizedConserves),
        ConservationVerdict::Green
    );
    assert!(
        fold_output_into_input_accountant(lawful_quantized()),
        "test-side folding accountant must RED/disagree on the lawful quantized case"
    );

    assert_eq!(judge_case(Case::SeamExact), ConservationVerdict::Green);
    assert!(
        child_parent_only_accountant(in_flight_seam().balance),
        "test-side child+parent-only accountant must RED/disagree on the in-flight seam"
    );
}
