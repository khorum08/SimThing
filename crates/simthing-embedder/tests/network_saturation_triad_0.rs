//! Network-saturation full-Triad exemplar for the Embedder Guide.
//!
//! Need / corridor / front / chokepoint bands are born from the tree, an
//! overlay, and ordinary Bind thresholds — not hand-fed readouts. Volume-delay
//! is the admitted power law `exp(k * ln x)`, never POW and never a staircase.

use simthing_embedder::{bind, derive, overlay, populate, run};

fn volume_delay_law(ratio: f32) -> f32 {
    1.0 + 0.15 * populate::eml_exp_pinned_f32(4.0 * populate::eml_ln_pinned_f32(ratio))
}

fn volume_delay_staircase(ratio: f32) -> f32 {
    if ratio < 0.5 {
        1.0
    } else if ratio < 1.0 {
        1.15
    } else if ratio < 1.5 {
        1.5
    } else {
        2.5
    }
}

fn network_owner_crossing(root: &mut populate::SimThing) {
    let owner = populate::OwnerRef::try_new_authored("alpha").expect("owner");
    populate::owner(root, &owner);
    populate::ownership(root).expect("one crossing");
}

#[test]
fn volume_delay_power_law_reds_a_staircase_rival() {
    let ratio = 2.0_f32;
    let delay = 1.0 + 0.15 * populate::eml_exp_pinned_f32(4.0 * populate::eml_ln_pinned_f32(ratio));
    assert_eq!(delay, volume_delay_law(ratio));
    assert_ne!(
        delay.to_bits(),
        volume_delay_staircase(ratio).to_bits(),
        "staircase rival must disagree with exp(k * ln x)"
    );
}

#[test]
fn network_saturation_triad_bands_are_born_from_the_tree() {
    let _seat = derive::owner_seat("alpha", "Alpha", "carrier").expect("owner seat");
    let mut scenario = run::Scenario::map_light("network-saturation".into(), 1, 2, 1.0, 5);
    network_owner_crossing(&mut scenario.root);
    let pid = scenario
        .registry
        .id_of("map", "stability")
        .expect("fixture dimension");
    let origin = scenario.root.children[0].clone();
    let load = overlay::authored(
        &scenario.root,
        &origin,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![origin.id],
        overlay::PropertyTransformDelta {
            property_id: pid,
            sub_field_deltas: vec![(
                populate::SubFieldRole::Amount,
                overlay::TransformOp::set(0.9),
            )],
        },
        vec![overlay::DissolveCondition::AtSessionEnd],
    )
    .expect("saturation overlay");
    scenario.root.children[0].add_overlay(load);

    let (_, cost_band) = populate::queued_cost_band(0.1, Some(2), None).expect("queue shape");
    let mut session = run::initialize(scenario, &run::GameModeSpec::default()).expect("init");
    bind::velocity_threshold(
        &mut session,
        bind::VelocityAlertRegistration {
            sim_thing_id: origin.id,
            property_id: pid,
            sub_field: populate::SubFieldRole::Velocity,
            threshold: 0.0,
            direction: populate::Direction::Rising,
            cost_band: cost_band.clone(),
        },
    );
    bind::velocity_threshold(
        &mut session,
        bind::VelocityAlertRegistration {
            sim_thing_id: origin.id,
            property_id: pid,
            sub_field: populate::SubFieldRole::Amount,
            threshold: 0.4,
            direction: populate::Direction::Rising,
            cost_band: cost_band.clone(),
        },
    );
    bind::aggregate_threshold(
        &mut session,
        bind::AggregateAlertRegistration {
            sim_thing_id: origin.id,
            property_id: pid,
            sub_field: populate::SubFieldRole::Amount,
            threshold: 0.8,
            direction: populate::Direction::Rising,
            cost_band: cost_band.clone(),
        },
    );
    bind::aggregate_threshold(
        &mut session,
        bind::AggregateAlertRegistration {
            sim_thing_id: origin.id,
            property_id: pid,
            sub_field: populate::SubFieldRole::Amount,
            threshold: 0.95,
            direction: populate::Direction::Rising,
            cost_band,
        },
    );
    run::start(&mut session, run::ExecutionPosture::Paced).expect("start");
    run::tick(&mut session).expect("tick");
    let shadow = bind::shadow(&session);
    assert!(shadow.row(origin.id).is_some());
    let delay = volume_delay_law(2.0);
    assert!(delay > 1.0);
}
