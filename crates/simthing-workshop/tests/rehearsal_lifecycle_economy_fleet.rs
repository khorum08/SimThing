//! 2.2 authoring-boundary probe for the existing Meridian Arm asset.
//! Board 5738509297: conjunction-first, capped-stock, then native funded births.
//! The refinery assertion expresses the frozen two-input contract. No spec/session
//! mutation, economic readback feedback, replacement model, or test-side birth is used.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_core::{
    ClampBehavior, GenerationStamp, ObjectResidencyRelation, Overlay, OverlayId, OverlayKind,
    OverlayLifecycle, OverlaySource, PropertyTransformDelta, SimThing, SubFieldRole, TransformOp,
};
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot, SimSession};
use simthing_feeder::BoundaryRequest;
use simthing_mapeditor::clause_scenario_ingest::{
    clause_source_content_identity, ingest_clause_scenario_path,
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions, ClauseScenarioIngestResult,
};
use simthing_mapeditor::load_studio_session_from_scenario_path;
use simthing_mapeditor::studio_live_session_bridge::{
    authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
    field_bearing_game_mode, StudioAuthoredLiveProfile,
};
use simthing_sim::{BoundaryDeltaEntry, OrdinaryGrowthRefusalReason, RecordedGrowthResidencyFact};
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

fn pin_profile(profile: &StudioAuthoredLiveProfile, label: &str) {
    // Serialize through JSON Value for canonical object-key ordering, including
    // intrinsic tree values and install targets omitted by the GameMode digest.
    // Runtime node identities remain included: this pins this exact run, not an
    // identity-normalized equivalence claim between separate admissions.
    let value = serde_json::to_value((
        &profile.game_mode,
        &profile.session_root,
        &profile.install_targets,
    ))
    .unwrap();
    println!(
        "PROFILE_FULL case={label} identity={} targets={:?}",
        clause_source_content_identity(&serde_json::to_vec(&value).unwrap()),
        profile.install_targets
    );
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
        pin_profile(&profile, label);
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
    let original = std::fs::read_to_string(source())
        .unwrap()
        .replace("\r\n", "\n");
    let mineral = "input = { resource = minerals amount = @refinery_input }";
    assert_eq!(
        original.matches(mineral).count(),
        2,
        "both existing faction refineries"
    );
    let mut ordered_economics = Vec::new();
    let mut trajectories = Vec::new();
    for energy_first in [false, true] {
        let label = if energy_first {
            "energy-then-minerals"
        } else {
            "minerals-then-energy"
        };
        let mut candidate = original.clone();
        for (owner, location) in [("terran", "A1"), ("pirate", "E1")] {
            let prefix = format!(
                "production_building = {owner}_refining {{\n      location = {location}\n      "
            );
            let single = format!("{prefix}{mineral}");
            let energy = format!(
                r#"input = {{ entity = {owner} property = "meridian::energy" role = balance amount = 1 }}"#
            );
            let costs = if energy_first {
                format!("{energy}\n      {mineral}")
            } else {
                format!("{mineral}\n      {energy}")
            };
            assert_eq!(candidate.matches(&single).count(), 1);
            candidate = candidate.replace(&single, &format!("{prefix}{costs}"));
        }
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
        for (recipe, owner, location) in [
            (
                recipes
                    .iter()
                    .find(|r| r.id.ends_with("terran_refining"))
                    .unwrap(),
                "terran",
                "A1",
            ),
            (
                recipes
                    .iter()
                    .find(|r| r.id.ends_with("pirate_refining"))
                    .unwrap(),
                "pirate",
                "E1",
            ),
        ] {
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
            assert_eq!(
                costs.len(),
                2,
                "complete conjunction: {label}/{}",
                recipe.id
            );
            assert_eq!(recipe.order_band, 0, "both faction recipes share band zero");
            assert!(recipe.inputs.iter().any(|i| i.property
                == PropertyKey::new("meridian_material", format!("{location}_minerals_quantity"))
                && i.unit_cost == 2.0
                && i.role == SubFieldRole::Amount
                && i.host_entity.as_deref() == Some(location)));
            assert!(recipe.inputs.iter().any(|i| i.property
                == PropertyKey::new("meridian", "energy")
                && i.unit_cost == 1.0
                && i.role == SubFieldRole::Named("balance".into())
                && i.host_entity.as_deref() == Some(owner)));
        }
        let mut economics = recipes.clone();
        for input in economics
            .iter_mut()
            .flat_map(|recipe| recipe.inputs.iter_mut())
        {
            input.host_span_token = None; // source order changes provenance, not economics
        }
        ordered_economics.push(economics);
        let profile = authored_live_profile_from_pack(&result.pack).unwrap();
        pin_profile(&profile, label);
        let mut session = SimSession::open_from_spec(
            driver_scenario_field_bearing_from_profile(&profile).unwrap(),
            &field_bearing_game_mode(&profile.game_mode),
        )
        .expect("both factions admit together through the ordinary band-0 economy");
        let before = trace(&session, &profile, label);
        session
            .step_once()
            .expect("both complete conjunctions execute");
        let after = trace(&session, &profile, label);
        for (energy, alloy) in [(0, 3), (1, 5)] {
            let produced = after[alloy] - before[alloy];
            assert!(
                produced > 0.0,
                "each faction refines in the same generation"
            );
            assert_eq!(
                before[energy] - after[energy],
                produced,
                "each alloy consumes its owner's energy"
            );
        }
        trajectories.push((before, after));
    }
    assert_eq!(ordered_economics[0], ordered_economics[1]);
    assert_eq!(trajectories[0], trajectories[1]);
    println!("CONJUNCTION-FIRST PASS both authored orders, both factions, ordinary admission and execution");
}

/// Source-only composition: the refinery cohort is the faction's spendable
/// energy stock, receiving the existing RF allocation from the two generators.
/// Owner balances move to this owned leaf (not copied), while spatial and RF
/// parentage remain distinct. Zero sibling weights direct the site's net supply
/// to that stock; generator and facility intrinsic rates remain unchanged.
/// Minerals use the same admitted RF Balance form on the existing mine, so
/// their explicit endowment is separate from the 3-per-generation source rate.
/// The old quantity emission is removed, not retained as a second mineral lane.
fn generator_stock_source(withheld: bool) -> String {
    let mut text = std::fs::read_to_string(source())
        .unwrap()
        .replace("\r\n", "\n");
    for (owner, site, minerals, energy) in [("terran", "A1", 20, 10), ("pirate", "E1", 14, 8)] {
        let mineral_input = "input = { resource = minerals amount = @refinery_input }";
        let prefix = format!("production_building = {owner}_refining {{\n      location = {site}\n      {mineral_input}");
        assert_eq!(text.matches(&prefix).count(), 1);
        text = text.replace(&prefix, &format!("production_building = {owner}_refining {{\n      location = {site}\n      input = {{ entity = {owner}_mine property = \"meridian::minerals\" role = balance amount = @refinery_input }}\n      input = {{ entity = {owner}_refinery property = \"meridian::energy\" role = balance amount = 1 }}"));
        let owner_stock = format!("flow = 0 weight = 1 balance = {energy}");
        assert_eq!(text.matches(&owner_stock).count(), 1);
        text = text.replace(&owner_stock, "flow = 0 weight = 1 balance = 0");
        let stock = format!("child = {owner}_refinery {{ kind = Cohort name = \"Refinery upkeep\"\n        property_value = {{ property = \"meridian::energy\" flow = @facility_upkeep weight = 1 }}");
        assert_eq!(text.matches(&stock).count(), 1);
        text = text.replace(&stock, &format!("child = {owner}_refinery {{ kind = Cohort name = \"Refinery upkeep\"\n        owner_ref = {owner}\n        property_value = {{ property = \"meridian::energy\" flow = @facility_upkeep weight = 1 balance = {} }}", if withheld { 0 } else { energy }));
        let emission = format!("    field_resource_quantity = {owner}_mining {{ location = {site} resource = minerals amount = @mine_rate }}\n");
        assert_eq!(text.matches(&emission).count(), 1);
        text = text.replace(&emission, "");
        let mine = format!("child = {owner}_mine {{ kind = Cohort name = \"Mine upkeep\"\n        property_value = {{ property = \"meridian::energy\" flow = @facility_upkeep weight = 1 }}");
        assert_eq!(text.matches(&mine).count(), 1);
        text = text.replace(&mine, &format!("{}\n        owner_ref = {owner}\n        property_value = {{ property = \"meridian::minerals\" flow = @mine_rate weight = 1 balance = {minerals} }}", mine.replace("weight = 1", "weight = 0")));
        let parent =
            format!("resource_parent = {{ property = \"meridian::energy\" parent = {owner} }}");
        text = text.replace(&parent, &format!("{parent}\n    resource_parent = {{ property = \"meridian::minerals\" parent = {owner} }}\n    property_value = {{ property = \"meridian::minerals\" flow = 0 weight = 1 }}"));
    }
    let owner_parent =
        "resource_parent = { property = \"meridian::energy\" parent = stellaristhing_base }";
    text = text.replace(owner_parent, &format!("{owner_parent}\n    resource_parent = {{ property = \"meridian::minerals\" parent = stellaristhing_base }}\n    property_value = {{ property = \"meridian::minerals\" flow = 0 weight = 1 }}"));
    text = text.replacen("  property_value = { property = \"meridian::energy\" flow = 0 weight = 1 }", "  property_value = { property = \"meridian::energy\" flow = 0 weight = 1 }\n  property_value = { property = \"meridian::minerals\" flow = 0 weight = 1 }", 1);
    let mineral_property = r#"    properties = { property = {
      id = meridian_minerals
      namespace = meridian
      name = minerals
      sub_field = { role = flow accumulator = IntrinsicFlow }
      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }
      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {"#;
    text = text.replace("    properties = { property = {", mineral_property);
    text = text.replace(
        "flow = @generator_rate weight = 1",
        "flow = @generator_rate weight = 0",
    );
    assert_eq!(text.matches("throttle_hint_max_per_tick = 1").count(), 2);
    text = text.replace(
        "throttle_hint_max_per_tick = 1",
        "throttle_hint_max_per_tick = 1\n      max_units_per_generation = 1",
    );
    if withheld {
        // Withhold the spendable surplus: two 1-energy generators cover the
        // unchanged two facility costs, leaving no refinery/fleet energy.
        text = text.replace("@generator_rate = 2", "@generator_rate = 1");
    }
    text
}

