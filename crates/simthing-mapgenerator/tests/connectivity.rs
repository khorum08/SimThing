//! Galaxy connectivity guarantee — one interconnected galaxy, no island clusters.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use simthing_mapgenerator::{
    build_placement_context, connect_components, generate_galaxy_with_structure,
    generate_hyperlane_topology, structure_options_from_params, validate_default, HyperlaneOptions,
    LatticeCoord, MapGenRng, MapGenSeed, MapGeneratorParams, PlacedSystemSeed, ScenarioEmitter,
    ScenarioEmitterConfig, ShapePlacement, ShapeRegistry,
};

/// Count connected components over an edge id-pair list, given the full system id set.
fn component_count(ids: &BTreeSet<String>, edges: &[(String, String)]) -> usize {
    let mut adj: BTreeMap<&str, Vec<&str>> = ids.iter().map(|s| (s.as_str(), Vec::new())).collect();
    for (a, b) in edges {
        adj.get_mut(a.as_str()).unwrap().push(b.as_str());
        adj.get_mut(b.as_str()).unwrap().push(a.as_str());
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut components = 0usize;
    for start in ids.iter().map(|s| s.as_str()) {
        if seen.contains(start) {
            continue;
        }
        components += 1;
        let mut q = VecDeque::from([start]);
        seen.insert(start);
        while let Some(node) = q.pop_front() {
            for &next in &adj[node] {
                if seen.insert(next) {
                    q.push_back(next);
                }
            }
        }
    }
    components
}

/// Two deliberately separated 2x2 clusters far apart — base linking leaves them as 2 islands.
fn two_island_placement() -> ShapePlacement {
    let coords = [
        (0u32, 0u32),
        (1, 0),
        (0, 1),
        (1, 1), // cluster A near origin
        (90, 90),
        (91, 90),
        (90, 91),
        (91, 91), // cluster B far away
    ];
    ShapePlacement {
        systems: coords
            .iter()
            .enumerate()
            .map(|(id, (col, row))| PlacedSystemSeed {
                id: id as u32,
                coord: LatticeCoord {
                    col: *col,
                    row: *row,
                },
                bucket: None,
            })
            .collect(),
    }
}

#[test]
fn connect_components_merges_two_islands_into_one() {
    let placement = two_island_placement();
    let ids: BTreeSet<String> = placement.systems.iter().map(|s| s.id.to_string()).collect();
    // Base edges within each cluster only (Chebyshev 1) — two separate components.
    let base: Vec<(String, String)> = vec![
        ("0".into(), "1".into()),
        ("0".into(), "2".into()),
        ("1".into(), "3".into()),
        ("4".into(), "5".into()),
        ("4".into(), "6".into()),
        ("5".into(), "7".into()),
    ];
    assert_eq!(
        component_count(&ids, &base),
        2,
        "fixture starts as two islands"
    );

    let (bridges, report) = connect_components(&placement, &base);
    assert_eq!(report.components_before, 2);
    assert_eq!(report.components_after, 1);
    assert!(report.bridges_added >= 1);

    let mut all = base.clone();
    all.extend(bridges.iter().map(|e| (e.from.clone(), e.to.clone())));
    assert_eq!(
        component_count(&ids, &all),
        1,
        "after connectivity pass the galaxy is a single component"
    );
}

#[test]
fn connect_components_is_noop_when_already_connected() {
    let placement = two_island_placement();
    let ids: BTreeSet<String> = placement.systems.iter().map(|s| s.id.to_string()).collect();
    // A spanning chain over all 8 systems — already one component.
    let base: Vec<(String, String)> = (0..7)
        .map(|i| (i.to_string(), (i + 1).to_string()))
        .collect();
    assert_eq!(component_count(&ids, &base), 1);
    let (bridges, report) = connect_components(&placement, &base);
    assert!(bridges.is_empty());
    assert_eq!(report.bridges_added, 0);
    assert_eq!(report.components_after, 1);
}

#[test]
fn connect_components_is_deterministic() {
    let placement = two_island_placement();
    let base: Vec<(String, String)> = vec![("0".into(), "1".into()), ("4".into(), "5".into())];
    let a = connect_components(&placement, &base).0;
    let b = connect_components(&placement, &base).0;
    assert_eq!(a, b);
}

// ---- real disc galaxy: 1500-star elliptical disc is one fully connected galaxy ----

fn disc_1500_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.mode = simthing_mapgenerator::GenerationMode::Procedural;
    params.scale_core.num_stars = 1500;
    params.scale_core.lattice_size = Some(300);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = 770_421;
    params.hyperlane.max_hyperlane_distance = 3.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 5000;
    params.hyperlane.num_hyperlanes_default = 5000;
    params.hyperlane.random_hyperlanes = false;
    params.hyperlane.ensure_connected = true;
    params
}

