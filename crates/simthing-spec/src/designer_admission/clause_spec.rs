//! CLAUSE-SPEC-0 — RON-first FrontierV2 designer scenario admission.
//!
//! This module admits and lowers a designer-authored FrontierV2 scenario to the
//! accepted FrontierV2 fixture artifact targets. It is metadata/admission only:
//! no ClauseScript parser, ClauseThing runtime, GPU dispatch, or production
//! `SimSession` wiring is created here.

use serde::{Deserialize, Serialize};

use crate::spec::region_field::MappingExecutionProfile;
use crate::spec::resource_flow::{ResourceFlowExecutionProfile, ResourceFlowOptInMode};

use super::artifact_target::{
    AcceptedFrontierArtifactTarget, accepted_frontier_v2_artifact_target_ids,
};
use super::diagnostic::{
    DesignerAdmissionDiagnostic, DesignerAdmissionDiagnosticCode, designer_admission_diagnostic,
};
use super::manifest::DesignerAdmissionPreflightManifest;
use super::preflight::resolve_frontier_artifact_target_id;
use super::preview::preview_designer_admission_preflight;

pub const CLAUSE_SPEC_FRONTIER_V2_PROFILE: &str = "FrontierV2";
pub const CLAUSE_SPEC_FRONTIER_V2_GRID_CAP: u32 = 32;
pub const CLAUSE_SPEC_FRONTIER_V2_MIN_TICKS: u32 = 2;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecFrontierV2Scenario {
    pub scenario_id: String,
    pub profile_name: String,
    #[serde(default)]
    pub enabled_by_default: bool,
    pub grid: ClauseSpecGrid,
    pub closed_loop_ticks: u32,
    #[serde(default)]
    pub factions: Vec<ClauseSpecFaction>,
    pub resource_flow: ClauseSpecResourceFlow,
    pub mapping: ClauseSpecMapping,
    pub movement_feedback: ClauseSpecMovementFeedback,
    pub structural_feedback: ClauseSpecStructuralFeedback,
    pub artifact_targets: ClauseSpecArtifactTargets,
    #[serde(default)]
    pub cpu_planner: bool,
    #[serde(default)]
    pub cpu_urgency: bool,
    #[serde(default)]
    pub cpu_commitment_emission: bool,
    #[serde(default)]
    pub scheduler_cache: bool,
    #[serde(default)]
    pub default_sim_session_wiring: bool,
    #[serde(default)]
    pub simthing_sim_semantic_state: bool,
    #[serde(default)]
    pub frontier_v2_5: bool,
    #[serde(default)]
    pub act_5: bool,
    #[serde(default)]
    pub event_3: bool,
    #[serde(default)]
    pub obs_5: bool,
    #[serde(default)]
    pub pipe_1: bool,
    #[serde(default)]
    pub clause_script_parser: bool,
    #[serde(default)]
    pub clausething_runtime: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClauseSpecGrid {
    pub rows: u32,
    pub cols: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecFaction {
    pub faction_id: String,
    pub initial_seed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClauseSpecResourceFlowRoute {
    ResourceFlowAllocator,
    ResourceFlowBypass,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecResourceFlow {
    pub opt_in: ResourceFlowOptInMode,
    pub execution_profile: ResourceFlowExecutionProfile,
    pub route: ClauseSpecResourceFlowRoute,
    pub depth_cap: u32,
    #[serde(default)]
    pub global_default_on: bool,
    #[serde(default)]
    pub shared_pool_tick_write: bool,
    #[serde(default)]
    pub parallel_fixture_economy: bool,
    #[serde(default)]
    pub nested_e11b: bool,
    #[serde(default)]
    pub e11b_5_dynamic_enrollment: bool,
    #[serde(default)]
    pub d2_hard_currency_allocator: bool,
    #[serde(default)]
    pub d2a_boundary_scheduling: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecMapping {
    pub execution_profile: MappingExecutionProfile,
    #[serde(default)]
    pub atlas: bool,
    #[serde(default)]
    pub active_mask: bool,
    #[serde(default)]
    pub perception: bool,
    #[serde(default)]
    pub source_identity: bool,
    #[serde(default)]
    pub semantic_wgsl: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClauseSpecMovementFeedbackMode {
    OwnColumnShadowOnly,
    CrossEntityWrite,
    ProductionWrite,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecMovementFeedback {
    pub mode: ClauseSpecMovementFeedbackMode,
    #[serde(default)]
    pub allow_cross_entity_write: bool,
    #[serde(default)]
    pub allow_production_write: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClauseSpecStructuralFeedbackMode {
    BoundaryRequestShadowOnly,
    ProductionCommitment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClauseSpecStructuralFeedback {
    pub mode: ClauseSpecStructuralFeedbackMode,
    #[serde(default)]
    pub allow_production_commitment: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClauseSpecArtifactTargets {
    #[serde(default)]
    pub accepted_frontier_v2_fixture_artifacts: bool,
    #[serde(default)]
    pub combined_feedback_fixture: bool,
    #[serde(default)]
    pub own_column_shadow: bool,
    #[serde(default)]
    pub boundary_request_shadow: bool,
    #[serde(default)]
    pub resource_flow_allocator_route: bool,
    #[serde(default)]
    pub additional_targets: Vec<String>,
}

impl ClauseSpecArtifactTargets {
    pub fn accepted_all() -> Self {
        Self {
            accepted_frontier_v2_fixture_artifacts: true,
            combined_feedback_fixture: true,
            own_column_shadow: true,
            boundary_request_shadow: true,
            resource_flow_allocator_route: true,
            additional_targets: Vec::new(),
        }
    }

    pub fn requested_target_ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        if self.accepted_frontier_v2_fixture_artifacts {
            ids.push("AcceptedFrontierV2FixtureArtifacts".into());
        }
        if self.combined_feedback_fixture {
            ids.push("FrontierV2CombinedFeedbackFixture".into());
        }
        if self.own_column_shadow {
            ids.push("FrontierV2OwnColumnShadow".into());
        }
        if self.boundary_request_shadow {
            ids.push("FrontierV2BoundaryRequestShadow".into());
        }
        if self.resource_flow_allocator_route {
            ids.push("ResourceFlowAllocatorRoute".into());
        }
        ids.extend(self.additional_targets.iter().cloned());
        ids
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClauseSpecFrontierV2Admission {
    pub scenario_id: String,
    pub accepted_artifact_targets: Vec<AcceptedFrontierArtifactTarget>,
    pub grid: ClauseSpecGrid,
    pub closed_loop_ticks: u32,
    pub lowering_summary: ClauseSpecFrontierV2LoweringSummary,
    pub diagnostics: Vec<DesignerAdmissionDiagnostic>,
    pub accepted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClauseSpecFrontierV2LoweringSummary {
    pub maps_to_frontier_v2_combined_feedback_fixture: bool,
    pub resource_route: AcceptedFrontierArtifactTarget,
    pub movement: AcceptedFrontierArtifactTarget,
    pub structural: AcceptedFrontierArtifactTarget,
    pub metadata_only: bool,
}

pub fn admit_clause_spec_frontier_v2_scenario(
    scenario: &ClauseSpecFrontierV2Scenario,
) -> ClauseSpecFrontierV2Admission {
    let manifest = scenario.to_preflight_manifest();
    let preview = preview_designer_admission_preflight(&manifest);
    let mut diagnostics = preview.diagnostics;

    validate_clause_spec_frontier_v2_fields(scenario, &mut diagnostics);

    let accepted_artifact_targets = scenario
        .artifact_targets
        .requested_target_ids()
        .iter()
        .filter_map(|id| resolve_frontier_artifact_target_id(id))
        .collect::<Vec<_>>();
    let accepted = diagnostics.is_empty();

    ClauseSpecFrontierV2Admission {
        scenario_id: scenario.scenario_id.clone(),
        accepted_artifact_targets,
        grid: scenario.grid,
        closed_loop_ticks: scenario.closed_loop_ticks,
        lowering_summary: ClauseSpecFrontierV2LoweringSummary {
            maps_to_frontier_v2_combined_feedback_fixture: true,
            resource_route: AcceptedFrontierArtifactTarget::ResourceFlowAllocatorRoute,
            movement: AcceptedFrontierArtifactTarget::FrontierV2OwnColumnShadow,
            structural: AcceptedFrontierArtifactTarget::FrontierV2BoundaryRequestShadow,
            metadata_only: true,
        },
        diagnostics,
        accepted,
    }
}

impl ClauseSpecFrontierV2Scenario {
    pub fn happy_path() -> Self {
        Self {
            scenario_id: "clause_spec0_frontier_v2_happy".into(),
            profile_name: CLAUSE_SPEC_FRONTIER_V2_PROFILE.into(),
            enabled_by_default: false,
            grid: ClauseSpecGrid { rows: 8, cols: 8 },
            closed_loop_ticks: 4,
            factions: vec![
                ClauseSpecFaction {
                    faction_id: "faction_a".into(),
                    initial_seed: 32,
                },
                ClauseSpecFaction {
                    faction_id: "faction_b".into(),
                    initial_seed: 24,
                },
            ],
            resource_flow: ClauseSpecResourceFlow {
                opt_in: ResourceFlowOptInMode::FlatStarOptIn,
                execution_profile: ResourceFlowExecutionProfile::RecursiveArenaResourceFlow,
                route: ClauseSpecResourceFlowRoute::ResourceFlowAllocator,
                depth_cap: 2,
                global_default_on: false,
                shared_pool_tick_write: false,
                parallel_fixture_economy: false,
                nested_e11b: false,
                e11b_5_dynamic_enrollment: false,
                d2_hard_currency_allocator: false,
                d2a_boundary_scheduling: false,
            },
            mapping: ClauseSpecMapping {
                execution_profile: MappingExecutionProfile::SparseRegionFieldV1,
                atlas: false,
                active_mask: false,
                perception: false,
                source_identity: false,
                semantic_wgsl: false,
            },
            movement_feedback: ClauseSpecMovementFeedback {
                mode: ClauseSpecMovementFeedbackMode::OwnColumnShadowOnly,
                allow_cross_entity_write: false,
                allow_production_write: false,
            },
            structural_feedback: ClauseSpecStructuralFeedback {
                mode: ClauseSpecStructuralFeedbackMode::BoundaryRequestShadowOnly,
                allow_production_commitment: false,
            },
            artifact_targets: ClauseSpecArtifactTargets::accepted_all(),
            cpu_planner: false,
            cpu_urgency: false,
            cpu_commitment_emission: false,
            scheduler_cache: false,
            default_sim_session_wiring: false,
            simthing_sim_semantic_state: false,
            frontier_v2_5: false,
            act_5: false,
            event_3: false,
            obs_5: false,
            pipe_1: false,
            clause_script_parser: false,
            clausething_runtime: false,
        }
    }

    pub fn to_preflight_manifest(&self) -> DesignerAdmissionPreflightManifest {
        let mut manifest = DesignerAdmissionPreflightManifest {
            manifest_id: self.scenario_id.clone(),
            profile_name: self.profile_name.clone(),
            enabled_by_default: self.enabled_by_default || self.default_sim_session_wiring,
            requested_artifact_targets: self.artifact_targets.requested_target_ids(),
            requested_guardrail_overrides: Vec::new(),
            requested_runtime_features: Vec::new(),
            requested_mapping_features: Vec::new(),
            requested_resource_flow_features: Vec::new(),
            requested_authoring_frontend: Vec::new(),
            cross_entity_movement_source_unit: None,
            cross_entity_movement_target_unit: None,
        };

        if self.resource_flow.route == ClauseSpecResourceFlowRoute::ResourceFlowBypass {
            manifest
                .requested_guardrail_overrides
                .push("resource_flow_bypass".into());
        }
        if self.resource_flow.shared_pool_tick_write {
            manifest
                .requested_guardrail_overrides
                .push("shared_pool_tick_write".into());
        }
        if self.resource_flow.parallel_fixture_economy {
            manifest
                .requested_guardrail_overrides
                .push("parallel_fixture_economy".into());
        }
        if self.resource_flow.global_default_on {
            manifest
                .requested_guardrail_overrides
                .push("default_on".into());
        }
        if self.movement_feedback.allow_cross_entity_write
            || self.movement_feedback.mode == ClauseSpecMovementFeedbackMode::CrossEntityWrite
        {
            manifest
                .requested_guardrail_overrides
                .push("cross_entity_movement_write".into());
            manifest.cross_entity_movement_source_unit = Some(1);
            manifest.cross_entity_movement_target_unit = Some(2);
        }
        if self.movement_feedback.allow_production_write
            || self.movement_feedback.mode == ClauseSpecMovementFeedbackMode::ProductionWrite
        {
            manifest
                .requested_guardrail_overrides
                .push("production_movement_write".into());
        }
        if self.structural_feedback.allow_production_commitment
            || self.structural_feedback.mode
                == ClauseSpecStructuralFeedbackMode::ProductionCommitment
        {
            manifest
                .requested_guardrail_overrides
                .push("production_commitment_emission".into());
        }
        if self.cpu_planner {
            manifest
                .requested_guardrail_overrides
                .push("cpu_planner".into());
        }
        if self.cpu_urgency {
            manifest
                .requested_guardrail_overrides
                .push("cpu_urgency".into());
        }
        if self.cpu_commitment_emission {
            manifest
                .requested_guardrail_overrides
                .push("cpu_commitment_emission".into());
        }
        if self.scheduler_cache {
            manifest
                .requested_runtime_features
                .push("scheduler_cache".into());
        }
        if self.simthing_sim_semantic_state {
            manifest
                .requested_runtime_features
                .push("simthing_sim_semantic_state".into());
        }
        if self.frontier_v2_5 {
            manifest
                .requested_runtime_features
                .push("frontier_v2_5".into());
        }
        for (enabled, token) in [
            (self.act_5, "act_5"),
            (self.event_3, "event_3"),
            (self.obs_5, "obs_5"),
            (self.pipe_1, "pipe_1"),
        ] {
            if enabled {
                manifest.requested_runtime_features.push(token.into());
            }
        }
        if self.mapping.atlas {
            manifest
                .requested_mapping_features
                .push("atlas_batching".into());
        }
        if self.mapping.active_mask {
            manifest
                .requested_mapping_features
                .push("active_mask".into());
        }
        if self.mapping.perception {
            manifest
                .requested_mapping_features
                .push("perception_fog".into());
        }
        if self.mapping.source_identity {
            manifest
                .requested_mapping_features
                .push("source_identity".into());
        }
        if self.mapping.semantic_wgsl {
            manifest
                .requested_runtime_features
                .push("semantic_wgsl".into());
        }
        if self.resource_flow.nested_e11b || self.resource_flow.depth_cap > 2 {
            manifest
                .requested_resource_flow_features
                .push("nested_e11b".into());
        }
        if self.resource_flow.e11b_5_dynamic_enrollment {
            manifest
                .requested_resource_flow_features
                .push("e11b5_dynamic_enrollment".into());
        }
        if self.resource_flow.d2_hard_currency_allocator
            || self.resource_flow.d2a_boundary_scheduling
        {
            manifest
                .requested_resource_flow_features
                .push("d2a_boundary_scheduling".into());
        }
        if self.clause_script_parser {
            manifest
                .requested_authoring_frontend
                .push("clause_script_parser".into());
        }
        if self.clausething_runtime {
            manifest
                .requested_authoring_frontend
                .push("clausething_runtime".into());
        }

        manifest
    }
}

fn validate_clause_spec_frontier_v2_fields(
    scenario: &ClauseSpecFrontierV2Scenario,
    diagnostics: &mut Vec<DesignerAdmissionDiagnostic>,
) {
    if scenario.scenario_id.trim().is_empty() {
        diagnostics.push(malformed("scenario_id must be non-empty"));
    }
    if scenario.profile_name != CLAUSE_SPEC_FRONTIER_V2_PROFILE {
        diagnostics.push(malformed("profile_name must be FrontierV2"));
    }
    if scenario.grid.rows == 0
        || scenario.grid.cols == 0
        || scenario.grid.rows > CLAUSE_SPEC_FRONTIER_V2_GRID_CAP
        || scenario.grid.cols > CLAUSE_SPEC_FRONTIER_V2_GRID_CAP
    {
        diagnostics.push(malformed(format!(
            "grid must be 1..={CLAUSE_SPEC_FRONTIER_V2_GRID_CAP} rows/cols"
        )));
    }
    if scenario.closed_loop_ticks < CLAUSE_SPEC_FRONTIER_V2_MIN_TICKS {
        diagnostics.push(malformed(format!(
            "closed_loop_ticks must be >= {CLAUSE_SPEC_FRONTIER_V2_MIN_TICKS}"
        )));
    }
    if scenario.factions.len() < 2 {
        diagnostics.push(malformed(
            "FrontierV2 scenario requires at least two factions",
        ));
    }
    if scenario.resource_flow.opt_in != ResourceFlowOptInMode::FlatStarOptIn {
        diagnostics.push(malformed("resource_flow.opt_in must be FlatStarOptIn"));
    }
    if scenario.resource_flow.execution_profile
        != ResourceFlowExecutionProfile::RecursiveArenaResourceFlow
    {
        diagnostics.push(malformed(
            "resource_flow.execution_profile must be RecursiveArenaResourceFlow",
        ));
    }
    if scenario.mapping.execution_profile != MappingExecutionProfile::SparseRegionFieldV1 {
        diagnostics.push(malformed(
            "mapping.execution_profile must be SparseRegionFieldV1",
        ));
    }

    let requested = scenario.artifact_targets.requested_target_ids();
    for required in accepted_frontier_v2_artifact_target_ids() {
        if !requested.iter().any(|id| id == required) {
            diagnostics.push(malformed(format!(
                "missing required FrontierV2 artifact target `{required}`"
            )));
        }
    }
}

fn malformed(message: impl Into<String>) -> DesignerAdmissionDiagnostic {
    designer_admission_diagnostic(
        DesignerAdmissionDiagnosticCode::MalformedManifestRejected,
        message,
        Some("CLAUSE-SPEC-0 admits only bounded, default-off FrontierV2 scenario specs"),
    )
}
