//! An authored sub-field bound reaches the ordinary native session (DA, relay
//! 5745626815), except on a cell that carries an RF arena's conserved quantity
//! (DA intervention 3(b), live Board 5879126789). There, a bound truncates the
//! governed integration and destroys the flow the arena settled, so the
//! Invariant Set's conservation "for any input" fails. A storage bound is a
//! budget, never a clamp. The shipped two-faction scenario is re-authored in a
//! temp copy with one bound per case.
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

/// The shipped energy property's cells, each authored exactly once.
const FLOW: &str = "sub_field = { role = flow accumulator = IntrinsicFlow }";
const ALLOCATED: &str =
    "sub_field = { role = Amount accumulator = AllocatedFlow { arena = meridian_energy } }";
const WEIGHT: &str =
    "sub_field = { role = weight default = 1 accumulator = AllocatorWeight { arena = meridian_energy } }";
const RATE: &str = "sub_field = { role = balance_rate }";
const BALANCE: &str =
    "sub_field = { role = balance governed_by = balance_rate accumulator = Balance }";

/// The shipped scenario with `clamp` appended inside one cell's block.
fn source(cell: &str, clamp: &str) -> String {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios/stellaristhing_base.clause"),
    )
    .unwrap()
    .replace("\r\n", "\n");
    for shipped in [FLOW, ALLOCATED, WEIGHT, RATE, BALANCE] {
        assert_eq!(text.matches(shipped).count(), 1, "shipped cell {shipped}");
    }
    if clamp.is_empty() {
        return text;
    }
    let bounded = format!("{} {clamp} }}", cell.strip_suffix(" }").unwrap());
    text.replace(cell, &bounded)
}

fn session_from(profile: &StudioAuthoredLiveProfile) -> Result<SimSession, String> {
    SimSession::open_from_spec(
        driver_scenario_field_bearing_from_profile(profile).map_err(|e| format!("{e:?}"))?,
        &field_bearing_game_mode(&profile.game_mode),
    )
    .map_err(|e| format!("{e:?}"))
}

/// The ordinary native path: parse -> hydrate -> profile -> session, plus the
/// derived canonical cache written beside it.
fn open(text: &str) -> (Result<SimSession, String>, PathBuf, tempfile::TempDir) {
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
    let session = load_clause_studio_session_from_path(
        &source,
        &ClauseScenarioIngestOptions::default(),
        &cache,
        None,
    )
    .map_err(|e| format!("{e:?}"))
    .and_then(|(_, native)| session_from(native.authored_live_profile.as_ref().unwrap()));
    (session, cache, dir)
}

fn clamp_of(sim: &SimSession, role: &str) -> ClampBehavior {
    let registry = &sim.proto.registry;
    let energy = registry
        .id_of("meridian", "energy")
        .expect("energy property");
    registry
        .property(energy)
        .layout
        .sub_fields
        .iter()
        .find(|sub| match &sub.role {
            SubFieldRole::Named(name) => name == role,
            other => format!("{other:?}") == role,
        })
        .expect("energy cell")
        .clamp
        .clone()
}

/// catches: a bound on a conserved RF cell (the arena's AllocatedFlow, the
/// settling Balance, or its governing rate) admitted into execution, where it
/// destroys settled flow; or the refusal over-reaching onto the property's
/// non-conserved cells, dropping the bound between hydration and the live
/// registry, or losing it across the canonical cache.
#[test]
fn only_conserved_rf_cells_refuse_an_authored_bound() {
    for (cell, role, clamp) in [
        (
            BALANCE,
            "Named(balance)",
            "clamp = Bounded { min = 0 max = 1 }",
        ),
        (BALANCE, "Named(balance)", "clamp = Floored { min = 0 }"),
        (
            RATE,
            "Named(balance_rate)",
            "clamp = Bounded { min = -1 max = 1 }",
        ),
        (ALLOCATED, "Amount", "clamp = Floored { min = 0 }"),
    ] {
        let (session, _, _dir) = open(&source(cell, clamp));
        let refusal = session
            .err()
            .unwrap_or_else(|| panic!("{role} {clamp}: a conserved cell must refuse"));
        assert!(
            refusal.contains("rf-conserved-cell-unclamped")
                && refusal.contains(&format!("sub_fields[role={role}]")),
            "{role} {clamp}: {refusal}"
        );
    }

    for (cell, role, clamp, expected) in [
        (
            WEIGHT,
            "weight",
            "clamp = Bounded { min = 0 max = 5 }",
            ClampBehavior::Bounded { min: 0.0, max: 5.0 },
        ),
        (
            FLOW,
            "flow",
            "clamp = Floored { min = 0 }",
            ClampBehavior::Floored { min: 0.0 },
        ),
        (
            BALANCE,
            "balance",
            "clamp = Unbounded",
            ClampBehavior::Unbounded,
        ),
        (BALANCE, "balance", "", ClampBehavior::Unbounded),
    ] {
        let (session, cache, _dir) = open(&source(cell, clamp));
        let mut sim =
            session.unwrap_or_else(|error| panic!("{role} {clamp:?} is admitted: {error}"));
        assert_eq!(
            clamp_of(&sim, role),
            expected,
            "{role}: the live registry holds it"
        );
        let cached = load_studio_session_from_scenario_path(&cache, None).expect("cache load");
        let rebound = session_from(cached.authored_live_profile.as_ref().unwrap())
            .expect("the cache rebinds the admitted bound");
        assert_eq!(
            clamp_of(&rebound, role),
            expected,
            "{role}: the cache holds it"
        );
        sim.step_once().expect("an ordinary generation");
    }
}
