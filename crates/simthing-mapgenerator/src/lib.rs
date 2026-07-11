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
pub mod connectivity;
pub mod coupling;
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
pub mod report;
pub mod rng;
pub mod shape_param_spec;
pub mod shape_registry;
pub mod special_routes;
pub mod star_names;
pub mod strategies;
pub mod strategy;
pub mod success_galaxy;
pub mod topology;
pub mod visual_spiral;

pub use cluster::{
    assign_clusters, generate_cluster_bridges, validate_cluster_options, ClusterAssignment,
    ClusterBridgeEdge, ClusterError, ClusterId, ClusterOptions, ClusterReport,
};
pub use connectivity::{connect_components, ConnectivityReport};
pub use coupling::{ClassifiedCouplingEdge, CouplingEdgeKind};
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
    cell_center_pixel, collect_cell_center_pixels, collect_rendered_star_pixels,
    count_bridge_edges, deterministic_unit_hash, jitter_fraction_from_hash,
    render_galaxy_preview_png, render_galaxy_preview_png_bytes, rendered_star_pixel,
    star_render_radius, write_galaxy_preview_png, GalaxyPreviewOptions, GalaxyPreviewScene,
    HyperlanePreviewFilter, PreviewPngError, DEFAULT_HYPERLANE_RGBA, GALAXY_PREVIEW_PNG_SIZE,
};
pub use report::{
    build_generation_report, generation_report_to_json, map_quality_is_acceptable_for_editor,
    normalized_report_json, write_generation_report_json, GenerationReport, ReportArtifacts,
    ReportError, CONNECTIVITY_BRIDGE_RATIO_FAIL_THRESHOLD,
    CONNECTIVITY_BRIDGE_RATIO_WARN_THRESHOLD, DENSE_PREVIEW_AVG_DEGREE_WARN_THRESHOLD,
    LONGEST_BRIDGE_CHEBYSHEV_WARN_THRESHOLD, MAP_QUALITY_FAIL, MAP_QUALITY_PASS, MAP_QUALITY_WARN,
    REPORT_SCHEMA_VERSION, TOPOLOGY_TARGET_RATIO_FAIL_THRESHOLD,
};
pub use rng::{MapGenRng, MapGenSeed};
pub use shape_param_spec::{
    apply_cli_shape_params, parse_shape_param_assignment, shape_param_specs, spec_for_key,
    validate_shape_params, validate_shape_params_for_params, ShapeParamParseError, ShapeParamSpec,
};
pub use shape_registry::{
    RegisteredShapeName, RegistryResolveError, ShapeParameterDescriptor, ShapeRegistry,
    ShapeStrategyDescriptor,
};
pub use special_routes::{
    generate_special_routes, validate_special_route_edges, validate_special_route_options,
    SpecialRouteEdge, SpecialRouteError, SpecialRouteKind, SpecialRouteOptions, SpecialRouteReport,
    SpecialRouteTopology,
};
pub use star_names::{
    assign_star_names, star_name_for_index_or_seed, StarNameAssignment,
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
pub use visual_spiral::{
    generate_visual_spiral_1500, visual_spiral_1500_params, VISUAL_SPIRAL_1500_LATTICE_EDGE,
    VISUAL_SPIRAL_1500_SEED, VISUAL_SPIRAL_1500_STARS,
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
    pub seed: u64,
    pub scenario: ScenarioText,
    pub lattice: SquareLattice,
    pub core_mask: CoreMask,
    pub placement: ShapePlacement,
    /// Base bounded hyperlane topology edges only (not bridges or special routes).
    pub base_hyperlane_edges: Vec<HyperlaneEdge>,
    /// All emitted `add_hyperlane` pairs including bridges and special routes.
    pub hyperlane_edges: Vec<HyperlaneEdge>,
    /// Producer-side edge classification for preview/report only.
    pub classified_edges: Vec<ClassifiedCouplingEdge>,
    pub nebulas: Vec<NebulaField>,
    /// Connectivity proof when `ensure_connected` ran: `components_after == 1` means one interconnected
    /// galaxy (no island clusters). `None` when connectivity was not requested.
    pub connectivity: Option<ConnectivityReport>,
    /// Base hyperlane edges selected by the bounded topology pass **before** connectivity repair.
    pub pre_connectivity_topology_count: u32,
    /// Topology target after clamping `num_hyperlanes_default` to `[min, max]`.
    pub effective_target_hyperlanes: u32,
}

impl GalaxyGenerationResult {
    pub fn preview_scene(&self) -> GalaxyPreviewScene {
        self.preview_scene_with_options(GalaxyPreviewOptions {
            seed: self.seed,
            ..GalaxyPreviewOptions::default()
        })
    }

    pub fn preview_scene_with_options(&self, options: GalaxyPreviewOptions) -> GalaxyPreviewScene {
        GalaxyPreviewScene {
            seed: self.seed,
            options,
            lattice: self.lattice.clone(),
            core_mask: self.core_mask.clone(),
            placement: self.placement.clone(),
            base_hyperlane_edges: self.base_hyperlane_edges.clone(),
            classified_edges: self.classified_edges.clone(),
            nebulas: self.nebulas.clone(),
        }
    }

    pub fn render_preview_png(&self) -> Result<Vec<u8>, PreviewPngError> {
        render_galaxy_preview_png_bytes(&self.preview_scene())
    }

    pub fn render_preview_png_with_options(
        &self,
        options: GalaxyPreviewOptions,
    ) -> Result<Vec<u8>, PreviewPngError> {
        render_galaxy_preview_png_bytes(&self.preview_scene_with_options(options))
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
    let mut base_hyperlane_edges = Vec::new();
    let mut hyperlane_edges = Vec::new();
    let mut classified_edges = Vec::new();
    let mut connectivity = None;
    let mut pre_connectivity_topology_count = 0u32;
    let mut effective_target_hyperlanes = 0u32;
    if let Some(options) = hyperlane_options {
        effective_target_hyperlanes = options.target_edge_count;
        let (topology, _report) = generate_hyperlane_topology(&placement, &options, &mut rng)?;
        pre_connectivity_topology_count = topology.edges.len() as u32;
        base_hyperlane_edges = topology.edges.clone();
        classified_edges.extend(topology.edges.into_iter().map(|edge| {
            ClassifiedCouplingEdge::new(edge.clone(), CouplingEdgeKind::BaseHyperlane)
        }));
        // Guarantee ONE interconnected galaxy (no island clusters) by adding minimal short bridges over
        // authored coordinates. Connectivity is part of the BASE hyperlane network (as in Stellaris).
        if params.hyperlane.ensure_connected {
            let existing: Vec<(String, String)> = base_hyperlane_edges
                .iter()
                .map(|edge| (edge.from.clone(), edge.to.clone()))
                .collect();
            let (bridges, report) = connect_components(&placement, &existing);
            for edge in bridges {
                classified_edges.push(ClassifiedCouplingEdge::new(
                    edge.clone(),
                    CouplingEdgeKind::BaseHyperlane,
                ));
                base_hyperlane_edges.push(edge);
            }
            connectivity = Some(report);
        }
        hyperlane_edges.extend(base_hyperlane_edges.iter().cloned());
    }
    if let Some(options) = special_route_options {
        let existing: Vec<(String, String)> = hyperlane_edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone()))
            .collect();
        let (topology, _report) =
            generate_special_routes(&placement, &options, &existing, &mut rng)?;
        for edge in topology.edges {
            let hyperlane = edge.to_hyperlane_edge();
            classified_edges.push(ClassifiedCouplingEdge::new(
                hyperlane.clone(),
                CouplingEdgeKind::SpecialRouteCoupling,
            ));
            hyperlane_edges.push(hyperlane);
        }
    }
    if let Some(ref options) = partition_options {
        let existing: Vec<(String, String)> = hyperlane_edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone()))
            .collect();
        let (assignments, _report) = assign_partitions(&placement, options)?;
        let (bridges, _report) =
            generate_partition_bridges(&placement, &assignments, options, &existing, &mut rng)?;
        for edge in bridges {
            let hyperlane = edge.to_hyperlane_edge();
            classified_edges.push(ClassifiedCouplingEdge::new(
                hyperlane.clone(),
                CouplingEdgeKind::PartitionBridgeCoupling,
            ));
            hyperlane_edges.push(hyperlane);
        }
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
        for edge in bridges {
            let hyperlane = edge.to_hyperlane_edge();
            classified_edges.push(ClassifiedCouplingEdge::new(
                hyperlane.clone(),
                CouplingEdgeKind::ClusterBridgeCoupling,
            ));
            hyperlane_edges.push(hyperlane);
        }
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
        seed: params.seed,
        scenario,
        lattice,
        core_mask,
        placement,
        base_hyperlane_edges,
        hyperlane_edges,
        classified_edges,
        nebulas,
        connectivity,
        pre_connectivity_topology_count,
        effective_target_hyperlanes,
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
