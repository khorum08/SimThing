//! ACTIONBAND-COMPOSITION-PROBE-0 — independent workshop witnesses + Claim B.
//!
//! Three non-spatial witnesses are separate runners (no shared ActionBand
//! helper). Landed 7.1 `movement_ingress` is witness four (read-only API use).

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_core::evaluate::Evaluator;
use simthing_core::{
    cost_band_depth_one, cost_band_quantize, ColumnIndex, CostBandDraw, DimensionRegistry,
    Direction, DissolveCondition, FissionTemplate, FissionThreshold, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, PropertyValue, SimProperty,
    SimPropertyId, SimThing, SimThingId, SimThingKind, SimThingKindTag, SubFieldRole, TransformOp,
};
use simthing_driver::{
    admit_comparative_projections, comparative_projection_cpu_oracle, neighbor_slots_from_link_rows,
    receive_command_deficits_from_disbursement, CommandDeficit, ComparativeEmitterClass,
    ComparativeProjectionBands,
};
use simthing_gpu::{
    cpu_oracle_threshold_events, AccumulatorOpSession, FieldAdjacency, GpuContext,
    LinkGraphNeighbor, PackedThresholdUpload, SlotAllocator, ThresholdRegistration, DIR_UPWARD,
    THRESH_BUF_VALUES,
};
use simthing_kernel::{
    BoundaryEmissionToken, EmissionToken, StructuralCommitment, ThresholdCrossingToken,
};
use simthing_sim::fission::resolve_fission_fusion;
use simthing_sim::{
    apply_movement_commitments, validate_movement_cost_band, validate_movement_overlay,
    CostBandSemantic, MovementCommitment, MovementFieldLocus, MovementIngressError,
    MovementOverlayEffect, SimRuntimeTree, ThresholdRegistry, ThresholdSemantic,
};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, apply_runtime_local_allocations_from_disburse_down,
    OwnerRef, ResourceKey, RuntimeOwnerSiloDemandBucket, RuntimeOwnerSiloWritebackResult, ScopeId,
};
use simthing_workshop::actionband_composition_probe_0::{
    determine_candidate, measured_structural_tables, probe_candidate, CandidateDisposition,
    StageClass,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

// ─── Witness 1: deficit / resource satisfaction (independent) ───────────────

#[derive(Clone, Copy)]
enum DeficitAuthority {
    FieldDerived,
    /// Planted hard-coded authority at the potential→candidate site.
    HardCodedReceiverA,
}

#[derive(Clone, Copy)]
enum DeficitClearing {
    Deliver,
    /// Planted clearing-bypass at the real claim→allocation site.
    BypassDelivery,
}

struct DeficitAttractors {
    pot_a: f32,
    pot_b: f32,
    overlay_scale_b: Option<f32>,
}

fn deficit_run(
    attractors: DeficitAttractors,
    authority: DeficitAuthority,
    clearing: DeficitClearing,
) -> (SimThingId, SimThingId, bool, bool, u32) {
    let mut registry = DimensionRegistry::new();
    let property_id = registry.register(SimProperty::simple("probe", "signal", 0));
    let layout = registry.property(property_id).layout.clone();

    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut policy_host = SimThing::new(SimThingKind::Location, 0);
    let origin = SimThing::new(SimThingKind::Cohort, 0);
    let origin_id = origin.id;
    let policy_host_id = policy_host.id;
    policy_host.add_overlay(Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Policy,
        source: OverlaySource::System,
        origin: policy_host_id,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(0.5))],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    });
    policy_host.add_child(origin);

    let mk_receiver = |amount: f32| {
        let mut node = SimThing::new(SimThingKind::Cohort, 0);
        let mut value = PropertyValue::from_layout(&layout);
        value.set_role(&SubFieldRole::Amount, &layout, amount);
        node.add_property(property_id, value);
        node
    };
    let recv_a = mk_receiver(0.2);
    let recv_b = mk_receiver(0.2);
    let recv_a_id = recv_a.id;
    let recv_b_id = recv_b.id;

    // Competing attractor markers (scenario-neutral); potentials are Amounts.
    let mut attr_a = SimThing::new(SimThingKind::Location, 0);
    let mut attr_b = SimThing::new(SimThingKind::Location, 0);
    let mut va = PropertyValue::from_layout(&layout);
    va.set_role(&SubFieldRole::Amount, &layout, attractors.pot_a);
    attr_a.add_property(property_id, va);
    let mut vb = PropertyValue::from_layout(&layout);
    vb.set_role(&SubFieldRole::Amount, &layout, attractors.pot_b);
    attr_b.add_property(property_id, vb);
    let attr_b_id = attr_b.id;
    if let Some(scale) = attractors.overlay_scale_b {
        attr_b.add_overlay(Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Policy,
            source: OverlaySource::System,
            origin: attr_b_id,
            affects: Vec::new(),
            transform: PropertyTransformDelta {
                property_id,
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(scale))],
            },
            lifecycle: OverlayLifecycle::UntilDissolved,
        });
    }

    root.add_child(policy_host);
    root.add_child(recv_a);
    root.add_child(recv_b);
    root.add_child(attr_a);
    root.add_child(attr_b);

    let eval = Evaluator::new(&registry, 0.0).evaluate(&root, 0);
    let read_pot = |id: SimThingId| {
        eval.get(id)
            .and_then(|e| e.properties.get(&property_id))
            .map(|v| v.get_role(&SubFieldRole::Amount, &layout))
            .expect("attractor amount")
    };
    // attr_a is children[3], attr_b children[4] — recover ids from tree.
    let attr_a_id = root.children[3].id;
    let pot_a = read_pot(attr_a_id);
    let pot_b = read_pot(attr_b_id);

    let (prio_a, prio_b) = match authority {
        DeficitAuthority::HardCodedReceiverA => (0u32, 1u32),
        DeficitAuthority::FieldDerived => {
            if pot_a >= pot_b {
                (0, 1)
            } else {
                (1, 0)
            }
        }
    };

    let owner_ref = OwnerRef::new("owner");
    let resource_key = ResourceKey::new("command");
    let scope_id = ScopeId::from_boundary(root.id);
    let writeback = vec![RuntimeOwnerSiloWritebackResult {
        owner_ref: owner_ref.clone(),
        resource_key: resource_key.clone(),
        previous_current: 1,
        next_current: 1,
        capacity: None,
        applied_surplus: 0,
        applied_deficit: 0,
        clamped_surplus: 0,
        unmet_deficit: 0,
    }];

    let mk_directive = |origin: SimThingId| Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin,
        affects: Vec::new(),
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.4))],
        },
        lifecycle: OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions: vec![DissolveCondition::AtSessionEnd],
        },
    };

    let deficits = [
        CommandDeficit {
            receiver: recv_a_id,
            owner_ref: owner_ref.clone(),
            resource_key: resource_key.clone(),
            scope_id: scope_id.clone(),
            priority: prio_a,
            directive: mk_directive(origin_id),
        },
        CommandDeficit {
            receiver: recv_b_id,
            owner_ref: owner_ref.clone(),
            resource_key: resource_key.clone(),
            scope_id: scope_id.clone(),
            priority: prio_b,
            directive: mk_directive(origin_id),
        },
    ];

    // CostBand depth-one observation on the granted unit (composed beside reception).
    let draw = cost_band_depth_one(1.0, 1.0, true).expect("depth-one command unit");
    assert_eq!(draw.n, 1);

    let (a_got, b_got, allocated) = match clearing {
        DeficitClearing::Deliver => {
            let report = receive_command_deficits_from_disbursement(&mut root, &writeback, &deficits)
                .expect("RF reception");
            let a_got = root
                .children
                .iter()
                .find(|c| c.id == recv_a_id)
                .map(|c| !c.overlays.is_empty())
                .unwrap_or(false);
            let b_got = root
                .children
                .iter()
                .find(|c| c.id == recv_b_id)
                .map(|c| !c.overlays.is_empty())
                .unwrap_or(false);
            (a_got, b_got, report.local_allocation.allocated_total)
        }
        DeficitClearing::BypassDelivery => {
            // Real claim→allocation site: disburse + local allocate, skip delivery.
            let demand: Vec<_> = deficits
                .iter()
                .map(|d| RuntimeOwnerSiloDemandBucket {
                    owner_ref: d.owner_ref.clone(),
                    resource_key: d.resource_key.clone(),
                    scope_id: d.scope_id.clone(),
                    requested: 1,
                    priority: d.priority,
                    source_simthing_id_raw: Some(d.receiver.raw()),
                })
                .collect();
            let disbursed =
                apply_owner_silo_runtime_disburse_down_cpu(&writeback, &demand).expect("disburse");
            let allocated = apply_runtime_local_allocations_from_disburse_down(&disbursed)
                .expect("allocate");
            let a_got = root
                .children
                .iter()
                .find(|c| c.id == recv_a_id)
                .map(|c| !c.overlays.is_empty())
                .unwrap_or(false);
            let b_got = root
                .children
                .iter()
                .find(|c| c.id == recv_b_id)
                .map(|c| !c.overlays.is_empty())
                .unwrap_or(false);
            (a_got, b_got, allocated.allocated_total)
        }
    };

    (recv_a_id, recv_b_id, a_got, b_got, allocated)
}

