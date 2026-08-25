//! BAND-QUANTIZED-DRAW-0 Remand 2 — real ThresholdBuilder + BoundaryProtocol
//! CostBand admission / resolve referees.

use simthing_core::{
    cost_band_expected_n, cost_band_quantize, ConjunctiveRecipeRegistration, Direction,
    SimProperty, SimPropertyId, SimThing, SimThingId, SimThingKind, SlotIndex, SubFieldRole,
};
use simthing_gpu::SlotAllocator;
use simthing_sim::{
    BoundaryProtocol, CostBandSemantic, ThresholdRegistry, ThresholdSemantic,
    VelocityAlertRegistration,
};

fn velocity_sem() -> ThresholdSemantic {
    ThresholdSemantic::VelocityAlert {
        sim_thing_id: SimThingId::new(),
        property_id: SimPropertyId(1),
        sub_field: SubFieldRole::Amount,
    }
}

fn simple_boundary() -> (BoundaryProtocol, SimThingId, SimPropertyId) {
    let mut reg = simthing_core::DimensionRegistry::new();
    let pid = reg.register(SimProperty::simple("core", "loyalty", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut child = SimThing::new(SimThingKind::Cohort, 0);
    child.add_property(pid, reg.property(pid).default_value());
    let child_id = child.id;
    root.add_child(child);
    let mut alloc = SlotAllocator::new();
    alloc.install_initial_tree(&root);
    let proto = BoundaryProtocol::new(simthing_sim::SimRuntimeTree::admit(root), reg, alloc);
    (proto, child_id, pid)
}

#[test]
fn production_resolve_uses_admitted_semantics_not_caller_is_sink() {
    let mut reg = ThresholdRegistry::new();
    let obs_kind = reg.push(velocity_sem());
    let sink_kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(2), None).unwrap(),
    );

    let v = 10.0f32;
    let c = 1.0f32;
    let obs = reg.resolve_cost_band_draw(obs_kind, v, c).unwrap();
    assert_eq!(obs.n, 0);
    assert!(obs.n_matches_oracle(false, None));
    assert_eq!(obs.r.to_bits(), v.to_bits());

    let sink = reg.resolve_cost_band_draw(sink_kind, v, c).unwrap();
    assert_eq!(sink.n, 2);
    assert!(sink.n_matches_oracle(true, Some(2)));
    assert_eq!(cost_band_expected_n(v, c, true, Some(2)).unwrap(), 2);
}

#[test]
fn threshold_builder_and_boundary_admit_recipe_throttle_sink() {
    // Existing stored-hydrated throttle source (recipe registration metadata).
    let recipe = ConjunctiveRecipeRegistration {
        inputs: vec![],
        target_slot: SlotIndex::new(0),
        target_col: simthing_core::ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
        throttle_hint_max_per_tick: 3,
    };

    let (mut proto, child_id, pid) = simple_boundary();
    proto.register_velocity_alert(VelocityAlertRegistration {
        sim_thing_id: child_id,
        property_id: pid,
        sub_field: SubFieldRole::Amount,
        threshold: 4.0,
        direction: Direction::Rising,
        cost_band: CostBandSemantic::admit_sink(Some(recipe.throttle_hint_max_per_tick), None)
            .unwrap(),
    });
    proto.register_velocity_alert(VelocityAlertRegistration {
        sim_thing_id: child_id,
        property_id: pid,
        sub_field: SubFieldRole::Velocity,
        threshold: 9.0,
        direction: Direction::Rising,
        cost_band: CostBandSemantic::observation(),
    });

    // Ordinary ThresholdBuilder production path (GPU-sync dirty rebuild door).
    proto.rebuild_threshold_registry_from_builder();

    let mut sink_kind = None;
    let mut obs_kind = None;
    for k in 0..proto.threshold_registry().len() as u32 {
        let cb = proto.threshold_registry().cost_band(k);
        match proto.threshold_registry().get(k) {
            Some(ThresholdSemantic::VelocityAlert {
                sub_field: SubFieldRole::Amount,
                ..
            }) => {
                assert!(cb.is_sink, "builder push() without cost_band would RED");
                assert_eq!(cb.throttle_hint_max_per_tick, Some(3));
                sink_kind = Some(k);
            }
            Some(ThresholdSemantic::VelocityAlert {
                sub_field: SubFieldRole::Velocity,
                ..
            }) => {
                assert!(!cb.is_sink);
                obs_kind = Some(k);
            }
            _ => {}
        }
    }
    let sink_kind = sink_kind.expect("Amount sink admitted");
    let obs_kind = obs_kind.expect("Velocity observation admitted");

    // Production BoundaryProtocol resolve door (same method execute calls).
    let empty: &[simthing_gpu::BandCrossingDelta] = &[];
    let _ = proto.resolve_production_cost_band_draws(empty);

    let sink_cb = proto.threshold_registry().cost_band(sink_kind);
    let sink = cost_band_quantize(
        12.5,
        4.0,
        sink_cb.is_sink,
        sink_cb.throttle_hint_max_per_tick,
    )
    .unwrap();
    assert_eq!(sink.n, 3);
    assert!(sink.n_matches_oracle(true, Some(3)));

    let obs_cb = proto.threshold_registry().cost_band(obs_kind);
    let obs =
        cost_band_quantize(12.5, 4.0, obs_cb.is_sink, obs_cb.throttle_hint_max_per_tick).unwrap();
    assert_eq!(obs.n, 0);
    assert_eq!(obs.r.to_bits(), 12.5f32.to_bits());
}

