//! 2.2 authoring-boundary probe for the existing Meridian Arm asset.
//! Board 5725574222: stop at a concrete ingress/substrate gap, without an engine repair.
//! The RED refinery assertion expresses the frozen two-input contract. No spec/session
//! mutation, economic readback feedback, replacement model, or test-side birth is used.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_core::{SimThing, SubFieldRole};
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot, SimSession};
use simthing_mapeditor::clause_scenario_ingest::{
    clause_source_content_identity, ingest_clause_scenario_path, ClauseScenarioIngestOptions,
    ClauseScenarioIngestResult,
};
use simthing_mapeditor::studio_live_session_bridge::{
    authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
    field_bearing_game_mode, StudioAuthoredLiveProfile,
};
use simthing_spec::PropertyKey;

fn source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause")
}

fn ingest(path: &Path) -> ClauseScenarioIngestResult {
    let result = ingest_clause_scenario_path(path, &ClauseScenarioIngestOptions::default())
        .expect("ordinary native parse/expand/hydrate/rebind");
    let cache = result.source_cache.as_ref().unwrap();
    println!(
        "SOURCE path={} identity={} dependencies={:?}",
        path.display(),
        cache.source_identity,
        cache.dependencies
    );
    // This is the exact executable GameMode projection digest, not a claim to
    // normalize the entire runtime profile's process-minted tree identities.
    println!(
        "GAME_MODE_PROJECTION identity={}",
        clause_source_content_identity(&serde_json::to_vec(&result.pack.game_mode).unwrap())
    );
    result
}

fn variant(text: &str) -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for name in [
        "stellaristhing_base.base.json",
        "stellaristhing_base.dependencies.json",
    ] {
        std::fs::copy(
            source().parent().unwrap().join(name),
            directory.path().join(name),
        )
        .unwrap();
    }
    std::fs::write(directory.path().join("stellaristhing_base.clause"), text).unwrap();
    directory
}

fn identities(node: &SimThing, ids: &mut BTreeSet<u32>) {
    ids.insert(node.id.raw());
    for child in &node.children {
        identities(child, ids);
    }
}

fn trace(session: &SimSession, profile: &StudioAuthoredLiveProfile, label: &str) -> Vec<f32> {
    let anchors = AnchorTableSnapshot::from_session(session);
    let mut values = Vec::new();
    for (host, namespace, property, role) in [
        (
            "terran",
            "meridian",
            "energy",
            SubFieldRole::Named("balance".into()),
        ),
        (
            "pirate",
            "meridian",
            "energy",
            SubFieldRole::Named("balance".into()),
        ),
        (
            "A1",
            "meridian_material",
            "A1_minerals_quantity",
            SubFieldRole::Amount,
        ),
        (
            "A1",
            "meridian_material",
            "A1_alloys_quantity",
            SubFieldRole::Amount,
        ),
        (
            "E1",
            "meridian_material",
            "E1_minerals_quantity",
            SubFieldRole::Amount,
        ),
        (
            "E1",
            "meridian_material",
            "E1_alloys_quantity",
            SubFieldRole::Amount,
        ),
    ] {
        let id = profile.install_targets[host][0];
        let value = observe_hosted_property_cell(
            &session.proto.registry,
            &session.proto.allocator,
            &anchors,
            id,
            &PropertyKey::new(namespace, property),
            &role,
        )
        .expect("existing authored observation locus");
        println!("CELL case={label} generation={} host={host} id={} slot={:?} property={namespace}::{property} role={role:?} value={value}",
            session.coord.day_index(), id.raw(), session.proto.allocator.slot_of(id));
        values.push(value);
    }
    println!(
        "CAPACITY case={label} generation={} live_rows={} allocator_capacity={} execution={:?}",
        session.coord.day_index(),
        session.proto.allocator.live_count(),
        session.proto.allocator.capacity(),
        session.persisted_execution_identity()
    );
    values
}

