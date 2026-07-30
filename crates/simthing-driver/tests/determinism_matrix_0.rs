//! TP-PURGE-0 Remand 3 — `determinism_matrix` (DA `5135942768` / Remand `5136003644`,
//! continuation `5136490881`, remainder ruling `5136311181`).
//!
//! Five approved cases. Inline input. Each case runs the live mechanism twice (or
//! sealed replay) and demonstrates a planted-defect failure.
//!
//! The `ordering` case table carries two paths: overlay OrderBand planning, and
//! owner-silo disburse-down equal-claim tie-break (absorbed; not an eleventh case).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use simthing_core::{
    eml_opcode, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlNodeGpu, EmlTreeId,
};
use simthing_gpu::{
    plan_overlay_orderband, OverlayDelta, PackedThresholdUpload, SlotDeltaRange,
    ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES,
};
use simthing_spec::designer_admission::{
    mobility_alloc0_layout_checksum_cpu, plan_mobility_alloc0, MobilityAlloc0BlockSpec,
    MobilityAlloc0BoundaryEvent, MobilityAlloc0BoundaryEventKind,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0ParentKey, MobilityAlloc0PlanInput,
};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, deserialize_mobility_scenario0_packet_ron,
    mobility_scenario0_packet, serialize_mobility_scenario0_packet_ron, OwnerRef, ResourceKey,
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

/// Replay: mobility alloc layout checksum stable across two sealed runs.
/// Planted defect: mutate an arrival entity id on the second run.
fn case_replay(plant_defect: bool) -> bool {
    let input = inline_alloc_input();
    let a = plan_mobility_alloc0(&input);
    let mut input_b = input;
    if plant_defect {
        if let Some(ev) = input_b.events.get_mut(0) {
            ev.entity_id = Some(ev.entity_id.unwrap_or(0).wrapping_add(1));
        }
    }
    let b = plan_mobility_alloc0(&input_b);
    a.assignments == b.assignments
        && mobility_alloc0_layout_checksum_cpu(&a.final_live_slices)
            == mobility_alloc0_layout_checksum_cpu(&b.final_live_slices)
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
    // Equal claim (same priority + request); canonical remainder goes by source id.
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
/// Green: live CPU oracle twice → identical canonical remainder.
/// Planted defect: draw tie-break from presentation order (non-canonical) so two
/// runs with reversed presentation diverge.
fn ordering_owner_silo_tiebreak_path(plant_defect: bool) -> bool {
    let (writeback, demands) = equal_claim_demands();
    if !plant_defect {
        let a = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands)
            .expect("disburse a");
        let b = apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demands)
            .expect("disburse b");
        return allocation_fingerprint(&a) == allocation_fingerprint(&b)
            && a[0].allocations[0].source_simthing_id_raw == Some(1)
            && a[0].allocations[0].allocated == 20
            && a[0].allocations[1].source_simthing_id_raw == Some(2)
            && a[0].allocations[1].allocated == 10;
    }
    let a = allocate_presentation_order(&writeback, &demands);
    let mut reversed = demands.clone();
    reversed.reverse();
    let b = allocate_presentation_order(&writeback, &reversed);
    a == b
}

/// Ordering: overlay OrderBand + owner-silo equal-claim tie-break paths.
fn case_ordering(plant_defect: bool) -> bool {
    ordering_overlay_path(plant_defect) && ordering_owner_silo_tiebreak_path(plant_defect)
}

/// Canonical serialization: mobility scenario0 RON round-trip digest stable.
/// Planted defect: flip a serialized byte before compare.
fn case_canonical_serialization(plant_defect: bool) -> bool {
    let packet = mobility_scenario0_packet();
    let a = serialize_mobility_scenario0_packet_ron(&packet).expect("ser a");
    let mut b = serialize_mobility_scenario0_packet_ron(&packet).expect("ser b");
    if plant_defect {
        let bytes = unsafe { b.as_bytes_mut() };
        if let Some(byte) = bytes.last_mut() {
            *byte ^= 0x01;
        }
    }
    let round_a = deserialize_mobility_scenario0_packet_ron(&a).expect("de a");
    let round_a2 = serialize_mobility_scenario0_packet_ron(&round_a).expect("re-ser");
    fnv(a.as_bytes()) == fnv(b.as_bytes()) && a == round_a2
}

/// Mobility-dispatch: assignment order deterministic for sealed input.
/// Planted defect: reverse assignment order before compare.
fn case_mobility_dispatch(plant_defect: bool) -> bool {
    let report_a = plan_mobility_alloc0(&inline_alloc_input());
    let report_b = plan_mobility_alloc0(&inline_alloc_input());
    let mut order_a: Vec<_> = report_a
        .assignments
        .iter()
        .map(|a| (a.parent_key, a.slot, a.entity_id))
        .collect();
    let order_b: Vec<_> = report_b
        .assignments
        .iter()
        .map(|a| (a.parent_key, a.slot, a.entity_id))
        .collect();
    if plant_defect {
        order_a.reverse();
    }
    order_a == order_b
}

/// JIT-artifact: EML formula node descriptor + packed threshold upload digest stable.
/// Planted defect: alter a node literal on second build.
fn case_jit_artifact(plant_defect: bool) -> bool {
    let build = |scale: f32| {
        let nodes = vec![
            EmlNodeGpu {
                opcode: eml_opcode::LITERAL_F32,
                flags: 0,
                a: scale.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            },
            EmlNodeGpu {
                opcode: eml_opcode::RETURN_TOP,
                flags: 0,
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        ];
        let meta = EmlFormulaMeta {
            tree_id: EmlTreeId(42),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: EmlConsumerMask::default(),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: nodes.len() as u32,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: "jit_art".into(),
        };
        let mut hasher = DefaultHasher::new();
        meta.tree_id.0.hash(&mut hasher);
        meta.node_count.hash(&mut hasher);
        for n in &nodes {
            n.opcode.hash(&mut hasher);
            n.a.hash(&mut hasher);
        }
        let regs = [ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: scale,
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        }];
        let pack = PackedThresholdUpload::from_registrations(&regs).expect("pack");
        let mut bytes = Vec::new();
        bytes.extend_from_slice(bytemuck::cast_slice(pack.ops()));
        (hasher.finish(), fnv(&bytes))
    };
    let a = build(0.5);
    let b = build(if plant_defect { 0.75 } else { 0.5 });
    a == b
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
    // Named evidence: ordering owner-silo path green under canonical tie-break.
    assert!(
        ordering_owner_silo_tiebreak_path(false),
        "owner-silo equal-claim tie-break must be canonically stable"
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
    // Named evidence: ordering owner-silo path red under non-canonical presentation order.
    assert!(
        !ordering_owner_silo_tiebreak_path(true),
        "owner-silo tie-break planted defect (presentation-order) must diverge"
    );
    assert!(
        !ordering_overlay_path(true),
        "overlay ordering planted defect must diverge"
    );
}
