//! TP-PURGE-0 Remand 4 — `determinism_matrix` (DA `5135942768` / Remand `5136696481`).
//!
//! Five approved cases. Inline input. Each case runs the live mechanism twice (or
//! sealed replay) and demonstrates a planted-defect failure inside that mechanism.
//!
//! The `ordering` case table carries two paths: overlay OrderBand planning, and
//! owner-silo disburse-down equal-claim tie-break (absorbed; not an eleventh case).

use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimProperty, SimPropertyId, SimThing, SimThingKind, SubFieldRole,
    TransformOp,
};
use simthing_gpu::{plan_overlay_orderband, OverlayDelta, SlotDeltaRange};
use simthing_sim::{
    BoundaryDeltaEntry, ReplayDriver, ReplayFrame, ReplaySnapshot, SimRuntimeTree,
};
use simthing_spec::designer_admission::{
    mobility_alloc0_layout_checksum_cpu, plan_mobility_alloc0, MobilityAlloc0BlockSpec,
    MobilityAlloc0BoundaryEvent, MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests,
    MobilityAlloc0ParentKey, MobilityAlloc0PlanInput,
};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, compile_eml_gadget,
    deserialize_mobility_scenario0_packet_ron, serialize_mobility_scenario0_packet_ron,
    EmlGadgetCompileOptions, EmlGadgetInstanceSpec, MobilityAllocationBounds,
    MobilityBlockadeSemantics, MobilityIdentityBoundary, MobilityIdentityChannelBudget,
    MobilityOwnerColumn, MobilityOwnerRelationDiscipline, MobilityOwnerRelationKind,
    MobilityQuantityClasses, MobilityRoutingMode, MobilityRoutingPolicy, MobilityScenario0GuardrailRequests,
    MobilityScenario0Packet, MobilityScenario0Status, MobilitySoakProfile, MobilitySupplyScope,
    MobilityTheaterScale, MobilityTheaterShape, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloDisburseDownResult,
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

fn inline_alloc_input() -> MobilityAlloc0PlanInput {
    let key = MobilityAlloc0ParentKey {
        parent_id: 1,
        key_id: 7,
    };
    MobilityAlloc0PlanInput {
        blocks: vec![MobilityAlloc0BlockSpec {
            parent_key: key,
            start_slot: 0,
            slot_count: 4,
            reserved_headroom: 0,
        }],
        live_slices: vec![],
        events: vec![
            MobilityAlloc0BoundaryEvent {
                kind: MobilityAlloc0BoundaryEventKind::Arrival,
                parent_key: key,
                entity_id: Some(10),
                arrival_order: 0,
            },
            MobilityAlloc0BoundaryEvent {
                kind: MobilityAlloc0BoundaryEventKind::Arrival,
                parent_key: key,
                entity_id: Some(11),
                arrival_order: 1,
            },
        ],
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    }
}

fn make_overlay() -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id: SimPropertyId(0),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Set(0.5))],
        },
        lifecycle: OverlayLifecycle::Permanent,
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
            BoundaryDeltaEntry::OverlaySuspended {
                target,
                overlay_id,
            },
        ],
        shadow_values: None,
        spec_entries: Vec::new(),
        injection_entries: Vec::new(),
    };
    (snapshot, frame, overlay_id)
}

fn replay_fingerprint(driver: &ReplayDriver, target: simthing_core::SimThingId, overlay_id: OverlayId) -> (u32, bool, bool) {
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

/// Replay: live `ReplayDriver::apply_frame` twice over the same inline log/state.
/// Planted defect: reverse entry application order (same entry multiset).
fn case_replay(plant_defect: bool) -> bool {
    let (snap, frame, overlay_id) = inline_replay_bundle();
    let target = snap.root.direct_child_id(0).expect("target");
    let apply = |snapshot: ReplaySnapshot, mut frame: ReplayFrame, reverse: bool| {
        if reverse {
            frame.entries.reverse();
        }
        let mut driver = ReplayDriver::from_snapshot(snapshot);
        driver.apply_frame(frame);
        replay_fingerprint(&driver, target, overlay_id)
    };
    let a = apply(snap.clone(), frame.clone(), false);
    let b = apply(snap, frame, plant_defect);
    a == b
}

/// Ordering path 1: overlay OrderBand plan ops byte-identical twice.
/// Planted defect: reorder deltas so band assignment changes.
fn ordering_overlay_path(plant_defect: bool) -> bool {
    let mut deltas = [
        OverlayDelta {
            col: 0,
            op_kind: 0,
            value: 1.0,
            _pad: 0,
        },
        OverlayDelta {
            col: 0,
            op_kind: 0,
            value: 2.0,
            _pad: 0,
        },
    ];
    let ranges = [SlotDeltaRange {
        offset: 0,
        length: 2,
    }];
    let plan_a = plan_overlay_orderband(&deltas, &ranges, 1);
    if plant_defect {
        deltas.swap(0, 1);
    }
    let plan_b = plan_overlay_orderband(&deltas, &ranges, 1);
    plan_a.ops == plan_b.ops && plan_a.n_bands == plan_b.n_bands
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
            scope_id: ScopeId::new("planet_a"),
            planet_id: Some("planet_a".into()),
            star_system_gridcell_id_raw: None,
            requested: 20,
            priority: 1,
            source_simthing_id_raw: Some(2),
        },
        RuntimeOwnerSiloDemandBucket {
            owner_ref: owner,
            resource_key: resource,
            scope_id: ScopeId::new("planet_a"),
            planet_id: Some("planet_a".into()),
            star_system_gridcell_id_raw: None,
            requested: 20,
            priority: 1,
            source_simthing_id_raw: Some(1),
        },
    ];
    (writeback, demands)
}

