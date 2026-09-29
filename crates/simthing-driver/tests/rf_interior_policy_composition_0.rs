//! rf_interior_policy_composition_0 — the interior-policy law completed (DA
//! ruling, live Board 5879126789, intervention 2; completes #2035/#2036).
//!
//! The recursive cycle reduces a subtree UP, applies the overlay modification,
//! and disburses DOWN (0.0.8.7 P0; core §8.4.3's upward statistic); weight
//! columns "default to Demand-proportional and are overlay-modifiable via
//! existing Add/Multiply/Set OrderBands" (RF invariants). So a standing
//! Multiply/Add stack deforms its host's participation exactly once:
//!
//! * at the host, on its rolled-up total, when the host's RF children are not
//!   its physical descendants — an owner seat holds no spatial participants;
//!   resource parentage is not containment (the shipped shape);
//! * through tree-position inheritance when they all are (Multiply only: an
//!   inherited Add would land once per participant).
//!
//! A Set still replaces. Neutral trees are untouched. An inherited Add, and a
//! policy reaching only some of its RF children, refuse typed.
//!
//! Shape: a root injecting `R` over two owner seats; the small seat owns one
//! site of four cohorts, the large seat two; every base weight is 1, so the
//! seats' plain totals are 4 and 8. Each seat's AllocatedFlow is the root's
//! split.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    PropertyValue, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
    TransformOp,
};
use simthing_driver::{derive_resource_flow_admission, Scenario, SimSession};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, GameModeSpec, PropertySpec};

const NAMESPACE: &str = "interior_policy";
const NAME: &str = "supply";

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.into())
}

fn property(registry: &mut DimensionRegistry) -> SimPropertyId {
    let field = |name: &str, accumulator: Option<AccumulatorRole>| SubFieldSpec {
        role: role(name),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: accumulator.map(|role| AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    };
    let mut balance = field(
        "balance",
        Some(AccumulatorRole::Balance(BalanceSpec::default())),
    );
    balance.governed_by = Some(role("balance_rate"));
    compile_property(
        &PropertySpec {
            admission_disposition: Default::default(),
            id: NAME.into(),
            namespace: NAMESPACE.into(),
            name: NAME.into(),
            display_name: NAME.into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", Some(AccumulatorRole::IntrinsicFlow)),
                field(
                    "allocated",
                    Some(AccumulatorRole::AllocatedFlow { arena: NAME.into() }),
                ),
                field(
                    "weight",
                    Some(AccumulatorRole::AllocatorWeight { arena: NAME.into() }),
                ),
                field("balance_rate", None),
                balance,
            ],
        },
        registry,
    )
    .expect("ordinary property admission")
    .0
}

fn author(registry: &DimensionRegistry, pid: SimPropertyId, node: &mut SimThing, flow: f32) {
    let layout = &registry.property(pid).layout;
    let mut value = PropertyValue::from_layout(layout);
    value.set_role(&role("flow"), layout, flow);
    value.set_role(&role("weight"), layout, 1.0);
    node.add_property(pid, value);
}

fn standing_policy(pid: SimPropertyId, host: SimThingId, op: TransformOp) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin: host,
        affects: vec![host],
        transform: PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(role("weight"), op)],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

fn site(registry: &DimensionRegistry, pid: SimPropertyId) -> SimThing {
    let mut site = SimThing::new(SimThingKind::Cohort, 0);
    author(registry, pid, &mut site, 0.0);
    for _ in 0..4 {
        let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
        author(registry, pid, &mut cohort, 0.0);
        site.add_child(cohort);
    }
    site
}

/// Where a seat's sites live: beside the seat under the root, owned through a
/// resource-parent edge (the shipped shape), or physically inside the seat.
#[derive(Clone, Copy)]
enum Topology {
    Seat,
    Contained,
    /// The large seat's first site physically inside it, its second by edge.
    Split,
}

/// Seat AllocatedFlow `[small, large]` for generations 1 and 2, or the
/// refusal the ordinary session raised at open. Each seat's policies stack in
/// the given order.
fn split(
    topology: Topology,
    root_flow: f32,
    small: &[TransformOp],
    large: &[TransformOp],
) -> Result<[[f32; 2]; 2], String> {
    let mut registry = DimensionRegistry::new();
    let pid = property(&mut registry);
    let mut root = SimThing::new(SimThingKind::World, 0);
    author(&registry, pid, &mut root, root_flow);
    let mut ids = [None; 2];
    for (index, (sites, policies)) in [(1, small), (2, large)].into_iter().enumerate() {
        let mut seat = SimThing::new(SimThingKind::Cohort, 0);
        author(&registry, pid, &mut seat, 0.0);
        for op in policies {
            seat.overlays
                .push(standing_policy(pid, seat.id, op.clone()));
        }
        ids[index] = Some(seat.id);
        for number in 0..sites {
            let mut site = site(&registry, pid);
            let contained = match topology {
                Topology::Seat => false,
                Topology::Contained => true,
                Topology::Split => index == 1 && number == 0,
            };
            if contained {
                seat.add_child(site);
            } else {
                site.add_resource_parent_edge(NAMESPACE, NAME, seat.id, None);
                root.add_child(site);
            }
        }
        root.add_child(seat);
    }
    let mut scenario = Scenario {
        name: "rf-interior-policy-composition".into(),
        registry,
        root,
        n_slots: 32,
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut scenario.registry,
        &mut scenario.root,
    );
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&scenario.root).unwrap();
    let mut rf =
        derive_resource_flow_admission(None, &scenario.registry, &scenario.root, &allocator)
            .expect("ordinary derived RF participation")
            .spec
            .expect("the supply property derives one arena");
    for arena in &mut rf.arenas {
        arena.max_orderband_depth = 16;
    }
    let spec = GameModeSpec {
        id: "rf-interior-policy-composition".into(),
        resource_flow: Some(rf),
        ..Default::default()
    };
    let mut session =
        SimSession::open_from_spec(scenario, &spec).map_err(|error| format!("{error:?}"))?;
    let registry = &session.proto.registry;
    let column = registry
        .column_range(pid)
        .col_for_role(&role("allocated"), &registry.property(pid).layout)
        .unwrap()
        .raw();
    let total = registry.total_columns;
    let slots = ids.map(|id| {
        session
            .proto
            .allocator
            .slot_of(id.unwrap())
            .unwrap()
            .as_usize()
    });
    let mut out = [[0.0; 2]; 2];
    for generation in out.iter_mut() {
        session.step_once().expect("one ordinary generation");
        let values = session.state.read_values();
        *generation = slots.map(|slot| values[slot * total + column]);
    }
    Ok(out)
}

