//! Shape-parameter validation surface for editor-facing fail-closed consumption.

use std::collections::BTreeMap;

use thiserror::Error;

use crate::params::{MapGeneratorParams, ValidationError};
use crate::shape_registry::ShapeRegistry;

/// Declarative bounds for one shape tuning parameter (producer-side only).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeParamSpec {
    pub key: &'static str,
    pub required: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default: Option<f64>,
    pub description: &'static str,
}

#[derive(Debug, Error, PartialEq)]
pub enum ShapeParamParseError {
    #[error("Invalid shape param '{raw}': expected KEY=VALUE with numeric VALUE")]
    InvalidFormat { raw: String },
    #[error("Invalid shape param '{key}': expected KEY=VALUE with numeric VALUE")]
    MissingValue { key: String },
    #[error("Invalid shape param '{key}': expected KEY=VALUE with numeric VALUE")]
    NonNumeric { key: String, value: String },
}

/// Parse one `--shape-param KEY=VALUE` assignment (fail-closed).
pub fn parse_shape_param_assignment(raw: &str) -> Result<(String, f64), ShapeParamParseError> {
    let trimmed = raw.trim();
    let Some((key, value)) = trimmed.split_once('=') else {
        return Err(ShapeParamParseError::InvalidFormat {
            raw: trimmed.to_string(),
        });
    };
    let key = key.trim();
    if key.is_empty() {
        return Err(ShapeParamParseError::InvalidFormat {
            raw: trimmed.to_string(),
        });
    }
    let value = value.trim();
    if value.is_empty() {
        return Err(ShapeParamParseError::MissingValue {
            key: key.to_string(),
        });
    }
    if value.contains('=') {
        return Err(ShapeParamParseError::InvalidFormat {
            raw: trimmed.to_string(),
        });
    }
    let parsed = value
        .parse::<f64>()
        .map_err(|_| ShapeParamParseError::NonNumeric {
            key: key.to_string(),
            value: value.to_string(),
        })?;
    if !parsed.is_finite() {
        return Err(ShapeParamParseError::NonNumeric {
            key: key.to_string(),
            value: value.to_string(),
        });
    }
    Ok((key.to_string(), parsed))
}

/// Apply CLI `--shape-param` assignments into params (fail-closed; no silent ignore).
pub fn apply_cli_shape_params(
    shape: &mut crate::params::ShapeParams,
    assignments: &[String],
) -> Result<(), ShapeParamParseError> {
    for raw in assignments {
        let (key, value) = parse_shape_param_assignment(raw)?;
        shape.shape_params.insert(key, value);
    }
    Ok(())
}

/// Lookup table for shapes with explicit numeric bounds (extends registry descriptors).
pub fn shape_param_specs(shape: &str) -> &'static [ShapeParamSpec] {
    match shape {
        "spiral_2" | "spiral_3" | "spiral_4" | "spiral_6" => &SPIRAL_SPECS,
        "elliptical" => &ELLIPTICAL_SPECS,
        "ring" => &RING_SPECS,
        "bar" => &BAR_SPECS,
        "starburst" | "cartwheel" | "spoked" => &SPOKE_SPECS,
        "static" | "arbitrary_static" => &STATIC_SPECS,
        _ => &[],
    }
}

pub fn spec_for_key(shape: &str, key: &str) -> Option<&'static ShapeParamSpec> {
    shape_param_specs(shape).iter().find(|spec| spec.key == key)
}

/// Validate shape params against registry + numeric bounds.
pub fn validate_shape_params(
    shape: &str,
    shape_params: &BTreeMap<String, f64>,
    registry: &ShapeRegistry,
) -> Result<(), ValidationError> {
    let descriptor = registry
        .get(shape)
        .ok_or_else(|| ValidationError::UnknownShape {
            shape: shape.to_string(),
            registered: registry.registered_names_sorted().join(", "),
        })?;
    for (key, value) in shape_params {
        if !descriptor.allows_key(key) {
            if key_declared_for_other_shape(shape, key) {
                return Err(ValidationError::ShapeParamNotValidForShape {
                    shape: shape.to_string(),
                    key: key.clone(),
                });
            }
            return Err(ValidationError::UnknownShapeParam {
                shape: shape.to_string(),
                key: key.clone(),
            });
        }
        if let Some(spec) = spec_for_key(shape, key) {
            if let Some(min) = spec.min {
                if *value < min {
                    return Err(ValidationError::ShapeParamOutOfRange {
                        shape: shape.to_string(),
                        key: key.clone(),
                        value: *value,
                        min: Some(min),
                        max: spec.max,
                    });
                }
            }
            if let Some(max) = spec.max {
                if *value > max {
                    return Err(ValidationError::ShapeParamOutOfRange {
                        shape: shape.to_string(),
                        key: key.clone(),
                        value: *value,
                        min: spec.min,
                        max: Some(max),
                    });
                }
            }
        }
        if !value.is_finite() {
            return Err(ValidationError::ShapeParamNonNumeric {
                key: key.clone(),
                value: value.to_string(),
            });
        }
    }
    for param in &descriptor.parameters {
        if param.required && !shape_params.contains_key(&param.key) {
            return Err(ValidationError::MissingRequiredShapeParam {
                shape: shape.to_string(),
                key: param.key.clone(),
            });
        }
    }
    Ok(())
}

