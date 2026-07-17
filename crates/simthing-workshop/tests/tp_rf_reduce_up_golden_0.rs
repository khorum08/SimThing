//! RF-CONSERVATION-ORACLE-0 — authored-pack-bound TP reduce-up golden.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use simthing_driver::{check_recipe_exact, RecipeInvocationObservation};
use simthing_workshop::tp_rf_reduce_up_golden::compute_tp_reduce_up_golden_from_clause;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn canonical_source() -> (String, PathBuf) {
    let path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let mut source = std::fs::read_to_string(&path).expect("read canonical TP ClauseScript");
    if std::env::var_os("SIMTHING_RF_GOLDEN_DRIFT").is_some() {
        let drifted = source.replacen("upkeep_per_ship = 2", "upkeep_per_ship = 3", 1);
        assert_ne!(drifted, source, "drift injection must alter authored input");
        source = drifted;
    }
    (
        source,
        path.parent().expect("scenario directory").to_path_buf(),
    )
}

/// RF-4 OVL target: one exact Owner channel, all descendant/sibling
/// contributions, and the selected child's marginal derived from the canonical
/// authored pack. `SIMTHING_RF_GOLDEN_DRIFT=1` changes the authored upkeep and
/// deliberately makes this test fail, proving the golden bites on pack drift.
#[test]
fn canonical_tp_reduce_up_golden_is_analytically_derived() {
    let (source, base) = canonical_source();
    let golden = compute_tp_reduce_up_golden_from_clause(source.as_bytes(), Some(&base))
        .expect("derive canonical TP reduce-up golden");

    eprintln!(
        "TP-RF-GOLDEN: owner={} selected={} siblings={} aggregate={} participants={}",
        golden.owner_arena,
        golden.selected_child_marginal,
        golden.sibling_contributions_preserved,
        golden.expected_owner_aggregate,
        golden.participant_contributions.len()
    );

    assert_eq!(golden.owner_arena, "terran");
    assert_eq!(golden.resource_key, "tp_energy");
    assert_eq!(golden.participant_contributions.len(), 10);
    assert_eq!(golden.selected_child_id, "terran_fleets#0");
    assert_eq!(golden.selected_child_marginal, 40.0);
    assert_eq!(golden.sibling_contributions_preserved, 360.0);
    assert_eq!(golden.expected_owner_aggregate, 400.0);
    assert_eq!(
        golden.selected_child_marginal + golden.sibling_contributions_preserved,
        golden.expected_owner_aggregate,
        "selected marginal must preserve every sibling contribution"
    );
    assert_eq!(
        golden
            .participant_contributions
            .iter()
            .map(|participant| participant.contribution)
            .sum::<f32>(),
        golden.expected_owner_aggregate
    );
    let participant_ids: HashSet<_> = golden
        .participant_contributions
        .iter()
        .map(|participant| participant.participant_id.as_str())
        .collect();
    assert_eq!(
        participant_ids.len(),
        golden.participant_contributions.len(),
        "every authored fleet participant must contribute exactly once"
    );
    assert!(
        golden
            .participant_contributions
            .iter()
            .all(|participant| !participant.target_location_id.is_empty()),
        "every participant must retain its authored location path"
    );
}

/// The recipe face is also extracted from the canonical pack rather than a
/// copied Rust constants table and remains exact under the independent oracle.
#[test]
fn canonical_tp_factory_recipe_passes_conservation_oracle() {
    let (source, base) = canonical_source();
    let golden = compute_tp_reduce_up_golden_from_clause(source.as_bytes(), Some(&base))
        .expect("derive canonical TP recipe observation");
    let observation = RecipeInvocationObservation {
        need_deltas: golden.factory_need_deltas,
        unit_costs: golden.factory_unit_costs,
        emit_count: golden.factory_emit_count,
    };
    assert_eq!(observation.emit_count, 4);
    assert!(
        check_recipe_exact(&observation).is_ok(),
        "canonical authored recipe must satisfy ADR per-recipe exact identity"
    );
}