fn energy_cell(
    session: &SimSession,
    profile: &StudioAuthoredLiveProfile,
    host: &str,
    role: &str,
) -> f32 {
    observe_hosted_property_cell(
        &session.proto.registry,
        &session.proto.allocator,
        &AnchorTableSnapshot::from_session(session),
        profile.install_targets[host][0],
        &PropertyKey::new("meridian", "energy"),
        &SubFieldRole::Named(role.into()),
    )
    .unwrap()
}

fn tree_node(root: &SimThing, id: simthing_core::SimThingId) -> Option<&SimThing> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|child| tree_node(child, id))
}

fn stock_snapshot(
    session: &SimSession,
    profile: &StudioAuthoredLiveProfile,
    label: &str,
) -> Vec<f32> {
    let anchors = AnchorTableSnapshot::from_session(session);
    let mut values = vec![
        energy_cell(session, profile, "terran", "balance"),
        energy_cell(session, profile, "pirate", "balance"),
    ];
    for (owner, site) in [("terran", "A1"), ("pirate", "E1")] {
        for (host, key, role) in [
            (
                format!("{owner}_mine"),
                PropertyKey::new("meridian", "minerals"),
                SubFieldRole::Named("balance".into()),
            ),
            (
                site.to_owned(),
                PropertyKey::new("meridian_material", format!("{site}_alloys_quantity")),
                SubFieldRole::Amount,
            ),
        ] {
            let id = profile.install_targets[&host][0];
            let value = observe_hosted_property_cell(
                &session.proto.registry,
                &session.proto.allocator,
                &anchors,
                id,
                &key,
                &role,
            )
            .unwrap();
            println!("STOCK_CELL case={label} generation={} host={host} id={} slot={:?} property={key:?} role={role:?} value={value}", session.coord.day_index(), id.raw(), session.proto.allocator.slot_of(id));
            values.push(value);
        }
    }
    values
}

fn restore_generators(session: &SimSession, profile: &StudioAuthoredLiveProfile) {
    for owner in ["terran", "pirate"] {
        for index in 1..=2 {
            let target = profile.install_targets[&format!("{owner}_generator_{index}")][0];
            session
                .tx
                .submit_boundary(BoundaryRequest::AttachOverlay {
                    target,
                    source_generation: GenerationStamp::new(
                        session.coord.day_index().try_into().unwrap(),
                    ),
                    overlay: Overlay {
                        id: OverlayId::new(),
                        kind: OverlayKind::Policy,
                        source: OverlaySource::System,
                        origin: target,
                        affects: vec![target],
                        transform: PropertyTransformDelta {
                            property_id: session
                                .proto
                                .registry
                                .id_of("meridian", "energy")
                                .unwrap(),
                            sub_field_deltas: vec![(
                                SubFieldRole::Named("flow".into()),
                                TransformOp::set(2.0),
                            )],
                        },
                        lifecycle: OverlayLifecycle::UntilDissolved,
                    },
                })
                .unwrap();
        }
    }
}