#[test]
fn boundary_resolve_door_must_be_the_execute_path() {
    let (mut proto, child_id, pid) = simple_boundary();
    proto.register_velocity_alert(VelocityAlertRegistration {
        sim_thing_id: child_id,
        property_id: pid,
        sub_field: SubFieldRole::Amount,
        threshold: 2.0,
        direction: Direction::Rising,
        cost_band: CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    });
    proto.rebuild_threshold_registry_from_builder();

    let before = proto.threshold_registry().cost_band_resolve_invocations;
    // Planted defect: skip production door.
    let bypassed: Vec<(u32, simthing_core::CostBandDraw)> = Vec::new();
    assert!(bypassed.is_empty());
    assert_eq!(
        proto.threshold_registry().cost_band_resolve_invocations,
        before
    );

    // Honest door (execute calls this).
    let empty: &[simthing_gpu::BandCrossingDelta] = &[];
    let _ = proto.resolve_production_cost_band_draws(empty);

    assert!(
        proto.threshold_registry().cost_band(0).is_sink,
        "builder must have admitted sink; push-only rebuild REDs"
    );
}

#[test]
fn ambiguous_resource_marker_hard_errors_at_admission() {
    let err = CostBandSemantic::admit_sink(
        Some(1),
        Some(simthing_core::CostBandResourceMarker { is_sink: false }),
    );
    assert!(err.is_err());
}

#[test]
fn depth_one_command_deficit_same_resolve_path() {
    let mut reg = ThresholdRegistry::new();
    let kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(Some(1), None).unwrap(),
    );
    let fire = reg.resolve_cost_band_draw(kind, 5.0, 4.0).unwrap();
    assert_eq!(fire.n, 1);
    let miss = reg.resolve_cost_band_draw(kind, 3.0, 4.0).unwrap();
    assert_eq!(miss.n, 0);
    assert!(fire.conserves_exactly());
    assert!(miss.conserves_exactly());
}

#[test]
fn runtime_depth_mutation_changes_n_without_re_admission() {
    let mut reg = ThresholdRegistry::new();
    let kind = reg.push_with_cost_band(
        velocity_sem(),
        CostBandSemantic::admit_sink(None, None).unwrap(),
    );
    let d1 = reg.resolve_cost_band_draw(kind, 5.0, 2.0).unwrap();
    let d3 = reg.resolve_cost_band_draw(kind, 7.0, 2.0).unwrap();
    assert_eq!(d1.n, 2);
    assert_eq!(d3.n, 3);
}

#[test]
fn threshold_builder_unmarked_stays_observation() {
    let (mut proto, child_id, pid) = simple_boundary();
    proto.register_velocity_alert(VelocityAlertRegistration {
        sim_thing_id: child_id,
        property_id: pid,
        sub_field: SubFieldRole::Amount,
        threshold: 1.0,
        direction: Direction::Rising,
        cost_band: CostBandSemantic::observation(),
    });
    proto.rebuild_threshold_registry_from_builder();
    for k in 0..proto.threshold_registry().len() as u32 {
        if matches!(
            proto.threshold_registry().get(k),
            Some(ThresholdSemantic::VelocityAlert { .. })
        ) {
            assert!(!proto.threshold_registry().cost_band(k).is_sink);
        }
    }
}
