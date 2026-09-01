//! RESIDENT-CLEARING-SCORE-AND-BANDS-0 — direct continuous self-consumption,
//! row-11 recurrence, and physical-order invariance witnesses.

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlExpressionRegistry, EmlNodeGpu,
    GateSpec, GenerationStamp, InputSpec, ScaleSpec, SimPropertyId, SimThingId, SlotIndex,
    SourceSpec, TransformOp,
};
use simthing_driver::{
    build_custom_layout, plan_arena_allocation, register_child_share_formula,
    run_arena_allocation_oracle, ArenaTreeLayout, FissionPolicy, GpuArenaDescriptor, HierarchyNode,
    NodeColumnRefs,
};
use simthing_gpu::{AccumulatorOpSession, EmlGpuProgramTable, GpuContext, PackedAccumulatorUpload};
use simthing_spec::{
    carry_unresolved_demand_to_next_generation, clear_constrained_claims_at_generation,
    AuthoredClearingProgram, ClearingRemainderAuthority, ConstrainedClaim,
    ConstrainedClearingResult, ConstrainedSupply, OwnerChannelScopeKey, OwnerRef, ResourceKey,
    RuntimeOwnerSiloDemandBucket, ScopeId, UnresolvedDemandObservation,
};
use std::collections::HashMap;

fn col(raw: usize) -> ColumnIndex {
    ColumnIndex::from_raw_for_oracle_or_rehearsal(raw)
}

fn cols() -> NodeColumnRefs {
    NodeColumnRefs {
        intrinsic_flow_col: col(0),
        allocated_flow_col: col(1),
        weight_col: col(2),
        intrinsic_flow_sum_col: col(3),
        weight_sum_col: col(4),
        balance_col: None,
        balance_governing_col: None,
        // Frozen ABI columns remain present but the 14.3 allocator plan must
        // neither read nor write them.
        propagated_intrinsic_flow_col: col(5),
        propagated_allocated_flow_col: col(6),
        propagated_weight_sum_col: col(7),
        hosted_simthing_id_col: col(8),
    }
}

fn d3_root(slots: [u32; 3], identity_base: u32) -> HierarchyNode {
    let cols = cols();
    let grandchild = HierarchyNode {
        participant_slot: SlotIndex::new(slots[2]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base + 2),
        depth: 2,
        children: vec![],
        cols,
    };
    let child = HierarchyNode {
        participant_slot: SlotIndex::new(slots[1]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base + 1),
        depth: 1,
        children: vec![grandchild],
        cols,
    };
    HierarchyNode {
        participant_slot: SlotIndex::new(slots[0]),
        hosted_simthing_id: SimThingId::from_session_raw(identity_base),
        depth: 0,
        children: vec![child],
        cols,
    }
}

fn layout_from_roots(roots: Vec<HierarchyNode>) -> ArenaTreeLayout {
    let cols = cols();
    build_custom_layout(
        0,
        &GpuArenaDescriptor {
            name: "resident-self-consumption".into(),
            flow_property_id: SimPropertyId(1),
            balance_property_id: None,
            max_participants: 64,
            max_coupling_fanout: 4,
            max_orderband_depth: 16,
            fission_policy: FissionPolicy::default(),
            participant_range: (0, 0),
            wildcard_max_expansion: None,
            reserved_orderband_depth: 0,
        },
        cols,
        roots,
    )
    .expect("D3 resident self-consumption layout")
}

fn d3_layout(slots: [u32; 3]) -> ArenaTreeLayout {
    layout_from_roots(vec![d3_root(slots, 1)])
}

fn wide_d3_layout() -> ArenaTreeLayout {
    layout_from_roots(
        (0..6)
            .map(|index| {
                let base = index * 10;
                d3_root([base, base + 1, base + 2], 1 + index * 3)
            })
            .collect(),
    )
}

