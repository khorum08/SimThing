//! Studio presentation model for spec-owned Scenario ingestion/admission reports.
//!
//! Display-only — Studio does not own ingestion authority or driver compile semantics.

use simthing_spec::{
    ingest_scenario, ingest_scenario_from_str, owner_silo_admission_classification_label,
    scenario_deferral_kind_label, scenario_ingestion_classification_label,
    serialize_scenario_authority, studio_canonical_ingestion_profile, ScenarioDeferralKind,
    ScenarioIngestionClassification, ScenarioIngestionResult, SimThingScenarioSpec,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StudioScenarioDeferralSummary {
    pub kind: String,
    pub path: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioScenarioErrorSummary {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioCompileReadinessSummary {
    pub structural_n4_ready: bool,
    pub mapping_plan_ready: bool,
    pub owner_silo_gpu_participant_accumulation_ready: bool,
    pub owner_silo_full_state_mutation_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioOwnerSiloSummary {
    pub classification: String,
    pub participant_count: u32,
    pub reducible_surplus_total: f32,
    pub resolvable_deficit_total: f32,
    pub unresolved_deficit_total: f32,
    pub gpu_participant_accumulation_ready: bool,
    pub full_state_mutation_deferred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudioScenarioAdmissionSummary {
    pub classification: String,
    pub canonical_tree_status: String,
    pub validation_error_count: usize,
    pub deferral_count: usize,
    pub deferrals: Vec<StudioScenarioDeferralSummary>,
    pub errors: Vec<StudioScenarioErrorSummary>,
    pub owner_silo: Option<StudioOwnerSiloSummary>,
    pub compile_readiness: StudioCompileReadinessSummary,
    pub legacy_world_root: bool,
}

/// Build a presentation summary from spec-owned ingestion authority.
pub fn build_studio_admission_summary_from_ingestion(
    result: &ScenarioIngestionResult,
) -> StudioScenarioAdmissionSummary {
    let legacy_world_root = result
        .deferrals
        .iter()
        .any(|d| d.kind == ScenarioDeferralKind::LegacyWorldRootCompatibility);
    let canonical_tree_status = if result.validation.canonical_validation_ok {
        "canonical_valid".into()
    } else if result.validation.legacy_compat_ok {
        "legacy_world_root_compatibility".into()
    } else {
        "canonical_invalid".into()
    };

    StudioScenarioAdmissionSummary {
        classification: scenario_ingestion_classification_label(result.classification).into(),
        canonical_tree_status,
        validation_error_count: result.errors.len(),
        deferral_count: result.deferrals.len(),
        deferrals: result
            .deferrals
            .iter()
            .map(|deferral| StudioScenarioDeferralSummary {
                kind: scenario_deferral_kind_label(deferral.kind).into(),
                path: deferral.path.clone(),
                reason: deferral.reason.clone(),
            })
            .collect(),
        errors: result
            .errors
            .iter()
            .map(|error| StudioScenarioErrorSummary {
                code: error.code.clone(),
                message: error.message.clone(),
            })
            .collect(),
        owner_silo: result
            .owner_silo
            .as_ref()
            .map(|report| StudioOwnerSiloSummary {
                classification: owner_silo_admission_classification_label(report.classification)
                    .into(),
                participant_count: report.participant_count,
                reducible_surplus_total: report.reducible_surplus_total,
                resolvable_deficit_total: report.resolvable_deficit_total,
                unresolved_deficit_total: report.unresolved_deficit_total,
                gpu_participant_accumulation_ready: result
                    .compile_readiness
                    .owner_silo_gpu_participant_accumulation_ready,
                full_state_mutation_deferred: result
                    .compile_readiness
                    .owner_silo_full_state_mutation_deferred,
            }),
        compile_readiness: StudioCompileReadinessSummary {
            structural_n4_ready: result.compile_readiness.structural_n4_ready,
            mapping_plan_ready: result.compile_readiness.mapping_plan_ready,
            owner_silo_gpu_participant_accumulation_ready: result
                .compile_readiness
                .owner_silo_gpu_participant_accumulation_ready,
            owner_silo_full_state_mutation_deferred: result
                .compile_readiness
                .owner_silo_full_state_mutation_deferred,
        },
        legacy_world_root,
    }
}

/// Ingest scenario JSON for admission display without requiring a Studio document.
pub fn studio_ingest_scenario_text_for_report(
    source_name: &str,
    json: &str,
) -> StudioScenarioAdmissionSummary {
    let (result, _) =
        ingest_scenario_from_str(source_name, json, studio_canonical_ingestion_profile());
    build_studio_admission_summary_from_ingestion(&result)
}

/// Ingest an already-deserialized scenario for admission display.
pub fn build_studio_admission_summary_from_spec(
    source_name: &str,
    spec: &SimThingScenarioSpec,
) -> StudioScenarioAdmissionSummary {
    let result = ingest_scenario(source_name, spec, studio_canonical_ingestion_profile());
    build_studio_admission_summary_from_ingestion(&result)
}

/// Returns true when the ingestion classification permits building a canonical Studio document.
pub fn studio_admission_allows_canonical_document(
    classification: ScenarioIngestionClassification,
) -> bool {
    matches!(
        classification,
        ScenarioIngestionClassification::Admitted
            | ScenarioIngestionClassification::PartiallyAdmitted
            | ScenarioIngestionClassification::Unsupported
    )
}

/// Snapshot scenario authority JSON before/after admission display to prove no mutation.
pub fn studio_scenario_authority_snapshot(spec: &SimThingScenarioSpec) -> String {
    serialize_scenario_authority(spec).expect("serialize scenario authority")
}
