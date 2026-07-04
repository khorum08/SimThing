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