fn assert_direct_recursive_shape(layout: &ArenaTreeLayout) {
    let plan = plan_arena_allocation(layout, &[], 256).expect("resident allocator plan");
    let cols = cols();
    let parent = &layout.participant_roots[0].children[0];
    let grandchild = &parent.children[0];
    let grandchild_disburse = plan
        .cpu_ops
        .iter()
        .find(|op| {
            op.targets
                == vec![(
                    grandchild.participant_slot,
                    grandchild.cols.allocated_flow_col,
                )]
                && matches!(op.combine, simthing_core::CombineFn::EvalEML { .. })
        })
        .expect("same allocator operation writes the next recursive level");
    let SourceSpec::ConjunctiveCrossing { inputs } = &grandchild_disburse.source else {
        panic!("child-share EvalEML must read its parent through the admitted input list");
    };
    assert_eq!(
        inputs
            .iter()
            .map(|input| (input.slot, input.col))
            .collect::<Vec<_>>(),
        vec![
            (parent.participant_slot, parent.cols.intrinsic_flow_sum_col),
            (parent.participant_slot, parent.cols.allocated_flow_col),
            (parent.participant_slot, parent.cols.weight_sum_col),
        ],
        "PARAM(1) is the exact level-N AllocatedFlow cell, not an intermediary",
    );

    let propagated = [
        cols.propagated_intrinsic_flow_col,
        cols.propagated_allocated_flow_col,
        cols.propagated_weight_sum_col,
    ];
    for op in &plan.cpu_ops {
        assert!(
            !op.targets
                .iter()
                .any(|(_, column)| propagated.contains(column)),
            "the resident plan must not materialize a propagated economic copy"
        );
        match &op.source {
            SourceSpec::SlotValue { col, .. } | SourceSpec::SlotRange { col, .. } => assert!(
                !propagated.contains(col),
                "the resident plan must not read a propagated economic copy"
            ),
            SourceSpec::ConjunctiveCrossing { inputs } => assert!(
                !inputs.iter().any(|input| propagated.contains(&input.col)),
                "the resident plan must not gather a propagated economic copy"
            ),
            SourceSpec::Constant(_) => {}
        }
    }
}

