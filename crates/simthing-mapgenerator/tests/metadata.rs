use simthing_mapgenerator::{
    metadata_passthrough_report, LatticeCoord, MapGeneratorParams, PlacedSystemSeed,
    ScenarioEmitter, ScenarioEmitterConfig, ShapePlacement, SquareLattice,
};

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
