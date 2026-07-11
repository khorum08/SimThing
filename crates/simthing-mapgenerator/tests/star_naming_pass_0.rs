//! STUDIO-STAR-NAMING-PASS-0 generator and emitter proofs.

use std::collections::BTreeSet;

use simthing_mapgenerator::{
    assign_star_names, HyperlaneEdge, HyperlaneTopology, LatticeCoord, MapGeneratorParams,
    PlacedSystemSeed, ScenarioEmitter, ScenarioEmitterConfig, ShapePlacement, SquareLattice,
};

fn placement(count: u32) -> ShapePlacement {
    ShapePlacement {
        systems: (0..count)
            .map(|id| PlacedSystemSeed {
                id,
                coord: LatticeCoord {
                    col: id % 32,
                    row: id / 32,
                },
                bucket: None,
            })
            .collect(),
    }
}

fn without_name_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| !line.trim_start().starts_with("name ="))
        .collect()
}

#[test]
fn star_naming_pass_assigns_non_empty_names() {
    let names = assign_star_names(
        770_421,
        placement(64).systems.iter().map(|system| system.id),
    );
    assert_eq!(names.len(), 64);
    assert!(names
        .iter()
        .all(|name| !name.display_name.trim().is_empty()));
}

#[test]
fn star_naming_pass_is_seed_stable() {
    let ids = [9, 2, 7, 1, 4];
    let first = assign_star_names(99, ids);
    let repeat = assign_star_names(99, ids.into_iter().rev());
    let different_seed = assign_star_names(100, ids);
    assert_eq!(first, repeat);
    assert_ne!(first, different_seed);

    let mut params = MapGeneratorParams::default();
    params.seed = 99;
    let placement = placement(5);
    let lattice = SquareLattice::new(32).expect("lattice");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let emit = || {
        emitter
            .emit(&params, &lattice, &placement, None, None)
            .expect("emit")
    };
    assert_eq!(emit(), emit());
}

#[test]
fn star_naming_pass_preserves_structure() {
    let placement = placement(128);
    let before = placement.clone();
    let _ = assign_star_names(42, placement.systems.iter().map(|system| system.id));
    assert_eq!(placement, before);

    let lattice = SquareLattice::new(32).expect("lattice");
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig {
        scenario_id: "structure_hold".into(),
        scenario_display_name: "Structure Hold".into(),
        default_initializer_ref: "example_rim_initializer".into(),
    });
    let mut first_params = MapGeneratorParams::default();
    first_params.seed = 42;
    let mut second_params = first_params.clone();
    second_params.seed = 43;
    let topology = HyperlaneTopology {
        edges: vec![HyperlaneEdge {
            from: "0".into(),
            to: "1".into(),
        }],
    };
    let first = emitter
        .emit(&first_params, &lattice, &placement, Some(&topology), None)
        .expect("first emit")
        .into_string();
    let second = emitter
        .emit(&second_params, &lattice, &placement, Some(&topology), None)
        .expect("second emit")
        .into_string();
    assert_eq!(without_name_lines(&first), without_name_lines(&second));
}

#[test]
fn star_naming_pass_names_are_unique_within_galaxy() {
    let names = assign_star_names(770_421, 0..5_000);
    let unique: BTreeSet<_> = names
        .iter()
        .map(|name| name.display_name.as_str())
        .collect();
    assert_eq!(unique.len(), names.len());
}

#[test]
fn star_naming_emitter_writes_names_not_blank() {
    let mut params = MapGeneratorParams::default();
    params.seed = 770_421;
    let placement = placement(8);
    let lattice = SquareLattice::new(32).expect("lattice");
    let emitted = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params))
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit")
        .into_string();

    assert!(!emitted.contains("name = \"\""));
    for assignment in assign_star_names(params.seed, 0..8) {
        assert!(emitted.contains(&format!("name = \"{}\"", assignment.display_name)));
    }
}