fn run_gpu_chain(layout: &ArenaTreeLayout, plant_propagated_copy_defect: bool) -> Option<Vec<u32>> {
    let ctx = match GpuContext::new_blocking() {
        Ok(ctx) => ctx,
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("resident clearing 14.3 requires the selected GPU adapter")
        }
        Err(_) => return None,
    };
    let mut plan = plan_arena_allocation(layout, &[], 256).expect("resident allocator plan");
    if plant_propagated_copy_defect {
        let parent = &layout.participant_roots[0].children[0];
        let grandchild = &parent.children[0];
        let op = plan
            .cpu_ops
            .iter_mut()
            .find(|op| {
                op.targets
                    == vec![(
                        grandchild.participant_slot,
                        grandchild.cols.allocated_flow_col,
                    )]
                    && matches!(op.combine, simthing_core::CombineFn::EvalEML { .. })
            })
            .expect("grandchild disbursement");
        let SourceSpec::ConjunctiveCrossing { inputs } = &mut op.source else {
            panic!("direct child-share input list");
        };
        inputs[1] = InputSpec {
            slot: parent.participant_slot,
            col: parent.cols.propagated_allocated_flow_col,
            unit_cost: 1.0,
        };
    }

    let mut registry = EmlExpressionRegistry::new();
    register_child_share_formula(&mut registry, cols()).expect("child-share formula registration");
    let upload_rows: Vec<_> = registry
        .formulas_for_gpu_upload()
        .map(|(id, meta, nodes)| {
            (
                id,
                meta.clone(),
                nodes
                    .iter()
                    .map(|node| EmlNodeGpu {
                        opcode: node.opcode,
                        flags: node.flags,
                        a: node.a,
                        b: node.b,
                        c: node.c,
                        d: node.d,
                    })
                    .collect(),
            )
        })
        .collect();
    let mut table = EmlGpuProgramTable::new(&ctx, 64, 4);
    let uploaded = table
        .upload_trees(&ctx, &upload_rows)
        .expect("resident child-share EML upload");
    for (tree_id, range_index) in uploaded {
        registry
            .mark_tree_uploaded(tree_id, range_index, table.generation)
            .expect("uploaded tree binding");
    }
    let upload = PackedAccumulatorUpload::from_ops_resolving_input_lists_with_eml(
        &plan.cpu_ops,
        Some(&registry),
    )
    .expect("packed resident input-list plan");

    let n_slots = layout
        .iter_all()
        .iter()
        .map(|node| node.participant_slot.raw())
        .max()
        .unwrap()
        + 1;
    let n_dims = 9u32;
    let mut values = vec![0.0f32; (n_slots * n_dims) as usize];
    let index =
        |slot: SlotIndex, column: ColumnIndex| (slot.raw() * n_dims + column.raw_u32()) as usize;
    let root = &layout.participant_roots[0];
    let child = &root.children[0];
    let grandchild = &child.children[0];
    values[index(root.participant_slot, root.cols.intrinsic_flow_col)] = 8.0;
    values[index(child.participant_slot, child.cols.weight_col)] = 1.0;
    values[index(grandchild.participant_slot, grandchild.cols.weight_col)] = 1.0;

    let mut session = AccumulatorOpSession::new_attached(&ctx, n_slots, n_dims, 4);
    session.upload_values(&ctx, &values);
    session.copy_values_to_previous(&ctx);
    session
        .upload_packed_ops(&ctx, &upload)
        .expect("resident ops upload");
    for band in 0..plan.n_bands {
        session
            .tick_with_eml(&ctx, band, Some(&table))
            .expect("resident OrderBand dispatch");
    }
    let observed = session.readback_full(&ctx).expect("proof readback");
    let direct_bits = vec![
        observed[index(child.participant_slot, child.cols.allocated_flow_col)].to_bits(),
        observed[index(
            grandchild.participant_slot,
            grandchild.cols.allocated_flow_col,
        )]
        .to_bits(),
    ];

    // The residual closure now consumes child AllocatedFlow directly too. Seal
    // the generic Sum scale on the same live session: no per-child propagated
    // negative cell may be materialized merely to subtract the child total.
    let residual = AccumulatorOp {
        source: SourceSpec::SlotRange {
            start: child.participant_slot,
            count: root.children.len() as u32,
            col: child.cols.allocated_flow_col,
        },
        combine: CombineFn::Sum,
        gate: GateSpec::OrderBand(0),
        scale: ScaleSpec::Constant(-1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(root.participant_slot, root.cols.weight_sum_col)],
    };
    let residual_upload =
        PackedAccumulatorUpload::from_ops(&[residual]).expect("scaled residual Sum upload");
    session
        .upload_packed_ops(&ctx, &residual_upload)
        .expect("scaled residual ops upload");
    session.tick(&ctx, 0).expect("scaled residual Sum dispatch");
    let residual_observed = session
        .readback_full(&ctx)
        .expect("residual proof readback");

    Some(vec![
        direct_bits[0],
        direct_bits[1],
        residual_observed[index(root.participant_slot, root.cols.weight_sum_col)].to_bits(),
    ])
}

#[test]
fn allocated_flow_is_direct_recursive_evaleml_input_and_gpu_self_consumes() {
    let compact = d3_layout([0, 1, 2]);
    let rebound = d3_layout([129, 65, 3]);
    let wide = wide_d3_layout();
    assert_direct_recursive_shape(&compact);
    assert_direct_recursive_shape(&rebound);
    assert_direct_recursive_shape(&wide);

    let mut oracle_values = HashMap::from([
        ((SlotIndex::new(0), cols().intrinsic_flow_col), 8.0),
        ((SlotIndex::new(1), cols().weight_col), 1.0),
        ((SlotIndex::new(2), cols().weight_col), 1.0),
    ]);
    run_arena_allocation_oracle(&compact, &mut oracle_values, 1.0);
    assert_eq!(
        oracle_values[&(SlotIndex::new(1), cols().allocated_flow_col)].to_bits(),
        8.0f32.to_bits()
    );
    assert_eq!(
        oracle_values[&(SlotIndex::new(2), cols().allocated_flow_col)].to_bits(),
        8.0f32.to_bits(),
        "level-N AllocatedFlow must become level-N+1 allocator input"
    );

    let Some(compact_bits) = run_gpu_chain(&compact, false) else {
        eprintln!("SKIP: no GPU adapter for resident clearing 14.3 direct-chain witness");
        return;
    };
    let rebound_bits = run_gpu_chain(&rebound, false).expect("same selected adapter");
    let wide_bits = run_gpu_chain(&wide, false).expect("same selected adapter");
    assert_eq!(
        compact_bits,
        vec![8.0f32.to_bits(), 8.0f32.to_bits(), (-8.0f32).to_bits()]
    );
    assert_eq!(rebound_bits, compact_bits, "epoch row rebind is bit-exact");
    assert_eq!(
        wide_bits, compact_bits,
        "one-workgroup and multi-workgroup dispatch cardinalities are bit-exact"
    );
    assert_ne!(
        run_gpu_chain(&compact, true).expect("same selected adapter"),
        compact_bits,
        "planted propagated-copy intermediary must RED"
    );
}

