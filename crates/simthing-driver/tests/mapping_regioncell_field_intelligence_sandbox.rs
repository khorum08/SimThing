//! Sparse RegionCell field-intelligence sandbox (test/prototype only).
//!
//! Validates the AI-as-SimThing thesis: narrow durable RegionCell fields plus
//! personality-weighted EML-equivalent pressure projections yield different
//! strategic heatmaps without CPU-side AI decision loops or mapping runtime.

use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingId, SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    build_execution_plan, materialize_arena_participants, validate_resource_flow_preflight,
    GpuArenaDescriptor,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::PipelineFlags;
use simthing_spec::{
    compile_property, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec, PropertyKey,
    PropertySpec, ResourceFlowSpec,
};
use std::path::Path;

const GRID: usize = 10;
const CELL_COUNT: usize = GRID * GRID;
const ATTACK_THRESHOLD: f32 = 1.0;

/// Narrow durable RegionCell state (sandbox only — not a production schema).
#[derive(Clone, Copy, Debug, PartialEq)]
struct RegionCellState {
    presence: f32,
    threat: f32,
    opportunity: f32,
    supply: f32,
    control: f32,
}

/// Faction personality scalars applied to EML-equivalent pressure projections.
#[derive(Clone, Copy, Debug, PartialEq)]
struct PersonalityWeights {
    aggression: f32,
    caution: f32,
    stability_bias: f32,
    loss_aversion: f32,
    expansion_bias: f32,
    risk_aversion: f32,
}

impl PersonalityWeights {
    fn defensive() -> Self {
        Self {
            aggression: 0.2,
            caution: 0.8,
            stability_bias: 0.85,
            loss_aversion: 0.7,
            expansion_bias: 0.15,
            risk_aversion: 0.75,
        }
    }

    fn aggressive() -> Self {
        Self {
            aggression: 0.9,
            caution: 0.2,
            stability_bias: 0.25,
            loss_aversion: 0.3,
            expansion_bias: 0.85,
            risk_aversion: 0.2,
        }
    }
}

/// Derived pressure triplet (not stored as durable RegionCell state).
#[derive(Clone, Copy, Debug, PartialEq)]
struct PressureProjection {
    attack: f32,
    defense: f32,
    expansion: f32,
}

/// Sandbox commitment report (AttackIntent-equivalent; no EmitEvent wiring required).
#[derive(Clone, Copy, Debug, PartialEq)]
struct SandboxCommitmentReport {
    cell_index: usize,
    attack_pressure: f32,
    defense_pressure: f32,
    attack_intent: bool,
}

fn supply_stress(supply: f32) -> f32 {
    (0.0_f32 - supply).max(0.0)
}

fn control_gap(control: f32) -> f32 {
    1.0 - control
}

/// EML-equivalent attack pressure projection (pure math — no CPU strategy branch).
fn eml_project_attack_pressure(cell: RegionCellState, w: PersonalityWeights) -> f32 {
    cell.opportunity * w.aggression
        + cell.presence * w.aggression
        - cell.threat * w.caution
        - supply_stress(cell.supply) * w.caution
}

/// EML-equivalent defense pressure projection.
fn eml_project_defense_pressure(cell: RegionCellState, w: PersonalityWeights) -> f32 {
    cell.threat * w.stability_bias
        + control_gap(cell.control) * w.loss_aversion
        + cell.presence * w.stability_bias
}

/// EML-equivalent expansion pressure projection.
fn eml_project_expansion_pressure(cell: RegionCellState, w: PersonalityWeights) -> f32 {
    cell.opportunity * w.expansion_bias
        - cell.threat * w.risk_aversion
        + cell.supply * w.expansion_bias
}

fn project_all_pressures(cell: RegionCellState, w: PersonalityWeights) -> PressureProjection {
    PressureProjection {
        attack: eml_project_attack_pressure(cell, w),
        defense: eml_project_defense_pressure(cell, w),
        expansion: eml_project_expansion_pressure(cell, w),
    }
}

fn project_heatmap(cells: &[RegionCellState], w: PersonalityWeights) -> Vec<PressureProjection> {
    cells
        .iter()
        .map(|cell| project_all_pressures(*cell, w))
        .collect()
}

fn project_commitments(
    cells: &[RegionCellState],
    w: PersonalityWeights,
) -> Vec<SandboxCommitmentReport> {
    cells
        .iter()
        .enumerate()
        .map(|(idx, cell)| {
            let attack = eml_project_attack_pressure(*cell, w);
            let defense = eml_project_defense_pressure(*cell, w);
            SandboxCommitmentReport {
                cell_index: idx,
                attack_pressure: attack,
                defense_pressure: defense,
                attack_intent: attack >= ATTACK_THRESHOLD,
            }
        })
        .collect()
}