#[test]
fn rehearsal_economy_fleet_existing_asset_executes_without_profile_replacement() {
    let original = std::fs::read_to_string(source()).unwrap();
    let no_energy = original
        .replace("@generator_rate = 2", "@generator_rate = 0")
        .replace("balance = 10", "balance = 0")
        .replace("balance = 8", "balance = 0");
    assert_ne!(original, no_energy);
    let directory = variant(&no_energy);
    for (label, path) in [
        ("canonical", source()),
        (
            "energy-withheld",
            directory.path().join("stellaristhing_base.clause"),
        ),
    ] {
        let result = ingest(&path);
        println!(
            "RECIPES case={label}: {:#?}",
            result.pack.game_mode.resource_economy
        );
        let profile = authored_live_profile_from_pack(&result.pack).unwrap();
        let scenario = driver_scenario_field_bearing_from_profile(&profile).unwrap();
        let initial_root = scenario.root.id;
        let mut initial_ids = BTreeSet::new();
        identities(&scenario.root, &mut initial_ids);
        println!(
            "N0 case={label} root={} existing_ids={initial_ids:?} targets={:?}",
            initial_root.raw(),
            profile.install_targets
        );
        let mut session =
            SimSession::open_from_spec(scenario, &field_bearing_game_mode(&profile.game_mode))
                .expect("ordinary field-bearing admission under standing E8 pin");
        assert_eq!(session.coord.day_index(), 0);
        let initial = trace(&session, &profile, label);
        for generation in 1..=3 {
            session
                .step_once()
                .expect("existing authored economic generation");
            assert_eq!(session.coord.day_index(), generation);
            let observed = trace(&session, &profile, label);
            assert!(observed.iter().all(|value| value.is_finite()));
            println!(
                "OBSERVED_ALLOY_DELTA case={label} generation={generation} terran={} pirate={}",
                observed[3] - initial[3],
                observed[5] - initial[5]
            );
        }
    }
    assert_eq!(std::fs::read_to_string(source()).unwrap(), original);
}

#[test]
fn rehearsal_economy_fleet_refinery_retains_every_authored_cost() {
    let original = std::fs::read_to_string(source()).unwrap();
    let mineral = "input = { resource = minerals amount = @refinery_input }";
    let energy = "input = { resource = energy amount = 1 }";
    assert_eq!(
        original.matches(mineral).count(),
        2,
        "both existing faction refineries"
    );
    let mut failures = Vec::new();
    for (label, replacement) in [
        ("minerals-then-energy", format!("{mineral}\n      {energy}")),
        ("energy-then-minerals", format!("{energy}\n      {mineral}")),
    ] {
        let candidate = original.replace(mineral, &replacement);
        let directory = variant(&candidate);
        let result = ingest(&directory.path().join("stellaristhing_base.clause"));
        let recipes = &result
            .pack
            .game_mode
            .resource_economy
            .as_ref()
            .unwrap()
            .recipes;
        assert_eq!(recipes.len(), 2);
        for recipe in recipes {
            let costs: Vec<_> = recipe
                .inputs
                .iter()
                .map(|input| {
                    (
                        input.property.namespace.as_str(),
                        input.property.name.as_str(),
                        input.unit_cost,
                        input.host_entity.as_deref(),
                        &input.role,
                    )
                })
                .collect();
            println!(
                "AUTHORED_TWO_COSTS case={label} recipe={} hydrated={costs:?}",
                recipe.id
            );
            if costs.len() != 2 {
                failures.push(format!(
                    "{label}/{}: authored minerals=2 AND energy=1; retained {costs:?}",
                    recipe.id
                ));
            }
        }
    }
    // The accepted source must not silently become a different economic law.
    // Keep this RED when ingress loses a cost; do not bless the defect as expected behavior.
    assert!(
        failures.is_empty(),
        "2.2 STOP: refinery input conjunction lost by native ingestion: {failures:#?}"
    );
}
