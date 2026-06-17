//! Studio session — SimThing-Spec scenario authority plus editor projections.

use std::path::PathBuf;

use simthing_mapgenerator::GenerationReport;
use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, SimThingScenarioSpec,
};

use crate::generation::{GenerationProfile, GenerationRunOutput};
use crate::hydration::{
    generate_simthing_spec_scenario, studio_projection_from_scenario_authority,
    studio_projection_from_simthing_spec, StudioHeatmapReadinessKind, StudioHydrationBoundary,
    StudioHydrationError,
};
use crate::scenario_projection::{
    build_gpu_residency_readiness, build_structural_projection, StudioGpuResidencyReadiness,
    StudioStructuralProjection,
};
use crate::view_model::StudioGalaxyViewModel;

#[derive(Debug, Clone, PartialEq)]
pub enum StudioSessionSource {
    Generated {
        generation_profile: GenerationProfile,
    },
    LoadedScenario {
        scenario_path: PathBuf,
        profile_hint: Option<GenerationProfile>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioScenarioSummary {
    pub scenario_id: String,
    pub system_count: u32,
    pub link_count: u32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub occupied_cells: u64,
    pub stead_valid: bool,
    pub links_valid: bool,
    pub rf_ready: bool,
    pub heatmap_readiness: StudioHeatmapReadinessKind,
    pub map_quality_status: Option<&'static str>,
}

impl StudioScenarioSummary {
    pub fn from_scenario(
        scenario: &SimThingScenarioSpec,
        report: Option<&GenerationReport>,
    ) -> Self {
        let stead_valid = validate_stead_mapping_consistency(scenario).is_ok();
        let links_valid = validate_scenario_links(scenario).is_ok();
        let rf = crate::hydration::rf_accumulator_readiness_from_simthing_spec(scenario);
        let heatmap = crate::hydration::heatmap_readiness_from_simthing_spec(scenario);
        Self {
            scenario_id: scenario.scenario_id.clone(),
            system_count: scenario.structural_grid.placements.len() as u32,
            link_count: scenario.links.len() as u32,
            grid_width: scenario.structural_grid.frame.width,
            grid_height: scenario.structural_grid.frame.height,
            occupied_cells: scenario.structural_grid.frame.occupied_cells,
            stead_valid,
            links_valid,
            rf_ready: rf.ready_for_spatial_rf_over_locations,
            heatmap_readiness: heatmap.readiness,
            map_quality_status: report.map(|r| r.output.map_quality_status),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StudioSession {
    pub scenario_authority: SimThingScenarioSpec,
    pub source: StudioSessionSource,
    pub scenario_summary: StudioScenarioSummary,
    pub structural_projection: StudioStructuralProjection,
    pub gpu_residency_readiness: StudioGpuResidencyReadiness,
    pub generated_output: Option<GenerationRunOutput>,
    pub hydration: StudioHydrationBoundary,
    pub view_model: StudioGalaxyViewModel,
    pub report_path: Option<PathBuf>,
    pub scenario_path: Option<PathBuf>,
}

impl StudioSession {
    pub fn from_generation(
        profile: GenerationProfile,
        output: GenerationRunOutput,
    ) -> Result<Self, StudioHydrationError> {
        let scenario_authority = generate_simthing_spec_scenario(&output)?;
        let hydration = studio_projection_from_simthing_spec(&scenario_authority, &output.report)?;
        let view_model = StudioGalaxyViewModel::from_hydration(&hydration);
        let structural_projection =
            build_structural_projection(&scenario_authority).map_err(StudioHydrationError::from)?;
        let gpu_residency_readiness =
            build_gpu_residency_readiness(&scenario_authority, &structural_projection);
        let scenario_summary =
            StudioScenarioSummary::from_scenario(&scenario_authority, Some(&output.report));

        Ok(Self {
            scenario_authority,
            source: StudioSessionSource::Generated {
                generation_profile: profile,
            },
            scenario_summary,
            structural_projection,
            gpu_residency_readiness,
            generated_output: Some(output),
            hydration,
            view_model,
            report_path: None,
            scenario_path: None,
        })
    }

    pub fn from_loaded_scenario(
        scenario_authority: SimThingScenarioSpec,
        scenario_path: PathBuf,
        profile_hint: Option<GenerationProfile>,
    ) -> Result<Self, StudioHydrationError> {
        let hydration = studio_projection_from_scenario_authority(&scenario_authority)?;
        let view_model = StudioGalaxyViewModel::from_hydration(&hydration);
        let structural_projection =
            build_structural_projection(&scenario_authority).map_err(StudioHydrationError::from)?;
        let gpu_residency_readiness =
            build_gpu_residency_readiness(&scenario_authority, &structural_projection);
        let scenario_summary = StudioScenarioSummary::from_scenario(&scenario_authority, None);

        Ok(Self {
            scenario_authority,
            source: StudioSessionSource::LoadedScenario {
                scenario_path: scenario_path.clone(),
                profile_hint,
            },
            scenario_summary,
            structural_projection,
            gpu_residency_readiness,
            generated_output: None,
            hydration,
            view_model,
            report_path: None,
            scenario_path: Some(scenario_path),
        })
    }

    pub fn profile(&self) -> GenerationProfile {
        match &self.source {
            StudioSessionSource::Generated { generation_profile } => generation_profile.clone(),
            StudioSessionSource::LoadedScenario { profile_hint, .. } => profile_hint
                .clone()
                .unwrap_or_else(|| profile_from_scenario_provenance(&self.scenario_authority)),
        }
    }

    pub fn is_loaded_scenario(&self) -> bool {
        matches!(self.source, StudioSessionSource::LoadedScenario { .. })
    }

    pub fn is_generated(&self) -> bool {
        matches!(self.source, StudioSessionSource::Generated { .. })
    }

    pub fn report(&self) -> Option<&GenerationReport> {
        self.generated_output.as_ref().map(|output| &output.report)
    }

    pub fn galaxy_name(&self) -> &str {
        &self.scenario_authority.scenario_id
    }

    pub fn status_message(&self) -> String {
        match &self.source {
            StudioSessionSource::Generated { .. } => {
                let quality = self.scenario_summary.map_quality_status.unwrap_or("PASS");
                format!(
                    "Generated {} systems — quality {}",
                    self.scenario_summary.system_count, quality
                )
            }
            StudioSessionSource::LoadedScenario { .. } => {
                let stead = if self.scenario_summary.stead_valid {
                    "STEAD valid"
                } else {
                    "STEAD invalid"
                };
                format!(
                    "Loaded scenario: {} systems, {} links, {}",
                    self.scenario_summary.system_count, self.scenario_summary.link_count, stead
                )
            }
        }
    }
}

fn profile_from_scenario_provenance(scenario: &SimThingScenarioSpec) -> GenerationProfile {
    let mut profile = GenerationProfile::default_spiral_2_dense_3000();
    if !scenario.provenance.generator_shape.is_empty() {
        profile.shape = scenario.provenance.generator_shape.clone();
    }
    profile.seed = scenario.provenance.generator_seed;
    profile.star_count = scenario.structural_grid.frame.occupied_cells as u32;
    profile.lattice_edge = scenario.structural_grid.frame.width;
    profile.target_hyperlanes = scenario.links.len() as u32;
    profile
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generation::run_generation;
    use crate::scenario_io::{
        load_studio_session_from_scenario_path, save_scenario_authority_to_path,
    };
    use tempfile::TempDir;

    #[test]
    fn studio_session_requires_hydrated_grid() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("hydrate session");

        assert_eq!(
            session.hydration.grid.occupied_cells as usize,
            session
                .generated_output
                .as_ref()
                .expect("generated output")
                .result
                .placement
                .systems
                .len()
        );
        assert_eq!(
            session.view_model.stars.len(),
            session.hydration.grid.gridcells.len()
        );
        assert_eq!(
            session.scenario_authority.scenario_id,
            session.hydration.simthing_spec_scenario_id
        );
    }

    #[test]
    fn failed_hydration_does_not_adopt_session() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let mut output = run_generation(&profile).expect("generate");
        output.result.placement.systems[1].coord = output.result.placement.systems[0].coord;

        let session = StudioSession::from_generation(profile, output);

        assert!(session.is_err());
    }

    #[test]
    fn generated_session_source_is_generated() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile.clone(), output).expect("session");
        assert!(session.is_generated());
        assert!(!session.is_loaded_scenario());
        assert!(session.generated_output.is_some());
        match session.source {
            StudioSessionSource::Generated { generation_profile } => {
                assert_eq!(generation_profile.preset_id, profile.preset_id)
            }
            _ => panic!("expected generated source"),
        }
    }

    #[test]
    fn loaded_session_source_is_loaded_scenario() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let generated = StudioSession::from_generation(profile, output).expect("session");
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("loaded-source.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &generated.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert!(loaded.is_loaded_scenario());
        assert!(!loaded.is_generated());
        assert!(loaded.generated_output.is_none());
    }

    #[test]
    fn loaded_session_does_not_require_generation_output_as_authority() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let generated = StudioSession::from_generation(profile, output).expect("session");
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("no-gen-output.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &generated.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert!(loaded.generated_output.is_none());
        assert_eq!(
            loaded.scenario_authority.scenario_id,
            generated.scenario_authority.scenario_id
        );
    }

    #[test]
    fn loaded_session_summary_derives_from_simthing_scenario_spec() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let generated = StudioSession::from_generation(profile, output).expect("session");
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("summary.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &generated.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert_eq!(
            loaded.scenario_summary.system_count,
            loaded.scenario_authority.structural_grid.placements.len() as u32
        );
        assert!(loaded.scenario_summary.stead_valid);
        assert!(loaded.scenario_summary.links_valid);
        assert!(loaded.scenario_summary.map_quality_status.is_none());
    }