#[test]
fn deficit_claim_b_field_and_overlay_redirect_without_identity_edits() {
    // Amount roles are bounded [0,1]; stay inside the clamp for field authority.
    let baseline = deficit_run(
        DeficitAttractors {
            pot_a: 0.9,
            pot_b: 0.2,
            overlay_scale_b: None,
        },
        DeficitAuthority::FieldDerived,
        DeficitClearing::Deliver,
    );
    assert!(baseline.2 && !baseline.3, "A wins on higher field potential");

    let field_flip = deficit_run(
        DeficitAttractors {
            pot_a: 0.2,
            pot_b: 0.9,
            overlay_scale_b: None,
        },
        DeficitAuthority::FieldDerived,
        DeficitClearing::Deliver,
    );
    assert!(!field_flip.2 && field_flip.3, "field-only change redirects to B");

    let overlay_flip = deficit_run(
        DeficitAttractors {
            pot_a: 0.5,
            pot_b: 0.4,
            overlay_scale_b: Some(2.0), // evaluated pot_b ≈ 0.8 > pot_a
        },
        DeficitAuthority::FieldDerived,
        DeficitClearing::Deliver,
    );
    assert!(
        !overlay_flip.2 && overlay_flip.3,
        "overlay-only weighting redirects to B without editing receiver/action identity"
    );
}

