//! Native recipe-input locus law through the ORDINARY session (DA, relay
//! 5730468245). The shipped two-faction scenario is re-authored in a temp copy
//! with the frozen 2.2 refinery conjunction: 2 local minerals + 1 EXISTING
//! owner `meridian::energy` balance -> 1 alloy. The shipped bytes are untouched.
use simthing_core::{
    GenerationStamp, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimThingId, SubFieldRole, TransformOp,
};
use simthing_driver::{observe_hosted_property_cell, AnchorTableSnapshot, SimSession};
use simthing_feeder::BoundaryRequest;
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::load_studio_session_from_scenario_path;
use simthing_mapeditor::studio_live_session_bridge::{
    driver_scenario_field_bearing_from_profile, field_bearing_game_mode,
};
use simthing_spec::{PropertyKey, ResourceRecipeSpec};
use std::path::Path;

const MINERALS: &str = "input = { resource = minerals amount = @refinery_input }";
const TERRAN_ENERGY: &str =
    "property_value = { property = \"meridian::energy\" flow = 0 weight = 1 balance = 10 }";

fn energy_input(owner: &str) -> String {
    format!(r#"input = {{ entity = {owner} property = "meridian::energy" role = balance amount = 1 }}"#)
}

fn shipped_source() -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause"),
    )
    .unwrap()
    .replace("\r\n", "\n")
}

/// Re-author both refineries as the two-cost conjunction; `terran_input` is
/// the Terran energy cost's text so negatives can name a bad locus.
fn conjunction_source(energy_first: bool, terran_balance: &str, terran_input: &str) -> String {
    let mut text = shipped_source();
    for (building, location, owner_input) in [
        ("terran_refining", "A1", terran_input.to_string()),
        ("pirate_refining", "E1", energy_input("pirate")),
    ] {
        let single = format!("production_building = {building} {{\n      location = {location}\n      {MINERALS}");
        let costs = if energy_first {
            format!("{owner_input}\n      {MINERALS}")
        } else {
            format!("{MINERALS}\n      {owner_input}")
        };
        assert_eq!(text.matches(&single).count(), 1, "{building} ships single-cost");
        text = text.replace(
            &single,
            &format!("production_building = {building} {{\n      location = {location}\n      {costs}"),
        );
    }
    assert_eq!(text.matches(TERRAN_ENERGY).count(), 1);
    text.replace(
        TERRAN_ENERGY,
        &TERRAN_ENERGY.replace("balance = 10", &format!("balance = {terran_balance}")),
    )
}