    #[test]
    fn generated_session_summary_derives_from_simthing_scenario_spec() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("session");
        assert!(session.scenario_summary.stead_valid);
        assert!(session.scenario_summary.map_quality_status.is_some());
    }

    #[test]
    fn loaded_scenario_status_does_not_say_generated() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let generated = StudioSession::from_generation(profile, output).expect("session");
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("status.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &generated.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        let status = loaded.status_message();
        assert!(status.contains("Loaded scenario"));
        assert!(!status.contains("Generated"));
    }

    #[test]
    fn scenario_save_load_roundtrip_preserves_scenario_summary() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("session");
        let summary = session.scenario_summary.clone();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("roundtrip-summary.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &session.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert_eq!(loaded.scenario_summary.system_count, summary.system_count);
        assert_eq!(loaded.scenario_summary.link_count, summary.link_count);
        assert_eq!(loaded.scenario_summary.grid_width, summary.grid_width);
    }

    #[test]
    fn scenario_save_load_roundtrip_preserves_structural_projection() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("session");
        let projection = session.structural_projection.clone();
        let dir = TempDir::new().expect("tempdir");
        let path = dir
            .path()
            .join("roundtrip-projection.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &session.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert_eq!(loaded.structural_projection, projection);
    }

    #[test]
    fn scenario_save_load_roundtrip_preserves_gpu_residency_readiness() {
        let profile = GenerationProfile::default_spiral_2_dense_3000();
        let output = run_generation(&profile).expect("generate");
        let session = StudioSession::from_generation(profile, output).expect("session");
        let readiness = session.gpu_residency_readiness.clone();
        let dir = TempDir::new().expect("tempdir");
        let path = dir.path().join("roundtrip-gpu.simthing-scenario.json");
        save_scenario_authority_to_path(&path, &session.scenario_authority).expect("save");
        let loaded = load_studio_session_from_scenario_path(&path, None).expect("load");
        assert_eq!(loaded.gpu_residency_readiness, readiness);
    }
}