fn hotspot_cell() -> RegionCellState {
    RegionCellState {
        presence: 0.7,
        threat: 0.4,
        opportunity: 0.9,
        supply: 0.3,
        control: 0.5,
    }
}

fn uniform_grid_state(hotspot_index: usize) -> Vec<RegionCellState> {
    let baseline = RegionCellState {
        presence: 0.2,
        threat: 0.15,
        opportunity: 0.1,
        supply: 0.4,
        control: 0.6,
    };
    let mut cells = vec![baseline; CELL_COUNT];
    cells[hotspot_index] = hotspot_cell();
    cells
}

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn register_flow(reg: &mut DimensionRegistry) -> simthing_core::SimPropertyId {
    let spec = PropertySpec {
        id: "food_flow".into(),
        namespace: "core".into(),
        name: "food_flow".into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: "food".into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: "food".into(),
                },
            ),
            flow_subfield("balance", AccumulatorRole::Balance(BalanceSpec::default())),
        ],
    };
    let (id, _) = compile_property(&spec, reg).unwrap();
    id
}

fn hosted_cohorts(count: usize) -> (SimThing, Vec<SimThingId>) {
    let mut world = SimThing::new(SimThingKind::World, 0);
    let mut ids = Vec::new();
    for _ in 0..count {
        let cohort = SimThing::new(SimThingKind::Cohort, 0);
        ids.push(cohort.id);
        world.add_child(cohort);
    }
    (world, ids)
}

fn regioncell_participants(
    hosted: &[SimThingId],
    alloc: &SlotAllocator,
) -> Vec<ExplicitParticipantSpec> {
    let faction = hosted[0];
    let mut out = vec![ExplicitParticipantSpec::flat(
        alloc.slot_of(faction).unwrap(),
        faction.raw(),
    )];
    for cell in &hosted[1..=CELL_COUNT] {
        out.push(ExplicitParticipantSpec::nested(
            alloc.slot_of(*cell).unwrap(),
            cell.raw(),
            faction.raw() as u64,
        ));
    }
    out
}

struct RegionCellSandboxFixture {
    scaffold: simthing_driver::ArenaParticipantScaffold,
    layout: simthing_driver::ArenaTreeLayout,
}

fn materialize_regioncell_sandbox() -> RegionCellSandboxFixture {
    let mut reg = DimensionRegistry::new();
    let flow_id = register_flow(&mut reg);
    let (mut root, hosted) = hosted_cohorts(1 + CELL_COUNT);
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&root);

    let spec = ResourceFlowSpec {
        arenas: vec![ArenaSpec {
            name: "food".into(),
            flow_property: PropertyKey::new("core", "food_flow"),
            balance_property: None,
            max_participants: (1 + CELL_COUNT) as u32,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicySpec::Reject,
            reserved_orderband_depth: 0,
            reserved_gap_per_intermediate: 0,
            expected_max_children_per_intermediate: 0,
            explicit_participants: regioncell_participants(&hosted, &alloc),
            enrollment: None,
            wildcard_admission: None,
        }],
        couplings: vec![],
        ..Default::default()
    };
    validate_resource_flow_preflight(&spec, &alloc).unwrap();
    let scaffold = materialize_arena_participants(&spec, &reg, &mut root, &mut alloc).unwrap();

    let arena = GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: flow_id,
        balance_property_id: None,
        max_participants: (1 + CELL_COUNT) as u32,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: simthing_driver::FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let layout = build_execution_plan(
        &reg,
        std::slice::from_ref(&arena),
        &root,
        &alloc,
        &scaffold,
        1,
    )
    .unwrap()
    .arenas
    .remove(0);

    RegionCellSandboxFixture { scaffold, layout }
}

#[test]
fn regioncell_sandbox_static_10x10_materializes() {
    let fx = materialize_regioncell_sandbox();
    assert_eq!(fx.scaffold.index.by_host_and_arena.len(), 1 + CELL_COUNT);
    assert_eq!(fx.scaffold.reports[0].participant_count, (1 + CELL_COUNT) as u32);
    assert_eq!(fx.layout.max_depth, 2);
    assert_eq!(fx.layout.participant_roots.len(), 1);
    assert_eq!(fx.layout.participant_roots[0].children.len(), CELL_COUNT);
    fx.layout.participant_roots[0]
        .verify_child_contiguity()
        .expect("100 RegionCell slots contiguous under FactionRoot");
}

