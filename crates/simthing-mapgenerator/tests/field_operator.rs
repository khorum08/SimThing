use simthing_mapgenerator::{
    build_placement_context, field_operator::emit_nebula_declarations,
    forbidden_field_surface_term, place_nebulas, LatticeCoord, MapGenRng, MapGenSeed,
    MapGeneratorParams, NebulaField, NebulaOptions, PlacedSystemSeed, ShapePlacement,
    ACCEPTED_NEBULA_KEYS,
};

fn sample_nebulas() -> Vec<NebulaField> {
    let mut params = MapGeneratorParams::default();
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 25.0;
    params.nebula.nebula_min_dist = 1.0;
    params.seed = 9191;
    let (lattice, _, _, _) = build_placement_context(&params).expect("ctx");
    let placement = ShapePlacement {
        systems: vec![PlacedSystemSeed {
            id: 0,
            coord: LatticeCoord { col: 2, row: 2 },
            bucket: None,
        }],
    };
    let options = NebulaOptions::from_params(&params);
    let mut rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    place_nebulas(&placement, &lattice, options, &mut rng)
        .expect("place")
        .0
}

#[test]
fn field_operator_emission_uses_only_accepted_keys() {
    let mut out = String::new();
    emit_nebula_declarations(&mut out, &sample_nebulas());
    assert!(out.contains("nebula = {"));
    assert!(out.contains("name = \"generated_nebula_0\""));
    assert!(out.contains("radius = 25"));
    for line in out.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("nebula =") || trimmed == "}" || trimmed.is_empty() {
            continue;
        }
        let key = trimmed.split('=').next().unwrap_or("").trim();
        assert!(
            ACCEPTED_NEBULA_KEYS.contains(&key),
            "unexpected nebula key {key:?} in {trimmed:?}"
        );
    }
}

#[test]
fn field_operator_emission_has_no_route_path_predecessor_movement_border_frontline_terms() {
    let mut out = String::new();
    emit_nebula_declarations(&mut out, &sample_nebulas());
    assert!(forbidden_field_surface_term(&out).is_none());
    assert!(!out.contains("field_operator"));
}