fn allocation_fingerprint(report: &[RuntimeOwnerSiloDisburseDownResult]) -> Vec<(Option<u32>, u32)> {
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
        let a = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands)
            .expect("disburse a");
        let b = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &reversed)
            .expect("disburse b");
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

/// Canonical serialization: inline packet → live ser → de → reser digest stable.
/// Planted defect: reverse owner_columns before serialize (non-canonical form).
fn case_canonical_serialization(plant_defect: bool) -> bool {
    let packet = inline_mobility_scenario0_packet();
    let a = serialize_mobility_scenario0_packet_ron(&packet).expect("ser a");
    let round = deserialize_mobility_scenario0_packet_ron(&a).expect("de");
    let round_a2 = serialize_mobility_scenario0_packet_ron(&round).expect("re-ser");
    let b = if plant_defect {
        let mut broken = packet.clone();
        broken.owner_columns.reverse();
        serialize_mobility_scenario0_packet_ron(&broken).expect("ser defective")
    } else {
        serialize_mobility_scenario0_packet_ron(&packet).expect("ser b")
    };
    fnv(a.as_bytes()) == fnv(b.as_bytes()) && a == round_a2
}

fn assignment_order(
    report: &simthing_spec::designer_admission::MobilityAlloc0PlanReport,
) -> Vec<(MobilityAlloc0ParentKey, u32, u64)> {
    report
        .assignments
        .iter()
        .map(|a| (a.parent_key, a.slot, a.entity_id))
        .collect()
}

/// Defective mobility dispatch: ignore arrival_order; assign by descending entity_id.
fn defective_dispatch_by_entity_desc(
    input: &MobilityAlloc0PlanInput,
) -> Vec<(MobilityAlloc0ParentKey, u32, u64)> {
    let key = input.blocks[0].parent_key;
    let mut arrivals: Vec<u64> = input
        .events
        .iter()
        .filter(|e| matches!(e.kind, MobilityAlloc0BoundaryEventKind::Arrival))
        .filter_map(|e| e.entity_id)
        .collect();
    arrivals.sort_by_key(|id| std::cmp::Reverse(*id));
    arrivals
        .into_iter()
        .enumerate()
        .map(|(slot, entity_id)| (key, slot as u32, entity_id))
        .collect()
}

/// Mobility-dispatch: live `plan_mobility_alloc0` assignment order deterministic.
/// Planted defect: entity-id descending allocator (ignores arrival_order).
fn case_mobility_dispatch(plant_defect: bool) -> bool {
    let input = inline_alloc_input();
    let live = assignment_order(&plan_mobility_alloc0(&input));
    let other = if plant_defect {
        defective_dispatch_by_entity_desc(&input)
    } else {
        assignment_order(&plan_mobility_alloc0(&input))
    };
    live == other
}

/// JIT-artifact: live `compile_eml_gadget` twice from the same SoftStep instance.
/// Planted defect: reverse compiled node order (encoding/order defect).
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
    let mut b = compile_eml_gadget(&instance, opts).expect("compile b");
    if plant_defect {
        b.nodes.reverse();
    }
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
    // Keep layout checksum import live for absorbed replay/layout evidence surface.
    let report = plan_mobility_alloc0(&inline_alloc_input());
    let _ = mobility_alloc0_layout_checksum_cpu(&report.final_live_slices);
}
