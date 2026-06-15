//! Data-driven shape strategy registry: single-source descriptors + executable dispatch (PR8).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lattice::{CoreMask, SquareLattice};
use crate::occupancy::OccupancyGrid;
use crate::params::MapGeneratorParams;
use crate::rng::MapGenRng;
use crate::strategies;
use crate::strategy::{ShapePlacement, ShapePlacementError, ShapeStrategy, ShapeStrategyContext};

/// Advertised shape name resolved through the registry (not a fixed enum of strategies).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RegisteredShapeName(pub String);

impl RegisteredShapeName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One declarative parameter a shape strategy may advertise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeParameterDescriptor {
    pub key: String,
    pub description: String,
    pub required: bool,
}

/// Descriptor for a registered shape strategy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeStrategyDescriptor {
    pub name: RegisteredShapeName,
    pub display_name: String,
    pub description: String,
    pub parameters: Vec<ShapeParameterDescriptor>,
    /// Whether PR8 provides an executable placement strategy for this entry.
    pub executable: bool,
    /// Whether placement requires explicit in-memory cells (static / arbitrary_static).
    pub requires_explicit_cells: bool,
}

impl ShapeStrategyDescriptor {
    pub fn allowed_keys(&self) -> impl Iterator<Item = &str> {
        self.parameters.iter().map(|p| p.key.as_str())
    }

    pub fn allows_key(&self, key: &str) -> bool {
        self.parameters.iter().any(|p| p.key == key)
    }
}

/// One registry row: descriptor metadata and optional executable strategy.
#[derive(Clone)]
pub struct ShapeStrategyEntry {
    pub descriptor: ShapeStrategyDescriptor,
    strategy: Option<&'static dyn ShapeStrategy>,
}

impl std::fmt::Debug for ShapeStrategyEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShapeStrategyEntry")
            .field("descriptor", &self.descriptor)
            .field("has_strategy", &self.strategy.is_some())
            .finish()
    }
}

impl ShapeStrategyEntry {
    pub fn new(
        descriptor: ShapeStrategyDescriptor,
        strategy: Option<&'static dyn ShapeStrategy>,
    ) -> Self {
        Self {
            descriptor,
            strategy,
        }
    }

    pub fn strategy(&self) -> Option<&'static dyn ShapeStrategy> {
        self.strategy
    }
}

/// Data-driven registry of shape strategy descriptors and executable dispatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeRegistry {
    entries: BTreeMap<String, ShapeStrategyEntry>,
}

#[derive(Debug, Error, PartialEq)]
pub enum RegistryResolveError {
    #[error("shape '{shape}' is not registered; registered shapes: {registered}")]
    UnknownShape { shape: String, registered: String },
    #[error(
        "shape '{shape}' is registered but has no executable strategy; executable shapes: {executable}"
    )]
    StrategyNotImplemented { shape: String, executable: String },
}

impl Default for ShapeRegistry {
    fn default() -> Self {
        Self::vanilla()
    }
}

impl ShapeRegistry {
    pub fn vanilla() -> Self {
        Self {
            entries: strategies::vanilla_entries(),
        }
    }

    /// Back-compat alias for PR1 tests/docs.
    pub fn vanilla_pr1() -> Self {
        Self::vanilla()
    }

    pub fn from_entries(entries: BTreeMap<String, ShapeStrategyEntry>) -> Self {
        Self { entries }
    }

    pub fn get(&self, name: &str) -> Option<&ShapeStrategyDescriptor> {
        self.entries.get(name).map(|e| &e.descriptor)
    }

    pub fn get_entry(&self, name: &str) -> Option<&ShapeStrategyEntry> {
        self.entries.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    pub fn registered_names(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(String::as_str)
    }

    pub fn registered_names_sorted(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }

    pub fn descriptors(&self) -> impl Iterator<Item = &ShapeStrategyDescriptor> {
        self.entries.values().map(|e| &e.descriptor)
    }

    /// Names with executable strategy implementations (derived from registry entries).
    pub fn executable_names_sorted(&self) -> Vec<String> {
        self.entries
            .values()
            .filter(|e| e.strategy.is_some())
            .map(|e| e.descriptor.name.0.clone())
            .collect()
    }

    pub fn is_executable(&self, name: &str) -> bool {
        self.entries.get(name).is_some_and(|e| e.strategy.is_some())
    }

    /// Resolve an executable strategy by name (registry-based lookup).
    pub fn resolve_strategy(
        &self,
        name: &str,
    ) -> Result<&'static dyn ShapeStrategy, RegistryResolveError> {
        let entry = self
            .entries
            .get(name)
            .ok_or_else(|| RegistryResolveError::UnknownShape {
                shape: name.to_string(),
                registered: self.registered_names_sorted().join(", "),
            })?;
        entry
            .strategy
            .ok_or_else(|| RegistryResolveError::StrategyNotImplemented {
                shape: name.to_string(),
                executable: self.executable_names_sorted().join(", "),
            })
    }

    /// Run a registered executable strategy and return in-memory placements.
    pub fn place(
        &self,
        params: &MapGeneratorParams,
        lattice: &SquareLattice,
        core_mask: &CoreMask,
        occupancy: &mut OccupancyGrid,
        rng: &mut MapGenRng,
        explicit_cells: Option<&[crate::lattice::LatticeCoord]>,
    ) -> Result<ShapePlacement, ShapePlacementError> {
        let shape = params.shape.shape.as_str();
        let strategy = self.resolve_strategy(shape).map_err(|err| match err {
            RegistryResolveError::UnknownShape { shape, registered } => {
                ShapePlacementError::UnknownShape { shape, registered }
            }
            RegistryResolveError::StrategyNotImplemented { shape, executable } => {
                ShapePlacementError::StrategyNotImplemented { shape, executable }
            }
        })?;
        let descriptor = self
            .get(shape)
            .expect("resolve_strategy implies descriptor exists");
        let mut ctx = ShapeStrategyContext {
            params,
            descriptor,
            lattice,
            core_mask,
            occupancy,
            rng,
            explicit_cells,
        };
        strategy.place(&mut ctx)
    }
}

// Custom serde: registry serializes descriptors only (strategies are not serializable).
impl Serialize for ShapeStrategyEntry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.descriptor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShapeStrategyEntry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let descriptor = ShapeStrategyDescriptor::deserialize(deserializer)?;
        Ok(Self {
            descriptor,
            strategy: None,
        })
    }
}
