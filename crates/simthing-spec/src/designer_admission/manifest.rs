//! RON-serializable designer admission preflight manifest (L1-1).

use serde::{Deserialize, Serialize};

/// Shallow designer-authored preflight manifest — not a full FrontierV2 scenario.
///
/// Lets a designer or future ClauseThing-facing importer ask what guardrails a
/// requested posture would trip at admission, without compiling runtime artifacts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesignerAdmissionPreflightManifest {
    pub manifest_id: String,
    pub profile_name: String,
    #[serde(default)]
    pub enabled_by_default: bool,
    #[serde(default)]
    pub requested_artifact_targets: Vec<String>,
    #[serde(default)]
    pub requested_guardrail_overrides: Vec<String>,
    #[serde(default)]
    pub requested_runtime_features: Vec<String>,
    #[serde(default)]
    pub requested_mapping_features: Vec<String>,
    #[serde(default)]
    pub requested_resource_flow_features: Vec<String>,
    #[serde(default)]
    pub requested_authoring_frontend: Vec<String>,
    /// Optional unit ids for cross-entity movement write preflight checks.
    #[serde(default)]
    pub cross_entity_movement_source_unit: Option<u32>,
    #[serde(default)]
    pub cross_entity_movement_target_unit: Option<u32>,
}

impl DesignerAdmissionPreflightManifest {
    /// Canonical happy-path preflight manifest for FrontierV2 artifact targets.
    pub fn frontier_v2_happy_path() -> Self {
        Self {
            manifest_id: "frontier_v2_preflight_happy".into(),
            profile_name: "FrontierV2".into(),
            enabled_by_default: false,
            requested_artifact_targets: vec![
                "AcceptedFrontierV2FixtureArtifacts".into(),
                "FrontierV2CombinedFeedbackFixture".into(),
                "ResourceFlowAllocatorRoute".into(),
            ],
            requested_guardrail_overrides: Vec::new(),
            requested_runtime_features: Vec::new(),
            requested_mapping_features: Vec::new(),
            requested_resource_flow_features: Vec::new(),
            requested_authoring_frontend: Vec::new(),
            cross_entity_movement_source_unit: None,
            cross_entity_movement_target_unit: None,
        }
    }
}
