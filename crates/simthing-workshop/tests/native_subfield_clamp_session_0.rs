//! An authored sub-field bound reaches EXECUTION through the ordinary native
//! path (DA, relay 5745626815). The shipped two-faction scenario is re-authored
//! in a temp copy with a finite bound on its governed energy balance; the bound
//! survives the canonical cache, and the existing governed integration saturates
//! the cell at the ceiling instead of letting it rise.
use simthing_core::{ClampBehavior, SubFieldRole};
use simthing_driver::SimSession;
use simthing_mapeditor::clause_scenario_ingest::{
    load_clause_studio_session_from_path, ClauseScenarioIngestOptions,
};
use simthing_mapeditor::load_studio_session_from_scenario_path;
use simthing_mapeditor::studio_live_session_bridge::{
    driver_scenario_field_bearing_from_profile, field_bearing_game_mode, StudioAuthoredLiveProfile,
};
use std::path::{Path, PathBuf};

/// The shipped governed energy balance, unchanged except for the authored bound.
const GOVERNED_BALANCE: &str =
    "sub_field = { role = balance governed_by = balance_rate accumulator = Balance }";

fn source(clamp: &str) -> String {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    assert_eq!(
        text.matches(GOVERNED_BALANCE).count(),
        1,
        "shipped governed balance cell"
    );
    if clamp.is_empty() {
        return text;
    }
    text.replace(
        GOVERNED_BALANCE,
        &GOVERNED_BALANCE.replace("accumulator = Balance }", &format!("accumulator = Balance {clamp} }}")),
    )
}

struct Live {
    sim: SimSession,
    cache: PathBuf,
    _dir: tempfile::TempDir,
}

fn session_from(profile: &StudioAuthoredLiveProfile) -> SimSession {
    SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(profile).expect("field bearing"),
        &field_bearing_game_mode(&profile.game_mode),
    )
    .expect("ordinary session admission")
}

/// The ordinary native path: parse -> hydrate -> profile -> session, plus the
/// derived canonical cache written beside it.
fn open(text: &str) -> Live {
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
    .expect("native load");
    let profile = native.authored_live_profile.as_ref().expect("live profile");
    Live {
        sim: session_from(profile),
        cache,
        _dir: dir,
    }
}

fn balance_clamp(sim: &SimSession) -> ClampBehavior {
    let registry = &sim.proto.registry;
    let energy = registry.id_of("meridian", "energy").expect("energy property");
    registry
        .property(energy)
        .layout
        .sub_fields
        .iter()
        .find(|sub| matches!(&sub.role, SubFieldRole::Named(name) if name == "balance"))
        .expect("governed balance cell")
        .clamp
        .clone()
}

/// Per-row settled balance for the governed energy cell.
fn balances(sim: &SimSession) -> Vec<f32> {
    let registry = &sim.proto.registry;
    let energy = registry.id_of("meridian", "energy").expect("energy property");
    let range = registry.column_range(energy);
    let offset = registry
        .property(energy)
        .layout
        .offset_of(&SubFieldRole::Named("balance".into()))
        .expect("balance offset");
    let col = range.start + offset.lane();
    let n_dims = registry.total_columns as usize;
    let values = sim.state.read_values();
    values.chunks(n_dims).map(|row| row[col]).collect()
}

/// catches: an authored bound dropped between hydration and the live registry,
/// lost across the canonical cache, or never reaching execution so a cell that
/// would rise past its ceiling keeps rising.
#[test]
fn an_authored_bound_survives_the_cache_and_saturates_in_execution() {
    let bound = ClampBehavior::Bounded { min: 0.0, max: 1.0 };
    let mut bounded = open(&source("clamp = Bounded { min = 0 max = 1 }"));
    let mut control = open(&source(""));

    assert_eq!(balance_clamp(&bounded.sim), bound, "live registry holds it");
    assert_eq!(
        balance_clamp(&control.sim),
        ClampBehavior::Unbounded,
        "omission is unchanged"
    );

    // The derived canonical cache is not a second authority.
    let cached = load_studio_session_from_scenario_path(&bounded.cache, None).expect("cache load");
    let rebound = session_from(cached.authored_live_profile.as_ref().unwrap());
    assert_eq!(balance_clamp(&rebound), bound, "cache rebind holds it");

    let mut early = (Vec::new(), Vec::new());
    for generation in 1..=5 {
        bounded.sim.step_once().expect("ordinary generation");
        control.sim.step_once().expect("ordinary generation");
        let (bounded_now, control_now) = (balances(&bounded.sim), balances(&control.sim));
        assert!(
            bounded_now.iter().all(|value| *value <= 1.0),
            "G{generation}: the ceiling holds every row"
        );
        if generation == 2 {
            early = (bounded_now, control_now);
        }
    }
    let (bounded_late, control_late) = (balances(&bounded.sim), balances(&control.sim));

    // Rows the unbounded control carries PAST the ceiling by rising, not rows
    // that merely start high: those are what saturation must hold.
    let rising: Vec<usize> = (0..control_late.len())
        .filter(|&slot| control_late[slot] > early.1[slot] + 0.1 && control_late[slot] > 1.0)
        .collect();
    println!(
        "rising rows {rising:?}; control {:?} vs bounded {:?}",
        rising.iter().map(|&s| control_late[s]).collect::<Vec<_>>(),
        rising.iter().map(|&s| bounded_late[s]).collect::<Vec<_>>()
    );
    assert!(
        !rising.is_empty(),
        "the control must carry at least one row past the ceiling by rising"
    );
    for slot in rising {
        assert_eq!(
            bounded_late[slot], 1.0,
            "row {slot} saturates exactly at its ceiling"
        );
        assert!(
            early.0[slot] < 1.0 || early.1[slot] >= 1.0,
            "row {slot} reached the ceiling by rising under the bound too"
        );
    }
}