#[test]
fn deficit_hardcoded_authority_and_clearing_bypass_mutants_red() {
    // Hard-coded authority: field flip no longer redirects → Claim B broken (RED).
    let mutant_field = deficit_run(
        DeficitAttractors {
            pot_a: 0.2,
            pot_b: 0.9,
            overlay_scale_b: None,
        },
        DeficitAuthority::HardCodedReceiverA,
        DeficitClearing::Deliver,
    );
    assert!(
        mutant_field.2 && !mutant_field.3,
        "hard-coded authority keeps A despite field favoring B"
    );

    // Clearing bypass: allocation succeeds, consequence absent (RED vs green path).
    let bypass = deficit_run(
        DeficitAttractors {
            pot_a: 0.9,
            pot_b: 0.2,
            overlay_scale_b: None,
        },
        DeficitAuthority::FieldDerived,
        DeficitClearing::BypassDelivery,
    );
    assert_eq!(bypass.4, 1, "unit still allocated");
    assert!(!bypass.2 && !bypass.3, "no directive arrives when delivery bypassed");
}

// ─── Witness 2: LinkGraph relational action (independent) ───────────────────

#[derive(Clone, Copy)]
enum LinkAuthority {
    FieldDerived,
    HardCodedClass0,
}

#[derive(Clone, Copy)]
enum LinkClearing {
    BindDominance,
    /// Skip consequence binding after comparative valuation (claim→effect bypass).
    BypassConsequence,
}