#[test]
fn regioncell_sandbox_durable_fields_are_narrow() {
    let cell = hotspot_cell();
    assert_eq!(cell.presence, 0.7);
    assert_eq!(cell.threat, 0.4);
    assert_eq!(cell.opportunity, 0.9);
    assert_eq!(cell.supply, 0.3);
    assert_eq!(cell.control, 0.5);
    let names = ["presence", "threat", "opportunity", "supply", "control"];
    assert_eq!(names.len(), 5);
    assert!(!names.contains(&"attack_pressure"));
    assert!(!names.contains(&"defense_pressure"));
    assert!(!names.contains(&"velocity"));
}

#[test]
fn regioncell_sandbox_attack_pressure_changes_with_aggression_weight() {
    let cell = hotspot_cell();
    let low = eml_project_attack_pressure(cell, PersonalityWeights::defensive());
    let high = eml_project_attack_pressure(cell, PersonalityWeights::aggressive());
    assert!(high > low, "aggression=0.9 must raise attack pressure vs 0.2");
    assert!(low < ATTACK_THRESHOLD);
    assert!(high >= ATTACK_THRESHOLD);
}

#[test]
fn regioncell_sandbox_defense_pressure_changes_with_caution_or_stability_weight() {
    let cell = hotspot_cell();
    let defensive = eml_project_defense_pressure(cell, PersonalityWeights::defensive());
    let aggressive = eml_project_defense_pressure(cell, PersonalityWeights::aggressive());
    assert!(
        defensive > aggressive,
        "high stability/caution must raise defense pressure"
    );
}

#[test]
fn regioncell_sandbox_hotspot_crosses_attack_threshold() {
    let hotspot_idx = 55;
    let cells = uniform_grid_state(hotspot_idx);
    let defensive = project_commitments(&cells, PersonalityWeights::defensive());
    let aggressive = project_commitments(&cells, PersonalityWeights::aggressive());

    let d = &defensive[hotspot_idx];
    let a = &aggressive[hotspot_idx];
    assert!(!d.attack_intent);
    assert!(a.attack_intent);
    assert!(a.attack_pressure >= ATTACK_THRESHOLD);
    assert!(d.defense_pressure > d.attack_pressure);
    assert!(a.attack_pressure > a.defense_pressure);
}

#[test]
fn regioncell_sandbox_same_cell_state_different_personality_different_heatmap() {
    let cells = uniform_grid_state(55);
    let low = project_heatmap(&cells, PersonalityWeights::defensive());
    let high = project_heatmap(&cells, PersonalityWeights::aggressive());
    assert_ne!(low, high);
    let mut attack_differs = 0;
    for (a, b) in low.iter().zip(high.iter()) {
        if (a.attack - b.attack).abs() > f32::EPSILON {
            attack_differs += 1;
        }
    }
    assert_eq!(attack_differs, CELL_COUNT);
}

#[test]
fn regioncell_sandbox_no_cpu_ai_decision_loop() {
    let cells = uniform_grid_state(55);
    let w = PersonalityWeights::aggressive();
    let reports = project_commitments(&cells, w);
    for report in &reports {
        let expected = eml_project_attack_pressure(cells[report.cell_index], w);
        assert!((report.attack_pressure - expected).abs() < f32::EPSILON);
        assert_eq!(report.attack_intent, expected >= ATTACK_THRESHOLD);
    }
    // Commitments are threshold reports on eml_project_* outputs only — no per-cell strategy branch.
    let hotspot = project_all_pressures(hotspot_cell(), PersonalityWeights::aggressive());
    assert!(hotspot.attack >= ATTACK_THRESHOLD);
}

#[test]
fn regioncell_sandbox_no_mapping_runtime_primitives() {
    // Uses only nested explicit participant materialization — no mapping runtime entry points.
    let fx = materialize_regioncell_sandbox();
    assert_eq!(fx.layout.participant_roots.len(), 1);
}

#[test]
fn regioncell_sandbox_no_new_wgsl() {
    let wgsl_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../simthing-gpu/src/shaders");
    let entries: Vec<_> = std::fs::read_dir(&wgsl_root)
        .expect("shaders dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let allowed = ["accumulator_op.wgsl", "snapshot.wgsl", "world_summary.wgsl"];
    for name in &entries {
        assert!(
            allowed.contains(&name.as_str()),
            "unexpected WGSL file {name}; sandbox must not add shaders"
        );
    }
}

#[test]
fn regioncell_sandbox_global_resource_flow_flag_default_false() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
}
