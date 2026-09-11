//! Full native/cache RF source exercised through the ordinary Studio session.
use simthing_core::SubFieldRole;
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot};
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::studio_live_session_bridge::{
    driver_scenario_field_bearing_from_profile, field_bearing_game_mode, StudioAuthoredLiveProfile,
};
use simthing_mapeditor::{
    load_studio_session_from_scenario_path, StudioLiveSessionBridge, StudioSession,
};
use simthing_spec::PropertyKey;
use std::{collections::BTreeMap, path::Path};

fn observe(session: &StudioSession) -> BTreeMap<String, f32> {
    let profile = session.authored_live_profile.as_ref().unwrap();
    let policies: Vec<_> = profile
        .game_mode
        .overlays
        .iter()
        .map(|overlay| {
            (
                &overlay.id,
                overlay.sub_field_deltas[0].1.as_multiply_literal(),
            )
        })
        .collect();
    eprintln!("authored policy multipliers: {policies:?}");
    let mut bridge = StudioLiveSessionBridge::default();
    bridge.open_from_loaded_studio_session(session).unwrap();
    let sim = bridge.sim_session().unwrap();
    let pid = sim.proto.registry.id_of("ingress_rf", "energy").unwrap();
    let parents: BTreeMap<_, _> = sim
        .spec_state
        .resource_flow_derivation
        .arenas
        .iter()
        .find(|arena| arena.arena == "native_energy")
        .unwrap()
        .participants
        .iter()
        .map(|participant| (participant.simthing_id, participant.parent))
        .collect();
    assert_eq!(parents.len(), 19);
    assert_eq!(
        parents[&profile.install_targets["alpha_3"][0]],
        Some(profile.install_targets["alpha_site"][0])
    );
    assert_eq!(
        parents[&profile.install_targets["beta_3"][0]],
        Some(profile.install_targets["beta_2"][0])
    );
    for owner in ["alpha", "beta"] {
        assert_eq!(
            sim.proto
                .root
                .child_count(profile.install_targets[owner][0]),
            Some(0),
            "owner seats never contain spatial participants"
        );
        assert_eq!(
            sim.proto
                .root
                .overlay_count(profile.install_targets[owner][0]),
            Some(1)
        );
        for index in 0..7 {
            let id = profile.install_targets[&format!("{owner}_{index}")][0];
            assert_eq!(sim.proto.root.owner_of(id).unwrap().as_str(), owner);
        }
        let zero = profile.install_targets[&format!("{owner}_4")][0];
        assert_eq!(
            sim.proto
                .root
                .property_on_node(zero, pid)
                .unwrap()
                .get_role(
                    &SubFieldRole::Named("flow".into()),
                    &sim.proto.registry.property(pid).layout
                ),
            0.0
        );
    }
    raw_owner_weights(sim, profile, "open");
    bridge.consume_scheduled_ticks(3).unwrap();
    let sim = bridge.sim_session().unwrap();
    raw_owner_weights(sim, profile, "post-RF");
    let snapshot = AnchorTableSnapshot::from_session(sim);
    let mut values = BTreeMap::new();
    for (target, ids) in &profile.install_targets {
        if target == "rehearsal_ingress_rf"
            || target.starts_with("alpha")
            || target.starts_with("beta")
        {
            let value = observe_hosted_property_cell(
                &sim.proto.registry,
                &sim.proto.allocator,
                &snapshot,
                ids[0],
                &PropertyKey::new("ingress_rf", "energy"),
                &SubFieldRole::Amount,
            )
            .unwrap();
            values.insert(target.clone(), value);
        }
    }
    eprintln!("native RF born: {values:?}");
    values
}

fn raw_owner_weights(
    sim: &simthing_driver::SimSession,
    profile: &StudioAuthoredLiveProfile,
    stage: &str,
) {
    let columns = simthing_driver::resolve_node_columns_for_property(
        &sim.proto.registry,
        sim.proto.registry.id_of("ingress_rf", "energy").unwrap(),
        "native_energy",
    )
    .unwrap();
    let values = sim.state.read_values();
    let weights: Vec<_> = ["alpha", "beta"]
        .iter()
        .map(|owner| {
            let slot = sim
                .proto
                .allocator
                .slot_of(profile.install_targets[*owner][0])
                .unwrap();
            (
                *owner,
                values[usize::from(slot) * sim.proto.registry.total_columns
                    + columns.weight_col.raw_u32() as usize],
            )
        })
        .collect();
    eprintln!("diagnostic raw owner weights {stage}: {weights:?}");
}

fn direct_programmatic_beta(session: &StudioSession) -> f32 {
    let profile = session.authored_live_profile.as_ref().unwrap();
    let mut sim = simthing_driver::SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(profile).unwrap(),
        &field_bearing_game_mode(&profile.game_mode),
    )
    .unwrap();
    for _ in 0..3 {
        sim.step_once().unwrap();
    }
    observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &AnchorTableSnapshot::from_session(&sim),
        profile.install_targets["beta"][0],
        &PropertyKey::new("ingress_rf", "energy"),
        &SubFieldRole::Amount,
    )
    .unwrap()
}

#[test]
fn rehearsal_ingress_native_rf_preserves_graph_and_attributable_born_changes() {
    let directory = tempfile::tempdir().unwrap();
    let sources = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    for file in [
        "rehearsal_ingress_rf.clause",
        "rehearsal_ingress_rf.dependencies.json",
        "rehearsal_ingress_cache.base.json",
    ] {
        std::fs::copy(sources.join(file), directory.path().join(file)).unwrap();
    }
    let source = directory.path().join("rehearsal_ingress_rf.clause");
    let cache = directory.path().join("rf.simthing-scenario.json");
    let (_, native) = load_clause_studio_session_from_path(
        &source,
        &ClauseScenarioIngestOptions::default(),
        &cache,
        None,
    )
    .unwrap();
    let baseline = observe(&native);
    assert_eq!(baseline["beta"], direct_programmatic_beta(&native));
    assert_eq!(baseline.len(), 19);
    assert_eq!(
        baseline,
        observe(&load_studio_session_from_scenario_path(&cache, None).unwrap())
    );
    let text = std::fs::read_to_string(&source)
        .unwrap()
        .replace("\r\n", "\n");
    for (variant, locus) in [
        (text.replace("child = beta_3 { kind = Cohort\n        property_value = { property = \"ingress_rf::energy\" flow = 8", "child = beta_3 { kind = Cohort\n        property_value = { property = \"ingress_rf::energy\" flow = 20"), "beta_3"),
        (text.replace("@beta_policy = 3", "@beta_policy = 7"), "beta"),
    ] {
        assert_ne!(variant, text, "variant must actually change the native source");
        std::fs::write(&source, variant).unwrap();
        let (_, native) = load_clause_studio_session_from_path(&source, &ClauseScenarioIngestOptions::default(), &cache, None).unwrap();
        let beta = native.authored_live_profile.as_ref().unwrap().game_mode.overlays.iter().find(|overlay| overlay.id == "beta_policy").unwrap();
        assert_eq!(beta.sub_field_deltas[0].1.as_multiply_literal(), Some(if locus == "beta" { 7.0 } else { 3.0 }));
        let changed = observe(&native);
        assert_eq!(changed["beta"], direct_programmatic_beta(&native));
        assert_eq!(changed, observe(&load_studio_session_from_scenario_path(&cache, None).unwrap()));
        assert_ne!(baseline[locus], changed[locus], "authored {locus} change must reach born economics");
    }
}
