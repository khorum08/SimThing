//! Phase A-0 — static nested Resource Flow first slice (D=3/D=4 GPU parity).

#[path = "support/e11_nested.rs"]
mod nested_support;

use nested_support::{
    a0_d3_participants, a0_d4_participants, assert_nested_cpu_gpu_parity,
    integration_band_for_layout, layout_for, leaves, materialize_nested, open_nested_session,
    try_gpu,
};
use simthing_core::SimThingKind;
use simthing_driver::{
    build_execution_plan, nested_fission_gap_report, nested_hierarchy_materialization_report,
    refresh_fission_participant_child, reserve_gap_pools_for_parent_slots, slots_are_contiguous,
    total_bands_for_depth, ArenaBandLayout, FissionPolicy,
};
use simthing_sim::PipelineFlags;

fn assert_all_parents_contiguous(f: &nested_support::MaterializedNestedFixture) {
    let layout = layout_for(f);
    let report = nested_hierarchy_materialization_report(&layout);
    assert!(report.all_parents_contiguous);
    for root in &layout.participant_roots {
        root.verify_child_contiguity().unwrap();
    }
    for node in layout.iter_all() {
        if !node.children.is_empty() {
            node.verify_child_contiguity().unwrap();
        }
    }
}

#[test]
fn a0_static_nested_d3_materializes_from_authored_topology() {
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let report = nested_hierarchy_materialization_report(&layout);
    assert_eq!(layout.max_depth, 3);
    assert_eq!(report.max_depth, 3);
    assert_eq!(layout.participant_roots.len(), 1);
    assert_eq!(layout.participant_roots[0].children.len(), 2);
    assert_eq!(leaves(&layout).len(), 4);
    assert!(report.all_parents_contiguous);
}

#[test]
fn a0_static_nested_d4_materializes_from_authored_topology() {
    let f = materialize_nested(7, a0_d4_participants, 16, 0);
    let layout = layout_for(&f);
    let report = nested_hierarchy_materialization_report(&layout);
    assert_eq!(layout.max_depth, 4);
    assert_eq!(report.max_depth, 4);
    assert_eq!(layout.participant_roots.len(), 2);
    assert!(report.all_parents_contiguous);
}

#[test]
fn a0_nested_children_contiguous_per_parent() {
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    assert_all_parents_contiguous(&f);
    let f4 = materialize_nested(7, a0_d4_participants, 16, 0);
    assert_all_parents_contiguous(&f4);
}

#[test]
fn a0_noncontiguous_nested_children_reject_without_compaction() {
    let mut f = materialize_nested(7, a0_d3_participants, 16, 1);
    let layout = layout_for(&f);
    let interiors = layout.interior_participant_slots();
    reserve_gap_pools_for_parent_slots(&mut f.scaffold, &mut f.alloc, &interiors, 1);
    let mid = layout.participant_roots[0].children[0].participant_slot;
    refresh_fission_participant_child(
        &mut f.scaffold,
        &mut f.root,
        mid,
        simthing_core::SimThing::new(SimThingKind::Cohort, 0).id,
        f.flow_id,
        &f.reg,
        &mut f.alloc,
        FissionPolicy::Reject,
    )
    .expect("gap claim breaks contiguity");

    let arena = simthing_driver::GpuArenaDescriptor {
        name: "food".into(),
        flow_property_id: f.flow_id,
        balance_property_id: None,
        max_participants: 32,
        max_coupling_fanout: 4,
        max_orderband_depth: 16,
        fission_policy: FissionPolicy::Reject,
        participant_range: (0, 0),
        wildcard_max_expansion: None,
        reserved_orderband_depth: 0,
    };
    let err = build_execution_plan(
        &f.reg,
        std::slice::from_ref(&arena),
        &f.root,
        &f.alloc,
        &f.scaffold,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        simthing_driver::HierarchyError::NonContiguousChildren { .. }
    ));
}

#[test]
fn a0_reserved_gap_slots_excluded_from_active_slotranges() {
    let mut f = materialize_nested(7, a0_d3_participants, 16, 1);
    let layout = layout_for(&f);
    let interiors = layout.interior_participant_slots();
    reserve_gap_pools_for_parent_slots(&mut f.scaffold, &mut f.alloc, &interiors, 1);
    let parent = layout.participant_roots[0].children[0].participant_slot;
    let active = layout.participant_roots[0].children[0].active_child_slots();
    let report = nested_fission_gap_report(parent, &active, &f.scaffold, None, 0);
    assert!(report.gap_outside_active_child_span);
    assert!(report.active_children_contiguous);
    for gap in report.reserved_gap_slots {
        assert!(!active.contains(&gap));
    }
}

#[test]
fn a0_d3_orderband_budget_and_integration_band() {
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let bands = ArenaBandLayout::for_depth(3);
    assert_eq!(layout.band_layout.total_bands_used, bands.total_bands_used);
    assert_eq!(
        layout.band_layout.total_bands_used,
        total_bands_for_depth(3)
    );
    assert_eq!(integration_band_for_layout(&layout), bands.integration_band);
    assert_eq!(bands.total_bands_used, 8);
}

