use simthing_mapgenerator::{
    metadata_passthrough_report, LatticeCoord, MapGeneratorParams, PlacedSystemSeed,
    ScenarioEmitter, ScenarioEmitterConfig, ShapePlacement, SquareLattice,
};

#[test]
fn metadata_passthrough_is_inert_or_deferred() {
    let params = MapGeneratorParams::default();
    let report = metadata_passthrough_report(&params);
    assert!(report.deferred);
    assert!(report.reason.contains("not admitted"));
    assert_eq!(report.captured.num_empires, params.metadata.num_empires);
    assert_eq!(
        report.captured.crisis_strength,
        params.metadata.crisis_strength
    );

    let lattice = SquareLattice::new(8).expect("lattice");
    let placement = ShapePlacement {
        systems: vec![PlacedSystemSeed {
            id: 0,
            coord: LatticeCoord { col: 1, row: 1 },
            bucket: None,
        }],
    };
    let emitter = ScenarioEmitter::with_default_config();
    let text = emitter
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit")
        .into_string();
    assert!(!text.contains("num_empires"));
    assert!(!text.contains("crisis_strength"));
}

#[test]
fn initializer_bucket_refs_are_deterministic() {
    let mut params = MapGeneratorParams::default();
    params.initializers.initializer_bucket_core = "core_initializer".into();
    params.initializers.initializer_bucket_arm = "arm_initializer".into();
    params.nebula.num_nebulas = 0;
    let lattice = SquareLattice::new(8).expect("lattice");
    let placement = ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: Some("core_initializer".into()),
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 1, row: 0 },
                bucket: Some("arm_initializer".into()),
            },
        ],
    };
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let a = emitter
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit a")
        .into_string();
    let b = emitter
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit b")
        .into_string();
    assert_eq!(a, b);
    assert!(a.contains("initializer = core_initializer"));
    assert!(a.contains("initializer = arm_initializer"));
}

#[test]
fn initializer_bucket_refs_emit_sibling_initializers_once() {
    let mut params = MapGeneratorParams::default();
    params.initializers.initializer_bucket_core = "core_initializer".into();
    params.initializers.initializer_bucket_arm = "arm_initializer".into();
    params.nebula.num_nebulas = 0;
    let lattice = SquareLattice::new(8).expect("lattice");
    let placement = ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: Some("core_initializer".into()),
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 1, row: 0 },
                bucket: Some("arm_initializer".into()),
            },
        ],
    };
    let emitter = ScenarioEmitter::with_default_config();
    let text = emitter
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit")
        .into_string();
    assert_eq!(text.matches("    core_initializer = {").count(), 1);
    assert_eq!(text.matches("    arm_initializer = {").count(), 1);
}

#[test]
fn shared_initializer_refs_remain_valid() {
    let mut params = MapGeneratorParams::default();
    params.nebula.num_nebulas = 0;
    let lattice = SquareLattice::new(8).expect("lattice");
    let shared = "shared_initializer".to_string();
    let placement = ShapePlacement {
        systems: vec![
            PlacedSystemSeed {
                id: 0,
                coord: LatticeCoord { col: 0, row: 0 },
                bucket: Some(shared.clone()),
            },
            PlacedSystemSeed {
                id: 1,
                coord: LatticeCoord { col: 1, row: 0 },
                bucket: Some(shared),
            },
        ],
    };
    let emitter = ScenarioEmitter::with_default_config();
    let text = emitter
        .emit(&params, &lattice, &placement, None, None)
        .expect("emit")
        .into_string();
    assert_eq!(text.matches("    shared_initializer = {").count(), 1);
    assert_eq!(text.matches("initializer = shared_initializer").count(), 2);
}

#[test]
fn crate_still_has_no_forbidden_runtime_deps() {
    let manifest = include_str!("../Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
        "simthing-clausething",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "Cargo.toml must not depend on {forbidden}"
        );
    }
}
