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
    clause_source_content_identity, ingest_clause_scenario_path, ClauseScenarioIngestOptions,
    ClauseScenarioIngestResult,
};
use simthing_mapeditor::studio_live_session_bridge::{
    authored_live_profile_from_pack, driver_scenario_field_bearing_from_profile,
    field_bearing_game_mode, StudioAuthoredLiveProfile,
};
use simthing_sim::BoundaryDeltaEntry;
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
}