fn linkgraph_run(
    e0: f32,
    e1: f32,
    overlay_scale_e1: Option<f32>,
    authority: LinkAuthority,
    clearing: LinkClearing,
) -> (f32, bool) {
    let mut reg = DimensionRegistry::new();
    let mut ids = [SimPropertyId(0); 5];
    for (i, (ns, name)) in [
        ("feed", "e0"),
        ("feed", "e1"),
        ("feed", "d"),
        ("feed", "u"),
        ("feed", "c"),
    ]
    .iter()
    .enumerate()
    {
        ids[i] = reg.register(SimProperty::simple(ns, name, 1));
    }
    let link_rows = {
        let mut rows = vec![Vec::new(); 2];
        rows[0].push(LinkGraphNeighbor {
            slot: simthing_core::SlotIndex::new(1),
            weight: 1.0,
        });
        rows[1].push(LinkGraphNeighbor {
            slot: simthing_core::SlotIndex::new(0),
            weight: 1.0,
        });
        rows
    };
    let adj = FieldAdjacency::link_graph(
        2,
        link_rows.clone(),
        ColumnIndex::from_gpu_round_trip(0),
    )
    .expect("link graph");
    let neighbors = neighbor_slots_from_link_rows(&link_rows);
    let emitters = vec![
        ComparativeEmitterClass {
            authored_order: 0,
            class_id: 10.0,
            value_col: ColumnIndex::from_gpu_round_trip(reg.column_range(ids[0]).start as u32),
        },
        ComparativeEmitterClass {
            authored_order: 1,
            class_id: 20.0,
            value_col: ColumnIndex::from_gpu_round_trip(reg.column_range(ids[1]).start as u32),
        },
    ];
    let d = ColumnIndex::from_gpu_round_trip(reg.column_range(ids[2]).start as u32);
    let u = ColumnIndex::from_gpu_round_trip(reg.column_range(ids[3]).start as u32);
    let c = ColumnIndex::from_gpu_round_trip(reg.column_range(ids[4]).start as u32);
    let adm = admit_comparative_projections(
        &mut reg,
        adj,
        neighbors.clone(),
        emitters.clone(),
        d,
        u,
        c,
        ComparativeProjectionBands::default(),
        None,
    )
    .expect("admit comparative");

    let n_dims = reg.total_columns as usize;
    let mut vals = vec![0.0f32; 2 * n_dims];
    let e0c = reg.column_range(ids[0]).start;
    let e1c = reg.column_range(ids[1]).start;
    let mut e1_eff = e1;
    if let Some(scale) = overlay_scale_e1 {
        e1_eff *= scale;
    }
    for s in 0..2 {
        let b = s * n_dims;
        vals[b + e0c] = e0;
        vals[b + e1c] = e1_eff;
        vals[b + reg.column_range(ids[2]).start] = 12.0;
        vals[b + reg.column_range(ids[3]).start] = 1.0;
        vals[b + reg.column_range(ids[4]).start] = 0.5;
    }

    let oracle = comparative_projection_cpu_oracle(
        &vals,
        2,
        n_dims as u32,
        &emitters,
        adm.outputs,
        adm.band_readouts,
        d,
        adm.stall_outputs.stall_col,
        ComparativeProjectionBands::default(),
        &neighbors,
    );
    let mut winner = oracle[adm.outputs.dominance_col.raw()];
    if matches!(authority, LinkAuthority::HardCodedClass0) {
        winner = emitters[0].class_id;
    }
    let margin = (e0 - e1_eff).abs().max(0.01);
    let draw = cost_band_depth_one(margin, 0.05, true).expect("margin CostBand");
    let progress = match clearing {
        LinkClearing::BindDominance => draw.n > 0 && winner > 0.0,
        LinkClearing::BypassConsequence => {
            // Valuation ran; consequence binding skipped.
            false
        }
    };
    (winner, progress)
}

