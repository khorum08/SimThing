//! Driver-side compile-readiness evaluation for ingested canonical scenarios.
//!
//! Reuses existing structural N4 theater compile surfaces only — no new GPU primitives.

use simthing_spec::{
    MappingExecutionProfile, ScenarioCompileReadinessReport, SimThingScenarioSpec,
};

use crate::structural_n4_theater_compile::{
    compile_structural_n4_theater, StructuralTheaterAdmission,
};

/// Evaluate structural N4 theater admission for an ingested scenario authority.
pub fn evaluate_scenario_compile_readiness(
    spec: &SimThingScenarioSpec,
) -> ScenarioCompileReadinessReport {
    let mut report = ScenarioCompileReadinessReport {
        mapping_plan_deferred: true,
        ..Default::default()
    };

    match compile_structural_n4_theater(spec, MappingExecutionProfile::SparseRegionFieldV1) {
        Ok(StructuralTheaterAdmission::Admit(_)) => {
            report.structural_n4_ready = true;
            report.note = Some(
                "structural N4 theater admitted; mapping plan compile requires operator specs"
                    .into(),
            );
        }
        Ok(StructuralTheaterAdmission::AtlasDeferred { reason, .. }) => {
            report.structural_n4_deferred = true;
            report.note = Some(format!("structural N4 atlas deferral: {reason:?}"));
        }
        Err(err) => {
            report.note = Some(format!("structural N4 compile error: {err}"));
        }
    }

    report
}
