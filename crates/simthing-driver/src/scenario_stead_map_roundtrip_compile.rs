//! SCENARIO-STEAD-MAP-ROUNDTRIP-0 — driver compile plan for STEAD map roundtrip proof.

use simthing_spec::{
    evaluate_scenario_stead_map_roundtrip_from_json_str, ScenarioSteadMapRoundtripReport, SpecError,
};

use crate::scenario_canonical_io_compile::{
    compile_scenario_canonical_io_plan_from_json_str, ScenarioCanonicalIoPlan,
};

/// Driver compile plan for STEAD map roundtrip over canonical ScenarioSpec I/O.
#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioSteadMapRoundtripPlan {
    pub canonical_io_plan: ScenarioCanonicalIoPlan,
    pub stead_roundtrip_report: ScenarioSteadMapRoundtripReport,
    pub stead_ids_stable: bool,
    pub links_stable: bool,
    pub spatial_tree_stable: bool,
    pub rf_metadata_stable: bool,
    pub studio_projection_rebuild_ready: bool,
    pub runtime_mutation_deferred: bool,
    pub savefile_persistence_deferred: bool,
}

/// Compile STEAD map roundtrip plan from JSON source bytes, composing canonical I/O from #828.
pub fn compile_scenario_stead_map_roundtrip_plan_from_json_str(
    source_label: &str,
    json: &str,
) -> Result<ScenarioSteadMapRoundtripPlan, SpecError> {
    let canonical_io_plan = compile_scenario_canonical_io_plan_from_json_str(source_label, json)?;
    let stead_roundtrip_report =
        evaluate_scenario_stead_map_roundtrip_from_json_str(source_label, json)?;

    Ok(ScenarioSteadMapRoundtripPlan {
        stead_ids_stable: stead_roundtrip_report.stead_ids_stable,
        links_stable: stead_roundtrip_report.links_stable,
        spatial_tree_stable: stead_roundtrip_report.spatial_tree_stable,
        rf_metadata_stable: stead_roundtrip_report.rf_metadata_stable,
        studio_projection_rebuild_ready: stead_roundtrip_report.studio_projection_rebuild_ready,
        runtime_mutation_deferred: true,
        savefile_persistence_deferred: true,
        canonical_io_plan,
        stead_roundtrip_report,
    })
}