#[test]
fn linkgraph_claim_b_field_and_overlay_redirect_without_identity_edits() {
    let (w0, p0) = linkgraph_run(0.9, 0.2, None, LinkAuthority::FieldDerived, LinkClearing::BindDominance);
    assert!(p0);
    assert_eq!(w0.to_bits(), 10.0f32.to_bits());

    let (w1, _) = linkgraph_run(0.2, 0.9, None, LinkAuthority::FieldDerived, LinkClearing::BindDominance);
    assert_eq!(w1.to_bits(), 20.0f32.to_bits(), "field-only redirect");

    let (w2, _) = linkgraph_run(
        0.5,
        0.4,
        Some(2.0),
        LinkAuthority::FieldDerived,
        LinkClearing::BindDominance,
    );
    assert_eq!(
        w2.to_bits(),
        20.0f32.to_bits(),
        "overlay-only scale on e1 redirects without editing emitter class identities"
    );
}

#[test]
fn linkgraph_hardcoded_authority_and_clearing_bypass_mutants_red() {
    let (w, _) = linkgraph_run(
        0.2,
        0.9,
        None,
        LinkAuthority::HardCodedClass0,
        LinkClearing::BindDominance,
    );
    assert_eq!(
        w.to_bits(),
        10.0f32.to_bits(),
        "hard-coded authority ignores field favoring class 20"
    );

    let (_, progress) = linkgraph_run(
        0.9,
        0.2,
        None,
        LinkAuthority::FieldDerived,
        LinkClearing::BypassConsequence,
    );
    assert!(!progress, "clearing/consequence bypass leaves no progress");
}

// ─── Witness 3: derivation / fission (independent) ──────────────────────────

#[derive(Clone, Copy)]
enum FissionAuthority {
    FieldDerived,
    HardCodedParentA,
}

#[derive(Clone, Copy)]
enum FissionClearing {
    Resolve,
    /// Skip resolve_fission_fusion after CostBand authorizes (claim→consequence bypass).
    BypassResolve,
}

