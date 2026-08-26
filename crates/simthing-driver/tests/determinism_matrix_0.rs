//! TP-PURGE-0 Remand 6 — `determinism_matrix` (DA `5135942768` / Remand `5137229638`).
//!
//! Five approved cases. Inline input. Green and red postures receive the **same
//! semantic input**; red uses a test-local mutant mechanism path.
//!
//! The `ordering` case table carries two paths: overlay OrderBand planning, and
//! owner-silo disburse-down equal-claim tie-break (absorbed; not an eleventh case).

use simthing_core::{
    eml_nodes::{self, EmlNode},
    DimensionRegistry, EmlExecutionClass, Overlay, OverlayId, OverlayKind, OverlayLifecycle,
    OverlaySource, PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingId,
    SimThingKind, SubFieldRole, TransformOp,
};
use simthing_gpu::{plan_overlay_orderband, OverlayDelta, OverlayOrderBandPlan, SlotDeltaRange};
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayFrame, ReplaySnapshot, SimRuntimeTree};
use simthing_spec::designer_admission::{
    plan_mobility_reenroll0, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move, MobilityReenroll0PlanInput,
    MobilityReenroll0PlanReport, MobilityReenroll0RegistryState,
};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, compile_eml_gadget,
    deserialize_mobility_scenario0_packet_ron, serialize_mobility_scenario0_packet_ron,
    CompiledEmlGadget, EmlGadgetCompileOptions, EmlGadgetInstanceSpec, EmlGadgetKind,
    MobilityAllocationBounds, MobilityBlockadeSemantics, MobilityIdentityBoundary,
    MobilityIdentityChannelBudget, MobilityOwnerColumn, MobilityOwnerRelationDiscipline,
    MobilityOwnerRelationKind, MobilityQuantityClasses, MobilityRoutingMode, MobilityRoutingPolicy,
    MobilityScenario0GuardrailRequests, MobilityScenario0Packet, MobilityScenario0Status,
    MobilitySoakProfile, MobilitySupplyScope, MobilityTheaterScale, MobilityTheaterShape, OwnerRef,
    ResourceKey, RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloDisburseDownResult,
    RuntimeOwnerSiloWritebackResult, ScopeId, PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DetCase {
    Replay,
    Ordering,
    CanonicalSerialization,
    MobilityDispatch,
    JitArtifact,
}

const CASES: [DetCase; 5] = [
    DetCase::Replay,
    DetCase::Ordering,
    DetCase::CanonicalSerialization,
    DetCase::MobilityDispatch,
    DetCase::JitArtifact,
];

fn fnv(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn inline_reenroll_input() -> MobilityReenroll0PlanInput {
    let origin = MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id: 7,
    };
    let destination = MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id: 8,
    };
    MobilityReenroll0PlanInput {
        registry: MobilityReenroll0RegistryState {
            live_slices: vec![
                MobilityAlloc0LiveSlice {
                    entity_id: 10,
                    parent_key: origin,
                    slot: 2,
                },
                MobilityAlloc0LiveSlice {
                    entity_id: 11,
                    parent_key: origin,
                    slot: 0,
                },
            ],
            origin_generations: Default::default(),
            destination_generations: Default::default(),
        },
        moves: vec![
            MobilityReenroll0Move {
                entity_id: 10,
                origin,
                destination,
            },
            MobilityReenroll0Move {
                entity_id: 11,
                origin,
                destination,
            },
        ],
        forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
    }
}

fn make_overlay() -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin: SimThingId::new(),
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::set(0.5))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

