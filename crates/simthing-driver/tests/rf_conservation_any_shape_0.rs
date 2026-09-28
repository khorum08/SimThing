//! rf_conservation_any_shape_0 — RF conservation for ANY tree shape (DA ruling,
//! live Board 5879126789, intervention 1).
//!
//! The Invariant Set holds "for any input, so each is provable over
//! inline-constructed input" (0.0.8.7 binding laws). RF conservation had only
//! been witnessed on depth-1 and depth-2 fixtures, and two settlement defects
//! hid beyond them for a whole track: #2065 created mass below depth 1 and the
//! historical interior path destroyed it (repaired by #2076). This witness
//! generates trees of depth 1..=5 carrying intrinsic sources AND sinks on the
//! root, interiors and leaves, zero and non-zero weights, and Set/Add/Multiply
//! weight policies on interiors, and judges every generation of the ORDINARY
//! resident session with the independent RF-1 structural check. Every
//! participant's Balance is governed, so no terminal allocation leaves the
//! arena: Σ intrinsic flow = Σ ΔBalance within the oracle's O(ε·n) bound.
//! Policies change shares, never totals.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    PropertyValue, SimPropertyId, SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
    TransformOp,
};
use simthing_driver::{
    check_arena_structural, derive_resource_flow_admission, ArenaConservationSnapshot,
    ArenaMemberObservation, ArenaStructuralEvidence, Scenario, SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, GameModeSpec, PropertySpec};

const SEEDS: u64 = 32;
const GENERATIONS: u32 = 3;
const MAX_NODES: usize = 40;

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
            id: "stock".into(),
            namespace: "any_shape".into(),
            name: "stock".into(),
            display_name: "stock".into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", Some(AccumulatorRole::IntrinsicFlow)),
                field(
                    "allocated",
                    Some(AccumulatorRole::AllocatedFlow {
                        arena: "stock".into(),
                    }),
                ),
                field(
                    "weight",
                    Some(AccumulatorRole::AllocatorWeight {
                        arena: "stock".into(),
                    }),
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

/// splitmix64: a deterministic shape generator (coverage, never validity).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn pick<T: Copy>(&mut self, choices: &[T]) -> T {
        choices[(self.next() % choices.len() as u64) as usize]
    }
}

/// One authored participant, in the order the snapshot observes it.
struct Participant {
    id: SimThingId,
    flow: f32,
    is_leaf: bool,
}

/// The generated tree plus the facts the oracle needs.
struct Shape {
    label: String,
    registry: DimensionRegistry,
    pid: SimPropertyId,
    root: SimThing,
    participants: Vec<Participant>,
    depth: usize,
    policies: usize,
}

fn author(
    registry: &DimensionRegistry,
    pid: SimPropertyId,
    node: &mut SimThing,
    flow: f32,
    weight: f32,
) {
    let layout = &registry.property(pid).layout;
    let mut value = PropertyValue::from_layout(layout);
    value.set_role(&role("flow"), layout, flow);
    value.set_role(&role("weight"), layout, weight);
    node.add_property(pid, value);
}