fn scope(name: &str) -> OwnerChannelScopeKey {
    OwnerChannelScopeKey {
        owner_ref: OwnerRef::new("owner/7"),
        resource_key: ResourceKey::new("resource/food"),
        scope_id: ScopeId::new(name),
    }
}

fn demand(
    scope: OwnerChannelScopeKey,
    source: u32,
    requested: u32,
) -> RuntimeOwnerSiloDemandBucket {
    RuntimeOwnerSiloDemandBucket {
        owner_ref: scope.owner_ref,
        resource_key: scope.resource_key,
        scope_id: scope.scope_id,
        requested,
        priority: 1,
        source_simthing_id_raw: Some(source),
    }
}

fn claim(demand: &RuntimeOwnerSiloDemandBucket) -> ConstrainedClaim {
    ConstrainedClaim::from_runtime_demand(demand, 1.0).expect("ordinary runtime demand claim")
}

#[test]
fn unresolved_demand_recurs_once_at_n_plus_one_and_drains_without_authored_path() {
    let n = GenerationStamp::new(10);
    let key = scope("scope/a");
    let current_demand = demand(key.clone(), 41, 10);
    let current = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: key.clone(),
            available: 4,
        }],
        &[claim(&current_demand)],
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: n,
        },
    )
    .expect("generation-N clear");
    let grant = &current[0].grants[0];
    assert_eq!(
        (grant.requested, grant.granted, grant.unresolved),
        (10, 4, 6)
    );
    let unresolved =
        UnresolvedDemandObservation::from_grant(grant, n).expect("u>0 ordinary observation");

    let next_authored = demand(key.clone(), 41, 2);
    let carried = carry_unresolved_demand_to_next_generation(
        n,
        next_authored.clone(),
        Some(unresolved.clone()),
    )
    .expect("neutral Current-to-Next carry");
    assert_eq!(carried.generation(), GenerationStamp::new(11));
    assert_eq!(carried.product().requested, 8, "d' + u exactly once");
    assert_eq!(current_demand.requested, 10, "generation N is unchanged");
    assert_eq!(grant.unresolved, 6, "generation N is not re-cleared");
    assert_eq!(
        carried.product().requested,
        next_authored.requested + unresolved.unresolved,
        "the parent sees the recurrent demand once"
    );
    assert_ne!(
        carried.product().requested,
        next_authored.requested + 2 * unresolved.unresolved,
        "a double-carry mutant must RED"
    );

    let next = clear_constrained_claims_at_generation(
        &[ConstrainedSupply {
            scope: key,
            available: 8,
        }],
        &[claim(carried.product())],
        &AuthoredClearingProgram::new(TransformOp::set(0.0)),
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(7),
            generation: carried.generation(),
        },
    )
    .expect("ordinary generation-N+1 clear");
    assert_eq!(next[0].grants[0].unresolved, 0, "new supply drains u");
    assert!(
        UnresolvedDemandObservation::from_grant(&next[0].grants[0], carried.generation()).is_none()
    );

    assert!(
        carry_unresolved_demand_to_next_generation(
            GenerationStamp::new(9),
            next_authored.clone(),
            Some(unresolved)
        )
        .is_err(),
        "same/incorrect-generation recurrence must fail closed"
    );

    let zero_control = carry_unresolved_demand_to_next_generation(n, next_authored.clone(), None)
        .expect("u=0 negative control");
    assert_eq!(
        zero_control.product(),
        &next_authored,
        "u=0 leaves the established demand product bit-exact"
    );
}