#[test]
fn rehearsal_economy_fleet_generator_stock_preserves_frozen_economy() {
    struct GeneratorCase {
        label: &'static str,
        withheld: bool,
        initial_energy: [f32; 2],
        expected_batches: [f32; 8],
    }
    const CASES: [GeneratorCase; 2] = [
        GeneratorCase {
            label: "generator-stock",
            withheld: false,
            initial_energy: [10.0, 8.0],
            expected_batches: [1.0; 8],
        },
        GeneratorCase {
            label: "generator-withheld-restored",
            withheld: true,
            initial_energy: [0.0; 2],
            expected_batches: [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        },
    ];
    let mut contract_failures = Vec::new();
    for case in CASES {
        let GeneratorCase {
            label,
            withheld,
            initial_energy,
            expected_batches,
        } = case;
        let directory = variant(&generator_stock_source(withheld));
        let result = ingest(&directory.path().join("stellaristhing_base.clause"));
        let profile = authored_live_profile_from_pack(&result.pack).unwrap();
        pin_profile(&profile, label);
        let economy = profile.game_mode.resource_economy.as_ref().unwrap();
        assert!(
            economy.emissions.is_empty(),
            "no extra Constant source or seed ledger"
        );
        assert!(economy.transfers.is_empty(), "no stock staging transfers");
        assert_eq!(economy.recipes.len(), 2);
        for recipe in &economy.recipes {
            assert_eq!(
                recipe.inputs.len(),
                2,
                "only the frozen mineral/energy costs"
            );
            assert_eq!(recipe.throttle_hint_max_per_tick, 1);
            assert_eq!(recipe.max_units_per_generation.unwrap().get(), 1);
            assert_eq!(recipe.order_band, 0);
            assert_eq!(recipe.output_coefficient, 1.0);
        }
        let scenario = driver_scenario_field_bearing_from_profile(&profile).unwrap();
        for (site, expected) in [("terran_mine", 20.0), ("pirate_mine", 14.0)] {
            let id = scenario.registry.id_of("meridian", "minerals").unwrap();
            let node = tree_node(&scenario.root, profile.install_targets[site][0]).unwrap();
            let authored = node.properties[&id].get_role(
                &SubFieldRole::Named("balance".into()),
                &scenario.registry.property(id).layout,
            );
            println!("AUTHORED_N0 case={label} site={site} minerals={authored}");
            assert_eq!(
                authored, expected,
                "explicit endowment survives native hydration/profile"
            );
        }
        let mut session =
            SimSession::open_from_spec(scenario, &field_bearing_game_mode(&profile.game_mode))
                .unwrap();
        let mut previous = stock_snapshot(&session, &profile, label);
        let installed = [previous[2], previous[3], previous[4], previous[5]];
        assert_eq!(
            installed,
            [20.0, 4.0, 14.0, 3.0],
            "frozen N0 mineral/alloy endowments"
        );
        let mut energy = [
            energy_cell(&session, &profile, "terran_refinery", "balance"),
            energy_cell(&session, &profile, "pirate_refinery", "balance"),
        ];
        assert_eq!(energy, initial_energy);
        println!("STOCK_N0 case={label} energy={energy:?}");
        let mut recovered = [0.0; 2];
        for generation in 1..=8 {
            if withheld && generation == 4 {
                restore_generators(&session, &profile);
            }
            session
                .step_once()
                .expect("generator-to-stock ordinary generation");
            let current = stock_snapshot(&session, &profile, label);
            for (faction, owner, mineral, alloy) in [(0, "terran", 2, 3), (1, "pirate", 4, 5)] {
                let host = format!("{owner}_refinery");
                let balance = energy_cell(&session, &profile, &host, "balance");
                let settled = energy_cell(&session, &profile, &host, "balance_rate");
                let batches = current[alloy] - previous[alloy];
                println!("STOCK_FLOW case={label} generation={generation} owner={owner} energy_before={} settled={settled} consumed={batches} energy_after={balance} minerals_before={} minerals_after={} alloys_before={} alloys_after={}", energy[faction], previous[mineral], current[mineral], previous[alloy], current[alloy]);
                assert_eq!(
                    balance,
                    energy[faction] + settled - batches,
                    "one energy economy reconciles"
                );
                assert_eq!(
                    current[mineral],
                    previous[mineral] + 3.0 - 2.0 * batches,
                    "mine production and refinery consumption reconcile"
                );
                assert_eq!(current[faction], 0.0, "no owner duplicate energy stock");
                assert!(
                    balance >= 0.0,
                    "no hidden energy overdraft in this composition"
                );
                for other_host in [
                    format!("{owner}_generator_1"),
                    format!("{owner}_generator_2"),
                    format!("{owner}_mine"),
                    if owner == "terran" {
                        "A1".into()
                    } else {
                        "E1".into()
                    },
                ] {
                    assert_eq!(
                        energy_cell(&session, &profile, &other_host, "balance"),
                        0.0,
                        "no duplicated energy stock at {other_host}"
                    );
                }
                let flows: Vec<_> = (1..=2)
                    .map(|i| {
                        energy_cell(
                            &session,
                            &profile,
                            &format!("{owner}_generator_{i}"),
                            "flow",
                        )
                    })
                    .collect();
                println!("GENERATOR_FLOW case={label} generation={generation} owner={owner} authored_effective_flow={flows:?}");
                if withheld && generation <= 3 {
                    assert_eq!(batches, 0.0, "withheld generators block refinery");
                }
                if withheld && generation >= 4 {
                    recovered[faction] += batches;
                }
                assert_eq!(
                    batches,
                    expected_batches[generation as usize - 1],
                    "exact rate, including recovery without banked unused capacity"
                );
                if batches > 1.0 {
                    contract_failures.push(format!(
                        "{label}/G{generation}/{owner}: {batches} alloys, frozen maximum 1"
                    ));
                }
                energy[faction] = balance;
            }
            previous = current;
        }
        if withheld {
            assert!(
                recovered.iter().all(|produced| *produced > 0.0),
                "actual generator restoration funds eventual recovery: {recovered:?}"
            );
        }
    }
    assert!(
        contract_failures.is_empty(),
        "2.2 STOP: frozen refinery throughput not preserved: {contract_failures:#?}"
    );

    // The next storage obligation needs a finite bound on the actual spendable
    // cell, not owner-silo metadata or a second stock lane. This is an ingress
    // probe of existing core Bounded semantics; it claims no overflow policy or
    // completed runtime storage proof. Keep the two GREEN stock cases above.
    let unbounded = generator_stock_source(false);
    let balance = "sub_field = { role = balance governed_by = balance_rate accumulator = Balance }";
    assert_eq!(unbounded.matches(balance).count(), 2);
    // Minerals are the first declaration, energy the second. Both endowments
    // fit within this finite diagnostic bound; the frozen rates are unchanged.
    let bounded = unbounded.replacen(
        balance,
        "sub_field = { role = balance governed_by = balance_rate accumulator = Balance clamp = Bounded { min = 0 max = 24 } }",
        1,
    );
    println!("BOUNDED_STORAGE_INGRESS source={bounded}");
    let directory = variant(&bounded);
    let path = directory.path().join("stellaristhing_base.clause");
    let result = ingest_clause_scenario_path(&path, &ClauseScenarioIngestOptions::default())
        .unwrap_or_else(|error| panic!(
            "2.2 STOP: native source cannot express the finite mineral Balance bound (0..24) needed by bounded storage: {error:?}"
        ));
    let mineral = result
        .pack
        .game_mode
        .properties
        .iter()
        .find(|property| property.namespace == "meridian" && property.name == "minerals")
        .unwrap();
    let stock = mineral
        .sub_fields
        .iter()
        .find(|field| field.role == SubFieldRole::Named("balance".into()))
        .unwrap();
    assert_eq!(
        stock.clamp,
        ClampBehavior::Bounded {
            min: 0.0,
            max: 24.0
        },
        "native hydration must preserve the authored stock bound"
    );
    bounded_stock_lifecycle();
}

/// Local mine production is outside conserved RF. The sole mineral Balance is
/// still the refinery's real input. The two authored modifiers reset its local
/// rate then supply the frozen 3/gen; the governed tail integrates it once.
/// Capacity curtails local production, never an allocation disbursed by RF.
fn local_stock_source(clamp: &str) -> String {
    let mut text = generator_stock_source(false);
    for declaration in [
        "      sub_field = { role = flow accumulator = IntrinsicFlow }\n",
        "      sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_minerals } }\n",
        "      sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_minerals } }\n",
    ] {
        assert!(text.contains(declaration));
        text = text.replacen(declaration, "", 1);
    }
    text = text
        .lines()
        .filter(|line| {
            !line.contains("resource_parent = { property = \"meridian::minerals\"")
                && !line.contains(
                    "property_value = { property = \"meridian::minerals\" flow = 0 weight = 1 }",
                )
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !clamp.is_empty() {
        text = text.replacen(
            "sub_field = { role = balance governed_by = balance_rate accumulator = Balance }",
            &format!("sub_field = {{ role = balance governed_by = balance_rate accumulator = Balance {clamp} }}"),
            1,
        );
    }
    for (owner, initial) in [("terran", 20), ("pirate", 14)] {
        let before = format!("property_value = {{ property = \"meridian::minerals\" flow = @mine_rate weight = 1 balance = {initial} }}");
        assert_eq!(text.matches(&before).count(), 1);
        text = text.replace(&before, &format!(r#"property_value = {{ property = "meridian::minerals" balance_rate = @mine_rate balance = {initial} }}
        overlays = {{
          modifier = {{ id = {owner}_mining_reset targets_property = "meridian::minerals" sub_field = balance_rate amount_mult = 0 }}
          modifier = {{ id = {owner}_mining_supply targets_property = "meridian::minerals" sub_field = balance_rate amount_add = @mine_rate }}
        }}"#));
    }
    text
}

fn local_stock_snapshot(session: &SimSession, profile: &StudioAuthoredLiveProfile) -> Vec<f32> {
    let mut values = vec![
        energy_cell(session, profile, "terran", "balance"),
        energy_cell(session, profile, "pirate", "balance"),
    ];
    for (owner, site) in [("terran", "A1"), ("pirate", "E1")] {
        let mine = profile.install_targets[&format!("{owner}_mine")][0];
        let slot = session.proto.allocator.slot_of(mine).unwrap();
        let mineral = session
            .proto
            .registry
            .id_of("meridian", "minerals")
            .unwrap();
        let col = session
            .proto
            .registry
            .column_range(mineral)
            .col_for_role(
                &SubFieldRole::Named("balance".into()),
                &session.proto.registry.property(mineral).layout,
            )
            .unwrap();
        let alloy = session
            .proto
            .registry
            .id_of("meridian_material", &format!("{site}_alloys_quantity"))
            .unwrap();
        let alloy_col = session
            .proto
            .registry
            .column_range(alloy)
            .col_for_role(
                &SubFieldRole::Amount,
                &session.proto.registry.property(alloy).layout,
            )
            .unwrap();
        let alloy_slot = session
            .proto
            .allocator
            .slot_of(profile.install_targets[site][0])
            .unwrap();
        let recipes = &session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .unwrap()
            .registrations
            .recipes;
        let consumers: Vec<_> = recipes
            .iter()
            .filter(|recipe| {
                recipe
                    .inputs
                    .iter()
                    .any(|input| input.slot == slot && input.col == col)
            })
            .collect();
        assert_eq!(
            consumers.len(),
            1,
            "the observed cell is the one actual refinery stock"
        );
        let recipe = consumers[0];
        assert_eq!(recipe.target_slot, alloy_slot);
        assert_eq!(recipe.target_col, alloy_col);
        assert_eq!(
            recipe
                .inputs
                .iter()
                .find(|input| input.slot == slot && input.col == col)
                .unwrap()
                .unit_cost,
            2.0
        );
        // Published GPU values are observations only. No RF anchor is invented
        // for local storage, and these values never drive a boundary request.
        values.push(session.state.read_values_row(slot.raw())[col.raw()]);
        values.push(session.state.read_values_row(alloy_slot.raw())[alloy_col.raw()]);
    }
    values
}

fn bounded_stock_lifecycle() {
    let bound = ClampBehavior::Bounded {
        min: 0.0,
        max: 24.0,
    };
    let mut histories = Vec::new();
    for (label, clamp, ceiling, from_cache) in [
        ("local-omitted", "", None, false),
        ("local-unbounded", "clamp = Unbounded", None, false),
        (
            "local-bounded",
            "clamp = Bounded { min = 0 max = 24 }",
            Some(24.0_f32),
            false,
        ),
        (
            "local-bounded-cache",
            "clamp = Bounded { min = 0 max = 24 }",
            Some(24.0_f32),
            true,
        ),
    ] {
        let text = local_stock_source(clamp);
        println!(
            "LOCAL_STORAGE_SOURCE case={label} identity={}\n{text}\nLOCAL_STORAGE_SOURCE_END",
            clause_source_content_identity(text.as_bytes())
        );
        let directory = variant(&text);
        let cache = directory.path().join("storage.simthing-scenario.json");
        let (_, native) = load_clause_studio_session_from_path(
            &directory.path().join("stellaristhing_base.clause"),
            &ClauseScenarioIngestOptions::default(),
            &cache,
            None,
        )
        .expect("ordinary source, hydrate, profile and canonical cache");
        let loaded = if from_cache {
            load_studio_session_from_scenario_path(&cache, None).expect("ordinary cache rebind")
        } else {
            native
        };
        let profile = loaded.authored_live_profile.unwrap();
        pin_profile(&profile, label);
        let expected_clamp = if ceiling.is_some() {
            bound.clone()
        } else {
            ClampBehavior::Unbounded
        };
        let property = profile
            .game_mode
            .properties
            .iter()
            .find(|p| p.namespace == "meridian" && p.name == "minerals")
            .unwrap();
        assert_eq!(
            property.sub_fields.len(),
            2,
            "one rate and one spendable stock"
        );
        assert_eq!(
            property
                .sub_fields
                .iter()
                .find(|s| s.role == SubFieldRole::Named("balance".into()))
                .unwrap()
                .clamp,
            expected_clamp,
            "native/cache spec retains the authored bound"
        );
        let economy = profile.game_mode.resource_economy.as_ref().unwrap();
        assert!(economy.emissions.is_empty() && economy.transfers.is_empty());
        assert_eq!(
            economy.recipes.len(),
            2,
            "only the two original capped refineries"
        );
        for recipe in &economy.recipes {
            assert_eq!(recipe.inputs.len(), 2);
            assert_eq!(recipe.max_units_per_generation.unwrap().get(), 1);
        }
        let mut session = SimSession::open_from_spec(
            driver_scenario_field_bearing_from_profile(&profile).unwrap(),
            &field_bearing_game_mode(&profile.game_mode),
        )
        .expect("ordinary local storage session");
        let mineral = session
            .proto
            .registry
            .id_of("meridian", "minerals")
            .unwrap();
        let energy = session.proto.registry.id_of("meridian", "energy").unwrap();
        assert_eq!(
            session
                .proto
                .registry
                .property(mineral)
                .layout
                .sub_fields
                .iter()
                .find(|s| s.role == SubFieldRole::Named("balance".into()))
                .unwrap()
                .clamp,
            expected_clamp,
            "live registry retains the authored bound"
        );
        assert!(
            session
                .spec_state
                .arena_registry
                .arenas
                .iter()
                .all(|a| a.flow_property_id != mineral && a.balance_property_id != Some(mineral)),
            "bounded stock cannot receive a conserved RF allocation"
        );
        assert!(
            session
                .spec_state
                .arena_registry
                .arenas
                .iter()
                .any(|a| a.flow_property_id == energy),
            "the same energy economy still executes RF"
        );
        let supplies: Vec<_> = ["terran", "pirate"]
            .iter()
            .map(|owner| {
                let host = profile.install_targets[&format!("{owner}_mine")][0];
                let specs: Vec<_> = profile
                    .game_mode
                    .overlays
                    .iter()
                    .filter(|o| {
                        o.id == format!("{owner}_mining_reset")
                            || o.id == format!("{owner}_mining_supply")
                    })
                    .collect();
                assert_eq!(specs.len(), 2);
                assert_eq!(
                    specs[0].sub_field_deltas,
                    vec![(
                        SubFieldRole::Named("balance_rate".into()),
                        TransformOp::multiply(0.0)
                    )]
                );
                assert_eq!(
                    specs[1].sub_field_deltas,
                    vec![(
                        SubFieldRole::Named("balance_rate".into()),
                        TransformOp::add(3.0)
                    )]
                );
                // Ordinary installation keeps source order on this two-overlay host.
                // This observes identity only; no stock readback drives the schedule.
                let installed = session.proto.root.snapshot_node(host).unwrap().overlay_ids;
                assert_eq!(installed.len(), 2);
                (host, installed[1])
            })
            .collect();
        let mut expected_stock = [20.0_f32, 14.0];
        let mut batches = [0.0_f32; 2];
        let mut produced = [0.0_f32; 2];
        let mut curtailed = [0.0_f32; 2];
        let mut history = Vec::new();
        let initial = local_stock_snapshot(&session, &profile);
        assert_eq!([initial[2], initial[4]], expected_stock);
        assert_eq!([initial[3], initial[5]], [4.0, 3.0]);
        for generation in 1..=50 {
            // The hot cycle precedes its boundary. Submit for the G11/G25
            // boundaries so the supply is off for hot cycles G12 through G25.
            // This fixed schedule never depends on economic readback.
            if generation == 11 || generation == 25 {
                for &(target, overlay_id) in &supplies {
                    session
                        .tx
                        .submit_boundary(if generation == 11 {
                            BoundaryRequest::SuspendOverlay { target, overlay_id }
                        } else {
                            BoundaryRequest::ActivateOverlay { target, overlay_id }
                        })
                        .unwrap();
                }
            }
            session.step_once().expect("ordinary storage generation");
            let observed = local_stock_snapshot(&session, &profile);
            let active = !(12..26).contains(&generation);
            for &(host, id) in &supplies {
                assert_eq!(
                    session.proto.root.overlay_is_active(host, id),
                    Some(!(11..25).contains(&generation))
                );
            }
            let offered: f32 = if active { 3.0 } else { 0.0 };
            for (index, owner, initial_stock, initial_alloy, initial_energy) in [
                (0, "terran", 20.0, 4.0, 10.0),
                (1, "pirate", 14.0, 3.0, 8.0),
            ] {
                // Independent accounting oracle only; never writes the session.
                let spend = if expected_stock[index] >= 2.0 {
                    2.0
                } else {
                    0.0
                };
                let after_spend = expected_stock[index] - spend;
                let incoming = ceiling.map_or(offered, |max| offered.min(max - after_spend));
                expected_stock[index] = after_spend + incoming;
                batches[index] += spend / 2.0;
                produced[index] += incoming;
                curtailed[index] += offered - incoming;
                let actual_stock = observed[2 + index * 2];
                assert_eq!(
                    actual_stock, expected_stock[index],
                    "{label} {owner} G{generation}: exact stock accounting"
                );
                assert_eq!(
                    observed[3 + index * 2],
                    initial_alloy + batches[index],
                    "exact capped lawful spending"
                );
                assert_eq!(
                    actual_stock + 2.0 * batches[index],
                    initial_stock + produced[index],
                    "no second credit or lost produced stock"
                );
                assert_eq!(
                    energy_cell(&session, &profile, &format!("{owner}_refinery"), "balance"),
                    initial_energy + 2.0 * generation as f32 - batches[index],
                    "energy RF reconciles with the same recipe"
                );
                assert!(actual_stock >= 0.0);
                if let Some(max) = ceiling {
                    assert!(actual_stock <= max);
                }
                println!("LOCAL_STORAGE case={label} owner={owner} generation={generation} spend={spend} offered={offered} produced={incoming} curtailed={} stock={actual_stock} cumulative_produced={} cumulative_curtailed={} batches={}",
                    offered - incoming, produced[index], curtailed[index], batches[index]);
            }
            history.push([observed[2], observed[4]]);
        }
        if ceiling.is_some() {
            assert_eq!(
                history[10],
                [24.0, 24.0],
                "both stocks saturate before interruption"
            );
            assert_eq!(
                history[11],
                [22.0, 22.0],
                "ordinary recipe spending creates visible headroom"
            );
            assert_eq!(
                history[22],
                [0.0, 0.0],
                "lawful spend reaches the floor without minting"
            );
            assert_eq!(
                history[24],
                [0.0, 0.0],
                "unfunded recipes cannot spend through the floor"
            );
            assert_eq!(
                history[25],
                [3.0, 3.0],
                "restoration adds exactly one generation of supply"
            );
            assert_eq!(
                history[26],
                [4.0, 4.0],
                "ordinary recipe resumes, no banked throughput"
            );
            assert_eq!(
                history[46],
                [24.0, 24.0],
                "refill reaches the ceiling again"
            );
            assert_eq!(history[49], [24.0, 24.0], "stock stays saturated");
            assert!(curtailed.iter().all(|x| *x > 0.0));
        } else {
            assert_eq!(curtailed, [0.0; 2]);
            assert_eq!(
                history[10],
                [31.0, 25.0],
                "control actually rises past the bound"
            );
            assert!(history[49].iter().all(|x| *x > 24.0));
        }
        histories.push(history);
    }
    assert_eq!(
        histories[0], histories[1],
        "omission is exactly explicit Unbounded"
    );
    assert_eq!(
        histories[2], histories[3],
        "canonical cache rebind preserves the full storage lifecycle"
    );
}

/// Continue from the GREEN rate gate using only native content. The shipyard
/// owns ordinary energy WIP allocated from the SAME generator flow. Giving it
/// a separate owned stock avoids two recipes consuming the same cell in band 0;
/// there is no additional source or permit resource. The scalar funded count
/// drives native structural products; actual delta-log births are checked below.
fn funded_output_source() -> String {
    let mut text = generator_stock_source(false);
    for (owner, site) in [("terran", "A1"), ("pirate", "E1")] {
        let child = format!("      child = {owner}_generator_1");
        assert_eq!(text.matches(&child).count(), 1);
        text = text.replace(
            &child,
            &format!(
                r#"      child = {owner}_shipyard {{ kind = Cohort name = "Shipyard energy WIP"
        owner_ref = {owner}
        property_value = {{ property = "meridian::energy" flow = 0 weight = 1 balance = 0 }}
      }}
{child}"#
            ),
        );
        let recipe = format!("    production_building = {owner}_refining");
        assert_eq!(text.matches(&recipe).count(), 1);
        text = text.replace(&recipe, &format!(r#"    production_building = {owner}_corvette_funding {{
      location = {site}
      input = {{ resource = alloys amount = 6 }}
      input = {{ entity = {owner}_shipyard property = "meridian::energy" role = balance amount = 4 }}
      output = {{ resource = corvette coefficient = 1 }}
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }}
{recipe}"#));
    }
    // Admit the shared hull column through a zero-valued existing site. This
    // carries no fleet identity or economic endowment at N0.
    text = text.replace(
        "    properties = { property = {",
        r#"    property_value = { property = "corvette::hull" Amount = 0 }
    properties = { property = {
      id = corvette_hull
      namespace = corvette
      name = hull
      sub_field = { role = Amount }
    } property = {"#,
    );
    let products = [("terran", "A1"), ("pirate", "E1")].map(|(owner, site)| format!(r#"
  structural_product = {owner}_corvettes {{
    funding = {{ entity = {site} property = "meridian_material::{site}_corvette_quantity" role = Amount }}
    count = 2
    parent = {site}
    template = {{
      kind = Fleet
      owner_ref = {owner}
      property_value = {{ property = "corvette::hull" Amount = 1 }}
      overlays = {{ modifier = {{ id = {owner}_hull targets_property = "corvette::hull" sub_field = Amount amount_mult = 2 }} }}
      children = {{
        child = crew {{ kind = Cohort }}
        child = engine {{ kind = Cohort }}
      }}
    }}
  }}
"#)).concat();
    let end = text.rfind('}').unwrap();
    text.insert_str(end, &products);
    text
}

#[test]
fn rehearsal_economy_fleet_native_funded_output_must_birth_fleets() {
    let text = funded_output_source();
    println!("NATIVE_SOURCE_BEGIN case=native-funded-birth\n{text}\nNATIVE_SOURCE_END");
    let directory = variant(&text);
    let result = ingest(&directory.path().join("stellaristhing_base.clause"));
    let profile = authored_live_profile_from_pack(&result.pack).unwrap();
    pin_profile(&profile, "native-funded-birth");
    assert_eq!(profile.game_mode.structural_products.len(), 2);
    let economy = profile.game_mode.resource_economy.as_ref().unwrap();
    assert_eq!(economy.recipes.len(), 4);
    assert!(economy.emissions.is_empty() && economy.transfers.is_empty());
    for recipe in economy.recipes.iter().filter(|r| r.id.contains("corvette")) {
        assert_eq!(recipe.inputs.len(), 2);
        assert!(recipe.inputs.iter().any(|i| i.unit_cost == 6.0
            && i.property.name.ends_with("_alloys_quantity")
            && i.role == SubFieldRole::Amount));
        assert!(recipe.inputs.iter().any(|i| i.unit_cost == 4.0
            && i.property == PropertyKey::new("meridian", "energy")
            && i.host_entity.as_ref().unwrap().ends_with("_shipyard")
            && i.role == SubFieldRole::Named("balance".into())));
        assert_eq!(recipe.max_units_per_generation.unwrap().get(), 1);
        assert_eq!(recipe.output_coefficient, 1.0);
    }
    let scenario = driver_scenario_field_bearing_from_profile(&profile).unwrap();
    let mut initial_ids = BTreeSet::new();
    identities(&scenario.root, &mut initial_ids);
    let mut session =
        SimSession::open_from_spec(scenario, &field_bearing_game_mode(&profile.game_mode)).unwrap();
    let initial_capacity = session.state.n_slots;
    let mut previous = stock_snapshot(&session, &profile, "native-funded-birth");
    assert_eq!(
        [previous[2], previous[3], previous[4], previous[5]],
        [20.0, 4.0, 14.0, 3.0]
    );
    let mut work = [0.0; 2];
    let mut produced = [0.0; 2];
    let mut first_funding = [None; 2];
    let mut births = [Vec::new(), Vec::new()];
    let mut funding_generations = [Vec::new(), Vec::new()];
    let mut cursor = 0;
    let initial_live = session.proto.allocator.live_count();
    for generation in 1..=40 {
        session
            .step_once()
            .expect("ordinary same-asset funded-output generation");
        let log = session.proto.delta_log();
        for entry in &log[cursor..] {
            match entry {
                BoundaryDeltaEntry::SimThingAdded { parent, node, .. } => {
                    let faction = if *parent == profile.install_targets["A1"][0] {
                        0
                    } else {
                        assert_eq!(*parent, profile.install_targets["E1"][0]);
                        1
                    };
                    assert!(!initial_ids.contains(&node.id().raw()));
                    births[faction].push((generation, node.id()));
                    println!(
                        "NATIVE_BIRTH generation={generation} parent={} id={} faction={faction}",
                        parent.raw(),
                        node.id().raw()
                    );
                }
                BoundaryDeltaEntry::GrowthResidencyRefused { .. } => {
                    panic!("unexpected refusal in a non-exhausting birth witness: {entry:?}")
                }
                _ => {}
            }
        }
        cursor = log.len();
        let current = stock_snapshot(&session, &profile, "native-funded-birth");
        for (faction, owner, site, alloy) in [(0, "terran", "A1", 3), (1, "pirate", "E1", 5)] {
            let total = observe_hosted_property_cell(
                &session.proto.registry,
                &session.proto.allocator,
                &AnchorTableSnapshot::from_session(&session),
                profile.install_targets[site][0],
                &PropertyKey::new("meridian_material", format!("{site}_corvette_quantity")),
                &SubFieldRole::Amount,
            )
            .unwrap();
            let made = total - produced[faction];
            let yard = format!("{owner}_shipyard");
            let energy = energy_cell(&session, &profile, &yard, "balance");
            let settled = energy_cell(&session, &profile, &yard, "balance_rate");
            assert_eq!(energy, work[faction] + settled - 4.0 * made);
            assert_eq!(current[alloy], previous[alloy] + 1.0 - 6.0 * made);
            assert!((0.0..=1.0).contains(&made));
            if made > 0.0 && first_funding[faction].is_none() {
                first_funding[faction] = Some(generation);
            }
            if made > 0.0 {
                funding_generations[faction].push(generation);
            }
            println!("FUNDING_FLOW generation={generation} owner={owner} shipyard_id={} energy_before={} settled={settled} energy_after={energy} alloys_before={} alloys_after={} scalar_funded_total={total} scalar_funded_delta={made} action_generation={:?}", profile.install_targets[&yard][0].raw(), work[faction], previous[alloy], current[alloy], session.action_band_execution_generation());
            work[faction] = energy;
            produced[faction] = total;
        }
        previous = current;
    }
    assert!(
        first_funding.iter().all(Option::is_some),
        "both factions really fund the recipe"
    );
    let mut final_ids = BTreeSet::new();
    let mut pending = vec![session.proto.root.id()];
    while let Some(id) = pending.pop() {
        assert!(final_ids.insert(id.raw()));
        for index in 0..session.proto.root.child_count(id).unwrap() {
            pending.push(session.proto.root.child_id(id, index).unwrap());
        }
    }
    let new_ids: Vec<_> = final_ids.difference(&initial_ids).copied().collect();
    for (faction, owner, site) in [(0, "terran", "A1"), (1, "pirate", "E1")] {
        assert_eq!(
            births[faction].len(),
            2,
            "both authored products actually born"
        );
        for (index, &(generation, id)) in births[faction].iter().enumerate() {
            assert_eq!(
                generation,
                funding_generations[faction][index] + 1,
                "funding at end Gk births at boundary G(k+1)"
            );
            let tree = &session.proto.root;
            let snapshot = tree.snapshot_node(id).unwrap();
            assert_eq!(snapshot.children.len(), 2);
            assert_eq!(snapshot.overlay_ids.len(), 1);
            for member in std::iter::once(id).chain(snapshot.children.iter().copied()) {
                assert!(
                    !initial_ids.contains(&member.raw()),
                    "whole subtree detached at N0"
                );
                assert_eq!(tree.owner_of(member).unwrap().as_str(), owner);
            }
            assert_eq!(
                session.proto.allocator.relation_of(id),
                Some(ObjectResidencyRelation::ChildOf(
                    profile.install_targets[site][0]
                ))
            );
            assert_eq!(
                session
                    .proto
                    .allocator
                    .committed_residency_placement(tree.id(), id)
                    .unwrap()
                    .quantity(),
                3
            );
            let hull = observe_hosted_property_cell(
                &session.proto.registry,
                &session.proto.allocator,
                &AnchorTableSnapshot::from_session(&session),
                id,
                &PropertyKey::new("corvette", "hull"),
                &SubFieldRole::Amount,
            )
            .unwrap();
            assert_eq!(
                hull,
                2.0_f32.powi((40 - generation) as i32),
                "the authored recurring hull overlay runs once per post-birth generation"
            );
            println!("BIRTH_SHAPE owner={owner} id={} children={:?} hull={hull} extent=3 parent={} generation={generation}", id.raw(), snapshot.children, profile.install_targets[site][0].raw());
        }
    }
    assert_eq!(new_ids.len(), 12);
    assert_eq!(session.proto.allocator.live_count(), initial_live + 12);
    assert_eq!(session.state.n_slots, initial_capacity);
    println!("NATIVE_BIRTH_PASS first_funding={first_funding:?} funded_total={produced:?} n0_ids={initial_ids:?} fresh_ids={new_ids:?} capacity={initial_capacity}");
    fleet_input_recovery();
    fleet_refusal_disposition();
}

/// Isolate the two fleet inputs without changing either faction's total N0
/// endowment. An authored zero claim temporarily withholds RF from the named
/// producer. A fixed ordinary overlay restores that claim at the G5 boundary.
fn fleet_input_recovery() {
    for alloy_missing in [true, false] {
        let label = if alloy_missing {
            "fleet-alloy-recovery"
        } else {
            "fleet-energy-work-recovery"
        };
        let mut text = funded_output_source();
        for (owner, initial_energy) in [("terran", 10), ("pirate", 8)] {
            if alloy_missing {
                // All bootstrap energy is in the yard, so energy-work is
                // independently sufficient while refinery output is withheld.
                let refinery = format!("property_value = {{ property = \"meridian::energy\" flow = @facility_upkeep weight = 1 balance = {initial_energy} }}");
                assert_eq!(text.matches(&refinery).count(), 1);
                text = text.replace(&refinery, "property_value = { property = \"meridian::energy\" flow = @facility_upkeep weight = 0 balance = 0 }");
                let yard = format!("child = {owner}_shipyard {{ kind = Cohort name = \"Shipyard energy WIP\"\n        owner_ref = {owner}\n        property_value = {{ property = \"meridian::energy\" flow = 0 weight = 1 balance = 0 }}");
                assert_eq!(text.matches(&yard).count(), 1);
                text = text.replace(
                    &yard,
                    &yard.replace("balance = 0", &format!("balance = {initial_energy}")),
                );
            } else {
                let yard = format!("child = {owner}_shipyard {{ kind = Cohort name = \"Shipyard energy WIP\"\n        owner_ref = {owner}\n        property_value = {{ property = \"meridian::energy\" flow = 0 weight = 1 balance = 0 }}");
                assert_eq!(text.matches(&yard).count(), 1);
                text = text.replace(&yard, &yard.replace("weight = 1", "weight = 0"));
            }
        }
        println!(
            "FLEET_RECOVERY_SOURCE case={label} identity={}\n{text}\nFLEET_RECOVERY_SOURCE_END",
            clause_source_content_identity(text.as_bytes())
        );
        let directory = variant(&text);
        let result = ingest(&directory.path().join("stellaristhing_base.clause"));
        let profile = authored_live_profile_from_pack(&result.pack).unwrap();
        pin_profile(&profile, label);
        let mut session = SimSession::open_from_spec(
            driver_scenario_field_bearing_from_profile(&profile).unwrap(),
            &field_bearing_game_mode(&profile.game_mode),
        )
        .unwrap();
        let mut expected_alloy = [4.0_f32, 3.0];
        let mut expected_mineral = [20.0_f32, 14.0];
        let mut expected_refinery = if alloy_missing { [0.0; 2] } else { [10.0, 8.0] };
        let mut expected_work = if alloy_missing { [10.0, 8.0] } else { [0.0; 2] };
        let mut funded = [Vec::new(), Vec::new()];
        let mut births = [Vec::new(), Vec::new()];
        let mut cursor = 0;
        for generation in 1..=22 {
            if generation == 5 {
                for owner in ["terran", "pirate"] {
                    let producer = if alloy_missing {
                        "refinery"
                    } else {
                        "shipyard"
                    };
                    let target = profile.install_targets[&format!("{owner}_{producer}")][0];
                    session
                        .tx
                        .submit_boundary(BoundaryRequest::AttachOverlay {
                            target,
                            source_generation: GenerationStamp::new(
                                session.coord.day_index().try_into().unwrap(),
                            ),
                            overlay: Overlay {
                                id: OverlayId::new(),
                                kind: OverlayKind::Policy,
                                source: OverlaySource::System,
                                origin: target,
                                affects: vec![target],
                                lifecycle: OverlayLifecycle::UntilDissolved,
                                transform: PropertyTransformDelta {
                                    property_id: session
                                        .proto
                                        .registry
                                        .id_of("meridian", "energy")
                                        .unwrap(),
                                    sub_field_deltas: vec![(
                                        SubFieldRole::Named("weight".into()),
                                        TransformOp::set(1.0),
                                    )],
                                },
                            },
                        })
                        .unwrap();
                }
            }
            session.step_once().unwrap();
            for entry in &session.proto.delta_log()[cursor..] {
                match entry {
                    BoundaryDeltaEntry::SimThingAdded { parent, node, .. } => {
                        let index = if *parent == profile.install_targets["A1"][0] {
                            0
                        } else {
                            assert_eq!(*parent, profile.install_targets["E1"][0]);
                            1
                        };
                        births[index].push((generation, node.id()));
                    }
                    BoundaryDeltaEntry::GrowthResidencyRefused { .. } => {
                        panic!("unexpected recovery refusal: {entry:?}")
                    }
                    _ => {}
                }
            }
            cursor = session.proto.delta_log().len();
            let stocks = stock_snapshot(&session, &profile, label);
            for (index, owner, site) in [(0, "terran", "A1"), (1, "pirate", "E1")] {
                let made = if expected_alloy[index] >= 6.0 && expected_work[index] >= 4.0 {
                    1.0
                } else {
                    0.0
                };
                let refined = if expected_mineral[index] >= 2.0 && expected_refinery[index] >= 1.0 {
                    1.0
                } else {
                    0.0
                };
                if generation <= 5 {
                    assert_eq!(made, 0.0, "missing input forbids funding");
                    assert!(births[index].is_empty());
                    if alloy_missing {
                        assert!(expected_work[index] >= 4.0 && expected_alloy[index] < 6.0);
                    } else if generation >= 4 {
                        assert!(
                            expected_alloy[index] >= 6.0 && expected_work[index] == 0.0,
                            "alloy is sufficient; missing energy-work alone stops funding"
                        );
                    }
                }
                let refinery_rate = if generation <= 5 {
                    if alloy_missing {
                        0.0
                    } else {
                        2.0
                    }
                } else {
                    1.0
                };
                let yard_rate = 2.0 - refinery_rate;
                expected_mineral[index] += 3.0 - 2.0 * refined;
                expected_alloy[index] += refined - 6.0 * made;
                expected_refinery[index] += refinery_rate - refined;
                expected_work[index] += yard_rate - 4.0 * made;
                if made > 0.0 {
                    funded[index].push(generation);
                }
                assert_eq!(stocks[2 + index * 2], expected_mineral[index]);
                assert_eq!(stocks[3 + index * 2], expected_alloy[index]);
                assert_eq!(stocks[index], 0.0, "no second owner stock");
                assert_eq!(
                    energy_cell(&session, &profile, &format!("{owner}_refinery"), "balance"),
                    expected_refinery[index]
                );
                assert_eq!(
                    energy_cell(&session, &profile, &format!("{owner}_shipyard"), "balance"),
                    expected_work[index]
                );
                let actual_funded = observe_hosted_property_cell(
                    &session.proto.registry,
                    &session.proto.allocator,
                    &AnchorTableSnapshot::from_session(&session),
                    profile.install_targets[site][0],
                    &PropertyKey::new("meridian_material", format!("{site}_corvette_quantity")),
                    &SubFieldRole::Amount,
                )
                .unwrap();
                assert_eq!(actual_funded, funded[index].len() as f32);
                assert_eq!(
                    births[index].len(),
                    funded[index]
                        .iter()
                        .filter(|g| **g < generation)
                        .count()
                        .min(2),
                    "exact N+0.5 funded birth, no duplicate or same-generation completion"
                );
                println!("FLEET_RECOVERY case={label} owner={owner} generation={generation} refined={refined} funded_delta={made} funded_total={actual_funded} births={} minerals={} alloys={} refinery_energy={} yard_energy={}",
                    births[index].len(), expected_mineral[index], expected_alloy[index], expected_refinery[index], expected_work[index]);
            }
        }
        for index in 0..2 {
            assert_eq!(
                births[index].len(),
                2,
                "both authored births recover by the finite horizon"
            );
            assert!(funded[index][0] > 5);
            assert_ne!(births[index][0].1, births[index][1].1);
            for (birth, funding) in births[index].iter().zip(&funded[index]) {
                assert_eq!(birth.0, funding + 1);
            }
        }
    }
}

/// Consume the already-admitted 2.1 refusal disposition: materials consumed by
/// a recipe stay consumed if placement refuses; the scalar output is a receipt,
/// not a reservation or completed fleet. The refused instance never retries.
/// No enrolled subtree is removed to manufacture capacity or a restart.
fn fleet_refusal_disposition() {
    let text = funded_output_source().replace("    count = 2\n", "    count = 50\n");
    assert_eq!(text.matches("    count = 50\n").count(), 2);
    println!(
        "FLEET_REFUSAL_SOURCE identity={}\n{text}\nFLEET_REFUSAL_SOURCE_END",
        clause_source_content_identity(text.as_bytes())
    );
    let directory = variant(&text);
    let result = ingest(&directory.path().join("stellaristhing_base.clause"));
    let profile = authored_live_profile_from_pack(&result.pack).unwrap();
    pin_profile(&profile, "fleet-refusal");
    let mut session = SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(&profile).unwrap(),
        &field_bearing_game_mode(&profile.game_mode),
    )
    .unwrap();
    let root = session.proto.root.id();
    let initial_live = session.proto.allocator.live_count();
    let initial_capacity = session.proto.allocator.growth_capacity_available(root);
    let mut alloy = [4.0_f32, 3.0];
    let mut work = [0.0_f32; 2];
    let mut funded = [0usize; 2];
    let mut born = [Vec::new(), Vec::new()];
    let mut refused = [BTreeSet::new(), BTreeSet::new()];
    let mut cursor = 0;
    for generation in 1..=80 {
        let capacity_before = session.proto.allocator.growth_capacity_available(root);
        let placements_before: Vec<_> = born
            .iter()
            .flatten()
            .map(|&id| {
                (
                    id,
                    session
                        .proto
                        .allocator
                        .committed_residency_placement(root, id)
                        .unwrap(),
                )
            })
            .collect();
        session.step_once().unwrap();
        for (id, placement) in placements_before {
            assert_eq!(
                session
                    .proto
                    .allocator
                    .committed_residency_placement(root, id),
                Some(placement),
                "the complete prior placement survives every later refusal"
            );
        }
        for entry in &session.proto.delta_log()[cursor..] {
            let parent = match entry {
                BoundaryDeltaEntry::SimThingAdded { parent, .. } => *parent,
                BoundaryDeltaEntry::GrowthResidencyRefused {
                    fact: RecordedGrowthResidencyFact::Refused(fact),
                } => fact.candidate().structural_parent(),
                _ => continue,
            };
            let index = if parent == profile.install_targets["A1"][0] {
                0
            } else {
                assert_eq!(parent, profile.install_targets["E1"][0]);
                1
            };
            match entry {
                BoundaryDeltaEntry::SimThingAdded {
                    node, residency, ..
                } => {
                    assert_eq!(residency.placement().quantity(), 3);
                    assert!(!refused[index].contains(&node.id()));
                    born[index].push(node.id());
                }
                BoundaryDeltaEntry::GrowthResidencyRefused {
                    fact: RecordedGrowthResidencyFact::Refused(fact),
                } => {
                    assert!(capacity_before < 3);
                    assert_eq!(
                        fact.reason(),
                        &OrdinaryGrowthRefusalReason::MarketUnresolved {
                            granted: capacity_before
                        }
                    );
                    assert_eq!(fact.candidate().quantity(), 3);
                    assert!(
                        refused[index].insert(fact.candidate().grantee()),
                        "a refused instance never retries"
                    );
                    assert!(!session.proto.root.contains_id(fact.candidate().grantee()));
                    assert!(session
                        .proto
                        .allocator
                        .slot_of(fact.candidate().grantee())
                        .is_none());
                    assert!(session
                        .proto
                        .allocator
                        .committed_residency_placement(root, fact.candidate().grantee())
                        .is_none());
                    println!("FLEET_REFUSAL generation={generation} faction={index} fact={fact:?}");
                }
                _ => unreachable!(),
            }
        }
        cursor = session.proto.delta_log().len();
        let stocks = stock_snapshot(&session, &profile, "fleet-refusal");
        for (index, owner, site, mineral0, energy0) in [
            (0, "terran", "A1", 20.0, 10.0),
            (1, "pirate", "E1", 14.0, 8.0),
        ] {
            assert_eq!(
                born[index].len() + refused[index].len(),
                funded[index],
                "each prior funded unit has exactly one explicit birth/refusal disposition"
            );
            let made = if alloy[index] >= 6.0 && work[index] >= 4.0 {
                1.0
            } else {
                0.0
            };
            alloy[index] += 1.0 - 6.0 * made;
            work[index] += 1.0 - 4.0 * made;
            funded[index] += made as usize;
            assert_eq!(stocks[2 + index * 2], mineral0 + generation as f32);
            assert_eq!(
                stocks[3 + index * 2],
                alloy[index],
                "exact debit persists on refusal; no fabricated refund"
            );
            assert_eq!(
                energy_cell(&session, &profile, &format!("{owner}_refinery"), "balance"),
                energy0
            );
            assert_eq!(
                energy_cell(&session, &profile, &format!("{owner}_shipyard"), "balance"),
                work[index]
            );
            let receipt = observe_hosted_property_cell(
                &session.proto.registry,
                &session.proto.allocator,
                &AnchorTableSnapshot::from_session(&session),
                profile.install_targets[site][0],
                &PropertyKey::new("meridian_material", format!("{site}_corvette_quantity")),
                &SubFieldRole::Amount,
            )
            .unwrap();
            assert_eq!(receipt, funded[index] as f32);
            for &id in &refused[index] {
                assert!(!session.proto.root.contains_id(id));
                assert!(session
                    .proto
                    .allocator
                    .committed_residency_placement(root, id)
                    .is_none());
            }
            println!("FLEET_DISPOSITION generation={generation} owner={owner} funded_receipt={receipt} born={} refused={} pending={made} alloys={} energy_work={}",
                born[index].len(), refused[index].len(), alloy[index], work[index]);
        }
        let total_born = born.iter().map(Vec::len).sum::<usize>();
        assert_eq!(
            session.proto.allocator.live_count(),
            initial_live + 3 * total_born
        );
        assert_eq!(
            session.proto.allocator.growth_capacity_available(root),
            initial_capacity - 3 * total_born as u32
        );
    }
    assert!(born.iter().all(|v| !v.is_empty()));
    assert!(
        refused.iter().all(|v| !v.is_empty()),
        "both factions encounter explicit funded refusals"
    );
}

/// The first post-birth economy floor: a genuinely born fleet carrying the
/// SAME energy property must join its participating spatial parent's RF tree.
/// This discriminates structural membership from economic participation. No
/// test-side arena enrollment, accumulator install, or readback feedback occurs.
#[test]
fn rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow() {
    struct UpkeepCase {
        label: &'static str,
        flow: i32,
        expected_surplus: f32,
    }
    const CASES: [UpkeepCase; 2] = [
        UpkeepCase {
            label: "zero-upkeep-control",
            flow: 0,
            expected_surplus: 2.0,
        },
        UpkeepCase {
            label: "one-energy-upkeep",
            flow: -1,
            expected_surplus: 1.0,
        },
    ];
    let mut failures = Vec::new();
    for case in CASES {
        let text = funded_output_source().replace(
            "      kind = Fleet",
            &format!("      kind = Fleet\n      property_value = {{ property = \"meridian::energy\" flow = {} weight = 0 balance = 0 }}", case.flow),
        );
        println!(
            "NATIVE_SOURCE_BEGIN case={}\n{text}\nNATIVE_SOURCE_END",
            case.label
        );
        let directory = variant(&text);
        let result = ingest(&directory.path().join("stellaristhing_base.clause"));
        let profile = authored_live_profile_from_pack(&result.pack).unwrap();
        pin_profile(&profile, case.label);
        let mut session = SimSession::open_from_spec(
            driver_scenario_field_bearing_from_profile(&profile).unwrap(),
            &field_bearing_game_mode(&profile.game_mode),
        )
        .unwrap();
        let energy_id = session.proto.registry.id_of("meridian", "energy").unwrap();
        let arena = session
            .spec_state
            .arena_registry
            .arenas
            .iter()
            .position(|a| a.flow_property_id == energy_id)
            .unwrap() as u32;
        let mut born = Vec::new();
        let mut cursor = 0;
        for generation in 1..=8 {
            let registry_generation = session.spec_state.arena_registry.generation;
            session.step_once().unwrap();
            let log = session.proto.delta_log();
            let mut born_this_boundary = 0;
            for entry in &log[cursor..] {
                if let BoundaryDeltaEntry::SimThingAdded { parent, node, .. } = entry {
                    born_this_boundary += 1;
                    born.push((*parent, node.id()));
                    let registry = &session.spec_state.arena_registry;
                    let member = registry
                        .participants
                        .iter()
                        .find(|p| p.arena_idx == arena && p.subtree_root == node.id())
                        .expect("automatic admission at the successful birth boundary");
                    assert_eq!(member.parent, Some(*parent));
                    assert_eq!(
                        Some(member.slot),
                        session.proto.allocator.slot_of(node.id())
                    );
                    for child in session
                        .proto
                        .root
                        .snapshot_node(node.id())
                        .unwrap()
                        .children
                    {
                        assert!(
                            !registry
                                .participants
                                .iter()
                                .any(|p| p.subtree_root == child),
                            "non-carrier child must not inherit RF membership"
                        );
                    }
                    println!(
                        "UPKEEP_BIRTH case={} generation={generation} parent={} id={}",
                        case.label,
                        parent.raw(),
                        node.id().raw()
                    );
                }
            }
            if born_this_boundary > 0 {
                let report = session
                    .last_resource_flow_structural_enrollment_report
                    .as_ref()
                    .unwrap();
                assert!(report.refusals.is_empty());
                assert_eq!(report.admissions.len(), born_this_boundary);
                assert_eq!(report.generation_before, registry_generation);
                assert_eq!(report.generation_after, registry_generation + 1);
                println!("UPKEEP_BIRTH_ENROLLMENT case={} generation={generation} non_carriers_excluded=true report={report:?}", case.label);
            }
            cursor = log.len();
            for owner in ["terran", "pirate"] {
                let refinery = energy_cell(
                    &session,
                    &profile,
                    &format!("{owner}_refinery"),
                    "balance_rate",
                );
                let yard = energy_cell(
                    &session,
                    &profile,
                    &format!("{owner}_shipyard"),
                    "balance_rate",
                );
                println!("UPKEEP_SETTLEMENT case={} generation={generation} owner={owner} refinery_rate={refinery} yard_rate={yard} surplus={} live={}", case.label, refinery + yard, session.proto.allocator.live_count());
                // All splits in this fixture are dyadic, so equality is exact.
                if generation == 8 && refinery + yard != case.expected_surplus {
                    failures.push(format!(
                        "{}/{owner}: G8 net spendable energy {}, expected {} after one born fleet",
                        case.label,
                        refinery + yard,
                        case.expected_surplus
                    ));
                }
            }
        }
        assert_eq!(born.len(), 2, "one genuinely new fleet per faction by G8");
        for (parent, id) in born {
            let flow = observe_hosted_property_cell(
                &session.proto.registry,
                &session.proto.allocator,
                &AnchorTableSnapshot::from_session(&session),
                id,
                &PropertyKey::new("meridian", "energy"),
                &SubFieldRole::Named("flow".into()),
            )
            .unwrap();
            assert_eq!(
                flow, case.flow as f32,
                "authored born flow is installed and observable"
            );
            let registry = &session.spec_state.arena_registry;
            assert!(
                registry
                    .participants
                    .iter()
                    .any(|p| p.arena_idx == arena && p.subtree_root == parent),
                "spatial parent is an existing energy participant"
            );
            let members: Vec<_> = registry
                .participants
                .iter()
                .filter(|p| p.subtree_root == id)
                .collect();
            println!("UPKEEP_MEMBERSHIP case={} born={} parent={} slot={:?} authored_observed_flow={flow} energy_arena={arena} members={members:?}", case.label, id.raw(), parent.raw(), session.proto.allocator.slot_of(id));
            if !members
                .iter()
                .any(|p| p.arena_idx == arena && p.parent == Some(parent))
            {
                failures.push(format!(
                    "{}/{}: born energy property is absent from parent RF arena",
                    case.label,
                    id.raw()
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "2.2 STOP: native born properties do not enter ongoing RF upkeep: {failures:#?}"
    );
    fleet_alloy_stock_ingress();
}

/// Admission checkpoint for the full alloy-upkeep composition, not a claim to
/// have executed upkeep. Keep one authored alloy Balance as both the refinery
/// target and shipyard input. Canonical input loci already admit this shape;
/// the native output door must reach the same existing stock without a second
/// generated quantity, a spec patch, or a test-side transfer/credit authority.
fn fleet_alloy_stock_ingress() {
    let mut text = funded_output_source();
    let declaration = "      id = corvette_hull";
    assert_eq!(text.matches(declaration).count(), 1);
    text = text.replace(
        declaration,
        r#"      id = meridian_alloys
      namespace = meridian
      name = alloys
      sub_field = { role = balance_rate }
      sub_field = { role = balance governed_by = balance_rate accumulator = Balance }
    } property = {
      id = corvette_hull"#,
    );
    for (site, initial) in [("A1", 4), ("E1", 3)] {
        let old_stock = format!("property_value = {{ property = \"meridian_material::{site}_alloys_quantity\" Amount = {initial} }}");
        assert_eq!(text.matches(&old_stock).count(), 1);
        text = text.replace(&old_stock, &format!("property_value = {{ property = \"meridian::alloys\" balance_rate = 0 balance = {initial} }}"));
        text = text.replacen("input = { resource = alloys amount = 6 }",
            &format!("input = {{ entity = {site} property = \"meridian::alloys\" role = balance amount = 6 }}"), 1);
        text = text.replacen("output = { resource = alloys coefficient = @refinery_output }",
            &format!("output = {{ entity = {site} property = \"meridian::alloys\" role = balance coefficient = @refinery_output }}"), 1);
    }
    assert_eq!(
        text.matches("role = balance coefficient = @refinery_output")
            .count(),
        2
    );
    println!(
        "ALLOY_STOCK_INGRESS_SOURCE identity={}\n{text}\nALLOY_STOCK_INGRESS_SOURCE_END",
        clause_source_content_identity(text.as_bytes())
    );
    let directory = variant(&text);
    let result = ingest_clause_scenario_path(&directory.path().join("stellaristhing_base.clause"),
        &ClauseScenarioIngestOptions::default()).unwrap_or_else(|error| panic!(
            "2.2 STOP: canonical refinery output cannot target the existing sole alloy Balance through native authoring; full 0.2-alloy upkeep remains unexecuted: {error:?}"));
    let economy = result.pack.game_mode.resource_economy.as_ref().unwrap();
    for (owner, site) in [("terran", "A1"), ("pirate", "E1")] {
        let recipe = economy
            .recipes
            .iter()
            .find(|r| r.id.ends_with(&format!("{owner}_refining")))
            .unwrap();
        assert_eq!(recipe.target, PropertyKey::new("meridian", "alloys"));
        assert_eq!(recipe.target_role, SubFieldRole::Named("balance".into()));
        assert_eq!(recipe.target_host_entity.as_deref(), Some(site));
        assert_eq!(recipe.output_coefficient, 1.0);
        let funding = economy
            .recipes
            .iter()
            .find(|r| r.id.ends_with(&format!("{owner}_corvette_funding")))
            .unwrap();
        assert!(funding.inputs.iter().any(|i| i.property == recipe.target
            && i.role == recipe.target_role
            && i.host_entity == recipe.target_host_entity
            && i.unit_cost == 6.0));
    }
}
