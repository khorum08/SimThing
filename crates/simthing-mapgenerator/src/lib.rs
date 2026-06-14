//! Standalone MapGeneratorCLI producer library (0.0.8.6).
//!
//! PR1: parameter model, shape registry descriptor shell, and validation only.
//! PR2: deterministic RNG, square lattice, core mask, occupancy primitives.
//! PR3: ShapeStrategy trait, registry dispatch, elliptical/static in-memory seams.
//! PR4: deterministic `static_galaxy_scenario` neutral-AST text emitter.

pub mod emitter;
pub mod lattice;
pub mod occupancy;
pub mod params;
pub mod rng;
pub mod shape_registry;
pub mod strategies;
pub mod strategy;

pub use emitter::{
    ScenarioEmitError, ScenarioEmitter, ScenarioEmitterConfig, ScenarioText,
    DEFAULT_INITIALIZER_DISPLAY_NAME, DEFAULT_INITIALIZER_REF, DEFAULT_SCENARIO_ID,
};
pub use lattice::{CoreMask, LatticeCoord, LatticeError, SquareLattice};
pub use occupancy::{OccupancyError, OccupancyGrid};
pub use params::{
    ArbitraryHyperlaneSourceMode, ArbitraryStaticParams, ClusterCountMethod, ClusteringParams,
    GenerationMode, HyperlaneGeometryParams, InertMetadataParams, InitializerBucketParams,
    MapGeneratorParams, NebulaFieldParams, OutputFormat, OutputParams, PartitionMethod,
    PartitioningParams, ScaleCoreParams, ShapeParams, SpecialRouteParams, ValidationError,
};
pub use rng::{MapGenRng, MapGenSeed};
pub use shape_registry::{
    RegisteredShapeName, RegistryResolveError, ShapeParameterDescriptor, ShapeRegistry,
    ShapeStrategyDescriptor,
};
pub use strategy::{
    PlacedSystemSeed, ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext,
};

/// Validate params against the default vanilla registry.
pub fn validate_default(params: &MapGeneratorParams) -> Result<(), ValidationError> {
    params.validate(&ShapeRegistry::default())
}

/// Build lattice, core mask, occupancy grid, and RNG from validated params.
pub fn build_placement_context(
    params: &MapGeneratorParams,
) -> Result<(SquareLattice, CoreMask, OccupancyGrid, MapGenRng), LatticeError> {
    let edge = SquareLattice::edge_from_scale(&params.scale_core)?;
    let lattice = SquareLattice::new(edge)?;
    let core_mask =
        lattice.core_mask_from_scale(params.scale_core.core_radius, params.scale_core.radius);
    let occupancy = OccupancyGrid::new(lattice.clone(), core_mask.clone());
    let rng = MapGenRng::from_seed(MapGenSeed::new(params.seed));
    Ok((lattice, core_mask, occupancy, rng))
}

/// Place via registry and emit declarative scenario text (PR3 + PR4 pipeline).
pub fn place_and_emit_scenario(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
    explicit_cells: Option<&[LatticeCoord]>,
    emitter: &ScenarioEmitter,
) -> Result<ScenarioText, PlaceAndEmitError> {
    validate_default(params)?;
    let (lattice, core_mask, mut occupancy, mut rng) = build_placement_context(params)?;
    let placement = registry.place(
        params,
        &lattice,
        &core_mask,
        &mut occupancy,
        &mut rng,
        explicit_cells,
    )?;
    let text = emitter.emit(params, &lattice, &placement)?;
    Ok(text)
}

#[derive(Debug, thiserror::Error)]
pub enum PlaceAndEmitError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("lattice error: {0}")]
    Lattice(#[from] LatticeError),
    #[error("placement error: {0}")]
    Placement(#[from] ShapePlacementError),
    #[error("emit error: {0}")]
    Emit(#[from] ScenarioEmitError),
}
