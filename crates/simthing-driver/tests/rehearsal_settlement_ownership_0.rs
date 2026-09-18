//! Settlement ownership law: a NESTED balance-governed arena settles every
//! intrinsic unit exactly once through the ordinary session, judged by RF-1.
//!
//! Disbursement pools a participant's intrinsic into its parent's sum exactly
//! when that parent sits below the root; the root disburses only its own
//! intrinsic. Settlement must be the exact dual: a participant keeps its own
//! intrinsic only when its parent does not pool it, and an interior below the
//! root holds the pool it disburses.
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    PropertyValue, SimPropertyId, SimThing, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, check_conservation, derive_resource_flow_admission,
    ArenaConservationSnapshot, ArenaMemberObservation, ArenaStructuralEvidence, Scenario,
    SimSession,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{compile_property, GameModeSpec, PropertySpec};

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.into())
}

fn governed_property(registry: &mut DimensionRegistry) -> SimPropertyId {
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
            id: "energy".into(),
            namespace: "ownership".into(),
            name: "energy".into(),
            display_name: "energy".into(),
            description: String::new(),
            sub_fields: vec![
                field("flow", Some(AccumulatorRole::IntrinsicFlow)),
                field(
                    "allocated",
                    Some(AccumulatorRole::AllocatedFlow {
                        arena: "energy".into(),
                    }),
                ),
                field(
                    "weight",
                    Some(AccumulatorRole::AllocatorWeight {
                        arena: "energy".into(),
                    }),
                ),
                field("balance_rate", None),
                balance,
            ],
        },
        registry,
    )
    .expect("ordinary governed property admission")
    .0
}

/// Authored flows for root -> {owner -> {site -> {c1, c2, c3}, keep}, lone}:
/// depth 0 root, depth 1 owner (interior) and lone (leaf), depth 2 site
/// (interior) and keep (leaf), depth 3 cohorts c1..c3 (leaves).
#[derive(Clone, Copy, Debug)]
struct Flows {
    root: f32,
    owner: f32,
    lone: f32,
    site: f32,
    keep: f32,
    cohorts: [f32; 3],
}

struct Settled {
    report: simthing_driver::ConservationReport,
    /// (name, depth, is_leaf, intrinsic, allocated, balance_delta) per participant.
    rows: Vec<(String, u32, bool, f32, f32, f32)>,
}

fn settle_one_generation(flows: Flows) -> Settled {
    let mut registry = DimensionRegistry::new();
    let pid = governed_property(&mut registry);
    let layout_of = |registry: &DimensionRegistry| registry.property(pid).layout.clone();
    let node = |kind: SimThingKind, flow: f32, registry: &DimensionRegistry| {
        let mut thing = SimThing::new(kind, 0);
        let layout = layout_of(registry);
        let mut value = PropertyValue::from_layout(&layout);
        value.set_role(&role("flow"), &layout, flow);
        value.set_role(&role("weight"), &layout, 1.0);
        thing.add_property(pid, value);
        thing
    };
    let mut root = node(SimThingKind::World, flows.root, &registry);
    let mut owner = node(SimThingKind::Cohort, flows.owner, &registry);
    let lone = node(SimThingKind::Cohort, flows.lone, &registry);
    let mut site = node(SimThingKind::Cohort, flows.site, &registry);
    let keep = node(SimThingKind::Cohort, flows.keep, &registry);
    let cohorts = flows
        .cohorts
        .map(|flow| node(SimThingKind::Cohort, flow, &registry));
    let mut names = std::collections::HashMap::new();
    names.insert(root.id, "root");
    names.insert(owner.id, "owner");
    names.insert(lone.id, "lone");
    names.insert(site.id, "site");
    names.insert(keep.id, "keep");
    for (cohort, name) in cohorts.iter().zip(["c1", "c2", "c3"]) {
        names.insert(cohort.id, name);
    }
    for cohort in cohorts {
        site.add_child(cohort);
    }
    owner.add_child(site);
    owner.add_child(keep);
    root.add_child(owner);
    root.add_child(lone);
    simthing_driver::resident_clearing_runtime::install_default_resident_rf_property(
        &mut registry,
        &mut root,
    );
    let mut allocator = SlotAllocator::new();
    allocator.install_initial_tree(&root).unwrap();
    let mut rf = derive_resource_flow_admission(None, &registry, &root, &allocator)
        .expect("ordinary derived nested RF participation")
        .spec
        .unwrap();
    for arena in &mut rf.arenas {
        arena.max_orderband_depth = 16;
    }
    let scenario = Scenario {
        name: "settlement-ownership".into(),
        registry,
        root,
        n_slots: 16,
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        shadow_seeds: vec![],
        tick_patches: vec![],
        install_targets: Default::default(),
    };
    let spec = GameModeSpec {
        id: "settlement-ownership".into(),
        resource_flow: Some(rf),
        ..Default::default()
    };
    let mut session = SimSession::open_from_spec(scenario, &spec)
        .expect("ordinary nested governed arena admission");
    let layout = build_execution_plan(&session.proto.registry, &session.spec_state.arena_registry)
        .expect("execution plan")
        .arenas
        .into_iter()
        .find(|arena| arena.flow_property_id == pid)
        .expect("nested energy arena");
    assert_eq!(layout.max_depth, 4, "root, owner/lone, site/keep, cohorts");

    let n_dims = session.proto.registry.total_columns as u32;
    let at = |values: &[f32], slot: simthing_core::SlotIndex, col: simthing_core::ColumnIndex| {
        values[(slot.raw() * n_dims + col.raw_u32()) as usize]
    };
    let before = session.state.read_values();
    session.step_once().expect("ordinary nested settlement generation");
    let after = session.state.read_values();

    let mut rows = Vec::new();
    let mut participants = Vec::new();
    let mut recipients = Vec::new();
    let root_id = layout.participant_roots[0].hosted_simthing_id;
    for node in layout.iter_all() {
        let slot = node.participant_slot;
        let balance = node.cols.balance_col.expect("governed Balance column");
        let intrinsic = at(&before, slot, node.cols.intrinsic_flow_col);
        let allocated = at(&after, slot, node.cols.allocated_flow_col);
        let delta = at(&after, slot, balance) - at(&before, slot, balance);
        let id = node.hosted_simthing_id.raw() as u64;
        if node.hosted_simthing_id != root_id {
            recipients.push(id);
        }
        rows.push((
            names[&node.hosted_simthing_id].to_string(),
            node.depth,
            !node.is_interior(),
            intrinsic,
            allocated,
            delta,
        ));
        participants.push(ArenaMemberObservation {
            id,
            is_leaf: !node.is_interior(),
            balance_governed: node.cols.balance_governing_col.is_some(),
            intrinsic_flow: intrinsic,
            allocated_flow: allocated,
            balance_delta: Some(delta),
        });
    }
    let report = check_conservation(
        &[],
        &[],
        &[ArenaConservationSnapshot {
            participants,
            structural_evidence: ArenaStructuralEvidence {
                declared_intrinsic_source_ids: vec![root_id.raw() as u64],
                inbound_coupling_endpoint_ids: vec![],
                parent_disbursement_recipient_ids: recipients,
            },
            inbound_coupling: 0.0,
            emission_consumption: 0.0,
        }],
    );
    Settled { report, rows }
}