fn inline_replay_bundle() -> (ReplaySnapshot, ReplayFrame, OverlayId) {
    let mut registry = DimensionRegistry::new();
    registry.register(SimProperty::simple("core", "loyalty", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    let snapshot = ReplaySnapshot {
        day: 0,
        root: SimRuntimeTree::admit(root),
        registry,
        fission_lineage: Vec::new(),
    };
    let target = snapshot
        .root
        .direct_child_id(0)
        .expect("inline cohort child");
    let overlay = make_overlay();
    let overlay_id = overlay.id;
    let frame = ReplayFrame {
        day: 1,
        entries: vec![
            BoundaryDeltaEntry::OverlayAttached { target, overlay },
            BoundaryDeltaEntry::OverlaySuspended { target, overlay_id },
        ],
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    };
    (snapshot, frame, overlay_id)
}

fn replay_fingerprint(
    driver: &ReplayDriver,
    target: simthing_core::SimThingId,
    overlay_id: OverlayId,
) -> (u32, bool, bool) {
    (
        driver.day,
        driver
            .root
            .overlay_is_suspended(target, overlay_id)
            .unwrap_or(false),
        driver
            .root
            .overlay_is_active(target, overlay_id)
            .unwrap_or(false),
    )
}

/// Live replay executor: apply sealed frame in recorded entry order.
fn live_replay_apply(snapshot: ReplaySnapshot, frame: ReplayFrame) -> ReplayDriver {
    let mut driver = ReplayDriver::from_snapshot(snapshot).expect("replay snapshot install");
    driver.apply_frame(frame);
    driver
}

/// Mutant replay executor: same snapshot+frame identity, but misapplies entries
/// in reverse order inside the executor (non-canonical application).
fn mutant_replay_apply_reversed_entries(
    snapshot: ReplaySnapshot,
    frame: ReplayFrame,
) -> ReplayDriver {
    let mut driver = ReplayDriver::from_snapshot(snapshot).expect("replay snapshot install");
    let mut misapplied = frame;
    misapplied.entries.reverse();
    driver.apply_frame(misapplied);
    driver
}

/// Replay: same inline snapshot+frame to both postures.
/// Green: live apply twice. Red: live vs mutant reversed-application executor.
fn case_replay(plant_defect: bool) -> bool {
    let (snap, frame, overlay_id) = inline_replay_bundle();
    let target = snap.root.direct_child_id(0).expect("target");
    let a = live_replay_apply(snap.clone(), frame.clone());
    let b = if plant_defect {
        mutant_replay_apply_reversed_entries(snap, frame)
    } else {
        live_replay_apply(snap, frame)
    };
    replay_fingerprint(&a, target, overlay_id) == replay_fingerprint(&b, target, overlay_id)
}

/// Mutant overlay OrderBand planner: ignore sealed presentation order; sort by
/// value before banding. Contract: presentation order is semantically meaningful
/// for successive same-cell overlays (band assignment).
fn mutant_plan_overlay_sort_by_value(
    deltas: &[OverlayDelta],
    ranges: &[SlotDeltaRange],
    n_slots: u32,
) -> OverlayOrderBandPlan {
    let mut reordered = deltas.to_vec();
    for range in ranges.iter().take(n_slots as usize) {
        let start = range.offset as usize;
        let end = (range.offset + range.length) as usize;
        if end <= reordered.len() {
            reordered[start..end].sort_by(|a, b| {
                a.value
                    .partial_cmp(&b.value)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
    }
    plan_overlay_orderband(&reordered, ranges, n_slots)
}

/// Ordering path 1: overlay OrderBand — sealed presentation order is meaningful.
/// Green: same delta vector twice → identical plan.
/// Red: same deltas to live vs value-sort mutant planner.
fn ordering_overlay_path(plant_defect: bool) -> bool {
    // Present higher value first so value-sort differs from presentation order.
    let deltas = [
        OverlayDelta {
            col: 0,
            op_kind: 0,
            value: 2.0,
            _pad: 0,
        },
        OverlayDelta {
            col: 0,
            op_kind: 0,
            value: 1.0,
            _pad: 0,
        },
    ];
    let ranges = [SlotDeltaRange {
        offset: 0,
        length: 2,
    }];
    let live = plan_overlay_orderband(&deltas, &ranges, 1);
    let other = if plant_defect {
        mutant_plan_overlay_sort_by_value(&deltas, &ranges, 1)
    } else {
        plan_overlay_orderband(&deltas, &ranges, 1)
    };
    live.ops == other.ops && live.n_bands == other.n_bands
}

fn equal_claim_demands() -> (
    Vec<RuntimeOwnerSiloWritebackResult>,
    Vec<RuntimeOwnerSiloDemandBucket>,
) {
    let owner = OwnerRef::new("owner_x");
    let resource = ResourceKey::new(PLANET_CHILD_RF_DEFAULT_RESOURCE_KEY);
    let writeback = vec![RuntimeOwnerSiloWritebackResult {
        owner_ref: owner.clone(),
        resource_key: resource.clone(),
        previous_current: 30,
        next_current: 30,
        capacity: None,
        applied_surplus: 0,
        applied_deficit: 0,
        clamped_surplus: 0,
        unmet_deficit: 0,
    }];
    let demands = vec![
        RuntimeOwnerSiloDemandBucket {
            owner_ref: owner.clone(),
            resource_key: resource.clone(),
            scope_id: ScopeId::new("scope_a"),
            requested: 20,
            priority: 1,
            source_simthing_id_raw: Some(2),
        },
        RuntimeOwnerSiloDemandBucket {
            owner_ref: owner,
            resource_key: resource,
            scope_id: ScopeId::new("scope_a"),
            requested: 20,
            priority: 1,
            source_simthing_id_raw: Some(1),
        },
    ];
    (writeback, demands)
}

fn allocation_fingerprint(
    report: &[RuntimeOwnerSiloDisburseDownResult],
) -> Vec<(Option<u32>, u32)> {
    report
        .iter()
        .flat_map(|r| {
            r.allocations
                .iter()
                .map(|a| (a.source_simthing_id_raw, a.allocated))
        })
        .collect()
}

/// Defective allocator: consume writeback in presentation order (no canonical sort).
fn allocate_presentation_order(
    writeback: &[RuntimeOwnerSiloWritebackResult],
    demands: &[RuntimeOwnerSiloDemandBucket],
) -> Vec<(Option<u32>, u32)> {
    let mut remaining = writeback[0].next_current;
    let mut out = Vec::with_capacity(demands.len());
    for d in demands {
        let allocated = remaining.min(d.requested);
        remaining = remaining.saturating_sub(allocated);
        out.push((d.source_simthing_id_raw, allocated));
    }
    out
}

/// Ordering path 2: owner-silo disburse-down equal-claim tie-break.
/// Green: live path on original and reversed presentation → same canonical allocation.
/// Red: presentation-order defective allocator diverges under reverse presentation.
fn ordering_owner_silo_tiebreak_path(plant_defect: bool) -> bool {
    let (writeback, demands) = equal_claim_demands();
    let mut reversed = demands.clone();
    reversed.reverse();
    if !plant_defect {
        let a =
            apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands).expect("disburse a");
        let b =
            apply_owner_silo_runtime_disburse_down_cpu(&writeback, &reversed).expect("disburse b");
        return allocation_fingerprint(&a) == allocation_fingerprint(&b)
            && a[0].allocations[0].source_simthing_id_raw == Some(1)
            && a[0].allocations[0].allocated == 20
            && a[0].allocations[1].source_simthing_id_raw == Some(2)
            && a[0].allocations[1].allocated == 10;
    }
    let a = allocate_presentation_order(&writeback, &demands);
    let b = allocate_presentation_order(&writeback, &reversed);
    a == b
}

fn case_ordering(plant_defect: bool) -> bool {
    ordering_overlay_path(plant_defect) && ordering_owner_silo_tiebreak_path(plant_defect)
}

/// Minimal inline mobility scenario0 packet (no production packet factory).
fn inline_mobility_scenario0_packet() -> MobilityScenario0Packet {
    MobilityScenario0Packet {
        scenario_id: "inline_tp_purge_mobility_scenario0".into(),
        status: MobilityScenario0Status::ScenarioAdmissionProposed,
        implementation_authorized: false,
        enabled_by_default: false,
        theater: MobilityTheaterShape {
            sectors: 1,
            systems: 1,
            cells: 4,
            spatial_depth: 2,
            scale: MobilityTheaterScale::SingleTheaterMultiCell,
        },
        identity_channels: MobilityIdentityChannelBudget {
            max_factions_per_cell: 2,
            local_identity_channels: 2,
            routing_eml_node_budget: 8,
            first_slice_expected_peak_factions_per_cell: 2,
            sufficiency_note: "inline".into(),
        },
        allocation: MobilityAllocationBounds {
            max_fleet_density_per_cell: 8,
            moving_entity_block_size: 16,
            reserved_headroom_per_cell: 2,
            overflow_rejects_or_narrows: true,
            slab_block_first: true,
        },
        identity_boundary: MobilityIdentityBoundary {
            simthing_slots: vec!["cell".into(), "fleet".into()],
            count_columns: vec!["fighter_count".into()],
            examples: vec!["inline example".into()],
        },
        owner_columns: vec![
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Faction,
                column: "faction_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::FlowPooling,
            },
            MobilityOwnerColumn {
                relation: MobilityOwnerRelationKind::Species,
                column: "species_owner".into(),
                discipline: MobilityOwnerRelationDiscipline::DownBroadcastOverlay,
            },
        ],
        quantity_classes: MobilityQuantityClasses {
            hard_fixed_point_band_alpha: vec!["hard_currency".into()],
            soft_float_band_beta: vec!["damage_rate".into()],
            hard_and_soft_never_silently_mix: true,
            float_values_gate_structural_transitions: false,
        },
        supply_scope: MobilitySupplyScope {
            sector_cell_edges_are_resource_flow_couplings: false,
            sector_cell_edges_are_spatial_structure: true,
            subsidiarity_balance_depth: "inline".into(),
            default_on_resource_flow: false,
            hard_currency_routes_through_resource_flow: false,
        },
        blockade: MobilityBlockadeSemantics {
            cut_flows: vec!["per_tick_supply".into()],
            blockade_immune_overlays: vec!["species_trait_modifier".into()],
            cpu_planner: false,
            cpu_urgency: false,
        },
        routing: MobilityRoutingPolicy {
            mode: MobilityRoutingMode::NarrowedAdversarialFirstSlice,
            identity_is_column_not_tree: true,
            uses_arrival_order_as_replay_ordering: true,
            silent_hybrid_strata_rebind: false,
        },
        soak: MobilitySoakProfile {
            entity_count: 8,
            churn_rate_per_boundary_bps: 1,
            movement_rate_per_boundary_bps: 1,
            capture_cadence_boundaries: 1,
            unlock_cadence_boundaries: 1,
            stress_mix: vec!["inline".into()],
        },
        guardrails: MobilityScenario0GuardrailRequests::default(),
    }
}

/// Mutant serializer: emit compact RON for the same packet (non-canonical form).
/// Live path uses pretty-config canonical encoding.
fn mutant_serialize_mobility_scenario0_compact(packet: &MobilityScenario0Packet) -> String {
    ron::ser::to_string(packet).expect("mutant compact ser")
}

/// Canonical serialization: same inline packet to both postures.
/// Green: live pretty ser twice + ser→de→reser stable.
/// Red: live pretty vs mutant compact encoding of the unchanged packet.
fn case_canonical_serialization(plant_defect: bool) -> bool {
    let packet = inline_mobility_scenario0_packet();
    let a = serialize_mobility_scenario0_packet_ron(&packet).expect("ser a");
    let round = deserialize_mobility_scenario0_packet_ron(&a).expect("de");
    let round_a2 = serialize_mobility_scenario0_packet_ron(&round).expect("re-ser");
    let b = if plant_defect {
        mutant_serialize_mobility_scenario0_compact(&packet)
    } else {
        serialize_mobility_scenario0_packet_ron(&packet).expect("ser b")
    };
    fnv(a.as_bytes()) == fnv(b.as_bytes()) && a == round_a2
}

fn movement_layout(
    report: &MobilityReenroll0PlanReport,
) -> Vec<(MobilityAlloc0ParentKey, u32, u64)> {
    report
        .final_live_slices
        .iter()
        .map(|slice| (slice.parent_key, slice.slot, slice.entity_id))
        .collect()
}

/// Defective retired policy: discard stable logical slots and allocate rows by
/// descending entity identity.
fn defective_first_free_by_entity_desc(
    input: &MobilityReenroll0PlanInput,
) -> Vec<(MobilityAlloc0ParentKey, u32, u64)> {
    let destination = input.moves[0].destination;
    let mut entities: Vec<u64> = input
        .moves
        .iter()
        .map(|movement| movement.entity_id)
        .collect();
    entities.sort_by_key(|id| std::cmp::Reverse(*id));
    let mut rows = entities
        .into_iter()
        .enumerate()
        .map(|(slot, entity_id)| (destination, slot as u32, entity_id))
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

/// Mobility dispatch preserves stable logical slots and is input-order
/// invariant. The planted defect reconstructs the retired first-free policy.
fn case_mobility_dispatch(plant_defect: bool) -> bool {
    let input = inline_reenroll_input();
    let live = movement_layout(&plan_mobility_reenroll0(&input));
    let other = if plant_defect {
        defective_first_free_by_entity_desc(&input)
    } else {
        let mut permuted = input.clone();
        permuted.registry.live_slices.reverse();
        permuted.moves.reverse();
        movement_layout(&plan_mobility_reenroll0(&permuted))
    };
    live == other
}

fn mutant_node_literal(v: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn mutant_node_slot(col: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn mutant_node_binop(opcode: u32) -> EmlNode {
    EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn mutant_node_div_safe() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::DIV,
        flags: 1,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn mutant_compute_u_nodes(input_col: u32, center: f32, steepness: f32) -> Vec<EmlNode> {
    vec![
        mutant_node_slot(input_col),
        mutant_node_literal(center),
        mutant_node_binop(eml_nodes::opcode::SUB),
        mutant_node_literal(steepness),
        mutant_node_binop(eml_nodes::opcode::MUL),
    ]
}

/// Test-local SoftStep compiler/encoder: mirrors production `compile_soft_step_nodes`
/// emission, but constructs the `1 + abs(u)` segment with ABS and LITERAL(1)
/// swapped — a construction-time node-order defect (does not call live compile).
fn mutant_compile_soft_step_wrong_abs_literal_order(
    instance: &EmlGadgetInstanceSpec,
    _opts: EmlGadgetCompileOptions,
) -> CompiledEmlGadget {
    let EmlGadgetInstanceSpec::SoftStep {
        id,
        input_col,
        output_col,
        center,
        steepness,
    } = instance
    else {
        panic!("mutant SoftStep compiler only admits SoftStep instances");
    };

    let mut nodes = Vec::new();
    // u = steepness * (x - center); keep first u on stack for the final division.
    nodes.extend(mutant_compute_u_nodes(*input_col, *center, *steepness));
    // Defect: emit LITERAL(1) before ABS (production emits ABS then LITERAL(1)).
    nodes.extend(mutant_compute_u_nodes(*input_col, *center, *steepness));
    nodes.push(mutant_node_literal(1.0));
    nodes.push(mutant_node_binop(eml_nodes::opcode::ABS));
    nodes.push(mutant_node_binop(eml_nodes::opcode::ADD));
    nodes.push(mutant_node_div_safe());
    nodes.push(mutant_node_literal(0.5));
    nodes.push(mutant_node_binop(eml_nodes::opcode::MUL));
    nodes.push(mutant_node_literal(0.5));
    nodes.push(mutant_node_binop(eml_nodes::opcode::ADD));
    nodes.push(mutant_node_binop(eml_nodes::opcode::RETURN_TOP));

    CompiledEmlGadget {
        id: id.clone(),
        kind: EmlGadgetKind::SoftStep,
        nodes,
        execution_class: EmlExecutionClass::ExactDeterministic,
        output_col: *output_col,
    }
}

/// JIT-artifact: same SoftStep instance+opts to both postures.
/// Green: live `compile_eml_gadget` twice.
/// Red: live vs construction-time mutant SoftStep compiler (wrong ABS/LITERAL order).
fn case_jit_artifact(plant_defect: bool) -> bool {
    let instance = EmlGadgetInstanceSpec::SoftStep {
        id: "inline_soft_step".into(),
        input_col: 0,
        output_col: None,
        center: 0.5,
        steepness: 2.0,
    };
    let opts = EmlGadgetCompileOptions { max_col: 8 };
    let a = compile_eml_gadget(&instance, opts).expect("compile a");
    let b = if plant_defect {
        mutant_compile_soft_step_wrong_abs_literal_order(&instance, opts)
    } else {
        compile_eml_gadget(&instance, opts).expect("compile b")
    };
    a.nodes == b.nodes && a.kind == b.kind
}

fn case_passes(case: DetCase, plant_defect: bool) -> bool {
    match case {
        DetCase::Replay => case_replay(plant_defect),
        DetCase::Ordering => case_ordering(plant_defect),
        DetCase::CanonicalSerialization => case_canonical_serialization(plant_defect),
        DetCase::MobilityDispatch => case_mobility_dispatch(plant_defect),
        DetCase::JitArtifact => case_jit_artifact(plant_defect),
    }
}

#[test]
fn determinism_matrix_cases_match() {
    for case in CASES {
        assert!(
            case_passes(case, false),
            "determinism case {case:?} must be stable"
        );
    }
    assert!(
        ordering_owner_silo_tiebreak_path(false),
        "owner-silo equal-claim tie-break must be presentation-order independent"
    );
}

#[test]
fn determinism_matrix_planted_defects_fail() {
    for case in CASES {
        assert!(
            !case_passes(case, true),
            "determinism case {case:?} must FAIL under planted defect"
        );
    }
    assert!(
        !ordering_owner_silo_tiebreak_path(true),
        "owner-silo tie-break planted defect (presentation-order) must diverge"
    );
    assert!(
        !ordering_overlay_path(true),
        "overlay ordering planted defect must diverge"
    );
}
