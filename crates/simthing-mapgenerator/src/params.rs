//! Full §3A lever parameter surface (validation only in PR1).

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::shape_registry::ShapeRegistry;

pub const MAX_BOUNDED_COUNT: u32 = 10_000;
pub const MAX_ODDS: f64 = 1.0;
pub const MAX_STRENGTH: f64 = 10.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationMode {
    Procedural,
    ArbitraryStatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClusterCountMethod {
    OneEveryXEmpire,
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartitionMethod {
    BreadthFirst,
    DepthFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Clause,
    StaticGalaxy,
    Manifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArbitraryHyperlaneSourceMode {
    #[default]
    AddHyperlane,
    PreventHyperlane,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleCoreParams {
    pub num_stars: u32,
    pub radius: f64,
    pub core_radius: f64,
    pub lattice_size: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeParams {
    pub shape: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub shape_params: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClusteringParams {
    pub cluster_count: Option<u32>,
    pub cluster_count_method: ClusterCountMethod,
    pub cluster_count_value: f64,
    pub cluster_count_max: Option<u32>,
    pub cluster_radius: f64,
    pub cluster_distance_from_core: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartitioningParams {
    pub home_system_partitions: u32,
    pub open_space_partitions: u32,
    pub partition_min_systems: u32,
    pub partition_max_systems: u32,
    pub partition_min_bridges: u32,
    pub partition_max_bridges: u32,
    pub partition_method: PartitionMethod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperlaneGeometryParams {
    pub max_hyperlane_distance: f64,
    pub num_hyperlanes_min: u32,
    pub num_hyperlanes_max: u32,
    pub num_hyperlanes_default: u32,
    pub random_hyperlanes: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialRouteParams {
    pub num_wormhole_pairs: u32,
    pub num_gateways: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NebulaFieldParams {
    pub num_nebulas: u32,
    pub nebula_size: f64,
    pub nebula_min_dist: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializerBucketParams {
    pub initializer_bucket_core: String,
    pub initializer_bucket_arm: String,
    pub initializer_bucket_fringe: String,
    pub initializer_bucket_cluster: String,
    pub spawn_weight: f64,
    pub spawn_design: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InertMetadataParams {
    pub num_empires: u32,
    pub fallen_empire_count: u32,
    pub marauder_empire_count: u32,
    pub advanced_empire_count: u32,
    pub colonizable_planet_odds: f64,
    pub primitive_odds: f64,
    pub crisis_strength: f64,
    pub extra_crisis_strength: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArbitraryStaticParams {
    pub explicit_point_cloud_path: Option<PathBuf>,
    pub explicit_graph_path: Option<PathBuf>,
    pub coordinate_transform: Option<String>,
    pub hyperlane_source_mode: ArbitraryHyperlaneSourceMode,
}

impl Default for ArbitraryStaticParams {
    fn default() -> Self {
        Self {
            explicit_point_cloud_path: None,
            explicit_graph_path: None,
            coordinate_transform: None,
            hyperlane_source_mode: ArbitraryHyperlaneSourceMode::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputParams {
    pub output_format: OutputFormat,
    pub output: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapGeneratorParams {
    pub mode: GenerationMode,
    pub scale_core: ScaleCoreParams,
    pub shape: ShapeParams,
    pub clustering: ClusteringParams,
    pub partitioning: PartitioningParams,
    pub hyperlane: HyperlaneGeometryParams,
    pub special_routes: SpecialRouteParams,
    pub nebula: NebulaFieldParams,
    pub initializers: InitializerBucketParams,
    pub metadata: InertMetadataParams,
    pub arbitrary: ArbitraryStaticParams,
    pub output: OutputParams,
    pub seed: u64,
    pub variation_seed: Option<u64>,
}

impl Default for MapGeneratorParams {
    fn default() -> Self {
        Self {
            mode: GenerationMode::Procedural,
            scale_core: ScaleCoreParams {
                num_stars: 100,
                radius: 450.0,
                core_radius: 120.0,
                lattice_size: Some(200),
            },
            shape: ShapeParams {
                shape: "elliptical".into(),
                shape_params: BTreeMap::new(),
            },
            clustering: ClusteringParams {
                cluster_count: Some(6),
                cluster_count_method: ClusterCountMethod::Constant,
                cluster_count_value: 6.0,
                cluster_count_max: Some(12),
                cluster_radius: 80.0,
                cluster_distance_from_core: 40.0,
            },
            partitioning: PartitioningParams {
                home_system_partitions: 4,
                open_space_partitions: 2,
                partition_min_systems: 5,
                partition_max_systems: 25,
                partition_min_bridges: 1,
                partition_max_bridges: 3,
                partition_method: PartitionMethod::BreadthFirst,
            },
            hyperlane: HyperlaneGeometryParams {
                max_hyperlane_distance: 4.0,
                num_hyperlanes_min: 1,
                num_hyperlanes_max: 3,
                num_hyperlanes_default: 2,
                random_hyperlanes: true,
            },
            special_routes: SpecialRouteParams {
                num_wormhole_pairs: 0,
                num_gateways: 0,
            },
            nebula: NebulaFieldParams {
                num_nebulas: 1,
                nebula_size: 35.0,
                nebula_min_dist: 20.0,
            },
            initializers: InitializerBucketParams {
                initializer_bucket_core: "example_rim_initializer".into(),
                initializer_bucket_arm: "example_rim_initializer".into(),
                initializer_bucket_fringe: "example_rim_initializer".into(),
                initializer_bucket_cluster: "example_rim_initializer".into(),
                spawn_weight: 1.0,
                spawn_design: None,
            },
            metadata: InertMetadataParams {
                num_empires: 6,
                fallen_empire_count: 0,
                marauder_empire_count: 0,
                advanced_empire_count: 0,
                colonizable_planet_odds: 0.5,
                primitive_odds: 0.1,
                crisis_strength: 1.0,
                extra_crisis_strength: 0.0,
            },
            arbitrary: ArbitraryStaticParams::default(),
            output: OutputParams {
                output_format: OutputFormat::Clause,
                output: None,
                output_dir: Some(PathBuf::from(".")),
                dry_run: false,
            },
            seed: 42,
            variation_seed: None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("{field}: must be > 0")]
    MustBePositive { field: &'static str },
    #[error("{field}: must be finite and > 0")]
    MustBeFinitePositive { field: &'static str },
    #[error("{field}: must be finite and >= 0")]
    MustBeFiniteNonNegative { field: &'static str },
    #[error("shape '{shape}' is not registered; registered shapes: {registered}")]
    UnknownShape { shape: String, registered: String },
    #[error("shape param '{key}' is not declared for shape '{shape}'")]
    UnknownShapeParam { shape: String, key: String },
    #[error("required shape param '{key}' missing for shape '{shape}'")]
    MissingRequiredShapeParam { shape: String, key: String },
    #[error("{field}: min must be <= max")]
    MinGreaterThanMax { field: &'static str },
    #[error("{field}: value {value} exceeds bound {max}")]
    ExceedsBound {
        field: &'static str,
        value: String,
        max: String,
    },
    #[error("initializer ref '{key}': invalid syntax")]
    InvalidInitializerRef { key: String },
    #[error("arbitrary_static mode requires at least one of explicit_point_cloud_path or explicit_graph_path")]
    ArbitraryPathsMissing,
    #[error("procedural mode cannot use shape 'arbitrary_static'; use mode=arbitrary_static")]
    ArbitraryShapeInProceduralMode,
}

impl MapGeneratorParams {
    pub fn from_json_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn validate(&self, registry: &ShapeRegistry) -> Result<(), ValidationError> {
        self.validate_scale_core()?;
        self.validate_shape(registry)?;
        self.validate_clustering()?;
        self.validate_partitioning()?;
        self.validate_hyperlane()?;
        self.validate_special_routes()?;
        self.validate_nebula()?;
        self.validate_initializers()?;
        self.validate_metadata()?;
        self.validate_mode()?;
        Ok(())
    }

    pub fn dry_run_summary(&self) -> String {
        format!(
            "MapGeneratorCLI dry-run (validation-only PR1)\n\
             mode={:?}\n\
             num_stars={}\n\
             shape={}\n\
             lattice_size={:?}\n\
             seed={}\n\
             output_format={:?}\n\
             metadata_passthrough=inert (num_empires={}, crisis_strength={})\n\
             NOTE: no placement, topology, emission, or lowering in PR1.",
            self.mode,
            self.scale_core.num_stars,
            self.shape.shape,
            self.scale_core.lattice_size,
            self.seed,
            self.output.output_format,
            self.metadata.num_empires,
            self.metadata.crisis_strength,
        )
    }

    fn validate_scale_core(&self) -> Result<(), ValidationError> {
        if self.scale_core.num_stars == 0 {
            return Err(ValidationError::MustBePositive { field: "num_stars" });
        }
        require_finite_positive(self.scale_core.radius, "radius")?;
        require_finite_non_negative(self.scale_core.core_radius, "core_radius")?;
        if let Some(size) = self.scale_core.lattice_size {
            if size == 0 {
                return Err(ValidationError::MustBePositive {
                    field: "lattice_size",
                });
            }
        }
        Ok(())
    }

    fn validate_shape(&self, registry: &ShapeRegistry) -> Result<(), ValidationError> {
        let descriptor =
            registry
                .get(&self.shape.shape)
                .ok_or_else(|| ValidationError::UnknownShape {
                    shape: self.shape.shape.clone(),
                    registered: registry.registered_names_sorted().join(", "),
                })?;
        for key in self.shape.shape_params.keys() {
            if !descriptor.allows_key(key) {
                return Err(ValidationError::UnknownShapeParam {
                    shape: self.shape.shape.clone(),
                    key: key.clone(),
                });
            }
        }
        for param in &descriptor.parameters {
            if param.required && !self.shape.shape_params.contains_key(&param.key) {
                return Err(ValidationError::MissingRequiredShapeParam {
                    shape: self.shape.shape.clone(),
                    key: param.key.clone(),
                });
            }
        }
        if self.mode == GenerationMode::Procedural && self.shape.shape == "arbitrary_static" {
            return Err(ValidationError::ArbitraryShapeInProceduralMode);
        }
        Ok(())
    }

    fn validate_clustering(&self) -> Result<(), ValidationError> {
        if let Some(count) = self.clustering.cluster_count {
            bounded_count(count, "cluster_count")?;
        }
        require_finite_non_negative(self.clustering.cluster_count_value, "cluster_count_value")?;
        if let Some(max) = self.clustering.cluster_count_max {
            bounded_count(max, "cluster_count_max")?;
        }
        require_finite_non_negative(self.clustering.cluster_radius, "cluster_radius")?;
        require_finite_non_negative(
            self.clustering.cluster_distance_from_core,
            "cluster_distance_from_core",
        )?;
        Ok(())
    }

    fn validate_partitioning(&self) -> Result<(), ValidationError> {
        bounded_count(
            self.partitioning.home_system_partitions,
            "home_system_partitions",
        )?;
        bounded_count(
            self.partitioning.open_space_partitions,
            "open_space_partitions",
        )?;
        if self.partitioning.partition_min_systems > self.partitioning.partition_max_systems {
            return Err(ValidationError::MinGreaterThanMax {
                field: "partition_systems",
            });
        }
        if self.partitioning.partition_min_bridges > self.partitioning.partition_max_bridges {
            return Err(ValidationError::MinGreaterThanMax {
                field: "partition_bridges",
            });
        }
        Ok(())
    }

    fn validate_hyperlane(&self) -> Result<(), ValidationError> {
        require_finite_positive(
            self.hyperlane.max_hyperlane_distance,
            "max_hyperlane_distance",
        )?;
        bounded_count(self.hyperlane.num_hyperlanes_min, "num_hyperlanes_min")?;
        bounded_count(self.hyperlane.num_hyperlanes_max, "num_hyperlanes_max")?;
        bounded_count(
            self.hyperlane.num_hyperlanes_default,
            "num_hyperlanes_default",
        )?;
        if self.hyperlane.num_hyperlanes_min > self.hyperlane.num_hyperlanes_max {
            return Err(ValidationError::MinGreaterThanMax {
                field: "num_hyperlanes",
            });
        }
        Ok(())
    }

    fn validate_special_routes(&self) -> Result<(), ValidationError> {
        bounded_count(self.special_routes.num_wormhole_pairs, "num_wormhole_pairs")?;
        bounded_count(self.special_routes.num_gateways, "num_gateways")?;
        Ok(())
    }

    fn validate_nebula(&self) -> Result<(), ValidationError> {
        bounded_count(self.nebula.num_nebulas, "num_nebulas")?;
        require_finite_non_negative(self.nebula.nebula_size, "nebula_size")?;
        require_finite_non_negative(self.nebula.nebula_min_dist, "nebula_min_dist")?;
        Ok(())
    }

    fn validate_initializers(&self) -> Result<(), ValidationError> {
        for key in [
            &self.initializers.initializer_bucket_core,
            &self.initializers.initializer_bucket_arm,
            &self.initializers.initializer_bucket_fringe,
            &self.initializers.initializer_bucket_cluster,
        ] {
            validate_initializer_ref(key)?;
        }
        require_finite_non_negative(self.initializers.spawn_weight, "spawn_weight")?;
        Ok(())
    }

    fn validate_metadata(&self) -> Result<(), ValidationError> {
        bounded_count(self.metadata.num_empires, "num_empires")?;
        bounded_count(self.metadata.fallen_empire_count, "fallen_empire_count")?;
        bounded_count(self.metadata.marauder_empire_count, "marauder_empire_count")?;
        bounded_count(self.metadata.advanced_empire_count, "advanced_empire_count")?;
        bounded_odds(
            self.metadata.colonizable_planet_odds,
            "colonizable_planet_odds",
        )?;
        bounded_odds(self.metadata.primitive_odds, "primitive_odds")?;
        bounded_strength(self.metadata.crisis_strength, "crisis_strength")?;
        bounded_strength(self.metadata.extra_crisis_strength, "extra_crisis_strength")?;
        Ok(())
    }

    fn validate_mode(&self) -> Result<(), ValidationError> {
        if self.mode == GenerationMode::ArbitraryStatic {
            if self.arbitrary.explicit_point_cloud_path.is_none()
                && self.arbitrary.explicit_graph_path.is_none()
            {
                return Err(ValidationError::ArbitraryPathsMissing);
            }
            // PR1 shell: a params file may set `shape` independently of `mode`; mode drives
            // validation messaging and shape is not constrained to `arbitrary_static` here.
        }
        Ok(())
    }
}

pub fn validate_initializer_ref(key: &str) -> Result<(), ValidationError> {
    if key.is_empty() {
        return Err(ValidationError::InvalidInitializerRef { key: key.into() });
    }
    let valid = key
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if !valid || key.starts_with('.') {
        return Err(ValidationError::InvalidInitializerRef { key: key.into() });
    }
    Ok(())
}

fn require_finite_positive(value: f64, field: &'static str) -> Result<(), ValidationError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(ValidationError::MustBeFinitePositive { field });
    }
    Ok(())
}

fn require_finite_non_negative(value: f64, field: &'static str) -> Result<(), ValidationError> {
    if !value.is_finite() || value < 0.0 {
        return Err(ValidationError::MustBeFiniteNonNegative { field });
    }
    Ok(())
}

fn bounded_count(value: u32, field: &'static str) -> Result<(), ValidationError> {
    if value > MAX_BOUNDED_COUNT {
        return Err(ValidationError::ExceedsBound {
            field,
            value: value.to_string(),
            max: MAX_BOUNDED_COUNT.to_string(),
        });
    }
    Ok(())
}

fn bounded_odds(value: f64, field: &'static str) -> Result<(), ValidationError> {
    if !value.is_finite() || value < 0.0 || value > MAX_ODDS {
        return Err(ValidationError::ExceedsBound {
            field,
            value: value.to_string(),
            max: MAX_ODDS.to_string(),
        });
    }
    Ok(())
}

fn bounded_strength(value: f64, field: &'static str) -> Result<(), ValidationError> {
    if !value.is_finite() || value < 0.0 || value > MAX_STRENGTH {
        return Err(ValidationError::ExceedsBound {
            field,
            value: value.to_string(),
            max: MAX_STRENGTH.to_string(),
        });
    }
    Ok(())
}
