//! Standalone MapGeneratorCLI producer library (0.0.8.6).
//!
//! PR1: parameter model, shape registry descriptor shell, and validation only.
//! PR2: deterministic RNG, square lattice, core mask, occupancy primitives.
//! PR3: ShapeStrategy trait, registry dispatch, elliptical/static in-memory seams.
//! PR4: deterministic `static_galaxy_scenario` neutral-AST text emitter.
//! PR6: bounded hyperlane topology + `add_hyperlane` emission.
//! PR6b: bounded wormhole/gateway special routes as long-range `add_hyperlane` pairs.
//! PR7: bounded partition/cluster assignment and cross-group bridge `add_hyperlane` pairs.
//! PR8: single-source vanilla shape registry + executable strategy dispatch.
//! PR9: nebula field declarations, initializer bucket emission, inert metadata reporting.
//! PR11: scale envelope hardening (u64 lattice capacity, occupancy free-list, bounded pair enumeration).
//! Post-closeout: producer-side 1000×1000 PNG preview for UI handoff (not runtime semantics).

pub mod cluster;
pub mod emitter;
pub mod field_operator;
pub mod lattice;
pub mod metadata;
pub mod nebula;
pub mod occupancy;
pub mod pair_candidates;
pub mod params;
pub mod partition;
pub mod preview_png;
pub mod rng;
pub mod shape_registry;
pub mod special_routes;
pub mod strategies;
pub mod strategy;
pub mod success_galaxy;
pub mod topology;

pub use cluster::{
    assign_clusters, generate_cluster_bridges, validate_cluster_options, ClusterAssignment,
    ClusterBridgeEdge, ClusterError, ClusterId, ClusterOptions, ClusterReport,
};
pub use emitter::{
    ScenarioEmitError, ScenarioEmitter, ScenarioEmitterConfig, ScenarioText,
    DEFAULT_INITIALIZER_DISPLAY_NAME, DEFAULT_INITIALIZER_REF, DEFAULT_SCENARIO_ID,
};
pub use field_operator::{forbidden_field_surface_term, ACCEPTED_NEBULA_KEYS};
pub use lattice::{CoreMask, LatticeCoord, LatticeError, SquareLattice};
pub use metadata::{metadata_passthrough_report, MetadataPassthroughReport};
pub use nebula::{
    apply_cluster_initializer_buckets, place_nebulas, NebulaError, NebulaField, NebulaOptions,
    NebulaReport,
};
pub use occupancy::{OccupancyError, OccupancyGrid};
pub use pair_candidates::{
    build_position_index, collect_farthest_pairs_with_filter, collect_pairs_with_filter,
    collect_pairs_within_chebyshev, PairCandidateStats, PRODUCER_MAX_HYPERLANE_DISTANCE,
    PRODUCER_PAIR_CANDIDATE_CAP,
};
pub use params::{
    ArbitraryHyperlaneSourceMode, ArbitraryStaticParams, ClusterCountMethod, ClusteringParams,
    GenerationMode, HyperlaneGeometryParams, InertMetadataParams, InitializerBucketParams,
    MapGeneratorParams, NebulaFieldParams, OutputFormat, OutputParams, PartitionMethod,
    PartitioningParams, ScaleCoreParams, ShapeParams, SpecialRouteParams, ValidationError,
};
pub use partition::{
    assign_partitions, generate_partition_bridges, validate_bridge_edges,
    validate_partition_options, BridgeEdge, PartitionAssignment, PartitionError, PartitionId,
    PartitionKind, PartitionOptions, PartitionReport,
};
pub use preview_png::{
    render_galaxy_preview_png, write_galaxy_preview_png, GalaxyPreviewScene, PreviewPngError,
    GALAXY_PREVIEW_PNG_SIZE,
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
pub use success_galaxy::success_galaxy_1000_params;
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

/// Full producer galaxy generation output (scenario text + preview scene inputs).
#[derive(Debug, Clone)]
pub struct GalaxyGenerationResult {
    pub scenario: ScenarioText,
    pub lattice: SquareLattice,
    pub core_mask: CoreMask,
    pub placement: ShapePlacement,
    pub hyperlane_edges: Vec<HyperlaneEdge>,
    pub nebulas: Vec<NebulaField>,
}

impl GalaxyGenerationResult {
    pub fn preview_scene(&self) -> GalaxyPreviewScene {
        GalaxyPreviewScene {
            lattice: self.lattice.clone(),
            core_mask: self.core_mask.clone(),
            placement: self.placement.clone(),
            hyperlane_edges: self.hyperlane_edges.clone(),
            nebulas: self.nebulas.clone(),
        }
    }

    pub fn render_preview_png(&self) -> Result<Vec<u8>, PreviewPngError> {
        render_galaxy_preview_png(&self.preview_scene())
    }
}

/// Build bounded topology options from params using requested star count as fixture capacity.
pub fn structure_options_from_params(
    params: &MapGeneratorParams,
) -> Result<
    (
        HyperlaneOptions,
        SpecialRouteOptions,
        PartitionOptions,
        ClusterOptions,
    ),
    HyperlaneError,
> {
    let fixture_edge = fixture_lattice_edge_for_system_count(params.scale_core.num_stars as usize)?;
    Ok((
        HyperlaneOptions::from_params(params, fixture_edge),
        SpecialRouteOptions::from_params(params, fixture_edge),
        PartitionOptions::from_params(params, fixture_edge),
        ClusterOptions::from_params(params),
    ))
}

/// Generate scenario text plus preview inputs using the full bounded producer pipeline.
pub fn generate_galaxy_with_structure(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
    explicit_cells: Option<&[LatticeCoord]>,
    emitter: &ScenarioEmitter,
    hyperlane_options: Option<HyperlaneOptions>,
    special_route_options: Option<SpecialRouteOptions>,
    partition_options: Option<PartitionOptions>,
    cluster_options: Option<ClusterOptions>,
) -> Result<GalaxyGenerationResult, PlaceAndEmitError> {
    validate_default(params)?;
    let (lattice, core_mask, mut occupancy, mut rng) = build_placement_context(params)?;
    let mut placement = registry.place(
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
    if let Some(ref options) = partition_options {
        let existing: Vec<(String, String)> = hyperlane_edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone()))
            .collect();
        let (assignments, _report) = assign_partitions(&placement, options)?;
        let (bridges, _report) =
            generate_partition_bridges(&placement, &assignments, options, &existing, &mut rng)?;
        hyperlane_edges.extend(bridges.into_iter().map(|edge| edge.to_hyperlane_edge()));
    }
    let mut cluster_assignments = None;
    if let Some(options) = cluster_options {
        let existing: Vec<(String, String)> = hyperlane_edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone()))
            .collect();
        let (assignments, _report) = assign_clusters(&placement, &options)?;
        let bridge_bounds = partition_options.as_ref().map(|options| {
            (
                options.min_bridges,
                options.max_bridges,
                options.fixture_lattice_edge,
                options.max_per_node_fanout,
            )
        });
        let (min_bridges, max_bridges, fixture_edge, fanout) = bridge_bounds.unwrap_or((
            0,
            1,
            fixture_lattice_edge_for_system_count(placement.systems.len())?,
            DEFAULT_MAX_PER_NODE_FANOUT,
        ));
        let (bridges, _report) = generate_cluster_bridges(
            &placement,
            &assignments,
            fixture_edge,
            min_bridges,
            max_bridges,
            fanout,
            &existing,
            &mut rng,
        )?;
        hyperlane_edges.extend(bridges.into_iter().map(|edge| edge.to_hyperlane_edge()));
        cluster_assignments = Some(assignments);
    }
    if let Some(assignments) = cluster_assignments.as_ref() {
        apply_cluster_initializer_buckets(
            &mut placement,
            assignments,
            &params.initializers.initializer_bucket_cluster,
        );
    }
    let nebula_options = NebulaOptions::from_params(params);
    let (nebulas, _nebula_report) = place_nebulas(&placement, &lattice, nebula_options, &mut rng)?;
    let hyperlanes = if hyperlane_edges.is_empty() {
        None
    } else {
        Some(HyperlaneTopology {
            edges: hyperlane_edges.clone(),
        })
    };
    let scenario = emitter.emit(
        params,
        &lattice,
        &placement,
        hyperlanes.as_ref(),
        Some(&nebulas),
    )?;
    Ok(GalaxyGenerationResult {
        scenario,
        lattice,
        core_mask,
        placement,
        hyperlane_edges,
        nebulas,
    })
}

