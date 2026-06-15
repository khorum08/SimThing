//! Proven 1000-star success-galaxy parameter preset (PR11 scale envelope).

use crate::params::{GenerationMode, MapGeneratorParams};

/// Parameter preset for the proven PR11 1000-star elliptical success galaxy.
pub fn success_galaxy_1000_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.mode = GenerationMode::Procedural;
    params.scale_core.num_stars = 1000;
    params.scale_core.lattice_size = Some(50);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = 11_000;
    params.hyperlane.max_hyperlane_distance = 3.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 12;
    params.hyperlane.num_hyperlanes_default = 6;
    params.hyperlane.random_hyperlanes = false;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 0;
    params.partitioning.home_system_partitions = 1;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 100;
    params.partitioning.partition_max_systems = 900;
    params.partitioning.partition_min_bridges = 0;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(4);
    params.clustering.cluster_radius = 50.0;
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 12.0;
    params.nebula.nebula_min_dist = 4.0;
    params
}