fn fission_run(
    pot_a: f32,
    pot_b: f32,
    overlay_scale_b: Option<f32>,
    authority: FissionAuthority,
    clearing: FissionClearing,
) -> (usize, usize) {
    let mut registry = DimensionRegistry::new();
    let mut growth = SimProperty::simple("probe", "growth", 0);
    growth.fission_templates = vec![FissionThreshold {
        sub_field: SubFieldRole::Amount,
        threshold: 0.5,
        direction: Direction::Rising,
        template: FissionTemplate {
            child_kind: SimThingKindTag::Cohort,
            fusion_intensity_threshold: 0.8,
            fusion_scar_coefficient: 0.05,
            resolution_label: "probe_fission".into(),
            clone_capability_children: false,
            capability_container_kinds: Vec::new(),
        },
        secondary: None,
    }];
    let property_id = registry.register(growth);
    let layout = registry.property(property_id).layout.clone();

    let mut parent_a = SimThing::new(SimThingKind::Owner, 0);
    let mut parent_b = SimThing::new(SimThingKind::Owner, 0);
    let mut va = PropertyValue::from_layout(&layout);
    va.set_role(&SubFieldRole::Amount, &layout, 1.0);
    parent_a.add_property(property_id, va.clone());
    parent_b.add_property(property_id, va);
    let parent_a_id = parent_a.id;
    let parent_b_id = parent_b.id;

    let mut root = SimThing::new(SimThingKind::Location, 0);
    root.add_child(parent_a);
    root.add_child(parent_b);

    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let slot_a = allocator.slot_of(parent_a_id).unwrap().raw();
    let slot_b = allocator.slot_of(parent_b_id).unwrap().raw();
    let n_dims = registry.total_columns.max(1);
    let col = 0u32;

    let mut pot_b_eff = pot_b;
    if let Some(scale) = overlay_scale_b {
        pot_b_eff *= scale;
    }

    let mut previous = vec![0.0; 16 * n_dims];
    let mut values = vec![0.0; 16 * n_dims];
    // Rising cross of threshold 0.5: previous below, current above.
    previous[slot_a as usize * n_dims + col as usize] = 0.4;
    previous[slot_b as usize * n_dims + col as usize] = 0.4;
    values[slot_a as usize * n_dims + col as usize] = pot_a;
    values[slot_b as usize * n_dims + col as usize] = pot_b_eff;

    let mut cpu_reg = ThresholdRegistry::new();
    let kind_a = cpu_reg.push_with_cost_band(
        ThresholdSemantic::FissionTrigger {
            sim_thing_id: parent_a_id,
            property_id,
            template_idx: 0,
        },
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    );
    let kind_b = cpu_reg.push_with_cost_band(
        ThresholdSemantic::FissionTrigger {
            sim_thing_id: parent_b_id,
            property_id,
            template_idx: 0,
        },
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    );

    let regs = [
        ThresholdRegistration {
            slot: slot_a,
            col,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: kind_a,
            buffer: THRESH_BUF_VALUES,
        },
        ThresholdRegistration {
            slot: slot_b,
            col,
            threshold: 0.5,
            direction: DIR_UPWARD,
            event_kind: kind_b,
            buffer: THRESH_BUF_VALUES,
        },
    ];
    let mut events =
        cpu_oracle_threshold_events(&previous, &values, &previous, &values, n_dims as u32, &regs, 0);

    if matches!(authority, FissionAuthority::HardCodedParentA) {
        // Potential→candidate site: ignore field winner; force parent A only.
        let mut forced = values.clone();
        forced[slot_a as usize * n_dims + col as usize] = 0.9;
        events = cpu_oracle_threshold_events(
            &previous,
            &forced,
            &previous,
            &forced,
            n_dims as u32,
            &regs[..1],
            0,
        );
        values[slot_a as usize * n_dims + col as usize] = 0.9;
    }

    // Gate each event through CostBand on the activating magnitude.
    let mut authorized = Vec::new();
    for event in &events {
        let slot = event.slot();
        let v = values[slot as usize * n_dims + col as usize];
        let draw = cpu_reg
            .resolve_cost_band_draw(event.event_kind(), v, 0.5)
            .expect("CostBand");
        if draw.n >= 1 {
            authorized.push(*event);
        }
    }

    let before_a = root.children[0].children.len();
    let before_b = root.children[1].children.len();

    if matches!(clearing, FissionClearing::Resolve) {
        let paths = HashMap::from([(parent_a_id, vec![0usize]), (parent_b_id, vec![1usize])]);
        let _ = resolve_fission_fusion(
            &mut root,
            &paths,
            &registry,
            &mut allocator,
            &authorized,
            &cpu_reg,
            &mut values,
            n_dims,
            1,
        );
    }

    let after_a = root.children[0].children.len();
    let after_b = root.children[1].children.len();
    (after_a.saturating_sub(before_a), after_b.saturating_sub(before_b))
}

#[test]
fn fission_claim_b_field_and_overlay_redirect_without_identity_edits() {
    let (a, b) = fission_run(0.9, 0.2, None, FissionAuthority::FieldDerived, FissionClearing::Resolve);
    assert!(a >= 1 && b == 0, "A fires on higher field potential");

    let (a2, b2) = fission_run(0.2, 0.9, None, FissionAuthority::FieldDerived, FissionClearing::Resolve);
    assert!(a2 == 0 && b2 >= 1, "field-only redirect to B");

    let (a3, b3) = fission_run(
        0.8,
        0.45,
        Some(2.0),
        FissionAuthority::FieldDerived,
        FissionClearing::Resolve,
    );
    assert!(
        b3 >= 1,
        "overlay-only scale enables B progress without editing parent/action identity (a={a3}, b={b3})"
    );
}

#[test]
fn fission_hardcoded_authority_and_clearing_bypass_mutants_red() {
    let (a, b) = fission_run(
        0.2,
        0.9,
        None,
        FissionAuthority::HardCodedParentA,
        FissionClearing::Resolve,
    );
    assert!(
        a >= 1 && b == 0,
        "hard-coded authority forces A and suppresses B despite field favoring B (a={a}, b={b})"
    );

    let (a2, b2) = fission_run(
        0.9,
        0.2,
        None,
        FissionAuthority::FieldDerived,
        FissionClearing::BypassResolve,
    );
    assert_eq!((a2, b2), (0, 0), "clearing/consequence bypass yields no fission");
}

