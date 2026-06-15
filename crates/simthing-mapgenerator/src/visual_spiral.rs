//! Visual preview preset: 1500-star spiral_4 on a 300×300 lattice (producer-only).

use crate::params::{GenerationMode, MapGeneratorParams};
use crate::{
    generate_galaxy_with_structure, structure_options_from_params, GalaxyGenerationResult,
    PlaceAndEmitError, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
};

pub const VISUAL_SPIRAL_1500_SEED: u64 = 42_1500;
pub const VISUAL_SPIRAL_1500_STARS: u32 = 1500;
pub const VISUAL_SPIRAL_1500_LATTICE_EDGE: u32 = 300;

/// Parameter preset for the visual remediation spiral galaxy preview.
pub fn visual_spiral_1500_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "spiral_4".into();
    params.mode = GenerationMode::Procedural;
    params.scale_core.num_stars = VISUAL_SPIRAL_1500_STARS;
    params.scale_core.lattice_size = Some(VISUAL_SPIRAL_1500_LATTICE_EDGE);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = VISUAL_SPIRAL_1500_SEED;
    params.hyperlane.max_hyperlane_distance = 4.0;
    params.hyperlane.num_hyperlanes_min = 3;
    params.hyperlane.num_hyperlanes_max = 18;
    params.hyperlane.num_hyperlanes_default = 10;
    params.hyperlane.random_hyperlanes = false;
    params.special_routes.num_wormhole_pairs = 0;
    params.special_routes.num_gateways = 0;
    params.partitioning.home_system_partitions = 1;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 200;
    params.partitioning.partition_max_systems = 1400;
    params.partitioning.partition_min_bridges = 0;
    params.partitioning.partition_max_bridges = 0;
    params.clustering.cluster_count = None;
    params.nebula.num_nebulas = 0;
    params
}

pub fn generate_visual_spiral_1500(
    registry: &ShapeRegistry,
) -> Result<GalaxyGenerationResult, PlaceAndEmitError> {
    let params = visual_spiral_1500_params();
    let (hyperlane, special, partition, _cluster) = structure_options_from_params(&params)?;
    generate_galaxy_with_structure(
        &params,
        registry,
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        Some(partition),
        None,
    )
}
