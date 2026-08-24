use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use simthing_driver::{
    clone_for_replay, fixture_convergence_static_512_participants, fixture_dynamic_multi_fission,
    fixture_repeated_resync, fixture_replay_static, fixture_static_flat_star_10_participants,
    fixture_static_flat_star_skewed_weights, fixture_two_arena_no_coupling, open_fixture_session,
    run_resource_flow_burn_in,
};
use simthing_spec::{GameModeSpec, ResourceEconomySpec, ResourceFlowSpec};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn retired_transfer_selector_is_an_authored_red() {
    let key = retired_mode_key();
    let mut authored = empty_resource_economy();
    authored
        .as_object_mut()
        .expect("resource economy object")
        .insert(key.clone(), Value::String("TransferOnly".into()));

    assert_unknown_field::<ResourceEconomySpec>(authored, &key, "transfer workload");
}

#[test]
fn retired_emission_selector_is_an_authored_red() {
    let key = retired_mode_key();
    let mut authored = empty_resource_economy();
    authored
        .as_object_mut()
        .expect("resource economy object")
        .insert(key.clone(), Value::String("EmissionOnly".into()));

    assert_unknown_field::<ResourceEconomySpec>(authored, &key, "emission workload");
}

#[test]
fn retired_resource_flow_selector_is_an_authored_red() {
    let key = retired_mode_key();
    let mut authored = json!({
        "arenas": [],
        "couplings": [],
        "base_obligations": [],
        "capacity_budget": null,
        "gated_rates": [],
        "need_bindings": []
    });
    authored
        .as_object_mut()
        .expect("resource flow object")
        .insert(key.clone(), Value::String("FlatStarOptIn".into()));

    assert_unknown_field::<ResourceFlowSpec>(authored, &key, "Resource Flow workload");
}

#[test]
fn retired_execution_profile_is_an_authored_red() {
    let key = ["resource_flow_execution", "_profile"].concat();
    let mut authored = serde_json::to_value(GameModeSpec::default()).expect("serialize game mode");
    authored
        .as_object_mut()
        .expect("game mode object")
        .insert(key.clone(), Value::String("DefaultDisabled".into()));

    assert_unknown_field::<GameModeSpec>(authored, &key, "game-mode admission");
}

#[test]
fn production_and_authored_fixtures_have_no_retired_selector_vocabulary() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let roots = [workspace.join("crates"), workspace.join("fixtures")];
    let forbidden = retired_vocabulary();
    let mut residues = Vec::new();

    for root in roots.into_iter().filter(|root| root.exists()) {
        let mut files = Vec::new();
        collect_files(&root, &mut files);
        for path in files.into_iter().filter(|path| is_scanned_surface(path)) {
            let Ok(source) = fs::read_to_string(&path) else {
                continue;
            };
            for token in &forbidden {
                if source.contains(token) {
                    residues.push((
                        path.strip_prefix(workspace).unwrap_or(&path).to_path_buf(),
                        token.clone(),
                    ));
                }
            }
        }
    }

    assert!(
        residues.is_empty(),
        "ACCUMULATOR-CONVERGENCE-SEAL-0-RETIRED-SELECTOR-RESIDUE: {residues:?}"
    );
}

#[test]
fn converged_resource_flow_burn_in_and_soak_remain_green() {
    let fixtures = [
        fixture_static_flat_star_10_participants(),
        fixture_static_flat_star_skewed_weights(),
        fixture_dynamic_multi_fission(),
        fixture_two_arena_no_coupling(),
        fixture_repeated_resync(),
        fixture_convergence_static_512_participants(),
    ];

    for fixture in &fixtures {
        let mut session = open_fixture_session(fixture).expect("open convergence soak session");
        let report = run_resource_flow_burn_in(&mut session, fixture)
            .unwrap_or_else(|error| panic!("{} convergence soak: {error}", fixture.name));
        assert_eq!(report.ticks_checked, fixture.ticks);
        assert!(report.total_ops > 0);
        assert!(report.n_bands > 0);
        if fixture.require_bit_exact {
            assert_eq!(report.max_abs_error.to_bits(), 0.0_f32.to_bits());
            assert!(report.replay_bit_exact);
        }
        println!(
            "ACCUMULATOR-CONVERGENCE-E11 scenario={} participants={} ticks={} syncs={} admissions={} max_abs_error={} replay_bit_exact={}",
            fixture.name,
            fixture.participant_count,
            report.ticks_checked,
            report.sync_cycles_checked,
            report.admissions_observed,
            report.max_abs_error,
            report.replay_bit_exact,
        );
    }

    let replay_fixture = fixture_replay_static();
    let mut first = open_fixture_session(&replay_fixture).expect("open first replay session");
    let mut second = clone_for_replay(&first, &replay_fixture);
    let first_report =
        run_resource_flow_burn_in(&mut first, &replay_fixture).expect("first replay burn");
    let second_report =
        run_resource_flow_burn_in(&mut second, &replay_fixture).expect("second replay burn");
    assert_eq!(first_report, second_report);
}

fn empty_resource_economy() -> Value {
    json!({
        "transfers": [],
        "recipes": [],
        "emissions": [],
        "emit_on_threshold": []
    })
}

fn retired_mode_key() -> String {
    ["opt", "_in_mode"].concat()
}

fn assert_unknown_field<T: DeserializeOwned>(authored: Value, key: &str, workload: &str) {
    let error = serde_json::from_value::<T>(authored)
        .err()
        .unwrap_or_else(|| panic!("retired selector was admitted for {workload}"));
    let diagnostic = error.to_string();
    assert!(
        diagnostic.contains("unknown field") && diagnostic.contains(key),
        "retired selector did not fail at {workload} admission: {diagnostic}"
    );
}

fn retired_vocabulary() -> Vec<String> {
    vec![
        retired_mode_key(),
        ["ResourceEconomy", "OptInMode"].concat(),
        ["ResourceFlow", "OptInMode"].concat(),
        ["ResourceFlowExecution", "Profile"].concat(),
        ["resource_flow_execution", "_profile"].concat(),
        ["use_accumulator_", "transfer"].concat(),
        ["use_accumulator_", "emission"].concat(),
        ["use_accumulator_", "resource_flow"].concat(),
        ["apply_resource_economy_", "opt_in"].concat(),
        ["resolve_resource_flow_", "execution"].concat(),
        ["resource_flow_", "opt_in"].concat(),
    ]
}

fn is_scanned_surface(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("rs" | "ron" | "clause" | "json" | "toml" | "yaml" | "yml")
    )
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out);
        } else {
            out.push(path);
        }
    }
}