struct Lawful {
    case: &'static str,
    topology: Topology,
    root_flow: f32,
    small: Vec<TransformOp>,
    large: Vec<TransformOp>,
    split: [f32; 2],
    generations: usize,
}

#[test]
fn every_host_shape_deforms_its_rolled_up_total_once() {
    use TransformOp as Op;
    let cases = vec![
        // The erased-aggregate law gave 1.6/2.4 whatever the subtrees needed.
        Lawful {
            case: "seat multiply: 4 x1 : 8 x1.5",
            topology: Topology::Seat,
            root_flow: 4.0,
            small: vec![Op::multiply(1.0)],
            large: vec![Op::multiply(1.5)],
            split: [1.0, 3.0],
            generations: 2,
        },
        Lawful {
            case: "seat add: 4 : 8 + 4",
            topology: Topology::Seat,
            root_flow: 4.0,
            small: vec![],
            large: vec![Op::add(4.0)],
            split: [1.0, 3.0],
            generations: 2,
        },
        // The stack composes in its own order: 0.5 x 8 + 8 = 12, (8 + 8) x 0.5 = 8.
        Lawful {
            case: "seat stack, multiply then add",
            topology: Topology::Seat,
            root_flow: 6.0,
            small: vec![],
            large: vec![Op::multiply(0.5), Op::add(8.0)],
            split: [1.5, 4.5],
            generations: 2,
        },
        Lawful {
            case: "seat stack, add then multiply",
            topology: Topology::Seat,
            root_flow: 6.0,
            small: vec![],
            large: vec![Op::add(8.0), Op::multiply(0.5)],
            split: [2.0, 4.0],
            generations: 2,
        },
        // Every large-seat cohort inherits x1.5, so its plain total is already
        // 12; a second application at the seat would give 18 (0.73/3.27).
        // Generation 1 only: the overlay pass re-applies a standing Multiply to
        // the persistent cohort cells every tick, a separate defect held for
        // its own ruling (it compounds leaf policies alike).
        Lawful {
            case: "contained multiply, carried once by inheritance",
            topology: Topology::Contained,
            root_flow: 4.0,
            small: vec![Op::multiply(1.0)],
            large: vec![Op::multiply(1.5)],
            split: [1.0, 3.0],
            generations: 1,
        },
        Lawful {
            case: "seat set replaces the participation",
            topology: Topology::Seat,
            root_flow: 6.0,
            small: vec![],
            large: vec![Op::set(2.0)],
            split: [4.0, 2.0],
            generations: 2,
        },
        Lawful {
            case: "neutral seats carry their totals",
            topology: Topology::Seat,
            root_flow: 6.0,
            small: vec![],
            large: vec![],
            split: [2.0, 4.0],
            generations: 2,
        },
    ];
    let mut unlawful = Vec::new();
    for case in cases {
        match split(case.topology, case.root_flow, &case.small, &case.large) {
            Err(error) => unlawful.push(format!("{}: session refused: {error}", case.case)),
            Ok(actual) => {
                for (generation, got) in actual.iter().take(case.generations).enumerate() {
                    if got
                        .iter()
                        .zip(case.split)
                        .any(|(got, want)| (got - want).abs() > 1e-5)
                    {
                        unlawful.push(format!(
                            "{}: generation {}: split {got:?}, lawful {:?}",
                            case.case,
                            generation + 1,
                            case.split
                        ));
                    }
                }
            }
        }
    }
    assert!(unlawful.is_empty(), "{unlawful:#?}");
}

#[test]
fn ambiguous_interior_policies_refuse_typed() {
    let cases = [
        (
            Topology::Contained,
            TransformOp::add(1.0),
            "InheritedAddWeightPolicy",
        ),
        (
            Topology::Split,
            TransformOp::multiply(1.5),
            "InteriorWeightPolicyReachSplit",
        ),
    ];
    for (topology, op, refusal) in cases {
        let error = split(topology, 4.0, &[], &[op])
            .expect_err("an ambiguous interior policy must refuse at open");
        assert!(error.contains(refusal), "expected {refusal}: {error}");
    }
}
