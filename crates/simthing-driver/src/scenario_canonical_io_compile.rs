//! SCENARIO-CANONICAL-LOAD-SAVE-ROUNDTRIP-0 — compile plan for canonical ScenarioSpec I/O.

use simthing_spec::{
    prove_scenario_canonical_load_save_roundtrip, ScenarioCanonicalRoundtripReport, SpecError,
};

/// Driver compile plan for Studio scenario import/export canonical I/O.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioCanonicalIoPlan {
    pub roundtrip_report: ScenarioCanonicalRoundtripReport,
    pub studio_import_export_ready: bool,
    pub savefile_persistence_deferred: bool,
    pub runtime_mutation_deferred: bool,
}

/// Compile canonical ScenarioSpec I/O roundtrip plan from JSON source bytes.
pub fn compile_scenario_canonical_io_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioCanonicalIoPlan, SpecError> {
    let roundtrip_report = prove_scenario_canonical_load_save_roundtrip(source_label, json)?;
    let studio_import_export_ready = roundtrip_report.scenario_authority_preserved
        && roundtrip_report.initial_load.loaded
        && roundtrip_report.initial_load.ingestion_ready
        && roundtrip_report.roundtrip_load.loaded
        && roundtrip_report.roundtrip_load.ingestion_ready;

    Ok(ScenarioCanonicalIoPlan {
        roundtrip_report,
        studio_import_export_ready,
        savefile_persistence_deferred: true,
        runtime_mutation_deferred: true,
    })
}