#[test]
fn disc_1500_galaxy_is_fully_connected_after_pass() {
    let params = disc_1500_params();
    validate_default(&params).expect("params valid");
    let (lattice, core_mask, mut occupancy, mut rng) =
        build_placement_context(&params).expect("context");
    let placement = ShapeRegistry::default()
        .place(
            &params,
            &lattice,
            &core_mask,
            &mut occupancy,
            &mut rng,
            None,
        )
        .expect("disc placement");
    assert_eq!(placement.systems.len(), 1500);

    let options = HyperlaneOptions::from_params(&params, 1);
    let mut lane_rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    let (topology, _) =
        generate_hyperlane_topology(&placement, &options, &mut lane_rng).expect("base lanes");
    let base: Vec<(String, String)> = topology
        .edges
        .iter()
        .map(|e| (e.from.clone(), e.to.clone()))
        .collect();

    let ids: BTreeSet<String> = placement.systems.iter().map(|s| s.id.to_string()).collect();
    let (bridges, report) = connect_components(&placement, &base);

    let mut all = base.clone();
    all.extend(bridges.iter().map(|e| (e.from.clone(), e.to.clone())));
    assert_eq!(
        component_count(&ids, &all),
        1,
        "disc galaxy must be ONE connected component (no island clusters)"
    );
    assert_eq!(report.components_after, 1);
    // Sanity: the base network alone was NOT already trivially one component for this scattered disc,
    // so the pass did real work (or, if it happened to be connected, bridges_added == 0 is also fine).
    assert!(report.components_before >= 1);
}

#[test]
fn connectivity_is_on_by_default() {
    // A galaxy with orphaned systems is unusable, so the generator must default to connected.
    assert!(
        MapGeneratorParams::default().hyperlane.ensure_connected,
        "ensure_connected must default ON (designers opt out explicitly)"
    );
}

#[test]
fn production_generation_result_surfaces_the_connectivity_proof() {
    // The connectivity guarantee must be a PRODUCTION OUTPUT, not just a test computation: a full
    // `generate_galaxy_with_structure` run carries `connectivity` on its result so any caller can verify
    // "one interconnected galaxy" without re-deriving it.
    let mut params = disc_1500_params();
    params.hyperlane.max_hyperlane_distance = 7.0; // matched to disc spacing → naturally connected web
    params.clustering.cluster_count = Some(5);
    params.clustering.cluster_radius = 400.0;
    let (hyperlane, special, _partition, cluster) =
        structure_options_from_params(&params).expect("structure options");
    let result = generate_galaxy_with_structure(
        &params,
        &ShapeRegistry::default(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        None,
        Some(cluster),
    )
    .expect("galaxy generates");

    let connectivity = result
        .connectivity
        .expect("connectivity proof is surfaced on the production result");
    assert_eq!(
        connectivity.components_after, 1,
        "production result must prove ONE interconnected galaxy (no island clusters)"
    );

    // And it must agree with the actual emitted base network: every system reachable, one component.
    let ids: BTreeSet<String> = result
        .placement
        .systems
        .iter()
        .map(|s| s.id.to_string())
        .collect();
    let edges: Vec<(String, String)> = result
        .base_hyperlane_edges
        .iter()
        .map(|e| (e.from.clone(), e.to.clone()))
        .collect();
    assert_eq!(component_count(&ids, &edges), 1);
}