fn canonical_snapshot(
    results: &[ConstrainedClearingResult],
) -> Vec<(String, Vec<(u32, u32, u32)>)> {
    results
        .iter()
        .map(|result| {
            (
                result.scope.scope_id.as_str().to_owned(),
                result
                    .grants
                    .iter()
                    .map(|grant| {
                        (
                            grant.source_simthing_id.raw(),
                            grant.granted,
                            grant.clearing_score.to_bits(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn scheduled_order(rows: &[(usize, u32, u32)], workgroup: u32, partitions: u32) -> Vec<usize> {
    let mut scheduled = rows.to_vec();
    scheduled.sort_by_key(|&(logical, physical_row, segment)| {
        (
            (physical_row / workgroup) % partitions,
            segment,
            physical_row % workgroup,
            logical,
        )
    });
    scheduled.into_iter().map(|row| row.0).collect()
}

#[test]
fn canonical_clearing_ignores_physical_order_and_atomic_arrival_mutant_reds() {
    let a = scope("scope/a");
    let b = scope("scope/b");
    let demands = vec![
        demand(a.clone(), 11, 1),
        demand(a.clone(), 12, 1),
        demand(b.clone(), 21, 2),
        demand(b.clone(), 22, 2),
    ];
    let program = AuthoredClearingProgram::new(TransformOp::set(1.0));
    let authority = ClearingRemainderAuthority {
        granter: SimThingId::from_session_raw(5),
        generation: GenerationStamp::new(17),
    };
    let supplies = vec![
        ConstrainedSupply {
            scope: a,
            available: 1,
        },
        ConstrainedSupply {
            scope: b,
            available: 2,
        },
    ];
    let variants = [
        (vec![(0, 0, 0), (1, 1, 0), (2, 2, 1), (3, 3, 1)], 32, 1),
        (vec![(0, 130, 1), (1, 2, 0), (2, 67, 3), (3, 33, 2)], 64, 3),
        (vec![(0, 7, 3), (1, 129, 2), (2, 1, 1), (3, 65, 0)], 16, 4),
    ];
    let mut snapshots = Vec::new();
    let mut arrivals = Vec::new();
    for (index, (rows, workgroup, partitions)) in variants.iter().enumerate() {
        let order = scheduled_order(rows, *workgroup, *partitions);
        arrivals.push(order.clone());
        let claims: Vec<_> = order.iter().map(|&row| claim(&demands[row])).collect();
        let mut local_supplies = supplies.clone();
        if index % 2 == 1 {
            local_supplies.reverse();
        }
        let result =
            clear_constrained_claims_at_generation(&local_supplies, &claims, &program, authority)
                .expect("canonical clear under physical perturbation");
        snapshots.push(canonical_snapshot(&result));
    }
    assert!(snapshots.windows(2).all(|pair| pair[0] == pair[1]));
    for (_, grants) in &snapshots[0] {
        assert!(
            grants.windows(2).all(|pair| pair[0].0 < pair[1].0),
            "hard total order is claimant logical identity within an equality band"
        );
        assert!(
            grants.iter().all(|grant| grant.2 == 1.0f32.to_bits()),
            "EML score bits and equality-band segmentation match the CPU oracle"
        );
    }

    // Planted atomic-append resolver: the first arrival in the exact tie wins.
    // The two physical schedules choose different logical claimants, proving
    // that physical arrival cannot be an admitted tie authority.
    let planted_winner = |order: &[usize]| demands[order[0]].source_simthing_id_raw.unwrap();
    assert_ne!(
        planted_winner(&arrivals[0]),
        planted_winner(&arrivals[1]),
        "planted atomic append-order tie resolver must RED"
    );
}
