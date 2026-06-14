//! Data-driven shape strategy registry (descriptor shell only — no placement algorithms in PR1).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

/// Descriptor for a registered shape strategy (algorithm deferred to later PRs).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeStrategyDescriptor {
    pub name: RegisteredShapeName,
    pub display_name: String,
    pub description: String,
    pub parameters: Vec<ShapeParameterDescriptor>,
}

impl ShapeStrategyDescriptor {
    pub fn allowed_keys(&self) -> impl Iterator<Item = &str> {
        self.parameters.iter().map(|p| p.key.as_str())
    }

    pub fn allows_key(&self, key: &str) -> bool {
        self.parameters.iter().any(|p| p.key == key)
    }
}

/// Data-driven registry of shape strategy descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeRegistry {
    strategies: BTreeMap<String, ShapeStrategyDescriptor>,
}

impl Default for ShapeRegistry {
    fn default() -> Self {
        Self::vanilla_pr1()
    }
}

impl ShapeRegistry {
    pub fn vanilla_pr1() -> Self {
        let mut strategies = BTreeMap::new();
        for descriptor in [
            Self::arbitrary_static_descriptor(),
            Self::elliptical_descriptor(),
            Self::ring_descriptor(),
            Self::spiral_2_descriptor(),
            Self::spiral_4_descriptor(),
        ] {
            strategies.insert(descriptor.name.0.clone(), descriptor);
        }
        Self { strategies }
    }

    pub fn get(&self, name: &str) -> Option<&ShapeStrategyDescriptor> {
        self.strategies.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.strategies.contains_key(name)
    }

    pub fn registered_names(&self) -> impl Iterator<Item = &str> {
        self.strategies.keys().map(String::as_str)
    }

    pub fn registered_names_sorted(&self) -> Vec<String> {
        self.strategies.keys().cloned().collect()
    }

    pub fn descriptors(&self) -> impl Iterator<Item = &ShapeStrategyDescriptor> {
        self.strategies.values()
    }

    fn arbitrary_static_descriptor() -> ShapeStrategyDescriptor {
        ShapeStrategyDescriptor {
            name: RegisteredShapeName("arbitrary_static".into()),
            display_name: "Arbitrary / static".into(),
            description: "Explicit point-cloud + graph admission (static_galaxy_scenario form)."
                .into(),
            parameters: vec![param(
                "coordinate_transform",
                "Optional coordinate transform label",
                false,
            )],
        }
    }

    fn elliptical_descriptor() -> ShapeStrategyDescriptor {
        ShapeStrategyDescriptor {
            name: RegisteredShapeName("elliptical".into()),
            display_name: "Elliptical".into(),
            description: "Elliptical disc sampling over the square lattice.".into(),
            parameters: vec![param("jitter", "Placement jitter scale", false)],
        }
    }

    fn ring_descriptor() -> ShapeStrategyDescriptor {
        ShapeStrategyDescriptor {
            name: RegisteredShapeName("ring".into()),
            display_name: "Ring".into(),
            description: "Annular band with central void.".into(),
            parameters: vec![param("band_width", "Ring band width scale", false)],
        }
    }

    fn spiral_2_descriptor() -> ShapeStrategyDescriptor {
        ShapeStrategyDescriptor {
            name: RegisteredShapeName("spiral_2".into()),
            display_name: "Two-arm spiral".into(),
            description: "Two-arm spiral curve quantized to lattice cells.".into(),
            parameters: spiral_params(),
        }
    }

    fn spiral_4_descriptor() -> ShapeStrategyDescriptor {
        ShapeStrategyDescriptor {
            name: RegisteredShapeName("spiral_4".into()),
            display_name: "Four-arm spiral".into(),
            description: "Four-arm spiral curve quantized to lattice cells.".into(),
            parameters: spiral_params(),
        }
    }
}

fn param(key: &str, description: &str, required: bool) -> ShapeParameterDescriptor {
    ShapeParameterDescriptor {
        key: key.into(),
        description: description.into(),
        required,
    }
}

fn spiral_params() -> Vec<ShapeParameterDescriptor> {
    vec![
        param(
            "num_arms",
            "Arm count override (must match registered name when set)",
            false,
        ),
        param("arm_tightness", "Arm tightness scale", false),
        param("arm_width", "Perpendicular arm width", false),
        param("jitter", "Placement jitter scale", false),
    ]
}