struct Live {
    sim: SimSession,
    terran: SimThingId,
    pirate: SimThingId,
    a1: SimThingId,
    e1: SimThingId,
    recipes: Vec<ResourceRecipeSpec>,
    cached_recipes: Vec<ResourceRecipeSpec>,
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
    let recipes = |profile: &simthing_mapeditor::studio_live_session_bridge::StudioAuthoredLiveProfile| {
        profile
            .game_mode
            .resource_economy
            .as_ref()
            .map(|economy| economy.recipes.clone())
            .unwrap_or_default()
    };
    let cached = load_studio_session_from_scenario_path(&cache, None)
        .map_err(|error| format!("{error:?}"))?;
    let sim = SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(profile)?,
        &field_bearing_game_mode(&profile.game_mode),
    )
    .map_err(|error| format!("{error:?}"))?;
    let id = |name: &str| profile.install_targets[name][0];
    Ok(Live {
        terran: id("terran"),
        pirate: id("pirate"),
        a1: id("A1"),
        e1: id("E1"),
        recipes: recipes(profile),
        cached_recipes: recipes(cached.authored_live_profile.as_ref().unwrap()),
        sim,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Faction {
    minerals: f32,
    alloys: f32,
    energy: f32,
    settled_energy: f32,
}

fn faction(live: &Live, owner: SimThingId, location: SimThingId, site: &str) -> Faction {
    let anchors = AnchorTableSnapshot::from_session(&live.sim);
    let cell = |entity: SimThingId, namespace: &str, name: &str, role: SubFieldRole| {
        observe_hosted_property_cell(
            &live.sim.proto.registry,
            &live.sim.proto.allocator,
            &anchors,
            entity,
            &PropertyKey::new(namespace, name),
            &role,
        )
        .unwrap_or_else(|error| panic!("{namespace}::{name} at {entity:?}: {error:?}"))
    };
    Faction {
        minerals: cell(
            location,
            "meridian_material",
            &format!("{site}_minerals_quantity"),
            SubFieldRole::Amount,
        ),
        alloys: cell(
            location,
            "meridian_material",
            &format!("{site}_alloys_quantity"),
            SubFieldRole::Amount,
        ),
        energy: cell(owner, "meridian", "energy", SubFieldRole::Named("balance".into())),
        settled_energy: cell(
            owner,
            "meridian",
            "energy",
            SubFieldRole::Named("balance_rate".into()),
        ),
    }
}

fn snapshot(live: &Live) -> [Faction; 2] {
    [
        faction(live, live.terran, live.a1, "A1"),
        faction(live, live.pirate, live.e1, "E1"),
    ]
}

/// Authored Terran energy income through the ordinary overlay door.
fn restore_terran_energy(live: &Live, generation: u32) {
    let energy = live.sim.proto.registry.id_of("meridian", "energy").unwrap();
    live.sim
        .tx
        .submit_boundary(BoundaryRequest::AttachOverlay {
            target: live.terran,
            source_generation: GenerationStamp::new(generation),
            overlay: Overlay {
                id: OverlayId::new(),
                kind: OverlayKind::Policy,
                source: OverlaySource::System,
                origin: live.terran,
                affects: vec![live.terran],
                transform: PropertyTransformDelta {
                    property_id: energy,
                    sub_field_deltas: vec![(
                        SubFieldRole::Named("flow".into()),
                        TransformOp::set(1.0),
                    )],
                },
                lifecycle: OverlayLifecycle::UntilDissolved,
            },
        })
        .unwrap();
}

/// A recipe's economics: the source span of an authored cost is provenance for
/// refusal attribution and moves with authored order, so it is cleared here.
fn economics(recipes: &[ResourceRecipeSpec]) -> Vec<ResourceRecipeSpec> {
    let mut recipes = recipes.to_vec();
    for input in recipes.iter_mut().flat_map(|recipe| recipe.inputs.iter_mut()) {
        input.host_span_token = None;
    }
    recipes
}

const WITHHELD_GENERATIONS: u32 = 3;
const RESTORED_GENERATIONS: u32 = 4;

fn trajectory(energy_first: bool) -> (Vec<[Faction; 2]>, Vec<ResourceRecipeSpec>, Vec<ResourceRecipeSpec>) {
    let mut live = open(&conjunction_source(energy_first, "0", &energy_input("terran")))
        .expect("the two-cost conjunction admits through the ordinary session");
    let mut states = vec![snapshot(&live)];
    for generation in 0..WITHHELD_GENERATIONS + RESTORED_GENERATIONS {
        if generation == WITHHELD_GENERATIONS {
            restore_terran_energy(&live, generation);
        }
        live.sim.step_once().expect("ordinary generation");
        states.push(snapshot(&live));
    }
    (states, live.recipes.clone(), live.cached_recipes.clone())
}

/// catches: a refinery producing without its energy cost (the dropped-cost
/// defect), a cost read from a fabricated ledger instead of the existing owner
/// balance, two factions' own energy contending as one cell, field order
/// changing execution, or a restored economy double- or ghost-counting.
#[test]
fn refinery_spends_existing_owner_energy_only_when_the_conjunction_is_funded() {
    let (states, recipes, cached) = trajectory(false);
    let (reversed, reversed_recipes, _) = trajectory(true);
    assert_eq!(
        economics(&recipes),
        economics(&reversed_recipes),
        "authored order carries no economics"
    );
    assert_eq!(
        economics(&recipes),
        economics(&cached),
        "the canonical cache rebinds the same conjunction"
    );
    assert_eq!(states, reversed, "execution is identical in both authored orders");
    for (building, owner) in [("terran_refining", "terran"), ("pirate_refining", "pirate")] {
        let recipe = recipes
            .iter()
            .find(|recipe| recipe.id.ends_with(building))
            .unwrap_or_else(|| panic!("{building} recipe"));
        assert_eq!(recipe.inputs.len(), 2, "{building}: both costs survive ingress");
        assert!(
            recipe.inputs.iter().any(|input| input.property
                == PropertyKey::new("meridian", "energy")
                && input.role == SubFieldRole::Named("balance".into())
                && input.unit_cost == 1.0
                && input.host_entity.as_deref() == Some(owner)),
            "{building} spends the EXISTING meridian::energy balance at {owner}: {:?}",
            recipe.inputs
        );
        assert!(
            recipe
                .inputs
                .iter()
                .all(|input| !input.property.name.contains("energy_quantity")),
            "{building}: no fabricated energy ledger"
        );
    }

    // Minerals accrue identically at both sites; measure the accrual on the
    // withheld faction, which never refines.
    let accrual = states[1][0].minerals - states[0][0].minerals;
    assert!(accrual > 0.0);
    let mut produced = [[0.0f32; 2]; 2];
    for (step, pair) in states.windows(2).enumerate() {
        let phase = usize::from(step as u32 >= WITHHELD_GENERATIONS);
        for faction in 0..2 {
            let (before, after) = (pair[0][faction], pair[1][faction]);
            let batches = after.alloys - before.alloys;
            assert!(batches >= 0.0, "generation {step}: alloys never decrease");
            assert_eq!(
                after.minerals - before.minerals,
                accrual - 2.0 * batches,
                "generation {step} faction {faction}: every alloy spends exactly 2 minerals"
            );
            assert_eq!(
                after.energy,
                before.energy + after.settled_energy - batches,
                "generation {step} faction {faction}: every alloy spends exactly 1 energy; \
                 settlement is the only other energy movement"
            );
            assert!(after.energy >= 0.0, "energy never overdrawn");
            produced[phase][faction] += batches;
        }
        if phase == 0 {
            assert!(
                pair[0][0].minerals >= 2.0,
                "withheld Terran has minerals available at generation {step}"
            );
        }
    }
    println!("alloys produced [withheld, restored] x [terran, pirate] = {produced:?}");
    assert_eq!(produced[0][0], 0.0, "withheld energy blocks refining despite minerals");
    assert!(produced[0][1] > 0.0, "the funded faction refines in the same band");
    assert!(produced[1][0] > 0.0, "restored energy resumes refining");
}

/// catches: an unknown property, role, or host on a canonical cost being
/// invented, defaulted, or deferred past activation instead of refused.
#[test]
fn unknown_canonical_cost_locus_refuses_before_activation() {
    for (bad_input, expected) in [
        (
            r#"input = { entity = terran property = "meridian::plasma" role = balance amount = 1 }"#,
            "plasma",
        ),
        (
            r#"input = { entity = terran property = "meridian::energy" role = reserve amount = 1 }"#,
            "reserve",
        ),
        (
            r#"input = { entity = nobody property = "meridian::energy" role = balance amount = 1 }"#,
            "nobody",
        ),
    ] {
        let error = match open(&conjunction_source(false, "10", bad_input)) {
            Ok(_) => panic!("{bad_input}: an unknown locus must refuse before activation"),
            Err(error) => error,
        };
        println!("refused {bad_input}: {error}");
        assert!(error.contains(expected), "{bad_input}: {error}");
    }
}

/// The two-cost conjunction with abundant minerals and, when `cap` is set, the
/// AUTHORITATIVE per-generation unit ceiling on both refineries.
fn capped_source(cap: Option<u32>, mine_rate: u32) -> String {
    let mut text = conjunction_source(false, "10", &energy_input("terran"));
    assert_eq!(text.matches("@mine_rate = 3\n").count(), 1);
    text = text.replace("@mine_rate = 3\n", &format!("@mine_rate = {mine_rate}\n"));
    if let Some(cap) = cap {
        let hint = "      throttle_hint_max_per_tick = 1\n";
        assert_eq!(text.matches(hint).count(), 2, "both refineries author the hint");
        text = text.replace(
            hint,
            &format!("{hint}      max_units_per_generation = {cap}\n"),
        );
    }
    text
}

/// Units executed per generation per faction, with the exact accounting
/// identities checked on every generation.
fn units_per_generation(text: &str, mine_rate: f32, generations: u32) -> Vec<[f32; 2]> {
    let mut live = open(text).expect("ordinary capped session");
    assert_eq!(
        economics(&live.recipes),
        economics(&live.cached_recipes),
        "the canonical cache rebinds the same cap"
    );
    let mut before = snapshot(&live);
    let mut units = Vec::new();
    for generation in 0..generations {
        live.sim.step_once().expect("ordinary generation");
        let after = snapshot(&live);
        let mut executed = [0.0f32; 2];
        for faction in 0..2 {
            let batches = after[faction].alloys - before[faction].alloys;
            assert_eq!(
                after[faction].minerals - before[faction].minerals,
                mine_rate - 2.0 * batches,
                "generation {generation} faction {faction}: 2 minerals per executed unit"
            );
            assert_eq!(
                after[faction].energy,
                before[faction].energy + after[faction].settled_energy - batches,
                "generation {generation} faction {faction}: 1 energy per executed unit"
            );
            executed[faction] = batches;
        }
        units.push(executed);
        before = after;
    }
    units
}

/// catches: the authored cap ignored in the ordinary session, applied to the
/// alloy credit without the matching mineral/energy debit, banked across
/// generations, or leaking onto a recipe that authors none (relay 5735839909).
#[test]
fn capped_refinery_executes_at_most_the_authored_units_per_generation() {
    let uncapped = units_per_generation(&capped_source(None, 20), 20.0, 1);
    println!("uncapped G1 units [terran, pirate] = {:?}", uncapped[0]);
    assert!(
        uncapped[0].iter().all(|&units| units > 2.0),
        "without the cap every affordable unit executes: {uncapped:?}"
    );
    for cap in [1u32, 2] {
        let capped = units_per_generation(&capped_source(Some(cap), 20), 20.0, 3);
        println!("cap {cap}: units per generation [terran, pirate] = {capped:?}");
        assert!(
            capped.iter().all(|units| *units == [cap as f32; 2]),
            "cap {cap}: exactly {cap} unit(s) per faction per generation: {capped:?}"
        );
    }
}