fn weight_policy(pid: SimPropertyId, host: SimThingId, op: TransformOp) -> Overlay {
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

fn policy(selector: u8) -> Option<TransformOp> {
    match selector {
        0 => Some(TransformOp::multiply(2.0)),
        1 => Some(TransformOp::multiply(0.5)),
        2 => Some(TransformOp::add(1.0)),
        3 => Some(TransformOp::set(2.0)),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn grow(
    rng: &mut Rng,
    registry: &DimensionRegistry,
    pid: SimPropertyId,
    parent: &mut SimThing,
    depth: usize,
    max_depth: usize,
    participants: &mut Vec<Participant>,
    policies: &mut usize,
) {
    let fanout = if depth == 1 {
        rng.pick(&[1usize, 2, 3])
    } else {
        rng.pick(&[1usize, 2])
    };
    for index in 0..fanout {
        if participants.len() >= MAX_NODES {
            return;
        }
        let mut child = SimThing::new(SimThingKind::Cohort, 0);
        let flow = rng.pick(&[0.0f32, 0.0, 0.5, 1.0, 2.0, -0.5, -1.0]);
        let weight = rng.pick(&[0.0f32, 0.5, 1.0, 1.0, 2.0, 4.0]);
        author(registry, pid, &mut child, flow, weight);
        let slot = participants.len();
        participants.push(Participant {
            id: child.id,
            flow,
            is_leaf: true,
        });
        // The first child always descends, so every generated tree reaches max_depth.
        if depth < max_depth && (index == 0 || rng.pick(&[true, false])) {
            grow(
                rng,
                registry,
                pid,
                &mut child,
                depth + 1,
                max_depth,
                participants,
                policies,
            );
        }
        if !child.children.is_empty() {
            participants[slot].is_leaf = false;
            if let Some(op) = policy(rng.pick(&[0u8, 1, 2, 3, 4, 4, 4])) {
                child.overlays.push(weight_policy(pid, child.id, op));
                *policies += 1;
            }
        }
        parent.add_child(child);
    }
}

fn generated(seed: u64) -> Shape {
    let mut rng = Rng(seed);
    let mut registry = DimensionRegistry::new();
    let pid = property(&mut registry);
    let depth = 1 + (seed % 5) as usize;
    let mut root = SimThing::new(SimThingKind::World, 0);
    let root_flow = rng.pick(&[0.0f32, 1.0, 3.0, -1.0]);
    author(&registry, pid, &mut root, root_flow, 0.0);
    let mut participants = vec![Participant {
        id: root.id,
        flow: root_flow,
        is_leaf: false,
    }];
    let mut policies = 0;
    grow(
        &mut rng,
        &registry,
        pid,
        &mut root,
        1,
        depth,
        &mut participants,
        &mut policies,
    );
    Shape {
        label: format!("seed {seed}"),
        registry,
        pid,
        root,
        participants,
        depth,
        policies,
    }
}

/// The shipped 1.2 economy's shape: two owner interiors carrying Multiply weight
/// policies (x1 and x1.5), one site each, and four cohorts per site (two +2
/// generators, two -1 upkeeps), with and without a root injection.
fn owner_economy(root_flow: f32) -> Shape {
    let mut registry = DimensionRegistry::new();
    let pid = property(&mut registry);
    let mut root = SimThing::new(SimThingKind::World, 0);
    author(&registry, pid, &mut root, root_flow, 0.0);
    let mut participants = vec![Participant {
        id: root.id,
        flow: root_flow,
        is_leaf: false,
    }];
    for multiplier in [1.0f32, 1.5] {
        let mut owner = SimThing::new(SimThingKind::Cohort, 0);
        author(&registry, pid, &mut owner, 0.0, 1.0);
        owner.overlays.push(weight_policy(
            pid,
            owner.id,
            TransformOp::multiply(multiplier),
        ));
        participants.push(Participant {
            id: owner.id,
            flow: 0.0,
            is_leaf: false,
        });
        let mut site = SimThing::new(SimThingKind::Cohort, 0);
        author(&registry, pid, &mut site, 0.0, 1.0);
        participants.push(Participant {
            id: site.id,
            flow: 0.0,
            is_leaf: false,
        });
        for flow in [2.0f32, 2.0, -1.0, -1.0] {
            let mut cohort = SimThing::new(SimThingKind::Cohort, 0);
            author(&registry, pid, &mut cohort, flow, 1.0);
            participants.push(Participant {
                id: cohort.id,
                flow,
                is_leaf: true,
            });
            site.add_child(cohort);
        }
        owner.add_child(site);
        root.add_child(owner);
    }
    Shape {
        label: format!("owner economy, root flow {root_flow}"),
        registry,
        pid,
        root,
        participants,
        depth: 3,
        policies: 2,
    }
}

/// Runs the ORDINARY resident session and returns every conservation violation.
fn judge(shape: Shape) -> Vec<String> {
    let Shape {
        label,
        registry,
        pid,
        root,
        participants,
        ..
    } = shape;
    let mut scenario = Scenario {
        name: "rf-conservation-any-shape".into(),
        registry,
        root,
        n_slots: 64,
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
            .expect("the stock property derives one arena");
    // A declared arena cap, sized for the deepest generated tree (depth 5).
    for arena in &mut rf.arenas {
        arena.max_orderband_depth = 64;
    }
    let spec = GameModeSpec {
        id: "rf-conservation-any-shape".into(),
        resource_flow: Some(rf),
        ..Default::default()
    };
    let mut session = SimSession::open_from_spec(scenario, &spec)
        .unwrap_or_else(|error| panic!("{label}: qualified ordinary resident ingress: {error:?}"));

    let registry = &session.proto.registry;
    let layout = &registry.property(pid).layout;
    let columns = ["balance", "allocated"].map(|name| {
        registry
            .column_range(pid)
            .col_for_role(&role(name), layout)
            .unwrap()
            .raw()
    });
    let total = registry.total_columns;
    let slots: Vec<usize> = participants
        .iter()
        .map(|participant| {
            session
                .proto
                .allocator
                .slot_of(participant.id)
                .unwrap()
                .as_usize()
        })
        .collect();
    let read = |values: &[f32], column: usize| -> Vec<f32> {
        slots
            .iter()
            .map(|slot| values[slot * total + column])
            .collect()
    };

    let mut failures = Vec::new();
    let mut before = read(&session.state.read_values(), columns[0]);
    for generation in 1..=GENERATIONS {
        session
            .step_once()
            .unwrap_or_else(|error| panic!("{label}: generation {generation}: {error:?}"));
        let values = session.state.read_values();
        let after = read(&values, columns[0]);
        let allocated = read(&values, columns[1]);
        let snapshot = ArenaConservationSnapshot {
            participants: participants
                .iter()
                .enumerate()
                .map(|(index, participant)| ArenaMemberObservation {
                    id: index as u64,
                    is_leaf: participant.is_leaf,
                    balance_governed: true,
                    intrinsic_flow: participant.flow,
                    allocated_flow: allocated[index],
                    balance_delta: Some(after[index] - before[index]),
                })
                .collect(),
            structural_evidence: ArenaStructuralEvidence {
                declared_intrinsic_source_ids: vec![0],
                inbound_coupling_endpoint_ids: vec![],
                parent_disbursement_recipient_ids: (1..participants.len() as u64).collect(),
            },
            inbound_coupling: 0.0,
            emission_consumption: 0.0,
        };
        if let Err(violation) = check_arena_structural(&snapshot) {
            failures.push(format!("{label}: generation {generation}: {violation:?}"));
        }
        // MUTANT: a quarter unit of mass lost from one participant must fail the
        // same check, so the bound is not vacuous and the observation is live.
        let mut mutant = snapshot.clone();
        let last = mutant.participants.len() - 1;
        mutant.participants[last].balance_delta = Some(after[last] - before[last] - 0.25);
        if check_arena_structural(&mutant).is_ok() {
            failures.push(format!(
                "{label}: generation {generation}: a lost 0.25 went undetected"
            ));
        }
        before = after;
    }
    failures
}

#[test]
fn conservation_holds_for_every_generated_tree_shape() {
    let mut failures = Vec::new();
    let mut depths = [0usize; 6];
    let (mut interiors, mut sinks, mut policies) = (0, 0, 0);
    for seed in 0..SEEDS {
        let shape = generated(seed);
        depths[shape.depth] += 1;
        interiors += shape
            .participants
            .iter()
            .skip(1)
            .filter(|participant| !participant.is_leaf && participant.flow != 0.0)
            .count();
        sinks += shape
            .participants
            .iter()
            .filter(|participant| participant.flow < 0.0)
            .count();
        policies += shape.policies;
        failures.extend(judge(shape));
    }
    println!(
        "coverage: {SEEDS} trees, depth histogram {:?}, {interiors} interiors owning intrinsic flow, {sinks} sinks, {policies} interior weight policies, {GENERATIONS} generations each",
        &depths[1..]
    );
    assert!(
        interiors > 0 && sinks > 0 && policies > 0,
        "the generator must cover interior flow, sinks and policies"
    );
    assert!(
        failures.is_empty(),
        "conservation violated:\n{}",
        failures.join("\n")
    );
}

#[test]
fn conservation_holds_for_the_shipped_owner_economy_shape() {
    let mut failures = Vec::new();
    for root_flow in [0.0f32, 3.0] {
        failures.extend(judge(owner_economy(root_flow)));
    }
    assert!(
        failures.is_empty(),
        "conservation violated:\n{}",
        failures.join("\n")
    );
}
