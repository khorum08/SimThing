//! rf_interior_policy_composition_0 — the interior-policy law completed (DA
//! ruling, live Board 5879126789, intervention 2; completes #2035/#2036).
//!
//! The recursive cycle reduces a subtree UP, applies the overlay modification,
//! and disburses DOWN (0.0.8.7 P0; core §8.4.3's upward statistic); weight
//! columns "default to Demand-proportional and are overlay-modifiable via
//! existing Add/Multiply/Set OrderBands" (RF invariants). So a standing Multiply
//! policy deforms its host's participation exactly once:
//!
//! * at the host, on its rolled-up total, when the host's RF children are not
//!   its physical descendants — an owner seat holds no spatial participants;
//!   resource parentage is not containment (the shipped shape);
//! * through tree-position inheritance when they all are.
//!
//! A Set still replaces. Neutral trees are untouched. A standing Add at an RF
//! interior, and a policy reaching only some of its RF children, refuse typed.
//!
//! Shape: a root injecting `R` over two owner seats; Terran owns one site of
//! four cohorts, Pirate two; every base weight is 1. Each owner's AllocatedFlow
//! is the root's split.

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

/// Where an owner's sites live: beside the seat under the root, owned through a
/// resource-parent edge (the shipped shape), or physically inside the seat.
#[derive(Clone, Copy)]
enum Topology {
    Seat,
    Contained,
    /// Pirate's first site physically inside the seat, its second by edge.
    Split,
}

/// Owner AllocatedFlow `[terran, pirate]` for generations 1 and 2, or the
/// refusal the ordinary session raised at open.
fn split(
    topology: Topology,
    root_flow: f32,
    terran: Option<TransformOp>,
    pirate: Option<TransformOp>,
) -> Result<[[f32; 2]; 2], String> {
    let mut registry = DimensionRegistry::new();
    let pid = property(&mut registry);
    let mut root = SimThing::new(SimThingKind::World, 0);
    author(&registry, pid, &mut root, root_flow);
    let mut ids = [None; 2];
    for (index, (sites, policy)) in [(1, terran), (2, pirate)].into_iter().enumerate() {
        let mut owner = SimThing::new(SimThingKind::Cohort, 0);
        author(&registry, pid, &mut owner, 0.0);
        if let Some(op) = policy {
            owner.overlays.push(standing_policy(pid, owner.id, op));
        }
        ids[index] = Some(owner.id);
        for number in 0..sites {
            let mut site = site(&registry, pid);
            let contained = match topology {
                Topology::Seat => false,
                Topology::Contained => true,
                Topology::Split => index == 1 && number == 0,
            };
            if contained {
                owner.add_child(site);
            } else {
                site.add_resource_parent_edge(NAMESPACE, NAME, owner.id, None);
                root.add_child(site);
            }
        }
        root.add_child(owner);
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

fn assert_split(
    case: &str,
    actual: Result<[[f32; 2]; 2], String>,
    expected: [f32; 2],
    generations: usize,
) {
    let actual = actual.unwrap_or_else(|error| panic!("{case}: session refused: {error}"));
    for (generation, split) in actual.iter().take(generations).enumerate() {
        assert!(
            split.iter().zip(expected).all(|(got, want)| (got - want).abs() <= 1e-5),
            "{case}: generation {}: owner split {split:?}, lawful {expected:?} (all generations: {actual:?})",
            generation + 1
        );
    }
}

#[test]
fn seat_multiply_policy_scales_the_rolled_up_total() {
    // Terran total 4 (x1); Pirate 1.5 x 8 = 12; the root's 4 splits 4:12 and stays
    // there: the seat's weight is re-derived every generation, never compounded.
    // The erased-aggregate law gave 1 : 1.5 whatever the subtrees needed.
    let actual = split(
        Topology::Seat,
        4.0,
        Some(TransformOp::multiply(1.0)),
        Some(TransformOp::multiply(1.5)),
    );
    assert_split("seat multiply", actual, [1.0, 3.0], 2);
}

#[test]
fn contained_multiply_policy_is_carried_once_by_inheritance() {
    // Every Pirate cohort inherits x1.5, so the seat's plain total is already
    // 12: a second application at the seat would give 1.5 x 12 = 18 (0.73/3.27).
    // Generation 1 only: the overlay pass re-applies a standing Multiply to the
    // persistent cohort cells every tick, a separate defect held for its own
    // ruling (it compounds leaf policies alike, and is not this law's to fix).
    let actual = split(
        Topology::Contained,
        4.0,
        Some(TransformOp::multiply(1.0)),
        Some(TransformOp::multiply(1.5)),
    );
    assert_split("contained multiply", actual, [1.0, 3.0], 1);
}

#[test]
fn set_policy_still_replaces_the_seats_participation() {
    // Pirate participates at exactly 2 whatever its subtree holds; Terran at 4.
    let actual = split(Topology::Seat, 6.0, None, Some(TransformOp::set(2.0)));
    assert_split("seat set", actual, [4.0, 2.0], 2);
}

#[test]
fn neutral_seats_carry_their_subtree_totals() {
    let actual = split(Topology::Seat, 6.0, None, None);
    assert_split("neutral", actual, [2.0, 4.0], 2);
}

#[test]
fn interior_add_policy_and_split_reach_refuse_typed() {
    let refusal = split(Topology::Seat, 5.0, None, Some(TransformOp::add(1.0)))
        .expect_err("a standing Add at an RF interior must refuse");
    assert!(refusal.contains("InteriorAddWeightPolicy"), "{refusal}");
    let refusal = split(Topology::Split, 4.0, None, Some(TransformOp::multiply(1.5)))
        .expect_err("a policy reaching only some RF children must refuse");
    assert!(
        refusal.contains("InteriorWeightPolicyReachSplit"),
        "{refusal}"
    );
}
