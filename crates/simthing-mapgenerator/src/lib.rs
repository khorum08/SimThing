//! Standalone MapGeneratorCLI producer library (0.0.8.6).
//!
//! PR1: parameter model, shape registry descriptor shell, and validation only.
//! PR2: deterministic RNG, square lattice, core mask, occupancy primitives.

pub mod lattice;
pub mod occupancy;
pub mod params;
pub mod rng;
pub mod shape_registry;

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
    RegisteredShapeName, ShapeParameterDescriptor, ShapeRegistry, ShapeStrategyDescriptor,
};

/// Validate params against the default vanilla PR1 registry.
pub fn validate_default(params: &MapGeneratorParams) -> Result<(), ValidationError> {
    params.validate(&ShapeRegistry::default())
}
