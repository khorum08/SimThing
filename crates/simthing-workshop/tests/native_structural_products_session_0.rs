//! Native funded structural products through the ORDINARY session (DA, relay
//! 5737649159). The shipped two-faction scenario is re-authored in a temp copy
//! with a shipyard recipe whose funded output births a source-authored,
//! detached fleet subtree through the existing 2.1 ActionBand -> AddChild door.
use simthing_core::{ObjectResidencyRelation, SimThingId};
use simthing_driver::{build_execution_plan, SimSession};
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::load_studio_session_from_scenario_path;
use simthing_mapeditor::studio_live_session_bridge::{
    driver_scenario_field_bearing_from_profile, field_bearing_game_mode,
    StudioAuthoredLiveProfile,
};
use simthing_sim::BoundaryDeltaEntry;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const SHIPYARDS: &str = r#"
    production_building = terran_shipyard {
      location = A1
      input = { resource = alloys amount = COST }
      output = { resource = corvettes coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
    production_building = pirate_shipyard {
      location = E1
      input = { resource = alloys amount = COST }
      output = { resource = corvettes coefficient = 1 }
      throttle_hint_max_per_tick = 1
      max_units_per_generation = 1
    }
"#;

fn product(owner: &str, site: &str, count: u32, crew: usize) -> String {
    let crew = (0..crew)
        .map(|index| format!("child = crew_{index} {{ kind = Cohort }}"))
        .collect::<Vec<_>>()
        .join("\n        ");
    format!(
        r#"
  structural_product = {owner}_corvettes {{
    funding = {{ entity = {site} property = "meridian_material::{site}_corvettes_quantity" role = Amount }}
    count = {count}
    parent = {site}
    template = {{
      kind = Fleet
      owner_ref = {owner}
      property_value = {{ property = "meridian_material::{site}_alloys_quantity" Amount = 1 }}
      overlays = {{ modifier = {{ id = {owner}_corvette_hull targets_property = "meridian_material::{site}_alloys_quantity" sub_field = Amount amount_mult = 2 }} }}
      children = {{
        {crew}
      }}
    }}
  }}
"#
    )
}

/// The shipped source plus both shipyards and the given product declarations.
fn source(products: &[String], shipyard_cost: u32) -> String {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    let economy_end = "      throttle_hint_max_per_tick = 1\n    }\n  }\n}";
    assert_eq!(text.matches(economy_end).count(), 1, "shipped economy tail");
    let shipyards = SHIPYARDS.replace("COST", &shipyard_cost.to_string());
    text.replace(
        economy_end,
        &format!(
            "      throttle_hint_max_per_tick = 1\n    }}\n{shipyards}  }}\n{}}}",
            products.concat()
        ),
    )
}

struct Live {
    sim: SimSession,
    targets: std::collections::HashMap<String, Vec<SimThingId>>,
    products_json: String,
    cached_products_json: String,
    cache: PathBuf,
    _dir: Option<tempfile::TempDir>,
}

fn session_from(profile: &StudioAuthoredLiveProfile) -> Result<SimSession, String> {
    SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(profile)?,
        &field_bearing_game_mode(&profile.game_mode),
    )
    .map_err(|error| format!("{error:?}"))
}

fn products_json(profile: &StudioAuthoredLiveProfile) -> String {
    serde_json::to_string(&profile.game_mode.structural_products).unwrap()
}

/// The ordinary native path: parse -> expand -> hydrate -> profile -> session,
/// plus the derived canonical cache re-loaded for rebind fidelity.
fn open(text: &str) -> Result<Live, String> {
    let dir = tempfile::tempdir().unwrap();
    let sources = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    for file in [
        "stellaristhing_base.base.json",
        "stellaristhing_base.dependencies.json",
    ] {
        std::fs::copy(sources.join(file), dir.path().join(file)).unwrap();
    }
    let source = dir.path().join("stellaristhing_base.clause");
    std::fs::write(&source, text).unwrap();
    let cache = dir.path().join("witness.simthing-scenario.json");
    let (_, native) = load_clause_studio_session_from_path(
        &source,
        &ClauseScenarioIngestOptions::default(),
        &cache,
        None,
    )
    .map_err(|error| format!("{error:?}"))?;
    let profile = native
        .authored_live_profile
        .as_ref()
        .ok_or("native load produced no live profile")?;
    let cached = load_studio_session_from_scenario_path(&cache, None)
        .map_err(|error| format!("{error:?}"))?;
    Ok(Live {
        sim: session_from(profile)?,
        targets: profile.install_targets.clone(),
        products_json: products_json(profile),
        cached_products_json: products_json(cached.authored_live_profile.as_ref().unwrap()),
        cache,
        _dir: Some(dir),
    })
}

fn all_ids(live: &Live) -> BTreeSet<SimThingId> {
    let tree = &live.sim.proto.root;
    let mut ids = BTreeSet::new();
    let mut pending = vec![tree.id()];
    while let Some(id) = pending.pop() {
        ids.insert(id);
        pending.extend(tree.snapshot_node(id).unwrap().children);
    }
    ids
}

/// Run `generations`; return per-generation (parent, born id) pairs read from
/// the cumulative boundary delta log, plus the typed growth refusals.
fn run(live: &mut Live, generations: u32) -> (Vec<Vec<(SimThingId, SimThingId)>>, usize) {
    let mut cursor = 0;
    let mut per_generation = Vec::new();
    let mut refusals = 0;
    for _ in 0..generations {
        live.sim.step_once().expect("ordinary generation");
        let log = live.sim.proto.delta_log();
        let fresh = &log[cursor..];
        cursor = log.len();
        per_generation.push(
            fresh
                .iter()
                .filter_map(|entry| match entry {
                    BoundaryDeltaEntry::SimThingAdded { parent, node, .. } => {
                        Some((*parent, node.id()))
                    }
                    _ => None,
                })
                .collect(),
        );
        refusals += fresh
            .iter()
            .filter(|entry| matches!(entry, BoundaryDeltaEntry::GrowthResidencyRefused { .. }))
            .count();
    }
    (per_generation, refusals)
}

/// (generation, born id) in birth order.
fn born(per_generation: &[Vec<(SimThingId, SimThingId)>]) -> Vec<(usize, SimThingId)> {
    per_generation
        .iter()
        .enumerate()
        .flat_map(|(generation, births)| births.iter().map(move |&(_, id)| (generation + 1, id)))
        .collect()
}

/// Kind-free structural shape of one born subtree: (subtree size, overlays on
/// the born root, resolved owner, committed placement quantity).
fn shape(live: &Live, id: SimThingId) -> (usize, usize, String, u32) {
    let tree = &live.sim.proto.root;
    let mut size = 0;
    let mut pending = vec![id];
    while let Some(node) = pending.pop() {
        size += 1;
        pending.extend(tree.snapshot_node(node).unwrap().children);
    }
    (
        size,
        tree.snapshot_node(id).unwrap().overlay_ids.len(),
        tree.owner_of(id).unwrap().as_str().to_string(),
        live.sim
            .proto
            .allocator
            .committed_residency_placement(tree.id(), id)
            .expect("ordinary committed residency placement")
            .quantity(),
    )
}

/// catches: a funded crossing that births nothing, births twice, births before
/// funding, loses the template's children/properties/overlays/owner, skips
/// ordinary residency placement, exceeds the authored bounded count, or means
/// something different after a canonical-cache rebind.
#[test]
fn funded_output_births_exactly_the_authored_detached_subtree() {
    let mut live = open(&source(&[product("terran", "A1", 2, 2)], 2))
        .expect("the funded product admits through the ordinary session");
    assert_eq!(
        live.products_json, live.cached_products_json,
        "the canonical cache carries the same declaration"
    );
    let n0 = all_ids(&live);
    let a1 = live.targets["A1"][0];
    let cargo = live
        .sim
        .proto
        .registry
        .id_of("meridian_material", "A1_alloys_quantity")
        .unwrap();
    let (per_generation, refusals) = run(&mut live, 8);
    assert_eq!(refusals, 0, "placement is available");
    assert!(
        per_generation[0].is_empty(),
        "nothing is born before the funded crossing dispatches"
    );
    let births = born(&per_generation);
    assert_eq!(
        births.iter().map(|(generation, _)| *generation).collect::<Vec<_>>(),
        vec![2, 3],
        "exactly the authored count, one birth per funded crossing, none after"
    );
    for (_, id) in &births {
        assert!(!n0.contains(id), "a genuinely fresh identity");
        assert!(per_generation.iter().flatten().all(|(parent, _)| *parent == a1));
        assert_eq!(
            shape(&live, *id),
            (3, 1, "terran".to_string(), 3),
            "the template's meaning is born whole"
        );
        assert!(live
            .sim
            .proto
            .root
            .snapshot_node(*id)
            .unwrap()
            .property_ids
            .contains(&cargo));
        assert_eq!(
            live.sim.proto.allocator.relation_of(*id),
            Some(ObjectResidencyRelation::ChildOf(a1))
        );
    }

    // The derived canonical cache is not a second authority: a session built
    // from it births the same product meaning on the same generations.
    let cached = load_studio_session_from_scenario_path(&live.cache, None).unwrap();
    let profile = cached.authored_live_profile.as_ref().unwrap();
    let mut rebound = Live {
        sim: session_from(profile).expect("cache rebind admits"),
        targets: profile.install_targets.clone(),
        products_json: products_json(profile),
        cached_products_json: String::new(),
        cache: live.cache.clone(),
        _dir: None,
    };
    let (rebound_births, _) = run(&mut rebound, 8);
    let rebound_births = born(&rebound_births);
    assert_eq!(
        rebound_births.iter().map(|(generation, _)| *generation).collect::<Vec<_>>(),
        vec![2, 3]
    );
    for (_, id) in &rebound_births {
        assert_eq!(shape(&rebound, *id), (3, 1, "terran".to_string(), 3));
    }
}

/// catches: a product born without its funding, or a fabricated child or
/// hidden placement when ordinary residency placement is unavailable.
#[test]
fn unfunded_or_unplaceable_products_birth_nothing() {
    let mut unfunded = open(&source(&[product("terran", "A1", 2, 2)], 100)).unwrap();
    let n0 = all_ids(&unfunded);
    let (births, refusals) = run(&mut unfunded, 6);
    assert!(births.iter().all(Vec::is_empty) && refusals == 0, "no funding, no birth");
    assert_eq!(all_ids(&unfunded), n0, "no identity appears without funding");

    // A subtree larger than the free residency extent: the ordinary growth
    // door refuses it typed, and nothing is fabricated.
    let mut unplaceable = open(&source(&[product("terran", "A1", 1, 80)], 2)).unwrap();
    let n0 = all_ids(&unplaceable);
    let (births, refusals) = run(&mut unplaceable, 6);
    println!("unplaceable: births {births:?} refusals {refusals}");
    assert!(births.iter().all(Vec::is_empty), "no fabricated subtree");
    assert!(refusals >= 1, "placement unavailability is a typed growth refusal");
    assert_eq!(all_ids(&unplaceable), n0, "no hidden placement or partial subtree");
}

/// catches: authored declaration order (or the physical order it implies)
/// changing which products are born, when, or with what structure.
#[test]
fn product_meaning_is_independent_of_declaration_order() {
    let terran = product("terran", "A1", 2, 1);
    let pirate = product("pirate", "E1", 2, 1);
    let mut outcomes = Vec::new();
    for order in [[&terran, &pirate], [&pirate, &terran]] {
        let mut live = open(&source(&[order[0].clone(), order[1].clone()], 2)).unwrap();
        let (a1, e1) = (live.targets["A1"][0], live.targets["E1"][0]);
        let (per_generation, refusals) = run(&mut live, 6);
        assert_eq!(refusals, 0);
        let mut outcome = Vec::new();
        for (generation, births) in per_generation.iter().enumerate() {
            for &(parent, id) in births {
                let site = if parent == a1 {
                    "A1"
                } else if parent == e1 {
                    "E1"
                } else {
                    "?"
                };
                outcome.push((generation + 1, site, shape(&live, id)));
            }
        }
        outcome.sort_by(|left, right| (left.0, left.1).cmp(&(right.0, right.1)));
        println!("declaration-order outcome: {outcome:?}");
        outcomes.push(outcome);
    }
    assert_eq!(outcomes[0], outcomes[1], "declaration order carries no meaning");
    assert_eq!(outcomes[0].len(), 4, "two births per faction");
}

/// catches: an unknown or ambiguous funding locus, host, parent, owner, or
/// template property, or an owner seat used as a spatial parent, being
/// retargeted, defaulted, or dropped instead of refused before activation.
#[test]
fn malformed_product_declarations_refuse_before_activation() {
    let good = product("terran", "A1", 2, 1);
    let cases: Vec<(String, &str)> = vec![
        (
            good.replace("A1_corvettes_quantity\"", "A1_frigates_quantity\""),
            "is not registered",
        ),
        (
            good.replace("role = Amount }", "role = reserve }"),
            "is not a scalar sub-field",
        ),
        (
            good.replace("funding = { entity = A1", "funding = { entity = nowhere"),
            "not in install_targets",
        ),
        (
            good.replace("parent = A1", "parent = nowhere"),
            "not in install_targets",
        ),
        (good.replace("parent = A1", "parent = terran"), "is an owner seat"),
        (
            good.replace("owner_ref = terran", "owner_ref = nobody"),
            "unknown owner_ref",
        ),
        (
            good.replace("count = 2", "count = 0"),
            "count must be a positive integer",
        ),
        (
            good.replace("funding = { entity = A1", "funding = { entity = E1"),
            "does not carry",
        ),
        (
            good.replace(
                "A1_alloys_quantity\" Amount = 1",
                "A1_rubble_quantity\" Amount = 1",
            ),
            "is not registered",
        ),
        (
            format!(
                "{good}{}",
                good.replace("terran_corvette_hull", "terran_corvette_hull_b")
            ),
            "duplicate structural_product id",
        ),
    ];
    for (declaration, expected) in cases {
        assert_ne!(declaration, good, "the case must alter the declaration");
        let error = match open(&source(&[declaration.clone()], 2)) {
            Ok(_) => panic!("must refuse before activation: {declaration}"),
            Err(error) => error,
        };
        println!("refused ({expected}): {error}");
        assert!(error.contains(expected), "{expected}: {error}");
    }
}

/// Two powered corvettes: the fleet and its reactor carry energy, the crew
/// carries none. An interior fleet participates with its children's upsweep
/// weight, so reactor weight 2 keeps A1's split dyadic (4 x 1 + 2 x 2): f32
/// settlement is then exact and the comparison below can be exact.
fn powered_product(flow: i32) -> String {
    format!(
        r#"
  structural_product = terran_corvettes {{
    funding = {{ entity = A1 property = "meridian_material::A1_corvettes_quantity" role = Amount }}
    count = 2
    parent = A1
    template = {{
      kind = Fleet
      owner_ref = terran
      property_value = {{ property = "meridian::energy" flow = {flow} weight = 1 }}
      children = {{
        child = reactor {{ kind = Cohort
          property_value = {{ property = "meridian::energy" flow = 0 weight = 2 }}
        }}
        child = crew {{ kind = Cohort }}
      }}
    }}
  }}
"#
    )
}

/// One generation's settled Balance deltas summed across `meridian_energy`,
/// each delta taken and summed in f64 so the harness adds no rounding.
fn settled_energy(live: &mut Live) -> f64 {
    let session = &live.sim;
    let arena = session
        .spec_state
        .arena_registry
        .arenas
        .iter()
        .position(|arena| arena.name == "meridian_energy")
        .unwrap() as u32;
    let layout = build_execution_plan(&session.proto.registry, &session.spec_state.arena_registry)
        .unwrap()
        .arenas
        .into_iter()
        .find(|layout| layout.arena_idx == arena)
        .unwrap();
    let n_dims = session.proto.registry.total_columns as u32;
    let before = live.sim.state.read_values();
    live.sim.step_once().expect("ordinary generation");
    let after = live.sim.state.read_values();
    layout
        .iter_all()
        .into_iter()
        .map(|node| {
            let at = (node.participant_slot.raw() * n_dims + node.cols.balance_col.unwrap().raw_u32())
                as usize;
            after[at] as f64 - before[at] as f64
        })
        .sum()
}

/// Structural-addition RF admission on the native path (DA, relay 5743461789).
/// catches: a funded birth left out of the existing derived energy arena, its
/// energy-less crew enrolled by inheritance, a resource parent other than its
/// structural host, declared growth not sized into the derived cap (the second
/// birth would refuse on capacity), or settlement that does not move by
/// exactly the born flows.
#[test]
fn funded_births_join_the_energy_arena_and_settle_their_authored_flow() {
    let mut settled = Vec::new();
    for flow in [0, -1] {
        let mut live = open(&source(&[powered_product(flow)], 2)).unwrap();
        let a1 = live.targets["A1"][0];
        let arena = live
            .sim
            .spec_state
            .arena_registry
            .arenas
            .iter()
            .position(|arena| arena.name == "meridian_energy")
            .unwrap() as u32;
        let n0_members = live.sim.spec_state.arena_registry.participants.len();
        let mut births = Vec::new();
        let mut cursor = live.sim.proto.delta_log().len();
        for generation in 1..=4 {
            live.sim.step_once().expect("ordinary generation");
            let log = live.sim.proto.delta_log();
            let fresh: Vec<SimThingId> = log[cursor..]
                .iter()
                .filter_map(|entry| match entry {
                    BoundaryDeltaEntry::SimThingAdded { node, .. } => Some(node.id()),
                    BoundaryDeltaEntry::GrowthResidencyRefused { .. } => {
                        panic!("placement is available")
                    }
                    _ => None,
                })
                .collect();
            cursor = log.len();
            for id in fresh {
                let report = live
                    .sim
                    .last_resource_flow_structural_enrollment_report
                    .clone()
                    .expect("the birth was consumed by enrollment");
                println!("G{generation} birth {id:?}: {report:?}");
                assert!(report.refusals.is_empty(), "{report:?}");
                assert_eq!(report.admissions.len(), 2, "fleet and reactor, never the crew");
                births.push(id);
            }
        }
        assert_eq!(births.len(), 2, "both declared units are born");
        let registry = &live.sim.spec_state.arena_registry;
        for &fleet in &births {
            let children = live.sim.proto.root.snapshot_node(fleet).unwrap().children;
            let parent_of = |id| {
                registry
                    .participants
                    .iter()
                    .find(|member| member.arena_idx == arena && member.subtree_root == id)
                    .map(|member| member.parent)
            };
            assert_eq!(parent_of(fleet), Some(Some(a1)), "joined under its structural host");
            assert_eq!(parent_of(children[0]), Some(Some(fleet)), "the powered reactor joins");
            assert_eq!(parent_of(children[1]), None, "the energy-less crew never joins");
        }
        assert_eq!(registry.participants.len(), n0_members + 4);
        settled.push(settled_energy(&mut live));
    }
    println!("settled energy with born flow 0 / -1: {settled:?}");
    assert_eq!(settled[1] - settled[0], -2.0, "exactly the two born fleets' authored flow");
}
