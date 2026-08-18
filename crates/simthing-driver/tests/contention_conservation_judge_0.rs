//! CONTENTION-CONSERVATION-JUDGE-0 referee.
//!
//! One table-driven battery. Every GREEN/RED is a verdict of `judge_conservation`.
//! Feedstock is inline and synthetic.

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
    let rows = vec![own(root_id, "ore", alpha, 0), own(crossing_id, "ore", beta, 0)];
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

#[derive(Clone, Copy, Debug)]
enum Case {
    LawfulA,
    LawfulB,
    MultiOwner,
    OverAccounting,
    UnderAccounting,
    OwnerUniformity,
    QuantizedConserves,
    CrossChannelSum,
    SeamExact,
    ChildParentOnly,
    StemThingExact,
    StemThingBroken,
    ActionBandIncluded,
    ActionBandOmitted,
}

fn expected(case: Case) -> ConservationVerdict {
    match case {
        Case::LawfulA | Case::LawfulB | Case::MultiOwner | Case::QuantizedConserves
        | Case::SeamExact | Case::StemThingExact | Case::ActionBandIncluded => {
            ConservationVerdict::Green
        }
        Case::OverAccounting => {
            ConservationVerdict::Red(ConservationJudgeReason::SeededOverAccounting)
        }
        Case::UnderAccounting => {
            ConservationVerdict::Red(ConservationJudgeReason::SeededUnderAccounting)
        }
        Case::OwnerUniformity => {
            ConservationVerdict::Red(ConservationJudgeReason::OwnerUniformityRejection)
        }
        Case::CrossChannelSum => ConservationVerdict::Red(ConservationJudgeReason::CrossChannelSum),
        Case::ChildParentOnly => ConservationVerdict::Red(ConservationJudgeReason::ChildParentOnly),
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
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::LawfulB => {
            let (root, rows) = two_owner_tree(3, 7);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::MultiOwner => {
            let (root, rows) = two_owner_tree(5, 5);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::OverAccounting => {
            let (root, rows) = two_owner_tree(6, 5);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::UnderAccounting => {
            let (root, rows) = two_owner_tree(3, 2);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::OwnerUniformity => {
            let (root, rows) = two_owner_tree(5, 5);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: true,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::QuantizedConserves => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            let input = cost_band_quantize(10.0, 3.0, true, None).expect("quantize");
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: Some(QuantizedChannelObservation {
                    input,
                    output_created: input.n,
                    fold_output_into_input: false,
                }),
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::CrossChannelSum => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            let input = cost_band_quantize(10.0, 3.0, true, None).expect("quantize");
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: Some(QuantizedChannelObservation {
                    input,
                    output_created: input.n,
                    fold_output_into_input: true,
                }),
                seam: None,
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::SeamExact => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            let in_flight = conserved(4);
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: Some(SeamObservation {
                    balance: OwnerChannelRfSeamBalance::observe(
                        conserved(0),
                        in_flight,
                        conserved(0),
                        in_flight,
                    ),
                    omit_seam: false,
                }),
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::ChildParentOnly => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            let in_flight = conserved(4);
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: Some(SeamObservation {
                    balance: OwnerChannelRfSeamBalance::observe(
                        conserved(0),
                        in_flight,
                        conserved(0),
                        in_flight,
                    ),
                    omit_seam: true,
                }),
                stemthing: None,
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::StemThingExact => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            let mut partition = ResidencyCapacityPartition::new(16);
            partition.issue(6).expect("issue");
            partition.deliver(2).expect("deliver");
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: Some(StemThingPartitionObservation {
                    free: partition.free(),
                    in_flight: partition.in_flight(),
                    occupied: partition.occupied(),
                    capacity: partition.capacity(),
                }),
                actionband_originated: &[],
            })
            .expect("reduce")
        }
        Case::StemThingBroken => {
            let (root, rows) = two_owner_tree(6, 4);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: Some(StemThingPartitionObservation {
                    free: 10,
                    in_flight: 0,
                    occupied: 0,
                    capacity: 16,
                }),
                actionband_originated: &[],
            })
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
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[ab],
            })
            .expect("reduce")
        }
        Case::ActionBandOmitted => {
            let (root, rows) = two_owner_tree(6, 4);
            let ab = own(root.id, "actuate", 2, 0);
            let channels = [ChannelBound {
                resource: ResourceKey::new("ore"),
                supply: 10,
                remainder: 0,
            }];
            judge_conservation(&ConservationSnapshot {
                root: &root,
                own_aggregates: &rows,
                channels: &channels,
                owner_uniformity_required: false,
                quantized: None,
                seam: None,
                stemthing: None,
                actionband_originated: &[ab],
            })
            .expect("reduce")
        }
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
        Case::OwnerUniformity,
        Case::QuantizedConserves,
        Case::CrossChannelSum,
        Case::SeamExact,
        Case::ChildParentOnly,
        Case::StemThingExact,
        Case::StemThingBroken,
        Case::ActionBandIncluded,
        Case::ActionBandOmitted,
    ];
    for case in cases {
        assert_eq!(judge_case(case), expected(case), "case {case:?}");
    }
}