fn row<'a>(settled: &'a Settled, name: &str) -> &'a (String, u32, bool, f32, f32, f32) {
    settled
        .rows
        .iter()
        .find(|row| row.0 == name)
        .unwrap_or_else(|| panic!("missing participant {name}"))
}

/// catches: a participant whose intrinsic its parent already pooled and
/// redistributed also seeding that intrinsic into its own settlement (#2065
/// leaf law applied at every depth — mass created at depth >= 2), and an
/// interior below the root losing its own intrinsic that nobody disburses
/// (mass destroyed), judged by RF-1 over the ordinary nested session.
#[test]
fn nested_governed_arena_settles_every_intrinsic_unit_exactly_once() {
    let cases = [
        // Deep leaves carry intrinsic their parents pool and redistribute.
        (
            "deep-leaf-intrinsic",
            Flows {
                root: 1.0,
                owner: 0.0,
                lone: 0.0,
                site: 0.0,
                keep: 1.5,
                cohorts: [2.0, 2.0, -1.0],
            },
        ),
        // An interior directly below the root owns intrinsic nobody disburses.
        (
            "depth-one-interior-intrinsic",
            Flows {
                root: 0.0,
                owner: 3.0,
                lone: 0.0,
                site: 0.0,
                keep: 0.0,
                cohorts: [0.0; 3],
            },
        ),
        // Every level carries intrinsic at once.
        (
            "every-level",
            Flows {
                root: 1.0,
                owner: 3.0,
                lone: 0.5,
                site: 0.25,
                keep: 1.5,
                cohorts: [2.0, 2.0, -1.0],
            },
        ),
    ];
    let mut failures = Vec::new();
    for (label, flows) in cases {
        let settled = settle_one_generation(flows);
        for row in &settled.rows {
            println!(
                "{label}: {} depth={} leaf={} intrinsic={} allocated={} balance_delta={}",
                row.0, row.1, row.2, row.3, row.4, row.5
            );
        }
        if !settled.report.all_pass() {
            failures.push(format!("{label}: {:?}", settled.report.structural_errors));
            continue;
        }
        // Exact per-participant ownership, not only the aggregate.
        for name in ["c1", "c2", "c3", "keep"] {
            let (_, depth, _, _, allocated, delta) = row(&settled, name);
            assert!(*depth >= 2);
            assert_eq!(
                delta.to_bits(),
                allocated.to_bits(),
                "{label}: pooled leaf {name} settles exactly its allocation"
            );
        }
        let (_, _, _, lone_intrinsic, lone_allocated, lone_delta) = row(&settled, "lone");
        assert_eq!(
            lone_delta.to_bits(),
            (lone_intrinsic + lone_allocated).to_bits(),
            "{label}: an unpooled leaf keeps its own intrinsic plus its allocation"
        );
        println!("{label}: RF-1 PASS");
    }
    assert!(
        failures.is_empty(),
        "nested settlement must conserve: {failures:#?}"
    );
}
