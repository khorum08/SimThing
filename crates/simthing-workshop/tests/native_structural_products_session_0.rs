//! Native funded structural products through the ORDINARY session (DA, relay
//! 5737649159). The shipped two-faction scenario is re-authored in a temp copy
//! with a shipyard recipe whose funded output births a source-authored,
//! detached fleet subtree through the existing 2.1 ActionBand -> AddChild door.
use simthing_core::{ColumnIndex, ObjectResidencyRelation, SubFieldRole, SimThingId};
use simthing_driver::SimSession;
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::load_studio_session_from_scenario_path;
use simthing_mapeditor::studio_live_session_bridge::{
    driver_scenario_field_bearing_from_profile, field_bearing_game_mode,
    StudioAuthoredLiveProfile,
};
use simthing_sim::{
    BoundaryDeltaEntry, OrdinaryGrowthRefusal,
    OrdinaryGrowthRefusalReason, RecordedGrowthResidencyFact,
};
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

// ---------------------------------------------------------------------------
// 0088 economy/fleet: finite-capacity negative proof (Bonsai, rung 0).
//
// Fills ordinary A1 residency capacity through funded whole-subtree births
// (each 3 rows) via the shipyard pipeline, then observes the next funded whole
// subtree refused by the existing ordinary residency market. On the first
// refusal generation `step_once` returns normally, no `SimThingAdded` occurs,
// all IDs and allocator live count / available capacity are unchanged across
// that step, every earlier fleet identity and placement survives, and the
// typed refusal is a capacity refusal of a *funded* candidate: the ordinary
// market is capacity-aware and grants zero (MarketUnresolved { granted: 0 })
// when a whole subtree no longer fits, while A1's economic funding (corvettes)
// is strictly positive at that boundary (the zero residency grant is distinct
// from the product's funding). The product remains
// in-progress (unfulfilled demand) at the point of refusal.
//
// Fixture (base 60830eef4907b2bc9a8bf94325f9b50f8891d528): one product, count
// 50, crew 2, shipyard cost 2. Observed there: 12 whole births (3 rows each,
// live +3 / capacity -3 each), first refusal at generation 17, A1 initial
// root capacity 36 rows, exhaustion at capacity 0. The assertions below pin
// the invariants, not the observed generation count; GENERATIONS is a bounded
// fixture window, not a capacity law.
#[test]
fn sequential_capacity_exhaustion_preserves_prior_births() {
    let product_count: u32 = 50;
    let mut live = open(&source(&[product("terran", "A1", product_count, 2)], 2))
        .expect("the funded product admits through the ordinary session");
    let root = live.sim.proto.root.id();
    let a1 = live.targets["A1"][0];

    let n0 = all_ids(&live);
    let live0 = live.sim.proto.allocator.live_count();
    let cap0 = live.sim.proto.allocator.growth_capacity_available(root);
    assert!(
        cap0 >= 3,
        "fixture must reserve room for at least one whole subtree"
    );

    // Bounded funding observation (0088): the product's economic funding is
    // A1's corvettes (funding property `A1_corvettes_quantity`, role Amount).
    // A zero residency grant at a generation does NOT by itself prove the
    // candidate was economically funded, so we read the funding property's
    // CPU shadow at the start of each boundary (the state that drives that
    // boundary's candidate), plus the view's generation/tick stamp to confirm
    // the shadow is current for the relevant generation.
    let corvettes_pid = live
        .sim
        .proto
        .registry
        .id_of("meridian_material", "A1_corvettes_quantity")
        .expect("funding property registered on A1");
    let corvettes_col = live
        .sim
        .proto
        .registry
        .column_range(corvettes_pid)
        .col_for_role(&SubFieldRole::Amount, &live.sim.proto.registry.property(corvettes_pid).layout)
        .expect("funding property carries an Amount role column");
    fn shadow_pool(live: &Live, a1: SimThingId, col: ColumnIndex) -> Option<f32> {
        live.sim.shadow_view().value(a1, col)
    }
    fn shadow_stamp(live: &Live) -> (i32, u64) {
        let view = live.sim.shadow_view();
        (view.generation().get() as i32, view.tick_index())
    }

    const GENERATIONS: u32 = 40;
    // One committed-entitlement fact per previously born subtree root. Read at the
    // generation where it is recorded (never from the final session), so the
    // first-refusal survival claim uses true pre/post facts, not final-state
    // quantities substituted in.
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct SubtreeFact {
        size: usize,
        overlays: usize,
        owner: String,
        qty: u32,
        relation: Option<ObjectResidencyRelation>,
    }
    // Cumulative committed-entitlement facts of every born subtree as of the end
    // of this generation (the post-step state that the next boundary's candidate
    // would see).
    fn subtree_fact(live: &Live, id: SimThingId) -> SubtreeFact {
        let s = shape(&live, id);
        SubtreeFact {
            size: s.0,
            overlays: s.1,
            owner: s.2,
            qty: s.3,
            relation: live.sim.proto.allocator.relation_of(id),
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Gen {
        born: Vec<(SimThingId, SimThingId)>,
        refused: Vec<OrdinaryGrowthRefusal>,
        live: usize,
        cap: u32,
        ids: BTreeSet<SimThingId>,
        // Post-step committed-entitlement facts of all born subtrees so far.
        born_facts: Vec<(SimThingId, SubtreeFact)>,
    }
    let mut per_generation: Vec<Gen> = Vec::new();
    let mut cursor = 0usize;
    let mut pool_before: Vec<Option<f32>> = Vec::with_capacity(GENERATIONS as usize);
    let mut pool_stamps: Vec<(i32, u64)> = Vec::with_capacity(GENERATIONS as usize);
    let mut cumulative_born_roots: Vec<SimThingId> = Vec::new();
    for g in 0..GENERATIONS as usize {
        // Record the state that drives this boundary's candidate, before the step.
        let (g_stamp, t_stamp) = shadow_stamp(&live);
        pool_before.push(shadow_pool(&live, a1, corvettes_col));
        pool_stamps.push((g_stamp, t_stamp));
        live.sim.step_once().expect("ordinary generation");
        let log = live.sim.proto.delta_log();
        let fresh = &log[cursor..];
        cursor = log.len();
        let born = fresh
            .iter()
            .filter_map(|entry| match entry {
                BoundaryDeltaEntry::SimThingAdded { parent, node, .. } => {
                    Some((*parent, node.id()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let refused = fresh
            .iter()
            .filter_map(|entry| match entry {
                BoundaryDeltaEntry::GrowthResidencyRefused { fact } => match fact {
                    RecordedGrowthResidencyFact::Refused(refusal) => Some(refusal.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>();
        let live_c = live.sim.proto.allocator.live_count();
        let cap_c = live.sim.proto.allocator.growth_capacity_available(root);
        let ids_c = all_ids(&live);
        // Cumulative born roots so far (birth order, stable across generations).
        for &(parent, id) in &born {
            cumulative_born_roots.push(id);
        }
        let born_facts: Vec<(SimThingId, SubtreeFact)> = cumulative_born_roots
            .iter()
            .map(|&id| (id, subtree_fact(&live, id)))
            .collect();
        per_generation.push(Gen {
            born,
            refused,
            live: live_c,
            cap: cap_c,
            ids: ids_c,
            born_facts,
        });
        let gen_no = g + 1;
        if gen_no <= 2
            || gen_no == 16
            || gen_no == 17
            || gen_no == 18
            || gen_no == 30
        {
            let (g_st, t_st) = pool_stamps[g];
            println!(
                "0088-funding g={} pool_before={} shadow_gen={} tick={}",
                gen_no,
                pool_before[g].map(|v| v as f32).unwrap_or(f32::NAN),
                g_st,
                t_st
            );
        }
    }

    // G1 is quiet: nothing is born before the first funded crossing dispatches.
    assert!(per_generation[0].born.is_empty(), "G1 births nothing");

    // Walk all generations: each whole subtree is a fresh 3-row identity under
    // A1 with exact +3 live / -3 capacity accounting; the identity set only ever
    // grows by births.
    let mut live_acc = live0;
    let mut cap_acc = cap0;
    let mut prior_ids = n0.clone();
    for gen in &per_generation {
        for &(parent, id) in &gen.born {
            assert!(parent == a1, "every birth is parented to the A1 works");
            assert!(
                !prior_ids.contains(&id),
                "a genuinely fresh identity (no duplicate / N0 reuse)"
            );
            assert_eq!(
                shape(&live, id),
                (3, 1, "terran".to_string(), 3),
                "each born subtree is whole: 3 rows, 1 overlay, terran owner, qty 3"
            );
        }
        let births = gen.born.len();
        assert_eq!(
            gen.live,
            live_acc + births * 3,
            "every born subtree adds exactly 3 live rows"
        );
        assert_eq!(
            gen.cap,
            cap_acc - births as u32 * 3,
            "every whole subtree takes exactly 3 capacity rows"
        );
        // Add every node of each born subtree (root + crew children) to the
        // expected identity set, not just the birth roots.
        for &(_, sub_root) in &gen.born {
            let mut pending = vec![sub_root];
            while let Some(cid) = pending.pop() {
                prior_ids.insert(cid);
                pending.extend(live.sim.proto.root.snapshot_node(cid).unwrap().children);
            }
        }
        assert_eq!(
            gen.ids,
            prior_ids,
            "identity set only ever grows by the born subtrees"
        );
        live_acc = gen.live;
        cap_acc = gen.cap;
    }

    // First refusal generation: the first gen with a typed refusal.
    let first_idx = match per_generation.iter().position(|g| !g.refused.is_empty()) {
        Some(i) => i,
        None => panic!("no refusal in the bounded window"),
    };
    let first = &per_generation[first_idx];
    let generation = first_idx + 1;
    // No birth on that generation (a pure refusal).
    assert!(
        first.born.is_empty(),
        "no SimThingAdded on the first refusal generation (g={generation})"
    );
    // The first refusal is a capacity refusal. The ordinary market is
    // capacity-aware: when a whole subtree can no longer fit, it grants zero
    // (a typed market/unavailability U-fact); in other configurations the
    // kernel refuses the placement of a funded claim. Either way the next
    // whole subtree is refused and nothing is fabricated. It is never an
    // unrelated rejection (provenance mismatch / lifecycle admission).
    assert!(
        !first.refused.is_empty(),
        "at least one typed refusal on the first refusal generation"
    );
    println!(
        "0088 first refusal: g={} count={} reason={:?}",
        generation,
        first.refused.len(),
        first.refused[0].reason()
    );
    for refusal in &first.refused {
        assert!(
            matches!(
                refusal.reason(),
                &OrdinaryGrowthRefusalReason::MarketUnresolved { granted: 0 }
                    | &OrdinaryGrowthRefusalReason::Placement(_)
            ),
            "first refusal is a capacity (zero-grant / placement) refusal, not an unrelated rejection: {:#?}",
            refusal.reason()
        );
    }
    // Funding is a distinct fact from the residency grant. A zero residency
    // grant (MarketUnresolved { granted: 0 }) does NOT by itself prove the
    // candidate was economically funded — the market is capacity-aware and
    // grants zero whenever a whole subtree cannot fit, whether or not funding
    // is present. Observe the product's funding property (A1 corvettes, role
    // Amount) at the start of the first refusal boundary (the state that drives
    // that candidate): it must be strictly positive (candidate funded), and the
    // shadow's generation stamp must equal the boundary generation (current, not
    // stale). Measured: g=17, A1 corvettes = 13 > 0, shadow_gen = 17.
    let funding_before_refusal = pool_before[first_idx];
    let (funding_gen, _) = pool_stamps[first_idx];
    assert!(
        funding_before_refusal.map_or(false, |v| v > 0.0),
        "the first refusal candidate was not economically funded at the first refusal boundary (A1 corvettes={:?} at g={})",
        funding_before_refusal,
        generation
    );
    assert_eq!(
        funding_gen,
        generation as i32,
        "funding shadow generation must be current for the refusal boundary (shadow_gen={}, boundary={})",
        funding_gen,
        generation
    );
    println!(
        "0088 funding: first-refusal candidate funded={:?} @ A1 (g={}, shadow_gen={})",
        funding_before_refusal,
        generation,
        funding_gen
    );
    // The product is still in-progress (unfulfilled demand) at the first refusal.
    let birth_before_refusal: Vec<SimThingId> = per_generation
        .iter()
        .take(first_idx)
        .flat_map(|g| g.born.iter().map(|(_, id)| *id))
        .collect();
    assert!(
        !birth_before_refusal.is_empty(),
        "at least one whole subtree born before the first refusal"
    );
    assert!(
        birth_before_refusal.len() < product_count as usize,
        "at first refusal the product is still in-progress (funded demand unmet)"
    );

    // Across the refusal step: live count, available capacity, and the identity
    // set are unchanged; available capacity is below one whole subtree.
    let pre_live = per_generation[first_idx - 1].live;
    let pre_cap = per_generation[first_idx - 1].cap;
    let pre_ids = per_generation[first_idx - 1].ids.clone();
    assert_eq!(first.live, pre_live, "refusal changes no live count");
    assert_eq!(first.cap, pre_cap, "refusal changes no available capacity");
    assert_eq!(first.ids, pre_ids, "refusal adds no identity");
    assert!(
        pre_cap < 3,
        "available capacity below one whole subtree when the next subtree refuses"
    );
    // Every earlier fleet identity and its committed placement survive. The
    // pre/post facts are read at the boundary (generation first_idx - 1, pre,
    // vs generation first_idx, post) and never from the final session: the
    // refusal step changes none of the earlier subtrees' committed placement.
    // (pre = generation first_idx - 1, post = generation first_idx; neither is
    // the full-run final state.)
    let pre = &per_generation[first_idx - 1].born_facts;
    let post = &per_generation[first_idx].born_facts;
    assert_eq!(
        pre.len(),
        post.len(),
        "the set of previously born subtrees is unchanged across the first refusal"
    );
    let authored = SubtreeFact {
        size: 3,
        overlays: 1,
        owner: "terran".to_string(),
        qty: 3,
        relation: Some(ObjectResidencyRelation::ChildOf(a1)),
    };
    // Same identities, same committed placement, element-for-element, across the
    // first refusal boundary (true pre vs post facts, not the final session).
    assert_eq!(
        pre,
        post,
        "earlier subtree identities and committed placement are unchanged across the first refusal"
    );
    // Each post fact must be the authored, committed placement.
    for (_sim_id, fact) in post.iter().cloned() {
        assert_eq!(
            fact,
            authored.clone(),
            "earlier subtree is still authored and placed after the first refusal"
        );
    }
    // Funded refusals continue: after the first refusal there are no births and
    // refusals keep arriving (funding continues, each whole subtree rejected).
    let post_refusals: usize = per_generation
        .iter()
        .skip(first_idx + 1)
        .map(|g| g.refused.len())
        .sum();
    for gen in per_generation.iter().skip(first_idx) {
        assert!(gen.born.is_empty(), "no births on or after the first refusal");
    }
    assert!(
        post_refusals >= 1,
        "funded refusals continue after the first (funding keeps arriving)"
    );

    println!(
        "0088-capacity: live0={} cap0={} births_before_refusal={} first_refusal_g={} cap@refusal={} post_refusals={}",
        live0,
        cap0,
        birth_before_refusal.len(),
        generation,
        pre_cap,
        post_refusals
    );
}