pub fn validate_shape_params_for_params(
    params: &MapGeneratorParams,
    registry: &ShapeRegistry,
) -> Result<(), ValidationError> {
    validate_shape_params(&params.shape.shape, &params.shape.shape_params, registry)
}

const SPIRAL_SPECS: [ShapeParamSpec; 5] = [
    ShapeParamSpec {
        key: "arm_width",
        required: false,
        min: Some(0.0),
        max: Some(100.0),
        default: Some(1.0),
        description: "Perpendicular arm width scale",
    },
    ShapeParamSpec {
        key: "arm_tightness",
        required: false,
        min: Some(0.01),
        max: Some(10.0),
        default: Some(1.0),
        description: "Arm tightness scale",
    },
    ShapeParamSpec {
        key: "jitter",
        required: false,
        min: Some(0.0),
        max: Some(50.0),
        default: Some(0.0),
        description: "Placement jitter scale",
    },
    ShapeParamSpec {
        key: "core_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Core void radius scale (producer-side)",
    },
    ShapeParamSpec {
        key: "num_arms",
        required: false,
        min: Some(1.0),
        max: Some(12.0),
        default: None,
        description: "Arm count override (must match registered name when set)",
    },
];

const ELLIPTICAL_SPECS: [ShapeParamSpec; 2] = [
    ShapeParamSpec {
        key: "core_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Core void radius scale (producer-side)",
    },
    ShapeParamSpec {
        key: "jitter",
        required: false,
        min: Some(0.0),
        max: Some(50.0),
        default: Some(0.0),
        description: "Placement jitter scale",
    },
];

const RING_SPECS: [ShapeParamSpec; 5] = [
    ShapeParamSpec {
        key: "ring_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Ring band center radius",
    },
    ShapeParamSpec {
        key: "arm_width",
        required: false,
        min: Some(0.0),
        max: Some(100.0),
        default: Some(2.0),
        description: "Ring band width scale",
    },
    ShapeParamSpec {
        key: "band_width",
        required: false,
        min: Some(0.0),
        max: Some(100.0),
        default: None,
        description: "Legacy alias for ring band width",
    },
    ShapeParamSpec {
        key: "core_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Core void radius scale (producer-side)",
    },
    ShapeParamSpec {
        key: "jitter",
        required: false,
        min: Some(0.0),
        max: Some(50.0),
        default: Some(0.0),
        description: "Placement jitter scale",
    },
];

const BAR_SPECS: [ShapeParamSpec; 4] = [
    ShapeParamSpec {
        key: "bar_length",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Bar length scale",
    },
    ShapeParamSpec {
        key: "bar_width",
        required: false,
        min: Some(0.0),
        max: Some(100.0),
        default: Some(2.0),
        description: "Bar width scale",
    },
    ShapeParamSpec {
        key: "core_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Core void radius scale (producer-side)",
    },
    ShapeParamSpec {
        key: "jitter",
        required: false,
        min: Some(0.0),
        max: Some(50.0),
        default: Some(0.0),
        description: "Placement jitter scale",
    },
];

const SPOKE_SPECS: [ShapeParamSpec; 4] = [
    ShapeParamSpec {
        key: "num_arms",
        required: false,
        min: Some(1.0),
        max: Some(24.0),
        default: None,
        description: "Spoke count",
    },
    ShapeParamSpec {
        key: "core_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Core void radius scale (producer-side)",
    },
    ShapeParamSpec {
        key: "jitter",
        required: false,
        min: Some(0.0),
        max: Some(50.0),
        default: Some(0.0),
        description: "Placement jitter scale",
    },
    ShapeParamSpec {
        key: "ring_radius",
        required: false,
        min: Some(0.0),
        max: None,
        default: None,
        description: "Ring band center radius (cartwheel)",
    },
];

const STATIC_SPECS: [ShapeParamSpec; 1] = [ShapeParamSpec {
    key: "coordinate_transform",
    required: false,
    min: None,
    max: None,
    default: None,
    description: "Optional coordinate transform label",
}];

fn key_declared_for_other_shape(current: &str, key: &str) -> bool {
    [
        "spiral_2",
        "spiral_3",
        "spiral_4",
        "spiral_6",
        "elliptical",
        "ring",
        "bar",
        "starburst",
        "cartwheel",
        "spoked",
        "static",
        "arbitrary_static",
    ]
    .into_iter()
    .filter(|shape| *shape != current)
    .any(|shape| spec_for_key(shape, key).is_some())
}
