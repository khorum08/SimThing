//! Standalone MapGeneratorCLI producer library (0.0.8.6).
//!
//! PR1: parameter model, shape registry descriptor shell, and validation only.

pub mod params;
pub mod shape_registry;

pub use params::{
    ArbitraryHyperlaneSourceMode, ArbitraryStaticParams, ClusterCountMethod, ClusteringParams,
    GenerationMode, HyperlaneGeometryParams, InertMetadataParams, InitializerBucketParams,
    MapGeneratorParams, NebulaFieldParams, OutputFormat, OutputParams, PartitionMethod,
    PartitioningParams, ScaleCoreParams, ShapeParams, SpecialRouteParams, ValidationError,
};
pub use shape_registry::{
    RegisteredShapeName, ShapeParameterDescriptor, ShapeRegistry, ShapeStrategyDescriptor,
};

/// Validate params against the default vanilla PR1 registry.
pub fn validate_default(params: &MapGeneratorParams) -> Result<(), ValidationError> {
    params.validate(&ShapeRegistry::default())
}
