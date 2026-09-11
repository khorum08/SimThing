//! CT-1a successor: full native/cache admission and installed born modifier results.
use simthing_core::SubFieldRole;
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot};
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::{
    load_studio_session_from_scenario_path, StudioLiveSessionBridge, StudioSession,
};
use simthing_spec::PropertyKey;
use std::path::Path;

fn born_amount(session: &StudioSession) -> f32 {
    let profile = session.authored_live_profile.as_ref().unwrap();
    let host = profile.install_targets["specimen"][0];
    let mut bridge = StudioLiveSessionBridge::default();
    bridge.open_from_loaded_studio_session(session).unwrap();
    assert_eq!(
        bridge.sim_session().unwrap().proto.root.overlay_count(host),
        Some(1)
    );
    let sim = bridge.sim_session().unwrap();
    let property = sim.proto.registry.id_of("simthing", "potency").unwrap();
    let seed = sim
        .proto
        .root
        .property_on_node(host, property)
        .unwrap()
        .get_role(
            &SubFieldRole::Amount,
            &sim.proto.registry.property(property).layout,
        );
    assert_eq!(seed, 40.0, "the installed source seed is preserved");
    let at_open = observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &AnchorTableSnapshot::from_session(sim),
        host,
        &PropertyKey::new("simthing", "potency"),
        &SubFieldRole::Amount,
    )
    .unwrap();
    eprintln!("potency seed={seed}, open={at_open}");
    bridge.consume_scheduled_ticks(1).unwrap();
    let sim = bridge.sim_session().unwrap();
    observe_hosted_property_cell(
        &sim.proto.registry,
        &sim.proto.allocator,
        &AnchorTableSnapshot::from_session(sim),
        host,
        &PropertyKey::new("simthing", "potency"),
        &SubFieldRole::Amount,
    )
    .unwrap()
}

#[test]
fn rehearsal_ingress_literal_successor_preserves_born_modifier_parity() {
    let source_base = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    let directory = tempfile::tempdir().unwrap();
    for name in [
        "rehearsal_ingress_literal_successor.clause",
        "rehearsal_ingress_literal_successor.dependencies.json",
        "rehearsal_ingress_literal_successor.base.json",
    ] {
        std::fs::copy(source_base.join(name), directory.path().join(name)).unwrap();
    }
    let source = directory
        .path()
        .join("rehearsal_ingress_literal_successor.clause");
    let original = std::fs::read_to_string(&source).unwrap();
    let cache = directory.path().join("successor.simthing-scenario.json");
    let mut observations = vec![];
    for (text, expected) in [
        (original.clone(), 50.0),
        (
            original.replace("amount_mult = 1.25", "amount_mult = 1.5"),
            60.0,
        ),
        (
            original.replace("amount_mult = 1.25", "amount_mult = 1"),
            40.0,
        ),
    ] {
        std::fs::write(&source, text).unwrap();
        let (_, native) = load_clause_studio_session_from_path(
            &source,
            &ClauseScenarioIngestOptions::default(),
            &cache,
            None,
        )
        .unwrap();
        assert_vertical_structure(&native);
        let native_amount = born_amount(&native);
        let cached = load_studio_session_from_scenario_path(&cache, None).unwrap();
        assert_vertical_structure(&cached);
        let cached_amount = born_amount(&cached);
        eprintln!("potency native={native_amount}, cache={cached_amount}, expected={expected}");
        observations.push((native_amount, cached_amount, expected));
    }
    for (native, cached, expected) in observations {
        assert_eq!(native, expected);
        assert_eq!(cached, expected);
    }
}

fn assert_vertical_structure(session: &StudioSession) {
    let spec = &session.scenario_authority;
    assert_eq!(spec.root.kind, simthing_core::SimThingKind::Scenario);
    assert!(!session.admission_summary.legacy_world_root);
    simthing_spec::validate_stead_mapping_consistency(spec).unwrap();
    simthing_spec::validate_scenario_links(spec).unwrap();
    assert_eq!(
        (
            spec.structural_grid.frame.width,
            spec.structural_grid.frame.height
        ),
        (8, 8)
    );
    assert_eq!(spec.structural_grid.frame.occupied_cells, 2);
    let cells: Vec<_> = spec
        .structural_grid
        .placements
        .iter()
        .map(|cell| (cell.system_id, cell.row, cell.col))
        .collect();
    assert_eq!(cells, vec![(1, 2, 3), (2, 2, 4)]);
    assert_eq!(spec.links.len(), 1);
    assert_eq!(spec.links[0].from_system_id, "1");
    assert_eq!(spec.links[0].to_system_id, "2");
    let grid = &session.hydration.grid;
    assert_eq!(grid.gridcells.len(), 2);
    assert_eq!(grid.hyperlanes.len(), 1);
    assert!(grid.gridcells.iter().all(|cell| !cell.children.is_empty()));
}
