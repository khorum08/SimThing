//! Standalone MapGeneratorCLI producer library (0.0.8.6).
//!
//! PR1: parameter model, shape registry descriptor shell, and validation only.
//! PR2: deterministic RNG, square lattice, core mask, occupancy primitives.
//! PR3: ShapeStrategy trait, registry dispatch, elliptical/static in-memory seams.
//! PR4: deterministic `static_galaxy_scenario` neutral-AST text emitter.
//! PR6: bounded hyperlane topology + `add_hyperlane` emission.
//! PR6b: bounded wormhole/gateway special routes as long-range `add_hyperlane` pairs.

pub mod emitter;
pub mod lattice;
pub mod occupancy;
pub mod params;
pub mod rng;
pub mod shape_registry;
pub mod special_routes;
pub mod strategies;
pub mod strategy;
pub mod topology;

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
pub use special_routes::{
    generate_special_routes, validate_special_route_edges, validate_special_route_options,
    SpecialRouteEdge, SpecialRouteError, SpecialRouteKind, SpecialRouteOptions, SpecialRouteReport,
    SpecialRouteTopology,
};
pub use strategy::{
    PlacedSystemSeed, ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext,
};
pub use topology::{
    canonical_pair, fixture_lattice_edge_for_system_count, generate_hyperlane_topology,
    grid_chebyshev_distance, lowered_grid_position, system_id_scalar, validate_hyperlane_edges,
    HyperlaneEdge, HyperlaneError, HyperlaneGenerationReport, HyperlaneOptions, HyperlaneTopology,
    DEFAULT_MAX_PER_NODE_FANOUT,
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
    place_and_emit_scenario_with_hyperlanes(params, registry, explicit_cells, emitter, None)
}

/// Place, optionally generate bounded hyperlanes, and emit declarative scenario text (PR6).
pub fn place_and_emit_scenario_with_hyperlanes(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
    explicit_cells: Option<&[LatticeCoord]>,
    emitter: &ScenarioEmitter,
    hyperlane_options: Option<HyperlaneOptions>,
) -> Result<ScenarioText, PlaceAndEmitError> {
    place_and_emit_scenario_with_couplings(
        params,
        registry,
        explicit_cells,
        emitter,
        hyperlane_options,
        None,
    )
}

/// Place, optionally generate bounded hyperlanes and special routes, and emit scenario text (PR6/PR6b).
pub fn place_and_emit_scenario_with_couplings(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
    explicit_cells: Option<&[LatticeCoord]>,
    emitter: &ScenarioEmitter,
    hyperlane_options: Option<HyperlaneOptions>,
    special_route_options: Option<SpecialRouteOptions>,
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
    let mut hyperlane_edges = Vec::new();
    if let Some(options) = hyperlane_options {
        let (topology, _report) = generate_hyperlane_topology(&placement, &options, &mut rng)?;
        hyperlane_edges.extend(topology.edges);
    }
    if let Some(options) = special_route_options {
        let existing: Vec<(String, String)> = hyperlane_edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone()))
            .collect();
        let (topology, _report) =
            generate_special_routes(&placement, &options, &existing, &mut rng)?;
        hyperlane_edges.extend(
            topology
                .edges
                .into_iter()
                .map(|edge| edge.to_hyperlane_edge()),
        );
    }
    let hyperlanes = if hyperlane_edges.is_empty() {
        None
    } else {
        Some(HyperlaneTopology {
            edges: hyperlane_edges,
        })
    };
    let text = emitter.emit(params, &lattice, &placement, hyperlanes.as_ref())?;
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
    #[error("hyperlane error: {0}")]
    Hyperlane(#[from] HyperlaneError),
    #[error("special route error: {0}")]
    SpecialRoute(#[from] SpecialRouteError),
    #[error("emit error: {0}")]
    Emit(#[from] ScenarioEmitError),
}