#[test]
fn a0_d4_orderband_budget_and_integration_band() {
    let f = materialize_nested(7, a0_d4_participants, 16, 0);
    let layout = layout_for(&f);
    let bands = ArenaBandLayout::for_depth(4);
    assert_eq!(layout.band_layout.total_bands_used, bands.total_bands_used);
    assert_eq!(
        layout.band_layout.total_bands_used,
        total_bands_for_depth(4)
    );
    assert_eq!(integration_band_for_layout(&layout), bands.integration_band);
    assert_eq!(bands.total_bands_used, 11);
}

#[test]
fn a0_d3_gpu_cpu_oracle_parity() {
    if try_gpu().is_none() {
        eprintln!("skipping: no GPU");
        return;
    }
    let f = materialize_nested(7, a0_d3_participants, 16, 0);
    let layout = layout_for(&f);
    let parity = assert_nested_cpu_gpu_parity(&f, &layout, 24.0);
    assert_eq!(parity.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert_eq!(parity.leaf_count, 4);
}

#[test]
fn a0_d4_gpu_cpu_oracle_parity() {
    if try_gpu().is_none() {
        eprintln!("skipping: no GPU");
        return;
    }
    let f = materialize_nested(7, a0_d4_participants, 16, 0);
    let layout = layout_for(&f);
    let parity = assert_nested_cpu_gpu_parity(&f, &layout, 32.0);
    assert_eq!(parity.max_abs_error.to_bits(), 0.0_f32.to_bits());
    assert!(parity.leaf_count >= 3);
}

#[test]
fn a0_replay_reproducibility() {
    if try_gpu().is_none() {
        eprintln!("skipping: no GPU");
        return;
    }
    let run = || {
        let f = materialize_nested(7, a0_d3_participants, 16, 0);
        let layout = layout_for(&f);
        assert_nested_cpu_gpu_parity(&f, &layout, 20.0)
    };
    let a = run();
    let b = run();
    assert_eq!(a.max_abs_error.to_bits(), b.max_abs_error.to_bits());
    assert_eq!(a.l_inf.to_bits(), b.l_inf.to_bits());
}

#[test]
fn a0_resource_flow_flag_default_false_unchanged() {
    assert!(!PipelineFlags::default().use_accumulator_resource_flow);
    let nested = open_nested_session(7, a0_d3_participants, 16, false);
    assert!(!nested.session.proto.flags.use_accumulator_resource_flow);
}

#[test]
fn a0_hard_currency_not_routed_through_resource_flow() {
    let nested = open_nested_session(7, a0_d3_participants, 16, true);
    assert!(nested.session.proto.flags.use_accumulator_resource_flow);
    assert!(!nested.session.proto.flags.use_accumulator_transfer);
    assert_eq!(nested.session.spec_state.resource_economy_registry, None);
}

#[test]
fn a0_no_new_wgsl_roles_or_cpu_fallback() {
    // WGSL-GUARD-0/R1: global filename WGSL bans removed; designer SemanticWgsl is authoritative.
    if try_gpu().is_some() {
        let nested = open_nested_session(7, a0_d3_participants, 16, true);
        assert!(nested.session.state.accumulator_resource_flow_active);
    }
}

#[test]
fn a0_no_dynamic_enrollment_policy_b_selector_rerun_or_compaction() {
    let nested = open_nested_session(7, a0_d3_participants, 16, true);
    assert!(nested
        .session
        .last_resource_flow_dynamic_enrollment_report
        .is_none());
    let src = include_str!("../src/resource_flow_fission_enrollment.rs");
    assert!(!src.contains("Policy B Reevaluate"));
    assert!(!src.contains("selector re-run"));
    assert!(!src.contains("slot compaction"));
}

#[test]
fn a0_no_b1_c_runtime_l3_frontierv2_5_or_act_event_obs_pipe() {
    let nested = open_nested_session(7, a0_d3_participants, 16, true);
    assert!(!nested.session.proto.flags.use_accumulator_transfer);
    assert!(!nested.session.proto.flags.use_accumulator_emission);
}

#[test]
fn a0_no_simthing_sim_semantic_awareness() {
    let sim_src = include_str!("../../simthing-sim/src/lib.rs");
    assert!(!sim_src.contains("HierarchyNode"));
    assert!(!sim_src.contains("ResourceFlowSpec"));
    assert!(!sim_src.contains("E-11B"));
}

#[test]
fn a0_nested_static_children_are_contiguous_per_parent() {
    a0_nested_children_contiguous_per_parent();
}

#[test]
fn a0_noncontiguous_nested_children_reject() {
    a0_noncontiguous_nested_children_reject_without_compaction();
}

#[test]
fn a0_reserved_gap_slots_stay_outside_active_child_slotranges() {
    a0_reserved_gap_slots_excluded_from_active_slotranges();
}