/// Generate the proven PR11 1000-star success galaxy and a 1000×1000 preview PNG.
pub fn generate_success_galaxy_with_preview(
    registry: &ShapeRegistry,
) -> Result<GalaxyGenerationResult, PlaceAndEmitError> {
    let params = success_galaxy_1000_params();
    let (hyperlane, special, partition, cluster) = structure_options_from_params(&params)?;
    generate_galaxy_with_structure(
        &params,
        registry,
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(hyperlane),
        Some(special),
        Some(partition),
        Some(cluster),
    )
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
    place_and_emit_scenario_with_structure(
        params,
        registry,
        explicit_cells,
        emitter,
        hyperlane_options,
        None,
        None,
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
    place_and_emit_scenario_with_structure(
        params,
        registry,
        explicit_cells,
        emitter,
        hyperlane_options,
        special_route_options,
        None,
        None,
    )
}

/// Place, optionally generate topology couplings and partition/cluster bridges, and emit scenario text (PR6–PR7).
pub fn place_and_emit_scenario_with_structure(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
    explicit_cells: Option<&[LatticeCoord]>,
    emitter: &ScenarioEmitter,
    hyperlane_options: Option<HyperlaneOptions>,
    special_route_options: Option<SpecialRouteOptions>,
    partition_options: Option<PartitionOptions>,
    cluster_options: Option<ClusterOptions>,
) -> Result<ScenarioText, PlaceAndEmitError> {
    Ok(generate_galaxy_with_structure(
        params,
        registry,
        explicit_cells,
        emitter,
        hyperlane_options,
        special_route_options,
        partition_options,
        cluster_options,
    )?
    .scenario)
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
    #[error("partition error: {0}")]
    Partition(#[from] PartitionError),
    #[error("cluster error: {0}")]
    Cluster(#[from] ClusterError),
    #[error("nebula error: {0}")]
    Nebula(#[from] NebulaError),
    #[error("emit error: {0}")]
    Emit(#[from] ScenarioEmitError),
}