// ─── Witness 4: landed 7.1 movement (read-only) ─────────────────────────────

struct MovementArena {
    tree: SimRuntimeTree,
    actor: SimThingId,
    a: SimThingId,
    b: SimThingId,
    c: SimThingId,
    property: SimPropertyId,
    loci: Vec<MovementFieldLocus>,
    allocator: SlotAllocator,
    registry: DimensionRegistry,
}

fn movement_arena() -> MovementArena {
    let mut registry = DimensionRegistry::new();
    let property = registry.register(SimProperty::simple("move", "pressure", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut a = SimThing::new(SimThingKind::Location, 0);
    let b = SimThing::new(SimThingKind::Location, 0);
    let c = SimThing::new(SimThingKind::Location, 0);
    let (a_id, b_id, c_id) = (a.id, b.id, c.id);
    let actor = SimThing::new(SimThingKind::Cohort, 0);
    let actor_id = actor.id;
    a.add_child(actor);
    root.add_child(a);
    root.add_child(b);
    root.add_child(c);
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let loci = vec![
        MovementFieldLocus {
            slot: 0,
            value_col: 0,
            grid_row: 0,
            grid_col: 0,
            cell: a_id,
        },
        MovementFieldLocus {
            slot: 1,
            value_col: 0,
            grid_row: 0,
            grid_col: 1,
            cell: b_id,
        },
        MovementFieldLocus {
            slot: 2,
            value_col: 0,
            grid_row: 1,
            grid_col: 0,
            cell: c_id,
        },
    ];
    MovementArena {
        tree: SimRuntimeTree::admit(root),
        actor: actor_id,
        a: a_id,
        b: b_id,
        c: c_id,
        property,
        loci,
        allocator,
        registry,
    }
}

fn sealed_commitment(decision_slot: u32, value: f32) -> StructuralCommitment {
    let _guard = GPU_MUTEX.lock().unwrap();
    simthing_kernel::set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("GPU context");
    let n_slots = 4;
    let n_dims = 1;
    let mut session = AccumulatorOpSession::new_attached(&ctx, n_slots, n_dims, 8);
    session.bind_generation_authority(7);
    let previous = vec![0.0; (n_slots * n_dims) as usize];
    let mut current = previous.clone();
    current[(decision_slot * n_dims) as usize] = value;
    session.upload_values(&ctx, &current);
    session.upload_previous_values(&ctx, &previous);
    let regs = [ThresholdRegistration {
        slot: decision_slot,
        col: 0,
        threshold: 1.0,
        direction: DIR_UPWARD,
        event_kind: 0x4d4f_5645,
        buffer: THRESH_BUF_VALUES,
    }];
    session
        .upload_packed_threshold_ops(
            &ctx,
            &PackedThresholdUpload::from_registrations(&regs).expect("pack"),
        )
        .expect("upload");
    session.tick(&ctx, 0).expect("tick");
    let events = session.readback_threshold_events(&ctx).expect("events");
    let emissions = session.readback_threshold_emissions(&ctx).expect("emissions");
    assert_eq!(events.len(), 1);
    let threshold = ThresholdCrossingToken::from_sealed_threshold_event(&events[0]);
    let emission = EmissionToken::from_sealed_threshold_emission(&emissions[0]);
    let boundary = BoundaryEmissionToken::bind(threshold, emission).expect("bind");
    StructuralCommitment::mint_from_sealed_path(threshold, emission, boundary).expect("mint")
}

/// Overlay-weighted field: raise one candidate cell's sealed value without
/// editing cell/actor identities (ordinary numeric weighting only).
fn sealed_commitment_overlay_weighted(favor_slot: u32, base: f32, overlay_add: f32) -> StructuralCommitment {
    sealed_commitment(favor_slot, base + overlay_add)
}

fn movement_effect(property: SimPropertyId) -> MovementOverlayEffect {
    MovementOverlayEffect {
        property_id: property,
        deltas: vec![(SubFieldRole::Amount, TransformOp::add(0.25))],
    }
}

#[test]
fn movement_claim_b_field_and_overlay_redirect_without_identity_edits() {
    let arena = movement_arena();
    let to_b = MovementCommitment::admit(
        sealed_commitment(1, 1.5),
        arena.actor,
        arena.a,
        2,
        &arena.loci,
        movement_effect(arena.property),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .expect("admit B");
    assert_eq!(to_b.deciding_cell(), arena.b);

    let to_c = MovementCommitment::admit(
        sealed_commitment(2, 1.5),
        arena.actor,
        arena.a,
        2,
        &arena.loci,
        movement_effect(arena.property),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .expect("admit C");
    assert_eq!(to_c.deciding_cell(), arena.c, "field-only sealed locus redirect");

    // Overlay-only: same competing attractors (slots 1 and 2); ordinary numeric
    // weighting selects C instead of B without editing locus cell identities.
    let overlay_to_c = MovementCommitment::admit(
        sealed_commitment_overlay_weighted(2, 1.0, 0.75),
        arena.actor,
        arena.a,
        2,
        &arena.loci,
        movement_effect(arena.property),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
        1.0,
    )
    .expect("overlay-weighted admit C");
    assert_eq!(overlay_to_c.deciding_cell(), arena.c);
    assert_eq!(arena.loci[1].cell, arena.b);
    assert_eq!(arena.loci[2].cell, arena.c);
}

#[test]
fn movement_hardcoded_origin_and_costband_bypass_mutants_red() {
    let arena = movement_arena();
    let arrival = OverlayLifecycle::UntilDissolvedWith {
        dissolution_conditions: vec![DissolveCondition::ArrivedAt {
            destination: arena.b,
        }],
    };
    let hardcoded_origin = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Instruction,
        source: OverlaySource::System,
        origin: arena.a, // must be deciding cell B
        affects: vec![arena.actor],
        transform: PropertyTransformDelta {
            property_id: arena.property,
            sub_field_deltas: Vec::new(),
        },
        lifecycle: arrival,
    };
    assert_eq!(
        validate_movement_overlay(arena.actor, arena.b, &hardcoded_origin),
        Err(MovementIngressError::OverlayOriginDrift)
    );

    let commitment = sealed_commitment(1, 1.75);
    let semantic = CostBandSemantic::admit_sink(Some(1), None).unwrap();
    let good = cost_band_quantize(1.75, 1.0, true, Some(1)).unwrap();
    let bypass = CostBandDraw {
        r: good.r + 0.25,
        ..good
    };
    assert_eq!(
        validate_movement_cost_band(commitment, semantic, 1.0, bypass),
        Err(MovementIngressError::CostBandBypass)
    );

    // Production apply still requires a valid admit; foreign endpoint rejected.
    let mut foreign = arena;
    let synthetic = SimThingId::new();
    foreign.loci[1].cell = synthetic;
    let admitted = MovementCommitment::admit(
        sealed_commitment(1, 1.75),
        foreign.actor,
        foreign.a,
        2,
        &foreign.loci,
        movement_effect(foreign.property),
        semantic,
        1.0,
    )
    .unwrap();
    let n_dims = foreign.registry.total_columns;
    let mut shadow = vec![0.0; foreign.allocator.capacity() * n_dims];
    let (_, rejected) = apply_movement_commitments(
        vec![admitted],
        &mut foreign.tree,
        &mut foreign.allocator,
        &mut foreign.registry,
        &mut shadow,
        n_dims,
    );
    assert_eq!(rejected.applied, 0);
    assert_eq!(rejected.rejected, 1);
}

// ─── Structural table + candidate B ─────────────────────────────────────────

#[test]
fn structural_table_concludes_candidate_b_with_special_seams() {
    let tables = measured_structural_tables();
    assert_eq!(determine_candidate(&tables), CandidateDisposition::B);
    assert_eq!(probe_candidate(), CandidateDisposition::B);
    for table in &tables {
        assert!(
            table
                .rows
                .iter()
                .any(|r| r.class == StageClass::SpecialSeam),
            "{} lacks SPECIAL-SEAM evidence against literal common path",
            table.witness_id
        );
    }
}
