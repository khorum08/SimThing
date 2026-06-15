//! Inert metadata passthrough reporting (PR9).
//!
//! The closed `static_galaxy_scenario` neutral-AST reader does not admit gameplay metadata keys.
//! Values are captured for producer dry-run reports only until a closed surface accepts them.

use crate::params::{InertMetadataParams, MapGeneratorParams};

/// Producer-side metadata passthrough status.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataPassthroughReport {
    pub deferred: bool,
    pub reason: &'static str,
    pub captured: InertMetadataParams,
}

/// Capture inert metadata levers for dry-run reporting; emission is deferred in PR9.
pub fn metadata_passthrough_report(params: &MapGeneratorParams) -> MetadataPassthroughReport {
    MetadataPassthroughReport {
        deferred: true,
        reason: "static_galaxy_scenario closed surface accepts name/random_hyperlanes/system/add_hyperlane/nebula only; inert metadata keys are not admitted without lowerer widening",
        captured: params.metadata.clone(),
    }
}
